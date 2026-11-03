//! Build a real self-extracting installer from an executable, mirroring the
//! app's flow (stage on the destination drive, name the installer after the
//! app, then remove the staging directory). Used to verify the produced
//! installer's GUI and that no staging/temp files are left behind.
//!
//! Usage: `wam_pkg <source_exe> <stub_exe> <out_dir> <app_name>`

use std::path::PathBuf;

use winappmigrator_core::{
    AppSelection, InstallerStub, PackageOptions, ScanSource, create_package,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(source), Some(stub), Some(out_dir), Some(app_name)) =
        (args.next(), args.next(), args.next(), args.next())
    else {
        eprintln!("usage: build_installer <source_exe> <stub_exe> <out_dir> <app_name>");
        return;
    };

    let installer_dir = PathBuf::from(&out_dir);
    let base_name: String = app_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_') {
                c
            } else {
                '_'
            }
        })
        .collect();
    let base_name = base_name.trim().to_owned();

    // Stage on the same drive as the destination, like the app.
    let staging_parent = installer_dir
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(&installer_dir)
        .join(format!(".winappmigrator-build-{}", std::process::id()));
    let staging = staging_parent.join(&base_name);

    let options = PackageOptions {
        selection: AppSelection::ExecutablePath(PathBuf::from(&source)),
        output_dir: staging.clone(),
        current_directory: std::env::current_dir().ok(),
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(PathBuf::from(&stub))),
        installer_dir: Some(installer_dir.clone()),
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: ScanSource::Live,
        user_profile: winappmigrator_core::ProfileChoice::Default,
    };

    let result = create_package(&options);
    let _ = std::fs::remove_dir_all(&staging_parent);

    match result {
        Ok(package) => {
            let sfx = package
                .self_extracting_archive_path
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            println!("INSTALLER: {sfx}");
            println!("STAGING REMOVED: {}", !staging_parent.exists());
        }
        Err(error) => println!("BUILD ERROR: {error}"),
    }
}
