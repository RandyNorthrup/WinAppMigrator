use std::collections::hash_map::RandomState;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use tempfile::TempDir;

use crate::journal::machine_state_root;
use crate::path_utils::{
    POWERSHELL_IMPORT_MANAGEMENT, POWERSHELL_IMPORT_SECURITY, hidden_command, machine_variable,
    powershell_command, tool_command, without_verbatim_prefix,
};

/// SIDs rather than names, because the names are translated on other Windows languages.
const ADMINISTRATORS_SID: &str = "*S-1-5-32-544";
const ADMINISTRATORS_ONLY_GRANTS: [&str; 2] = ["*S-1-5-32-544:(OI)(CI)F", "*S-1-5-18:(OI)(CI)F"];
/// The mandatory labels of a process running with full rights: High, and System.
const ELEVATED_LABELS: [&str; 2] = ["S-1-16-12288", "S-1-16-16384"];
/// The path travels in this variable, so that no quoting can go wrong.
const ACL_PATH_VARIABLE: &str = "WAM_ACL_PATH";
const READ_SDDL_SCRIPT: &str = "(Get-Acl -LiteralPath $env:WAM_ACL_PATH).Sddl";
const READ_OWNER_SCRIPT: &str = "(Get-Acl -LiteralPath $env:WAM_ACL_PATH).\
GetOwner([Security.Principal.SecurityIdentifier]).Value";
/// `yes` if only Administrators, its members, SYSTEM, and TrustedInstaller own a file and can change
/// it. The bits are every right that writes to a file or its permissions, and the generic ones that
/// include them. An administrator who made a file is named on it by their own account.
const ONLY_ADMINISTRATORS_CAN_CHANGE_SCRIPT: &str = "Import-Module Microsoft.PowerShell.LocalAccounts \
-ErrorAction Ignore; \
$acl = Get-Acl -LiteralPath $env:WAM_ACL_PATH; \
$admins = @('S-1-5-32-544', 'S-1-5-18', \
'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464') + \
@(Get-LocalGroupMember -SID 'S-1-5-32-544' -ErrorAction Ignore | ForEach-Object { $_.SID.Value }); \
$owner = $acl.GetOwner([Security.Principal.SecurityIdentifier]).Value; \
$write = 0x2 -bor 0x4 -bor 0x10 -bor 0x100 -bor 0x10000 -bor 0x40000 -bor 0x80000 -bor 0x10000000 -bor 0x40000000; \
$others = @($acl.GetAccessRules($true, $true, [Security.Principal.SecurityIdentifier]) | \
Where-Object { $_.AccessControlType -eq 'Allow' -and ([int]$_.FileSystemRights -band $write) -and \
$admins -notcontains $_.IdentityReference.Value }); \
if ($admins -contains $owner -and $others.Count -eq 0) { 'yes' } else { 'no' }";
/// `yes` if a folder made here would give someone other than administrators what lets them keep it:
/// the right to change its permissions or take it over, by what it inherits. A process can open a new
/// folder in the moment before its permissions are closed, and a handle keeps what it was given.
/// `CREATOR OWNER` is whoever makes the folder, which is this process. An administrator's own account
/// is not let off here, unlike in [`ONLY_ADMINISTRATORS_CAN_CHANGE_SCRIPT`]: it is what a program of
/// theirs runs as with no elevation at all, on nearly any PC, and that is who this is about.
/// Where a setup program keeps what it unpacks, in the admin-only folder this keeps its state in.
const TOOL_TEMP_FOLDER: &str = "tool-temp";
pub const TOOL_TEMP_PREFIX: &str = "tool-";
const OTHERS_COULD_KEEP_A_NEW_FOLDER_SCRIPT: &str = "$ErrorActionPreference = 'Stop'; \
$acl = Get-Acl -LiteralPath $env:WAM_ACL_PATH; \
$admins = @('S-1-5-32-544', 'S-1-5-18', 'S-1-3-0', \
'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464'); \
$control = 0x40000 -bor 0x80000 -bor 0x10000000; \
$others = @($acl.GetAccessRules($true, $true, [Security.Principal.SecurityIdentifier]) | \
Where-Object { $_.AccessControlType -eq 'Allow' -and ([int]$_.FileSystemRights -band $control) -and \
([int]$_.InheritanceFlags -band 1) -and $admins -notcontains $_.IdentityReference.Value }); \
if ($others.Count -gt 0) { 'yes' } else { 'no' }";
/// `yes` if only Administrators, its members, SYSTEM, and TrustedInstaller own and can change all of
/// what is directly in a folder. See [`ONLY_ADMINISTRATORS_CAN_CHANGE_SCRIPT`]. Not the file that marks
/// one install at a time, which is owned by whoever last made it, and nothing is ever read from.
const ONLY_ADMINISTRATORS_MADE_WHAT_IS_INSIDE_SCRIPT: &str = "Import-Module Microsoft.PowerShell.LocalAccounts \
-ErrorAction Ignore; \
$admins = @('S-1-5-32-544', 'S-1-5-18', \
'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464') + \
@(Get-LocalGroupMember -SID 'S-1-5-32-544' -ErrorAction Ignore | ForEach-Object { $_.SID.Value }); \
$write = 0x2 -bor 0x4 -bor 0x10 -bor 0x100 -bor 0x10000 -bor 0x40000 -bor 0x80000 -bor 0x10000000 -bor 0x40000000; \
$bad = @(Get-ChildItem -Force -LiteralPath $env:WAM_ACL_PATH | Where-Object { $_.Name -ne 'busy.lock' } | Where-Object { \
$acl = Get-Acl -LiteralPath $_.FullName; \
$owner = $acl.GetOwner([Security.Principal.SecurityIdentifier]).Value; \
$others = @($acl.GetAccessRules($true, $true, [Security.Principal.SecurityIdentifier]) | \
Where-Object { $_.AccessControlType -eq 'Allow' -and ([int]$_.FileSystemRights -band $write) -and \
$admins -notcontains $_.IdentityReference.Value }); \
-not ($admins -contains $owner -and $others.Count -eq 0) }); \
if ($bad.Count -eq 0) { 'yes' } else { 'no' }";
/// The owner a protected folder has in SDDL, the form that reads the same in any language.
const SDDL_ADMINISTRATORS_OWNER: &str = "O:BA";
const SDDL_GROUP_MARKER: &str = "G:";
const SDDL_DACL_MARKER: &str = "D:";
const SDDL_DACL_PROTECTED_FLAG: char = 'P';
/// [`ADMINISTRATORS_ONLY_GRANTS`] as Windows writes them back, in sorted order.
const SDDL_ADMINISTRATORS_ONLY_ACES: [&str; 2] = ["A;OICI;FA;;;BA", "A;OICI;FA;;;SY"];
const FILE_SHARE_READ: u32 = 0x0000_0001;
const FILE_SHARE_WRITE: u32 = 0x0000_0002;
/// What lets `CreateFile` open a folder at all.
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
/// Opens whatever has the name, rather than what a link with that name points at.
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
/// Lets someone who is not elevated start a file, which is how Windows finds out from its
/// manifest that it has to ask for elevation. No one but administrators can change it.
const USERS_MAY_START_GRANT: &str = "*S-1-5-32-545:(RX)";
/// Put briefly in a folder while something is done in it. See [`pin_held_folder`].
const PIN_FILE_PREFIX: &str = ".wam-pin-";
const PIN_FILE_RANDOM_DIGITS: usize = 16;
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x0000_0002;
const GENERIC_WRITE: u32 = 0x4000_0000;
const DELETE: u32 = 0x0001_0000;
const FILE_FLAG_DELETE_ON_CLOSE: u32 = 0x0400_0000;

/// Whether this process runs with full administrator rights.
pub fn is_elevated() -> bool {
    elevation() == Some(true)
}

/// Whether this could be running elevated, for what is only safe when it is not. Not being able to
/// find out counts as could.
pub(crate) fn may_be_elevated() -> bool {
    elevation() != Some(false)
}

/// Asked once, since the answer cannot change during a run. `None` means it could not be
/// found out, which is not the same as no: asking runs a program, and that can be made to
/// fail. Asking `whoami` rather than PowerShell also takes a fraction of the time.
fn elevation() -> Option<bool> {
    static ELEVATED: OnceLock<Option<bool>> = OnceLock::new();
    *ELEVATED.get_or_init(query_elevation)
}

fn query_elevation() -> Option<bool> {
    let output = tool_command(OsStr::new("whoami"))
        .args(["/groups", "/fo", "csv"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| groups_show_elevation(&String::from_utf8_lossy(&output.stdout)))
}

/// The group list shows the process's mandatory label by SID, whatever the language.
fn groups_show_elevation(groups: &str) -> bool {
    ELEVATED_LABELS.iter().any(|label| groups.contains(label))
}

/// A folder that elevated code reads from: new, held open so it and the folders above it
/// cannot be renamed and replaced, and when elevated writable only by administrators.
/// Removed when dropped.
///
/// Permissions on a folder do not stop whoever controls its parent from renaming it and
/// putting another in its place, and everything reads by path. Holding it open does.
pub struct ProtectedFolder {
    // In this order, because a field drops in the order it is declared, and the folder
    // cannot be removed while it is held.
    _hold: File,
    temp: TempDir,
}

impl ProtectedFolder {
    /// Create one in `parent`, with a name starting `prefix`. A failure means nothing
    /// should be written there.
    pub fn new_in(parent: &Path, prefix: &str) -> io::Result<Self> {
        let temp = tempfile::Builder::new().prefix(prefix).tempdir_in(parent)?;
        Self::protect(temp)
    }

    /// Create one in the system's temp folder.
    pub fn new_in_temp(prefix: &str) -> io::Result<Self> {
        let temp = tempfile::Builder::new().prefix(prefix).tempdir()?;
        Self::protect(temp)
    }

    fn protect(temp: TempDir) -> io::Result<Self> {
        let hold = hold_real_folder(temp.path())?;
        protect_staging_folder(temp.path())?;
        Ok(Self { _hold: hold, temp })
    }

    pub fn path(&self) -> &Path {
        self.temp.path()
    }

    /// Keep the folder from being removed when this is dropped, and go on holding it.
    pub fn keep(self) -> (PathBuf, File) {
        let Self { _hold: hold, temp } = self;
        (temp.keep(), hold)
    }
}

/// Hold a folder open with no delete sharing. While the result is alive, neither it nor
/// any folder above it can be renamed, and what is inside can still be used normally.
pub fn hold_folder(folder: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(folder)
}

/// [`hold_folder`], for a folder that must be the real thing. This opens whatever has that
/// name and not what a link there points at, so what it checks is what it holds. Once held it
/// cannot be renamed, deleted, or replaced by something else of that name, and a cloud
/// placeholder, which redirects nothing, passes.
///
/// Whoever can write to it can still empty it and turn that same folder into a link, which a
/// hold does not stop. [`pin_held_folder`] does.
pub fn hold_real_folder(folder: &Path) -> io::Result<File> {
    let hold = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(folder)?;
    if hold.metadata()?.is_dir() {
        return Ok(hold);
    }
    Err(io::Error::other(format!(
        "{} is a link or a file, not a folder",
        folder.display()
    )))
}

/// A folder elevated code keeps things in from one run to the next. It is used only if it
/// is a real folder, administrators own it, and only they can change it. Its parent must
/// exist.
///
/// Anyone can create a folder in `ProgramData`, so one may be there first, made by someone
/// who can go on changing it, and whoever made it may hold a handle that keeps its rights
/// after the permissions change. So one that fails the check is never adopted. An empty one
/// is replaced, which also fails while anyone holds it open, and any other means an
/// administrator has to look at it.
///
/// Without elevation there is no boundary to protect, as for [`ProtectedFolder`].
pub fn ensure_protected_folder(folder: &Path) -> io::Result<()> {
    match fs::create_dir(folder) {
        Ok(()) => return protect_new_folder(folder),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error),
    }
    let hold = hold_real_folder(folder)?;
    match elevation() {
        Some(true) => {}
        Some(false) => return Ok(()),
        None => return Err(unknown_elevation()),
    }
    let is_protected = read_with_folder_pinned(folder, &hold).is_ok();
    drop(hold);
    if is_protected {
        return Ok(());
    }
    fs::remove_dir(folder).map_err(|error| {
        io::Error::other(format!(
            "{} was not made by WinAppMigrator, and others can change what is in it. An \
             administrator should check it and remove it: {error}",
            folder.display()
        ))
    })?;
    fs::create_dir(folder)?;
    protect_new_folder(folder)
}

/// A folder this run just made, closed to everyone else. If that fails it is not left there to be
/// taken up by a later run, with whatever stopped this one still in it.
fn protect_new_folder(folder: &Path) -> io::Result<()> {
    let hold = hold_real_folder(folder)?;
    let protected = protect_staging_folder(folder);
    drop(hold);
    if protected.is_err() {
        let _ = fs::remove_dir_all(folder);
    }
    protected
}

/// Check a folder that an install trusts what is in, from [`ensure_protected_folder`]. Its own
/// permissions say nothing of what got in before they were closed, which is what stops the run that
/// made it, so a later run has to look as well. This takes a moment, so it is not asked of every folder
/// that function looks after: a log folder is written to, and nothing in it is believed.
pub fn refuse_what_others_put_in(folder: &Path) -> io::Result<()> {
    match elevation() {
        Some(true) => {}
        Some(false) => return Ok(()),
        None => return Err(unknown_elevation()),
    }
    if only_administrators_made_what_is_in(folder) {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{} has something in it that was not made by WinAppMigrator, and others can change it. An \
         administrator should check it and remove it",
        folder.display()
    )))
}

fn only_administrators_made_what_is_in(folder: &Path) -> bool {
    let text = folder.to_string_lossy();
    powershell_command(format!(
        "{POWERSHELL_IMPORT_SECURITY}{POWERSHELL_IMPORT_MANAGEMENT}{ONLY_ADMINISTRATORS_MADE_WHAT_IS_INSIDE_SCRIPT}"
    ))
    .env(ACL_PATH_VARIABLE, without_verbatim_prefix(&text))
    .output()
    .is_ok_and(|output| {
        output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "yes"
    })
}

/// Whether a folder made in `parent` is one that someone could go on controlling, however it is
/// closed afterward, as anywhere in an account's own profile: they can open it in the moment it
/// appears and keep the right to change its permissions. Without elevation there is no boundary to
/// keep. Not being able to tell is taken for yes.
pub fn others_could_keep_a_folder_made_in(parent: &Path) -> bool {
    match elevation() {
        Some(true) => {}
        Some(false) => return false,
        None => return true,
    }
    its_permissions_say_others_could_keep_a_folder(parent)
}

fn its_permissions_say_others_could_keep_a_folder(parent: &Path) -> bool {
    let text = parent.to_string_lossy();
    powershell_command(format!(
        "{POWERSHELL_IMPORT_SECURITY}{OTHERS_COULD_KEEP_A_NEW_FOLDER_SCRIPT}"
    ))
    .env(ACL_PATH_VARIABLE, without_verbatim_prefix(&text))
    .output()
    .map_or(true, |output| {
        !output.status.success() || String::from_utf8_lossy(&output.stdout).trim() != "no"
    })
}

/// Check a folder [`ensure_protected_folder`] looks after. A failure means what is in it cannot
/// be trusted.
pub fn confirm_protected_folder(folder: &Path) -> io::Result<()> {
    let hold = hold_real_folder(folder)?;
    let result = match elevation() {
        Some(true) => read_with_folder_pinned(folder, &hold),
        Some(false) => Ok(()),
        None => Err(unknown_elevation()),
    };
    drop(hold);
    result
}

/// Keep a held folder from being turned into a link for as long as the result is alive.
/// Turning a folder into a link needs it to be empty, and a file held open in it cannot be
/// moved out. While a folder is held, it cannot be renamed, and so with both, anything done
/// by a path through it stays there, even if whoever can write to it can make that wait.
///
/// This is by a path too, so the folder may have been turned into a link just before this,
/// putting the file somewhere else. So it has to be seen where it was meant to be, with the
/// folder still real on both sides of looking. Its name cannot be guessed, so that what is seen
/// there is not something put there ahead of time, though whoever has the file land in their own
/// folder can read the name and race to copy it. None of these steps can be made to wait: a
/// test of that is described in `SECURITY.md`. The file goes when closed, however this process
/// ends, unless the power does first, and is hidden so it does not flicker on a Desktop.
pub(crate) fn pin_held_folder(folder: &Path, hold: &File) -> io::Result<File> {
    let random = RandomState::new().build_hasher().finish();
    let path = folder.join(format!(
        "{PIN_FILE_PREFIX}{random:0width$x}",
        width = PIN_FILE_RANDOM_DIGITS
    ));
    let pin = OpenOptions::new()
        .write(true)
        .create_new(true)
        .access_mode(GENERIC_WRITE | DELETE)
        .share_mode(0)
        .attributes(FILE_ATTRIBUTE_HIDDEN)
        .custom_flags(FILE_FLAG_DELETE_ON_CLOSE)
        .open(&path)?;
    let in_place = hold.metadata()?.is_dir()
        && fs::symlink_metadata(&path).is_ok()
        && hold.metadata()?.is_dir();
    if in_place {
        return Ok(pin);
    }
    Err(io::Error::other(format!(
        "{} became a link while it was being used",
        folder.display()
    )))
}

/// Whether a file's name is what [`pin_held_folder`] calls one, and not a name a person gave a
/// file of their own.
pub(crate) fn is_pin_file_name(name: &str) -> bool {
    name.strip_prefix(PIN_FILE_PREFIX).is_some_and(|random| {
        random.len() == PIN_FILE_RANDOM_DIGITS
            && random.chars().all(|digit| digit.is_ascii_hexdigit())
    })
}

/// Check permissions by a path, knowing they are that folder's. What reads them follows links,
/// so someone with write access could otherwise turn the folder into a link to one of the right
/// kind for just that long, and back. The caller holds the folder.
fn read_with_folder_pinned(folder: &Path, hold: &File) -> io::Result<()> {
    let pin = pin_held_folder(folder, hold)?;
    let result = confirm_administrators_only(folder);
    if !hold.metadata()?.is_dir() {
        return Err(io::Error::other(format!(
            "{} became a link while it was being looked at",
            folder.display()
        )));
    }
    drop(pin);
    result
}

/// Let someone who is not elevated start a file in an administrators-only folder. They could
/// not otherwise open it to find out it needs elevation. It stays theirs to read and not to
/// change, and the folder stays closed to them. Without elevation there is nothing to add.
pub fn allow_users_to_start(file: &Path) -> io::Result<()> {
    if !is_elevated() {
        return Ok(());
    }
    let output = hidden_command("icacls")
        .arg(file)
        .args(["/grant", USERS_MAY_START_GRANT])
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "could not let users start {}: {}{}",
        file.display(),
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

fn unknown_elevation() -> io::Error {
    io::Error::other("could not tell whether this process is elevated")
}

/// Make a freshly created staging folder writable only by administrators, so a process
/// that is not elevated cannot change what elevated code is about to read from it.
///
/// Without elevation there is no boundary to protect, and this process could not read
/// the folder afterward, so development builds and tests leave it alone. Not knowing
/// is a failure: a process that was never asked to be fooled about this cannot tell.
fn protect_staging_folder(folder: &Path) -> io::Result<()> {
    match elevation() {
        Some(true) => {
            reset_to_what_it_inherits(folder)?;
            take_ownership_for_administrators(folder)?;
            restrict_to_administrators(folder)?;
            confirm_administrators_only(folder)?;
            refuse_anything_already_inside(folder)
        }
        Some(false) => Ok(()),
        None => Err(unknown_elevation()),
    }
}

/// A new folder can come with permissions that are not marked as inherited, when the folder
/// above holds them in an older form. They can name the signed-in user, and cutting off
/// inheritance leaves them in place. Resetting marks all of them as inherited.
fn reset_arguments(folder: &Path) -> Vec<OsString> {
    vec![folder.as_os_str().to_owned(), OsString::from("/reset")]
}

/// The owner of a folder can always change its permissions, so the owner has to be
/// administrators too. It normally is already, but policy can make it the signed-in user.
fn administrators_owner_arguments(folder: &Path) -> Vec<OsString> {
    vec![
        folder.as_os_str().to_owned(),
        OsString::from("/setowner"),
        OsString::from(ADMINISTRATORS_SID),
    ]
}

/// The arguments that replace a folder's permissions with administrators and SYSTEM.
fn administrators_only_arguments(folder: &Path) -> Vec<OsString> {
    let mut arguments = vec![
        folder.as_os_str().to_owned(),
        OsString::from("/inheritance:r"),
        OsString::from("/grant:r"),
    ];
    arguments.extend(ADMINISTRATORS_ONLY_GRANTS.map(OsString::from));
    arguments
}

fn run_icacls(folder: &Path, arguments: Vec<OsString>) -> io::Result<()> {
    let output = hidden_command("icacls").args(arguments).output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "could not restrict {} to administrators: {}{}",
        folder.display(),
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

fn reset_to_what_it_inherits(folder: &Path) -> io::Result<()> {
    run_icacls(folder, reset_arguments(folder))
}

fn take_ownership_for_administrators(folder: &Path) -> io::Result<()> {
    run_icacls(folder, administrators_owner_arguments(folder))
}

fn restrict_to_administrators(folder: &Path) -> io::Result<()> {
    run_icacls(folder, administrators_only_arguments(folder))
}

/// Ask Windows what the folder's permissions came out as. `icacls` reports success on a
/// volume that keeps none, such as FAT32 or exFAT, where the folder stays open to everyone,
/// so it is the result that counts and not the exit code.
fn confirm_administrators_only(folder: &Path) -> io::Result<()> {
    let output = powershell_command(format!("{POWERSHELL_IMPORT_SECURITY}{READ_SDDL_SCRIPT}"))
        .env(ACL_PATH_VARIABLE, folder)
        .output()?;
    let sddl = String::from_utf8_lossy(&output.stdout);
    if output.status.success() && sddl_is_administrators_only(sddl.trim()) {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{} could not be restricted to administrators, and its drive may not keep permissions: \
         {}{}",
        folder.display(),
        sddl.trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

/// True when a security descriptor says administrators own the folder, it inherits nothing,
/// and only administrators and SYSTEM are on it, with full access to it and what is inside.
fn sddl_is_administrators_only(sddl: &str) -> bool {
    let Some((owner_and_group, dacl)) = sddl.split_once(SDDL_DACL_MARKER) else {
        return false;
    };
    let owner = owner_and_group
        .split_once(SDDL_GROUP_MARKER)
        .map_or(owner_and_group, |(owner, _)| owner);
    let Some((flags, aces)) = dacl.split_once('(') else {
        return false;
    };
    let Some(aces) = aces.strip_suffix(')') else {
        return false;
    };
    let mut aces: Vec<&str> = aces.split(")(").collect();
    aces.sort_unstable();
    owner == SDDL_ADMINISTRATORS_OWNER
        && flags.contains(SDDL_DACL_PROTECTED_FLAG)
        && aces == SDDL_ADMINISTRATORS_ONLY_ACES
}

/// The folder was empty when it was created. Anything there now arrived in the moment
/// before the permissions changed, and its own permissions may still be open.
fn refuse_anything_already_inside(folder: &Path) -> io::Result<()> {
    if fs::read_dir(folder)?.next().is_some() {
        return Err(io::Error::other(format!(
            "another process wrote to {} before it could be protected",
            folder.display()
        )));
    }
    Ok(())
}

/// Whose a file or folder is, as a SID: whoever made it, unless someone with the rights to changed
/// that. `None` if that cannot be told.
pub fn owner_of(path: &Path) -> Option<String> {
    let text = path.to_string_lossy();
    let output = powershell_command(format!("{POWERSHELL_IMPORT_SECURITY}{READ_OWNER_SCRIPT}"))
        .env(ACL_PATH_VARIABLE, without_verbatim_prefix(&text))
        .output()
        .ok()?;
    let owner = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (output.status.success() && owner.starts_with("S-1-")).then_some(owner)
}

/// Whether only someone with administrator rights could have written this file as it is now: it is
/// theirs, and no one else may change it. Editing a file keeps its owner, and a folder under `C:\\`
/// lets anyone edit one by default. Without elevation there is no one to be fooled, since this reads
/// only what its own account can.
pub fn only_administrators_can_change(path: &Path) -> bool {
    if !is_elevated() {
        return true;
    }
    let text = path.to_string_lossy();
    powershell_command(format!(
        "{POWERSHELL_IMPORT_SECURITY}{ONLY_ADMINISTRATORS_CAN_CHANGE_SCRIPT}"
    ))
    .env(ACL_PATH_VARIABLE, without_verbatim_prefix(&text))
    .output()
    .is_ok_and(|output| {
        output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "yes"
    })
}

pub fn target_requires_admin(target_root: &Path) -> bool {
    path_under_any_env(
        target_root,
        &["ProgramFiles", "ProgramFiles(x86)", "SystemRoot"],
    )
}

/// True when writing to `destination` modifies a machine-wide, protected
/// location (ProgramData or the same protected roots as `target_requires_admin`)
/// and therefore needs elevation. Used to gate real-location user-data restore.
pub fn destination_requires_admin(destination: &Path) -> bool {
    path_under_any_env(
        destination,
        &[
            "ProgramFiles",
            "ProgramFiles(x86)",
            "ProgramData",
            "SystemRoot",
        ],
    )
}

fn path_under_any_env(path: &Path, env_vars: &[&str]) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    env_vars.iter().any(|env_var| {
        machine_variable(env_var)
            .map(|value| lower.starts_with(&value.to_ascii_lowercase()))
            .unwrap_or(false)
    })
}

/// Where what setup programs unpacked is kept, which an install sweeps as it does its own.
pub fn tool_temp_root_in(state_root: &Path) -> PathBuf {
    state_root.join(TOOL_TEMP_FOLDER)
}

/// Where a setup program this starts elevated keeps what it unpacks. Some load what they have just
/// unpacked: DISM its host and providers, an older setup engine its own parts. In a folder another
/// program of the same account can reach, that program can put its own DLL beside them, so each gets
/// a folder only administrators can write to, which goes when it has finished. Without elevation there
/// is nothing to protect, and `TEMP` is left as it was given.
pub(crate) fn protected_temp_for_a_tool() -> io::Result<Option<ProtectedFolder>> {
    if !is_elevated() {
        return Ok(None);
    }
    let root = machine_state_root()
        .ok_or_else(|| io::Error::other("Windows does not say where ProgramData is"))?;
    ensure_protected_folder(&root)?;
    let temp = tool_temp_root_in(&root);
    ensure_protected_folder(&temp)?;
    ProtectedFolder::new_in(&temp, TOOL_TEMP_PREFIX).map(Some)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn asking_about_elevation_twice_spawns_nothing_the_second_time() {
        let first = is_elevated();
        let started = std::time::Instant::now();
        assert_eq!(is_elevated(), first);
        assert!(started.elapsed() < std::time::Duration::from_millis(100));
    }

    /// A place that cannot be read says nothing, which is not the same as saying nobody could.
    #[test]
    fn a_folder_that_cannot_be_asked_counts_as_one_others_could_keep() {
        assert!(its_permissions_say_others_could_keep_a_folder(Path::new(
            r"C:\Definitely\Not\There"
        )));
    }

    /// As a program with no rights at all can prepare one: full control for whatever is made in it,
    /// and its permissions closed to whoever would look. As this test is whoever is looking, it closes
    /// them to itself, whether or not it is running as an administrator.
    #[test]
    fn a_folder_closed_to_reading_says_nothing_of_who_could_keep_what_is_made_in_it()
    -> std::io::Result<()> {
        let me =
            crate::path_utils::current_user_sid().ok_or_else(|| io::Error::other("who is this"))?;
        let work = tempfile::tempdir()?;
        let prepared = work.path().join("prepared");
        fs::create_dir(&prepared)?;
        let closed = hidden_command("icacls")
            .arg(&prepared)
            .args([
                "/grant",
                "*S-1-3-4:(S)",
                "/deny",
                &format!("*{me}:(RC)"),
                "/deny",
                "*S-1-5-32-544:(RC)",
            ])
            .output()?;
        assert!(closed.status.success(), "{closed:?}");
        assert!(its_permissions_say_others_could_keep_a_folder(&prepared));
        Ok(())
    }

    #[test]
    fn this_shell_can_tell_whether_it_is_elevated() {
        assert!(elevation().is_some());
    }

    #[test]
    fn elevation_is_read_from_the_mandatory_label() {
        let medium = r#""Everyone","Well-known group","S-1-1-0","Mandatory group"
"Mandatory Label\Medium Mandatory Level","Label","S-1-16-8192","""#;
        let high = r#""BUILTIN\Administrators","Alias","S-1-5-32-544","Mandatory group"
"Mandatory Label\High Mandatory Level","Label","S-1-16-12288","""#;
        let system =
            r#""Etiqueta obligatoria\Nivel obligatorio del sistema","Etiqueta","S-1-16-16384","""#;
        assert!(!groups_show_elevation(medium));
        assert!(groups_show_elevation(high));
        assert!(groups_show_elevation(system));
    }

    #[test]
    fn the_reset_arguments_name_the_folder() {
        assert_eq!(
            reset_arguments(Path::new(r"D:\Stage Here")),
            [r"D:\Stage Here", "/reset"].map(OsString::from)
        );
    }

    #[test]
    fn the_owner_arguments_name_administrators() {
        assert_eq!(
            administrators_owner_arguments(Path::new(r"D:\Stage Here")),
            [r"D:\Stage Here", "/setowner", "*S-1-5-32-544"].map(OsString::from)
        );
    }

    #[test]
    fn the_acl_arguments_name_only_administrators_and_system() {
        assert_eq!(
            administrators_only_arguments(Path::new(r"D:\Stage Here")),
            [
                r"D:\Stage Here",
                "/inheritance:r",
                "/grant:r",
                "*S-1-5-32-544:(OI)(CI)F",
                "*S-1-5-18:(OI)(CI)F",
            ]
            .map(OsString::from)
        );
    }

    #[test]
    fn only_what_was_asked_for_counts_as_protected() {
        const GROUP: &str = "G:S-1-5-21-2150206276-3426440022-602146264-513";
        for good in [
            format!("O:BA{GROUP}D:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)"),
            format!("O:BA{GROUP}D:P(A;OICI;FA;;;BA)(A;OICI;FA;;;SY)"),
            "O:BAG:DUD:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)".to_owned(),
            "O:BAD:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)".to_owned(),
        ] {
            assert!(sddl_is_administrators_only(&good), "{good}");
        }
        for bad in [
            // What a FAT32 folder reports, whatever was asked of it.
            "O:WDG:WD".to_owned(),
            // What cutting off inheritance leaves when the folder above holds its
            // permissions in an older form.
            format!("O:BA{GROUP}D:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)(A;OICI;FA;;;LA)"),
            // The signed-in user still owns it, and so can change it.
            format!("O:S-1-5-21-1-2-3-1001{GROUP}D:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)"),
            // Still inheriting from its parent.
            format!("O:BA{GROUP}D:AI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)"),
            // Someone else on it, or only half of it.
            format!("O:BA{GROUP}D:PAI(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)(A;OICI;FA;;;WD)"),
            format!("O:BA{GROUP}D:PAI(A;OICI;FA;;;BU)(A;OICI;FA;;;BA)"),
            format!("O:BA{GROUP}D:PAI(A;OICI;FA;;;BA)"),
            // Not what was granted.
            format!("O:BA{GROUP}D:PAI(A;;FA;;;SY)(A;;FA;;;BA)"),
            format!("O:BA{GROUP}D:PAI(D;OICI;FA;;;SY)(A;OICI;FA;;;BA)"),
            // No permissions at all means open to everyone.
            format!("O:BA{GROUP}D:NO_ACCESS_CONTROL"),
            format!("O:BA{GROUP}D:P"),
            format!("O:BA{GROUP}"),
            String::new(),
        ] {
            assert!(!sddl_is_administrators_only(&bad), "{bad}");
        }
    }

    /// Runs the real command on a temp folder. A normal shell is locked out by it, which
    /// shows it worked. An elevated shell is not, and takes the other branch.
    #[test]
    fn the_acl_locks_out_a_process_that_is_not_elevated() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        restrict_to_administrators(folder.path())?;
        let listed = fs::read_dir(folder.path()).map(|_| ());
        // The owner can always change permissions, which lets the folder be cleaned up.
        let reset = hidden_command("icacls")
            .arg(folder.path())
            .arg("/reset")
            .output()?;
        assert!(reset.status.success());
        if is_elevated() {
            assert!(listed.is_ok());
        } else {
            let denied = listed.err().map(|error| error.kind());
            assert_eq!(denied, Some(io::ErrorKind::PermissionDenied));
        }
        Ok(())
    }

    #[test]
    fn a_folder_someone_got_into_first_is_refused() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        assert!(refuse_anything_already_inside(folder.path()).is_ok());
        fs::write(folder.path().join("planted.exe"), b"x")?;
        assert!(refuse_anything_already_inside(folder.path()).is_err());
        Ok(())
    }

    #[test]
    fn a_protected_folder_is_usable_and_gone_when_dropped() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = ProtectedFolder::new_in(parent.path(), "stage-")?;
        let path = folder.path().to_path_buf();
        assert_eq!(path.parent(), Some(parent.path()));
        fs::write(path.join("manifest.json"), b"{}")?;
        assert!(path.join("manifest.json").is_file());
        drop(folder);
        assert!(!path.exists());
        Ok(())
    }

    /// Run for real in a normal shell, against the rename that an access list does not stop.
    #[test]
    fn a_held_folder_and_the_folders_above_it_cannot_be_renamed() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let above = parent.path().join("above");
        let folder = above.join("stage");
        fs::create_dir_all(&folder)?;

        let hold = hold_folder(&folder)?;
        assert!(fs::rename(&folder, above.join("moved")).is_err());
        assert!(fs::rename(&above, parent.path().join("moved")).is_err());
        fs::write(folder.join("inside.txt"), b"still usable")?;
        fs::remove_file(folder.join("inside.txt"))?;

        drop(hold);
        fs::rename(&above, parent.path().join("moved"))?;
        Ok(())
    }

    #[test]
    fn holding_a_real_folder_refuses_a_link_and_a_file() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let real = parent.path().join("real");
        fs::create_dir(&real)?;
        let link = parent.path().join("link");
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&real)
            .output()?;
        assert!(made.status.success());
        let file = parent.path().join("file.txt");
        fs::write(&file, b"x")?;

        assert!(hold_real_folder(&real).is_ok());
        assert!(hold_real_folder(&link).is_err());
        assert!(hold_real_folder(&file).is_err());
        Ok(())
    }

    #[test]
    fn a_kept_folder_stays_after_it_is_dropped() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = ProtectedFolder::new_in(parent.path(), "kept-")?;
        let (path, hold) = folder.keep();
        drop(hold);
        assert!(path.is_dir());
        Ok(())
    }

    #[test]
    fn a_folder_that_lasts_is_made_once_and_then_used_as_it_is() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = parent.path().join("state");
        ensure_protected_folder(&folder)?;
        fs::write(folder.join("kept.txt"), b"x")?;
        ensure_protected_folder(&folder)?;
        confirm_protected_folder(&folder)?;
        assert!(folder.join("kept.txt").is_file());
        Ok(())
    }

    #[test]
    fn pinning_a_folder_leaves_nothing_in_it() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        let hold = hold_real_folder(folder.path())?;
        let pin = pin_held_folder(folder.path(), &hold)?;
        assert_eq!(fs::read_dir(folder.path())?.count(), 1);
        drop(pin);
        // Whether or not this process is elevated, in which case a temp folder also fails.
        let _ = read_with_folder_pinned(folder.path(), &hold);
        assert_eq!(fs::read_dir(folder.path())?.count(), 0);
        Ok(())
    }

    /// Turn a folder that already exists into a link, the way anyone who can write to it can,
    /// by asking the file system directly, and return the Windows error, `0` if it worked.
    pub(crate) fn try_to_turn_into_a_link(folder: &Path, target: &Path) -> io::Result<String> {
        let script = format!(
            r#"Import-Module Microsoft.PowerShell.Utility; Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;
public static class WamLink {{
  [DllImport("kernel32", CharSet = CharSet.Unicode, SetLastError = true)]
  static extern SafeFileHandle CreateFile(string name, uint access, uint share, IntPtr security,
    uint disposition, uint flags, IntPtr template);
  [DllImport("kernel32", SetLastError = true)]
  static extern bool DeviceIoControl(SafeFileHandle device, uint code, byte[] input, int inputSize,
    IntPtr output, int outputSize, out int returned, IntPtr overlapped);
  public static int Make(string folder, string target) {{
    using (var handle = CreateFile(folder, 0x40000000, 7, IntPtr.Zero, 3, 0x02200000, IntPtr.Zero)) {{
      if (handle.IsInvalid) {{ return Marshal.GetLastWin32Error(); }}
      var name = System.Text.Encoding.Unicode.GetBytes(@"\??\" + target);
      var data = new byte[name.Length + 20];
      BitConverter.GetBytes(0xA0000003u).CopyTo(data, 0);
      BitConverter.GetBytes((ushort)(name.Length + 12)).CopyTo(data, 4);
      BitConverter.GetBytes((ushort)name.Length).CopyTo(data, 10);
      BitConverter.GetBytes((ushort)(name.Length + 2)).CopyTo(data, 12);
      name.CopyTo(data, 16);
      int returned;
      return DeviceIoControl(handle, 0x000900A4, data, data.Length, IntPtr.Zero, 0, out returned, IntPtr.Zero)
        ? 0 : Marshal.GetLastWin32Error();
    }}
  }}
}}
'@; [WamLink]::Make('{}', '{}')"#,
            folder.display(),
            target.display()
        );
        let output = powershell_command(&script).output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    /// All of what pinning rests on, from the file system itself. Holding a folder does not keep
    /// it from being turned into a link, and a file held open in it does, and when it happens
    /// anyway, it shows on the hold, and pinning notices.
    #[test]
    fn a_pinned_folder_cannot_be_turned_into_a_link_and_one_only_held_can() -> io::Result<()> {
        const DIRECTORY_NOT_EMPTY: &str = "145";
        const WORKED: &str = "0";
        let work = tempfile::tempdir()?;
        let parent = fs::canonicalize(work.path())?;
        let parent = Path::new(crate::path_utils::without_verbatim_prefix(
            parent.to_str().ok_or_else(|| io::Error::other("a name"))?,
        ));
        let folder = parent.join("folder");
        let elsewhere = parent.join("elsewhere");
        fs::create_dir(&folder)?;
        fs::create_dir(&elsewhere)?;
        let hold = hold_real_folder(&folder)?;

        let pin = pin_held_folder(&folder, &hold)?;
        assert_eq!(
            try_to_turn_into_a_link(&folder, &elsewhere)?,
            DIRECTORY_NOT_EMPTY
        );
        assert!(hold.metadata()?.is_dir());
        drop(pin);

        assert_eq!(try_to_turn_into_a_link(&folder, &elsewhere)?, WORKED);
        assert!(!hold.metadata()?.is_dir(), "it shows on the hold");
        assert!(hold_real_folder(&folder).is_err());
        assert!(
            pin_held_folder(&folder, &hold).is_err(),
            "pinning notices the file went somewhere else"
        );
        assert_eq!(fs::read_dir(&elsewhere)?.count(), 0, "and leaves nothing");
        drop(hold);
        // A link goes without touching what it points at.
        fs::remove_dir(&folder)
    }

    #[test]
    fn a_file_of_ones_own_is_not_taken_for_a_pin() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        let hold = hold_real_folder(folder.path())?;
        let pin = pin_held_folder(folder.path(), &hold)?;
        let name = fs::read_dir(folder.path())?
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .next()
            .ok_or_else(|| io::Error::other("no pin"))?;
        assert!(is_pin_file_name(&name), "{name}");
        drop(pin);
        for name in [
            ".wam-pin-notes",
            ".wam-pin-",
            "pin",
            ".wam-pin-0123456789abcdefg",
        ] {
            assert!(!is_pin_file_name(name), "{name}");
        }
        Ok(())
    }

    /// The rest of what pinning is for, against what can be tried from a normal shell.
    #[test]
    fn a_pinned_folder_cannot_be_emptied_or_moved() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = parent.path().join("folder");
        fs::create_dir(&folder)?;
        let hold = hold_real_folder(&folder)?;
        let pin = pin_held_folder(&folder, &hold)?;

        let pin_path = fs::read_dir(&folder)?
            .flatten()
            .map(|entry| entry.path())
            .next()
            .ok_or_else(|| io::Error::other("no pin"))?;
        assert!(fs::rename(&pin_path, parent.path().join("moved")).is_err());
        assert!(fs::remove_file(&pin_path).is_err());
        assert!(fs::remove_dir(&folder).is_err());
        assert!(fs::rename(&folder, parent.path().join("renamed")).is_err());

        drop(pin);
        drop(hold);
        fs::remove_dir(&folder)
    }

    #[test]
    fn a_link_is_not_used_as_a_folder_that_lasts() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let elsewhere = parent.path().join("elsewhere");
        fs::create_dir(&elsewhere)?;
        let link = parent.path().join("state");
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());

        assert!(ensure_protected_folder(&link).is_err());
        assert!(confirm_protected_folder(&link).is_err());
        Ok(())
    }

    #[test]
    fn a_protected_folder_cannot_be_renamed_out_from_under_its_user() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = ProtectedFolder::new_in(parent.path(), "stage-")?;
        assert!(fs::rename(folder.path(), parent.path().join("moved")).is_err());
        Ok(())
    }
}
