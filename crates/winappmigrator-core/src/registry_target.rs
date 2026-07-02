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
