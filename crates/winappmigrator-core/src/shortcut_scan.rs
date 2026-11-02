use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::models::{ShortcutEntry, Warning, WarningSeverity};
use crate::path_utils::{hidden_command, path_is_within};
use crate::system_context::SystemContext;

const LNK_HEADER_SIZE: u32 = 0x4C;
const LNK_FLAGS_OFFSET: usize = 0x14;
const LNK_HAS_TARGET_ID_LIST: u32 = 0x1;
const LNK_HAS_LINK_INFO: u32 = 0x2;
const LNK_ID_LIST_SIZE_FIELD: usize = 2;
const LNK_INFO_HEADER_SIZE_OFFSET: usize = 0x04;
const LNK_INFO_FLAGS_OFFSET: usize = 0x08;
const LNK_INFO_BASE_OFFSET: usize = 0x10;
const LNK_INFO_SUFFIX_OFFSET: usize = 0x18;
const LNK_INFO_UNICODE_BASE_OFFSET: usize = 0x1C;
const LNK_INFO_UNICODE_SUFFIX_OFFSET: usize = 0x20;
const LNK_INFO_UNICODE_HEADER_SIZE: u32 = 0x24;
const LNK_INFO_HAS_LOCAL_PATH: u32 = 0x1;

#[derive(Debug, Clone)]
pub struct ShortcutScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellShortcut {
    path: Option<String>,
    target_path: Option<String>,
    arguments: Option<String>,
    working_directory: Option<String>,
    icon_location: Option<String>,
}

pub fn scan_shortcuts(
    context: &ShortcutScanContext,
    system: &SystemContext,
) -> (Vec<ShortcutEntry>, Vec<Warning>) {
    if system.is_offline() {
        // The live shell folders and WScript.Shell COM only see this PC, and a
        // foreign disk's .lnk files are not parsed, so an installer built from a
        // connected drive restores no shortcuts. The warning says so.
        let warning = Warning::new(
            "shortcuts_offline_skipped",
            WarningSeverity::Info,
            "Start Menu and Desktop shortcuts are not read from a connected drive, so this \
             installer will not recreate them.",
            None,
        );
        return (Vec::new(), vec![warning]);
    }
    scan_live_shortcuts(context)
}

fn scan_live_shortcuts(context: &ShortcutScanContext) -> (Vec<ShortcutEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let script = r#"
$roots = @(
  [Environment]::GetFolderPath('Desktop'),
  [Environment]::GetFolderPath('CommonDesktopDirectory'),
  [Environment]::GetFolderPath('StartMenu'),
  [Environment]::GetFolderPath('CommonStartMenu')
) | Where-Object { $_ -and (Test-Path $_) } | Select-Object -Unique
$shell = New-Object -ComObject WScript.Shell
$items = foreach ($root in $roots) {
  Get-ChildItem -LiteralPath $root -Filter *.lnk -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
    $shortcut = $shell.CreateShortcut($_.FullName)
    [pscustomobject]@{
      Path = $_.FullName
      TargetPath = $shortcut.TargetPath
      Arguments = $shortcut.Arguments
      WorkingDirectory = $shortcut.WorkingDirectory
      IconLocation = $shortcut.IconLocation
    }
  }
}
$items | ConvertTo-Json -Depth 4
"#;
    let output = match run_powershell(script) {
        Ok(output) => output,
        Err(error) => {
            warnings.push(Warning::new(
                "shortcuts_unreadable",
                WarningSeverity::Warning,
                format!("Could not query shortcuts: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };
    let shortcuts = match parse_json_array::<PowerShellShortcut>(&output) {
        Ok(shortcuts) => shortcuts,
        Err(error) => {
            warnings.push(Warning::new(
                "shortcuts_parse_failed",
                WarningSeverity::Warning,
                format!("Could not parse shortcut query output: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };

    let entries = shortcuts
        .into_iter()
        .filter_map(|shortcut| shortcut_entry(context, shortcut))
        .collect();
    (entries, warnings)
}

fn shortcut_entry(
    context: &ShortcutScanContext,
    shortcut: PowerShellShortcut,
) -> Option<ShortcutEntry> {
    let target_path = PathBuf::from(shortcut.target_path?);
    if !path_matches(context, &target_path) {
        return None;
    }

    Some(ShortcutEntry {
        original_lnk_path: PathBuf::from(shortcut.path?),
        target_path,
        arguments: shortcut.arguments.filter(|value| !value.trim().is_empty()),
        working_directory: shortcut
            .working_directory
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from),
        icon_path: shortcut
            .icon_location
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from),
        target_is_selected_app: true,
    })
}

fn run_powershell(script: &str) -> std::io::Result<String> {
    let output = hidden_command("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn parse_json_array<T>(json: &str) -> serde_json::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let trimmed = json.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    if trimmed.starts_with('[') {
        serde_json::from_str(trimmed)
    } else {
        serde_json::from_str(trimmed).map(|item| vec![item])
    }
}

fn path_matches(context: &ShortcutScanContext, candidate: &Path) -> bool {
    candidate
        .to_string_lossy()
        .eq_ignore_ascii_case(&context.executable_path.to_string_lossy())
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_within(candidate, root))
}

/// Read the target of a `.lnk` file from its `LinkInfo` section, without the COM object
/// `scan_live_shortcuts` uses, so it also works on a shortcut from another PC. A shortcut
/// that only stores a shell item list has no such path, and neither does an MSI
/// "advertised" one: both give `None`. The format is documented in MS-SHLLINK.
pub(crate) fn lnk_target_path(data: &[u8]) -> Option<PathBuf> {
    if read_u32(data, 0)? != LNK_HEADER_SIZE {
        return None;
    }
    let flags = read_u32(data, LNK_FLAGS_OFFSET)?;
    let mut offset = LNK_HEADER_SIZE as usize;
    if flags & LNK_HAS_TARGET_ID_LIST != 0 {
        let list_size = usize::from(read_u16(data, offset)?);
        offset += LNK_ID_LIST_SIZE_FIELD + list_size;
    }
    if flags & LNK_HAS_LINK_INFO == 0 {
        return None;
    }
    let info = data.get(offset..)?;
    if read_u32(info, LNK_INFO_FLAGS_OFFSET)? & LNK_INFO_HAS_LOCAL_PATH == 0 {
        return None;
    }
    let header_size = read_u32(info, LNK_INFO_HEADER_SIZE_OFFSET)?;
    if header_size >= LNK_INFO_UNICODE_HEADER_SIZE
        && let Some(base) =
            read_u32(info, LNK_INFO_UNICODE_BASE_OFFSET).and_then(|at| utf16_string(info, at))
    {
        let suffix = read_u32(info, LNK_INFO_UNICODE_SUFFIX_OFFSET)
            .and_then(|at| utf16_string(info, at))
            .unwrap_or_default();
        return Some(PathBuf::from(format!("{base}{suffix}")));
    }
    let base = ansi_string(info, read_u32(info, LNK_INFO_BASE_OFFSET)?)?;
    let suffix = read_u32(info, LNK_INFO_SUFFIX_OFFSET)
        .and_then(|at| ansi_string(info, at))
        .unwrap_or_default();
    Some(PathBuf::from(format!("{base}{suffix}"))).filter(|path| !path.as_os_str().is_empty())
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// A NUL-terminated string in the local code page, read as Latin-1: a path outside that
/// fails the existence check callers apply, and the shortcut is simply not used.
fn ansi_string(data: &[u8], offset: u32) -> Option<String> {
    let start = data.get(offset as usize..)?;
    let end = start.iter().position(|&byte| byte == 0)?;
    Some(start[..end].iter().map(|&byte| char::from(byte)).collect())
}

fn utf16_string(data: &[u8], offset: u32) -> Option<String> {
    let (pairs, _) = data.get(offset as usize..)?.as_chunks::<2>();
    let units: Vec<u16> = pairs
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|&unit| unit != 0)
        .collect();
    Some(String::from_utf16_lossy(&units))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A shortcut holding the given local path, in the ANSI form or also the Unicode one.
    fn lnk_bytes(base: &str, suffix: &str, unicode: bool, id_list: &[u8]) -> Vec<u8> {
        let mut data = vec![0_u8; LNK_HEADER_SIZE as usize];
        data[0..4].copy_from_slice(&LNK_HEADER_SIZE.to_le_bytes());
        let mut flags = LNK_HAS_LINK_INFO;
        if !id_list.is_empty() {
            flags |= LNK_HAS_TARGET_ID_LIST;
            data.extend_from_slice(&(id_list.len() as u16).to_le_bytes());
            data.extend_from_slice(id_list);
        }
        data[LNK_FLAGS_OFFSET..LNK_FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());

        let header_size: u32 = if unicode {
            LNK_INFO_UNICODE_HEADER_SIZE
        } else {
            0x1C
        };
        let base_at = header_size;
        let suffix_at = base_at + base.len() as u32 + 1;
        let unicode_base_at = suffix_at + suffix.len() as u32 + 1;
        let unicode_suffix_at = unicode_base_at + (base.encode_utf16().count() as u32 + 1) * 2;
        let mut info = vec![0_u8; header_size as usize];
        let mut put = |at: usize, value: u32| {
            info[at..at + 4].copy_from_slice(&value.to_le_bytes());
        };
        put(LNK_INFO_HEADER_SIZE_OFFSET, header_size);
        put(LNK_INFO_FLAGS_OFFSET, LNK_INFO_HAS_LOCAL_PATH);
        put(LNK_INFO_BASE_OFFSET, base_at);
        put(LNK_INFO_SUFFIX_OFFSET, suffix_at);
        if unicode {
            put(LNK_INFO_UNICODE_BASE_OFFSET, unicode_base_at);
            put(LNK_INFO_UNICODE_SUFFIX_OFFSET, unicode_suffix_at);
        }
        info.extend_from_slice(base.as_bytes());
        info.push(0);
        info.extend_from_slice(suffix.as_bytes());
        info.push(0);
        if unicode {
            for text in [base, suffix] {
                info.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
                info.extend_from_slice(&[0, 0]);
            }
        }
        data.extend_from_slice(&info);
        data
    }

    #[test]
    fn reads_a_shortcut_target() {
        let path = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
        assert_eq!(
            lnk_target_path(&lnk_bytes(path, "", false, &[])),
            Some(PathBuf::from(path))
        );
        assert_eq!(
            lnk_target_path(&lnk_bytes(path, "", true, &[1, 2, 3, 4])),
            Some(PathBuf::from(path)),
            "skips the item list, and reads the Unicode form when there is one"
        );
        assert_eq!(
            lnk_target_path(&lnk_bytes(r"C:\Program Files\", "App\\app.exe", false, &[])),
            Some(PathBuf::from(r"C:\Program Files\App\app.exe")),
            "the path can be split into a base and a suffix"
        );
    }

    #[test]
    fn a_shortcut_without_a_local_path_gives_nothing() {
        let mut advertised = vec![0_u8; LNK_HEADER_SIZE as usize];
        advertised[0..4].copy_from_slice(&LNK_HEADER_SIZE.to_le_bytes());
        assert_eq!(lnk_target_path(&advertised), None);
        assert_eq!(lnk_target_path(b"not a shortcut"), None);
        assert_eq!(lnk_target_path(&[]), None);
    }

    #[test]
    fn a_truncated_shortcut_does_not_panic() {
        let whole = lnk_bytes(r"C:\App\app.exe", "", true, &[9, 9]);
        for cut in 0..whole.len() {
            let _ = lnk_target_path(&whole[..cut]);
        }
    }

    #[test]
    fn a_target_in_a_folder_with_a_longer_name_is_not_inside_the_install_folder() {
        let context = ShortcutScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\Git\git-bash.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Git")),
        };
        assert!(path_matches(
            &context,
            Path::new(r"C:\PROGRAM FILES\GIT\cmd\git.exe")
        ));
        assert!(!path_matches(
            &context,
            Path::new(r"C:\Program Files\GitHub CLI\gh.exe")
        ));
    }
}
