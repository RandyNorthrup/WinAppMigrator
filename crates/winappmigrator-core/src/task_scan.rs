use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer};

use crate::models::{ScheduledTaskEntry, Warning, WarningSeverity};
use crate::path_utils::hidden_command;

#[derive(Debug, Clone)]
pub struct TaskScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
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

pub fn scan_scheduled_tasks(context: &TaskScanContext) -> (Vec<ScheduledTaskEntry>, Vec<Warning>) {
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
    (tasks, warnings)
}

fn task_from_action(
    context: &TaskScanContext,
    action: PowerShellTaskAction,
) -> Option<ScheduledTaskEntry> {
    let command = action.execute?;
    let command_path = PathBuf::from(expand_env_markers(&command));
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
        restore_instructions:
            "Restore uses Register-ScheduledTask with the archived XML definition.".to_owned(),
        definition_xml: action.definition_xml,
    })
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

fn path_matches(context: &TaskScanContext, candidate: &Path) -> bool {
    candidate
        .to_string_lossy()
        .eq_ignore_ascii_case(&context.executable_path.to_string_lossy())
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| candidate.starts_with(root))
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
