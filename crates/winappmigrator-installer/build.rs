//! Embed a Windows application manifest that requests administrator elevation.
//!
//! The installer writes to `Program Files`, `HKLM`, services, and shortcuts, all
//! of which require elevation, so it **always** requests `requireAdministrator`
//! (every build — release and debug). Windows prompts once via UAC on launch and
//! the whole installer runs elevated, so every write succeeds — the standard
//! behavior users expect from an installer.
//!
//! Because it is always elevated, a non-elevated process cannot spawn the
//! installer via `CreateProcess` (Windows returns `ERROR_ELEVATION_REQUIRED`);
//! the end-to-end test runs the reinstall only from an elevated shell and skips
//! otherwise.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    use embed_manifest::manifest::ExecutionLevel;
    use embed_manifest::{embed_manifest, new_manifest};

    let manifest = new_manifest("WinAppMigrator.Installer")
        .requested_execution_level(ExecutionLevel::RequireAdministrator);
    if let Err(error) = embed_manifest(manifest) {
        // Don't fail the build over manifest embedding; the installer still works
        // (the user can right-click "Run as administrator"), just without the
        // automatic UAC prompt.
        println!("cargo:warning=could not embed installer manifest: {error}");
    }
}
