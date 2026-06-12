use std::fs;
use std::path::{Path, PathBuf};

use tracing::warn;
use winreg::RegKey;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY,
};

use crate::errors::{Result, WinAppMigratorError};
use crate::models::{AppArchitecture, InstallScope, InstalledApp, RegistryHive};
use crate::path_utils::{
    looks_like_executable, normalize_name_for_match, split_command_executable_path,
};

const HKLM_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const HKLM_WOW6432_UNINSTALL: &str =
    r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";
const HKCU_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const DISPLAY_NAME_VALUE: &str = "DisplayName";
const PUBLISHER_VALUE: &str = "Publisher";
const DISPLAY_VERSION_VALUE: &str = "DisplayVersion";
const INSTALL_LOCATION_VALUE: &str = "InstallLocation";
const UNINSTALL_STRING_VALUE: &str = "UninstallString";
const DISPLAY_ICON_VALUE: &str = "DisplayIcon";

#[derive(Debug, Clone)]
struct RegistryLocation {
    hive: RegistryHive,
    hive_label: &'static str,
    subkey: &'static str,
    architecture: AppArchitecture,
    install_scope: InstallScope,
    flags: u32,
}

#[derive(Debug, Clone, Default)]
pub struct RawUninstallEntry {
    pub display_name: Option<String>,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub install_location: Option<String>,
    pub uninstall_string: Option<String>,
    pub display_icon: Option<String>,
}

pub fn list_installed_apps() -> Result<Vec<InstalledApp>> {
    let locations = [
        RegistryLocation {
            hive: RegistryHive::LocalMachine,
            hive_label: "HKLM",
            subkey: HKLM_UNINSTALL,
            architecture: AppArchitecture::X64,
            install_scope: InstallScope::PerMachine,
            flags: KEY_READ | KEY_WOW64_64KEY,
        },
        RegistryLocation {
            hive: RegistryHive::LocalMachine,
            hive_label: "HKLM",
            subkey: HKLM_WOW6432_UNINSTALL,
            architecture: AppArchitecture::X86,
            install_scope: InstallScope::PerMachine,
            flags: KEY_READ | KEY_WOW64_32KEY,
        },
        RegistryLocation {
            hive: RegistryHive::CurrentUser,
            hive_label: "HKCU",
            subkey: HKCU_UNINSTALL,
            architecture: AppArchitecture::Unknown,
            install_scope: InstallScope::PerUser,
            flags: KEY_READ,
        },
    ];

    let mut apps = Vec::new();
    for location in locations {
        apps.extend(read_uninstall_location(&location)?);
    }

    apps.sort_by(|left, right| {
        left.display_name
            .to_ascii_lowercase()
            .cmp(&right.display_name.to_ascii_lowercase())
    });
    Ok(apps)
}

fn read_uninstall_location(location: &RegistryLocation) -> Result<Vec<InstalledApp>> {
    let root = match location.hive {
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
    };

    let uninstall_key = match root.open_subkey_with_flags(location.subkey, location.flags) {
        Ok(key) => key,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(WinAppMigratorError::Registry(format!(
                "{}\\{}: {}",
                location.hive_label, location.subkey, error
            )));
        }
    };

    let mut apps = Vec::new();
    for key_name in uninstall_key.enum_keys() {
        let key_name = match key_name {
            Ok(name) => name,
            Err(error) => {
                warn!(%error, "failed to enumerate uninstall subkey name");
                continue;
            }
        };
        let child = match uninstall_key.open_subkey_with_flags(&key_name, location.flags) {
            Ok(child) => child,
            Err(error) => {
                warn!(%error, key_name, "failed to open uninstall subkey");
                continue;
            }
        };
        let raw = read_raw_entry(&child);
        let source_path = format!(r"{}\{}\{}", location.hive_label, location.subkey, key_name);
        if let Some(app) = parse_uninstall_entry(
            &raw,
            source_path,
            location.architecture.clone(),
            location.install_scope.clone(),
        ) {
            apps.push(app);
        }
    }

    Ok(apps)
}

fn read_raw_entry(key: &RegKey) -> RawUninstallEntry {
    RawUninstallEntry {
        display_name: read_optional_string(key, DISPLAY_NAME_VALUE),
        publisher: read_optional_string(key, PUBLISHER_VALUE),
        version: read_optional_string(key, DISPLAY_VERSION_VALUE),
        install_location: read_optional_string(key, INSTALL_LOCATION_VALUE),
        uninstall_string: read_optional_string(key, UNINSTALL_STRING_VALUE),
        display_icon: read_optional_string(key, DISPLAY_ICON_VALUE),
    }
}

fn read_optional_string(key: &RegKey, value_name: &str) -> Option<String> {
    key.get_value::<String, _>(value_name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn parse_uninstall_entry(
    raw: &RawUninstallEntry,
    registry_source_path: String,
    architecture: AppArchitecture,
    install_scope: InstallScope,
) -> Option<InstalledApp> {
    let display_name = raw.display_name.as_ref()?.trim().to_owned();
    if display_name.is_empty() {
        return None;
    }

    let install_location = raw.install_location.as_ref().map(PathBuf::from);
    let main_executable =
        guess_main_executable(raw, install_location.as_deref(), display_name.as_str());

    Some(InstalledApp {
        display_name,
        publisher: raw.publisher.clone(),
        version: raw.version.clone(),
        install_location,
        uninstall_string: raw.uninstall_string.clone(),
        main_executable,
        registry_source_path,
        architecture,
        install_scope,
    })
}

fn guess_main_executable(
    raw: &RawUninstallEntry,
    install_location: Option<&Path>,
    display_name: &str,
) -> Option<PathBuf> {
    for value in [&raw.display_icon, &raw.uninstall_string]
        .into_iter()
        .flatten()
    {
        if let Some(path) = split_command_executable_path(value)
            && looks_like_executable(&path)
            && path.exists()
        {
            return Some(path);
        }
    }

    install_location.and_then(|path| guess_executable_from_install_dir(path, display_name))
}

fn guess_executable_from_install_dir(
    install_location: &Path,
    display_name: &str,
) -> Option<PathBuf> {
    let entries = fs::read_dir(install_location).ok()?;
    let normalized_display = normalize_name_for_match(display_name);
    let mut executable_paths = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if !looks_like_executable(&path) {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .map(normalize_name_for_match)
            .unwrap_or_default();
        if !stem.is_empty()
            && !normalized_display.is_empty()
            && (normalized_display.contains(&stem) || stem.contains(&normalized_display))
        {
            return Some(path);
        }
        executable_paths.push(path);
    }

    if executable_paths.len() == 1 {
        executable_paths.into_iter().next()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_registry_uninstall_entry() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let raw = RawUninstallEntry {
            display_name: Some("Example App".to_owned()),
            publisher: Some("Example Co".to_owned()),
            version: Some("1.2.3".to_owned()),
            install_location: Some(r"C:\Example".to_owned()),
            uninstall_string: Some(r#""C:\Example\uninstall.exe" /remove"#.to_owned()),
            display_icon: None,
        };

        let app = parse_uninstall_entry(
            &raw,
            r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\Example".to_owned(),
            AppArchitecture::X64,
            InstallScope::PerMachine,
        )
        .ok_or_else(|| std::io::Error::other("app should parse"))?;

        assert_eq!(app.display_name, "Example App");
        assert_eq!(app.publisher.as_deref(), Some("Example Co"));
        assert_eq!(app.version.as_deref(), Some("1.2.3"));
        assert_eq!(app.architecture, AppArchitecture::X64);
        assert_eq!(app.install_scope, InstallScope::PerMachine);
        Ok(())
    }

    #[test]
    fn skips_entries_without_display_name() {
        let raw = RawUninstallEntry::default();

        let app = parse_uninstall_entry(
            &raw,
            "HKCU\\Example".to_owned(),
            AppArchitecture::Unknown,
            InstallScope::PerUser,
        );

        assert!(app.is_none());
    }
}
