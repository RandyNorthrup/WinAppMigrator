use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use tracing::{debug, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};
use winappmigrator_core::{
    AppSelection, InstalledApp, InventorySnapshotOptions, PackageArchiveFormat, PackageOptions,
    RestoreOptions, Result, ScanOptions, VerificationOptions, WinAppMigratorError, create_package,
    extract_embedded_self_archive, has_embedded_self_archive, list_installed_apps, restore_package,
    scan_and_write_reports, verify_package, write_inventory_snapshot,
};

const EXIT_SUCCESS: u8 = 0;
const DISPLAY_SEPARATOR: &str =
    "--------------------------------------------------------------------------------";

#[derive(Debug, Parser)]
#[command(name = "winappmigrator")]
#[command(about = "Analyze installed Windows desktop applications for migration planning.")]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub logging: LoggingArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub struct LoggingArgs {
    #[arg(long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    #[arg(long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(long, global = true, value_name = "PATH")]
    pub log_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List installed desktop applications from uninstall registry entries.
    List,

    /// Scan an executable or installed application name and write reports.
    Scan(ScanCommand),

    /// Write a read-only installed-application inventory snapshot.
    Snapshot(SnapshotCommand),

    /// Build a migration package directory.
    Package(PackageCommand),

    /// Restore a migration package. Defaults to dry-run unless --apply is supplied.
    Restore(RestoreCommand),

    /// Verify a package and optionally verify an applied restore target.
    Verify(VerifyCommand),
}

#[derive(Debug, Args)]
pub struct ScanCommand {
    #[arg(long = "exe", value_name = "PATH", conflicts_with = "name")]
    pub executable_path: Option<PathBuf>,

    #[arg(long, value_name = "DISPLAY_NAME", conflicts_with = "executable_path")]
    pub name: Option<String>,

    #[arg(long, value_name = "DIR")]
    pub out: PathBuf,

    #[arg(long)]
    pub yes: bool,

    #[arg(long = "non-interactive")]
    pub non_interactive: bool,

    /// Exclude per-user and ProgramData application data from the scan.
    #[arg(long = "skip-user-data")]
    pub skip_user_data: bool,
}

#[derive(Debug, Args)]
pub struct SnapshotCommand {
    #[arg(long, value_name = "DIR")]
    pub out: PathBuf,
}

#[derive(Debug, Args)]
pub struct PackageCommand {
    #[arg(long = "exe", value_name = "PATH", conflicts_with = "name")]
    pub executable_path: Option<PathBuf>,

    #[arg(long, value_name = "DISPLAY_NAME", conflicts_with = "executable_path")]
    pub name: Option<String>,

    #[arg(long, value_name = "DIR")]
    pub out: PathBuf,

    #[arg(long)]
    pub yes: bool,

    #[arg(long = "non-interactive")]
    pub non_interactive: bool,

    #[arg(long, value_enum)]
    pub archive: Option<ArchiveFormatArg>,

    #[arg(long)]
    pub sfx: bool,

    /// Exclude per-user and ProgramData application data from the package.
    #[arg(long = "skip-user-data")]
    pub skip_user_data: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ArchiveFormatArg {
    Zip,
}

#[derive(Debug, Args)]
pub struct RestoreCommand {
    #[arg(long = "package", value_name = "DIR")]
    pub package_root: PathBuf,

    #[arg(long = "target-root", value_name = "DIR")]
    pub target_root: PathBuf,

    #[arg(long, conflicts_with = "apply")]
    pub dry_run: bool,

    #[arg(long, conflicts_with = "dry_run")]
    pub apply: bool,

    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct VerifyCommand {
    #[arg(long = "package", value_name = "DIR_OR_ZIP")]
    pub package_root: PathBuf,

    #[arg(long = "target-root", value_name = "DIR")]
    pub target_root: Option<PathBuf>,

    #[arg(long = "out", value_name = "DIR")]
    pub output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(name = "winappmigrator-sfx")]
#[command(about = "Restore an embedded WinAppMigrator self-extracting package.")]
pub struct SfxCli {
    #[arg(long = "target-root", value_name = "DIR")]
    pub target_root: Option<PathBuf>,

    #[arg(long, conflicts_with = "apply")]
    pub dry_run: bool,

    #[arg(long, conflicts_with = "dry_run")]
    pub apply: bool,

    #[arg(long)]
    pub force: bool,
}

pub fn run<I, T>(args: I) -> Result<u8>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let raw_args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    if should_run_sfx_mode(&raw_args)? {
        return run_sfx(raw_args);
    }

    let cli = Cli::parse_from(raw_args);
    let _log_guard = init_logging(&cli.logging)?;
    debug!(?cli, "parsed CLI arguments");

    match cli.command {
        Commands::List => {
            run_list()?;
            Ok(EXIT_SUCCESS)
        }
        Commands::Scan(command) => {
            run_scan(command)?;
            Ok(EXIT_SUCCESS)
        }
        Commands::Snapshot(command) => {
            run_snapshot(command)?;
            Ok(EXIT_SUCCESS)
        }
        Commands::Package(command) => {
            run_package(command)?;
            Ok(EXIT_SUCCESS)
        }
        Commands::Restore(command) => {
            run_restore(command)?;
            Ok(EXIT_SUCCESS)
        }
        Commands::Verify(command) => {
            run_verify(command)?;
            Ok(EXIT_SUCCESS)
        }
    }
}

fn run_list() -> Result<()> {
    let apps = list_installed_apps()?;
    println!("Installed applications: {}", apps.len());
    for app in apps {
        println!("{DISPLAY_SEPARATOR}");
        println!("Name: {}", app.display_name);
        println!("Publisher: {}", optional_display(app.publisher.as_deref()));
        println!("Version: {}", optional_display(app.version.as_deref()));
        println!(
            "Install location: {}",
            optional_path_display(app.install_location.as_ref())
        );
        println!(
            "Uninstall string: {}",
            optional_display(app.uninstall_string.as_deref())
        );
        println!(
            "Main executable guess: {}",
            optional_path_display(app.main_executable.as_ref())
        );
        println!("Registry source: {}", app.registry_source_path);
        println!("Architecture: {:?}", app.architecture);
        println!("Install scope: {:?}", app.install_scope);
    }
    Ok(())
}

fn run_snapshot(command: SnapshotCommand) -> Result<()> {
    let report = write_inventory_snapshot(&InventorySnapshotOptions {
        output_dir: command.out,
    })?;

    println!("Inventory snapshot complete.");
    println!("Applications: {}", report.applications.len());
    println!("Snapshot JSON: {}", report.artifacts.json.display());
    println!("Snapshot Markdown: {}", report.artifacts.markdown.display());
    Ok(())
}

fn run_scan(command: ScanCommand) -> Result<()> {
    let selection = resolve_scan_selection(&command)?;
    let mut options = ScanOptions {
        selection,
        output_dir: command.out,
        current_directory: std::env::current_dir().ok(),
        recursive_dependencies: true,
        include_hashes: true,
        include_user_data: !command.skip_user_data,
    };

    if let AppSelection::ExecutablePath(path) = &options.selection {
        info!(path = %path.display(), "starting executable scan");
    }
    let report = scan_and_write_reports(&options)?;
    options.output_dir = report
        .generated_artifacts
        .as_ref()
        .and_then(|artifacts| artifacts.scan_report_json.parent().map(PathBuf::from))
        .unwrap_or_else(|| options.output_dir.clone());

    println!("Scan complete.");
    println!("Executable: {}", report.main_executable.path.display());
    println!("Architecture: {:?}", report.architecture);
    println!("Dependencies: {}", report.dependencies.len());
    println!(
        "Missing dependencies: {}",
        report
            .dependencies
            .iter()
            .filter(|dependency| dependency.missing)
            .count()
    );
    println!("Warnings: {}", report.warnings.len());
    println!("Confidence score: {}", report.confidence_score);
    if let Some(artifacts) = report.generated_artifacts {
        println!("Report JSON: {}", artifacts.scan_report_json.display());
        println!(
            "Report Markdown: {}",
            artifacts.scan_report_markdown.display()
        );
        println!(
            "Dependency graph: {}",
            artifacts.dependency_graph_json.display()
        );
        println!("Warnings: {}", artifacts.warnings_markdown.display());
    }

    Ok(())
}

fn run_package(command: PackageCommand) -> Result<()> {
    let selection = resolve_package_selection(&command)?;
    let options = PackageOptions {
        selection,
        output_dir: command.out,
        current_directory: std::env::current_dir().ok(),
        archive_format: command.archive.map(Into::into),
        create_self_extracting_archive: command.sfx,
        self_extracting_stub: if command.sfx {
            Some(
                std::env::current_exe()
                    .map_err(|source| WinAppMigratorError::io("current executable path", source))?,
            )
        } else {
            None
        },
        include_user_data: !command.skip_user_data,
    };
    let result = create_package(&options)?;

    println!("Package complete.");
    println!("Package root: {}", result.package_root.display());
    println!("Manifest: {}", result.manifest_path.display());
    println!("Reports: {}", result.reports_dir.display());
    println!("Files copied: {}", result.files_copied);
    println!("Warnings: {}", result.warnings.len());
    if let Some(path) = result.archive_path {
        println!("Archive: {}", path.display());
    }
    if let Some(path) = result.self_extracting_archive_path {
        println!("Self-extracting archive: {}", path.display());
    }
    Ok(())
}

fn run_restore(command: RestoreCommand) -> Result<()> {
    let options = RestoreOptions {
        package_root: command.package_root,
        target_root: command.target_root,
        dry_run: command.dry_run || !command.apply,
        apply: command.apply,
        force: command.force,
    };
    let result = restore_package(&options)?;

    println!("Restore complete.");
    println!("Applied: {}", result.applied);
    println!("Changes logged: {}", result.changes_logged.len());
    println!("Warnings: {}", result.warnings.len());
    println!("Errors: {}", result.errors.len());
    if let Some(path) = result.restore_log_json {
        println!("Restore JSON log: {}", path.display());
    }
    if let Some(path) = result.restore_log_markdown {
        println!("Restore Markdown log: {}", path.display());
    }
    Ok(())
}

fn run_verify(command: VerifyCommand) -> Result<()> {
    let report = verify_package(&VerificationOptions {
        package_root: command.package_root,
        target_root: command.target_root,
        output_dir: command.output_dir,
    })?;

    println!("Verification complete.");
    println!("Package valid: {}", report.package_valid);
    println!(
        "Restore valid: {}",
        report
            .restore_valid
            .map(|value| value.to_string())
            .unwrap_or_else(|| "not run".to_owned())
    );
    println!("Items checked: {}", report.items.len());
    println!("Warnings: {}", report.warnings.len());
    println!("Errors: {}", report.errors.len());
    if let Some(path) = report.report_json {
        println!("Verification JSON: {}", path.display());
    }
    if let Some(path) = report.report_markdown {
        println!("Verification Markdown: {}", path.display());
    }
    Ok(())
}

fn resolve_scan_selection(command: &ScanCommand) -> Result<AppSelection> {
    if let Some(path) = command.executable_path.as_ref() {
        return Ok(AppSelection::ExecutablePath(path.clone()));
    }

    if let Some(name) = command.name.as_ref() {
        let matches = find_apps_by_name(name)?;
        return choose_app_match(name, matches, command.yes || command.non_interactive);
    }

    Err(WinAppMigratorError::InvalidInput(
        "scan requires either --exe or --name".to_owned(),
    ))
}

fn resolve_package_selection(command: &PackageCommand) -> Result<AppSelection> {
    if let Some(path) = command.executable_path.as_ref() {
        return Ok(AppSelection::ExecutablePath(path.clone()));
    }

    if let Some(name) = command.name.as_ref() {
        let matches = find_apps_by_name(name)?;
        return choose_app_match(name, matches, command.yes || command.non_interactive);
    }

    Err(WinAppMigratorError::InvalidInput(
        "package requires either --exe or --name".to_owned(),
    ))
}

fn find_apps_by_name(name: &str) -> Result<Vec<InstalledApp>> {
    let needle = name.to_ascii_lowercase();
    let matches = list_installed_apps()?
        .into_iter()
        .filter(|app| app.display_name.to_ascii_lowercase().contains(&needle))
        .collect();
    Ok(matches)
}

fn choose_app_match(
    name: &str,
    matches: Vec<InstalledApp>,
    non_interactive: bool,
) -> Result<AppSelection> {
    match matches.len() {
        0 => Err(WinAppMigratorError::InvalidInput(format!(
            "no installed app matched name: {name}"
        ))),
        1 => {
            let app = matches.into_iter().next().ok_or_else(|| {
                WinAppMigratorError::InvalidInput("no app match available".to_owned())
            })?;
            Ok(AppSelection::InstalledApp(Box::new(app)))
        }
        _ if non_interactive => Err(WinAppMigratorError::InvalidInput(format!(
            "multiple installed apps matched name '{name}'; rerun interactively or use --exe"
        ))),
        _ => choose_interactively(matches),
    }
}

fn choose_interactively(matches: Vec<InstalledApp>) -> Result<AppSelection> {
    println!("Multiple matches found:");
    for (index, app) in matches.iter().enumerate() {
        println!(
            "{}. {} ({})",
            index + 1,
            app.display_name,
            optional_display(app.publisher.as_deref())
        );
    }
    print!("Choose an application number: ");
    io::stdout()
        .flush()
        .map_err(|source| WinAppMigratorError::io("stdout", source))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|source| WinAppMigratorError::io("stdin", source))?;
    let selected = input.trim().parse::<usize>().map_err(|error| {
        WinAppMigratorError::InvalidInput(format!("invalid selection number: {error}"))
    })?;

    if selected == 0 || selected > matches.len() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "selection out of range: {selected}"
        )));
    }

    let app = matches
        .into_iter()
        .nth(selected - 1)
        .ok_or_else(|| WinAppMigratorError::InvalidInput("selected app missing".to_owned()))?;
    Ok(AppSelection::InstalledApp(Box::new(app)))
}

fn init_logging(args: &LoggingArgs) -> Result<Option<WorkerGuard>> {
    let level = if args.quiet {
        LevelFilter::WARN
    } else if args.verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();
    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(filter);

    if let Some(log_file) = args.log_file.as_ref() {
        if let Some(parent) = log_file.parent() {
            fs::create_dir_all(parent).map_err(|source| WinAppMigratorError::io(parent, source))?;
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .map_err(|source| WinAppMigratorError::io(log_file, source))?;
        let (writer, guard) = tracing_appender::non_blocking(file);
        let file_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(writer);
        let subscriber = Registry::default().with(stderr_layer).with(file_layer);
        tracing::subscriber::set_global_default(subscriber).map_err(|error| {
            WinAppMigratorError::InvalidInput(format!("failed to initialize logging: {error}"))
        })?;
        return Ok(Some(guard));
    }

    let subscriber = Registry::default().with(stderr_layer);
    tracing::subscriber::set_global_default(subscriber).map_err(|error| {
        WinAppMigratorError::InvalidInput(format!("failed to initialize logging: {error}"))
    })?;
    Ok(None)
}

fn optional_display(value: Option<&str>) -> &str {
    value.unwrap_or("Unknown")
}

fn optional_path_display(value: Option<&PathBuf>) -> String {
    value
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Unknown".to_owned())
}

fn should_run_sfx_mode(args: &[OsString]) -> Result<bool> {
    let exe_path = std::env::current_exe()
        .map_err(|source| WinAppMigratorError::io("current executable path", source))?;
    if !has_embedded_self_archive(&exe_path)? {
        return Ok(false);
    }
    let first_user_arg = args.get(1).and_then(|arg| arg.to_str());
    Ok(!matches!(
        first_user_arg,
        Some(
            "list"
                | "scan"
                | "snapshot"
                | "package"
                | "restore"
                | "verify"
                | "--help"
                | "-h"
                | "--version"
                | "-V",
        )
    ))
}

fn run_sfx(raw_args: Vec<OsString>) -> Result<u8> {
    let sfx = SfxCli::parse_from(raw_args);
    let Some(target_root) = sfx.target_root else {
        println!("WinAppMigrator self-extracting package detected.");
        println!("Dry-run restore: --target-root <DIR>");
        println!("Apply restore: --target-root <DIR> --apply");
        return Ok(EXIT_SUCCESS);
    };
    let executable = std::env::current_exe()
        .map_err(|source| WinAppMigratorError::io("current executable path", source))?;
    let temp_dir = tempfile::tempdir()
        .map_err(|source| WinAppMigratorError::io("temporary extraction directory", source))?;
    let extracted = extract_embedded_self_archive(&executable, temp_dir.path())?;
    if !extracted {
        return Err(WinAppMigratorError::InvalidInput(
            "current executable does not contain an embedded package".to_owned(),
        ));
    }
    let result = restore_package(&RestoreOptions {
        package_root: temp_dir.path().to_path_buf(),
        target_root,
        dry_run: sfx.dry_run || !sfx.apply,
        apply: sfx.apply,
        force: sfx.force,
    })?;
    println!("SFX restore complete.");
    println!("Applied: {}", result.applied);
    println!("Changes logged: {}", result.changes_logged.len());
    println!("Warnings: {}", result.warnings.len());
    println!("Errors: {}", result.errors.len());
    Ok(EXIT_SUCCESS)
}

impl From<ArchiveFormatArg> for PackageArchiveFormat {
    fn from(value: ArchiveFormatArg) -> Self {
        match value {
            ArchiveFormatArg::Zip => PackageArchiveFormat::Zip,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_scan_by_exe() -> Result<()> {
        let cli = Cli::try_parse_from([
            "winappmigrator",
            "scan",
            "--exe",
            r"C:\App\App.exe",
            "--out",
            r"C:\Out",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        match cli.command {
            Commands::Scan(scan) => {
                assert_eq!(scan.executable_path, Some(PathBuf::from(r"C:\App\App.exe")));
                assert_eq!(scan.out, PathBuf::from(r"C:\Out"));
            }
            _ => {
                return Err(WinAppMigratorError::InvalidInput(
                    "expected scan command".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn parses_snapshot_command() -> Result<()> {
        let cli = Cli::try_parse_from(["winappmigrator", "snapshot", "--out", r"C:\Out"])
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        match cli.command {
            Commands::Snapshot(snapshot) => {
                assert_eq!(snapshot.out, PathBuf::from(r"C:\Out"));
            }
            _ => {
                return Err(WinAppMigratorError::InvalidInput(
                    "expected snapshot command".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn parses_restore_default_dry_run() -> Result<()> {
        let cli = Cli::try_parse_from([
            "winappmigrator",
            "restore",
            "--package",
            r"C:\Pkg",
            "--target-root",
            r"C:\Restored",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        match cli.command {
            Commands::Restore(restore) => {
                assert_eq!(restore.package_root, PathBuf::from(r"C:\Pkg"));
                assert!(!restore.apply);
            }
            _ => {
                return Err(WinAppMigratorError::InvalidInput(
                    "expected restore command".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn parses_package_archive_and_sfx_flags() -> Result<()> {
        let cli = Cli::try_parse_from([
            "winappmigrator",
            "package",
            "--exe",
            r"C:\App\App.exe",
            "--out",
            r"C:\Out\AppPackage",
            "--archive",
            "zip",
            "--sfx",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        match cli.command {
            Commands::Package(package) => {
                assert!(matches!(package.archive, Some(ArchiveFormatArg::Zip)));
                assert!(package.sfx);
            }
            _ => {
                return Err(WinAppMigratorError::InvalidInput(
                    "expected package command".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn parses_verify_command() -> Result<()> {
        let cli = Cli::try_parse_from([
            "winappmigrator",
            "verify",
            "--package",
            r"C:\Pkg\App.zip",
            "--target-root",
            r"C:\Restored",
            "--out",
            r"C:\Verify",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        match cli.command {
            Commands::Verify(verify) => {
                assert_eq!(verify.package_root, PathBuf::from(r"C:\Pkg\App.zip"));
                assert_eq!(verify.target_root, Some(PathBuf::from(r"C:\Restored")));
                assert_eq!(verify.output_dir, Some(PathBuf::from(r"C:\Verify")));
            }
            _ => {
                return Err(WinAppMigratorError::InvalidInput(
                    "expected verify command".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn parses_global_log_flags_after_subcommand() -> Result<()> {
        let cli = Cli::try_parse_from([
            "winappmigrator",
            "list",
            "--verbose",
            "--log-file",
            r"C:\Temp\wam.log",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        assert!(cli.logging.verbose);
        assert_eq!(
            cli.logging.log_file,
            Some(PathBuf::from(r"C:\Temp\wam.log"))
        );
        Ok(())
    }
}
