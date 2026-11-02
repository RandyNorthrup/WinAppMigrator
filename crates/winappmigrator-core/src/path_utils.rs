use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

const EXE_EXTENSION: &str = "exe";
const WINDOWS_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
const SYSTEM_ROOT_VALUE: &str = "SystemRoot";
const DEFAULT_SYSTEM_ROOT: &str = r"C:\Windows";
const POWERSHELL_EXE: &str = "powershell.exe";

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
        // Both come from the user's environment, which a process that is not elevated can
        // change. A module folder could replace the commands the scripts use, signature
        // checks included, and a profiler is loaded into every .NET program.
        command.env(
            "PSModulePath",
            system_root().join(r"System32\WindowsPowerShell\v1.0\Modules"),
        );
        command.env_remove("COR_ENABLE_PROFILING");
        command.env_remove("CORECLR_ENABLE_PROFILING");
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
    fn powershell_ignores_the_users_module_folders_and_profilers() {
        let command = hidden_command("powershell.exe");
        let envs: Vec<_> = command.get_envs().collect();
        assert!(envs.iter().any(|(name, value)| {
            *name == "PSModulePath"
                && value.is_some_and(|value| Path::new(value).starts_with(system_root()))
        }));
        assert!(envs.contains(&(OsStr::new("COR_ENABLE_PROFILING"), None)));
        assert!(hidden_command("sc.exe").get_envs().next().is_none());
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
