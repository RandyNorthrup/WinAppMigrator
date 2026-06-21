//! The self-extracting installer stub produced by WinAppMigrator.
//!
//! This binary is not a builder. Building installers is the job of the
//! WinAppMigrator app on the source machine, which embeds a copy of this stub
//! plus the application package into one self-extracting `.exe`. When that
//! installer is run on the **target** machine it reinstalls the application and
//! its bundled data, then hash-verifies the result. Restore therefore lives
//! here, in the installer — never in the app.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::Parser;
use winappmigrator_core::{
    RestoreOptions, Result, VerificationOptions, WinAppMigratorError,
    extract_embedded_self_archive, has_embedded_self_archive, restore_package, verify_package,
};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;

/// Arguments accepted by the installer when it carries an embedded package.
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
}

pub fn run<I, T>(args: I) -> Result<u8>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let raw_args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let executable = std::env::current_exe()
        .map_err(|source| WinAppMigratorError::io("current executable path", source))?;

    // A bare stub (no embedded package) is only ever the template the app
    // copies when building an installer — it has nothing to install.
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
        println!("WinAppMigrator self-extracting installer.");
        println!("Preview the reinstall: --target-root <DIR>");
        println!("Reinstall:             --target-root <DIR> --apply");
        return Ok(EXIT_SUCCESS);
    };

    let temp_dir = tempfile::tempdir()
        .map_err(|source| WinAppMigratorError::io("temporary extraction directory", source))?;
    let extracted = extract_embedded_self_archive(executable, temp_dir.path())?;
    if !extracted {
        return Err(WinAppMigratorError::InvalidInput(
            "installer does not contain an embedded package".to_owned(),
        ));
    }

    let applied = cli.apply;
    let result = restore_package(&RestoreOptions {
        package_root: temp_dir.path().to_path_buf(),
        target_root: target_root.clone(),
        dry_run: cli.dry_run || !cli.apply,
        apply: cli.apply,
        force: cli.force,
        restore_user_data: cli.restore_user_data,
    })?;
    println!(
        "Reinstall {}.",
        if applied { "applied" } else { "previewed" }
    );
    println!("Changes logged: {}", result.changes_logged.len());
    println!("Warnings: {}", result.warnings.len());
    println!("Errors: {}", result.errors.len());

    // Hash-verify the bundled package, and — when applied — the reinstalled
    // files on the target.
    let verification = verify_package(&VerificationOptions {
        package_root: temp_dir.path().to_path_buf(),
        target_root: applied.then_some(target_root),
        output_dir: None,
        restore_user_data: cli.restore_user_data,
    })?;
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
    fn parses_apply_target_and_user_data() -> Result<()> {
        let cli = InstallerCli::try_parse_from([
            "winappmigrator-installer",
            "--target-root",
            r"C:\Restored",
            "--apply",
            "--restore-user-data",
        ])
        .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        assert!(cli.apply);
        assert!(cli.restore_user_data);
        assert_eq!(cli.target_root, Some(PathBuf::from(r"C:\Restored")));
        Ok(())
    }

    #[test]
    fn defaults_to_preview() -> Result<()> {
        let cli = InstallerCli::try_parse_from(["winappmigrator-installer"])
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;

        assert!(!cli.apply);
        assert!(cli.target_root.is_none());
        Ok(())
    }
}
