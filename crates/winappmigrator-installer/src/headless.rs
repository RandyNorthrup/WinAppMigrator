//! Headless (command-line) restore, used by automation, CI, and the e2e test.
//!
//! Behaviorally identical to the previous console stub: extract the embedded
//! package, restore it to `--target-root`, and hash-verify. License restore is
//! on by default (opt out with `--no-restore-license`); user-data restore is
//! opt-in (`--restore-user-data`).

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use winappmigrator_core::{
    EmbeddedPackage, RestoreOptions, Result, VerificationOptions, WinAppMigratorError,
    has_embedded_self_archive, restore_package, verify_package,
};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;

/// Arguments accepted by the installer when driven headlessly.
#[derive(Debug, Parser)]
#[command(name = "winappmigrator-installer")]
#[command(about = "Reinstall an embedded WinAppMigrator application package and hash-verify it.")]
#[command(version)]
pub struct InstallerCli {
    /// Where the application is reinstalled.
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
}

pub fn run() -> ExitCode {
    match run_inner(std::env::args_os()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
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
    let Some(target_root) = cli.target_root else {
        if cli.apply {
            return Err(WinAppMigratorError::InvalidInput(
                "--apply needs --target-root <DIR>; nothing was installed".to_owned(),
            ));
        }
        println!("WinAppMigrator self-extracting installer.");
        println!("Run it with no arguments to open the graphical installer, or:");
        println!("  preview the reinstall: --target-root <DIR>");
        println!("  reinstall:             --target-root <DIR> --apply");
        return Ok(EXIT_SUCCESS);
    };

    // Checked before anything is parsed, and held open so it cannot be swapped afterward.
    let Some(package) = EmbeddedPackage::open(executable)? else {
        return Err(WinAppMigratorError::InvalidInput(
            "installer does not contain an embedded package".to_owned(),
        ));
    };

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
    let result = restore_package(&RestoreOptions {
        package_root: temp_dir.path().to_path_buf(),
        target_root: target_root.clone(),
        dry_run: cli.dry_run || !cli.apply,
        apply: cli.apply,
        force: cli.force,
        restore_user_data: cli.restore_user_data,
        restore_license,
    })?;
    println!(
        "Reinstall {}.",
        if applied { "applied" } else { "previewed" }
    );
    println!("Changes logged: {}", result.changes_logged.len());
    println!("Warnings: {}", result.warnings.len());
    println!("Errors: {}", result.errors.len());

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

    if verification.package_valid && restore_ok && result.errors.is_empty() {
        Ok(EXIT_SUCCESS)
    } else {
        Ok(EXIT_FAILURE)
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

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
