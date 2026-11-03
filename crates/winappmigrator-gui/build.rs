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
//!    installer. This stages the stub binary so `lib.rs` can `include_bytes!` it.
//!
//!    Resolution order for the stub to embed:
//!      1. `WINAPPMIGRATOR_INSTALLER_STUB` — an explicit path (the release build
//!         sets this to the freshly built stub).
//!      2. the sibling `winappmigrator-installer(.exe)` already in the profile's
//!         target directory.
//!
//!    A debug build with neither (a first-pass `cargo build`) embeds nothing, and takes a
//!    sibling stub at runtime. A release build with neither fails: an app without the stub
//!    builds no installers, and a program beside it in Downloads should never be taken for one.

use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    embed_app_manifest();
    embed_file_details();
    restrict_import_dll_search();
    stage_installer_stub();
}

/// Resolve the DLLs this executable imports from System32 only, so one dropped beside
/// it cannot load before `main` gets to run. `0x800` is
/// `LOAD_LIBRARY_SEARCH_SYSTEM32`.
fn restrict_import_dll_search() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        println!("cargo:rustc-link-arg-bins=/DEPENDENTLOADFLAG:0x800");
    }
}

/// Embed the icon and the details Explorer shows, and the publisher's own name, so the
/// UAC prompt and SmartScreen describe a real product instead of a bare file name.
fn embed_file_details() {
    const ICON: &str = "../winappmigrator-ui/assets/winappmigrator.ico";
    println!("cargo:rerun-if-changed={ICON}");
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    let mut resource = winresource::WindowsResource::new();
    resource
        .set_icon(ICON)
        .set("FileDescription", "WinAppMigrator")
        .set("ProductName", "WinAppMigrator")
        .set("CompanyName", "Randy Northrup")
        .set("LegalCopyright", "Copyright (c) 2026 Randy Northrup");
    if let Err(error) = resource.compile_for("winappmigrator") {
        println!("cargo:warning=could not embed the icon and file details: {error}");
    }
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

    println!("cargo:rerun-if-env-changed=WINAPPMIGRATOR_INSTALLER_STUB");
    if let Some(path) = source.as_ref() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let problem = match source.as_ref() {
        Some(path) => stage(path, &staged).err(),
        None => Some(format!(
            "there is no {}. Build it first: cargo build --release -p winappmigrator-installer",
            stub_file_name()
        )),
    };
    if let Some(problem) = problem {
        if std::env::var("PROFILE").is_ok_and(|profile| profile == "release") {
            println!("cargo:error=a release of the app has to carry its installer stub: {problem}");
            std::process::exit(1);
        }
        println!("cargo:warning=no installer stub is embedded in this build: {problem}");
        let _ = std::fs::write(&staged, []);
    }

    println!("cargo:rustc-env=WAM_INSTALLER_STUB={}", staged.display());
}

fn stage(stub: &Path, staged: &Path) -> Result<(), String> {
    let bytes = std::fs::copy(stub, staged).map_err(|error| error.to_string())?;
    if bytes == 0 {
        return Err(format!("{} is empty", stub.display()));
    }
    Ok(())
}

fn stub_file_name() -> &'static str {
    if cfg!(windows) {
        "winappmigrator-installer.exe"
    } else {
        "winappmigrator-installer"
    }
}
