use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use tempfile::TempDir;

use crate::path_utils::{POWERSHELL_IMPORT_SECURITY, hidden_command};

/// SIDs rather than names, because the names are translated on other Windows languages.
const ADMINISTRATORS_SID: &str = "*S-1-5-32-544";
const ADMINISTRATORS_ONLY_GRANTS: [&str; 2] = ["*S-1-5-32-544:(OI)(CI)F", "*S-1-5-18:(OI)(CI)F"];
/// The mandatory labels of a process running with full rights: High, and System.
const ELEVATED_LABELS: [&str; 2] = ["S-1-16-12288", "S-1-16-16384"];
const POWERSHELL_EXE: &str = "powershell.exe";
/// The path travels in this variable, so that no quoting can go wrong.
const ACL_PATH_VARIABLE: &str = "WAM_ACL_PATH";
const READ_SDDL_SCRIPT: &str = "(Get-Acl -LiteralPath $env:WAM_ACL_PATH).Sddl";
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
/// Put briefly in a folder while its permissions are read. See [`read_with_folder_pinned`].
const PIN_FILE_PREFIX: &str = ".wam-pin-";
const GENERIC_WRITE: u32 = 0x4000_0000;
const DELETE: u32 = 0x0001_0000;
const FILE_FLAG_DELETE_ON_CLOSE: u32 = 0x0400_0000;

/// Whether this process runs with full administrator rights.
pub fn is_elevated() -> bool {
    elevation() == Some(true)
}

/// Asked once, since the answer cannot change during a run. `None` means it could not be
/// found out, which is not the same as no: asking runs a program, and that can be made to
/// fail. Asking `whoami` rather than PowerShell also takes a fraction of the time.
fn elevation() -> Option<bool> {
    static ELEVATED: OnceLock<Option<bool>> = OnceLock::new();
    *ELEVATED.get_or_init(query_elevation)
}

fn query_elevation() -> Option<bool> {
    let output = hidden_command("whoami")
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
        let hold = hold_folder(temp.path())?;
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
/// hold does not stop, so a path that matters has to be checked again after the last thing
/// that can be made to wait.
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
    let is_protected = read_with_folder_pinned(folder).is_ok();
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

fn protect_new_folder(folder: &Path) -> io::Result<()> {
    let _hold = hold_real_folder(folder)?;
    protect_staging_folder(folder)
}

/// Check a folder [`ensure_protected_folder`] looks after. A failure means what is in it cannot
/// be trusted.
pub fn confirm_protected_folder(folder: &Path) -> io::Result<()> {
    let hold = hold_real_folder(folder)?;
    let result = match elevation() {
        Some(true) => read_with_folder_pinned(folder),
        Some(false) => Ok(()),
        None => Err(unknown_elevation()),
    };
    drop(hold);
    result
}

/// Check permissions by a path, knowing they are that folder's. What reads them follows links,
/// so someone with write access could turn the folder into a link to one of the right kind for
/// just that long, and back. Turning a folder into a link needs it to be empty, and a file held
/// open in it cannot be moved out, so with one there it stays what it is. The caller holds the
/// folder, so it cannot be renamed either. The file goes when closed, however this process ends.
fn read_with_folder_pinned(folder: &Path) -> io::Result<()> {
    let pin = OpenOptions::new()
        .write(true)
        .create_new(true)
        .access_mode(GENERIC_WRITE | DELETE)
        .share_mode(0)
        .custom_flags(FILE_FLAG_DELETE_ON_CLOSE)
        .open(folder.join(format!("{PIN_FILE_PREFIX}{}", std::process::id())))?;
    let result = confirm_administrators_only(folder);
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
    let output = hidden_command(POWERSHELL_EXE)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("{POWERSHELL_IMPORT_SECURITY}{READ_SDDL_SCRIPT}"),
        ])
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
        std::env::var_os(env_var)
            .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asking_about_elevation_twice_spawns_nothing_the_second_time() {
        let first = is_elevated();
        let started = std::time::Instant::now();
        assert_eq!(is_elevated(), first);
        assert!(started.elapsed() < std::time::Duration::from_millis(100));
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
        // Whether or not this process is elevated, in which case a temp folder also fails.
        let _ = read_with_folder_pinned(folder.path());
        assert_eq!(fs::read_dir(folder.path())?.count(), 0);
        Ok(())
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
