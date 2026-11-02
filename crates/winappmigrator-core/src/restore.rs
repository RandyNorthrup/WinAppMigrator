use std::borrow::Cow;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use filetime::{FileTime, set_file_times};
use winreg::RegValue;
use winreg::enums::REG_NONE;

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    ensure_target_root, ensure_within_root, package_destination_path,
    restore_destination_for_entry, safe_relative_path, split_user_data_relative,
    user_data_real_destination, write_root_for,
};
use crate::hashing::sha256_file;
use crate::models::{
    DriverEntry, DriverKind, EnvironmentVariableEntry, LicenseArtifact, PackageManifest,
    RegistryEntry, RegistryValueKind, RestoreOptions, RestorePlan, RestoreResult,
    RuntimePrerequisite, ScheduledTaskEntry, ServiceEntry, ShortcutEntry, VerificationOptions,
    Warning, WarningSeverity,
};
use crate::package::{PreparedPackage, read_manifest};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{escape_powershell_single, hidden_command, sanitize_filename, system_root};
use crate::prerequisites::{
    PrerequisiteInstallOutcome, PrerequisiteSpec, install_bundle, is_present, spec_for_bundle,
};
use crate::privilege::{destination_requires_admin, is_elevated, target_requires_admin};
use crate::registry_target::{
    app_path_entries, environment_registry_target, is_path_variable, merge_path_entries,
    registry_hive_name, registry_raw_type, registry_root, registry_value_name, service_image_path,
};
use crate::restore_policy::{
    ServiceAccount, driver_destination, existing_service_refusal, is_valid_service_name,
    service_image_path_value,
};
use crate::verify::verify_prepared_package;

const RESTORE_LOG_JSON: &str = "restore_log.json";
const RESTORE_LOG_MARKDOWN: &str = "restore_log.md";
const POWERSHELL_EXE: &str = "powershell.exe";
const SC_EXE: &str = "sc.exe";
/// Buffer for copying a file out of the package. Large files are the slow ones.
const COPY_BUFFER_BYTES: usize = 1024 * 1024;

pub fn plan_restore(options: &RestoreOptions) -> Result<RestorePlan> {
    let manifest = read_manifest(&options.package_root)?;
    Ok(plan_from_manifest(manifest, options))
}

fn plan_from_manifest(manifest: PackageManifest, options: &RestoreOptions) -> RestorePlan {
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

    RestorePlan {
        package_root: options.package_root.clone(),
        target_root: options.target_root.clone(),
        dry_run: !options.apply || options.dry_run,
        planned_file_writes,
        planned_registry_writes: manifest.registry_keys_exported,
        planned_service_changes: manifest.services_detected,
        planned_driver_actions: manifest.drivers_detected,
        planned_task_changes: manifest.scheduled_tasks_detected,
        planned_shortcut_changes: manifest.shortcuts_detected,
        planned_environment_changes: manifest.environment_variables_detected,
        warnings,
    }
}

pub fn restore_package(options: &RestoreOptions) -> Result<RestoreResult> {
    if options.apply && options.dry_run {
        return Err(WinAppMigratorError::InvalidInput(
            "--apply and --dry-run cannot both be supplied".to_owned(),
        ));
    }
    // A relative or empty location would resolve against the working directory.
    if !options.target_root.is_absolute() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the install location must be a full path: \"{}\"",
            options.target_root.display()
        )));
    }

    let prepared = PreparedPackage::new(&options.package_root)?;
    let mut effective_options = options.clone();
    effective_options.package_root = prepared.package_root().to_path_buf();
    // Read once. Everything below works from this copy, so nothing can change between
    // the hash check and the install, even if the file on disk does.
    let manifest = read_manifest(&effective_options.package_root)?;
    if options.apply {
        refuse_damaged_package(&effective_options, &manifest)?;
    }

    let plan = plan_from_manifest(manifest.clone(), &effective_options);
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
    if options.apply
        && options.restore_user_data
        && !is_elevated()
        && user_data_restore_requires_admin(&effective_options, &plan)
    {
        return Err(WinAppMigratorError::InvalidInput(
            "restoring user data into ProgramData or other protected locations requires elevated privileges"
                .to_owned(),
        ));
    }

    let map = RestoreMap::new(&manifest, &options.target_root, options.restore_user_data)?;
    if options.apply {
        ensure_target_root(&options.target_root)?;
    }
    let output_dir = restore_output_dir(&options.package_root, &effective_options.package_root);
    let mut changes_logged = Vec::new();
    let mut warnings = plan.warnings.clone();
    let mut errors = Vec::new();
    if options.restore_user_data
        && plan
            .planned_file_writes
            .iter()
            .any(|file| split_user_data_relative(&file.original_path).is_some())
    {
        warnings.push(Warning::new(
            "restore_user_data_real_locations",
            WarningSeverity::Info,
            "User data is restored to this machine's real %AppData%/%ProgramData% locations instead of the target root.",
            None,
        ));
    }

    // Install runtime prerequisites first so the app's own services/drivers and
    // launch have their runtimes available.
    restore_prerequisites(
        &effective_options,
        &manifest,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
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
        &map,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_license(
        &effective_options,
        &manifest,
        &map,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_services(
        &effective_options,
        &map,
        &plan.planned_service_changes,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_drivers(
        &effective_options,
        &plan.planned_driver_actions,
        &map,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_scheduled_tasks(
        &effective_options,
        &plan.planned_task_changes,
        &map,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_shortcuts(
        &effective_options,
        &map,
        &plan.planned_shortcut_changes,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );
    restore_environment(
        &effective_options,
        &plan.planned_environment_changes,
        &map,
        &mut changes_logged,
        &mut warnings,
        &mut errors,
    );

    let verification = if options.apply {
        match verify_prepared_package(
            &effective_options.package_root,
            &manifest,
            &VerificationOptions {
                package_root: effective_options.package_root.clone(),
                target_root: Some(options.target_root.clone()),
                output_dir: Some(output_dir.clone()),
                restore_user_data: options.restore_user_data,
                restore_license: options.restore_license,
            },
        ) {
            Ok(report) => {
                if !report.package_valid || report.restore_valid == Some(false) {
                    errors.extend(report.errors.clone());
                }
                warnings.extend(report.warnings.clone());
                changes_logged.push(format!(
                    "post-restore verification package_valid={} restore_valid={}",
                    report.package_valid,
                    report
                        .restore_valid
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "not_run".to_owned())
                ));
                Some(report)
            }
            Err(error) => {
                errors.push(Warning::new(
                    "post_restore_verification_failed",
                    WarningSeverity::Error,
                    format!("Post-restore verification could not run: {error}"),
                    None,
                ));
                None
            }
        }
    } else {
        None
    };

    let restore_log_json = output_dir.join(RESTORE_LOG_JSON);
    let restore_log_markdown = output_dir.join(RESTORE_LOG_MARKDOWN);
    let result = RestoreResult {
        applied: options.apply,
        changes_logged,
        warnings,
        errors,
        verification,
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
        let destination =
            restore_destination_for_entry(file, &options.target_root, options.restore_user_data)?;

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
            copy_restore_file(&source, &destination, &options.target_root)?;
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
    map: &RestoreMap,
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

        match apply_registry_entry(entry, options.force, map) {
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
    map: &RestoreMap,
    services: &[ServiceEntry],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in services {
        let binary_path = map
            .destination_of(&service.binary_path)
            .map_or_else(|| service.binary_path.clone(), Path::to_path_buf);
        let arguments = service.arguments.as_deref().map(|text| map.rewrite(text));
        let account =
            ServiceAccount::from_recorded(service.account_name.as_deref(), &service.service_name);
        if !options.apply {
            changes_logged.push(format!(
                "would restore service {} -> {} (runs as {})",
                service.service_name,
                service_image_path_value(&binary_path, arguments.as_deref()),
                account.label()
            ));
            if let ServiceAccount::Custom(original) = &account {
                warnings.push(custom_account_warning(service, original));
            }
            continue;
        }
        match apply_service(
            service,
            &binary_path,
            arguments.as_deref(),
            &account,
            &options.target_root,
            options.force,
        ) {
            Ok(messages) => changes_logged.extend(messages),
            Err(warning) => {
                errors.push(warning);
                continue;
            }
        }
        if let ServiceAccount::Custom(original) = &account {
            warnings.push(custom_account_warning(service, original));
        }
        for entry in &service.parameters {
            match apply_registry_entry(entry, true, map) {
                Ok(message) => changes_logged.push(message),
                Err(warning) => errors.push(warning),
            }
        }
    }
}

fn custom_account_warning(service: &ServiceEntry, account: &str) -> Warning {
    Warning::new(
        "service_account_not_restored",
        WarningSeverity::Warning,
        format!(
            "The service {} ran as {account} on the source. Its password is not in the installer, \
             so the service is set up as LocalService. If it does not work, set its account and \
             password in Services.",
            service.service_name
        ),
        Some(service.service_name.clone()),
    )
}

/// Install runtime prerequisites recorded in the manifest. Each prerequisite is
/// probed first and skipped if already present; otherwise the bundled installer
/// (or winget fallback) runs, and the result is re-probed.
fn restore_prerequisites(
    options: &RestoreOptions,
    manifest: &PackageManifest,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for prerequisite in &manifest.runtime_prerequisites {
        let Some(bundle) = prerequisite.bundle.as_ref() else {
            continue;
        };
        // Everything about how this installs comes from the compiled spec, not the
        // manifest. A kind with no spec has nothing that could run.
        let Some(spec) = spec_for_bundle(&prerequisite.kind, bundle) else {
            warnings.push(Warning::new(
                "prerequisite_skipped",
                WarningSeverity::Info,
                format!(
                    "{}: there is no known installer for this runtime.",
                    prerequisite.name
                ),
                None,
            ));
            continue;
        };
        if spec.probe.as_ref().is_some_and(is_present) {
            changes_logged.push(format!(
                "prerequisite already present: {}",
                prerequisite.name
            ));
            continue;
        }
        if !options.apply {
            let via = if spec.windows_feature.is_some() {
                "Windows feature"
            } else if spec.bundleable && bundle.package_relative_path.is_some() {
                "bundled installer"
            } else {
                "winget"
            };
            changes_logged.push(format!(
                "would install prerequisite {} ({via})",
                prerequisite.name
            ));
            continue;
        }
        if spec.requires_admin && !is_elevated() {
            errors.push(Warning::new(
                "prerequisite_requires_admin",
                WarningSeverity::Error,
                format!(
                    "Installing {} requires elevation; re-run the installer as administrator.",
                    prerequisite.name
                ),
                None,
            ));
            continue;
        }
        match install_bundle(&prerequisite.kind, bundle, &options.package_root) {
            PrerequisiteInstallOutcome::Installed => {
                changes_logged.push(format!("installed prerequisite {}", prerequisite.name));
                confirm_prerequisite(prerequisite, &spec, false, warnings, errors);
            }
            PrerequisiteInstallOutcome::InstalledNeedsReboot => {
                changes_logged.push(format!(
                    "installed prerequisite {} (reboot required)",
                    prerequisite.name
                ));
                warnings.push(Warning::new(
                    "prerequisite_reboot_required",
                    WarningSeverity::Warning,
                    format!(
                        "{} was installed but a reboot is required to complete it.",
                        prerequisite.name
                    ),
                    None,
                ));
                confirm_prerequisite(prerequisite, &spec, true, warnings, errors);
            }
            PrerequisiteInstallOutcome::Skipped(reason) => {
                warnings.push(Warning::new(
                    "prerequisite_skipped",
                    WarningSeverity::Info,
                    format!("{}: {reason}", prerequisite.name),
                    None,
                ));
            }
            PrerequisiteInstallOutcome::Failed(reason) => {
                errors.push(Warning::new(
                    "prerequisite_install_failed",
                    WarningSeverity::Error,
                    format!(
                        "Could not install {}: {reason}. Acquire it from {}.",
                        prerequisite.name, spec.download_url
                    ),
                    None,
                ));
            }
        }
    }
}

/// Re-probe a prerequisite after installing it. A failure to register is an
/// error, unless a reboot is pending (then it is only a warning).
fn confirm_prerequisite(
    prerequisite: &RuntimePrerequisite,
    spec: &PrerequisiteSpec,
    reboot_pending: bool,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    let Some(probe) = spec.probe.as_ref() else {
        return;
    };
    if is_present(probe) {
        return;
    }
    if reboot_pending {
        warnings.push(Warning::new(
            "prerequisite_pending_reboot",
            WarningSeverity::Warning,
            format!(
                "{} is not yet registered; it should be active after a reboot.",
                prerequisite.name
            ),
            None,
        ));
    } else {
        errors.push(Warning::new(
            "prerequisite_not_verified",
            WarningSeverity::Error,
            format!(
                "{} reported success but did not register; the install may have failed.",
                prerequisite.name
            ),
            None,
        ));
    }
}

/// Restore captured license data: files and framework-store directories back to
/// their original (or re-anchored) locations, plus transferable registry values.
fn restore_license(
    options: &RestoreOptions,
    manifest: &PackageManifest,
    map: &RestoreMap,
    changes_logged: &mut Vec<String>,
    _warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    // The user (or automation) can opt out of restoring license data even when
    // it was captured; skip both files and transferable registry values.
    if !options.restore_license {
        return;
    }
    for artifact in &manifest.license_artifacts_detected {
        if !artifact.should_copy {
            continue;
        }
        let (Some(relative), Some(original)) = (
            artifact.package_relative_path.as_ref(),
            artifact.path.as_ref(),
        ) else {
            continue;
        };
        let source = match package_destination_path(&options.package_root, relative) {
            Ok(path) => path,
            Err(error) => {
                errors.push(Warning::new(
                    "license_source_invalid",
                    WarningSeverity::Error,
                    format!("Packaged license path is invalid: {error}"),
                    Some(relative.display().to_string()),
                ));
                continue;
            }
        };
        let destination = match license_destination(original, map, options.restore_user_data) {
            Ok(path) => path,
            Err(error) => {
                errors.push(Warning::new(
                    "license_destination_invalid",
                    WarningSeverity::Error,
                    format!("Could not resolve license destination: {error}"),
                    Some(original.display().to_string()),
                ));
                continue;
            }
        };

        if !options.apply {
            changes_logged.push(format!(
                "would restore license {} -> {}",
                source.display(),
                destination.display()
            ));
            continue;
        }
        if destination_requires_admin(&destination) && !is_elevated() {
            errors.push(Warning::new(
                "license_requires_admin",
                WarningSeverity::Error,
                "Restoring license data to a protected location requires elevation.",
                Some(destination.display().to_string()),
            ));
            continue;
        }
        // Honor --force for both files and framework-store directories: never
        // overwrite an existing license/store on the target without it.
        if destination.exists() && !options.force {
            errors.push(Warning::new(
                "license_destination_exists",
                WarningSeverity::Warning,
                "License destination exists; use --force to overwrite.",
                Some(destination.display().to_string()),
            ));
            continue;
        }
        let result = if artifact.store_files.is_empty() {
            copy_restore_file(&source, &destination, &options.target_root)
        } else {
            copy_license_store(artifact, &source, &destination, &options.target_root)
        };
        match result {
            Ok(()) => changes_logged.push(format!(
                "restored license {} -> {}",
                source.display(),
                destination.display()
            )),
            Err(error) => errors.push(Warning::new(
                "license_restore_failed",
                WarningSeverity::Error,
                format!("Could not restore license data: {error}"),
                Some(destination.display().to_string()),
            )),
        }
    }

    for entry in &manifest.license_registry_entries {
        if !options.apply {
            changes_logged.push(format!(
                "would restore license registry {}\\{}",
                registry_hive_name(&entry.hive),
                entry.key_path
            ));
            continue;
        }
        match apply_registry_entry(entry, options.force, map) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
}

/// Resolve where a license artifact should land on the target: re-anchored to
/// the real user-data base when applicable, otherwise the original path with
/// app-install paths rewritten to the target root.
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

/// Copy the files the manifest lists for a license store. They were all hash-checked in
/// the package before anything was written.
fn copy_license_store(
    artifact: &LicenseArtifact,
    source: &Path,
    destination: &Path,
    target_root: &Path,
) -> Result<()> {
    for file in &artifact.store_files {
        let relative = safe_relative_path(&file.relative_path);
        copy_restore_file(
            &package_destination_path(source, &relative)?,
            &package_destination_path(destination, &relative)?,
            target_root,
        )?;
    }
    Ok(())
}

/// Restore application-owned drivers by placing the bundled `.sys` and
/// re-registering the driver service. Inbox/vendor drivers cannot be
/// redistributed and are reported with precise reinstall guidance.
fn restore_drivers(
    options: &RestoreOptions,
    drivers: &[DriverEntry],
    map: &RestoreMap,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for driver in drivers {
        let Some(relative) = driver.package_relative_path.as_ref() else {
            // Not bundled (inbox/vendor driver): precise, actionable guidance.
            changes_logged.push(format!("driver not auto-restored: {}", driver.service_name));
            warnings.push(Warning::new(
                "driver_manual_reinstall",
                WarningSeverity::Warning,
                driver.restore_instructions.clone(),
                Some(driver.service_name.clone()),
            ));
            continue;
        };

        let destination = match driver_destination(driver, map) {
            Ok(path) => path,
            Err(error) => {
                errors.push(Warning::new(
                    "driver_destination_invalid",
                    WarningSeverity::Error,
                    format!("Could not resolve a safe driver destination: {error}"),
                    Some(driver.service_name.clone()),
                ));
                continue;
            }
        };

        if !options.apply {
            changes_logged.push(format!(
                "would install driver {} -> {}",
                driver.service_name,
                destination.display()
            ));
            continue;
        }
        if !is_elevated() {
            errors.push(Warning::new(
                "driver_requires_admin",
                WarningSeverity::Error,
                "Driver registration requires elevation; re-run the installer as administrator.",
                Some(driver.service_name.clone()),
            ));
            continue;
        }
        let source = match package_destination_path(&options.package_root, relative) {
            Ok(path) => path,
            Err(error) => {
                errors.push(Warning::new(
                    "driver_source_invalid",
                    WarningSeverity::Error,
                    format!("Bundled driver path is invalid: {error}"),
                    Some(relative.display().to_string()),
                ));
                continue;
            }
        };
        match place_and_register_driver(driver, &source, &destination, options) {
            Ok(messages) => changes_logged.extend(messages),
            Err(warning) => errors.push(warning),
        }
    }
}

/// Decide whether a driver can be restored before touching anything, then put its image
/// in place and register it. An existing image is not replaced unless `--force` was
/// given, and an image whose hash is wrong is never registered: a driver loads into the
/// kernel and is the last thing to take on trust.
fn place_and_register_driver(
    driver: &DriverEntry,
    source: &Path,
    destination: &Path,
    options: &RestoreOptions,
) -> std::result::Result<Vec<String>, Warning> {
    let name = driver.service_name.as_str();
    let problem = |code: &'static str, message: String| {
        Warning::new(code, WarningSeverity::Error, message, Some(name.to_owned()))
    };
    if !is_valid_service_name(name) {
        return Err(problem(
            "driver_name_invalid",
            "The driver's name is not one Windows accepts, so it was not set up.".to_owned(),
        ));
    }
    let current = service_image_path(name).map_err(|error| {
        problem(
            "driver_query_failed",
            format!("Could not read the driver's current settings: {error}"),
        )
    })?;
    if let Some(current) = current.as_deref() {
        if !options.force {
            return Err(Warning::new(
                "driver_exists",
                WarningSeverity::Warning,
                "Driver service exists; use --force to reconfigure it.",
                Some(name.to_owned()),
            ));
        }
        if let Some(refusal) =
            existing_service_refusal(name, current, &options.target_root, system_root())
        {
            return Err(problem("driver_exists_elsewhere", refusal));
        }
    }
    if destination.exists() && !options.force {
        return Err(Warning::new(
            "driver_image_exists",
            WarningSeverity::Warning,
            "Driver image exists; use --force to overwrite.",
            Some(destination.display().to_string()),
        ));
    }
    let matches_manifest = |path: &Path| match driver.hash_sha256.as_deref() {
        Some(expected) => {
            sha256_file(path).is_ok_and(|actual| actual.eq_ignore_ascii_case(expected))
        }
        None => false,
    };
    if !matches_manifest(source) {
        return Err(problem(
            "driver_hash_failed",
            "The bundled driver image does not match its recorded hash, so it was not installed."
                .to_owned(),
        ));
    }
    copy_restore_file(source, destination, &options.target_root).map_err(|error| {
        problem(
            "driver_copy_failed",
            format!("Could not place driver image: {error}"),
        )
    })?;
    if !matches_manifest(destination) {
        return Err(problem(
            "driver_hash_failed",
            "The driver image changed after it was placed, so it was not registered.".to_owned(),
        ));
    }
    let mut messages = vec![format!(
        "placed driver image {} -> {}",
        source.display(),
        destination.display()
    )];
    messages.push(apply_driver(driver, destination, current.is_some())?);
    Ok(messages)
}

/// The arguments for `sc.exe`. A driver's `ImagePath` is a path, not a command line, and
/// the kernel does not remove quotes from it, so unlike a service's it stays bare.
fn sc_driver_arguments(driver: &DriverEntry, image_path: &Path, exists: bool) -> Vec<OsString> {
    let mut command: Vec<OsString> = vec![
        if exists { "config" } else { "create" }.into(),
        driver.service_name.as_str().into(),
        "type=".into(),
        driver_sc_type(&driver.driver_kind).into(),
        "binPath=".into(),
        image_path.into(),
    ];
    if !exists && let Some(start_type) = driver.start_type.as_deref().and_then(sc_start_type) {
        command.extend(["start=".into(), start_type.into()]);
    }
    command
}

fn apply_driver(
    driver: &DriverEntry,
    image_path: &Path,
    exists: bool,
) -> std::result::Result<String, Warning> {
    let mut command = hidden_command(SC_EXE);
    command.args(sc_driver_arguments(driver, image_path, exists));
    let (code, done) = if exists {
        ("driver_config_failed", "configured")
    } else {
        ("driver_create_failed", "created")
    };
    run_checked(&mut command, code, &driver.service_name)?;
    Ok(format!("{done} driver {}", driver.service_name))
}

fn driver_sc_type(kind: &DriverKind) -> &'static str {
    match kind {
        DriverKind::Kernel | DriverKind::Unknown => "kernel",
        DriverKind::FileSystem => "filesys",
        DriverKind::Recognizer => "rec",
    }
}

fn restore_scheduled_tasks(
    options: &RestoreOptions,
    tasks: &[ScheduledTaskEntry],
    map: &RestoreMap,
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
        match apply_scheduled_task(task, options.force, map) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
}

fn restore_shortcuts(
    options: &RestoreOptions,
    map: &RestoreMap,
    shortcuts: &[ShortcutEntry],
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in shortcuts {
        let target_path = map
            .destination_of(&shortcut.target_path)
            .map_or_else(|| shortcut.target_path.clone(), Path::to_path_buf);
        let working_directory = shortcut
            .working_directory
            .as_ref()
            .map(|working| PathBuf::from(map.rewrite(&working.to_string_lossy())));
        let icon_path = shortcut
            .icon_path
            .as_ref()
            .map(|icon| PathBuf::from(map.rewrite(&icon.to_string_lossy())));

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

/// Hash-check the package before anything is written, so a truncated or damaged
/// copy is refused instead of running prerequisites and overwriting the app first.
fn refuse_damaged_package(options: &RestoreOptions, manifest: &PackageManifest) -> Result<()> {
    let report = verify_prepared_package(
        &options.package_root,
        manifest,
        &VerificationOptions {
            package_root: options.package_root.clone(),
            target_root: None,
            output_dir: None,
            restore_user_data: options.restore_user_data,
            restore_license: options.restore_license,
        },
    )?;
    if report.package_valid {
        return Ok(());
    }
    let detail = report
        .errors
        .first()
        .map_or_else(String::new, |error| format!(": {}", error.message));
    Err(WinAppMigratorError::InvalidInput(format!(
        "the package failed its hash check and nothing was installed{detail}"
    )))
}

fn restore_environment(
    options: &RestoreOptions,
    environment_variables: &[EnvironmentVariableEntry],
    map: &RestoreMap,
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
        match apply_environment_variable(variable, options.force, &options.target_root, map) {
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
    map: &RestoreMap,
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

    let value = registry_value_from_entry(entry, map)?;
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
    target_root: &Path,
    map: &RestoreMap,
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
    let existing = key.get_raw_value(&variable.name).ok();
    let is_path = is_path_variable(&variable.name);
    if existing.is_some() && !is_path && !force {
        return Err(Warning::new(
            "environment_value_exists",
            WarningSeverity::Warning,
            "Environment variable exists; use --force to overwrite.",
            Some(format!("{:?}\\{}", variable.scope, variable.name)),
        ));
    }

    let rewritten = map.rewrite(&variable.value);
    let (data, vtype) = if is_path {
        let current = key
            .get_value::<String, _>(&variable.name)
            .unwrap_or_default();
        let entries = app_path_entries(&rewritten, target_root);
        let vtype = existing.map_or_else(
            || registry_raw_type(&variable.value_kind),
            |value| value.vtype,
        );
        (merge_path_entries(&current, &entries), vtype)
    } else {
        (rewritten, registry_raw_type(&variable.value_kind))
    };
    let value = RegValue {
        bytes: Cow::Owned(encode_utf16_nul(&data)),
        vtype,
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
    map: &RestoreMap,
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
            let rewritten = map.rewrite(entry.value_data.as_deref().unwrap_or_default());
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

/// The arguments for `sc.exe create` or `sc.exe config`, apart from what they run.
fn sc_service_arguments(
    verb: &str,
    service: &ServiceEntry,
    binary_path: &Path,
    arguments: Option<&str>,
    account: &ServiceAccount,
) -> Vec<OsString> {
    let mut command: Vec<OsString> = vec![
        verb.into(),
        service.service_name.as_str().into(),
        "binPath=".into(),
        service_image_path_value(binary_path, arguments).into(),
    ];
    if let Some(start_type) = service.start_type.as_deref().and_then(sc_start_type) {
        command.extend(["start=".into(), start_type.into()]);
    }
    if let Some(display_name) = service.display_name.as_deref() {
        command.extend(["DisplayName=".into(), display_name.into()]);
    }
    if let Some(object_name) = account.sc_object_name() {
        command.extend(["obj=".into(), object_name.into()]);
        if account.sc_takes_empty_password() {
            command.extend(["password=".into(), "".into()]);
        }
    }
    command
}

fn apply_service(
    service: &ServiceEntry,
    binary_path: &Path,
    arguments: Option<&str>,
    account: &ServiceAccount,
    target_root: &Path,
    force: bool,
) -> std::result::Result<Vec<String>, Warning> {
    let name = service.service_name.as_str();
    if !is_valid_service_name(name) {
        return Err(Warning::new(
            "service_name_invalid",
            WarningSeverity::Error,
            "The service's name is not one Windows accepts, so it was not set up.",
            Some(name.to_owned()),
        ));
    }
    let current = service_image_path(name).map_err(|error| {
        Warning::new(
            "service_query_failed",
            WarningSeverity::Error,
            format!("Could not read the service's current settings: {error}"),
            Some(name.to_owned()),
        )
    })?;
    if current.is_some() && !force {
        return Err(Warning::new(
            "service_exists",
            WarningSeverity::Warning,
            "Service exists; use --force to reconfigure it.",
            Some(name.to_owned()),
        ));
    }
    if let Some(refusal) = current
        .as_deref()
        .and_then(|current| existing_service_refusal(name, current, target_root, system_root()))
    {
        return Err(Warning::new(
            "service_exists_elsewhere",
            WarningSeverity::Error,
            refusal,
            Some(name.to_owned()),
        ));
    }

    let (verb, error_code, done) = if current.is_some() {
        ("config", "service_config_failed", "configured")
    } else {
        ("create", "service_create_failed", "created")
    };
    let mut command = hidden_command(SC_EXE);
    command.args(sc_service_arguments(
        verb,
        service,
        binary_path,
        arguments,
        account,
    ));
    run_checked(&mut command, error_code, name)?;
    let messages = vec![format!(
        "{done} service {name} (runs as {})",
        account.label()
    )];

    if let Some(description) = service.description.as_deref() {
        let mut command = hidden_command(SC_EXE);
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
        let mut command = hidden_command(SC_EXE);
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
    map: &RestoreMap,
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

    let rewritten_xml = map.rewrite_xml(xml).map_err(|error| {
        Warning::new(
            "scheduled_task_xml_invalid",
            WarningSeverity::Error,
            format!("The scheduled task's definition could not be read: {error}"),
            Some(format!("{}{}", task.task_path, task.task_name)),
        )
    })?;
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
        escape_powershell_single(xml_path.display().to_string().as_str()),
        escape_powershell_single(&task.task_name),
        escape_powershell_single(&task.task_path),
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
        escape_powershell_single(shortcut.original_lnk_path.display().to_string().as_str()),
        escape_powershell_single(target_path.display().to_string().as_str()),
    );
    if let Some(arguments) = shortcut.arguments.as_deref() {
        script.push_str(&format!(
            " $shortcut.Arguments = '{}';",
            escape_powershell_single(arguments)
        ));
    }
    if let Some(working_directory) = working_directory {
        script.push_str(&format!(
            " $shortcut.WorkingDirectory = '{}';",
            escape_powershell_single(working_directory.display().to_string().as_str())
        ));
    }
    if let Some(icon_path) = icon_path {
        script.push_str(&format!(
            " $shortcut.IconLocation = '{}';",
            escape_powershell_single(icon_path.display().to_string().as_str())
        ));
    }
    script.push_str(" $shortcut.Save();");
    run_powershell_script(&script)
        .map(|_| format!("restored shortcut {}", shortcut.original_lnk_path.display()))
}

/// Put a file from the package at its destination. It is written beside the destination
/// under a new name, then renamed over it. A rename replaces whatever is at the
/// destination and cannot be carried through it, unlike opening it for writing, which
/// follows a link and also writes through a hard link. It also means a failure part-way
/// through a file leaves the old one in place.
fn copy_restore_file(source: &Path, destination: &Path, target_root: &Path) -> Result<()> {
    let root = write_root_for(destination, target_root).ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "refusing to write outside the install location and the known data folders: {}",
            destination.display()
        ))
    })?;
    ensure_within_root(&root, destination)?;
    let parent = destination.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "destination has no parent folder: {}",
            destination.display()
        ))
    })?;
    fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;

    let partial = parent.join(partial_file_name());
    let copied = write_partial_copy(source, &partial).and_then(|()| {
        fs::rename(&partial, destination)
            .map_err(|error| WinAppMigratorError::io(destination, error))
    });
    if copied.is_err() {
        let _ = fs::remove_file(&partial);
    }
    copied
}

/// A name no other copy in this run is using. It is opened with `create_new`, which
/// fails on anything already there, so the name does not need to be secret.
fn partial_file_name() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    format!(
        ".wam-{}-{}.partial",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

fn write_partial_copy(source: &Path, partial: &Path) -> Result<()> {
    let mut input = File::open(source).map_err(|error| WinAppMigratorError::io(source, error))?;
    let output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(partial)
        .map_err(|error| WinAppMigratorError::io(partial, error))?;
    let mut output = BufWriter::with_capacity(COPY_BUFFER_BYTES, output);
    io::copy(&mut input, &mut output).map_err(|error| WinAppMigratorError::io(partial, error))?;
    drop(
        output
            .into_inner()
            .map_err(|error| WinAppMigratorError::io(partial, error.into_error()))?,
    );
    let metadata = input
        .metadata()
        .map_err(|error| WinAppMigratorError::io(source, error))?;
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    set_file_times(partial, accessed, modified)
        .map_err(|error| WinAppMigratorError::io(partial, error))?;
    fs::set_permissions(partial, metadata.permissions())
        .map_err(|error| WinAppMigratorError::io(partial, error))?;
    Ok(())
}

fn write_restore_logs(
    json_path: &Path,
    markdown_path: &Path,
    result: &RestoreResult,
) -> Result<()> {
    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
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
    if let Some(verification) = result.verification.as_ref() {
        markdown.push_str("\n## Verification\n");
        markdown.push_str(&format!(
            "- Package valid: {}\n",
            verification.package_valid
        ));
        markdown.push_str(&format!(
            "- Restore valid: {}\n",
            verification
                .restore_valid
                .map(|value| value.to_string())
                .unwrap_or_else(|| "not run".to_owned())
        ));
        markdown.push_str(&format!("- Items checked: {}\n", verification.items.len()));
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

fn restore_output_dir(original_package_root: &Path, effective_package_root: &Path) -> PathBuf {
    if original_package_root.is_file() {
        original_package_root
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| effective_package_root.to_path_buf())
    } else {
        effective_package_root.to_path_buf()
    }
}

fn restore_requires_admin(plan: &RestorePlan) -> bool {
    plan.planned_registry_writes
        .iter()
        .any(|entry| entry.requires_admin)
        || !plan.planned_service_changes.is_empty()
        || plan
            .planned_task_changes
            .iter()
            .any(|task| task.requires_admin)
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

/// True when any planned user-data file will be restored to a machine-wide
/// protected location (for example ProgramData) and therefore needs elevation.
fn user_data_restore_requires_admin(options: &RestoreOptions, plan: &RestorePlan) -> bool {
    plan.planned_file_writes.iter().any(|file| {
        restore_destination_for_entry(file, &options.target_root, options.restore_user_data)
            .map(|destination| destination_requires_admin(&destination))
            .unwrap_or(false)
    })
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

fn scheduled_task_exists(task: &ScheduledTaskEntry) -> std::result::Result<bool, Warning> {
    let script = format!(
        "$task = Get-ScheduledTask -TaskName '{}' -TaskPath '{}' -ErrorAction SilentlyContinue; if ($null -eq $task) {{ exit 1 }}",
        escape_powershell_single(&task.task_name),
        escape_powershell_single(&task.task_path)
    );
    let output = hidden_command(POWERSHELL_EXE)
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
    let output = hidden_command(POWERSHELL_EXE)
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
            restore_user_data: false,
            restore_license: true,
        };

        assert!(restore_package(&options).is_err());
    }

    #[test]
    fn a_parent_step_cannot_carry_a_driver_out_of_the_install_location() -> Result<()> {
        let driver = test_driver(r"C:\App\..\..\Windows\System32\drivers\flt.sys");
        let destination = driver_destination(&driver, &app_map()?)?;
        assert_eq!(destination, PathBuf::from(r"D:\Restored\drivers\flt.sys"));
        Ok(())
    }

    /// The opposite of a service: quotes here would stop the driver from loading.
    #[test]
    fn a_drivers_path_reaches_sc_without_quotes() {
        let driver = test_driver(r"C:\Program Files\Acme\flt.sys");
        let arguments =
            sc_driver_arguments(&driver, Path::new(r"D:\Program Files\Acme\flt.sys"), false);
        assert_eq!(
            strings(&arguments),
            [
                "create",
                "AcmeFlt",
                "type=",
                "kernel",
                "binPath=",
                r"D:\Program Files\Acme\flt.sys",
                "start=",
                "demand",
            ]
        );
    }

    fn driver_options(work: &Path, force: bool) -> RestoreOptions {
        RestoreOptions {
            package_root: work.join("package"),
            target_root: work.join("target"),
            dry_run: false,
            apply: true,
            force,
            restore_user_data: false,
            restore_license: true,
        }
    }

    /// A service name no real PC has, so the query answers "not there" and nothing is
    /// written to the registry. These all stop before registering.
    fn unregistered_driver(hash: Option<String>) -> DriverEntry {
        let mut driver = test_driver(r"C:\App\drivers\flt.sys");
        driver.service_name = "WamTestDriverThatDoesNotExist".to_owned();
        driver.hash_sha256 = hash;
        driver
    }

    #[test]
    fn a_driver_image_with_the_wrong_hash_is_not_placed() -> Result<()> {
        let work = temp_folder()?;
        let options = driver_options(work.path(), true);
        let source = work.path().join(r"package\drivers\flt.sys");
        write_file(&source, b"tampered")?;
        fs::create_dir_all(&options.target_root)
            .map_err(|error| WinAppMigratorError::io(&options.target_root, error))?;
        let destination = options.target_root.join(r"drivers\flt.sys");

        for hash in [None, Some("00".repeat(32))] {
            let refused = place_and_register_driver(
                &unregistered_driver(hash),
                &source,
                &destination,
                &options,
            );
            assert_eq!(
                refused.err().map(|warning| warning.code).as_deref(),
                Some("driver_hash_failed")
            );
            assert!(!destination.exists());
        }
        Ok(())
    }

    #[test]
    fn an_existing_driver_image_is_not_overwritten_without_force() -> Result<()> {
        let work = temp_folder()?;
        let options = driver_options(work.path(), false);
        let source = work.path().join(r"package\drivers\flt.sys");
        write_file(&source, b"new")?;
        let destination = options.target_root.join(r"drivers\flt.sys");
        write_file(&destination, b"existing")?;

        let refused = place_and_register_driver(
            &unregistered_driver(Some(sha256_file(&source)?)),
            &source,
            &destination,
            &options,
        );

        assert_eq!(
            refused.err().map(|warning| warning.code).as_deref(),
            Some("driver_image_exists")
        );
        assert_eq!(
            fs::read(&destination).map_err(|error| WinAppMigratorError::io(&destination, error))?,
            b"existing"
        );
        Ok(())
    }

    fn sample_service() -> ServiceEntry {
        ServiceEntry {
            service_name: "AcmeSync".to_owned(),
            display_name: Some("Acme Sync".to_owned()),
            binary_path: PathBuf::from(r"C:\My Apps\Foo Bar\svc.exe"),
            arguments: Some("--run".to_owned()),
            parameters: Vec::new(),
            start_type: Some("automatic".to_owned()),
            account_name: Some(r"NT AUTHORITY\LocalService".to_owned()),
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: String::new(),
        }
    }

    fn strings(arguments: &[OsString]) -> Vec<String> {
        arguments
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect()
    }

    /// What reaches `sc.exe`, and so the registry, keeps its quotes and arguments.
    #[test]
    fn sc_receives_a_quoted_path_its_arguments_and_the_account() {
        let service = sample_service();
        let account =
            ServiceAccount::from_recorded(service.account_name.as_deref(), &service.service_name);
        let arguments = sc_service_arguments(
            "create",
            &service,
            Path::new(r"D:\My Apps\Foo Bar\svc.exe"),
            Some("--run"),
            &account,
        );
        assert_eq!(
            strings(&arguments),
            [
                "create",
                "AcmeSync",
                "binPath=",
                r#""D:\My Apps\Foo Bar\svc.exe" --run"#,
                "start=",
                "auto",
                "DisplayName=",
                "Acme Sync",
                "obj=",
                r"NT AUTHORITY\LocalService",
                "password=",
                "",
            ]
        );
        let mut command = hidden_command(SC_EXE);
        command.args(&arguments);
        let passed: Vec<_> = command.get_args().collect();
        assert_eq!(passed[3], r#""D:\My Apps\Foo Bar\svc.exe" --run"#);
    }

    #[test]
    fn a_service_with_no_recorded_account_passes_no_obj() {
        let mut service = sample_service();
        service.account_name = None;
        service.display_name = None;
        service.start_type = None;
        let arguments = sc_service_arguments(
            "config",
            &service,
            Path::new(r"D:\App\svc.exe"),
            None,
            &ServiceAccount::Unspecified,
        );
        assert_eq!(
            strings(&arguments),
            ["config", "AcmeSync", "binPath=", r#""D:\App\svc.exe""#]
        );
    }

    fn write_file(path: &Path, bytes: &[u8]) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
        }
        fs::write(path, bytes).map_err(|error| WinAppMigratorError::io(path, error))
    }

    fn temp_folder() -> Result<tempfile::TempDir> {
        tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp folder", error))
    }

    #[test]
    fn copying_creates_the_file_and_leaves_nothing_else_behind() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"new")?;
        let root = work.path().join("target");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let destination = root.join(r"bin\app.exe");

        copy_restore_file(&source, &destination, &root)?;
        write_file(&source, b"newer")?;
        copy_restore_file(&source, &destination, &root)?;

        assert_eq!(
            fs::read(&destination).map_err(|error| WinAppMigratorError::io(&destination, error))?,
            b"newer"
        );
        let names: Vec<_> = fs::read_dir(root.join("bin"))
            .map_err(|error| WinAppMigratorError::io(&root, error))?
            .flatten()
            .map(|entry| entry.file_name())
            .collect();
        assert_eq!(names, ["app.exe"]);
        Ok(())
    }

    #[test]
    fn copying_refuses_a_destination_outside_the_known_folders() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"x")?;
        let root = work.path().join("target");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        // Not a temp path: temp folders sit inside %LOCALAPPDATA%, which is a known folder.
        let outside = Path::new(r"Z:\wam-not-a-known-folder\outside.bin");

        let refused = copy_restore_file(&source, outside, &root);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("refusing")),
            "{refused:?}"
        );
        Ok(())
    }

    #[test]
    fn copying_replaces_a_hard_link_instead_of_writing_through_it() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"new")?;
        let root = work.path().join("target");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let victim = work.path().join("victim.bin");
        write_file(&victim, b"keep")?;
        let destination = root.join("app.exe");
        fs::hard_link(&victim, &destination)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;

        copy_restore_file(&source, &destination, &root)?;

        assert_eq!(
            fs::read(&victim).map_err(|error| WinAppMigratorError::io(&victim, error))?,
            b"keep"
        );
        assert_eq!(
            fs::read(&destination).map_err(|error| WinAppMigratorError::io(&destination, error))?,
            b"new"
        );
        Ok(())
    }

    #[test]
    fn copying_refuses_a_file_symbolic_link_at_the_destination() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"new")?;
        let root = work.path().join("target");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let victim = work.path().join("victim.bin");
        write_file(&victim, b"keep")?;
        let destination = root.join("app.exe");
        // Symbolic links need Developer Mode or elevation, so this cannot always run.
        if std::os::windows::fs::symlink_file(&victim, &destination).is_err() {
            eprintln!("skipping: this shell cannot create a symbolic link");
            return Ok(());
        }

        assert!(copy_restore_file(&source, &destination, &root).is_err());
        assert_eq!(
            fs::read(&victim).map_err(|error| WinAppMigratorError::io(&victim, error))?,
            b"keep"
        );
        Ok(())
    }

    fn app_map() -> Result<RestoreMap> {
        let manifest: PackageManifest = serde_json::from_str(
            r#"{
                "schema_version": "",
                "source_computer_name": null,
                "source_windows_version": null,
                "scan_timestamp_utc": "",
                "main_executable_path": "C:\\App\\app.exe",
                "application_metadata": null,
                "file_inventory": [],
                "registry_keys_exported": [],
                "services_detected": [],
                "scheduled_tasks_detected": [],
                "shortcuts_detected": [],
                "environment_variables_detected": [],
                "runtime_prerequisites": [],
                "warnings": [],
                "items_intentionally_excluded": []
            }"#,
        )?;
        RestoreMap::new(&manifest, Path::new(r"D:\Restored"), false)
    }

    #[test]
    fn license_destination_rewrites_app_paths() -> Result<()> {
        let destination =
            license_destination(Path::new(r"C:\App\license.lic"), &app_map()?, false)?;
        assert_eq!(destination, PathBuf::from(r"D:\Restored\license.lic"));
        Ok(())
    }

    #[test]
    fn driver_sc_type_maps_each_kind() {
        assert_eq!(driver_sc_type(&DriverKind::Kernel), "kernel");
        assert_eq!(driver_sc_type(&DriverKind::FileSystem), "filesys");
        assert_eq!(driver_sc_type(&DriverKind::Recognizer), "rec");
        assert_eq!(driver_sc_type(&DriverKind::Unknown), "kernel");
    }

    fn test_driver(image_path: &str) -> DriverEntry {
        DriverEntry {
            service_name: "AcmeFlt".to_owned(),
            display_name: None,
            image_path: PathBuf::from(image_path),
            driver_kind: DriverKind::Kernel,
            start_type: Some("manual".to_owned()),
            account_name: None,
            description: None,
            requires_admin: true,
            restore_instructions: String::new(),
            app_owned: true,
            signer: None,
            hash_sha256: None,
            package_relative_path: Some(PathBuf::from("drivers/AcmeFlt_flt.sys")),
        }
    }

    #[test]
    fn driver_destination_reanchors_under_target_root() -> Result<()> {
        let driver = test_driver(r"C:\App\drivers\flt.sys");
        let destination = driver_destination(&driver, &app_map()?)?;
        assert_eq!(destination, PathBuf::from(r"D:\Restored\drivers\flt.sys"));
        Ok(())
    }

    #[test]
    fn driver_destination_never_escapes_target_root() -> Result<()> {
        let target = PathBuf::from(r"D:\Restored");
        // No rewrite matches a machine path, so it must fall back under target.
        let driver = test_driver(r"C:\Windows\System32\drivers\flt.sys");
        let destination = driver_destination(&driver, &app_map()?)?;
        assert!(
            destination.starts_with(&target),
            "driver destination {destination:?} escaped target root"
        );
        Ok(())
    }
}
