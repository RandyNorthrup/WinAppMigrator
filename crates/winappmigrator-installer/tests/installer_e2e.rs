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

use winappmigrator_core::{
    AppSelection, EmbeddedPackage, InstallerStub, PackageOptions, create_package,
    read_embedded_manifest,
};

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
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub.clone())),
        installer_dir: None,
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: winappmigrator_core::ScanSource::Live,
        user_profile: winappmigrator_core::ProfileChoice::Default,
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

    // The real stub has nothing appended to it, so it reads as a bare stub and not as a
    // damaged installer, and what was built passes the footer's hash check.
    assert!(EmbeddedPackage::open(&stub)?.is_none());
    let package = EmbeddedPackage::open(&installer)?.ok_or("the installer has no package")?;
    let manifest = read_embedded_manifest(&package)?;
    assert!(!manifest.file_inventory.is_empty());
    for file in manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
    {
        assert!(
            file.hash_sha256.is_some(),
            "{} has no hash",
            file.original_path.display()
        );
    }
    drop(package);

    // 2. Reinstall on a clean target, headlessly. The installer reinstalls,
    //    hash-verifies both its package and the reinstalled files, and exits
    //    non-zero on any mismatch — so a successful exit asserts the pipeline.
    //    An install records itself in ProgramData and adds an entry to Settings, which
    //    only GitHub's throwaway runner may have done to it. Elsewhere, even when
    //    elevated, this stops at a preview, which changes nothing.
    let target = work.path().join("target");
    let may_install = std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true")
        && std::env::var("RUNNER_ENVIRONMENT").as_deref() == Ok("github-hosted");
    let apply = match Command::new(&installer)
        .arg("--target-root")
        .arg(&target)
        .args(if may_install {
            ["--apply", "--force"].as_slice()
        } else {
            ["--dry-run"].as_slice()
        })
        .output()
    {
        Ok(output) => output,
        // ERROR_ELEVATION_REQUIRED (740): the installer embeds a
        // `requireAdministrator` manifest, so a non-elevated process cannot spawn
        // it. Skip the reinstall assertion on a developer's normal shell. On CI this
        // falls through to a failure, so the test cannot pass without having run.
        Err(error) if error.raw_os_error() == Some(740) && std::env::var_os("CI").is_none() => {
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

    if !may_install {
        assert!(!target.exists(), "a preview changes nothing");
        return Ok(());
    }
    assert!(target.is_dir(), "target directory was not created");
    assert!(
        count_files(&target) >= 1,
        "reinstall produced no files on the target"
    );

    Ok(())
}
