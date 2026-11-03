use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::account_settings::{
    say_no_more_of_licenses_than_the_apps_key, say_no_more_than_the_apps_key, take_account_settings,
};
use crate::account_traces::{
    AccountTraces, Withholding, withhold_settings, withhold_what_is_an_accounts,
};
use crate::dll_resolver::{
    DllResolutionContext, VisualCppRuntime, find_app_local_dlls, resolve_imports,
};
use crate::env_scan::{EnvironmentScanContext, scan_environment};
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::DRIVER_FILE_NOT_COPIED_CODE;
use crate::file_discovery::{
    DiscoveredFiles, FileDiscoveryContext, discover_files_unless, names_of_app,
    package_relative_path,
};
use crate::install_source::{
    InstallSource, add_app_paths_roots, detect_install_source_unless, is_usable_main_executable,
    primary_executable, resolve_install_location, squirrel_current_executable,
};
use crate::integration_scan::{IntegrationInputs, OwnedFiles};
use crate::license_scan::{
    LicenseScanContext, is_license_env_var, is_license_file, scan_license_artifacts,
};
use crate::models::DependencySourceCategory;
use crate::models::{
    AppArchitecture, AppSelection, DependencyEntry, DotNetFlavor, DriverEntry, FileEntry,
    InstallScope, InstalledApp, ProgramNeed, ProgramRole, RuntimePrerequisite,
    RuntimePrerequisiteKind, SCAN_SCHEMA_VERSION, ScanOptions, ScanReport, ScheduledTaskEntry,
    ServiceEntry, ShortcutEntry, SourcePlatform, Warning, WarningSeverity,
};
use crate::near_match::{AppBeingScanned, NearMatchLook, Picks, look_for_near_matches};
use crate::path_utils::comparable_path;
use crate::path_utils::{
    is_drive_root, is_really_within, looks_like_executable, path_is_within, resolved_location,
};
use crate::pe_scan::{read_program_need, scan_pe};
use crate::progress::{ScanMonitor, ScanStep};
use crate::registry_scan::{
    RegistryScanContext, context_product_names, recorded_keys_of_one_account,
    scan_registry_artifacts,
};
use crate::reporting::artifact_paths;
use crate::restore_policy::architecture_from_name;
use crate::runtime_detection::{AppRuntimes, ArchitectureKey, DotNetNeed, inspect_app_runtimes};
use crate::secret_rules::{
    LeftOut, ScannedSettings, is_license_registry_entry, is_only_true_where_it_was_installed,
    names_who_ran_the_setup, take_inno_setup_logs, take_licenses_from_settings, take_secret_files,
    take_secrets_from_license_values, take_secrets_from_settings,
};
use crate::service_scan::{ServiceScanContext, scan_drivers, scan_services};
use crate::shortcut_scan::{ShortcutScanContext, scan_shortcuts};
use crate::system_context::SystemContext;
use crate::task_scan::{TaskScanContext, scan_scheduled_tasks};

const BASE_CONFIDENCE_SCORE: i32 = 100;
const MISSING_DLL_PENALTY: i32 = 8;
const RUNTIME_PREREQUISITE_PENALTY: i32 = 4;
const EXTERNAL_MANIFEST_PENALTY: i32 = 5;
const NON_EXE_EXTENSION_PENALTY: i32 = 3;
/// Managed .NET images carry GAC/framework dependencies the PE import table does
/// not list, so certainty is slightly reduced.
const MANAGED_DOTNET_PENALTY: i32 = 4;
/// Each declared side-by-side assembly dependency is serviced by WinSxS and may
/// need manual handling on the target.
const SXS_DEPENDENCY_PENALTY: i32 = 2;
const MAX_SXS_DEPENDENCY_PENALTY: i32 = 8;
const MIN_CONFIDENCE_SCORE: i32 = 0;
const MAX_CONFIDENCE_SCORE: i32 = 100;
/// How many of an app's programs to record what they need of. Its main program, its services, and
/// its drivers always are, and they come first.
const MOST_PROGRAMS_RECORDED: usize = 2000;

#[derive(Debug, Clone)]
struct ConfidenceInputs<'a> {
    dependencies: &'a [DependencyEntry],
    runtime_prerequisite_count: usize,
    external_manifest_detected: bool,
    non_exe_extension: bool,
    managed_dotnet: bool,
    sxs_dependency_count: usize,
    service_count: usize,
    driver_count: usize,
    scheduled_task_count: usize,
    admin_registry_count: usize,
    files_not_copied_count: usize,
}

/// What each of an app's programs needs from Windows, from the headers of each. Its main program,
/// its services, and the drivers it owns are what it cannot do without, so they come first. By
/// where each is read from now and by what its PC called it, since on a drive those differ.
fn read_program_needs_of(
    files: &[FileEntry],
    main_program: &Path,
    services: &[ServiceEntry],
    drivers: &[DriverEntry],
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> Vec<ProgramNeed> {
    let mut needs = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut note = |readable: &Path, logical: &Path, role: ProgramRole| {
        if !seen.insert(logical.to_string_lossy().to_ascii_lowercase()) {
            return;
        }
        let need = system
            .may_read(readable)
            .then(|| read_program_need(readable, logical, role))
            .flatten();
        // A helper that is not really a program is nothing to say. What the app cannot do without is.
        if need.is_none() && role != ProgramRole::Other {
            warnings.push(Warning::new(
                "program_needs_unreadable",
                WarningSeverity::Warning,
                "What this needs from Windows could not be read, so whether it can run on the other \
                 PC will not be checked.",
                Some(logical.display().to_string()),
            ));
        }
        needs.extend(need);
    };
    let copied = |logical: &Path| {
        files
            .iter()
            .find(|file| file.should_copy && file.original_path.as_path() == logical)
            .map(|file| file.path.clone())
    };
    if let Some(readable) =
        copied(main_program).or_else(|| system.resolve_source_path(main_program))
    {
        note(&readable, main_program, ProgramRole::Main);
    }
    for service in services {
        if let Some(readable) = copied(&service.binary_path) {
            note(&readable, &service.binary_path, ProgramRole::Service);
        }
    }
    for driver in drivers.iter().filter(|driver| driver.app_owned) {
        if let Some(readable) = system.resolve_source_path(&driver.image_path) {
            note(&readable, &driver.image_path, ProgramRole::Driver);
        }
    }
    let others = files
        .iter()
        .filter(|file| file.should_copy && looks_like_executable(&file.original_path))
        .take(MOST_PROGRAMS_RECORDED);
    for file in others {
        note(&file.path, &file.original_path, ProgramRole::Other);
    }
    needs
}

/// Find out everything about an app that a build goes by, and write nothing anywhere. It stops with
/// [`WinAppMigratorError::Cancelled`] between steps, and part-way through going over its files and
/// its install record, if `monitor` is cancelled.
///
/// A report says how the PC was when it was looked at. On a connected drive it holds what can be
/// read without its hives, which do not stay loaded after this.
pub fn scan_app(options: &ScanOptions, monitor: &ScanMonitor) -> Result<ScanReport> {
    let is_cancelled = || monitor.is_cancelled();
    monitor.report(ScanStep::InstallRecord);
    // The single seam every scanner reads through: the live OS, or a mounted
    // offline volume from another PC. Held for the whole scan so offline hive
    // mounts stay live until every scanner has run. It reads one account, so another's cannot
    // come along by accident.
    let owner = match &options.selection {
        AppSelection::InstalledApp(app) => app.owner_profile.as_deref(),
        AppSelection::ExecutablePath(_) => None,
    };
    let system = SystemContext::open_for_scan(
        &options.source,
        &options.user_profile,
        owner,
        monitor.cancel_flag(),
    )?;
    let mut source_app = match &options.selection {
        AppSelection::InstalledApp(app) => Some((**app).clone()),
        AppSelection::ExecutablePath(_) => None,
    };
    // A selection came from a listing, and is asked about like anything else on a drive.
    if let Some(app) = source_app.as_mut() {
        app.install_location = app
            .install_location
            .take()
            .filter(|location| system.may_read(location));
        app.main_executable = app
            .main_executable
            .take()
            .filter(|executable| system.may_read(executable));
    }
    // Reverse-engineer how the app was installed; for MSI products this reads the
    // authoritative Windows Installer component database. That is slow, so it runs
    // once, here, and everything that needs the result is passed it.
    let mut install_source = source_app
        .as_ref()
        .map(|app| detect_install_source_unless(app, &system, &is_cancelled));
    monitor.checkpoint()?;
    let executable_path = selected_executable_path(options, install_source.as_ref(), &system)?;
    let executable_path = install_source
        .as_ref()
        .and_then(|source| squirrel_current_executable(&executable_path, source, &system))
        .unwrap_or(executable_path);
    // Whatever led here, on a drive nothing is read through a link or from off the drive.
    if !system.may_read(&executable_path) {
        let why = if system.is_offline() {
            "goes through a link or is not on the connected drive"
        } else {
            "is not on a disk of this PC, or has a name that would open something else"
        };
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} {why}, so it was not read.",
            executable_path.display()
        )));
    }
    // What an app is allowed to have in another account's folders is what its machine-wide record
    // says. An uninstall entry in an account's own registry can be written with no rights at all,
    // and so can a shortcut, and either could otherwise name someone else's folder as the app, for
    // an administrator to package.
    let installed_for_one_account = source_app
        .as_ref()
        .is_some_and(|app| app.install_scope == InstallScope::PerUser);
    let machine_record = source_app
        .as_ref()
        .filter(|_| !installed_for_one_account)
        .and_then(|app| app.install_location.clone())
        // As it is written, with no link on the way. On `C:\` any account may be able to make the
        // folder it names, and make that a link to someone else's.
        .filter(|recorded| {
            let real = resolved_location(recorded);
            path_is_within(&real, recorded) && path_is_within(recorded, &real)
        });
    let is_someone_elses = |path: &Path| {
        let is_where_its_record_says = machine_record
            .as_deref()
            .is_some_and(|recorded| is_really_within(path, recorded));
        source_app.is_some()
            && !is_where_its_record_says
            && (system.is_in_another_profile(path)
                || (installed_for_one_account && !system.is_the_accounts_own(path)))
    };
    // Its folder too, all of which is taken with it.
    if is_someone_elses(&executable_path) || executable_path.parent().is_some_and(is_someone_elses)
    {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} is in another account's folders or is not this account's, and this app's own \
             record does not say it belongs there, so it was not read.",
            executable_path.display()
        )));
    }
    validate_scan_input(&executable_path)?;
    if let (Some(app), Some(source)) = (source_app.as_ref(), install_source.as_mut())
        && app.main_executable.is_none()
    {
        add_app_paths_roots(source, &executable_path, &system);
    }

    let mut pe_scan = scan_pe(&executable_path)?;
    if pe_scan.metadata.external_manifest.is_some() {
        pe_scan.warnings.push(Warning::new(
            "external_manifest_detected",
            WarningSeverity::Warning,
            "External application manifest detected; side-by-side dependency interpretation is deferred.",
            pe_scan
                .metadata
                .external_manifest
                .as_ref()
                .map(|path| path.display().to_string()),
        ));
    }
    if !looks_like_executable(&executable_path) {
        pe_scan.warnings.push(Warning::new(
            "non_exe_extension",
            WarningSeverity::Info,
            "Selected file does not use the .exe extension; PE scanning will continue.",
            Some(executable_path.display().to_string()),
        ));
    }
    if source_app.is_some() && !is_usable_main_executable(&executable_path, &system) {
        pe_scan.warnings.push(Warning::new(
            "main_executable_not_the_app",
            WarningSeverity::Warning,
            "No ordinary application executable was found, so this file stands in for it. Architecture and dependency results describe this file, and may not describe the application.",
            Some(executable_path.display().to_string()),
        ));
    }

    let context = DllResolutionContext {
        executable_path: executable_path.clone(),
        app_architecture: pe_scan.architecture.clone(),
        current_directory: options.current_directory.clone(),
        recursive: options.recursive_dependencies,
        include_hashes: options.include_hashes,
    };
    let (dependencies, resolver_warnings) =
        resolve_imports(&pe_scan.direct_imports, &context, &system);
    let app_local_dlls = find_app_local_dlls(&executable_path, &system)?;
    let install_location = match (source_app.as_ref(), install_source.as_ref()) {
        (Some(app), Some(source)) => resolve_install_location(app, &executable_path, source),
        _ => executable_path
            .parent()
            .filter(|parent| !is_drive_root(parent))
            .map(Path::to_path_buf),
    }
    .filter(|location| !is_someone_elses(location));
    let recorded_location = source_app
        .as_ref()
        .and_then(|app| app.install_location.clone())
        .filter(|location| !is_someone_elses(location));
    // What the report and the installer work from: files are stored relative to this
    // location, so the installer has to offer the same one.
    if let Some(app) = source_app.as_mut() {
        app.install_location.clone_from(&install_location);
        app.main_executable = Some(executable_path.clone());
    }
    let app_name = source_app.as_ref().map(|app| app.display_name.clone());
    let publisher = source_app.as_ref().and_then(|app| app.publisher.clone());
    let source_registry_path = source_app
        .as_ref()
        .map(|app| app.registry_source_path.clone());
    // What the PC itself calls these. On a drive they are somewhere else now, and everything on
    // it that mentions them still means where they were. Disks are read by where they are now,
    // and what a PC recorded is compared with this.
    let logical_executable = system.logical_path(&executable_path);
    let logical_location = install_location
        .as_deref()
        .map(|location| system.logical_path(location));

    // What exact names leave out, and what a person ticked of that, by the rules a scan goes by
    // from here on. Before anything is read by where the app is, since this adds to that.
    monitor.report(ScanStep::NearMatches);
    let near_names = names_of_app(
        Some(&executable_path),
        install_location.as_deref(),
        app_name.as_deref(),
        publisher.as_deref(),
    );
    let (near_look, mut picks) = match source_app.as_ref() {
        Some(app) => {
            let scan = AppBeingScanned {
                app,
                install_location: install_location.as_deref(),
                names: &near_names,
                installed_for_one_account,
            };
            let look = look_for_near_matches(&scan, &system, &is_cancelled);
            let picks = look.pick(&options.near_matches, &scan, &system, &is_cancelled);
            (look, picks)
        }
        None => (NearMatchLook::default(), Picks::default()),
    };
    monitor.checkpoint()?;
    let also_the_apps = std::mem::take(&mut picks.also_the_apps);

    monitor.report(ScanStep::SystemItems);
    let (mut services, service_warnings) = scan_services(
        &ServiceScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
            also_the_apps: also_the_apps.clone(),
        },
        &system,
    );
    let (drivers, driver_warnings) = scan_drivers(
        &ServiceScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
            also_the_apps: also_the_apps.clone(),
        },
        &system,
    );
    let (mut scheduled_tasks, task_warnings) = scan_scheduled_tasks(
        &TaskScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
            also_the_apps: also_the_apps.clone(),
        },
        &system,
    );
    monitor.checkpoint()?;
    let (mut shortcuts, shortcut_warnings) = scan_shortcuts(
        &ShortcutScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
            also_the_apps: also_the_apps.clone(),
        },
        &system,
    );
    let (mut environment_variables, environment_warnings) = scan_environment(
        &EnvironmentScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
            also_the_apps: also_the_apps.clone(),
        },
        &system,
    );
    monitor.checkpoint()?;

    // Fold concrete locations referenced by Windows integrations into the
    // footprint discovery so split installs (Program Files plus a service
    // binary or a ProgramData data tree) are captured automatically.
    let mut extra_roots: Vec<PathBuf> = collect_extra_roots(
        source_app.as_ref(),
        recorded_location.as_deref(),
        &services,
        &scheduled_tasks,
        &shortcuts,
    )
    .iter()
    .filter_map(|root| system.resolve_source_path(root))
    .collect();
    if let Some(source) = install_source.as_mut() {
        // What an install record says for an app of one account's is a file in that account's own
        // folder, which could say anything. It goes no further than the app and that account.
        if installed_for_one_account {
            let stays = |path: &PathBuf| {
                system.may_read(path)
                    && (install_location
                        .as_deref()
                        .is_some_and(|location| is_really_within(path, location))
                        || system.is_in_the_accounts_profile(path))
            };
            source.roots.retain(stays);
            source.loose_files.retain(stays);
        }
        extra_roots.extend(source.roots.iter().cloned());
    }
    extra_roots.append(&mut picks.roots);
    monitor.report(ScanStep::Files);
    let DiscoveredFiles {
        files: mut file_inventory,
        empty_folders,
        locations: footprint_locations,
        warnings: file_warnings,
    } = discover_files_unless(
        &FileDiscoveryContext {
            executable_path: executable_path.clone(),
            install_location: install_location.clone(),
            app_name: app_name.clone(),
            publisher: publisher.clone(),
            include_hashes: options.include_hashes,
            include_user_data: options.include_user_data,
            include_program_data: options.include_program_data,
            installed_for_one_account,
            extra_roots,
            extra_files: install_source
                .as_ref()
                .map(|source| source.loose_files.clone())
                .unwrap_or_default()
                .into_iter()
                .chain(std::mem::take(&mut picks.loose_files))
                .collect(),
        },
        &system,
        &is_cancelled,
    );
    monitor.checkpoint()?;
    let near_matches = near_look.rows(&footprint_locations, &picks, &system);

    // What an entry in the registry points at says whose it is, so this comes after what the app has
    // is known. As its PC calls them, which is what those values say.
    let registry_context = RegistryScanContext {
        executable_path: logical_executable.clone(),
        install_location: logical_location.clone(),
        display_name: app_name.clone(),
        publisher: publisher.clone(),
        source_registry_path,
        recorded_keys: install_source
            .as_ref()
            .map(|source| source.registry_keys.clone())
            .map(|keys| {
                if installed_for_one_account {
                    recorded_keys_of_one_account(keys)
                } else {
                    keys
                }
            })
            .unwrap_or_default()
            .into_iter()
            .chain(std::mem::take(&mut picks.recorded_keys))
            .collect(),
        installed_for_one_account,
        also_the_apps,
        also_named: std::mem::take(&mut picks.also_named),
        not_named: std::mem::take(&mut picks.not_named),
    };
    monitor.report(ScanStep::Registry);
    let names = context_product_names(&registry_context);
    let windows_dir = system.logical_path(&system.windows_dir());
    let owned_files = OwnedFiles::new(
        &logical_executable,
        logical_location.as_deref(),
        &windows_dir,
        file_inventory
            .iter()
            .filter(|file| file.should_copy)
            .map(|file| system.logical_path(&file.path)),
    );
    // One account cannot put a font in the Windows folder, whatever its own records say.
    let recorded_files: Vec<PathBuf> = install_source
        .as_ref()
        .filter(|_| !installed_for_one_account)
        .map(|source| source.recorded_files.clone())
        .unwrap_or_default()
        .iter()
        .map(|file| system.logical_path(file))
        .collect();
    let mut registry_artifacts = scan_registry_artifacts(
        &registry_context,
        &IntegrationInputs {
            owned: &owned_files,
            names: &names,
            display_name: registry_context.display_name.as_deref(),
            publisher: registry_context.publisher.as_deref(),
            recorded_files: &recorded_files,
        },
        &system,
    );
    monitor.checkpoint()?;
    monitor.report(ScanStep::Finishing);
    // A font in the Windows folder the app's install record names is copied like any of its files.
    for mut font in std::mem::take(&mut registry_artifacts.font_files) {
        font.package_relative_path = Some(package_relative_path(
            &font.original_path,
            install_location.as_deref(),
        ));
        file_inventory.push(font);
    }

    let app_runtimes = inspect_app_runtimes(
        &file_inventory,
        &executable_path,
        install_location.as_deref(),
        &pe_scan.architecture,
        &system,
    );
    let (runtime_prerequisites, runtime_warnings) = runtime_prerequisites(
        &app_runtimes,
        &dependencies,
        &app_local_dlls,
        &file_inventory,
        &pe_scan.architecture,
        system.is_offline(),
    );
    monitor.checkpoint()?;
    let account_traces = AccountTraces::of_scan(&system);
    let mut left_out = LeftOut::default();
    // First, so that nothing else counts one as a license, or anything else that would be copied.
    take_secret_files(&mut file_inventory, &system, &mut left_out);
    let left_inno_logs_out = take_inno_setup_logs(&mut file_inventory);
    monitor.checkpoint()?;
    let (mut license_artifacts, mut license_registry_entries, license_warnings) =
        scan_license_artifacts(
            &LicenseScanContext {
                publisher: source_app.as_ref().and_then(|app| app.publisher.as_deref()),
                discovered_files: &file_inventory,
                registry_entries: &registry_artifacts.entries,
                names: &names,
                executable: &logical_executable,
                install_location: logical_location.as_deref(),
                include_license_data: options.include_license_data,
                include_hashes: options.include_hashes,
            },
            &system,
            &mut left_out,
        );
    // What is license data goes by the License option, and nowhere else: a value, a file, or a
    // variable that was captured to be restored as anything else would not.
    registry_artifacts.entries.retain(|entry| {
        !is_license_registry_entry(entry, &names)
            && !names_who_ran_the_setup(entry)
            && !is_only_true_where_it_was_installed(entry)
    });
    file_inventory.retain(|file| !is_license_file(&file.original_path));
    // After what is a license has gone its own way, which is the License option's to say, and before
    // anything names what it finds, since a key's name can be what an account was keeping.
    if !options.include_user_data {
        take_account_settings(
            &mut registry_artifacts.entries,
            &registry_context.folders_of_the_app(),
            &|text| system.expand_source_variables(text),
            &mut left_out,
        );
        say_no_more_than_the_apps_key(&mut registry_artifacts.warnings);
        if !options.include_license_data {
            say_no_more_of_licenses_than_the_apps_key(
                &mut license_artifacts,
                &mut left_out,
                &|path| system.is_in_the_accounts_profile(path),
            );
        }
    }
    if !options.include_license_data {
        environment_variables.retain(|variable| {
            let is_a_license = is_license_env_var(&variable.name);
            if is_a_license {
                left_out.license(format!("The environment variable {}", variable.name));
            }
            !is_a_license
        });
    }
    // Before what looks like a credential is named, since what names one goes by these.
    if !options.include_user_data {
        withhold_settings(
            &mut Withholding {
                shortcuts: &mut shortcuts,
                tasks: &mut scheduled_tasks,
                app_folders: &registry_context.folders_of_the_app(),
                main_program: &logical_executable,
                installed_for_one_account,
                traces: &account_traces,
            },
            &mut left_out,
        );
    }
    let mut settings = ScannedSettings {
        registry: &mut registry_artifacts.entries,
        services: &mut services,
        tasks: &mut scheduled_tasks,
        shortcuts: &mut shortcuts,
        environment: &mut environment_variables,
        file_associations: &mut registry_artifacts.file_associations,
        com_registrations: &mut registry_artifacts.com_registrations,
        firewall_rules: &mut registry_artifacts.firewall_rules,
        source_app: source_app.as_mut(),
    };
    if !options.include_license_data {
        take_licenses_from_settings(&mut settings, &names, &mut left_out);
    }
    // And what looks like a credential goes nowhere at all, whatever else was asked for.
    take_secrets_from_settings(&mut settings, &mut left_out);
    take_secrets_from_license_values(&mut license_registry_entries, &mut left_out);
    let account_traces_of_the_scan = account_traces;
    let mut warnings = system.warnings();
    warnings.append(&mut picks.warnings);
    warnings.extend(pe_scan.warnings);
    warnings.extend(resolver_warnings);
    warnings.extend(file_warnings);
    warnings.extend(registry_artifacts.warnings.clone());
    warnings.extend(service_warnings);
    warnings.extend(driver_warnings);
    warnings.extend(task_warnings);
    warnings.extend(shortcut_warnings);
    warnings.extend(environment_warnings);
    warnings.extend(runtime_warnings);
    warnings.extend(missing_dependency_warnings(&dependencies));
    warnings.extend(driver_component_warnings(&drivers));
    warnings.extend(license_warnings);
    warnings.extend(left_out_warnings(&left_out));
    say_which_driver_files_stay(&mut warnings, &drivers);

    let artifacts = artifact_paths(&options.output_dir);

    let confidence_score = calculate_confidence_score(ConfidenceInputs {
        dependencies: &dependencies,
        runtime_prerequisite_count: runtime_prerequisites.len(),
        external_manifest_detected: pe_scan.metadata.external_manifest.is_some(),
        non_exe_extension: !looks_like_executable(&executable_path),
        managed_dotnet: pe_scan.metadata.is_dotnet,
        sxs_dependency_count: pe_scan.metadata.sxs_dependencies.len(),
        service_count: services.len(),
        driver_count: drivers.len(),
        scheduled_task_count: scheduled_tasks.len(),
        admin_registry_count: registry_artifacts
            .entries
            .iter()
            .filter(|entry| entry.requires_admin)
            .count(),
        files_not_copied_count: file_inventory
            .iter()
            .filter(|entry| !entry.should_copy)
            .count(),
    });

    let identity = system.source_identity();
    let source_platform = Some(SourcePlatform {
        windows_build: identity.windows_build,
        architecture: identity
            .architecture_name
            .as_deref()
            .map_or(AppArchitecture::Unknown, architecture_from_name),
    });
    let program_needs = read_program_needs_of(
        &file_inventory,
        &logical_executable,
        &services,
        &drivers,
        &system,
        &mut warnings,
    );
    // A package with none of an account's data has no reason to carry its name, or the PC's.
    let source_computer_name = options
        .include_user_data
        .then_some(identity.computer_name)
        .flatten();
    let user_data_profile = options
        .include_user_data
        .then(|| system.user_hives().into_iter().next().map(|user| user.name))
        .flatten();
    let mut report = ScanReport {
        schema_version: SCAN_SCHEMA_VERSION.to_owned(),
        generated_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        source_volume: system.volume_map().cloned(),
        other_accounts: system.other_accounts_folders(
            install_location.as_deref(),
            file_inventory
                .iter()
                .filter(|file| file.should_copy)
                .map(|file| file.path.as_path())
                .chain(drivers.iter().map(|driver| driver.image_path.as_path()))
                .chain(
                    license_artifacts
                        .iter()
                        .filter_map(|artifact| artifact.path.as_deref()),
                ),
        ),
        source_computer_name,
        source_windows_version: identity.windows_version,
        source_platform,
        program_needs,
        user_data_profile,
        source_app,
        main_executable: pe_scan.metadata,
        architecture: pe_scan.architecture,
        install_location,
        install_method: install_source.as_ref().map(|source| source.method.clone()),
        authoritative_file_count: install_source
            .as_ref()
            .map(|source| source.authoritative_file_count)
            .unwrap_or(0),
        install_evidence: install_source
            .as_ref()
            .map(|source| source.evidence.clone())
            .unwrap_or_default(),
        footprint_locations,
        file_inventory,
        empty_folders,
        dependencies,
        app_local_dlls,
        registry_entries: registry_artifacts.entries,
        com_registrations: registry_artifacts.com_registrations,
        file_associations: registry_artifacts.file_associations,
        services,
        drivers,
        scheduled_tasks,
        shortcuts,
        environment_variables,
        runtime_prerequisites,
        license_artifacts,
        license_registry_entries,
        secrets_left_out: left_out.secrets,
        licenses_left_out: left_out.licenses,
        settings_left_out: left_out.settings,
        firewall_rules: registry_artifacts.firewall_rules,
        certificates: registry_artifacts.certificates,
        fonts_reported: registry_artifacts.fonts_reported,
        near_matches,
        account_traces: account_traces_of_the_scan,
        warnings,
        items_excluded: vec![
            "Windows directory, System32, and SysWOW64 files are reported but not copied."
                .to_owned(),
            "Shared runtimes are reported as prerequisites, not copied. Folders outside the install records are matched by the product's exact name, so a folder the app uses under another name is only there if it was ticked when this was built."
                .to_owned(),
            "Registry, services, scheduled tasks, and shortcuts are archived for restore; apply mode remains explicit and privilege-gated."
                .to_owned(),
            "Application-owned drivers are bundled and re-registered automatically; vendor/inbox drivers are reported with precise reinstall guidance and are not redistributed."
                .to_owned(),
        ],
        confidence_score,
        generated_artifacts: Some(artifacts),
    };

    if left_inno_logs_out {
        report
            .items_excluded
            .push(INNO_SETUP_LOG_LEFT_OUT.to_owned());
    }
    use_what_the_source_pc_calls_things(&mut report, &system);
    // With what it is called where it came from, as what is said of each goes by that.
    if !options.include_user_data {
        withhold_what_is_an_accounts(&mut report, options.include_license_data);
    }
    Ok(report)
}

const INNO_SETUP_LOG_LEFT_OUT: &str = "Inno Setup's uninstall log (unins000.dat) stays behind: it names the PC and the account that ran the setup, and lists where things were on that PC. The app's entry in Settings removes it another way.";

/// Everything found on a drive so far is where it is now. All that goes on from here is about the
/// PC it came from, whose paths are in its registry, and what an installer offers and rewrites.
/// So it goes by what that PC called them, in this one place. Only a file's `path` still says
/// where it can be read.
fn use_what_the_source_pc_calls_things(report: &mut ScanReport, system: &SystemContext) {
    if !system.is_offline() {
        return;
    }
    let logical = |path: &Path| system.logical_path(path);
    let install_location = report.install_location.as_deref().map(logical);
    if let Some(app) = report.source_app.as_mut() {
        app.install_location = app.install_location.as_deref().map(logical);
        app.main_executable = app.main_executable.as_deref().map(logical);
    }
    report.main_executable.path = logical(&report.main_executable.path);
    report.main_executable.external_manifest = report
        .main_executable
        .external_manifest
        .as_deref()
        .map(logical);
    for location in &mut report.footprint_locations {
        location.path = logical(&location.path);
    }
    for dependency in &mut report.dependencies {
        dependency.resolved_path = dependency.resolved_path.as_deref().map(logical);
    }
    for file in &mut report.app_local_dlls {
        file.original_path = logical(&file.original_path);
    }
    for file in &mut report.file_inventory {
        file.original_path = logical(&file.path);
        if file.package_relative_path.is_some() {
            file.package_relative_path = Some(package_relative_path(
                &file.original_path,
                install_location.as_deref(),
            ));
        }
    }
    for folder in &mut report.empty_folders {
        folder.original_path = logical(&folder.path);
        folder.package_relative_path = Some(package_relative_path(
            &folder.original_path,
            install_location.as_deref(),
        ));
    }
    for artifact in &mut report.license_artifacts {
        artifact.path = artifact.path.as_deref().map(logical);
    }
    report.install_location = install_location;
}

fn selected_executable_path(
    options: &ScanOptions,
    install_source: Option<&InstallSource>,
    system: &SystemContext,
) -> Result<PathBuf> {
    match &options.selection {
        AppSelection::ExecutablePath(path) => Ok(path.clone()),
        AppSelection::InstalledApp(app) => app
            .main_executable
            .clone()
            .or_else(|| install_source.and_then(|source| primary_executable(app, source, system)))
            .ok_or_else(|| {
                WinAppMigratorError::InvalidInput(format!(
                    "could not find an executable for {}. A driver, runtime, or add-in may have no program of its own to package.",
                    app.display_name
                ))
            }),
    }
}

fn validate_scan_input(executable_path: &Path) -> Result<()> {
    let metadata = fs::metadata(executable_path)
        .map_err(|source| WinAppMigratorError::io(executable_path, source))?;
    if !metadata.is_file() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "scan target is not a file: {}",
            executable_path.display()
        )));
    }
    Ok(())
}

/// `recorded_location` is what the uninstall entry says, which stays part of the app even
/// when the executable turned out to live elsewhere and `source_app` now names that.
fn collect_extra_roots(
    source_app: Option<&InstalledApp>,
    recorded_location: Option<&Path>,
    services: &[ServiceEntry],
    tasks: &[ScheduledTaskEntry],
    shortcuts: &[ShortcutEntry],
) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for service in services {
        push_parent_dir(&mut roots, &service.binary_path);
    }
    for task in tasks {
        push_parent_dir(&mut roots, &task.action_command);
    }
    for shortcut in shortcuts {
        if shortcut.target_is_selected_app {
            push_parent_dir(&mut roots, &shortcut.target_path);
        }
    }
    roots.extend(recorded_location.map(Path::to_path_buf));
    if let Some(app) = source_app {
        if let Some(location) = app.install_location.as_ref() {
            roots.push(location.clone());
        }
        if let Some(executable) = app.main_executable.as_ref() {
            push_parent_dir(&mut roots, executable);
        }
    }
    roots.sort();
    roots.dedup();
    roots
}

fn push_parent_dir(roots: &mut Vec<PathBuf>, path: &Path) {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        roots.push(parent.to_path_buf());
    }
}

/// What the app's programs were found to need, as what an installer can act on, with a warning
/// naming each one nothing can be done about here.
fn runtime_prerequisites(
    runtimes: &AppRuntimes,
    dependencies: &[DependencyEntry],
    app_local_dlls: &[FileEntry],
    file_inventory: &[FileEntry],
    app_architecture: &AppArchitecture,
    offline: bool,
) -> (Vec<RuntimePrerequisite>, Vec<Warning>) {
    let signals = collect_runtime_signal_names(dependencies, app_local_dlls, file_inventory);
    let mut prerequisites = runtime_prerequisites_from_signals(&signals, runtimes);
    let mut warnings = Vec::new();
    for dll in &runtimes.debug_dlls {
        warnings.push(Warning::new(
            "runtime_is_a_debug_build",
            WarningSeverity::Warning,
            format!(
                "A program needs {dll}, a debug build of the Visual C++ runtime. Microsoft has no \
                 installer for those, and may not let them be passed on, so it will not run on a \
                 PC without Visual Studio."
            ),
            Some(dll.clone()),
        ));
    }
    for program in &runtimes.single_file_programs {
        warnings.push(Warning::new(
            "runtime_of_a_single_file_program_unknown",
            WarningSeverity::Warning,
            format!(
                "{program} is a .NET program packed into one file, which says nothing of which .NET \
                 it needs, so none is planned for it. If it needs one, it will not start on a PC \
                 without it."
            ),
            Some(program.clone()),
        ));
    }
    if runtimes.programs_left_out > 0 {
        warnings.push(Warning::new(
            "runtime_inspection_limited",
            WarningSeverity::Info,
            format!(
                "{} of the app's programs were not looked at for the runtimes they need, as it has \
                 so many or they are so large.",
                runtimes.programs_left_out
            ),
            None,
        ));
    }
    // Attach a presence probe and a source-presence reading where one is known,
    // so packaging and the installer can confirm the runtime before and after
    // installing it. `is_present` reads the *live* registry, so it only reflects
    // the source when scanning the live machine; for an offline disk we leave
    // the source presence unknown rather than reporting this PC's runtimes.
    for prerequisite in &mut prerequisites {
        let architecture = prerequisite
            .architecture
            .clone()
            .unwrap_or_else(|| app_architecture.clone());
        if let Some(probe) = crate::prerequisites::presence_probe(&prerequisite.kind, &architecture)
        {
            if !offline {
                prerequisite.present_on_source = Some(crate::prerequisites::is_present(&probe));
            }
            prerequisite.presence_probe = Some(probe);
        }
        if prerequisite.install_manually
            && crate::prerequisites::spec(&prerequisite.kind, &architecture).is_none()
        {
            warnings.push(Warning::new(
                "runtime_has_no_installer",
                WarningSeverity::Warning,
                format!(
                    "The app needs {}, and there is no installer for it here, so none is run. \
                     Install it yourself on the other PC.",
                    prerequisite.name
                ),
                Some(prerequisite.evidence.clone()),
            ));
        }
    }
    (prerequisites, warnings)
}

/// Lowercased leaf file names that signal a bundled or required runtime:
/// imported DLL names, app-local DLLs, and the discovered install footprint.
fn collect_runtime_signal_names(
    dependencies: &[DependencyEntry],
    app_local_dlls: &[FileEntry],
    file_inventory: &[FileEntry],
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for dependency in dependencies {
        names.insert(dependency.name.to_ascii_lowercase());
    }
    for file in app_local_dlls.iter().chain(file_inventory.iter()) {
        if let Some(leaf) = file
            .original_path
            .file_name()
            .and_then(|name| name.to_str())
        {
            names.insert(leaf.to_ascii_lowercase());
        }
    }
    names
}

fn build_label(key: ArchitectureKey) -> &'static str {
    match key {
        ArchitectureKey::X86 => "x86",
        ArchitectureKey::X64 => "x64",
        ArchitectureKey::Arm64 => "Arm64",
        ArchitectureKey::Unknown => "any build",
    }
}

/// A Visual C++ runtime as a prerequisite. Those from before 2005 have no redistributable anyone
/// can still get, so they are `Unknown`, which nothing installs.
fn visual_cpp_prerequisite(
    runtime: VisualCppRuntime,
    build: ArchitectureKey,
    evidence: &str,
) -> RuntimePrerequisite {
    let (year, kind) = match runtime {
        VisualCppRuntime::Vs2002 => ("2002", RuntimePrerequisiteKind::Unknown),
        VisualCppRuntime::Vs2003 => ("2003", RuntimePrerequisiteKind::Unknown),
        VisualCppRuntime::Vs2005 => ("2005", RuntimePrerequisiteKind::VisualCpp2005),
        VisualCppRuntime::Vs2008 => ("2008", RuntimePrerequisiteKind::VisualCpp2008),
        VisualCppRuntime::Vs2010 => ("2010", RuntimePrerequisiteKind::VisualCpp2010),
        VisualCppRuntime::Vs2012 => ("2012", RuntimePrerequisiteKind::VisualCpp2012),
        VisualCppRuntime::Vs2013 => ("2013", RuntimePrerequisiteKind::VisualCpp2013),
        VisualCppRuntime::Vs2015Plus => {
            ("2015+", RuntimePrerequisiteKind::VisualCppRedistributable)
        }
    };
    RuntimePrerequisite {
        name: format!(
            "Microsoft Visual C++ {year} Redistributable ({})",
            build_label(build)
        ),
        kind,
        evidence: evidence.to_owned(),
        install_manually: true,
        present_on_source: None,
        bundle: None,
        presence_probe: None,
        architecture: Some(build.architecture()),
    }
}

/// A .NET as a prerequisite.
fn dotnet_prerequisite(
    need: DotNetNeed,
    build: ArchitectureKey,
    evidence: &str,
) -> RuntimePrerequisite {
    let of_a_build = |name: String| format!("{name} ({})", build_label(build));
    let (name, kind, install_manually) = match need {
        DotNetNeed::Framework35 => (
            "Microsoft .NET Framework 3.5".to_owned(),
            RuntimePrerequisiteKind::DotNetFramework35,
            true,
        ),
        DotNetNeed::Framework4x => (
            "Microsoft .NET Framework 4.x".to_owned(),
            RuntimePrerequisiteKind::DotNetFramework4x,
            false,
        ),
        DotNetNeed::Modern { flavor, major } => {
            let label = match flavor {
                DotNetFlavor::Core => format!(".NET {major} Runtime"),
                DotNetFlavor::Desktop => format!(".NET {major} Desktop Runtime"),
                DotNetFlavor::AspNetCore => format!("ASP.NET Core {major} Runtime"),
            };
            (
                of_a_build(label),
                RuntimePrerequisiteKind::DotNetRuntime { flavor, major },
                true,
            )
        }
        DotNetNeed::ModernOfNoKnownVersion => (
            ".NET, of a version the app does not say".to_owned(),
            RuntimePrerequisiteKind::DotNetDesktopRuntime,
            true,
        ),
        DotNetNeed::SelfContained => (
            ".NET runtime (bundled)".to_owned(),
            RuntimePrerequisiteKind::DotNetDesktopRuntime,
            false,
        ),
    };
    RuntimePrerequisite {
        name,
        kind,
        evidence: evidence.to_owned(),
        install_manually,
        present_on_source: None,
        bundle: None,
        presence_probe: None,
        architecture: Some(build.architecture()),
    }
}

/// Derive runtime prerequisites from what the app's programs need and the names of its files.
/// Pure so the detection rules can be unit-tested with crafted inputs. `install_manually` is false
/// when the runtime ships with the app and true when it must be installed on the target system.
fn runtime_prerequisites_from_signals(
    names: &BTreeSet<String>,
    runtimes: &AppRuntimes,
) -> Vec<RuntimePrerequisite> {
    let mut prerequisites = Vec::new();
    let has = |needle: &str| names.contains(needle);
    let any = |needles: &[&str]| needles.iter().any(|needle| names.contains(*needle));

    for ((runtime, build), evidence) in &runtimes.visual_cpp {
        prerequisites.push(visual_cpp_prerequisite(*runtime, *build, evidence));
    }

    let mut dotnet = runtimes.dotnet.clone();
    // A program of its own can be native and find .NET at run time, so what it ships counts too.
    if any(&["coreclr.dll", "hostfxr.dll", "hostpolicy.dll", "clrjit.dll"]) {
        dotnet
            .entry((DotNetNeed::SelfContained, ArchitectureKey::Unknown))
            .or_insert_with(|| "Self-contained .NET host present (coreclr/hostfxr)".to_owned());
    }
    if dotnet.is_empty() && any(&["mscoree.dll", "mscoreei.dll", "clr.dll"]) {
        dotnet.insert(
            (DotNetNeed::Framework4x, ArchitectureKey::Unknown),
            ".NET Framework CLR host (mscoree)".to_owned(),
        );
    }
    let mut said_of_any_build = BTreeSet::new();
    for ((need, build), evidence) in &dotnet {
        // What is part of Windows, or ships with the app, is one thing to say, whichever of its
        // programs said it and whatever build each is.
        let is_of_any_build = matches!(
            need,
            DotNetNeed::SelfContained
                | DotNetNeed::Framework35
                | DotNetNeed::Framework4x
                | DotNetNeed::ModernOfNoKnownVersion
        );
        if is_of_any_build && !said_of_any_build.insert(*need) {
            continue;
        }
        prerequisites.push(dotnet_prerequisite(*need, *build, evidence));
    }

    if any(&["jvm.dll", "java.dll", "jli.dll", "javaw.exe", "java.exe"]) {
        prerequisites.push(RuntimePrerequisite {
            name: "Java Runtime Environment".to_owned(),
            kind: RuntimePrerequisiteKind::JavaRuntime,
            evidence: "Java runtime artifacts present (jvm/java)".to_owned(),
            install_manually: !has("jvm.dll"),
            present_on_source: None,
            bundle: None,
            presence_probe: None,
            architecture: None,
        });
    }

    let python = names
        .iter()
        .any(|name| name.starts_with("python") && name.ends_with(".dll"))
        || any(&["python.exe", "pythonw.exe"]);
    if python {
        prerequisites.push(RuntimePrerequisite {
            name: "Python Runtime".to_owned(),
            kind: RuntimePrerequisiteKind::PythonRuntime,
            evidence: "Python runtime artifacts present (python*.dll)".to_owned(),
            install_manually: false,
            present_on_source: None,
            bundle: None,
            presence_probe: None,
            architecture: None,
        });
    }

    let electron = any(&["node.dll", "libnode.dll"])
        || (has("ffmpeg.dll")
            && (has("resources.pak") || names.iter().any(|name| name.ends_with(".asar"))));
    if electron {
        prerequisites.push(RuntimePrerequisite {
            name: "Electron / Node runtime".to_owned(),
            kind: RuntimePrerequisiteKind::ElectronNodeRuntime,
            evidence: "Electron/Node artifacts present (node/ffmpeg/asar)".to_owned(),
            install_manually: false,
            present_on_source: None,
            bundle: None,
            presence_probe: None,
            architecture: None,
        });
    }

    prerequisites
}

/// Each thing left out, for whoever is building to see. Only ever what it is called.
fn left_out_warnings(left_out: &LeftOut) -> Vec<Warning> {
    let about = |code: &'static str, item: &crate::models::LeftOutItem| {
        Warning::new(
            code,
            WarningSeverity::Info,
            format!("Left out of the package: {}.", item.why),
            Some(item.name.clone()),
        )
    };
    left_out
        .secrets
        .iter()
        .map(|item| about("secret_left_out", item))
        .chain(
            left_out
                .licenses
                .iter()
                .map(|item| about("license_left_out", item)),
        )
        .chain(
            left_out
                .settings
                .iter()
                .map(|item| about("settings_left_out", item)),
        )
        .collect()
}

fn missing_dependency_warnings(dependencies: &[DependencyEntry]) -> Vec<Warning> {
    dependencies
        .iter()
        .filter_map(|dependency| {
            if dependency.missing {
                return Some(Warning::new(
                    "missing_dependency",
                    WarningSeverity::Warning,
                    format!("Dependency could not be resolved: {}", dependency.name),
                    None,
                ));
            }
            // Where this PC happens to find it says nothing of the other PC.
            matches!(
                dependency.source_category,
                DependencySourceCategory::Path | DependencySourceCategory::CurrentDirectory
            )
            .then(|| {
                Warning::new(
                    "dependency_only_found_on_this_pc",
                    WarningSeverity::Warning,
                    format!(
                        "{} was only found through this PC's search path, and is not part of the \
                         app, so it does not come along. The app may not start on a PC without it.",
                        dependency.name
                    ),
                    None,
                )
            })
        })
        .collect()
}

/// A driver the app registers goes with it. Any other driver file does not, and an app that loads
/// its own on demand will not work without it, which is something to say and not just to note.
fn say_which_driver_files_stay(warnings: &mut [Warning], drivers: &[DriverEntry]) {
    for warning in warnings
        .iter_mut()
        .filter(|warning| warning.code == DRIVER_FILE_NOT_COPIED_CODE)
    {
        let registered = warning.source.as_deref().is_some_and(|source| {
            drivers.iter().any(|driver| {
                driver.app_owned
                    && comparable_path(&driver.image_path) == comparable_path(Path::new(source))
            })
        });
        if !registered {
            warning.severity = WarningSeverity::Warning;
        }
    }
}

fn driver_component_warnings(drivers: &[DriverEntry]) -> Vec<Warning> {
    drivers
        .iter()
        .map(|driver| {
            Warning::new(
                "driver_kernel_component_detected",
                WarningSeverity::Warning,
                "Driver or kernel component detected. A driver the app owns is bundled and re-registered; a vendor or inbox driver is not redistributed and must be reinstalled from its signed vendor package.",
                Some(format!(
                    "{} -> {}",
                    driver.service_name,
                    driver.image_path.display()
                )),
            )
        })
        .collect()
}

fn calculate_confidence_score(inputs: ConfidenceInputs<'_>) -> u8 {
    // Scoring is intentionally conservative and explainable:
    // unresolved imports are the largest immediate migration risk, runtime
    // prerequisites require manual installation, manifests may imply side-by-side
    // assemblies, and non-.exe scan targets slightly reduce certainty.
    let missing_count = inputs
        .dependencies
        .iter()
        .filter(|dependency| dependency.missing)
        .count() as i32;
    let mut score = BASE_CONFIDENCE_SCORE
        - (missing_count * MISSING_DLL_PENALTY)
        - (inputs.runtime_prerequisite_count as i32 * RUNTIME_PREREQUISITE_PENALTY);

    score -= inputs.service_count as i32 * 8;
    score -= inputs.driver_count as i32 * 12;
    score -= inputs.scheduled_task_count as i32 * 3;
    score -= inputs.admin_registry_count as i32 * 2;
    score -= inputs.files_not_copied_count.min(10) as i32;

    if inputs.external_manifest_detected {
        score -= EXTERNAL_MANIFEST_PENALTY;
    }
    if inputs.non_exe_extension {
        score -= NON_EXE_EXTENSION_PENALTY;
    }
    if inputs.managed_dotnet {
        score -= MANAGED_DOTNET_PENALTY;
    }
    score -= (inputs.sxs_dependency_count as i32 * SXS_DEPENDENCY_PENALTY)
        .min(MAX_SXS_DEPENDENCY_PENALTY);

    score.clamp(MIN_CONFIDENCE_SCORE, MAX_CONFIDENCE_SCORE) as u8
}

#[cfg(test)]
mod tests {
    use crate::models::DependencySourceCategory;

    use super::*;

    #[test]
    fn the_recorded_install_location_stays_a_root_when_the_executable_is_elsewhere() {
        let app = InstalledApp {
            display_name: "NVIDIA App".to_owned(),
            publisher: None,
            version: None,
            install_location: Some(PathBuf::from(r"C:\Vendor\NVIDIA app")),
            uninstall_string: None,
            main_executable: Some(PathBuf::from(r"C:\Vendor\NVIDIA app\CEF\NVIDIA App.exe")),
            registry_source_path: r"HKLM\Example".to_owned(),
            architecture: AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        };
        let roots = collect_extra_roots(
            Some(&app),
            Some(Path::new(r"C:\Vendor\Installer2\Display.NvApp")),
            &[],
            &[],
            &[],
        );

        assert!(roots.contains(&PathBuf::from(r"C:\Vendor\Installer2\Display.NvApp")));
        assert!(roots.contains(&PathBuf::from(r"C:\Vendor\NVIDIA app")));
        assert!(roots.contains(&PathBuf::from(r"C:\Vendor\NVIDIA app\CEF")));
    }

    #[test]
    fn serializes_scan_options() -> Result<()> {
        let options = ScanOptions::for_executable(r"C:\App\App.exe", r"C:\Out");

        let json = serde_json::to_string(&options)?;

        assert!(json.contains("executable_path"));
        assert!(json.contains("C:"));
        Ok(())
    }

    #[test]
    fn confidence_drops_for_missing_dependency() {
        let dependency = DependencyEntry {
            name: "missing.dll".to_owned(),
            resolved_path: None,
            architecture: AppArchitecture::Unknown,
            hash_sha256: None,
            version_info: None,
            publisher: None,
            source_category: DependencySourceCategory::Unresolved,
            should_copy: false,
            system_provided: false,
            runtime_prerequisite: false,
            missing: true,
            delay_loaded: false,
        };

        let score = calculate_confidence_score(ConfidenceInputs {
            dependencies: &[dependency],
            runtime_prerequisite_count: 0,
            external_manifest_detected: false,
            non_exe_extension: false,
            managed_dotnet: false,
            sxs_dependency_count: 0,
            service_count: 0,
            driver_count: 0,
            scheduled_task_count: 0,
            admin_registry_count: 0,
            files_not_copied_count: 0,
        });

        assert_eq!(score, 92);
    }

    fn name_set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    fn kinds_of(prerequisites: &[RuntimePrerequisite]) -> Vec<&RuntimePrerequisiteKind> {
        prerequisites.iter().map(|found| &found.kind).collect()
    }

    fn needs_dotnet(need: DotNetNeed) -> AppRuntimes {
        let mut runtimes = AppRuntimes::default();
        runtimes
            .dotnet
            .insert((need, ArchitectureKey::X64), "app.exe".to_owned());
        runtimes
    }

    #[test]
    fn detects_bundled_dotnet_and_electron_runtimes() {
        let names = name_set(&["coreclr.dll", "hostfxr.dll", "ffmpeg.dll", "app.asar"]);
        let prerequisites = runtime_prerequisites_from_signals(&names, &AppRuntimes::default());

        assert!(
            prerequisites.iter().any(|prerequisite| {
                prerequisite.kind == RuntimePrerequisiteKind::DotNetDesktopRuntime
                    && !prerequisite.install_manually
            }),
            "self-contained .NET ships the runtime"
        );
        assert!(
            prerequisites
                .iter()
                .any(|prerequisite| prerequisite.kind
                    == RuntimePrerequisiteKind::ElectronNodeRuntime),
            "ffmpeg.dll + .asar should detect Electron"
        );
    }

    #[test]
    fn dotnet_framework_2_0_maps_to_netfx3() {
        // CLR v2.0.50727 is the YDIS case: it needs the .NET Framework 3.5 Windows feature, not
        // the modern .NET desktop runtime.
        let prerequisites = runtime_prerequisites_from_signals(
            &name_set(&["mscoree.dll"]),
            &needs_dotnet(DotNetNeed::Framework35),
        );
        assert_eq!(
            kinds_of(&prerequisites),
            [&RuntimePrerequisiteKind::DotNetFramework35]
        );
        assert!(prerequisites[0].install_manually);
    }

    #[test]
    fn dotnet_framework_4_x_is_inbox_and_modern_dotnet_is_the_version_it_says() {
        let framework = runtime_prerequisites_from_signals(
            &name_set(&["mscoree.dll"]),
            &needs_dotnet(DotNetNeed::Framework4x),
        );
        assert_eq!(
            kinds_of(&framework),
            [&RuntimePrerequisiteKind::DotNetFramework4x]
        );
        assert!(!framework[0].install_manually);

        for major in [6, 8, 10] {
            let modern = runtime_prerequisites_from_signals(
                &BTreeSet::new(),
                &needs_dotnet(DotNetNeed::Modern {
                    flavor: DotNetFlavor::Desktop,
                    major,
                }),
            );
            assert_eq!(
                kinds_of(&modern),
                [&RuntimePrerequisiteKind::DotNetRuntime {
                    flavor: DotNetFlavor::Desktop,
                    major
                }],
                "{major}"
            );
            assert!(modern[0].install_manually);
            assert!(
                modern[0].name.contains(&major.to_string()),
                "{}",
                modern[0].name
            );
        }
    }

    #[test]
    fn a_dotnet_of_no_known_version_has_nothing_installed_for_it_and_is_named() {
        let runtimes = needs_dotnet(DotNetNeed::ModernOfNoKnownVersion);
        let (prerequisites, warnings) =
            runtime_prerequisites(&runtimes, &[], &[], &[], &AppArchitecture::X64, true);
        assert_eq!(
            kinds_of(&prerequisites),
            [&RuntimePrerequisiteKind::DotNetDesktopRuntime]
        );
        let warning = warnings
            .iter()
            .find(|warning| warning.code == "runtime_has_no_installer");
        assert!(
            warning.is_some_and(|warning| warning.message.contains(&prerequisites[0].name)),
            "{warnings:?}"
        );
    }

    #[test]
    fn each_visual_cpp_runtime_is_its_own_for_its_own_build() {
        let mut runtimes = AppRuntimes::default();
        for (runtime, build) in [
            (VisualCppRuntime::Vs2010, ArchitectureKey::X86),
            (VisualCppRuntime::Vs2013, ArchitectureKey::X64),
            (VisualCppRuntime::Vs2015Plus, ArchitectureKey::X64),
            (VisualCppRuntime::Vs2015Plus, ArchitectureKey::X86),
        ] {
            runtimes
                .visual_cpp
                .insert((runtime, build), "a.exe imports it".to_owned());
        }
        let prerequisites = runtime_prerequisites_from_signals(&BTreeSet::new(), &runtimes);
        let shown: Vec<(RuntimePrerequisiteKind, Option<AppArchitecture>)> = prerequisites
            .iter()
            .map(|found| (found.kind.clone(), found.architecture.clone()))
            .collect();
        assert_eq!(
            shown,
            [
                (
                    RuntimePrerequisiteKind::VisualCpp2010,
                    Some(AppArchitecture::X86)
                ),
                (
                    RuntimePrerequisiteKind::VisualCpp2013,
                    Some(AppArchitecture::X64)
                ),
                (
                    RuntimePrerequisiteKind::VisualCppRedistributable,
                    Some(AppArchitecture::X86)
                ),
                (
                    RuntimePrerequisiteKind::VisualCppRedistributable,
                    Some(AppArchitecture::X64)
                ),
            ]
        );
    }

    #[test]
    fn a_runtime_no_one_can_install_is_named_rather_than_replaced_with_another() {
        let mut runtimes = AppRuntimes::default();
        runtimes.visual_cpp.insert(
            (VisualCppRuntime::Vs2003, ArchitectureKey::X86),
            "old.exe imports msvcr71.dll".to_owned(),
        );
        runtimes.visual_cpp.insert(
            (VisualCppRuntime::Vs2013, ArchitectureKey::Arm64),
            "arm.exe imports msvcr120.dll".to_owned(),
        );
        runtimes.debug_dlls.insert("msvcp140d.dll".to_owned());
        runtimes.programs_left_out = 3;
        let (prerequisites, warnings) =
            runtime_prerequisites(&runtimes, &[], &[], &[], &AppArchitecture::X64, true);
        assert_eq!(prerequisites.len(), 2);
        let named: Vec<&str> = warnings
            .iter()
            .filter(|warning| warning.code == "runtime_has_no_installer")
            .map(|warning| warning.message.as_str())
            .collect();
        assert_eq!(named.len(), 2, "{warnings:?}");
        assert!(named.iter().any(|message| message.contains("2003")));
        assert!(named.iter().any(|message| message.contains("2013")));
        for code in ["runtime_is_a_debug_build", "runtime_inspection_limited"] {
            assert!(
                warnings.iter().any(|warning| warning.code == code),
                "{code}"
            );
        }
    }

    #[test]
    fn detects_java_and_python_runtimes() {
        let java =
            runtime_prerequisites_from_signals(&name_set(&["jvm.dll"]), &AppRuntimes::default());
        assert!(
            java.iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::JavaRuntime)
        );
        let python = runtime_prerequisites_from_signals(
            &name_set(&["python311.dll"]),
            &AppRuntimes::default(),
        );
        assert!(
            python
                .iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::PythonRuntime)
        );
    }
}
