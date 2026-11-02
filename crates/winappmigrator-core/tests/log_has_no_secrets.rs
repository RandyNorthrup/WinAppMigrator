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
    CertificateEntry, EnvironmentScope, EnvironmentVariableEntry, FileClassification, FileEntry,
    FirewallRuleEntry, LeftOutItem, MatchConfidence, PACKAGE_SCHEMA_VERSION, PackageManifest,
    PackageOptions, RegistryEntry, RegistryHive, RegistryValueKind, RestoreOptions, create_package,
    restore_package,
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
        source_platform: None,
        program_needs: Vec::new(),
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
            // The same, in what a package registers: a protocol, a shell extension, and a font.
            value(
                r"Software\Classes\acme\shell\open\command",
                "(Default)",
                format!(r"C:\Windows\System32\cmd.exe /c {SECRET}"),
            ),
            value(r"Software\Classes\acme", "URL Protocol", String::new()),
            value(
                r"Software\Classes\*\shellex\ContextMenuHandlers\Evil",
                "(Default)",
                SECRET.to_owned(),
            ),
            value(
                r"Software\Microsoft\Windows NT\CurrentVersion\Fonts",
                "Evil (TrueType)",
                SECRET.to_owned(),
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
        secrets_left_out: vec![LeftOutItem {
            name: r"HKLM\Software\Acme\ApiKey".to_owned(),
            why: "it looks like a credential".to_owned(),
        }],
        licenses_left_out: Vec::new(),
        firewall_rules_detected: vec![FirewallRuleEntry {
            name: "Evil".to_owned(),
            description: Some(SECRET.to_owned()),
            direction: "In".to_owned(),
            action: "AllowBypass".to_owned(),
            enabled: true,
            program: PathBuf::from(format!(r"C:\Windows\{SECRET}.exe")),
            service: None,
            protocol: None,
            local_ports: vec![SECRET.to_owned()],
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: Vec::new(),
            profiles: Vec::new(),
            not_carried: Vec::new(),
        }],
        certificates_reported: vec![CertificateEntry {
            store: "ROOT".to_owned(),
            hive: RegistryHive::LocalMachine,
            thumbprint: "ABC".to_owned(),
            subject: "CN=Named Only".to_owned(),
            why: "it is named for it".to_owned(),
        }],
        fonts_reported: Vec::new(),
        warnings: Vec::new(),
        items_intentionally_excluded: Vec::new(),
        included_user_data: false,
        included_program_data: false,
        included_license_data: true,
        ui_theme: None,
        user_data_profile: None,
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
            user_folder: None,
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

/// A real app's folder in a temp folder, through the real scan and packaging, with a program it can
/// read and what must not reach a package beside it. Reads the registry and writes nothing to it.
/// What a registry value holds is only tried for real on the throwaway runner, since setting one
/// would change this machine, and the rules for those are tested by themselves.
///
/// What it looks for is made up as it runs, since the program it copies holds its own constants.
fn app_with_secrets(root: &Path, secret: &str, license: &str) -> TestResult<PathBuf> {
    let app = root.join("Secret Holder");
    fs::create_dir_all(app.join("conf"))?;
    let program = app.join("holder.exe");
    fs::copy(std::env::current_exe()?, &program)?;
    fs::write(app.join(".env"), format!("API_KEY={secret}"))?;
    fs::write(
        app.join(r"conf\server.pem"),
        format!("-----BEGIN RSA PRIVATE KEY-----\n{secret}\n-----END RSA PRIVATE KEY-----\n"),
    )?;
    fs::write(app.join("license.lic"), format!("LICENSE {license}"))?;
    fs::write(app.join(r"conf\settings.json"), b"{}")?;
    fs::write(
        app.join(r"conf\cacert.pem"),
        b"-----BEGIN CERTIFICATE-----\nMIIB\n-----END CERTIFICATE-----\n",
    )?;
    Ok(program)
}

fn files_holding(folder: &Path, needle: &str) -> TestResult<Vec<PathBuf>> {
    let wide: Vec<u8> = needle.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let mut holding = Vec::new();
    let mut folders = vec![folder.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(&folder)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                folders.push(entry.path());
                continue;
            }
            let bytes = fs::read(entry.path())?;
            let has = |what: &[u8]| bytes.windows(what.len()).any(|window| window == what);
            if has(needle.as_bytes()) || has(&wide) {
                holding.push(entry.path());
            }
        }
    }
    Ok(holding)
}

#[test]
fn credentials_reach_no_package_and_licenses_only_go_by_the_option() -> TestResult {
    let work = tempfile::tempdir()?;
    let secret = format!("canary-{}-secret", std::process::id());
    let license = format!("canary-{}-license", std::process::id());
    let program = app_with_secrets(work.path(), &secret, &license)?;
    let package_of = |name: &str, license: bool| {
        create_package(&PackageOptions {
            include_user_data: false,
            include_program_data: false,
            include_license_data: license,
            ..PackageOptions::for_executable(&program, work.path().join(name))
        })
    };

    let off = package_of("off", false)?;
    assert_eq!(
        files_holding(&off.package_root, &secret)?,
        Vec::<PathBuf>::new()
    );
    assert_eq!(
        files_holding(&off.package_root, &license)?,
        Vec::<PathBuf>::new()
    );
    let manifest: PackageManifest = serde_json::from_slice(&fs::read(&off.manifest_path)?)?;
    let names: Vec<&str> = manifest
        .secrets_left_out
        .iter()
        .map(|item| item.name.as_str())
        .collect();
    for wanted in [".env", "server.pem"] {
        assert!(
            names.iter().any(|name| name.ends_with(wanted)),
            "{wanted} in {names:?}"
        );
    }
    assert!(
        manifest
            .licenses_left_out
            .iter()
            .any(|item| item.name.ends_with("license.lic")),
        "{:?}",
        manifest.licenses_left_out
    );
    let copied: Vec<String> = manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .map(|file| file.original_path.display().to_string())
        .collect();
    assert!(
        copied.iter().any(|file| file.ends_with("settings.json")),
        "{copied:?}"
    );
    assert!(
        copied.iter().any(|file| file.ends_with("cacert.pem")),
        "a certificate an app ships is not a secret: {copied:?}"
    );
    let every_warning = serde_json::to_string(&manifest.warnings)?;
    assert!(!every_warning.contains(&secret), "{every_warning}");

    let on = package_of("on", true)?;
    assert_eq!(
        files_holding(&on.package_root, &secret)?,
        Vec::<PathBuf>::new()
    );
    assert!(
        !files_holding(&on.package_root, &license)?.is_empty(),
        "with the option on, it is what the option asked for"
    );
    Ok(())
}
