//! Licensing/activation artifact discovery.
//!
//! This module helps an application's owner migrate their **own** license to a
//! new PC. It is deliberately conservative and read-only: it locates likely
//! license files, framework stores, environment variables, and registry
//! entries, and reports how transferable each is. It never decrypts license
//! data, never bypasses or emulates activation, and never emulates a hardware
//! dongle. Many licenses are machine-bound and will require reactivation on the
//! target; every discovered artifact is reported with that caveat.

use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY};

use crate::hashing::sha256_file;
use crate::models::{
    FileEntry, LicenseArtifact, LicenseArtifactKind, LicenseFramework, TransferableHint, Warning,
    WarningSeverity,
};

/// File extensions that strongly indicate a license/activation file.
const STRONG_LICENSE_EXTENSIONS: &[&str] = &["lic", "lservrc", "v2c", "rlm", "entitlement"];
/// Ambiguous extensions only treated as license data when the file name itself
/// signals licensing.
const AMBIGUOUS_LICENSE_EXTENSIONS: &[&str] = &["dat", "key", "bin", "txt", "xml"];
const LICENSE_NAME_SIGNALS: &[&str] = &["license", "licence", "activation", "entitlement"];
/// Registry value/sub-key names that indicate stored licensing data.
const LICENSE_REGISTRY_SIGNALS: &[&str] = &[
    "license",
    "licence",
    "licensing",
    "activation",
    "serialnumber",
    "serial",
    "productkey",
    "regcode",
    "registrationcode",
    "licensekey",
];
/// Cap on registry sub-keys enumerated per vendor key to keep the scan bounded.
const MAX_VENDOR_SUBKEYS: usize = 256;

pub struct LicenseScanContext<'a> {
    pub publisher: Option<&'a str>,
    pub discovered_files: &'a [FileEntry],
    pub include_license_data: bool,
    pub include_hashes: bool,
}

/// Classification of a matched license file.
struct LicenseFileClass {
    framework: LicenseFramework,
    transferable: TransferableHint,
    machine_bound: bool,
}

pub fn scan_license_artifacts(
    context: &LicenseScanContext<'_>,
) -> (Vec<LicenseArtifact>, Vec<Warning>) {
    let mut artifacts = Vec::new();
    let mut warnings = Vec::new();

    detect_license_files(context, &mut artifacts);
    detect_framework_stores(&mut artifacts);
    detect_license_env_vars(&mut artifacts);
    detect_registry_licenses(context, &mut artifacts);

    if !artifacts.is_empty() {
        warnings.push(Warning::new(
            "license_data_detected",
            WarningSeverity::Warning,
            "Licensing data was detected. WinAppMigrator captures it to help migrate your own license; it never decrypts, bypasses activation, or emulates dongles. Review each item before relying on it.",
            None,
        ));
        if artifacts.iter().any(|artifact| artifact.machine_bound) {
            warnings.push(Warning::new(
                "license_machine_bound",
                WarningSeverity::Warning,
                "One or more licenses are machine-bound (hardware dongle, machine-locked activation, or encrypted) and will likely require reactivation on the target PC.",
                None,
            ));
        }
        if context.include_license_data {
            warnings.push(Warning::new(
                "license_data_included",
                WarningSeverity::Info,
                "License files are copied into the package `license/` directory. They are not restored automatically; place and reactivate them manually on the target.",
                None,
            ));
        }
    }

    (artifacts, warnings)
}

fn detect_license_files(context: &LicenseScanContext<'_>, artifacts: &mut Vec<LicenseArtifact>) {
    for file in context.discovered_files {
        let Some(class) = license_file_match(&file.original_path) else {
            continue;
        };
        let requires_admin = is_protected_path(&file.original_path);
        let should_copy = context.include_license_data;
        let hash_sha256 = if should_copy && context.include_hashes {
            sha256_file(&file.original_path).ok()
        } else {
            file.hash_sha256.clone()
        };
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
            size_bytes: file.size_bytes,
            hash_sha256,
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

    // Ambiguous extensions only count when the file name itself signals a license.
    let name_signals = LICENSE_NAME_SIGNALS
        .iter()
        .any(|signal| name.contains(signal));
    if name_signals && AMBIGUOUS_LICENSE_EXTENSIONS.contains(&extension.as_str()) {
        return Some(LicenseFileClass {
            framework: LicenseFramework::FileBased,
            transferable: TransferableHint::Unknown,
            machine_bound: false,
        });
    }
    None
}

fn detect_framework_stores(artifacts: &mut Vec<LicenseArtifact>) {
    let program_data = std::env::var_os("ProgramData").map(PathBuf::from);
    let Some(program_data) = program_data else {
        return;
    };
    let stores: &[(&str, LicenseFramework, bool)] = &[
        (r"CodeMeter\CmAct", LicenseFramework::CodeMeter, true),
        (r"Safenet Sentinel", LicenseFramework::SentinelHaspLdk, true),
        ("FlexLM", LicenseFramework::FlexLm, false),
        ("FLEXlm", LicenseFramework::FlexLm, false),
    ];
    for (relative, framework, machine_bound) in stores {
        let path = program_data.join(relative);
        if path.is_dir() {
            artifacts.push(LicenseArtifact {
                kind: LicenseArtifactKind::FrameworkStore,
                framework: framework.clone(),
                path: Some(path.clone()),
                registry_location: None,
                detail: format!("{} license store present", framework.label()),
                machine_bound: *machine_bound,
                requires_admin: true,
                transferable: if *machine_bound {
                    TransferableHint::Unlikely
                } else {
                    TransferableHint::Unknown
                },
                // Framework stores are reported, not bulk-copied: they are
                // machine-wide and typically hardware/activation bound.
                should_copy: false,
                package_relative_path: None,
                size_bytes: None,
                hash_sha256: None,
            });
        }
    }
}

fn detect_license_env_vars(artifacts: &mut Vec<LicenseArtifact>) {
    for (name, value) in std::env::vars_os() {
        let Some(name) = name.to_str() else {
            continue;
        };
        if !is_license_env_var(name) {
            continue;
        }
        artifacts.push(LicenseArtifact {
            kind: LicenseArtifactKind::EnvironmentVariable,
            framework: LicenseFramework::EnvironmentVariable,
            path: None,
            registry_location: None,
            detail: format!(
                "License environment variable {name}={}",
                value.to_string_lossy()
            ),
            machine_bound: false,
            requires_admin: false,
            transferable: TransferableHint::Unknown,
            should_copy: false,
            package_relative_path: None,
            size_bytes: None,
            hash_sha256: None,
        });
    }
}

fn is_license_env_var(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper == "LM_LICENSE_FILE" || upper.ends_with("_LICENSE_FILE")
}

fn detect_registry_licenses(
    context: &LicenseScanContext<'_>,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    let Some(publisher) = context.publisher else {
        return;
    };
    let vendor = sanitize_key_segment(publisher);
    if vendor.is_empty() {
        return;
    }
    for (hive, hive_name, requires_admin) in [
        (HKEY_LOCAL_MACHINE, "HKLM", true),
        (HKEY_CURRENT_USER, "HKCU", false),
    ] {
        for prefix in [r"SOFTWARE", r"SOFTWARE\WOW6432Node"] {
            let key_path = format!(r"{prefix}\{vendor}");
            let root = RegKey::predef(hive);
            let Ok(vendor_key) = root.open_subkey_with_flags(&key_path, KEY_READ | KEY_WOW64_32KEY)
            else {
                continue;
            };
            scan_vendor_key(&vendor_key, hive_name, &key_path, requires_admin, artifacts);
        }
    }
}

/// Inspect a vendor key (and its immediate sub-keys) for value names that look
/// like stored licensing data. Values are reported by location only — they are
/// not copied, since they are often DPAPI-encrypted and machine-bound.
fn scan_vendor_key(
    vendor_key: &RegKey,
    hive_name: &str,
    key_path: &str,
    requires_admin: bool,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    report_license_values(vendor_key, hive_name, key_path, requires_admin, artifacts);
    for subkey_name in vendor_key.enum_keys().flatten().take(MAX_VENDOR_SUBKEYS) {
        let signals = is_license_registry_name(&subkey_name);
        let Ok(subkey) =
            vendor_key.open_subkey_with_flags(&subkey_name, KEY_READ | KEY_WOW64_32KEY)
        else {
            continue;
        };
        let sub_path = format!(r"{key_path}\{subkey_name}");
        if signals {
            artifacts.push(registry_artifact(
                hive_name,
                &sub_path,
                None,
                requires_admin,
                format!("Registry key suggests licensing data: {hive_name}\\{sub_path}"),
            ));
        }
        report_license_values(&subkey, hive_name, &sub_path, requires_admin, artifacts);
    }
}

fn report_license_values(
    key: &RegKey,
    hive_name: &str,
    key_path: &str,
    requires_admin: bool,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    for (value_name, _) in key.enum_values().flatten() {
        if is_license_registry_name(&value_name) {
            artifacts.push(registry_artifact(
                hive_name,
                key_path,
                Some(&value_name),
                requires_admin,
                format!("Registry value suggests licensing data: {value_name}"),
            ));
        }
    }
}

fn registry_artifact(
    hive_name: &str,
    key_path: &str,
    value_name: Option<&str>,
    requires_admin: bool,
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
        // HKLM licensing data is machine-scoped; values may also be DPAPI-bound.
        machine_bound: requires_admin,
        requires_admin,
        transferable: TransferableHint::Unknown,
        should_copy: false,
        package_relative_path: None,
        size_bytes: None,
        hash_sha256: None,
    }
}

fn is_license_registry_name(name: &str) -> bool {
    let normalized: String = name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    LICENSE_REGISTRY_SIGNALS
        .iter()
        .any(|signal| normalized.contains(signal))
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
    }

    #[test]
    fn recognizes_license_env_vars() {
        assert!(is_license_env_var("LM_LICENSE_FILE"));
        assert!(is_license_env_var("ACME_LICENSE_FILE"));
        assert!(!is_license_env_var("PATH"));
    }

    #[test]
    fn recognizes_license_registry_names() {
        assert!(is_license_registry_name("LicenseKey"));
        assert!(is_license_registry_name("Serial Number"));
        assert!(is_license_registry_name("ProductKey"));
        assert!(!is_license_registry_name("InstallPath"));
    }
}
