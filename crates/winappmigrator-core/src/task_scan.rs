use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer};
use walkdir::WalkDir;

use crate::models::{ScheduledTaskEntry, Warning, WarningSeverity};
use crate::path_utils::{
    POWERSHELL_IMPORT_TASKS, POWERSHELL_IMPORT_UTILITY, POWERSHELL_UTF8_OUTPUT,
    escape_powershell_single, hidden_command, path_is_within,
};
use crate::system_context::SystemContext;

const OFFLINE_TASKS_SUBDIR: &str = r"System32\Tasks";
const TASK_RESTORE_INSTRUCTIONS: &str =
    "Restore uses Register-ScheduledTask with the archived XML definition.";

/// The XML of a scheduled task on this PC, or `None` if there is no such task.
pub(crate) fn live_task_xml(task_path: &str, task_name: &str) -> std::io::Result<Option<String>> {
    let script = format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_TASKS}$task = Get-ScheduledTask -TaskName '{name}' -TaskPath '{path}' -ErrorAction SilentlyContinue; if ($null -eq $task) {{ exit 2 }}; Export-ScheduledTask -TaskName '{name}' -TaskPath '{path}'",
        name = escape_powershell_single(task_name),
        path = escape_powershell_single(task_path),
    );
    let output = hidden_command("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script.as_str()])
        .output()?;
    if output.status.code() == Some(2) {
        return Ok(None);
    }
    if output.status.success() {
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    } else {
        Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct TaskScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

pub fn scan_scheduled_tasks(
    context: &TaskScanContext,
    system: &SystemContext,
) -> (Vec<ScheduledTaskEntry>, Vec<Warning>) {
    if system.is_offline() {
        // The live Task Scheduler cannot enumerate another disk's tasks; read the
        // authoritative on-disk XML definitions instead.
        scan_offline_tasks(context, system)
    } else {
        scan_live_tasks(context)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellTaskAction {
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    task_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    task_path: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    execute: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    arguments: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    working_directory: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    trigger_summary: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    run_level: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    account: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    definition_xml: Option<String>,
}

fn scan_live_tasks(context: &TaskScanContext) -> (Vec<ScheduledTaskEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let script = r#"
$items = foreach ($task in Get-ScheduledTask) {
  $definitionXml = $null
  try {
    $definitionXml = Export-ScheduledTask -TaskName $task.TaskName -TaskPath $task.TaskPath
  } catch {
    $definitionXml = $null
  }
  $triggerSummary = ($task.Triggers | ForEach-Object { $_.CimClass.CimClassName }) -join '; '
  foreach ($action in $task.Actions) {
    [pscustomobject]@{
      TaskName = [string]$task.TaskName
      TaskPath = [string]$task.TaskPath
      Execute = if ($null -ne $action.Execute) { [string]$action.Execute } else { $null }
      Arguments = if ($null -ne $action.Arguments) { [string]$action.Arguments } else { $null }
      WorkingDirectory = if ($null -ne $action.WorkingDirectory) { [string]$action.WorkingDirectory } else { $null }
      TriggerSummary = if ($null -ne $triggerSummary) { [string]$triggerSummary } else { $null }
      RunLevel = if ($null -ne $task.Principal.RunLevel) { [string]$task.Principal.RunLevel } else { $null }
      Account = if ($null -ne $task.Principal.UserId) { [string]$task.Principal.UserId } else { $null }
      DefinitionXml = $definitionXml
    }
  }
}
$items | ConvertTo-Json -Depth 4
"#;
    let output = match run_powershell(script) {
        Ok(output) => output,
        Err(error) => {
            warnings.push(Warning::new(
                "scheduled_tasks_unreadable",
                WarningSeverity::Warning,
                format!("Could not query scheduled tasks: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };
    let actions = match parse_json_array::<PowerShellTaskAction>(&output) {
        Ok(actions) => actions,
        Err(error) => {
            warnings.push(Warning::new(
                "scheduled_tasks_parse_failed",
                WarningSeverity::Warning,
                format!("Could not parse scheduled task query output: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };

    let tasks = actions
        .into_iter()
        .filter_map(|action| task_from_action(context, action))
        .collect();
    (one_entry_per_task(tasks), warnings)
}

/// The query gives one row per action, and each row carries the task's whole definition, so
/// a task with two programs would otherwise be restored twice. The second would find the
/// first already there.
fn one_entry_per_task(mut tasks: Vec<ScheduledTaskEntry>) -> Vec<ScheduledTaskEntry> {
    let mut seen = HashSet::new();
    tasks.retain(|task| {
        seen.insert(format!("{}{}", task.task_path, task.task_name).to_ascii_lowercase())
    });
    tasks
}

fn task_from_action(
    context: &TaskScanContext,
    action: PowerShellTaskAction,
) -> Option<ScheduledTaskEntry> {
    let command = action.execute?;
    // Installers write the command both with quotes and without, and they mean the same.
    let command_path = PathBuf::from(expand_env_markers(command.trim().trim_matches('"')));
    if !path_matches(context, &command_path) {
        return None;
    }

    let requires_admin =
        task_requires_admin(action.run_level.as_deref(), action.account.as_deref());

    Some(ScheduledTaskEntry {
        task_name: action.task_name.unwrap_or_default(),
        task_path: action.task_path.unwrap_or_default(),
        action_command: command_path,
        arguments: action.arguments,
        working_directory: action.working_directory.map(PathBuf::from),
        trigger_summary: action.trigger_summary,
        run_level: action.run_level,
        account: action.account,
        requires_admin,
        restore_instructions: TASK_RESTORE_INSTRUCTIONS.to_owned(),
        definition_xml: action.definition_xml,
    })
}

/// Offline: read scheduled-task definitions straight from the disk's
/// `Windows\System32\Tasks` tree (the same XML the live Task Scheduler stores
/// and that `Export-ScheduledTask` returns), matching those whose action runs
/// the selected app.
fn scan_offline_tasks(
    context: &TaskScanContext,
    system: &SystemContext,
) -> (Vec<ScheduledTaskEntry>, Vec<Warning>) {
    let mut tasks = Vec::new();
    let mut warnings = Vec::new();
    let windows_dir = system.windows_dir();
    let tasks_root = windows_dir.join(OFFLINE_TASKS_SUBDIR);

    for entry in WalkDir::new(&tasks_root).follow_links(false) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(Warning::new(
                    "offline_task_walk_error",
                    WarningSeverity::Info,
                    format!("Could not read a scheduled-task entry: {error}"),
                    Some(tasks_root.display().to_string()),
                ));
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(xml) = read_text_file(entry.path()) else {
            warnings.push(Warning::new(
                "offline_task_unreadable",
                WarningSeverity::Info,
                format!(
                    "Could not read scheduled-task definition {}; if it launches the app it was not captured.",
                    entry.path().display()
                ),
                Some(entry.path().display().to_string()),
            ));
            continue;
        };
        let Some(parsed) = parse_task_xml(&xml) else {
            warnings.push(Warning::new(
                "offline_task_parse_failed",
                WarningSeverity::Info,
                format!(
                    "Could not parse scheduled-task definition {}; if it launches the app it was not captured.",
                    entry.path().display()
                ),
                Some(entry.path().display().to_string()),
            ));
            continue;
        };
        let Some(execute) = parsed.execute.as_deref() else {
            continue;
        };
        let command_path = system.resolve_source_path(&expand_task_path(execute, &windows_dir));
        if !path_matches(context, &command_path) {
            continue;
        }

        let (task_name, task_path) = task_name_and_path(&tasks_root, entry.path());
        let working_directory = parsed
            .working_directory
            .as_deref()
            .map(|value| system.resolve_source_path(&expand_task_path(value, &windows_dir)));
        tasks.push(ScheduledTaskEntry {
            task_name,
            task_path,
            action_command: command_path,
            arguments: parsed.arguments,
            working_directory,
            trigger_summary: parsed.trigger_summary,
            run_level: parsed.run_level.clone(),
            account: parsed.account.clone(),
            requires_admin: task_requires_admin(
                parsed.run_level.as_deref(),
                parsed.account.as_deref(),
            ),
            restore_instructions: TASK_RESTORE_INSTRUCTIONS.to_owned(),
            definition_xml: Some(xml),
        });
    }

    tasks.sort_by(|left, right| left.task_name.cmp(&right.task_name));
    (tasks, warnings)
}

#[derive(Default)]
struct ParsedTask {
    execute: Option<String>,
    arguments: Option<String>,
    working_directory: Option<String>,
    trigger_summary: Option<String>,
    run_level: Option<String>,
    account: Option<String>,
}

/// Extract the fields we need from a Task Scheduler XML definition. Namespaces
/// are ignored (roxmltree exposes local tag names), so both namespaced and bare
/// documents parse.
fn parse_task_xml(xml: &str) -> Option<ParsedTask> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let mut parsed = ParsedTask::default();
    for node in document.descendants() {
        match node.tag_name().name() {
            "Command" if parsed.execute.is_none() => parsed.execute = node_text(node),
            "Arguments" if parsed.arguments.is_none() => parsed.arguments = node_text(node),
            "WorkingDirectory" if parsed.working_directory.is_none() => {
                parsed.working_directory = node_text(node)
            }
            "RunLevel" if parsed.run_level.is_none() => parsed.run_level = node_text(node),
            "UserId" if parsed.account.is_none() => parsed.account = node_text(node),
            "Triggers" if parsed.trigger_summary.is_none() => {
                let summary = node
                    .children()
                    .filter(|child| child.is_element())
                    .map(|child| child.tag_name().name().to_owned())
                    .collect::<Vec<_>>()
                    .join("; ");
                if !summary.is_empty() {
                    parsed.trigger_summary = Some(summary);
                }
            }
            _ => {}
        }
    }
    parsed.execute.is_some().then_some(parsed)
}

fn node_text(node: roxmltree::Node<'_, '_>) -> Option<String> {
    let text = node.text().map(str::trim).unwrap_or_default();
    (!text.is_empty()).then(|| text.to_owned())
}

/// Derive `(TaskName, TaskPath)` from a task file's location under the Tasks
/// root, e.g. `<root>\Vendor\Update` → (`Update`, `\Vendor\`).
fn task_name_and_path(tasks_root: &Path, file: &Path) -> (String, String) {
    let relative = file.strip_prefix(tasks_root).unwrap_or(file);
    let task_name = relative
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_owned();
    let parent = relative
        .parent()
        .map(|value| value.to_string_lossy().replace('/', "\\"))
        .unwrap_or_default();
    let task_path = if parent.is_empty() {
        "\\".to_owned()
    } else {
        format!(r"\{parent}\")
    };
    (task_name, task_path)
}

/// Expand `%SystemRoot%`/`\SystemRoot\` markers in a task path against the
/// scanned system's Windows directory; other values pass through unchanged
/// (drive-letter re-rooting happens afterward via the context).
fn expand_task_path(value: &str, windows_dir: &Path) -> PathBuf {
    let trimmed = value.trim().trim_matches('"');
    for marker in [r"%SystemRoot%\", r"%windir%\", r"\SystemRoot\"] {
        if trimmed.len() >= marker.len() && trimmed[..marker.len()].eq_ignore_ascii_case(marker) {
            return windows_dir.join(&trimmed[marker.len()..]);
        }
    }
    PathBuf::from(trimmed)
}

/// Read a text file that may be UTF-16 (as Task XML usually is) or UTF-8.
fn read_text_file(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        let (pairs, _odd_byte) = rest.as_chunks::<2>();
        let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
        Some(String::from_utf16_lossy(&units))
    } else if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        Some(String::from_utf8_lossy(rest).into_owned())
    } else {
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }
}

fn task_requires_admin(run_level: Option<&str>, account: Option<&str>) -> bool {
    run_level.is_some_and(|value| {
        let normalized = value.trim().to_ascii_lowercase();
        normalized == "1" || normalized.contains("highest")
    }) || account.is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "system" | "localsystem" | "nt authority\\system"
        )
    })
}

fn deserialize_optional_string<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(value)) => Some(value),
        Some(serde_json::Value::Number(value)) => Some(value.to_string()),
        Some(serde_json::Value::Bool(value)) => Some(value.to_string()),
        Some(value) => Some(value.to_string()),
    })
}

fn run_powershell(script: &str) -> std::io::Result<String> {
    let script = format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_TASKS}{POWERSHELL_IMPORT_UTILITY}{script}"
    );
    let output = hidden_command("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
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

fn path_matches(context: &TaskScanContext, candidate: &Path) -> bool {
    candidate
        .to_string_lossy()
        .eq_ignore_ascii_case(&context.executable_path.to_string_lossy())
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_within(candidate, root))
}

fn expand_env_markers(value: &str) -> String {
    let mut expanded = value.to_owned();
    for (key, val) in std::env::vars() {
        let marker = format!("%{key}%");
        expanded = expanded.replace(&marker, &val);
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> TaskScanContext {
        TaskScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\Demo\demo.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Demo")),
        }
    }

    fn action(name: &str, execute: &str) -> PowerShellTaskAction {
        PowerShellTaskAction {
            task_name: Some(name.to_owned()),
            task_path: Some(r"\Demo\".to_owned()),
            execute: Some(execute.to_owned()),
            arguments: None,
            working_directory: None,
            trigger_summary: None,
            run_level: None,
            account: None,
            definition_xml: None,
        }
    }

    #[test]
    fn a_quoted_command_is_the_same_program_as_a_bare_one() {
        for execute in [
            r#""C:\Program Files\Demo\demo.exe""#,
            r"C:\Program Files\Demo\demo.exe",
            r#"  "C:\Program Files\Demo\demo.exe"  "#,
            r"c:\program files\demo\bin\other.exe",
        ] {
            let task = task_from_action(&context(), action("Update", execute));
            assert!(
                task.is_some_and(|task| !task.action_command.to_string_lossy().contains('"')),
                "{execute}"
            );
        }
        assert!(
            task_from_action(
                &context(),
                action("Other", r#""C:\Program Files\Demos\x.exe""#)
            )
            .is_none()
        );
    }

    #[test]
    fn a_task_with_two_programs_is_one_task() {
        let tasks = [
            action("Update", r"C:\Program Files\Demo\demo.exe"),
            action("UPDATE", r"C:\Program Files\Demo\second.exe"),
            action("Other", r"C:\Program Files\Demo\demo.exe"),
        ]
        .into_iter()
        .filter_map(|action| task_from_action(&context(), action))
        .collect();
        let names: Vec<String> = one_entry_per_task(tasks)
            .into_iter()
            .map(|task| task.task_name)
            .collect();
        assert_eq!(names, ["Update", "Other"]);
    }

    const SAMPLE_TASK_XML: &str = r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <Triggers><LogonTrigger><Enabled>true</Enabled></LogonTrigger></Triggers>
  <Principals>
    <Principal id="Author"><UserId>S-1-5-18</UserId><RunLevel>HighestAvailable</RunLevel></Principal>
  </Principals>
  <Actions Context="Author">
    <Exec>
      <Command>C:\Program Files\Demo\demo.exe</Command>
      <Arguments>/run</Arguments>
      <WorkingDirectory>C:\Program Files\Demo</WorkingDirectory>
    </Exec>
  </Actions>
</Task>"#;

    #[test]
    fn parses_task_xml_fields_ignoring_namespace() {
        let Some(parsed) = parse_task_xml(SAMPLE_TASK_XML) else {
            panic!("task XML should parse");
        };
        assert_eq!(
            parsed.execute.as_deref(),
            Some(r"C:\Program Files\Demo\demo.exe")
        );
        assert_eq!(parsed.arguments.as_deref(), Some("/run"));
        assert_eq!(
            parsed.working_directory.as_deref(),
            Some(r"C:\Program Files\Demo")
        );
        assert_eq!(parsed.run_level.as_deref(), Some("HighestAvailable"));
        assert_eq!(parsed.account.as_deref(), Some("S-1-5-18"));
        assert_eq!(parsed.trigger_summary.as_deref(), Some("LogonTrigger"));
    }

    #[test]
    fn derives_task_name_and_path_from_file_location() {
        let root = Path::new(r"E:\Windows\System32\Tasks");
        assert_eq!(
            task_name_and_path(root, Path::new(r"E:\Windows\System32\Tasks\Vendor\Update")),
            ("Update".to_owned(), r"\Vendor\".to_owned())
        );
        assert_eq!(
            task_name_and_path(root, Path::new(r"E:\Windows\System32\Tasks\GoogleUpdate")),
            ("GoogleUpdate".to_owned(), "\\".to_owned())
        );
    }

    #[test]
    fn expands_systemroot_marker_against_the_scanned_windows_dir() {
        let windows = Path::new(r"E:\Windows");
        assert_eq!(
            expand_task_path(r"%SystemRoot%\System32\svc.exe", windows),
            PathBuf::from(r"E:\Windows\System32\svc.exe")
        );
        // A plain drive path is left for the context to re-root.
        assert_eq!(
            expand_task_path(r"C:\Program Files\Demo\demo.exe", windows),
            PathBuf::from(r"C:\Program Files\Demo\demo.exe")
        );
    }

    #[test]
    fn classifies_highest_or_system_tasks_as_admin_only() {
        assert!(task_requires_admin(Some("Highest"), Some("user")));
        assert!(task_requires_admin(Some("1"), Some("user")));
        assert!(task_requires_admin(Some("HighestAvailable"), Some("user")));
        assert!(task_requires_admin(Some("Limited"), Some("SYSTEM")));
        assert!(!task_requires_admin(Some("Limited"), Some("DOMAIN\\user")));
        assert!(!task_requires_admin(None, None));
    }

    #[test]
    fn parses_numeric_powershell_task_fields() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let actions = parse_json_array::<PowerShellTaskAction>(
            r#"[{
                "TaskName": "Example",
                "TaskPath": "\\WinAppMigrator\\",
                "Execute": "C:\\App\\app.exe",
                "RunLevel": 1,
                "Account": "SYSTEM"
            }]"#,
        )?;
        let Some(action) = actions.into_iter().next() else {
            return Err(std::io::Error::other("missing task action").into());
        };
        let Some(task) = task_from_action(
            &TaskScanContext {
                executable_path: PathBuf::from(r"C:\App\app.exe"),
                install_location: None,
            },
            action,
        ) else {
            return Err(std::io::Error::other("task should match executable path").into());
        };

        assert_eq!(task.task_name, "Example");
        assert_eq!(task.run_level.as_deref(), Some("1"));
        assert!(task.requires_admin);
        Ok(())
    }
}
