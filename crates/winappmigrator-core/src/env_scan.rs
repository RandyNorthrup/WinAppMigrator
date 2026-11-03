use std::path::PathBuf;

use winreg::enums::{KEY_READ, REG_EXPAND_SZ, REG_MULTI_SZ};
use winreg::{RegKey, RegValue};

use crate::models::{
    EnvironmentScope, EnvironmentVariableEntry, MatchConfidence, RegistryValueKind, Warning,
    WarningSeverity,
};
use crate::path_utils::text_mentions_path;
use crate::registry_scan::registry_value_text;
use crate::registry_target::is_path_variable;
use crate::system_context::SystemContext;

const HKCU_ENVIRONMENT: &str = r"Environment";
const HKLM_ENVIRONMENT: &str = r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";

/// The app's paths are as its own PC calls them, since that is what the variables hold.
#[derive(Debug, Clone)]
pub struct EnvironmentScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    /// Other folders that are the app's, because a person added them.
    pub also_the_apps: Vec<PathBuf>,
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

    // Per-user environment: only ever the account this scan is about.
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
        let value_text = registry_value_text(&raw);
        if value_matches_context(context, &value_text) {
            let value = if is_path_variable(&name) {
                only_the_apps_entries(context, &value_text)
            } else {
                value_text
            };
            entries.push(EnvironmentVariableEntry {
                scope: scope.clone(),
                name,
                value,
                value_kind: environment_value_kind(&raw),
                requires_admin: scope == EnvironmentScope::Machine,
                confidence: MatchConfidence::High,
            });
        }
    }
}

/// Of a `Path`, what is the app's. The rest is what else is on this PC, and in a user's `Path` it is
/// where in that account's folders, which a package has no reason to carry.
fn only_the_apps_entries(context: &EnvironmentScanContext, value: &str) -> String {
    value
        .split(';')
        .filter(|entry| value_matches_context(context, entry))
        .collect::<Vec<_>>()
        .join(";")
}

fn value_matches_context(context: &EnvironmentScanContext, value: &str) -> bool {
    text_mentions_path(value, &context.executable_path, true)
        || context
            .install_location
            .iter()
            .chain(&context.also_the_apps)
            .any(|path| text_mentions_path(value, path, false))
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
            also_the_apps: Vec::new(),
        };

        assert!(value_matches_context(&context, r"C:\Tools;C:\App\bin"));
        assert!(!value_matches_context(&context, r"C:\Tools;C:\Other"));
    }

    #[test]
    fn of_a_path_only_what_is_the_apps_goes() {
        let context = EnvironmentScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\App\App.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\App")),
            also_the_apps: vec![PathBuf::from(r"D:\AppData")],
        };

        assert_eq!(
            only_the_apps_entries(
                &context,
                r"C:\Windows;C:\Users\someone\Tools;C:\Program Files\App\bin;;C:\Program Files\Apple;D:\AppData"
            ),
            r"C:\Program Files\App\bin;D:\AppData"
        );
        assert_eq!(
            only_the_apps_entries(&context, r"C:\Program Files\App"),
            r"C:\Program Files\App"
        );
        assert_eq!(only_the_apps_entries(&context, r"C:\Windows"), "");
    }
}
