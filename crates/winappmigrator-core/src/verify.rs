use std::fs;
use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::{RegKey, RegValue};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{package_destination_path, user_data_real_destination};
use crate::hashing::sha256_file;
use crate::models::{
    EnvironmentVariableEntry, FileEntry, PackageManifest, RegistryEntry, RegistryValueKind,
    ScheduledTaskEntry, ServiceEntry, ShortcutEntry, VERIFICATION_SCHEMA_VERSION,
    VerificationCategory, VerificationItem, VerificationOptions, VerificationReport,
    VerificationStatus, Warning, WarningSeverity,
};
use crate::package::{PreparedPackage, read_manifest};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{escape_powershell_single, hidden_command, system_root};
use crate::prerequisites::{is_present, spec_for_bundle};
use crate::registry_target::{
    app_path_entries, environment_registry_target, is_path_variable, path_contains_entry,
    registry_hive_name, registry_raw_type, registry_root, registry_value_name, service_image_path,
    service_object_name,
};
use crate::restore_policy::{ServiceAccount, image_path_matches, paths_equal};

const VERIFICATION_REPORT_JSON: &str = "verification_report.json";
const VERIFICATION_REPORT_MARKDOWN: &str = "verification_report.md";
const POWERSHELL_EXE: &str = "powershell.exe";

pub fn verify_package(options: &VerificationOptions) -> Result<VerificationReport> {
    let prepared = PreparedPackage::new(&options.package_root)?;
    let manifest = read_manifest(prepared.package_root())?;
    verify_prepared_package(prepared.package_root(), &manifest, options)
}

/// Verify a package that is already unpacked, against a manifest that was already read.
/// Restoring goes through here, so it checks the same manifest it acts on.
pub(crate) fn verify_prepared_package(
    package_root: &Path,
    manifest: &PackageManifest,
    options: &VerificationOptions,
) -> Result<VerificationReport> {
    let mut items = Vec::new();
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    verify_manifest(manifest, &mut items, &mut errors);
    verify_package_files(package_root, manifest, &mut items, &mut errors);
    verify_packaged_license(package_root, manifest, &mut items, &mut errors);
    verify_packaged_prerequisites(package_root, manifest, &mut items, &mut errors);

    let restore_valid = if let Some(target_root) = options.target_root.as_ref() {
        verify_restored_target(
            target_root,
            manifest,
            options.restore_user_data,
            options.restore_license,
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

    let (report_json, report_markdown) = verification_artifact_paths(options, package_root);
    let mut report = VerificationReport {
        schema_version: VERIFICATION_SCHEMA_VERSION.to_owned(),
        verified_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        package_root: package_root.to_path_buf(),
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
    restore_user_data: bool,
    restore_license: bool,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    let map = RestoreMap::new(manifest, target_root, restore_user_data)?;
    verify_restored_files(&map, manifest, items, errors)?;
    verify_prerequisites(manifest, items);
    // License artifacts and their transferable registry values are only expected
    // on the target when the restore was told to write them.
    if restore_license {
        verify_license_restored(manifest, &map, restore_user_data, items, errors)?;
        verify_registry_entries(&manifest.license_registry_entries, &map, items, errors);
    }
    verify_registry_entries(&manifest.registry_keys_exported, &map, items, errors);
    verify_environment_variables(
        &manifest.environment_variables_detected,
        target_root,
        &map,
        items,
        errors,
    );
    verify_services(&map, &manifest.services_detected, items, warnings, errors);
    verify_drivers(manifest, items, warnings, errors);
    verify_scheduled_tasks(
        &manifest.scheduled_tasks_detected,
        &map,
        items,
        warnings,
        errors,
    );
    verify_shortcuts(&map, &manifest.shortcuts_detected, items, errors);
    Ok(())
}

fn verify_restored_files(
    map: &RestoreMap,
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
        let Some(destination) = map.destination_of(&file.original_path) else {
            continue;
        };
        let item = if destination.is_file() {
            verify_file_size_and_hash(
                VerificationCategory::RestoredFile,
                relative.display().to_string(),
                destination,
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
    verify_hash(
        category,
        name,
        path,
        file.size_bytes,
        file.hash_sha256.as_deref(),
    )
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
    if let Some(expected_size) = expected_size
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
    if let Some(expected_hash) = expected_hash {
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

/// Verify the license payloads embedded in the package (hash of each copied
/// license file). Counts toward `package_valid`.
fn verify_packaged_license(
    package_root: &Path,
    manifest: &PackageManifest,
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
        let item = if artifact.hash_sha256.is_some() {
            if packaged.is_file() {
                verify_hash(
                    VerificationCategory::PackageFile,
                    relative.display().to_string(),
                    &packaged,
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
            }
        } else if packaged.is_dir() {
            // Framework-store directory artifact: confirm the store was bundled.
            passed_item(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                "License store payload is present in the package.",
                Some(packaged.display().to_string()),
            )
        } else {
            failed_item(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                "Packaged license store is missing.",
                Some(packaged.display().to_string()),
            )
        };
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
}

/// Verify the bundled prerequisite installer payloads (hash). Counts toward
/// `package_valid`.
fn verify_packaged_prerequisites(
    package_root: &Path,
    manifest: &PackageManifest,
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
        let item = if packaged.is_file() {
            verify_hash(
                VerificationCategory::PackageFile,
                relative.display().to_string(),
                &packaged,
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
    manifest: &PackageManifest,
    map: &RestoreMap,
    restore_user_data: bool,
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    for artifact in &manifest.license_artifacts_detected {
        if !artifact.should_copy {
            continue;
        }
        let Some(original) = artifact.path.as_ref() else {
            continue;
        };
        let destination = license_destination(original, map, restore_user_data)?;
        let name = original.display().to_string();
        let item = if artifact.hash_sha256.is_some() {
            if destination.is_file() {
                verify_hash(
                    VerificationCategory::License,
                    name,
                    &destination,
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
            }
        } else if destination.is_dir() {
            passed_item(
                VerificationCategory::License,
                name,
                "Restored license store directory is present.",
                Some(destination.display().to_string()),
            )
        } else {
            failed_item(
                VerificationCategory::License,
                name,
                "Restored license store is missing.",
                Some(destination.display().to_string()),
            )
        };
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
    Ok(())
}

fn license_destination(
    original: &Path,
    map: &RestoreMap,
    restore_user_data: bool,
) -> Result<PathBuf> {
    if restore_user_data && let Some(destination) = user_data_real_destination(original)? {
        return Ok(destination);
    }
    Ok(PathBuf::from(map.rewrite(&original.to_string_lossy())))
}

fn verify_registry_entries(
    entries: &[RegistryEntry],
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

fn verify_environment_variables(
    variables: &[EnvironmentVariableEntry],
    target_root: &Path,
    map: &RestoreMap,
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
    map: &RestoreMap,
    services: &[ServiceEntry],
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in services {
        let expected_binary = map
            .destination_of(&service.binary_path)
            .unwrap_or(&service.binary_path);
        let expected_arguments = service.arguments.as_deref().map(|text| map.rewrite(text));
        let item = match service_image_path(&service.service_name) {
            Ok(Some(actual)) => {
                if !image_path_matches(
                    &actual,
                    expected_binary,
                    expected_arguments.as_deref(),
                    system_root(),
                ) {
                    failed_item(
                        VerificationCategory::Service,
                        service.service_name.clone(),
                        "Service exists but does not run the restored binary with its arguments.",
                        Some(actual),
                    )
                } else if !service_account_matches(service) {
                    failed_item(
                        VerificationCategory::Service,
                        service.service_name.clone(),
                        "Service exists but does not run as the expected account.",
                        service_object_name(&service.service_name),
                    )
                } else {
                    passed_item(
                        VerificationCategory::Service,
                        service.service_name.clone(),
                        "Service exists, and runs the restored binary with its arguments and account.",
                        Some(actual),
                    )
                }
            }
            Ok(None) => failed_item(
                VerificationCategory::Service,
                service.service_name.clone(),
                "Service is missing.",
                Some(service.service_name.clone()),
            ),
            Err(error) => failed_item(
                VerificationCategory::Service,
                service.service_name.clone(),
                format!("Service is unreadable: {error}"),
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
        verify_registry_entries(&service.parameters, map, items, errors);
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
fn service_account_matches(service: &ServiceEntry) -> bool {
    let expected =
        ServiceAccount::from_recorded(service.account_name.as_deref(), &service.service_name);
    let Some(expected) = expected.sc_object_name() else {
        return true;
    };
    let actual = service_object_name(&service.service_name);
    ServiceAccount::from_recorded(actual.as_deref(), &service.service_name).sc_object_name()
        == Some(expected)
}

fn verify_drivers(
    manifest: &PackageManifest,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    let mut manual_pending = false;
    for driver in &manifest.drivers_detected {
        if driver.package_relative_path.is_some() {
            // Bundled, application-owned driver: confirm the service is registered.
            let item = if driver_service_registered(&driver.service_name) {
                passed_item(
                    VerificationCategory::Driver,
                    driver.service_name.clone(),
                    "Driver service is registered on the target.",
                    Some(driver.image_path.display().to_string()),
                )
            } else {
                failed_item(
                    VerificationCategory::Driver,
                    driver.service_name.clone(),
                    "Bundled driver service is not registered after restore.",
                    Some(driver.image_path.display().to_string()),
                )
            };
            if item.status == VerificationStatus::Failed {
                errors.push(Warning::new(
                    "verification_driver_failed",
                    WarningSeverity::Error,
                    item.message.clone(),
                    item.source.clone(),
                ));
            }
            items.push(item);
        } else {
            manual_pending = true;
            items.push(VerificationItem {
                category: VerificationCategory::Driver,
                name: driver.service_name.clone(),
                status: VerificationStatus::Skipped,
                message: driver.restore_instructions.clone(),
                source: Some(driver.image_path.display().to_string()),
            });
        }
    }
    if manual_pending {
        warnings.push(Warning::new(
            "verification_driver_manual",
            WarningSeverity::Warning,
            "One or more vendor/inbox drivers are not redistributed by WinAppMigrator; reinstall them with the vendor's signed driver package on the target.",
            None,
        ));
    }
}

fn driver_service_registered(service_name: &str) -> bool {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{service_name}"))
        .is_ok()
}

fn verify_scheduled_tasks(
    tasks: &[ScheduledTaskEntry],
    map: &RestoreMap,
    items: &mut Vec<VerificationItem>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for task in tasks {
        let name = format!("{}{}", task.task_path, task.task_name);
        let item = match scheduled_task_xml(task) {
            Ok(Some(xml)) => {
                let expected_command = map.rewrite(&task.action_command.to_string_lossy());
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
    map: &RestoreMap,
    shortcuts: &[ShortcutEntry],
    items: &mut Vec<VerificationItem>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in shortcuts {
        let expected_target = map
            .destination_of(&shortcut.target_path)
            .unwrap_or(&shortcut.target_path);
        let item = match shortcut_target(&shortcut.original_lnk_path) {
            Ok(Some(actual)) => {
                if paths_equal(Path::new(&actual), expected_target) {
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
            let expected = map.rewrite(entry.value_data.as_deref().unwrap_or_default());
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

fn scheduled_task_xml(task: &ScheduledTaskEntry) -> std::io::Result<Option<String>> {
    let script = format!(
        "$task = Get-ScheduledTask -TaskName '{}' -TaskPath '{}' -ErrorAction SilentlyContinue; if ($null -eq $task) {{ exit 2 }}; Export-ScheduledTask -TaskName '{}' -TaskPath '{}'",
        escape_powershell_single(&task.task_name),
        escape_powershell_single(&task.task_path),
        escape_powershell_single(&task.task_name),
        escape_powershell_single(&task.task_path)
    );
    let output = hidden_command(POWERSHELL_EXE)
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

#[cfg(test)]
mod tests {
    use super::*;

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
