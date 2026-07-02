use std::path::{Path, PathBuf};

use winreg::enums::{KEY_READ, REG_EXPAND_SZ, REG_MULTI_SZ};
use winreg::{RegKey, RegValue};

use crate::models::{
    EnvironmentScope, EnvironmentVariableEntry, MatchConfidence, RegistryValueKind, Warning,
    WarningSeverity,
};
use crate::system_context::SystemContext;

const HKCU_ENVIRONMENT: &str = r"Environment";
const HKLM_ENVIRONMENT: &str = r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";

#[derive(Debug, Clone)]
pub struct EnvironmentScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

pub fn scan_environment(
    context: &EnvironmentScanContext,
    system: &SystemContext,
) -> (Vec<EnvironmentVariableEntry>, Vec<Warning>) {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();

    match system.open_hklm(HKLM_ENVIRONMENT, KEY_READ) {
        Ok(key) => collect_env_values(context, EnvironmentScope::Machine, &key, &mut entries),
        Err(error) => warnings.push(Warning::new(
            "environment_registry_unreadable",
            WarningSeverity::Info,
            format!("Could not read machine environment registry key: {error}"),
            Some(HKLM_ENVIRONMENT.to_owned()),
        )),
    }

    // Per-user environment: live yields the current user; offline every profile.
    for user in system.user_hives() {
        if let Ok(key) = user.open(HKCU_ENVIRONMENT, KEY_READ) {
            collect_env_values(context, EnvironmentScope::User, &key, &mut entries);
        }
    }

    entries.sort_by(|left, right| {
        format!("{:?}:{}", left.scope, left.name.to_ascii_lowercase()).cmp(&format!(
            "{:?}:{}",
            right.scope,
            right.name.to_ascii_lowercase()
        ))
    });
    entries.dedup_by(|left, right| left.scope == right.scope && left.name == right.name);
    (entries, warnings)
}

fn collect_env_values(
    context: &EnvironmentScanContext,
    scope: EnvironmentScope,
    key: &RegKey,
    entries: &mut Vec<EnvironmentVariableEntry>,
) {
    for value in key.enum_values().flatten() {
        let name = value.0;
        let raw = value.1;
        let value_text = raw.to_string();
        if value_matches_context(context, &value_text) {
            entries.push(EnvironmentVariableEntry {
                scope: scope.clone(),
                name,
                value: value_text,
                value_kind: environment_value_kind(&raw),
                requires_admin: scope == EnvironmentScope::Machine,
                confidence: MatchConfidence::High,
            });
        }
    }
}

fn value_matches_context(context: &EnvironmentScanContext, value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    path_text_matches(&context.executable_path, &lower)
        || context
            .install_location
            .as_ref()
            .is_some_and(|path| path_text_matches(path, &lower))
}

fn path_text_matches(path: &Path, lower_text: &str) -> bool {
    lower_text.contains(&path.to_string_lossy().to_ascii_lowercase())
}

fn environment_value_kind(value: &RegValue<'_>) -> RegistryValueKind {
    if value.vtype == REG_EXPAND_SZ {
        RegistryValueKind::ExpandString
    } else if value.vtype == REG_MULTI_SZ {
        RegistryValueKind::MultiString
    } else {
        RegistryValueKind::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_install_path_inside_environment_value() {
        let context = EnvironmentScanContext {
            executable_path: PathBuf::from(r"C:\App\App.exe"),
            install_location: Some(PathBuf::from(r"C:\App")),
        };

        assert!(value_matches_context(&context, r"C:\Tools;C:\App\bin"));
        assert!(!value_matches_context(&context, r"C:\Tools;C:\Other"));
    }
}
