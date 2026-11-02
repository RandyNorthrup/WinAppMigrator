//! Undoing an install, as the app's entry in Settings asks for it.
//!
//! An install leaves a bare copy of this stub in its admin-only folder, and points the entry at
//! it. Given `--uninstall <id>`, the stub reads only from the fixed place installs are recorded,
//! opens a small window, and with `--quiet` does the same with no window, printing what it did.
//! The checking and the undoing are in `winappmigrator_core::rollback`.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use winappmigrator_core::journal::{UNINSTALL_FLAG, machine_state_root};
use winappmigrator_core::logging;
use winappmigrator_core::rollback::{
    UninstallOutcome, open_install, report_lines, schedule_removal_of_running_uninstaller,
};
use winappmigrator_core::{InstallMonitor, Result, WinAppMigratorError};

/// Arguments accepted by the stub when uninstalling.
#[derive(Debug, Parser)]
#[command(name = "winappmigrator-installer")]
#[command(about = "Undo an install made by a WinAppMigrator installer.")]
pub struct UninstallCli {
    /// Which install to undo. The id is in the app's entry in Settings.
    #[arg(long = "uninstall", value_name = "ID")]
    pub id: String,

    /// No window. Print what was done, and exit non-zero if anything could not be undone.
    #[arg(long)]
    pub quiet: bool,

    /// Also remove the app's user data and license data, which otherwise stay.
    #[arg(long = "remove-user-data")]
    pub remove_user_data: bool,

    /// Also write the log to this file, which must not exist yet.
    #[arg(long = "log", value_name = "FILE")]
    pub log: Option<PathBuf>,
}

/// True when the command line asks for an uninstall.
pub fn is_requested(args: &[std::ffi::OsString]) -> bool {
    args.iter()
        .skip(1)
        .any(|argument| argument == UNINSTALL_FLAG)
}

/// Where installs are recorded. Fixed, and not something a command line can change.
pub(crate) fn state_root() -> Result<PathBuf> {
    machine_state_root().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(
            "this PC's ProgramData folder could not be found, so there is no record to read"
                .to_owned(),
        )
    })
}

pub fn run(args: Vec<std::ffi::OsString>) -> ExitCode {
    logging::install("uninstall");
    let cli = UninstallCli::parse_from(args);
    if !cli.quiet {
        return match crate::uninstall_gui::run(cli) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                tracing::error!(%error, "the uninstaller's window could not open");
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        };
    }
    match run_quiet(&cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            tracing::error!(%error, "could not uninstall");
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Uninstall with no window. `Ok(false)` means something could not be undone.
fn run_quiet(cli: &UninstallCli) -> Result<bool> {
    if let Some(path) = cli.log.as_deref() {
        logging::also_log_to(path).map_err(|error| WinAppMigratorError::io(path, error))?;
    }
    let log = logging::open_machine_log();
    let opened = open_install(&state_root()?, &cli.id)?;
    let summary = opened.summary();
    println!("Uninstalling {}.", summary.app_name);
    let outcome = opened.uninstall(cli.remove_user_data, &InstallMonitor::default());
    print_outcome(&outcome);
    if let Some(log) = log {
        println!("Log: {}", log.display());
    }
    finish(&outcome);
    Ok(outcome.finished)
}

fn print_outcome(outcome: &UninstallOutcome) {
    for line in report_lines(&outcome.report) {
        println!("{line}");
    }
}

/// Have what is left go once this program exits, which is all that is keeping it there.
pub(crate) fn finish(outcome: &UninstallOutcome) {
    if let Some(folder) = outcome.remove_after_exit.as_deref()
        && let Err(error) = schedule_removal_of_running_uninstaller(folder)
    {
        // Harmless: an administrator's folder with this program in it, and the next install
        // sweeps it away.
        tracing::warn!(%error, "could not arrange to remove the uninstaller");
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use clap::CommandFactory;
    use winappmigrator_core::journal::QUIET_FLAG;

    use super::*;

    #[test]
    fn the_cli_is_valid_and_takes_what_the_entry_in_settings_gives_it() -> Result<()> {
        UninstallCli::command().debug_assert();
        let cli =
            UninstallCli::try_parse_from(["uninstall.exe", UNINSTALL_FLAG, "wam-abc", QUIET_FLAG])
                .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert_eq!(cli.id, "wam-abc");
        assert!(cli.quiet);
        assert!(!cli.remove_user_data, "user data stays unless asked for");
        Ok(())
    }

    #[test]
    fn an_uninstall_is_told_from_an_install_by_its_flag() {
        let args = |list: &[&str]| list.iter().map(OsString::from).collect::<Vec<_>>();
        assert!(is_requested(&args(&["x.exe", "--uninstall", "wam-abc"])));
        assert!(!is_requested(&args(&["x.exe", "--target-root", r"C:\App"])));
        assert!(
            !is_requested(&args(&["--uninstall"])),
            "that is the program's name"
        );
    }
}
