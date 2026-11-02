use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::dll_resolver::{
    DllResolutionContext, find_app_local_dlls, is_runtime_prerequisite_dll, resolve_imports,
};
use crate::env_scan::{EnvironmentScanContext, scan_environment};
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{FileDiscoveryContext, discover_files, package_relative_path};
use crate::install_source::{
    InstallSource, add_app_paths_roots, detect_install_source, is_usable_main_executable,
    primary_executable, resolve_install_location, squirrel_current_executable,
};
use crate::license_scan::{LicenseScanContext, scan_license_artifacts};
use crate::models::{
    AppSelection, DependencyEntry, DriverEntry, FileEntry, InstallScope, InstalledApp,
    RuntimePrerequisite, RuntimePrerequisiteKind, SCAN_SCHEMA_VERSION, ScanOptions, ScanReport,
    ScheduledTaskEntry, ServiceEntry, ShortcutEntry, Warning, WarningSeverity,
};
use crate::path_utils::{
    has_parent_step, is_drive_root, looks_like_executable, path_is_within, resolved_location,
};
use crate::pe_scan::scan_pe;
use crate::registry_scan::{
    RegistryScanContext, recorded_keys_of_one_account, scan_registry_artifacts,
};
use crate::reporting::{artifact_paths, write_scan_reports};
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

pub fn scan_and_write_reports(options: &ScanOptions) -> Result<ScanReport> {
    // The single seam every scanner reads through: the live OS, or a mounted
    // offline volume from another PC. Held for the whole scan so offline hive
    // mounts stay live until every scanner has run. It reads one account, so another's cannot
    // come along by accident.
    let owner = match &options.selection {
        AppSelection::InstalledApp(app) => app.owner_profile.as_deref(),
        AppSelection::ExecutablePath(_) => None,
    };
    let system = SystemContext::open_for_scan(&options.source, &options.user_profile, owner)?;
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
        .map(|app| detect_install_source(app, &system));
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
            "is not on a disk of this PC"
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
            .is_some_and(|recorded| path_is_within(path, recorded) && !has_parent_step(path));
        source_app.is_some()
            && !is_where_its_record_says
            && (system.is_in_another_profile(path)
                || (installed_for_one_account && !system.is_the_accounts_own(path)))
    };
    if is_someone_elses(&executable_path) {
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

    let registry_artifacts = scan_registry_artifacts(
        &RegistryScanContext {
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
                .unwrap_or_default(),
        },
        &system,
    );
    let (services, service_warnings) = scan_services(
        &ServiceScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
        },
        &system,
    );
    let (drivers, driver_warnings) = scan_drivers(
        &ServiceScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
        },
        &system,
    );
    let (scheduled_tasks, task_warnings) = scan_scheduled_tasks(
        &TaskScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
        },
        &system,
    );
    let (shortcuts, shortcut_warnings) = scan_shortcuts(
        &ShortcutScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
        },
        &system,
    );
    let (environment_variables, environment_warnings) = scan_environment(
        &EnvironmentScanContext {
            executable_path: logical_executable.clone(),
            install_location: logical_location.clone(),
        },
        &system,
    );

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
                !has_parent_step(path)
                    && (install_location
                        .as_deref()
                        .is_some_and(|location| path_is_within(path, location))
                        || system.is_in_the_accounts_profile(path))
            };
            source.roots.retain(stays);
            source.loose_files.retain(stays);
        }
        extra_roots.extend(source.roots.iter().cloned());
    }
    let (file_inventory, footprint_locations, file_warnings) = discover_files(
        &FileDiscoveryContext {
            executable_path: executable_path.clone(),
            install_location: install_location.clone(),
            app_name,
            publisher,
            include_hashes: options.include_hashes,
            include_user_data: options.include_user_data,
            include_program_data: options.include_program_data,
            installed_for_one_account,
            extra_roots,
            extra_files: install_source
                .as_ref()
                .map(|source| source.loose_files.clone())
                .unwrap_or_default(),
        },
        &system,
    );

    let runtime_prerequisites = runtime_prerequisites(
        &dependencies,
        &app_local_dlls,
        &file_inventory,
        pe_scan.metadata.is_dotnet,
        pe_scan.metadata.dotnet_runtime_version.as_deref(),
        &pe_scan.architecture,
        system.is_offline(),
    );
    let (license_artifacts, license_registry_entries, license_warnings) = scan_license_artifacts(
        &LicenseScanContext {
            publisher: source_app.as_ref().and_then(|app| app.publisher.as_deref()),
            discovered_files: &file_inventory,
            include_license_data: options.include_license_data,
            include_hashes: options.include_hashes,
        },
        &system,
    );
    let mut warnings = system.warnings();
    warnings.extend(pe_scan.warnings);
    warnings.extend(resolver_warnings);
    warnings.extend(file_warnings);
    warnings.extend(registry_artifacts.warnings.clone());
    warnings.extend(service_warnings);
    warnings.extend(driver_warnings);
    warnings.extend(task_warnings);
    warnings.extend(shortcut_warnings);
    warnings.extend(environment_warnings);
    warnings.extend(missing_dependency_warnings(&dependencies));
    warnings.extend(driver_component_warnings(&drivers));
    warnings.extend(license_warnings);

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
    let user_data_profile = system.user_hives().into_iter().next().map(|user| user.name);
    let mut report = ScanReport {
        schema_version: SCAN_SCHEMA_VERSION.to_owned(),
        generated_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        source_volume: system.volume_map().cloned(),
        other_accounts: system.other_accounts_folders(install_location.as_deref()),
        source_computer_name: identity.computer_name,
        source_windows_version: identity.windows_version,
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
        warnings,
        items_excluded: vec![
            "Windows directory, System32, and SysWOW64 files are reported but not copied."
                .to_owned(),
            "Shared runtimes are reported as prerequisites, not copied. Folders outside the install records are matched by the product's exact name, so a folder the app uses under another name is missed."
                .to_owned(),
            "Registry, services, scheduled tasks, and shortcuts are archived for restore; apply mode remains explicit and privilege-gated."
                .to_owned(),
            "Application-owned drivers are bundled and re-registered automatically; vendor/inbox drivers are reported with precise reinstall guidance and are not redistributed."
                .to_owned(),
        ],
        confidence_score,
        generated_artifacts: Some(artifacts),
    };

    use_what_the_source_pc_calls_things(&mut report, &system);
    write_scan_reports(&report, &options.output_dir)?;
    Ok(report)
}

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

fn runtime_prerequisites(
    dependencies: &[DependencyEntry],
    app_local_dlls: &[FileEntry],
    file_inventory: &[FileEntry],
    is_dotnet: bool,
    dotnet_runtime_version: Option<&str>,
    architecture: &crate::models::AppArchitecture,
    offline: bool,
) -> Vec<RuntimePrerequisite> {
    let signals = collect_runtime_signal_names(dependencies, app_local_dlls, file_inventory);
    let vc_dependency = dependencies
        .iter()
        .find(|dependency| is_runtime_prerequisite_dll(&dependency.name))
        .map(|dependency| dependency.name.clone());
    let mut prerequisites = runtime_prerequisites_from_signals(
        &signals,
        vc_dependency.as_deref(),
        is_dotnet,
        dotnet_runtime_version,
    );
    // Attach a presence probe and a source-presence reading where one is known,
    // so packaging and the installer can confirm the runtime before and after
    // installing it. `is_present` reads the *live* registry, so it only reflects
    // the source when scanning the live machine; for an offline disk we leave
    // the source presence unknown rather than reporting this PC's runtimes.
    for prerequisite in &mut prerequisites {
        if let Some(probe) = crate::prerequisites::presence_probe(&prerequisite.kind, architecture)
        {
            if !offline {
                prerequisite.present_on_source = Some(crate::prerequisites::is_present(&probe));
            }
            prerequisite.presence_probe = Some(probe);
        }
    }
    prerequisites
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

/// Derive runtime prerequisites from collected signal names. Pure so the
/// detection rules can be unit-tested with crafted inputs. `install_manually`
/// is false when the runtime ships with the app (bundled app-local) and true
/// when it must be installed on the target system.
fn runtime_prerequisites_from_signals(
    names: &BTreeSet<String>,
    vc_dependency: Option<&str>,
    is_dotnet: bool,
    dotnet_runtime_version: Option<&str>,
) -> Vec<RuntimePrerequisite> {
    let mut prerequisites = Vec::new();
    let has = |needle: &str| names.contains(needle);
    let any = |needles: &[&str]| needles.iter().any(|needle| names.contains(*needle));

    if let Some(name) = vc_dependency {
        prerequisites.push(RuntimePrerequisite {
            name: "Microsoft Visual C++ Redistributable".to_owned(),
            kind: RuntimePrerequisiteKind::VisualCppRedistributable,
            evidence: format!("Imported runtime DLL: {name}"),
            install_manually: true,
            present_on_source: None,
            bundle: None,
            presence_probe: None,
        });
    }

    let dotnet_core = any(&["coreclr.dll", "hostfxr.dll", "hostpolicy.dll", "clrjit.dll"]);
    // `mscoree`/`clr` is the .NET *Framework* shim; modern .NET (Core) apps host
    // through `hostfxr`/`coreclr` instead.
    let dotnet_framework = any(&["mscoree.dll", "mscoreei.dll", "clr.dll"]);
    if is_dotnet || dotnet_core || dotnet_framework {
        prerequisites.push(dotnet_prerequisite(
            dotnet_core,
            dotnet_framework,
            dotnet_runtime_version,
        ));
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
        });
    }

    prerequisites
}

/// Pick the correct .NET prerequisite for a managed image. The distinguishing
/// signals are the host shim (`mscoree` = .NET Framework, `hostfxr`/`coreclr` =
/// modern self-contained .NET) and the CLR metadata version: `v1.x`/`v2.0`
/// requires the .NET Framework 3.5 Windows feature (NetFx3), while `v4.0` is the
/// in-box .NET Framework 4.x on Windows 10/11.
fn dotnet_prerequisite(
    dotnet_core: bool,
    dotnet_framework: bool,
    dotnet_runtime_version: Option<&str>,
) -> RuntimePrerequisite {
    let version = dotnet_runtime_version.unwrap_or_default();
    let needs_netfx3 = version.starts_with("v1.") || version.starts_with("v2.");
    let (name, kind, evidence, install_manually) = if dotnet_core {
        (
            ".NET runtime (bundled)",
            RuntimePrerequisiteKind::DotNetDesktopRuntime,
            "Self-contained .NET host present (coreclr/hostfxr)".to_owned(),
            false,
        )
    } else if dotnet_framework && needs_netfx3 {
        (
            "Microsoft .NET Framework 3.5",
            RuntimePrerequisiteKind::DotNetFramework35,
            format!(".NET Framework CLR host (mscoree) with CLR {version}; needs NetFx3"),
            true,
        )
    } else if dotnet_framework {
        (
            "Microsoft .NET Framework 4.x",
            RuntimePrerequisiteKind::DotNetFramework4x,
            format!(
                ".NET Framework CLR host (mscoree){}; ships with Windows 10/11",
                if version.is_empty() {
                    String::new()
                } else {
                    format!(" with CLR {version}")
                }
            ),
            false,
        )
    } else if needs_netfx3 {
        // Managed image (COR20) with no host-shim import but an old CLR metadata
        // version: still the legacy Framework, so NetFx3 is the right runtime.
        (
            "Microsoft .NET Framework 3.5",
            RuntimePrerequisiteKind::DotNetFramework35,
            format!("Managed (.NET) image with CLR {version}; needs NetFx3"),
            true,
        )
    } else {
        (
            "Microsoft .NET Desktop Runtime",
            RuntimePrerequisiteKind::DotNetDesktopRuntime,
            "Managed (.NET) image: CLR/COR20 header present".to_owned(),
            true,
        )
    };
    RuntimePrerequisite {
        name: name.to_owned(),
        kind,
        evidence,
        install_manually,
        present_on_source: None,
        bundle: None,
        presence_probe: None,
    }
}

fn missing_dependency_warnings(dependencies: &[DependencyEntry]) -> Vec<Warning> {
    dependencies
        .iter()
        .filter(|dependency| dependency.missing)
        .map(|dependency| {
            Warning::new(
                "missing_dependency",
                WarningSeverity::Warning,
                format!("Dependency could not be resolved: {}", dependency.name),
                None,
            )
        })
        .collect()
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
    use crate::models::{AppArchitecture, DependencySourceCategory};

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

    #[test]
    fn detects_bundled_dotnet_and_electron_runtimes() {
        let names = name_set(&["coreclr.dll", "hostfxr.dll", "ffmpeg.dll", "app.asar"]);
        let prerequisites =
            runtime_prerequisites_from_signals(&names, None, true, Some("v4.0.30319"));

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
        // mscoree + CLR v2.0.50727 is the YDIS case: it needs the .NET Framework
        // 3.5 Windows feature, not the modern .NET desktop runtime.
        let names = name_set(&["mscoree.dll"]);
        let prerequisites =
            runtime_prerequisites_from_signals(&names, None, true, Some("v2.0.50727"));

        assert!(
            prerequisites.iter().any(|prerequisite| {
                prerequisite.kind == RuntimePrerequisiteKind::DotNetFramework35
                    && prerequisite.install_manually
            }),
            "CLR v2.0 must map to .NET Framework 3.5, got {:?}",
            prerequisites.iter().map(|p| &p.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn dotnet_framework_4_x_is_inbox_not_dotnet_8() {
        let names = name_set(&["mscoree.dll"]);
        let prerequisites =
            runtime_prerequisites_from_signals(&names, None, true, Some("v4.0.30319"));

        assert!(
            prerequisites.iter().any(|prerequisite| {
                prerequisite.kind == RuntimePrerequisiteKind::DotNetFramework4x
                    && !prerequisite.install_manually
            }),
            "CLR v4.0 with mscoree must be listed as in-box 4.x, not a runtime install"
        );
        assert!(
            !prerequisites
                .iter()
                .any(|prerequisite| prerequisite.kind
                    == RuntimePrerequisiteKind::DotNetDesktopRuntime),
            ".NET Framework must never be reported as the modern .NET desktop runtime"
        );
    }

    #[test]
    fn detects_java_and_python_runtimes() {
        let java = runtime_prerequisites_from_signals(&name_set(&["jvm.dll"]), None, false, None);
        assert!(
            java.iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::JavaRuntime)
        );
        let python =
            runtime_prerequisites_from_signals(&name_set(&["python311.dll"]), None, false, None);
        assert!(
            python
                .iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::PythonRuntime)
        );
    }
}
