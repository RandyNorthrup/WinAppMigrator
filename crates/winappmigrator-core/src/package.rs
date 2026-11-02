use std::fs;
use std::path::{Path, PathBuf};

use filetime::{FileTime, set_file_times};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::archive::{
    EmbeddedPackage, create_self_extracting_archive, create_zip_archive, extract_zip_archive,
};
use crate::authenticode::check_microsoft_signature;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    has_reserved_device_name, package_destination_path, safe_relative_path,
};
use crate::hashing::sha256_file;
use crate::models::{
    AppArchitecture, DriverEntry, FileEntry, LicenseArtifact, LicenseStoreFile,
    PACKAGE_SCHEMA_VERSION, PackageArchiveFormat, PackageManifest, PackageOptions, PackageResult,
    RegistryEntry, RegistryHive, RegistryValueKind, RuntimePrerequisite, ScanOptions, ScanReport,
    ScheduledTaskEntry, Warning, WarningSeverity,
};
use crate::path_utils::{
    DEEPEST_WALK, hidden_command, is_on_a_local_disk, sanitize_filename, walk_asking,
};
use crate::prerequisites::PREREQUISITES_DIR;
use crate::privilege::ProtectedFolder;
use crate::progress::{BuildMonitor, BuildStep};
use crate::scan::scan_and_write_reports;
use crate::system_context::{OtherAccountsFolders, VolumeMap};

const FILES_DIR: &str = "files";
const REGISTRY_DIR: &str = "registry";
const SHORTCUTS_DIR: &str = "shortcuts";
const SERVICES_DIR: &str = "services";
const DRIVERS_DIR: &str = "drivers";
const SCHEDULED_TASKS_DIR: &str = "scheduled_tasks";
const ENVIRONMENT_DIR: &str = "environment";
const LICENSE_DIR: &str = "license";
const MANIFESTS_DIR: &str = "manifests";
const REPORTS_DIR: &str = "reports";
const MANIFEST_FILE: &str = "manifest.json";
const ZIP_EXTENSION: &str = "zip";
const TEMP_PACKAGE_PREFIX: &str = "winappmigrator-package-";
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
    create_package_monitored(options, &BuildMonitor::default())
}

/// [`create_package`], reporting progress to `monitor` and stopping with
/// [`WinAppMigratorError::Cancelled`] if it is cancelled.
pub fn create_package_monitored(
    options: &PackageOptions,
    monitor: &BuildMonitor,
) -> Result<PackageResult> {
    create_package_dirs(&options.output_dir)?;
    monitor.report(BuildStep::Scanning);

    let reports_dir = options.output_dir.join(REPORTS_DIR);
    let scan_options = ScanOptions {
        selection: options.selection.clone(),
        output_dir: reports_dir.clone(),
        current_directory: options.current_directory.clone(),
        recursive_dependencies: true,
        // What goes in the package is hashed as it is copied below. A hash of the original
        // could not be checked anyway when reading it needed a backup-mode copy, and it
        // would be wrong for a file that changes in between.
        include_hashes: false,
        include_user_data: options.include_user_data,
        include_program_data: options.include_program_data,
        include_license_data: options.include_license_data,
        source: options.source.clone(),
        user_profile: options.user_profile.clone(),
    };
    let report = scan_and_write_reports(&scan_options)?;
    monitor.checkpoint()?;
    let mut warnings = report.warnings.clone();
    let mut files_copied = 0_usize;
    let mut failed_copies: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    let mut staged: std::collections::HashMap<PathBuf, (String, u64)> =
        std::collections::HashMap::new();

    // The scan looked before anything was read, and this looks again just before, as anything
    // read from here on goes in the package.
    let reader = SourceReader::new(&report);
    let total_files = report
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
        .count();
    for (done, file) in report
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
        .enumerate()
    {
        monitor.checkpoint()?;
        monitor.report(BuildStep::Copying {
            done,
            total: total_files,
        });
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
        match reader
            .check(&file.path)
            .and_then(|()| copy_file_preserving_basic_metadata(&file.path, &destination))
            .and_then(|()| reader.check_after_copying(&file.path))
            .and_then(|()| staged_fingerprint(&destination))
        {
            Ok(fingerprint) => {
                staged.insert(file.original_path.clone(), fingerprint);
                files_copied += 1;
            }
            Err(error) => {
                // Nothing goes into a package without a hash to check it by.
                let _ = fs::remove_file(&destination);
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
            if let Some((hash, size)) = staged.get(&entry.original_path) {
                entry.hash_sha256 = Some(hash.clone());
                entry.size_bytes = Some(*size);
            }
            // Where it could be read during this build means nothing to an installer.
            entry.path = entry.original_path.clone();
            entry
        })
        .collect();

    monitor.checkpoint()?;
    monitor.report(BuildStep::Bundling);
    let license_artifacts_detected = package_license_artifacts(
        &options.output_dir,
        &report.license_artifacts,
        &reader,
        &mut warnings,
    );

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
    let drivers_detected =
        package_drivers(&options.output_dir, &report.drivers, &reader, &mut warnings);

    let manifest = PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
        source_computer_name: report.source_computer_name.clone(),
        source_windows_version: report.source_windows_version.clone(),
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
        secrets_left_out: report.secrets_left_out.clone(),
        licenses_left_out: report.licenses_left_out.clone(),
        firewall_rules_detected: report.firewall_rules.clone(),
        certificates_reported: report.certificates.clone(),
        fonts_reported: report.fonts_reported.clone(),
        warnings: warnings.clone(),
        items_intentionally_excluded: report.items_excluded.clone(),
        included_user_data: options.include_user_data,
        included_program_data: options.include_program_data,
        included_license_data: options.include_license_data,
        ui_theme: options.ui_theme.clone(),
        user_data_profile: report.user_data_profile.clone(),
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
        monitor.checkpoint()?;
        create_zip_archive(&options.output_dir, &zip_path, monitor)?;
        archive_path = Some(zip_path.clone());
    }

    let mut self_extracting_archive_path = None;
    if options.create_self_extracting_archive {
        let stub = options.self_extracting_stub.as_ref().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "a self-extracting archive requires an installer stub".to_owned(),
            )
        })?;
        let installer_path = installer_output_path(options);
        if let Some(parent) = installer_path.parent() {
            fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
        }
        monitor.checkpoint()?;
        monitor.report(BuildStep::WritingInstaller);
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

/// Finds, for anything a build copies, where it can be read now and whether it may be. On this PC
/// that is simply where it is.
struct SourceReader {
    volume: Option<VolumeMap>,
    other_accounts: OtherAccountsFolders,
}

impl SourceReader {
    fn new(report: &ScanReport) -> Self {
        Self {
            volume: report.source_volume.clone(),
            other_accounts: report.other_accounts.clone(),
        }
    }

    /// Fails for a path on a drive that goes off it or through a link, and on this PC, for one that
    /// is not on a disk here, or that leads into another account's folders by now. All from the top
    /// each time, since a folder that was fine for the last file may not be now.
    fn check(&self, path: &Path) -> Result<()> {
        let refusal = match self.volume.as_ref() {
            Some(volume) if !volume.may_read(path) => {
                Some("goes through a link or is not on the connected drive")
            }
            None if !is_on_a_local_disk(path) => Some("is not on a disk of this PC"),
            None if self.other_accounts.hold(path) => Some("leads into another account's folders"),
            _ => None,
        };
        match refusal {
            Some(why) => Err(WinAppMigratorError::InvalidInput(format!(
                "{} {why}, so it was not read",
                path.display()
            ))),
            None => Ok(()),
        }
    }

    /// [`Self::check`] again once a file has been copied, on this PC. Looking twice leaves only a
    /// link that is made and undone again around the moment it is opened. A drive that may be
    /// failing is not asked for that, and nothing is expected to be changing it.
    fn check_after_copying(&self, path: &Path) -> Result<()> {
        if self.volume.is_some() {
            return Ok(());
        }
        self.check(path)
    }

    /// Where something the source PC calls `logical` can be read, if it can.
    fn locate(&self, logical: &Path) -> Result<PathBuf> {
        let located = match self.volume.as_ref() {
            Some(volume) => volume.to_volume(logical).ok_or_else(|| {
                WinAppMigratorError::InvalidInput(format!(
                    "{} is not on the connected drive, so it was not read",
                    logical.display()
                ))
            })?,
            None => logical.to_path_buf(),
        };
        self.check(&located)?;
        Ok(located)
    }
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
        // One of the app's programs can be another build than its main one.
        let architecture = prerequisite.architecture.as_ref().unwrap_or(architecture);
        if options.bundle_prerequisites
            && prerequisite.install_manually
            && let Some(spec) = crate::prerequisites::spec(&prerequisite.kind, architecture)
        {
            prerequisite.bundle = Some(plan_one_prerequisite(
                options,
                &prerequisite,
                &spec,
                architecture,
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
    architecture: &AppArchitecture,
    warnings: &mut Vec<Warning>,
) -> crate::models::PrerequisiteBundle {
    // With no payload, the installer takes the spec's other route: winget or a Windows
    // feature (e.g. the .NET runtime).
    let without_payload = crate::prerequisites::bundle_record(architecture, None, None, None);
    if !spec.bundleable {
        return without_payload;
    }
    let relative = PathBuf::from(PREREQUISITES_DIR).join(&spec.file_name);
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
            return without_payload;
        }
    };
    if let Err(error) = crate::prerequisites::download(&spec.download_url, &destination) {
        warnings.push(Warning::new(
            "prerequisite_download_failed",
            WarningSeverity::Warning,
            format!(
                "Could not download {} ({error}); the installer will try winget on the target.",
                prerequisite.name
            ),
            Some(spec.download_url.clone()),
        ));
        return without_payload;
    }
    if let Err(reason) = check_microsoft_signature(&destination) {
        warnings.push(Warning::new(
            "prerequisite_signature_failed",
            WarningSeverity::Warning,
            format!(
                "The download of {} failed its signature check, so it was not bundled: {reason}. \
                 The installer will try winget on the target.",
                prerequisite.name
            ),
            Some(spec.download_url.clone()),
        ));
        let _ = fs::remove_file(&destination);
        return without_payload;
    }
    match sha256_file(&destination) {
        // A bundled payload must carry an integrity hash, otherwise
        // verification would trivially pass on a corrupt/missing payload.
        Ok(hash) => {
            let size = fs::metadata(&destination).ok().map(|meta| meta.len());
            crate::prerequisites::bundle_record(architecture, Some(relative), Some(hash), size)
        }
        Err(error) => {
            warnings.push(Warning::new(
                "prerequisite_hash_failed",
                WarningSeverity::Warning,
                format!(
                    "Downloaded {} but could not hash it ({error}); the installer will try winget on the target instead.",
                    prerequisite.name
                ),
                Some(destination.display().to_string()),
            ));
            let _ = fs::remove_file(&destination);
            without_payload
        }
    }
}

/// Copy application-owned driver images into the package `drivers/` directory so
/// the installer can re-register them. Inbox/vendor drivers are left as metadata.
fn package_drivers(
    output_dir: &Path,
    drivers: &[DriverEntry],
    reader: &SourceReader,
    warnings: &mut Vec<Warning>,
) -> Vec<DriverEntry> {
    let mut packaged = Vec::with_capacity(drivers.len());
    for driver in drivers {
        let mut driver = driver.clone();
        let image = reader
            .locate(&driver.image_path)
            .ok()
            .filter(|image| image.is_file());
        if let Some(image) = image.as_deref().filter(|_| driver.app_owned) {
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
                copy_file_preserving_basic_metadata(image, &destination)
                    .and_then(|()| reader.check_after_copying(image))
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
    reader: &SourceReader,
    warnings: &mut Vec<Warning>,
) -> Vec<LicenseArtifact> {
    let mut packaged = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let mut artifact = artifact.clone();
        if artifact.should_copy
            && package_one_license_artifact(output_dir, &mut artifact, reader, warnings)
        {
            packaged.push(artifact);
            continue;
        }
        if artifact.should_copy {
            warnings.push(Warning::new(
                "license_file_not_copied",
                WarningSeverity::Warning,
                "License data could not be copied (missing or unreadable); recorded as metadata only.",
                artifact.path.as_ref().map(|path| path.display().to_string()),
            ));
            artifact.should_copy = false;
            artifact.package_relative_path = None;
            artifact.hash_sha256 = None;
        }
        packaged.push(artifact);
    }
    packaged
}

/// Copy one artifact, returning whether it is now in the package with its hashes.
fn package_one_license_artifact(
    output_dir: &Path,
    artifact: &mut LicenseArtifact,
    reader: &SourceReader,
    warnings: &mut Vec<Warning>,
) -> bool {
    let Some(logical) = artifact.path.clone() else {
        return false;
    };
    let relative = PathBuf::from(LICENSE_DIR).join(safe_relative_path(&logical));
    let Ok(destination) = package_destination_path(output_dir, &relative) else {
        return false;
    };
    let Ok(source) = reader.locate(&logical) else {
        return false;
    };
    if source.is_file() {
        let staged = copy_file_preserving_basic_metadata(&source, &destination)
            .and_then(|()| reader.check_after_copying(&source))
            .and_then(|()| staged_fingerprint(&destination));
        let Ok((hash, size)) = staged else {
            let _ = fs::remove_file(&destination);
            return false;
        };
        artifact.hash_sha256 = Some(hash);
        artifact.size_bytes = Some(size);
        artifact.package_relative_path = Some(relative);
        return true;
    }
    if !source.is_dir() {
        return false;
    }
    artifact.store_files = copy_license_store(&source, &destination, reader, warnings);
    if artifact.store_files.is_empty() {
        return false;
    }
    artifact.package_relative_path = Some(relative);
    true
}

/// Recursively copy a license store into the package, best-effort. Files that cannot be
/// read (locked/access-denied) are warned about and skipped rather than aborting the
/// whole package. Returns what was copied, each with its hash.
fn copy_license_store(
    source: &Path,
    destination: &Path,
    reader: &SourceReader,
    warnings: &mut Vec<Warning>,
) -> Vec<LicenseStoreFile> {
    let mut copied = Vec::new();
    // Nothing is followed, not even at the top: it was a folder when it was scanned, and any account
    // can make what is there now.
    if fs::symlink_metadata(source).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return copied;
    }
    for entry in walk_asking(source, DEEPEST_WALK, |path, _| reader.check(path).is_ok()) {
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
        if !entry.file_type.is_file() {
            continue;
        }
        let Ok(relative) = entry.path.strip_prefix(source) else {
            continue;
        };
        if has_reserved_device_name(relative) || relative.to_str().is_none() {
            continue;
        }
        let target = destination.join(safe_relative_path(relative));
        match copy_file_preserving_basic_metadata(&entry.path, &target)
            .and_then(|()| reader.check_after_copying(&entry.path))
            .and_then(|()| staged_fingerprint(&target))
        {
            Ok((hash_sha256, size_bytes)) => copied.push(LicenseStoreFile {
                relative_path: relative.to_path_buf(),
                size_bytes,
                hash_sha256,
            }),
            Err(error) => {
                let _ = fs::remove_file(&target);
                warnings.push(Warning::new(
                    "license_store_file_skipped",
                    WarningSeverity::Info,
                    format!("Skipped a license store file: {error}"),
                    Some(entry.path.display().to_string()),
                ));
            }
        }
    }
    copied
}

/// The SHA-256 and size of a file as it now sits in the package.
fn staged_fingerprint(path: &Path) -> Result<(String, u64)> {
    let hash = sha256_file(path)?;
    let size = fs::metadata(path)
        .map_err(|source| WinAppMigratorError::io(path, source))?
        .len();
    Ok((hash, size))
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
    // A plain in-process copy fails on files whose NTFS ACL denies this process —
    // common for a foreign disk owned by another PC's SIDs. Fall back to a
    // backup-semantics copy (robocopy /B) that reads through SeBackupPrivilege
    // without modifying the source, then apply metadata. `fs::copy` is tried
    // first so the common (accessible) case stays fast and dependency-free.
    match fs::copy(source, destination) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            copy_file_backup_mode(source, destination)?;
        }
        Err(error) => return Err(WinAppMigratorError::io(destination, error)),
    }
    let Ok(metadata) = fs::metadata(source) else {
        // Source metadata may be unreadable even when a backup-mode copy
        // succeeded; the bytes are what matter, so keep the copied file as-is.
        return Ok(());
    };
    let _ = fs::set_permissions(destination, metadata.permissions());
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    let _ = set_file_times(destination, accessed, modified);
    Ok(())
}

/// Copy a single file using `robocopy /B` (backup mode), which reads through the
/// process's `SeBackupPrivilege` so restrictive NTFS ACLs on a foreign disk do
/// not block the copy. It is strictly read-only on the source (never changes its
/// ACLs or attributes). robocopy copies by directory + file name, so the source
/// and destination file names must match; the caller passes a destination whose
/// name equals the source's.
pub(crate) fn copy_file_backup_mode(source: &Path, destination: &Path) -> Result<()> {
    let (Some(source_dir), Some(file_name)) = (source.parent(), source.file_name()) else {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "cannot backup-copy a file without a parent directory: {}",
            source.display()
        )));
    };
    let Some(destination_dir) = destination.parent() else {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "backup-copy destination has no parent directory: {}",
            destination.display()
        )));
    };
    let status = hidden_command("robocopy")
        .arg(source_dir)
        .arg(destination_dir)
        .arg(file_name)
        // It goes by the name, ignoring case, and a folder can be case-sensitive, so `/XJ`: a link
        // of the same name must not be what is copied.
        .args([
            "/B", "/XJ", "/NJH", "/NJS", "/NP", "/NDL", "/NC", "/NS", "/R:1", "/W:1",
        ])
        .status()
        .map_err(|error| WinAppMigratorError::io(source, error))?;
    // robocopy encodes success in exit codes 0-7 (bit 3+ are failures).
    let copied_ok = status.code().is_some_and(|code| code < 8);
    if !copied_ok {
        return Err(WinAppMigratorError::io(
            destination,
            std::io::Error::other(format!(
                "robocopy backup-mode copy failed (exit {:?})",
                status.code()
            )),
        ));
    }
    // Robocopy may rename destination when names differ; ours match, but if the
    // destination file name differs from the source, move it into place.
    if let Some(dest_name) = destination.file_name()
        && dest_name != file_name
    {
        let robocopied = destination_dir.join(file_name);
        if robocopied.exists() {
            fs::rename(&robocopied, destination)
                .map_err(|error| WinAppMigratorError::io(destination, error))?;
        }
    }
    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes).map_err(|source| WinAppMigratorError::io(path, source))
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
    sanitize_filename(value).trim_matches('_').to_owned()
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

/// A package ready to read: the folder it is already in, or a zip unpacked to a temporary
/// folder, which is removed when this is dropped.
pub(crate) struct PreparedPackage {
    package_root: PathBuf,
    _temp_dir: Option<ProtectedFolder>,
}

impl PreparedPackage {
    pub(crate) fn new(package_root: &Path) -> Result<Self> {
        if package_root.is_file()
            && package_root
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case(ZIP_EXTENSION))
        {
            let temp_dir = ProtectedFolder::new_in_temp(TEMP_PACKAGE_PREFIX)
                .map_err(|source| WinAppMigratorError::io("temporary package folder", source))?;
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

    pub(crate) fn package_root(&self) -> &Path {
        &self.package_root
    }
}

pub fn read_manifest(package_root: &Path) -> Result<PackageManifest> {
    let path = package_root.join(MANIFEST_FILE);
    let bytes = fs::read(&path).map_err(|source| WinAppMigratorError::io(&path, source))?;
    serde_json::from_slice(&bytes).map_err(WinAppMigratorError::from)
}

/// Read the manifest of the package embedded in an installer, without extracting
/// the package.
pub fn read_embedded_manifest(package: &EmbeddedPackage) -> Result<PackageManifest> {
    Ok(serde_json::from_slice(&package.read_file(MANIFEST_FILE)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::restore_review::ReviewedPackage;

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
        let manifest = read_manifest(&package_dir)?;
        let review = ReviewedPackage::new(&manifest, &root.join("restore"), false, true)?;

        assert!(result.manifest_path.is_file());
        assert!(result.files_copied >= 1);
        assert!(!review.files.is_empty());
        for file in &review.files {
            let Some(hash) = file.hash_sha256.as_deref() else {
                panic!("{} has no hash", file.original_path.display());
            };
            let Some(relative) = file.package_relative_path.as_ref() else {
                panic!("{} has no package path", file.original_path.display());
            };
            let staged = package_destination_path(&package_dir, relative)?;
            assert_eq!(hash, sha256_file(&staged)?);
            assert_eq!(
                file.size_bytes,
                fs::metadata(&staged).ok().map(|meta| meta.len())
            );
        }
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
    fn a_license_store_records_a_hash_for_every_file() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let store = root.path().join("FlexLM");
        fs::create_dir_all(store.join("sub"))
            .map_err(|error| WinAppMigratorError::io(&store, error))?;
        fs::write(store.join("a.lic"), b"a")
            .map_err(|error| WinAppMigratorError::io(&store, error))?;
        fs::write(store.join(r"sub\b.lic"), b"bb")
            .map_err(|error| WinAppMigratorError::io(&store, error))?;
        let mut warnings = Vec::new();

        let reader = SourceReader {
            volume: None,
            other_accounts: OtherAccountsFolders::default(),
        };
        let mut copied =
            copy_license_store(&store, &root.path().join("stage"), &reader, &mut warnings);
        copied.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        assert_eq!(copied.len(), 2);
        assert_eq!(copied[0].relative_path, PathBuf::from("a.lic"));
        assert_eq!(copied[0].size_bytes, 1);
        assert_eq!(copied[1].relative_path, PathBuf::from(r"sub\b.lic"));
        assert_eq!(copied[1].size_bytes, 2);
        assert_eq!(
            copied[0].hash_sha256,
            sha256_file(&root.path().join(r"stage\a.lic"))?
        );
        Ok(())
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
