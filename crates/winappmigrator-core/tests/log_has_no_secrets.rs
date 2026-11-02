//! Pins what the log may hold: paths and key names, and never what a registry value, a license
//! file, or an environment variable holds.
//!
//! This is its own test program because `tracing` remembers which logging calls have nowhere
//! to go, and tests running beside this one in the same process would change the answer.
//!
//! It runs what a failed install logs, with each kind of secret on its way through. Everything
//! is real but the registry, which it never reaches, because a file fails first and rolls the
//! install back. That leaves the machine as it was, and only temp folders are written to.

use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use winappmigrator_core::hashing::sha256_file;
use winappmigrator_core::logging::LogSink;
use winappmigrator_core::{
    EnvironmentScope, EnvironmentVariableEntry, FileClassification, FileEntry, MatchConfidence,
    PACKAGE_SCHEMA_VERSION, PackageManifest, RegistryEntry, RegistryHive, RegistryValueKind,
    RestoreOptions, restore_package,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const SECRET: &str = "s3cr3t-value-that-must-not-be-logged";
const INSTALLED_FROM: &str = r"C:\Program Files\Acme";

fn hold_exclusively(path: &Path) -> std::io::Result<File> {
    OpenOptions::new().read(true).share_mode(0).open(path)
}

fn value(key: &str, name: &str, data: String) -> RegistryEntry {
    RegistryEntry {
        hive: RegistryHive::CurrentUser,
        key_path: key.to_owned(),
        value_name: Some(name.to_owned()),
        value_data: Some(data),
        value_kind: Some(RegistryValueKind::String),
        value_bytes_hex: None,
        confidence: MatchConfidence::High,
        requires_admin: false,
    }
}

fn manifest_for(files: Vec<FileEntry>) -> PackageManifest {
    PackageManifest {
        schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
        source_computer_name: None,
        source_windows_version: None,
        scan_timestamp_utc: String::new(),
        main_executable_path: Path::new(INSTALLED_FROM).join("acme.exe"),
        application_metadata: None,
        file_inventory: files,
        registry_keys_exported: vec![
            value(r"Software\Acme", "ApiKey", SECRET.to_owned()),
            // Refused, and the reason it is refused quotes what it would run.
            value(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                "Evil",
                format!(r"C:\Windows\System32\cmd.exe /c {SECRET}"),
            ),
        ],
        com_registrations_detected: Vec::new(),
        file_associations_detected: Vec::new(),
        services_detected: Vec::new(),
        drivers_detected: Vec::new(),
        scheduled_tasks_detected: Vec::new(),
        shortcuts_detected: Vec::new(),
        environment_variables_detected: vec![EnvironmentVariableEntry {
            scope: EnvironmentScope::User,
            name: "ACME_TOKEN".to_owned(),
            value: SECRET.to_owned(),
            value_kind: RegistryValueKind::String,
            requires_admin: false,
            confidence: MatchConfidence::High,
        }],
        runtime_prerequisites: Vec::new(),
        license_artifacts_detected: Vec::new(),
        license_registry_entries: vec![value(r"Software\Acme\License", "Key", SECRET.to_owned())],
        warnings: Vec::new(),
        items_intentionally_excluded: Vec::new(),
        included_user_data: false,
        included_program_data: false,
        included_license_data: true,
        ui_theme: None,
    }
}

fn package_of(root: &Path, files: &[(&str, &[u8])]) -> TestResult<PathBuf> {
    let package = root.join("package");
    let mut entries = Vec::new();
    for (relative, bytes) in files {
        let staged = package.join("files").join(relative);
        fs::create_dir_all(package.join("files"))?;
        fs::write(&staged, bytes)?;
        let original = Path::new(INSTALLED_FROM).join(relative);
        entries.push(FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(Path::new("files").join(relative)),
            suggested_destination_path: None,
            size_bytes: Some(bytes.len() as u64),
            hash_sha256: Some(sha256_file(&staged)?),
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
        });
    }
    fs::write(
        package.join("manifest.json"),
        serde_json::to_vec(&manifest_for(entries))?,
    )?;
    Ok(package)
}

#[test]
fn the_log_never_holds_what_values_hold() -> TestResult {
    let work = tempfile::tempdir()?;
    let package = package_of(
        work.path(),
        &[("license.dat", SECRET.as_bytes()), ("held.dat", b"new")],
    )?;
    let target = work.path().join("target");
    let mut options = RestoreOptions {
        package_root: package,
        target_root: target.clone(),
        dry_run: true,
        apply: false,
        force: true,
        restore_user_data: false,
        restore_license: true,
        state_root: work.path().join("state"),
    };
    let sink = LogSink::default();
    let captured = sink.capture();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(sink)
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::with_default(subscriber, || -> TestResult {
        let preview = restore_package(&options)?;
        assert!(
            preview
                .refused
                .iter()
                .any(|item| item.message.contains(SECRET)),
            "what a window shows may say it, which makes this a real test of what is logged"
        );

        options.apply = true;
        options.dry_run = false;
        fs::create_dir_all(&target)?;
        fs::write(target.join("held.dat"), SECRET)?;
        let held = hold_exclusively(&target.join("held.dat"))?;
        let failed = restore_package(&options)?;
        drop(held);
        assert!(
            failed.rollback.is_some(),
            "it has to get as far as failing: {failed:?}"
        );
        Ok(())
    })?;

    let log = captured
        .lock()
        .map_err(|error| error.to_string())?
        .to_owned();
    let log = String::from_utf8_lossy(&log);
    println!("{log}");
    for expected in [
        "would copy",
        "held.dat",
        "Evil",
        "install failed",
        "restore_file_failed",
    ] {
        assert!(
            log.contains(expected),
            "the log should say {expected}: {log}"
        );
    }
    assert!(!log.contains(SECRET), "{log}");
    Ok(())
}
