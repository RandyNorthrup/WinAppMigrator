use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf, Prefix};
use std::process::Command;
use std::sync::OnceLock;

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

use crate::privilege::is_elevated;

const EXE_EXTENSION: &str = "exe";
const WINDOWS_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
const SYSTEM_ROOT_VALUE: &str = "SystemRoot";
const BUILD_NUMBER_VALUE: &str = "CurrentBuildNumber";
const PROFILE_LIST_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList";
const SYSTEM_DRIVE_MARKER: &str = "%SystemDrive%";
const DEFAULT_SYSTEM_ROOT: &str = r"C:\Windows";
const POWERSHELL_EXE: &str = "powershell.exe";
const POWERSHELL_HOME: &str = r"WindowsPowerShell\v1.0";
/// The start of a person's SID: a local or domain account, and an Entra ID one.
const PERSON_SID_PREFIXES: [&str; 2] = ["S-1-5-21-", "S-1-12-1-"];
const SHELL_FILE: &str = "explorer.exe";
/// Each `explorer.exe` in this session, with who it runs as, by its path and SID, which read the
/// same on every language of Windows. `-IncludeUserName` needs elevation, which is the only case
/// where the question comes up. Not through CIM, which took 20 seconds on Windows Server.
const SESSION_SHELL_SCRIPT: &str = "$ErrorActionPreference = 'Stop'; \
     $session = [Diagnostics.Process]::GetCurrentProcess().SessionId; \
     foreach ($process in @(Get-Process -Name explorer -IncludeUserName -ErrorAction Ignore)) { \
       if ($process.SessionId -eq $session -and $process.UserName) { \
         '{0}|{1}' -f $process.Path, ([Security.Principal.NTAccount]$process.UserName).\
Translate([Security.Principal.SecurityIdentifier]).Value } }; \
     exit 0";
/// What PowerShell and the modules it uses read from the environment, none of which can
/// load code. Paths to the Windows folder are set separately, from the registry.
const POWERSHELL_ENVIRONMENT: &[&str] = &[
    "ALLUSERSPROFILE",
    "APPDATA",
    "CommonProgramFiles",
    "CommonProgramFiles(x86)",
    "CommonProgramW6432",
    "COMPUTERNAME",
    "HOMEDRIVE",
    "HOMEPATH",
    "LOCALAPPDATA",
    "NUMBER_OF_PROCESSORS",
    "OS",
    "PATHEXT",
    "PROCESSOR_ARCHITECTURE",
    "ProgramData",
    "ProgramFiles",
    "ProgramFiles(x86)",
    "ProgramW6432",
    "PUBLIC",
    "SystemDrive",
    "TEMP",
    "TMP",
    "USERDOMAIN",
    "USERNAME",
    "USERPROFILE",
];
/// Put at the start of a PowerShell script whose output is read back. It otherwise writes
/// the console's code page, which turns `größe` into bytes that are not UTF-8. This form
/// writes no byte order mark, and uses no command, so it costs no search for one.
pub const POWERSHELL_UTF8_OUTPUT: &str =
    "[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false); ";
/// Put at the start of a PowerShell script, for the commands it uses. PowerShell finds a
/// command it has not loaded by looking through every module, and with no module cache that
/// means reading all of them. A call this short never saves a cache, and the fixed
/// environment leaves out the location one might be in, so on Windows Server each call would
/// take more than 20 seconds. Naming the module goes straight to it.
pub const POWERSHELL_IMPORT_SECURITY: &str = "Import-Module Microsoft.PowerShell.Security; ";
pub const POWERSHELL_IMPORT_UTILITY: &str = "Import-Module Microsoft.PowerShell.Utility; ";
pub const POWERSHELL_IMPORT_MANAGEMENT: &str = "Import-Module Microsoft.PowerShell.Management; ";
pub const POWERSHELL_IMPORT_TASKS: &str = "Import-Module ScheduledTasks; ";
pub const POWERSHELL_IMPORT_APPX: &str = "Import-Module Appx; ";

/// Build a `Command` that does not pop up a console window for the child
/// process. Every external tool WinAppMigrator runs (sc.exe, PowerShell, winget,
/// bundled installers, cmd) must use this so building or installing never flashes
/// terminal windows over the GUI/app.
pub fn hidden_command(program: impl AsRef<OsStr>) -> Command {
    let tool = resolve_tool(program.as_ref());
    let is_powershell = tool
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case(POWERSHELL_EXE));
    let mut command = Command::new(tool);
    if is_powershell {
        set_powershell_environment(&mut command);
    }
    // Never where this process happens to be. A folder dialog moves that to whatever was chosen,
    // such as a drive from another PC, and a program looks there for what it loads.
    command.current_dir(system_root());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW: run the child without allocating a console.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

/// Give PowerShell a fixed environment instead of inheriting one. The user's environment
/// is theirs to change, and an elevated process gets it. PowerShell is a .NET program, so
/// `COMPLUS_InstallRoot` can load a runtime from any folder, `COR_PROFILER` can load a DLL,
/// and `PSModulePath` can replace the commands it runs, signature checks included. Those
/// are the ones known about, so what is passed is a list of what it needs.
fn set_powershell_environment(command: &mut Command) {
    command.env_clear();
    for name in POWERSHELL_ENVIRONMENT {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    let system32 = system_root().join("System32");
    let powershell_home = system32.join(POWERSHELL_HOME);
    command
        .env("SystemRoot", system_root())
        .env("windir", system_root())
        .env("ComSpec", system32.join("cmd.exe"))
        .env("PSModulePath", powershell_home.join("Modules"));
    if let Ok(path) = std::env::join_paths([system32, powershell_home]) {
        command.env("Path", path);
    }
}

/// The Windows directory, read from the admin-only registry. The `SystemRoot`
/// and `windir` environment variables are user-controlled, so an elevated process
/// must not trust them.
pub fn system_root() -> &'static Path {
    static SYSTEM_ROOT: OnceLock<PathBuf> = OnceLock::new();
    SYSTEM_ROOT.get_or_init(|| {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(WINDOWS_VERSION_KEY)
            .and_then(|key| key.get_value::<String, _>(SYSTEM_ROOT_VALUE))
            .map_or_else(|_| PathBuf::from(DEFAULT_SYSTEM_ROOT), PathBuf::from)
    })
}

/// A path as Windows' own functions hand it back, without the `\\?\` they put on it.
pub fn without_verbatim_prefix(path: &str) -> &str {
    path.strip_prefix(r"\\?\").unwrap_or(path)
}

/// Which build of Windows this is, from the admin-only registry. Zero if that cannot be read,
/// which reads as older than anything.
pub fn windows_build() -> u32 {
    static BUILD: OnceLock<u32> = OnceLock::new();
    *BUILD.get_or_init(|| {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(WINDOWS_VERSION_KEY)
            .and_then(|key| key.get_value::<String, _>(BUILD_NUMBER_VALUE))
            .ok()
            .and_then(|build| build.trim().parse().ok())
            .unwrap_or_default()
    })
}

/// `ProgramData`, from the admin-only registry.
pub fn program_data_dir() -> Option<&'static Path> {
    static FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOLDER
        .get_or_init(|| profile_list_folder("ProgramData"))
        .as_deref()
}

/// The shared Public profile, from the admin-only registry.
pub fn public_profile_dir() -> Option<&'static Path> {
    static FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOLDER
        .get_or_init(|| profile_list_folder("Public"))
        .as_deref()
}

/// This account's profile folder, from the admin-only registry. Which account comes from
/// `whoami`, because a variable could name any other.
pub fn user_profile_dir() -> Option<&'static Path> {
    static FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOLDER.get_or_init(query_user_profile).as_deref()
}

/// The SID of the account running this process, from `whoami`, because a variable could
/// name any other.
pub fn current_user_sid() -> Option<&'static str> {
    static SID: OnceLock<Option<String>> = OnceLock::new();
    SID.get_or_init(|| {
        let output = hidden_command("whoami")
            .args(["/user", "/fo", "csv", "/nh"])
            .output()
            .ok()?;
        let line = String::from_utf8_lossy(&output.stdout);
        sid_from_whoami_line(&line).map(str::to_owned)
    })
    .as_deref()
}

fn query_user_profile() -> Option<PathBuf> {
    profile_of_sid(current_user_sid()?)
}

/// An account's profile folder, from the admin-only registry.
pub fn profile_of_sid(sid: &str) -> Option<PathBuf> {
    let recorded = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"{PROFILE_LIST_KEY}\{sid}"))
        .and_then(|key| key.get_value::<String, _>("ProfileImagePath"))
        .ok()?;
    Some(expand_system_drive(&recorded, system_root()))
}

/// True for a person's account, local or from Entra ID, and not one Windows runs things as.
pub fn is_person_sid(sid: &str) -> bool {
    PERSON_SID_PREFIXES
        .iter()
        .any(|prefix| sid.starts_with(prefix))
}

/// An account on this PC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub sid: String,
    pub profile: PathBuf,
}

impl UserAccount {
    /// What to call it: the name of its profile folder, which is the same for every language.
    pub fn name(&self) -> String {
        file_name_string(&self.profile)
    }
}

/// Whose per-user folders and settings to use. An elevated process can be run by another
/// administrator, when the person at the PC typed that account's password at the UAC prompt, and
/// its `%APPDATA%` and `HKEY_CURRENT_USER` are then that administrator's and not theirs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedInUser {
    /// The account whose desktop this process runs on.
    Desktop(UserAccount),
    /// This session has no desktop, as under a scheduled task, SSH, or a build agent. So there is
    /// no one else it could be meant for, and it is the account running this.
    NoDesktop(UserAccount),
    /// It could not be told, and this says why. Nothing is to be guessed from that.
    Unknown(String),
}

impl SignedInUser {
    pub fn account(&self) -> Option<&UserAccount> {
        match self {
            Self::Desktop(account) | Self::NoDesktop(account) => Some(account),
            Self::Unknown(_) => None,
        }
    }
}

/// Who is signed in here, asked once, since that does not change during a run. Asking runs
/// PowerShell, so the first call takes about a second, and is not for a window's thread.
pub fn signed_in_user() -> &'static SignedInUser {
    SIGNED_IN_USER.get_or_init(query_signed_in_user)
}

/// [`signed_in_user`], if something has asked already, without waiting for it.
pub fn signed_in_user_if_known() -> Option<&'static SignedInUser> {
    SIGNED_IN_USER.get()
}

static SIGNED_IN_USER: OnceLock<SignedInUser> = OnceLock::new();

fn query_signed_in_user() -> SignedInUser {
    // Not elevated, this process is the one on the desktop, whoever else is on the PC.
    if !is_elevated() {
        return signed_in_user_from(
            current_user_sid().map(str::to_owned).into_iter().collect(),
            current_user_sid(),
            &profile_of_sid,
        );
    }
    let script =
        format!("{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_MANAGEMENT}{SESSION_SHELL_SCRIPT}");
    let output = hidden_command(POWERSHELL_EXE)
        .args(["-NoProfile", "-NonInteractive", "-Command", script.as_str()])
        .output();
    let shown = match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).into_owned()
        }
        Ok(output) => {
            return SignedInUser::Unknown(format!(
                "Windows would not say whose desktop this is: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Err(error) => {
            return SignedInUser::Unknown(format!(
                "Windows could not be asked whose desktop this is: {error}"
            ));
        }
    };
    signed_in_user_from(
        desktop_owners(&shown, system_root()),
        current_user_sid(),
        &profile_of_sid,
    )
}

/// Who owns the desktop of this session, going by what asking Windows printed: a line of the
/// program's path and its owner for each `explorer.exe` in it. One from anywhere else does not
/// count, since a program of that name can be started by whoever is signed in or by anyone else
/// with a password. One whose path could not be read is not ruled out by that.
fn desktop_owners(shown: &str, windows_dir: &Path) -> Vec<String> {
    let shell = windows_dir.join(SHELL_FILE);
    let mut owners: Vec<String> = Vec::new();
    for line in shown.lines() {
        let Some((path, sid)) = line.trim().split_once('|') else {
            continue;
        };
        if (path.is_empty() || paths_are_the_same(Path::new(path), &shell))
            && !owners.iter().any(|known| known.eq_ignore_ascii_case(sid))
        {
            owners.push(sid.to_owned());
        }
    }
    owners
}

fn paths_are_the_same(left: &Path, right: &Path) -> bool {
    path_is_within(left, right) && path_is_within(right, left)
}

fn signed_in_user_from(
    desktop_owners: Vec<String>,
    running_as: Option<&str>,
    profile_of: &dyn Fn(&str) -> Option<PathBuf>,
) -> SignedInUser {
    let account = |sid: &str| {
        profile_of(sid).map(|profile| UserAccount {
            sid: sid.to_owned(),
            profile,
        })
    };
    match desktop_owners.as_slice() {
        [owner] if is_person_sid(owner) => account(owner).map_or_else(
            || {
                SignedInUser::Unknown(format!(
                    "the account signed in here, {owner}, has no profile on record"
                ))
            },
            SignedInUser::Desktop,
        ),
        [owner] => SignedInUser::Unknown(format!(
            "the desktop here belongs to {owner}, which is not a person's account"
        )),
        [] => match running_as {
            Some(sid) if is_person_sid(sid) => account(sid).map_or_else(
                || {
                    SignedInUser::Unknown(format!(
                        "no one is signed in here, and the account running this, {sid}, has no \
                         profile on record"
                    ))
                },
                SignedInUser::NoDesktop,
            ),
            Some(sid) => SignedInUser::Unknown(format!(
                "no one is signed in here, and {sid}, which is running this, is not a person's \
                 account"
            )),
            None => SignedInUser::Unknown(
                "no one is signed in here, and this account has no SID".into(),
            ),
        },
        _ => SignedInUser::Unknown(format!(
            "more than one account has a desktop in this session: {}",
            desktop_owners.join(", ")
        )),
    }
}

/// `"domain\user","S-1-5-21-..."` is one line, and a name can hold a comma, so the SID is
/// the last field.
fn sid_from_whoami_line(line: &str) -> Option<&str> {
    let sid = line.trim().rsplit(',').next()?.trim_matches('"');
    sid.starts_with("S-1-").then_some(sid)
}

fn profile_list_folder(value: &str) -> Option<PathBuf> {
    let recorded = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(PROFILE_LIST_KEY)
        .and_then(|key| key.get_value::<String, _>(value))
        .ok()?;
    Some(expand_system_drive(&recorded, system_root()))
}

/// These values are written `%SystemDrive%\ProgramData`. The drive comes from the Windows
/// folder rather than from a variable the user could set.
fn expand_system_drive(recorded: &str, windows_dir: &Path) -> PathBuf {
    let Some(rest) = recorded
        .get(..SYSTEM_DRIVE_MARKER.len())
        .filter(|marker| marker.eq_ignore_ascii_case(SYSTEM_DRIVE_MARKER))
        .map(|_| &recorded[SYSTEM_DRIVE_MARKER.len()..])
    else {
        return PathBuf::from(recorded);
    };
    let drive = windows_dir.components().next();
    let mut path = drive.map_or_else(PathBuf::new, |drive| PathBuf::from(drive.as_os_str()));
    path.push(rest);
    path
}

/// Turn a bare tool name into an absolute path. Windows searches the running
/// executable's own folder first, so `sc.exe` planted beside an elevated installer
/// in Downloads would otherwise run instead of the real one. `PATH` is no better, since a
/// user can add to it. Only the Windows folder is used, and paths are kept as given.
fn resolve_tool(program: &OsStr) -> PathBuf {
    let path = Path::new(program);
    if path.is_absolute() || path.components().count() != 1 {
        return path.to_path_buf();
    }
    let file_name = if path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.with_extension(EXE_EXTENSION)
    };
    let system32 = system_root().join("System32");
    [
        system32.join(&file_name),
        system32.join(r"WindowsPowerShell\v1.0").join(&file_name),
        system_root().join(&file_name),
    ]
    .into_iter()
    .find(|candidate| candidate.is_file())
    // An absolute path that does not exist fails cleanly instead of falling back
    // to the search order this function exists to avoid.
    .unwrap_or_else(|| system32.join(&file_name))
}

/// Escape a value for a single-quoted PowerShell string. PowerShell also ends those
/// strings on the typographic quotes U+2018 to U+201B, so they are doubled as well;
/// otherwise a crafted file name breaks out of the string and runs as code.
pub fn escape_powershell_single(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if matches!(
            character,
            '\'' | '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}'
        ) {
            escaped.push(character);
        }
        escaped.push(character);
    }
    escaped
}

pub fn file_name_string(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .map_or_else(String::new, ToOwned::to_owned)
}

pub fn path_has_extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case(extension))
}

pub fn looks_like_executable(path: &Path) -> bool {
    path_has_extension(path, EXE_EXTENSION)
}

/// Replace every character that is not ASCII-alphanumeric or one of `-_.` with
/// `_`, producing a string safe to use as a file or directory name on Windows.
/// Does not trim; callers that want no leading/trailing `_` should
/// `trim_matches('_')` the result.
pub fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

pub fn split_command_executable_path(command: &str) -> Option<PathBuf> {
    let trimmed = trim_icon_suffix(command.trim());
    if trimmed.is_empty() {
        return None;
    }

    if let Some(rest) = trimmed.strip_prefix('"') {
        let closing_quote = rest.find('"')?;
        let candidate = &rest[..closing_quote];
        return Some(PathBuf::from(candidate));
    }

    let lower = trimmed.to_ascii_lowercase();
    if let Some(exe_end) = lower.find(".exe") {
        let end = exe_end + ".exe".len();
        return Some(PathBuf::from(trimmed[..end].trim()));
    }

    trimmed.split_whitespace().next().map(PathBuf::from)
}

pub fn trim_icon_suffix(value: &str) -> &str {
    let trimmed = value.trim();
    if let Some((path_part, icon_part)) = trimmed.rsplit_once(',')
        && icon_part.trim().parse::<i32>().is_ok()
    {
        return path_part.trim();
    }
    trimmed
}

pub fn normalize_name_for_match(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// A path as lowercase text with one kind of separator and none at the end, for comparing paths
/// that come from different records. `C:\` becomes `c:`. It goes by the path's parts, because
/// Windows reads `C:\.\Windows`, `C:\\Windows`, and `\\?\C:\Windows` all as `C:\Windows`, and
/// what decides where something may go cannot be told apart from it by how it is written.
fn comparable_path(path: &Path) -> String {
    let mut text = String::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => match prefix.kind() {
                Prefix::VerbatimDisk(drive) | Prefix::Disk(drive) => {
                    text.push(char::from(drive));
                    text.push(':');
                }
                Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => {
                    text.push_str(r"\\");
                    text.push_str(&server.to_string_lossy());
                    text.push('\\');
                    text.push_str(&share.to_string_lossy());
                }
                _ => text.push_str(&prefix.as_os_str().to_string_lossy()),
            },
            Component::RootDir => {
                if text.is_empty() {
                    text.push('\\');
                }
            }
            Component::CurDir => {}
            Component::ParentDir | Component::Normal(_) => {
                if !text.is_empty() && !text.ends_with('\\') {
                    text.push('\\');
                }
                text.push_str(&component.as_os_str().to_string_lossy());
            }
        }
    }
    text.to_ascii_lowercase()
}

/// How deep to go where nothing says how deep. Far past anything real, and a walk holds one folder
/// open for each level.
pub const DEEPEST_WALK: usize = 64;

/// What a walk came to.
#[derive(Debug)]
pub struct WalkedEntry {
    pub path: PathBuf,
    pub file_type: std::fs::FileType,
}

/// A folder a walk could not read.
#[derive(Debug)]
pub struct WalkFailure {
    pub path: PathBuf,
    pub error: std::io::Error,
}

impl std::fmt::Display for WalkFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.path.display(), self.error)
    }
}

/// A folder's contents, with `may_look` asked about each thing, by its path and whether its listing
/// calls it a folder, before anything is done with it. `walkdir` opens a folder before its filter
/// sees it, and on a drive only opening one can connect to a share: what a listing calls the folder
/// `x.` opens as the link `x` beside it. Links are not followed below the root.
pub struct AskingWalk<F> {
    open: Vec<std::fs::ReadDir>,
    max_depth: usize,
    may_look: F,
    failed: Option<WalkFailure>,
}

pub fn walk_asking<F: FnMut(&Path, bool) -> bool>(
    root: &Path,
    max_depth: usize,
    may_look: F,
) -> AskingWalk<F> {
    let mut walk = AskingWalk {
        open: Vec::new(),
        max_depth,
        may_look,
        failed: None,
    };
    if (walk.may_look)(root, true) {
        walk.enter(root);
    }
    walk
}

impl<F> AskingWalk<F> {
    fn enter(&mut self, folder: &Path) {
        match std::fs::read_dir(folder) {
            Ok(entries) => self.open.push(entries),
            Err(error) => {
                self.failed = Some(WalkFailure {
                    path: folder.to_path_buf(),
                    error,
                });
            }
        }
    }
}

impl<F: FnMut(&Path, bool) -> bool> Iterator for AskingWalk<F> {
    type Item = Result<WalkedEntry, WalkFailure>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(failure) = self.failed.take() {
                return Some(Err(failure));
            }
            let entry = match self.open.last_mut()?.next() {
                Some(Ok(entry)) => entry,
                Some(Err(error)) => {
                    return Some(Err(WalkFailure {
                        path: PathBuf::new(),
                        error,
                    }));
                }
                None => {
                    self.open.pop();
                    continue;
                }
            };
            let path = entry.path();
            // From the listing, which opens nothing. A link is neither a file nor a folder here.
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if !(self.may_look)(&path, file_type.is_dir()) {
                continue;
            }
            if file_type.is_dir() && self.open.len() < self.max_depth {
                self.enter(&path);
            }
            return Some(Ok(WalkedEntry { path, file_type }));
        }
    }
}

/// Whether a path climbs anywhere. Lexically, `C:\\app\\..\\..\\Users` is inside `C:\\app`.
pub fn has_parent_step(path: &Path) -> bool {
    path.components()
        .any(|component| component == Component::ParentDir)
}

/// Whether a path is on a disk of this PC, going by how it is written. Only looking at a share
/// connects to it, as whoever is looking, which here is an administrator. That is what a value in
/// any account's registry could ask for, and `\\?\GLOBALROOT` and `\??\UNC` are other ways to say it.
pub fn is_on_a_local_disk(path: &Path) -> bool {
    match path.components().next() {
        Some(Component::Prefix(prefix)) => {
            matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_))
        }
        Some(Component::RootDir) => false,
        _ => true,
    }
}

/// Where a location really is, as far as it exists yet: links followed and short names spelled
/// out for what is there, and the rest as it was given. For what must not be fooled by either.
pub fn resolved_location(path: &Path) -> PathBuf {
    let mut missing = Vec::new();
    let mut current = path;
    loop {
        if let Ok(real) = std::fs::canonicalize(current) {
            let mut location = real;
            location.extend(missing.iter().rev());
            return location;
        }
        match (current.file_name(), current.parent()) {
            (Some(name), Some(parent)) => {
                missing.push(name);
                current = parent;
            }
            _ => return path.to_path_buf(),
        }
    }
}

/// True when `path` is `root` or lies inside it, ignoring case. Unlike a text prefix,
/// `C:\Program Files\GitHub CLI` is not inside `C:\Program Files\Git`.
pub fn path_is_within(path: &Path, root: &Path) -> bool {
    let path = comparable_path(path);
    let root = comparable_path(root);
    !root.is_empty()
        && (path == root
            || path
                .strip_prefix(root.as_str())
                .is_some_and(|rest| rest.starts_with('\\')))
}

/// `path` relative to `root`, ignoring case: Windows paths from different records can
/// differ in case while naming the same folder. `None` when `path` is not inside `root`.
pub fn strip_prefix_ignore_case(path: &Path, root: &Path) -> Option<PathBuf> {
    let mut remaining = path.components();
    for expected in root.components() {
        let actual = remaining.next()?;
        if !actual
            .as_os_str()
            .eq_ignore_ascii_case(expected.as_os_str())
        {
            return None;
        }
    }
    Some(remaining.as_path().to_path_buf())
}

/// True when `path` names something strictly below `root` by its components alone. Unlike
/// [`path_is_within`], a `..` in `path` means no, because it may not stay inside.
pub fn path_is_strictly_inside(path: &Path, root: &Path) -> bool {
    strip_prefix_ignore_case(path, root).is_some_and(|relative| {
        !relative.as_os_str().is_empty()
            && relative
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
    })
}

/// True for `C:\` and other paths with nothing below the root.
pub fn is_drive_root(path: &Path) -> bool {
    !path
        .components()
        .any(|component| matches!(component, Component::Normal(_)))
}

/// True when text such as a registry value mentions `path` as a whole path. The next
/// character must end the path, so `C:\Program Files\Git` is not mentioned by
/// `C:\Program Files\GitHub CLI\gh.exe`, or by `C:\Program Files\Git Extensions`.
/// `followed_by_arguments` also accepts a space, which is right for an executable in a
/// command line and wrong for a directory.
pub fn text_mentions_path(text: &str, path: &Path, followed_by_arguments: bool) -> bool {
    let needle = comparable_path(path);
    if needle.is_empty() {
        return false;
    }
    // Both steps swap single bytes for single bytes, so `end` stays a char boundary.
    let lower = text.to_ascii_lowercase().replace('/', "\\");
    let mut from = 0;
    while let Some(found) = lower[from..].find(&needle) {
        let end = from + found + needle.len();
        match lower[end..].chars().next() {
            None | Some('\\' | '"' | '\'' | ';' | ',' | '\0' | '\r' | '\n') => return true,
            Some(' ' | '\t') if followed_by_arguments => return true,
            _ => from = end,
        }
    }
    false
}

/// True for an executable that exists to install, update, or support an application
/// and is not the application itself.
pub fn is_auxiliary_executable(path: &Path) -> bool {
    const AUXILIARY_MARKERS: &[&str] = &[
        "unins",
        "setup",
        "install",
        "update",
        "crashpad",
        "crashreporter",
        "crashhandler",
        "bugreport",
        "redist",
        "dotnet",
        "helper",
        "maintenancetool",
    ];
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    AUXILIARY_MARKERS.iter().any(|marker| stem.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_tool_names_resolve_to_absolute_system_paths() {
        for (tool, suffix) in [
            ("reg", r"System32\reg.exe"),
            ("sc.exe", r"System32\sc.exe"),
            ("powershell.exe", r"WindowsPowerShell\v1.0\powershell.exe"),
            ("explorer", "explorer.exe"),
        ] {
            let command = hidden_command(tool);
            let program = Path::new(command.get_program());
            assert!(program.is_absolute(), "{tool} resolved to {program:?}");
            assert!(program.ends_with(suffix), "{tool} resolved to {program:?}");
        }
    }

    #[test]
    fn unknown_tool_never_falls_back_to_the_search_order() {
        let command = hidden_command("winappmigrator-no-such-tool");
        assert!(Path::new(command.get_program()).is_absolute());
    }

    #[test]
    fn machine_wide_folders_come_from_the_registry_and_read_like_paths() {
        assert_eq!(
            expand_system_drive(r"%SystemDrive%\ProgramData", Path::new(r"D:\Windows")),
            PathBuf::from(r"D:\ProgramData")
        );
        assert_eq!(
            expand_system_drive(r"%systemdrive%\Users\Public", Path::new(r"C:\Windows")),
            PathBuf::from(r"C:\Users\Public")
        );
        assert_eq!(
            expand_system_drive(r"E:\Data", Path::new(r"C:\Windows")),
            PathBuf::from(r"E:\Data")
        );
        // On this machine the registry and the environment agree, unless someone set the
        // variable, and only an absolute path is ever answered.
        for folder in [program_data_dir(), public_profile_dir()] {
            assert!(folder.is_some_and(Path::is_absolute), "{folder:?}");
        }
    }

    #[test]
    fn the_accounts_sid_is_the_last_field_of_whoami() {
        assert_eq!(
            sid_from_whoami_line("\"pc\\bob, jr\",\"S-1-5-21-1-2-3-1001\"\r\n"),
            Some("S-1-5-21-1-2-3-1001")
        );
        assert_eq!(sid_from_whoami_line("ERROR"), None);
        assert_eq!(sid_from_whoami_line(""), None);
    }

    #[test]
    fn only_the_real_shell_says_whose_desktop_it_is() {
        let windows = Path::new(r"C:\Windows");
        assert_eq!(
            desktop_owners(
                "C:\\WINDOWS\\Explorer.EXE|S-1-5-21-1-1-1-1001\r\n\
                 C:\\Windows\\explorer.exe|s-1-5-21-1-1-1-1001\r\n\
                 C:\\Users\\Public\\explorer.exe|S-1-5-21-9-9-9-1002\r\n\
                 nonsense\r\n",
                windows
            ),
            vec!["S-1-5-21-1-1-1-1001".to_owned()]
        );
        // Windows would not say where this one is from, so it could be real.
        assert_eq!(
            desktop_owners(
                "C:\\Windows\\explorer.exe|S-1-5-21-1-1-1-1001\r\n|S-1-5-21-9-9-9-1003\r\n",
                windows
            ),
            vec![
                "S-1-5-21-1-1-1-1001".to_owned(),
                "S-1-5-21-9-9-9-1003".to_owned()
            ]
        );
        assert!(desktop_owners("", windows).is_empty());
    }

    #[test]
    fn who_is_signed_in_is_said_and_not_guessed() {
        let alice = "S-1-5-21-1-1-1-1001";
        let admin = "S-1-5-21-1-1-1-500";
        let profile = |sid: &str| {
            (sid != "S-1-5-21-0-0-0-0").then(|| PathBuf::from(format!(r"C:\Users\{sid}")))
        };
        let known = |sid: &str| UserAccount {
            sid: sid.to_owned(),
            profile: PathBuf::from(format!(r"C:\Users\{sid}")),
        };

        // Another administrator at the UAC prompt: it is still her desktop.
        assert_eq!(
            signed_in_user_from(vec![alice.to_owned()], Some(admin), &profile),
            SignedInUser::Desktop(known(alice))
        );
        // No desktop in this session: the account running this is all there is.
        assert_eq!(
            signed_in_user_from(Vec::new(), Some(admin), &profile),
            SignedInUser::NoDesktop(known(admin))
        );
        for unknown in [
            signed_in_user_from(
                vec![alice.to_owned(), admin.to_owned()],
                Some(admin),
                &profile,
            ),
            signed_in_user_from(Vec::new(), Some("S-1-5-18"), &profile),
            signed_in_user_from(Vec::new(), None, &profile),
            signed_in_user_from(vec!["S-1-5-18".to_owned()], Some(admin), &profile),
            signed_in_user_from(vec!["S-1-5-21-0-0-0-0".to_owned()], Some(admin), &profile),
        ] {
            assert!(matches!(unknown, SignedInUser::Unknown(_)), "{unknown:?}");
        }
    }

    #[test]
    fn a_person_is_told_from_what_windows_runs_things_as() {
        assert!(is_person_sid("S-1-5-21-1-2-3-1001"));
        assert!(is_person_sid("S-1-12-1-1-2-3-4"));
        for sid in [
            "S-1-5-18",
            "S-1-5-19",
            "S-1-5-20",
            "S-1-5-80-1-2",
            "nonsense",
        ] {
            assert!(!is_person_sid(sid), "{sid}");
        }
    }

    #[test]
    fn someone_is_signed_in_on_any_pc_these_tests_run_on() {
        assert!(
            signed_in_user()
                .account()
                .is_some_and(|account| account.profile.is_dir()),
            "{:?}",
            signed_in_user()
        );
    }

    #[test]
    fn this_accounts_profile_is_found_in_the_registry() {
        assert!(user_profile_dir().is_some_and(Path::is_dir));
    }

    #[test]
    fn a_tool_on_path_is_not_found() {
        // Cargo is on `PATH` for any test run, and has no business being run elevated.
        let command = hidden_command("cargo");
        assert!(
            Path::new(command.get_program()).starts_with(system_root()),
            "{:?}",
            command.get_program()
        );
    }

    #[test]
    fn powershell_is_given_its_module_folder_and_tools_from_the_windows_folder() {
        let command = hidden_command("powershell.exe");
        let envs: Vec<_> = command.get_envs().collect();
        for name in ["PSModulePath", "SystemRoot", "ComSpec", "Path"] {
            assert!(
                envs.iter().any(|(key, value)| {
                    *key == name
                        && value.is_some_and(|value| Path::new(value).starts_with(system_root()))
                }),
                "{name}"
            );
        }
        assert!(hidden_command("sc.exe").get_envs().next().is_none());
    }

    fn powershell_output(script: &str) -> std::io::Result<String> {
        let output = hidden_command("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .output()?;
        assert!(output.status.success());
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Run for real. Cargo sets these for every test, so if PowerShell saw the environment
    /// it inherited, they would be there.
    #[test]
    fn powershell_inherits_nothing_it_was_not_given() -> std::io::Result<()> {
        assert!(std::env::var_os("CARGO_MANIFEST_DIR").is_some());
        let names = powershell_output("(Get-ChildItem env:).Name")?;
        assert!(!names.contains("CARGO_MANIFEST_DIR"), "{names}");
        assert!(names.contains("SystemRoot"), "{names}");
        Ok(())
    }

    #[test]
    fn powershell_output_can_be_read_as_utf8() -> std::io::Result<()> {
        let script = format!("{POWERSHELL_UTF8_OUTPUT}Write-Output 'größe'");
        assert_eq!(powershell_output(&script)?.trim_end(), "größe");
        Ok(())
    }

    #[test]
    fn this_windows_has_a_build_number_and_this_account_a_sid() {
        assert!(windows_build() > 0);
        assert!(current_user_sid().is_some_and(|sid| sid.starts_with("S-1-")));
    }

    #[test]
    fn a_walk_asks_before_it_goes_in() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        std::fs::create_dir_all(work.path().join(r"a\deep\deeper"))?;
        std::fs::create_dir_all(work.path().join("not this"))?;
        std::fs::write(work.path().join(r"a\one.txt"), b"1")?;
        std::fs::write(work.path().join(r"a\deep\two.txt"), b"2")?;
        std::fs::write(work.path().join(r"a\deep\deeper\three.txt"), b"3")?;
        std::fs::write(work.path().join(r"not this\four.txt"), b"4")?;
        let names = |max_depth| -> Vec<String> {
            let mut names: Vec<String> = walk_asking(work.path(), max_depth, |path, _| {
                file_name_string(path) != "not this"
            })
            .flatten()
            .filter(|entry| entry.file_type.is_file())
            .map(|entry| file_name_string(&entry.path))
            .collect();
            names.sort();
            names
        };
        assert_eq!(names(DEEPEST_WALK), ["one.txt", "three.txt", "two.txt"]);
        assert_eq!(names(3), ["one.txt", "two.txt"], "that far and no further");

        let mut asked = 0;
        let found = walk_asking(work.path(), DEEPEST_WALK, |_, _| {
            asked += 1;
            false
        })
        .count();
        assert_eq!(
            (found, asked),
            (0, 1),
            "not even the top, if the answer is no"
        );

        let missing = work.path().join("not there");
        let failures: Vec<String> = walk_asking(&missing, DEEPEST_WALK, |_, _| true)
            .filter_map(|entry| entry.err().map(|failure| failure.to_string()))
            .collect();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("not there"));
        Ok(())
    }

    #[test]
    fn only_a_path_on_a_disk_of_this_pc_counts_as_one() {
        for local in [
            r"C:\Users\me",
            r"C:relative",
            r"\\?\C:\Users\me",
            r"relative\path",
        ] {
            assert!(is_on_a_local_disk(Path::new(local)), "{local}");
        }
        for elsewhere in [
            r"\\server\share\folder",
            r"\\localhost\C$\Users\bob",
            "//server/share/folder",
            r"\\?\UNC\server\share",
            r"\\.\C:\Users\bob",
            r"\\.\pipe\name",
            r"\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy1\Users\bob",
            r"\\?\Volume{00000000-0000-0000-0000-000000000000}\Users\bob",
            r"\??\UNC\server\share",
            r"\Device\Mup\server\share",
            r"\Windows",
        ] {
            assert!(!is_on_a_local_disk(Path::new(elsewhere)), "{elsewhere}");
        }
    }

    #[test]
    fn what_this_runs_does_not_start_in_whatever_folder_this_process_is_in() {
        let command = hidden_command("reg");
        assert_eq!(command.get_current_dir(), Some(system_root()));
    }

    #[test]
    fn paths_are_passed_through_unchanged() {
        let payload = r"C:\Packages\vc_redist.x64.exe";
        assert_eq!(hidden_command(payload).get_program(), payload);
    }

    #[test]
    fn powershell_escaping_doubles_every_quote_form() {
        assert_eq!(escape_powershell_single("plain"), "plain");
        assert_eq!(escape_powershell_single("a'b"), "a''b");
        assert_eq!(
            escape_powershell_single("x\u{2019};calc;\u{2019}"),
            "x\u{2019}\u{2019};calc;\u{2019}\u{2019}"
        );
        assert_eq!(
            escape_powershell_single("\u{2018}\u{201A}\u{201B}"),
            "\u{2018}\u{2018}\u{201A}\u{201A}\u{201B}\u{201B}"
        );
    }

    #[test]
    fn parses_quoted_executable_from_command() -> Result<(), Box<dyn std::error::Error>> {
        let parsed = split_command_executable_path(r#""C:\Program Files\App\App.exe" /quiet"#)
            .ok_or_else(|| std::io::Error::other("path should parse"))?;

        assert_eq!(parsed, PathBuf::from(r"C:\Program Files\App\App.exe"));
        Ok(())
    }

    #[test]
    fn a_path_is_within_a_folder_only_at_a_segment_boundary() {
        let git = Path::new(r"C:\Program Files\Git\");
        assert!(path_is_within(Path::new(r"C:\Program Files\Git"), git));
        assert!(path_is_within(
            Path::new(r"c:\program files\git\bin\git.exe"),
            git
        ));
        assert!(!path_is_within(
            Path::new(r"C:\Program Files\GitHub CLI\gh.exe"),
            git
        ));
        assert!(!path_is_within(Path::new(r"C:\Program Files"), git));
    }

    #[test]
    fn a_path_is_the_same_path_however_it_is_written() {
        let windows = Path::new(r"C:\Windows");
        for spelled in [
            r"C:\.\Windows\x",
            r"C:\\Windows\x",
            r"C:/Windows/x",
            r"c:\windows\.\x\",
            r"\\?\C:\Windows\x",
        ] {
            assert!(path_is_within(Path::new(spelled), windows), "{spelled}");
        }
        assert!(path_is_within(
            Path::new(r"C:\Windows"),
            Path::new(r"\\?\C:\Windows\")
        ));
        assert!(path_is_within(
            Path::new(r"C:\ProgramData\.\WinAppMigrator\installs"),
            Path::new(r"C:\ProgramData\WinAppMigrator")
        ));
        assert!(path_is_within(
            Path::new(r"//Server/Share/x"),
            Path::new(r"\\?\UNC\server\share")
        ));
        assert!(!path_is_within(Path::new(r"C:\Windows2\x"), windows));
        assert!(!path_is_within(Path::new(r"C:\x\Windows"), windows));
    }

    #[test]
    fn a_resolved_location_follows_a_link_and_keeps_what_is_not_there_yet() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let real = std::fs::canonicalize(work.path())?;
        std::fs::create_dir(work.path().join("elsewhere"))?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(work.path().join("link"))
            .arg(work.path().join("elsewhere"))
            .output()?;
        assert!(made.status.success());

        let location = resolved_location(&work.path().join(r"link\not\there"));
        assert!(path_is_within(
            &location,
            &real.join(r"elsewhere\not\there")
        ));
        assert!(path_is_within(
            &real.join(r"elsewhere\not\there"),
            &location
        ));
        Ok(())
    }

    #[test]
    fn strips_a_prefix_regardless_of_case() {
        assert_eq!(
            strip_prefix_ignore_case(
                Path::new(r"C:\Program Files\NVIDIA app\CEF\app.exe"),
                Path::new(r"c:\PROGRAM FILES\NVIDIA App\"),
            ),
            Some(PathBuf::from(r"CEF\app.exe"))
        );
        assert_eq!(
            strip_prefix_ignore_case(
                Path::new(r"C:\Program Files\GitHub"),
                Path::new(r"C:\Program Files\Git")
            ),
            None
        );
    }

    #[test]
    fn being_strictly_inside_rules_out_the_folder_itself_and_parent_steps() {
        let root = Path::new(r"C:\Program Files\Acme");
        assert!(path_is_strictly_inside(
            Path::new(r"c:\program files\acme\bin\svc.exe"),
            root
        ));
        assert!(!path_is_strictly_inside(root, root));
        assert!(!path_is_strictly_inside(
            Path::new(r"C:\Program Files\Acme\..\Other\svc.exe"),
            root
        ));
        assert!(!path_is_strictly_inside(
            Path::new(r"C:\Program Files\Acme Other\svc.exe"),
            root
        ));
    }

    #[test]
    fn recognizes_drive_roots() {
        assert!(is_drive_root(Path::new(r"C:\")));
        assert!(is_drive_root(Path::new("C:")));
        assert!(!is_drive_root(Path::new(r"C:\CocosDashboard")));
    }

    #[test]
    fn text_mentions_a_path_only_as_a_whole_path() {
        let git = Path::new(r"C:\Program Files\Git\");
        assert!(text_mentions_path(r"C:\Program Files\Git", git, false));
        assert!(text_mentions_path(
            r#""C:\PROGRAM FILES\GIT\bin\git.exe" %1"#,
            git,
            false
        ));
        assert!(text_mentions_path(
            r"C:\Windows;C:\Program Files\Git;C:\Tools",
            git,
            false
        ));
        assert!(!text_mentions_path(
            r"C:\Program Files\GitHub CLI\gh.exe",
            git,
            false
        ));
        assert!(!text_mentions_path(
            r"C:\Program Files\Git Extensions\x.exe",
            git,
            false
        ));

        assert!(text_mentions_path("C:/Program Files/Git/bin", git, false));

        let exe = Path::new(r"C:\App\app.exe");
        assert!(text_mentions_path(r"C:\App\app.exe --flag", exe, true));
        assert!(text_mentions_path(r"C:\App\app.exe,0", exe, false));
        assert!(!text_mentions_path(r"C:\App\app.exe.config", exe, true));
    }

    #[test]
    fn recognizes_auxiliary_executables() {
        for name in [
            "unins000.exe",
            "Uninstall OpenCode.exe",
            "helper.exe",
            "Update.exe",
            "GoogleUpdate.exe",
            "CocosDashboard_setup.exe",
            "VC_redist.x64.exe",
            "notification_helper.exe",
        ] {
            assert!(is_auxiliary_executable(Path::new(name)), "{name}");
        }
        for name in [
            "git-bash.exe",
            "chrome.exe",
            "Code.exe",
            "pwsh.exe",
            "firefox.exe",
        ] {
            assert!(!is_auxiliary_executable(Path::new(name)), "{name}");
        }
    }

    #[test]
    fn trims_display_icon_index() -> Result<(), Box<dyn std::error::Error>> {
        let parsed = split_command_executable_path(r"C:\Tools\App.exe,0")
            .ok_or_else(|| std::io::Error::other("path should parse"))?;

        assert_eq!(parsed, PathBuf::from(r"C:\Tools\App.exe"));
        Ok(())
    }
}
