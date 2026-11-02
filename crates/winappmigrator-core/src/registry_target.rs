//! Shared mappings between the migrator's serializable registry model and the
//! raw `winreg` primitives used to write and verify it.
//!
//! The restore (write) and verify (read-back) paths must agree exactly on which
//! live hive, key path, and value type a captured entry corresponds to — a drift
//! between them is a silent correctness bug (a value written to one location and
//! checked in another). Keeping these mappings in one place makes that agreement
//! structural rather than a copy-paste convention.
//!
//! Note: these resolve to the **live** machine's registry. They are used by
//! restore/verify, which always operate on the target PC; offline (foreign-disk)
//! registry access goes through [`crate::system_context::SystemContext`] instead.

use std::path::Path;

use winreg::RegKey;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN,
    REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD,
    REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};

use crate::models::{EnvironmentScope, RegistryHive, RegistryValueKind};

/// The live `winreg` predefined root for a captured hive.
pub(crate) fn registry_root(hive: &RegistryHive) -> RegKey {
    match hive {
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
    }
}

/// The root and subkey path holding environment variables for a scope. The
/// machine path (`SYSTEM\CurrentControlSet\...\Session Manager\Environment`) is a
/// load-bearing magic string that must be identical on the write and verify
/// sides — hence its single definition here.
pub(crate) fn environment_registry_target(scope: &EnvironmentScope) -> (RegKey, &'static str) {
    match scope {
        EnvironmentScope::User => (RegKey::predef(HKEY_CURRENT_USER), "Environment"),
        EnvironmentScope::Machine => (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
        ),
    }
}

/// The `ImagePath` of a service on this PC. `Ok(None)` when no such service exists. A
/// service with no `ImagePath` reads as an empty one, which matches nothing.
pub(crate) fn service_image_path(service_name: &str) -> std::io::Result<Option<String>> {
    let key = match RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{service_name}"))
    {
        Ok(key) => key,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(Some(
        key.get_value::<String, _>("ImagePath").unwrap_or_default(),
    ))
}

/// The account a service on this PC is set to run as, if it records one.
pub(crate) fn service_object_name(service_name: &str) -> Option<String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{service_name}"))
        .and_then(|key| key.get_value::<String, _>("ObjectName"))
        .ok()
}

/// `Path` and `PSModulePath` are shared with everything else on the machine, so they are
/// merged entry by entry instead of being replaced with the source machine's value.
pub(crate) fn is_path_variable(name: &str) -> bool {
    name.eq_ignore_ascii_case("Path") || name.eq_ignore_ascii_case("PSModulePath")
}

/// The entries of a captured `Path`, already rewritten for the target, that belong
/// to the application being restored.
pub(crate) fn app_path_entries(rewritten: &str, target_root: &Path) -> Vec<String> {
    let root = normalized_path_entry(&target_root.to_string_lossy());
    let prefix = format!("{root}\\");
    rewritten
        .split(';')
        .map(str::trim)
        .filter(|entry| {
            let entry = normalized_path_entry(entry);
            !entry.is_empty() && (entry == root || entry.starts_with(&prefix))
        })
        .map(ToOwned::to_owned)
        .collect()
}

/// Append any of `app_entries` missing from `existing`, leaving the rest as it is.
pub(crate) fn merge_path_entries(existing: &str, app_entries: &[String]) -> String {
    let mut merged = existing.trim_end_matches(';').to_owned();
    for entry in app_entries {
        if !path_contains_entry(&merged, entry) {
            if !merged.is_empty() {
                merged.push(';');
            }
            merged.push_str(entry);
        }
    }
    merged
}

pub(crate) fn path_contains_entry(value: &str, entry: &str) -> bool {
    let entry = normalized_path_entry(entry);
    value
        .split(';')
        .any(|existing| normalized_path_entry(existing) == entry)
}

fn normalized_path_entry(entry: &str) -> String {
    entry.trim().trim_end_matches('\\').to_ascii_lowercase()
}

/// Short hive label (`HKLM`/`HKCU`) used in log and warning messages. (Distinct
/// from the full `HKEY_LOCAL_MACHINE` names a `.reg` export header requires.)
pub(crate) fn registry_hive_name(hive: &RegistryHive) -> &'static str {
    match hive {
        RegistryHive::LocalMachine => "HKLM",
        RegistryHive::CurrentUser => "HKCU",
    }
}

/// Normalize an optional value name to the empty string `winreg` uses to address
/// a key's default value.
pub(crate) fn registry_value_name(value_name: Option<&str>) -> &str {
    match value_name {
        None | Some("(Default)") => "",
        Some(value) => value,
    }
}

/// Map the serializable value kind back to the raw `winreg` value type.
pub(crate) fn registry_raw_type(kind: &RegistryValueKind) -> RegType {
    match kind {
        RegistryValueKind::None => REG_NONE,
        RegistryValueKind::String => REG_SZ,
        RegistryValueKind::ExpandString => REG_EXPAND_SZ,
        RegistryValueKind::Binary => REG_BINARY,
        RegistryValueKind::Dword => REG_DWORD,
        RegistryValueKind::DwordBigEndian => REG_DWORD_BIG_ENDIAN,
        RegistryValueKind::Link => REG_LINK,
        RegistryValueKind::MultiString => REG_MULTI_SZ,
        RegistryValueKind::ResourceList => REG_RESOURCE_LIST,
        RegistryValueKind::FullResourceDescriptor => REG_FULL_RESOURCE_DESCRIPTOR,
        RegistryValueKind::ResourceRequirementsList => REG_RESOURCE_REQUIREMENTS_LIST,
        RegistryValueKind::Qword => REG_QWORD,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_lists_are_merged_and_other_variables_replaced() {
        assert!(is_path_variable("PATH"));
        assert!(is_path_variable("PSModulePath"));
        assert!(!is_path_variable("ACME_HOME"));
    }

    #[test]
    fn only_entries_under_the_target_root_belong_to_the_app() {
        let entries = app_path_entries(
            r"C:\Windows;C:\Program Files\App\bin;C:\Program Files\App;C:\Program Files\Apple;;",
            Path::new(r"C:\Program Files\App"),
        );

        assert_eq!(
            entries,
            vec![r"C:\Program Files\App\bin", r"C:\Program Files\App"]
        );
    }

    #[test]
    fn merging_keeps_the_existing_value_and_adds_only_missing_entries() {
        let merged = merge_path_entries(
            r"%SystemRoot%;C:\Tools\;",
            &[r"C:\App".to_owned(), r"c:\tools".to_owned()],
        );

        assert_eq!(merged, r"%SystemRoot%;C:\Tools\;C:\App");
    }

    #[test]
    fn merging_into_an_empty_value_has_no_leading_separator() {
        assert_eq!(merge_path_entries("", &[r"C:\App".to_owned()]), r"C:\App");
    }
}
