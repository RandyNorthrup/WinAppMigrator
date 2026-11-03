use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::errors::{Result, WinAppMigratorError};
use crate::models::{DependencyEntry, GeneratedArtifacts, ScanReport, Warning};

const SCAN_REPORT_JSON: &str = "scan_report.json";
const SCAN_REPORT_MARKDOWN: &str = "scan_report.md";
const DEPENDENCY_GRAPH_JSON: &str = "dependency_graph.json";
const WARNINGS_MARKDOWN: &str = "warnings.md";

pub fn artifact_paths(output_dir: &Path) -> GeneratedArtifacts {
    GeneratedArtifacts {
        scan_report_json: output_dir.join(SCAN_REPORT_JSON),
        scan_report_markdown: output_dir.join(SCAN_REPORT_MARKDOWN),
        dependency_graph_json: output_dir.join(DEPENDENCY_GRAPH_JSON),
        warnings_markdown: output_dir.join(WARNINGS_MARKDOWN),
    }
}

pub fn write_scan_reports(report: &ScanReport, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).map_err(|source| WinAppMigratorError::io(output_dir, source))?;
    let artifacts = report
        .generated_artifacts
        .clone()
        .unwrap_or_else(|| artifact_paths(output_dir));

    write_json(&artifacts.scan_report_json, report)?;
    write_text(
        &artifacts.scan_report_markdown,
        &render_markdown_scan_report(report),
    )?;
    write_json(
        &artifacts.dependency_graph_json,
        &DependencyGraph::from_report(report),
    )?;
    write_text(
        &artifacts.warnings_markdown,
        &render_warnings_markdown(&report.warnings),
    )?;

    Ok(())
}

/// Each part of a scan report, in the order they are in it.
const REPORT_SECTIONS: [fn(&mut String, &ScanReport); 25] = [
    write_summary,
    write_source_app,
    write_main_executable,
    write_architecture,
    write_install_location,
    write_install_source,
    write_data_locations,
    write_files_collected,
    write_dll_dependencies,
    write_missing_dependencies,
    write_runtime_prerequisites,
    write_license_data,
    write_registry_entries,
    write_com_registrations,
    write_file_associations,
    write_services,
    write_drivers_and_kernel_components,
    write_scheduled_tasks,
    write_shortcuts,
    write_environment_variables,
    write_items_excluded,
    write_warnings,
    write_prerequisites_manual_steps,
    write_restore_instructions,
    write_confidence_score,
];

pub fn render_markdown_scan_report(report: &ScanReport) -> String {
    let mut output = String::new();
    push_line(&mut output, "# WinAppMigrator Scan Report");
    push_line(&mut output, "");
    for write in REPORT_SECTIONS {
        write(&mut output, report);
    }
    output
}

fn write_summary(output: &mut String, report: &ScanReport) {
    push_line(output, "## Summary");
    push_line(
        output,
        &format!("- Generated UTC: {}", report.generated_at_utc),
    );
    push_line(
        output,
        &format!("- Confidence score: {}", report.confidence_score),
    );
    push_line(
        output,
        &format!("- Direct dependencies: {}", report.dependencies.len()),
    );
    push_line(
        output,
        &format!(
            "- Delay-loaded imports: {}",
            report
                .dependencies
                .iter()
                .filter(|dependency| dependency.delay_loaded)
                .count()
        ),
    );
    push_line(
        output,
        &format!("- File inventory entries: {}", report.file_inventory.len()),
    );
    push_line(
        output,
        &format!(
            "- Environment variables: {}",
            report.environment_variables.len()
        ),
    );
    push_line(
        output,
        &format!(
            "- Missing dependencies: {}",
            report
                .dependencies
                .iter()
                .filter(|dependency| dependency.missing)
                .count()
        ),
    );
    push_line(output, "");
}

fn write_source_app(output: &mut String, report: &ScanReport) {
    push_line(output, "## Source App");
    if let Some(app) = report.source_app.as_ref() {
        push_line(output, &format!("- Name: {}", app.display_name));
        push_line(
            output,
            &format!("- Publisher: {}", option_text(app.publisher.as_deref())),
        );
        push_line(
            output,
            &format!("- Version: {}", option_text(app.version.as_deref())),
        );
    } else {
        push_line(output, "- Source app: executable path scan");
    }
    push_line(output, "");
}

fn write_main_executable(output: &mut String, report: &ScanReport) {
    push_line(output, "## Main Executable");
    push_line(
        output,
        &format!("- Path: {}", report.main_executable.path.display()),
    );
    push_line(
        output,
        &format!(
            "- File size bytes: {}",
            report.main_executable.file_size_bytes
        ),
    );
    push_line(
        output,
        &format!("- Architecture: {:?}", report.architecture),
    );
    push_line(
        output,
        &format!(
            "- Digital signature: {:?}",
            report.main_executable.digital_signature
        ),
    );
    push_line(
        output,
        &format!("- Managed (.NET): {}", report.main_executable.is_dotnet),
    );
    push_line(
        output,
        &format!(
            "- Embedded manifest: {}",
            report.main_executable.embedded_manifest
        ),
    );
    if let Some(level) = report.main_executable.requested_execution_level.as_deref() {
        push_line(output, &format!("- Requested execution level: {level}"));
    }
    if !report.main_executable.sxs_dependencies.is_empty() {
        push_line(
            output,
            &format!(
                "- Side-by-side dependencies: {}",
                report.main_executable.sxs_dependencies.join(", ")
            ),
        );
    }
    push_line(output, "");
}

fn write_architecture(output: &mut String, report: &ScanReport) {
    push_line(output, "## Architecture");
    push_line(
        output,
        &format!("- PE architecture: {:?}", report.architecture),
    );
    push_line(output, "");
}

fn write_install_location(output: &mut String, report: &ScanReport) {
    push_line(output, "## Install Location");
    push_line(
        output,
        &format!(
            "- {}",
            report
                .install_location
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "Unknown".to_owned())
        ),
    );
    push_line(output, "");
}

fn write_install_source(output: &mut String, report: &ScanReport) {
    push_line(output, "## Install Source");
    push_line(
        output,
        &format!(
            "- Detected install method: {}",
            report
                .install_method
                .as_ref()
                .map(|method| method.label())
                .unwrap_or("Unknown")
        ),
    );
    if report.authoritative_file_count > 0 {
        push_line(
            output,
            &format!(
                "- Installer-recorded files: {}",
                report.authoritative_file_count
            ),
        );
    }
    for signal in &report.install_evidence {
        push_line(output, &format!("- Evidence: {signal}"));
    }
    push_line(output, "");
}

fn write_data_locations(output: &mut String, report: &ScanReport) {
    push_line(output, "## Data Locations");
    if report.footprint_locations.is_empty() {
        push_line(output, "- No additional data locations were found.");
    } else {
        push_line(output, "| Location | Kind | Scope | Files | Bytes |");
        push_line(output, "| --- | --- | --- | ---: | ---: |");
        for location in &report.footprint_locations {
            push_line(
                output,
                &format!(
                    "| {} | {} | {:?} | {} | {} |",
                    location.path.display(),
                    location.kind.label(),
                    location.scope,
                    location.file_count,
                    location.total_bytes
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_files_collected(output: &mut String, report: &ScanReport) {
    push_line(output, "## Files Collected");
    if report.file_inventory.is_empty() {
        push_line(output, "- No files discovered.");
    } else {
        push_line(output, "| Path | Classification | Copy? | Size | SHA-256 |");
        push_line(output, "| --- | --- | --- | ---: | --- |");
        for file in &report.file_inventory {
            push_line(
                output,
                &format!(
                    "| {} | {:?} | {} | {} | {} |",
                    file.original_path.display(),
                    file.classification,
                    file.should_copy,
                    file.size_bytes
                        .map(|size| size.to_string())
                        .unwrap_or_else(|| "Unknown".to_owned()),
                    file.hash_sha256.as_deref().unwrap_or("Not calculated")
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_dll_dependencies(output: &mut String, report: &ScanReport) {
    push_line(output, "## DLL Dependencies");
    push_dependency_table(output, &report.dependencies, false);
    push_line(output, "");
}

fn write_missing_dependencies(output: &mut String, report: &ScanReport) {
    push_line(output, "## Missing Dependencies");
    push_dependency_table(output, &report.dependencies, true);
    push_line(output, "");
}

fn write_runtime_prerequisites(output: &mut String, report: &ScanReport) {
    push_line(output, "## Runtime Prerequisites");
    if report.runtime_prerequisites.is_empty() {
        push_line(output, "- None detected.");
    } else {
        for prerequisite in &report.runtime_prerequisites {
            push_line(
                output,
                &format!(
                    "- {} ({:?}): {}",
                    prerequisite.name, prerequisite.kind, prerequisite.evidence
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_license_data(output: &mut String, report: &ScanReport) {
    push_line(output, "## License Data");
    if report.license_artifacts.is_empty() {
        push_line(output, "- No licensing data detected.");
    } else {
        push_line(
            output,
            "Licensing data may be machine-bound and require reactivation. WinAppMigrator never decrypts, bypasses activation, or emulates dongles.",
        );
        push_line(
            output,
            "| Framework | Kind | Location | Transferable | Machine-bound | Copied |",
        );
        push_line(output, "| --- | --- | --- | --- | --- | --- |");
        for artifact in &report.license_artifacts {
            let location = artifact
                .path
                .as_ref()
                .map(|path| path.display().to_string())
                .or_else(|| artifact.registry_location.clone())
                .unwrap_or_else(|| artifact.detail.clone());
            push_line(
                output,
                &format!(
                    "| {} | {:?} | {} | {:?} | {} | {} |",
                    artifact.framework.label(),
                    artifact.kind,
                    location,
                    artifact.transferable,
                    artifact.machine_bound,
                    artifact.should_copy
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_registry_entries(output: &mut String, report: &ScanReport) {
    push_line(output, "## Registry Entries");
    if report.registry_entries.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for entry in &report.registry_entries {
            push_line(
                output,
                &format!(
                    "- {:?}\\{} {} {:?}",
                    entry.hive,
                    entry.key_path,
                    entry.value_name.as_deref().unwrap_or(""),
                    entry.confidence
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_com_registrations(output: &mut String, report: &ScanReport) {
    push_line(output, "## COM Registrations");
    if report.com_registrations.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for registration in &report.com_registrations {
            push_line(
                output,
                &format!(
                    "- {:?} {} {:?} -> {}",
                    registration.hive,
                    registration.clsid,
                    registration.server_kind,
                    registration
                        .server_path
                        .as_ref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "Unknown server".to_owned())
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_file_associations(output: &mut String, report: &ScanReport) {
    push_line(output, "## File Associations");
    if report.file_associations.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for association in &report.file_associations {
            push_line(
                output,
                &format!(
                    "- {:?} extension={} prog_id={} app={} verb={}",
                    association.hive,
                    association.extension.as_deref().unwrap_or("Unknown"),
                    association.prog_id.as_deref().unwrap_or("Unknown"),
                    association.application_name.as_deref().unwrap_or("Unknown"),
                    association.verb.as_deref().unwrap_or("Unknown")
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_services(output: &mut String, report: &ScanReport) {
    push_line(output, "## Services");
    if report.services.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for service in &report.services {
            push_line(
                output,
                &format!(
                    "- {} -> {}",
                    service.service_name,
                    service.binary_path.display()
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_drivers_and_kernel_components(output: &mut String, report: &ScanReport) {
    push_line(output, "## Drivers And Kernel Components");
    if report.drivers.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for driver in &report.drivers {
            push_line(
                output,
                &format!(
                    "- {} ({:?}) -> {}",
                    driver.service_name,
                    driver.driver_kind,
                    driver.image_path.display()
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_scheduled_tasks(output: &mut String, report: &ScanReport) {
    push_line(output, "## Scheduled Tasks");
    if report.scheduled_tasks.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for task in &report.scheduled_tasks {
            push_line(
                output,
                &format!(
                    "- {}{} -> {}",
                    task.task_path,
                    task.task_name,
                    task.action_command.display()
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_shortcuts(output: &mut String, report: &ScanReport) {
    push_line(output, "## Shortcuts");
    if report.shortcuts.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for shortcut in &report.shortcuts {
            push_line(
                output,
                &format!(
                    "- {} -> {}",
                    shortcut.original_lnk_path.display(),
                    shortcut.target_path.display()
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_environment_variables(output: &mut String, report: &ScanReport) {
    push_line(output, "## Environment Variables");
    if report.environment_variables.is_empty() {
        push_line(output, "- None discovered.");
    } else {
        for variable in &report.environment_variables {
            push_line(
                output,
                &format!(
                    "- {:?} `{}` = `{}`",
                    variable.scope, variable.name, variable.value
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_items_excluded(output: &mut String, report: &ScanReport) {
    push_line(output, "## Items Excluded");
    if report.items_excluded.is_empty() {
        push_line(output, "- None recorded.");
    } else {
        for item in &report.items_excluded {
            push_line(output, &format!("- {item}"));
        }
    }
    push_line(output, "");
}

fn write_warnings(output: &mut String, report: &ScanReport) {
    push_line(output, "## Warnings");
    if report.warnings.is_empty() {
        push_line(output, "- None.");
    } else {
        for warning in &report.warnings {
            push_line(
                output,
                &format!(
                    "- {:?} `{}`: {}",
                    warning.severity, warning.code, warning.message
                ),
            );
        }
    }
    push_line(output, "");
}

fn write_prerequisites_manual_steps(output: &mut String, report: &ScanReport) {
    push_line(output, "## Prerequisites & Manual Steps");
    if report.runtime_prerequisites.is_empty()
        && report.dependencies.iter().all(|entry| !entry.missing)
    {
        push_line(output, "- None detected by scanning.");
    } else {
        push_line(
            output,
            "- Detected runtime prerequisites are bundled into the installer (or installed via winget), and checked on the target where a check exists.",
        );
        push_line(
            output,
            "- Investigate any unresolved dependencies before migration.",
        );
    }
    push_line(output, "");
}

fn write_restore_instructions(output: &mut String, _report: &ScanReport) {
    push_line(output, "## Restore Instructions");
    push_line(
        output,
        "- Run the produced installer on the target PC. Use its Preview changes button first, or run it with `--target-root <DIR>` and no `--apply`.",
    );
    push_line(output, "");
}

fn write_confidence_score(output: &mut String, report: &ScanReport) {
    push_line(output, "## Confidence Score");
    push_line(
        output,
        &format!("- Score: {} / 100", report.confidence_score),
    );
}

pub fn render_warnings_markdown(warnings: &[Warning]) -> String {
    let mut output = String::new();
    push_line(&mut output, "# WinAppMigrator Warnings");
    push_line(&mut output, "");
    if warnings.is_empty() {
        push_line(&mut output, "- No warnings.");
        return output;
    }

    for warning in warnings {
        push_line(
            &mut output,
            &format!(
                "- {:?} `{}`: {}{}",
                warning.severity,
                warning.code,
                warning.message,
                warning
                    .source
                    .as_ref()
                    .map(|source| format!(" Source: {source}"))
                    .unwrap_or_default()
            ),
        );
    }
    output
}

fn push_dependency_table(
    output: &mut String,
    dependencies: &[DependencyEntry],
    missing_only: bool,
) {
    let filtered: Vec<&DependencyEntry> = dependencies
        .iter()
        .filter(|dependency| !missing_only || dependency.missing)
        .collect();
    if filtered.is_empty() {
        push_line(output, "- None.");
        return;
    }

    push_line(
        output,
        "| Name | Source | Resolved path | Copy? | Runtime? | Missing? |",
    );
    push_line(output, "| --- | --- | --- | --- | --- | --- |");
    for dependency in filtered {
        push_line(
            output,
            &format!(
                "| {} | {:?} | {} | {} | {} | {} |",
                dependency.name,
                dependency.source_category,
                dependency
                    .resolved_path
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "Unresolved".to_owned()),
                dependency.should_copy,
                dependency.runtime_prerequisite,
                dependency.missing
            ),
        );
    }
}

fn option_text(value: Option<&str>) -> &str {
    value.unwrap_or("Unknown")
}

fn push_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes).map_err(|source| WinAppMigratorError::io(path, source))
}

fn write_text(path: &Path, value: &str) -> Result<()> {
    fs::write(path, value).map_err(|source| WinAppMigratorError::io(path, source))
}

#[derive(Debug, Clone, Serialize)]
struct DependencyGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize)]
struct GraphNode {
    id: String,
    label: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize)]
struct GraphEdge {
    source: String,
    target: String,
    relation: String,
}

impl DependencyGraph {
    fn from_report(report: &ScanReport) -> Self {
        let executable_id = report.main_executable.path.display().to_string();
        let mut nodes = vec![GraphNode {
            id: executable_id.clone(),
            label: report.main_executable.file_name.clone(),
            kind: "executable".to_owned(),
        }];
        let mut edges = Vec::new();

        for dependency in &report.dependencies {
            let dependency_id = dependency
                .resolved_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| dependency.name.clone());
            nodes.push(GraphNode {
                id: dependency_id.clone(),
                label: dependency.name.clone(),
                kind: if dependency.missing {
                    "missing_dll".to_owned()
                } else {
                    "dll".to_owned()
                },
            });
            edges.push(GraphEdge {
                source: executable_id.clone(),
                target: dependency_id,
                relation: "imports".to_owned(),
            });
        }

        Self { nodes, edges }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        AppArchitecture, DependencySourceCategory, ExecutableMetadata, RuntimePrerequisite,
        SCAN_SCHEMA_VERSION, SignatureStatus,
    };

    use super::*;

    #[test]
    fn renders_markdown_report_sections() {
        let report = ScanReport {
            account_traces: Default::default(),
            schema_version: SCAN_SCHEMA_VERSION.to_owned(),
            generated_at_utc: "2026-06-12T00:00:00Z".to_owned(),
            source_volume: None,
            other_accounts: Default::default(),
            source_computer_name: None,
            source_windows_version: None,
            source_platform: None,
            program_needs: Vec::new(),
            user_data_profile: None,
            source_app: None,
            main_executable: ExecutableMetadata {
                path: "C:/App/App.exe".into(),
                file_name: "App.exe".to_owned(),
                file_size_bytes: 10,
                modified_utc: None,
                version_info: None,
                digital_signature: SignatureStatus::NotPresent,
                external_manifest: None,
                is_dotnet: false,
                dotnet_runtime_version: None,
                dotnet_target_framework: None,
                embedded_manifest: false,
                requested_execution_level: None,
                sxs_dependencies: Vec::new(),
            },
            architecture: AppArchitecture::X64,
            install_location: None,
            install_method: None,
            authoritative_file_count: 0,
            install_evidence: Vec::new(),
            footprint_locations: Vec::new(),
            file_inventory: Vec::new(),
            dependencies: vec![DependencyEntry {
                name: "custom.dll".to_owned(),
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
            }],
            app_local_dlls: Vec::new(),
            registry_entries: Vec::new(),
            com_registrations: Vec::new(),
            file_associations: Vec::new(),
            services: Vec::new(),
            drivers: Vec::new(),
            scheduled_tasks: Vec::new(),
            shortcuts: Vec::new(),
            environment_variables: Vec::new(),
            runtime_prerequisites: Vec::<RuntimePrerequisite>::new(),
            license_artifacts: Vec::new(),
            license_registry_entries: Vec::new(),
            secrets_left_out: Vec::new(),
            licenses_left_out: Vec::new(),
            settings_left_out: Vec::new(),
            empty_folders: Vec::new(),
            firewall_rules: Vec::new(),
            certificates: Vec::new(),
            fonts_reported: Vec::new(),
            near_matches: Vec::new(),
            warnings: Vec::new(),
            items_excluded: Vec::new(),
            confidence_score: 92,
            generated_artifacts: None,
        };

        let markdown = render_markdown_scan_report(&report);

        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("## Missing Dependencies"));
        assert!(markdown.contains("custom.dll"));
    }
}
