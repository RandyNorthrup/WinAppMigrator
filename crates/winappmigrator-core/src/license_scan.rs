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

use winreg::enums::{
    KEY_READ, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ,
    REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_RESOURCE_LIST,
    REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};
use winreg::{RegKey, RegValue};

use crate::hashing::{hex_lower, sha256_file};
use crate::models::{
    FileEntry, LicenseArtifact, LicenseArtifactKind, LicenseFramework, MatchConfidence,
    RegistryEntry, RegistryHive, RegistryValueKind, TransferableHint, Warning, WarningSeverity,
};
use crate::registry_scan::registry_value_text;
use crate::system_context::SystemContext;

/// File extensions that strongly indicate a license/activation file.
const STRONG_LICENSE_EXTENSIONS: &[&str] = &["lic", "lservrc", "v2c", "rlm", "entitlement"];
/// Ambiguous extensions only treated as license data when the file name itself
/// signals licensing. `.txt` is intentionally excluded: it is overwhelmingly a
/// EULA / third-party license *text* (e.g. `7za_license.txt`), not the owner's
/// activation data, and produced false positives.
const AMBIGUOUS_LICENSE_EXTENSIONS: &[&str] = &["dat", "key", "bin", "xml"];
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
    system: &SystemContext,
) -> (Vec<LicenseArtifact>, Vec<RegistryEntry>, Vec<Warning>) {
    let mut artifacts = Vec::new();
    let mut registry_entries = Vec::new();
    let mut warnings = Vec::new();

    detect_license_files(context, &mut artifacts);
    detect_framework_stores(context, system, &mut artifacts);
    // License environment variables can only be read from the live session; an
    // offline disk's are captured (path-matched) by the environment scan instead.
    if !system.is_offline() {
        detect_license_env_vars(&mut artifacts);
    }
    detect_registry_licenses(
        context,
        system,
        &mut artifacts,
        &mut registry_entries,
        &mut warnings,
    );
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

fn detect_framework_stores(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    artifacts: &mut Vec<LicenseArtifact>,
) {
    let Some(program_data) = system.program_data() else {
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
    // Skip build-tool variables that merely end with the suffix (e.g. cargo sets
    // CARGO_PKG_LICENSE_FILE) — they are not the application's license.
    if upper.starts_with("CARGO_") || upper.starts_with("RUST") {
        return false;
    }
    upper == "LM_LICENSE_FILE" || upper.ends_with("_LICENSE_FILE")
}

/// Context for a single hive sweep, threaded through the vendor-key walk so the
/// captured registry values carry the right hive and transferability.
struct RegistryScanCtx {
    hive: RegistryHive,
    hive_name: &'static str,
    requires_admin: bool,
    include_license_data: bool,
}

fn detect_registry_licenses(
    context: &LicenseScanContext<'_>,
    system: &SystemContext,
    artifacts: &mut Vec<LicenseArtifact>,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Some(publisher) = context.publisher else {
        return;
    };
    let vendor = sanitize_key_segment(publisher);
    if vendor.is_empty() {
        return;
    }
    // Read the native view for the plain `SOFTWARE` prefix and the 32-bit view
    // via the explicit `WOW6432Node` prefix, for both the machine hive and each
    // user hive (live: current user; offline: every profile).
    let hklm_ctx = RegistryScanCtx {
        hive: RegistryHive::LocalMachine,
        hive_name: "HKLM",
        requires_admin: true,
        include_license_data: context.include_license_data,
    };
    for prefix in [r"SOFTWARE", r"SOFTWARE\WOW6432Node"] {
        let key_path = format!(r"{prefix}\{vendor}");
        if let Ok(vendor_key) = system.open_hklm(&key_path, KEY_READ) {
            scan_vendor_key(
                &vendor_key,
                &hklm_ctx,
                &key_path,
                artifacts,
                entries,
                warnings,
            );
        }
    }
    for user in system.user_hives() {
        let user_ctx = RegistryScanCtx {
            hive: RegistryHive::CurrentUser,
            hive_name: "HKCU",
            requires_admin: false,
            include_license_data: context.include_license_data,
        };
        for prefix in [r"SOFTWARE", r"SOFTWARE\WOW6432Node"] {
            let key_path = format!(r"{prefix}\{vendor}");
            if let Ok(vendor_key) = user.open(&key_path, KEY_READ) {
                scan_vendor_key(
                    &vendor_key,
                    &user_ctx,
                    &key_path,
                    artifacts,
                    entries,
                    warnings,
                );
            }
        }
    }
}

/// Inspect a vendor key (and its immediate sub-keys) for value names that look
/// like stored licensing data. Locations are always reported. Transferable
/// (non-machine-bound, user-scoped) values are additionally captured so the
/// installer can restore them; machine-bound (HKLM) values are reported only.
fn scan_vendor_key(
    vendor_key: &RegKey,
    ctx: &RegistryScanCtx,
    key_path: &str,
    artifacts: &mut Vec<LicenseArtifact>,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    report_license_values(vendor_key, ctx, key_path, artifacts, entries);
    for subkey_name in vendor_key.enum_keys().flatten().take(MAX_VENDOR_SUBKEYS) {
        let sub_path = format!(r"{key_path}\{subkey_name}");
        // The key name alone can signal licensing data, so report the location
        // even when the key's values cannot be read below — a locked license key
        // must never be dropped without a trace.
        if is_license_registry_name(&subkey_name) {
            artifacts.push(registry_artifact(
                ctx.hive_name,
                &sub_path,
                None,
                ctx.requires_admin,
                ctx.requires_admin,
                format!(
                    "Registry key suggests licensing data: {}\\{sub_path}",
                    ctx.hive_name
                ),
            ));
        }
        match vendor_key.open_subkey_with_flags(&subkey_name, KEY_READ) {
            Ok(subkey) => report_license_values(&subkey, ctx, &sub_path, artifacts, entries),
            Err(error) => warnings.push(Warning::new(
                "license_subkey_unreadable",
                WarningSeverity::Info,
                format!(
                    "Could not read license registry key {}\\{sub_path} ({error}); any values it holds were not captured.",
                    ctx.hive_name
                ),
                Some(format!("{}\\{sub_path}", ctx.hive_name)),
            )),
        }
    }
}

fn report_license_values(
    key: &RegKey,
    ctx: &RegistryScanCtx,
    key_path: &str,
    artifacts: &mut Vec<LicenseArtifact>,
    entries: &mut Vec<RegistryEntry>,
) {
    for (value_name, value) in key.enum_values().flatten() {
        if !is_license_registry_name(&value_name) {
            continue;
        }
        let value_kind = map_registry_type(value.vtype.clone());
        // A DPAPI-encrypted binary blob is bound to the source user's master key,
        // so it is reported (and restored best-effort) but flagged machine-bound:
        // it will likely need reactivation rather than decrypting on the target.
        let dpapi = matches!(value_kind, RegistryValueKind::Binary) && is_dpapi_blob(&value.bytes);
        let machine_bound = ctx.requires_admin || dpapi;
        artifacts.push(registry_artifact(
            ctx.hive_name,
            key_path,
            Some(&value_name),
            ctx.requires_admin,
            machine_bound,
            format!("Registry value suggests licensing data: {value_name}"),
        ));
        // Capture user-scoped (HKCU) license values — including plain binary
        // serials/tokens and DPAPI blobs (best-effort, may need reactivation).
        // Machine-scoped (HKLM) license data is left as metadata only.
        if ctx.include_license_data && !ctx.requires_admin && is_restorable_value_kind(&value_kind)
        {
            entries.push(reg_value_to_entry(
                ctx.hive.clone(),
                key_path,
                &value_name,
                &value,
                ctx.requires_admin,
            ));
        }
    }
}

/// DPAPI provider GUID `{df9d8cd0-1501-11d1-8c7a-00c04fc297eb}` in the on-disk
/// (mixed-endian) byte order it appears in a `CryptProtectData` blob header.
const DPAPI_PROVIDER_GUID: [u8; 16] = [
    0xD0, 0x8C, 0x9D, 0xDF, 0x01, 0x15, 0xD1, 0x11, 0x8C, 0x7A, 0x00, 0xC0, 0x4F, 0xC2, 0x97, 0xEB,
];

/// Heuristically detect a DPAPI blob: a 4-byte version, then the DPAPI provider
/// GUID at offset 4.
fn is_dpapi_blob(bytes: &[u8]) -> bool {
    bytes.len() >= DPAPI_PROVIDER_GUID.len() + 4 && bytes[4..20] == DPAPI_PROVIDER_GUID
}

fn reg_value_to_entry(
    hive: RegistryHive,
    key_path: &str,
    value_name: &str,
    value: &RegValue,
    requires_admin: bool,
) -> RegistryEntry {
    let value_kind = map_registry_type(value.vtype.clone());
    let (value_data, value_bytes_hex) = match value_kind {
        RegistryValueKind::String | RegistryValueKind::ExpandString => {
            (Some(registry_value_text(value)), None)
        }
        _ => (None, Some(hex_lower(&value.bytes))),
    };
    RegistryEntry {
        hive,
        key_path: key_path.to_owned(),
        value_name: Some(value_name.to_owned()),
        value_data,
        value_kind: Some(value_kind),
        value_bytes_hex,
        confidence: MatchConfidence::Medium,
        requires_admin,
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

fn map_registry_type(vtype: RegType) -> RegistryValueKind {
    match vtype {
        REG_NONE => RegistryValueKind::None,
        REG_SZ => RegistryValueKind::String,
        REG_EXPAND_SZ => RegistryValueKind::ExpandString,
        REG_BINARY => RegistryValueKind::Binary,
        REG_DWORD => RegistryValueKind::Dword,
        REG_DWORD_BIG_ENDIAN => RegistryValueKind::DwordBigEndian,
        REG_LINK => RegistryValueKind::Link,
        REG_MULTI_SZ => RegistryValueKind::MultiString,
        REG_RESOURCE_LIST => RegistryValueKind::ResourceList,
        REG_FULL_RESOURCE_DESCRIPTOR => RegistryValueKind::FullResourceDescriptor,
        REG_RESOURCE_REQUIREMENTS_LIST => RegistryValueKind::ResourceRequirementsList,
        REG_QWORD => RegistryValueKind::Qword,
    }
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
        // EULA / third-party license *text* must not be flagged as the owner's
        // license (regression: 7za_license.txt false positive on Solar Fire).
        assert!(license_file_match(Path::new(r"C:\App\7za_license.txt")).is_none());
        assert!(license_file_match(Path::new(r"C:\App\license.txt")).is_none());
    }

    #[test]
    fn recognizes_license_env_vars() {
        assert!(is_license_env_var("LM_LICENSE_FILE"));
        assert!(is_license_env_var("ACME_LICENSE_FILE"));
        assert!(!is_license_env_var("PATH"));
        // Build-tool variables that merely end with the suffix are not licenses.
        assert!(!is_license_env_var("CARGO_PKG_LICENSE_FILE"));
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
    fn detects_dpapi_blob() {
        let mut blob = vec![0x01, 0x00, 0x00, 0x00];
        blob.extend_from_slice(&DPAPI_PROVIDER_GUID);
        blob.extend_from_slice(&[0u8; 8]);
        assert!(is_dpapi_blob(&blob));
        // A plain serial/token is not flagged as DPAPI.
        assert!(!is_dpapi_blob(b"ACME-1234-5678-PLAINKEY"));
        assert!(!is_dpapi_blob(&[0u8; 10]));
    }

    #[test]
    fn recognizes_license_registry_names() {
        assert!(is_license_registry_name("LicenseKey"));
        assert!(is_license_registry_name("Serial Number"));
        assert!(is_license_registry_name("ProductKey"));
        assert!(!is_license_registry_name("InstallPath"));
    }
}
