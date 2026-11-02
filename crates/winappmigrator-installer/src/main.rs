//! The WinAppMigrator self-extracting installer.
//!
//! This is the stub the WinAppMigrator app embeds into every installer it
//! builds. Run on a target machine it reinstalls the packaged application, its
//! data, and its runtime prerequisites, then hash-verifies the result.
//!
//! It has two front ends:
//! - **GUI** (double-click, no arguments): a window that shows the app, lets the
//!   user pick the install location and — when they were captured at build time
//!   — whether to restore user data and license data, then installs with live
//!   progress. It always requests administrator elevation on launch.
//! - **Headless** (any arguments): the classic `--target-root … --apply` flow,
//!   for automation, CI, and the end-to-end test.

// Always a windowed app — never flash a console window when double-clicked, in
// any build. Headless mode still writes to a redirected stdout handle (as the
// e2e test and automation use), which a windowed subsystem does not affect.
#![windows_subsystem = "windows"]

use std::process::ExitCode;

use winappmigrator_installer::{gui, headless};

fn main() -> ExitCode {
    // First, before anything else can load a DLL.
    winappmigrator_ui::hardening::restrict_dll_search();
    // Arguments beyond the program name mean a scripted/headless restore; a bare
    // double-click (argv == [exe]) opens the GUI. Deciding on argument count also
    // keeps the GUI path from ever touching CLI parsing.
    if std::env::args_os().count() > 1 {
        return headless::run();
    }
    match gui::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
