//! Embed a Windows application manifest that requests administrator elevation.
//!
//! The installer writes to `Program Files`, `HKLM`, services, and shortcuts, all
//! of which require elevation. Requesting `requireAdministrator` makes Windows
//! prompt once via UAC on launch, so the whole installer runs elevated and every
//! write succeeds — the standard behavior users expect from an installer.
//!
//! Release builds request `requireAdministrator`; debug builds request an
//! explicit `asInvoker`. Both always embed a manifest: a debug installer must
//! still declare a `requestedExecutionLevel`, otherwise Windows' installer-
//! detection heuristic (any exe named `*installer*`/`*setup*`) auto-elevates it
//! through a broker as a separate process — which broke run/screenshot/close
//! during development. An explicit `asInvoker` manifest suppresses that.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let is_windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some();
    if !is_windows {
        return;
    }
    let is_release = std::env::var("PROFILE").as_deref() == Ok("release");

    use embed_manifest::manifest::ExecutionLevel;
    use embed_manifest::{embed_manifest, new_manifest};

    let level = if is_release {
        ExecutionLevel::RequireAdministrator
    } else {
        ExecutionLevel::AsInvoker
    };
    let manifest = new_manifest("WinAppMigrator.Installer").requested_execution_level(level);
    if let Err(error) = embed_manifest(manifest) {
        // Don't fail the build over manifest embedding; the installer still works
        // (the user can right-click "Run as administrator"), just without the
        // automatic UAC prompt.
        println!("cargo:warning=could not embed installer manifest: {error}");
    }
}
