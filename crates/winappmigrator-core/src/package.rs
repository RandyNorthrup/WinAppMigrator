use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use filetime::{FileTime, set_file_times};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::archive::{create_self_extracting_archive, create_zip_archive};
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{package_destination_path, safe_relative_path};
use crate::hashing::sha256_file;
use crate::models::{
    AppArchitecture, DriverEntry, FileEntry, LicenseArtifact, PACKAGE_SCHEMA_VERSION,
    PackageArchiveFormat, PackageManifest, PackageOptions, PackageResult, RegistryEntry,
    RegistryHive, RegistryValueKind, RuntimePrerequisite, ScanOptions, ScheduledTaskEntry, Warning,
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
const LICENSE_DIR: &str = "license";
const PREREQUISITES_DIR: &str = "prerequisites";
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
        include_program_data: options.include_program_data,
        include_license_data: options.include_license_data,
    };
    let report = scan_and_write_reports(&scan_options)?;
    let mut warnings = report.warnings.clone();
    let mut files_copied = 0_usize;
    let mut failed_copies: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

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
        // A single locked or access-denied file (a running app's binary, an
        // AppData installer cache entry, etc.) must not abort the whole package.
        // Record it as a warning, mark it not-copied in the manifest so
        // verification stays consistent, and continue.
        match copy_file_preserving_basic_metadata(&file.original_path, &destination) {
            Ok(()) => files_copied += 1,
            Err(error) => {
                warnings.push(Warning::new(
                    "file_copy_failed",
                    WarningSeverity::Warning,
                    format!("Could not copy file into package: {error}"),
                    Some(file.original_path.display().to_string()),
                ));
                failed_copies.insert(file.original_path.clone());
            }
        }
    }

    let packaged_inventory: Vec<FileEntry> = report
        .file_inventory
        .iter()
        .cloned()
        .map(|mut entry| {
            if failed_copies.contains(&entry.original_path) {
                entry.should_copy = false;
            }
            entry
        })
        .collect();

    let license_artifacts_detected = package_license_artifacts(
        &options.output_dir,
        &report.license_artifacts,
        &mut warnings,
    )?;

    // Make runtime prerequisites installable on the target: download and embed
    // the redistributables when requested, recording a winget/URL fallback when a
    // download fails. Bundle application-owned driver images for automatic
    // re-registration.
    let runtime_prerequisites = bundle_prerequisites(
        options,
        &report.architecture,
        &report.runtime_prerequisites,
        &mut warnings,
    );
    let drivers_detected = package_drivers(&options.output_dir, &report.drivers, &mut warnings);

    let manifest = PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
        source_computer_name: std::env::var("COMPUTERNAME").ok(),
        source_windows_version: source_windows_version(),
        scan_timestamp_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        main_executable_path: report.main_executable.path.clone(),
        application_metadata: report.source_app.clone(),
        file_inventory: packaged_inventory,
        registry_keys_exported: report.registry_entries.clone(),
        com_registrations_detected: report.com_registrations.clone(),
        file_associations_detected: report.file_associations.clone(),
        services_detected: report.services.clone(),
        drivers_detected,
        scheduled_tasks_detected: report.scheduled_tasks.clone(),
        shortcuts_detected: report.shortcuts.clone(),
        environment_variables_detected: report.environment_variables.clone(),
        runtime_prerequisites,
        license_artifacts_detected,
        license_registry_entries: report.license_registry_entries.clone(),
        warnings: warnings.clone(),
        items_intentionally_excluded: report.items_excluded.clone(),
    };

    let manifest_path = options.output_dir.join(MANIFEST_FILE);
    write_json(&manifest_path, &manifest)?;
    write_integration_artifacts(&options.output_dir, &manifest)?;
    write_restore_plan(&options.output_dir.join(RESTORE_PLAN_FILE), &manifest)?;

    let zip_path = options.output_dir.with_extension("zip");
    let mut archive_path = None;
    if matches!(options.archive_format, Some(PackageArchiveFormat::Zip))
        || options.create_self_extracting_archive
    {
        create_zip_archive(&options.output_dir, &zip_path)?;
        archive_path = Some(zip_path.clone());
    }

    let mut self_extracting_archive_path = None;
    if options.create_self_extracting_archive {
        let stub = options.self_extracting_stub.as_ref().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "self-extracting archive requires a CLI stub executable".to_owned(),
            )
        })?;
        let installer_path = installer_output_path(options);
        if let Some(parent) = installer_path.parent() {
            fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
        }
        create_self_extracting_archive(stub, &zip_path, &installer_path)?;
        self_extracting_archive_path = Some(installer_path);

        // The standalone zip is only an intermediate for the installer. Unless a
        // zip artifact was explicitly requested, drop it so the deliverable is a
        // single self-contained executable.
        if !matches!(options.archive_format, Some(PackageArchiveFormat::Zip)) {
            let _ = fs::remove_file(&zip_path);
            archive_path = None;
        }
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

/// Compute the installer's output path. The file name encodes the chosen
/// contents so the deliverable is self-describing, e.g.
/// `Example_userdata_license_installer.exe`. That one executable carries the
/// app, its data, license, prerequisites, and drivers.
fn installer_output_path(options: &PackageOptions) -> PathBuf {
    let base = options
        .output_dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "app".to_owned());
    let mut name = sanitize_artifact_name(&base);
    if name.is_empty() {
        name = "app".to_owned();
    }
    if options.include_user_data {
        name.push_str("_userdata");
    }
    if options.include_license_data {
        name.push_str("_license");
    }
    name.push_str("_installer.exe");
    let directory = options
        .installer_dir
        .clone()
        .or_else(|| options.output_dir.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| options.output_dir.clone());
    directory.join(name)
}

/// Resolve a concrete install plan for each runtime prerequisite that must be
/// present on the target. When `bundle_prerequisites` is set, the official
/// redistributable is downloaded and embedded; if the download fails (or the
/// runtime is winget-only), a winget/URL fallback is recorded so the installer
/// can still acquire it. Prerequisites that already ship app-local get no plan.
fn bundle_prerequisites(
    options: &PackageOptions,
    architecture: &AppArchitecture,
    prerequisites: &[RuntimePrerequisite],
    warnings: &mut Vec<Warning>,
) -> Vec<RuntimePrerequisite> {
    let mut planned = Vec::with_capacity(prerequisites.len());
    for prerequisite in prerequisites {
        let mut prerequisite = prerequisite.clone();
        if options.bundle_prerequisites
            && prerequisite.install_manually
            && let Some(spec) = crate::prerequisites::spec(&prerequisite.kind, architecture)
        {
            prerequisite.bundle = Some(plan_one_prerequisite(
                options,
                &prerequisite,
                &spec,
                warnings,
            ));
        }
        planned.push(prerequisite);
    }
    planned
}

fn plan_one_prerequisite(
    options: &PackageOptions,
    prerequisite: &RuntimePrerequisite,
    spec: &crate::prerequisites::PrerequisiteSpec,
    warnings: &mut Vec<Warning>,
) -> crate::models::PrerequisiteBundle {
    if !spec.bundleable {
        // winget/URL acquisition on the target (e.g. .NET runtime).
        return crate::prerequisites::bundle_from_spec(spec, None, None, None);
    }
    let relative = PathBuf::from(PREREQUISITES_DIR).join(spec.file_name);
    let destination = match package_destination_path(&options.output_dir, &relative) {
        Ok(path) => path,
        Err(error) => {
            warnings.push(Warning::new(
                "prerequisite_path_invalid",
                WarningSeverity::Warning,
                format!(
                    "Could not place bundled prerequisite {}: {error}",
                    spec.file_name
                ),
                None,
            ));
            return crate::prerequisites::bundle_from_spec(spec, None, None, None);
        }
    };
    match crate::prerequisites::download(&spec.download_url, &destination) {
        Ok(()) => match sha256_file(&destination) {
            // A bundled payload must carry an integrity hash, otherwise
            // verification would trivially pass on a corrupt/missing payload.
            Ok(hash) => {
                let size = fs::metadata(&destination).ok().map(|meta| meta.len());
                crate::prerequisites::bundle_from_spec(spec, Some(relative), Some(hash), size)
            }
            Err(error) => {
                warnings.push(Warning::new(
                    "prerequisite_hash_failed",
                    WarningSeverity::Warning,
                    format!(
                        "Downloaded {} but could not hash it ({error}); the installer will acquire it on the target via winget/URL instead.",
                        prerequisite.name
                    ),
                    Some(destination.display().to_string()),
                ));
                let _ = fs::remove_file(&destination);
                crate::prerequisites::bundle_from_spec(spec, None, None, None)
            }
        },
        Err(error) => {
            warnings.push(Warning::new(
                "prerequisite_download_failed",
                WarningSeverity::Warning,
                format!(
                    "Could not download {} ({error}); the installer will acquire it on the target via winget/URL.",
                    prerequisite.name
                ),
                Some(spec.download_url.clone()),
            ));
            crate::prerequisites::bundle_from_spec(spec, None, None, None)
        }
    }
}

/// Copy application-owned driver images into the package `drivers/` directory so
/// the installer can re-register them. Inbox/vendor drivers are left as metadata.
fn package_drivers(
    output_dir: &Path,
    drivers: &[DriverEntry],
    warnings: &mut Vec<Warning>,
) -> Vec<DriverEntry> {
    let mut packaged = Vec::with_capacity(drivers.len());
    for driver in drivers {
        let mut driver = driver.clone();
        if driver.app_owned && driver.image_path.is_file() {
            let leaf = driver
                .image_path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "driver.sys".to_owned());
            // Prefix with the service name so two app-owned drivers sharing a .sys
            // leaf name (e.g. x64\flt.sys and x86\flt.sys) cannot collide.
            let unique = format!("{}_{}", sanitize_artifact_name(&driver.service_name), leaf);
            let relative = PathBuf::from(DRIVERS_DIR).join(safe_relative_path(Path::new(&unique)));
            match package_destination_path(output_dir, &relative).and_then(|destination| {
                copy_file_preserving_basic_metadata(&driver.image_path, &destination)
                    .map(|()| destination)
            }) {
                Ok(destination) => match sha256_file(&destination) {
                    // Only mark as bundled when we have an integrity hash, so
                    // verification cannot trivially pass on a bad image.
                    Ok(hash) => {
                        driver.hash_sha256 = Some(hash);
                        driver.package_relative_path = Some(relative);
                    }
                    Err(error) => {
                        let _ = fs::remove_file(&destination);
                        warnings.push(Warning::new(
                            "driver_hash_failed",
                            WarningSeverity::Warning,
                            format!(
                                "Bundled driver image for '{}' but could not hash it ({error}); reporting it for manual reinstall instead.",
                                driver.service_name
                            ),
                            Some(driver.image_path.display().to_string()),
                        ));
                    }
                },
                Err(error) => {
                    warnings.push(Warning::new(
                        "driver_copy_failed",
                        WarningSeverity::Warning,
                        format!(
                            "Could not bundle driver image for '{}': {error}",
                            driver.service_name
                        ),
                        Some(driver.image_path.display().to_string()),
                    ));
                }
            }
        }
        packaged.push(driver);
    }
    packaged
}

/// Copy copyable license file artifacts into the package `license/` directory
/// and return the (possibly updated) artifact list for the manifest. Non-file
/// artifacts (registry values, environment, dongles) are recorded as metadata
/// only; framework-store directories are copied recursively when transferable.
fn package_license_artifacts(
    output_dir: &Path,
    artifacts: &[LicenseArtifact],
    warnings: &mut Vec<Warning>,
) -> Result<Vec<LicenseArtifact>> {
    let mut packaged = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let mut artifact = artifact.clone();
        if artifact.should_copy {
            match artifact.path.as_ref() {
                Some(source) if source.is_file() => {
                    let relative = PathBuf::from(LICENSE_DIR).join(safe_relative_path(source));
                    let destination = package_destination_path(output_dir, &relative)?;
                    copy_file_preserving_basic_metadata(source, &destination)?;
                    artifact.hash_sha256 = sha256_file(&destination).ok();
                    artifact.size_bytes = fs::metadata(&destination).ok().map(|meta| meta.len());
                    artifact.package_relative_path = Some(relative);
                }
                Some(source) if source.is_dir() => {
                    let relative = PathBuf::from(LICENSE_DIR).join(safe_relative_path(source));
                    let destination = package_destination_path(output_dir, &relative)?;
                    let copied = copy_dir_recursive(source, &destination, warnings)?;
                    if copied > 0 {
                        artifact.package_relative_path = Some(relative);
                    } else {
                        warnings.push(Warning::new(
                            "license_store_empty",
                            WarningSeverity::Info,
                            "License store had no readable files to copy; recorded as metadata only.",
                            Some(source.display().to_string()),
                        ));
                        artifact.should_copy = false;
                    }
                }
                _ => {
                    warnings.push(Warning::new(
                        "license_file_not_copied",
                        WarningSeverity::Warning,
                        "License data could not be copied (missing or unreadable); recorded as metadata only.",
                        artifact.path.as_ref().map(|path| path.display().to_string()),
                    ));
                    artifact.should_copy = false;
                }
            }
        }
        packaged.push(artifact);
    }
    Ok(packaged)
}

/// Recursively copy a directory tree into the package, best-effort. Files that
/// cannot be read (locked/access-denied) are warned about and skipped rather
/// than aborting the whole package. Returns the number of files copied.
fn copy_dir_recursive(
    source: &Path,
    destination: &Path,
    warnings: &mut Vec<Warning>,
) -> Result<usize> {
    let mut copied = 0_usize;
    for entry in walkdir::WalkDir::new(source).follow_links(false) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(Warning::new(
                    "license_store_walk_failed",
                    WarningSeverity::Info,
                    format!("Could not enumerate part of a license store: {error}"),
                    Some(source.display().to_string()),
                ));
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(relative) = entry.path().strip_prefix(source) else {
            continue;
        };
        let target = destination.join(relative);
        match copy_file_preserving_basic_metadata(entry.path(), &target) {
            Ok(()) => copied += 1,
            Err(error) => warnings.push(Warning::new(
                "license_store_file_skipped",
                WarningSeverity::Info,
                format!("Skipped a license store file: {error}"),
                Some(entry.path().display().to_string()),
            )),
        }
    }
    Ok(copied)
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
        root.join(LICENSE_DIR),
        root.join(PREREQUISITES_DIR),
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
            restore_user_data: false,
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

    #[test]
    fn installer_name_encodes_chosen_contents() {
        let file_name = |options: &PackageOptions| {
            installer_output_path(options)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        };
        let mut options = PackageOptions::for_executable(r"C:\App\App.exe", r"C:\work\Example");
        options.include_user_data = false;
        options.include_license_data = false;
        assert_eq!(
            file_name(&options).as_deref(),
            Some("Example_installer.exe")
        );

        options.include_user_data = true;
        assert_eq!(
            file_name(&options).as_deref(),
            Some("Example_userdata_installer.exe")
        );

        options.include_license_data = true;
        assert_eq!(
            file_name(&options).as_deref(),
            Some("Example_userdata_license_installer.exe")
        );

        options.include_user_data = false;
        assert_eq!(
            file_name(&options).as_deref(),
            Some("Example_license_installer.exe")
        );

        // Default installer directory is the parent of the staging dir.
        assert_eq!(
            installer_output_path(&options).parent(),
            Some(Path::new(r"C:\work"))
        );

        // An explicit installer directory overrides the default.
        options.installer_dir = Some(PathBuf::from(r"D:\out"));
        assert_eq!(
            installer_output_path(&options).parent(),
            Some(Path::new(r"D:\out"))
        );
    }
}
