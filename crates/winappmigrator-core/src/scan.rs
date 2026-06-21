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
use crate::file_discovery::{FileDiscoveryContext, discover_files};
use crate::install_source::detect_install_source;
use crate::license_scan::{LicenseScanContext, scan_license_artifacts};
use crate::models::{
    AppSelection, DependencyEntry, DriverEntry, FileEntry, InstalledApp, RuntimePrerequisite,
    RuntimePrerequisiteKind, SCAN_SCHEMA_VERSION, ScanOptions, ScanReport, ScheduledTaskEntry,
    ServiceEntry, ShortcutEntry, Warning, WarningSeverity,
};
use crate::path_utils::looks_like_executable;
use crate::pe_scan::scan_pe;
use crate::registry_scan::{RegistryScanContext, scan_registry_artifacts};
use crate::reporting::{artifact_paths, write_scan_reports};
use crate::service_scan::{ServiceScanContext, scan_drivers, scan_services};
use crate::shortcut_scan::{ShortcutScanContext, scan_shortcuts};
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
    let executable_path = selected_executable_path(options)?;
    validate_scan_input(&executable_path)?;

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

    let context = DllResolutionContext {
        executable_path: executable_path.clone(),
        app_architecture: pe_scan.architecture.clone(),
        current_directory: options.current_directory.clone(),
        recursive: options.recursive_dependencies,
        include_hashes: options.include_hashes,
    };
    let (dependencies, resolver_warnings) = resolve_imports(&pe_scan.direct_imports, &context);
    let app_local_dlls = find_app_local_dlls(&executable_path)?;
    let source_app = match &options.selection {
        AppSelection::InstalledApp(app) => Some((**app).clone()),
        AppSelection::ExecutablePath(_) => None,
    };
    let install_location = source_app
        .as_ref()
        .and_then(|app| app.install_location.clone())
        .or_else(|| executable_path.parent().map(Path::to_path_buf));
    let app_name = source_app.as_ref().map(|app| app.display_name.clone());
    let publisher = source_app.as_ref().and_then(|app| app.publisher.clone());
    let source_registry_path = source_app
        .as_ref()
        .map(|app| app.registry_source_path.clone());

    // Reverse-engineer how the app was installed; for MSI products this reads
    // the authoritative Windows Installer component database.
    let install_source = source_app.as_ref().map(detect_install_source);

    let registry_artifacts = scan_registry_artifacts(&RegistryScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
        display_name: app_name.clone(),
        publisher: publisher.clone(),
        source_registry_path,
    });
    let (services, service_warnings) = scan_services(&ServiceScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
    });
    let (drivers, driver_warnings) = scan_drivers(&ServiceScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
    });
    let (scheduled_tasks, task_warnings) = scan_scheduled_tasks(&TaskScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
    });
    let (shortcuts, shortcut_warnings) = scan_shortcuts(&ShortcutScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
    });
    let (environment_variables, environment_warnings) = scan_environment(&EnvironmentScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
    });

    // Fold concrete locations referenced by Windows integrations into the
    // footprint discovery so split installs (Program Files plus a service
    // binary or a ProgramData data tree) are captured automatically.
    let mut extra_roots =
        collect_extra_roots(source_app.as_ref(), &services, &scheduled_tasks, &shortcuts);
    if let Some(source) = install_source.as_ref() {
        extra_roots.extend(source.roots.iter().cloned());
    }
    let (file_inventory, footprint_locations, file_warnings) =
        discover_files(&FileDiscoveryContext {
            executable_path: executable_path.clone(),
            install_location: install_location.clone(),
            app_name,
            publisher,
            include_hashes: options.include_hashes,
            include_user_data: options.include_user_data,
            include_program_data: options.include_program_data,
            extra_roots,
        });

    let runtime_prerequisites = runtime_prerequisites(
        &dependencies,
        &app_local_dlls,
        &file_inventory,
        pe_scan.metadata.is_dotnet,
        &pe_scan.architecture,
    );
    let (license_artifacts, license_registry_entries, license_warnings) =
        scan_license_artifacts(&LicenseScanContext {
            publisher: source_app.as_ref().and_then(|app| app.publisher.as_deref()),
            discovered_files: &file_inventory,
            include_license_data: options.include_license_data,
            include_hashes: options.include_hashes,
        });
    let mut warnings = pe_scan.warnings;
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

    let report = ScanReport {
        schema_version: SCAN_SCHEMA_VERSION.to_owned(),
        generated_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
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
            "Non-app-owned shared files outside the app directory are never copied; shared runtimes are reported as prerequisites."
                .to_owned(),
            "Registry, services, scheduled tasks, and shortcuts are archived for restore; apply mode remains explicit and privilege-gated."
                .to_owned(),
            "Application-owned drivers are bundled and re-registered automatically; vendor/inbox drivers are reported with precise reinstall guidance and are not redistributed."
                .to_owned(),
        ],
        confidence_score,
        generated_artifacts: Some(artifacts),
    };

    write_scan_reports(&report, &options.output_dir)?;
    Ok(report)
}

fn selected_executable_path(options: &ScanOptions) -> Result<PathBuf> {
    match &options.selection {
        AppSelection::ExecutablePath(path) => Ok(path.clone()),
        AppSelection::InstalledApp(app) => app
            .main_executable
            .clone()
            .or_else(|| crate::install_source::primary_executable(app))
            .ok_or_else(|| {
                WinAppMigratorError::InvalidInput(format!(
                    "installed app has no main executable guess: {}",
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

fn collect_extra_roots(
    source_app: Option<&InstalledApp>,
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
    architecture: &crate::models::AppArchitecture,
) -> Vec<RuntimePrerequisite> {
    let signals = collect_runtime_signal_names(dependencies, app_local_dlls, file_inventory);
    let vc_dependency = dependencies
        .iter()
        .find(|dependency| is_runtime_prerequisite_dll(&dependency.name))
        .map(|dependency| dependency.name.clone());
    let mut prerequisites =
        runtime_prerequisites_from_signals(&signals, vc_dependency.as_deref(), is_dotnet);
    // Attach a presence probe and a source-presence reading where one is known,
    // so packaging and the installer can confirm the runtime before and after
    // installing it.
    for prerequisite in &mut prerequisites {
        if let Some(probe) = crate::prerequisites::presence_probe(&prerequisite.kind, architecture)
        {
            prerequisite.present_on_source = Some(crate::prerequisites::is_present(&probe));
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
    let dotnet_framework = any(&["mscoree.dll", "mscoreei.dll", "clr.dll"]);
    if is_dotnet || dotnet_core || dotnet_framework {
        let (name, evidence, install_manually) = if dotnet_core {
            (
                ".NET runtime (bundled)",
                "Self-contained .NET host present (coreclr/hostfxr)",
                false,
            )
        } else if dotnet_framework {
            (
                "Microsoft .NET Framework",
                ".NET Framework CLR host referenced (mscoree)",
                true,
            )
        } else {
            (
                "Microsoft .NET Desktop Runtime",
                "Managed (.NET) image: CLR/COR20 header present",
                true,
            )
        };
        prerequisites.push(RuntimePrerequisite {
            name: name.to_owned(),
            kind: RuntimePrerequisiteKind::DotNetDesktopRuntime,
            evidence: evidence.to_owned(),
            install_manually,
            present_on_source: None,
            bundle: None,
            presence_probe: None,
        });
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
                "Driver or kernel component detected. It is archived as metadata only and must be reinstalled with a signed vendor driver package.",
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
        let prerequisites = runtime_prerequisites_from_signals(&names, None, true);

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
    fn dotnet_framework_requires_manual_install() {
        let names = name_set(&["mscoree.dll"]);
        let prerequisites = runtime_prerequisites_from_signals(&names, None, false);

        assert!(prerequisites.iter().any(|prerequisite| {
            prerequisite.kind == RuntimePrerequisiteKind::DotNetDesktopRuntime
                && prerequisite.install_manually
        }));
    }

    #[test]
    fn detects_java_and_python_runtimes() {
        let java = runtime_prerequisites_from_signals(&name_set(&["jvm.dll"]), None, false);
        assert!(
            java.iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::JavaRuntime)
        );
        let python = runtime_prerequisites_from_signals(&name_set(&["python311.dll"]), None, false);
        assert!(
            python
                .iter()
                .any(|prerequisite| prerequisite.kind == RuntimePrerequisiteKind::PythonRuntime)
        );
    }
}
