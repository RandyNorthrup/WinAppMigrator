use std::fs;
use std::path::Path;

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::app_inventory::list_installed_apps;
use crate::errors::{Result, WinAppMigratorError};
use crate::models::{
    INVENTORY_SNAPSHOT_SCHEMA_VERSION, InstallScope, InventorySnapshotArtifacts,
    InventorySnapshotOptions, InventorySnapshotReport,
};

const SNAPSHOT_JSON: &str = "inventory_snapshot.json";
const SNAPSHOT_MARKDOWN: &str = "inventory_snapshot.md";

pub fn write_inventory_snapshot(
    options: &InventorySnapshotOptions,
) -> Result<InventorySnapshotReport> {
    fs::create_dir_all(&options.output_dir)
        .map_err(|source| WinAppMigratorError::io(&options.output_dir, source))?;
    let artifacts = InventorySnapshotArtifacts {
        json: options.output_dir.join(SNAPSHOT_JSON),
        markdown: options.output_dir.join(SNAPSHOT_MARKDOWN),
    };
    let applications = list_installed_apps()?;
    let report = InventorySnapshotReport {
        schema_version: INVENTORY_SNAPSHOT_SCHEMA_VERSION.to_owned(),
        generated_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
        applications,
        warnings: Vec::new(),
        artifacts,
    };

    write_json(&report.artifacts.json, &report)?;
    write_markdown(
        &report.artifacts.markdown,
        &render_snapshot_markdown(&report),
    )?;
    Ok(report)
}

fn render_snapshot_markdown(report: &InventorySnapshotReport) -> String {
    let mut output = String::new();
    output.push_str("# WinAppMigrator Inventory Snapshot\n\n");
    output.push_str(&format!("- Generated UTC: {}\n", report.generated_at_utc));
    output.push_str(&format!("- Applications: {}\n", report.applications.len()));
    output.push_str(&format!(
        "- Per-user: {}\n",
        report
            .applications
            .iter()
            .filter(|app| app.install_scope == InstallScope::PerUser)
            .count()
    ));
    output.push_str(&format!(
        "- Per-machine: {}\n",
        report
            .applications
            .iter()
            .filter(|app| app.install_scope == InstallScope::PerMachine)
            .count()
    ));
    output.push_str("\n## Applications\n");
    output.push_str("| Name | Version | Publisher | Scope | Main executable |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for app in &report.applications {
        output.push_str(&format!(
            "| {} | {} | {} | {:?} | {} |\n",
            escape_markdown_cell(&app.display_name),
            escape_markdown_cell(app.version.as_deref().unwrap_or("Unknown")),
            escape_markdown_cell(app.publisher.as_deref().unwrap_or("Unknown")),
            app.install_scope,
            escape_markdown_cell(
                &app.main_executable
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            ),
        ));
    }
    output
}

fn write_json(path: &Path, report: &InventorySnapshotReport) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(report)?;
    fs::write(path, bytes).map_err(|source| WinAppMigratorError::io(path, source))
}

fn write_markdown(path: &Path, value: &str) -> Result<()> {
    fs::write(path, value).map_err(|source| WinAppMigratorError::io(path, source))
}

fn escape_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppArchitecture, InstalledApp};

    #[test]
    fn renders_snapshot_markdown_rows() {
        let report = InventorySnapshotReport {
            schema_version: INVENTORY_SNAPSHOT_SCHEMA_VERSION.to_owned(),
            generated_at_utc: "2026-06-19T00:00:00Z".to_owned(),
            applications: vec![InstalledApp {
                display_name: "Example App".to_owned(),
                publisher: Some("Example Co".to_owned()),
                version: Some("1.0".to_owned()),
                install_location: None,
                uninstall_string: None,
                main_executable: Some(r"C:\Example\App.exe".into()),
                registry_source_path: r"HKCU\Software\Example".to_owned(),
                architecture: AppArchitecture::X64,
                install_scope: InstallScope::PerUser,
            }],
            warnings: Vec::new(),
            artifacts: InventorySnapshotArtifacts {
                json: "inventory_snapshot.json".into(),
                markdown: "inventory_snapshot.md".into(),
            },
        };

        let markdown = render_snapshot_markdown(&report);

        assert!(markdown.contains("Example App"));
        assert!(markdown.contains("Per-user: 1"));
    }
}
