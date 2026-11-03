use std::fs;
use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use winreg::RegValue;

use crate::archive::{EmbeddedPackage, PackageDigests};
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{package_destination_path, safe_relative_path};
use crate::hashing::{ReadBack, read_back_files, sha256_file};
use crate::models::{
    EnvironmentVariableEntry, FileEntry, LicenseStoreFile, PackageManifest, RegistryEntry,
    RegistryValueKind, VERIFICATION_SCHEMA_VERSION, VerificationCategory, VerificationItem,
    VerificationOptions, VerificationReport, VerificationStatus, Warning, WarningSeverity,
};
use crate::package::{PreparedPackage, read_embedded_manifest, read_manifest};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{
    POWERSHELL_IMPORT_UTILITY, POWERSHELL_UTF8_OUTPUT, escape_powershell_single, hidden_command,
    system_root,
};
use crate::prerequisites::{is_present, spec_for_bundle};
use crate::progress::BuildMonitor;
use crate::registry_target::{
    app_path_entries, environment_registry_target, is_path_variable, path_contains_entry,
    registry_hive_name, registry_raw_type, registry_root, service_image_path, service_object_name,
    stored_firewall_rules,
};
use crate::restore_policy::{
    ServiceAccount, image_path_matches, paths_equal, read_task_definition, restored_value_bytes,
    restored_value_name, restored_value_text, stored_firewall_rule_matches,
};
use crate::restore_review::{
    ReviewedDriver, ReviewedLicense, ReviewedPackage, ReviewedService, ReviewedShortcut,
    ReviewedTask,
};
use crate::task_scan::live_task_xml;

const VERIFICATION_REPORT_JSON: &str = "verification_report.json";
const VERIFICATION_REPORT_MARKDOWN: &str = "verification_report.md";
const POWERSHELL_EXE: &str = "powershell.exe";

/// How a package's own files are checked.
#[derive(Clone, Copy)]
pub(crate) enum PackageFiles<'a> {
    /// By reading them where they are.
    OnDisk,
    /// By what was seen of each as it came out of the package. Reading it again from the folder
    /// only an administrator can write to, a moment later, could not tell anything more.
    Seen(&'a PackageDigests),
}

impl PackageFiles<'_> {
    /// Whether a file is in the package, at this path and by this name in the package.
    fn has(self, path: &Path, relative: &Path) -> bool {
        match self {
            Self::OnDisk => path.is_file(),
            Self::Seen(digests) => digests.get(relative).is_some(),
        }
    }

    fn verify(
        self,
        name: String,
        path: &Path,
        relative: &Path,
        expected_size: Option<u64>,
        expected_hash: Option<&str>,
    ) -> Result<VerificationItem> {
        let category = VerificationCategory::PackageFile;
        match self {
            Self::OnDisk => verify_hash(category, name, path, expected_size, expected_hash),
            Self::Seen(digests) => {
                let Some(seen) = digests.get(relative) else {
                    return Ok(failed_item(
                        category,
                        name,
                        "Packaged file is missing.",
                        Some(path.display().to_string()),
                    ));
                };
                Ok(compare_with_expected(
                    category,
                    name,
                    path,
                    seen.size,
                    Some(seen.sha256.as_str()),
                    expected_size,
                    expected_hash,
                ))
            }
        }
    }
}

pub fn verify_package(options: &VerificationOptions) -> Result<VerificationReport> {
    let prepared = PreparedPackage::new(&options.package_root)?;
    let manifest = read_manifest(prepared.package_root())?;
    verify_prepared_package(
        prepared.package_root(),
        &manifest,
        options,
        PackageFiles::OnDisk,
        &|| false,
    )
}

/// Check a finished installer as whoever runs it will find it. The whole of what is on the disk is
/// read back and compared with its footer, then everything in the package is unpacked to nowhere,
/// and what comes out is compared with what its manifest recorded as it went in.
pub fn verify_installer(installer: &Path, monitor: &BuildMonitor) -> Result<VerificationReport> {
    let package =
        EmbeddedPackage::open_unless(installer, &|| monitor.is_cancelled())?.ok_or_else(|| {
            WinAppMigratorError::Archive(format!("{} has no package in it", installer.display()))
        })?;
    monitor.checkpoint()?;
    let manifest = read_embedded_manifest(&package)?;
    let digests = package.digests(&|| monitor.is_cancelled())?;
    // It writes nothing: only the installer stays in the folder it went to.
    check_package(
        installer,
        &manifest,
        &VerificationOptions {
            package_root: installer.to_path_buf(),
            target_root: None,
            output_dir: None,
            restore_user_data: false,
            restore_license: false,
        },
        PackageFiles::Seen(&digests),
        &|| false,
    )
}

/// Check what was just unpacked from an installer, by what was seen as it came out, reading nothing.
pub fn verify_unpacked_package(
    package: &EmbeddedPackage,
    options: &VerificationOptions,
) -> Result<VerificationReport> {
    let manifest = read_embedded_manifest(package)?;
    let digests = package
        .digests_of_unpacked(&options.package_root)
        .ok_or_else(|| {
            WinAppMigratorError::InvalidInput(format!(
                "this installer's package was not unpacked to {}",
                options.package_root.display()
            ))
        })?;
    verify_prepared_package(
        &options.package_root,
        &manifest,
        options,
        PackageFiles::Seen(&digests),
        &|| false,
    )
}

/// Verify a package that is already unpacked, against a manifest that was already read.
/// Restoring goes through here, so it checks the same manifest it acts on. What was installed is
/// always read back: `files` only says how the package's own files are checked.
pub(crate) fn verify_prepared_package(
    package_root: &Path,
    manifest: &PackageManifest,
    options: &VerificationOptions,
    files: PackageFiles<'_>,
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<VerificationReport> {
    let mut report = check_package(package_root, manifest, options, files, is_cancelled)?;
    let (json_path, markdown_path) = verification_artifact_paths(options, package_root);
    if let (Some(json_path), Some(markdown_path)) = (json_path, markdown_path) {
        report.report_json = Some(json_path.clone());
        report.report_markdown = Some(markdown_path.clone());
        write_verification_reports(&json_path, &markdown_path, &report)?;
    }
    Ok(report)
}

fn check_package(
    package_root: &Path,
    manifest: &PackageManifest,
    options: &VerificationOptions,
    files: PackageFiles<'_>,
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<VerificationReport> {
    let mut items = Vec::new();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    verify_manifest(manifest, &mut items, &mut errors);
    verify_package_files(package_root, manifest, files, &mut items, &mut errors);
    verify_packaged_license(package_root, manifest, files, &mut items, &mut errors);
    verify_packaged_prerequisites(package_root, manifest, files, &mut items, &mut errors);
    verify_packaged_drivers(package_root, manifest, files, &mut items, &mut errors);

    let restore_valid = if let Some(target_root) = options.target_root.as_ref() {
        verify_restored_target(
            target_root,
            manifest,
            options.restore_user_data,
            options.restore_license,
            is_cancelled,
            &mut items,
            &mut warnings,
            &mut errors,
        )?;
        Some(restored_items_are_valid(&items))
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

    Ok(VerificationReport {
        schema_version: VERIFICATION_SCHEMA_VERSION.to_owned(),
        verified_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        package_root: package_root.to_path_buf(),
        target_root: options.target_root.clone(),
        package_valid,
        restore_valid,
        items,
        warnings,
        errors,
        report_json: None,
        report_markdown: None,
    })
}

fn restored_items_are_valid(items: &[VerificationItem]) -> bool {
    !items
        .iter()
        .filter(|item| {
            !matches!(
                item.category,
                VerificationCategory::Manifest | VerificationCategory::PackageFile
            )
        })
        .any(|item| item.status == VerificationStatus::Failed)
}

/// A check of what was installed, leaving out what the rules refused at the moment it would
/// have been done. The review the check works from could not see those coming, so it expects
/// them to be there, and them not being there is what was meant to happen.
pub(crate) fn without_what_was_refused(
    mut report: VerificationReport,
    refused: &[&str],
) -> VerificationReport {
    let is_refused = |name: &str| refused.contains(&name);
    for item in &mut report.items {
        // A file goes by where it is in the package, and was refused by where it would have gone.
        if is_refused(&item.name) || item.source.as_deref().is_some_and(is_refused) {
            item.status = VerificationStatus::Skipped;
        }
    }
    report
        .errors
        .retain(|error| !error.source.as_deref().is_some_and(is_refused));
    if report.restore_valid.is_some() {
        report.restore_valid = Some(restored_items_are_valid(&report.items));
    }
    report
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
    files: PackageFiles<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for file in manifest
        .file_inventory
        .iter()
        .filter(|entry| entry.should_copy)
    {
        match verify_one_package_file(package_root, file, files) {
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

fn verify_one_package_file(
    package_root: &Path,
    file: &FileEntry,
    files: PackageFiles<'_>,
) -> Result<VerificationItem> {
    let Some(relative) = file.package_relative_path.as_ref() else {
        return Ok(failed_item(
            VerificationCategory::PackageFile,
            file.original_path.display().to_string(),
            "Copyable file has no package relative path.",
            Some(file.original_path.display().to_string()),
        ));
    };
    let packaged = package_destination_path(package_root, relative)?;
    if !files.has(&packaged, relative) {
        return Ok(failed_item(
            VerificationCategory::PackageFile,
            relative.display().to_string(),
            "Packaged file is missing.",
            Some(packaged.display().to_string()),
        ));
    }
    files.verify(
        relative.display().to_string(),
        &packaged,
        relative,
        file.size_bytes,
        file.hash_sha256.as_deref(),
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_restored_target(
    target_root: &Path,
    manifest: &PackageManifest,
    restore_user_data: bool,
    restore_license: bool,
    is_cancelled: &(dyn Fn() -> bool + Sync),
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    // The same review the restore used, so this expects exactly what it was allowed to do.
    let review = ReviewedPackage::new(manifest, target_root, restore_user_data, restore_license)?;
    for refused in &review.refused {
        items.push(VerificationItem {
            category: VerificationCategory::Refused,
            name: refused.source.clone().unwrap_or_default(),
            status: VerificationStatus::Skipped,
            message: refused.message.clone(),
            source: None,
        });
    }
    verify_restored_files(&review, is_cancelled, items, errors)?;
    verify_restored_folders(&review, items, errors);
    verify_prerequisites(manifest, items);
    verify_license_restored(&review.license, items, errors)?;
    verify_registry_entries(
        review.license_registry.iter().copied(),
        &review.map,
        items,
        errors,
    );
    verify_registry_entries(review.registry.iter().copied(), &review.map, items, errors);
    verify_environment_variables(&review.environment, target_root, &review.map, items, errors);
    verify_firewall_rules(&review, items, errors);
    verify_services(&review.services, &review.map, items, warnings, errors);
    verify_drivers(&review, items, warnings, errors);
    verify_scheduled_tasks(&review.tasks, items, warnings, errors);
    verify_shortcuts(&review.shortcuts, items, warnings, errors);
    Ok(())
}

/// Each empty folder of the app's is there, as a folder. There is nothing else of one to check.
fn verify_restored_folders(
    review: &ReviewedPackage<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for folder in &review.empty_folders {
        let Some(destination) = review.map.folder_destination_of(&folder.original_path) else {
            continue;
        };
        let name = destination.display().to_string();
        if destination.is_dir() {
            items.push(VerificationItem {
                category: VerificationCategory::RestoredFile,
                name,
                status: VerificationStatus::Passed,
                message: "Empty folder is there.".to_owned(),
                source: Some(destination.display().to_string()),
            });
        } else {
            let message = "Empty folder is missing.";
            items.push(failed_item(
                VerificationCategory::RestoredFile,
                name.clone(),
                message,
                Some(name.clone()),
            ));
            errors.push(Warning::new(
                "verification_restored_file_failed",
                WarningSeverity::Error,
                message,
                Some(name),
            ));
        }
    }
}

/// Read back every file that was installed, from where it now is. This is what "Reinstall hashes
/// valid" goes by, and nothing that was read earlier stands in for it.
fn verify_restored_files(
    review: &ReviewedPackage<'_>,
    is_cancelled: &(dyn Fn() -> bool + Sync),
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    let installed: Vec<PathBuf> = review
        .files
        .iter()
        .filter_map(|file| review.map.destination_of(&file.original_path))
        .filter(|destination| destination.is_file())
        .map(Path::to_path_buf)
        .collect();
    let mut read_back = read_back_files(&installed, is_cancelled)?;
    for file in &review.files {
        let (Some(relative), Some(destination)) = (
            file.package_relative_path.as_ref(),
            review.map.destination_of(&file.original_path),
        ) else {
            continue;
        };
        let item = match read_back.remove(destination) {
            Some(read) => {
                let ReadBack { size, sha256 } = read?;
                compare_with_expected(
                    VerificationCategory::RestoredFile,
                    relative.display().to_string(),
                    destination,
                    size,
                    Some(&sha256),
                    file.size_bytes,
                    file.hash_sha256.as_deref(),
                )
            }
            None => failed_item(
                VerificationCategory::RestoredFile,
                relative.display().to_string(),
                "Restored file is missing.",
                Some(destination.display().to_string()),
            ),
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

/// Verify a file on disk against an expected size and SHA-256.
fn verify_hash(
    category: VerificationCategory,
    name: String,
    path: &Path,
    expected_size: Option<u64>,
    expected_hash: Option<&str>,
) -> Result<VerificationItem> {
    let metadata = fs::metadata(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    // Only read it if that can tell anything.
    let actual_hash = if expected_hash.is_none()
        || expected_size.is_some_and(|expected| expected != metadata.len())
    {
        None
    } else {
        Some(sha256_file(path)?)
    };
    Ok(compare_with_expected(
        category,
        name,
        path,
        metadata.len(),
        actual_hash.as_deref(),
        expected_size,
        expected_hash,
    ))
}

/// Say whether what was read of a file is what was expected of it.
fn compare_with_expected(
    category: VerificationCategory,
    name: String,
    path: &Path,
    actual_size: u64,
    actual_hash: Option<&str>,
    expected_size: Option<u64>,
    expected_hash: Option<&str>,
) -> VerificationItem {
    if let Some(expected_size) = expected_size
        && actual_size != expected_size
    {
        return failed_item(
            category,
            name,
            format!(
                "File size mismatch. Expected {expected_size} bytes, found {actual_size} bytes."
            ),
            Some(path.display().to_string()),
        );
    }
    // A missing hash is a failure, not a pass. Every copied file gets one when it is
    // packaged, so there is nothing else to check it by, and saying it passed would be
    // claiming a check that never happened.
    let Some(expected_hash) = expected_hash else {
        return failed_item(
            category,
            name,
            "The manifest records no hash for this file, so it cannot be checked.",
            Some(path.display().to_string()),
        );
    };
    if !actual_hash.is_some_and(|actual| actual.eq_ignore_ascii_case(expected_hash)) {
        return failed_item(
            category,
            name,
            "SHA-256 hash mismatch.",
            Some(path.display().to_string()),
        );
    }
    passed_item(
        category,
        name,
        "File size and hash match manifest.",
        Some(path.display().to_string()),
    )
}

/// Verify the license payloads embedded in the package (hash of each copied
/// license file). Counts toward `package_valid`.
fn verify_packaged_license(
    package_root: &Path,
    manifest: &PackageManifest,
    files: PackageFiles<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for artifact in &manifest.license_artifacts_detected {
        if !artifact.should_copy {
            continue;
        }
        let Some(relative) = artifact.package_relative_path.as_ref() else {
            continue;
        };
        let Ok(packaged) = package_destination_path(package_root, relative) else {
            continue;
        };
        if artifact.store_files.is_empty() {
            let item = if files.has(&packaged, relative) {
                files
                    .verify(
                        relative.display().to_string(),
                        &packaged,
                        relative,
                        artifact.size_bytes,
                        artifact.hash_sha256.as_deref(),
                    )
                    .unwrap_or_else(|error| {
                        failed_item(
                            VerificationCategory::PackageFile,
                            relative.display().to_string(),
                            format!("License payload verification failed: {error}"),
                            Some(packaged.display().to_string()),
                        )
                    })
            } else {
                failed_item(
                    VerificationCategory::PackageFile,
                    relative.display().to_string(),
                    "Packaged license payload is missing.",
                    Some(packaged.display().to_string()),
                )
            };
            push_license_payload_item(item, items, errors);
            continue;
        }
        for file in &artifact.store_files {
            let name = relative.join(&file.relative_path).display().to_string();
            let in_package = relative.join(safe_relative_path(&file.relative_path));
            let item = match store_file_path(&packaged, file) {
                Ok(path) if files.has(&path, &in_package) => files
                    .verify(
                        name.clone(),
                        &path,
                        &in_package,
                        Some(file.size_bytes),
                        Some(&file.hash_sha256),
                    )
                    .unwrap_or_else(|error| {
                        failed_item(
                            VerificationCategory::PackageFile,
                            name,
                            format!("License store verification failed: {error}"),
                            None,
                        )
                    }),
                Ok(path) => failed_item(
                    VerificationCategory::PackageFile,
                    name,
                    "Packaged license store file is missing.",
                    Some(path.display().to_string()),
                ),
                Err(error) => failed_item(
                    VerificationCategory::PackageFile,
                    name,
                    format!("Packaged license store path is invalid: {error}"),
                    None,
                ),
            };
            push_license_payload_item(item, items, errors);
        }
    }
}

fn push_license_payload_item(
    item: VerificationItem,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    if item.status == VerificationStatus::Failed {
        errors.push(Warning::new(
            "verification_license_payload_failed",
            WarningSeverity::Error,
            item.message.clone(),
            item.source.clone(),
        ));
    }
    items.push(item);
}

/// Verify the bundled prerequisite installer payloads (hash). Counts toward
/// `package_valid`.
fn verify_packaged_prerequisites(
    package_root: &Path,
    manifest: &PackageManifest,
    files: PackageFiles<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for prerequisite in &manifest.runtime_prerequisites {
        let Some(bundle) = prerequisite.bundle.as_ref() else {
            continue;
        };
        let Some(relative) = bundle.package_relative_path.as_ref() else {
            continue;
        };
        let Ok(packaged) = package_destination_path(package_root, relative) else {
            continue;
        };
        let item = if files.has(&packaged, relative) {
            files
                .verify(
                    relative.display().to_string(),
                    &packaged,
                    relative,
                    bundle.size_bytes,
                    bundle.hash_sha256.as_deref(),
                )
                .unwrap_or_else(|error| {
                    failed_item(
                        VerificationCategory::PackageFile,
                        relative.display().to_string(),
                        format!("Prerequisite payload verification failed: {error}"),
                        Some(packaged.display().to_string()),
                    )
                })
        } else {
            failed_item(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                "Bundled prerequisite payload is missing.",
                Some(packaged.display().to_string()),
            )
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_prerequisite_payload_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
}

/// Verify the driver images in the package (hash). A driver with no hash fails here, so
/// it stops the install before anything is written. Counts toward `package_valid`.
fn verify_packaged_drivers(
    package_root: &Path,
    manifest: &PackageManifest,
    files: PackageFiles<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for driver in &manifest.drivers_detected {
        let Some(relative) = driver.package_relative_path.as_ref() else {
            continue;
        };
        let item = match package_destination_path(package_root, relative) {
            Ok(packaged) if files.has(&packaged, relative) => files
                .verify(
                    relative.display().to_string(),
                    &packaged,
                    relative,
                    None,
                    driver.hash_sha256.as_deref(),
                )
                .unwrap_or_else(|error| {
                    failed_item(
                        VerificationCategory::PackageFile,
                        relative.display().to_string(),
                        format!("Driver image verification failed: {error}"),
                        Some(packaged.display().to_string()),
                    )
                }),
            Ok(packaged) => failed_item(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                "Bundled driver image is missing.",
                Some(packaged.display().to_string()),
            ),
            Err(error) => failed_item(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                format!("Bundled driver path is invalid: {error}"),
                None,
            ),
        };
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_driver_image_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
}

/// Verify runtime prerequisites are present on the target after restore.
///
/// A present prerequisite passes; an absent one is reported as Skipped (not
/// Failed) because absence can mean a reboot is pending after a reboot-required
/// install. The authoritative failure signal is `restore`'s own errors
/// (`restore_prerequisites`/`confirm_prerequisite`), which mark a genuine,
/// non-reboot install failure as an error.
fn verify_prerequisites(manifest: &PackageManifest, items: &mut Vec<VerificationItem>) {
    for prerequisite in &manifest.runtime_prerequisites {
        let Some(bundle) = prerequisite.bundle.as_ref() else {
            continue;
        };
        let probe = spec_for_bundle(&prerequisite.kind, bundle).and_then(|spec| spec.probe);
        let item = match probe.as_ref() {
            Some(probe) if is_present(probe) => passed_item(
                VerificationCategory::Prerequisite,
                prerequisite.name.clone(),
                "Runtime prerequisite is present on the target.",
                None,
            ),
            Some(_) => VerificationItem {
                category: VerificationCategory::Prerequisite,
                name: prerequisite.name.clone(),
                status: VerificationStatus::Skipped,
                message:
                    "Runtime prerequisite not detected; it may require a reboot or manual completion (see the restore log for the authoritative outcome)."
                        .to_owned(),
                source: None,
            },
            None => VerificationItem {
                category: VerificationCategory::Prerequisite,
                name: prerequisite.name.clone(),
                status: VerificationStatus::Skipped,
                message: "Prerequisite presence is not registry-probable; install was best-effort."
                    .to_owned(),
                source: None,
            },
        };
        items.push(item);
    }
}

/// Verify restored license files match their packaged hashes at their target
/// locations.
fn verify_license_restored(
    licenses: &[ReviewedLicense<'_>],
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    for license in licenses {
        let artifact = license.artifact;
        let destination = &license.destination;
        let name = destination.display().to_string();
        if artifact.store_files.is_empty() {
            let item = if destination.is_file() {
                verify_hash(
                    VerificationCategory::License,
                    name,
                    destination,
                    artifact.size_bytes,
                    artifact.hash_sha256.as_deref(),
                )?
            } else {
                failed_item(
                    VerificationCategory::License,
                    name,
                    "Restored license file is missing.",
                    Some(destination.display().to_string()),
                )
            };
            push_license_item(item, items, errors);
            continue;
        }
        for file in &artifact.store_files {
            let path = store_file_path(destination, file)?;
            let name = path.display().to_string();
            let item = if path.is_file() {
                verify_hash(
                    VerificationCategory::License,
                    name,
                    &path,
                    Some(file.size_bytes),
                    Some(&file.hash_sha256),
                )?
            } else {
                failed_item(
                    VerificationCategory::License,
                    name,
                    "Restored license store file is missing.",
                    Some(path.display().to_string()),
                )
            };
            push_license_item(item, items, errors);
        }
    }
    Ok(())
}

fn push_license_item(
    item: VerificationItem,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    if item.status == VerificationStatus::Failed {
        errors.push(Warning::new(
            "verification_license_failed",
            WarningSeverity::Error,
            item.message.clone(),
            item.source.clone(),
        ));
    }
    items.push(item);
}

/// A license store file under a store folder, by a path the manifest supplied.
fn store_file_path(store: &Path, file: &LicenseStoreFile) -> Result<PathBuf> {
    package_destination_path(store, &safe_relative_path(&file.relative_path))
}

fn verify_registry_entries<'a>(
    entries: impl IntoIterator<Item = &'a RegistryEntry>,
    map: &RestoreMap,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for entry in entries {
        let item = match verify_registry_entry(entry, map) {
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
    map: &RestoreMap,
) -> std::result::Result<VerificationItem, Warning> {
    let key = registry_root(&entry.hive)
        .and_then(|root| root.open_subkey(&entry.key_path))
        .map_err(|error| {
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

    let value_name = restored_value_name(entry, map);
    let actual = key.get_raw_value(value_name.as_ref()).map_err(|error| {
        registry_warning(
            "verification_registry_value_missing",
            entry,
            format!("Registry value is missing or unreadable: {error}"),
        )
    })?;
    if registry_value_matches(entry, &actual, map)? {
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

/// Each rule by what Windows Firewall has stored. What it says it has in the PC's language.
fn verify_firewall_rules(
    review: &ReviewedPackage<'_>,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    if review.firewall_rules.is_empty() {
        return;
    }
    let stored = stored_firewall_rules().unwrap_or_default();
    for reviewed in &review.firewall_rules {
        let rule = reviewed.entry;
        let name = format!("The firewall rule {}", rule.name);
        let there = stored.iter().any(|text| {
            stored_firewall_rule_matches(text, &rule.name, &rule.direction, &reviewed.program)
        });
        if there {
            items.push(passed_item(
                VerificationCategory::FirewallRule,
                name.clone(),
                "Windows Firewall has this rule.",
                Some(name),
            ));
        } else {
            let message = "Windows Firewall does not have this rule.";
            errors.push(Warning::new(
                "verification_firewall_rule_missing",
                WarningSeverity::Error,
                message,
                Some(name.clone()),
            ));
            items.push(failed_item(
                VerificationCategory::FirewallRule,
                name.clone(),
                message,
                Some(name),
            ));
        }
    }
}

fn verify_environment_variables(
    variables: &[&EnvironmentVariableEntry],
    target_root: &Path,
    map: &RestoreMap,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for variable in variables {
        let name = format!("{:?}\\{}", variable.scope, variable.name);
        let item = match environment_registry_target(&variable.scope)
            .and_then(|(root, key_path)| root.open_subkey(key_path))
            .and_then(|key| key.get_raw_value(&variable.name))
        {
            Ok(actual) => {
                let expected = map.rewrite(&variable.value);
                let matches = if is_path_variable(&variable.name) {
                    let actual = actual.to_string();
                    app_path_entries(&expected, target_root)
                        .iter()
                        .all(|entry| path_contains_entry(&actual, entry))
                } else {
                    actual.to_string() == expected
                };
                if matches {
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
    services: &[ReviewedService<'_>],
    map: &RestoreMap,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in services {
        let name = || service.entry.service_name.clone();
        let item = match service_image_path(&service.entry.service_name) {
            Ok(Some(actual)) => {
                if !image_path_matches(
                    &actual,
                    &service.binary,
                    service.arguments.as_deref(),
                    system_root(),
                ) {
                    failed_item(
                        VerificationCategory::Service,
                        name(),
                        "Service exists but does not run the restored binary with its arguments.",
                        Some(name()),
                    )
                } else if !service_account_matches(service) {
                    failed_item(
                        VerificationCategory::Service,
                        name(),
                        "Service exists but does not run as the expected account.",
                        service_object_name(&service.entry.service_name),
                    )
                } else {
                    passed_item(
                        VerificationCategory::Service,
                        name(),
                        "Service exists, and runs the restored binary with its arguments and account.",
                        Some(actual),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::Service,
                name(),
                "Service is missing.",
                Some(name()),
            ),
            Err(error) => failed_item(
                VerificationCategory::Service,
                name(),
                format!("Service is unreadable: {error}"),
                Some(name()),
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
        verify_registry_entries(&service.entry.parameters, map, items, errors);
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

/// The account on the target is the one restoring asked for, and that is not always the
/// recorded one: a custom account is set up as LocalService.
fn service_account_matches(service: &ReviewedService<'_>) -> bool {
    let Some(expected) = service.account.sc_object_name() else {
        return true;
    };
    let actual = service_object_name(&service.entry.service_name);
    ServiceAccount::from_recorded(actual.as_deref(), &service.entry.service_name).sc_object_name()
        == Some(expected)
}

fn verify_drivers(
    review: &ReviewedPackage<'_>,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for driver in &review.manual_drivers {
        items.push(VerificationItem {
            category: VerificationCategory::Driver,
            name: driver.service_name.clone(),
            status: VerificationStatus::Skipped,
            message: driver.restore_instructions.clone(),
            source: Some(driver.image_path.display().to_string()),
        });
    }
    for driver in &review.drivers {
        let item = verify_one_driver(driver);
        if item.status == VerificationStatus::Failed {
            errors.push(Warning::new(
                "verification_driver_failed",
                WarningSeverity::Error,
                item.message.clone(),
                item.source.clone(),
            ));
        }
        items.push(item);
    }
    if !review.manual_drivers.is_empty() {
        warnings.push(Warning::new(
            "verification_driver_manual",
            WarningSeverity::Warning,
            "One or more vendor/inbox drivers are not redistributed by WinAppMigrator; reinstall them with the vendor's signed driver package on the target.",
            None,
        ));
    }
}

/// A bundled driver is restored when its service points at its image and the image on
/// disk is the one that was packaged.
fn verify_one_driver(reviewed: &ReviewedDriver<'_>) -> VerificationItem {
    let driver = reviewed.entry;
    let destination = &reviewed.destination;
    let name = || driver.service_name.clone();
    let source = Some(destination.display().to_string());
    match service_image_path(&driver.service_name) {
        Ok(Some(actual)) if image_path_matches(&actual, destination, None, system_root()) => {}
        Ok(Some(_)) => {
            return failed_item(
                VerificationCategory::Driver,
                name(),
                "Driver service does not point at the restored image.",
                source,
            );
        }
        Ok(None) => {
            return failed_item(
                VerificationCategory::Driver,
                name(),
                "Bundled driver service is not registered after restore.",
                source,
            );
        }
        Err(error) => {
            return failed_item(
                VerificationCategory::Driver,
                name(),
                format!("Driver service is unreadable: {error}"),
                source,
            );
        }
    }
    if !destination.is_file() {
        return failed_item(
            VerificationCategory::Driver,
            name(),
            "Driver image is missing.",
            source,
        );
    }
    verify_hash(
        VerificationCategory::Driver,
        name(),
        destination,
        None,
        driver.hash_sha256.as_deref(),
    )
    .unwrap_or_else(|error| {
        failed_item(
            VerificationCategory::Driver,
            name(),
            format!("Driver image could not be checked: {error}"),
            source,
        )
    })
}

fn verify_scheduled_tasks(
    tasks: &[ReviewedTask<'_>],
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for task in tasks {
        let name = format!("{}{}", task.entry.task_path, task.entry.task_name);
        let item = match live_task_xml(&task.entry.task_path, &task.entry.task_name) {
            Ok(Some(xml)) => {
                if task_runs_commands(&xml, &task.commands) {
                    passed_item(
                        VerificationCategory::ScheduledTask,
                        name.clone(),
                        "Scheduled task exists and runs the restored programs.",
                        Some(name),
                    )
                } else {
                    failed_item(
                        VerificationCategory::ScheduledTask,
                        name.clone(),
                        "Scheduled task exists but does not run the restored programs.",
                        Some(name),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::ScheduledTask,
                name.clone(),
                "Scheduled task is missing.",
                Some(name),
            ),
            Err(error) => failed_item(
                VerificationCategory::ScheduledTask,
                name.clone(),
                format!("Scheduled task query failed: {error}"),
                Some(name),
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

/// True when a task's definition runs exactly these commands.
fn task_runs_commands(xml: &str, expected: &[String]) -> bool {
    let Ok(definition) = read_task_definition(xml) else {
        return false;
    };
    let same = |left: &str, right: &str| {
        paths_equal(
            Path::new(left.trim().trim_matches('"')),
            Path::new(right.trim().trim_matches('"')),
        )
    };
    definition.commands.len() == expected.len()
        && definition
            .commands
            .iter()
            .zip(expected)
            .all(|(actual, wanted)| same(actual, wanted))
}

/// A shortcut in a Startup folder has to be right, because it makes something run. Any other
/// is only a way in to an app that works without it, as it is when restoring.
fn verify_shortcuts(
    shortcuts: &[ReviewedShortcut],
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in shortcuts {
        let destination = &shortcut.placement.destination;
        let name = || destination.display().to_string();
        let mut item = match shortcut_target(destination) {
            Ok(Some(actual)) => {
                if paths_equal(Path::new(&actual), &shortcut.target) {
                    passed_item(
                        VerificationCategory::Shortcut,
                        name(),
                        "Shortcut exists and target path matches restored target.",
                        Some(actual),
                    )
                } else {
                    failed_item(
                        VerificationCategory::Shortcut,
                        name(),
                        "Shortcut exists but target path does not match restored target.",
                        Some(actual),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::Shortcut,
                name(),
                "Shortcut is missing.",
                Some(name()),
            ),
            Err(error) => failed_item(
                VerificationCategory::Shortcut,
                name(),
                format!("Shortcut query failed: {error}"),
                Some(name()),
            ),
        };
        if item.status == VerificationStatus::Failed {
            let problem = Warning::new(
                "verification_shortcut_failed",
                WarningSeverity::Error,
                item.message.clone(),
                Some(name()),
            );
            if shortcut.placement.autostart {
                errors.push(problem);
            } else {
                item.status = VerificationStatus::Warning;
                warnings.push(problem);
            }
        }
        items.push(item);
    }
}

fn registry_value_matches(
    entry: &RegistryEntry,
    actual: &RegValue<'_>,
    map: &RestoreMap,
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
            Ok(actual.to_string() == restored_value_text(entry, map))
        }
        RegistryValueKind::None => Ok(actual.bytes.is_empty()),
        _ => {
            let expected = restored_value_bytes(entry, map).map_err(|message| {
                registry_warning("verification_registry_hex_failed", entry, message)
            })?;
            Ok(actual.bytes.as_ref() == expected.as_slice())
        }
    }
}

fn shortcut_target(path: &Path) -> std::io::Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let script = format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_UTILITY}$shell = New-Object -ComObject WScript.Shell; $shortcut = $shell.CreateShortcut('{}'); $shortcut.TargetPath",
        escape_powershell_single(path.display().to_string().as_str())
    );
    let output = hidden_command(POWERSHELL_EXE)
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

fn registry_entry_name(entry: &RegistryEntry) -> String {
    format!(
        "{}\\{}\\{}",
        registry_hive_name(&entry.hive),
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_was_refused_while_installing_is_not_held_against_the_install() {
        let refused = r"HKLM\SOFTWARE\Classes\CLSID\{A}\InprocServer32\(Default)";
        let error = |source: &str| {
            Warning::new(
                "verification_registry_failed",
                WarningSeverity::Error,
                "Registry key is missing.",
                Some(source.to_owned()),
            )
        };
        let report = |items, errors| VerificationReport {
            schema_version: String::new(),
            verified_at_utc: String::new(),
            package_root: PathBuf::new(),
            target_root: None,
            package_valid: true,
            restore_valid: Some(false),
            items,
            warnings: Vec::new(),
            errors,
            report_json: None,
            report_markdown: None,
        };
        let failed = |name: &str| {
            failed_item(
                VerificationCategory::Registry,
                name,
                "Registry key is missing.",
                Some(name.to_owned()),
            )
        };

        let only_refused = without_what_was_refused(
            report(vec![failed(refused)], vec![error(refused)]),
            &[refused],
        );
        assert_eq!(only_refused.restore_valid, Some(true));
        assert!(only_refused.errors.is_empty());

        let also_real = without_what_was_refused(
            report(
                vec![failed(refused), failed(r"HKLM\SOFTWARE\Acme\Other")],
                vec![error(refused), error(r"HKLM\SOFTWARE\Acme\Other")],
            ),
            &[refused],
        );
        assert_eq!(also_real.restore_valid, Some(false));
        assert_eq!(also_real.errors.len(), 1);
    }

    fn task_xml(commands: &[&str]) -> String {
        let actions: String = commands
            .iter()
            .map(|command| format!("<Exec><Command>{command}</Command></Exec>"))
            .collect();
        format!("<Task><Actions>{actions}</Actions></Task>")
    }

    #[test]
    fn a_task_is_checked_for_exactly_its_programs() {
        let expected = [r"D:\Apps\Acme\acme.exe".to_owned()];
        assert!(task_runs_commands(
            &task_xml(&[r"D:\Apps\Acme\acme.exe"]),
            &expected
        ));
        assert!(task_runs_commands(
            &task_xml(&[r#""d:\apps\acme\ACME.exe""#]),
            &expected
        ));
        // It only had to contain the expected text before.
        assert!(!task_runs_commands(
            &task_xml(&[r"D:\Apps\Acme\acme.exe.bak"]),
            &expected
        ));
        assert!(!task_runs_commands(
            &task_xml(&[r"D:\Apps\Acme\acme.exe", r"C:\evil.exe"]),
            &expected
        ));
        assert!(!task_runs_commands("<Task>", &expected));
    }

    fn checked(expected_hash: Option<&str>) -> std::io::Result<VerificationStatus> {
        let folder = tempfile::tempdir()?;
        let file = folder.path().join("a.bin");
        fs::write(&file, b"x")?;
        let item = verify_hash(
            VerificationCategory::PackageFile,
            "a.bin".to_owned(),
            &file,
            None,
            expected_hash,
        )
        .map_err(std::io::Error::other)?;
        Ok(item.status)
    }

    #[test]
    fn a_missing_hash_is_a_failure_and_never_a_pass() -> std::io::Result<()> {
        assert_eq!(checked(None)?, VerificationStatus::Failed);
        assert_eq!(checked(Some("00"))?, VerificationStatus::Failed);
        let right = "2d711642b726b04401627ca9fbac32f5c8530fb1903cc4db02258717921a4881";
        assert_eq!(checked(Some(right))?, VerificationStatus::Passed);
        Ok(())
    }

    #[test]
    fn restore_destination_strips_files_root() -> Result<()> {
        let destination = crate::file_discovery::restore_destination(
            Path::new(r"C:\Target"),
            Path::new(r"files\bin\app.exe"),
        )?;

        assert_eq!(destination, PathBuf::from(r"C:\Target\bin\app.exe"));
        Ok(())
    }
}
