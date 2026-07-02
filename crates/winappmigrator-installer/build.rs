//! Embed a Windows application manifest that requests administrator elevation.
//!
//! The installer writes to `Program Files`, `HKLM`, services, and shortcuts, all
//! of which require elevation. Requesting `requireAdministrator` makes Windows
//! prompt once via UAC on launch, so the whole installer runs elevated and every
//! write succeeds — the standard behavior users expect from an installer.
//!
//! Only release builds get the manifest: debug builds stay `asInvoker` so the
//! GUI can be run and screenshotted during development without a UAC prompt.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let is_windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some();
    let is_release = std::env::var("PROFILE").as_deref() == Ok("release");
    if !(is_windows && is_release) {
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
