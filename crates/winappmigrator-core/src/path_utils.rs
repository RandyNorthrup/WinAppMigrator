use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXE_EXTENSION: &str = "exe";

/// Build a `Command` that does not pop up a console window for the child
/// process. Every external tool WinAppMigrator runs (sc.exe, PowerShell, winget,
/// bundled installers, cmd) must use this so building or installing never flashes
/// terminal windows over the GUI/app.
pub fn hidden_command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW: run the child without allocating a console.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
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

pub fn path_eq_case_insensitive(left: &Path, right: &Path) -> bool {
    left.to_string_lossy()
        .eq_ignore_ascii_case(&right.to_string_lossy())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_executable_from_command() -> Result<(), Box<dyn std::error::Error>> {
        let parsed = split_command_executable_path(r#""C:\Program Files\App\App.exe" /quiet"#)
            .ok_or_else(|| std::io::Error::other("path should parse"))?;

        assert_eq!(parsed, PathBuf::from(r"C:\Program Files\App\App.exe"));
        Ok(())
    }

    #[test]
    fn trims_display_icon_index() -> Result<(), Box<dyn std::error::Error>> {
        let parsed = split_command_executable_path(r"C:\Tools\App.exe,0")
            .ok_or_else(|| std::io::Error::other("path should parse"))?;

        assert_eq!(parsed, PathBuf::from(r"C:\Tools\App.exe"));
        Ok(())
    }
}
