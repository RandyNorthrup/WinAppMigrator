//! End-to-end test of the whole product pipeline against the real binary.
//!
//! It reverse-engineers a genuine PE executable, builds a self-extracting
//! installer using the actual `winappmigrator-installer` stub binary, runs that
//! installer headlessly against a clean target directory, and asserts the
//! reinstall plus hash verification pass. The installer exits non-zero on any
//! hash mismatch, so a successful exit is a strong end-to-end assertion.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use winappmigrator_core::{AppSelection, PackageArchiveFormat, PackageOptions, create_package};

type TestResult = Result<(), Box<dyn Error>>;

/// Path to the built installer stub binary (the installer template that both
/// carries the package and, given `--target-root … --apply`, reinstalls it).
fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

fn count_files(root: &Path) -> usize {
    let Ok(entries) = fs::read_dir(root) else {
        return 0;
    };
    let mut total = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            total += count_files(&path);
        } else {
            total += 1;
        }
    }
    total
}

#[test]
fn build_installer_then_reinstall_and_verify_on_clean_target() -> TestResult {
    let stub = stub_binary();
    assert!(stub.is_file(), "stub binary missing: {}", stub.display());

    let work = tempfile::tempdir()?;

    // A real installed application stands in as a genuine PE with imports: copy
    // the stub binary itself into an isolated "install directory".
    let source_dir = work.path().join("source");
    fs::create_dir_all(&source_dir)?;
    let app_exe = source_dir.join("DemoApp.exe");
    fs::copy(&stub, &app_exe)?;

    // 1. Reverse-engineer + build the self-extracting installer (what the app
    //    does on the source machine).
    let out_dir = work.path().join("DemoApp");
    let options = PackageOptions {
        selection: AppSelection::ExecutablePath(app_exe.clone()),
        output_dir: out_dir.clone(),
        current_directory: Some(source_dir.clone()),
        archive_format: Some(PackageArchiveFormat::Zip),
        create_self_extracting_archive: true,
        self_extracting_stub: Some(stub.clone()),
        installer_dir: None,
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: winappmigrator_core::ScanSource::Live,
    };
    let result = create_package(&options)?;
    assert!(
        result.files_copied >= 1,
        "expected the app exe to be captured"
    );
    let installer = result
        .self_extracting_archive_path
        .ok_or("no self-extracting installer was produced")?;
    assert!(installer.is_file(), "installer not written");

    // 2. Reinstall on a clean target, headlessly. The installer reinstalls,
    //    hash-verifies both its package and the reinstalled files, and exits
    //    non-zero on any mismatch — so a successful exit asserts the pipeline.
    let target = work.path().join("target");
    let apply = match Command::new(&installer)
        .arg("--target-root")
        .arg(&target)
        .arg("--apply")
        .arg("--force")
        .output()
    {
        Ok(output) => output,
        // ERROR_ELEVATION_REQUIRED (740): the installer embeds a
        // `requireAdministrator` manifest, so a non-elevated process cannot spawn
        // it. Skip the reinstall assertion on such hosts; CI runs elevated and
        // exercises the full reinstall + verify.
        Err(error) if error.raw_os_error() == Some(740) => {
            eprintln!(
                "skipping installer reinstall: spawning the elevated installer needs an elevated shell ({error})"
            );
            return Ok(());
        }
        Err(error) => return Err(Box::new(error)),
    };
    assert!(
        apply.status.success(),
        "installer reinstall/verify failed: {}\n{}",
        apply.status,
        String::from_utf8_lossy(&apply.stderr)
    );

    assert!(target.is_dir(), "target directory was not created");
    assert!(
        count_files(&target) >= 1,
        "reinstall produced no files on the target"
    );

    Ok(())
}
