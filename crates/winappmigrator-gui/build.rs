//! Build script for the WinAppMigrator app. It does two things:
//!
//! 1. **Embeds an application manifest** so the app always runs **elevated**.
//!    Migrating an app — and especially scanning a *connected offline drive*
//!    (mounting its registry hives with `reg load`, reading ACL-protected files
//!    through backup semantics) — needs administrator rights. Requesting
//!    `requireAdministrator` in every build means Windows prompts once via UAC at
//!    launch and the whole session runs elevated, instead of prompting or gating
//!    for elevation part-way through a task.
//!
//! 2. **Embeds the installer stub.** WinAppMigrator is a single portable
//!    executable: it carries the compiled installer stub inside itself and writes
//!    it out (plus the chosen app's package) to produce each self-extracting
//!    installer. This stages the stub binary so `main.rs` can `include_bytes!` it.
//!
//!    Resolution order for the stub to embed:
//!      1. `WINAPPMIGRATOR_INSTALLER_STUB` — an explicit path (the release build
//!         sets this to the freshly built stub).
//!      2. the sibling `winappmigrator-installer(.exe)` already in the profile's
//!         target directory.
//!
//!    If neither exists yet (a first-pass `cargo build`), nothing is embedded and
//!    the app falls back to a sibling stub at runtime, so builds always succeed.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    embed_app_manifest();
    stage_installer_stub();
}

/// Embed the `requireAdministrator` elevation manifest (Windows only) in every
/// build, so the app always runs elevated.
fn embed_app_manifest() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    use embed_manifest::manifest::ExecutionLevel;
    use embed_manifest::{embed_manifest, new_manifest};

    let manifest = new_manifest("WinAppMigrator")
        .requested_execution_level(ExecutionLevel::RequireAdministrator);
    if let Err(error) = embed_manifest(manifest) {
        // Don't fail the build over manifest embedding; the app still runs (the
        // user can right-click "Run as administrator"), just without the
        // automatic UAC prompt at launch.
        println!("cargo:warning=could not embed app manifest: {error}");
    }
}

/// Stage the installer stub binary for `include_bytes!`.
fn stage_installer_stub() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default());
    let staged = out_dir.join("installer_stub.bin");

    let explicit = std::env::var_os("WINAPPMIGRATOR_INSTALLER_STUB").map(PathBuf::from);
    let sibling = out_dir
        .ancestors()
        .nth(3) // OUT_DIR = target/<profile>/build/<pkg>/out -> up 3 = target/<profile>
        .map(|profile_dir| profile_dir.join(stub_file_name()));
    let source = [explicit, sibling]
        .into_iter()
        .flatten()
        .find(|path| path.is_file());

    match source.as_ref() {
        Some(path) => {
            if let Err(error) = std::fs::copy(path, &staged) {
                println!(
                    "cargo:warning=could not stage installer stub ({error}); the app will fall back to a sibling stub"
                );
                let _ = std::fs::write(&staged, []);
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
        None => {
            let _ = std::fs::write(&staged, []);
        }
    }

    println!("cargo:rustc-env=WAM_INSTALLER_STUB={}", staged.display());
    println!("cargo:rerun-if-env-changed=WINAPPMIGRATOR_INSTALLER_STUB");
}

fn stub_file_name() -> &'static str {
    if cfg!(windows) {
        "winappmigrator-installer.exe"
    } else {
        "winappmigrator-installer"
    }
}
