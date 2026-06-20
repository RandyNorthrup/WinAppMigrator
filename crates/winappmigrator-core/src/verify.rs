use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN,
    REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD,
    REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};
use winreg::{RegKey, RegValue};

use crate::archive::extract_zip_archive;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{package_destination_path, safe_relative_path};
use crate::hashing::sha256_file;
use crate::models::{
    EnvironmentScope, EnvironmentVariableEntry, FileEntry, PackageManifest, RegistryEntry,
    RegistryHive, RegistryValueKind, ScheduledTaskEntry, ServiceEntry, ShortcutEntry,
    VERIFICATION_SCHEMA_VERSION, VerificationCategory, VerificationItem, VerificationOptions,
    VerificationReport, VerificationStatus, Warning, WarningSeverity,
};
use crate::package::read_manifest;

const ZIP_EXTENSION: &str = "zip";
const VERIFICATION_REPORT_JSON: &str = "verification_report.json";
const VERIFICATION_REPORT_MARKDOWN: &str = "verification_report.md";
const POWERSHELL_EXE: &str = "powershell.exe";

pub fn verify_package(options: &VerificationOptions) -> Result<VerificationReport> {
    let prepared = PreparedPackage::new(&options.package_root)?;
    let package_root = prepared.package_root().to_path_buf();
    let manifest = read_manifest(&package_root)?;
    let mut items = Vec::new();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    verify_manifest(&manifest, &mut items, &mut errors);
    verify_package_files(&package_root, &manifest, &mut items, &mut errors);

    let restore_valid = if let Some(target_root) = options.target_root.as_ref() {
        verify_restored_target(
            target_root,
            &manifest,
            &mut items,
            &mut warnings,
            &mut errors,
        )?;
        Some(
            !items
                .iter()
                .filter(|item| {
                    !matches!(
                        item.category,
                        VerificationCategory::Manifest | VerificationCategory::PackageFile
                    )
                })
                .any(|item| item.status == VerificationStatus::Failed),
        )
    } else {
        None
    };

    let package_valid = !items
        .iter()
        .filter(|item| {
            matches!(
                item.category,
                VerificationCategory::Manifest | VerificationCategory::PackageFile
            )
        })
        .any(|item| item.status == VerificationStatus::Failed);

    let (report_json, report_markdown) = verification_artifact_paths(options, &package_root);
    let mut report = VerificationReport {
        schema_version: VERIFICATION_SCHEMA_VERSION.to_owned(),
        verified_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        package_root,
        target_root: options.target_root.clone(),
        package_valid,
        restore_valid,
        items,
        warnings,
        errors,
        report_json: report_json.clone(),
        report_markdown: report_markdown.clone(),
    };

    if let (Some(json_path), Some(markdown_path)) = (report_json, report_markdown) {
        write_verification_reports(&json_path, &markdown_path, &report)?;
        report.report_json = Some(json_path);
        report.report_markdown = Some(markdown_path);
    }

    Ok(report)
}

fn verify_manifest(
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    if manifest.schema_version == crate::models::PACKAGE_SCHEMA_VERSION {
        items.push(passed_item(
            VerificationCategory::Manifest,
            "manifest schema",
            "Package manifest schema is supported.",
            None,
        ));
    } else {
        let source = Some(manifest.schema_version.clone());
        items.push(failed_item(
            VerificationCategory::Manifest,
            "manifest schema",
            "Package manifest schema is unsupported.",
            source.clone(),
        ));
        errors.push(Warning::new(
            "verification_manifest_schema_unsupported",
            WarningSeverity::Error,
            "Package manifest schema is unsupported.",
            source,
        ));
    }
}

fn verify_package_files(
    package_root: &Path,
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for file in manifest
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
    {
        match verify_one_package_file(package_root, file) {
            Ok(item) => {
                if item.status == VerificationStatus::Failed {
                    errors.push(Warning::new(
                        "verification_package_file_failed",
                        WarningSeverity::Error,
                        item.message.clone(),
                        item.source.clone(),
                    ));
                }
                items.push(item);
            }
            Err(error) => {
                let source = Some(file.original_path.display().to_string());
                items.push(failed_item(
                    VerificationCategory::PackageFile,
                    file.original_path.display().to_string(),
                    format!("Package file verification failed: {error}"),
                    source.clone(),
                ));
                errors.push(Warning::new(
                    "verification_package_file_failed",
                    WarningSeverity::Error,
                    format!("Package file verification failed: {error}"),
                    source,
                ));
            }
        }
    }
}

fn verify_one_package_file(package_root: &Path, file: &FileEntry) -> Result<VerificationItem> {
    let Some(relative) = file.package_relative_path.as_ref() else {
        return Ok(failed_item(
            VerificationCategory::PackageFile,
            file.original_path.display().to_string(),
            "Copyable file has no package relative path.",
            Some(file.original_path.display().to_string()),
        ));
    };
    let packaged = package_destination_path(package_root, relative)?;
    if !packaged.is_file() {
        return Ok(failed_item(
            VerificationCategory::PackageFile,
            relative.display().to_string(),
            "Packaged file is missing.",
            Some(packaged.display().to_string()),
        ));
    }
    verify_file_size_and_hash(
        VerificationCategory::PackageFile,
        relative.display().to_string(),
        &packaged,
        file,
    )
}

fn verify_restored_target(
    target_root: &Path,
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    let path_rewrites = path_rewrites(manifest, target_root)?;
    verify_restored_files(target_root, manifest, items, errors)?;
    verify_registry_entries(
        &manifest.registry_keys_exported,
        &path_rewrites,
        items,
        errors,
    );
    verify_environment_variables(
        &manifest.environment_variables_detected,
        &path_rewrites,
        items,
        errors,
    );
    verify_services(
        target_root,
        manifest,
        &manifest.services_detected,
        items,
        warnings,
        errors,
    );
    verify_drivers(manifest, items, warnings);
    verify_scheduled_tasks(
        &manifest.scheduled_tasks_detected,
        &path_rewrites,
        items,
        warnings,
        errors,
    );
    verify_shortcuts(
        target_root,
        manifest,
        &manifest.shortcuts_detected,
        items,
        errors,
    );
    Ok(())
}

fn verify_restored_files(
    target_root: &Path,
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    for file in manifest
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
    {
        let Some(relative) = file.package_relative_path.as_ref() else {
            continue;
        };
        let destination = restore_destination(target_root, relative)?;
        let item = if destination.is_file() {
            verify_file_size_and_hash(
                VerificationCategory::RestoredFile,
                relative.display().to_string(),
                &destination,
                file,
            )?
        } else {
            failed_item(
                VerificationCategory::RestoredFile,
                relative.display().to_string(),
                "Restored file is missing.",
                Some(destination.display().to_string()),
            )
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_restored_file_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
    Ok(())
}

fn verify_file_size_and_hash(
    category: VerificationCategory,
    name: String,
    path: &Path,
    file: &FileEntry,
) -> Result<VerificationItem> {
    let metadata = fs::metadata(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    if let Some(expected_size) = file.size_bytes
        && metadata.len() != expected_size
    {
        return Ok(failed_item(
            category,
            name,
            format!(
                "File size mismatch. Expected {expected_size} bytes, found {} bytes.",
                metadata.len()
            ),
            Some(path.display().to_string()),
        ));
    }
    if let Some(expected_hash) = file.hash_sha256.as_deref() {
        let actual_hash = sha256_file(path)?;
        if !actual_hash.eq_ignore_ascii_case(expected_hash) {
            return Ok(failed_item(
                category,
                name,
                "SHA-256 hash mismatch.",
                Some(path.display().to_string()),
            ));
        }
    }
    Ok(passed_item(
        category,
        name,
        "File size and hash match manifest.",
        Some(path.display().to_string()),
    ))
}

fn verify_registry_entries(
    entries: &[RegistryEntry],
    path_rewrites: &[(String, String)],
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for entry in entries {
        let item = match verify_registry_entry(entry, path_rewrites) {
            Ok(item) => item,
            Err(warning) => {
                errors.push(warning.clone());
                failed_item(
                    VerificationCategory::Registry,
                    registry_entry_name(entry),
                    warning.message,
                    warning.source,
                )
            }
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_registry_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
}

fn verify_registry_entry(
    entry: &RegistryEntry,
    path_rewrites: &[(String, String)],
) -> std::result::Result<VerificationItem, Warning> {
    let root = registry_root(&entry.hive);
    let key = root.open_subkey(&entry.key_path).map_err(|error| {
        registry_warning(
            "verification_registry_key_missing",
            entry,
            format!("Registry key is missing or unreadable: {error}"),
        )
    })?;
    if entry.value_name.is_none() && entry.value_kind == Some(RegistryValueKind::None) {
        return Ok(passed_item(
            VerificationCategory::Registry,
            registry_entry_name(entry),
            "Registry key exists.",
            Some(registry_entry_name(entry)),
        ));
    }

    let value_name = registry_value_name(entry.value_name.as_deref());
    let actual = key.get_raw_value(value_name).map_err(|error| {
        registry_warning(
            "verification_registry_value_missing",
            entry,
            format!("Registry value is missing or unreadable: {error}"),
        )
    })?;
    if registry_value_matches(entry, &actual, path_rewrites)? {
        Ok(passed_item(
            VerificationCategory::Registry,
            registry_entry_name(entry),
            "Registry value matches archived data.",
            Some(registry_entry_name(entry)),
        ))
    } else {
        Ok(failed_item(
            VerificationCategory::Registry,
            registry_entry_name(entry),
            "Registry value does not match archived data.",
            Some(registry_entry_name(entry)),
        ))
    }
}

fn verify_environment_variables(
    variables: &[EnvironmentVariableEntry],
    path_rewrites: &[(String, String)],
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for variable in variables {
        let (root, key_path) = environment_registry_target(&variable.scope);
        let name = format!("{:?}\\{}", variable.scope, variable.name);
        let item = match root
            .open_subkey(key_path)
            .and_then(|key| key.get_raw_value(&variable.name))
        {
            Ok(actual) => {
                let expected = rewrite_known_paths(&variable.value, path_rewrites);
                if actual.to_string() == expected {
                    passed_item(
                        VerificationCategory::Environment,
                        name,
                        "Environment variable value matches archived data.",
                        Some(variable.name.clone()),
                    )
                } else {
                    failed_item(
                        VerificationCategory::Environment,
                        name,
                        "Environment variable value does not match archived data.",
                        Some(variable.name.clone()),
                    )
                }
            }
            Err(error) => failed_item(
                VerificationCategory::Environment,
                name,
                format!("Environment variable is missing or unreadable: {error}"),
                Some(variable.name.clone()),
            ),
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_environment_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
}

fn verify_services(
    target_root: &Path,
    manifest: &PackageManifest,
    services: &[ServiceEntry],
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in services {
        let expected_binary = mapped_original_path(manifest, target_root, &service.binary_path)
            .unwrap_or_else(|| service.binary_path.clone());
        let item = match read_service_image_path(&service.service_name) {
            Ok(actual) => {
                if command_path_matches(&actual, &expected_binary) {
                    passed_item(
                        VerificationCategory::Service,
                        service.service_name.clone(),
                        "Service exists and binary path matches restored target.",
                        Some(actual),
                    )
                } else {
                    failed_item(
                        VerificationCategory::Service,
                        service.service_name.clone(),
                        "Service exists but binary path does not match restored target.",
                        Some(actual),
                    )
                }
            }
            Err(error) => failed_item(
                VerificationCategory::Service,
                service.service_name.clone(),
                format!("Service is missing or unreadable: {error}"),
                Some(service.service_name.clone()),
            ),
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_service_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
    if !services.is_empty() {
        warnings.push(Warning::new(
            "verification_service_scope",
            WarningSeverity::Info,
            "Service verification reads HKLM service configuration and does not start services.",
            None,
        ));
    }
}

fn verify_drivers(
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
) {
    for driver in &manifest.drivers_detected {
        items.push(VerificationItem {
            category: VerificationCategory::Driver,
            name: driver.service_name.clone(),
            status: VerificationStatus::Skipped,
            message: "Driver metadata is archived for manual vendor installer verification; automatic driver restore verification is intentionally skipped."
                .to_owned(),
            source: Some(driver.image_path.display().to_string()),
        });
    }
    if !manifest.drivers_detected.is_empty() {
        warnings.push(Warning::new(
            "verification_driver_manual",
            WarningSeverity::Warning,
            "Driver and kernel components require manual validation with the signed vendor driver installer.",
            None,
        ));
    }
}

fn verify_scheduled_tasks(
    tasks: &[ScheduledTaskEntry],
    path_rewrites: &[(String, String)],
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for task in tasks {
        let name = format!("{}{}", task.task_path, task.task_name);
        let item = match scheduled_task_xml(task) {
            Ok(Some(xml)) => {
                let expected_command = rewrite_known_paths(
                    task.action_command.display().to_string().as_str(),
                    path_rewrites,
                );
                if xml
                    .to_ascii_lowercase()
                    .contains(&expected_command.to_ascii_lowercase())
                {
                    passed_item(
                        VerificationCategory::ScheduledTask,
                        name,
                        "Scheduled task exists and references the restored command path.",
                        Some(expected_command),
                    )
                } else {
                    failed_item(
                        VerificationCategory::ScheduledTask,
                        name,
                        "Scheduled task exists but does not reference the restored command path.",
                        Some(expected_command),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::ScheduledTask,
                name,
                "Scheduled task is missing.",
                Some(format!("{}{}", task.task_path, task.task_name)),
            ),
            Err(error) => failed_item(
                VerificationCategory::ScheduledTask,
                name,
                format!("Scheduled task query failed: {error}"),
                Some(format!("{}{}", task.task_path, task.task_name)),
            ),
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_scheduled_task_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
    if !tasks.is_empty() {
        warnings.push(Warning::new(
            "verification_scheduled_task_scope",
            WarningSeverity::Info,
            "Scheduled task verification checks registration and command reference only.",
            None,
        ));
    }
}

fn verify_shortcuts(
    target_root: &Path,
    manifest: &PackageManifest,
    shortcuts: &[ShortcutEntry],
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in shortcuts {
        let expected_target = mapped_original_path(manifest, target_root, &shortcut.target_path)
            .unwrap_or_else(|| shortcut.target_path.clone());
        let item = match shortcut_target(&shortcut.original_lnk_path) {
            Ok(Some(actual)) => {
                if command_path_matches(&actual, &expected_target) {
                    passed_item(
                        VerificationCategory::Shortcut,
                        shortcut.original_lnk_path.display().to_string(),
                        "Shortcut exists and target path matches restored target.",
                        Some(actual),
                    )
                } else {
                    failed_item(
                        VerificationCategory::Shortcut,
                        shortcut.original_lnk_path.display().to_string(),
                        "Shortcut exists but target path does not match restored target.",
                        Some(actual),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::Shortcut,
                shortcut.original_lnk_path.display().to_string(),
                "Shortcut is missing.",
                Some(shortcut.original_lnk_path.display().to_string()),
            ),
            Err(error) => failed_item(
                VerificationCategory::Shortcut,
                shortcut.original_lnk_path.display().to_string(),
                format!("Shortcut query failed: {error}"),
                Some(shortcut.original_lnk_path.display().to_string()),
            ),
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_shortcut_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
}

fn registry_value_matches(
    entry: &RegistryEntry,
    actual: &RegValue<'_>,
    path_rewrites: &[(String, String)],
) -> std::result::Result<bool, Warning> {
    let kind = entry.value_kind.as_ref().ok_or_else(|| {
        registry_warning(
            "verification_registry_value_kind_missing",
            entry,
            "Registry value kind is missing.",
        )
    })?;
    if actual.vtype != registry_raw_type(kind) {
        return Ok(false);
    }
    match kind {
        RegistryValueKind::String | RegistryValueKind::ExpandString => {
            let expected = rewrite_known_paths(
                entry.value_data.as_deref().unwrap_or_default(),
                path_rewrites,
            );
            Ok(actual.to_string() == expected)
        }
        RegistryValueKind::None => Ok(actual.bytes.is_empty()),
        _ => {
            let expected = hex_decode(entry.value_bytes_hex.as_deref().unwrap_or_default())
                .map_err(|message| {
                    registry_warning("verification_registry_hex_failed", entry, message)
                })?;
            Ok(actual.bytes.as_ref() == expected.as_slice())
        }
    }
}

fn restore_destination(target_root: &Path, package_relative_path: &Path) -> Result<PathBuf> {
    let relative = safe_relative_path(package_relative_path);
    let without_files = relative.strip_prefix("files").unwrap_or(&relative);
    let destination = target_root.join(without_files);
    crate::file_discovery::ensure_within_root(target_root, &destination)?;
    Ok(destination)
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

fn read_service_image_path(service_name: &str) -> std::io::Result<String> {
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = root.open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{service_name}"))?;
    key.get_value::<String, _>("ImagePath")
}

fn scheduled_task_xml(task: &ScheduledTaskEntry) -> std::io::Result<Option<String>> {
    let script = format!(
        "$task = Get-ScheduledTask -TaskName '{}' -TaskPath '{}' -ErrorAction SilentlyContinue; if ($null -eq $task) {{ exit 2 }}; Export-ScheduledTask -TaskName '{}' -TaskPath '{}'",
        escape_ps_single(&task.task_name),
        escape_ps_single(&task.task_path),
        escape_ps_single(&task.task_name),
        escape_ps_single(&task.task_path)
    );
    let output = Command::new(POWERSHELL_EXE)
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

fn shortcut_target(path: &Path) -> std::io::Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let script = format!(
        "$shell = New-Object -ComObject WScript.Shell; $shortcut = $shell.CreateShortcut('{}'); $shortcut.TargetPath",
        escape_ps_single(path.display().to_string().as_str())
    );
    let output = Command::new(POWERSHELL_EXE)
        .args(["-NoProfile", "-NonInteractive", "-Command", script.as_str()])
        .output()?;
    if output.status.success() {
        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        ))
    } else {
        Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

fn command_path_matches(actual_command: &str, expected_path: &Path) -> bool {
    let expected = expected_path.to_string_lossy().to_ascii_lowercase();
    actual_command.to_ascii_lowercase().contains(&expected)
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

fn registry_entry_name(entry: &RegistryEntry) -> String {
    format!(
        "{}\\{}\\{}",
        registry_hive_name(&entry.hive),
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    )
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
        Some(registry_entry_name(entry)),
    )
}

fn passed_item(
    category: VerificationCategory,
    name: impl Into<String>,
    message: impl Into<String>,
    source: Option<String>,
) -> VerificationItem {
    VerificationItem {
        category,
        name: name.into(),
        status: VerificationStatus::Passed,
        message: message.into(),
        source,
    }
}

fn failed_item(
    category: VerificationCategory,
    name: impl Into<String>,
    message: impl Into<String>,
    source: Option<String>,
) -> VerificationItem {
    VerificationItem {
        category,
        name: name.into(),
        status: VerificationStatus::Failed,
        message: message.into(),
        source,
    }
}

fn verification_artifact_paths(
    options: &VerificationOptions,
    package_root: &Path,
) -> (Option<PathBuf>, Option<PathBuf>) {
    let default_output_dir = if options.package_root.is_file() {
        options
            .package_root
            .parent()
            .unwrap_or(package_root)
            .to_path_buf()
    } else {
        package_root.to_path_buf()
    };
    let output_dir = options.output_dir.as_ref().unwrap_or(&default_output_dir);
    (
        Some(output_dir.join(VERIFICATION_REPORT_JSON)),
        Some(output_dir.join(VERIFICATION_REPORT_MARKDOWN)),
    )
}

fn write_verification_reports(
    json_path: &Path,
    markdown_path: &Path,
    report: &VerificationReport,
) -> Result<()> {
    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
    }
    let json = serde_json::to_vec_pretty(report)?;
    fs::write(json_path, json).map_err(|source| WinAppMigratorError::io(json_path, source))?;
    fs::write(markdown_path, render_verification_markdown(report))
        .map_err(|source| WinAppMigratorError::io(markdown_path, source))?;
    Ok(())
}

fn render_verification_markdown(report: &VerificationReport) -> String {
    let mut output = String::new();
    output.push_str("# WinAppMigrator Verification Report\n\n");
    output.push_str(&format!("- Verified UTC: {}\n", report.verified_at_utc));
    output.push_str(&format!("- Package valid: {}\n", report.package_valid));
    output.push_str(&format!(
        "- Restore valid: {}\n",
        report
            .restore_valid
            .map(|value| value.to_string())
            .unwrap_or_else(|| "not run".to_owned())
    ));
    output.push_str("\n## Items\n");
    for item in &report.items {
        output.push_str(&format!(
            "- {:?} {:?} `{}`: {}{}\n",
            item.status,
            item.category,
            item.name,
            item.message,
            item.source
                .as_ref()
                .map(|source| format!(" Source: {source}"))
                .unwrap_or_default()
        ));
    }
    output.push_str("\n## Warnings\n");
    if report.warnings.is_empty() {
        output.push_str("- None.\n");
    } else {
        for warning in &report.warnings {
            output.push_str(&format!(
                "- {:?} `{}`: {}\n",
                warning.severity, warning.code, warning.message
            ));
        }
    }
    output.push_str("\n## Errors\n");
    if report.errors.is_empty() {
        output.push_str("- None.\n");
    } else {
        for error in &report.errors {
            output.push_str(&format!(
                "- {:?} `{}`: {}\n",
                error.severity, error.code, error.message
            ));
        }
    }
    output
}

fn escape_ps_single(value: &str) -> String {
    value.replace('\'', "''")
}

struct PreparedPackage {
    package_root: PathBuf,
    _temp_dir: Option<TempDir>,
}

impl PreparedPackage {
    fn new(package_root: &Path) -> Result<Self> {
        if package_root.is_file()
            && package_root
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case(ZIP_EXTENSION))
        {
            let temp_dir = tempfile::tempdir().map_err(|source| {
                WinAppMigratorError::io("temporary verification extraction directory", source)
            })?;
            extract_zip_archive(package_root, temp_dir.path())?;
            return Ok(Self {
                package_root: temp_dir.path().to_path_buf(),
                _temp_dir: Some(temp_dir),
            });
        }
        Ok(Self {
            package_root: package_root.to_path_buf(),
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
    fn rewrites_paths_case_insensitively() {
        let rewrites = vec![(r"C:\App".to_owned(), r"D:\Restored".to_owned())];

        assert_eq!(
            rewrite_known_paths(r"run C:\APP\bin.exe", &rewrites),
            r"run D:\Restored\bin.exe"
        );
    }

    #[test]
    fn restore_destination_strips_files_root() -> Result<()> {
        let destination =
            restore_destination(Path::new(r"C:\Target"), Path::new(r"files\bin\app.exe"))?;

        assert_eq!(destination, PathBuf::from(r"C:\Target\bin\app.exe"));
        Ok(())
    }
}
