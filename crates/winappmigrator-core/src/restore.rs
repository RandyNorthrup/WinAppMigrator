use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use filetime::{FileTime, set_file_times};
use tempfile::TempDir;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN,
    REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD,
    REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};
use winreg::{RegKey, RegValue};

use crate::archive::extract_zip_archive;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{package_destination_path, safe_relative_path};
use crate::models::{
    EnvironmentScope, EnvironmentVariableEntry, PackageManifest, RegistryEntry, RegistryHive,
    RegistryValueKind, RestoreOptions, RestorePlan, RestoreResult, ScheduledTaskEntry,
    ServiceEntry, ShortcutEntry, Warning, WarningSeverity,
};
use crate::package::read_manifest;
use crate::privilege::{is_elevated, target_requires_admin};

const RESTORE_LOG_JSON: &str = "restore_log.json";
const RESTORE_LOG_MARKDOWN: &str = "restore_log.md";
const ZIP_EXTENSION: &str = "zip";
const POWERSHELL_EXE: &str = "powershell.exe";
const SC_EXE: &str = "sc.exe";

pub fn plan_restore(options: &RestoreOptions) -> Result<RestorePlan> {
    let manifest = read_manifest(&options.package_root)?;
    let mut warnings = manifest.warnings.clone();
    let mut planned_file_writes = Vec::new();

    for file in manifest
        .file_inventory
        .into_iter()
        .filter(|entry| entry.should_copy)
    {
        if file.package_relative_path.is_none() {
            warnings.push(Warning::new(
                "restore_package_path_missing",
                WarningSeverity::Warning,
                "A copyable file has no package relative path.",
                Some(file.original_path.display().to_string()),
            ));
            continue;
        }
        planned_file_writes.push(file);
    }

    Ok(RestorePlan {
        package_root: options.package_root.clone(),
        target_root: options.target_root.clone(),
        dry_run: !options.apply || options.dry_run,
        planned_file_writes,
        planned_registry_writes: manifest.registry_keys_exported,
        planned_service_changes: manifest.services_detected,
        planned_task_changes: manifest.scheduled_tasks_detected,
        planned_shortcut_changes: manifest.shortcuts_detected,
        planned_environment_changes: manifest.environment_variables_detected,
        warnings,
    })
}

pub fn restore_package(options: &RestoreOptions) -> Result<RestoreResult> {
    if options.apply && options.dry_run {
        return Err(WinAppMigratorError::InvalidInput(
            "--apply and --dry-run cannot both be supplied".to_owned(),
        ));
    }

    let prepared = PreparedPackage::new(options)?;
    let mut effective_options = options.clone();
    effective_options.package_root = prepared.package_root().to_path_buf();

    let manifest = read_manifest(&effective_options.package_root)?;
    let plan = plan_restore(&effective_options)?;
    if options.apply && restore_requires_admin(&plan) && !is_elevated() {
        return Err(WinAppMigratorError::InvalidInput(
            "restore plan includes admin-only registry, service, scheduled task, shortcut, or protected target changes"
                .to_owned(),
        ));
    }
    if options.apply && target_requires_admin(&options.target_root) && !is_elevated() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "target root requires elevated privileges: {}",
            options.target_root.display()
        )));
    }

    let path_rewrites = path_rewrites(&manifest, &options.target_root)?;
    let mut changes_logged = Vec::new();
    let mut warnings = plan.warnings.clone();
    let mut errors = Vec::new();

    restore_files(
        &effective_options,
        &plan,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    )?;
    restore_registry(
        &effective_options,
        &plan.planned_registry_writes,
        &path_rewrites,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_services(
        &effective_options,
        &manifest,
        &plan.planned_service_changes,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_scheduled_tasks(
        &effective_options,
        &plan.planned_task_changes,
        &path_rewrites,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_shortcuts(
        &effective_options,
        &manifest,
        &plan.planned_shortcut_changes,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_environment(
        &effective_options,
        &plan.planned_environment_changes,
        &path_rewrites,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );

    let restore_log_json = effective_options.package_root.join(RESTORE_LOG_JSON);
    let restore_log_markdown = effective_options.package_root.join(RESTORE_LOG_MARKDOWN);
    let result = RestoreResult {
        applied: options.apply,
        changes_logged,
        warnings,
        errors,
        restore_log_json: Some(restore_log_json.clone()),
        restore_log_markdown: Some(restore_log_markdown.clone()),
    };
    write_restore_logs(&restore_log_json, &restore_log_markdown, &result)?;
    Ok(result)
}

fn restore_files(
    options: &RestoreOptions,
    plan: &RestorePlan,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    for file in &plan.planned_file_writes {
        let Some(relative) = file.package_relative_path.as_ref() else {
            continue;
        };
        let source = package_destination_path(&options.package_root, relative)?;
        let destination = restore_destination(&options.target_root, relative)?;

        if destination.exists() && !options.force {
            let warning = Warning::new(
                "restore_destination_exists",
                WarningSeverity::Warning,
                "Destination exists; use --force to overwrite.",
                Some(destination.display().to_string()),
            );
            if options.apply {
                errors.push(warning);
            } else {
                warnings.push(warning);
            }
            continue;
        }

        if options.apply {
            copy_restore_file(&source, &destination)?;
            changes_logged.push(format!(
                "copied {} -> {}",
                source.display(),
                destination.display()
            ));
        } else {
            changes_logged.push(format!(
                "would copy {} -> {}",
                source.display(),
                destination.display()
            ));
        }
    }
    Ok(())
}

fn restore_registry(
    options: &RestoreOptions,
    entries: &[RegistryEntry],
    path_rewrites: &[(String, String)],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for entry in entries {
        if !options.apply {
            changes_logged.push(format!(
                "would restore registry {}\\{} {}",
                registry_hive_name(&entry.hive),
                entry.key_path,
                entry.value_name.as_deref().unwrap_or("(key)")
            ));
            continue;
        }

        match apply_registry_entry(entry, options.force, path_rewrites) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if !entries.is_empty() {
        warnings.push(Warning::new(
            "registry_restore_review_required",
            WarningSeverity::Info,
            "Registry values were restored from the archived structured data with known app paths rewritten to the target root.",
            None,
        ));
    }
}

fn restore_services(
    options: &RestoreOptions,
    manifest: &PackageManifest,
    services: &[ServiceEntry],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in services {
        let binary_path =
            mapped_original_path(manifest, &options.target_root, &service.binary_path)
                .unwrap_or_else(|| service.binary_path.clone());
        if !options.apply {
            changes_logged.push(format!(
                "would restore service {} -> {}",
                service.service_name,
                binary_path.display()
            ));
            continue;
        }
        match apply_service(service, &binary_path, options.force) {
            Ok(messages) => changes_logged.extend(messages),
            Err(warning) => errors.push(warning),
        }
        if service.account_name.as_ref().is_some_and(|account| {
            !matches!(
                account.to_ascii_lowercase().as_str(),
                "localsystem" | "nt authority\\localservice" | "nt authority\\networkservice"
            )
        }) {
            warnings.push(Warning::new(
                "service_account_secret_not_restored",
                WarningSeverity::Warning,
                "Service account credentials are never archived; review this service account after restore.",
                Some(service.service_name.clone()),
            ));
        }
    }
}

fn restore_scheduled_tasks(
    options: &RestoreOptions,
    tasks: &[ScheduledTaskEntry],
    path_rewrites: &[(String, String)],
    changes_logged: &mut Vec<String>,
    _warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for task in tasks {
        if !options.apply {
            changes_logged.push(format!(
                "would restore scheduled task {}{}",
                task.task_path, task.task_name
            ));
            continue;
        }
        match apply_scheduled_task(task, options.force, path_rewrites) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
}

fn restore_shortcuts(
    options: &RestoreOptions,
    manifest: &PackageManifest,
    shortcuts: &[ShortcutEntry],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in shortcuts {
        let target_path =
            mapped_original_path(manifest, &options.target_root, &shortcut.target_path)
                .unwrap_or_else(|| shortcut.target_path.clone());
        let working_directory = shortcut.working_directory.as_ref().map(|working| {
            mapped_original_path(manifest, &options.target_root, working)
                .unwrap_or_else(|| working.clone())
        });
        let icon_path = shortcut.icon_path.as_ref().map(|icon| {
            mapped_original_path(manifest, &options.target_root, icon)
                .unwrap_or_else(|| icon.clone())
        });

        if !options.apply {
            changes_logged.push(format!(
                "would restore shortcut {} -> {}",
                shortcut.original_lnk_path.display(),
                target_path.display()
            ));
            continue;
        }
        if shortcut.original_lnk_path.exists() && !options.force {
            errors.push(Warning::new(
                "shortcut_destination_exists",
                WarningSeverity::Warning,
                "Shortcut exists; use --force to overwrite.",
                Some(shortcut.original_lnk_path.display().to_string()),
            ));
            continue;
        }
        match apply_shortcut(
            shortcut,
            &target_path,
            working_directory.as_deref(),
            icon_path.as_deref(),
        ) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if shortcuts.iter().any(shortcut_requires_admin) {
        warnings.push(Warning::new(
            "shortcut_admin_scope",
            WarningSeverity::Info,
            "One or more restored shortcuts target a public desktop or common Start Menu location.",
            None,
        ));
    }
}

fn restore_environment(
    options: &RestoreOptions,
    environment_variables: &[EnvironmentVariableEntry],
    path_rewrites: &[(String, String)],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for variable in environment_variables {
        if !options.apply {
            changes_logged.push(format!(
                "would restore {:?} environment variable {}",
                variable.scope, variable.name
            ));
            continue;
        }
        match apply_environment_variable(variable, options.force, path_rewrites) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if !environment_variables.is_empty() {
        warnings.push(Warning::new(
            "environment_restore_session_refresh_required",
            WarningSeverity::Info,
            "Environment variable changes may require a new process, shell, or sign-in session to be observed.",
            None,
        ));
    }
}

fn apply_registry_entry(
    entry: &RegistryEntry,
    force: bool,
    path_rewrites: &[(String, String)],
) -> std::result::Result<String, Warning> {
    let root = registry_root(&entry.hive);
    let (key, _) = root.create_subkey(&entry.key_path).map_err(|error| {
        Warning::new(
            "registry_key_create_failed",
            WarningSeverity::Error,
            format!("Could not create registry key: {error}"),
            Some(format!(
                "{}\\{}",
                registry_hive_name(&entry.hive),
                entry.key_path
            )),
        )
    })?;

    if entry.value_name.is_none() && entry.value_kind == Some(RegistryValueKind::None) {
        return Ok(format!(
            "ensured registry key {}\\{}",
            registry_hive_name(&entry.hive),
            entry.key_path
        ));
    }

    let value_name = registry_value_name(entry.value_name.as_deref());
    if key.get_raw_value(value_name).is_ok() && !force {
        return Err(Warning::new(
            "registry_value_exists",
            WarningSeverity::Warning,
            "Registry value exists; use --force to overwrite.",
            Some(format!(
                "{}\\{}\\{}",
                registry_hive_name(&entry.hive),
                entry.key_path,
                entry.value_name.as_deref().unwrap_or("(Default)")
            )),
        ));
    }

    let value = registry_value_from_entry(entry, path_rewrites)?;
    key.set_raw_value(value_name, &value).map_err(|error| {
        Warning::new(
            "registry_value_write_failed",
            WarningSeverity::Error,
            format!("Could not write registry value: {error}"),
            Some(format!(
                "{}\\{}\\{}",
                registry_hive_name(&entry.hive),
                entry.key_path,
                entry.value_name.as_deref().unwrap_or("(Default)")
            )),
        )
    })?;
    Ok(format!(
        "restored registry {}\\{} {}",
        registry_hive_name(&entry.hive),
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    ))
}

fn apply_environment_variable(
    variable: &EnvironmentVariableEntry,
    force: bool,
    path_rewrites: &[(String, String)],
) -> std::result::Result<String, Warning> {
    let (root, key_path) = environment_registry_target(&variable.scope);
    let (key, _) = root.create_subkey(key_path).map_err(|error| {
        Warning::new(
            "environment_key_create_failed",
            WarningSeverity::Error,
            format!("Could not create environment registry key: {error}"),
            Some(format!("{:?}\\{}", variable.scope, variable.name)),
        )
    })?;
    if key.get_raw_value(&variable.name).is_ok() && !force {
        return Err(Warning::new(
            "environment_value_exists",
            WarningSeverity::Warning,
            "Environment variable exists; use --force to overwrite.",
            Some(format!("{:?}\\{}", variable.scope, variable.name)),
        ));
    }

    let rewritten = rewrite_known_paths(&variable.value, path_rewrites);
    let value = RegValue {
        bytes: Cow::Owned(encode_utf16_nul(&rewritten)),
        vtype: registry_raw_type(&variable.value_kind),
    };
    key.set_raw_value(&variable.name, &value).map_err(|error| {
        Warning::new(
            "environment_value_write_failed",
            WarningSeverity::Error,
            format!("Could not write environment variable: {error}"),
            Some(format!("{:?}\\{}", variable.scope, variable.name)),
        )
    })?;
    Ok(format!(
        "restored {:?} environment variable {}",
        variable.scope, variable.name
    ))
}

fn registry_value_from_entry(
    entry: &RegistryEntry,
    path_rewrites: &[(String, String)],
) -> std::result::Result<RegValue<'static>, Warning> {
    let kind = entry.value_kind.as_ref().ok_or_else(|| {
        registry_warning(
            "registry_value_kind_missing",
            entry,
            "Registry value kind is missing.",
        )
    })?;
    match kind {
        RegistryValueKind::String | RegistryValueKind::ExpandString => {
            let rewritten = rewrite_known_paths(
                entry.value_data.as_deref().unwrap_or_default(),
                path_rewrites,
            );
            Ok(RegValue {
                bytes: Cow::Owned(encode_utf16_nul(&rewritten)),
                vtype: registry_raw_type(kind),
            })
        }
        RegistryValueKind::None => Ok(RegValue {
            bytes: Cow::Owned(Vec::new()),
            vtype: REG_NONE,
        }),
        _ => {
            let bytes = hex_decode(entry.value_bytes_hex.as_deref().unwrap_or_default()).map_err(
                |message| registry_warning("registry_value_decode_failed", entry, message),
            )?;
            Ok(RegValue {
                bytes: Cow::Owned(bytes),
                vtype: registry_raw_type(kind),
            })
        }
    }
}

fn apply_service(
    service: &ServiceEntry,
    binary_path: &Path,
    force: bool,
) -> std::result::Result<Vec<String>, Warning> {
    let mut query = Command::new(SC_EXE);
    query.arg("query").arg(&service.service_name);
    let exists = command_success(&mut query);
    if exists && !force {
        return Err(Warning::new(
            "service_exists",
            WarningSeverity::Warning,
            "Service exists; use --force to reconfigure it.",
            Some(service.service_name.clone()),
        ));
    }

    let mut messages = Vec::new();
    let binary = quote_path(binary_path);
    if exists {
        let mut command = Command::new(SC_EXE);
        command
            .arg("config")
            .arg(&service.service_name)
            .arg("binPath=")
            .arg(&binary);
        run_checked(&mut command, "service_config_failed", &service.service_name)?;
        messages.push(format!("configured service {}", service.service_name));
    } else {
        let mut command = Command::new(SC_EXE);
        command
            .arg("create")
            .arg(&service.service_name)
            .arg("binPath=")
            .arg(&binary);
        if let Some(start_type) = service.start_type.as_deref().and_then(sc_start_type) {
            command.arg("start=").arg(start_type);
        }
        run_checked(&mut command, "service_create_failed", &service.service_name)?;
        messages.push(format!("created service {}", service.service_name));
    }

    if let Some(description) = service.description.as_deref() {
        let mut command = Command::new(SC_EXE);
        command
            .arg("description")
            .arg(&service.service_name)
            .arg(description);
        run_checked(
            &mut command,
            "service_description_failed",
            &service.service_name,
        )?;
    }
    if !service.dependencies.is_empty() {
        let dependencies = service.dependencies.join("/");
        let mut command = Command::new(SC_EXE);
        command
            .arg("config")
            .arg(&service.service_name)
            .arg("depend=")
            .arg(&dependencies);
        run_checked(
            &mut command,
            "service_dependencies_failed",
            &service.service_name,
        )?;
    }

    Ok(messages)
}

fn apply_scheduled_task(
    task: &ScheduledTaskEntry,
    force: bool,
    path_rewrites: &[(String, String)],
) -> std::result::Result<String, Warning> {
    let Some(xml) = task.definition_xml.as_deref() else {
        return Err(Warning::new(
            "scheduled_task_xml_missing",
            WarningSeverity::Error,
            "Scheduled task XML definition was not archived; exact restore is unavailable.",
            Some(format!("{}{}", task.task_path, task.task_name)),
        ));
    };
    if scheduled_task_exists(task)? && !force {
        return Err(Warning::new(
            "scheduled_task_exists",
            WarningSeverity::Warning,
            "Scheduled task exists; use --force to overwrite.",
            Some(format!("{}{}", task.task_path, task.task_name)),
        ));
    }

    let rewritten_xml = rewrite_known_paths(xml, path_rewrites);
    let xml_path = std::env::temp_dir().join(format!(
        "winappmigrator-task-{}-{}.xml",
        std::process::id(),
        sanitize_filename(&task.task_name)
    ));
    fs::write(&xml_path, rewritten_xml)
        .map_err(|error| warning_io("scheduled_task_xml_write_failed", &xml_path, error))?;
    let force_flag = if force { "$true" } else { "$false" };
    let script = format!(
        "$xml = Get-Content -Raw -LiteralPath '{}'; Register-ScheduledTask -TaskName '{}' -TaskPath '{}' -Xml $xml -Force:{force_flag} | Out-Null",
        escape_ps_single(xml_path.display().to_string().as_str()),
        escape_ps_single(&task.task_name),
        escape_ps_single(&task.task_path),
    );
    let result = run_powershell_script(&script).map(|_| {
        format!(
            "restored scheduled task {}{}",
            task.task_path, task.task_name
        )
    });
    let _ = fs::remove_file(&xml_path);
    result
}

fn apply_shortcut(
    shortcut: &ShortcutEntry,
    target_path: &Path,
    working_directory: Option<&Path>,
    icon_path: Option<&Path>,
) -> std::result::Result<String, Warning> {
    if let Some(parent) = shortcut.original_lnk_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| warning_io("shortcut_parent_create_failed", parent, error))?;
    }
    let mut script = format!(
        "$shell = New-Object -ComObject WScript.Shell; $shortcut = $shell.CreateShortcut('{}'); $shortcut.TargetPath = '{}';",
        escape_ps_single(shortcut.original_lnk_path.display().to_string().as_str()),
        escape_ps_single(target_path.display().to_string().as_str()),
    );
    if let Some(arguments) = shortcut.arguments.as_deref() {
        script.push_str(&format!(
            " $shortcut.Arguments = '{}';",
            escape_ps_single(arguments)
        ));
    }
    if let Some(working_directory) = working_directory {
        script.push_str(&format!(
            " $shortcut.WorkingDirectory = '{}';",
            escape_ps_single(working_directory.display().to_string().as_str())
        ));
    }
    if let Some(icon_path) = icon_path {
        script.push_str(&format!(
            " $shortcut.IconLocation = '{}';",
            escape_ps_single(icon_path.display().to_string().as_str())
        ));
    }
    script.push_str(" $shortcut.Save();");
    run_powershell_script(&script)
        .map(|_| format!("restored shortcut {}", shortcut.original_lnk_path.display()))
}

fn restore_destination(target_root: &Path, package_relative_path: &Path) -> Result<PathBuf> {
    let relative = safe_relative_path(package_relative_path);
    let without_files = relative.strip_prefix("files").unwrap_or(&relative);
    let destination = target_root.join(without_files);
    crate::file_discovery::ensure_within_root(target_root, &destination)?;
    Ok(destination)
}

fn copy_restore_file(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    fs::copy(source, destination).map_err(|error| WinAppMigratorError::io(destination, error))?;
    let metadata = fs::metadata(source).map_err(|error| WinAppMigratorError::io(source, error))?;
    fs::set_permissions(destination, metadata.permissions())
        .map_err(|error| WinAppMigratorError::io(destination, error))?;
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    set_file_times(destination, accessed, modified)
        .map_err(|error| WinAppMigratorError::io(destination, error))?;
    Ok(())
}

fn write_restore_logs(
    json_path: &Path,
    markdown_path: &Path,
    result: &RestoreResult,
) -> Result<()> {
    let json = serde_json::to_vec_pretty(result)?;
    fs::write(json_path, json).map_err(|error| WinAppMigratorError::io(json_path, error))?;

    let mut markdown = String::new();
    markdown.push_str("# WinAppMigrator Restore Log\n\n");
    markdown.push_str(&format!("- Applied: {}\n", result.applied));
    markdown.push_str("\n## Changes\n");
    for change in &result.changes_logged {
        markdown.push_str(&format!("- {change}\n"));
    }
    markdown.push_str("\n## Warnings\n");
    for warning in &result.warnings {
        markdown.push_str(&format!(
            "- {:?} {}: {}\n",
            warning.severity, warning.code, warning.message
        ));
    }
    markdown.push_str("\n## Errors\n");
    for error in &result.errors {
        markdown.push_str(&format!(
            "- {:?} {}: {}\n",
            error.severity, error.code, error.message
        ));
    }
    fs::write(markdown_path, markdown)
        .map_err(|error| WinAppMigratorError::io(markdown_path, error))
}

fn restore_requires_admin(plan: &RestorePlan) -> bool {
    plan.planned_registry_writes
        .iter()
        .any(|entry| entry.requires_admin)
        || !plan.planned_service_changes.is_empty()
        || !plan.planned_task_changes.is_empty()
        || plan
            .planned_shortcut_changes
            .iter()
            .any(shortcut_requires_admin)
        || plan
            .planned_environment_changes
            .iter()
            .any(|variable| variable.requires_admin)
}

fn shortcut_requires_admin(shortcut: &ShortcutEntry) -> bool {
    let path = shortcut
        .original_lnk_path
        .to_string_lossy()
        .to_ascii_lowercase();
    path.contains(r"\programdata\microsoft\windows\start menu")
        || path.contains(r"\users\public\desktop")
}

fn path_rewrites(manifest: &PackageManifest, target_root: &Path) -> Result<Vec<(String, String)>> {
    let mut rewrites = Vec::new();
    for file in &manifest.file_inventory {
        if !file.should_copy {
            continue;
        }
        let Some(relative) = file.package_relative_path.as_ref() else {
            continue;
        };
        let destination = restore_destination(target_root, relative)?;
        rewrites.push((
            file.original_path.to_string_lossy().to_string(),
            destination.to_string_lossy().to_string(),
        ));
    }
    if let Some(source_root) = manifest.main_executable_path.parent() {
        rewrites.push((
            source_root.to_string_lossy().to_string(),
            target_root.to_string_lossy().to_string(),
        ));
    }
    rewrites.sort_by_key(|entry| std::cmp::Reverse(entry.0.len()));
    Ok(rewrites)
}

fn mapped_original_path(
    manifest: &PackageManifest,
    target_root: &Path,
    original: &Path,
) -> Option<PathBuf> {
    manifest
        .file_inventory
        .iter()
        .find(|file| {
            file.should_copy
                && file
                    .original_path
                    .to_string_lossy()
                    .eq_ignore_ascii_case(&original.to_string_lossy())
        })
        .and_then(|file| file.package_relative_path.as_ref())
        .and_then(|relative| restore_destination(target_root, relative).ok())
}

fn rewrite_known_paths(value: &str, rewrites: &[(String, String)]) -> String {
    let mut rewritten = value.to_owned();
    for (source, destination) in rewrites {
        rewritten = replace_ignore_ascii_case(&rewritten, source, destination);
    }
    rewritten
}

fn replace_ignore_ascii_case(haystack: &str, needle: &str, replacement: &str) -> String {
    if needle.is_empty() {
        return haystack.to_owned();
    }
    let lower_haystack = haystack.to_ascii_lowercase();
    let lower_needle = needle.to_ascii_lowercase();
    let mut result = String::new();
    let mut cursor = 0_usize;
    while let Some(found) = lower_haystack[cursor..].find(&lower_needle) {
        let absolute = cursor + found;
        result.push_str(&haystack[cursor..absolute]);
        result.push_str(replacement);
        cursor = absolute + needle.len();
    }
    result.push_str(&haystack[cursor..]);
    result
}

fn registry_root(hive: &RegistryHive) -> RegKey {
    match hive {
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
    }
}

fn environment_registry_target(scope: &EnvironmentScope) -> (RegKey, &'static str) {
    match scope {
        EnvironmentScope::User => (RegKey::predef(HKEY_CURRENT_USER), "Environment"),
        EnvironmentScope::Machine => (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
        ),
    }
}

fn registry_hive_name(hive: &RegistryHive) -> &'static str {
    match hive {
        RegistryHive::LocalMachine => "HKLM",
        RegistryHive::CurrentUser => "HKCU",
    }
}

fn registry_value_name(value_name: Option<&str>) -> &str {
    match value_name {
        None | Some("(Default)") => "",
        Some(value) => value,
    }
}

fn registry_raw_type(kind: &RegistryValueKind) -> RegType {
    match kind {
        RegistryValueKind::None => REG_NONE,
        RegistryValueKind::String => REG_SZ,
        RegistryValueKind::ExpandString => REG_EXPAND_SZ,
        RegistryValueKind::Binary => REG_BINARY,
        RegistryValueKind::Dword => REG_DWORD,
        RegistryValueKind::DwordBigEndian => REG_DWORD_BIG_ENDIAN,
        RegistryValueKind::Link => REG_LINK,
        RegistryValueKind::MultiString => REG_MULTI_SZ,
        RegistryValueKind::ResourceList => REG_RESOURCE_LIST,
        RegistryValueKind::FullResourceDescriptor => REG_FULL_RESOURCE_DESCRIPTOR,
        RegistryValueKind::ResourceRequirementsList => REG_RESOURCE_REQUIREMENTS_LIST,
        RegistryValueKind::Qword => REG_QWORD,
    }
}

fn encode_utf16_nul(value: &str) -> Vec<u8> {
    value
        .encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect()
}

fn hex_decode(value: &str) -> std::result::Result<Vec<u8>, &'static str> {
    if !value.len().is_multiple_of(2) {
        return Err("registry hex data has an odd length");
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks(2) {
        let text = std::str::from_utf8(pair).map_err(|_| "registry hex data is not UTF-8")?;
        let byte = u8::from_str_radix(text, 16).map_err(|_| "registry hex data is invalid")?;
        bytes.push(byte);
    }
    Ok(bytes)
}

fn registry_warning(
    code: &'static str,
    entry: &RegistryEntry,
    message: impl Into<String>,
) -> Warning {
    Warning::new(
        code,
        WarningSeverity::Error,
        message,
        Some(format!(
            "{}\\{}\\{}",
            registry_hive_name(&entry.hive),
            entry.key_path,
            entry.value_name.as_deref().unwrap_or("(Default)")
        )),
    )
}

fn warning_io(code: &'static str, path: &Path, error: std::io::Error) -> Warning {
    Warning::new(
        code,
        WarningSeverity::Error,
        format!("{error}"),
        Some(path.display().to_string()),
    )
}

fn sc_start_type(value: &str) -> Option<&'static str> {
    match value {
        "automatic" => Some("auto"),
        "manual" => Some("demand"),
        "disabled" => Some("disabled"),
        "boot" => Some("boot"),
        "system" => Some("system"),
        _ => None,
    }
}

fn quote_path(path: &Path) -> String {
    let text = path.display().to_string();
    if text.contains(' ') {
        format!("\"{}\"", text.replace('"', "\\\""))
    } else {
        text
    }
}

fn run_checked(
    command: &mut Command,
    code: &'static str,
    source: &str,
) -> std::result::Result<(), Warning> {
    let output = command.output().map_err(|error| {
        Warning::new(
            code,
            WarningSeverity::Error,
            format!("Failed to start command: {error}"),
            Some(source.to_owned()),
        )
    })?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(Warning::new(
        code,
        WarningSeverity::Error,
        format!("Command failed: {}{}", stdout, stderr),
        Some(source.to_owned()),
    ))
}

fn command_success(command: &mut Command) -> bool {
    command.output().is_ok_and(|output| output.status.success())
}

fn scheduled_task_exists(task: &ScheduledTaskEntry) -> std::result::Result<bool, Warning> {
    let script = format!(
        "$task = Get-ScheduledTask -TaskName '{}' -TaskPath '{}' -ErrorAction SilentlyContinue; if ($null -eq $task) {{ exit 1 }}",
        escape_ps_single(&task.task_name),
        escape_ps_single(&task.task_path)
    );
    let output = Command::new(POWERSHELL_EXE)
        .args(["-NoProfile", "-NonInteractive", "-Command", script.as_str()])
        .output()
        .map_err(|error| {
            Warning::new(
                "scheduled_task_query_failed",
                WarningSeverity::Error,
                format!("Failed to query scheduled task: {error}"),
                Some(format!("{}{}", task.task_path, task.task_name)),
            )
        })?;
    Ok(output.status.success())
}

fn run_powershell_script(script: &str) -> std::result::Result<(), Warning> {
    let output = Command::new(POWERSHELL_EXE)
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|error| {
            Warning::new(
                "powershell_start_failed",
                WarningSeverity::Error,
                format!("Failed to start PowerShell: {error}"),
                None,
            )
        })?;
    if output.status.success() {
        return Ok(());
    }
    Err(Warning::new(
        "powershell_command_failed",
        WarningSeverity::Error,
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
        None,
    ))
}

fn escape_ps_single(value: &str) -> String {
    value.replace('\'', "''")
}

fn sanitize_filename(value: &str) -> String {
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

struct PreparedPackage {
    package_root: PathBuf,
    _temp_dir: Option<TempDir>,
}

impl PreparedPackage {
    fn new(options: &RestoreOptions) -> Result<Self> {
        if options.package_root.is_file()
            && options
                .package_root
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case(ZIP_EXTENSION))
        {
            let temp_dir = tempfile::tempdir().map_err(|source| {
                WinAppMigratorError::io("temporary package extraction directory", source)
            })?;
            extract_zip_archive(&options.package_root, temp_dir.path())?;
            return Ok(Self {
                package_root: temp_dir.path().to_path_buf(),
                _temp_dir: Some(temp_dir),
            });
        }
        Ok(Self {
            package_root: options.package_root.clone(),
            _temp_dir: None,
        })
    }

    fn package_root(&self) -> &Path {
        &self.package_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_apply_and_dry_run_together() {
        let options = RestoreOptions {
            package_root: PathBuf::from("pkg"),
            target_root: PathBuf::from("target"),
            dry_run: true,
            apply: true,
            force: false,
        };

        assert!(restore_package(&options).is_err());
    }

    #[test]
    fn rewrites_paths_case_insensitively() {
        let rewrites = vec![(r"C:\App".to_owned(), r"D:\Restored".to_owned())];

        assert_eq!(
            rewrite_known_paths(r"run C:\APP\bin.exe", &rewrites),
            r"run D:\Restored\bin.exe"
        );
    }
}
