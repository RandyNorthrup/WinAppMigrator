//! Headless (command-line) restore, used by automation, CI, and the e2e test.
//!
//! Extract the embedded package, restore it to `--target-root`, or to where the manifest
//! says the app was if that can be used, and hash-verify. If anything fails, what was done
//! is put back. License restore is on by default (opt out with `--no-restore-license`);
//! user-data restore is opt-in (`--restore-user-data`).

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use winappmigrator_core::journal::machine_state_root;
use winappmigrator_core::logging;
use winappmigrator_core::rollback::report_lines;
use winappmigrator_core::{
    EmbeddedPackage, InstallMonitor, PackageManifest, RestoreOptions, Result, VerificationOptions,
    WinAppMigratorError, default_install_location, has_embedded_self_archive,
    install_location_problem, read_embedded_manifest, restore_package_monitored, verify_package,
};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;

/// Arguments accepted by the installer when driven headlessly.
#[derive(Debug, Parser)]
#[command(name = "winappmigrator-installer")]
#[command(about = "Reinstall an embedded WinAppMigrator application package and hash-verify it.")]
#[command(version)]
pub struct InstallerCli {
    /// Where the application is reinstalled. Defaults to where it was, if that can be used.
    #[arg(long = "target-root", value_name = "DIR")]
    pub target_root: Option<PathBuf>,

    #[arg(long, conflicts_with = "apply")]
    pub dry_run: bool,

    #[arg(long, conflicts_with = "dry_run")]
    pub apply: bool,

    #[arg(long)]
    pub force: bool,

    /// Restore packaged user/ProgramData data to the real %AppData%/%ProgramData%
    /// locations instead of staging it under the target root.
    #[arg(long = "restore-user-data")]
    pub restore_user_data: bool,

    /// Skip restoring captured license artifacts and license registry values.
    #[arg(long = "no-restore-license")]
    pub no_restore_license: bool,

    /// Also write the log to this file, which must not exist yet.
    #[arg(long = "log", value_name = "FILE")]
    pub log: Option<PathBuf>,
}

pub fn run() -> ExitCode {
    match run_inner(std::env::args_os()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            tracing::error!(%error, "nothing was installed");
            if let Some(log) = logging::current_log_file() {
                eprintln!("log: {}", log.display());
            }
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_inner<I, T>(args: I) -> Result<u8>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let raw_args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let executable = std::env::current_exe()
        .map_err(|source| WinAppMigratorError::io("current executable path", source))?;

    // A bare stub (no embedded package) is only ever the template the app copies
    // when building an installer — it has nothing to install.
    if !has_embedded_self_archive(&executable)? {
        println!("WinAppMigrator installer stub — it carries no application package.");
        println!(
            "Build an installer with the WinAppMigrator app, then run the produced .exe on the \
             target machine."
        );
        return Ok(EXIT_SUCCESS);
    }

    run_installer(raw_args, &executable)
}

fn run_installer(raw_args: Vec<OsString>, executable: &Path) -> Result<u8> {
    let cli = InstallerCli::parse_from(raw_args);
    if cli.target_root.is_none() && !cli.apply && !cli.dry_run {
        println!("WinAppMigrator self-extracting installer.");
        println!("Run it with no arguments to open the graphical installer, or:");
        println!("  preview the reinstall: --dry-run [--target-root <DIR>]");
        println!("  reinstall:             --apply [--target-root <DIR>]");
        return Ok(EXIT_SUCCESS);
    }
    if let Some(path) = cli.log.as_deref() {
        logging::also_log_to(path).map_err(|error| WinAppMigratorError::io(path, error))?;
    }
    // A preview changes nothing, which includes not starting a log.
    if cli.apply {
        logging::open_machine_log();
    }

    // Checked before anything is parsed, and held open so it cannot be swapped afterward.
    let Some(package) = EmbeddedPackage::open(executable)? else {
        return Err(WinAppMigratorError::InvalidInput(
            "installer does not contain an embedded package".to_owned(),
        ));
    };
    let manifest = read_embedded_manifest(&package)?;
    let target_root = choose_target_root(cli.target_root.clone(), &manifest)?;
    let state_root = machine_state_root().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(
            "this PC's ProgramData folder could not be found, so an install could not be undone"
                .to_owned(),
        )
    })?;

    // Remove extraction directories from previous or crashed runs first.
    crate::cleanup::sweep_stale_extractions(executable);
    // Stage the extracted package on the install destination's drive (falling
    // back to the installer's own drive, then the system temp drive) so a large
    // package never fails to extract for lack of space on the system drive.
    let temp_dir = crate::cleanup::create_staging_dir(Some(&target_root), executable)
        .map_err(|source| WinAppMigratorError::io("staging extraction directory", source))?;
    package.extract_to(temp_dir.path())?;

    let applied = cli.apply;
    let restore_license = !cli.no_restore_license;
    let result = restore_package_monitored(
        &RestoreOptions {
            package_root: temp_dir.path().to_path_buf(),
            target_root: target_root.clone(),
            dry_run: cli.dry_run || !cli.apply,
            apply: cli.apply,
            force: cli.force,
            restore_user_data: cli.restore_user_data,
            restore_license,
            state_root,
        },
        &InstallMonitor::default(),
        Some(&package),
    )?;
    println!(
        "Reinstall {}.",
        if applied { "applied" } else { "previewed" }
    );
    println!("Changes logged: {}", result.changes_logged.len());
    // The restore logs are in the staging folder, which is removed when this returns, so
    // this is the only place automation can see what went wrong.
    println!("Warnings: {}", result.warnings.len());
    for warning in &result.warnings {
        println!("  {}: {}", warning.code, warning.message);
    }
    println!("Errors: {}", result.errors.len());
    for error in &result.errors {
        println!("  {}: {}", error.code, error.message);
    }
    println!("Refused: {}", result.refused.len());
    for refused in &result.refused {
        println!("  {}", refused.message);
    }

    // An applied restore already verified the package and the reinstalled files. A
    // preview changed nothing, so only the package itself has anything to check.
    let verification = match result.verification.as_ref() {
        Some(report) => report.clone(),
        None => verify_package(&VerificationOptions {
            package_root: temp_dir.path().to_path_buf(),
            target_root: None,
            output_dir: None,
            restore_user_data: cli.restore_user_data,
            restore_license,
        })?,
    };
    println!("Package hashes valid: {}", verification.package_valid);
    let restore_ok = match verification.restore_valid {
        Some(valid) => {
            println!("Reinstall hashes valid: {valid}");
            valid
        }
        None => true,
    };
    if let Some(id) = result.install_id.as_deref() {
        println!("Install id: {id}");
    }
    if let Some(report) = result.rollback.as_ref() {
        println!("The install failed, so what it changed was put back.");
        for line in report_lines(report) {
            println!("{line}");
        }
    }
    if let Some(log) = logging::current_log_file() {
        println!("Log: {}", log.display());
    }

    if verification.package_valid && restore_ok && result.errors.is_empty() {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

/// The install location asked for, or else where the package's app was, if the same checks
/// the window makes allow it. A package names that itself, so it is not to be taken on trust.
fn choose_target_root(asked_for: Option<PathBuf>, manifest: &PackageManifest) -> Result<PathBuf> {
    if let Some(asked_for) = asked_for {
        return Ok(asked_for);
    }
    let Some(recorded) = default_install_location(manifest) else {
        return Err(WinAppMigratorError::InvalidInput(
            "this package does not say where its app was, so give a --target-root".to_owned(),
        ));
    };
    match install_location_problem(&recorded) {
        Some(problem) => Err(WinAppMigratorError::InvalidInput(format!(
            "this package's app was in {}, and that cannot be used: {problem}. Give a \
             --target-root",
            recorded.display()
        ))),
        None => Ok(recorded),
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use winappmigrator_core::path_utils::system_root;

    use super::*;

    fn manifest_with_app_at(location: &Path) -> Result<PackageManifest> {
        let mut manifest: PackageManifest = serde_json::from_str(
            r#"{
                "schema_version": "",
                "source_computer_name": null,
                "source_windows_version": null,
                "scan_timestamp_utc": "",
                "main_executable_path": "",
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
        manifest.main_executable_path = location.join("app.exe");
        Ok(manifest)
    }

    #[test]
    fn with_no_target_it_goes_where_the_package_says_if_the_windows_checks_allow_it() -> Result<()>
    {
        let good = Path::new(r"C:\Program Files\Acme");
        assert_eq!(
            choose_target_root(None, &manifest_with_app_at(good)?)?,
            good.to_path_buf()
        );
        let asked_for = PathBuf::from(r"D:\Elsewhere");
        assert_eq!(
            choose_target_root(Some(asked_for.clone()), &manifest_with_app_at(good)?)?,
            asked_for
        );
        for bad in [system_root(), Path::new(r"C:\"), Path::new("relative")] {
            let refused = choose_target_root(None, &manifest_with_app_at(bad)?);
            assert!(
                matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                    if message.contains("--target-root")),
                "{bad:?}: {refused:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn a_log_can_be_asked_for() -> Result<()> {
        let cli = InstallerCli::try_parse_from([
            "winappmigrator-installer",
            "--apply",
            "--log",
            r"C:\Logs\install.log",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert_eq!(cli.log, Some(PathBuf::from(r"C:\Logs\install.log")));
        assert_eq!(cli.target_root, None);
        Ok(())
    }

    #[test]
    fn installer_cli_is_valid() {
        InstallerCli::command().debug_assert();
    }

    #[test]
    fn license_restore_is_on_by_default() -> Result<()> {
        let cli = InstallerCli::try_parse_from(["winappmigrator-installer"])
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert!(!cli.no_restore_license, "license restore defaults to on");
        Ok(())
    }

    #[test]
    fn parses_apply_target_and_flags() -> Result<()> {
        let cli = InstallerCli::try_parse_from([
            "winappmigrator-installer",
            "--target-root",
            r"C:\Restored",
            "--apply",
            "--restore-user-data",
            "--no-restore-license",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        assert!(cli.apply);
        assert!(cli.restore_user_data);
        assert!(cli.no_restore_license);
        assert_eq!(cli.target_root, Some(PathBuf::from(r"C:\Restored")));
        Ok(())
    }
}
