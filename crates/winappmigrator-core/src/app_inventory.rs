use std::fs;
use std::path::{Path, PathBuf};

use tracing::warn;
use winreg::RegKey;
use winreg::enums::{KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::errors::{Result, WinAppMigratorError};
use crate::models::{AppArchitecture, InstallScope, InstalledApp};
use crate::path_utils::{
    looks_like_executable, normalize_name_for_match, split_command_executable_path,
};
use crate::system_context::{SystemContext, UserHive};

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
struct HklmUninstallLocation {
    subkey: &'static str,
    architecture: AppArchitecture,
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

/// List apps installed on the live machine.
pub fn list_installed_apps() -> Result<Vec<InstalledApp>> {
    list_installed_apps_from(&SystemContext::live())
}

/// List apps installed on the system `system` points at (live OS or a connected
/// offline volume). Paths in each entry are re-rooted onto the source system.
pub fn list_installed_apps_from(system: &SystemContext) -> Result<Vec<InstalledApp>> {
    let hklm_locations = [
        HklmUninstallLocation {
            subkey: HKLM_UNINSTALL,
            architecture: AppArchitecture::X64,
            flags: KEY_READ | KEY_WOW64_64KEY,
        },
        HklmUninstallLocation {
            subkey: HKLM_WOW6432_UNINSTALL,
            architecture: AppArchitecture::X86,
            flags: KEY_READ | KEY_WOW64_32KEY,
        },
    ];

    let mut apps = Vec::new();
    for location in hklm_locations {
        apps.extend(read_hklm_uninstall(system, &location)?);
    }
    // Per-user apps: live yields the current user; offline yields every profile.
    for user in system.user_hives() {
        apps.extend(read_hkcu_uninstall(system, &user));
    }

    apps.sort_by(|left, right| {
        left.display_name
            .to_ascii_lowercase()
            .cmp(&right.display_name.to_ascii_lowercase())
    });
    Ok(apps)
}

fn read_hklm_uninstall(
    system: &SystemContext,
    location: &HklmUninstallLocation,
) -> Result<Vec<InstalledApp>> {
    let uninstall_key = match system.open_hklm(location.subkey, location.flags) {
        Ok(key) => key,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(WinAppMigratorError::Registry(format!(
                "HKLM\\{}: {}",
                location.subkey, error
            )));
        }
    };
    Ok(collect_uninstall_entries(
        system,
        &uninstall_key,
        location.flags,
        "HKLM",
        location.subkey,
        location.architecture.clone(),
        InstallScope::PerMachine,
    ))
}

fn read_hkcu_uninstall(system: &SystemContext, user: &UserHive) -> Vec<InstalledApp> {
    let uninstall_key = match user.open(HKCU_UNINSTALL, KEY_READ) {
        Ok(key) => key,
        Err(_) => return Vec::new(),
    };
    collect_uninstall_entries(
        system,
        &uninstall_key,
        KEY_READ,
        "HKCU",
        HKCU_UNINSTALL,
        AppArchitecture::Unknown,
        InstallScope::PerUser,
    )
}

#[allow(clippy::too_many_arguments)]
fn collect_uninstall_entries(
    system: &SystemContext,
    uninstall_key: &RegKey,
    flags: u32,
    hive_label: &str,
    subkey: &str,
    architecture: AppArchitecture,
    install_scope: InstallScope,
) -> Vec<InstalledApp> {
    let mut apps = Vec::new();
    for key_name in uninstall_key.enum_keys() {
        let key_name = match key_name {
            Ok(name) => name,
            Err(error) => {
                warn!(%error, "failed to enumerate uninstall subkey name");
                continue;
            }
        };
        let child = match uninstall_key.open_subkey_with_flags(&key_name, flags) {
            Ok(child) => child,
            Err(error) => {
                warn!(%error, key_name, "failed to open uninstall subkey");
                continue;
            }
        };
        let raw = read_raw_entry(&child);
        let source_path = format!(r"{hive_label}\{subkey}\{key_name}");
        if let Some(app) = parse_uninstall_entry(
            system,
            &raw,
            source_path,
            architecture.clone(),
            install_scope.clone(),
        ) {
            apps.push(app);
        }
    }
    apps
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
    system: &SystemContext,
    raw: &RawUninstallEntry,
    registry_source_path: String,
    architecture: AppArchitecture,
    install_scope: InstallScope,
) -> Option<InstalledApp> {
    let display_name = raw.display_name.as_ref()?.trim().to_owned();
    if display_name.is_empty() {
        return None;
    }

    // Registry records paths as they were on the source system (e.g. C:\...);
    // re-root them onto the scanned volume before any existence checks.
    let install_location = raw
        .install_location
        .as_ref()
        .map(PathBuf::from)
        .map(|path| system.resolve_source_path(&path));
    let main_executable =
        guess_main_executable(system, raw, install_location.as_deref(), display_name.as_str());

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
    system: &SystemContext,
    raw: &RawUninstallEntry,
    install_location: Option<&Path>,
    display_name: &str,
) -> Option<PathBuf> {
    for value in [&raw.display_icon, &raw.uninstall_string]
        .into_iter()
        .flatten()
    {
        if let Some(path) = split_command_executable_path(value) {
            let path = system.resolve_source_path(&path);
            if looks_like_executable(&path) && path.exists() {
                return Some(path);
            }
        }
    }

    install_location.and_then(|path| guess_executable_from_install_dir(path, display_name))
}

fn guess_executable_from_install_dir(
    install_location: &Path,
    display_name: &str,
) -> Option<PathBuf> {
    // Many modern installers place the launcher in a subdirectory (commonly
    // `bin`) rather than the install root, so check the root first and then a
    // few well-known subdirectories before giving up.
    if let Some(path) = find_executable_in_dir(install_location, display_name) {
        return Some(path);
    }
    for subdir in ["bin", "app", "Application", "program", "Programs"] {
        let candidate_dir = install_location.join(subdir);
        if candidate_dir.is_dir()
            && let Some(path) = find_executable_in_dir(&candidate_dir, display_name)
        {
            return Some(path);
        }
    }
    None
}

fn find_executable_in_dir(dir: &Path, display_name: &str) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    let normalized_display = normalize_name_for_match(display_name);
    let mut executable_paths = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if !looks_like_executable(&path) {
            continue;
        }
        if is_auxiliary_executable(&path) {
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

/// Recognize uninstallers and helper stubs that should not be treated as the
/// application's main executable.
fn is_auxiliary_executable(path: &Path) -> bool {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    const AUXILIARY_MARKERS: &[&str] = &[
        "unins",
        "uninstall",
        "setup",
        "install",
        "update",
        "updater",
        "crashpad",
        "crashreporter",
        "vcredist",
        "vc_redist",
        "dotnet",
        "helper",
    ];
    AUXILIARY_MARKERS
        .iter()
        .any(|marker| stem.starts_with(marker) || stem.contains(marker))
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
            &SystemContext::live(),
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
            &SystemContext::live(),
            &raw,
            "HKCU\\Example".to_owned(),
            AppArchitecture::Unknown,
            InstallScope::PerUser,
        );

        assert!(app.is_none());
    }
}
