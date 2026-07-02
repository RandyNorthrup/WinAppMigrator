use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::models::{ShortcutEntry, Warning, WarningSeverity};
use crate::path_utils::hidden_command;
use crate::system_context::SystemContext;

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
        // The live shell folders and WScript.Shell COM only see this PC. Reading
        // a foreign disk's .lnk files is not required for a correct migration —
        // the installer recreates shortcuts from the captured install location on
        // restore — so we skip source-shortcut enumeration and note it.
        let warning = Warning::new(
            "shortcuts_offline_skipped",
            WarningSeverity::Info,
            "Source Start Menu/Desktop shortcuts are not read from a connected drive; the \
             installer recreates shortcuts from the app's install location on restore.",
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
            .is_some_and(|root| candidate.starts_with(root))
}
