use std::fs;
use std::path::{Path, PathBuf};

use tracing::warn;
use winreg::RegKey;
use winreg::enums::{KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{holds_a_known_base_dir, known_base_dirs};
use crate::install_source::{
    ExecutableHints, is_usable_main_executable, is_windows_or_installer_cache_path,
};
use crate::models::{AppArchitecture, InstallScope, InstalledApp};
use crate::path_utils::{
    is_auxiliary_executable, is_drive_root, looks_like_executable, normalize_name_for_match,
    path_is_within, split_command_executable_path,
};
use crate::pe_scan::read_pe_architecture;
use crate::product_match::ProductNames;
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
const SYSTEM_COMPONENT_VALUE: &str = "SystemComponent";

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
    /// Marked `SystemComponent=1`: a part of another product, which Windows leaves out
    /// of its own installed-apps list.
    pub system_component: bool,
}

/// What every entry in a listing is checked against, read once for the whole listing.
pub struct InventoryContext {
    hints: ExecutableHints,
    machine_hints: ExecutableHints,
    known_base_dirs: Vec<PathBuf>,
}

impl InventoryContext {
    pub fn read(system: &SystemContext) -> Self {
        Self {
            hints: ExecutableHints::read(system),
            machine_hints: ExecutableHints::read_machine_wide(system),
            known_base_dirs: known_base_dirs(system),
        }
    }
}

/// One entry of the Uninstall key, hidden or not, with no looking for what it runs. For telling
/// what might be part of an app from what is someone else's, which only takes where each says it is.
#[derive(Debug, Clone)]
pub struct UninstallRecord {
    pub display_name: String,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub uninstall_string: Option<String>,
    pub registry_source_path: String,
    pub architecture: AppArchitecture,
    pub install_scope: InstallScope,
    pub owner_profile: Option<String>,
    /// Marked as a part of something else, so no list of apps shows it.
    pub hidden: bool,
    /// Where it says it is installed, if that is a folder of its own, as it can be read now.
    pub install_location: Option<PathBuf>,
    /// The folders of its icon and its uninstaller, as it can be read now, if those are its own.
    pub program_folders: Vec<PathBuf>,
}

impl UninstallRecord {
    /// What [`crate::install_source::detect_install_source`] goes by for it.
    pub(crate) fn as_installed_app(&self) -> InstalledApp {
        InstalledApp {
            display_name: self.display_name.clone(),
            publisher: self.publisher.clone(),
            version: self.version.clone(),
            install_location: self.install_location.clone(),
            uninstall_string: self.uninstall_string.clone(),
            main_executable: None,
            registry_source_path: self.registry_source_path.clone(),
            architecture: self.architecture.clone(),
            install_scope: self.install_scope.clone(),
            owner_profile: self.owner_profile.clone(),
        }
    }
}

/// Every entry of the Uninstall key on the system `system` points at, the machine's and the
/// account's, with the hidden ones. Reads no folders but to ask whether each may be read.
pub fn read_uninstall_records(system: &SystemContext) -> Vec<UninstallRecord> {
    let known_bases = known_base_dirs(system);
    let mut records = Vec::new();
    for (subkey, architecture, flags) in [
        (
            HKLM_UNINSTALL,
            AppArchitecture::X64,
            KEY_READ | KEY_WOW64_64KEY,
        ),
        (
            HKLM_WOW6432_UNINSTALL,
            AppArchitecture::X86,
            KEY_READ | KEY_WOW64_32KEY,
        ),
    ] {
        if let Ok(key) = system.open_hklm(subkey, flags) {
            collect_uninstall_records(
                system,
                &known_bases,
                &key,
                flags,
                &format!(r"HKLM\{subkey}"),
                &architecture,
                &InstallScope::PerMachine,
                None,
                &mut records,
            );
        }
    }
    for user in system.user_hives() {
        if let Ok(key) = user.open(HKCU_UNINSTALL, KEY_READ) {
            collect_uninstall_records(
                system,
                &known_bases,
                &key,
                KEY_READ,
                &format!(r"HKCU\{HKCU_UNINSTALL}"),
                &AppArchitecture::Unknown,
                &InstallScope::PerUser,
                Some(&user.id),
                &mut records,
            );
        }
    }
    records
}

#[allow(clippy::too_many_arguments)]
fn collect_uninstall_records(
    system: &SystemContext,
    known_bases: &[PathBuf],
    uninstall_key: &RegKey,
    flags: u32,
    prefix: &str,
    architecture: &AppArchitecture,
    install_scope: &InstallScope,
    owner_profile: Option<&str>,
    records: &mut Vec<UninstallRecord>,
) {
    for key_name in uninstall_key.enum_keys().flatten() {
        let Ok(child) = uninstall_key.open_subkey_with_flags(&key_name, flags) else {
            continue;
        };
        let raw = read_raw_entry(&child);
        let Some(display_name) = raw.display_name.clone() else {
            continue;
        };
        let its_own_folder = |value: &Option<String>| {
            value
                .as_deref()
                .and_then(split_command_executable_path)
                .and_then(|path| system.resolve_source_path_quietly(&path))
                .filter(|path| !is_windows_or_installer_cache_path(path, system))
                .and_then(|path| path.parent().map(Path::to_path_buf))
                .and_then(|folder| usable_install_location(folder, known_bases, system))
        };
        records.push(UninstallRecord {
            display_name,
            publisher: raw.publisher.clone(),
            version: raw.version.clone(),
            uninstall_string: raw.uninstall_string.clone(),
            registry_source_path: format!(r"{prefix}\{key_name}"),
            architecture: architecture.clone(),
            install_scope: install_scope.clone(),
            owner_profile: owner_profile.map(ToOwned::to_owned),
            hidden: raw.system_component,
            install_location: raw
                .install_location
                .as_ref()
                .map(PathBuf::from)
                .and_then(|path| system.resolve_source_path_quietly(&path))
                .and_then(|path| usable_install_location(path, known_bases, system)),
            program_folders: [&raw.display_icon, &raw.uninstall_string]
                .into_iter()
                .filter_map(its_own_folder)
                .collect(),
        });
    }
}

/// List apps installed on the system `system` points at (live OS or a connected
/// offline volume). Paths in each entry are re-rooted onto the source system.
pub fn list_installed_apps_from(system: &SystemContext) -> Result<Vec<InstalledApp>> {
    let inventory = InventoryContext::read(system);
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
        apps.extend(read_hklm_uninstall(system, &inventory, &location)?);
    }
    // Per-user apps: live yields the current user; offline yields every profile.
    for user in system.user_hives() {
        apps.extend(read_hkcu_uninstall(system, &inventory, &user));
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
    inventory: &InventoryContext,
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
        inventory,
        &uninstall_key,
        location.flags,
        "HKLM",
        location.subkey,
        location.architecture.clone(),
        InstallScope::PerMachine,
        None,
    ))
}

fn read_hkcu_uninstall(
    system: &SystemContext,
    inventory: &InventoryContext,
    user: &UserHive,
) -> Vec<InstalledApp> {
    let uninstall_key = match user.open(HKCU_UNINSTALL, KEY_READ) {
        Ok(key) => key,
        Err(_) => return Vec::new(),
    };
    collect_uninstall_entries(
        system,
        inventory,
        &uninstall_key,
        KEY_READ,
        "HKCU",
        HKCU_UNINSTALL,
        AppArchitecture::Unknown,
        InstallScope::PerUser,
        Some(&user.id),
    )
}

#[allow(clippy::too_many_arguments)]
fn collect_uninstall_entries(
    system: &SystemContext,
    inventory: &InventoryContext,
    uninstall_key: &RegKey,
    flags: u32,
    hive_label: &str,
    subkey: &str,
    architecture: AppArchitecture,
    install_scope: InstallScope,
    owner_profile: Option<&str>,
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
            inventory,
            &raw,
            source_path,
            architecture.clone(),
            install_scope.clone(),
        ) {
            apps.push(InstalledApp {
                owner_profile: owner_profile.map(ToOwned::to_owned),
                ..app
            });
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
        system_component: key
            .get_value::<u32, _>(SYSTEM_COMPONENT_VALUE)
            .is_ok_and(|value| value == 1),
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
    inventory: &InventoryContext,
    raw: &RawUninstallEntry,
    registry_source_path: String,
    architecture: AppArchitecture,
    install_scope: InstallScope,
) -> Option<InstalledApp> {
    let display_name = raw.display_name.as_ref()?.trim().to_owned();
    if display_name.is_empty() || raw.system_component {
        return None;
    }

    // Registry records paths as they were on the source system (e.g. C:\...);
    // re-root them onto the scanned volume before any existence checks.
    let install_location = raw
        .install_location
        .as_ref()
        .map(PathBuf::from)
        .and_then(|path| system.resolve_source_path(&path))
        .and_then(|path| usable_install_location(path, &inventory.known_base_dirs, system));
    // A folder can be listed with no look at what is in it, so what is found there may be a link.
    let main_executable = guess_main_executable(
        system,
        if install_scope == InstallScope::PerUser {
            &inventory.hints
        } else {
            &inventory.machine_hints
        },
        raw,
        install_location.as_deref(),
        display_name.as_str(),
    )
    .filter(|path| system.may_read(path));
    let architecture = architecture_of(main_executable.as_deref(), architecture);

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
        owner_profile: None,
    })
}

/// What an app's program says it is. Which view of the registry its entry is in only says what its
/// setup was: Eclipse Mosquitto's is 32-bit, and its programs are not.
fn architecture_of(main_executable: Option<&Path>, listed_in: AppArchitecture) -> AppArchitecture {
    main_executable
        .and_then(|program| read_pe_architecture(program).ok())
        .filter(|architecture| *architecture != AppArchitecture::Unknown)
        .unwrap_or(listed_in)
}

/// The recorded install location, or `None` when it names a folder that holds other
/// things side by side: a drive, `C:\Program Files`, the Windows directory. Taken at its
/// word, that would make every app in it part of this one.
fn usable_install_location(
    path: PathBuf,
    known_base_dirs: &[PathBuf],
    system: &SystemContext,
) -> Option<PathBuf> {
    let text = path.to_string_lossy();
    let trimmed = text.trim_end_matches(['\\', '/']);
    let path = if trimmed.ends_with(':') {
        path.clone()
    } else {
        PathBuf::from(trimmed)
    };
    if !system.may_read(&path) {
        warn!(location = %path.display(), "ignoring an install location that may not be read");
        return None;
    }
    let windows_dir = system.windows_dir();
    let is_windows_host_dir = path_is_within(&path, &windows_dir)
        && ["System32", "SysWOW64"]
            .iter()
            .any(|name| path_is_within(&windows_dir.join(name), &path));
    if is_drive_root(&path) || holds_a_known_base_dir(&path, known_base_dirs) || is_windows_host_dir
    {
        warn!(location = %path.display(), "ignoring an install location that is a shared folder");
        return None;
    }
    Some(path)
}

/// Guess the main executable from what the uninstall entry says, without reading the
/// install record, which is too slow to do for every row. In order: the entry's icon,
/// App Paths and Start Menu shortcuts, the install folder, and the uninstall command,
/// which is only ever kept if it is a real program that handles its own uninstall. Each
/// step skips uninstallers, updaters, Windows hosts, and installer caches. `None` leaves
/// it to `install_source::primary_executable` when the app is scanned.
fn guess_main_executable(
    system: &SystemContext,
    hints: &ExecutableHints,
    raw: &RawUninstallEntry,
    install_location: Option<&Path>,
    display_name: &str,
) -> Option<PathBuf> {
    let resolved = |value: &Option<String>| {
        value
            .as_deref()
            .and_then(split_command_executable_path)
            .and_then(|path| system.resolve_source_path(&path))
            .filter(|path| is_usable_main_executable(path, system))
    };
    resolved(&raw.display_icon)
        .or_else(|| {
            let names = ProductNames::new(Some(display_name), raw.publisher.as_deref(), &[], &[]);
            hints.find(&names, install_location)
        })
        .or_else(|| {
            install_location
                .and_then(|path| guess_executable_from_install_dir(path, display_name, system))
        })
        .or_else(|| resolved(&raw.uninstall_string))
}

fn guess_executable_from_install_dir(
    install_location: &Path,
    display_name: &str,
    system: &SystemContext,
) -> Option<PathBuf> {
    // Many modern installers place the launcher in a subdirectory (commonly
    // `bin`) rather than the install root, so check the root first and then a
    // few well-known subdirectories before giving up.
    if let Some(path) = find_executable_in_dir(install_location, display_name, system) {
        return Some(path);
    }
    for subdir in ["bin", "app", "Application", "program", "Programs"] {
        let candidate_dir = install_location.join(subdir);
        if system.is_dir(&candidate_dir)
            && let Some(path) = find_executable_in_dir(&candidate_dir, display_name, system)
        {
            return Some(path);
        }
    }
    None
}

fn find_executable_in_dir(
    dir: &Path,
    display_name: &str,
    system: &SystemContext,
) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    let normalized_display = normalize_name_for_match(display_name);
    let mut executable_paths = Vec::new();
    let mut overlapping = None;

    for entry in entries.flatten() {
        let path = entry.path();
        if !looks_like_executable(&path) || !system.may_read(&path) {
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
        if !stem.is_empty() && stem == normalized_display {
            return Some(path);
        }
        if overlapping.is_none()
            && !stem.is_empty()
            && !normalized_display.is_empty()
            && (normalized_display.contains(&stem) || stem.contains(&normalized_display))
        {
            overlapping = Some(path.clone());
        }
        executable_paths.push(path);
    }

    overlapping.or_else(|| {
        if executable_paths.len() == 1 {
            executable_paths.into_iter().next()
        } else {
            None
        }
    })
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
            system_component: false,
        };

        let system = SystemContext::live();
        let app = parse_uninstall_entry(
            &system,
            &InventoryContext::read(&system),
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
    fn an_app_is_what_its_program_says_and_otherwise_where_it_was_listed() -> Result<()> {
        let running =
            std::env::current_exe().map_err(|error| WinAppMigratorError::io("exe", error))?;
        let own = read_pe_architecture(&running)?;
        assert_ne!(own, AppArchitecture::Unknown);
        let other = if own == AppArchitecture::X86 {
            AppArchitecture::X64
        } else {
            AppArchitecture::X86
        };
        assert_eq!(architecture_of(Some(&running), other.clone()), own);
        assert_eq!(architecture_of(None, other.clone()), other);
        let not_a_program = running.with_extension("not-there");
        assert_eq!(architecture_of(Some(&not_a_program), other.clone()), other);
        Ok(())
    }

    #[test]
    fn skips_system_components_as_windows_does() {
        let raw = RawUninstallEntry {
            display_name: Some("Runtime Part".to_owned()),
            system_component: true,
            ..RawUninstallEntry::default()
        };

        let system = SystemContext::live();
        let app = parse_uninstall_entry(
            &system,
            &InventoryContext::read(&system),
            &raw,
            "HKLM\\Example".to_owned(),
            AppArchitecture::X64,
            InstallScope::PerMachine,
        );

        assert!(app.is_none());
    }

    #[test]
    fn skips_entries_without_display_name() {
        let raw = RawUninstallEntry::default();

        let system = SystemContext::live();
        let app = parse_uninstall_entry(
            &system,
            &InventoryContext::read(&system),
            &raw,
            "HKCU\\Example".to_owned(),
            AppArchitecture::Unknown,
            InstallScope::PerUser,
        );

        assert!(app.is_none());
    }

    fn unique_temp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "wam-inventory-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ))
    }

    fn app_from(raw: &RawUninstallEntry) -> Option<InstalledApp> {
        let system = SystemContext::live();
        parse_uninstall_entry(
            &system,
            &InventoryContext {
                hints: ExecutableHints::default(),
                machine_hints: ExecutableHints::default(),
                known_base_dirs: known_base_dirs(&system),
            },
            raw,
            r"HKLM\Example".to_owned(),
            AppArchitecture::X64,
            InstallScope::PerMachine,
        )
    }

    #[test]
    fn an_install_location_that_is_a_shared_folder_is_ignored() {
        let system = SystemContext::live();
        let windows_dir = system.windows_dir();
        let mut shared: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\"),
            windows_dir.join("System32"),
            windows_dir.clone(),
        ];
        shared.extend(system.program_files_dirs());
        shared.extend(system.program_data());
        shared.extend(
            system
                .user_profiles()
                .into_iter()
                .map(|profile| profile.join(r"AppData\Local\Programs")),
        );
        for path in shared {
            let raw = RawUninstallEntry {
                display_name: Some("Sloppy Installer".to_owned()),
                install_location: Some(format!("{}\\", path.display())),
                ..RawUninstallEntry::default()
            };
            let Some(app) = app_from(&raw) else {
                panic!("the entry itself is fine");
            };
            assert_eq!(app.install_location, None, "{}", path.display());
        }
    }

    #[test]
    fn an_install_location_loses_its_trailing_separator() {
        let raw = RawUninstallEntry {
            display_name: Some("Demo".to_owned()),
            install_location: Some(r"C:\Program Files\Git\".to_owned()),
            ..RawUninstallEntry::default()
        };
        let Some(app) = app_from(&raw) else {
            panic!("the entry itself is fine");
        };
        assert_eq!(
            app.install_location,
            Some(PathBuf::from(r"C:\Program Files\Git"))
        );
    }

    #[test]
    fn a_windows_host_or_uninstaller_is_not_the_main_executable()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        // NVIDIA App's uninstall command is rundll32; Git's is unins000.exe, and its icon
        // is an .ico file.
        let system = SystemContext::live();
        let base = unique_temp("hosts");
        fs::create_dir_all(&base)?;
        for name in ["unins000.exe", "git-bash.exe", "git-cmd.exe", "helper.exe"] {
            fs::write(base.join(name), b"MZ")?;
        }
        fs::write(base.join("git-for-windows.ico"), b"icon")?;

        let git = app_from(&RawUninstallEntry {
            display_name: Some("Git".to_owned()),
            install_location: Some(format!("{}\\", base.display())),
            display_icon: Some(base.join("git-for-windows.ico").display().to_string()),
            uninstall_string: Some(format!("\"{}\"", base.join("unins000.exe").display())),
            ..RawUninstallEntry::default()
        });
        let hosted = app_from(&RawUninstallEntry {
            display_name: Some("WAM Hosted Demo".to_owned()),
            uninstall_string: Some(format!(
                "\"{}\" \"C:\\Vendor\\Core.dll\",Uninstall",
                system
                    .windows_dir()
                    .join(r"System32\rundll32.exe")
                    .display()
            )),
            ..RawUninstallEntry::default()
        });
        let icon_under_windows = app_from(&RawUninstallEntry {
            display_name: Some("WAM Icon Demo".to_owned()),
            display_icon: Some(
                system
                    .windows_dir()
                    .join(r"System32\notepad.exe")
                    .display()
                    .to_string(),
            ),
            ..RawUninstallEntry::default()
        });
        let _ = fs::remove_dir_all(&base);

        let Some(git) = git else {
            panic!("the Git entry is fine");
        };
        assert_eq!(git.main_executable, Some(base.join("git-bash.exe")));
        let Some(hosted) = hosted else {
            panic!("the hosted entry is fine");
        };
        assert_eq!(hosted.main_executable, None);
        let Some(icon_under_windows) = icon_under_windows else {
            panic!("the icon entry is fine");
        };
        assert_eq!(icon_under_windows.main_executable, None);
        Ok(())
    }

    #[test]
    fn an_exact_name_beats_one_that_merely_overlaps()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("exact");
        fs::create_dir_all(&base)?;
        for name in ["git-bash.exe", "git-cmd.exe", "git.exe"] {
            fs::write(base.join(name), b"MZ")?;
        }
        let found = find_executable_in_dir(&base, "Git", &SystemContext::live());
        let _ = fs::remove_dir_all(&base);

        assert_eq!(found, Some(base.join("git.exe")));
        Ok(())
    }
}
