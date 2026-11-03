use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use filetime::{FileTime, set_file_times};
use tracing::{info, warn};
use winreg::RegValue;
use winreg::enums::{REG_DWORD, REG_NONE, REG_SZ};

use crate::archive::EmbeddedPackage;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    ensure_target_root, ensure_within_root, first_link_to, link_refusal, package_destination_path,
    pin_path_below_root, safe_relative_path, split_user_data_relative, write_root_for,
};
use crate::hashing::sha256_file;
use crate::integration_scan::{first_path_in, parse_firewall_rule};
use crate::journal::{
    Change, ItemKind, Journal, QUIET_FLAG, STAGING_PREFIX, StoredValue, UNINSTALL_FLAG,
    find_backup, install_on_record_at, lock_state,
};
use crate::models::{
    DriverEntry, DriverKind, EnvironmentScope, EnvironmentVariableEntry, FileClassification,
    FileEntry, FirewallRuleEntry, LicenseArtifact, PackageManifest, RegistryEntry, RegistryHive,
    RegistryValueKind, RestoreOptions, RestoreResult, RuntimePrerequisite, ServiceEntry,
    VerificationOptions, Warning, WarningSeverity,
};
use crate::package::{PreparedPackage, read_manifest};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{
    POWERSHELL_IMPORT_MANAGEMENT, POWERSHELL_IMPORT_TASKS, POWERSHELL_IMPORT_UTILITY,
    POWERSHELL_UTF8_OUTPUT, escape_powershell_single, hidden_command, path_is_strictly_inside,
    path_is_within, powershell_command, strip_prefix_ignore_case, system_root,
};
use crate::prerequisites::{
    PrerequisiteInstallOutcome, PrerequisiteSpec, install_bundle, is_present, spec_for_bundle,
};
use crate::privilege::{destination_requires_admin, is_elevated, target_requires_admin};
use crate::progress::{InstallMonitor, InstallPhase, InstallStep};
use crate::registry_scan::registry_value_text;
use crate::registry_target::{
    app_path_entries, environment_registry_target, is_path_variable, merge_path_entries,
    path_contains_entry, read_service_config, registry_hive_name, registry_raw_type, registry_root,
    registry_value_name, service_image_path, stored_firewall_rules,
};
use crate::restore_policy::{
    COM_SERVER_KEYS, EVENT_LOG_PREFIX, Registration, SERVICES_PREFIX, ServiceAccount,
    com_class_key, com_server_refusal, existing_service_refusal, existing_task_refusal,
    firewall_fingerprint, firewall_identity, is_autostart_entry, is_uninstall_entry_key,
    netsh_add_rule_arguments, program_placement_refusal, registration_of, registration_unit_of,
    restored_value_bytes, restored_value_name, restored_value_text, service_image_path_value,
    stored_firewall_rule_matches,
};
use crate::restore_review::{
    ReviewedDriver, ReviewedPackage, ReviewedService, ReviewedShortcut, ReviewedTask,
    STATE_ROOT_REFUSAL, install_location_problem, overlaps_state_root, refuse_what_cannot_run_here,
    store_file_destination,
};
use crate::rollback::{
    interrupted_installs, put_back_backup, recover_interrupted_installs, roll_back,
};
use crate::service_scan::DELAYED_AUTOMATIC_START_LABEL;
use crate::task_scan::live_task_xml;
use crate::verify::{PackageFiles, verify_prepared_package, without_what_was_refused};

pub(crate) const SC_EXE: &str = "sc.exe";
/// Buffer for copying a file out of the package. Large files are the slow ones.
const COPY_BUFFER_BYTES: usize = 1024 * 1024;
const UNINSTALL_KEY_FOR_NEW_ENTRIES: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
/// Well within what a key's name may be.
const MAX_UNINSTALL_KEY_NAME_CHARS: usize = 200;
const UNINSTALL_STRING_VALUE: &str = "UninstallString";
const QUIET_UNINSTALL_STRING_VALUE: &str = "QuietUninstallString";
const NO_MODIFY_VALUE: &str = "NoModify";
const NO_REPAIR_VALUE: &str = "NoRepair";
/// What an entry always gets from the install, in place of what the source's said.
const UNINSTALL_COMMAND_VALUES: [&str; 4] = [
    UNINSTALL_STRING_VALUE,
    QUIET_UNINSTALL_STRING_VALUE,
    NO_MODIFY_VALUE,
    NO_REPAIR_VALUE,
];
const DISPLAY_NAME_VALUE: &str = "DisplayName";
const DISPLAY_VERSION_VALUE: &str = "DisplayVersion";
const PUBLISHER_VALUE: &str = "Publisher";
const INSTALL_LOCATION_VALUE: &str = "InstallLocation";
/// Failures found at the moment of installing that are the rules refusing something, and so
/// are left out like any other refusal rather than failing the install. They depend on what
/// is on this PC at that moment, so the review before the install could not see them.
const REFUSALS_FOUND_WHEN_INSTALLING: [&str; 7] = [
    FILE_EXISTS_ELSEWHERE_CODE,
    COM_CLASS_EXISTS_CODE,
    REGISTRATION_EXISTS_CODE,
    FIREWALL_RULE_EXISTS_CODE,
    "scheduled_task_exists_elsewhere",
    "service_exists_elsewhere",
    "driver_exists_elsewhere",
];
const FILE_EXISTS_ELSEWHERE_CODE: &str = "file_exists_elsewhere";
const COM_CLASS_EXISTS_CODE: &str = "registry_com_class_exists";
const REGISTRATION_EXISTS_CODE: &str = "registration_exists_elsewhere";
const CLASS_IS_SOMEONES: &str = "this COM class is already on this PC, as something else's";
const SERVICES_LEFT_ALONE_CODES: [&str; 2] =
    ["service_exists_elsewhere", "driver_exists_elsewhere"];
/// How far down to look for what says a unit on this PC is the app's: a type library's file is
/// `{id}\<version>\<language>\<build>`.
const DEEPEST_LOOK_IN_A_UNIT: usize = 4;
const APP_ID_KEY_PREFIX: &str = r"software\classes\appid\";
const APP_PATHS_KEY_PREFIX: &str = r"software\microsoft\windows\currentversion\app paths\";
const CLASS_REFERENCE_KEY: &str = "CLSID";
/// As a place to compare with, this has nothing in it.
const NOWHERE: &str = "";
const FIREWALL_RULE_EXISTS_CODE: &str = "firewall_rule_exists";
const NETSH_EXE: &str = "netsh.exe";
const SHELL_KEY_SEGMENT: &str = "shell";
/// What is under `Classes` that is not a type or a protocol, and goes by rules of its own.
const NOT_TYPES: &[&str] = &[
    "appid",
    "applications",
    "clsid",
    "interface",
    "typelib",
    "wow6432node",
];
const OPEN_VERB: &str = "open";
const EVENT_LOG_KEY: &str = r"SYSTEM\CurrentControlSet\Services\EventLog";
const EXISTING_IS_SOMETHING_ELSE: &str =
    "this PC already has this, set to something else, which an install does not take over";
const CANCELLED_CODE: &str = "install_cancelled";
const JOURNAL_WRITE_FAILED_CODE: &str = "journal_write_failed";
/// What PowerShell says can quote its script, on any language of Windows, and a script can hold
/// what must not be logged, such as a shortcut's arguments.
const POWERSHELL_FAILED_CODE: &str = "powershell_command_failed";

/// What a restore did or would do, kept apart so what reaches beyond files can be listed
/// first. A preview of a large app is otherwise thousands of files before anything else.
#[derive(Default)]
struct Changes {
    services: Vec<String>,
    drivers: Vec<String>,
    tasks: Vec<String>,
    autostart: Vec<String>,
    /// What goes into how Windows finds and starts things.
    registrations: Vec<String>,
    firewall: Vec<String>,
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
            self.registrations,
            self.firewall,
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
    restore_package_monitored(options, &InstallMonitor::default(), None)
}

/// Restore a package, reporting progress to `monitor` and stopping if it is cancelled.
///
/// A preview changes nothing at all. An install keeps a journal, and if any step the package
/// asked for fails, or it is cancelled, or the result does not pass its checks, it puts back
/// everything it did and says so in the result. An `Err` means it stopped before changing
/// anything. `installer` is where a copy of the stub to uninstall with is taken from, if
/// the app is to get an uninstaller and an entry in Settings.
pub fn restore_package_monitored(
    options: &RestoreOptions,
    monitor: &InstallMonitor,
    installer: Option<&EmbeddedPackage>,
) -> Result<RestoreResult> {
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
    if overlaps_state_root(&options.target_root, &options.state_root) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "that install location cannot be used: {STATE_ROOT_REFUSAL}: \"{}\"",
            options.target_root.display()
        )));
    }

    let prepared = PreparedPackage::new(&options.package_root)?;
    let mut effective_options = options.clone();
    effective_options.package_root = prepared.package_root().to_path_buf();
    // Read once. Everything below works from this copy, so nothing can change between
    // the hash check and the install, even if the file on disk does.
    let manifest = read_manifest(&effective_options.package_root)?;
    // What cannot run here stops here, before the record of an install is begun, let alone anything
    // it would change. What may not run is said along with everything else.
    let fit = refuse_what_cannot_run_here(&manifest)?;
    let mut recovered_warnings: Vec<Warning> = fit
        .warnings
        .iter()
        .map(|warning| {
            Warning::new(
                "platform_may_not_fit",
                WarningSeverity::Warning,
                warning.clone(),
                None,
            )
        })
        .collect();
    // Unpacked from an installer, each file was hashed as it came out, and is in a folder only an
    // administrator can write to. Reading it all again a moment later could not tell anything more.
    let digests =
        installer.and_then(|package| package.digests_of_unpacked(&effective_options.package_root));
    let package_files = digests
        .as_deref()
        .map_or(PackageFiles::OnDisk, PackageFiles::Seen);
    let mut busy_lock = None;
    if options.apply {
        monitor.report(InstallStep::new(InstallPhase::CheckingPackage, 0, 0));
        refuse_damaged_package(&effective_options, &manifest, package_files)?;
        // There is nowhere safe to keep a record without this, so nothing goes ahead, and held
        // until the end, so that no other install can change what this one is changing.
        busy_lock = Some(lock_state(&options.state_root).map_err(nowhere_to_keep_a_record)?);
        // Before anything looks at what is on this PC, since what an install that never
        // finished left there is not to be taken for anything.
        let recovery = recover_interrupted_installs(&options.state_root, monitor)?;
        for folder in &recovery.not_trusted {
            recovered_warnings.push(Warning::new(
                "earlier_install_not_trusted",
                WarningSeverity::Warning,
                "A record of an install that never finished was left alone, because others can \
                 change it. Whatever that install changed is still there.",
                Some(folder.display().to_string()),
            ));
        }
        for earlier in recovery.recovered {
            recovered_warnings.push(Warning::new(
                "earlier_install_undone",
                WarningSeverity::Warning,
                format!(
                    "An earlier install of {} never finished, so it was undone first. {} changes \
                     were put back.",
                    earlier.app_name,
                    earlier.report.undone.len()
                ),
                None,
            ));
            recovered_warnings.extend(
                earlier
                    .report
                    .kept
                    .iter()
                    .chain(&earlier.report.left_behind)
                    .map(|left| {
                        Warning::new(
                            "earlier_install_left",
                            WarningSeverity::Warning,
                            left.clone(),
                            None,
                        )
                    }),
            );
        }
    } else {
        for app_name in interrupted_installs(&options.state_root) {
            recovered_warnings.push(Warning::new(
                "earlier_install_never_finished",
                WarningSeverity::Warning,
                format!(
                    "An earlier install of {app_name} never finished. Installing undoes it first."
                ),
                None,
            ));
        }
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
    refuse_roots_under_links(
        &roots_outside_the_install_location(&review, &options.target_root),
        &options.target_root,
    )?;

    let app_name = app_display_name(&manifest);
    let mut target_hold = None;
    let journal = if options.apply {
        let mut journal = start_journal(&effective_options, &app_name, installer)?;
        match prepare_target(&mut journal, &options.target_root, &options.state_root) {
            Ok(hold) => target_hold = Some(hold),
            Err(error) => {
                let report = roll_back(journal, monitor);
                for line in crate::rollback::report_lines(&report) {
                    info!(rollback = %line);
                }
                if !report.failed.is_empty() {
                    return Err(WinAppMigratorError::InvalidInput(format!(
                        "{error}. What had been done could not all be put back: {}",
                        report.failed.join(" ")
                    )));
                }
                return Err(error);
            }
        }
        info!(
            id = journal.id(),
            app = %app_name,
            target = %options.target_root.display(),
            "install started"
        );
        Some(journal)
    } else {
        None
    };

    let mut warnings = recovered_warnings;
    warnings.extend(manifest.warnings.iter().cloned());
    warnings.extend(review.warnings.iter().cloned());
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

    let earlier_install_here = install_on_record_at(
        &effective_options.state_root,
        &effective_options.target_root,
        journal.as_ref().map(Journal::folder),
    );
    let mut run = Run {
        options: &effective_options,
        review: &review,
        monitor,
        journal,
        changes: Changes::default(),
        warnings,
        errors: Vec::new(),
        refused: review.refused.clone(),
        com_classes_left_alone: Vec::new(),
        registrations_left_alone: Vec::new(),
        earlier_install_here,
        data_folders_already_here: Vec::new(),
        cancelled: false,
        app_name: &app_name,
        manifest: &manifest,
        package_files,
    };
    run.apply_steps();
    let verification = run.verify_result();

    let mut install_id = None;
    let mut rollback = None;
    let Run {
        journal,
        changes,
        mut warnings,
        mut errors,
        refused,
        ..
    } = run;
    let mut changes_logged = changes.in_order();
    if let Some(report) = verification.as_ref() {
        changes_logged.push(format!(
            "post-restore verification package_valid={} restore_valid={}",
            report.package_valid,
            report
                .restore_valid
                .map(|value| value.to_string())
                .unwrap_or_else(|| "not_run".to_owned())
        ));
        warnings.extend(report.warnings.iter().cloned());
    }
    if let Some(mut journal) = journal {
        let committed = if errors.is_empty() {
            journal.commit()
        } else {
            Ok(())
        };
        // The install location has to be let go of before it can be taken back.
        drop(target_hold);
        if let Err(error) = committed {
            errors.push(Warning::new(
                "journal_commit_failed",
                WarningSeverity::Error,
                format!("The record of this install could not be finished: {error}"),
                None,
            ));
        }
        if errors.is_empty() {
            info!(id = journal.id(), "install finished");
            install_id = Some(journal.id().to_owned());
        } else {
            warn!(
                id = journal.id(),
                "install failed, putting back what it changed"
            );
            rollback = Some(roll_back(journal, monitor));
        }
    }

    let result = RestoreResult {
        applied: options.apply,
        changes_logged,
        warnings,
        errors,
        refused,
        verification,
        install_id,
        rollback,
    };
    log_result(&result);
    drop(busy_lock);
    Ok(result)
}

fn nowhere_to_keep_a_record(error: WinAppMigratorError) -> WinAppMigratorError {
    match error {
        // Someone else's install, not a problem with where records are kept.
        WinAppMigratorError::InvalidInput(message) if message.contains("another install") => {
            WinAppMigratorError::InvalidInput(message)
        }
        error => WinAppMigratorError::InvalidInput(format!(
            "there is nowhere safe to keep the record that would let this install be undone, so \
             nothing was installed: {error}"
        )),
    }
}

/// Start the record of this install, with what it needs to be undone later. A failure here
/// means nothing has changed, and it should not go ahead, since it could not be undone.
fn start_journal(
    options: &RestoreOptions,
    app_name: &str,
    installer: Option<&EmbeddedPackage>,
) -> Result<Journal> {
    // Only ever a folder an installer made, since undoing an install removes this.
    let staging = options
        .package_root
        .file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with(STAGING_PREFIX))
        .then_some(options.package_root.as_path());
    let mut journal = Journal::begin(&options.state_root, app_name, &options.target_root, staging)
        .map_err(nowhere_to_keep_a_record)?;
    if let Some(installer) = installer
        && let Err(error) = journal.write_uninstaller(installer)
    {
        let folder = journal.folder().to_path_buf();
        journal.close();
        let _ = fs::remove_dir_all(folder);
        return Err(nowhere_to_keep_a_record(error));
    }
    Ok(journal)
}

/// Record what it takes to make the install location, make it, and hold it. Then it is asked
/// again whether it can be used: what really is there could not be told until it was, nor can
/// anything that has changed since it was first asked, and from here on it cannot change.
fn prepare_target(journal: &mut Journal, target_root: &Path, state_root: &Path) -> Result<File> {
    journal.note_new_folders_to(target_root, ItemKind::Program)?;
    let hold = ensure_target_root(target_root)?;
    let problem = install_location_problem(target_root).or_else(|| {
        overlaps_state_root(target_root, state_root).then(|| STATE_ROOT_REFUSAL.to_owned())
    });
    if let Some(problem) = problem {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "that install location cannot be used: {problem}: \"{}\"",
            target_root.display()
        )));
    }
    Ok(hold)
}

fn app_display_name(manifest: &PackageManifest) -> String {
    manifest
        .application_metadata
        .as_ref()
        .map(|app| app.display_name.clone())
        .filter(|name| !name.is_empty())
        .or_else(|| {
            manifest
                .main_executable_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "Application".to_owned())
}

/// Where a package suggests installing, which is where its app was. Whether that can be used
/// here is for [`install_location_problem`] to say.
pub fn default_install_location(manifest: &PackageManifest) -> Option<PathBuf> {
    manifest
        .application_metadata
        .as_ref()
        .and_then(|app| app.install_location.clone())
        .or_else(|| {
            manifest
                .main_executable_path
                .parent()
                .map(Path::to_path_buf)
        })
}

/// Write what happened to the log. Paths and key names only: what a registry value, license,
/// or environment variable holds never goes in. A refusal's reason or a warning's message can
/// quote a value, so only what it is about is written. An error's message is why something
/// failed, built here from a path and what the system said.
fn log_result(result: &RestoreResult) {
    for change in &result.changes_logged {
        info!(change = %change);
    }
    for warning in &result.warnings {
        warn!(code = %warning.code, source = warning.source.as_deref().unwrap_or_default());
    }
    for refused in &result.refused {
        warn!(code = %refused.code, source = refused.source.as_deref().unwrap_or_default(), "refused");
    }
    for error in &result.errors {
        if error.code == POWERSHELL_FAILED_CODE {
            warn!(code = %error.code, source = error.source.as_deref().unwrap_or_default());
        } else {
            warn!(code = %error.code, source = error.source.as_deref().unwrap_or_default(), message = %error.message);
        }
    }
    if let Some(report) = result.rollback.as_ref() {
        for line in crate::rollback::report_lines(report) {
            info!(rollback = %line);
        }
    }
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

/// The folders outside the install location that what must be restored would be checked against.
/// A Desktop or Start Menu shortcut is only a way in, so one that cannot be made is left out then.
fn roots_outside_the_install_location(
    review: &ReviewedPackage<'_>,
    target_root: &Path,
) -> Vec<PathBuf> {
    let files = review
        .files
        .iter()
        .filter_map(|file| review.map.destination_of(&file.original_path));
    let licenses = review
        .license
        .iter()
        .map(|license| license.destination.as_path());
    let mut roots: Vec<PathBuf> = files
        .chain(licenses)
        .filter_map(|destination| write_root_for(destination, target_root))
        .collect();
    roots.extend(
        review
            .shortcuts
            .iter()
            .filter(|shortcut| shortcut.placement.autostart)
            .map(|shortcut| shortcut.placement.root.clone()),
    );
    roots.sort();
    roots.dedup();
    roots
}

/// Writing under a link is refused when the first file gets there, which would be after runtimes
/// and other files. This can be told before anything has happened, and the way out is not the
/// same as for the install location, which can be chosen.
fn refuse_roots_under_links(roots: &[PathBuf], target_root: &Path) -> Result<()> {
    for root in roots
        .iter()
        .filter(|root| !path_is_within(root, target_root))
    {
        if let Some(link) = first_link_to(root) {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "this app has things to restore to {}, and {} is a link to somewhere else, which \
                 others may be able to change, so nothing was installed. If what goes there is \
                 user data or license data, install it without its user data and license data. \
                 If it is a shortcut in a Startup folder, put that folder back where it was",
                root.display(),
                link.display()
            )));
        }
    }
    Ok(())
}

/// How a file should be treated when the app is uninstalled.
fn file_item_kind(file: &FileEntry) -> ItemKind {
    match file.classification {
        FileClassification::UserData | FileClassification::MachineAppData => ItemKind::UserData,
        _ => ItemKind::Program,
    }
}

/// One restore on its way, and everything it has found out so far.
struct Run<'a, 'p> {
    options: &'a RestoreOptions,
    review: &'a ReviewedPackage<'p>,
    manifest: &'a PackageManifest,
    monitor: &'a InstallMonitor,
    app_name: &'a str,
    /// What was done, for undoing it. Only an install has one, so this is also what says
    /// whether anything is really being changed.
    journal: Option<Journal>,
    changes: Changes,
    warnings: Vec<Warning>,
    errors: Vec<Warning>,
    refused: Vec<Warning>,
    /// COM classes that belong to something else on this PC, each with why.
    com_classes_left_alone: Vec<(String, String)>,
    /// What else this PC has that a package would have taken over.
    registrations_left_alone: Vec<LeftAlone>,
    /// Whether a record says an install was at this location before this one.
    earlier_install_here: bool,
    /// The folders below a data folder that something here would go under, which were there before
    /// this install put anything in them.
    data_folders_already_here: Vec<PathBuf>,
    cancelled: bool,
    /// How the package's own files are checked.
    package_files: PackageFiles<'a>,
}

impl Run<'_, '_> {
    /// Every step, in order. Each one stops at the first thing that fails, and none starts
    /// after that: anything that fails means the install failed, and it is all put back.
    fn apply_steps(&mut self) {
        self.note_data_folders();
        // Runtimes first, so the app's own services and drivers have them available.
        self.restore_prerequisites();
        self.restore_files();
        self.restore_empty_folders();
        // Services first, so a registry key under one can never make it look like it
        // already existed.
        self.restore_services();
        self.restore_drivers();
        self.restore_registry();
        self.restore_firewall_rules();
        self.restore_license();
        self.restore_scheduled_tasks();
        self.restore_shortcuts();
        self.restore_environment();
        self.write_uninstall_entry();
    }

    /// Before anything is put in them, so that what this install makes does not count.
    fn note_data_folders(&mut self) {
        let review = self.review;
        let target_root = &self.options.target_root;
        let files = review
            .files
            .iter()
            .filter_map(|file| review.map.destination_of(&file.original_path))
            .map(Path::to_path_buf);
        // A store goes by its files, and what it is called says nothing of what is around it.
        let licenses = review.license.iter().flat_map(|license| {
            let of_a_store = license
                .artifact
                .store_files
                .iter()
                .map(|file| store_file_destination(&license.destination, file));
            let of_a_file = license
                .artifact
                .store_files
                .is_empty()
                .then(|| license.destination.clone());
            of_a_store.chain(of_a_file).collect::<Vec<_>>()
        });
        let mut folders: Vec<PathBuf> = files
            .chain(licenses)
            .filter_map(|destination| data_folder_of(&destination, target_root))
            .filter(|folder| folder.exists())
            .collect();
        folders.sort();
        folders.dedup();
        self.data_folders_already_here = folders;
    }

    /// Whether to go no further. A preview changes nothing, so problems it finds only stop
    /// it if it is cancelled.
    fn stopped(&mut self) -> bool {
        if self.monitor.is_cancelled() {
            if !self.cancelled {
                self.cancelled = true;
                let what = if self.journal.is_some() {
                    "The install was cancelled."
                } else {
                    "The preview was cancelled."
                };
                self.errors.push(Warning::new(
                    CANCELLED_CODE,
                    WarningSeverity::Warning,
                    what,
                    None,
                ));
            }
            return true;
        }
        self.journal.is_some() && !self.errors.is_empty()
    }

    fn progress(&self, phase: InstallPhase, done: usize, total: usize) {
        self.monitor.report(InstallStep::new(phase, done, total));
    }

    /// Something a step tried did not work, which fails the install, unless it is the rules
    /// refusing something, which is left out like anything else they refuse.
    fn record_failure(&mut self, failure: Warning) {
        if REFUSALS_FOUND_WHEN_INSTALLING.contains(&failure.code.as_str()) {
            let item = failure.source.clone().unwrap_or_default();
            self.refused.push(Warning::new(
                failure.code,
                WarningSeverity::Warning,
                format!("{item} was left out: {}.", failure.message),
                Some(item),
            ));
        } else {
            self.errors.push(failure);
        }
    }

    fn verify_result(&mut self) -> Option<crate::models::VerificationReport> {
        if self.journal.is_none() || self.stopped() {
            return None;
        }
        self.progress(InstallPhase::Verifying, 0, 0);
        let options = self.options;
        let monitor = self.monitor;
        match verify_prepared_package(
            &options.package_root,
            self.manifest,
            &VerificationOptions {
                package_root: options.package_root.clone(),
                target_root: Some(options.target_root.clone()),
                output_dir: Some(options.package_root.clone()),
                restore_user_data: options.restore_user_data,
                restore_license: options.restore_license,
            },
            self.package_files,
            &|| monitor.is_cancelled(),
        ) {
            Ok(report) => {
                let refused: Vec<&str> = self
                    .refused
                    .iter()
                    .filter(|item| REFUSALS_FOUND_WHEN_INSTALLING.contains(&item.code.as_str()))
                    .filter_map(|item| item.source.as_deref())
                    .collect();
                let report = without_what_was_refused(report, &refused);
                if !report.package_valid || report.restore_valid == Some(false) {
                    self.errors.extend(report.errors.clone());
                }
                Some(report)
            }
            Err(WinAppMigratorError::Cancelled) => {
                self.stopped();
                None
            }
            Err(error) => {
                self.errors.push(Warning::new(
                    "post_restore_verification_failed",
                    WarningSeverity::Error,
                    format!("Post-restore verification could not run: {error}"),
                    None,
                ));
                None
            }
        }
    }

    fn restore_files(&mut self) {
        let review = self.review;
        let total = review.files.len();
        for (done, file) in review.files.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Files, done, total);
            let (Some(relative), Some(destination)) = (
                file.package_relative_path.as_ref(),
                review.map.destination_of(&file.original_path),
            ) else {
                continue;
            };
            let source = match package_destination_path(&self.options.package_root, relative) {
                Ok(source) => source,
                Err(error) => {
                    self.errors.push(Warning::new(
                        "restore_file_failed",
                        WarningSeverity::Error,
                        format!("{error}"),
                        Some(destination.display().to_string()),
                    ));
                    return;
                }
            };

            if let Some(refusal) = program_left_alone(
                destination,
                &self.options.target_root,
                self.earlier_install_here,
                &self.data_folders_already_here,
            ) {
                self.record_failure(Warning::new(
                    FILE_EXISTS_ELSEWHERE_CODE,
                    WarningSeverity::Warning,
                    refusal,
                    Some(destination.display().to_string()),
                ));
                continue;
            }
            if destination.exists() && !self.options.force {
                let warning = Warning::new(
                    "restore_destination_exists",
                    WarningSeverity::Warning,
                    "Destination exists; use --force to overwrite.",
                    Some(destination.display().to_string()),
                );
                if self.journal.is_some() {
                    self.errors.push(warning);
                    return;
                }
                self.warnings.push(warning);
                continue;
            }

            let Some(journal) = self.journal.as_mut() else {
                self.changes.files.push(format!(
                    "would copy {} -> {}",
                    source.display(),
                    destination.display()
                ));
                continue;
            };
            let copied = copy_restore_file(
                journal,
                &source,
                destination,
                &self.options.target_root,
                file_item_kind(file),
                file.hash_sha256.as_deref(),
            );
            match copied {
                Ok(()) => self.changes.files.push(format!(
                    "copied {} -> {}",
                    source.display(),
                    destination.display()
                )),
                Err(error) => {
                    self.errors.push(Warning::new(
                        "restore_file_failed",
                        WarningSeverity::Error,
                        format!("Could not write {}: {error}", destination.display()),
                        Some(destination.display().to_string()),
                    ));
                    return;
                }
            }
        }
    }

    /// What the app had with nothing in it, as some programs count on finding it. A folder that is
    /// already there is nothing this did, so it is nothing to undo either.
    fn restore_empty_folders(&mut self) {
        let review = self.review;
        for folder in &review.empty_folders {
            if self.stopped() {
                return;
            }
            let Some(destination) = review.map.folder_destination_of(&folder.original_path) else {
                continue;
            };
            match fs::symlink_metadata(destination) {
                Ok(found) if found.is_dir() => continue,
                Ok(_) => {
                    let warning = Warning::new(
                        "restore_destination_exists",
                        WarningSeverity::Warning,
                        "Something other than a folder is where this empty folder goes.",
                        Some(destination.display().to_string()),
                    );
                    if self.journal.is_some() {
                        self.errors.push(warning);
                        return;
                    }
                    self.warnings.push(warning);
                    continue;
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    self.errors.push(Warning::new(
                        "restore_file_failed",
                        WarningSeverity::Error,
                        format!("Could not look at {}: {error}", destination.display()),
                        Some(destination.display().to_string()),
                    ));
                    return;
                }
            }
            let Some(journal) = self.journal.as_mut() else {
                self.changes.files.push(format!(
                    "would create the empty folder {}",
                    destination.display()
                ));
                continue;
            };
            match create_restored_folder(
                journal,
                destination,
                &self.options.target_root,
                file_item_kind(folder),
            ) {
                Ok(()) => self.changes.files.push(format!(
                    "created the empty folder {}",
                    destination.display()
                )),
                Err(error) => {
                    self.errors.push(Warning::new(
                        "restore_file_failed",
                        WarningSeverity::Error,
                        format!("Could not make {}: {error}", destination.display()),
                        Some(destination.display().to_string()),
                    ));
                    return;
                }
            }
        }
    }

    /// Before anything is done with a value or key: a new empty key can take over a class by
    /// itself, and a value beside a server can change what it does.
    fn is_left_alone_as_a_com_class(&mut self, entry: &RegistryEntry) -> bool {
        if self
            .registrations_left_alone
            .iter()
            .any(|left_alone| left_alone.covers(entry))
        {
            self.record_failure(registry_warning(
                REGISTRATION_EXISTS_CODE,
                entry,
                EXISTING_IS_SOMETHING_ELSE,
            ));
            return true;
        }
        let Some(class) = com_class_key(&entry.key_path) else {
            return false;
        };
        let refusal = self
            .com_classes_left_alone
            .iter()
            .find(|left_alone| left_alone.0 == class)
            .map(|left_alone| left_alone.1.clone());
        let Some(refusal) = refusal else {
            return false;
        };
        self.record_failure(registry_warning(COM_CLASS_EXISTS_CODE, entry, refusal));
        true
    }

    fn restore_registry(&mut self) {
        let review = self.review;
        let total = review.registry.len();
        if self.journal.is_some() {
            let entries: Vec<&RegistryEntry> = review
                .registry
                .iter()
                .chain(&review.license_registry)
                .copied()
                .collect();
            let on_this_pc = OnThisPc {
                map: &review.map,
                earlier_install_here: self.earlier_install_here,
            };
            self.com_classes_left_alone = com_classes_to_leave_alone(&entries, on_this_pc);
            self.registrations_left_alone =
                registrations_to_leave_alone(&entries, on_this_pc, &self.com_classes_left_alone);
            // An app's row in Settings is that app's. There is nothing to compare: an install always
            // writes how to remove it to its row, which the package does not have to say.
            if !self.earlier_install_here {
                for row in &review.uninstall_keys {
                    let key = plain_key(&row.key_path);
                    let is_there = hives_it_counts_for(&row.hive).into_iter().any(|hive| {
                        views_of_unit(&key).iter().any(|view| {
                            registry_root(hive)
                                .and_then(|root| root.open_subkey(view))
                                .is_ok()
                        })
                    });
                    if is_there {
                        self.registrations_left_alone
                            .push(LeftAlone { key, value: None });
                    }
                }
            }
            // A service or driver left out just now takes what would have gone under it along.
            self.registrations_left_alone
                .extend(self.refused.iter().filter_map(|refused| {
                    let is_a_service = SERVICES_LEFT_ALONE_CODES.contains(&refused.code.as_str());
                    let name = refused.source.as_deref().filter(|_| is_a_service)?;
                    Some(LeftAlone {
                        key: format!("{SERVICES_PREFIX}{}", name.to_ascii_lowercase()),
                        value: None,
                    })
                }));
        }
        for (done, entry) in review.registry.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Registry, done, total);
            if self.is_left_alone_as_a_com_class(entry) {
                continue;
            }
            let autostart = is_autostart_entry(entry);
            let registers = registration_of(entry).is_some();
            let Some(journal) = self.journal.as_mut() else {
                let message = format!(
                    "would restore registry {}\\{} {}",
                    registry_hive_name(&entry.hive),
                    entry.key_path,
                    entry.value_name.as_deref().unwrap_or("(key)")
                );
                if autostart {
                    self.changes.autostart.push(message);
                } else if registers {
                    self.changes.registrations.push(message);
                } else {
                    self.changes.registry.push(message);
                }
                continue;
            };
            let item = if is_uninstall_entry_key(&entry.key_path) {
                ItemKind::UninstallEntry
            } else {
                ItemKind::Program
            };
            match apply_registry_entry(journal, entry, self.options.force, &review.map, item) {
                Ok(message) if autostart => self.changes.autostart.push(message),
                Ok(message) if registers => self.changes.registrations.push(message),
                Ok(message) => self.changes.registry.push(message),
                Err(warning) => self.record_failure(warning),
            }
        }
        if !review.registry.is_empty() {
            self.warnings.push(Warning::new(
                "registry_restore_review_required",
                WarningSeverity::Info,
                "Registry values were restored from the archived structured data with known app paths rewritten to the target root.",
                None,
            ));
        }
    }

    /// Each rule by what it is for, through `netsh`, which is what Windows Firewall itself goes by.
    /// One already there is left as it is: the same rule needs nothing, and another of the same
    /// name, way, and program is someone's that this does not change.
    fn restore_firewall_rules(&mut self) {
        let review = self.review;
        for reviewed in &review.firewall_rules {
            if self.stopped() {
                return;
            }
            let rule = reviewed.entry;
            let Some(journal) = self.journal.as_mut() else {
                self.changes.firewall.push(format!(
                    "would add firewall rule {} for {}",
                    rule.name,
                    reviewed.program.display()
                ));
                continue;
            };
            let failed = |code: &'static str, message: String| {
                Warning::new(
                    code,
                    WarningSeverity::Error,
                    message,
                    Some(firewall_rule_item(&rule.name)),
                )
            };
            let stored = match stored_firewall_rules() {
                Ok(stored) => stored,
                Err(error) if error.kind() == io::ErrorKind::NotFound => Vec::new(),
                Err(error) => {
                    self.errors.push(failed(
                        "firewall_rules_unreadable",
                        format!("Windows Firewall's rules could not be read: {error}"),
                    ));
                    return;
                }
            };
            let fingerprint = firewall_fingerprint(rule);
            let identity = firewall_identity(
                rule.protocol.as_deref(),
                &rule.local_ports,
                &rule.remote_ports,
            );
            let existing: Vec<FirewallRuleEntry> = stored
                .iter()
                .filter(|text| {
                    stored_firewall_rule_matches(
                        text,
                        &rule.name,
                        &rule.direction,
                        &reviewed.program,
                    )
                })
                .filter_map(|text| parse_firewall_rule(text))
                .filter(|known| {
                    firewall_identity(
                        known.protocol.as_deref(),
                        &known.local_ports,
                        &known.remote_ports,
                    ) == identity
                })
                .collect();
            if existing
                .iter()
                .any(|known| firewall_fingerprint(known) == fingerprint)
            {
                self.changes
                    .firewall
                    .push(format!("firewall rule {} is already there", rule.name));
                continue;
            }
            if !existing.is_empty() {
                self.record_failure(failed(
                    FIREWALL_RULE_EXISTS_CODE,
                    "a rule of this name for this program is already there, set to something \
                     else, which an install does not change"
                        .to_owned(),
                ));
                continue;
            }
            let noted = journal.note(Change::FirewallRule {
                name: rule.name.trim().to_owned(),
                direction: rule.direction.clone(),
                program: reviewed.program.clone(),
                protocol: rule.protocol.clone(),
                local_ports: rule.local_ports.clone(),
                remote_ports: rule.remote_ports.clone(),
                fingerprint,
            });
            if let Err(error) = noted {
                self.errors
                    .push(failed(JOURNAL_WRITE_FAILED_CODE, error.to_string()));
                return;
            }
            let added = run_checked(
                hidden_command(NETSH_EXE).args(netsh_add_rule_arguments(rule, &reviewed.program)),
                "firewall_rule_add_failed",
                &firewall_rule_item(&rule.name),
            );
            match added {
                Ok(()) => self.changes.firewall.push(format!(
                    "added firewall rule {} for {}",
                    rule.name,
                    reviewed.program.display()
                )),
                Err(warning) => {
                    self.errors.push(warning);
                    return;
                }
            }
        }
    }

    fn restore_services(&mut self) {
        let review = self.review;
        let total = review.services.len();
        for (done, service) in review.services.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Services, done, total);
            if let ServiceAccount::Custom(original) = &service.account {
                self.warnings
                    .push(custom_account_warning(service.entry, original));
            }
            let Some(journal) = self.journal.as_mut() else {
                self.changes.services.push(format!(
                    "would restore service {} -> {} (runs as {})",
                    service.entry.service_name,
                    service_image_path_value(&service.binary, service.arguments.as_deref()),
                    service.account.label()
                ));
                continue;
            };
            let earlier_place =
                place_of_an_earlier_install(self.options, self.earlier_install_here);
            match apply_service(
                journal,
                service,
                earlier_place,
                self.options.force,
                &mut self.warnings,
            ) {
                Ok(messages) => self.changes.services.extend(messages),
                Err(warning) => {
                    self.record_failure(warning);
                    continue;
                }
            }
            for entry in &service.entry.parameters {
                match apply_registry_entry(journal, entry, true, &review.map, ItemKind::Program) {
                    Ok(message) => self.changes.services.push(message),
                    Err(warning) => {
                        self.errors.push(warning);
                        break;
                    }
                }
            }
        }
    }

    /// Install runtime prerequisites recorded in the manifest. Each prerequisite is probed
    /// first and skipped if already present; otherwise the bundled installer (or winget
    /// fallback) runs, and the result is re-probed. One that gets installed cannot be taken
    /// back out, since anything may be using it, so it is reported if the install fails.
    fn restore_prerequisites(&mut self) {
        let manifest = self.manifest;
        let total = manifest.runtime_prerequisites.len();
        for (done, prerequisite) in manifest.runtime_prerequisites.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Prerequisites, done, total);
            let Some(bundle) = prerequisite.bundle.as_ref() else {
                continue;
            };
            // Everything about how this installs comes from the compiled spec, not the
            // manifest. A kind with no spec has nothing that could run.
            let Some(spec) = spec_for_bundle(&prerequisite.kind, bundle) else {
                self.warnings.push(Warning::new(
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
                self.changes.prerequisites.push(format!(
                    "prerequisite already present: {}",
                    prerequisite.name
                ));
                continue;
            }
            if self.journal.is_none() {
                let via = if spec.windows_feature.is_some() {
                    "Windows feature"
                } else if spec.bundleable && bundle.package_relative_path.is_some() {
                    "bundled installer"
                } else {
                    "winget"
                };
                self.changes.prerequisites.push(format!(
                    "would install prerequisite {} ({via})",
                    prerequisite.name
                ));
                continue;
            }
            if spec.requires_admin && !is_elevated() {
                self.errors.push(Warning::new(
                    "prerequisite_requires_admin",
                    WarningSeverity::Error,
                    format!(
                        "Installing {} requires elevation; re-run the installer as administrator.",
                        prerequisite.name
                    ),
                    None,
                ));
                return;
            }
            // Before it runs, as with anything else. It cannot be taken back out, and it is
            // only reported, so a failure to install that says so is harmless.
            self.note_prerequisite(prerequisite);
            if self.stopped() {
                return;
            }
            match install_bundle(&prerequisite.kind, bundle, &self.options.package_root) {
                PrerequisiteInstallOutcome::Installed => {
                    self.changes
                        .prerequisites
                        .push(format!("installed prerequisite {}", prerequisite.name));
                    confirm_prerequisite(
                        prerequisite,
                        &spec,
                        false,
                        &mut self.warnings,
                        &mut self.errors,
                    );
                }
                PrerequisiteInstallOutcome::InstalledNeedsReboot => {
                    self.changes.prerequisites.push(format!(
                        "installed prerequisite {} (reboot required)",
                        prerequisite.name
                    ));
                    self.warnings.push(Warning::new(
                        "prerequisite_reboot_required",
                        WarningSeverity::Warning,
                        format!(
                            "{} was installed but a reboot is required to complete it.",
                            prerequisite.name
                        ),
                        None,
                    ));
                    confirm_prerequisite(
                        prerequisite,
                        &spec,
                        true,
                        &mut self.warnings,
                        &mut self.errors,
                    );
                }
                PrerequisiteInstallOutcome::Skipped(reason) => {
                    let noted = self.journal.as_mut().map(|journal| {
                        journal.note(Change::PrerequisiteSkipped {
                            name: prerequisite.name.clone(),
                        })
                    });
                    if let Some(Err(error)) = noted {
                        self.errors.push(Warning::new(
                            JOURNAL_WRITE_FAILED_CODE,
                            WarningSeverity::Error,
                            format!("Could not record a runtime that was skipped: {error}"),
                            None,
                        ));
                    }
                    self.warnings.push(Warning::new(
                        "prerequisite_skipped",
                        WarningSeverity::Info,
                        format!("{}: {reason}", prerequisite.name),
                        None,
                    ));
                }
                PrerequisiteInstallOutcome::Failed(reason) => {
                    self.errors.push(Warning::new(
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

    fn note_prerequisite(&mut self, prerequisite: &RuntimePrerequisite) {
        let noted = self.journal.as_mut().map(|journal| {
            journal.note(Change::Prerequisite {
                name: prerequisite.name.clone(),
            })
        });
        if let Some(Err(error)) = noted {
            self.errors.push(Warning::new(
                "journal_write_failed",
                WarningSeverity::Error,
                format!("Could not record an installed runtime: {error}"),
                None,
            ));
        }
    }

    /// Restore captured license data: files and framework-store directories back to
    /// their original (or re-anchored) locations, plus transferable registry values.
    fn restore_license(&mut self) {
        let review = self.review;
        let total = review.license.len() + review.license_registry.len();
        for (done, license) in review.license.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::License, done, total);
            let destination = &license.destination;
            let source = match package_destination_path(
                &self.options.package_root,
                license.package_relative_path,
            ) {
                Ok(path) => path,
                Err(error) => {
                    self.errors.push(Warning::new(
                        "license_source_invalid",
                        WarningSeverity::Error,
                        format!("Packaged license path is invalid: {error}"),
                        Some(license.package_relative_path.display().to_string()),
                    ));
                    continue;
                }
            };

            let Some(journal) = self.journal.as_mut() else {
                self.changes.license.push(format!(
                    "would restore license {} -> {}",
                    source.display(),
                    destination.display()
                ));
                continue;
            };
            if destination_requires_admin(destination) && !is_elevated() {
                self.errors.push(Warning::new(
                    "license_requires_admin",
                    WarningSeverity::Error,
                    "Restoring license data to a protected location requires elevation.",
                    Some(destination.display().to_string()),
                ));
                continue;
            }
            if let Some(refusal) = program_left_alone(
                destination,
                &self.options.target_root,
                self.earlier_install_here,
                &self.data_folders_already_here,
            ) {
                self.record_failure(Warning::new(
                    FILE_EXISTS_ELSEWHERE_CODE,
                    WarningSeverity::Warning,
                    refusal,
                    Some(destination.display().to_string()),
                ));
                continue;
            }
            // Honor --force for both files and framework-store directories: never
            // overwrite an existing license/store on the target without it.
            if destination.exists() && !self.options.force {
                self.errors.push(Warning::new(
                    "license_destination_exists",
                    WarningSeverity::Warning,
                    "License destination exists; use --force to overwrite.",
                    Some(destination.display().to_string()),
                ));
                continue;
            }
            // A store is copied file by file, and each goes by the same rule as any other.
            let programs_left_alone: Vec<(PathBuf, String)> = license
                .artifact
                .store_files
                .iter()
                .filter_map(|file| {
                    let to = package_destination_path(
                        destination,
                        &safe_relative_path(&file.relative_path),
                    )
                    .ok()?;
                    let refusal = program_left_alone(
                        &to,
                        &self.options.target_root,
                        self.earlier_install_here,
                        &self.data_folders_already_here,
                    )?;
                    Some((to, refusal))
                })
                .collect();
            let result = if license.artifact.store_files.is_empty() {
                copy_restore_file(
                    journal,
                    &source,
                    destination,
                    &self.options.target_root,
                    ItemKind::License,
                    license.artifact.hash_sha256.as_deref(),
                )
            } else {
                copy_license_store(
                    journal,
                    license.artifact,
                    &source,
                    destination,
                    &self.options.target_root,
                    &programs_left_alone,
                )
            };
            for (to, refusal) in programs_left_alone {
                self.record_failure(Warning::new(
                    FILE_EXISTS_ELSEWHERE_CODE,
                    WarningSeverity::Warning,
                    refusal,
                    Some(to.display().to_string()),
                ));
            }
            match result {
                Ok(()) => self.changes.license.push(format!(
                    "restored license {} -> {}",
                    source.display(),
                    destination.display()
                )),
                Err(error) => self.errors.push(Warning::new(
                    "license_restore_failed",
                    WarningSeverity::Error,
                    format!("Could not restore license data: {error}"),
                    Some(destination.display().to_string()),
                )),
            }
        }

        for (index, entry) in review.license_registry.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::License, review.license.len() + index, total);
            if self.is_left_alone_as_a_com_class(entry) {
                continue;
            }
            let Some(journal) = self.journal.as_mut() else {
                self.changes.license.push(format!(
                    "would restore license registry {}\\{}",
                    registry_hive_name(&entry.hive),
                    entry.key_path
                ));
                continue;
            };
            match apply_registry_entry(
                journal,
                entry,
                self.options.force,
                &review.map,
                ItemKind::License,
            ) {
                Ok(message) => self.changes.license.push(message),
                Err(warning) => self.record_failure(warning),
            }
        }
    }

    /// Restore application-owned drivers by placing the bundled `.sys` and re-registering
    /// the driver service. Inbox/vendor drivers cannot be redistributed and are reported
    /// with precise reinstall guidance.
    fn restore_drivers(&mut self) {
        let review = self.review;
        for driver in &review.manual_drivers {
            // Not bundled (inbox/vendor driver): precise, actionable guidance.
            self.changes
                .drivers
                .push(format!("driver not auto-restored: {}", driver.service_name));
            self.warnings.push(Warning::new(
                "driver_manual_reinstall",
                WarningSeverity::Warning,
                driver.restore_instructions.clone(),
                Some(driver.service_name.clone()),
            ));
        }
        let total = review.drivers.len();
        for (done, driver) in review.drivers.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Drivers, done, total);
            let name = &driver.entry.service_name;
            let Some(journal) = self.journal.as_mut() else {
                self.changes.drivers.push(format!(
                    "would install driver {name} -> {}",
                    driver.destination.display()
                ));
                continue;
            };
            if !is_elevated() {
                self.errors.push(Warning::new(
                    "driver_requires_admin",
                    WarningSeverity::Error,
                    "Driver registration requires elevation; re-run the installer as administrator.",
                    Some(name.clone()),
                ));
                continue;
            }
            let earlier_place =
                place_of_an_earlier_install(self.options, self.earlier_install_here);
            match place_and_register_driver(journal, driver, self.options, earlier_place) {
                Ok(messages) => self.changes.drivers.extend(messages),
                Err(warning) => self.record_failure(warning),
            }
        }
    }

    fn restore_scheduled_tasks(&mut self) {
        let review = self.review;
        let total = review.tasks.len();
        for (done, task) in review.tasks.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::ScheduledTasks, done, total);
            let name = format!("{}{}", task.entry.task_path, task.entry.task_name);
            if task.runs_as_system {
                self.warnings.push(Warning::new(
                    "scheduled_task_runs_as_system",
                    WarningSeverity::Warning,
                    format!(
                        "The scheduled task {name} runs as SYSTEM, with full rights over this PC."
                    ),
                    Some(name.clone()),
                ));
            }
            let Some(journal) = self.journal.as_mut() else {
                self.changes
                    .tasks
                    .push(format!("would restore scheduled task {name}"));
                continue;
            };
            let earlier_place =
                place_of_an_earlier_install(self.options, self.earlier_install_here);
            match apply_scheduled_task(journal, task, self.options, earlier_place) {
                Ok(message) => self.changes.tasks.push(message),
                Err(warning) => self.record_failure(warning),
            }
        }
    }

    fn restore_shortcuts(&mut self) {
        let review = self.review;
        let total = review.shortcuts.len();
        for (done, shortcut) in review.shortcuts.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Shortcuts, done, total);
            let destination = &shortcut.placement.destination;
            let autostart = shortcut.placement.autostart;
            let kind = if autostart {
                "startup shortcut"
            } else {
                "shortcut"
            };
            let Some(journal) = self.journal.as_mut() else {
                let message = format!(
                    "would restore {kind} {} -> {}",
                    destination.display(),
                    shortcut.target.display()
                );
                if autostart {
                    self.changes.autostart.push(message);
                } else {
                    self.changes.shortcuts.push(message);
                }
                continue;
            };
            let outcome = if destination.exists() && !self.options.force {
                Err(Warning::new(
                    "shortcut_destination_exists",
                    WarningSeverity::Warning,
                    "Shortcut exists; use --force to overwrite.",
                    Some(destination.display().to_string()),
                ))
            } else {
                apply_shortcut(journal, shortcut)
            };
            match outcome {
                Ok(message) if autostart => self.changes.autostart.push(message),
                Ok(message) => self.changes.shortcuts.push(message),
                // A Desktop or Start Menu shortcut is only a way in to an app that works
                // without it, so losing one is not worth undoing the install. One in a
                // Startup folder makes something run, so that is a part of what it does.
                Err(warning) if !autostart && warning.code != JOURNAL_WRITE_FAILED_CODE => {
                    self.warnings.push(warning);
                }
                Err(warning) => self.errors.push(warning),
            }
        }
        if review
            .shortcuts
            .iter()
            .any(|shortcut| shortcut.placement.for_all_users)
        {
            self.warnings.push(Warning::new(
                "shortcut_admin_scope",
                WarningSeverity::Info,
                "One or more restored shortcuts target a public desktop or common Start Menu location.",
                None,
            ));
        }
    }

    fn restore_environment(&mut self) {
        let review = self.review;
        let total = review.environment.len();
        for (done, variable) in review.environment.iter().enumerate() {
            if self.stopped() {
                return;
            }
            self.progress(InstallPhase::Environment, done, total);
            let Some(journal) = self.journal.as_mut() else {
                self.changes.environment.push(format!(
                    "would restore {:?} environment variable {}",
                    variable.scope, variable.name
                ));
                continue;
            };
            match apply_environment_variable(
                journal,
                variable,
                self.options.force,
                &self.options.target_root,
                &review.map,
            ) {
                Ok(message) => self.changes.environment.push(message),
                Err(warning) => self.errors.push(warning),
            }
        }
        if !review.environment.is_empty() {
            self.warnings.push(Warning::new(
                "environment_restore_session_refresh_required",
                WarningSeverity::Info,
                "Environment variable changes may require a new process, shell, or sign-in session to be observed.",
                None,
            ));
        }
    }

    /// Give the app an entry in Settings that runs the uninstaller. If the package had an
    /// entry, it keeps what it said about the app, with these in place of what it said about
    /// how to remove it from the PC it came from. The rules have already left those out.
    fn write_uninstall_entry(&mut self) {
        if self.stopped() {
            return;
        }
        let review = self.review;
        let Some(journal) = self.journal.as_mut() else {
            return;
        };
        let Some(uninstaller) = journal.uninstaller_path().map(Path::to_path_buf) else {
            return;
        };
        self.monitor
            .report(InstallStep::new(InstallPhase::UninstallEntry, 0, 0));
        let id = journal.id().to_owned();
        // A row that is another app's stays that app's, and this install has one of its own.
        let mut keys: Vec<_> = review
            .uninstall_keys
            .iter()
            .filter(|key| {
                !self
                    .registrations_left_alone
                    .iter()
                    .any(|left_alone| left_alone.covers_key(&key.key_path))
            })
            .cloned()
            .collect();
        let captured = !keys.is_empty();
        if keys.is_empty() {
            keys.push(crate::restore_review::UninstallKey {
                hive: RegistryHive::LocalMachine,
                // Named for the app and not the install, so a second install over the first
                // takes over its line in Settings, and the first's comes back with it.
                key_path: uninstall_key_for(self.app_name),
                has_display_name: false,
            });
        }
        let values = uninstall_entry_values(
            &uninstaller,
            &id,
            self.app_name,
            self.manifest,
            &self.options.target_root,
        );
        for key in &keys {
            for (name, value) in &values {
                // What a captured entry already says about the app stays as it said.
                let wanted = UNINSTALL_COMMAND_VALUES.contains(name)
                    || !captured
                    || (*name == DISPLAY_NAME_VALUE && !key.has_display_name);
                if !wanted {
                    continue;
                }
                if let Err(warning) = set_journaled_value(
                    journal,
                    &key.hive,
                    &key.key_path,
                    name,
                    value,
                    ItemKind::UninstallEntry,
                ) {
                    self.errors.push(warning);
                    return;
                }
            }
            self.changes.registry.push(format!(
                "set the uninstaller for {}\\{}",
                registry_hive_name(&key.hive),
                key.key_path
            ));
        }
    }
}

/// The key of an entry for an app that came with none. A key's name can be nearly anything but a
/// separator, so two apps only share one if they have the same name.
fn uninstall_key_for(app_name: &str) -> String {
    let name: String = app_name
        .chars()
        .filter(|character| !character.is_control())
        .map(|character| if character == '\\' { '_' } else { character })
        .take(MAX_UNINSTALL_KEY_NAME_CHARS)
        .collect();
    format!("{UNINSTALL_KEY_FOR_NEW_ENTRIES}\\WinAppMigrator {name}")
}

/// The command that uninstalls an install, with the program's path quoted.
pub(crate) fn uninstall_command(uninstaller: &Path, id: &str) -> String {
    format!("\"{}\" {UNINSTALL_FLAG} {id}", uninstaller.display())
}

/// The values of an app's entry in Settings.
fn uninstall_entry_values(
    uninstaller: &Path,
    id: &str,
    app_name: &str,
    manifest: &PackageManifest,
    target_root: &Path,
) -> Vec<(&'static str, RegValue<'static>)> {
    let text = |value: &str| RegValue {
        bytes: Cow::Owned(encode_utf16_nul(value)),
        vtype: REG_SZ,
    };
    let flag = || RegValue {
        bytes: Cow::Owned(1_u32.to_le_bytes().to_vec()),
        vtype: REG_DWORD,
    };
    let command = uninstall_command(uninstaller, id);
    let mut values = vec![
        (UNINSTALL_STRING_VALUE, text(&command)),
        (
            QUIET_UNINSTALL_STRING_VALUE,
            text(&format!("{command} {QUIET_FLAG}")),
        ),
        // There is nothing that could change or repair it, only remove it.
        (NO_MODIFY_VALUE, flag()),
        (NO_REPAIR_VALUE, flag()),
        (DISPLAY_NAME_VALUE, text(app_name)),
        (INSTALL_LOCATION_VALUE, text(&target_root.to_string_lossy())),
    ];
    let app = manifest.application_metadata.as_ref();
    if let Some(version) = app.and_then(|app| app.version.as_deref()) {
        values.push((DISPLAY_VERSION_VALUE, text(version)));
    }
    if let Some(publisher) = app.and_then(|app| app.publisher.as_deref()) {
        values.push((PUBLISHER_VALUE, text(publisher)));
    }
    values
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

/// Copy the files the manifest lists for a license store. They were all hash-checked in
/// the package before anything was written.
fn copy_license_store(
    journal: &mut Journal,
    artifact: &LicenseArtifact,
    source: &Path,
    destination: &Path,
    target_root: &Path,
    left_alone: &[(PathBuf, String)],
) -> Result<()> {
    for file in &artifact.store_files {
        let relative = safe_relative_path(&file.relative_path);
        let to = package_destination_path(destination, &relative)?;
        if left_alone.iter().any(|(path, _)| *path == to) {
            continue;
        }
        copy_restore_file(
            journal,
            &package_destination_path(source, &relative)?,
            &to,
            target_root,
            ItemKind::License,
            Some(&file.hash_sha256),
        )?;
    }
    Ok(())
}

/// The folder a file outside the install location would go under, first below the data folder it
/// is in, or that data folder if the file would go straight in it.
fn data_folder_of(destination: &Path, target_root: &Path) -> Option<PathBuf> {
    if path_is_strictly_inside(destination, target_root) {
        return None;
    }
    let root = write_root_for(destination, target_root)?;
    let relative = strip_prefix_ignore_case(destination, &root)?;
    let mut parts = relative.components();
    let first = parts.next()?;
    Some(if parts.next().is_some() {
        root.join(first)
    } else {
        root
    })
}

/// [`program_placement_refusal`], for a folder of those that were there before. It is the file's
/// own folder of that kind that counts: a data folder is on the list for what goes straight in it,
/// and that says nothing of a new folder beside what is there.
fn program_left_alone(
    destination: &Path,
    target_root: &Path,
    earlier_install_here: bool,
    folders_already_here: &[PathBuf],
) -> Option<String> {
    let was_there = data_folder_of(destination, target_root).is_some_and(|its_folder| {
        folders_already_here.iter().any(|folder| {
            path_is_within(&its_folder, folder) && path_is_within(folder, &its_folder)
        })
    });
    program_placement_refusal(destination, target_root, earlier_install_here, was_there)
}

/// Decide whether a driver can be restored before touching anything, then put its image
/// in place and register it. An existing image is not replaced unless `--force` was
/// given, and an image whose hash is wrong is never registered: a driver loads into the
/// kernel and is the last thing to take on trust.
fn place_and_register_driver(
    journal: &mut Journal,
    reviewed: &ReviewedDriver<'_>,
    options: &RestoreOptions,
    earlier_place: &Path,
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
        if let Some(refusal) = existing_service_refusal(name, current, earlier_place, system_root())
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
    copy_restore_file(
        journal,
        source,
        destination,
        &options.target_root,
        ItemKind::Program,
        driver.hash_sha256.as_deref(),
    )
    .map_err(|error| {
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
    let prior = read_service_config(name).map_err(|error| {
        problem(
            "driver_query_failed",
            format!("Could not read the driver's current settings: {error}"),
        )
    })?;
    journal
        .note(Change::Service {
            name: name.to_owned(),
            driver: true,
            binary: destination.to_path_buf(),
            arguments: None,
            prior,
        })
        .map_err(|error| problem("journal_write_failed", error.to_string()))?;
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

/// Hash-check the package before anything is written, so a truncated or damaged
/// copy is refused instead of running prerequisites and overwriting the app first.
fn refuse_damaged_package(
    options: &RestoreOptions,
    manifest: &PackageManifest,
    package_files: PackageFiles<'_>,
) -> Result<()> {
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
        package_files,
        &|| false,
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

/// Write a registry value through the journal, keeping what it replaces.
fn set_journaled_value(
    journal: &mut Journal,
    hive: &RegistryHive,
    key_path: &str,
    name: &str,
    value: &RegValue<'_>,
    item: ItemKind,
) -> std::result::Result<(), Warning> {
    let source = || {
        format!(
            "{}\\{key_path}\\{}",
            registry_hive_name(hive),
            if name.is_empty() { "(Default)" } else { name }
        )
    };
    let failed = |code: &'static str, message: String| {
        Warning::new(code, WarningSeverity::Error, message, Some(source()))
    };
    journal
        .note_new_keys(hive, key_path, item)
        .map_err(|error| failed("journal_write_failed", error.to_string()))?;
    let (key, _) = registry_root(hive)
        .and_then(|root| root.create_subkey(key_path))
        .map_err(|error| {
            failed(
                "registry_key_create_failed",
                format!("Could not create registry key: {error}"),
            )
        })?;
    let prior = read_prior_value(&key, name).map_err(|error| {
        failed(
            "registry_value_read_failed",
            format!("Could not read the value this would replace: {error}"),
        )
    })?;
    journal
        .note(Change::RegistryValue {
            hive: hive.clone(),
            key: key_path.to_owned(),
            name: name.to_owned(),
            prior,
            written: StoredValue::from_reg(value),
            item,
        })
        .map_err(|error| failed("journal_write_failed", error.to_string()))?;
    key.set_raw_value(name, value).map_err(|error| {
        failed(
            "registry_value_write_failed",
            format!("Could not write registry value: {error}"),
        )
    })
}

/// What a value is now, if it is there. Not being able to tell is not the same as it not being
/// there: writing over it could then not be undone.
fn read_prior_value(key: &winreg::RegKey, name: &str) -> io::Result<Option<StoredValue>> {
    match key.get_raw_value(name) {
        Ok(value) => Ok(Some(StoredValue::from_reg(&value))),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn apply_registry_entry(
    journal: &mut Journal,
    entry: &RegistryEntry,
    force: bool,
    map: &RestoreMap,
    item: ItemKind,
) -> std::result::Result<String, Warning> {
    let key_source = || format!("{}\\{}", registry_hive_name(&entry.hive), entry.key_path);
    journal
        .note_new_keys(&entry.hive, &entry.key_path, item)
        .map_err(|error| {
            Warning::new(
                "journal_write_failed",
                WarningSeverity::Error,
                error.to_string(),
                Some(key_source()),
            )
        })?;
    let (key, _) = registry_root(&entry.hive)
        .and_then(|root| root.create_subkey(&entry.key_path))
        .map_err(|error| {
            Warning::new(
                "registry_key_create_failed",
                WarningSeverity::Error,
                format!("Could not create registry key: {error}"),
                Some(key_source()),
            )
        })?;

    if entry.value_name.is_none() && entry.value_kind == Some(RegistryValueKind::None) {
        return Ok(format!(
            "ensured registry key {}\\{}",
            registry_hive_name(&entry.hive),
            entry.key_path
        ));
    }

    let value_name = restored_value_name(entry, map);
    let value_name = value_name.as_ref();
    if !force && key.get_raw_value(value_name).is_ok() {
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
    set_journaled_value(
        journal,
        &entry.hive,
        &entry.key_path,
        value_name,
        &value,
        item,
    )?;
    Ok(format!(
        "restored registry {}\\{} {}",
        registry_hive_name(&entry.hive),
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    ))
}

/// What is on this PC goes by what the package would write, and by whether what names the install
/// location counts for anything. A package suggests that place itself, so naming it says nothing
/// unless a record says an install was there before: a package that said its home was Chrome's folder
/// would otherwise have everything of Chrome's be its own.
#[derive(Clone, Copy)]
struct OnThisPc<'a> {
    map: &'a RestoreMap,
    earlier_install_here: bool,
}

impl OnThisPc<'_> {
    fn place_of_an_earlier_install(&self) -> &Path {
        if self.earlier_install_here {
            self.map.target_root()
        } else {
            Path::new(NOWHERE)
        }
    }

    fn runs_from_an_earlier_install(&self, text: &str) -> bool {
        first_path_in(text)
            .is_some_and(|path| path_is_strictly_inside(&path, self.place_of_an_earlier_install()))
    }
}

/// Where what is on this PC can be taken for an earlier install of this app.
fn place_of_an_earlier_install(options: &RestoreOptions, earlier_install_here: bool) -> &Path {
    if earlier_install_here {
        &options.target_root
    } else {
        Path::new(NOWHERE)
    }
}

/// The COM classes in `entries` that belong to something else on this PC, each with why. All of
/// a class goes together, whatever order it comes in, since a value beside a server can change
/// what it does, and a new empty key can take over a class by itself.
///
/// A class is someone else's if its key is on this PC and it is not from an earlier install here.
/// That goes for both views, since what starts a class in its own process can find it in either, and
/// a per-user registration wins over the machine's, so a new one in `HKCU` takes over a class that
/// only exists in `HKLM` just as surely. It takes no server of its own to count: a class a service
/// hosts has none, and adding one redirects it. The only thing a package can then ask is what this
/// PC has already, as when installing an app again, and it has to be all of it: otherwise a server
/// copied from this PC would let in what comes beside it.
fn com_classes_to_leave_alone(
    entries: &[&RegistryEntry],
    on_this_pc: OnThisPc<'_>,
) -> Vec<(String, String)> {
    let mut left_alone: Vec<(String, String)> = Vec::new();
    let mut looked_at: Vec<(String, &RegistryHive)> = Vec::new();
    for entry in entries {
        let Some(class) = com_class_key(&entry.key_path) else {
            continue;
        };
        if looked_at
            .iter()
            .any(|seen| seen.0 == class && seen.1 == &entry.hive)
        {
            continue;
        }
        looked_at.push((class.clone(), &entry.hive));
        let id = class.rsplit('\\').next().unwrap_or_default();
        let views = [
            class.clone(),
            format!(r"software\classes\clsid\{id}"),
            format!(r"software\classes\wow6432node\clsid\{id}"),
        ];
        let refusal = hives_it_counts_for(&entry.hive)
            .into_iter()
            .find_map(|hive| {
                views.iter().find_map(|view| {
                    let live = registry_root(hive)
                        .and_then(|root| root.open_subkey(view))
                        .ok()?;
                    COM_SERVER_KEYS
                        .iter()
                        .find_map(|server| {
                            let existing = live
                                .open_subkey(server)
                                .and_then(|key| key.get_value::<String, _>(""))
                                .ok()?;
                            com_server_refusal(
                                server,
                                &existing,
                                "",
                                on_this_pc.place_of_an_earlier_install(),
                                system_root(),
                            )
                        })
                        .or_else(|| {
                            (!names_an_earlier_install(&live, on_this_pc, DEEPEST_LOOK_IN_A_UNIT))
                                .then(|| CLASS_IS_SOMEONES.to_owned())
                        })
                })
            });
        let Some(refusal) = refusal else {
            continue;
        };
        let is_what_this_pc_has = entries
            .iter()
            .filter(|other| com_class_key(&other.key_path).as_deref() == Some(class.as_str()))
            .all(|other| is_as_it_is_on_this_pc(other, on_this_pc.map));
        if !is_what_this_pc_has && !left_alone.iter().any(|known| known.0 == class) {
            left_alone.push((class, refusal));
        }
    }
    left_alone
}

/// Something this PC has that a package must not change, by the key it is under, and the one value
/// of it if that is all it is. Comparing goes by a key's normalized name, so a 32-bit view and the
/// 64-bit view are not taken for different things.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LeftAlone {
    key: String,
    value: Option<String>,
}

impl LeftAlone {
    /// Whether this is all of a key, or one above it.
    fn covers_key(&self, key_path: &str) -> bool {
        self.value.is_none()
            && plain_key(key_path)
                .strip_prefix(self.key.as_str())
                .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
    }

    fn covers(&self, entry: &RegistryEntry) -> bool {
        let key = plain_key(&entry.key_path);
        let under = key
            .strip_prefix(self.key.as_str())
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
            // A driver's name in the list of them is part of that driver.
            || registration_unit_of(entry).is_some_and(|unit| unit == self.key);
        under
            && self.value.as_deref().is_none_or(|value| {
                value == registry_value_name(entry.value_name.as_deref()).to_ascii_lowercase()
            })
    }
}

/// A key in lowercase, without what makes it the 32-bit view.
fn plain_key(key_path: &str) -> String {
    key_path
        .split('\\')
        .filter(|part| !part.is_empty() && !part.eq_ignore_ascii_case("wow6432node"))
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("\\")
}

/// What an entry would change on this PC, if it is set to something else there, in either hive it
/// would count for. One that is what would be written, or runs from an earlier install at the same
/// place, is no one else's.
fn differs_on_this_pc(entry: &RegistryEntry, on_this_pc: OnThisPc<'_>) -> bool {
    let map = on_this_pc.map;
    let Ok(written) = registry_value_from_entry(entry, map) else {
        return true;
    };
    let name = restored_value_name(entry, map);
    let written = registry_value_text(&written);
    hives_it_counts_for(&entry.hive).into_iter().any(|hive| {
        let live = registry_root(hive)
            .and_then(|root| root.open_subkey(&entry.key_path))
            .and_then(|key| key.get_raw_value(name.as_ref()));
        live.is_ok_and(|live| {
            let live = registry_value_text(&live);
            !live.trim().eq_ignore_ascii_case(written.trim())
                && !on_this_pc.runs_from_an_earlier_install(&live)
        })
    })
}

/// Whether what an entry would write is what this PC has for it already, so nothing would change.
/// That goes for what kind of value it is as well. Only a key on its own has no value to compare.
fn is_as_it_is_on_this_pc(entry: &RegistryEntry, map: &RestoreMap) -> bool {
    let Ok(key) = registry_root(&entry.hive).and_then(|root| root.open_subkey(&entry.key_path))
    else {
        return false;
    };
    if entry.value_name.is_none() && entry.value_kind == Some(RegistryValueKind::None) {
        return true;
    }
    let Ok(written) = registry_value_from_entry(entry, map) else {
        return false;
    };
    key.get_raw_value(restored_value_name(entry, map).as_ref())
        .is_ok_and(|live| {
            live.vtype == written.vtype
                && registry_value_text(&live)
                    .trim()
                    .eq_ignore_ascii_case(registry_value_text(&written).trim())
        })
}

/// A per-user registration wins over the machine's, so a new one takes that over just as surely.
fn hives_it_counts_for(hive: &RegistryHive) -> Vec<&'static RegistryHive> {
    match hive {
        RegistryHive::CurrentUser => {
            vec![&RegistryHive::CurrentUser, &RegistryHive::LocalMachine]
        }
        RegistryHive::LocalMachine => vec![&RegistryHive::LocalMachine],
    }
}

/// The command something opens with on this PC, in a hive it counts for.
fn live_open_command(hive: &RegistryHive, class_key: &str) -> Option<String> {
    let classes = registry_root(hive).ok()?.open_subkey(class_key).ok()?;
    let shell = classes.open_subkey(SHELL_KEY_SEGMENT).ok();
    let default_verb = shell
        .as_ref()
        .and_then(|shell| shell.get_value::<String, _>("").ok())
        .unwrap_or_default();
    [default_verb.as_str(), OPEN_VERB]
        .iter()
        .filter(|verb| !verb.trim().is_empty())
        .find_map(|verb| {
            let shell = shell.as_ref()?;
            shell
                .open_subkey(format!(r"{}\command", verb.trim()))
                .ok()?
                .get_value::<String, _>("")
                .ok()
                .filter(|command| !command.trim().is_empty())
        })
}

/// Whether a type on this PC is someone's: what opens it runs from elsewhere, or it opens by a class
/// or a handler, which there is no command to go by for.
fn live_type_is_someones(hive: &RegistryHive, class_key: &str, on_this_pc: OnThisPc<'_>) -> bool {
    let Ok(class) = registry_root(hive).and_then(|root| root.open_subkey(class_key)) else {
        return false;
    };
    if let Some(command) = live_open_command(hive, class_key) {
        return !on_this_pc.runs_from_an_earlier_install(&command);
    }
    if class.open_subkey(CLASS_REFERENCE_KEY).is_ok() {
        return true;
    }
    class.open_subkey(SHELL_KEY_SEGMENT).is_ok_and(|shell| {
        shell.enum_keys().flatten().any(|verb| {
            !shell
                .open_subkey(format!(r"{verb}\command"))
                .and_then(|command| command.get_value::<String, _>(""))
                .is_ok_and(|command| on_this_pc.runs_from_an_earlier_install(&command))
        })
    })
}

/// What in a type's keys is a verb, which is its own thing to add to the type's menu. A folder has
/// a menu for its background too.
fn verb_prefix_of(key: &str, class_key: &str) -> Option<String> {
    [
        format!(r"{class_key}\shell\"),
        format!(r"{class_key}\background\shell\"),
    ]
    .into_iter()
    .find(|prefix| key.starts_with(prefix.as_str()))
}

/// The keys one of a package's units can be on this PC under, in the views that it has.
fn views_of_unit(unit: &str) -> Vec<String> {
    let mut views = vec![unit.to_owned()];
    if let Some(rest) = unit.strip_prefix(r"software\classes\") {
        views.push(format!(r"software\classes\wow6432node\{rest}"));
    } else if let Some(rest) = unit.strip_prefix(r"software\") {
        views.push(format!(r"software\wow6432node\{rest}"));
    }
    views
}

/// Whether anything under a key is in the install location of an earlier install at the same place.
fn names_an_earlier_install(key: &winreg::RegKey, on_this_pc: OnThisPc<'_>, deeper: usize) -> bool {
    key.enum_values()
        .flatten()
        .any(|(_, value)| on_this_pc.runs_from_an_earlier_install(&registry_value_text(&value)))
        || (deeper > 0
            && key.enum_keys().flatten().any(|name| {
                key.open_subkey(&name)
                    .is_ok_and(|below| names_an_earlier_install(&below, on_this_pc, deeper - 1))
            }))
}

/// Whether this PC has one of a package's units as something else's: its key is there, and what is
/// in it is not from an earlier install here. A value a package only adds to it changes what it does
/// just as surely as one it replaces.
fn unit_is_someones(unit: &str, of_it: &[&RegistryEntry], on_this_pc: OnThisPc<'_>) -> bool {
    let is_there_as_someones = of_it
        .iter()
        .flat_map(|entry| hives_it_counts_for(&entry.hive))
        .any(|hive| {
            views_of_unit(unit).iter().any(|view| {
                registry_root(hive)
                    .and_then(|root| root.open_subkey(view))
                    .is_ok_and(|key| {
                        !names_an_earlier_install(&key, on_this_pc, DEEPEST_LOOK_IN_A_UNIT)
                    })
            })
        });
    is_there_as_someones
        && !of_it
            .iter()
            .all(|entry| is_as_it_is_on_this_pc(entry, on_this_pc.map))
}

/// `AppID\<program>` and `App Paths\<program>` go by a program's file name alone, so this PC's own
/// go by what they have. A file of that name in a package does not make it the same program.
fn names_a_program_of_windows(unit: &str) -> bool {
    let Some(name) = unit
        .strip_prefix(APP_ID_KEY_PREFIX)
        .or_else(|| unit.strip_prefix(APP_PATHS_KEY_PREFIX))
    else {
        return false;
    };
    // Looking for a name that is really a path would go wherever it says, even onto a network.
    let is_a_plain_name = !name.contains(['/', '\\', ':'])
        && matches!(
            Path::new(name).components().collect::<Vec<_>>().as_slice(),
            [Component::Normal(_)]
        );
    is_a_plain_name
        && !name.starts_with('{')
        && [
            system_root().to_path_buf(),
            system_root().join("System32"),
            system_root().join("SysWOW64"),
        ]
        .iter()
        .any(|folder| folder.join(name).is_file())
}

/// What in a package this PC already has as something else's: everything a package could take from
/// another program by setting it, with the rest of the install going ahead. `--force` does not
/// change this. It reads the live registry, and only for what a package asks for.
fn registrations_to_leave_alone(
    entries: &[&RegistryEntry],
    on_this_pc: OnThisPc<'_>,
    com_left_alone: &[(String, String)],
) -> Vec<LeftAlone> {
    let map = on_this_pc.map;
    let mut left_alone: Vec<LeftAlone> = Vec::new();
    let mut leave = |key: String, value: Option<String>| {
        let item = LeftAlone { key, value };
        if !left_alone.contains(&item) {
            left_alone.push(item);
        }
    };
    // All of something goes together, and an event log source is by its name in any log.
    let mut units: BTreeMap<String, Vec<&RegistryEntry>> = BTreeMap::new();
    for entry in entries {
        if com_class_key(&entry.key_path).is_none()
            && let Some(unit) = registration_unit_of(entry)
        {
            units.entry(unit).or_default().push(entry);
        }
    }
    for (unit, of_it) in &units {
        let source_elsewhere = unit
            .strip_prefix(EVENT_LOG_PREFIX)
            .and_then(|rest| rest.split_once('\\'))
            .is_some_and(|(log, source)| event_source_is_in_another_log(log, source));
        if source_elsewhere
            || names_a_program_of_windows(unit)
            || unit_is_someones(unit, of_it, on_this_pc)
            || of_it
                .iter()
                .any(|entry| differs_on_this_pc(entry, on_this_pc))
        {
            leave(unit.clone(), None);
        }
    }
    // A type or protocol, by what opening one runs. What is not from an earlier install is someone's.
    // A verb is only its own, when that is all the package adds to the type.
    let mut classes: BTreeMap<String, Vec<&RegistryEntry>> = BTreeMap::new();
    for entry in entries {
        let key = plain_key(&entry.key_path);
        if let Some(rest) = key.strip_prefix(r"software\classes\") {
            let top = rest.split('\\').next().unwrap_or_default();
            if !NOT_TYPES.contains(&top) {
                classes.entry(top.to_owned()).or_default().push(entry);
            }
        }
    }
    for (class, of_it) in &classes {
        let class_key = format!(r"software\classes\{class}");
        let is_extension = class.starts_with('.');
        let shell_keys = [
            format!(r"{class_key}\shell"),
            format!(r"{class_key}\background\shell"),
        ];
        let open_key = format!(r"{class_key}\shell\{OPEN_VERB}");
        let says_what_it_is = of_it.iter().any(|entry| {
            let key = plain_key(&entry.key_path);
            if key == open_key || key.starts_with(&format!(r"{open_key}\")) {
                return true;
            }
            // Which verb opening one runs is what it opens with, whichever that is.
            if shell_keys.contains(&key) {
                return registry_value_name(entry.value_name.as_deref()).is_empty()
                    && entry.value_kind != Some(RegistryValueKind::None);
            }
            verb_prefix_of(&key, &class_key).is_none()
                && !key.contains(r"\shellex")
                && !key.contains(r"\openwithprogids")
                && !key.contains(r"\openwithlist")
                && !is_extension
        });
        let hives: BTreeSet<&'static str> = of_it
            .iter()
            .flat_map(|entry| hives_it_counts_for(&entry.hive))
            .map(|hive| registry_hive_name(hive))
            .collect();
        let live_is_someones = hives.iter().any(|name| {
            let hive = if *name == registry_hive_name(&RegistryHive::LocalMachine) {
                &RegistryHive::LocalMachine
            } else {
                &RegistryHive::CurrentUser
            };
            live_type_is_someones(hive, &class_key, on_this_pc)
        });
        if says_what_it_is
            && live_is_someones
            && !of_it.iter().all(|entry| is_as_it_is_on_this_pc(entry, map))
        {
            leave(class_key.clone(), None);
            continue;
        }
        for entry in of_it {
            let key = plain_key(&entry.key_path);
            if !differs_on_this_pc(entry, on_this_pc) {
                continue;
            }
            if let Some(prefix) = verb_prefix_of(&key, &class_key) {
                let verb = key[prefix.len()..].split('\\').next().unwrap_or_default();
                leave(format!("{prefix}{verb}"), None);
            } else if key.contains(r"\shellex") {
                leave(key, None);
            } else if is_extension {
                // What it says it opens with may be a type of an earlier install here.
                let current = registry_root(&entry.hive)
                    .and_then(|root| root.open_subkey(&class_key))
                    .and_then(|key| key.get_value::<String, _>(""))
                    .unwrap_or_default();
                let its_own = live_open_command(
                    &entry.hive,
                    &format!(r"software\classes\{}", current.trim()),
                )
                .is_some_and(|command| on_this_pc.runs_from_an_earlier_install(&command));
                if !its_own {
                    leave(
                        key,
                        Some(registry_value_name(entry.value_name.as_deref()).to_ascii_lowercase()),
                    );
                }
            } else {
                leave(class_key.clone(), None);
            }
        }
    }
    // Lists where one name means one thing. A class left alone takes what names it with it.
    for entry in entries {
        if is_autostart_entry(entry) && differs_on_this_pc(entry, on_this_pc) {
            leave(
                plain_key(&entry.key_path),
                Some(registry_value_name(entry.value_name.as_deref()).to_ascii_lowercase()),
            );
            continue;
        }
        match registration_of(entry) {
            Some(Registration::ShellExtension { class, .. }) => {
                let is_left_alone = class.as_deref().is_some_and(|class| {
                    com_left_alone
                        .iter()
                        .any(|(key, _)| key.rsplit('\\').next() == Some(class))
                });
                if is_left_alone || differs_on_this_pc(entry, on_this_pc) {
                    leave(plain_key(&entry.key_path), None);
                }
            }
            Some(Registration::Font { .. } | Registration::DefaultAppsEntry { .. })
                if differs_on_this_pc(entry, on_this_pc) =>
            {
                leave(
                    plain_key(&entry.key_path),
                    Some(registry_value_name(entry.value_name.as_deref()).to_ascii_lowercase()),
                );
            }
            _ => {}
        }
    }
    left_alone
}

/// A source's name means one thing wherever it is, so one in another log counts as being there.
fn event_source_is_in_another_log(log: &str, source: &str) -> bool {
    let Ok(logs) =
        winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE).open_subkey(EVENT_LOG_KEY)
    else {
        return false;
    };
    logs.enum_keys().flatten().any(|other| {
        !other.eq_ignore_ascii_case(log) && logs.open_subkey(format!(r"{other}\{source}")).is_ok()
    })
}

fn firewall_rule_item(name: &str) -> String {
    format!("The firewall rule {name}")
}

fn apply_environment_variable(
    journal: &mut Journal,
    variable: &EnvironmentVariableEntry,
    force: bool,
    target_root: &Path,
    map: &RestoreMap,
) -> std::result::Result<String, Warning> {
    let failed = |code: &'static str, message: String| {
        Warning::new(
            code,
            WarningSeverity::Error,
            message,
            Some(format!("{:?}\\{}", variable.scope, variable.name)),
        )
    };
    let (key, _) = environment_registry_target(&variable.scope)
        .and_then(|(root, key_path)| root.create_subkey(key_path))
        .map_err(|error| {
            failed(
                "environment_key_create_failed",
                format!("Could not create environment registry key: {error}"),
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
    let prior = read_prior_value(&key, &variable.name).map_err(|error| {
        failed(
            "environment_value_read_failed",
            format!("Could not read the value this would replace: {error}"),
        )
    })?;

    let rewritten = map.rewrite(&variable.value);
    let (change, value) = if is_path {
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
        let added: Vec<String> = entries
            .iter()
            .filter(|entry| !path_contains_entry(&current, entry))
            .cloned()
            .collect();
        let value = RegValue {
            bytes: Cow::Owned(encode_utf16_nul(&merge_path_entries(&current, &entries))),
            vtype,
        };
        (
            Change::PathEntries {
                scope: variable.scope.clone(),
                name: variable.name.clone(),
                added,
                prior,
                written: StoredValue::from_reg(&value),
            },
            value,
        )
    } else {
        let value = RegValue {
            bytes: Cow::Owned(encode_utf16_nul(&rewritten)),
            vtype: registry_raw_type(&variable.value_kind),
        };
        (
            Change::EnvironmentValue {
                scope: variable.scope.clone(),
                name: variable.name.clone(),
                prior,
                written: StoredValue::from_reg(&value),
            },
            value,
        )
    };
    journal
        .note(change)
        .map_err(|error| failed("journal_write_failed", error.to_string()))?;
    key.set_raw_value(&variable.name, &value).map_err(|error| {
        failed(
            "environment_value_write_failed",
            format!("Could not write environment variable: {error}"),
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
    journal: &mut Journal,
    reviewed: &ReviewedService<'_>,
    earlier_place: &Path,
    force: bool,
    warnings: &mut Vec<Warning>,
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
        .and_then(|current| existing_service_refusal(name, current, earlier_place, system_root()))
    {
        return Err(Warning::new(
            "service_exists_elsewhere",
            WarningSeverity::Error,
            refusal,
            Some(name.to_owned()),
        ));
    }
    let problem = |code: &'static str, message: String| {
        Warning::new(code, WarningSeverity::Error, message, Some(name.to_owned()))
    };
    let prior = read_service_config(name).map_err(|error| {
        problem(
            "service_query_failed",
            format!("Could not read the service's current settings: {error}"),
        )
    })?;
    journal
        .note(Change::Service {
            name: name.to_owned(),
            driver: false,
            binary: reviewed.binary.clone(),
            arguments: reviewed.arguments.clone(),
            prior,
        })
        .map_err(|error| problem("journal_write_failed", error.to_string()))?;

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
        // Only what Services shows about it, so not worth undoing the install for.
        if let Err(warning) = run_checked(
            &mut command,
            "service_description_failed",
            &service.service_name,
        ) {
            warnings.push(warning);
        }
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

/// The folders of a task's path, outermost first, as the Task Scheduler names them.
fn task_folder_levels(task_path: &str) -> Vec<String> {
    let mut levels = Vec::new();
    let mut current = String::new();
    for part in task_path.split('\\').filter(|part| !part.is_empty()) {
        current.push('\\');
        current.push_str(part);
        levels.push(current.clone());
    }
    levels
}

/// The task folders on this list that are not there. Windows makes them when a task is
/// registered and does not take them away, so an undo has to know which were new.
fn missing_task_folders(levels: &[String]) -> io::Result<Vec<String>> {
    if levels.is_empty() {
        return Ok(Vec::new());
    }
    let names = levels
        .iter()
        .map(|level| format!("'{}'", escape_powershell_single(level)))
        .collect::<Vec<_>>()
        .join(",");
    let script = format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_UTILITY}\
         $s = New-Object -ComObject Schedule.Service; $s.Connect(); \
         foreach ($path in @({names})) {{ try {{ $null = $s.GetFolder($path) }} catch {{ $path }} }}"
    );
    let output = powershell_command(&script).output()?;
    if !output.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Register a task. Its definition is passed through a file in the staging folder, which
/// other processes cannot write to. A fixed name in `%TEMP%` could be planted as a link.
fn apply_scheduled_task(
    journal: &mut Journal,
    task: &ReviewedTask<'_>,
    options: &RestoreOptions,
    earlier_place: &Path,
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
        if let Some(refusal) = existing_task_refusal(existing, earlier_place) {
            return Err(Warning::new(
                "scheduled_task_exists_elsewhere",
                WarningSeverity::Error,
                refusal,
                Some(name),
            ));
        }
    }
    let problem = |code: &'static str, message: String| {
        Warning::new(
            code,
            WarningSeverity::Error,
            message,
            Some(format!("{}{}", task.entry.task_path, task.entry.task_name)),
        )
    };

    if journal.first_look_at_task_folder(&task.entry.task_path) {
        let missing =
            missing_task_folders(&task_folder_levels(&task.entry.task_path)).map_err(|error| {
                problem(
                    "scheduled_task_query_failed",
                    format!("Could not look at the task's folder: {error}"),
                )
            })?;
        for path in missing {
            journal
                .note(Change::TaskFolderCreated { path })
                .map_err(|error| problem("journal_write_failed", error.to_string()))?;
        }
    }
    journal
        .note(Change::Task {
            task_path: task.entry.task_path.clone(),
            task_name: task.entry.task_name.clone(),
            prior_xml: existing,
            written_xml: None,
        })
        .map_err(|error| problem("journal_write_failed", error.to_string()))?;

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
    let registered = run_powershell_script(&script);
    let _ = fs::remove_file(&xml_path);
    registered?;
    // How Windows wrote it back is what an uninstall compares against later.
    if let Ok(Some(xml)) = live_task_xml(&task.entry.task_path, &task.entry.task_name) {
        journal
            .note(Change::TaskWritten {
                task_path: task.entry.task_path.clone(),
                task_name: task.entry.task_name.clone(),
                xml,
            })
            .map_err(|error| problem("journal_write_failed", error.to_string()))?;
    }
    Ok(format!("restored scheduled task {name}"))
}

/// Write a task's definition into `folder` under a new name, failing rather than opening
/// anything already there.
pub(crate) fn write_task_definition(folder: &Path, xml: &str) -> io::Result<PathBuf> {
    let path = folder.join(format!("task-{}.xml", partial_file_name()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(xml.as_bytes())?;
    Ok(path)
}

pub(crate) fn register_task_script(
    xml_path: &Path,
    task_path: &str,
    task_name: &str,
    force: bool,
) -> String {
    let force_flag = if force { "$true" } else { "$false" };
    format!(
        "{POWERSHELL_IMPORT_MANAGEMENT}{POWERSHELL_IMPORT_TASKS}$xml = Get-Content -Raw -Encoding UTF8 -LiteralPath '{}'; Register-ScheduledTask -TaskName '{}' -TaskPath '{}' -Xml $xml -Force:{force_flag} | Out-Null",
        escape_powershell_single(&xml_path.display().to_string()),
        escape_powershell_single(task_name),
        escape_powershell_single(task_path),
    )
}

/// Make a shortcut, keeping one already there. It is not this install's to lose, and if this
/// one cannot be made it goes back, since a shortcut is not worth failing an install for.
fn apply_shortcut(
    journal: &mut Journal,
    shortcut: &ReviewedShortcut,
) -> std::result::Result<String, Warning> {
    let placement = &shortcut.placement;
    let destination = &placement.destination;
    // Before anything is recorded, because whether a record can be kept is another matter: not
    // being able to is worth undoing an install for, and this may not be.
    if let Some(link) = first_link_to(&placement.root) {
        return Err(Warning::new(
            "shortcut_destination_invalid",
            WarningSeverity::Error,
            link_refusal(&link),
            Some(destination.display().to_string()),
        ));
    }
    let journal_failed = |error: WinAppMigratorError| {
        Warning::new(
            "journal_write_failed",
            WarningSeverity::Error,
            error.to_string(),
            Some(destination.display().to_string()),
        )
    };
    journal
        .note_new_folders_to(&placement.root, ItemKind::Program)
        .map_err(journal_failed)?;
    fs::create_dir_all(&placement.root)
        .map_err(|error| warning_io("shortcut_parent_create_failed", &placement.root, error))?;
    let invalid_destination = |error: WinAppMigratorError| {
        Warning::new(
            "shortcut_destination_invalid",
            WarningSeverity::Error,
            error.to_string(),
            Some(destination.display().to_string()),
        )
    };
    ensure_within_root(&placement.root, destination).map_err(invalid_destination)?;
    if let Some(parent) = destination.parent() {
        journal
            .note_new_folders(&placement.root, parent, ItemKind::Program)
            .map_err(journal_failed)?;
    }
    let _pinned = pin_path_below_root(&placement.root, destination).map_err(invalid_destination)?;
    let replaces = fs::symlink_metadata(destination).is_ok();
    let backup = journal
        .note_file(
            destination,
            &placement.root,
            ItemKind::Program,
            replaces,
            None,
            None,
        )
        .map_err(journal_failed)?;
    if let Some(number) = backup {
        journal
            .back_up(destination, number)
            .map_err(journal_failed)?;
    }
    if let Err(warning) = run_powershell_script(&shortcut_script(shortcut)) {
        if let Some(found) = backup.and_then(|number| find_backup(journal.folder(), number))
            && let Err(error) = put_back_backup(&found, destination)
        {
            return Err(Warning::new(
                "shortcut_put_back_failed",
                WarningSeverity::Error,
                format!(
                    "The shortcut could not be made, and the one it replaced could not be put \
                     back: {error}"
                ),
                Some(destination.display().to_string()),
            ));
        }
        journal
            .note(Change::FileUndone {
                path: destination.clone(),
            })
            .map_err(journal_failed)?;
        return Err(warning);
    }
    let sha256 = sha256_file(destination).map_err(journal_failed)?;
    journal
        .note(Change::FileWritten {
            path: destination.clone(),
            sha256,
        })
        .map_err(journal_failed)?;
    Ok(format!("restored shortcut {}", destination.display()))
}

fn shortcut_script(shortcut: &ReviewedShortcut) -> String {
    let mut script = format!(
        "{POWERSHELL_IMPORT_UTILITY}$shell = New-Object -ComObject WScript.Shell; $shortcut = $shell.CreateShortcut('{}'); $shortcut.TargetPath = '{}';",
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

/// Put a file from the package at its destination, recording it first and keeping what it
/// replaces. It is written beside the destination under a new name, then renamed over it. A
/// rename replaces the name at the destination and cannot be carried through it, unlike
/// opening it for writing, which follows a link and also writes through a hard link. A failure
/// part-way through a file also leaves the old one in place, and if the process is killed,
/// the journal knows the new name and can remove it.
fn copy_restore_file(
    journal: &mut Journal,
    source: &Path,
    destination: &Path,
    target_root: &Path,
    item: ItemKind,
    sha256: Option<&str>,
) -> Result<()> {
    if journal
        .state_root()
        .is_some_and(|state_root| path_is_within(destination, state_root))
    {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "refusing to write where installs keep their records: {}",
            destination.display()
        )));
    }
    let root = write_root_for(destination, target_root).ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "refusing to write outside the install location and the known data folders: {}",
            destination.display()
        ))
    })?;
    // A known data folder need not be there yet, unlike the install location.
    if fs::symlink_metadata(&root).is_err() {
        journal.note_new_folders_to(&root, item)?;
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
    }
    ensure_within_root(&root, destination)?;
    let parent = destination.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "destination has no parent folder: {}",
            destination.display()
        ))
    })?;
    journal.note_new_folders(&root, parent, item)?;
    let _pinned = pin_path_below_root(&root, destination)?;

    let replaces = match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.is_dir() => {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "a folder is where this file has to go: {}",
                destination.display()
            )));
        }
        Ok(_) => true,
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => return Err(WinAppMigratorError::io(destination, error)),
    };
    let partial = parent.join(partial_file_name());
    let backup = journal.note_file(destination, &root, item, replaces, sha256, Some(&partial))?;
    // With a file to put back, what is written has to be on disk before it takes that file's
    // place. After a power cut, what stands there may be taken for something the user changed.
    let copied = write_partial_copy(source, &partial, replaces).and_then(|()| {
        if let Some(number) = backup {
            journal.back_up(destination, number)?;
        }
        fs::rename(&partial, destination)
            .map_err(|error| WinAppMigratorError::io(destination, error))
    });
    if copied.is_err() {
        let _ = fs::remove_file(&partial);
    }
    copied
}

/// Only what a file in a folder would be called, for what checks the way to one.
const FOLDER_PIN_PLACEHOLDER: &str = "_";

/// Make a folder of the app's with nothing in it, held to the same as a file going there.
fn create_restored_folder(
    journal: &mut Journal,
    destination: &Path,
    target_root: &Path,
    item: ItemKind,
) -> Result<()> {
    if journal
        .state_root()
        .is_some_and(|state_root| path_is_within(destination, state_root))
    {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "refusing to write where installs keep their records: {}",
            destination.display()
        )));
    }
    let root = write_root_for(destination, target_root).ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "refusing to write outside the install location and the known data folders: {}",
            destination.display()
        ))
    })?;
    if fs::symlink_metadata(&root).is_err() {
        journal.note_new_folders_to(&root, item)?;
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
    }
    ensure_within_root(&root, destination)?;
    journal.note_new_folders(&root, destination, item)?;
    // What is held on the way to a file makes each folder it takes, and goes through no link.
    let in_it = destination.join(FOLDER_PIN_PLACEHOLDER);
    let _pinned = pin_path_below_root(&root, &in_it)?;
    Ok(())
}

/// A rename does not replace a read-only file, and an earlier install of the same app can
/// have left one, since a copy keeps the read-only attribute. Whether to replace it has
/// been decided by this point. On Windows this clears that one attribute, and nothing else.
#[allow(clippy::permissions_set_readonly_false)]
pub(crate) fn make_replaceable(destination: &Path) -> Result<()> {
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
pub(crate) fn partial_file_name() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    format!(
        ".wam-{}-{}.partial",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

/// Copy a file under a new name, with its times and read-only attribute. With `durable`, its
/// contents are on disk before this returns, for a copy that may be all that is left.
pub(crate) fn write_partial_copy(source: &Path, partial: &Path, durable: bool) -> Result<()> {
    let mut input = File::open(source).map_err(|error| WinAppMigratorError::io(source, error))?;
    let output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(partial)
        .map_err(|error| WinAppMigratorError::io(partial, error))?;
    let mut output = BufWriter::with_capacity(COPY_BUFFER_BYTES, output);
    io::copy(&mut input, &mut output).map_err(|error| WinAppMigratorError::io(partial, error))?;
    let output = output
        .into_inner()
        .map_err(|error| WinAppMigratorError::io(partial, error.into_error()))?;
    if durable {
        output
            .sync_all()
            .map_err(|error| WinAppMigratorError::io(partial, error))?;
    }
    drop(output);
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
        || !review.firewall_rules.is_empty()
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

pub(crate) fn encode_utf16_nul(value: &str) -> Vec<u8> {
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

pub(crate) fn sc_start_type(value: &str) -> Option<&'static str> {
    match value {
        "automatic" => Some("auto"),
        DELAYED_AUTOMATIC_START_LABEL => Some("delayed-auto"),
        "manual" => Some("demand"),
        "disabled" => Some("disabled"),
        "boot" => Some("boot"),
        "system" => Some("system"),
        _ => None,
    }
}

pub(crate) fn run_checked(
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

/// What PowerShell said went wrong, which is its first line. The lines after that say where in
/// the script it happened, in the language of this Windows, and quote the script.
fn powershell_error_summary(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or_default()
        .to_owned()
}

pub(crate) fn run_powershell_script(script: &str) -> std::result::Result<(), Warning> {
    let output = powershell_command(script).output().map_err(|error| {
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
            powershell_error_summary(&String::from_utf8_lossy(&output.stdout)),
            powershell_error_summary(&String::from_utf8_lossy(&output.stderr))
        ),
        None,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::fs::OpenOptionsExt;
    use std::sync::{Arc, Mutex};

    use crate::journal::{install_folders, read_journal};
    use crate::restore_policy::driver_destination;
    use crate::rollback::{UndoMode, open_install, undo_journal};

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
            state_root: PathBuf::from("state"),
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
            assert!(
                crate::restore_policy::is_com_server_entry(&entry),
                "{server}"
            );
            assert_eq!(
                com_classes_to_leave_alone(&[&entry], earlier_install_here(&map)).len(),
                1,
                "{server}"
            );
        }

        let own_class = com_entry(
            RegistryHive::LocalMachine,
            r"SOFTWARE\Classes\CLSID\{5A7E6E0C-0000-4000-8000-000000000000}\InprocServer32"
                .to_owned(),
            RegistryValueKind::String,
        );
        assert!(com_classes_to_leave_alone(&[&own_class], earlier_install_here(&map)).is_empty());
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
            state_root: work.join("state"),
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
            let mut journal = journal_for_test(&options)?;
            let refused =
                place_and_register_driver(&mut journal, &reviewed, &options, &options.target_root);
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

        let mut journal = journal_for_test(&options)?;
        let refused =
            place_and_register_driver(&mut journal, &reviewed, &options, &options.target_root);

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

    fn journal_for_test(options: &RestoreOptions) -> Result<Journal> {
        Journal::begin(&options.state_root, "Acme", &options.target_root, None)
    }

    fn file_entry(relative: &str, hash: Option<String>, size: u64) -> FileEntry {
        let original = Path::new(r"C:\Program Files\Acme").join(relative);
        FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(Path::new("files").join(relative)),
            size_bytes: Some(size),
            hash_sha256: hash,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    fn bare_manifest() -> PackageManifest {
        PackageManifest {
            schema_version: crate::models::PACKAGE_SCHEMA_VERSION.to_owned(),
            source_computer_name: None,
            source_windows_version: None,
            source_platform: None,
            program_needs: Vec::new(),
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
            secrets_left_out: Vec::new(),
            licenses_left_out: Vec::new(),
            settings_left_out: Vec::new(),
            empty_folders: Vec::new(),
            firewall_rules_detected: Vec::new(),
            certificates_reported: Vec::new(),
            fonts_reported: Vec::new(),
            left_out_by_choice: Vec::new(),
            added_by_choice: Vec::new(),
            warnings: Vec::new(),
            items_intentionally_excluded: Vec::new(),
            included_user_data: false,
            included_program_data: false,
            included_license_data: false,
            user_data_profile: None,
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
            state_root: root.join("state"),
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
        assert!(!options.state_root.exists(), "not even a journal's folder");
        assert_eq!(result.install_id, None);
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

    /// Copy through a journal of its own, for what only cares about the copying.
    fn copy_file(source: &Path, destination: &Path, root: &Path, state: &Path) -> Result<()> {
        let mut journal = Journal::begin(state, "Acme", root, None)?;
        copy_restore_file(
            &mut journal,
            source,
            destination,
            root,
            ItemKind::Program,
            None,
        )
    }

    #[test]
    fn copying_creates_the_file_and_leaves_nothing_else_behind() -> Result<()> {
        let work = temp_folder()?;
        let source = work.path().join("source.bin");
        write_file(&source, b"new")?;
        let root = work.path().join("target");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let destination = root.join(r"bin\app.exe");

        copy_file(&source, &destination, &root, &work.path().join("state"))?;
        write_file(&source, b"newer")?;
        copy_file(&source, &destination, &root, &work.path().join("state"))?;

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

        copy_file(&source, &destination, &root, &work.path().join("state"))?;

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

        let refused = copy_file(&source, outside, &root, &work.path().join("state"));
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

        copy_file(&source, &destination, &root, &work.path().join("state"))?;

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

        assert!(copy_file(&source, &destination, &root, &work.path().join("state")).is_err());
        assert_eq!(
            fs::read(&victim).map_err(|error| WinAppMigratorError::io(&victim, error))?,
            b"keep"
        );
        Ok(())
    }

    /// Hold a file open the way another program might, so it cannot be moved or replaced.
    fn hold_exclusively(path: &Path) -> Result<File> {
        OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(path)
            .map_err(|error| WinAppMigratorError::io(path, error))
    }

    fn read_text(path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(|error| WinAppMigratorError::io(path, error))
    }

    fn is_read_only(path: &Path) -> Result<bool> {
        Ok(fs::metadata(path)
            .map_err(|error| WinAppMigratorError::io(path, error))?
            .permissions()
            .readonly())
    }

    fn set_read_only(path: &Path) -> Result<()> {
        let mut permissions = fs::metadata(path)
            .map_err(|error| WinAppMigratorError::io(path, error))?
            .permissions();
        permissions.set_readonly(true);
        fs::set_permissions(path, permissions).map_err(|error| WinAppMigratorError::io(path, error))
    }

    fn names_in(folder: &Path) -> Result<Vec<String>> {
        let mut names: Vec<String> = fs::read_dir(folder)
            .map_err(|error| WinAppMigratorError::io(folder, error))?
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        Ok(names)
    }

    fn junction(link: &Path, target: &Path) -> Result<()> {
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(link)
            .arg(target)
            .output()
            .map_err(|error| WinAppMigratorError::io(link, error))?;
        assert!(made.status.success());
        Ok(())
    }

    /// Install a package of plain files into a new target, and return how, with its id.
    fn installed(
        work: &Path,
        files: &[(&str, &[u8])],
        edit: impl FnOnce(&mut PackageManifest),
    ) -> Result<(RestoreOptions, String)> {
        let package = package_of(work, files, edit)?;
        let mut options = restore_options(work, package, true);
        options.force = true;
        let result = restore_package(&options)?;
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let id = result.install_id.ok_or_else(|| {
            WinAppMigratorError::InvalidInput("a finished install has an id".to_owned())
        })?;
        Ok((options, id))
    }

    fn uninstall(
        options: &RestoreOptions,
        id: &str,
        remove_user_data: bool,
    ) -> Result<crate::rollback::UninstallOutcome> {
        Ok(open_install(&options.state_root, id)?
            .uninstall(remove_user_data, &InstallMonitor::default()))
    }

    #[test]
    fn a_failure_part_way_puts_back_everything_it_did() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(
            work.path(),
            &[
                ("kept.dat", b"new kept"),
                (r"sub\deep\new.dat", b"new"),
                ("held.dat", b"new held"),
                ("never.dat", b"never reached"),
            ],
            |_| {},
        )?;
        let mut options = restore_options(work.path(), package, true);
        options.force = true;
        let target = options.target_root.clone();
        write_file(&target.join("kept.dat"), b"old kept")?;
        set_read_only(&target.join("kept.dat"))?;
        write_file(&target.join("held.dat"), b"old held")?;
        write_file(&target.join("unrelated.txt"), b"mine")?;
        let held = hold_exclusively(&target.join("held.dat"))?;

        let result = restore_package(&options)?;
        drop(held);

        assert!(!result.errors.is_empty(), "the held file should fail it");
        assert_eq!(result.install_id, None);
        let report = result.rollback.ok_or_else(|| {
            WinAppMigratorError::InvalidInput("a failed install is rolled back".to_owned())
        })?;
        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert_eq!(read_text(&target.join("kept.dat"))?, "old kept");
        assert!(is_read_only(&target.join("kept.dat"))?, "kept as it was");
        assert_eq!(read_text(&target.join("held.dat"))?, "old held");
        assert_eq!(
            names_in(&target)?,
            ["held.dat", "kept.dat", "unrelated.txt"],
            "nothing new is left, including partial files and folders"
        );
        assert!(
            install_folders(&options.state_root)?.is_empty(),
            "the backups and the record go with it"
        );
        Ok(())
    }

    /// Change what was installed the moment the install moves on to checking it.
    fn monitor_that_changes_a_file_before_it_is_checked(file: PathBuf) -> InstallMonitor {
        InstallMonitor::new(move |step: InstallStep| {
            if step.phase == InstallPhase::Verifying {
                let _ = fs::write(&file, b"changed");
            }
        })
    }

    #[test]
    fn a_failure_takes_back_the_folders_it_made_for_the_install_location() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let mut options = restore_options(work.path(), package, true);
        options.target_root = work.path().join(r"new\deep\target");
        let monitor =
            monitor_that_changes_a_file_before_it_is_checked(options.target_root.join("a.dat"));

        let result = restore_package_monitored(&options, &monitor, None)?;

        assert!(!result.errors.is_empty());
        assert!(!work.path().join("new").exists(), "{:?}", result.rollback);
        Ok(())
    }

    #[test]
    fn a_file_that_fails_its_check_after_being_written_rolls_the_install_back() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a"), ("b.dat", b"b")], |_| {})?;
        let mut options = restore_options(work.path(), package, true);
        options.force = true;
        let target = options.target_root.clone();
        let monitor = monitor_that_changes_a_file_before_it_is_checked(target.join("a.dat"));

        let result = restore_package_monitored(&options, &monitor, None)?;

        assert!(
            result
                .errors
                .iter()
                .any(|error| error.code == "verification_restored_file_failed"),
            "{:?}",
            result.errors
        );
        assert_eq!(result.install_id, None);
        let report = result.rollback.ok_or_else(|| {
            WinAppMigratorError::InvalidInput("it should be rolled back".to_owned())
        })?;
        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert!(!target.exists(), "what it changed goes too, as a rollback");
        Ok(())
    }

    #[test]
    fn cancelling_part_way_puts_everything_back() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(
            work.path(),
            &[("a.dat", b"a"), ("b.dat", b"b"), ("c.dat", b"c")],
            |_| {},
        )?;
        let mut options = restore_options(work.path(), package, true);
        options.force = true;
        write_file(&options.target_root.join("a.dat"), b"old a")?;
        let holder: Arc<Mutex<Option<InstallMonitor>>> = Arc::new(Mutex::new(None));
        let observer_holder = Arc::clone(&holder);
        let monitor = InstallMonitor::new(move |step: InstallStep| {
            if step.phase == InstallPhase::Files
                && step.done == 2
                && let Ok(held) = observer_holder.lock()
                && let Some(monitor) = held.as_ref()
            {
                monitor.cancel();
            }
        });
        if let Ok(mut held) = holder.lock() {
            *held = Some(monitor.clone());
        }

        let result = restore_package_monitored(&options, &monitor, None)?;

        assert_eq!(
            result.errors.first().map(|error| error.code.as_str()),
            Some(CANCELLED_CODE)
        );
        assert_eq!(result.install_id, None);
        assert!(result.rollback.is_some());
        assert_eq!(read_text(&options.target_root.join("a.dat"))?, "old a");
        assert!(!options.target_root.join("b.dat").exists());
        assert!(install_folders(&options.state_root)?.is_empty());
        Ok(())
    }

    #[test]
    fn a_preview_reports_progress_and_cancelling_it_changes_nothing() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let options = restore_options(work.path(), package, false);
        let monitor = InstallMonitor::default();
        monitor.cancel();

        let result = restore_package_monitored(&options, &monitor, None)?;

        assert!(result.rollback.is_none());
        assert_eq!(
            result.errors.first().map(|error| error.code.as_str()),
            Some(CANCELLED_CODE)
        );
        assert!(!options.state_root.exists());
        assert!(!options.target_root.exists());
        Ok(())
    }

    #[test]
    fn progress_is_reported_for_every_file_and_for_checking() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a"), ("b.dat", b"b")], |_| {})?;
        let options = restore_options(work.path(), package, true);
        let seen: Arc<Mutex<Vec<InstallStep>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let monitor = InstallMonitor::new(move |step| {
            if let Ok(mut steps) = sink.lock() {
                steps.push(step);
            }
        });

        restore_package_monitored(&options, &monitor, None)?;

        let steps = seen
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        let phases: Vec<InstallPhase> = steps.iter().map(|step| step.phase).collect();
        assert_eq!(
            phases,
            [
                InstallPhase::CheckingPackage,
                InstallPhase::Files,
                InstallPhase::Files,
                InstallPhase::Verifying,
            ]
        );
        assert_eq!(steps[2].fraction(), Some(0.5));
        Ok(())
    }

    /// As a killed process leaves things: some of it done, and a journal not marked finished.
    fn interrupted_install(work: &Path) -> Result<RestoreOptions> {
        let package = package_of(
            work,
            &[("a.dat", b"new a"), (r"sub\b.dat", b"new b")],
            |_| {},
        )?;
        let mut options = restore_options(work, package.clone(), true);
        options.force = true;
        write_file(&options.target_root.join("a.dat"), b"old a")?;
        let mut journal = journal_for_test(&options)?;
        journal.note_new_folders_to(&options.target_root, ItemKind::Program)?;
        for relative in ["a.dat", r"sub\b.dat"] {
            let source = package.join("files").join(relative);
            copy_restore_file(
                &mut journal,
                &source,
                &options.target_root.join(relative),
                &options.target_root,
                ItemKind::Program,
                Some(&sha256_file(&source)?),
            )?;
        }
        // A journal is only ever closed like this when a process dies.
        drop(journal);
        assert_eq!(read_text(&options.target_root.join("a.dat"))?, "new a");
        Ok(options)
    }

    #[test]
    fn an_install_that_never_finished_is_seen_by_a_preview_and_undone_by_the_next_install()
    -> Result<()> {
        let work = temp_folder()?;
        let interrupted = interrupted_install(work.path())?;
        let other = work.path().join("other");
        let other_package = package_of(&other, &[("other.dat", b"other")], |_| {})?;
        let mut next = restore_options(&other, other_package, false);
        next.state_root = interrupted.state_root.clone();

        // A preview says so and changes nothing.
        let preview = restore_package(&next)?;
        assert!(
            preview
                .warnings
                .iter()
                .any(|warning| warning.code == "earlier_install_never_finished")
        );
        assert_eq!(
            read_text(&interrupted.target_root.join("a.dat"))?,
            "new a",
            "a preview leaves it alone"
        );
        assert_eq!(install_folders(&next.state_root)?.len(), 1);

        next.apply = true;
        next.dry_run = false;
        let result = restore_package(&next)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.code == "earlier_install_undone")
        );
        assert_eq!(read_text(&interrupted.target_root.join("a.dat"))?, "old a");
        assert!(!interrupted.target_root.join("sub").exists());
        assert!(next.target_root.join("other.dat").is_file());
        assert_eq!(
            install_folders(&next.state_root)?.len(),
            1,
            "only the new one"
        );
        Ok(())
    }

    #[test]
    fn an_install_still_running_is_left_alone_by_another() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let options = restore_options(work.path(), package.clone(), true);
        let mut running = journal_for_test(&options)?;
        running.note_new_folders_to(&options.target_root, ItemKind::Program)?;
        fs::create_dir_all(&options.target_root)
            .map_err(|error| WinAppMigratorError::io(&options.target_root, error))?;
        copy_restore_file(
            &mut running,
            &package.join(r"files\a.dat"),
            &options.target_root.join("a.dat"),
            &options.target_root,
            ItemKind::Program,
            None,
        )?;
        let other = work.path().join("other");
        let other_package = package_of(&other, &[("other.dat", b"other")], |_| {})?;
        let mut next = restore_options(&other, other_package, true);
        next.state_root = options.state_root.clone();

        let result = restore_package(&next)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(options.target_root.join("a.dat").is_file());
        assert_eq!(install_folders(&next.state_root)?.len(), 2);
        drop(running);
        Ok(())
    }

    #[test]
    fn what_cannot_be_undone_stops_the_next_install_until_it_can_be() -> Result<()> {
        let work = temp_folder()?;
        let interrupted = interrupted_install(work.path())?;
        let other = work.path().join("other");
        let other_package = package_of(&other, &[("other.dat", b"other")], |_| {})?;
        let mut next = restore_options(&other, other_package, true);
        next.state_root = interrupted.state_root.clone();
        let held = hold_exclusively(&interrupted.target_root.join(r"sub\b.dat"))?;

        let refused = restore_package(&next);

        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("could not be fully undone") && message.contains("b.dat")),
            "{:?}",
            refused.as_ref().err()
        );
        assert!(!next.target_root.join("other.dat").exists());
        assert_eq!(install_folders(&next.state_root)?.len(), 1);

        drop(held);
        let result = restore_package(&next)?;
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(!interrupted.target_root.join("sub").exists());
        Ok(())
    }

    #[test]
    fn a_rollback_cut_off_part_way_can_be_run_again_and_again() -> Result<()> {
        let work = temp_folder()?;
        let interrupted = interrupted_install(work.path())?;
        let folder = install_folders(&interrupted.state_root)?
            .pop()
            .ok_or_else(|| WinAppMigratorError::InvalidInput("no journal".to_owned()))?;
        let journal = read_journal(&folder)?;
        let monitor = InstallMonitor::default();

        // Undo only what came last, as a rollback does before it is killed.
        let mut part = journal.clone();
        let roots: Vec<Change> = part
            .changes
            .iter()
            .filter(|change| matches!(change, Change::Root { .. }))
            .cloned()
            .collect();
        let tail = part.changes.split_off(part.changes.len() - 2);
        part.changes = roots;
        part.changes.extend(tail);
        let first = undo_journal(
            &part,
            UndoMode::Rollback,
            &monitor,
            InstallPhase::RollingBack,
        );
        assert!(first.failed.is_empty(), "{:?}", first.failed);
        assert!(!interrupted.target_root.join(r"sub\b.dat").exists());
        assert_eq!(read_text(&interrupted.target_root.join("a.dat"))?, "new a");

        let second = undo_journal(
            &journal,
            UndoMode::Rollback,
            &monitor,
            InstallPhase::RollingBack,
        );
        assert!(second.failed.is_empty(), "{:?}", second.failed);
        assert_eq!(read_text(&interrupted.target_root.join("a.dat"))?, "old a");
        assert!(!interrupted.target_root.join("sub").exists());

        let third = undo_journal(
            &journal,
            UndoMode::Rollback,
            &monitor,
            InstallPhase::RollingBack,
        );
        assert!(third.failed.is_empty(), "{:?}", third.failed);
        assert!(third.undone.is_empty(), "{:?}", third.undone);
        assert_eq!(read_text(&interrupted.target_root.join("a.dat"))?, "old a");
        Ok(())
    }

    #[test]
    fn uninstalling_removes_what_is_unchanged_and_the_record_of_it() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(
            work.path(),
            &[("a.dat", b"a"), (r"sub\deep\b.dat", b"b")],
            |_| {},
        )?;
        let summary = open_install(&options.state_root, &id)?.summary();
        assert_eq!(summary.files, 2);
        assert!(summary.finished);

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(outcome.report.kept.is_empty(), "{:?}", outcome.report.kept);
        assert!(!options.target_root.exists());
        assert!(install_folders(&options.state_root)?.is_empty());
        Ok(())
    }

    #[test]
    fn uninstalling_keeps_what_changed_what_the_user_added_and_the_folder_they_are_in() -> Result<()>
    {
        let work = temp_folder()?;
        let (options, id) = installed(
            work.path(),
            &[
                ("a.dat", b"a"),
                ("changed.dat", b"as installed"),
                (r"sub\b.dat", b"b"),
            ],
            |_| {},
        )?;
        let target = options.target_root.clone();
        write_file(&target.join("changed.dat"), b"edited by the user")?;
        write_file(&target.join(r"sub\mine.txt"), b"added by the user")?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert_eq!(names_in(&target)?, ["changed.dat", "sub"]);
        assert_eq!(names_in(&target.join("sub"))?, ["mine.txt"]);
        assert_eq!(
            read_text(&target.join("changed.dat"))?,
            "edited by the user"
        );
        let kept = outcome.report.kept.join("\n");
        assert!(kept.contains("changed.dat"), "{kept}");
        assert!(kept.contains("sub"), "{kept}");
        // Said once, for the folder that has them, and not for each folder above it as well.
        assert_eq!(
            outcome
                .report
                .kept
                .iter()
                .filter(|line| line.starts_with("The folder"))
                .count(),
            1,
            "{kept}"
        );
        Ok(())
    }

    /// A folder below where the app was, as a package lists it.
    fn empty_folder(relative: &str) -> FileEntry {
        let mut folder = file_entry(relative, None, 0);
        folder.size_bytes = None;
        folder.reason = "An empty folder of the app's".to_owned();
        folder
    }

    #[test]
    fn an_empty_folder_is_made_and_goes_when_the_install_does() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a")], |manifest| {
            manifest.empty_folders = vec![
                empty_folder("textfile_inputs"),
                empty_folder(r"deep\down\there"),
            ];
        })?;
        let target = options.target_root.clone();
        assert!(target.join("textfile_inputs").is_dir());
        assert!(target.join(r"deep\down\there").is_dir());
        assert!(names_in(&target.join("textfile_inputs"))?.is_empty());

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(!target.join("textfile_inputs").exists());
        assert!(!target.join("deep").exists());
        Ok(())
    }

    #[test]
    fn an_empty_folder_that_got_something_in_it_stays_and_says_so() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a")], |manifest| {
            manifest.empty_folders = vec![empty_folder("textfile_inputs")];
        })?;
        let target = options.target_root.clone();
        write_file(&target.join(r"textfile_inputs\mine.prom"), b"added")?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert_eq!(names_in(&target.join("textfile_inputs"))?, ["mine.prom"]);
        let kept = outcome.report.kept.join("\n");
        assert!(kept.contains("textfile_inputs"), "{kept}");
        Ok(())
    }

    #[test]
    fn a_folder_already_there_is_nothing_an_uninstall_takes() -> Result<()> {
        let work = temp_folder()?;
        let target = work.path().join("target");
        let already = target.join("textfile_inputs");
        fs::create_dir_all(&already).map_err(|error| WinAppMigratorError::io(&already, error))?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a")], |manifest| {
            manifest.empty_folders = vec![empty_folder("textfile_inputs")];
        })?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(already.is_dir());
        Ok(())
    }

    #[test]
    fn a_preview_makes_no_empty_folder_and_says_it_would() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |manifest| {
            manifest.empty_folders = vec![empty_folder("textfile_inputs")];
        })?;
        let options = restore_options(work.path(), package, false);
        let result = restore_package(&options)?;
        let said = result.changes_logged.join("\n");
        assert!(said.contains("would create the empty folder"), "{said}");
        assert!(!options.target_root.join("textfile_inputs").exists());
        Ok(())
    }

    #[test]
    fn a_failed_install_takes_back_the_empty_folders_it_made() -> Result<()> {
        let work = temp_folder()?;
        // A folder where a file has to go is what stops this install, after the folders are made.
        let target = work.path().join("target");
        let package = package_of(work.path(), &[("a.dat", b"a")], |manifest| {
            manifest.empty_folders = vec![empty_folder("made"), empty_folder(r"a.dat\below")];
        })?;
        let mut options = restore_options(work.path(), package, true);
        options.force = true;
        let result = restore_package(&options)?;
        assert!(!result.errors.is_empty());
        assert!(!target.join("made").exists());
        Ok(())
    }

    #[test]
    fn uninstalling_brings_back_what_was_overwritten_as_it_was() -> Result<()> {
        let work = temp_folder()?;
        let target = work.path().join("target");
        write_file(&target.join("a.dat"), b"from before")?;
        set_read_only(&target.join("a.dat"))?;
        let (options, id) = installed(work.path(), &[("a.dat", b"new")], |_| {})?;
        assert_eq!(read_text(&target.join("a.dat"))?, "new");

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert_eq!(read_text(&target.join("a.dat"))?, "from before");
        assert!(is_read_only(&target.join("a.dat"))?);
        Ok(())
    }

    #[test]
    fn a_changed_file_that_replaced_another_stays_and_the_other_goes_beside_it() -> Result<()> {
        let work = temp_folder()?;
        let target = work.path().join("target");
        write_file(&target.join("a.dat"), b"from before")?;
        let (options, id) = installed(work.path(), &[("a.dat", b"new")], |_| {})?;
        write_file(&target.join("a.dat"), b"edited since")?;

        let outcome = uninstall(&options, &id, false)?;

        assert_eq!(read_text(&target.join("a.dat"))?, "edited since");
        assert_eq!(
            read_text(&target.join("a.dat.before-install"))?,
            "from before"
        );
        assert!(
            outcome
                .report
                .kept
                .join("\n")
                .contains("a.dat.before-install"),
            "{:?}",
            outcome.report.kept
        );
        Ok(())
    }

    #[test]
    fn user_data_stays_unless_someone_asks_for_it_to_go() -> Result<()> {
        let mark_user_data = |manifest: &mut PackageManifest| {
            manifest.file_inventory[1].classification = FileClassification::UserData;
        };
        let work = temp_folder()?;
        let (options, id) = installed(
            work.path(),
            &[("a.dat", b"a"), ("settings.json", b"{}")],
            mark_user_data,
        )?;
        let outcome = uninstall(&options, &id, false)?;
        assert!(outcome.finished, "{:?}", outcome.report);
        assert_eq!(names_in(&options.target_root)?, ["settings.json"]);
        assert!(outcome.report.kept.join("\n").contains("settings.json"));

        let work = temp_folder()?;
        let (options, id) = installed(
            work.path(),
            &[("a.dat", b"a"), ("settings.json", b"{}")],
            mark_user_data,
        )?;
        let outcome = uninstall(&options, &id, true)?;
        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(!options.target_root.exists());
        Ok(())
    }

    #[test]
    fn an_uninstall_that_could_not_finish_can_be_run_again() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a"), ("b.dat", b"b")], |_| {})?;
        let held = hold_exclusively(&options.target_root.join("a.dat"))?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(!outcome.finished);
        assert!(outcome.report.failed.join("\n").contains("a.dat"));
        assert!(!options.target_root.join("b.dat").exists());
        assert_eq!(install_folders(&options.state_root)?.len(), 1);

        drop(held);
        let outcome = uninstall(&options, &id, false)?;
        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(!options.target_root.exists());
        assert!(install_folders(&options.state_root)?.is_empty());
        Ok(())
    }

    #[test]
    fn a_folder_swapped_for_a_link_does_not_carry_an_uninstall_outside() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(
            work.path(),
            &[("a.dat", b"a"), (r"sub\b.dat", b"b")],
            |_| {},
        )?;
        let target = options.target_root.clone();
        // What is at the other end has the very same contents, so only the link check stops it.
        let victim = work.path().join("victim");
        write_file(&victim.join("b.dat"), b"b")?;
        fs::remove_dir_all(target.join("sub"))
            .map_err(|error| WinAppMigratorError::io(&target, error))?;
        junction(&target.join("sub"), &victim)?;

        let outcome = uninstall(&options, &id, false)?;

        assert_eq!(read_text(&victim.join("b.dat"))?, "b");
        assert!(
            outcome.report.failed.join("\n").contains("link"),
            "{:?}",
            outcome.report
        );
        assert!(target.join("sub").exists(), "the link is left as it is");
        Ok(())
    }

    #[test]
    fn an_install_location_swapped_for_a_link_does_not_carry_a_rollback_outside() -> Result<()> {
        let work = temp_folder()?;
        let interrupted = interrupted_install(work.path())?;
        let target = interrupted.target_root.clone();
        let victim = work.path().join("victim");
        write_file(&victim.join(r"sub\b.dat"), b"new b")?;
        let moved = work.path().join("moved");
        fs::rename(&target, &moved).map_err(|error| WinAppMigratorError::io(&target, error))?;
        junction(&target, &victim)?;
        let folder = install_folders(&interrupted.state_root)?
            .pop()
            .ok_or_else(|| WinAppMigratorError::InvalidInput("no journal".to_owned()))?;

        let report = undo_journal(
            &read_journal(&folder)?,
            UndoMode::Rollback,
            &InstallMonitor::default(),
            InstallPhase::RollingBack,
        );

        assert_eq!(read_text(&victim.join(r"sub\b.dat"))?, "new b");
        assert!(
            report.failed.join("\n").contains("now leads to"),
            "{report:?}"
        );
        Ok(())
    }

    #[test]
    fn a_hard_link_swapped_in_only_loses_its_name() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a")], |_| {})?;
        let installed_file = options.target_root.join("a.dat");
        let victim = work.path().join("victim.dat");
        write_file(&victim, b"a")?;
        fs::remove_file(&installed_file)
            .map_err(|error| WinAppMigratorError::io(&installed_file, error))?;
        fs::hard_link(&victim, &installed_file)
            .map_err(|error| WinAppMigratorError::io(&installed_file, error))?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(outcome.finished, "{:?}", outcome.report);
        assert!(!installed_file.exists());
        assert_eq!(read_text(&victim)?, "a");
        Ok(())
    }

    #[test]
    fn an_id_that_is_not_one_is_refused_before_anything_is_read() -> Result<()> {
        let work = temp_folder()?;
        let state = work.path().join("state");
        for id in [r"..\..\elsewhere", "wam-notthere", "", "CON"] {
            assert!(open_install(&state, id).is_err(), "{id}");
        }
        Ok(())
    }

    #[test]
    fn an_install_location_where_installs_are_recorded_is_refused() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let mut options = restore_options(work.path(), package, true);
        options.target_root = options.state_root.join(r"installs\wam-evil");
        let refused = restore_package(&options);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("keep their records")),
            "{:?}",
            refused.err()
        );
        assert!(!options.state_root.exists());
        Ok(())
    }

    #[test]
    fn nothing_is_written_where_installs_are_recorded_whatever_the_rules_said() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let options = restore_options(work.path(), package.clone(), true);
        let mut journal = journal_for_test(&options)?;
        let inside = options.state_root.join(r"installs\wam-evil\journal.jsonl");
        let refused = copy_restore_file(
            &mut journal,
            &package.join(r"files\a.dat"),
            &inside,
            &options.state_root,
            ItemKind::UserData,
            None,
        );
        assert!(refused.is_err());
        assert!(!inside.exists());
        Ok(())
    }

    #[test]
    fn another_install_running_stops_this_one_before_anything_changes() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("a.dat", b"a")], |_| {})?;
        let options = restore_options(work.path(), package, true);
        let busy = lock_state(&options.state_root)?;

        let refused = restore_package(&options);

        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("another install")),
            "{:?}",
            refused.err()
        );
        assert!(!options.target_root.exists());
        drop(busy);
        assert!(restore_package(&options)?.errors.is_empty());
        Ok(())
    }

    #[test]
    fn an_uninstall_waits_its_turn_too() -> Result<()> {
        let work = temp_folder()?;
        let (options, id) = installed(work.path(), &[("a.dat", b"a")], |_| {})?;
        let busy = lock_state(&options.state_root)?;

        let outcome = uninstall(&options, &id, false)?;

        assert!(!outcome.finished);
        assert!(outcome.report.failed.join(" ").contains("another install"));
        assert!(options.target_root.join("a.dat").is_file());
        drop(busy);
        assert!(uninstall(&options, &id, false)?.finished);
        Ok(())
    }

    /// What it replaced comes back when nobody has changed it, and otherwise stays where it was.
    #[test]
    fn undoing_an_install_long_after_it_stopped_leaves_what_someone_put_there_since() -> Result<()>
    {
        let work = temp_folder()?;
        let interrupted = interrupted_install(work.path())?;
        write_file(
            &interrupted.target_root.join(r"sub\b.dat"),
            b"the user's own",
        )?;
        write_file(&interrupted.target_root.join("a.dat"), b"the user's own")?;
        let other = work.path().join("other");
        let other_package = package_of(&other, &[("other.dat", b"other")], |_| {})?;
        let mut next = restore_options(&other, other_package, true);
        next.state_root = interrupted.state_root.clone();

        let result = restore_package(&next)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(
            read_text(&interrupted.target_root.join(r"sub\b.dat"))?,
            "the user's own"
        );
        assert_eq!(
            read_text(&interrupted.target_root.join("a.dat"))?,
            "the user's own"
        );
        let said = result
            .warnings
            .iter()
            .map(|warning| warning.message.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(said.contains("b.dat stays"), "{said}");
        Ok(())
    }

    #[test]
    fn a_finished_install_is_not_undone_because_of_a_bad_line_in_its_record() -> Result<()> {
        let work = temp_folder()?;
        let (options, _id) = installed(work.path(), &[("a.dat", b"a")], |_| {})?;
        let folder = install_folders(&options.state_root)?
            .pop()
            .ok_or_else(|| WinAppMigratorError::InvalidInput("no journal".to_owned()))?;
        let journal = folder.join("journal.jsonl");
        let text = read_text(&journal)?;
        let (first, rest) = text
            .split_once('\n')
            .ok_or_else(|| WinAppMigratorError::InvalidInput("no lines".to_owned()))?;
        write_file(
            &journal,
            format!("{first}\nnot a record\n{rest}").as_bytes(),
        )?;

        let other = work.path().join("other");
        let other_package = package_of(&other, &[("other.dat", b"other")], |_| {})?;
        let mut next = restore_options(&other, other_package, true);
        next.state_root = options.state_root.clone();
        let result = restore_package(&next)?;

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(options.target_root.join("a.dat").is_file());
        Ok(())
    }

    #[test]
    fn what_powershell_says_goes_in_without_the_script_it_quotes() {
        let said = "Exception calling \"Save\": Unable to save shortcut\r\n\
                    At line:1 char:200\r\n\
                    + ... $shortcut.Arguments = 'secret'; $shortcut.Save(); ...\r\n";
        assert_eq!(
            powershell_error_summary(said),
            "Exception calling \"Save\": Unable to save shortcut"
        );
        assert_eq!(powershell_error_summary(""), "");
    }

    #[test]
    fn the_uninstall_entry_runs_the_copy_in_the_records_folder_with_its_path_quoted() {
        let manifest = bare_manifest();
        let exe = Path::new(r"C:\ProgramData\WinAppMigrator\installs\wam-abc\uninstall.exe");
        let values =
            uninstall_entry_values(exe, "wam-abc", "Acme", &manifest, Path::new(r"C:\Acme"));
        let text = |name: &str| {
            values
                .iter()
                .find(|(value_name, _)| *value_name == name)
                .map(|(_, value)| value.to_string())
        };
        assert_eq!(
            text(UNINSTALL_STRING_VALUE).as_deref(),
            Some(
                r#""C:\ProgramData\WinAppMigrator\installs\wam-abc\uninstall.exe" --uninstall wam-abc"#
            )
        );
        assert_eq!(
            text(QUIET_UNINSTALL_STRING_VALUE).as_deref(),
            Some(
                r#""C:\ProgramData\WinAppMigrator\installs\wam-abc\uninstall.exe" --uninstall wam-abc --quiet"#
            )
        );
        assert_eq!(text(DISPLAY_NAME_VALUE).as_deref(), Some("Acme"));
    }

    #[test]
    fn apps_only_share_an_uninstall_entry_if_they_have_the_same_name() {
        assert_ne!(uninstall_key_for("日本語"), uninstall_key_for("中文語"));
        assert_ne!(uninstall_key_for("Acme 1.0"), uninstall_key_for("Acme 2.0"));
        assert_eq!(uninstall_key_for("Acme"), uninstall_key_for("Acme"));
        let odd = uninstall_key_for("Acme\\Evil\u{7}");
        assert!(odd.ends_with(r"\WinAppMigrator Acme_Evil"), "{odd}");
        let long = uninstall_key_for(&"x".repeat(1000));
        assert!(long.chars().count() < 300);
    }

    fn earlier_install_here(map: &RestoreMap) -> OnThisPc<'_> {
        OnThisPc {
            map,
            earlier_install_here: true,
        }
    }

    #[test]
    fn being_in_the_install_location_says_nothing_without_a_record_of_an_install_there()
    -> Result<()> {
        let map = RestoreMap::for_tests(r"C:\Program Files\Acme");
        let inside = r#""C:\Program Files\Acme\acme.exe" /x"#;
        let on_this_pc = |earlier_install_here| OnThisPc {
            map: &map,
            earlier_install_here,
        };
        assert!(on_this_pc(true).runs_from_an_earlier_install(inside));
        assert!(!on_this_pc(false).runs_from_an_earlier_install(inside));
        assert!(!on_this_pc(true).runs_from_an_earlier_install(r"C:\Program Files\Other\o.exe"));
        // Whatever it is written as. Most of what is in the registry is not a full path.
        for not_a_full_path in [
            "txtfile",
            "{00000000-0000-0000-0000-000000000000}",
            "arial.ttf",
            r"%SystemRoot%\system32\notepad.exe %1",
            "Someone's",
            r"bin\a.exe",
        ] {
            for earlier in [true, false] {
                assert!(
                    !on_this_pc(earlier).runs_from_an_earlier_install(not_a_full_path),
                    "{not_a_full_path}"
                );
            }
        }
        Ok(())
    }

    /// Reads only: this asks where the data folder is, and compares names.
    #[test]
    fn a_data_folder_on_the_list_says_nothing_of_a_new_folder_in_it() -> Result<()> {
        let data = PathBuf::from(
            std::env::var_os("ProgramData")
                .ok_or_else(|| WinAppMigratorError::InvalidInput("no ProgramData".to_owned()))?,
        );
        let target = Path::new(r"C:\Program Files\Acme");
        let left_alone = |destination: PathBuf, folders: &[PathBuf]| {
            program_left_alone(&destination, target, false, folders).is_some()
        };
        let other = data.join("WAM Other");
        assert_eq!(
            data_folder_of(&other.join(r"deep\down\a.dll"), target).as_ref(),
            Some(&other)
        );
        assert_eq!(
            data_folder_of(&data.join("a.dll"), target).as_ref(),
            Some(&data)
        );
        assert_eq!(data_folder_of(&target.join("a.dll"), target), None);

        let there = std::slice::from_ref(&other);
        assert!(left_alone(other.join("version.dll"), there));
        assert!(left_alone(other.join(r"deep\version.dll"), there));
        assert!(!left_alone(data.join(r"WAM Acme\helper.exe"), there));

        let the_data_folder = std::slice::from_ref(&data);
        assert!(left_alone(data.join("straight-in.exe"), the_data_folder));
        assert!(
            !left_alone(data.join(r"WAM Acme\helper.exe"), the_data_folder),
            "a new folder there has no program in it to load this"
        );
        Ok(())
    }

    #[test]
    fn a_verb_on_a_folders_background_is_a_verb() {
        let directory = r"software\classes\directory";
        assert_eq!(
            verb_prefix_of(
                r"software\classes\directory\background\shell\acme\command",
                directory
            )
            .as_deref(),
            Some(r"software\classes\directory\background\shell\")
        );
        assert_eq!(
            verb_prefix_of(r"software\classes\directory\shell\acme", directory).as_deref(),
            Some(r"software\classes\directory\shell\")
        );
        assert_eq!(
            verb_prefix_of(r"software\classes\directory\defaulticon", directory),
            None
        );
    }

    /// Reads only: the class is Windows' own, and every PC has it.
    #[test]
    fn all_of_a_class_that_belongs_to_something_else_is_left_alone() -> Result<()> {
        let map = app_map()?;
        let refused_key = format!(r"{SHELL_LINK_CLASS}\InprocServer32");
        let refused = com_entry(
            RegistryHive::LocalMachine,
            refused_key.clone(),
            RegistryValueKind::String,
        );
        let own = com_entry(
            RegistryHive::LocalMachine,
            r"SOFTWARE\Classes\CLSID\{5A7E6E0C-0000-4000-8000-000000000000}\InprocServer32"
                .to_owned(),
            RegistryValueKind::String,
        );
        let class = SHELL_LINK_CLASS.to_ascii_lowercase();
        let classes = |entries: &[&RegistryEntry]| -> Vec<String> {
            com_classes_to_leave_alone(entries, earlier_install_here(&map))
                .into_iter()
                .map(|left_alone| left_alone.0)
                .collect()
        };
        assert_eq!(classes(&[&refused, &own]), std::slice::from_ref(&class));
        // In any order, and even with no server in the package at all.
        let mut sibling = refused.clone();
        sibling.value_name = Some("ThreadingModel".to_owned());
        assert_eq!(classes(&[&sibling, &refused]), std::slice::from_ref(&class));
        assert_eq!(classes(&[&sibling]), std::slice::from_ref(&class));
        assert!(classes(&[&own]).is_empty());

        // The 32-bit class of that name is a class of its own, and is looked at where it is.
        let thirty_two_bit = SHELL_LINK_CLASS.replace(r"SOFTWARE\", r"SOFTWARE\WOW6432Node\");
        let mut sibling_32 = com_entry(
            RegistryHive::LocalMachine,
            format!(r"{thirty_two_bit}\InprocServer32"),
            RegistryValueKind::String,
        );
        sibling_32.value_name = Some("ThreadingModel".to_owned());
        assert_eq!(
            classes(&[&sibling_32]),
            [thirty_two_bit.to_ascii_lowercase()]
        );
        Ok(())
    }

    /// Reads only. What this PC has for a class is what the package has, as it would be when
    /// installing an app again.
    #[test]
    fn a_class_the_package_has_all_of_is_not_taken_for_someone_elses() -> Result<()> {
        let map = app_map()?;
        let key = format!(r"{SHELL_LINK_CLASS}\InprocServer32");
        let existing: String = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
            .open_subkey(&key)
            .and_then(|key| key.get_value(""))
            .map_err(|error| WinAppMigratorError::io(Path::new(&key), error))?;
        let threading: String = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
            .open_subkey(&key)
            .and_then(|key| key.get_value("ThreadingModel"))
            .map_err(|error| WinAppMigratorError::io(Path::new(&key), error))?;
        let mut server = com_entry(RegistryHive::LocalMachine, key, RegistryValueKind::String);
        server.value_data = Some(existing);
        let mut sibling = server.clone();
        sibling.value_name = Some("ThreadingModel".to_owned());
        sibling.value_data = Some(threading);
        assert!(
            com_classes_to_leave_alone(&[&sibling, &server], earlier_install_here(&map)).is_empty()
        );

        // Its server copied from this PC does not let in what is beside it.
        let mut beside_it = server.clone();
        beside_it.key_path = beside_it.key_path.replace("InprocServer32", "TreatAs");
        beside_it.value_data = Some("{5A7E6E0C-0000-4000-8000-000000000000}".to_owned());
        assert_eq!(
            com_classes_to_leave_alone(&[&server, &beside_it], earlier_install_here(&map)).len(),
            1
        );
        // A key said to have nothing in it does not say what its server is, whatever text it has.
        let mut says_nothing = server.clone();
        says_nothing.value_kind = Some(RegistryValueKind::None);
        says_nothing.key_path = says_nothing
            .key_path
            .replace("InprocServer32", "LocalServer32");
        assert_eq!(
            com_classes_to_leave_alone(&[&says_nothing], earlier_install_here(&map)).len(),
            1
        );
        Ok(())
    }

    /// A new location is empty, and a hold does not stop that being turned into a link. Whatever
    /// comes next must not go through it.
    #[test]
    fn an_install_location_turned_into_a_link_after_it_was_checked_is_not_written_through()
    -> Result<()> {
        use crate::privilege::tests::try_to_turn_into_a_link;

        let work = temp_folder()?;
        let options = restore_options(work.path(), work.path().join("no package"), true);
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let source = work.path().join("source.dat");
        write_file(&source, b"content")?;
        let hash = sha256_file(&source)?;

        let mut journal = journal_for_test(&options)?;
        journal.note_new_folders_to(&options.target_root, ItemKind::Program)?;
        let hold = ensure_target_root(&options.target_root)?;
        let first = options.target_root.join("first.dat");
        copy_restore_file(
            &mut journal,
            &source,
            &first,
            &options.target_root,
            ItemKind::Program,
            Some(&hash),
        )?;
        fs::remove_file(&first).map_err(|error| WinAppMigratorError::io(&first, error))?;
        let made = try_to_turn_into_a_link(&options.target_root, &elsewhere)
            .map_err(|error| WinAppMigratorError::io(&options.target_root, error))?;
        assert_eq!(made, "0");

        for relative in ["at_the_top.dat", r"in\a\folder.dat"] {
            let refused = copy_restore_file(
                &mut journal,
                &source,
                &options.target_root.join(relative),
                &options.target_root,
                ItemKind::Program,
                Some(&hash),
            );
            assert!(
                matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("link")),
                "{relative}: {refused:?}"
            );
        }
        assert!(names_in(&elsewhere)?.is_empty(), "nothing went through it");
        drop(hold);
        Ok(())
    }

    #[test]
    fn a_data_folder_under_a_link_is_refused_up_front_and_says_what_can_be_done() -> Result<()> {
        let work = temp_folder()?;
        let target_root = work.path().join("target");
        let real = work.path().join(r"real\data");
        fs::create_dir_all(&real).map_err(|error| WinAppMigratorError::io(&real, error))?;
        let link = work.path().join("link");
        junction(&link, &work.path().join("real"))?;

        assert!(refuse_roots_under_links(std::slice::from_ref(&real), &target_root).is_ok());
        let refused = refuse_roots_under_links(&[real, link.join("data")], &target_root);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("is a link") && message.contains("without its user data")),
            "{refused:?}"
        );
        // The install location has a rule of its own, and a way out of its own.
        assert!(refuse_roots_under_links(&[link.join("data")], &link.join("data")).is_ok());
        Ok(())
    }

    /// Not worth undoing an install for, so it must not read as the record failing.
    #[test]
    fn a_shortcut_folder_under_a_link_is_a_warning_and_nothing_is_recorded() -> Result<()> {
        let work = temp_folder()?;
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let link = work.path().join("link");
        junction(&link, &elsewhere)?;
        let options = restore_options(work.path(), work.path().join("no package"), true);
        let mut journal = journal_for_test(&options)?;
        let shortcut = ReviewedShortcut {
            original_lnk_path: work.path().join("Acme.lnk"),
            placement: crate::restore_policy::ShortcutPlacement {
                destination: link.join("Acme.lnk"),
                root: link.clone(),
                for_all_users: false,
                autostart: false,
            },
            target: work.path().join("acme.exe"),
            arguments: None,
            working_directory: None,
            icon: None,
        };
        let refused = apply_shortcut(&mut journal, &shortcut);
        assert_eq!(
            refused.err().map(|warning| warning.code).as_deref(),
            Some("shortcut_destination_invalid")
        );
        assert!(names_in(&elsewhere)?.is_empty());
        assert!(journal.read_back()?.changes.is_empty());
        Ok(())
    }

    #[test]
    fn a_location_under_a_link_is_refused_and_says_where_to_go() -> Result<()> {
        let work = temp_folder()?;
        let package = package_of(work.path(), &[("acme.exe", b"program")], |_| {})?;
        let mut options = restore_options(work.path(), package, true);
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let link = work.path().join("link");
        junction(&link, &elsewhere)?;
        options.target_root = link.join("Acme");

        let refused = restore_package(&options);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message)) if message.contains("is a link")),
            "{refused:?}"
        );
        assert!(names_in(&elsewhere)?.is_empty());
        Ok(())
    }

    /// An install that never finished, with part of it on a drive that has gone, then another
    /// that wrote the same thing. When the first is looked at again, it must leave that alone.
    #[test]
    fn what_was_undone_while_waiting_for_a_drive_is_not_undone_again() -> Result<()> {
        let Some(drive) = ('D'..='Z')
            .map(|letter| format!(r"{letter}:\"))
            .find(|drive| !Path::new(drive).exists())
        else {
            return Ok(());
        };
        let gone = Path::new(&drive).join("Apps");
        let work = temp_folder()?;
        let options = restore_options(work.path(), work.path().join("no package"), true);
        let source = work.path().join("source.dat");
        write_file(&source, b"the same in both")?;
        let hash = sha256_file(&source)?;
        let written = options.target_root.join("a.dat");

        let mut journal = journal_for_test(&options)?;
        journal.note_new_folders_to(&options.target_root, ItemKind::Program)?;
        copy_restore_file(
            &mut journal,
            &source,
            &written,
            &options.target_root,
            ItemKind::Program,
            Some(&hash),
        )?;
        journal.note(Change::Root {
            path: gone.clone(),
            real: gone.display().to_string(),
        })?;
        journal.note(Change::File {
            path: gone.join("b.dat"),
            root: gone.clone(),
            item: ItemKind::Program,
            backup: None,
            sha256: Some(hash.clone()),
            partial: None,
        })?;
        let folder = journal.folder().to_path_buf();
        drop(journal);

        let first = recover_interrupted_installs(&options.state_root, &InstallMonitor::default())?;
        assert!(first.recovered[0].report.waits_for_a_drive());
        assert!(!written.exists(), "what could be reached was undone");
        assert!(folder.exists(), "and the record stays for the rest");

        // The next install writes the same thing.
        write_file(&written, b"the same in both")?;
        let second = recover_interrupted_installs(&options.state_root, &InstallMonitor::default())?;
        assert!(second.recovered[0].report.waits_for_a_drive());
        assert!(written.exists(), "it is not that install's to undo");
        assert!(second.recovered[0].report.undone.is_empty());
        Ok(())
    }

    #[test]
    fn a_shortcut_cut_off_before_its_hash_was_recorded_is_still_undone() -> Result<()> {
        let work = temp_folder()?;
        let root = work.path().join("links");
        let link = root.join("app.lnk");
        write_file(&link, b"from before")?;
        let options = restore_options(work.path(), work.path().join("no package"), true);
        let mut journal = journal_for_test(&options)?;
        let backup = journal.note_file(&link, &root, ItemKind::Program, true, None, None)?;
        let number =
            backup.ok_or_else(|| WinAppMigratorError::InvalidInput("no backup".to_owned()))?;
        journal.back_up(&link, number)?;
        write_file(&link, b"made, and the record of it never written")?;
        let folder = journal.folder().to_path_buf();
        drop(journal);

        let report = undo_journal(
            &read_journal(&folder)?,
            UndoMode::Recovery,
            &InstallMonitor::default(),
            InstallPhase::RollingBack,
        );

        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert_eq!(read_text(&link)?, "from before");
        Ok(())
    }

    #[test]
    fn a_tasks_folders_are_named_outermost_first() {
        assert_eq!(task_folder_levels(r"\"), Vec::<String>::new());
        assert_eq!(task_folder_levels(r"\Acme\"), [r"\Acme"]);
        assert_eq!(
            task_folder_levels(r"\Acme\Sync\"),
            [r"\Acme", r"\Acme\Sync"]
        );
    }

    #[test]
    fn a_delayed_start_reaches_sc_as_what_it_asks_for() {
        let mut service = sample_service();
        service.start_type = Some(DELAYED_AUTOMATIC_START_LABEL.to_owned());
        let arguments = sc_service_arguments(
            "create",
            &service,
            Path::new(r"D:\App\svc.exe"),
            None,
            &ServiceAccount::Unspecified,
        );
        let arguments = strings(&arguments);
        let start = arguments.iter().position(|argument| argument == "start=");
        assert_eq!(
            start
                .and_then(|index| arguments.get(index + 1))
                .map(String::as_str),
            Some("delayed-auto")
        );
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
