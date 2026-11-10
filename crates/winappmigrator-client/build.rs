//! The client takes a snapshot of Windows' drive and reads what only administrators can, so it asks
//! for elevation every time it starts.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    // Resolve the DLLs it imports from System32 only, as it is run from a USB stick or a share,
    // where a planted DLL would otherwise load elevated. Windows 7 ignores this, and has nothing
    // newer to load in its place.
    println!("cargo:rustc-link-arg-bins=/DEPENDENTLOADFLAG:0x800");

    use embed_manifest::manifest::ExecutionLevel;
    use embed_manifest::{embed_manifest, new_manifest};

    let manifest = new_manifest("WinAppMigrator.Client")
        .requested_execution_level(ExecutionLevel::RequireAdministrator);
    if let Err(error) = embed_manifest(manifest) {
        println!("cargo:warning=could not embed the client's manifest: {error}");
    }
}
