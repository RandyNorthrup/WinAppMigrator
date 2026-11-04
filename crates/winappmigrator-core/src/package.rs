use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use filetime::{FileTime, set_file_times};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::account_traces::{
    say_nothing_of_who_it_was, what_a_package_still_says, withhold_what_is_an_accounts,
};
use crate::archive::{
    EmbeddedPackage, HELD_FILES, PendingInstaller, extract_zip_archive, package_folder_items,
    zip_name_of,
};
use crate::authenticode::check_microsoft_signature;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    MAX_EMPTY_FOLDERS, has_reserved_device_name, is_user_profile_path, package_destination_path,
    safe_relative_path,
};
use crate::footprint::{Footprint, FootprintChoices, ScanFlags, apply_choices, recount_locations};
use crate::hashing::{hex_lower, sha256_file};
use crate::models::{
    AppArchitecture, AppSelection, DriverEntry, FileClassification, FileEntry, FootprintKind,
    InstallerStub, LeftOutItem, LicenseArtifact, LicenseStoreFile, PACKAGE_SCHEMA_VERSION,
    PackageManifest, PackageOptions, PackageResult, RegistryEntry, RegistryHive, RegistryValueKind,
    RuntimePrerequisite, ScanOptions, ScanReport, ScheduledTaskEntry, Warning, WarningSeverity,
};
use crate::packer::{PackItem, PackOutcome, PackedFile, Packer, SourceRules};
use crate::path_utils::{
    DEEPEST_WALK, comparable_path, hidden_command, is_on_a_local_disk, path_is_within,
    sanitize_filename, walk_asking,
};
use crate::prerequisites::PREREQUISITES_DIR;
use crate::privilege::ProtectedFolder;
use crate::progress::{BuildMonitor, BuildStep};
use crate::registry_target::hex_decode;
use crate::reporting::write_scan_reports;
use crate::scan::scan_app;
use crate::space::{BuildInputs, RoomCheck, build_needs, build_places};
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
/// Where a file only a backup-mode copy can read goes while it is read, in the protected folder.
const SPOOL_DIR: &str = ".spool";
const COPY_BUFFER_BYTES: usize = 1024 * 1024;
/// A file a package does not have, for one reason or the other.
const FILE_COPY_FAILED_CODE: &str = "file_copy_failed";
const FILE_GONE_CODE: &str = "file_gone_since_it_was_looked_at";
/// How often to look at whether a backup-mode copy has finished, or the build was cancelled.
const BACKUP_COPY_FIRST_LOOK: std::time::Duration = std::time::Duration::from_millis(1);
const BACKUP_COPY_LONGEST_LOOK: std::time::Duration = std::time::Duration::from_millis(50);

pub fn create_package(options: &PackageOptions) -> Result<PackageResult> {
    create_package_monitored(options, &BuildMonitor::default())
}

/// What a package scan has to be asked, for what goes in it to be what a build goes on to read.
pub fn scan_options_for(options: &PackageOptions) -> ScanOptions {
    ScanOptions {
        selection: options.selection.clone(),
        output_dir: options.output_dir.join(REPORTS_DIR),
        current_directory: options.current_directory.clone(),
        recursive_dependencies: true,
        // What goes in the package is hashed as it is read below. A hash of the original
        // could not be checked anyway when reading it needed a backup-mode copy, and it
        // would be wrong for a file that changes in between.
        include_hashes: false,
        include_user_data: options.include_user_data,
        include_program_data: options.include_program_data,
        include_license_data: options.include_license_data,
        source: options.source.clone(),
        user_profile: options.user_profile.clone(),
        near_matches: Vec::new(),
    }
}

/// [`create_package`], reporting progress to `monitor` and stopping with
/// [`WinAppMigratorError::Cancelled`] if it is cancelled. It scans first, and takes all of what
/// that finds. [`create_package_from_scan`] is for a person who has seen it and chosen.
pub fn create_package_monitored(
    options: &PackageOptions,
    monitor: &BuildMonitor,
) -> Result<PackageResult> {
    create_package_dirs(&options.output_dir)?;
    monitor.report(BuildStep::Scanning);
    let report = scan_app(&scan_options_for(options), &monitor.sharing_cancel())?;
    monitor.checkpoint()?;
    build_package(options, &report, &WhatWasChosen::default(), monitor)
}

/// Build a package from a scan made earlier, with what a person chose applied. Nothing is scanned
/// again: what goes in is what the scan found less what they left out, each file is hashed from what
/// is read of it now, and what has gone since is said to have. The scan has to be of what `options`
/// selects, so a build cannot go by another app's by accident.
pub fn create_package_from_scan(
    options: &PackageOptions,
    report: &ScanReport,
    choices: &FootprintChoices,
    monitor: &BuildMonitor,
) -> Result<PackageResult> {
    monitor.report(BuildStep::Preparing);
    if !is_scan_of(report, &options.selection) {
        return Err(WinAppMigratorError::InvalidInput(
            "this scan is of another app than the one being built, so nothing was built".to_owned(),
        ));
    }
    let is_off = |classification: &FileClassification| match classification {
        FileClassification::UserData => !options.include_user_data,
        FileClassification::MachineAppData => !options.include_program_data,
        _ => false,
    };
    // What a build goes on to do with a scan goes by what the scan found, so it has to be what this
    // build was asked for. The window looks again whenever an option changes.
    let brought_in_what_is_off = report
        .file_inventory
        .iter()
        .chain(&report.empty_folders)
        .any(|file| file.should_copy && is_off(&file.classification))
        || (!options.include_user_data && report.user_data_profile.is_some())
        || (!options.include_license_data
            && (!report.license_registry_entries.is_empty()
                || report
                    .license_artifacts
                    .iter()
                    .any(|artifact| artifact.should_copy)));
    if brought_in_what_is_off {
        return Err(WinAppMigratorError::InvalidInput(
            "this scan brought in data that this build was not asked to, so nothing was built"
                .to_owned(),
        ));
    }
    refuse_a_scan_whose_program_is_gone(report)?;
    let adds_something = report.near_matches.iter().any(|near| near.picked);
    if choices.is_empty() && !adds_something {
        create_package_dirs(&options.output_dir)?;
        return build_package(options, report, &WhatWasChosen::default(), monitor);
    }
    let flags = ScanFlags {
        include_user_data: options.include_user_data,
        include_program_data: options.include_program_data,
        include_license_data: options.include_license_data,
    };
    let footprint = Footprint::of(report, flags)?;
    let chosen = apply_choices(report, &footprint, choices)?;
    monitor.checkpoint()?;
    let had_data = report
        .file_inventory
        .iter()
        .chain(&report.empty_folders)
        .any(|file| file.should_copy && is_data(&file.classification));
    // A package with no data left has nothing to ask about it, or to be named for.
    let keeps_data = chosen.has_user_data || chosen.has_program_data || !had_data;
    let adjusted = PackageOptions {
        include_user_data: options.include_user_data && keeps_data,
        include_program_data: options.include_program_data && keeps_data,
        ..options.clone()
    };
    create_package_dirs(&adjusted.output_dir)?;
    build_package(
        &adjusted,
        &chosen.report,
        &WhatWasChosen {
            left_out: chosen.left_out,
            added: chosen.added,
        },
        monitor,
    )
}

/// What a person chose, as a package says it to whoever installs it.
#[derive(Default)]
struct WhatWasChosen {
    left_out: Vec<LeftOutItem>,
    added: Vec<String>,
}

fn is_scan_of(report: &ScanReport, selection: &AppSelection) -> bool {
    match (selection, report.source_app.as_ref()) {
        (AppSelection::InstalledApp(selected), Some(scanned)) => {
            selected
                .registry_source_path
                .eq_ignore_ascii_case(&scanned.registry_source_path)
                && selected.display_name == scanned.display_name
                && selected.owner_profile == scanned.owner_profile
        }
        (AppSelection::ExecutablePath(path), None) => {
            path.as_path() == report.main_executable.path.as_path()
        }
        _ => false,
    }
}

/// An app can be uninstalled, or its drive taken away, while a scan sits on a screen. A package of
/// whatever is left would not run, so that is said before anything is read.
fn refuse_a_scan_whose_program_is_gone(report: &ScanReport) -> Result<()> {
    let main = report.main_executable.path.as_path();
    let readable = report
        .file_inventory
        .iter()
        .find(|file| file.original_path.as_path() == main)
        .map_or(main, |file| file.path.as_path());
    let reader = SourceReader::new(report);
    if reader.check(readable).is_ok() && fs::metadata(readable).is_ok() {
        return Ok(());
    }
    Err(WinAppMigratorError::InvalidInput(format!(
        "{} is not there any more, so nothing was built. Look at the app again.",
        main.display()
    )))
}

fn is_data(classification: &FileClassification) -> bool {
    matches!(
        classification,
        FileClassification::UserData | FileClassification::MachineAppData
    )
}

/// What a package says of a scan. A scan looks at an app's data whichever way the options are, so that
/// the screen can say what there is to bring. What that data is called, and whose folders it is in, is
/// no business of another PC's when the option for it is off. And a scan writes nowhere: where it would
/// have is nothing to go by when a build writes, or to pass on.
fn what_a_package_says_of(report: &ScanReport, options: &PackageOptions) -> ScanReport {
    let mut said = report.clone();
    said.generated_artifacts = None;
    // How it was removed where it was, which can be a setup in someone's Downloads. Nothing on another
    // PC reads this, as an install gives the app's entry an uninstaller of its own.
    if let Some(app) = said.source_app.as_mut() {
        app.uninstall_string = None;
    }
    // A person can leave all of the data out, of a scan that was asked for it. An app of one account's
    // is in that account's folders, and what is said of its own files still has to be.
    if !options.include_user_data {
        withhold_what_is_an_accounts(&mut said, options.include_license_data);
        said.user_data_profile = None;
        said.source_computer_name = None;
        let app_folder = report.install_location.as_deref();
        said.secrets_left_out.retain(|item| {
            let path = Path::new(&item.name);
            !is_user_profile_path(path)
                || app_folder.is_some_and(|folder| path_is_within(path, folder))
        });
    }
    let off = |classification: &FileClassification| match classification {
        FileClassification::UserData => !options.include_user_data,
        FileClassification::MachineAppData => !options.include_program_data,
        _ => false,
    };
    let is_staying = |file: &FileEntry| !file.should_copy && off(&file.classification);
    let files_staying: HashSet<String> = said
        .file_inventory
        .iter()
        .filter(|file| is_staying(file))
        .flat_map(|file| [&file.path, &file.original_path])
        .map(|path| comparable_path(path))
        .collect();
    said.file_inventory.retain(|file| !is_staying(file));
    let is_going = |path: &Path| {
        said.file_inventory
            .iter()
            .any(|file| file.should_copy && path_is_within(&file.original_path, path))
    };
    let folders_staying: Vec<PathBuf> = said
        .footprint_locations
        .iter()
        .filter(|location| {
            let is_off = match location.kind {
                FootprintKind::RoamingAppData
                | FootprintKind::LocalAppData
                | FootprintKind::LocalLowAppData
                | FootprintKind::Documents
                | FootprintKind::SavedGames => !options.include_user_data,
                FootprintKind::ProgramData | FootprintKind::PublicDocuments => {
                    !options.include_program_data
                }
                FootprintKind::InstallDirectory
                | FootprintKind::RegistryReference
                | FootprintKind::Other => {
                    !options.include_user_data && is_user_profile_path(&location.path)
                }
            };
            is_off && !location.included && !is_going(&location.path)
        })
        .map(|location| location.path.clone())
        .collect();
    said.footprint_locations
        .retain(|location| !folders_staying.contains(&location.path));
    let is_going_file: HashSet<String> = said
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .flat_map(|file| [&file.path, &file.original_path])
        .map(|path| comparable_path(path))
        .collect();
    said.warnings.retain(|warning| {
        warning.source.as_deref().is_none_or(|source| {
            let source = Path::new(source);
            let comparable = comparable_path(source);
            // A warning can be about a place on a connected drive, by where the drive is now, and about
            // what was never going to be listed. Anything in an account's folders that is not going in
            // is that account's to know about, and not whoever runs this.
            let in_an_account = !options.include_user_data
                && is_user_profile_path(source)
                && !is_going_file.contains(&comparable);
            !in_an_account
                && !files_staying.contains(&comparable)
                && !folders_staying
                    .iter()
                    .any(|folder| path_is_within(source, folder))
        })
    });
    said
}

/// The package itself, from what a scan said. With an installer to build, an app's files go straight
/// from where they are into the package: each is read once, and hashed and compressed from what that
/// read returned. Only what has to be looked at or kept as a file is put in the staging folder first.
/// Without one, the same files are copied to the staging folder, hashed as they go, and that folder
/// is the package.
fn build_package(
    options: &PackageOptions,
    report: &ScanReport,
    what_was_chosen: &WhatWasChosen,
    monitor: &BuildMonitor,
) -> Result<PackageResult> {
    let what_it_says = what_a_package_says_of(report, options);
    let report = &what_it_says;
    let reports_dir = options.output_dir.join(REPORTS_DIR);
    let mut warnings = report.warnings.clone();

    let stub = if options.create_self_extracting_archive {
        Some(options.self_extracting_stub.as_ref().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "a self-extracting archive requires an installer stub".to_owned(),
            )
        })?)
    } else {
        None
    };
    let installer_path = stub.map(|_| installer_output_path(options));
    // The scan looked before anything was read, and this looks again just before, as anything
    // read from here on goes in the package.
    let reader = SourceReader::new(report);
    let inputs = build_inputs(options, report, stub, &reader)?;
    let needs = build_needs(&inputs);
    let room = RoomCheck::on_this_pc(&build_places(
        &options.output_dir,
        installer_path.as_deref(),
        &inputs,
    ));
    monitor.report(BuildStep::RoomChecked(room.clone()));
    refuse_without_room(&room)?;
    for note in room.could_not_tell() {
        warnings.push(Warning::new(
            "free_space_unknown",
            WarningSeverity::Warning,
            note,
            None,
        ));
    }

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
    monitor.checkpoint()?;

    let (items, not_packable, secrets_since) =
        files_to_pack(options, report, &reader, &mut warnings)?;
    let spool = options.output_dir.join(SPOOL_DIR);
    let mut building = None;
    let outcomes = match (stub, installer_path.as_deref()) {
        (Some(stub), Some(installer_path)) => {
            let (pending, file, base) =
                PendingInstaller::start(stub, installer_path, needs.surely_one_file)?;
            let mut packer = Packer::new(file, base, monitor.clone());
            packer.expect(&items);
            let check_before = |path: &Path| reader.check(path);
            let check_after = |path: &Path| reader.check_after_copying(path);
            let rules = SourceRules {
                before: &check_before,
                after: &check_after,
                connected_drive: report.source_volume.is_some(),
                spool: Some(&spool),
            };
            let outcomes = packer.pack(&items, &rules)?;
            building = Some(BuildingInstaller { packer, pending });
            outcomes
        }
        _ => copy_to_package_folder(options, &items, &reader, monitor)?,
    };
    let _ = fs::remove_dir_all(&spool);

    let mut files_copied = 0_usize;
    let mut failed_copies = not_packable;
    let mut packed: HashMap<&Path, &PackedFile> = HashMap::new();
    let copyable: Vec<&FileEntry> = report
        .file_inventory
        .iter()
        .filter(|entry| {
            entry.should_copy
                && entry.package_relative_path.is_some()
                && !failed_copies.contains(&entry.original_path)
        })
        .collect();
    for (file, outcome) in copyable.iter().zip(&outcomes) {
        match outcome {
            Ok(done) => {
                packed.insert(file.original_path.as_path(), done);
                files_copied += 1;
            }
            Err(error) => {
                // Nothing goes into a package without a hash to check it by. A scan can be some
                // time ago, so it says when a file is simply not there any more.
                let is_gone =
                    reader.check(&file.path).is_ok() && fs::symlink_metadata(&file.path).is_err();
                warnings.push(if is_gone {
                    Warning::new(
                        FILE_GONE_CODE,
                        WarningSeverity::Warning,
                        "This was there when the app was looked at and is gone now, so it is not \
                         in the installer.",
                        Some(file.original_path.display().to_string()),
                    )
                } else {
                    Warning::new(
                        FILE_COPY_FAILED_CODE,
                        WarningSeverity::Warning,
                        format!("Could not copy file into package: {error}"),
                        Some(file.original_path.display().to_string()),
                    )
                });
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
            if let Some(done) = packed.get(entry.original_path.as_path()) {
                entry.hash_sha256 = Some(done.sha256.clone());
                entry.size_bytes = Some(done.size);
            }
            // Where it could be read during this build means nothing to an installer.
            entry.path = entry.original_path.clone();
            entry
        })
        .collect();

    let mut manifest = manifest_of(
        report,
        ScanFlags {
            include_user_data: options.include_user_data,
            include_program_data: options.include_program_data,
            include_license_data: options.include_license_data,
        },
        packaged_inventory,
        runtime_prerequisites,
        license_artifacts_detected,
        drivers_detected,
    );
    manifest.scan_timestamp_utc = OffsetDateTime::now_utc().format(&Rfc3339)?;
    keep_what_an_installer_will_make(&mut manifest.empty_folders, &mut warnings);
    manifest.warnings.clone_from(&warnings);
    manifest.secrets_left_out.extend(secrets_since);
    manifest
        .left_out_by_choice
        .clone_from(&what_was_chosen.left_out);
    manifest.added_by_choice.clone_from(&what_was_chosen.added);
    // Who it was goes from all that is only said, and is looked for in all that was written.
    let traces = (!options.include_user_data).then(|| report.account_traces.with_this_pc());
    if let Some(traces) = traces.as_ref() {
        say_nothing_of_who_it_was(&mut manifest, traces);
        warnings.clone_from(&manifest.warnings);
    }

    let manifest_path = options.output_dir.join(MANIFEST_FILE);
    write_json(&manifest_path, &manifest)?;
    write_scan_reports(&what_was_built(report, &manifest), &reports_dir)?;
    write_integration_artifacts(&options.output_dir, &manifest)?;
    write_restore_plan(&options.output_dir.join(RESTORE_PLAN_FILE), &manifest)?;
    if let Some(traces) = traces.as_ref() {
        let (written, not_read) = written_about_the_package(&options.output_dir, &reports_dir);
        if !not_read.is_empty() {
            warnings.push(Warning::new(
                crate::account_traces::MENTIONS_CODE,
                WarningSeverity::Warning,
                format!(
                    "This package has {} that could not be looked through for who it came from: {}. \
                     What it says may say that.",
                    if not_read.len() == 1 { "a file" } else { "files" },
                    not_read.join(", ")
                ),
                None,
            ));
        }
        warnings.extend(what_a_package_still_says(
            &manifest,
            &written,
            traces,
            &|done, total| {
                monitor.report(BuildStep::LookingOverWhatItSays { done, total });
                monitor.checkpoint()
            },
        )?);
    }

    let mut self_extracting_archive_path = None;
    let mut data_file_path = None;
    if let (Some(building), Some(stub)) = (building, stub) {
        // In this order, so that if anything goes wrong below, the file is closed before what clears
        // it up goes. The last one named goes first.
        let BuildingInstaller {
            pending,
            mut packer,
        } = building;
        monitor.checkpoint()?;
        // What is in the staging folder goes in after what it describes, which is why the manifest
        // can say what those files came out as. Anything that does not hash as it did when it was
        // put there stops the build.
        let mut held = package_folder_items(&options.output_dir, &held_hashes(&manifest))?;
        // Whatever could not be removed of it, none of it is the package's.
        held.retain(|item| !item.name.starts_with(SPOOL_DIR));
        packer.expect(&held);
        for outcome in packer.pack(&held, &HELD_FILES)? {
            outcome.map_err(|reason| {
                WinAppMigratorError::Archive(format!(
                    "a file in the staging folder could not go in the package: {reason}"
                ))
            })?;
        }
        monitor.checkpoint()?;
        monitor.report(BuildStep::WritingInstaller);
        let finished = packer.finish()?;
        let written = pending.complete(stub, finished, monitor)?;
        self_extracting_archive_path = Some(written.installer);
        data_file_path = written.data_file;
    }

    Ok(PackageResult {
        package_root: options.output_dir.clone(),
        manifest_path,
        reports_dir,
        files_copied,
        warnings,
        self_extracting_archive_path,
        data_file_path,
    })
}

/// All a build writes of its own about a package, by name, for looking through.
fn written_about_the_package(
    root: &Path,
    reports_dir: &Path,
) -> (Vec<(String, Vec<u8>)>, Vec<String>) {
    let mut written = Vec::new();
    let mut folders: Vec<PathBuf> = [
        REGISTRY_DIR,
        SERVICES_DIR,
        DRIVERS_DIR,
        SCHEDULED_TASKS_DIR,
        SHORTCUTS_DIR,
        ENVIRONMENT_DIR,
    ]
    .iter()
    .map(|folder| root.join(folder))
    .collect();
    folders.push(reports_dir.to_path_buf());
    let mut not_read = Vec::new();
    let called = |path: &Path| {
        path.strip_prefix(root)
            .unwrap_or(path)
            .display()
            .to_string()
    };
    let mut files: Vec<PathBuf> = Vec::new();
    for folder in &folders {
        match fs::read_dir(folder) {
            Ok(entries) => files.extend(
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file()),
            ),
            // Most of these are only made if there is something to put in them.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => not_read.push(called(folder)),
        }
    }
    files.push(root.join(RESTORE_PLAN_FILE));
    for file in files {
        match fs::read(&file) {
            Ok(bytes) => written.push((called(&file), bytes)),
            Err(_) => not_read.push(called(&file)),
        }
    }
    (written, not_read)
}

/// As many empty folders as an installer will make, of what was left once a person had chosen, which
/// is why this waits for a build. An installer holds to this whatever a package asks of it.
fn keep_what_an_installer_will_make(folders: &mut Vec<FileEntry>, warnings: &mut Vec<Warning>) {
    if folders.len() <= MAX_EMPTY_FOLDERS {
        return;
    }
    folders.truncate(MAX_EMPTY_FOLDERS);
    warnings.push(Warning::new(
        "empty_folders_truncated",
        WarningSeverity::Warning,
        format!(
            "This app has more than {MAX_EMPTY_FOLDERS} empty folders, and only that many will be \
             made again."
        ),
        None,
    ));
}

/// What a scan said, as it came out. A report goes in the package with the manifest, and has to say
/// the same: a file that could not be read is not going, and what the build found to say is said too.
fn what_was_built(report: &ScanReport, manifest: &PackageManifest) -> ScanReport {
    let mut built = report.clone();
    built.file_inventory.clone_from(&manifest.file_inventory);
    built.empty_folders.clone_from(&manifest.empty_folders);
    built.warnings.clone_from(&manifest.warnings);
    built
        .secrets_left_out
        .clone_from(&manifest.secrets_left_out);
    built
        .license_artifacts
        .clone_from(&manifest.license_artifacts_detected);
    built.drivers.clone_from(&manifest.drivers_detected);
    built
        .runtime_prerequisites
        .clone_from(&manifest.runtime_prerequisites);
    built
        .licenses_left_out
        .clone_from(&manifest.licenses_left_out);
    built
        .settings_left_out
        .clone_from(&manifest.settings_left_out);
    built
        .items_excluded
        .clone_from(&manifest.items_intentionally_excluded);
    if !manifest.included_user_data {
        // Where a drive is on the PC building this is no one else's business, nor who else is on it.
        built.source_volume = None;
        built.other_accounts = OtherAccountsFolders::default();
        let traces = report.account_traces.with_this_pc();
        for evidence in &mut built.install_evidence {
            *evidence = traces.scrub(evidence);
        }
    }
    // The scan worked out how much of each place it hoped to copy. This is how much of it went.
    recount_locations(&mut built);
    built
}

/// A manifest of what a scan said, with the parts a build works out for itself given.
fn manifest_of(
    report: &ScanReport,
    flags: ScanFlags,
    file_inventory: Vec<FileEntry>,
    runtime_prerequisites: Vec<RuntimePrerequisite>,
    license_artifacts_detected: Vec<LicenseArtifact>,
    drivers_detected: Vec<DriverEntry>,
) -> PackageManifest {
    PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
        source_computer_name: report.source_computer_name.clone(),
        source_windows_version: report.source_windows_version.clone(),
        source_platform: report.source_platform.clone(),
        program_needs: report.program_needs.clone(),
        scan_timestamp_utc: String::new(),
        main_executable_path: report.main_executable.path.clone(),
        application_metadata: report.source_app.clone(),
        file_inventory,
        empty_folders: report
            .empty_folders
            .iter()
            .cloned()
            .map(|mut folder| {
                // Where it could be read during this build means nothing to an installer.
                folder.path = folder.original_path.clone();
                folder
            })
            .collect(),
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
        settings_left_out: report.settings_left_out.clone(),
        firewall_rules_detected: report.firewall_rules.clone(),
        certificates_reported: report.certificates.clone(),
        fonts_reported: report.fonts_reported.clone(),
        left_out_by_choice: Vec::new(),
        added_by_choice: Vec::new(),
        warnings: Vec::new(),
        items_intentionally_excluded: report.items_excluded.clone(),
        included_user_data: flags.include_user_data,
        included_program_data: flags.include_program_data,
        included_license_data: flags.include_license_data,
        user_data_profile: report.user_data_profile.clone(),
    }
}

/// A manifest as a package of this scan would have it, for going by the rules before anything has
/// been read, with what a build would bundle said to be bundled. Without the files a choice left out.
pub(crate) fn manifest_for_review(
    report: &ScanReport,
    flags: ScanFlags,
    keep_file: &dyn Fn(&FileEntry) -> bool,
) -> PackageManifest {
    // What nothing goes by, for what can come to a good many of them.
    let inventory = report
        .file_inventory
        .iter()
        .filter(|file| file.should_copy && keep_file(file))
        .cloned()
        .map(|mut file| {
            file.reason = String::new();
            file
        })
        .collect();
    let hash_to_come = || Some(String::new());
    let drivers = report
        .drivers
        .iter()
        .cloned()
        .map(|mut driver| {
            if driver.app_owned {
                driver.package_relative_path = Some(driver_package_path(&driver));
                driver.hash_sha256 = hash_to_come();
            }
            driver
        })
        .collect();
    let licenses = report
        .license_artifacts
        .iter()
        .cloned()
        .map(|mut artifact| {
            if artifact.should_copy
                && let Some(logical) = artifact.path.as_deref()
            {
                artifact.package_relative_path = Some(license_package_path(logical));
                artifact.hash_sha256 = hash_to_come();
            }
            artifact
        })
        .collect();
    manifest_of(
        report,
        flags,
        inventory,
        report.runtime_prerequisites.clone(),
        licenses,
        drivers,
    )
}

/// How big a driver a package would bundle is, or nothing for one it would not.
pub(crate) fn driver_image_bytes(report: &ScanReport, driver: &DriverEntry) -> u64 {
    if !driver.app_owned {
        return 0;
    }
    SourceReader::new(report)
        .locate(&driver.image_path)
        .ok()
        .and_then(|image| fs::metadata(image).ok())
        .map_or(0, |metadata| metadata.len())
}

/// Where a package keeps a driver it bundles. By the service's name too, so two with the same file
/// name, as `x64\flt.sys` and `x86\flt.sys` are, do not collide.
fn driver_package_path(driver: &DriverEntry) -> PathBuf {
    let leaf = driver
        .image_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "driver.sys".to_owned());
    let unique = format!("{}_{}", sanitize_artifact_name(&driver.service_name), leaf);
    PathBuf::from(DRIVERS_DIR).join(safe_relative_path(Path::new(&unique)))
}

fn license_package_path(logical: &Path) -> PathBuf {
    PathBuf::from(LICENSE_DIR).join(safe_relative_path(logical))
}

/// An installer on its way out, with the package going into it. In this order, so that the file is
/// closed before what clears it up goes.
struct BuildingInstaller {
    packer: Packer,
    pending: PendingInstaller,
}

/// What can be known of a build before anything is read for it.
fn build_inputs(
    options: &PackageOptions,
    report: &ScanReport,
    stub: Option<&InstallerStub>,
    reader: &SourceReader,
) -> Result<BuildInputs> {
    let copyable: Vec<&FileEntry> = report
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .collect();
    let stub_bytes = match stub {
        Some(InstallerStub::File(path)) => fs::metadata(path)
            .map_err(|error| WinAppMigratorError::io(path, error))?
            .len(),
        Some(InstallerStub::Embedded(bytes)) => bytes.len() as u64,
        None => 0,
    };
    // A drive can say what it likes about how big a file is, and adding that up must not come back
    // around to something that fits.
    let adding = |total: u64, size: u64| total.saturating_add(size);
    let held_license = report
        .license_artifacts
        .iter()
        .filter(|artifact| artifact.should_copy)
        .filter_map(|artifact| artifact.size_bytes)
        .fold(0, adding);
    let held_drivers = report
        .drivers
        .iter()
        .filter(|driver| driver.app_owned)
        .filter_map(|driver| reader.locate(&driver.image_path).ok())
        .filter_map(|image| fs::metadata(image).ok())
        .map(|metadata| metadata.len())
        .fold(0, adding);
    Ok(BuildInputs {
        payload_bytes: copyable
            .iter()
            .filter_map(|file| file.size_bytes)
            .fold(0, adding),
        payload_files: copyable.len(),
        largest_file_bytes: copyable
            .iter()
            .filter_map(|file| file.size_bytes)
            .max()
            .unwrap_or(0),
        held_bytes: held_license.saturating_add(held_drivers),
        prerequisites: if options.bundle_prerequisites {
            report.runtime_prerequisites.len()
        } else {
            0
        },
        stub_bytes,
        connected_drive: report.source_volume.is_some(),
    })
}

fn refuse_without_room(room: &RoomCheck) -> Result<()> {
    let shortfalls = room.shortfalls();
    if shortfalls.is_empty() {
        return Ok(());
    }
    Err(WinAppMigratorError::InvalidInput(format!(
        "there is not enough room to build this installer, so nothing was read or written. {}. \
         Free up some space there, or choose an output folder on another drive.",
        shortfalls.join(". ")
    )))
}

/// What is to go in the package from where the app is, the files that have nowhere to go, and those
/// that look like a credential by now. A scan can be some time ago, and a certificate file is only
/// taken for a key by what is in it, which renewing it can change.
fn files_to_pack(
    options: &PackageOptions,
    report: &ScanReport,
    reader: &SourceReader,
    warnings: &mut Vec<Warning>,
) -> Result<(Vec<PackItem>, HashSet<PathBuf>, Vec<LeftOutItem>)> {
    let mut items = Vec::new();
    let mut not_packable = HashSet::new();
    let mut secrets_since = Vec::new();
    for file in report.file_inventory.iter().filter(|file| file.should_copy) {
        let is_a_secret_by_now = crate::secret_rules::is_secret_file(&file.original_path, || {
            // First, since nearly any other look at a path follows a link.
            reader.check(&file.path).ok()?;
            // One that has gone says so when it is copied, and holds nothing now. Anything else that
            // stops it being looked at says nothing of what is in it.
            match fs::symlink_metadata(&file.path) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Some(Vec::new()),
                _ => read_small_file(&file.path),
            }
        });
        if is_a_secret_by_now {
            let name = file.original_path.display().to_string();
            warnings.push(Warning::new(
                "secret_left_out",
                WarningSeverity::Info,
                "Left out of the package: it looks like a credential by now.",
                Some(name.clone()),
            ));
            secrets_since.push(LeftOutItem {
                name,
                why: crate::secret_rules::LEFT_OUT_REASON_SECRET.to_owned(),
            });
            not_packable.insert(file.original_path.clone());
            continue;
        }
        let Some(relative) = file.package_relative_path.as_ref() else {
            warnings.push(Warning::new(
                "package_relative_path_missing",
                WarningSeverity::Warning,
                "Copyable file had no package relative path.",
                Some(file.original_path.display().to_string()),
            ));
            not_packable.insert(file.original_path.clone());
            continue;
        };
        // Whether a name can be written at all, on this PC, before anything is read for it.
        package_destination_path(&options.output_dir, relative)?;
        items.push(PackItem {
            name: zip_name_of(relative),
            source: file.path.clone(),
            size_hint: file.size_bytes.unwrap_or(0),
            expected_sha256: None,
        });
    }
    Ok((items, not_packable, secrets_since))
}

/// A package that stays a folder: each file copied there, hashed as it goes. A single locked or
/// access-denied file must not abort the whole package, so it says so and the rest go on.
fn copy_to_package_folder(
    options: &PackageOptions,
    items: &[PackItem],
    reader: &SourceReader,
    monitor: &BuildMonitor,
) -> Result<Vec<PackOutcome>> {
    let bytes_total = items
        .iter()
        .fold(0_u64, |total, item| total.saturating_add(item.size_hint));
    let mut bytes_done = 0_u64;
    let mut outcomes = Vec::with_capacity(items.len());
    for (done, item) in items.iter().enumerate() {
        monitor.checkpoint()?;
        monitor.report(BuildStep::Packing {
            done,
            total: items.len(),
            bytes_done,
            bytes_total,
        });
        let destination = options.output_dir.join(&item.name);
        let copied = reader
            .check(&item.source)
            .and_then(|()| copy_hashing_into_package(&item.source, &destination))
            .and_then(|fingerprint| {
                reader.check_after_copying(&item.source)?;
                Ok(fingerprint)
            });
        outcomes.push(match copied {
            Ok((sha256, size)) => {
                bytes_done = bytes_done.saturating_add(size);
                Ok(PackedFile { sha256, size })
            }
            Err(error) => {
                let _ = fs::remove_file(&destination);
                Err(error.to_string())
            }
        });
    }
    Ok(outcomes)
}

/// What everything in the staging folder that a manifest gives a hash for has to hash to.
fn held_hashes(manifest: &PackageManifest) -> HashMap<String, String> {
    let mut hashes = HashMap::new();
    let mut note = |relative: &Path, hash: &str| {
        hashes.insert(zip_name_of(relative).to_lowercase(), hash.to_owned());
    };
    for artifact in &manifest.license_artifacts_detected {
        let Some(relative) = artifact.package_relative_path.as_deref() else {
            continue;
        };
        if let Some(hash) = artifact.hash_sha256.as_deref() {
            note(relative, hash);
        }
        for file in &artifact.store_files {
            note(
                &relative.join(safe_relative_path(&file.relative_path)),
                &file.hash_sha256,
            );
        }
    }
    for driver in &manifest.drivers_detected {
        if let (Some(relative), Some(hash)) = (
            driver.package_relative_path.as_deref(),
            driver.hash_sha256.as_deref(),
        ) {
            note(relative, hash);
        }
    }
    for prerequisite in &manifest.runtime_prerequisites {
        let bundle = prerequisite.bundle.as_ref();
        if let (Some(relative), Some(hash)) = (
            bundle.and_then(|bundle| bundle.package_relative_path.as_deref()),
            bundle.and_then(|bundle| bundle.hash_sha256.as_deref()),
        ) {
            note(relative, hash);
        }
    }
    hashes
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
            let relative = driver_package_path(&driver);
            match package_destination_path(output_dir, &relative).and_then(|destination| {
                let copied = copy_hashing_into_package(image, &destination).and_then(|copied| {
                    reader.check_after_copying(image)?;
                    Ok(copied)
                });
                if copied.is_err() {
                    let _ = fs::remove_file(&destination);
                }
                copied
            }) {
                // Only mark as bundled when we have an integrity hash, so
                // verification cannot trivially pass on a bad image.
                Ok((hash, _)) => {
                    driver.hash_sha256 = Some(hash);
                    driver.package_relative_path = Some(relative);
                }
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
    let relative = license_package_path(&logical);
    let Ok(destination) = package_destination_path(output_dir, &relative) else {
        return false;
    };
    let Ok(source) = reader.locate(&logical) else {
        return false;
    };
    if source.is_file() {
        // A license goes by its option, and what looks like a credential goes nowhere, by now as well.
        if crate::secret_rules::is_secret_file(&source, || read_small_file(&source)) {
            warnings.push(Warning::new(
                "license_store_file_left_out",
                WarningSeverity::Info,
                "A license file looks like a credential, and was left out.",
                Some(logical.display().to_string()),
            ));
            return false;
        }
        let staged = copy_hashing_into_package(&source, &destination).and_then(|copied| {
            reader.check_after_copying(&source)?;
            Ok(copied)
        });
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
        // A store is copied whole, and what looks like a credential goes no more than anywhere else.
        if crate::secret_rules::is_secret_file(&entry.path, || read_small_file(&entry.path)) {
            warnings.push(Warning::new(
                "license_store_file_left_out",
                WarningSeverity::Info,
                "A file in a license store looks like a credential, and was left out.",
                Some(entry.path.display().to_string()),
            ));
            continue;
        }
        let target = destination.join(safe_relative_path(relative));
        match copy_hashing_into_package(&entry.path, &target).and_then(|copied| {
            reader.check_after_copying(&entry.path)?;
            Ok(copied)
        }) {
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

/// What is in a file no bigger than a key file is, to tell whether it holds a private key.
fn read_small_file(path: &Path) -> Option<Vec<u8>> {
    fs::metadata(path)
        .ok()
        .filter(|metadata| metadata.len() <= crate::secret_rules::MAX_KEY_FILE_BYTES)
        .and_then(|_| fs::read(path).ok())
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

/// Copy a file into the package's folder, and say what the bytes that were copied hash to and how
/// many there were. Those and nothing else: the file changing while it is read cannot make the hash
/// describe anything but what was written.
fn copy_hashing_into_package(source: &Path, destination: &Path) -> Result<(String, u64)> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
    }
    // A plain in-process copy fails on files whose NTFS ACL denies this process —
    // common for a foreign disk owned by another PC's SIDs. Fall back to a
    // backup-semantics copy (robocopy /B) that reads through SeBackupPrivilege
    // without modifying the source, then apply metadata, which has to be read to be hashed.
    // The plain copy is tried first so the common (accessible) case stays fast and
    // dependency-free.
    let fingerprint = match copy_and_hash(source, destination) {
        Ok(fingerprint) => fingerprint,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            let _ = fs::remove_file(destination);
            copy_file_backup_mode(source, destination)?;
            let size = fs::metadata(destination)
                .map_err(|error| WinAppMigratorError::io(destination, error))?
                .len();
            (sha256_file(destination)?, size)
        }
        Err(error) => return Err(WinAppMigratorError::io(destination, error)),
    };
    let Ok(metadata) = fs::metadata(source) else {
        // Source metadata may be unreadable even when a backup-mode copy
        // succeeded; the bytes are what matter, so keep the copied file as-is.
        return Ok(fingerprint);
    };
    let _ = fs::set_permissions(destination, metadata.permissions());
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    let _ = set_file_times(destination, accessed, modified);
    Ok(fingerprint)
}

fn copy_and_hash(source: &Path, destination: &Path) -> std::io::Result<(String, u64)> {
    let mut input = File::open(source)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; COPY_BUFFER_BYTES];
    let mut size = 0_u64;
    loop {
        let read = input.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        output.write_all(&buffer[..read])?;
        size += read as u64;
    }
    Ok((hex_lower(&hasher.finalize()), size))
}

/// Copy a single file using `robocopy /B` (backup mode), which reads through the
/// process's `SeBackupPrivilege` so restrictive NTFS ACLs on a foreign disk do
/// not block the copy. It is strictly read-only on the source (never changes its
/// ACLs or attributes). robocopy copies by directory + file name, so the source
/// and destination file names must match; the caller passes a destination whose
/// name equals the source's.
pub(crate) fn copy_file_backup_mode(source: &Path, destination: &Path) -> Result<()> {
    copy_file_backup_mode_unless(source, destination, &|| false)
}

/// [`copy_file_backup_mode`], giving up with [`WinAppMigratorError::Cancelled`] if asked to. A large
/// file on a failing drive can take as long as anything in a build.
pub(crate) fn copy_file_backup_mode_unless(
    source: &Path,
    destination: &Path,
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<()> {
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
    let mut robocopy = hidden_command("robocopy")
        .arg(source_dir)
        .arg(destination_dir)
        .arg(file_name)
        // It goes by the name, ignoring case, and a folder can be case-sensitive, so `/XJ`: a link
        // of the same name must not be what is copied.
        .args([
            "/B", "/XJ", "/NJH", "/NJS", "/NP", "/NDL", "/NC", "/NS", "/R:1", "/W:1",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|error| WinAppMigratorError::io(source, error))?;
    // Quickly at first, since a small file takes it no time and there can be thousands of them.
    let mut look = BACKUP_COPY_FIRST_LOOK;
    let status = loop {
        let waited = robocopy.try_wait();
        let cancelled = is_cancelled();
        if cancelled || waited.is_err() {
            let _ = robocopy.kill();
            let _ = robocopy.wait();
        }
        if cancelled {
            return Err(WinAppMigratorError::Cancelled);
        }
        match waited.map_err(|error| WinAppMigratorError::io(source, error))? {
            Some(status) => break status,
            None => {
                std::thread::sleep(look);
                look = (look * 2).min(BACKUP_COPY_LONGEST_LOOK);
            }
        }
    };
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
            let bytes = hex_decode(entry.value_bytes_hex.as_deref().unwrap_or_default())
                .unwrap_or_default();
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
    use crate::footprint::ChoiceId;
    use crate::models::{
        CertificateEntry, DependencyEntry, DependencySourceCategory, EnvironmentScope,
        EnvironmentVariableEntry, MatchConfidence, ServiceEntry, ShortcutEntry,
    };
    use crate::progress::{ScanMonitor, ScanStep};
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

    /// A real scan of a program in a folder, read-only, with something beside it to pack.
    fn app_in(root: &Path) -> Result<PathBuf> {
        let app = root.join("app");
        fs::create_dir_all(app.join("data"))
            .map_err(|error| WinAppMigratorError::io(&app, error))?;
        let exe = app.join("fixture.exe");
        fs::copy(
            std::env::current_exe().map_err(|error| WinAppMigratorError::io("exe", error))?,
            &exe,
        )
        .map_err(|error| WinAppMigratorError::io(&exe, error))?;
        fs::write(app.join(r"data\a.txt"), "hello hello hello hello")
            .map_err(|error| WinAppMigratorError::io(&app, error))?;
        Ok(exe)
    }

    fn installer_options(root: &Path, exe: &Path, stub: &Path) -> PackageOptions {
        PackageOptions {
            create_self_extracting_archive: true,
            self_extracting_stub: Some(InstallerStub::File(stub.to_path_buf())),
            installer_dir: Some(root.join("out")),
            include_user_data: false,
            include_program_data: false,
            ..PackageOptions::for_executable(exe, root.join("staging").join("App"))
        }
    }

    fn plain_stub(root: &Path) -> Result<PathBuf> {
        let stub = root.join("stub.bin");
        // Not a program, so it has no signature to refuse. Nothing here runs it.
        fs::write(&stub, b"a stub").map_err(|error| WinAppMigratorError::io(&stub, error))?;
        Ok(stub)
    }

    #[test]
    fn an_apps_files_go_straight_into_the_installer_with_what_was_read_recorded() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let result = create_package(&installer_options(root.path(), &exe, &stub))?;
        let installer = result
            .self_extracting_archive_path
            .clone()
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        assert_eq!(result.data_file_path, None);

        // Nothing of the app was copied anywhere on the way.
        let staged_files = result.package_root.join(FILES_DIR);
        assert_eq!(
            fs::read_dir(&staged_files)
                .map_err(|error| WinAppMigratorError::io(&staged_files, error))?
                .count(),
            0
        );

        // What the manifest recorded is what comes out, and what is really there.
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let manifest = read_embedded_manifest(&package)?;
        let digests = package.digests(&|| false)?;
        let copied: Vec<&FileEntry> = manifest
            .file_inventory
            .iter()
            .filter(|file| file.should_copy)
            .collect();
        assert!(copied.len() >= 2);
        for file in copied {
            let relative = file
                .package_relative_path
                .as_deref()
                .unwrap_or(Path::new(""));
            let seen = digests
                .get(relative)
                .ok_or_else(|| WinAppMigratorError::Archive(relative.display().to_string()))?;
            assert_eq!(Some(&seen.sha256), file.hash_sha256.as_ref());
            assert_eq!(Some(seen.size), file.size_bytes);
            assert_eq!(seen.sha256, sha256_file(&file.original_path)?);
            assert_eq!(file.path, file.original_path);
        }
        assert!(digests.get(Path::new(MANIFEST_FILE)).is_some());
        assert!(
            digests.get(Path::new(r"reports\warnings.md")).is_some()
                || !manifest.warnings.is_empty()
        );
        assert!(manifest.source_platform.is_some());
        let main = manifest
            .program_needs
            .iter()
            .find(|program| program.role == crate::models::ProgramRole::Main)
            .ok_or_else(|| WinAppMigratorError::Archive("no main program".to_owned()))?;
        assert_eq!(main.machine, crate::restore_policy::MACHINE_X64);
        drop(package);

        let report = crate::verify::verify_installer(&installer, &BuildMonitor::default())?;
        assert!(report.package_valid, "{:?}", report.errors);
        assert!(report.restore_valid.is_none());
        let out = root.path().join("out");
        assert_eq!(
            fs::read_dir(&out)
                .map_err(|error| WinAppMigratorError::io(&out, error))?
                .count(),
            1,
            "only the installer is in the folder it went to, even after checking it"
        );
        Ok(())
    }

    #[test]
    fn a_file_named_twice_in_a_package_only_goes_in_once() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let package = root.path().join("package");
        fs::create_dir_all(&package).map_err(|error| WinAppMigratorError::io(&package, error))?;
        let path = package.join("a.txt");
        fs::write(&path, b"x").map_err(|error| WinAppMigratorError::io(&path, error))?;
        let item = PackItem {
            name: "a.txt".to_owned(),
            source: path,
            size_hint: 1,
            expected_sha256: None,
        };
        let output = root.path().join("out.zip");
        let file =
            File::create(&output).map_err(|error| WinAppMigratorError::io(&output, error))?;
        let mut packer = Packer::new(file, 0, BuildMonitor::default());
        let first = packer.pack(std::slice::from_ref(&item), &HELD_FILES)?;
        let second = packer.pack(std::slice::from_ref(&item), &HELD_FILES)?;
        assert!(first[0].is_ok());
        assert!(
            second[0].is_err(),
            "whichever part of a build it comes from"
        );
        Ok(())
    }

    #[test]
    fn a_cancelled_build_leaves_nothing_where_the_installer_goes() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let out = root.path().join("out");
        let earlier = create_package(&options)?
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let earlier_hash = sha256_file(&earlier)?;

        // Cancelled by what it reports, part-way through packing, as a person pressing the button.
        let slot: std::sync::Arc<std::sync::Mutex<Option<BuildMonitor>>> =
            std::sync::Arc::new(std::sync::Mutex::new(None));
        let canceller = std::sync::Arc::clone(&slot);
        let monitor = BuildMonitor::new(move |step| {
            if matches!(step, BuildStep::Packing { .. })
                && let Ok(canceller) = canceller.lock()
                && let Some(monitor) = canceller.as_ref()
            {
                monitor.cancel();
            }
        });
        *slot
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))? =
            Some(monitor.clone());
        let staging = root.path().join("staging");
        fs::remove_dir_all(&staging).map_err(|error| WinAppMigratorError::io(&staging, error))?;
        let cancelled = create_package_monitored(&options, &monitor);
        assert!(matches!(cancelled, Err(WinAppMigratorError::Cancelled)));

        let names: Vec<String> = fs::read_dir(&out)
            .map_err(|error| WinAppMigratorError::io(&out, error))?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            ["App_installer.exe"],
            "and nothing partial beside it"
        );
        assert_eq!(
            sha256_file(&earlier)?,
            earlier_hash,
            "and the earlier one is as it was"
        );
        Ok(())
    }

    /// As late as a cancel can come, with what is left of the package still to go in.
    #[test]
    fn a_build_cancelled_at_the_last_moment_leaves_nothing_either() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let slot: std::sync::Arc<std::sync::Mutex<Option<BuildMonitor>>> =
            std::sync::Arc::new(std::sync::Mutex::new(None));
        let canceller = std::sync::Arc::clone(&slot);
        let first_total = std::sync::Mutex::new(None);
        let monitor = BuildMonitor::new(move |step| {
            // The count goes up when what is in the staging folder starts going in.
            if let BuildStep::Packing { total, .. } = step
                && let Ok(mut first_total) = first_total.lock()
                && *first_total.get_or_insert(total) < total
                && let Ok(canceller) = canceller.lock()
                && let Some(monitor) = canceller.as_ref()
            {
                monitor.cancel();
            }
        });
        *slot
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))? =
            Some(monitor.clone());
        let cancelled = create_package_monitored(&options, &monitor);
        assert!(
            matches!(cancelled, Err(WinAppMigratorError::Cancelled)),
            "{cancelled:?}"
        );
        let out = root.path().join("out");
        let names: Vec<String> = fs::read_dir(&out)
            .map_err(|error| WinAppMigratorError::io(&out, error))?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn a_backup_mode_copy_can_be_called_off() {
        let source = Path::new(r"C:\Windows\System32\notepad.exe");
        let destination = std::env::temp_dir().join("not written").join("notepad.exe");
        let called_off = copy_file_backup_mode_unless(source, &destination, &|| true);
        assert!(matches!(called_off, Err(WinAppMigratorError::Cancelled)));
    }

    #[test]
    fn a_build_without_room_says_how_much_and_where_and_reads_nothing() {
        let room = RoomCheck {
            drives: vec![crate::space::DriveRoom {
                drive: "D:".to_owned(),
                needed_bytes: 10 * 1024 * 1024 * 1024,
                free_bytes: Some(2 * 1024 * 1024 * 1024),
                what_for: vec!["the installer"],
            }],
        };
        let refused = refuse_without_room(&room);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("D:")
                    && message.contains("10.0 GB")
                    && message.contains("2.0 GB")
                    && message.contains("another drive")),
            "{refused:?}"
        );
        assert!(refuse_without_room(&RoomCheck::default()).is_ok());
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

    /// A scan of what `app_in` makes, as a window would ask for it.
    fn scan_of(options: &PackageOptions) -> Result<ScanReport> {
        scan_app(&scan_options_for(options), &ScanMonitor::default())
    }

    fn data_folder_id(exe: &Path) -> Result<ChoiceId> {
        let folder = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .join("data");
        Ok(ChoiceId::Folder(crate::path_utils::comparable_path(
            &folder,
        )))
    }

    const NO_FLAGS: ScanFlags = ScanFlags {
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
    };

    #[test]
    fn what_a_package_says_of_data_it_was_asked_for_stays_even_if_some_could_not_come() -> Result<()>
    {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let mut report = scan_of(&installer_options(root.path(), &exe, &stub))?;
        report.warnings.clear();
        let placeholder = PathBuf::from(r"C:\Users\alice\AppData\Roaming\Acme\online-only.dat");
        let files_before = report.file_inventory.len();
        report.file_inventory.push(FileEntry {
            path: placeholder.clone(),
            original_path: placeholder.clone(),
            package_relative_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::UserData,
            should_copy: false,
            reason: String::new(),
            user_folder: None,
        });
        report.warnings.push(Warning::new(
            "cloud_placeholder_skipped",
            WarningSeverity::Info,
            "Online-only",
            Some(placeholder.display().to_string()),
        ));
        let asked_for = PackageOptions {
            include_user_data: true,
            ..installer_options(root.path(), &exe, &stub)
        };
        let said = what_a_package_says_of(&report, &asked_for);
        assert_eq!(said.file_inventory.len(), files_before + 1);
        assert_eq!(
            said.warnings.len(),
            1,
            "it was asked for, and it is not coming"
        );
        let not_asked_for = PackageOptions {
            include_user_data: false,
            ..asked_for
        };
        let said = what_a_package_says_of(&report, &not_asked_for);
        assert_eq!(said.file_inventory.len(), files_before);
        assert!(said.warnings.is_empty());
        Ok(())
    }

    #[test]
    fn a_scan_that_brought_in_data_cannot_be_built_without_asking_for_it() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let mut report = scan_of(&options)?;
        let data = PathBuf::from(r"C:\Users\alice\AppData\Roaming\Acme\saved.dat");
        report.file_inventory.push(FileEntry {
            path: data.clone(),
            original_path: data,
            package_relative_path: Some(PathBuf::from(r"files\_external\saved.dat")),
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        });
        let is_refused = |report: &ScanReport| {
            matches!(
                create_package_from_scan(
                    &options,
                    report,
                    &FootprintChoices::default(),
                    &BuildMonitor::default(),
                ),
                Err(WinAppMigratorError::InvalidInput(_))
            )
        };
        assert!(is_refused(&report));
        report.file_inventory.pop();
        // Only what an account has in its keys, or only a license, is the same.
        report.user_data_profile = Some("alice".to_owned());
        assert!(is_refused(&report));
        report.user_data_profile = None;
        report.license_registry_entries = vec![RegistryEntry {
            hive: RegistryHive::CurrentUser,
            key_path: r"Software\Acme".to_owned(),
            value_name: Some("LicenseKey".to_owned()),
            value_data: Some("x".to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: crate::models::MatchConfidence::Medium,
            requires_admin: false,
        }];
        assert!(is_refused(&report));
        report.license_registry_entries.clear();
        assert!(!is_refused(&report), "and a scan as asked for builds");
        Ok(())
    }

    #[test]
    fn with_all_of_the_data_left_out_a_package_does_not_say_whose_it_was() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let mut report = scan_of(&options)?;
        report.user_data_profile = Some("the-secret-account".to_owned());
        report.source_computer_name = Some("THE-SECRET-PC".to_owned());
        report.install_location = Some(PathBuf::from(
            r"C:\Users\the-secret-account\AppData\Local\Programs\Acme",
        ));
        report.secrets_left_out = vec![
            LeftOutItem {
                name: r"C:\Users\the-secret-account\AppData\Roaming\Acme\credentials.json"
                    .to_owned(),
                why: "its name says it holds credentials".to_owned(),
            },
            LeftOutItem {
                name: r"C:\Users\the-secret-account\AppData\Local\Programs\Acme\.env".to_owned(),
                why: "its name says it holds credentials".to_owned(),
            },
        ];
        let said = what_a_package_says_of(&report, &options);
        assert_eq!(said.user_data_profile, None);
        assert_eq!(said.source_computer_name, None);
        assert_eq!(
            said.secrets_left_out.len(),
            1,
            "the app's own is still said"
        );
        assert!(said.secrets_left_out[0].name.ends_with(".env"));
        Ok(())
    }

    /// Every place a package says what it says, read as it was written.
    fn what_a_built_package_holds(result: &PackageResult) -> Result<String> {
        let mut all = fs::read_to_string(&result.manifest_path)
            .map_err(|error| WinAppMigratorError::io(&result.manifest_path, error))?;
        for entry in fs::read_dir(&result.reports_dir)
            .map_err(|error| WinAppMigratorError::io(&result.reports_dir, error))?
        {
            let path = entry
                .map_err(|error| WinAppMigratorError::io(&result.reports_dir, error))?
                .path();
            all.push_str(
                &fs::read_to_string(&path)
                    .map_err(|error| WinAppMigratorError::io(&path, error))?,
            );
        }
        Ok(all)
    }

    #[test]
    fn a_package_only_names_the_pc_it_came_from_with_the_user_data_option() -> Result<()> {
        const NAME: &str = "THE-SECRET-PC";
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let mut options = installer_options(root.path(), &exe, &stub);
        options.include_user_data = false;
        options.include_program_data = false;

        let mut report = scan_of(&options)?;
        assert_eq!(
            report.source_computer_name, None,
            "a scan with the option off does not hold on to it either"
        );
        // A scan is not all that decides: a person can leave all the data out of one that had it.
        report.source_computer_name = Some(NAME.to_owned());
        let without = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let held = what_a_built_package_holds(&without)?;
        assert!(
            held.contains("source_computer_name"),
            "this read the manifest"
        );
        assert!(!held.contains(NAME), "{held}");

        options.include_user_data = true;
        options.include_program_data = true;
        options.output_dir = root.path().join("with-data");
        let with = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        assert!(what_a_built_package_holds(&with)?.contains(NAME));
        Ok(())
    }

    #[test]
    fn a_scan_with_the_user_data_option_on_knows_what_pc_it_is_on() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let mut options = installer_options(root.path(), &exe, &stub);
        options.include_user_data = true;
        assert_eq!(
            scan_of(&options)?.source_computer_name,
            std::env::var("COMPUTERNAME").ok()
        );
        Ok(())
    }

    #[test]
    fn a_report_says_what_the_manifest_says_of_a_file_that_did_not_go_in() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        let gone = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .join(r"data\a.txt");
        assert!(
            report
                .file_inventory
                .iter()
                .any(|file| file.original_path == gone && file.should_copy)
        );
        fs::remove_file(&gone).map_err(|error| WinAppMigratorError::io(&gone, error))?;

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;

        let manifest: PackageManifest = read_json(&result.manifest_path)?;
        let said: ScanReport = read_json(&result.reports_dir.join("scan_report.json"))?;
        let program_size = fs::metadata(&exe)
            .map_err(|error| WinAppMigratorError::io(&exe, error))?
            .len();
        let app_folder = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?;
        let its_folder = said
            .footprint_locations
            .iter()
            .find(|location| location.path == app_folder);
        assert_eq!(
            its_folder.map(|location| (location.copyable_bytes, location.included)),
            Some((program_size, true)),
            "what a location is said to have copied goes by what it did"
        );
        let is_going = |files: &[FileEntry]| {
            files
                .iter()
                .find(|file| file.original_path == gone)
                .map(|file| file.should_copy)
        };
        assert_eq!(is_going(&manifest.file_inventory), Some(false));
        assert_eq!(is_going(&said.file_inventory), Some(false));
        assert!(
            said.warnings
                .iter()
                .any(|warning| warning.code == FILE_GONE_CODE)
        );
        let warnings_file = result.reports_dir.join("warnings.md");
        let warnings = fs::read_to_string(&warnings_file)
            .map_err(|error| WinAppMigratorError::io(&warnings_file, error))?;
        let name = gone
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
        assert!(warnings.contains(&name), "{warnings}");
        Ok(())
    }

    fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
        let text =
            fs::read_to_string(path).map_err(|error| WinAppMigratorError::io(path, error))?;
        serde_json::from_str(&text)
            .map_err(|error| WinAppMigratorError::Archive(format!("{}: {error}", path.display())))
    }

    fn empty_folder(index: usize) -> FileEntry {
        let path = PathBuf::from(format!(r"C:\App\empty {index}"));
        FileEntry {
            path: path.clone(),
            original_path: path,
            package_relative_path: None,
            size_bytes: None,
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    #[test]
    fn only_what_is_left_of_empty_folders_counts_toward_how_many_are_made() {
        let mut warnings = Vec::new();
        let mut folders: Vec<FileEntry> = (0..MAX_EMPTY_FOLDERS).map(empty_folder).collect();
        keep_what_an_installer_will_make(&mut folders, &mut warnings);
        assert_eq!(folders.len(), MAX_EMPTY_FOLDERS);
        assert!(warnings.is_empty(), "exactly that many is no loss");

        folders.push(empty_folder(MAX_EMPTY_FOLDERS));
        keep_what_an_installer_will_make(&mut folders, &mut warnings);
        assert_eq!(folders.len(), MAX_EMPTY_FOLDERS);
        assert_eq!(warnings.len(), 1);
    }

    /// A person leaves a folder out that holds most of them, and what is left is all made.
    #[test]
    fn a_scan_keeps_every_empty_folder_for_a_person_to_choose_from() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let app = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?;
        for index in 0..3 {
            let folder = app.join(format!("empty {index}"));
            fs::create_dir(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
        }
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        assert_eq!(scan_of(&options)?.empty_folders.len(), 3);
        Ok(())
    }

    /// As the app does: a scan writes nothing, so it is told of no folder, and a build comes later.
    #[test]
    fn a_build_writes_its_reports_where_it_is_building_whatever_a_scan_was_told() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&PackageOptions {
            output_dir: PathBuf::new(),
            ..options.clone()
        })?;
        assert_eq!(
            report
                .generated_artifacts
                .as_ref()
                .map(|artifacts| artifacts.scan_report_json.clone()),
            Some(PathBuf::from(REPORTS_DIR).join("scan_report.json")),
            "it is told of no folder"
        );

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;

        assert!(
            !Path::new(REPORTS_DIR).exists(),
            "nothing is written where this happens to be running"
        );
        let installer = result
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert!(
            !package.read_file("reports/scan_report.json")?.is_empty(),
            "and they are in the package"
        );
        Ok(())
    }

    #[test]
    fn what_a_package_says_holds_nothing_of_data_that_is_not_going() -> Result<()> {
        const THEIRS: &str = r"C:\Users\the-secret-account\AppData\Roaming\Acme";
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = PackageOptions {
            create_self_extracting_archive: false,
            ..installer_options(root.path(), &exe, &stub)
        };
        let mut report = scan_of(&options)?;
        let staying = PathBuf::from(THEIRS).join("the-secret-host.conf");
        report.file_inventory.push(FileEntry {
            path: staying.clone(),
            original_path: staying.clone(),
            package_relative_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::UserData,
            should_copy: false,
            reason: "User data found but excluded by current scan options".to_owned(),
            user_folder: None,
        });
        report
            .footprint_locations
            .push(crate::models::FootprintLocation {
                path: PathBuf::from(THEIRS),
                kind: crate::models::FootprintKind::RoamingAppData,
                scope: crate::models::InstallScope::PerUser,
                file_count: 1,
                total_bytes: 1,
                copyable_bytes: 0,
                included: false,
            });
        report.warnings.push(Warning::new(
            "cloud_placeholder_skipped",
            WarningSeverity::Info,
            "Online-only",
            Some(staying.display().to_string()),
        ));
        report.warnings.push(Warning::new(
            "another",
            WarningSeverity::Info,
            "this one stays",
            Some(r"Software\Acme".to_owned()),
        ));
        // One that was never going to be listed, as a file of another account's is not.
        report.warnings.push(Warning::new(
            "file_discovery_left_out",
            WarningSeverity::Warning,
            r"C:\Users\the-secret-account\Documents\the-secret-host.txt was left out",
            Some(r"C:\Users\the-secret-account\Documents\the-secret-host.txt".to_owned()),
        ));

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;

        let manifest = fs::read_to_string(&result.manifest_path)
            .map_err(|error| WinAppMigratorError::io(&result.manifest_path, error))?;
        let reports = result.reports_dir.join("scan_report.json");
        let said = fs::read_to_string(&reports)
            .map_err(|error| WinAppMigratorError::io(&reports, error))?;
        for what in [&manifest, &said] {
            assert!(!what.contains("the-secret-account"), "{what}");
            assert!(!what.contains("the-secret-host"), "{what}");
            assert!(what.contains("this one stays"), "{what}");
        }
        Ok(())
    }

    #[test]
    fn a_scan_writes_nothing_and_a_build_from_it_does_not_scan_again() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        assert!(!options.output_dir.exists(), "a scan writes nowhere");

        let late = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .join(r"data\added since.txt");
        fs::write(&late, "new").map_err(|error| WinAppMigratorError::io(&late, error))?;

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let installer = result
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let manifest = read_embedded_manifest(&package)?;
        assert!(
            !manifest
                .file_inventory
                .iter()
                .any(|file| file.original_path == late),
            "it goes by what the scan found"
        );
        assert!(manifest.left_out_by_choice.is_empty());
        assert!(manifest.added_by_choice.is_empty());
        Ok(())
    }

    #[test]
    fn what_is_left_out_is_not_in_the_package_or_its_manifest_and_the_rest_is() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        let mut choices = FootprintChoices::default();
        choices.set_included(&data_folder_id(&exe)?, false);

        let result =
            create_package_from_scan(&options, &report, &choices, &BuildMonitor::default())?;
        let installer = result
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let manifest = read_embedded_manifest(&package)?;
        let digests = package.digests(&|| false)?;
        assert!(
            !manifest
                .file_inventory
                .iter()
                .any(|file| file.original_path.to_string_lossy().contains(r"\data\")),
            "{:?}",
            manifest.file_inventory
        );
        assert!(digests.get(Path::new(r"files\data\a.txt")).is_none());
        assert!(digests.get(Path::new(r"files\fixture.exe")).is_some());
        assert!(
            manifest
                .left_out_by_choice
                .iter()
                .any(|item| item.name.ends_with("data")),
            "{:?}",
            manifest.left_out_by_choice
        );
        assert_eq!(result.files_copied, 1);
        Ok(())
    }

    #[test]
    fn what_the_screen_adds_up_is_what_a_build_goes_by() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        let footprint = Footprint::of(&report, NO_FLAGS)?;
        let stub_bytes = fs::metadata(&stub)
            .map_err(|error| WinAppMigratorError::io(&stub, error))?
            .len();

        let reader = SourceReader::new(&report);
        let installer_stub = options.self_extracting_stub.as_ref();
        assert_eq!(
            footprint.build_inputs(&FootprintChoices::default(), &[], stub_bytes, false),
            build_inputs(&options, &report, installer_stub, &reader)?
        );

        let mut choices = FootprintChoices::default();
        choices.set_included(&data_folder_id(&exe)?, false);
        let chosen = apply_choices(&report, &footprint, &choices)?;
        assert_eq!(
            footprint.build_inputs(&choices, &chosen.also, stub_bytes, false),
            build_inputs(&options, &chosen.report, installer_stub, &reader)?
        );
        Ok(())
    }

    #[test]
    fn a_file_changed_since_the_scan_goes_in_as_it_is_and_one_gone_is_said_to_be() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let folder = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .to_path_buf();
        let gone = folder.join(r"data\gone.txt");
        fs::write(&gone, "here for now").map_err(|error| WinAppMigratorError::io(&gone, error))?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;

        fs::remove_file(&gone).map_err(|error| WinAppMigratorError::io(&gone, error))?;
        let changed = folder.join(r"data\a.txt");
        fs::write(&changed, "something else, and a different length")
            .map_err(|error| WinAppMigratorError::io(&changed, error))?;

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.code == FILE_GONE_CODE
                    && warning
                        .source
                        .as_deref()
                        .is_some_and(|source| source.ends_with("gone.txt"))),
            "{:?}",
            result.warnings
        );
        let installer = result
            .self_extracting_archive_path
            .clone()
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let manifest = read_embedded_manifest(&package)?;
        let digests = package.digests(&|| false)?;
        let entry = manifest
            .file_inventory
            .iter()
            .find(|file| file.original_path == changed)
            .ok_or_else(|| WinAppMigratorError::Archive("the changed file".to_owned()))?;
        assert_eq!(entry.hash_sha256, Some(sha256_file(&changed)?));
        assert_eq!(
            digests
                .get(Path::new(r"files\data\a.txt"))
                .map(|seen| &seen.sha256),
            entry.hash_sha256.as_ref()
        );
        assert!(digests.get(Path::new(r"files\data\gone.txt")).is_none());
        assert!(
            manifest
                .file_inventory
                .iter()
                .any(|file| file.original_path == gone && !file.should_copy),
            "the manifest does not say it is there"
        );
        let report = crate::verify::verify_installer(&installer, &BuildMonitor::default())?;
        assert!(report.package_valid, "{:?}", report.errors);
        Ok(())
    }

    const WHO_IT_WAS: [&str; 21] = [
        "acct-service-command",
        "acct-service-args",
        "acct-session-host",
        "PCNAME-TASK-AUTHOR",
        "acct-task-author",
        "acct-trigger",
        "S-1-5-21-111-222-333-1001",
        "acct-second-action",
        "acct-lnk-path",
        "secret-doc-in-args",
        "acct-out-of-reach",
        "fileserver-mark",
        "acct-refused-share",
        "acct-hive-note",
        "acct-classes-note",
        "acct-path-dll",
        "acct-env-value",
        "acct-sys-file",
        "PCNAME-SERVICE",
        "acct-service",
        "acct-cert-subject",
    ];

    /// Nothing reads how an app was removed on the PC it came from, and it can be a file of someone's.
    #[test]
    fn how_an_app_was_removed_where_it_was_is_not_said() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let mut report = scan_of(&options)?;
        report.source_app = Some(crate::models::InstalledApp {
            display_name: "Acme".to_owned(),
            publisher: None,
            version: None,
            install_location: None,
            uninstall_string: Some(r#""E:\alice\Downloads\foo-setup.exe" /x"#.to_owned()),
            main_executable: None,
            registry_source_path: String::new(),
            architecture: AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        });
        let said = what_a_package_says_of(&report, &options);
        assert_eq!(
            said.source_app
                .as_ref()
                .map(|app| app.uninstall_string.clone()),
            Some(None)
        );
        Ok(())
    }

    /// All the ways a scan has been found to say who it was, as a real scan would have them.
    fn plant_who_it_was(report: &mut ScanReport, exe: &Path) {
        let exe_text = exe.display().to_string();
        report.account_traces.account_names =
            ["acct-out-of-reach", "acct-hive-note", "acct-classes-note"]
                .map(str::to_owned)
                .to_vec();
        report.scheduled_tasks.push(ScheduledTaskEntry {
            task_name: "Probe".to_owned(),
            task_path: r"\".to_owned(),
            action_command: exe.to_path_buf(),
            arguments: None,
            working_directory: None,
            trigger_summary: Some("LogonTrigger".to_owned()),
            run_level: None,
            account: Some("S-1-5-21-111-222-333-1001".to_owned()),
            requires_admin: false,
            restore_instructions: String::new(),
            definition_xml: Some(format!(
                "<?xml version=\"1.0\"?><Task xmlns=\"http://schemas.microsoft.com/windows/2004/02/mit/task\"><RegistrationInfo><Author>PCNAME-TASK-AUTHOR\\acct-task-author</Author></RegistrationInfo><Triggers><LogonTrigger><UserId>PCNAME-TASK-AUTHOR\\acct-trigger</UserId></LogonTrigger></Triggers><Principals><Principal id=\"Author\"><UserId>S-1-5-21-111-222-333-1001</UserId></Principal></Principals><Actions Context=\"Author\"><Exec><Command>{exe_text}</Command></Exec><Exec><Command>C:\\Users\\acct-second-action\\x.cmd</Command></Exec></Actions></Task>"
            )),
        });
        report.shortcuts.push(ShortcutEntry {
            original_lnk_path: PathBuf::from(r"C:\Users\acct-lnk-path\Desktop\My App.lnk"),
            target_path: exe.to_path_buf(),
            arguments: Some(r#""C:\Users\acct\Documents\secret-doc-in-args.txt""#.to_owned()),
            working_directory: None,
            icon_path: None,
            target_is_selected_app: true,
        });
        report.shortcuts.push(ShortcutEntry {
            original_lnk_path: PathBuf::from(r"C:\Users\acct-lnk-path\Desktop\Plain.lnk"),
            target_path: exe.to_path_buf(),
            arguments: None,
            working_directory: Some(PathBuf::from(r"C:\Users\acct-lnk-path\Work")),
            icon_path: Some(PathBuf::from(r"C:\Users\acct-lnk-path\i.ico")),
            target_is_selected_app: true,
        });
        for (code, message) in [
            (
                "user_folder_out_of_reach",
                r"acct-out-of-reach's Documents folder was at \\fileserver-mark\home$\acct-refused-share\Documents, which is not on a disk of this PC.",
            ),
            (
                "network_paths_not_followed",
                r"A path recorded on this PC could not be read, so nothing there was included: \\fileserver-mark\home$\acct-refused-share\Documents",
            ),
            (
                "user_hive_unreadable",
                "The settings of acct-hive-note could not be read, so they were not included: x",
            ),
            (
                "user_classes_unreadable",
                "The COM classes and file types registered for acct-classes-note could not be read",
            ),
        ] {
            report
                .warnings
                .push(Warning::new(code, WarningSeverity::Warning, message, None));
        }
        report.dependencies.push(DependencyEntry {
            name: "zlib1.dll".to_owned(),
            resolved_path: Some(PathBuf::from(r"C:\Users\acct-path-dll\tools\bin\zlib1.dll")),
            architecture: AppArchitecture::X64,
            hash_sha256: None,
            version_info: None,
            publisher: None,
            source_category: DependencySourceCategory::Path,
            should_copy: false,
            system_provided: false,
            runtime_prerequisite: false,
            missing: false,
            delay_loaded: false,
        });
        report.environment_variables.push(EnvironmentVariableEntry {
            scope: EnvironmentScope::Machine,
            name: "PYTHONPATH".to_owned(),
            value: format!(
                r"{};C:\Users\acct-env-value\work",
                exe.parent().unwrap_or(exe).display()
            ),
            value_kind: RegistryValueKind::String,
            requires_admin: false,
            confidence: MatchConfidence::High,
        });
        report.file_inventory.push(FileEntry {
            path: PathBuf::from(r"C:\Users\acct-sys-file\AppData\Roaming\App\x.sys"),
            original_path: PathBuf::from(r"C:\Users\acct-sys-file\AppData\Roaming\App\x.sys"),
            package_relative_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::ManualRuntimeInstall,
            should_copy: false,
            reason: "Driver/kernel file".to_owned(),
            user_folder: None,
        });
        report.services.push(ServiceEntry {
            service_name: "ProbeSvc".to_owned(),
            display_name: None,
            binary_path: exe.to_path_buf(),
            arguments: None,
            parameters: Vec::new(),
            start_type: None,
            account_name: Some(r"PCNAME-SERVICE\acct-service".to_owned()),
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: r"sc create C:\Users\acct-service-command\a".to_owned(),
        });
        report.services.push(ServiceEntry {
            service_name: "ProbeSvcOfTheirs".to_owned(),
            display_name: None,
            binary_path: exe.to_path_buf(),
            arguments: Some(r"--data C:\Users\acct-service-args\work".to_owned()),
            parameters: Vec::new(),
            start_type: None,
            account_name: None,
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: String::new(),
        });
        report.warnings.push(Warning::new(
            "registry_key_unreadable",
            WarningSeverity::Warning,
            "A key could not be read.",
            Some(r"HKCU\Software\Acme\Sessions\acct-session-host".to_owned()),
        ));
        report.certificates.push(CertificateEntry {
            store: "TrustedPeople".to_owned(),
            hive: RegistryHive::CurrentUser,
            thumbprint: "AA".to_owned(),
            subject: "CN=acct-cert-subject, O=Acme".to_owned(),
            why: "named".to_owned(),
        });
    }

    fn what_says_who_it_was(root: &Path) -> Vec<String> {
        let mut found = Vec::new();
        let mut folders = vec![root.to_path_buf()];
        while let Some(folder) = folders.pop() {
            let Ok(entries) = fs::read_dir(&folder) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // What is in the app's own files is the app's, and here it is this test's program.
                    if entry.file_name() != FILES_DIR {
                        folders.push(path);
                    }
                    continue;
                }
                let text = fs::read(&path)
                    .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
                    .unwrap_or_default();
                for mark in WHO_IT_WAS {
                    if text.contains(mark) {
                        found.push(format!("{mark} in {}", path.display()));
                    }
                }
            }
        }
        found
    }

    #[test]
    fn a_package_with_none_of_an_accounts_data_says_nothing_of_who_it_was() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = PackageOptions {
            create_self_extracting_archive: false,
            ..installer_options(root.path(), &exe, &stub)
        };
        let mut report = scan_of(&options)?;
        plant_who_it_was(&mut report, &exe);
        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        assert_eq!(
            what_says_who_it_was(&result.package_root),
            Vec::<String>::new()
        );
        let manifest: PackageManifest = serde_json::from_slice(
            &fs::read(&result.manifest_path)
                .map_err(|error| WinAppMigratorError::io(&result.manifest_path, error))?,
        )
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        assert_eq!(manifest.shortcuts_detected.len(), 1);
        assert_eq!(
            manifest.shortcuts_detected[0].original_lnk_path,
            Path::new(r"%USERPROFILE%\Desktop\Plain.lnk")
        );
        assert_eq!(manifest.scheduled_tasks_detected.len(), 1);
        assert_eq!(
            manifest.scheduled_tasks_detected[0].account.as_deref(),
            Some(crate::restore_policy::WHOEVER_INSTALLS_IT)
        );
        let said = |name: &str| {
            manifest
                .settings_left_out
                .iter()
                .any(|item| item.name.contains(name))
        };
        assert!(said("Shortcuts"), "{:?}", manifest.settings_left_out);
        assert!(said("Probe"), "{:?}", manifest.settings_left_out);
        Ok(())
    }

    #[test]
    fn a_package_with_an_accounts_data_can_still_say_whose_it_is() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = PackageOptions {
            create_self_extracting_archive: false,
            include_user_data: true,
            ..installer_options(root.path(), &exe, &stub)
        };
        let mut report = scan_of(&options)?;
        plant_who_it_was(&mut report, &exe);
        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let found = what_says_who_it_was(&result.package_root);
        for mark in ["acct-lnk-path", "secret-doc-in-args", "acct-env-value"] {
            assert!(
                found.iter().any(|said| said.contains(mark)),
                "{mark}: {found:?}"
            );
        }
        assert!(
            !result
                .warnings
                .iter()
                .any(|warning| warning.code == "package_mentions_an_account"),
            "{:?}",
            result.warnings
        );
        Ok(())
    }

    #[test]
    fn what_a_build_cannot_change_it_points_out_and_goes_on() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = PackageOptions {
            create_self_extracting_archive: false,
            ..installer_options(root.path(), &exe, &stub)
        };
        let mut report = scan_of(&options)?;
        report.account_traces.account_names = vec!["acct-in-a-value".to_owned()];
        report.registry_entries.push(RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: r"Software\Acme".to_owned(),
            value_name: Some("Owner".to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_data: Some("acct-in-a-value".to_owned()),
            value_bytes_hex: None,
            requires_admin: true,
            confidence: MatchConfidence::High,
        });
        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let mention = result
            .warnings
            .iter()
            .find(|warning| warning.code == "package_mentions_an_account")
            .ok_or_else(|| WinAppMigratorError::Archive("it says nothing".to_owned()))?;
        assert!(
            mention.message.contains("registry values (1)"),
            "{mention:?}"
        );
        assert!(!mention.message.contains("acct-in-a-value"), "{mention:?}");
        assert!(
            fs::read_to_string(&result.manifest_path)
                .map_err(|error| WinAppMigratorError::io(&result.manifest_path, error))?
                .contains("acct-in-a-value"),
            "a value goes as it is"
        );
        Ok(())
    }

    #[test]
    fn a_key_put_in_a_file_since_the_scan_stays_out() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let folder = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .to_path_buf();
        let pem = folder.join(r"data\tls.pem");
        fs::write(&pem, "-----BEGIN CERTIFICATE-----\n")
            .map_err(|error| WinAppMigratorError::io(&pem, error))?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        assert!(
            report
                .file_inventory
                .iter()
                .any(|file| file.original_path == pem && file.should_copy),
            "a certificate alone is an ordinary file"
        );
        fs::write(&pem, "-----BEGIN PRIVATE KEY-----\nCANARY\n")
            .map_err(|error| WinAppMigratorError::io(&pem, error))?;

        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let installer = result
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::Archive("no installer".to_owned()))?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let manifest = read_embedded_manifest(&package)?;
        let digests = package.digests(&|| false)?;
        assert!(digests.get(Path::new(r"files\data\tls.pem")).is_none());
        assert!(
            digests.get(Path::new(r"files\data\a.txt")).is_some(),
            "the rest goes in"
        );
        assert!(
            manifest
                .secrets_left_out
                .iter()
                .any(|item| item.name.ends_with("tls.pem")),
            "{:?}",
            manifest.secrets_left_out
        );
        assert!(
            manifest
                .file_inventory
                .iter()
                .any(|file| file.original_path == pem && !file.should_copy),
            "the manifest does not say it is there"
        );
        Ok(())
    }

    #[test]
    fn a_license_file_that_holds_a_key_by_now_stays_out() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let license = root.path().join("license.key");
        let write = |text: &str| {
            fs::write(&license, text).map_err(|error| WinAppMigratorError::io(&license, error))
        };
        write("ORDINARY LICENSE TEXT")?;
        let out = root.path().join("out");
        let reader = SourceReader {
            volume: None,
            other_accounts: Default::default(),
        };
        let artifact = || LicenseArtifact {
            kind: crate::models::LicenseArtifactKind::File,
            framework: crate::models::LicenseFramework::FileBased,
            path: Some(license.clone()),
            registry_location: None,
            detail: String::new(),
            machine_bound: false,
            requires_admin: false,
            transferable: crate::models::TransferableHint::Likely,
            should_copy: true,
            package_relative_path: None,
            size_bytes: Some(21),
            hash_sha256: None,
            store_files: Vec::new(),
        };
        let mut warnings = Vec::new();
        assert!(package_one_license_artifact(
            &out,
            &mut artifact(),
            &reader,
            &mut warnings
        ));
        write("-----BEGIN PRIVATE KEY-----\nCANARY\n")?;
        assert!(!package_one_license_artifact(
            &out.join("again"),
            &mut artifact(),
            &reader,
            &mut warnings
        ));
        assert!(
            warnings
                .iter()
                .any(|warning| warning.code == "license_store_file_left_out"),
            "{warnings:?}"
        );
        Ok(())
    }

    #[test]
    fn a_key_file_that_has_gone_is_gone_rather_than_a_credential() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let folder = exe
            .parent()
            .ok_or_else(|| WinAppMigratorError::Archive("no folder".to_owned()))?
            .to_path_buf();
        let pem = folder.join(r"data\tls.pem");
        fs::write(&pem, "-----BEGIN CERTIFICATE-----\n")
            .map_err(|error| WinAppMigratorError::io(&pem, error))?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        fs::remove_file(&pem).map_err(|error| WinAppMigratorError::io(&pem, error))?;
        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.code == FILE_GONE_CODE),
            "{:?}",
            result.warnings
        );
        assert!(
            !result
                .warnings
                .iter()
                .any(|warning| warning.code == "secret_left_out"),
            "{:?}",
            result.warnings
        );
        Ok(())
    }

    #[test]
    fn a_scan_whose_program_has_gone_builds_nothing() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;
        fs::remove_file(&exe).map_err(|error| WinAppMigratorError::io(&exe, error))?;

        let built = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        );
        assert!(
            matches!(&built, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("not there any more")),
            "{built:?}"
        );
        assert!(!root.path().join("out").exists(), "and wrote nothing");
        Ok(())
    }

    #[test]
    fn a_scan_of_something_else_cannot_be_built_from() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let report = scan_of(&options)?;

        let other = root.path().join(r"other\fixture.exe");
        let elsewhere = PackageOptions {
            selection: AppSelection::ExecutablePath(other),
            ..options
        };
        let built = create_package_from_scan(
            &elsewhere,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        );
        assert!(
            matches!(&built, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("another app")),
            "{built:?}"
        );

        let installed = |name: &str, path: &str| {
            AppSelection::InstalledApp(Box::new(crate::models::InstalledApp {
                display_name: name.to_owned(),
                publisher: None,
                version: None,
                install_location: None,
                uninstall_string: None,
                main_executable: None,
                registry_source_path: path.to_owned(),
                architecture: AppArchitecture::X64,
                install_scope: crate::models::InstallScope::PerMachine,
                owner_profile: None,
            }))
        };
        let mut scanned = report.clone();
        scanned.source_app = Some(crate::models::InstalledApp {
            display_name: "One".to_owned(),
            publisher: None,
            version: None,
            install_location: None,
            uninstall_string: None,
            main_executable: None,
            registry_source_path: r"HKLM\One".to_owned(),
            architecture: AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        });
        assert!(is_scan_of(&scanned, &installed("One", r"hklm\one")));
        assert!(!is_scan_of(&scanned, &installed("Two", r"HKLM\One")));
        assert!(!is_scan_of(&scanned, &installed("One", r"HKLM\Two")));
        assert!(!is_scan_of(&scanned, &AppSelection::ExecutablePath(exe)));
        Ok(())
    }

    #[test]
    fn a_scan_can_be_stopped_before_it_starts() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let monitor = ScanMonitor::default();
        monitor.cancel();
        let scanned = scan_app(&scan_options_for(&options), &monitor);
        assert!(
            matches!(scanned, Err(WinAppMigratorError::Cancelled)),
            "{scanned:?}"
        );
        Ok(())
    }

    #[test]
    fn a_scan_stopped_part_way_goes_no_further_than_the_step_it_was_on() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let exe = app_in(root.path())?;
        let stub = plain_stub(root.path())?;
        let options = installer_options(root.path(), &exe, &stub);
        let seen: std::sync::Arc<std::sync::Mutex<Vec<ScanStep>>> = std::sync::Arc::default();
        let slot: std::sync::Arc<std::sync::Mutex<Option<ScanMonitor>>> = std::sync::Arc::default();
        let (sink, canceller) = (std::sync::Arc::clone(&seen), std::sync::Arc::clone(&slot));
        let monitor = ScanMonitor::new(move |step| {
            if let Ok(mut steps) = sink.lock() {
                steps.push(step);
            }
            if step == ScanStep::Files
                && let Ok(canceller) = canceller.lock()
                && let Some(monitor) = canceller.as_ref()
            {
                monitor.cancel();
            }
        });
        *slot
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))? =
            Some(monitor.clone());
        let scanned = scan_app(&scan_options_for(&options), &monitor);
        assert!(
            matches!(scanned, Err(WinAppMigratorError::Cancelled)),
            "{scanned:?}"
        );
        let steps = seen
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert_eq!(steps.last(), Some(&ScanStep::Files), "{steps:?}");
        assert!(!steps.contains(&ScanStep::Registry), "{steps:?}");
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
