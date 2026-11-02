use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

const EXE_EXTENSION: &str = "exe";
const WINDOWS_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
const SYSTEM_ROOT_VALUE: &str = "SystemRoot";
const PROFILE_LIST_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList";
const SYSTEM_DRIVE_MARKER: &str = "%SystemDrive%";
const DEFAULT_SYSTEM_ROOT: &str = r"C:\Windows";
const POWERSHELL_EXE: &str = "powershell.exe";
const POWERSHELL_HOME: &str = r"WindowsPowerShell\v1.0";
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
/// writes no byte order mark.
pub const POWERSHELL_UTF8_OUTPUT: &str =
    "[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding $false; ";

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

fn query_user_profile() -> Option<PathBuf> {
    let output = hidden_command("whoami")
        .args(["/user", "/fo", "csv", "/nh"])
        .output()
        .ok()?;
    let line = String::from_utf8_lossy(&output.stdout);
    let sid = sid_from_whoami_line(&line)?;
    let recorded = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"{PROFILE_LIST_KEY}\{sid}"))
        .and_then(|key| key.get_value::<String, _>("ProfileImagePath"))
        .ok()?;
    Some(expand_system_drive(&recorded, system_root()))
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

/// A path as lowercase text with forward slashes and any trailing separator removed, for
/// comparing paths that come from different records. `C:\` becomes `c:`.
fn comparable_path(path: &Path) -> String {
    let lower = path
        .to_string_lossy()
        .replace('/', "\\")
        .to_ascii_lowercase();
    lower.trim_end_matches('\\').to_owned()
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
