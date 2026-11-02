use std::borrow::Cow;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use filetime::{FileTime, set_file_times};
use winreg::RegValue;
use winreg::enums::REG_NONE;

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    ensure_target_root, ensure_within_root, package_destination_path, safe_relative_path,
    split_user_data_relative, write_root_for,
};
use crate::hashing::sha256_file;
use crate::models::{
    DriverEntry, DriverKind, EnvironmentScope, EnvironmentVariableEntry, LicenseArtifact,
    PackageManifest, RegistryEntry, RegistryHive, RegistryValueKind, RestoreOptions, RestoreResult,
    RuntimePrerequisite, ServiceEntry, VerificationOptions, Warning, WarningSeverity,
};
use crate::package::{PreparedPackage, read_manifest};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{escape_powershell_single, hidden_command, system_root};
use crate::prerequisites::{
    PrerequisiteInstallOutcome, PrerequisiteSpec, install_bundle, is_present, spec_for_bundle,
};
use crate::privilege::{destination_requires_admin, is_elevated, target_requires_admin};
use crate::registry_target::{
    app_path_entries, environment_registry_target, is_path_variable, merge_path_entries,
    registry_hive_name, registry_raw_type, registry_root, registry_value_name, service_image_path,
};
use crate::restore_policy::{
    COM_SERVER_KEYS, ServiceAccount, com_server_refusal, existing_service_refusal,
    existing_task_refusal, is_autostart_entry, is_com_server_entry, paths_equal,
    restored_value_bytes, restored_value_text, service_image_path_value,
};
use crate::restore_review::{
    ReviewedDriver, ReviewedPackage, ReviewedService, ReviewedShortcut, ReviewedTask,
    install_location_problem,
};
use crate::task_scan::live_task_xml;
use crate::verify::verify_prepared_package;

const RESTORE_LOG_JSON: &str = "restore_log.json";
const RESTORE_LOG_MARKDOWN: &str = "restore_log.md";
const POWERSHELL_EXE: &str = "powershell.exe";
const SC_EXE: &str = "sc.exe";
const REFUSED_LOG_HEADING: &str = "\n## Refused\n";
/// Buffer for copying a file out of the package. Large files are the slow ones.
const COPY_BUFFER_BYTES: usize = 1024 * 1024;

/// What a restore did or would do, kept apart so what reaches beyond files can be listed
/// first. A preview of a large app is otherwise thousands of files before anything else.
#[derive(Default)]
struct Changes {
    services: Vec<String>,
    drivers: Vec<String>,
    tasks: Vec<String>,
    autostart: Vec<String>,
    shortcuts: Vec<String>,
    environment: Vec<String>,
    prerequisites: Vec<String>,
    registry: Vec<String>,
    license: Vec<String>,
    files: Vec<String>,
}

impl Changes {
    fn in_order(self) -> Vec<String> {
        [
            self.services,
            self.drivers,
            self.tasks,
            self.autostart,
            self.shortcuts,
            self.environment,
            self.prerequisites,
            self.registry,
            self.license,
            self.files,
        ]
        .concat()
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
    if let Some(problem) = install_location_problem(&options.target_root) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "that install location cannot be used: {problem}: \"{}\"",
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

    let review = ReviewedPackage::new(
        &manifest,
        &options.target_root,
        options.restore_user_data,
        options.restore_license,
    )?;
    if options.apply && restore_requires_admin(&review) && !is_elevated() {
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
        && user_data_restore_requires_admin(&review)
    {
        return Err(WinAppMigratorError::InvalidInput(
            "restoring user data into ProgramData or other protected locations requires elevated privileges"
                .to_owned(),
        ));
    }
    if options.apply && !options.force {
        refuse_replacing_anything(&review)?;
    }
    // Held until the install has finished and been checked.
    let _target_hold = if options.apply {
        Some(ensure_target_root(&options.target_root)?)
    } else {
        None
    };

    let output_dir = restore_output_dir(&options.package_root, &effective_options.package_root);
    let mut changes = Changes::default();
    let mut warnings = manifest.warnings.clone();
    warnings.extend(review.warnings.iter().cloned());
    if options.apply
        && let Some(warning) = install_location_link_warning(&options.target_root)
    {
        warnings.push(warning);
    }
    let mut errors = Vec::new();
    if options.restore_user_data
        && review
            .files
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
        &mut changes.prerequisites,
        &mut warnings,
        &mut errors,
    );
    restore_files(
        &effective_options,
        &review,
        &mut changes.files,
        &mut warnings,
        &mut errors,
    )?;
    // Services first, so a registry key under one can never make it look like it already
    // existed.
    restore_services(
        &effective_options,
        &review,
        &mut changes.services,
        &mut warnings,
        &mut errors,
    );
    restore_drivers(
        &effective_options,
        &review,
        &mut changes.drivers,
        &mut warnings,
        &mut errors,
    );
    restore_registry(
        &effective_options,
        &review,
        &mut changes,
        &mut warnings,
        &mut errors,
    );
    restore_license(
        &effective_options,
        &review,
        &mut changes.license,
        &mut errors,
    );
    restore_scheduled_tasks(
        &effective_options,
        &review,
        &mut changes.tasks,
        &mut warnings,
        &mut errors,
    );
    restore_shortcuts(
        &effective_options,
        &review,
        &mut changes,
        &mut warnings,
        &mut errors,
    );
    restore_environment(
        &effective_options,
        &review,
        &mut changes.environment,
        &mut warnings,
        &mut errors,
    );
    let mut changes_logged = changes.in_order();

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
        refused: review.refused,
        verification,
        restore_log_json: Some(restore_log_json.clone()),
        restore_log_markdown: Some(restore_log_markdown.clone()),
    };
    write_restore_logs(&restore_log_json, &restore_log_markdown, &result)?;
    Ok(result)
}

/// Without `--force`, nothing already there gets replaced, so if anything is there the
/// install does not start. Going on would register services and tasks against whatever is
/// at those paths, which could be a file another program put there for that purpose.
fn refuse_replacing_anything(review: &ReviewedPackage<'_>) -> Result<()> {
    let existing = review
        .files
        .iter()
        .filter_map(|file| review.map.destination_of(&file.original_path))
        .find(|destination| destination.exists());
    match existing {
        Some(destination) => Err(WinAppMigratorError::InvalidInput(format!(
            "{} already exists, so nothing was installed. Use --force to replace it",
            destination.display()
        ))),
        None => Ok(()),
    }
}

/// An install location can sit under a link higher up, such as a volume mounted on a folder.
/// That can be intended, so it is reported rather than refused.
fn install_location_link_warning(target_root: &Path) -> Option<Warning> {
    let actual = fs::canonicalize(target_root).ok()?;
    let text = actual.to_string_lossy();
    let actual = text.strip_prefix(r"\\?\").unwrap_or(&text);
    (!paths_equal(Path::new(actual), target_root)).then(|| {
        Warning::new(
            "install_location_is_a_link",
            WarningSeverity::Warning,
            format!(
                "The install location is really {actual}, because a folder above it is a link."
            ),
            Some(target_root.display().to_string()),
        )
    })
}

fn restore_files(
    options: &RestoreOptions,
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) -> Result<()> {
    for file in &review.files {
        let (Some(relative), Some(destination)) = (
            file.package_relative_path.as_ref(),
            review.map.destination_of(&file.original_path),
        ) else {
            continue;
        };
        let source = package_destination_path(&options.package_root, relative)?;

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
            copy_restore_file(&source, destination, &options.target_root)?;
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
    review: &ReviewedPackage<'_>,
    changes: &mut Changes,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for entry in &review.registry {
        let log = if is_autostart_entry(entry) {
            &mut changes.autostart
        } else {
            &mut changes.registry
        };
        if !options.apply {
            log.push(format!(
                "would restore registry {}\\{} {}",
                registry_hive_name(&entry.hive),
                entry.key_path,
                entry.value_name.as_deref().unwrap_or("(key)")
            ));
            continue;
        }

        match apply_registry_entry(entry, options.force, &review.map) {
            Ok(message) => log.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if !review.registry.is_empty() {
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
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for service in &review.services {
        if let ServiceAccount::Custom(original) = &service.account {
            warnings.push(custom_account_warning(service.entry, original));
        }
        if !options.apply {
            changes_logged.push(format!(
                "would restore service {} -> {} (runs as {})",
                service.entry.service_name,
                service_image_path_value(&service.binary, service.arguments.as_deref()),
                service.account.label()
            ));
            continue;
        }
        match apply_service(service, &options.target_root, options.force) {
            Ok(messages) => changes_logged.extend(messages),
            Err(warning) => {
                errors.push(warning);
                continue;
            }
        }
        for entry in &service.entry.parameters {
            match apply_registry_entry(entry, true, &review.map) {
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
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    errors: &mut Vec<Warning>,
) {
    for license in &review.license {
        let destination = &license.destination;
        let source =
            match package_destination_path(&options.package_root, license.package_relative_path) {
                Ok(path) => path,
                Err(error) => {
                    errors.push(Warning::new(
                        "license_source_invalid",
                        WarningSeverity::Error,
                        format!("Packaged license path is invalid: {error}"),
                        Some(license.package_relative_path.display().to_string()),
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
        if destination_requires_admin(destination) && !is_elevated() {
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
        let result = if license.artifact.store_files.is_empty() {
            copy_restore_file(&source, destination, &options.target_root)
        } else {
            copy_license_store(license.artifact, &source, destination, &options.target_root)
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

    for entry in &review.license_registry {
        if !options.apply {
            changes_logged.push(format!(
                "would restore license registry {}\\{}",
                registry_hive_name(&entry.hive),
                entry.key_path
            ));
            continue;
        }
        match apply_registry_entry(entry, options.force, &review.map) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
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
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for driver in &review.manual_drivers {
        // Not bundled (inbox/vendor driver): precise, actionable guidance.
        changes_logged.push(format!("driver not auto-restored: {}", driver.service_name));
        warnings.push(Warning::new(
            "driver_manual_reinstall",
            WarningSeverity::Warning,
            driver.restore_instructions.clone(),
            Some(driver.service_name.clone()),
        ));
    }
    for driver in &review.drivers {
        let name = &driver.entry.service_name;
        if !options.apply {
            changes_logged.push(format!(
                "would install driver {name} -> {}",
                driver.destination.display()
            ));
            continue;
        }
        if !is_elevated() {
            errors.push(Warning::new(
                "driver_requires_admin",
                WarningSeverity::Error,
                "Driver registration requires elevation; re-run the installer as administrator.",
                Some(name.clone()),
            ));
            continue;
        }
        match place_and_register_driver(driver, options) {
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
    reviewed: &ReviewedDriver<'_>,
    options: &RestoreOptions,
) -> std::result::Result<Vec<String>, Warning> {
    let driver = reviewed.entry;
    let destination = reviewed.destination.as_path();
    let name = driver.service_name.as_str();
    let problem = |code: &'static str, message: String| {
        Warning::new(code, WarningSeverity::Error, message, Some(name.to_owned()))
    };
    let source = package_destination_path(&options.package_root, reviewed.package_relative_path)
        .map_err(|error| {
            problem(
                "driver_source_invalid",
                format!("Bundled driver path is invalid: {error}"),
            )
        })?;
    let source = source.as_path();
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
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for task in &review.tasks {
        let name = format!("{}{}", task.entry.task_path, task.entry.task_name);
        if task.runs_as_system {
            warnings.push(Warning::new(
                "scheduled_task_runs_as_system",
                WarningSeverity::Warning,
                format!("The scheduled task {name} runs as SYSTEM, with full rights over this PC."),
                Some(name.clone()),
            ));
        }
        if !options.apply {
            changes_logged.push(format!("would restore scheduled task {name}"));
            continue;
        }
        match apply_scheduled_task(task, options) {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
}

fn restore_shortcuts(
    options: &RestoreOptions,
    review: &ReviewedPackage<'_>,
    changes: &mut Changes,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for shortcut in &review.shortcuts {
        let destination = &shortcut.placement.destination;
        let log = if shortcut.placement.autostart {
            &mut changes.autostart
        } else {
            &mut changes.shortcuts
        };
        let kind = if shortcut.placement.autostart {
            "startup shortcut"
        } else {
            "shortcut"
        };
        if !options.apply {
            log.push(format!(
                "would restore {kind} {} -> {}",
                destination.display(),
                shortcut.target.display()
            ));
            continue;
        }
        if destination.exists() && !options.force {
            errors.push(Warning::new(
                "shortcut_destination_exists",
                WarningSeverity::Warning,
                "Shortcut exists; use --force to overwrite.",
                Some(destination.display().to_string()),
            ));
            continue;
        }
        match apply_shortcut(shortcut) {
            Ok(message) => log.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if review
        .shortcuts
        .iter()
        .any(|shortcut| shortcut.placement.for_all_users)
    {
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
    review: &ReviewedPackage<'_>,
    changes_logged: &mut Vec<String>,
    warnings: &mut Vec<Warning>,
    errors: &mut Vec<Warning>,
) {
    for variable in &review.environment {
        if !options.apply {
            changes_logged.push(format!(
                "would restore {:?} environment variable {}",
                variable.scope, variable.name
            ));
            continue;
        }
        match apply_environment_variable(variable, options.force, &options.target_root, &review.map)
        {
            Ok(message) => changes_logged.push(message),
            Err(warning) => errors.push(warning),
        }
    }
    if !review.environment.is_empty() {
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
    // Before the key is created: a new empty key can take over a class by itself.
    if is_com_server_entry(entry) {
        refuse_taking_over_com_class(entry, map)?;
    }
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

/// Look at what the class is registered as on this PC, before writing over it. A per-user
/// registration wins over the machine's, so a new one in `HKCU` takes over a class that
/// only exists in `HKLM` just as surely. Any server of the class counts: registering a
/// `LocalServer32` or `TreatAs` for an in-process class redirects it as well.
fn refuse_taking_over_com_class(
    entry: &RegistryEntry,
    map: &RestoreMap,
) -> std::result::Result<(), Warning> {
    let new_value = map.rewrite(entry.value_data.as_deref().unwrap_or_default());
    let Some((class_key, _)) = entry.key_path.trim_end_matches('\\').rsplit_once('\\') else {
        return Ok(());
    };
    let mut hives = vec![&entry.hive];
    if entry.hive == RegistryHive::CurrentUser {
        hives.push(&RegistryHive::LocalMachine);
    }
    for hive in hives {
        for server in COM_SERVER_KEYS {
            let existing = registry_root(hive)
                .open_subkey(format!(r"{class_key}\{server}"))
                .and_then(|key| key.get_value::<String, _>(""));
            if let Ok(existing) = existing
                && let Some(refusal) =
                    com_server_refusal(&existing, &new_value, map.target_root(), system_root())
            {
                return Err(registry_warning(
                    "registry_com_class_exists",
                    entry,
                    refusal,
                ));
            }
        }
    }
    Ok(())
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
        if entries.is_empty() {
            return Ok(format!(
                "left {:?} environment variable {} as it is: none of it belongs to this app",
                variable.scope, variable.name
            ));
        }
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
        RegistryValueKind::String | RegistryValueKind::ExpandString => Ok(RegValue {
            bytes: Cow::Owned(encode_utf16_nul(&restored_value_text(entry, map))),
            vtype: registry_raw_type(kind),
        }),
        RegistryValueKind::None => Ok(RegValue {
            bytes: Cow::Owned(Vec::new()),
            vtype: REG_NONE,
        }),
        _ => {
            let bytes = restored_value_bytes(entry, map).map_err(|message| {
                registry_warning("registry_value_decode_failed", entry, message)
            })?;
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
    reviewed: &ReviewedService<'_>,
    target_root: &Path,
    force: bool,
) -> std::result::Result<Vec<String>, Warning> {
    let service = reviewed.entry;
    let name = service.service_name.as_str();
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
        &reviewed.binary,
        reviewed.arguments.as_deref(),
        &reviewed.account,
    ));
    run_checked(&mut command, error_code, name)?;
    let messages = vec![format!(
        "{done} service {name} (runs as {})",
        reviewed.account.label()
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

/// Register a task. Its definition is passed through a file in the staging folder, which
/// other processes cannot write to. A fixed name in `%TEMP%` could be planted as a link.
fn apply_scheduled_task(
    task: &ReviewedTask<'_>,
    options: &RestoreOptions,
) -> std::result::Result<String, Warning> {
    let name = format!("{}{}", task.entry.task_path, task.entry.task_name);
    let existing =
        live_task_xml(&task.entry.task_path, &task.entry.task_name).map_err(|error| {
            Warning::new(
                "scheduled_task_query_failed",
                WarningSeverity::Error,
                format!("Failed to query scheduled task: {error}"),
                Some(name.clone()),
            )
        })?;
    if let Some(existing) = existing.as_deref() {
        if !options.force {
            return Err(Warning::new(
                "scheduled_task_exists",
                WarningSeverity::Warning,
                "Scheduled task exists; use --force to overwrite.",
                Some(name),
            ));
        }
        if let Some(refusal) = existing_task_refusal(existing, &options.target_root) {
            return Err(Warning::new(
                "scheduled_task_exists_elsewhere",
                WarningSeverity::Error,
                refusal,
                Some(name),
            ));
        }
    }

    let xml_path = write_task_definition(&options.package_root, &task.xml).map_err(|error| {
        warning_io(
            "scheduled_task_xml_write_failed",
            &options.package_root,
            error,
        )
    })?;
    let script = register_task_script(
        &xml_path,
        &task.entry.task_path,
        &task.entry.task_name,
        options.force,
    );
    let result = run_powershell_script(&script).map(|()| format!("restored scheduled task {name}"));
    let _ = fs::remove_file(&xml_path);
    result
}

/// Write a task's definition into `folder` under a new name, failing rather than opening
/// anything already there.
fn write_task_definition(folder: &Path, xml: &str) -> io::Result<PathBuf> {
    let path = folder.join(format!("task-{}.xml", partial_file_name()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(xml.as_bytes())?;
    Ok(path)
}

fn register_task_script(xml_path: &Path, task_path: &str, task_name: &str, force: bool) -> String {
    let force_flag = if force { "$true" } else { "$false" };
    format!(
        "$xml = Get-Content -Raw -Encoding UTF8 -LiteralPath '{}'; Register-ScheduledTask -TaskName '{}' -TaskPath '{}' -Xml $xml -Force:{force_flag} | Out-Null",
        escape_powershell_single(&xml_path.display().to_string()),
        escape_powershell_single(task_name),
        escape_powershell_single(task_path),
    )
}

fn apply_shortcut(shortcut: &ReviewedShortcut) -> std::result::Result<String, Warning> {
    let placement = &shortcut.placement;
    let destination = &placement.destination;
    fs::create_dir_all(&placement.root)
        .map_err(|error| warning_io("shortcut_parent_create_failed", &placement.root, error))?;
    ensure_within_root(&placement.root, destination).map_err(|error| {
        Warning::new(
            "shortcut_destination_invalid",
            WarningSeverity::Error,
            error.to_string(),
            Some(destination.display().to_string()),
        )
    })?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| warning_io("shortcut_parent_create_failed", parent, error))?;
    }
    run_powershell_script(&shortcut_script(shortcut))
        .map(|()| format!("restored shortcut {}", destination.display()))
}

fn shortcut_script(shortcut: &ReviewedShortcut) -> String {
    let mut script = format!(
        "$shell = New-Object -ComObject WScript.Shell; $shortcut = $shell.CreateShortcut('{}'); $shortcut.TargetPath = '{}';",
        escape_powershell_single(&shortcut.placement.destination.display().to_string()),
        escape_powershell_single(&shortcut.target.display().to_string()),
    );
    if let Some(arguments) = shortcut.arguments.as_deref() {
        script.push_str(&format!(
            " $shortcut.Arguments = '{}';",
            escape_powershell_single(arguments)
        ));
    }
    if let Some(working_directory) = shortcut.working_directory.as_deref() {
        script.push_str(&format!(
            " $shortcut.WorkingDirectory = '{}';",
            escape_powershell_single(working_directory)
        ));
    }
    if let Some(icon) = shortcut.icon.as_deref() {
        script.push_str(&format!(
            " $shortcut.IconLocation = '{}';",
            escape_powershell_single(icon)
        ));
    }
    script.push_str(" $shortcut.Save();");
    script
}

/// Put a file from the package at its destination. It is written beside the destination
/// under a new name, then renamed over it. A rename replaces the name at the destination
/// and cannot be carried through it, unlike opening it for writing, which follows a link
/// and also writes through a hard link. A failure part-way through a file also leaves the
/// old one in place.
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
        make_replaceable(destination)?;
        fs::rename(&partial, destination)
            .map_err(|error| WinAppMigratorError::io(destination, error))
    });
    if copied.is_err() {
        let _ = fs::remove_file(&partial);
    }
    copied
}

/// A rename does not replace a read-only file, and an earlier install of the same app can
/// have left one, since a copy keeps the read-only attribute. Whether to replace it has
/// been decided by this point. On Windows this clears that one attribute, and nothing else.
#[allow(clippy::permissions_set_readonly_false)]
fn make_replaceable(destination: &Path) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(destination) else {
        return Ok(());
    };
    let mut permissions = metadata.permissions();
    if permissions.readonly() {
        permissions.set_readonly(false);
        fs::set_permissions(destination, permissions)
            .map_err(|error| WinAppMigratorError::io(destination, error))?;
    }
    Ok(())
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
    markdown.push_str(REFUSED_LOG_HEADING);
    for refused in &result.refused {
        markdown.push_str(&format!("- {}\n", refused.message));
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

/// Worked out from what the restore will do, not from what the manifest says about it.
fn restore_requires_admin(review: &ReviewedPackage<'_>) -> bool {
    review
        .registry
        .iter()
        .chain(review.license_registry.iter())
        .any(|entry| entry.hive == RegistryHive::LocalMachine)
        || !review.services.is_empty()
        || !review.drivers.is_empty()
        || !review.tasks.is_empty()
        || review
            .shortcuts
            .iter()
            .any(|shortcut| shortcut.placement.for_all_users)
        || review
            .environment
            .iter()
            .any(|variable| variable.scope == EnvironmentScope::Machine)
}

/// True when any planned user-data file will be restored to a machine-wide
/// protected location (for example ProgramData) and therefore needs elevation.
fn user_data_restore_requires_admin(review: &ReviewedPackage<'_>) -> bool {
    review.files.iter().any(|file| {
        review
            .map
            .destination_of(&file.original_path)
            .is_some_and(destination_requires_admin)
    })
}

fn encode_utf16_nul(value: &str) -> Vec<u8> {
    value
        .encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect()
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
    use crate::models::{FileClassification, FileEntry};
    use crate::restore_policy::driver_destination;

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

    /// Windows' own shortcut class, which is on every PC. Only reads the registry, and the
    /// refusal comes before anything could be written.
    const SHELL_LINK_CLASS: &str = r"SOFTWARE\Classes\CLSID\{00021401-0000-0000-C000-000000000046}";

    fn com_entry(hive: RegistryHive, key: String, kind: RegistryValueKind) -> RegistryEntry {
        RegistryEntry {
            hive,
            key_path: key,
            value_name: None,
            value_data: Some(r"C:\App\evil.dll".to_owned()),
            value_kind: Some(kind),
            value_bytes_hex: None,
            confidence: crate::models::MatchConfidence::High,
            requires_admin: false,
        }
    }

    #[test]
    fn a_class_already_on_this_pc_is_not_taken_over_by_any_route() -> Result<()> {
        let map = app_map()?;
        for (hive, server, kind) in [
            (
                RegistryHive::LocalMachine,
                "InprocServer32",
                RegistryValueKind::String,
            ),
            // A new per-user key wins over the machine's, even with nothing in it.
            (
                RegistryHive::CurrentUser,
                "InprocServer32",
                RegistryValueKind::None,
            ),
            // Another kind of server redirects the class just the same.
            (
                RegistryHive::CurrentUser,
                "LocalServer32",
                RegistryValueKind::String,
            ),
            (
                RegistryHive::CurrentUser,
                "TreatAs",
                RegistryValueKind::String,
            ),
        ] {
            let entry = com_entry(hive, format!(r"{SHELL_LINK_CLASS}\{server}"), kind);
            assert!(is_com_server_entry(&entry), "{server}");
            let refused = refuse_taking_over_com_class(&entry, &map);
            assert_eq!(
                refused.err().map(|warning| warning.code).as_deref(),
                Some("registry_com_class_exists"),
                "{server}"
            );
        }

        let own_class = com_entry(
            RegistryHive::LocalMachine,
            r"SOFTWARE\Classes\CLSID\{5A7E6E0C-0000-4000-8000-000000000000}\InprocServer32"
                .to_owned(),
            RegistryValueKind::String,
        );
        assert!(refuse_taking_over_com_class(&own_class, &map).is_ok());
        Ok(())
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

    fn reviewed_driver<'a>(
        driver: &'a DriverEntry,
        options: &RestoreOptions,
    ) -> ReviewedDriver<'a> {
        ReviewedDriver {
            entry: driver,
            package_relative_path: Path::new(r"drivers\flt.sys"),
            destination: options.target_root.join(r"drivers\flt.sys"),
        }
    }

    #[test]
    fn a_driver_image_with_the_wrong_hash_is_not_placed() -> Result<()> {
        let work = temp_folder()?;
        let options = driver_options(work.path(), true);
        write_file(&options.package_root.join(r"drivers\flt.sys"), b"tampered")?;
        fs::create_dir_all(&options.target_root)
            .map_err(|error| WinAppMigratorError::io(&options.target_root, error))?;

        for hash in [None, Some("00".repeat(32))] {
            let driver = unregistered_driver(hash);
            let reviewed = reviewed_driver(&driver, &options);
            let refused = place_and_register_driver(&reviewed, &options);
            assert_eq!(
                refused.err().map(|warning| warning.code).as_deref(),
                Some("driver_hash_failed")
            );
            assert!(!reviewed.destination.exists());
        }
        Ok(())
    }

    #[test]
    fn an_existing_driver_image_is_not_overwritten_without_force() -> Result<()> {
        let work = temp_folder()?;
        let options = driver_options(work.path(), false);
        let source = options.package_root.join(r"drivers\flt.sys");
        write_file(&source, b"new")?;
        let driver = unregistered_driver(Some(sha256_file(&source)?));
        let reviewed = reviewed_driver(&driver, &options);
        write_file(&reviewed.destination, b"existing")?;

        let refused = place_and_register_driver(&reviewed, &options);

        assert_eq!(
            refused.err().map(|warning| warning.code).as_deref(),
            Some("driver_image_exists")
        );
        assert_eq!(
            fs::read(&reviewed.destination)
                .map_err(|error| WinAppMigratorError::io(&reviewed.destination, error))?,
            b"existing"
        );
        Ok(())
    }

    #[test]
    fn a_task_definition_goes_in_the_staging_folder_under_a_new_name() -> Result<()> {
        let work = temp_folder()?;
        let path = write_task_definition(work.path(), "<Task>é</Task>")
            .map_err(|error| WinAppMigratorError::io(work.path(), error))?;
        assert_eq!(path.parent(), Some(work.path()));
        assert_eq!(
            fs::read_to_string(&path).map_err(|error| WinAppMigratorError::io(&path, error))?,
            "<Task>é</Task>"
        );
        let second = write_task_definition(work.path(), "x")
            .map_err(|error| WinAppMigratorError::io(work.path(), error))?;
        assert_ne!(path, second);
        Ok(())
    }

    #[test]
    fn the_task_script_names_its_file_and_escapes_what_it_is_given() {
        let script = register_task_script(
            Path::new(r"D:\Stage\task-1.xml"),
            r"\Acme's\",
            "Sync's",
            true,
        );
        assert!(script.contains("-Encoding UTF8"));
        assert!(script.contains(r"-LiteralPath 'D:\Stage\task-1.xml'"));
        assert!(script.contains("-TaskName 'Sync''s'"));
        assert!(script.contains(r"-TaskPath '\Acme''s\'"));
        assert!(script.contains("-Force:$true"));
    }

    fn file_entry(relative: &str, hash: Option<String>, size: u64) -> FileEntry {
        let original = Path::new(r"C:\Program Files\Acme").join(relative);
        FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(Path::new("files").join(relative)),
            suggested_destination_path: None,
            size_bytes: Some(size),
            hash_sha256: hash,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
        }
    }

    fn bare_manifest() -> PackageManifest {
        PackageManifest {
            schema_version: crate::models::PACKAGE_SCHEMA_VERSION.to_owned(),
            source_computer_name: None,
            source_windows_version: None,
            scan_timestamp_utc: String::new(),
            main_executable_path: PathBuf::from(r"C:\Program Files\Acme\acme.exe"),
            application_metadata: None,
            file_inventory: Vec::new(),
            registry_keys_exported: Vec::new(),
            com_registrations_detected: Vec::new(),
            file_associations_detected: Vec::new(),
            services_detected: Vec::new(),
            drivers_detected: Vec::new(),
            scheduled_tasks_detected: Vec::new(),
            shortcuts_detected: Vec::new(),
            environment_variables_detected: Vec::new(),
            runtime_prerequisites: Vec::new(),
            license_artifacts_detected: Vec::new(),
            license_registry_entries: Vec::new(),
            warnings: Vec::new(),
            items_intentionally_excluded: Vec::new(),
            included_user_data: false,
            included_program_data: false,
            included_license_data: false,
            ui_theme: None,
        }
    }

    /// A package of real files in a temp folder, with the manifest pointing at them.
    fn package_of(
        root: &Path,
        files: &[(&str, &[u8])],
        edit: impl FnOnce(&mut PackageManifest),
    ) -> Result<PathBuf> {
        let package = root.join("package");
        let mut manifest = bare_manifest();
        for (relative, bytes) in files {
            let staged = package.join("files").join(relative);
            write_file(&staged, bytes)?;
            manifest.file_inventory.push(file_entry(
                relative,
                Some(sha256_file(&staged)?),
                bytes.len() as u64,
            ));
        }
        edit(&mut manifest);
        write_file(
            &package.join("manifest.json"),
            &serde_json::to_vec(&manifest)?,
        )?;
        Ok(package)
    }

    fn restore_options(root: &Path, package: PathBuf, apply: bool) -> RestoreOptions {
        RestoreOptions {
            package_root: package,
            target_root: root.join("target"),
            dry_run: !apply,
            apply,
            force: false,
            restore_user_data: false,
            restore_license: true,
        }
    }

    /// Temp folders only, and with files alone, so this needs no elevation and touches
    /// nothing else.
    #[test]
    fn installing_a_package_of_files_writes_and_verifies_them() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(
            work.path(),
            &[("acme.exe", b"program"), (r"bin\data.dat", b"data")],
            |_| {},
        )?;
        let options = restore_options(work.path(), package, true);

        let result = restore_package(&options)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.refused.is_empty());
        let report = result.verification.ok_or_else(|| {
            WinAppMigratorError::InvalidInput("an install is always verified".to_owned())
        })?;
        assert!(report.package_valid);
        assert_eq!(report.restore_valid, Some(true));
        let installed = options.target_root.join(r"bin\data.dat");
        assert_eq!(
            fs::read(&installed).map_err(|error| WinAppMigratorError::io(&installed, error))?,
            b"data"
        );
        Ok(())
    }

    #[test]
    fn a_file_with_no_hash_stops_the_install_before_anything_is_written() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |manifest| {
            manifest.file_inventory[0].hash_sha256 = None;
        })?;
        let options = restore_options(work.path(), package, true);

        let refused = restore_package(&options);

        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("hash check")),
            "{refused:?}"
        );
        assert!(!options.target_root.exists());
        Ok(())
    }

    #[test]
    fn a_changed_file_stops_the_install_before_anything_is_written() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |_| {})?;
        write_file(&package.join(r"files\acme.exe"), b"PROGRAM")?;
        let options = restore_options(work.path(), package, true);

        assert!(restore_package(&options).is_err());
        assert!(!options.target_root.exists());
        Ok(())
    }

    #[test]
    fn without_force_an_existing_file_stops_the_install_before_anything_changes() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(
            work.path(),
            &[("acme.exe", b"program"), ("other.dat", b"data")],
            |_| {},
        )?;
        let options = restore_options(work.path(), package, true);
        let planted = options.target_root.join("acme.exe");
        write_file(&planted, b"planted")?;

        let refused = restore_package(&options);

        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("already exists")),
            "{refused:?}"
        );
        assert_eq!(
            fs::read(&planted).map_err(|error| WinAppMigratorError::io(&planted, error))?,
            b"planted"
        );
        assert!(!options.target_root.join("other.dat").exists());
        Ok(())
    }

    #[test]
    fn with_force_an_existing_file_is_replaced() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |_| {})?;
        let mut options = restore_options(work.path(), package, true);
        options.force = true;
        let planted = options.target_root.join("acme.exe");
        write_file(&planted, b"planted")?;

        let result = restore_package(&options)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(
            fs::read(&planted).map_err(|error| WinAppMigratorError::io(&planted, error))?,
            b"program"
        );
        Ok(())
    }

    #[test]
    fn a_system_folder_is_refused_as_an_install_location_before_anything_happens() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |_| {})?;
        let mut options = restore_options(work.path(), package, false);
        options.target_root = system_root().to_path_buf();
        let refused = restore_package(&options);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("cannot be used")),
            "{refused:?}"
        );
        Ok(())
    }

    #[test]
    fn a_junction_at_the_install_location_stops_the_install() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |_| {})?;
        let options = restore_options(work.path(), package, true);
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&options.target_root)
            .arg(&elsewhere)
            .output()
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        assert!(made.status.success());

        assert!(restore_package(&options).is_err());
        assert!(!elsewhere.join("acme.exe").exists());
        Ok(())
    }

    /// A preview touches nothing, so this can include everything an install would refuse
    /// or do. Registry, services, and tasks are only described.
    #[test]
    fn a_preview_lists_what_reaches_beyond_files_first_and_says_what_it_refuses() -> Result<()> {
        let work = temp_folder()?;
        let many_files: Vec<String> = (0..50).map(|index| format!("file{index}.dat")).collect();
        let mut files: Vec<(&str, &[u8])> = vec![("acme.exe", b"program")];
        files.extend(many_files.iter().map(|name| (name.as_str(), &b"x"[..])));
        let task_xml = r#"<Task><Actions><Exec><Command>C:\Program Files\Acme\acme.exe</Command></Exec></Actions></Task>"#;
        let package = package_of(work.path(), &files, |manifest| {
            let mut service = sample_service();
            service.binary_path = PathBuf::from(r"C:\Program Files\Acme\acme.exe");
            manifest.services_detected = vec![service];
            let run = |name: &str, data: &str| RegistryEntry {
                hive: RegistryHive::LocalMachine,
                key_path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run".to_owned(),
                value_name: Some(name.to_owned()),
                value_data: Some(data.to_owned()),
                value_kind: Some(RegistryValueKind::String),
                value_bytes_hex: None,
                confidence: crate::models::MatchConfidence::High,
                requires_admin: true,
            };
            let mut hijack = run("Debugger", r"C:\Program Files\Acme\acme.exe");
            hijack.key_path =
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\sethc.exe"
                    .to_owned();
            manifest.registry_keys_exported = vec![
                run("Acme", r#""C:\Program Files\Acme\acme.exe" --tray"#),
                run("Evil", r"C:\Windows\System32\cmd.exe /c calc"),
                hijack,
            ];
            manifest.scheduled_tasks_detected = vec![crate::models::ScheduledTaskEntry {
                task_name: "Sync".to_owned(),
                task_path: r"\Acme\".to_owned(),
                action_command: PathBuf::from(r"C:\Program Files\Acme\acme.exe"),
                arguments: None,
                working_directory: None,
                trigger_summary: None,
                run_level: None,
                account: None,
                requires_admin: false,
                restore_instructions: String::new(),
                definition_xml: Some(task_xml.to_owned()),
            }];
        })?;
        let options = restore_options(work.path(), package, false);

        let result = restore_package(&options)?;

        let first_file = result
            .changes_logged
            .iter()
            .position(|change| change.starts_with("would copy"));
        assert_eq!(result.changes_logged.len(), 3 + files.len());
        assert_eq!(first_file, Some(3));
        assert!(result.changes_logged[0].contains("service AcmeSync"));
        assert!(result.changes_logged[1].contains("scheduled task"));
        assert!(result.changes_logged[2].contains(r"CurrentVersion\Run Acme"));
        assert_eq!(result.refused.len(), 2);
        assert!(
            result
                .refused
                .iter()
                .any(|item| item.message.contains("Evil"))
        );
        assert!(
            result
                .refused
                .iter()
                .any(|item| item.message.contains("Image File Execution Options"))
        );
        assert!(!options.target_root.exists(), "a preview creates nothing");
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
    fn copying_replaces_a_read_only_file_from_an_earlier_install() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"new")?;
        let root = work.path().join("target");
        let destination = root.join("app.exe");
        write_file(&destination, b"old")?;
        let mut permissions = fs::metadata(&destination)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?
            .permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&destination, permissions)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;

        copy_restore_file(&source, &destination, &root)?;

        assert_eq!(
            fs::read(&destination).map_err(|error| WinAppMigratorError::io(&destination, error))?,
            b"new"
        );
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
        // Symbolic links need Developer Mode or elevation, so this cannot always run. CI
        // is elevated, so there this fails rather than skipping.
        if let Err(error) = std::os::windows::fs::symlink_file(&victim, &destination) {
            assert!(
                std::env::var_os("CI").is_none(),
                "CI should be able to create a symbolic link: {error}"
            );
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
