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
use crate::models::{
    AppSelection, DependencyEntry, RuntimePrerequisite, RuntimePrerequisiteKind,
    SCAN_SCHEMA_VERSION, ScanOptions, ScanReport, Warning, WarningSeverity,
};
use crate::path_utils::looks_like_executable;
use crate::pe_scan::scan_pe;
use crate::registry_scan::{RegistryScanContext, scan_registry};
use crate::reporting::{artifact_paths, write_scan_reports};
use crate::service_scan::{ServiceScanContext, scan_services};
use crate::shortcut_scan::{ShortcutScanContext, scan_shortcuts};
use crate::task_scan::{TaskScanContext, scan_scheduled_tasks};

const BASE_CONFIDENCE_SCORE: i32 = 100;
const MISSING_DLL_PENALTY: i32 = 8;
const RUNTIME_PREREQUISITE_PENALTY: i32 = 4;
const EXTERNAL_MANIFEST_PENALTY: i32 = 5;
const NON_EXE_EXTENSION_PENALTY: i32 = 3;
const MIN_CONFIDENCE_SCORE: i32 = 0;
const MAX_CONFIDENCE_SCORE: i32 = 100;

#[derive(Debug, Clone)]
struct ConfidenceInputs<'a> {
    dependencies: &'a [DependencyEntry],
    runtime_prerequisite_count: usize,
    external_manifest_detected: bool,
    non_exe_extension: bool,
    service_count: usize,
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

    let (file_inventory, file_warnings) = discover_files(&FileDiscoveryContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
        app_name: app_name.clone(),
        publisher: publisher.clone(),
        include_hashes: options.include_hashes,
    });
    let (registry_entries, registry_warnings) = scan_registry(&RegistryScanContext {
        executable_path: executable_path.clone(),
        install_location: install_location.clone(),
        display_name: app_name,
        publisher,
        source_registry_path,
    });
    let (services, service_warnings) = scan_services(&ServiceScanContext {
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

    let runtime_prerequisites = runtime_prerequisites(&dependencies);
    let mut warnings = pe_scan.warnings;
    warnings.extend(resolver_warnings);
    warnings.extend(file_warnings);
    warnings.extend(registry_warnings);
    warnings.extend(service_warnings);
    warnings.extend(task_warnings);
    warnings.extend(shortcut_warnings);
    warnings.extend(environment_warnings);
    warnings.extend(missing_dependency_warnings(&dependencies));

    let artifacts = artifact_paths(&options.output_dir);

    let confidence_score = calculate_confidence_score(ConfidenceInputs {
        dependencies: &dependencies,
        runtime_prerequisite_count: runtime_prerequisites.len(),
        external_manifest_detected: pe_scan.metadata.external_manifest.is_some(),
        non_exe_extension: !looks_like_executable(&executable_path),
        service_count: services.len(),
        scheduled_task_count: scheduled_tasks.len(),
        admin_registry_count: registry_entries
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
        file_inventory,
        dependencies,
        app_local_dlls,
        registry_entries,
        services,
        scheduled_tasks,
        shortcuts,
        environment_variables,
        runtime_prerequisites,
        warnings,
        items_excluded: vec![
            "Windows directory, System32, and SysWOW64 files are reported but not copied."
                .to_owned(),
            "Non-app-owned shared files outside the app directory are never copied; shared runtimes are reported as prerequisites."
                .to_owned(),
            "Registry, services, scheduled tasks, and shortcuts are archived for restore; apply mode remains explicit and privilege-gated."
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
        AppSelection::InstalledApp(app) => app.main_executable.clone().ok_or_else(|| {
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

fn runtime_prerequisites(dependencies: &[DependencyEntry]) -> Vec<RuntimePrerequisite> {
    let mut seen = BTreeSet::new();
    let mut prerequisites = Vec::new();

    for dependency in dependencies {
        if !is_runtime_prerequisite_dll(&dependency.name) {
            continue;
        }
        let key = dependency.name.to_ascii_lowercase();
        if seen.insert(key) {
            prerequisites.push(RuntimePrerequisite {
                name: "Microsoft Visual C++ Redistributable".to_owned(),
                kind: RuntimePrerequisiteKind::VisualCppRedistributable,
                evidence: format!("Imported DLL: {}", dependency.name),
                install_manually: true,
            });
        }
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
    score -= inputs.scheduled_task_count as i32 * 3;
    score -= inputs.admin_registry_count as i32 * 2;
    score -= inputs.files_not_copied_count.min(10) as i32;

    if inputs.external_manifest_detected {
        score -= EXTERNAL_MANIFEST_PENALTY;
    }
    if inputs.non_exe_extension {
        score -= NON_EXE_EXTENSION_PENALTY;
    }

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
            service_count: 0,
            scheduled_task_count: 0,
            admin_registry_count: 0,
            files_not_copied_count: 0,
        });

        assert_eq!(score, 92);
    }
}
