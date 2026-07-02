//! Where a scan reads its state from: the **live** running OS, or an **offline
//! volume** — a disk pulled from another (often failing) PC and mounted here,
//! e.g. `E:\`.
//!
//! Every discovery module used to reach the live system directly through two
//! seams: `RegKey::predef(HKLM|HKCU)` and `std::env::var_os(known folder)`.
//! [`SystemContext`] is the single abstraction both seams now go through, so the
//! same scanners work against a foreign disk. For an offline volume it:
//!
//! * mounts the disk's registry hives read-only via `reg load` (which uses the
//!   process's backup privilege, so locked `config` hives read regardless of
//!   ACL — non-destructively), unloading them again on drop;
//! * derives every known-folder base (`Program Files`, `ProgramData`, user
//!   profiles, `Public`, `Windows`) from the mounted volume root instead of the
//!   live environment; and
//! * re-roots absolute paths recorded on the source system (`C:\Program
//!   Files\App`) onto the mounted volume (`E:\Program Files\App`).
//!
//! Offline scanning requires elevation (for `reg load` and backup-mode reads);
//! [`SystemContext::open`] fails clearly otherwise.

use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::warn;
use winreg::RegKey;
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY,
};

use crate::errors::{Result, WinAppMigratorError};
use crate::path_utils::hidden_command;
use crate::privilege::is_elevated;

/// Profile directory names under `Users\` that are not real user profiles.
pub const NON_USER_PROFILE_NAMES: &[&str] = &["All Users", "Default", "Default User", "Public"];

const CONFIG_SOFTWARE: &str = r"Windows\System32\config\SOFTWARE";
const CONFIG_SYSTEM: &str = r"Windows\System32\config\SYSTEM";
const NTUSER_DAT: &str = "NTUSER.DAT";
const DEFAULT_CONTROL_SET: &str = "ControlSet001";

/// Where a scan reads its state from. Cheap to clone and serialize so it can be
/// threaded through [`crate::models::ScanOptions`] / `PackageOptions` and moved
/// into worker threads.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ScanSource {
    /// The live, running operating system (the default and historical behavior).
    #[default]
    Live,
    /// A disk from another system, mounted at `volume_root` (e.g. `E:\`).
    OfflineVolume { volume_root: PathBuf },
}

impl ScanSource {
    pub fn offline(volume_root: impl Into<PathBuf>) -> Self {
        Self::OfflineVolume {
            volume_root: volume_root.into(),
        }
    }

    pub fn is_offline(&self) -> bool {
        matches!(self, Self::OfflineVolume { .. })
    }
}

/// A per-user registry hive to scan: the live current user, or one mounted
/// `NTUSER.DAT` from an offline volume.
pub struct UserHive {
    /// The account name (best-effort: the profile directory name offline).
    pub name: String,
    /// The user's profile directory (`%USERPROFILE%` live; `<root>\Users\<name>`
    /// offline).
    pub profile_dir: PathBuf,
    kind: UserHiveKind,
}

enum UserHiveKind {
    Live,
    Offline { mount: String },
}

impl UserHive {
    /// Open a subkey under this user's hive. `logical` is relative to the hive
    /// root exactly as it would appear under `HKEY_CURRENT_USER`, e.g.
    /// `Software\Vendor\App` or `Environment`.
    pub fn open(&self, logical: &str, flags: u32) -> std::io::Result<RegKey> {
        match &self.kind {
            UserHiveKind::Live => {
                RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(logical, flags)
            }
            UserHiveKind::Offline { mount } => RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(format!(r"{mount}\{logical}"), mask_offline_flags(flags)),
        }
    }
}

/// The mounted hives of an offline volume. Dropping it unloads every mount.
struct OfflineHives {
    volume_root: PathBuf,
    software_mount: String,
    system_mount: Option<String>,
    control_set: String,
    users: Vec<OfflineUser>,
    _guard: MountGuard,
}

struct OfflineUser {
    name: String,
    profile_dir: PathBuf,
    mount: String,
}

/// Unloads every `reg load`-mounted hive when the context is dropped, so no
/// mount leaks into the live registry after a scan.
struct MountGuard {
    mounts: Vec<String>,
}

impl MountGuard {
    fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    fn push(&mut self, mount: String) {
        self.mounts.push(mount);
    }
}

impl Drop for MountGuard {
    fn drop(&mut self) {
        // Unload in reverse so children unload before any (hypothetical) parent.
        for mount in self.mounts.iter().rev() {
            if let Err(error) = reg_unload(mount) {
                warn!(%error, mount, "failed to unload offline registry hive");
            }
        }
    }
}

/// The single seam every scanner reads state through. Construct once per scan
/// with [`SystemContext::open`] and pass `&SystemContext` to each scanner.
pub struct SystemContext {
    source: ScanSource,
    offline: Option<OfflineHives>,
}

impl SystemContext {
    /// A context bound to the live, running OS.
    pub fn live() -> Self {
        Self {
            source: ScanSource::Live,
            offline: None,
        }
    }

    /// Build the context for `source`. For an offline volume this mounts the
    /// disk's registry hives (requiring elevation) and returns an error if the
    /// volume is not a Windows system disk or the process is not elevated.
    pub fn open(source: &ScanSource) -> Result<Self> {
        match source {
            ScanSource::Live => Ok(Self::live()),
            ScanSource::OfflineVolume { volume_root } => {
                let offline = mount_offline(volume_root)?;
                Ok(Self {
                    source: source.clone(),
                    offline: Some(offline),
                })
            }
        }
    }

    pub fn source(&self) -> &ScanSource {
        &self.source
    }

    pub fn is_offline(&self) -> bool {
        self.offline.is_some()
    }

    /// A short human description for UI and warnings.
    pub fn describe(&self) -> String {
        match &self.offline {
            None => "this PC".to_owned(),
            Some(hives) => format!("connected drive {}", hives.volume_root.display()),
        }
    }

    // ---- Registry -----------------------------------------------------------

    /// Open a subkey under `HKEY_LOCAL_MACHINE` addressed by its **logical**
    /// path (as it appears on the live machine), e.g.
    /// `Software\Microsoft\Windows\CurrentVersion\Uninstall` or
    /// `SYSTEM\CurrentControlSet\Services`. Offline this transparently maps the
    /// logical path onto the mounted `SOFTWARE` / `SYSTEM` hive (resolving
    /// `CurrentControlSet` to the disk's active control set).
    pub fn open_hklm(&self, logical: &str, flags: u32) -> std::io::Result<RegKey> {
        match &self.offline {
            None => RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(logical, flags),
            Some(hives) => {
                let mapped = hives.map_hklm(logical).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("offline hive has no mapping for HKLM\\{logical}"),
                    )
                })?;
                RegKey::predef(HKEY_LOCAL_MACHINE)
                    .open_subkey_with_flags(mapped, mask_offline_flags(flags))
            }
        }
    }

    /// The per-user hives to scan. Live yields exactly the current user; offline
    /// yields every real profile whose `NTUSER.DAT` mounted successfully.
    pub fn user_hives(&self) -> Vec<UserHive> {
        match &self.offline {
            None => {
                let profile_dir = std::env::var_os("USERPROFILE")
                    .map(PathBuf::from)
                    .unwrap_or_default();
                let name = std::env::var_os("USERNAME")
                    .and_then(|value| value.into_string().ok())
                    .or_else(|| {
                        profile_dir
                            .file_name()
                            .and_then(|value| value.to_str())
                            .map(ToOwned::to_owned)
                    })
                    .unwrap_or_else(|| "current user".to_owned());
                vec![UserHive {
                    name,
                    profile_dir,
                    kind: UserHiveKind::Live,
                }]
            }
            Some(hives) => hives
                .users
                .iter()
                .map(|user| UserHive {
                    name: user.name.clone(),
                    profile_dir: user.profile_dir.clone(),
                    kind: UserHiveKind::Offline {
                        mount: user.mount.clone(),
                    },
                })
                .collect(),
        }
    }

    // ---- Known folders ------------------------------------------------------

    /// `Program Files` roots to search for install directories.
    pub fn program_files_dirs(&self) -> Vec<PathBuf> {
        match &self.offline {
            None => {
                let mut dirs = Vec::new();
                for env_var in ["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"] {
                    if let Some(path) = std::env::var_os(env_var).map(PathBuf::from)
                        && !dirs.contains(&path)
                    {
                        dirs.push(path);
                    }
                }
                dirs
            }
            Some(hives) => vec![
                hives.volume_root.join("Program Files"),
                hives.volume_root.join("Program Files (x86)"),
            ],
        }
    }

    /// The machine-wide `ProgramData` root.
    pub fn program_data(&self) -> Option<PathBuf> {
        match &self.offline {
            None => std::env::var_os("ProgramData").map(PathBuf::from),
            Some(hives) => Some(hives.volume_root.join("ProgramData")),
        }
    }

    /// The shared `Public` profile (`C:\Users\Public`).
    pub fn public_dir(&self) -> Option<PathBuf> {
        match &self.offline {
            None => std::env::var_os("PUBLIC")
                .map(PathBuf::from)
                .or_else(|| Some(PathBuf::from(r"C:\Users\Public"))),
            Some(hives) => Some(hives.volume_root.join(r"Users\Public")),
        }
    }

    /// The Windows directory (`%SystemRoot%`).
    pub fn windows_dir(&self) -> PathBuf {
        match &self.offline {
            None => std::env::var_os("SystemRoot")
                .or_else(|| std::env::var_os("windir"))
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(r"C:\Windows")),
            Some(hives) => hives.volume_root.join("Windows"),
        }
    }

    /// Every real user profile directory (`Users\<name>`), current user first
    /// when live. Excludes `Default`, `Public`, and the `All Users` junction.
    pub fn user_profiles(&self) -> Vec<PathBuf> {
        match &self.offline {
            None => {
                let mut profiles = Vec::new();
                if let Some(current) = std::env::var_os("USERPROFILE").map(PathBuf::from) {
                    profiles.push(current.clone());
                    if let Some(users_root) = current.parent() {
                        collect_profile_dirs(users_root, &mut profiles);
                    }
                }
                profiles
            }
            Some(hives) => {
                let mut profiles = Vec::new();
                collect_profile_dirs(&hives.volume_root.join("Users"), &mut profiles);
                profiles
            }
        }
    }

    // ---- Path re-rooting ----------------------------------------------------

    /// Re-root a path recorded on the source system onto the mounted volume:
    /// `C:\Program Files\App` → `<volume_root>\Program Files\App`. Live and
    /// already-rooted paths are returned unchanged.
    pub fn resolve_source_path(&self, recorded: &Path) -> PathBuf {
        match &self.offline {
            None => recorded.to_path_buf(),
            Some(hives) => reroot_onto_volume(recorded, &hives.volume_root),
        }
    }
}

// ---- Offline mounting -------------------------------------------------------

fn mount_offline(volume_root: &Path) -> Result<OfflineHives> {
    if !is_elevated() {
        return Err(WinAppMigratorError::InvalidInput(
            "Scanning a connected drive requires administrator rights (to read the drive's \
             registry hives and protected files). Restart WinAppMigrator as administrator."
                .to_owned(),
        ));
    }

    // Detect a Windows volume by its listable \Windows directory; the hive files
    // under \Windows\System32\config are ACL-protected and may not even stat
    // until reg load reads them through backup privilege, so don't precheck them.
    if !volume_root.join("Windows").is_dir() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} is not a Windows system drive (no \\Windows folder).",
            volume_root.display()
        )));
    }

    let pid = std::process::id();
    let mut guard = MountGuard::new();

    // reg load reports the authoritative failure (missing hive, access denied,
    // already loaded) if the SOFTWARE hive cannot be mounted.
    let software_hive = volume_root.join(CONFIG_SOFTWARE);
    let software_mount = format!("WAM_{pid}_SOFTWARE");
    reg_load(&software_mount, &software_hive)?;
    guard.push(software_mount.clone());

    // The SYSTEM hive (services, drivers, machine environment) is desirable but
    // not fatal if unreadable; degrade to a default control set.
    let system_hive = volume_root.join(CONFIG_SYSTEM);
    let system_mount = if system_hive.is_file() {
        let mount = format!("WAM_{pid}_SYSTEM");
        match reg_load(&mount, &system_hive) {
            Ok(()) => {
                guard.push(mount.clone());
                Some(mount)
            }
            Err(error) => {
                warn!(%error, "could not mount offline SYSTEM hive; services/drivers skipped");
                None
            }
        }
    } else {
        None
    };
    let control_set = system_mount
        .as_deref()
        .and_then(resolve_control_set)
        .unwrap_or_else(|| DEFAULT_CONTROL_SET.to_owned());

    let mut users = Vec::new();
    let mut used_mounts = 0_usize;
    for (name, profile_dir, ntuser) in enumerate_user_hives(volume_root) {
        let mount = format!("WAM_{pid}_USER_{used_mounts}");
        match reg_load(&mount, &ntuser) {
            Ok(()) => {
                guard.push(mount.clone());
                users.push(OfflineUser {
                    name,
                    profile_dir,
                    mount,
                });
                used_mounts += 1;
            }
            Err(error) => {
                warn!(%error, profile = %profile_dir.display(), "could not mount offline user hive");
            }
        }
    }

    Ok(OfflineHives {
        volume_root: volume_root.to_path_buf(),
        software_mount,
        system_mount,
        control_set,
        users,
        _guard: guard,
    })
}

impl OfflineHives {
    /// Map a logical `HKLM\...` path onto this volume's mounted hives, returning
    /// the subkey path under `HKEY_LOCAL_MACHINE` to open.
    fn map_hklm(&self, logical: &str) -> Option<String> {
        if let Some(rest) = strip_prefix_ci(logical, "Software") {
            // The SOFTWARE hive root corresponds to HKLM\Software.
            return Some(join_mount(&self.software_mount, &rest));
        }
        if let Some(rest) = strip_prefix_ci(logical, "SYSTEM") {
            let mount = self.system_mount.as_deref()?;
            let rest = rest.trim_start_matches('\\');
            // CurrentControlSet is a live-only symlink; resolve to the active set.
            if let Some(after) = strip_prefix_ci(rest, "CurrentControlSet") {
                return Some(join_mount(mount, &format!(r"{}{}", self.control_set, after)));
            }
            return Some(join_mount(mount, rest));
        }
        None
    }
}

/// `reg load HKLM\<mount> <hive_file>`.
fn reg_load(mount: &str, hive_file: &Path) -> Result<()> {
    let output = hidden_command("reg")
        .args([
            "load",
            &format!(r"HKLM\{mount}"),
            &hive_file.to_string_lossy(),
        ])
        .output()
        .map_err(|error| WinAppMigratorError::io(hive_file, error))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(WinAppMigratorError::Registry(format!(
            "reg load HKLM\\{mount} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

/// `reg unload HKLM\<mount>`.
fn reg_unload(mount: &str) -> Result<()> {
    let output = hidden_command("reg")
        .args(["unload", &format!(r"HKLM\{mount}")])
        .output()
        .map_err(|error| WinAppMigratorError::io(PathBuf::from(mount), error))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(WinAppMigratorError::Registry(format!(
            "reg unload HKLM\\{mount} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

/// Read `SYSTEM\Select\Current` from the mounted SYSTEM hive and format it as a
/// control-set key name (`ControlSet001`).
fn resolve_control_set(system_mount: &str) -> Option<String> {
    let select = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(format!(r"{system_mount}\Select"), KEY_READ)
        .ok()?;
    let current: u32 = select.get_value("Current").ok()?;
    Some(format!("ControlSet{current:03}"))
}

/// Enumerate `<root>\Users\<name>\NTUSER.DAT` for every real profile.
fn enumerate_user_hives(volume_root: &Path) -> Vec<(String, PathBuf, PathBuf)> {
    let mut hives = Vec::new();
    let users_root = volume_root.join("Users");
    let Ok(entries) = std::fs::read_dir(&users_root) else {
        return hives;
    };
    for entry in entries.flatten() {
        let profile_dir = entry.path();
        if !profile_dir.is_dir() || is_non_user_profile(&profile_dir) {
            continue;
        }
        let ntuser = profile_dir.join(NTUSER_DAT);
        if ntuser.is_file() {
            let name = profile_dir
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("user")
                .to_owned();
            hives.push((name, profile_dir, ntuser));
        }
    }
    hives
}

fn collect_profile_dirs(users_root: &Path, profiles: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(users_root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && !is_non_user_profile(&path) && !profiles.contains(&path) {
            profiles.push(path);
        }
    }
}

fn is_non_user_profile(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    NON_USER_PROFILE_NAMES
        .iter()
        .any(|skipped| name.eq_ignore_ascii_case(skipped))
}

// ---- Pure helpers -----------------------------------------------------------

fn mask_offline_flags(flags: u32) -> u32 {
    // WOW64 redirection views only exist on the live registry; mounted hives
    // contain explicit WOW6432Node subkeys, so strip the redirection flags.
    flags & !(KEY_WOW64_32KEY | KEY_WOW64_64KEY)
}

/// Strip a leading path segment (case-insensitively). Returns the remainder
/// including its leading separator (`""` when `path` equals `prefix`), or `None`
/// when `path` does not start with `prefix` as a whole segment.
fn strip_prefix_ci(path: &str, prefix: &str) -> Option<String> {
    let lower_path = path.to_ascii_lowercase();
    let lower_prefix = prefix.to_ascii_lowercase();
    let rest = lower_path.strip_prefix(&lower_prefix)?;
    if rest.is_empty() {
        Some(String::new())
    } else if rest.starts_with('\\') {
        Some(path[prefix.len()..].to_owned())
    } else {
        None
    }
}

fn join_mount(mount: &str, rest: &str) -> String {
    let rest = rest.trim_start_matches('\\');
    if rest.is_empty() {
        mount.to_owned()
    } else {
        format!(r"{mount}\{rest}")
    }
}

/// Re-root an absolute source path onto `volume_root` by replacing its drive
/// prefix. Paths already under `volume_root`, or without a drive prefix, are
/// returned unchanged.
fn reroot_onto_volume(recorded: &Path, volume_root: &Path) -> PathBuf {
    if recorded.starts_with(volume_root) {
        return recorded.to_path_buf();
    }
    let mut components = recorded.components();
    match components.next() {
        Some(Component::Prefix(_)) => {
            // Drop the drive prefix and its root, rejoin the tail on the volume.
            let mut rebuilt = volume_root.to_path_buf();
            for component in components {
                match component {
                    Component::RootDir | Component::CurDir => {}
                    Component::Normal(part) => rebuilt.push(part),
                    Component::ParentDir => rebuilt.push(".."),
                    Component::Prefix(_) => {}
                }
            }
            rebuilt
        }
        _ => recorded.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reroots_system_drive_paths_onto_the_volume() {
        let volume = Path::new(r"E:\");
        assert_eq!(
            reroot_onto_volume(Path::new(r"C:\Program Files\App\app.exe"), volume),
            PathBuf::from(r"E:\Program Files\App\app.exe")
        );
    }

    #[test]
    fn reroot_is_idempotent_for_paths_already_on_the_volume() {
        let volume = Path::new(r"E:\");
        assert_eq!(
            reroot_onto_volume(Path::new(r"E:\Program Files\App"), volume),
            PathBuf::from(r"E:\Program Files\App")
        );
    }

    #[test]
    fn reroot_leaves_relative_paths_untouched() {
        let volume = Path::new(r"E:\");
        assert_eq!(
            reroot_onto_volume(Path::new(r"relative\path"), volume),
            PathBuf::from(r"relative\path")
        );
    }

    #[test]
    fn maps_software_logical_paths_onto_the_software_mount() {
        let hives = OfflineHives {
            volume_root: PathBuf::from(r"E:\"),
            software_mount: "WAM_1_SOFTWARE".to_owned(),
            system_mount: Some("WAM_1_SYSTEM".to_owned()),
            control_set: "ControlSet001".to_owned(),
            users: Vec::new(),
            _guard: MountGuard::new(),
        };
        assert_eq!(
            hives.map_hklm(r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
            Some(r"WAM_1_SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall".to_owned())
        );
    }

    #[test]
    fn maps_current_control_set_to_the_active_set() {
        let hives = OfflineHives {
            volume_root: PathBuf::from(r"E:\"),
            software_mount: "WAM_1_SOFTWARE".to_owned(),
            system_mount: Some("WAM_1_SYSTEM".to_owned()),
            control_set: "ControlSet002".to_owned(),
            users: Vec::new(),
            _guard: MountGuard::new(),
        };
        assert_eq!(
            hives.map_hklm(r"SYSTEM\CurrentControlSet\Services"),
            Some(r"WAM_1_SYSTEM\ControlSet002\Services".to_owned())
        );
    }

    #[test]
    fn strip_prefix_requires_whole_segment() {
        assert_eq!(strip_prefix_ci("Software\\X", "Software"), Some("\\X".to_owned()));
        assert_eq!(strip_prefix_ci("Software", "Software"), Some(String::new()));
        // Must not match a partial segment like "Soft".
        assert_eq!(strip_prefix_ci("SoftwareX\\Y", "Software"), None);
    }
}
