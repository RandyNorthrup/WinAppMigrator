use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf, Prefix};
use std::process::Command;
use std::sync::OnceLock;

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

use crate::privilege::{is_elevated, may_be_elevated};

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
const TEMP_FOLDER: &str = "Temp";
/// Where the registry says where `Program Files` is.
const WINDOWS_VERSION_FOLDERS_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion";
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
/// What a tool may take from this process's environment, none of which can load code or say where
/// Windows is. Where Windows and its folders are is set separately, from the registry.
const TOOL_ENVIRONMENT: &[&str] = &["COMPUTERNAME", "USERDOMAIN", "USERNAME"];
/// What this is built as, which is what a program it starts runs as, emulated or not. An installer
/// picks what to install by this.
const PROCESSOR_THIS_RUNS_AS: &str = "AMD64";
/// Where an account's folders are, which a tool can go by. From what the registry says of that
/// account when this is elevated, and otherwise, as nothing is at stake, as they were given.
const ACCOUNT_FOLDER_VARIABLES: [&str; 5] = [
    "APPDATA",
    "HOMEDRIVE",
    "HOMEPATH",
    "LOCALAPPDATA",
    "USERPROFILE",
];
/// Windows' own, so that it does not go by what an account has said it is.
const TOOL_EXTENSIONS: &str = ".COM;.EXE;.BAT;.CMD";
/// The folders every Windows has and where the admin-only registry says each is, as the variable a
/// program would read and the value there.
const MACHINE_FOLDER_VARIABLES: [(&str, &str); 6] = [
    ("ProgramFiles", "ProgramFilesDir"),
    ("ProgramFiles(x86)", "ProgramFilesDir (x86)"),
    ("ProgramW6432", "ProgramW6432Dir"),
    ("CommonProgramFiles", "CommonFilesDir"),
    ("CommonProgramFiles(x86)", "CommonFilesDir (x86)"),
    ("CommonProgramW6432", "CommonW6432Dir"),
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
    let mut command = tool_command(program.as_ref());
    // Without elevation there is no one to protect, and a tool may need to write where it can.
    if !may_be_elevated() {
        for name in ACCOUNT_FOLDER_VARIABLES.iter().chain(&["TEMP", "TMP"]) {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
    } else if let Some(profile) = current_user_sid().and_then(profile_of_sid) {
        set_account_folders(&mut command, &profile);
    }
    command
}

/// An account's folders, as what they are by default, from where the registry says its profile is. A
/// tool started here that needs one moved somewhere else can do without.
fn set_account_folders(command: &mut Command, profile: &Path) {
    command
        .env("USERPROFILE", profile)
        .env("APPDATA", profile.join(r"AppData\Roaming"))
        .env("LOCALAPPDATA", profile.join(r"AppData\Local"));
    if let Some(drive) = profile.components().next() {
        command.env("HOMEDRIVE", drive.as_os_str());
    }
    let without_drive: PathBuf = profile.components().skip(1).collect();
    command.env("HOMEPATH", without_drive);
}

/// [`hidden_command`], for asking whether this is elevated, which that goes by.
pub(crate) fn tool_command(program: &OsStr) -> Command {
    let tool = resolve_tool(program);
    let mut command = Command::new(tool);
    set_tool_environment(&mut command);
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

/// Expand `%VAR%` references as this PC means them: the folders every Windows has from the registry,
/// and anything else from the current process environment. Unknown variables are left intact.
pub(crate) fn expand_env_vars(value: &str) -> String {
    expand_vars_with(value, &|name| {
        machine_variable(name)
            .or_else(|| std::env::var_os(name).and_then(|value| value.into_string().ok()))
    })
}

/// What a variable naming one of the folders every Windows has should say, from the admin-only
/// registry. The variable of that name is an account's to set, and an elevated process gets it.
pub(crate) fn machine_variable(name: &str) -> Option<String> {
    let text = |path: &Path| path.display().to_string();
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        "systemroot" | "windir" => Some(text(system_root())),
        "systemdrive" => system_root()
            .components()
            .next()
            .map(|drive| drive.as_os_str().to_string_lossy().to_string()),
        "comspec" => Some(text(&system_root().join(r"System32\cmd.exe"))),
        "programdata" | "allusersprofile" => program_data_dir().map(text),
        "public" => public_profile_dir().map(text),
        _ => {
            let (_, value) = MACHINE_FOLDER_VARIABLES
                .iter()
                .find(|(variable, _)| variable.eq_ignore_ascii_case(name))?;
            RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey(WINDOWS_VERSION_FOLDERS_KEY)
                .and_then(|key| key.get_value::<String, _>(value))
                .ok()
        }
    }
}

/// The `Program Files` folders of this PC, from the admin-only registry, with none listed twice.
pub fn program_files_folders() -> Vec<PathBuf> {
    let mut folders: Vec<PathBuf> = Vec::new();
    for name in ["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"] {
        if let Some(folder) = machine_variable(name).map(PathBuf::from)
            && !folders.contains(&folder)
        {
            folders.push(folder);
        }
    }
    folders
}

/// Expand `%VAR%` references using what `lookup` says of each. Unknown variables are left intact.
pub(crate) fn expand_vars_with(value: &str, lookup: &dyn Fn(&str) -> Option<String>) -> String {
    let mut output = String::new();
    let mut rest = value;
    while let Some(start) = rest.find('%') {
        output.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find('%') else {
            output.push('%');
            output.push_str(after);
            return output;
        };
        let name = &after[..end];
        match lookup(name) {
            Some(resolved) => output.push_str(&resolved),
            None => {
                output.push('%');
                output.push_str(name);
                output.push('%');
            }
        }
        rest = &after[end + 1..];
    }
    output.push_str(rest);
    output
}

/// PowerShell, to run this script and nothing else of the user's. See [`hidden_command`].
pub fn powershell_command(script: impl AsRef<OsStr>) -> Command {
    let mut command = hidden_command(POWERSHELL_EXE);
    command
        .args(["-NoProfile", "-NonInteractive", "-Command"])
        .arg(script);
    command
}

/// Give a tool a fixed environment instead of inheriting one. The user's environment is theirs to
/// change, and an elevated process gets it. PowerShell is a .NET program, so `COMPLUS_InstallRoot` can
/// load a runtime from any folder, `COR_PROFILER` can load a DLL, and `PSModulePath` can replace the
/// commands it runs, signature checks included. DISM and an older setup engine unpack what they then
/// load into `TEMP`, and a tool looks along `Path`. Those are the ones known about, so what is passed
/// is a list of what a tool needs. `TEMP` is Windows' own here, and what an elevated tool would unpack
/// and load goes in a folder of its own: see [`crate::privilege::run_with_protected_temp`].
fn set_tool_environment(command: &mut Command) {
    command.env_clear();
    for name in TOOL_ENVIRONMENT {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    for name in [
        "SystemRoot",
        "windir",
        "SystemDrive",
        "ComSpec",
        "ProgramData",
        "ALLUSERSPROFILE",
        "PUBLIC",
    ]
    .into_iter()
    .chain(MACHINE_FOLDER_VARIABLES.iter().map(|(name, _)| *name))
    {
        if let Some(value) = machine_variable(name) {
            command.env(name, value);
        }
    }
    let system32 = system_root().join("System32");
    let powershell_home = system32.join(POWERSHELL_HOME);
    let windows_temp = system_root().join(TEMP_FOLDER);
    command
        .env("TEMP", &windows_temp)
        .env("TMP", &windows_temp)
        .env("OS", "Windows_NT")
        .env("PROCESSOR_ARCHITECTURE", PROCESSOR_THIS_RUNS_AS)
        .env("PATHEXT", TOOL_EXTENSIONS)
        .env("PSModulePath", powershell_home.join("Modules"));
    if let Ok(processors) = std::thread::available_parallelism() {
        command.env("NUMBER_OF_PROCESSORS", processors.to_string());
    }
    if let Ok(path) =
        std::env::join_paths([system32.clone(), system32.join("Wbem"), powershell_home])
    {
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

/// Which version of Windows this is by its own numbering, from the admin-only registry. `None` if
/// that cannot be read.
pub fn windows_version_numbers() -> Option<(u16, u16)> {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(WINDOWS_VERSION_KEY)
        .ok()?;
    let number = |name: &str| {
        key.get_value::<u32, _>(name)
            .ok()
            .and_then(|value| u16::try_from(value).ok())
    };
    Some((
        number("CurrentMajorVersionNumber")?,
        number("CurrentMinorVersionNumber")?,
    ))
}

/// `PROCESSOR_ARCHITECTURE` as Windows itself records it. The variable of that name says what this
/// process is emulated as, and anyone can set it.
pub fn windows_architecture_name() -> Option<String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
        .and_then(|key| key.get_value::<String, _>("PROCESSOR_ARCHITECTURE"))
        .ok()
}

/// `ProgramData`, from the admin-only registry.
pub fn program_data_dir() -> Option<&'static Path> {
    static FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOLDER
        .get_or_init(|| profile_list_folder("ProgramData"))
        .as_deref()
}

/// The folder profiles are in, from the admin-only registry.
pub fn profiles_folder() -> Option<PathBuf> {
    profile_list_folder("ProfilesDirectory")
}

/// The shared Public profile, from the admin-only registry.
pub fn public_profile_dir() -> Option<&'static Path> {
    static FOLDER: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOLDER
        .get_or_init(|| profile_list_folder("Public"))
        .as_deref()
}

/// The SID of the account running this process, from `whoami`, because a variable could
/// name any other.
pub fn current_user_sid() -> Option<&'static str> {
    static SID: OnceLock<Option<String>> = OnceLock::new();
    SID.get_or_init(|| {
        let output = tool_command(OsStr::new("whoami"))
            .args(["/user", "/fo", "csv", "/nh"])
            .output()
            .ok()?;
        let line = String::from_utf8_lossy(&output.stdout);
        sid_from_whoami_line(&line).map(str::to_owned)
    })
    .as_deref()
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
    let output = powershell_command(script.as_str()).output();
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
pub(crate) fn comparable_path(path: &Path) -> String {
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
    /// What the listing said of it, which takes nothing more to find out. A size here can lag behind
    /// a file that is open for writing, so it is not to be taken as what would be read.
    pub metadata: Option<std::fs::Metadata>,
    /// A folder the walk went no further into, as it was as deep as it was asked to go. What is in it
    /// was not looked at, so it is not to be taken for empty.
    pub too_deep: bool,
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
                // It would likely say the same again, on a failing disk or a damaged folder.
                Some(Err(error)) => {
                    self.open.pop();
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
            let too_deep = file_type.is_dir() && self.open.len() >= self.max_depth;
            if file_type.is_dir() && !too_deep {
                self.enter(&path);
            }
            return Some(Ok(WalkedEntry {
                path,
                file_type,
                metadata: entry.metadata().ok(),
                too_deep,
            }));
        }
    }
}

/// Whether a path climbs anywhere. Lexically, `C:\\app\\..\\..\\Users` is inside `C:\\app`.
pub fn has_parent_step(path: &Path) -> bool {
    path.components()
        .any(|component| component == Component::ParentDir)
}

/// Whether a path is really in a folder, going by where both really are and not how they are spelled.
/// `C:\\app\\data\\x` is not in `C:\\app` if `data` is a link to somewhere else. Finding that out looks
/// through links, so on a drive, ask only about a path that may be read.
pub fn is_really_within(path: &Path, folder: &Path) -> bool {
    !has_parent_step(path)
        && location_can_be_told(path)
        && path_is_within(&resolved_location(path), &resolved_location(folder))
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

/// Whether a path written this way could send Windows to the network for it, which connects as
/// whoever is asking, and hands the far end the account's name and password hash. This goes by how
/// it is written and touches nothing. A share can be `\\server\share` or `//server/share`, or a
/// mix of the two, or `\\?\UNC\`, `\??\UNC\` or `\\?\GLOBALROOT\`, or a `file:` address, and rather
/// than say which of the rest can get there, only a plain drive or a relative path counts as
/// staying here. What a value holds can start with a quote, or an `@`, as a resource does.
pub fn is_network_path(text: &str) -> bool {
    let text = text.trim_start_matches(['"', ' ', '@']);
    let is_an_address = text
        .get(..5)
        .is_some_and(|start| start.eq_ignore_ascii_case("file:"));
    is_an_address || !is_on_a_local_disk(Path::new(text))
}

/// Whether [`resolved_location`] can say where a path really is. Being missing is not a problem,
/// since nothing is there. Anything else is: it can be listed through a link to a place with no
/// name to give, such as a shadow copy or a volume with no letter, which would read as being wherever
/// the link is. Windows says such a place is not found when asked its name, so it takes looking
/// at whether something is there.
pub fn location_can_be_told(path: &Path) -> bool {
    let mut current = path;
    loop {
        match std::fs::canonicalize(current) {
            Ok(_) => return true,
            Err(error) if error.kind() != std::io::ErrorKind::NotFound => return false,
            Err(_) if std::fs::symlink_metadata(current).is_ok() => return false,
            Err(_) => {}
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return true,
        }
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
    // Nothing is inside no place at all, and everything not written as a full path would look it.
    if root.as_os_str().is_empty() {
        return false;
    }
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
        assert!(
            current_user_sid()
                .and_then(profile_of_sid)
                .is_some_and(|profile| profile.is_dir())
        );
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
    fn expands_known_env_vars_and_keeps_unknown() {
        // USERPROFILE is always present on Windows; avoid mutating the env.
        if let Some(profile) = std::env::var_os("USERPROFILE").and_then(|v| v.into_string().ok()) {
            assert_eq!(
                expand_env_vars(r"%USERPROFILE%\Documents"),
                format!(r"{profile}\Documents")
            );
        }
        assert_eq!(
            expand_env_vars(r"%WAM_DEFINITELY_MISSING%\x"),
            r"%WAM_DEFINITELY_MISSING%\x"
        );
        assert_eq!(expand_env_vars("no-vars-here"), "no-vars-here");
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
    }

    #[test]
    fn every_tool_is_given_what_windows_says_and_nothing_of_this_process() {
        for tool in ["sc.exe", "cmd", "whoami", "powershell.exe"] {
            let command = hidden_command(tool);
            let said = |name: &str| {
                command
                    .get_envs()
                    .find(|(key, _)| key.eq_ignore_ascii_case(name))
                    .and_then(|(_, value)| value)
                    .map(|value| value.to_string_lossy().to_string())
            };
            let windows = system_root().display().to_string();
            assert_eq!(
                said("SystemRoot").as_deref(),
                Some(windows.as_str()),
                "{tool}"
            );
            if may_be_elevated() {
                assert_eq!(
                    said("TEMP"),
                    Some(system_root().join("Temp").display().to_string()),
                    "{tool}"
                );
            }
            assert_eq!(
                said("ProgramData"),
                machine_variable("ProgramData"),
                "{tool}"
            );
            assert!(
                said("Path").is_some_and(|path| path.starts_with(&windows)),
                "{tool}"
            );
            for not_passed in ["COR_PROFILER", "COMPLUS_InstallRoot", "PYTHONPATH"] {
                assert_eq!(said(not_passed), None, "{tool}");
            }
        }
    }

    #[test]
    fn what_a_variable_says_of_windows_own_folders_comes_from_the_registry() {
        assert_eq!(
            machine_variable("systemroot"),
            Some(system_root().display().to_string())
        );
        assert!(
            machine_variable("ProgramFiles").is_some_and(|folder| Path::new(&folder).is_absolute())
        );
        assert_eq!(machine_variable("WAM_DEFINITELY_MISSING"), None);
        assert!(!program_files_folders().is_empty());
        assert!(
            expand_env_vars(r"%SystemRoot%\x").starts_with(&system_root().display().to_string())
        );
    }

    fn powershell_output(script: &str) -> std::io::Result<String> {
        let output = powershell_command(script).output()?;
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
    fn a_walk_says_of_a_folder_it_went_no_further_into() {
        let work = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
        std::fs::create_dir_all(work.path().join(r"a\b\c"))
            .unwrap_or_else(|error| panic!("{error}"));
        std::fs::write(work.path().join(r"a\b\c\deep.txt"), "x")
            .unwrap_or_else(|error| panic!("{error}"));
        let found: Vec<(String, bool)> = walk_asking(work.path(), 2, |_, _| true)
            .flatten()
            .map(|entry| (file_name_string(&entry.path), entry.too_deep))
            .collect();
        assert_eq!(
            found,
            [("a".to_owned(), false), ("b".to_owned(), true)],
            "c and what is in it are not seen, and b says so"
        );
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
    fn being_inside_a_folder_goes_by_where_it_really_is() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let app = work.path().join("app");
        let elsewhere = work.path().join("elsewhere");
        std::fs::create_dir_all(app.join("data"))?;
        std::fs::create_dir_all(&elsewhere)?;
        let linked = app.join("linked");
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&linked)
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());
        assert!(is_really_within(&app.join(r"data\x.txt"), &app));
        assert!(is_really_within(&app.join("not yet there"), &app));
        assert!(!is_really_within(&linked.join("x.txt"), &app));
        assert!(!is_really_within(
            &app.join(r"data\..\..\elsewhere\x"),
            &app
        ));
        Ok(())
    }

    #[test]
    fn a_link_to_a_place_with_no_name_cannot_have_its_location_told() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let gone = work.path().join("gone");
        let link = work.path().join("link");
        std::fs::create_dir_all(&gone)?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&gone)
            .output()?;
        assert!(made.status.success());
        // A link that leads nowhere reads as not found when asked its name, just as a place with
        // no letter does, and it is there all the same.
        std::fs::remove_dir(&gone)?;
        assert!(!location_can_be_told(&link));
        assert!(!location_can_be_told(&link.join("x.txt")));
        assert!(location_can_be_told(&work.path().join(r"not there\x.txt")));
        assert!(location_can_be_told(work.path()));
        Ok(())
    }

    #[test]
    fn however_a_share_is_written_it_is_a_network_path() {
        for share in [
            r"\\server\share\a.ico",
            "//server/share/a.ico",
            r"\/server/share\a.ico",
            r"/\server\share\a.ico",
            r"\\?\UNC\server\share\a.ico",
            r"\\?/UNC/server/share/a.ico",
            r"\??\UNC\server\share\a.ico",
            r"\\.\UNC\server\share\a.ico",
            r"\\?\GLOBALROOT\Device\Mup\server\share\a.ico",
            r"\Device\Mup\server\share\a.ico",
            r#""\\server\share\a.ico",0"#,
            r"@\\server\share\a.dll,-1",
            r"file://server/share/a.ico",
            r"FILE:\\server\share\a.ico",
        ] {
            assert!(is_network_path(share), "{share}");
        }
        for here in [
            r"C:\Program Files\Acme\a.ico,0",
            "C:/Program Files/Acme/a.ico",
            r#""C:\Program Files\Acme\a.ico",0"#,
            r"\\?\C:\Program Files\Acme\a.ico",
            r"@C:\Acme\a.dll,-1",
            r"%SystemRoot%\system32\a.dll",
            "a.ico",
            "",
        ] {
            assert!(!is_network_path(here), "{here}");
        }
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
    fn nothing_is_inside_no_place_at_all() {
        for path in [
            "txtfile",
            r"bin\a.exe",
            r"%SystemRoot%\x.exe",
            r"C:\App\a.exe",
            "",
        ] {
            assert!(
                !path_is_strictly_inside(Path::new(path), Path::new("")),
                "{path}"
            );
        }
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
