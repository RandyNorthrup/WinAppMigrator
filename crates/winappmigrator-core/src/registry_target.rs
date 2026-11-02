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
    HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_READ, KEY_WRITE, REG_BINARY, REG_DWORD,
    REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ, REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ,
    REG_NONE, REG_QWORD, REG_RESOURCE_LIST, REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};

use crate::journal::ServiceConfig;
use crate::models::{EnvironmentScope, RegistryHive, RegistryValueKind};
use crate::path_utils::{SignedInUser, path_is_strictly_inside, signed_in_user};

const USER_ENVIRONMENT_KEY: &str = "Environment";
const MACHINE_ENVIRONMENT_KEY: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";

/// The live root for a captured hive. A per-user one is whoever is signed in, by their hive under
/// `HKEY_USERS`. `HKEY_CURRENT_USER` is that of the account running this, which is another
/// administrator's when one was named at the UAC prompt, and what was installed for the person at
/// the PC would never be seen by them. An error means there is no telling who that is, and nothing
/// is to be written for anyone.
pub(crate) fn registry_root(hive: &RegistryHive) -> std::io::Result<RegKey> {
    match hive {
        RegistryHive::LocalMachine => Ok(RegKey::predef(HKEY_LOCAL_MACHINE)),
        RegistryHive::CurrentUser => match signed_in_user() {
            SignedInUser::Desktop(account) | SignedInUser::NoDesktop(account) => {
                RegKey::predef(HKEY_USERS)
                    .open_subkey_with_flags(&account.sid, KEY_READ | KEY_WRITE)
            }
            SignedInUser::Unknown(reason) => Err(std::io::Error::other(format!(
                "could not tell who is signed in: {reason}"
            ))),
        },
    }
}

/// The root and subkey path holding environment variables for a scope. The
/// machine path (`SYSTEM\CurrentControlSet\...\Session Manager\Environment`) is a
/// load-bearing magic string that must be identical on the write and verify
/// sides — hence its single definition here.
pub(crate) fn environment_registry_target(
    scope: &EnvironmentScope,
) -> std::io::Result<(RegKey, &'static str)> {
    let (hive, key) = environment_location(scope);
    Ok((registry_root(&hive)?, key))
}

/// Where the environment variables for a scope are kept, as the hive and key that restoring
/// and undoing record.
pub(crate) fn environment_location(scope: &EnvironmentScope) -> (RegistryHive, &'static str) {
    match scope {
        EnvironmentScope::User => (RegistryHive::CurrentUser, USER_ENVIRONMENT_KEY),
        EnvironmentScope::Machine => (RegistryHive::LocalMachine, MACHINE_ENVIRONMENT_KEY),
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

/// How a service or driver on this PC is set up. `Ok(None)` when there is no such service.
pub(crate) fn read_service_config(service_name: &str) -> std::io::Result<Option<ServiceConfig>> {
    let key = match RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(format!(r"SYSTEM\CurrentControlSet\Services\{service_name}"))
    {
        Ok(key) => key,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(Some(ServiceConfig {
        image_path: key.get_value::<String, _>("ImagePath").unwrap_or_default(),
        start: key.get_value::<u32, _>("Start").ok(),
        delayed: key
            .get_value::<u32, _>("DelayedAutostart")
            .is_ok_and(|value| value != 0),
        object_name: key.get_value::<String, _>("ObjectName").ok(),
        display_name: key.get_value::<String, _>("DisplayName").ok(),
        description: key.get_value::<String, _>("Description").ok(),
        dependencies: key
            .get_value::<Vec<String>, _>("DependOnService")
            .unwrap_or_default(),
        service_type: key.get_value::<u32, _>("Type").ok(),
    }))
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
    rewritten
        .split(';')
        .map(str::trim)
        .filter(|entry| {
            !entry.is_empty()
                && (normalized_path_entry(entry) == root
                    || path_is_strictly_inside(Path::new(entry), target_root))
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

/// Take `added` back out of a `Path`, leaving everything else as it is, including whatever
/// was added since and how it was written.
pub(crate) fn remove_path_entries(current: &str, added: &[String]) -> String {
    let added: Vec<String> = added
        .iter()
        .map(|entry| normalized_path_entry(entry))
        .collect();
    current
        .split(';')
        .filter(|entry| !added.contains(&normalized_path_entry(entry)))
        .collect::<Vec<_>>()
        .join(";")
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

/// Decode a captured binary value.
pub(crate) fn hex_decode(value: &str) -> Result<Vec<u8>, &'static str> {
    if !value.len().is_multiple_of(2) {
        return Err("registry hex data has an odd length");
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks(2) {
        let text = std::str::from_utf8(pair).map_err(|_| "registry hex data is not UTF-8")?;
        let byte = u8::from_str_radix(text, 16).map_err(|_| "registry hex data is invalid")?;
        bytes.push(byte);
    }
    Ok(bytes)
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
    fn a_parent_step_does_not_make_a_path_entry_the_apps_own() {
        let entries = app_path_entries(
            r"C:\Program Files\App\..\..\Users\Public;C:\Program Files\App\bin\",
            Path::new(r"C:\Program Files\App"),
        );
        assert_eq!(entries, vec![r"C:\Program Files\App\bin\"]);
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
    fn undoing_a_path_takes_out_only_what_was_added() {
        let added = [r"C:\App\bin".to_owned(), r"C:\App\tools".to_owned()];
        assert_eq!(
            remove_path_entries(
                r"%SystemRoot%;C:\Tools\;c:\app\BIN\;D:\New;C:\App\tools;C:\App\binary",
                &added
            ),
            r"%SystemRoot%;C:\Tools\;D:\New;C:\App\binary"
        );
        assert_eq!(remove_path_entries(r"C:\App\bin", &added), "");
        assert_eq!(remove_path_entries(r"D:\Other;", &added), r"D:\Other;");
    }

    #[test]
    fn merging_into_an_empty_value_has_no_leading_separator() {
        assert_eq!(merge_path_entries("", &[r"C:\App".to_owned()]), r"C:\App");
    }
}
