use std::collections::HashSet;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::models::{ScheduledTaskEntry, Warning, WarningSeverity};
use crate::path_utils::{
    POWERSHELL_IMPORT_TASKS, POWERSHELL_UTF8_OUTPUT, escape_powershell_single, hidden_command,
    path_is_strictly_inside,
};
use crate::restore_policy::{is_system_account, node_text};
use crate::system_context::SystemContext;

/// Where the Task Scheduler keeps each task, as a file named for it, under the Windows folder.
const TASKS_SUBDIR: &str = r"System32\Tasks";
const TASK_RESTORE_INSTRUCTIONS: &str =
    "Restore uses Register-ScheduledTask with the archived XML definition.";
/// Far above any real definition. Anything larger is not a task, and on a drive from elsewhere it
/// could be made big enough to run this out of memory, which is why there is a limit on how many
/// one app can have kept too.
const MAX_TASK_FILE_BYTES: u64 = 1024 * 1024;
const MAX_TASKS_OF_AN_APP: usize = 256;
const UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
const UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
/// Where a definition names a variable-less Windows folder, as a driver's path can.
const SYSTEM_ROOT_MARKER: &str = r"\SystemRoot\";
const SYSTEM_ROOT_VARIABLE: &str = r"%SystemRoot%\";

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

/// The app's paths as its own PC calls them, which is what a task's definition holds.
#[derive(Debug, Clone)]
pub struct TaskScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

/// One task's file, read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFile {
    pub task_name: String,
    pub task_path: String,
    /// The program actions, in order.
    pub actions: Vec<TaskAction>,
    pub trigger_summary: Option<String>,
    pub run_level: Option<String>,
    pub account: Option<String>,
    pub definition_xml: String,
}

/// One program a task runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAction {
    pub command: String,
    pub arguments: Option<String>,
    pub working_directory: Option<String>,
}

/// The scheduled tasks that run the app. Read from each task's own file, whichever PC it is, with
/// no copy of Windows asked about it: the Task Scheduler that could be asked only knows this PC.
pub fn scan_scheduled_tasks(
    context: &TaskScanContext,
    system: &SystemContext,
) -> (Vec<ScheduledTaskEntry>, Vec<Warning>) {
    let tasks_root = system.windows_dir().join(TASKS_SUBDIR);
    if !system.may_read(&tasks_root) {
        return (
            Vec::new(),
            vec![Warning::new(
                "scheduled_tasks_unreadable",
                WarningSeverity::Warning,
                "The folder of scheduled tasks on this drive goes through a link, so its tasks \
                 were not read.",
                Some(tasks_root.display().to_string()),
            )],
        );
    }
    let expand = |text: &str| system.expand_source_variables(text);
    let windows_dir = system.logical_path(&system.windows_dir());
    let mut tasks = Vec::new();
    let mut left_out = Vec::new();
    let may_read = |path: &Path| system.may_read(path);
    let mut warnings = for_each_task_file(&tasks_root, &may_read, &mut |file| {
        let Some(task) = task_entry(context, &file, &expand, &windows_dir) else {
            return;
        };
        // Machine-wide, but run as one person, and told what to do by them.
        let someone_elses = task
            .account
            .as_deref()
            .is_some_and(|account| system.is_another_persons_account(account));
        if someone_elses || tasks.len() >= MAX_TASKS_OF_AN_APP {
            left_out.push(format!("{}{}", task.task_path, task.task_name));
        } else {
            tasks.push(task);
        }
    });
    warnings.extend(left_out.into_iter().map(|task| {
        Warning::new(
            "scheduled_task_left_out",
            WarningSeverity::Warning,
            format!(
                "The scheduled task {task} was left out: it runs as another account than the one \
                 whose settings are being included, or the app has too many to keep."
            ),
            Some(task),
        )
    }));
    tasks.sort_by(|left, right| {
        (left.task_path.as_str(), left.task_name.as_str())
            .cmp(&(right.task_path.as_str(), right.task_name.as_str()))
    });
    (one_entry_per_task(tasks), warnings)
}

/// Every task under a `Tasks` folder. One that cannot be read gives a warning, and takes no other
/// with it. Links are not followed, so a folder of tasks stays in itself, whatever is in it.
pub fn read_task_files(tasks_root: &Path) -> (Vec<TaskFile>, Vec<Warning>) {
    let mut files = Vec::new();
    let warnings = for_each_task_file(tasks_root, &|_| true, &mut |file| files.push(file));
    files.sort_by(|left, right| {
        (left.task_path.as_str(), left.task_name.as_str())
            .cmp(&(right.task_path.as_str(), right.task_name.as_str()))
    });
    (files, warnings)
}

/// Hand over each task as it is read, with nothing kept that is not wanted, however many there are.
fn for_each_task_file(
    tasks_root: &Path,
    may_read: &dyn Fn(&Path) -> bool,
    visit: &mut dyn FnMut(TaskFile),
) -> Vec<Warning> {
    let mut warnings = Vec::new();
    let entries = WalkDir::new(tasks_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| may_read(entry.path()));
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(Warning::new(
                    "scheduled_task_folder_unreadable",
                    WarningSeverity::Warning,
                    format!("Could not read part of the scheduled tasks: {error}"),
                    Some(tasks_root.display().to_string()),
                ));
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        match read_task_file(tasks_root, entry.path()) {
            Ok(file) => visit(file),
            Err(reason) => warnings.push(Warning::new(
                "scheduled_task_unreadable",
                WarningSeverity::Warning,
                format!(
                    "Could not read the scheduled task {}: {reason}. If it runs the app, it was \
                     not included.",
                    entry.path().display()
                ),
                Some(entry.path().display().to_string()),
            )),
        }
    }
    warnings
}

fn read_task_file(tasks_root: &Path, path: &Path) -> Result<TaskFile, String> {
    let size = std::fs::metadata(path)
        .map_err(|error| error.to_string())?
        .len();
    if size > MAX_TASK_FILE_BYTES {
        return Err(format!("it is {size} bytes, too large to be a task"));
    }
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let definition_xml = decode_task_xml(&bytes)?;
    let parsed = parse_task_xml(&definition_xml)?;
    let (task_name, task_path) = task_name_and_path(tasks_root, path);
    Ok(TaskFile {
        task_name,
        task_path,
        actions: parsed.actions,
        trigger_summary: parsed.trigger_summary,
        run_level: parsed.run_level,
        account: parsed.account,
        definition_xml,
    })
}

/// The first program action that runs the app makes it the app's task. A task can run several, and
/// it has only the one definition, so it is one task.
fn task_entry(
    context: &TaskScanContext,
    file: &TaskFile,
    expand: &dyn Fn(&str) -> String,
    windows_dir: &Path,
) -> Option<ScheduledTaskEntry> {
    let (action, command_path) = file.actions.iter().find_map(|action| {
        let command_path = task_path_of(&action.command, expand, windows_dir);
        path_matches(context, &command_path).then_some((action, command_path))
    })?;
    let requires_admin = task_requires_admin(file.run_level.as_deref(), file.account.as_deref());
    Some(ScheduledTaskEntry {
        task_name: file.task_name.clone(),
        task_path: file.task_path.clone(),
        action_command: command_path,
        arguments: action.arguments.clone(),
        working_directory: action
            .working_directory
            .as_deref()
            .map(|folder| task_path_of(folder, expand, windows_dir)),
        trigger_summary: file.trigger_summary.clone(),
        run_level: file.run_level.clone(),
        account: file.account.clone(),
        requires_admin,
        restore_instructions: TASK_RESTORE_INSTRUCTIONS.to_owned(),
        definition_xml: Some(file.definition_xml.clone()),
    })
}

/// Listed twice only if a case-sensitive folder or a stray copy made it so. The Task Scheduler
/// goes by name without case, and the second would find the first already there.
fn one_entry_per_task(mut tasks: Vec<ScheduledTaskEntry>) -> Vec<ScheduledTaskEntry> {
    let mut seen = HashSet::new();
    tasks.retain(|task| {
        seen.insert(format!("{}{}", task.task_path, task.task_name).to_ascii_lowercase())
    });
    tasks
}

#[derive(Default)]
struct ParsedTask {
    actions: Vec<TaskAction>,
    trigger_summary: Option<String>,
    run_level: Option<String>,
    account: Option<String>,
}

/// Extract the fields we need from a Task Scheduler XML definition. Namespaces
/// are ignored (roxmltree exposes local tag names), so both namespaced and bare
/// documents parse.
fn parse_task_xml(xml: &str) -> Result<ParsedTask, String> {
    let document = roxmltree::Document::parse(xml).map_err(|error| error.to_string())?;
    let mut parsed = ParsedTask::default();
    for node in document.descendants().filter(roxmltree::Node::is_element) {
        match node.tag_name().name() {
            "Exec" => {
                let part = |name: &str| {
                    node.children()
                        .find(|child| child.tag_name().name() == name)
                        .map(node_text)
                        .filter(|text| !text.is_empty())
                };
                if let Some(command) = part("Command") {
                    parsed.actions.push(TaskAction {
                        command,
                        arguments: part("Arguments"),
                        working_directory: part("WorkingDirectory"),
                    });
                }
            }
            "RunLevel" if parsed.run_level.is_none() => {
                parsed.run_level = Some(node_text(node)).filter(|text| !text.is_empty());
            }
            // Who it runs as. A trigger can have a `UserId` too, which comes first, and is not.
            "UserId" | "GroupId"
                if parsed.account.is_none()
                    && node
                        .parent_element()
                        .is_some_and(|parent| parent.tag_name().name() == "Principal") =>
            {
                parsed.account = Some(node_text(node)).filter(|text| !text.is_empty());
            }
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
    Ok(parsed)
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

/// A path in a task's definition, as its PC meant it. Installers write a command both with quotes
/// and without, and they mean the same. Only a comparison: this never goes near the disk.
fn task_path_of(value: &str, expand: &dyn Fn(&str) -> String, windows_dir: &Path) -> PathBuf {
    let trimmed = value.trim().trim_matches('"');
    for marker in [SYSTEM_ROOT_MARKER, SYSTEM_ROOT_VARIABLE] {
        if let Some(rest) = trimmed
            .get(..marker.len())
            .filter(|head| head.eq_ignore_ascii_case(marker))
            .map(|_| &trimmed[marker.len()..])
        {
            return windows_dir.join(rest);
        }
    }
    PathBuf::from(expand(trimmed))
}

/// Turn a task's file into text. The Task Scheduler writes UTF-16 with a byte order mark, and
/// other programs that make these files do not always, so the first bytes say which it is. Text
/// that is not what it says it is, is not read as something else: a path in it would not be what
/// was written, and the definition is restored from this.
fn decode_task_xml(bytes: &[u8]) -> Result<String, String> {
    let utf16 = |rest: &[u8], from_bytes: fn([u8; 2]) -> u16| {
        let (pairs, odd_byte) = rest.as_chunks::<2>();
        if !odd_byte.is_empty() {
            return Err("it ends part-way through a character".to_owned());
        }
        let units: Vec<u16> = pairs.iter().map(|pair| from_bytes(*pair)).collect();
        String::from_utf16(&units).map_err(|error| format!("it is not valid UTF-16: {error}"))
    };
    if let Some(rest) = bytes.strip_prefix(&UTF16_LE_BOM) {
        return utf16(rest, u16::from_le_bytes);
    }
    if let Some(rest) = bytes.strip_prefix(&UTF16_BE_BOM) {
        return utf16(rest, u16::from_be_bytes);
    }
    let rest = bytes.strip_prefix(&UTF8_BOM).unwrap_or(bytes);
    // No mark, and every other byte empty, as `<` and `?` are in UTF-16.
    match rest {
        [b'<', 0, ..] => return utf16(rest, u16::from_le_bytes),
        [0, b'<', ..] => return utf16(rest, u16::from_be_bytes),
        _ => {}
    }
    std::str::from_utf8(rest)
        .map(ToOwned::to_owned)
        .map_err(|error| format!("it is not valid UTF-8 or UTF-16: {error}"))
}

fn task_requires_admin(run_level: Option<&str>, account: Option<&str>) -> bool {
    run_level.is_some_and(|value| {
        let normalized = value.trim().to_ascii_lowercase();
        normalized == "1" || normalized.contains("highest")
    }) || account.is_some_and(is_system_account)
}

fn path_matches(context: &TaskScanContext, candidate: &Path) -> bool {
    candidate
        .to_string_lossy()
        .eq_ignore_ascii_case(&context.executable_path.to_string_lossy())
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_strictly_inside(candidate, root))
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

    fn windows() -> PathBuf {
        PathBuf::from(r"C:\Windows")
    }

    fn no_variables(text: &str) -> String {
        text.to_owned()
    }

    fn task_with(name: &str, commands: &[&str]) -> TaskFile {
        TaskFile {
            task_name: name.to_owned(),
            task_path: r"\Demo\".to_owned(),
            actions: commands
                .iter()
                .map(|command| TaskAction {
                    command: (*command).to_owned(),
                    arguments: None,
                    working_directory: None,
                })
                .collect(),
            trigger_summary: None,
            run_level: None,
            account: None,
            definition_xml: String::new(),
        }
    }

    #[test]
    fn who_a_task_runs_as_is_its_principal_and_not_whoever_triggers_it() {
        let parsed = parse_task_xml(
            r#"<Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
                 <Triggers><LogonTrigger><UserId>PC\alice</UserId></LogonTrigger></Triggers>
                 <Principals><Principal id="Author"><UserId>S-1-5-21-1-2-3-1002</UserId></Principal></Principals>
                 <Actions><Exec><Command>x.exe</Command></Exec></Actions>
               </Task>"#,
        );
        assert_eq!(
            parsed.ok().and_then(|parsed| parsed.account).as_deref(),
            Some("S-1-5-21-1-2-3-1002")
        );
    }

    fn entry_for(file: TaskFile) -> Option<ScheduledTaskEntry> {
        task_entry(&context(), &file, &no_variables, &windows())
    }

    #[test]
    fn a_quoted_command_is_the_same_program_as_a_bare_one() {
        for execute in [
            r#""C:\Program Files\Demo\demo.exe""#,
            r"C:\Program Files\Demo\demo.exe",
            r#"  "C:\Program Files\Demo\demo.exe"  "#,
            r"c:\program files\demo\bin\other.exe",
        ] {
            let task = entry_for(task_with("Update", &[execute]));
            assert!(
                task.is_some_and(|task| !task.action_command.to_string_lossy().contains('"')),
                "{execute}"
            );
        }
        assert!(entry_for(task_with("Other", &[r#""C:\Program Files\Demos\x.exe""#])).is_none());
        assert!(
            entry_for(task_with(
                "Climbs out",
                &[r"C:\Program Files\Demo\..\..\Windows\x.exe"]
            ))
            .is_none(),
            "a parent step does not make a path the app's"
        );
    }

    #[test]
    fn a_task_with_two_programs_is_one_task_and_either_can_be_the_apps() {
        let tasks: Vec<ScheduledTaskEntry> = [
            task_with(
                "Update",
                &[r"C:\Windows\helper.exe", r"C:\Program Files\Demo\demo.exe"],
            ),
            task_with("UPDATE", &[r"C:\Program Files\Demo\second.exe"]),
            task_with("Other", &[r"C:\Program Files\Demo\demo.exe"]),
            task_with("Neither", &[r"C:\Windows\helper.exe"]),
        ]
        .into_iter()
        .filter_map(entry_for)
        .collect();
        let tasks = one_entry_per_task(tasks);
        let names: Vec<&str> = tasks.iter().map(|task| task.task_name.as_str()).collect();
        assert_eq!(names, ["Update", "Other"]);
        assert_eq!(
            tasks[0].action_command,
            PathBuf::from(r"C:\Program Files\Demo\demo.exe"),
            "the program that matched is the one it is known by"
        );
    }

    const SAMPLE_TASK_XML: &str = r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <Triggers><LogonTrigger><Enabled>true</Enabled></LogonTrigger></Triggers>
  <Principals>
    <Principal id="Author"><UserId>S-1-5-18</UserId><RunLevel>HighestAvailable</RunLevel></Principal>
  </Principals>
  <Actions Context="Author">
    <Exec>
      <Command>C:\Windows\helper.exe</Command>
    </Exec>
    <Exec>
      <Command>C:\Program Files\Demo\demo.exe</Command>
      <Arguments>/run</Arguments>
      <WorkingDirectory>C:\Program Files\Demo</WorkingDirectory>
    </Exec>
    <ComHandler><ClassId>{00000000-0000-0000-0000-000000000000}</ClassId></ComHandler>
  </Actions>
</Task>"#;

    #[test]
    fn parses_task_xml_fields_ignoring_namespace() -> Result<(), String> {
        let parsed = parse_task_xml(SAMPLE_TASK_XML)?;
        assert_eq!(
            parsed.actions,
            vec![
                TaskAction {
                    command: r"C:\Windows\helper.exe".to_owned(),
                    arguments: None,
                    working_directory: None,
                },
                TaskAction {
                    command: r"C:\Program Files\Demo\demo.exe".to_owned(),
                    arguments: Some("/run".to_owned()),
                    working_directory: Some(r"C:\Program Files\Demo".to_owned()),
                },
            ]
        );
        assert_eq!(parsed.run_level.as_deref(), Some("HighestAvailable"));
        assert_eq!(parsed.account.as_deref(), Some("S-1-5-18"));
        assert_eq!(parsed.trigger_summary.as_deref(), Some("LogonTrigger"));
        Ok(())
    }

    #[test]
    fn a_comment_in_a_command_does_not_hide_what_the_task_scheduler_reads() -> Result<(), String> {
        let parsed = parse_task_xml(
            r"<Task><Actions><Exec><Command>C:\Program Files\De<!-- x -->mo\demo.exe</Command></Exec></Actions></Task>",
        )?;
        assert_eq!(parsed.actions[0].command, r"C:\Program Files\Demo\demo.exe");
        Ok(())
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
    fn a_path_in_a_definition_is_read_as_its_own_pc_meant_it() {
        let expand = |text: &str| text.replace("%ProgramFiles%", r"C:\Program Files");
        assert_eq!(
            task_path_of(r"%SystemRoot%\System32\svc.exe", &expand, &windows()),
            PathBuf::from(r"C:\Windows\System32\svc.exe")
        );
        assert_eq!(
            task_path_of(r"\SystemRoot\System32\svc.exe", &expand, &windows()),
            PathBuf::from(r"C:\Windows\System32\svc.exe")
        );
        assert_eq!(
            task_path_of(r#""%ProgramFiles%\Demo\demo.exe""#, &expand, &windows()),
            PathBuf::from(r"C:\Program Files\Demo\demo.exe")
        );
        assert_eq!(
            task_path_of(r"C:\Program Files\Demo\demo.exe", &expand, &windows()),
            PathBuf::from(r"C:\Program Files\Demo\demo.exe")
        );
    }

    #[test]
    fn classifies_highest_or_system_tasks_as_admin_only() {
        assert!(task_requires_admin(Some("Highest"), Some("user")));
        assert!(task_requires_admin(Some("1"), Some("user")));
        assert!(task_requires_admin(Some("HighestAvailable"), Some("user")));
        assert!(task_requires_admin(Some("Limited"), Some("SYSTEM")));
        assert!(task_requires_admin(
            Some("LeastPrivilege"),
            Some("S-1-5-18")
        ));
        assert!(!task_requires_admin(Some("Limited"), Some("DOMAIN\\user")));
        assert!(!task_requires_admin(None, None));
    }

    const NON_ASCII_TASK: &str = r"<?xml version='1.0'?><Task><Actions><Exec><Command>C:\Größe 日本\a.exe</Command></Exec></Actions></Task>";
    const NON_ASCII_COMMAND: &str = r"C:\Größe 日本\a.exe";

    fn with_bom(bom: &[u8], units: impl Iterator<Item = [u8; 2]>) -> Vec<u8> {
        let mut bytes = bom.to_vec();
        for unit in units {
            bytes.extend_from_slice(&unit);
        }
        bytes
    }

    #[test]
    fn every_encoding_a_task_comes_in_reads_the_same() -> Result<(), String> {
        let units = || NON_ASCII_TASK.encode_utf16();
        for (label, bytes) in [
            (
                "UTF-16 with a mark, as the Task Scheduler writes",
                with_bom(&UTF16_LE_BOM, units().map(u16::to_le_bytes)),
            ),
            (
                "UTF-16 big-endian with a mark",
                with_bom(&UTF16_BE_BOM, units().map(u16::to_be_bytes)),
            ),
            (
                "UTF-16 with no mark",
                with_bom(&[], units().map(u16::to_le_bytes)),
            ),
            (
                "UTF-16 big-endian with no mark",
                with_bom(&[], units().map(u16::to_be_bytes)),
            ),
            (
                "UTF-8 with a mark",
                [&UTF8_BOM[..], NON_ASCII_TASK.as_bytes()].concat(),
            ),
            ("UTF-8", NON_ASCII_TASK.as_bytes().to_vec()),
        ] {
            let text = decode_task_xml(&bytes)?;
            assert_eq!(text, NON_ASCII_TASK, "{label}");
            assert_eq!(
                parse_task_xml(&text)?.actions[0].command,
                NON_ASCII_COMMAND,
                "{label}"
            );
        }
        Ok(())
    }

    #[test]
    fn text_that_is_not_what_it_says_is_not_read_as_something_else() {
        let mut odd = with_bom(
            &UTF16_LE_BOM,
            "<Task/>".encode_utf16().map(u16::to_le_bytes),
        );
        odd.push(0);
        assert!(decode_task_xml(&odd).is_err());
        // A surrogate with no other half.
        let broken = with_bom(&UTF16_LE_BOM, [[0x00, 0xD8], [b'<', 0]].into_iter());
        assert!(decode_task_xml(&broken).is_err());
        // Latin-1, which is not UTF-8.
        assert!(decode_task_xml(b"<Task>Gr\xF6\xDFe</Task>").is_err());
        assert_eq!(decode_task_xml(b""), Ok(String::new()));
    }

    /// The whole of it, in a folder that stands in for a PC's: JSON through PowerShell lost every
    /// task over one that broke it, and no file here can do that.
    #[test]
    fn a_task_that_cannot_be_read_takes_no_other_with_it() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let root = work.path().join("Tasks");
        std::fs::create_dir_all(root.join(r"Vendor\Deep"))?;
        let write_utf16 = |path: PathBuf, text: &str| {
            std::fs::write(
                path,
                with_bom(&UTF16_LE_BOM, text.encode_utf16().map(u16::to_le_bytes)),
            )
        };
        write_utf16(root.join("Plain"), SAMPLE_TASK_XML)?;
        write_utf16(root.join(r"Vendor\Größe 日本"), NON_ASCII_TASK)?;
        write_utf16(
            root.join(r"Vendor\Deep\Newline in a value"),
            "<Task><Actions><Exec><Command>C:\\a.exe</Command><Arguments>--x \"y\"\r\n\u{1}?\
             </Arguments></Exec></Actions></Task>"
                .replace('\u{1}', "")
                .as_str(),
        )?;
        write_utf16(root.join("Not XML"), "this is not a task <")?;
        std::fs::write(root.join("Broken encoding"), b"\xFF\xFE\x00\xD8")?;
        std::fs::write(root.join("Not text"), b"\x80\x81\x82")?;
        std::fs::write(
            root.join("Too large"),
            vec![b' '; MAX_TASK_FILE_BYTES as usize + 1],
        )?;

        let (files, warnings) = read_task_files(&root);
        let names: Vec<String> = files
            .iter()
            .map(|file| format!("{}{}", file.task_path, file.task_name))
            .collect();
        assert_eq!(
            names,
            [
                r"\Plain",
                r"\Vendor\Größe 日本",
                r"\Vendor\Deep\Newline in a value"
            ]
        );
        assert_eq!(files[1].actions[0].command, NON_ASCII_COMMAND);
        assert_eq!(warnings.len(), 4, "{warnings:?}");
        for warning in &warnings {
            assert_eq!(warning.code, "scheduled_task_unreadable");
            assert_eq!(warning.severity, WarningSeverity::Warning);
            assert!(warning.source.is_some());
        }
        Ok(())
    }

    #[test]
    fn a_task_is_not_read_through_a_link() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let root = work.path().join("Tasks");
        let elsewhere = work.path().join("elsewhere");
        std::fs::create_dir_all(&root)?;
        std::fs::create_dir_all(&elsewhere)?;
        std::fs::write(elsewhere.join("Theirs"), SAMPLE_TASK_XML)?;
        std::fs::write(root.join("Own"), SAMPLE_TASK_XML)?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(root.join("Linked"))
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());
        let (files, warnings) = read_task_files(&root);
        let names: Vec<&str> = files.iter().map(|file| file.task_name.as_str()).collect();
        assert_eq!(names, ["Own"]);
        assert!(warnings.is_empty(), "{warnings:?}");
        Ok(())
    }
}
