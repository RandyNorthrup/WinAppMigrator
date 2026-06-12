use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::models::{ScheduledTaskEntry, Warning, WarningSeverity};

#[derive(Debug, Clone)]
pub struct TaskScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellTaskAction {
    task_name: Option<String>,
    task_path: Option<String>,
    execute: Option<String>,
    arguments: Option<String>,
    working_directory: Option<String>,
    trigger_summary: Option<String>,
    run_level: Option<String>,
    account: Option<String>,
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
      TaskName = $task.TaskName
      TaskPath = $task.TaskPath
      Execute = $action.Execute
      Arguments = $action.Arguments
      WorkingDirectory = $action.WorkingDirectory
      TriggerSummary = $triggerSummary
      RunLevel = $task.Principal.RunLevel
      Account = $task.Principal.UserId
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

    Some(ScheduledTaskEntry {
        task_name: action.task_name.unwrap_or_default(),
        task_path: action.task_path.unwrap_or_default(),
        action_command: command_path,
        arguments: action.arguments,
        working_directory: action.working_directory.map(PathBuf::from),
        trigger_summary: action.trigger_summary,
        run_level: action.run_level,
        account: action.account,
        restore_instructions:
            "Restore uses Register-ScheduledTask with the archived XML definition.".to_owned(),
        definition_xml: action.definition_xml,
    })
}

fn run_powershell(script: &str) -> std::io::Result<String> {
    let output = Command::new("powershell.exe")
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
