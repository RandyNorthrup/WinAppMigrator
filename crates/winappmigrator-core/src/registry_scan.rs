use std::path::{Path, PathBuf};

use tracing::warn;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN,
    REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD,
    REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};
use winreg::{RegKey, RegValue};

use crate::models::{
    MatchConfidence, RegistryEntry, RegistryHive, RegistryValueKind, Warning, WarningSeverity,
};
use crate::path_utils::{
    looks_like_executable, normalize_name_for_match, split_command_executable_path,
};

const APP_PATHS: &str = r"Software\Microsoft\Windows\CurrentVersion\App Paths";
const HKLM_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const HKLM_WOW6432_UNINSTALL: &str =
    r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";
const HKCU_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const HKCU_SOFTWARE: &str = r"Software";
const HKLM_SOFTWARE: &str = r"Software";
const HKCU_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKCU_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKLM_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_WOW6432_CLASSES_CLSID: &str = r"Software\WOW6432Node\Classes\CLSID";
const MAX_REGISTRY_CAPTURE_DEPTH: usize = 8;
const MAX_REGISTRY_CAPTURE_KEYS: usize = 512;
const MAX_REGISTRY_VALUE_BYTES: usize = 1024 * 1024;
const MAX_REGISTRY_MATCH_SCAN_DEPTH: usize = 4;
const RUN_KEYS: &[&str] = &[
    r"Software\Microsoft\Windows\CurrentVersion\Run",
    r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
];

#[derive(Debug, Clone)]
pub struct RegistryScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    pub display_name: Option<String>,
    pub publisher: Option<String>,
    pub source_registry_path: Option<String>,
}

pub fn scan_registry(context: &RegistryScanContext) -> (Vec<RegistryEntry>, Vec<Warning>) {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();

    scan_app_paths(
        context,
        RegistryHive::LocalMachine,
        &mut entries,
        &mut warnings,
    );
    scan_app_paths(
        context,
        RegistryHive::CurrentUser,
        &mut entries,
        &mut warnings,
    );
    scan_uninstall_entries(
        context,
        RegistryHive::LocalMachine,
        HKLM_UNINSTALL,
        &mut entries,
        &mut warnings,
    );
    scan_uninstall_entries(
        context,
        RegistryHive::LocalMachine,
        HKLM_WOW6432_UNINSTALL,
        &mut entries,
        &mut warnings,
    );
    scan_uninstall_entries(
        context,
        RegistryHive::CurrentUser,
        HKCU_UNINSTALL,
        &mut entries,
        &mut warnings,
    );
    scan_run_keys(
        context,
        RegistryHive::LocalMachine,
        &mut entries,
        &mut warnings,
    );
    scan_run_keys(
        context,
        RegistryHive::CurrentUser,
        &mut entries,
        &mut warnings,
    );
    scan_software_children(
        context,
        RegistryHive::CurrentUser,
        &mut entries,
        &mut warnings,
    );
    scan_software_children(
        context,
        RegistryHive::LocalMachine,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        RegistryHive::CurrentUser,
        HKCU_CLASSES_APPLICATIONS,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        RegistryHive::CurrentUser,
        HKCU_CLASSES_CLSID,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        RegistryHive::LocalMachine,
        HKLM_CLASSES_APPLICATIONS,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        RegistryHive::LocalMachine,
        HKLM_CLASSES_CLSID,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        RegistryHive::LocalMachine,
        HKLM_WOW6432_CLASSES_CLSID,
        &mut entries,
        &mut warnings,
    );

    entries.sort_by(|left, right| {
        left.key_path
            .to_ascii_lowercase()
            .cmp(&right.key_path.to_ascii_lowercase())
            .then_with(|| left.value_name.cmp(&right.value_name))
    });
    entries.dedup_by(|left, right| {
        left.hive == right.hive
            && left.key_path == right.key_path
            && left.value_name == right.value_name
    });

    (entries, warnings)
}

struct RegistryCaptureContext<'a> {
    root: &'a RegKey,
    hive: RegistryHive,
    confidence: MatchConfidence,
    entries: &'a mut Vec<RegistryEntry>,
    warnings: &'a mut Vec<Warning>,
    keys_seen: usize,
}

fn scan_class_registrations(
    context: &RegistryScanContext,
    hive: RegistryHive,
    base_path: &str,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Some(root) = open_root(hive.clone()) else {
        return;
    };
    let Ok(base_key) = root.open_subkey_with_flags(base_path, KEY_READ) else {
        return;
    };
    for child_name in base_key.enum_keys().flatten() {
        let key_path = format!(r"{base_path}\{child_name}");
        if registry_tree_matches_context(&root, &key_path, context, 0) {
            capture_registry_subtree(
                &root,
                hive.clone(),
                &key_path,
                MatchConfidence::High,
                entries,
                warnings,
            );
        }
    }
}

fn scan_uninstall_entries(
    context: &RegistryScanContext,
    hive: RegistryHive,
    uninstall_path: &str,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Some(root) = open_root(hive.clone()) else {
        return;
    };
    let key = match root.open_subkey_with_flags(uninstall_path, KEY_READ) {
        Ok(key) => key,
        Err(_) => return,
    };

    for child_name in key.enum_keys().flatten() {
        let key_path = format!(r"{uninstall_path}\{child_name}");
        if source_registry_path_matches(context, &hive, &key_path) {
            capture_registry_subtree(
                &root,
                hive.clone(),
                &key_path,
                MatchConfidence::High,
                entries,
                warnings,
            );
            continue;
        }

        let Ok(child) = root.open_subkey_with_flags(&key_path, KEY_READ) else {
            continue;
        };
        if uninstall_key_matches_context(context, &child) {
            capture_registry_subtree(
                &root,
                hive.clone(),
                &key_path,
                MatchConfidence::High,
                entries,
                warnings,
            );
        }
    }
}

fn scan_app_paths(
    context: &RegistryScanContext,
    hive: RegistryHive,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Some(root) = open_root(hive.clone()) else {
        return;
    };
    let key = match root.open_subkey_with_flags(APP_PATHS, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            warn!(%error, "App Paths registry key unavailable");
            return;
        }
    };

    for child_name in key.enum_keys().flatten() {
        let child = match key.open_subkey_with_flags(&child_name, KEY_READ) {
            Ok(child) => child,
            Err(error) => {
                warnings.push(Warning::new(
                    "registry_key_unreadable",
                    WarningSeverity::Info,
                    format!("Could not read App Paths child key: {error}"),
                    Some(child_name),
                ));
                continue;
            }
        };
        let mut matched = false;
        for value_name in ["", "Path"] {
            if let Ok(value) = child.get_raw_value(value_name) {
                let value_display = raw_value_to_display(&value);
                if value_matches_context(context, &value_display) {
                    matched = true;
                }
            }
        }
        if matched {
            capture_registry_subtree(
                &root,
                hive.clone(),
                &format!(r"{APP_PATHS}\{child_name}"),
                MatchConfidence::High,
                entries,
                warnings,
            );
        }
    }
}

fn scan_run_keys(
    context: &RegistryScanContext,
    hive: RegistryHive,
    entries: &mut Vec<RegistryEntry>,
    _warnings: &mut Vec<Warning>,
) {
    let Some(root) = open_root(hive.clone()) else {
        return;
    };
    for key_path in RUN_KEYS {
        let Ok(key) = root.open_subkey_with_flags(key_path, KEY_READ) else {
            continue;
        };
        for value in key.enum_values().flatten() {
            let value_name = value.0;
            let raw_value = value.1;
            let value_data = raw_value_to_display(&raw_value);
            if value_matches_context(context, &value_data) {
                entries.push(entry_from_raw_value(
                    hive.clone(),
                    (*key_path).to_owned(),
                    value_name,
                    raw_value,
                    MatchConfidence::High,
                    hive == RegistryHive::LocalMachine,
                ));
            }
        }
    }
}

fn scan_software_children(
    context: &RegistryScanContext,
    hive: RegistryHive,
    entries: &mut Vec<RegistryEntry>,
    _warnings: &mut Vec<Warning>,
) {
    let Some(root) = open_root(hive.clone()) else {
        return;
    };
    let software_path = match hive {
        RegistryHive::LocalMachine => HKLM_SOFTWARE,
        RegistryHive::CurrentUser => HKCU_SOFTWARE,
    };
    let Ok(software) = root.open_subkey_with_flags(software_path, KEY_READ) else {
        return;
    };
    let tokens = related_tokens(context);
    if tokens.is_empty() {
        return;
    }

    for child_name in software.enum_keys().flatten() {
        let normalized = normalize_name_for_match(&child_name);
        if tokens.iter().any(|token| normalized.contains(token)) {
            capture_registry_subtree(
                &root,
                hive.clone(),
                &format!(r"{software_path}\{child_name}"),
                MatchConfidence::Medium,
                entries,
                _warnings,
            );
        }
    }
}

fn capture_registry_subtree(
    root: &RegKey,
    hive: RegistryHive,
    key_path: &str,
    confidence: MatchConfidence,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let mut context = RegistryCaptureContext {
        root,
        hive,
        confidence,
        entries,
        warnings,
        keys_seen: 0,
    };
    capture_registry_subtree_inner(&mut context, key_path, 0);
}

fn capture_registry_subtree_inner(
    context: &mut RegistryCaptureContext<'_>,
    key_path: &str,
    depth: usize,
) {
    if depth > MAX_REGISTRY_CAPTURE_DEPTH || context.keys_seen >= MAX_REGISTRY_CAPTURE_KEYS {
        context.warnings.push(Warning::new(
            "registry_capture_truncated",
            WarningSeverity::Warning,
            "Registry subtree capture reached the configured safety limit.",
            Some(key_path.to_owned()),
        ));
        return;
    }
    context.keys_seen += 1;

    let key = match context.root.open_subkey_with_flags(key_path, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            context.warnings.push(Warning::new(
                "registry_key_unreadable",
                WarningSeverity::Warning,
                format!("Could not read registry key: {error}"),
                Some(key_path.to_owned()),
            ));
            return;
        }
    };

    let mut value_count = 0_usize;
    for value in key.enum_values().flatten() {
        value_count += 1;
        let value_name = value.0;
        let raw_value = value.1;
        if raw_value.bytes.len() > MAX_REGISTRY_VALUE_BYTES {
            context.warnings.push(Warning::new(
                "registry_value_too_large",
                WarningSeverity::Warning,
                "Registry value exceeded the configured capture size limit.",
                Some(format!("{key_path}\\{value_name}")),
            ));
            continue;
        }
        context.entries.push(entry_from_raw_value(
            context.hive.clone(),
            key_path.to_owned(),
            value_name,
            raw_value,
            context.confidence.clone(),
            context.hive == RegistryHive::LocalMachine,
        ));
    }
    if value_count == 0 {
        context.entries.push(RegistryEntry {
            hive: context.hive.clone(),
            key_path: key_path.to_owned(),
            value_name: None,
            value_data: None,
            value_kind: Some(RegistryValueKind::None),
            value_bytes_hex: None,
            confidence: context.confidence.clone(),
            requires_admin: context.hive == RegistryHive::LocalMachine,
        });
    }

    for child_name in key.enum_keys().flatten() {
        capture_registry_subtree_inner(context, &format!(r"{key_path}\{child_name}"), depth + 1);
    }
}

fn entry_from_raw_value(
    hive: RegistryHive,
    key_path: String,
    value_name: String,
    raw_value: RegValue<'_>,
    confidence: MatchConfidence,
    requires_admin: bool,
) -> RegistryEntry {
    RegistryEntry {
        hive,
        key_path,
        value_name: Some(display_value_name(&value_name)),
        value_data: Some(raw_value_to_display(&raw_value)),
        value_kind: Some(registry_value_kind(&raw_value.vtype)),
        value_bytes_hex: Some(hex_encode(raw_value.bytes.as_ref())),
        confidence,
        requires_admin,
    }
}

fn display_value_name(value_name: &str) -> String {
    if value_name.is_empty() {
        "(Default)".to_owned()
    } else {
        value_name.to_owned()
    }
}

fn raw_value_to_display(value: &RegValue<'_>) -> String {
    value.to_string()
}

fn registry_value_kind(value_type: &RegType) -> RegistryValueKind {
    if value_type == &REG_NONE {
        RegistryValueKind::None
    } else if value_type == &REG_SZ {
        RegistryValueKind::String
    } else if value_type == &REG_EXPAND_SZ {
        RegistryValueKind::ExpandString
    } else if value_type == &REG_BINARY {
        RegistryValueKind::Binary
    } else if value_type == &REG_DWORD {
        RegistryValueKind::Dword
    } else if value_type == &REG_DWORD_BIG_ENDIAN {
        RegistryValueKind::DwordBigEndian
    } else if value_type == &REG_LINK {
        RegistryValueKind::Link
    } else if value_type == &REG_MULTI_SZ {
        RegistryValueKind::MultiString
    } else if value_type == &REG_RESOURCE_LIST {
        RegistryValueKind::ResourceList
    } else if value_type == &REG_FULL_RESOURCE_DESCRIPTOR {
        RegistryValueKind::FullResourceDescriptor
    } else if value_type == &REG_RESOURCE_REQUIREMENTS_LIST {
        RegistryValueKind::ResourceRequirementsList
    } else if value_type == &REG_QWORD {
        RegistryValueKind::Qword
    } else {
        RegistryValueKind::Binary
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push_str(&format!("{byte:02x}"));
    }
    encoded
}

fn registry_tree_matches_context(
    root: &RegKey,
    key_path: &str,
    context: &RegistryScanContext,
    depth: usize,
) -> bool {
    if depth > MAX_REGISTRY_MATCH_SCAN_DEPTH {
        return false;
    }
    let Ok(key) = root.open_subkey_with_flags(key_path, KEY_READ) else {
        return false;
    };
    for value in key.enum_values().flatten() {
        if value_matches_context(context, &raw_value_to_display(&value.1)) {
            return true;
        }
    }
    for child_name in key.enum_keys().flatten() {
        if registry_tree_matches_context(
            root,
            &format!(r"{key_path}\{child_name}"),
            context,
            depth + 1,
        ) {
            return true;
        }
    }
    false
}

fn source_registry_path_matches(
    context: &RegistryScanContext,
    hive: &RegistryHive,
    key_path: &str,
) -> bool {
    let Some(source) = context.source_registry_path.as_deref() else {
        return false;
    };
    let expected_prefix = match hive {
        RegistryHive::LocalMachine => "HKLM\\",
        RegistryHive::CurrentUser => "HKCU\\",
    };
    source
        .strip_prefix(expected_prefix)
        .is_some_and(|source_path| source_path.eq_ignore_ascii_case(key_path))
}

fn uninstall_key_matches_context(context: &RegistryScanContext, key: &RegKey) -> bool {
    for value_name in [
        "DisplayName",
        "Publisher",
        "DisplayVersion",
        "InstallLocation",
        "UninstallString",
        "DisplayIcon",
    ] {
        if let Ok(value) = key.get_raw_value(value_name)
            && value_matches_context(context, &raw_value_to_display(&value))
        {
            return true;
        }
    }
    false
}

fn value_matches_context(context: &RegistryScanContext, value: &str) -> bool {
    let value_lower = value.to_ascii_lowercase();
    if path_text_matches(&context.executable_path, &value_lower) {
        return true;
    }
    if context
        .install_location
        .as_ref()
        .is_some_and(|path| path_text_matches(path, &value_lower))
    {
        return true;
    }
    split_command_executable_path(value).is_some_and(|path| command_path_matches(context, &path))
}

fn path_text_matches(path: &Path, lower_text: &str) -> bool {
    lower_text.contains(&path.to_string_lossy().to_ascii_lowercase())
}

fn command_path_matches(context: &RegistryScanContext, candidate: &Path) -> bool {
    if path_eq(candidate, &context.executable_path) {
        return true;
    }
    if candidate.is_absolute()
        && context
            .install_location
            .as_ref()
            .is_some_and(|root| path_starts_with_ignore_case(candidate, root))
    {
        return true;
    }
    if !candidate.is_absolute() && looks_like_executable(candidate) {
        let candidate_name = candidate
            .file_name()
            .map(|value| value.to_string_lossy().to_ascii_lowercase());
        let executable_name = context
            .executable_path
            .file_name()
            .map(|value| value.to_string_lossy().to_ascii_lowercase());
        return candidate_name.is_some() && candidate_name == executable_name;
    }
    false
}

fn path_eq(left: &Path, right: &Path) -> bool {
    left.to_string_lossy()
        .eq_ignore_ascii_case(&right.to_string_lossy())
}

fn path_starts_with_ignore_case(path: &Path, root: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .starts_with(&root.to_string_lossy().to_ascii_lowercase())
}

fn related_tokens(context: &RegistryScanContext) -> Vec<String> {
    [
        context.display_name.as_deref(),
        context.publisher.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(normalize_name_for_match)
    .filter(|token| token.len() >= 3)
    .collect()
}

fn open_root(hive: RegistryHive) -> Option<RegKey> {
    Some(match hive {
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_value_match_does_not_match_short_tokens_in_username() {
        let context = RegistryScanContext {
            executable_path: PathBuf::from(
                r"C:\Users\admin\AppData\Local\WinAppMigratorE2E\SumatraPDF\SumatraPDF.exe",
            ),
            install_location: Some(PathBuf::from(
                r"C:\Users\admin\AppData\Local\WinAppMigratorE2E\SumatraPDF",
            )),
            display_name: Some("SumatraPDF".to_owned()),
            publisher: None,
            source_registry_path: None,
        };

        assert!(!value_matches_context(&context, "ad 9.6.8"));
    }
}
