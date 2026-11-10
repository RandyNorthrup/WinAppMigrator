//! Licensing/activation artifact discovery.
//!
//! This module helps an application's owner migrate their **own** license to a
//! new PC. It is deliberately conservative and read-only: it locates likely
//! license files, framework stores, environment variables, and registry
//! entries, and reports how transferable each is. It never decrypts license
//! data, never bypasses or emulates activation, and never emulates a hardware
//! dongle. Many licenses are machine-bound and will require reactivation on the
//! target; every discovered artifact is reported with that caveat.

use std::path::Path;

use winreg::RegKey;
use winreg::enums::KEY_READ;

use crate::hashing::sha256_file;
use crate::models::{
    FileEntry, LicenseArtifact, LicenseArtifactKind, LicenseFramework, RegistryEntry, RegistryHive,
    RegistryValueKind, TransferableHint, Warning, WarningSeverity,
};
use crate::path_utils::{DEEPEST_WALK, path_is_within, text_mentions_path, walk_asking};
use crate::product_match::{MatchScope, ProductNames};
use crate::registry_scan::registry_value_text;
use crate::secret_rules::{LeftOut, is_dpapi_hex, is_license_registry_entry, registry_entry_name};
use crate::system_context::SystemContext;

/// File extensions that strongly indicate a license/activation file.
const STRONG_LICENSE_EXTENSIONS: &[&str] = &["lic", "lservrc", "v2c", "rlm", "entitlement"];
/// Ambiguous extensions only treated as license data when the file name itself
/// signals licensing. `.txt` is intentionally excluded: it is overwhelmingly a
/// EULA / third-party license *text* (e.g. `7za_license.txt`), not the owner's
/// activation data, and produced false positives.
const AMBIGUOUS_LICENSE_EXTENSIONS: &[&str] = &["dat", "key", "bin", "xml"];
const LICENSE_NAME_SIGNALS: &[&str] = &["license", "licence", "activation", "entitlement"];
/// How many dots make a name a namespace rather than a file with an extension.
const NAMESPACE_DOTS: usize = 3;
/// Registry value/sub-key names that indicate stored licensing data.
const LICENSE_REGISTRY_SIGNALS: &[&str] = &[
    "license",
    "licence",
    "licensing",
    "activation",
    "serialnumber",
    "serialkey",
    "productkey",
    "regcode",
    "registrationcode",
    "licensekey",
];
const MACHINE_ENVIRONMENT_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";
const USER_ENVIRONMENT_KEY: &str = "Environment";
/// Cap on registry sub-keys enumerated per vendor key to keep the scan bounded.
const MAX_VENDOR_SUBKEYS: usize = 256;
/// `serial` by itself is a word many a setting has: `SerialPort` says nothing about a license.
const SERIAL_WORD: &str = "serial";
/// `Inno Setup: User Info: Serial`, where Inno Setup keeps a serial typed into a setup.
const INNO_SETUP_SERIAL: &str = "innosetupuserinfoserial";
/// What says an app uses a license framework, by how a file of it starts, in lowercase. A store is
/// on the PC for whatever uses that framework, and without one of these, this app is not one.
const FLEXLM_CLIENT_FILES: &[&str] = &["lmgr", "lmutil", "lmgrd", "flexnet", "fnp_", "fnpcommsvc"];
const CODEMETER_CLIENT_FILES: &[&str] = &["wibucm", "codemeter", "axprotector"];
const SENTINEL_CLIENT_FILES: &[&str] = &["hasp", "aksusb", "sntl", "apidsp", "lsapi", "lsdsp"];
const LICENSE_FILE_VARIABLE_SUFFIX: &str = "_LICENSE_FILE";

pub(crate) struct LicenseScanContext<'a> {
    pub publisher: Option<&'a str>,
    pub discovered_files: &'a [FileEntry],
    /// What was captured from under the app's own keys, which is where its license values are.
    pub registry_entries: &'a [RegistryEntry],
    /// The names the app goes by, and where it is, as its PC calls them. A variable that names
    /// neither is another app's business.
    pub names: &'a ProductNames,
    pub executable: &'a Path,
    pub install_location: Option<&'a Path>,
    pub include_license_data: bool,
    pub include_hashes: bool,
}

/// Classification of a matched license file.
struct LicenseFileClass {
    framework: LicenseFramework,
    transferable: TransferableHint,
    machine_bound: bool,
}

/// Whether a file is what these rules call a license.
pub(crate) fn is_license_file(path: &Path) -> bool {
    license_file_match(path).is_some()
}

pub(crate) fn scan_license_artifacts(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    left_out: &mut LeftOut,
) -> (Vec<LicenseArtifact>, Vec<RegistryEntry>, Vec<Warning>) {
    let mut artifacts = Vec::new();
    let mut registry_entries = Vec::new();
    let mut warnings = Vec::new();

    detect_license_files(context, &mut artifacts, left_out);
    detect_framework_stores(context, system, &mut artifacts);
    detect_license_env_vars(context, system, &mut artifacts);
    detect_captured_license_values(context, &mut artifacts, &mut registry_entries, left_out);
    detect_vendor_license_values(context, system, &mut artifacts);
    dedupe_registry_entries(&mut registry_entries);

    if !artifacts.is_empty() {
        // Informational only: the installer restores license data automatically.
        // No manual action is required of the user.
        warnings.push(Warning::new(
            "license_data_detected",
            WarningSeverity::Info,
            "Licensing/activation data was detected. With the License option on, the installer restores what can be moved; machine-wide values and hardware-bound stores are only reported. WinAppMigrator never decrypts, bypasses activation, or emulates dongles.",
            None,
        ));
        if artifacts.iter().any(|artifact| artifact.machine_bound) {
            warnings.push(Warning::new(
                "license_machine_bound",
                WarningSeverity::Info,
                "Some licenses are machine-bound (hardware dongle, machine-locked activation, or encrypted). The installer restores what it can; these may still require reactivation on the new PC.",
                None,
            ));
        }
    }

    (artifacts, registry_entries, warnings)
}

/// Drop duplicate captured registry values (the WOW64 32-bit view can surface
/// the same value under two key paths).
fn dedupe_registry_entries(entries: &mut Vec<RegistryEntry>) {
    let mut seen = std::collections::HashSet::new();
    entries.retain(|entry| {
        seen.insert((
            format!("{:?}", entry.hive),
            entry.key_path.clone(),
            entry.value_name.clone(),
        ))
    });
}

/// Each license file. With the License option off that is all that is known about one: where it
/// is, to say that it was left out. It is not read, and goes in no package.
fn detect_license_files(
    context: &LicenseScanContext<'_>,
    artifacts: &mut Vec<LicenseArtifact>,
    left_out: &mut LeftOut,
) {
    for file in context.discovered_files {
        let Some(class) = license_file_match(&file.original_path) else {
            continue;
        };
        let requires_admin = is_protected_path(&file.original_path);
        let should_copy = context.include_license_data;
        if !should_copy {
            left_out.license(file.original_path.display().to_string());
        }
        let hash_sha256 = (should_copy && context.include_hashes)
            .then(|| sha256_file(&file.path).ok())
            .flatten();
        artifacts.push(LicenseArtifact {
            kind: LicenseArtifactKind::File,
            framework: class.framework,
            path: Some(file.original_path.clone()),
            registry_location: None,
            detail: format!(
                "License file detected: {}",
                file.original_path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default()
            ),
            machine_bound: class.machine_bound,
            requires_admin,
            transferable: class.transferable,
            should_copy,
            package_relative_path: None,
            size_bytes: file.size_bytes.filter(|_| should_copy),
            hash_sha256,
            store_files: Vec::new(),
        });
    }
}

/// Map a file path to a license classification, if it looks like license data.
fn license_file_match(path: &Path) -> Option<LicenseFileClass> {
    let name = path.file_name()?.to_str()?.to_ascii_lowercase();
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_default();

    if STRONG_LICENSE_EXTENSIONS.contains(&extension.as_str()) {
        return Some(match extension.as_str() {
            "v2c" => LicenseFileClass {
                framework: LicenseFramework::SentinelHaspLdk,
                transferable: TransferableHint::Unlikely,
                machine_bound: true,
            },
            "rlm" => LicenseFileClass {
                framework: LicenseFramework::RepriseRlm,
                transferable: TransferableHint::Unknown,
                machine_bound: false,
            },
            "lservrc" => LicenseFileClass {
                framework: LicenseFramework::SentinelHaspLdk,
                transferable: TransferableHint::Unknown,
                machine_bound: false,
            },
            // ".lic" and ".entitlement" are typically plain text licenses.
            _ => LicenseFileClass {
                framework: LicenseFramework::FileBased,
                transferable: TransferableHint::Likely,
                machine_bound: false,
            },
        });
    }

    // Ambiguous extensions only count when the file name itself signals a license. One named like
    // `Windows.ApplicationModel.Activation.Thing.xml` is the documentation of a library by that
    // name, and says nothing about anyone's license.
    let name_signals = LICENSE_NAME_SIGNALS
        .iter()
        .any(|signal| name.contains(signal))
        && name.matches('.').count() < NAMESPACE_DOTS;
    if name_signals && AMBIGUOUS_LICENSE_EXTENSIONS.contains(&extension.as_str()) {
        return Some(LicenseFileClass {
            framework: LicenseFramework::FileBased,
            transferable: TransferableHint::Unknown,
            machine_bound: false,
        });
    }
    None
}

fn detect_framework_stores(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    let Some(program_data) = system.program_data() else {
        return;
    };
    let stores: &[(&str, LicenseFramework, bool, &[&str])] = &[
        (
            r"CodeMeter\CmAct",
            LicenseFramework::CodeMeter,
            true,
            CODEMETER_CLIENT_FILES,
        ),
        (
            r"Safenet Sentinel",
            LicenseFramework::SentinelHaspLdk,
            true,
            SENTINEL_CLIENT_FILES,
        ),
        (
            "FlexLM",
            LicenseFramework::FlexLm,
            false,
            FLEXLM_CLIENT_FILES,
        ),
        (
            "FLEXlm",
            LicenseFramework::FlexLm,
            false,
            FLEXLM_CLIENT_FILES,
        ),
    ];
    // By what is in the app's own folder. What a person ticked as a near match is something else's as
    // often as not, and says nothing of what the app itself uses.
    let own_files: Vec<&FileEntry> = context
        .discovered_files
        .iter()
        .filter(|file| {
            context
                .install_location
                .is_none_or(|folder| path_is_within(&system.logical_path(&file.path), folder))
        })
        .collect();
    for (relative, framework, machine_bound, client_files) in stores {
        // These are on the PC for whatever uses the framework, and may hold other apps' licenses.
        if !app_has_a_file_starting(&own_files, client_files) {
            continue;
        }
        let path = program_data.join(relative);
        // Any account can make a folder in `ProgramData`, so one by this name can be a link to
        // whatever an administrator can read, and it would be packaged as a license.
        let is_real_folder = system.may_read(&path)
            && crate::source_fs::symlink_metadata(&path)
                .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink());
        if is_real_folder && !system.is_in_another_profile(&path) {
            let machine_bound = *machine_bound;
            artifacts.push(LicenseArtifact {
                kind: LicenseArtifactKind::FrameworkStore,
                framework: framework.clone(),
                path: Some(path.clone()),
                registry_location: None,
                detail: format!("{} license store present", framework.label()),
                machine_bound,
                requires_admin: true,
                transferable: if machine_bound {
                    TransferableHint::Unlikely
                } else {
                    TransferableHint::Unknown
                },
                // Machine-bound stores (CodeMeter/Sentinel) are hardware/activation
                // bound and never copied. Non-machine-bound stores (FlexLM) are
                // copied and restored when license backup is enabled.
                should_copy: context.include_license_data && !machine_bound,
                package_relative_path: None,
                size_bytes: (context.include_license_data && !machine_bound)
                    .then(|| size_of_a_store(&path, system)),
                hash_sha256: None,
                store_files: Vec::new(),
            });
        }
    }
}

/// How big a store is, for what a build says it needs. By the walk that copies it, so what that
/// would not read is not counted.
fn size_of_a_store(folder: &Path, system: &SystemContext) -> u64 {
    walk_asking(folder, DEEPEST_WALK, |path, _| system.may_read(path))
        .flatten()
        .filter(|entry| entry.file_type.is_file())
        .filter_map(|entry| entry.metadata)
        .map(|metadata| metadata.len())
        .fold(0, u64::saturating_add)
}

fn app_has_a_file_starting(files: &[&FileEntry], starts: &[&str]) -> bool {
    files.iter().any(|file| {
        file.original_path
            .file_name()
            .map(|name| name.to_string_lossy().to_ascii_lowercase())
            .is_some_and(|name| starts.iter().any(|start| name.starts_with(start)))
    })
}

/// From the registry of the PC and of the account being scanned. This process's own could be
/// another administrator's, named at the UAC prompt. Only by name, however it was asked for, and
/// only one that is this app's: it names the app's own place, or is named for the app.
fn detect_license_env_vars(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    let mut keys: Vec<RegKey> = system
        .open_hklm(MACHINE_ENVIRONMENT_KEY, KEY_READ)
        .into_iter()
        .collect();
    for user in system.user_hives() {
        keys.extend(user.open(USER_ENVIRONMENT_KEY, KEY_READ).ok());
    }
    let variables = keys.iter().flat_map(|key| key.enum_values().flatten());
    for (name, value) in variables {
        if !is_license_env_var(&name)
            || !license_variable_is_the_apps(context, &name, &registry_value_text(&value))
        {
            continue;
        }
        artifacts.push(LicenseArtifact {
            kind: LicenseArtifactKind::EnvironmentVariable,
            framework: LicenseFramework::EnvironmentVariable,
            path: None,
            registry_location: None,
            detail: format!("License environment variable {name}"),
            machine_bound: false,
            requires_admin: false,
            transferable: TransferableHint::Unknown,
            should_copy: false,
            package_relative_path: None,
            size_bytes: None,
            hash_sha256: None,
            store_files: Vec::new(),
        });
    }
}

/// `ACME_LICENSE_FILE` is Acme's, and any such variable is the app's if it points at the app.
/// `LM_LICENSE_FILE` is for everything on the PC that uses FlexLM, so only what it says counts.
fn license_variable_is_the_apps(context: &LicenseScanContext<'_>, name: &str, value: &str) -> bool {
    let points_at_the_app = text_mentions_path(value, context.executable, true)
        || context
            .install_location
            .is_some_and(|location| text_mentions_path(value, location, false));
    let upper = name.to_ascii_uppercase();
    let prefix = upper
        .strip_suffix(LICENSE_FILE_VARIABLE_SUFFIX)
        .unwrap_or_default();
    points_at_the_app
        || context
            .names
            .matches_top_level(prefix, MatchScope::ProgramFolders)
}

pub(crate) fn is_license_env_var(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    // Skip build-tool variables that merely end with the suffix (e.g. cargo sets
    // CARGO_PKG_LICENSE_FILE) — they are not the application's license.
    if upper.starts_with("CARGO_") || upper.starts_with("RUST") {
        return false;
    }
    upper == "LM_LICENSE_FILE" || upper.ends_with(LICENSE_FILE_VARIABLE_SUFFIX)
}

/// License values among what was captured from under the app's own keys. Their locations are
/// always reported. A per-user one is passed on with the License option on, and a machine-wide one
/// is only ever reported. The scan takes all of them out of what it restores as settings, so that
/// they go by the option, and by `--no-restore-license`.
fn detect_captured_license_values(
    context: &LicenseScanContext<'_>,
    artifacts: &mut Vec<LicenseArtifact>,
    captured: &mut Vec<RegistryEntry>,
    left_out: &mut LeftOut,
) {
    for entry in context
        .registry_entries
        .iter()
        .filter(|entry| is_license_registry_entry(entry, context.names))
    {
        let Some(value_name) = entry.value_name.as_deref() else {
            continue;
        };
        let machine_wide = entry.hive == RegistryHive::LocalMachine;
        // A blob DPAPI made is bound to that account on that PC, and will need reactivating.
        let dpapi = entry.value_kind == Some(RegistryValueKind::Binary)
            && entry.value_bytes_hex.as_deref().is_some_and(is_dpapi_hex);
        artifacts.push(registry_artifact(
            hive_label(&entry.hive),
            &entry.key_path,
            Some(value_name),
            machine_wide,
            machine_wide || dpapi,
            format!("Registry value suggests licensing data: {value_name}"),
        ));
        if !context.include_license_data {
            left_out.license(registry_entry_name(entry));
        } else if !machine_wide
            && entry
                .value_kind
                .as_ref()
                .is_some_and(is_restorable_value_kind)
        {
            captured.push(entry.clone());
        }
    }
}

fn hive_label(hive: &RegistryHive) -> &'static str {
    match hive {
        RegistryHive::LocalMachine => "HKLM",
        RegistryHive::CurrentUser => "HKCU",
    }
}

/// What a publisher's own key says about a license, which could be for any of its products, so it
/// is only reported, and only by name.
fn detect_vendor_license_values(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    let Some(publisher) = context.publisher else {
        return;
    };
    let vendor = sanitize_key_segment(publisher);
    if vendor.is_empty() {
        return;
    }
    let report = |key: &RegKey, key_path: &str, hive: &str, admin: bool, found: &mut Vec<_>| {
        for (value_name, _) in key.enum_values().flatten() {
            if is_license_registry_name(&value_name) {
                found.push(registry_artifact(
                    hive,
                    key_path,
                    Some(&value_name),
                    admin,
                    admin,
                    format!("Registry value suggests licensing data: {value_name}"),
                ));
            }
        }
        for subkey in key.enum_keys().flatten().take(MAX_VENDOR_SUBKEYS) {
            if is_license_registry_name(&subkey) {
                let sub_path = format!(r"{key_path}\{subkey}");
                found.push(registry_artifact(
                    hive,
                    &sub_path,
                    None,
                    admin,
                    admin,
                    format!("Registry key suggests licensing data: {hive}\\{sub_path}"),
                ));
            }
        }
    };
    for prefix in [r"SOFTWARE", r"SOFTWARE\WOW6432Node"] {
        let key_path = format!(r"{prefix}\{vendor}");
        if let Ok(key) = system.open_hklm(&key_path, KEY_READ) {
            report(&key, &key_path, "HKLM", true, artifacts);
        }
        for user in system.user_hives() {
            if let Ok(key) = user.open(&key_path, KEY_READ) {
                report(&key, &key_path, "HKCU", false, artifacts);
            }
        }
    }
}

/// Whether a registry value kind holds restorable license data. Text, numeric,
/// and binary values are captured (binary covers serials/tokens and DPAPI blobs,
/// the latter restored best-effort and flagged machine-bound). Opaque kinds
/// (None, links, resource lists) are not license data and are skipped.
fn is_restorable_value_kind(kind: &RegistryValueKind) -> bool {
    matches!(
        kind,
        RegistryValueKind::String
            | RegistryValueKind::ExpandString
            | RegistryValueKind::Dword
            | RegistryValueKind::Qword
            | RegistryValueKind::MultiString
            | RegistryValueKind::Binary
    )
}

fn registry_artifact(
    hive_name: &str,
    key_path: &str,
    value_name: Option<&str>,
    requires_admin: bool,
    machine_bound: bool,
    detail: String,
) -> LicenseArtifact {
    let location = match value_name {
        Some(value) => format!(r"{hive_name}\{key_path}\{value}"),
        None => format!(r"{hive_name}\{key_path}"),
    };
    LicenseArtifact {
        kind: LicenseArtifactKind::RegistryValue,
        framework: LicenseFramework::RegistryBased,
        path: None,
        registry_location: Some(location),
        detail,
        machine_bound,
        requires_admin,
        transferable: if machine_bound {
            TransferableHint::Unlikely
        } else {
            TransferableHint::Unknown
        },
        should_copy: false,
        package_relative_path: None,
        size_bytes: None,
        hash_sha256: None,
        store_files: Vec::new(),
    }
}

pub(crate) fn is_license_registry_name(name: &str) -> bool {
    let normalized: String = name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    LICENSE_REGISTRY_SIGNALS
        .iter()
        .any(|signal| normalized.contains(signal))
        || normalized == SERIAL_WORD
        || normalized == INNO_SETUP_SERIAL
}

fn is_protected_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.contains(r"\program files") || lower.contains(r"\programdata")
}

fn sanitize_key_segment(value: &str) -> String {
    value
        .chars()
        .filter(|character| !matches!(character, '\\' | '/'))
        .collect::<String>()
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_strong_license_extensions() {
        assert!(matches!(
            license_file_match(Path::new(r"C:\App\product.lic")),
            Some(LicenseFileClass {
                framework: LicenseFramework::FileBased,
                transferable: TransferableHint::Likely,
                ..
            })
        ));
        assert!(matches!(
            license_file_match(Path::new(r"C:\App\activation_1.v2c")),
            Some(LicenseFileClass {
                framework: LicenseFramework::SentinelHaspLdk,
                machine_bound: true,
                ..
            })
        ));
    }

    #[test]
    fn ambiguous_extension_requires_name_signal() {
        assert!(license_file_match(Path::new(r"C:\App\data.dat")).is_none());
        assert!(license_file_match(Path::new(r"C:\App\license.dat")).is_some());
        assert!(license_file_match(Path::new(r"C:\App\readme.txt")).is_none());
        // EULA / third-party license *text* must not be flagged as the owner's
        // license (regression: 7za_license.txt false positive on Solar Fire).
        assert!(license_file_match(Path::new(r"C:\App\7za_license.txt")).is_none());
        assert!(license_file_match(Path::new(r"C:\App\license.txt")).is_none());
        assert!(license_file_match(Path::new(r"C:\App\app.license.xml")).is_some());
        assert!(
            license_file_match(Path::new(
                r"C:\Kits\Windows.ApplicationModel.Activation.Contract.xml"
            ))
            .is_none(),
            "the documentation of a library of that name"
        );
    }

    #[test]
    fn recognizes_license_env_vars() {
        assert!(is_license_env_var("LM_LICENSE_FILE"));
        assert!(is_license_env_var("ACME_LICENSE_FILE"));
        assert!(!is_license_env_var("PATH"));
        // Build-tool variables that merely end with the suffix are not licenses.
        assert!(!is_license_env_var("CARGO_PKG_LICENSE_FILE"));
    }

    fn context_of<'a>(names: &'a ProductNames, location: &'a Path) -> LicenseScanContext<'a> {
        LicenseScanContext {
            publisher: None,
            discovered_files: &[],
            registry_entries: &[],
            names,
            executable: Path::new(r"C:\Program Files\Acme\acme.exe"),
            install_location: Some(location),
            include_license_data: false,
            include_hashes: false,
        }
    }

    #[test]
    fn a_license_variable_is_the_apps_if_it_says_so_and_not_otherwise() {
        let names = ProductNames::new(Some("Acme"), None, &[], &[]);
        let context = context_of(&names, Path::new(r"C:\Program Files\Acme"));
        for (name, value) in [
            ("ACME_LICENSE_FILE", r"\\server\licenses\site.lic"),
            ("LM_LICENSE_FILE", r"C:\Program Files\Acme\site.lic"),
            ("OTHER_LICENSE_FILE", r"C:\Program Files\Acme\site.lic"),
        ] {
            assert!(
                license_variable_is_the_apps(&context, name, value),
                "{name}"
            );
        }
        for (name, value) in [
            ("LM_LICENSE_FILE", r"\\server\licenses\site.lic"),
            ("LM_LICENSE_FILE", r"C:\Program Files\Other\site.lic"),
            ("OTHER_LICENSE_FILE", r"C:\Program Files\Other\site.lic"),
            (
                "ACMEOTHER_LICENSE_FILE",
                r"C:\Program Files\Acme Other\x.lic",
            ),
        ] {
            assert!(
                !license_variable_is_the_apps(&context, name, value),
                "{name}"
            );
        }
    }

    fn found_file(path: &str) -> FileEntry {
        FileEntry {
            path: path.into(),
            original_path: path.into(),
            package_relative_path: Some("files/x".into()),
            size_bytes: Some(7),
            hash_sha256: Some("00".repeat(32)),
            classification: crate::models::FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    #[test]
    fn with_the_option_off_a_license_file_is_a_name_and_nothing_more() {
        let names = ProductNames::new(Some("Acme"), None, &[], &[]);
        let files = [found_file(r"C:\A\license.lic"), found_file(r"C:\A\app.exe")];
        let context = LicenseScanContext {
            discovered_files: &files,
            ..context_of(&names, Path::new(r"C:\A"))
        };
        let mut artifacts = Vec::new();
        let mut left_out = LeftOut::default();
        detect_license_files(&context, &mut artifacts, &mut left_out);
        assert_eq!(artifacts.len(), 1);
        let artifact = &artifacts[0];
        assert!(!artifact.should_copy);
        assert_eq!(artifact.hash_sha256, None);
        assert_eq!(artifact.size_bytes, None);
        assert_eq!(artifact.package_relative_path, None);
        assert_eq!(left_out.licenses.len(), 1);
        assert_eq!(left_out.licenses[0].name, r"C:\A\license.lic");
    }

    #[test]
    fn a_license_store_is_only_for_an_app_that_uses_that_framework() {
        let with = [found_file(r"C:\A\lmgr11.dll")];
        let without = [found_file(r"C:\A\app.exe")];
        assert!(app_has_a_file_starting(
            &with.iter().collect::<Vec<_>>(),
            FLEXLM_CLIENT_FILES
        ));
        assert!(!app_has_a_file_starting(
            &without.iter().collect::<Vec<_>>(),
            FLEXLM_CLIENT_FILES
        ));
    }

    fn registry_value(hive: RegistryHive, name: &str) -> RegistryEntry {
        RegistryEntry {
            hive,
            key_path: r"Software\Acme\App".to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some("x".to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: Some("7800".to_owned()),
            confidence: crate::models::MatchConfidence::Medium,
            requires_admin: false,
        }
    }

    #[test]
    fn a_license_value_goes_by_the_option_and_a_machine_wide_one_is_only_reported() {
        let names = ProductNames::new(Some("Acme"), None, &[], &[]);
        let entries = [
            registry_value(RegistryHive::CurrentUser, "LicenseKey"),
            registry_value(RegistryHive::LocalMachine, "SerialNumber"),
            registry_value(RegistryHive::CurrentUser, "Theme"),
        ];
        let context = LicenseScanContext {
            registry_entries: &entries,
            ..context_of(&names, Path::new(r"C:\A"))
        };
        let (mut artifacts, mut captured, mut left_out) =
            (Vec::new(), Vec::new(), LeftOut::default());
        detect_captured_license_values(&context, &mut artifacts, &mut captured, &mut left_out);
        assert!(captured.is_empty(), "off means not captured");
        assert_eq!(artifacts.len(), 2);
        assert_eq!(left_out.licenses.len(), 2);

        let on = LicenseScanContext {
            include_license_data: true,
            ..context
        };
        let (mut artifacts, mut captured, mut left_out) =
            (Vec::new(), Vec::new(), LeftOut::default());
        detect_captured_license_values(&on, &mut artifacts, &mut captured, &mut left_out);
        assert_eq!(captured.len(), 1, "only the per-user one");
        assert_eq!(captured[0].value_name.as_deref(), Some("LicenseKey"));
        assert!(left_out.licenses.is_empty());
    }

    #[test]
    fn restorable_value_kinds_cover_license_data() {
        // Text, numeric, and binary license values are captured for restore...
        assert!(is_restorable_value_kind(&RegistryValueKind::String));
        assert!(is_restorable_value_kind(&RegistryValueKind::ExpandString));
        assert!(is_restorable_value_kind(&RegistryValueKind::Dword));
        assert!(is_restorable_value_kind(&RegistryValueKind::Qword));
        assert!(is_restorable_value_kind(&RegistryValueKind::MultiString));
        assert!(is_restorable_value_kind(&RegistryValueKind::Binary));
        // ...but opaque, non-data kinds are skipped.
        assert!(!is_restorable_value_kind(&RegistryValueKind::None));
        assert!(!is_restorable_value_kind(&RegistryValueKind::Link));
    }

    #[test]
    fn recognizes_license_registry_names() {
        assert!(is_license_registry_name("LicenseKey"));
        assert!(is_license_registry_name("Serial Number"));
        assert!(is_license_registry_name("ProductKey"));
        assert!(is_license_registry_name("Serial"));
        assert!(is_license_registry_name("Inno Setup: User Info: Serial"));
        assert!(!is_license_registry_name("Inno Setup: User Info: Name"));
        assert!(!is_license_registry_name("InstallPath"));
        assert!(!is_license_registry_name("SerialPort"));
        assert!(!is_license_registry_name("Serializer"));
        assert!(!is_license_registry_name("SerializationFormat"));
    }
}
