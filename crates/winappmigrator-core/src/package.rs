use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use filetime::{FileTime, set_file_times};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::archive::{create_self_extracting_archive, create_zip_archive};
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::package_destination_path;
use crate::models::{
    PACKAGE_SCHEMA_VERSION, PackageArchiveFormat, PackageManifest, PackageOptions, PackageResult,
    RegistryEntry, RegistryHive, RegistryValueKind, ScanOptions, ScheduledTaskEntry, Warning,
    WarningSeverity,
};
use crate::scan::scan_and_write_reports;

const FILES_DIR: &str = "files";
const REGISTRY_DIR: &str = "registry";
const SHORTCUTS_DIR: &str = "shortcuts";
const SERVICES_DIR: &str = "services";
const DRIVERS_DIR: &str = "drivers";
const SCHEDULED_TASKS_DIR: &str = "scheduled_tasks";
const ENVIRONMENT_DIR: &str = "environment";
const MANIFESTS_DIR: &str = "manifests";
const REPORTS_DIR: &str = "reports";
const MANIFEST_FILE: &str = "manifest.json";
const RESTORE_PLAN_FILE: &str = "restore_plan.md";
const REGISTRY_ENTRIES_FILE: &str = "registry_entries.json";
const REGISTRY_EXPORT_FILE: &str = "registry_export.reg";
const COM_REGISTRATIONS_FILE: &str = "com_registrations.json";
const FILE_ASSOCIATIONS_FILE: &str = "file_associations.json";
const SERVICES_FILE: &str = "services.json";
const DRIVERS_FILE: &str = "drivers.json";
const SCHEDULED_TASKS_FILE: &str = "scheduled_tasks.json";
const SHORTCUTS_FILE: &str = "shortcuts.json";
const ENVIRONMENT_FILE: &str = "environment_variables.json";

pub fn create_package(options: &PackageOptions) -> Result<PackageResult> {
    create_package_dirs(&options.output_dir)?;

    let reports_dir = options.output_dir.join(REPORTS_DIR);
    let scan_options = ScanOptions {
        selection: options.selection.clone(),
        output_dir: reports_dir.clone(),
        current_directory: options.current_directory.clone(),
        recursive_dependencies: true,
        include_hashes: true,
        include_user_data: options.include_user_data,
    };
    let report = scan_and_write_reports(&scan_options)?;
    let mut warnings = report.warnings.clone();
    let mut files_copied = 0_usize;

    for file in report
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
    {
        let Some(relative) = file.package_relative_path.as_ref() else {
            warnings.push(Warning::new(
                "package_relative_path_missing",
                WarningSeverity::Warning,
                "Copyable file had no package relative path.",
                Some(file.original_path.display().to_string()),
            ));
            continue;
        };
        let destination = package_destination_path(&options.output_dir, relative)?;
        copy_file_preserving_basic_metadata(&file.original_path, &destination)?;
        files_copied += 1;
    }

    let manifest = PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
        source_computer_name: std::env::var("COMPUTERNAME").ok(),
        source_windows_version: source_windows_version(),
        scan_timestamp_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        main_executable_path: report.main_executable.path.clone(),
        application_metadata: report.source_app.clone(),
        file_inventory: report.file_inventory.clone(),
        registry_keys_exported: report.registry_entries.clone(),
        com_registrations_detected: report.com_registrations.clone(),
        file_associations_detected: report.file_associations.clone(),
        services_detected: report.services.clone(),
        drivers_detected: report.drivers.clone(),
        scheduled_tasks_detected: report.scheduled_tasks.clone(),
        shortcuts_detected: report.shortcuts.clone(),
        environment_variables_detected: report.environment_variables.clone(),
        runtime_prerequisites: report.runtime_prerequisites.clone(),
        warnings: warnings.clone(),
        items_intentionally_excluded: report.items_excluded.clone(),
    };

    let manifest_path = options.output_dir.join(MANIFEST_FILE);
    write_json(&manifest_path, &manifest)?;
    write_integration_artifacts(&options.output_dir, &manifest)?;
    write_restore_plan(&options.output_dir.join(RESTORE_PLAN_FILE), &manifest)?;

    let mut archive_path = None;
    if matches!(options.archive_format, Some(PackageArchiveFormat::Zip))
        || options.create_self_extracting_archive
    {
        let path = options.output_dir.with_extension("zip");
        create_zip_archive(&options.output_dir, &path)?;
        archive_path = Some(path);
    }

    let mut self_extracting_archive_path = None;
    if options.create_self_extracting_archive {
        let archive = archive_path.as_ref().ok_or_else(|| {
            WinAppMigratorError::Archive(
                "self-extracting archive requested without zip archive output".to_owned(),
            )
        })?;
        let stub = options.self_extracting_stub.as_ref().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "self-extracting archive requires a CLI stub executable".to_owned(),
            )
        })?;
        let path = options.output_dir.with_extension("sfx.exe");
        create_self_extracting_archive(stub, archive, &path)?;
        self_extracting_archive_path = Some(path);
    }

    Ok(PackageResult {
        package_root: options.output_dir.clone(),
        manifest_path,
        reports_dir,
        files_copied,
        warnings,
        archive_path,
        self_extracting_archive_path,
    })
}

fn create_package_dirs(root: &Path) -> Result<()> {
    for directory in [
        root.to_path_buf(),
        root.join(FILES_DIR),
        root.join(REGISTRY_DIR),
        root.join(SHORTCUTS_DIR),
        root.join(SERVICES_DIR),
        root.join(DRIVERS_DIR),
        root.join(SCHEDULED_TASKS_DIR),
        root.join(ENVIRONMENT_DIR),
        root.join(MANIFESTS_DIR),
        root.join(REPORTS_DIR),
    ] {
        fs::create_dir_all(&directory)
            .map_err(|source| WinAppMigratorError::io(&directory, source))?;
    }
    Ok(())
}

fn copy_file_preserving_basic_metadata(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
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

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes).map_err(|source| WinAppMigratorError::io(path, source))
}

fn source_windows_version() -> Option<String> {
    Command::new("cmd")
        .args(["/C", "ver"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn write_integration_artifacts(root: &Path, manifest: &PackageManifest) -> Result<()> {
    write_json(
        &root.join(REGISTRY_DIR).join(REGISTRY_ENTRIES_FILE),
        &manifest.registry_keys_exported,
    )?;
    write_registry_export(
        &root.join(REGISTRY_DIR).join(REGISTRY_EXPORT_FILE),
        &manifest.registry_keys_exported,
    )?;
    write_json(
        &root.join(REGISTRY_DIR).join(COM_REGISTRATIONS_FILE),
        &manifest.com_registrations_detected,
    )?;
    write_json(
        &root.join(REGISTRY_DIR).join(FILE_ASSOCIATIONS_FILE),
        &manifest.file_associations_detected,
    )?;
    write_json(
        &root.join(SERVICES_DIR).join(SERVICES_FILE),
        &manifest.services_detected,
    )?;
    write_json(
        &root.join(DRIVERS_DIR).join(DRIVERS_FILE),
        &manifest.drivers_detected,
    )?;
    write_json(
        &root.join(SCHEDULED_TASKS_DIR).join(SCHEDULED_TASKS_FILE),
        &manifest.scheduled_tasks_detected,
    )?;
    write_task_xml_files(root, &manifest.scheduled_tasks_detected)?;
    write_json(
        &root.join(SHORTCUTS_DIR).join(SHORTCUTS_FILE),
        &manifest.shortcuts_detected,
    )?;
    write_json(
        &root.join(ENVIRONMENT_DIR).join(ENVIRONMENT_FILE),
        &manifest.environment_variables_detected,
    )?;
    Ok(())
}

fn write_task_xml_files(root: &Path, tasks: &[ScheduledTaskEntry]) -> Result<()> {
    for task in tasks {
        let Some(xml) = task.definition_xml.as_ref() else {
            continue;
        };
        let filename = format!(
            "{}.xml",
            sanitize_artifact_name(&format!("{}{}", task.task_path, task.task_name))
        );
        let path = root.join(SCHEDULED_TASKS_DIR).join(filename);
        fs::write(&path, xml).map_err(|source| WinAppMigratorError::io(&path, source))?;
    }
    Ok(())
}

fn write_registry_export(path: &Path, entries: &[RegistryEntry]) -> Result<()> {
    let mut output = String::from("Windows Registry Editor Version 5.00\n\n");
    let mut current_key = String::new();
    for entry in entries {
        let key = format!(r"{}\{}", registry_hive_name(&entry.hive), entry.key_path);
        if key != current_key {
            if !current_key.is_empty() {
                output.push('\n');
            }
            output.push_str(&format!("[{key}]\n"));
            current_key = key;
        }
        if entry.value_name.is_none() && entry.value_kind == Some(RegistryValueKind::None) {
            continue;
        }
        let value_name = registry_export_value_name(entry.value_name.as_deref());
        let value = registry_export_value(entry);
        output.push_str(&format!("{value_name}={value}\n"));
    }
    fs::write(path, output).map_err(|source| WinAppMigratorError::io(path, source))
}

fn registry_hive_name(hive: &RegistryHive) -> &'static str {
    match hive {
        RegistryHive::LocalMachine => "HKEY_LOCAL_MACHINE",
        RegistryHive::CurrentUser => "HKEY_CURRENT_USER",
    }
}

fn registry_export_value_name(value_name: Option<&str>) -> String {
    match value_name {
        None | Some("(Default)") => "@".to_owned(),
        Some(name) => format!("\"{}\"", escape_reg_string(name)),
    }
}

fn registry_export_value(entry: &RegistryEntry) -> String {
    match entry.value_kind.as_ref() {
        Some(RegistryValueKind::String) => format!(
            "\"{}\"",
            escape_reg_string(entry.value_data.as_deref().unwrap_or_default())
        ),
        Some(RegistryValueKind::Dword) => {
            let bytes = hex_decode(entry.value_bytes_hex.as_deref().unwrap_or_default());
            if bytes.len() == 4 {
                let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                format!("dword:{value:08x}")
            } else {
                "hex:".to_owned()
            }
        }
        Some(RegistryValueKind::ExpandString) => registry_hex_value("hex(2)", entry),
        Some(RegistryValueKind::Binary) => registry_hex_value("hex", entry),
        Some(RegistryValueKind::DwordBigEndian) => registry_hex_value("hex(5)", entry),
        Some(RegistryValueKind::Link) => registry_hex_value("hex(6)", entry),
        Some(RegistryValueKind::MultiString) => registry_hex_value("hex(7)", entry),
        Some(RegistryValueKind::ResourceList) => registry_hex_value("hex(8)", entry),
        Some(RegistryValueKind::FullResourceDescriptor) => registry_hex_value("hex(9)", entry),
        Some(RegistryValueKind::ResourceRequirementsList) => registry_hex_value("hex(a)", entry),
        Some(RegistryValueKind::Qword) => registry_hex_value("hex(b)", entry),
        Some(RegistryValueKind::None) | None => "hex(0):".to_owned(),
    }
}

fn registry_hex_value(prefix: &str, entry: &RegistryEntry) -> String {
    let bytes = entry.value_bytes_hex.as_deref().unwrap_or_default();
    let pairs = bytes
        .as_bytes()
        .chunks(2)
        .filter_map(|chunk| std::str::from_utf8(chunk).ok())
        .collect::<Vec<_>>()
        .join(",");
    format!("{prefix}:{pairs}")
}

fn escape_reg_string(value: &str) -> String {
    value.replace('\\', r"\\").replace('"', "\\\"")
}

fn hex_decode(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks(2)
        .filter_map(|pair| {
            let text = std::str::from_utf8(pair).ok()?;
            u8::from_str_radix(text, 16).ok()
        })
        .collect()
}

fn sanitize_artifact_name(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    sanitized.trim_matches('_').to_owned()
}

fn write_restore_plan(path: &Path, manifest: &PackageManifest) -> Result<()> {
    let mut markdown = String::new();
    markdown.push_str("# WinAppMigrator Restore Plan\n\n");
    markdown.push_str(
        "Restore defaults to dry-run. Use apply mode only after reviewing this plan.\n\n",
    );
    markdown.push_str("## Files\n");
    for file in manifest
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
    {
        markdown.push_str(&format!(
            "- {} -> {}\n",
            file.package_relative_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "missing package path".to_owned()),
            file.original_path.display()
        ));
    }
    markdown.push_str("\n## Integration Items\n");
    markdown.push_str("Registry, environment variables, services, scheduled tasks, and shortcuts are archived as structured artifacts. Apply mode restores them only after privilege and overwrite checks pass.\n\n");
    for warning in &manifest.warnings {
        markdown.push_str(&format!(
            "- {:?} {}: {}\n",
            warning.severity, warning.code, warning.message
        ));
    }
    fs::write(path, markdown).map_err(|source| WinAppMigratorError::io(path, source))
}

pub fn read_manifest(package_root: &Path) -> Result<PackageManifest> {
    let path = package_root.join(MANIFEST_FILE);
    let bytes = fs::read(&path).map_err(|source| WinAppMigratorError::io(&path, source))?;
    serde_json::from_slice(&bytes).map_err(WinAppMigratorError::from)
}

pub fn files_dir(package_root: &Path) -> PathBuf {
    package_root.join(FILES_DIR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RestoreOptions;
    use crate::restore::plan_restore;

    #[test]
    fn creates_package_from_isolated_pe_copy() -> Result<()> {
        let root = temp_test_dir("package")?;
        let app_dir = root.join("app");
        let package_dir = root.join("package");
        fs::create_dir_all(&app_dir).map_err(|error| WinAppMigratorError::io(&app_dir, error))?;
        let source_exe = std::env::current_exe()
            .map_err(|error| WinAppMigratorError::io("current_exe", error))?;
        let app_exe = app_dir.join("fixture.exe");
        fs::copy(&source_exe, &app_exe)
            .map_err(|error| WinAppMigratorError::io(&app_exe, error))?;

        let result = create_package(&PackageOptions::for_executable(&app_exe, &package_dir))?;
        let restore_plan = plan_restore(&RestoreOptions {
            package_root: package_dir.clone(),
            target_root: root.join("restore"),
            dry_run: true,
            apply: false,
            force: false,
        })?;

        assert!(result.manifest_path.is_file());
        assert!(result.files_copied >= 1);
        assert!(!restore_plan.planned_file_writes.is_empty());
        fs::remove_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        Ok(())
    }

    fn temp_test_dir(label: &str) -> Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!(
            "winappmigrator-{label}-{}-{}",
            std::process::id(),
            OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        Ok(dir)
    }
}
