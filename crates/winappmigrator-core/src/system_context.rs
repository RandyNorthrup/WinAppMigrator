//! Where a scan reads its state from: the **live** running OS, or an **offline
//! volume** — a disk pulled from another (often failing) PC and mounted here,
//! e.g. `E:\`.
//!
//! Every discovery module used to reach the live system directly through two
//! seams: `RegKey::predef(HKLM|HKCU)` and `std::env::var_os(known folder)`.
//! [`SystemContext`] is the single abstraction both seams now go through, so the
//! same scanners work against a foreign disk. For an offline volume it:
//!
//! * copies the disk's registry hives to an administrators-only folder and loads the copies
//!   with `reg load`, unloading them again on drop. Loading a hive can replay its transaction
//!   logs into it, so this is what leaves the drive untouched, and asks only for a read of a disk
//!   that may be failing;
//! * derives every known-folder base (`Program Files`, `ProgramData`, user
//!   profiles, `Public`, `Windows`) from the mounted volume root instead of the
//!   live environment; and
//! * maps a path recorded on the source PC to where it is now, through [`VolumeMap`], which is
//!   the one place that does.
//!
//! **A disk from another PC is not trusted.** What its registry says is only ever a path on that
//! disk: nothing it says can lead a scan to this PC's own files, or to the network. A path that
//! cannot be read as one, and any link on the way to one, which would be resolved against this
//! PC, is not followed, and is listed in the scan's warnings.
//!
//! An elevated process may also be run by another administrator than the person signed in, so
//! the live seam goes through whoever's desktop this is, and not `HKEY_CURRENT_USER`. See
//! [`crate::path_utils::SignedInUser`].
//!
//! Offline scanning needs administrator rights (for `reg load` and backup-mode
//! reads). WinAppMigrator always runs elevated, so the mount succeeds; `reg load`
//! surfaces any failure (missing hive, access denied) directly.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::Read as _;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Component, Path, PathBuf, Prefix};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::warn;
use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::opens_as_device_on;
use crate::journal::machine_state_root;
use crate::models::{Warning, WarningSeverity};
use crate::package::copy_file_backup_mode_unless;
use crate::path_utils::{
    SignedInUser, file_name_string, has_parent_step, hidden_command, is_on_a_local_disk,
    is_person_sid, location_can_be_told, machine_variable, path_is_within, profiles_folder,
    program_data_dir, program_files_folders, public_profile_dir, resolved_location, signed_in_user,
    strip_prefix_ignore_case, system_root, windows_architecture_name, windows_build,
};
use crate::privilege::{
    ProtectedFolder, confirm_protected_folder, ensure_protected_folder, is_elevated, owner_of,
};

/// Profile directory names under `Users\` that are not real user profiles.
pub const NON_USER_PROFILE_NAMES: &[&str] = &["All Users", "Default", "Default User", "Public"];

const CONFIG_SOFTWARE: &str = r"Windows\System32\config\SOFTWARE";
const CONFIG_SYSTEM: &str = r"Windows\System32\config\SYSTEM";
const WINDOWS_FOLDER: &str = "Windows";
const NTUSER_DAT: &str = "NTUSER.DAT";
const USR_CLASS_DAT: &str = r"AppData\Local\Microsoft\Windows\UsrClass.dat";
const USER_CLASSES_KEY: &str = r"Software\Classes";
const DEFAULT_CONTROL_SET: &str = "ControlSet001";
const DEFAULT_SYSTEM_DRIVE: char = 'C';
const PROFILE_LIST_KEY: &str = r"Software\Microsoft\Windows NT\CurrentVersion\ProfileList";
const PROFILE_IMAGE_PATH_VALUE: &str = "ProfileImagePath";
const SYSTEM_DRIVE_VARIABLE: &str = "%systemdrive%";
const LOGON_UI_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Authentication\LogonUI";
const LAST_SIGNED_IN_VALUE: &str = "LastLoggedOnUserSID";
/// What Windows calls a hive's logs. They have to be beside it for it to read them.
const HIVE_LOG_SUFFIXES: [&str; 2] = [".LOG1", ".LOG2"];
/// How much of a name a value on a drive gets to put in front of whoever reads this.
const MAX_IDENTITY_CHARS: usize = 128;
/// Kept in the folder of installs' records, because that is the one place only administrators
/// can change that every administrator's run can find, and a crash can leave a hive there.
const HIVES_FOLDER: &str = "hives";
const HIVE_FOLDER_PREFIX: &str = "m-";
const MOUNT_PREFIX: &str = "WAM_";
const SOFTWARE_LABEL: &str = "SOFTWARE";
const SYSTEM_LABEL: &str = "SYSTEM";
const USER_LABEL: &str = "USER_";
const CLASSES_LABEL: &str = "CLASSES_";
const LEASE_FILE: &str = "in-use.lock";
const ERROR_SHARING_VIOLATION: i32 = 32;
/// Windows will not load a hive anywhere near this. On a drive made to attack this, one could be
/// made big enough to fill this PC's disk, for each account it says it has.
const MAX_HIVE_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_ACCOUNTS: usize = 64;
/// As a task names them, in English. A task usually has a SID for these instead.
const WINDOWS_OWN_ACCOUNTS: &[&str] = &[
    "SYSTEM",
    "LOCAL SERVICE",
    "LocalService",
    "NETWORK SERVICE",
    "NetworkService",
    "INTERACTIVE",
    "Authenticated Users",
    "Users",
    "Administrators",
    "Everyone",
];
const MAX_HIVE_BYTES_IN_ALL: u64 = 32 * 1024 * 1024 * 1024;
const UNLOAD_ATTEMPTS: u32 = 20;
const UNLOAD_RETRY_WAIT: Duration = Duration::from_millis(250);
/// How many of the paths a drive named and this did not follow to list, of however many.
const MAX_LISTED_REFUSALS: usize = 10;

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

/// Whose per-user data and settings a scan reads. It is only ever one account's, since they all
/// land in one account's on the target, and one person's private files are no part of another's
/// app.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum ProfileChoice {
    /// An app installed for one account, that account. Otherwise on this PC whoever is signed
    /// in, and on a drive whoever last signed in there, or the only account it has.
    #[default]
    Default,
    /// This one, by [`UserHive::id`].
    Profile(String),
    /// No one's.
    Nobody,
}

/// Where a drive from another PC is now, and what its PC called it. Paths in the scan's result
/// are as they were on that PC, so that they mean what its registry means by them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VolumeMap {
    pub volume_root: PathBuf,
    /// The letter of its Windows drive on that PC.
    pub system_drive: char,
}

impl VolumeMap {
    /// Where a path recorded on the source PC is on this drive, by how it is written. Only a full
    /// path on the source's Windows drive has an answer, made of ordinary names. Others say
    /// nothing about this disk, and going by them could reach this PC's own files or the network:
    /// `\Windows\x` and `x\y` depend on where this process is, `D:\x` was on another disk of that
    /// PC, and `\\server\x` would hand the server this account's password hash. A `..` could climb
    /// out of a volume mounted on a folder, and so could `.. ` once Windows drops the space. A
    /// path already on the drive stays where it is.
    pub fn to_volume(&self, recorded: &Path) -> Option<PathBuf> {
        let tail = strip_prefix_ignore_case(recorded, &self.volume_root)
            .or_else(|| self.below_system_drive(recorded))?;
        let mut on_volume = self.volume_root.clone();
        for component in tail.components() {
            let Component::Normal(name) = component else {
                return None;
            };
            if !is_ordinary_name(name) {
                return None;
            }
            on_volume.push(name);
        }
        Some(on_volume)
    }

    fn below_system_drive(&self, recorded: &Path) -> Option<PathBuf> {
        let mut components = recorded.components();
        let Component::Prefix(prefix) = components.next()? else {
            return None;
        };
        let drive = match prefix.kind() {
            Prefix::Disk(drive) | Prefix::VerbatimDisk(drive) => char::from(drive),
            _ => return None,
        };
        if !drive.eq_ignore_ascii_case(&self.system_drive) {
            return None;
        }
        (components.next()? == Component::RootDir).then(|| components.as_path().to_path_buf())
    }

    /// What the source PC called a path on this drive, which is what its registry calls it.
    pub fn to_logical(&self, on_volume: &Path) -> Option<PathBuf> {
        let tail = strip_prefix_ignore_case(on_volume, &self.volume_root)?;
        let mut logical = self.logical_root();
        logical.push(tail);
        Some(logical)
    }

    /// The source PC's Windows drive, such as `C:\`.
    fn logical_root(&self) -> PathBuf {
        PathBuf::from(format!(r"{}:\", self.system_drive))
    }

    /// Whether a path can be read without leaving the drive: it is on it, it is written in
    /// ordinary names, and nothing on the way to it from the drive's root is a link. Any link
    /// on the drive, junctions and mount points included, would be resolved against this PC.
    pub fn may_read(&self, on_volume: &Path) -> bool {
        self.may_read_noting(on_volume, &mut HashSet::new())
    }

    /// [`Self::may_read`], remembering in `checked` the folders it looked at, for asking about
    /// many files in the same few folders.
    pub fn may_read_noting(&self, on_volume: &Path, checked: &mut HashSet<String>) -> bool {
        let Some(relative) = strip_prefix_ignore_case(on_volume, &self.volume_root) else {
            return false;
        };
        let mut current = self.volume_root.clone();
        for component in relative.components() {
            let Component::Normal(name) = component else {
                return false;
            };
            if !is_ordinary_name(name) {
                return false;
            }
            current.push(name);
            // As it is written. On NTFS a folder can be made case-sensitive, and on a drive made
            // to attack this, `Data` and `DATA` can be a folder and a link. Names that are not text
            // were refused just now, so that no two come out the same here.
            let key = current.to_string_lossy().into_owned();
            if checked.contains(&key) {
                continue;
            }
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_symlink() => return false,
                Ok(_) => {
                    checked.insert(key);
                }
                // Nothing below a missing folder can exist, so there is nothing to lead anywhere.
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return true,
                Err(_) => return false,
            }
        }
        true
    }
}

/// A recorded path as Windows would read it. A name cannot end in a dot or a space, so Windows drops
/// them, and an installer told to use `C:\\Program Files\\Acme, Inc.\\App` makes `Acme, Inc` and
/// records what it was told. Read this way it is the real name that is asked about. `None` for a name
/// with nothing else in it, which is how `..` can be spelled.
fn as_windows_reads_it(recorded: &Path) -> Option<PathBuf> {
    let mut read = PathBuf::new();
    for component in recorded.components() {
        match component {
            Component::Normal(name) => match name.to_str() {
                Some(text) => {
                    let name = text.trim_end_matches(['.', ' ']);
                    // A stream's name, which on its own would read as a drive and replace all of this.
                    if name.is_empty() || name.contains(':') {
                        return None;
                    }
                    read.push(name);
                }
                None => read.push(name),
            },
            other => read.push(other.as_os_str()),
        }
    }
    Some(read)
}

/// What a name on a drive may not be: a stream, a device, a second separator after a `\\?\`, or
/// what Windows would strip from the end of a name. That is how `..` can be spelled so that nothing
/// here sees it, and how a real folder `x.` opens the link `x` beside it. A name that is not text
/// cannot be written to a manifest, or told from another by what is kept of it.
fn is_ordinary_name(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    !name.is_empty()
        && !name.ends_with(['.', ' '])
        && !name
            .chars()
            .any(|character| matches!(character, ':' | '/' | '\\') || character.is_control())
        && !opens_as_device_on(name, windows_build())
}

/// Whether this is the name of an account every Windows has, rather than a person's.
pub(crate) fn is_windows_own_account_name(name: &str) -> bool {
    WINDOWS_OWN_ACCOUNTS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(name))
}

/// What the PC an app was on called itself, and which Windows it ran.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceIdentity {
    pub computer_name: Option<String>,
    pub windows_version: Option<String>,
    pub windows_build: Option<u32>,
    /// `PROCESSOR_ARCHITECTURE`, as that PC recorded it.
    pub architecture_name: Option<String>,
}

/// A profile a PC records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedProfile {
    /// What to ask for it by: its SID, or the name of its folder when no record gave one.
    pub id: String,
    pub sid: Option<String>,
    /// The name of its folder, which is what the PC it is on can say about it.
    pub name: String,
    /// Where it can be read.
    pub dir: PathBuf,
}

/// A per-user registry hive to scan.
#[derive(Clone)]
pub struct UserHive {
    /// See [`ListedProfile::id`].
    pub id: String,
    pub sid: Option<String>,
    /// The name of the profile's folder.
    pub name: String,
    /// The user's profile directory, where it can be read (`<root>\Users\<name>` offline).
    pub profile_dir: PathBuf,
    kind: UserHiveKind,
}

#[derive(Clone)]
enum UserHiveKind {
    /// Under `HKEY_USERS`, because that account is signed in or something has it loaded.
    Loaded { sid: String },
    /// A copy of its `NTUSER.DAT`, with one of its `UsrClass.dat`, which is what appears under
    /// `Software\Classes` when the account is signed in.
    Mounted {
        mount: String,
        classes: Option<String>,
    },
}

impl UserHive {
    /// Open a subkey under this user's hive. `logical` is relative to the hive
    /// root exactly as it would appear under `HKEY_CURRENT_USER`, e.g.
    /// `Software\Vendor\App` or `Environment`.
    pub fn open(&self, logical: &str, flags: u32) -> std::io::Result<RegKey> {
        match &self.kind {
            UserHiveKind::Loaded { sid } => RegKey::predef(HKEY_USERS)
                .open_subkey_with_flags(format!(r"{sid}\{logical}"), flags),
            UserHiveKind::Mounted { mount, classes } => RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(
                    mounted_user_key(mount, classes.as_deref(), logical),
                    mask_offline_flags(flags),
                ),
        }
    }
}

/// Where a key in a user's hive is under `HKEY_LOCAL_MACHINE`. Windows puts `UsrClass.dat` under
/// `Software\Classes` when the account signs in, and `NTUSER.DAT` does not have them itself.
fn mounted_user_key(mount: &str, classes: Option<&str>, logical: &str) -> String {
    match (classes, strip_prefix_ci(logical, USER_CLASSES_KEY)) {
        (Some(classes), Some(rest)) => join_mount(classes, &rest),
        _ => join_mount(mount, logical),
    }
}

/// The folder a run's hive copies are in, and what it has loaded from them. Dropping it unloads
/// every one, so that no mount outlasts a scan, and removes the folder, whose hives can hold
/// secrets. If this process dies first, [`sweep_leaked_hive_mounts`] does that for it. Windows
/// also unloads them when it restarts, and only the copies are left then.
struct HiveMounts {
    id: String,
    next_folder: usize,
    bytes_left: u64,
    /// What says a person has stopped waiting, for a copy of what can be very large.
    cancel: Option<Arc<AtomicBool>>,
    mounts: Vec<String>,
    // In this order, because a field drops in the order it is declared, and the folder cannot
    // be removed while anything in it is held.
    _lease: File,
    folder: ProtectedFolder,
}

impl HiveMounts {
    fn begin() -> Result<Self> {
        if !is_elevated() {
            return Err(WinAppMigratorError::InvalidInput(
                "reading a connected drive or another account's settings needs administrator \
                 rights"
                    .to_owned(),
            ));
        }
        sweep_leaked_hive_mounts();
        let parent = hives_folder().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "this PC's ProgramData folder could not be found".to_owned(),
            )
        })?;
        for folder in parent.ancestors().take(2).collect::<Vec<_>>().iter().rev() {
            ensure_protected_folder(folder)
                .map_err(|error| WinAppMigratorError::io(folder, error))?;
        }
        let folder = ProtectedFolder::new_in(&parent, HIVE_FOLDER_PREFIX)
            .map_err(|error| WinAppMigratorError::io(&parent, error))?;
        let id = file_name_string(folder.path())
            .trim_start_matches(HIVE_FOLDER_PREFIX)
            .to_owned();
        let lease_path = folder.path().join(LEASE_FILE);
        let lease = OpenOptions::new()
            .write(true)
            .create_new(true)
            .share_mode(0)
            .open(&lease_path)
            .map_err(|error| WinAppMigratorError::io(&lease_path, error))?;
        Ok(Self {
            id,
            next_folder: 0,
            bytes_left: MAX_HIVE_BYTES_IN_ALL,
            cancel: None,
            mounts: Vec::new(),
            _lease: lease,
            folder,
        })
    }

    /// Copy a hive here and load the copy, returning the key under `HKEY_LOCAL_MACHINE`.
    fn load(&mut self, label: &str, hive_file: &Path) -> Result<String> {
        let copy_folder = self.folder.path().join(self.next_folder.to_string());
        self.next_folder += 1;
        let copy = copy_hive(
            hive_file,
            &copy_folder,
            &mut self.bytes_left,
            self.cancel.as_deref(),
        )?;
        let mount = format!("{MOUNT_PREFIX}{}_{label}", self.id);
        reg_load(&mount, &copy)?;
        self.mounts.push(mount.clone());
        Ok(mount)
    }
}

impl Drop for HiveMounts {
    fn drop(&mut self) {
        // Unload in reverse so children unload before any (hypothetical) parent.
        for mount in self.mounts.iter().rev() {
            if let Err(error) = unload_patiently(mount) {
                warn!(%error, mount, "failed to unload a registry hive");
            }
        }
    }
}

/// Windows and antivirus can be reading a hive that was only just loaded, and it cannot be
/// unloaded until they stop.
fn unload_patiently(mount: &str) -> Result<()> {
    let mut attempts_left = UNLOAD_ATTEMPTS;
    loop {
        match reg_unload(mount) {
            Ok(()) => return Ok(()),
            Err(_) if attempts_left > 0 => {
                attempts_left -= 1;
                std::thread::sleep(UNLOAD_RETRY_WAIT);
            }
            Err(error) => return Err(error),
        }
    }
}

fn hives_folder() -> Option<PathBuf> {
    machine_state_root().map(|root| root.join(HIVES_FOLDER))
}

/// Which run a mount belongs to, for a name this app gives a mount and no other: `WAM_`, an id,
/// and what it is. Anything else under that prefix is not this app's to touch.
fn run_of_mount(name: &str) -> Option<&str> {
    let rest = name.strip_prefix(MOUNT_PREFIX)?;
    let (id, label) = rest.split_once('_')?;
    let numbered = |prefix: &str| {
        label.strip_prefix(prefix).is_some_and(|number| {
            !number.is_empty() && number.chars().all(|digit| digit.is_ascii_digit())
        })
    };
    let is_ours = !id.is_empty()
        && id
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
        && (label == SOFTWARE_LABEL
            || label == SYSTEM_LABEL
            || numbered(USER_LABEL)
            || numbered(CLASSES_LABEL));
    is_ours.then_some(id)
}

/// What a sweep did. See [`sweep_leaked_hive_mounts`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SweepReport {
    pub unloaded: Vec<String>,
    /// Another run of this app is using these.
    pub still_in_use: Vec<String>,
    /// These should have gone, and would not. The next sweep tries again.
    pub failed: Vec<String>,
}

/// Unload what a run that died left loaded, and remove its hive copies. Run when the app starts,
/// and before mounting anything. A hive held by a run that is still going is left alone, which
/// its lease says, and a key this app did not name is not looked at. Needs elevation, and without
/// it, nothing could have been loaded.
pub fn sweep_leaked_hive_mounts() -> SweepReport {
    let mut report = SweepReport::default();
    if !is_elevated() {
        return report;
    }
    let Some(hives) = hives_folder() else {
        return report;
    };
    let keys: Vec<String> = RegKey::predef(HKEY_LOCAL_MACHINE)
        .enum_keys()
        .flatten()
        .collect();
    for key in &keys {
        let Some(id) = run_of_mount(key) else {
            continue;
        };
        let lease = hives
            .join(format!("{HIVE_FOLDER_PREFIX}{id}"))
            .join(LEASE_FILE);
        if lease_is_held(&lease) {
            report.still_in_use.push(key.clone());
            continue;
        }
        match unload_patiently(key) {
            Ok(()) => report.unloaded.push(key.clone()),
            Err(error) => {
                warn!(%error, key, "failed to unload a hive a run left behind");
                report.failed.push(key.clone());
            }
        }
    }
    // Anyone can have made this before this app first ran, as a link to somewhere else, and what is
    // below deletes what is in it.
    let is_this_apps = hives.parent().is_some_and(|parent| {
        !fs::symlink_metadata(parent).is_ok_and(|metadata| metadata.file_type().is_symlink())
    }) && confirm_protected_folder(&hives).is_ok();
    if !is_this_apps {
        return report;
    }
    let Ok(entries) = fs::read_dir(&hives) else {
        return report;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if file_name_string(&path).starts_with(HIVE_FOLDER_PREFIX)
            && !lease_is_held(&path.join(LEASE_FILE))
            && let Err(error) = fs::remove_dir_all(&path)
        {
            warn!(%error, folder = %path.display(), "failed to remove the hives a run left behind");
        }
    }
    report
}

/// A run holds its lease open with no sharing for as long as it lives, and the system closes it
/// however the run ends. Not being able to tell counts as held, so that nothing is pulled out
/// from under a run.
fn lease_is_held(lease: &Path) -> bool {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(0)
        .open(lease)
    {
        Ok(_) => false,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION) => true,
        Err(_) => true,
    }
}

/// Copy a hive, and the logs that say what was written last, to a folder of its own. Loading the
/// copy replays them into the copy, so that what was on the drive is read in full and the drive
/// is not written to. Failing to copy a log loses at most what it held, and says nothing is wrong
/// with the hive.
fn copy_hive(
    hive_file: &Path,
    into: &Path,
    bytes_left: &mut u64,
    cancel: Option<&AtomicBool>,
) -> Result<PathBuf> {
    let name = hive_file.file_name().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!("not a hive file: {}", hive_file.display()))
    })?;
    fs::create_dir_all(into).map_err(|error| WinAppMigratorError::io(into, error))?;
    let copy = into.join(name);
    copy_a_hive_file(hive_file, &copy, bytes_left, cancel)?;
    for suffix in HIVE_LOG_SUFFIXES {
        let mut log_name = name.to_os_string();
        log_name.push(suffix);
        let log = hive_file.with_file_name(&log_name);
        if fs::symlink_metadata(&log).is_ok_and(|metadata| metadata.is_file())
            && let Err(error) = copy_a_hive_file(&log, &into.join(&log_name), bytes_left, cancel)
        {
            warn!(%error, log = %log.display(), "could not copy a hive's log");
        }
    }
    Ok(copy)
}

/// Hives are protected on a drive that kept its permissions, and its accounts are not this PC's,
/// so a plain read may be refused where one in backup mode is not. A copy has to be writable to
/// load, whatever the original was marked.
fn copy_a_hive_file(
    source: &Path,
    destination: &Path,
    bytes_left: &mut u64,
    cancel: Option<&AtomicBool>,
) -> Result<()> {
    let size = fs::symlink_metadata(source).map_or(0, |metadata| metadata.len());
    let most_of_it = MAX_HIVE_BYTES.min(*bytes_left);
    if size > most_of_it {
        return Err(too_large_for_a_hive(source, size));
    }
    *bytes_left -= size;
    // By reading it, since `fs::copy` also brings the streams stored under its name, of any size.
    match File::open(source) {
        Ok(from) => {
            let mut to = File::create(destination)
                .map_err(|error| WinAppMigratorError::io(destination, error))?;
            std::io::copy(&mut from.take(MAX_HIVE_BYTES), &mut to)
                .map_err(|error| WinAppMigratorError::io(source, error))?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            copy_a_denied_hive_file(source, destination, size, most_of_it, bytes_left, cancel)?;
        }
        Err(error) => return Err(WinAppMigratorError::io(source, error)),
    }
    let metadata =
        fs::metadata(destination).map_err(|error| WinAppMigratorError::io(destination, error))?;
    let mut permissions = metadata.permissions();
    if permissions.readonly() {
        #[allow(clippy::permissions_set_readonly_false)]
        permissions.set_readonly(false);
        fs::set_permissions(destination, permissions)
            .map_err(|error| WinAppMigratorError::io(destination, error))?;
    }
    Ok(())
}

fn too_large_for_a_hive(source: &Path, size: u64) -> WinAppMigratorError {
    WinAppMigratorError::InvalidInput(format!(
        "{} is {size} bytes, too large to be a registry hive, or more than this drive's hives could \
         add up to",
        source.display()
    ))
}

/// Copy what only backup mode can read. A size says nothing of the streams stored under the same name,
/// which that copy brings too, and a drive can make those any size at all. So this watches how much
/// room the copy is taking, and stops it there.
fn copy_a_denied_hive_file(
    source: &Path,
    destination: &Path,
    size: u64,
    most: u64,
    bytes_left: &mut u64,
    cancel: Option<&AtomicBool>,
) -> Result<()> {
    let folder = destination.parent().unwrap_or(destination);
    let room_at_first = fs4::available_space(folder).unwrap_or(u64::MAX);
    let taken =
        || room_at_first.saturating_sub(fs4::available_space(folder).unwrap_or(room_at_first));
    let too_much = AtomicBool::new(false);
    let should_stop = || {
        if cancel.is_some_and(|cancel| cancel.load(Ordering::Relaxed)) {
            return true;
        }
        let over = taken() > most;
        if over {
            too_much.store(true, Ordering::Relaxed);
        }
        over
    };
    let copied = copy_file_backup_mode_unless(source, destination, &should_stop);
    if too_much.load(Ordering::Relaxed) {
        let _ = fs::remove_file(destination);
        return Err(too_large_for_a_hive(source, taken().max(most)));
    }
    copied?;
    // Its size has been counted. What it brought besides has not.
    *bytes_left = bytes_left.saturating_sub(taken().saturating_sub(size));
    Ok(())
}

/// The hives of a drive, and what could be read from them. Only `SOFTWARE` is required.
struct OfflineHives {
    map: VolumeMap,
    software_mount: String,
    system_mount: Option<String>,
    control_set: String,
    identity: SourceIdentity,
}

/// What a scan of this PC saw, to ask about again just before a file is copied. Whoever can write in
/// a folder that was scanned can make part of it a link to someone else's, or anywhere else, after
/// that. Nothing for a drive, where nothing that may be read goes through a link.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OtherAccountsFolders {
    is_of_this_pc: bool,
    folders: Vec<PathBuf>,
    /// Where the app is, which is allowed to be in one if the scan found that it may be.
    app_folder: Option<PathBuf>,
    /// The links that were on the way to what is being copied, such as a folder moved to another
    /// disk, and where each led. Any other has been made since. A walk does not go through a link,
    /// so all of these are above where it started.
    links_seen: BTreeMap<String, PathBuf>,
}

impl OtherAccountsFolders {
    /// Whether a path leads somewhere it should not now, wherever it says it is: through a link that
    /// was not there when it was scanned or leads somewhere else now, or into one of the other
    /// accounts' folders.
    pub fn hold(&self, path: &Path) -> bool {
        if !self.is_of_this_pc {
            return false;
        }
        let has_a_new_link = path.ancestors().any(|step| {
            fs::symlink_metadata(step).is_ok_and(|metadata| metadata.file_type().is_symlink())
                && self.links_seen.get(&comparison_key(step)) != Some(&resolved_location(step))
        });
        if has_a_new_link || !location_can_be_told(path) {
            return true;
        }
        let real = resolved_location(path);
        self.folders
            .iter()
            .any(|folder| path_is_within(&real, folder))
            && !self
                .app_folder
                .as_deref()
                .is_some_and(|folder| path_is_within(&real, folder))
    }
}

/// What this scan has been told about the accounts to read, and whose they are.
enum Users {
    /// Whoever is signed in here, asked when something needs to know. [`SystemContext::live`] is
    /// also for asking about folders, which must not have to run anything.
    SignedIn,
    Fixed(Vec<UserHive>),
}

/// The single seam every scanner reads state through. Construct once per scan
/// with [`SystemContext::open`] and pass `&SystemContext` to each scanner.
pub struct SystemContext {
    offline: Option<OfflineHives>,
    users: Users,
    listed: Vec<ListedProfile>,
    notes: Vec<Warning>,
    /// What a drive named that was not followed.
    refused: Mutex<BTreeSet<String>>,
    /// The folders on the drive seen to be free of links.
    checked: Mutex<HashSet<String>>,
    // Last, so that no key below it is open when it goes.
    _mounts: Option<HiveMounts>,
}

impl SystemContext {
    /// A context bound to the live, running OS, reading whoever is signed in.
    pub fn live() -> Self {
        Self {
            offline: None,
            users: Users::SignedIn,
            listed: Vec::new(),
            notes: Vec::new(),
            refused: Mutex::new(BTreeSet::new()),
            checked: Mutex::new(HashSet::new()),
            _mounts: None,
        }
    }

    /// Build the context for listing what is on `source`, which reads every account's on a
    /// drive. For an offline volume this mounts copies of the disk's registry hives (which needs
    /// administrator rights — the app runs elevated) and returns an error if the volume is not a
    /// Windows system disk or the hives cannot be mounted.
    pub fn open(source: &ScanSource) -> Result<Self> {
        match source {
            ScanSource::Live => Ok(Self::live()),
            ScanSource::OfflineVolume { volume_root } => {
                let mut mounts = HiveMounts::begin()?;
                let offline = mount_machine_hives(volume_root, &mut mounts)?;
                let mut context = Self::with_offline(offline);
                context.listed = context.read_listed_profiles();
                let listed = context.listed.clone();
                let users = context.mount_users(&listed, &mut mounts, false);
                context.users = Users::Fixed(users);
                context._mounts = Some(mounts);
                Ok(context)
            }
        }
    }

    /// Build the context for scanning one app, which reads one account. `owner` is whose the app
    /// is, if it was installed for one account.
    pub fn open_for_scan(
        source: &ScanSource,
        choice: &ProfileChoice,
        owner: Option<&str>,
        cancel: Arc<AtomicBool>,
    ) -> Result<Self> {
        match source {
            ScanSource::Live => {
                let mut context = Self::live();
                context.listed = context.read_listed_profiles();
                let chosen = context.choose_profile(choice, owner);
                context.settle_users(chosen, None);
                Ok(context)
            }
            ScanSource::OfflineVolume { volume_root } => {
                let mut mounts = HiveMounts::begin()?;
                mounts.cancel = Some(cancel);
                let offline = mount_machine_hives(volume_root, &mut mounts)?;
                let mut context = Self::with_offline(offline);
                context.listed = context.read_listed_profiles();
                let chosen = context.choose_profile(choice, owner);
                context.settle_users(chosen, Some(&mut mounts));
                context._mounts = Some(mounts);
                Ok(context)
            }
        }
    }

    /// Build the context for finding out which accounts have data for an app, which reads all of
    /// them. Without elevation, on this PC, that is those already loaded.
    pub fn open_every_profile(source: &ScanSource) -> Result<Self> {
        match source {
            ScanSource::Live => {
                let mut context = Self::live();
                context.listed = context.read_listed_profiles();
                let listed = context.listed.clone();
                // Only for an account that has to be loaded. Most PCs have the one, signed in.
                let needs_mounts = listed.iter().any(|profile| loaded_user(profile).is_none());
                let mut mounts = needs_mounts.then(HiveMounts::begin).and_then(Result::ok);
                let users = match mounts.as_mut() {
                    Some(mounts) => context.mount_users(&listed, mounts, false),
                    None => listed.iter().filter_map(loaded_user).collect(),
                };
                context.users = Users::Fixed(users);
                context._mounts = mounts;
                Ok(context)
            }
            ScanSource::OfflineVolume { .. } => Self::open(source),
        }
    }

    /// A drive with no hive loaded, which takes elevation, for testing what is read from its
    /// folders: one account is being scanned, and it has no settings to read.
    #[cfg(test)]
    pub(crate) fn offline_for_tests(volume_root: &Path, account: &Path) -> Self {
        let mut context = Self::with_offline(OfflineHives {
            map: VolumeMap {
                volume_root: volume_root.to_path_buf(),
                system_drive: 'C',
            },
            software_mount: "WAM_test_SOFTWARE".to_owned(),
            system_mount: None,
            control_set: DEFAULT_CONTROL_SET.to_owned(),
            identity: SourceIdentity::default(),
        });
        context.users = Users::Fixed(vec![UserHive {
            id: file_name_string(account),
            sid: None,
            name: file_name_string(account),
            profile_dir: account.to_path_buf(),
            kind: UserHiveKind::Mounted {
                mount: "WAM_test_USER_0".to_owned(),
                classes: None,
            },
        }]);
        context
    }

    fn with_offline(offline: OfflineHives) -> Self {
        Self {
            offline: Some(offline),
            users: Users::Fixed(Vec::new()),
            listed: Vec::new(),
            notes: Vec::new(),
            refused: Mutex::new(BTreeSet::new()),
            checked: Mutex::new(HashSet::new()),
            _mounts: None,
        }
    }

    /// Whichever profile a choice comes to. Anything that keeps it from coming to one is said.
    fn choose_profile(&mut self, choice: &ProfileChoice, owner: Option<&str>) -> Option<usize> {
        match choice {
            ProfileChoice::Nobody => None,
            ProfileChoice::Profile(id) => {
                let found = find_profile(&self.listed, id);
                if found.is_none() {
                    self.note(
                        "user_profile_not_found",
                        format!(
                            "The account {id} is not on record here, so no one's per-user data \
                             or settings were included."
                        ),
                    );
                }
                found
            }
            ProfileChoice::Default => {
                if let Some(found) = owner.and_then(|owner| find_profile(&self.listed, owner)) {
                    return Some(found);
                }
                if self.offline.is_some() {
                    self.default_on_a_drive()
                } else {
                    self.default_on_this_pc()
                }
            }
        }
    }

    fn default_on_this_pc(&mut self) -> Option<usize> {
        match signed_in_user() {
            SignedInUser::Desktop(account) | SignedInUser::NoDesktop(account) => {
                let found = find_profile(&self.listed, &account.sid);
                if found.is_none() {
                    self.note(
                        "user_profile_not_found",
                        format!(
                            "The account signed in here, {}, has no profile on record, so no \
                             one's per-user data or settings were included.",
                            account.name()
                        ),
                    );
                }
                found
            }
            SignedInUser::Unknown(reason) => {
                self.note(
                    "signed_in_user_unknown",
                    format!(
                        "Could not tell who is signed in ({reason}), so no one's per-user data \
                         or settings were included. Choose whose to include."
                    ),
                );
                None
            }
        }
    }

    fn default_on_a_drive(&mut self) -> Option<usize> {
        let last = self.last_signed_in_on_drive();
        if let Some(found) = last.and_then(|last| find_profile(&self.listed, &last)) {
            return Some(found);
        }
        if self.listed.len() == 1 {
            return Some(0);
        }
        if !self.listed.is_empty() {
            self.note(
                "user_profile_unknown",
                "This drive does not say who last signed in, and it has more than one account, \
                 so no one's per-user data or settings were included. Choose whose to include.",
            );
        }
        None
    }

    /// Load the chosen account, and with it, the only per-user hive this scan can see.
    fn settle_users(&mut self, chosen: Option<usize>, mounts: Option<&mut HiveMounts>) {
        let Some(profile) = chosen.and_then(|index| self.listed.get(index).cloned()) else {
            self.users = Users::Fixed(Vec::new());
            return;
        };
        let profiles = std::slice::from_ref(&profile);
        let users = match mounts {
            Some(mounts) => self.mount_users(profiles, mounts, true),
            None => match loaded_user(&profile) {
                Some(user) => vec![user],
                None => match HiveMounts::begin() {
                    Ok(mut mounts) => {
                        let users = self.mount_users(profiles, &mut mounts, true);
                        self._mounts = Some(mounts);
                        users
                    }
                    Err(error) => {
                        self.note(
                            "user_hive_unreadable",
                            format!(
                                "The settings of {} could not be read, so they were not \
                                 included: {error}",
                                profile.name
                            ),
                        );
                        Vec::new()
                    }
                },
            },
        };
        self.users = Users::Fixed(users);
    }

    /// Load each profile's hive, and what is under `Software\Classes` too if asked to. On this
    /// PC, one that is signed in is loaded already, and its files could not be read anyway.
    fn mount_users(
        &mut self,
        profiles: &[ListedProfile],
        mounts: &mut HiveMounts,
        with_classes: bool,
    ) -> Vec<UserHive> {
        let mut users = Vec::new();
        for (index, profile) in profiles.iter().enumerate() {
            if self.offline.is_none()
                && let Some(user) = loaded_user(profile)
            {
                users.push(user);
                continue;
            }
            let ntuser = profile.dir.join(NTUSER_DAT);
            if !self.may_read(&ntuser) {
                self.note_refused(&ntuser);
                continue;
            }
            let mount = match mounts.load(&format!("{USER_LABEL}{index}"), &ntuser) {
                Ok(mount) => mount,
                Err(error) => {
                    warn!(%error, profile = %profile.dir.display(), "could not load a user's hive");
                    self.note(
                        "user_hive_unreadable",
                        format!(
                            "The settings of {} could not be read, so they were not included: \
                             {error}",
                            profile.name
                        ),
                    );
                    continue;
                }
            };
            let classes_file = profile.dir.join(USR_CLASS_DAT);
            let classes = if with_classes
                && self.may_read(&classes_file)
                && fs::symlink_metadata(&classes_file).is_ok()
            {
                match mounts.load(&format!("{CLASSES_LABEL}{index}"), &classes_file) {
                    Ok(classes) => Some(classes),
                    Err(error) => {
                        self.note(
                            "user_classes_unreadable",
                            format!(
                                "The COM classes and file types registered for {} could not be \
                                 read, so they were not included: {error}",
                                profile.name
                            ),
                        );
                        None
                    }
                }
            } else {
                None
            };
            users.push(UserHive {
                id: profile.id.clone(),
                sid: profile.sid.clone(),
                name: profile.name.clone(),
                profile_dir: profile.dir.clone(),
                kind: UserHiveKind::Mounted { mount, classes },
            });
        }
        users
    }

    pub fn is_offline(&self) -> bool {
        self.offline.is_some()
    }

    fn note(&mut self, code: &str, message: impl Into<String>) {
        self.notes
            .push(Warning::new(code, WarningSeverity::Warning, message, None));
    }

    /// What this context has to say about what it read, including what a drive named and it did
    /// not follow. The paths are in the message and not its source, since a value can hold
    /// anything, and only a source goes in a log.
    pub fn warnings(&self) -> Vec<Warning> {
        let mut warnings = self.notes.clone();
        let refused = self
            .refused
            .lock()
            .map(|refused| refused.iter().cloned().collect::<Vec<String>>())
            .unwrap_or_default();
        if !refused.is_empty() {
            let on_a_drive = self.offline.is_some();
            let shown = refused
                .iter()
                .take(MAX_LISTED_REFUSALS)
                .cloned()
                .collect::<Vec<_>>()
                .join("; ");
            let more = refused.len().saturating_sub(MAX_LISTED_REFUSALS);
            let more = if more > 0 {
                format!(" and {more} more")
            } else {
                String::new()
            };
            let some = if refused.len() == 1 {
                "A path"
            } else {
                "Some paths"
            };
            warnings.push(if on_a_drive {
                Warning::new(
                    "drive_paths_not_followed",
                    WarningSeverity::Warning,
                    format!(
                        "{some} recorded on this drive could not be read, so nothing there was \
                         included. Each is on another disk, is not a full path, or goes through a \
                         link, which would lead off the drive: {shown}{more}"
                    ),
                    None,
                )
            } else {
                Warning::new(
                    "network_paths_not_followed",
                    WarningSeverity::Warning,
                    format!(
                        "{some} recorded on this PC could not be read, so nothing there was \
                         included. Each is not on a disk of this PC, where looking at it would \
                         connect to it, or has a name that would open something else: {shown}{more}"
                    ),
                    None,
                )
            });
        }
        warnings
    }

    fn note_refused(&self, recorded: &Path) {
        if let Ok(mut refused) = self.refused.lock() {
            refused.insert(recorded.display().to_string());
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

    /// The per-user hives to read. For a scan, that is the one account it is about, or none. For
    /// a listing of a drive, every account's.
    pub fn user_hives(&self) -> Vec<UserHive> {
        match &self.users {
            Users::Fixed(users) => users.clone(),
            Users::SignedIn => signed_in_user()
                .account()
                .filter(|account| loaded_hive(&account.sid))
                .map(|account| UserHive {
                    id: account.sid.clone(),
                    sid: Some(account.sid.clone()),
                    name: account.name(),
                    profile_dir: account.profile.clone(),
                    kind: UserHiveKind::Loaded {
                        sid: account.sid.clone(),
                    },
                })
                .into_iter()
                .collect(),
        }
    }

    /// Who an app's data defaults to, before asking: whoever is signed in on this PC, and on a
    /// drive whoever last did.
    pub fn default_profile_id(&self) -> Option<String> {
        match &self.offline {
            None => signed_in_user()
                .account()
                .map(|account| account.sid.clone()),
            Some(_) => self.last_signed_in_on_drive(),
        }
    }

    fn last_signed_in_on_drive(&self) -> Option<String> {
        self.open_hklm(LOGON_UI_KEY, KEY_READ)
            .and_then(|key| key.get_value::<String, _>(LAST_SIGNED_IN_VALUE))
            .ok()
    }

    /// The profile records of this PC or drive, in an order that does not change, with no folder
    /// listed twice. A record's path is a value like any other. On a drive, a folder with a hive
    /// and no record is kept too, since this may be the only chance to get at it, and is known
    /// by its name.
    fn read_listed_profiles(&self) -> Vec<ListedProfile> {
        let drive = self.offline.as_ref().map_or_else(
            || {
                system_root()
                    .to_string_lossy()
                    .chars()
                    .next()
                    .unwrap_or(DEFAULT_SYSTEM_DRIVE)
            },
            |hives| hives.map.system_drive,
        );
        let mut seen = BTreeSet::new();
        let mut listed = Vec::new();
        if let Ok(key) = self.open_hklm(PROFILE_LIST_KEY, KEY_READ) {
            let mut sids: Vec<String> = key
                .enum_keys()
                .flatten()
                .filter(|sid| is_person_sid(sid) && !sid.ends_with(BACKUP_PROFILE_SUFFIX))
                .collect();
            sids.sort();
            for sid in sids {
                let Ok(recorded) = key
                    .open_subkey_with_flags(&sid, KEY_READ)
                    .and_then(|profile| profile.get_value::<String, _>(PROFILE_IMAGE_PATH_VALUE))
                else {
                    continue;
                };
                let Some(logical) = expand_profile_path(&recorded, drive) else {
                    self.note_refused(Path::new(&recorded));
                    continue;
                };
                let Some(dir) = self.resolve_source_path(&logical) else {
                    continue;
                };
                let name = file_name_string(&logical);
                let no_hive = matches!(
                    fs::symlink_metadata(dir.join(NTUSER_DAT)),
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound
                );
                if !dir.is_dir()
                    || no_hive
                    || is_non_user_profile(&dir)
                    || !seen.insert(comparison_key(&resolved_location(&dir)))
                {
                    continue;
                }
                listed.push(ListedProfile {
                    id: sid.clone(),
                    sid: Some(sid),
                    name,
                    dir,
                });
            }
        }
        if let Some(hives) = &self.offline {
            for (name, dir, _) in enumerate_user_hives(&hives.map) {
                if seen.insert(comparison_key(&resolved_location(&dir))) {
                    listed.push(ListedProfile {
                        id: name.clone(),
                        sid: None,
                        name,
                        dir,
                    });
                }
            }
        }
        if listed.len() > MAX_ACCOUNTS {
            for profile in listed.drain(MAX_ACCOUNTS..) {
                self.note_refused(&profile.dir);
            }
        }
        listed
    }

    // ---- Known folders ------------------------------------------------------

    /// `Program Files` roots to search for install directories.
    pub fn program_files_dirs(&self) -> Vec<PathBuf> {
        match &self.offline {
            None => program_files_folders(),
            Some(hives) => ["Program Files", "Program Files (x86)"]
                .into_iter()
                .map(|name| hives.map.volume_root.join(name))
                .filter(|dir| self.may_read(dir))
                .collect(),
        }
    }

    /// The machine-wide `ProgramData` root.
    pub fn program_data(&self) -> Option<PathBuf> {
        match &self.offline {
            None => program_data_dir().map(Path::to_path_buf),
            Some(hives) => {
                Some(hives.map.volume_root.join("ProgramData")).filter(|dir| self.may_read(dir))
            }
        }
    }

    /// The shared `Public` profile (`C:\Users\Public`).
    pub fn public_dir(&self) -> Option<PathBuf> {
        match &self.offline {
            None => public_profile_dir()
                .map(Path::to_path_buf)
                .or_else(|| Some(PathBuf::from(r"C:\Users\Public"))),
            Some(hives) => {
                Some(hives.map.volume_root.join(r"Users\Public")).filter(|dir| self.may_read(dir))
            }
        }
    }

    /// The Windows directory (`%SystemRoot%`). On a drive, mounting checked that it is real.
    pub fn windows_dir(&self) -> PathBuf {
        match &self.offline {
            None => system_root().to_path_buf(),
            Some(hives) => hives.map.volume_root.join(WINDOWS_FOLDER),
        }
    }

    /// Every real user profile directory (`Users\<name>`), current user first
    /// when live. Excludes `Default`, `Public`, and the `All Users` junction. This is for telling
    /// what holds other things side by side, and for hints. Whose data to read is
    /// [`Self::user_hives`].
    pub fn user_profiles(&self) -> Vec<PathBuf> {
        match &self.offline {
            None => {
                let mut profiles = Vec::new();
                // Whoever this is about first, by what the admin-only registry says of them.
                if let Some(account) = signed_in_user().account() {
                    profiles.push(account.profile.clone());
                }
                let users_root = profiles_folder().or_else(|| {
                    profiles
                        .first()
                        .and_then(|profile| profile.parent())
                        .map(Path::to_path_buf)
                });
                if let Some(users_root) = users_root {
                    collect_profile_dirs(&users_root, &mut profiles, &Path::is_dir);
                }
                profiles
            }
            Some(hives) => {
                let mut profiles = Vec::new();
                let users_root = hives.map.volume_root.join("Users");
                if self.may_read(&users_root) {
                    collect_profile_dirs(&users_root, &mut profiles, &|path: &Path| {
                        self.is_dir(path)
                    });
                }
                for profile in &self.listed {
                    if !profiles.contains(&profile.dir) {
                        profiles.push(profile.dir.clone());
                    }
                }
                profiles
            }
        }
    }

    /// Whether a folder belongs to an account other than the one this scan is about. Its data is
    /// that person's own, however a value on the PC came to name it.
    pub fn is_in_another_profile(&self, path: &Path) -> bool {
        // Finding out looks through links. What may not be read is no one's to take anyway.
        if !self.may_read(path) || !location_can_be_told(path) {
            return true;
        }
        let path = resolved_location(path);
        self.folders_of_others()
            .iter()
            .any(|folder| path_is_within(&path, folder))
    }

    /// Whether a path is in the profile of the account this scan is about, by where it really is.
    pub fn is_in_the_accounts_profile(&self, path: &Path) -> bool {
        if !self.may_read(path) || has_parent_step(path) || !location_can_be_told(path) {
            return false;
        }
        let real = resolved_location(path);
        self.user_hives()
            .iter()
            .any(|user| path_is_within(&real, &resolved_location(&user.profile_dir)))
    }

    /// Whether what an app installed for one account says is its own could be. Its entry in that
    /// account's registry takes no rights to write, and can name anything an administrator can
    /// read, so it has to be in that account's profile, or that account has to own it. A drive
    /// keeps who owns what, though not on a volume with no permissions, and `C:\\Windows.old` or the
    /// recycle bin there are as much someone else's as a profile. Without elevation there is no one
    /// to be fooled, since this reads only what its own account can.
    pub fn is_the_accounts_own(&self, path: &Path) -> bool {
        if !is_elevated() {
            return self.may_read(path);
        }
        if self.is_in_the_accounts_profile(path) {
            return true;
        }
        if !self.may_read(path) || has_parent_step(path) || !location_can_be_told(path) {
            return false;
        }
        owner_of(&resolved_location(path)).is_some_and(|owner| {
            self.user_hives().iter().any(|user| {
                user.sid
                    .as_deref()
                    .is_some_and(|sid| sid.eq_ignore_ascii_case(&owner))
            })
        })
    }

    /// Where every account's folder is but those of who this scan is about.
    fn folders_of_others(&self) -> Vec<PathBuf> {
        let readers: Vec<String> = self
            .user_hives()
            .iter()
            .map(|user| comparison_key(&resolved_location(&user.profile_dir)))
            .collect();
        let mut profiles = self.user_profiles();
        profiles.extend(self.listed.iter().map(|profile| profile.dir.clone()));
        let mut folders: Vec<PathBuf> = profiles
            .iter()
            .map(|profile| resolved_location(profile))
            .filter(|real| !readers.contains(&comparison_key(real)))
            .collect();
        folders.sort();
        folders.dedup();
        folders
    }

    /// What a build asks about again just before it copies each file on this PC.
    pub fn other_accounts_folders<'a>(
        &self,
        app_folder: Option<&Path>,
        to_copy: impl Iterator<Item = &'a Path>,
    ) -> OtherAccountsFolders {
        if self.offline.is_some() {
            return OtherAccountsFolders::default();
        }
        let mut looked_at = BTreeSet::new();
        let mut links_seen = BTreeMap::new();
        for path in to_copy {
            for step in path.ancestors() {
                let key = comparison_key(step);
                if !looked_at.insert(key.clone()) {
                    break;
                }
                if fs::symlink_metadata(step)
                    .is_ok_and(|metadata| metadata.file_type().is_symlink())
                {
                    links_seen.insert(key, resolved_location(step));
                }
            }
        }
        OtherAccountsFolders {
            is_of_this_pc: true,
            folders: self.folders_of_others(),
            app_folder: app_folder.map(resolved_location),
            links_seen,
        }
    }

    /// Whether an account a scheduled task runs as is the one this scan is about.
    pub fn is_the_account(&self, account: &str) -> bool {
        let name = account.rsplit('\\').next().unwrap_or(account);
        self.user_hives().iter().any(|user| {
            user.sid
                .as_deref()
                .is_some_and(|sid| sid.eq_ignore_ascii_case(account))
                || user.name.eq_ignore_ascii_case(name)
                // A folder is `bob.PC` when a `bob` had one already.
                || user
                    .name
                    .split_once('.')
                    .is_some_and(|(stem, _)| stem.eq_ignore_ascii_case(name))
        })
    }

    /// Whether an account a scheduled task runs as is anyone but the one this scan is about, or
    /// one of Windows' own. That is theirs, along with what it was told to do. An account that
    /// cannot be told apart from a person counts as one: all it costs is a warning.
    pub fn is_another_persons_account(&self, account: &str) -> bool {
        if self.is_the_account(account) {
            return false;
        }
        let name = account.rsplit('\\').next().unwrap_or(account);
        if account
            .get(..4)
            .is_some_and(|start| start.eq_ignore_ascii_case("S-1-"))
        {
            return is_person_sid(account);
        }
        !is_windows_own_account_name(name)
    }

    // ---- Path re-rooting ----------------------------------------------------

    /// What a drive says about itself. `None` on this PC.
    pub fn volume_map(&self) -> Option<&VolumeMap> {
        self.offline.as_ref().map(|hives| &hives.map)
    }

    /// Where a path recorded on the source PC can be read: on this PC, what it says, and on a
    /// drive, where that is now, if it can be read at all. See [`VolumeMap::to_volume`] and
    /// [`Self::may_read`]. What it will not follow is listed in [`Self::warnings`].
    pub fn resolve_source_path(&self, recorded: &Path) -> Option<PathBuf> {
        let resolved = self.resolve_source_path_quietly(recorded);
        if resolved.is_none() {
            self.note_refused(recorded);
        }
        resolved
    }

    /// [`Self::resolve_source_path`], for going through what says it is elsewhere and is nothing to
    /// do with this scan, such as the rest of the Uninstall key, so that it is not listed as something
    /// of this app's that was not followed.
    pub fn resolve_source_path_quietly(&self, recorded: &Path) -> Option<PathBuf> {
        let read = as_windows_reads_it(recorded);
        let resolved = match &self.offline {
            None => read.filter(|read| self.may_read(read)),
            Some(hives) => read
                .and_then(|read| hives.map.to_volume(&read))
                .filter(|on_volume| self.may_read(on_volume)),
        };
        // An MSI component that keys on a folder says so by ending in a separator.
        let names_a_folder = recorded.to_string_lossy().ends_with(['\\', '/']);
        resolved.map(|path| if names_a_folder { path.join("") } else { path })
    }

    /// Whether a path found while scanning can be read. On this PC, if it is on a disk of this PC and
    /// written in ordinary names, since any account can make a real folder `x.` beside a link `x`,
    /// which is how a link gets past anything that looks at the name it was listed by. On a drive,
    /// see [`VolumeMap::may_read`], which every path is asked again just before reading it. That has
    /// to come first, since nearly any other look at a path follows a link.
    pub fn may_read(&self, path: &Path) -> bool {
        let Some(hives) = &self.offline else {
            return path.is_absolute()
                && is_on_a_local_disk(path)
                && path.components().all(|component| match component {
                    Component::Normal(name) => is_ordinary_name(name),
                    _ => true,
                });
        };
        match self.checked.lock() {
            Ok(mut checked) => hives.map.may_read_noting(path, &mut checked),
            Err(_) => hives.map.may_read(path),
        }
    }

    /// Whether this is a folder, with no link on a drive followed to find that out. Looking at a
    /// link to a share is enough to connect to it.
    pub fn is_dir(&self, path: &Path) -> bool {
        self.may_read(path) && path.is_dir()
    }

    /// Whether this is a file, with no link on a drive followed to find that out.
    pub fn is_file(&self, path: &Path) -> bool {
        self.may_read(path) && path.is_file()
    }

    /// What the PC itself calls a path that can be read here. It is the same path on this PC.
    pub fn logical_path(&self, path: &Path) -> PathBuf {
        self.volume_map()
            .and_then(|map| map.to_logical(path))
            .unwrap_or_else(|| path.to_path_buf())
    }

    /// Expand `%Variable%` as it read on the source PC. On this PC, what the environment says.
    /// On a drive, the folders any Windows has, on its Windows drive: its other variables are not
    /// known here, and this PC's mean nothing there, so they stay as they are and match nothing.
    pub fn expand_source_variables(&self, text: &str) -> String {
        crate::path_utils::expand_vars_with(text, &|name| self.source_variable(name))
    }

    fn source_variable(&self, name: &str) -> Option<String> {
        let lower = name.to_ascii_lowercase();
        if matches!(lower.as_str(), "userprofile" | "appdata" | "localappdata") {
            let user = self.user_hives().into_iter().next()?;
            let profile = self.logical_path(&user.profile_dir);
            let folder = match lower.as_str() {
                "appdata" => profile.join(r"AppData\Roaming"),
                "localappdata" => profile.join(r"AppData\Local"),
                _ => profile,
            };
            return Some(folder.display().to_string());
        }
        let Some(hives) = &self.offline else {
            return machine_variable(name).or_else(|| std::env::var(name).ok());
        };
        let drive = hives.map.system_drive;
        let folder = match lower.as_str() {
            "systemdrive" => return Some(format!("{drive}:")),
            "systemroot" | "windir" => WINDOWS_FOLDER,
            "programfiles" | "programw6432" => "Program Files",
            "programfiles(x86)" => "Program Files (x86)",
            "commonprogramfiles" | "commonprogramw6432" => r"Program Files\Common Files",
            "commonprogramfiles(x86)" => r"Program Files (x86)\Common Files",
            "programdata" | "allusersprofile" => "ProgramData",
            "public" => r"Users\Public",
            _ => return None,
        };
        Some(format!(r"{drive}:\{folder}"))
    }

    /// The name of the PC the app is on, and the version of Windows it runs.
    pub fn source_identity(&self) -> SourceIdentity {
        match &self.offline {
            Some(hives) => hives.identity.clone(),
            None => SourceIdentity {
                computer_name: std::env::var("COMPUTERNAME").ok(),
                windows_version: live_windows_version(),
                windows_build: Some(windows_build()).filter(|build| *build != 0),
                architecture_name: windows_architecture_name(),
            },
        }
    }
}

fn live_windows_version() -> Option<String> {
    // `/D`, or an account's `AutoRun` runs here as whoever is running this.
    hidden_command("cmd")
        .args(["/D", "/C", "ver"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn find_profile(listed: &[ListedProfile], id: &str) -> Option<usize> {
    listed
        .iter()
        .position(|profile| profile.id.eq_ignore_ascii_case(id))
}

/// A profile whose folder went wrong, which Windows renames and replaces.
const BACKUP_PROFILE_SUFFIX: &str = ".bak";

/// What a profile record means by its path, written `%SystemDrive%\Users\alice` or in full. No
/// other variable belongs there.
fn expand_profile_path(recorded: &str, system_drive: char) -> Option<PathBuf> {
    let trimmed = recorded.trim();
    let expanded = match trimmed.get(..SYSTEM_DRIVE_VARIABLE.len()) {
        Some(marker) if marker.eq_ignore_ascii_case(SYSTEM_DRIVE_VARIABLE) => {
            format!("{system_drive}:{}", &trimmed[SYSTEM_DRIVE_VARIABLE.len()..])
        }
        _ => trimmed.to_owned(),
    };
    (!expanded.is_empty() && !expanded.contains('%')).then(|| PathBuf::from(expanded))
}

fn comparison_key(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .trim_end_matches('\\')
        .to_ascii_lowercase()
}

fn loaded_hive(sid: &str) -> bool {
    RegKey::predef(HKEY_USERS)
        .open_subkey_with_flags(sid, KEY_READ)
        .is_ok()
}

/// A profile on this PC whose hive is already loaded, as a signed-in account's is.
fn loaded_user(profile: &ListedProfile) -> Option<UserHive> {
    let sid = profile.sid.as_deref().filter(|sid| loaded_hive(sid))?;
    Some(UserHive {
        id: profile.id.clone(),
        sid: profile.sid.clone(),
        name: profile.name.clone(),
        profile_dir: profile.dir.clone(),
        kind: UserHiveKind::Loaded {
            sid: sid.to_owned(),
        },
    })
}

// ---- Offline mounting -------------------------------------------------------

/// Whether a folder chosen as a drive is laid out as a Windows install. It takes a `Windows` and a
/// `Users` that are folders on it: looking through a link to find out could connect to a share.
pub fn looks_like_a_windows_drive(root: &Path) -> bool {
    ["Windows", "Users"].iter().all(|folder| {
        fs::symlink_metadata(root.join(folder))
            .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink())
    })
}

fn mount_machine_hives(volume_root: &Path, mounts: &mut HiveMounts) -> Result<OfflineHives> {
    let windows = volume_root.join(WINDOWS_FOLDER);
    let is_real_folder = fs::symlink_metadata(&windows)
        .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink());
    // Detect a Windows volume by its listable \Windows directory; the hive files
    // under \Windows\System32\config are ACL-protected and may not even stat
    // until they are read through backup privilege, so don't precheck them.
    if !is_real_folder {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} is not a Windows system drive (no \\Windows folder).",
            volume_root.display()
        )));
    }
    if path_is_within(
        &resolved_location(&windows),
        &resolved_location(system_root()),
    ) || path_is_within(
        &resolved_location(system_root()),
        &resolved_location(&windows),
    ) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} is this PC's own system drive. Choose This PC to read this PC.",
            volume_root.display()
        )));
    }
    let provisional = VolumeMap {
        volume_root: volume_root.to_path_buf(),
        system_drive: DEFAULT_SYSTEM_DRIVE,
    };
    let software_hive = volume_root.join(CONFIG_SOFTWARE);
    if !provisional.may_read(&software_hive) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} is not a Windows system drive: the way to its registry goes through a link.",
            volume_root.display()
        )));
    }

    // Copying and loading report the authoritative failure (missing hive, access denied, already
    // loaded) if the SOFTWARE hive cannot be mounted.
    let software_mount = mounts.load(SOFTWARE_LABEL, &software_hive)?;

    // The SYSTEM hive (services, drivers, machine environment) is desirable but
    // not fatal if unreadable; degrade to a default control set.
    let system_hive = volume_root.join(CONFIG_SYSTEM);
    let system_mount = if provisional.may_read(&system_hive) {
        match mounts.load(SYSTEM_LABEL, &system_hive) {
            Ok(mount) => Some(mount),
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
    let map = VolumeMap {
        volume_root: volume_root.to_path_buf(),
        system_drive: read_system_drive(&software_mount),
    };
    let identity = read_identity(&software_mount, system_mount.as_deref(), &control_set);
    Ok(OfflineHives {
        map,
        software_mount,
        system_mount,
        control_set,
        identity,
    })
}

/// The letter Windows was on, from where it says it is. It is only ever used to know which paths
/// are on this drive, so a false one says only that they are not.
fn read_system_drive(software_mount: &str) -> char {
    let root = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(
            join_mount(software_mount, CURRENT_VERSION_KEY_IN_HIVE),
            KEY_READ,
        )
        .and_then(|key| key.get_value::<String, _>("SystemRoot"))
        .unwrap_or_default();
    system_drive_of(&root)
}

fn system_drive_of(system_root: &str) -> char {
    let mut characters = system_root.chars();
    match (characters.next(), characters.next()) {
        (Some(letter), Some(':')) if letter.is_ascii_alphabetic() => letter.to_ascii_uppercase(),
        _ => DEFAULT_SYSTEM_DRIVE,
    }
}

/// Without the `Software` that is the hive's own root.
const CURRENT_VERSION_KEY_IN_HIVE: &str = r"Microsoft\Windows NT\CurrentVersion";
const COMPUTER_NAME_KEY_IN_CONTROL_SET: &str = r"Control\ComputerName\ComputerName";
const ENVIRONMENT_KEY_IN_CONTROL_SET: &str = r"Control\Session Manager\Environment";

/// What the drive's own registry says about its PC, rather than what this PC says about itself.
fn read_identity(
    software_mount: &str,
    system_mount: Option<&str>,
    control_set: &str,
) -> SourceIdentity {
    let computer_name = system_mount.and_then(|mount| {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(
                format!(r"{mount}\{control_set}\{COMPUTER_NAME_KEY_IN_CONTROL_SET}"),
                KEY_READ,
            )
            .and_then(|key| key.get_value::<String, _>("ComputerName"))
            .ok()
    });
    let architecture_name = system_mount.and_then(|mount| {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(
                format!(r"{mount}\{control_set}\{ENVIRONMENT_KEY_IN_CONTROL_SET}"),
                KEY_READ,
            )
            .and_then(|key| key.get_value::<String, _>("PROCESSOR_ARCHITECTURE"))
            .ok()
    });
    let version_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(
            join_mount(software_mount, CURRENT_VERSION_KEY_IN_HIVE),
            KEY_READ,
        )
        .ok();
    let build: Option<String> = version_key
        .as_ref()
        .and_then(|key| key.get_value("CurrentBuildNumber").ok());
    let version = version_key
        .as_ref()
        .zip(build.as_deref())
        .map(|(key, build)| {
            let number = |name: &str| key.get_value::<u32, _>(name).ok();
            windows_version_text(
                number("CurrentMajorVersionNumber").unwrap_or(10),
                number("CurrentMinorVersionNumber").unwrap_or(0),
                build,
                number("UBR").unwrap_or(0),
            )
        });
    SourceIdentity {
        computer_name: computer_name.map(|name| bounded_text(&name)),
        windows_version: version.map(|version| bounded_text(&version)),
        windows_build: build.and_then(|build| build.trim().parse().ok()),
        architecture_name: architecture_name.map(|name| bounded_text(&name)),
    }
}

/// In the form `ver` prints, which is what a build from this PC records.
fn windows_version_text(major: u32, minor: u32, build: &str, revision: u32) -> String {
    format!("Microsoft Windows [Version {major}.{minor}.{build}.{revision}]")
}

/// What goes in front of a person from a value on a drive: a line, of a sensible length.
fn bounded_text(text: &str) -> String {
    text.chars()
        .filter(|character| !character.is_control())
        .take(MAX_IDENTITY_CHARS)
        .collect()
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
                return Some(join_mount(
                    mount,
                    &format!(r"{}{}", self.control_set, after),
                ));
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
fn enumerate_user_hives(map: &VolumeMap) -> Vec<(String, PathBuf, PathBuf)> {
    let mut hives = Vec::new();
    let users_root = map.volume_root.join("Users");
    if !map.may_read(&users_root) {
        return hives;
    }
    let Ok(entries) = fs::read_dir(&users_root) else {
        return hives;
    };
    for entry in entries.flatten() {
        let profile_dir = entry.path();
        // Asked first, by name: `x ` can be a link and read as the real `x` beside it, until
        // something is asked for inside it.
        if !map.may_read(&profile_dir) {
            continue;
        }
        let is_real_folder = fs::symlink_metadata(&profile_dir)
            .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink());
        if !is_real_folder || is_non_user_profile(&profile_dir) {
            continue;
        }
        let ntuser = profile_dir.join(NTUSER_DAT);
        if map.may_read(&ntuser)
            && fs::symlink_metadata(&ntuser).is_ok_and(|metadata| metadata.is_file())
        {
            let name = profile_dir
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("user")
                .to_owned();
            hives.push((name, profile_dir, ntuser));
        }
    }
    hives.sort();
    hives
}

fn collect_profile_dirs(
    users_root: &Path,
    profiles: &mut Vec<PathBuf>,
    is_folder: &dyn Fn(&Path) -> bool,
) {
    let Ok(entries) = fs::read_dir(users_root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_folder(&path) && !is_non_user_profile(&path) && !profiles.contains(&path) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> VolumeMap {
        VolumeMap {
            volume_root: PathBuf::from(r"E:\"),
            system_drive: 'C',
        }
    }

    fn hives(control_set: &str) -> OfflineHives {
        OfflineHives {
            map: map(),
            software_mount: "WAM_1_SOFTWARE".to_owned(),
            system_mount: Some("WAM_1_SYSTEM".to_owned()),
            control_set: control_set.to_owned(),
            identity: SourceIdentity::default(),
        }
    }

    #[test]
    fn reroots_system_drive_paths_onto_the_volume() {
        assert_eq!(
            map().to_volume(Path::new(r"C:\Program Files\App\app.exe")),
            Some(PathBuf::from(r"E:\Program Files\App\app.exe"))
        );
        assert_eq!(
            map().to_volume(Path::new(r"c:\program files\app")),
            Some(PathBuf::from(r"E:\program files\app"))
        );
        assert_eq!(
            map().to_volume(Path::new(r"\\?\C:\Program Files\App")),
            Some(PathBuf::from(r"E:\Program Files\App"))
        );
    }

    #[test]
    fn reroot_is_idempotent_for_paths_already_on_the_volume() {
        assert_eq!(
            map().to_volume(Path::new(r"E:\Program Files\App")),
            Some(PathBuf::from(r"E:\Program Files\App"))
        );
    }

    #[test]
    fn a_value_on_a_drive_cannot_name_anything_off_it() {
        for recorded in [
            // Rooted, relative, and relative to a drive's current folder: this PC's.
            r"\Windows\System32\config\SAM",
            r"relative\path",
            r"..\..\Users\me\secret.txt",
            r"secret.txt",
            r"C:secret.txt",
            // Climbing, in every spelling Windows reads as climbing.
            r"C:\Program Files\..\..\secret.txt",
            r"C:\..\secret.txt",
            r"C:\a\.. \b",
            r"C:\a\.. .\b",
            r"C:\a\..\b",
            r"E:\a\..\..\b",
            // Another disk of that PC, and whatever has that letter here.
            r"D:\Games\App",
            // The network, which would be sent this account's password hash.
            r"\\attacker\share\app.exe",
            r"//attacker/share/app.exe",
            r"\\?\UNC\attacker\share\app.exe",
            r"\\.\C:\secret.txt",
            r"\\.\PhysicalDrive0",
            r"\\?\GLOBALROOT\Device\HarddiskVolume1\secret.txt",
            r"\??\C:\secret.txt",
            // Streams and devices.
            r"C:\Users\a\file.txt:hidden",
            r"C:\Users\a\NUL",
            r"C:\CON\x",
            r"C:\Users\a.\x",
            r"C:\Users\a \x",
            "C:\\Users\\a\u{1}b",
            "",
        ] {
            assert_eq!(map().to_volume(Path::new(recorded)), None, "{recorded}");
        }
    }

    #[test]
    fn a_volume_mounted_on_a_folder_cannot_be_climbed_out_of() {
        let map = VolumeMap {
            volume_root: PathBuf::from(r"C:\Mounted\Old PC"),
            system_drive: 'C',
        };
        assert_eq!(
            map.to_volume(Path::new(r"C:\Program Files\App")),
            Some(PathBuf::from(r"C:\Mounted\Old PC\Program Files\App"))
        );
        for recorded in [
            r"C:\..\..\Users\me\secret.txt",
            r"C:\Mounted\Old PC\..\..\secret.txt",
        ] {
            assert_eq!(map.to_volume(Path::new(recorded)), None, "{recorded}");
        }
    }

    #[test]
    fn a_path_is_told_as_its_own_pc_called_it() {
        assert_eq!(
            map().to_logical(Path::new(r"E:\Program Files\App\app.exe")),
            Some(PathBuf::from(r"C:\Program Files\App\app.exe"))
        );
        assert_eq!(
            map().to_logical(Path::new(r"e:\")),
            Some(PathBuf::from(r"C:\"))
        );
        assert_eq!(map().to_logical(Path::new(r"C:\Windows")), None);
        let elsewhere = VolumeMap {
            volume_root: PathBuf::from(r"F:\Images\Old"),
            system_drive: 'D',
        };
        assert_eq!(
            elsewhere.to_logical(Path::new(r"F:\Images\Old\Users\me")),
            Some(PathBuf::from(r"D:\Users\me"))
        );
        assert_eq!(
            elsewhere.to_volume(Path::new(r"D:\Users\me")),
            Some(PathBuf::from(r"F:\Images\Old\Users\me"))
        );
        assert_eq!(
            elsewhere.to_volume(Path::new(r"C:\Users\me")),
            None,
            "not the drive that PC ran from"
        );
    }

    #[test]
    fn nothing_past_a_link_on_a_drive_is_read() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let volume = work.path().join("volume");
        let outside = work.path().join("this PC");
        fs::create_dir_all(volume.join(r"Program Files\App\real"))?;
        fs::create_dir_all(&outside)?;
        fs::write(outside.join("secret.txt"), b"x")?;
        fs::write(volume.join(r"Program Files\App\real\ok.txt"), b"x")?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(volume.join(r"Program Files\App\linked"))
            .arg(&outside)
            .output()?;
        assert!(made.status.success());

        let map = VolumeMap {
            volume_root: volume.clone(),
            system_drive: 'C',
        };
        let app = volume.join(r"Program Files\App");
        assert!(map.may_read(&app.join(r"real\ok.txt")));
        assert!(map.may_read(&app.join("not there yet")));
        assert!(!map.may_read(&app.join("linked")));
        assert!(!map.may_read(&app.join(r"linked\secret.txt")));
        assert!(!map.may_read(&outside.join("secret.txt")));
        // Remembering what it looked at does not make it forget a link.
        let mut checked = HashSet::new();
        assert!(map.may_read_noting(&app.join(r"real\ok.txt"), &mut checked));
        assert!(!map.may_read_noting(&app.join(r"linked\secret.txt"), &mut checked));
        assert!(map.may_read_noting(&app.join(r"real\ok.txt"), &mut checked));
        Ok(())
    }

    #[test]
    fn a_folder_and_a_link_that_differ_only_in_case_are_each_looked_at() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let volume = work.path().join("volume");
        let tree = volume.join("Tree");
        let outside = work.path().join("this PC");
        fs::create_dir_all(tree.join("Data"))?;
        fs::create_dir_all(&outside)?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(tree.join("Link"))
            .arg(&outside)
            .output()?;
        assert!(made.status.success());
        let map = VolumeMap {
            volume_root: volume,
            system_drive: 'C',
        };
        // What is checked is remembered as written, so that a link spelled like a folder that
        // was looked at is looked at too. A case-sensitive folder takes more than this can make.
        let mut checked = HashSet::new();
        assert!(map.may_read_noting(&tree.join("Data"), &mut checked));
        assert!(checked.iter().any(|key| key.ends_with(r"Tree\Data")));
        assert!(!checked.iter().any(|key| key.ends_with(r"tree\data")));
        assert!(!map.may_read_noting(&tree.join("Link"), &mut checked));
        Ok(())
    }

    #[test]
    fn a_task_for_another_person_is_told_from_one_for_the_account_being_read() {
        let mut system = SystemContext::with_offline(hives("ControlSet001"));
        system.users = Users::Fixed(vec![UserHive {
            id: "S-1-5-21-1-2-3-1001".to_owned(),
            sid: Some("S-1-5-21-1-2-3-1001".to_owned()),
            name: "alice".to_owned(),
            profile_dir: PathBuf::from(r"E:\Users\alice"),
            kind: UserHiveKind::Loaded {
                sid: "S-1-5-21-1-2-3-1001".to_owned(),
            },
        }]);
        system.listed = vec![ListedProfile {
            id: "S-1-5-21-1-2-3-1002".to_owned(),
            sid: Some("S-1-5-21-1-2-3-1002".to_owned()),
            name: "bob".to_owned(),
            dir: PathBuf::from(r"E:\Users\bob"),
        }];
        for own_or_shared in [
            "S-1-5-21-1-2-3-1001",
            "s-1-5-21-1-2-3-1001",
            "alice",
            r"OLD-PC\Alice",
            "S-1-5-18",
            "SYSTEM",
            r"NT AUTHORITY\LocalService",
            "S-1-5-32-545",
            "Users",
        ] {
            assert!(
                !system.is_another_persons_account(own_or_shared),
                "{own_or_shared}"
            );
        }
        for someone_else in [
            "S-1-5-21-1-2-3-1002",
            "S-1-5-21-9-9-9-1001",
            "S-1-12-1-1-2-3-4",
            "bob",
            r"OLD-PC\BOB",
            r"CORP\carol",
        ] {
            assert!(
                system.is_another_persons_account(someone_else),
                "{someone_else}"
            );
        }
    }

    const DEEPEST_FOR_THIS_TEST: usize = 8;

    #[test]
    fn a_name_windows_would_read_as_another_is_not_ordinary() {
        use std::os::windows::ffi::OsStringExt;
        for name in ["x.", "x ", "x..", "x. .", "CON", "nul", "a:b", "a\\b"] {
            assert!(!is_ordinary_name(OsStr::new(name)), "{name}");
        }
        assert!(
            !is_ordinary_name(&std::ffi::OsString::from_wide(&[0x78, 0xD800])),
            "a name that is not text"
        );
        for name in ["x", "x.y", ".x", "a b", "Größe", "console"] {
            assert!(is_ordinary_name(OsStr::new(name)), "{name}");
        }
    }

    #[test]
    fn a_real_folder_named_for_a_link_with_a_dot_is_asked_about_as_the_link() -> std::io::Result<()>
    {
        let work = tempfile::tempdir()?;
        let volume = work.path().join("volume");
        let outside = work.path().join("this PC");
        fs::create_dir_all(&volume)?;
        fs::create_dir_all(&outside)?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(volume.join("x"))
            .arg(&outside)
            .output()?;
        assert!(made.status.success());
        let twin = volume.join("x.");
        fs::create_dir(format!(r"\\?\{}", twin.display()))?;
        let map = VolumeMap {
            volume_root: volume,
            system_drive: 'C',
        };
        assert!(!map.may_read(&twin));
        assert!(!map.may_read(&twin.join("y")));
        Ok(())
    }

    #[test]
    fn what_a_folder_leads_to_is_asked_about_again_whatever_it_says_it_is() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let bob = work.path().join("bob");
        let mine = work.path().join("mine");
        fs::create_dir_all(&bob)?;
        fs::create_dir_all(mine.join("app"))?;
        let swapped = mine.join("app").join("swapped");
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&swapped)
            .arg(&bob)
            .output()?;
        assert!(made.status.success());
        let others = OtherAccountsFolders {
            is_of_this_pc: true,
            folders: vec![resolved_location(&bob)],
            app_folder: Some(resolved_location(&mine.join("app"))),
            links_seen: BTreeMap::new(),
        };
        assert!(others.hold(&swapped.join("secret.txt")));
        assert!(others.hold(&bob.join("secret.txt")));
        assert!(!others.hold(&mine.join(r"app\own.txt")));
        let in_theirs = OtherAccountsFolders {
            is_of_this_pc: true,
            folders: vec![resolved_location(&bob)],
            app_folder: Some(resolved_location(&bob.join("Apps"))),
            links_seen: BTreeMap::new(),
        };
        assert!(
            !in_theirs.hold(&bob.join(r"Apps\x.exe")),
            "where the app itself was found to be"
        );
        assert!(in_theirs.hold(&bob.join("letter.txt")));
        assert!(!OtherAccountsFolders::default().hold(&bob.join("secret.txt")));

        // A link to anywhere at all that was not there when it was scanned, and one that was.
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)?;
        let moved = mine.join("moved");
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&moved)
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());
        let scanned_before = OtherAccountsFolders {
            is_of_this_pc: true,
            ..OtherAccountsFolders::default()
        };
        assert!(scanned_before.hold(&moved.join("x.txt")), "it is new");
        let system = SystemContext::live();
        let moved_file = moved.join("x.txt");
        let scanned_after =
            system.other_accounts_folders(None, std::iter::once(moved_file.as_path()));
        assert!(!scanned_after.hold(&moved_file), "it was there");

        // The same link, leading somewhere else now.
        let somewhere_else = work.path().join("somewhere else");
        fs::create_dir_all(&somewhere_else)?;
        fs::remove_dir(&moved)?;
        let remade = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&moved)
            .arg(&somewhere_else)
            .output()?;
        assert!(remade.status.success());
        assert!(scanned_after.hold(&moved_file), "it leads somewhere else");
        Ok(())
    }

    #[test]
    fn on_this_pc_a_name_windows_would_read_as_another_is_not_looked_at() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)?;
        fs::write(elsewhere.join("secret.txt"), b"x")?;
        let app = work.path().join("app");
        fs::create_dir_all(&app)?;
        fs::write(app.join("own.txt"), b"x")?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(app.join("x"))
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());
        fs::create_dir(format!(r"\\?\{}.", app.join("x").display()))?;

        let system = SystemContext::live();
        assert!(!system.may_read(&app.join("x.")));
        assert!(!system.may_read(&app.join("x ")));
        assert!(!system.may_read(&app.join(r"x.\secret.txt")));
        assert!(system.may_read(&app.join("own.txt")));
        let found: Vec<String> =
            crate::path_utils::walk_asking(&app, DEEPEST_FOR_THIS_TEST, |path, _| {
                system.may_read(path)
            })
            .flatten()
            .filter(|entry| entry.file_type.is_file())
            .map(|entry| file_name_string(&entry.path))
            .collect();
        assert_eq!(found, ["own.txt"], "not what is past the link");
        Ok(())
    }

    #[test]
    fn a_recorded_path_is_read_as_windows_would_read_it() {
        let read = |text: &str| as_windows_reads_it(Path::new(text));
        assert_eq!(
            read(r"C:\Program Files\Acme, Inc.\App"),
            Some(PathBuf::from(r"C:\Program Files\Acme, Inc\App"))
        );
        assert_eq!(read(r"C:\a. .\b "), Some(PathBuf::from(r"C:\a\b")));
        assert_eq!(read(r"C:\a\b.txt"), Some(PathBuf::from(r"C:\a\b.txt")));
        assert_eq!(
            read(r"C:\a\..\b"),
            Some(PathBuf::from(r"C:\a\..\b")),
            "a real parent step stays, for what looks for one"
        );
        for spelled in [r"C:\a\.. \b", r"C:\a\...\b", r"C:\a\ \b", r"C:\a\. "] {
            assert_eq!(read(spelled), None, "{spelled}");
        }
        let system = SystemContext::live();
        assert_eq!(
            system.resolve_source_path(Path::new(r"C:\Windows.\System32")),
            Some(PathBuf::from(r"C:\Windows\System32"))
        );
        // What says it is a folder still does, here and on a drive.
        assert_eq!(
            system.resolve_source_path(Path::new(r"C:\Windows\System32\")),
            Some(PathBuf::from(r"C:\Windows\System32\"))
        );
        let on_a_drive = SystemContext::with_offline(hives("ControlSet001"));
        let folder = on_a_drive.resolve_source_path(Path::new(r"C:\Program Files\App\"));
        assert!(
            folder
                .as_ref()
                .is_some_and(|path| path.to_string_lossy().ends_with(r"Program Files\App\")),
            "{folder:?}"
        );
    }

    #[test]
    fn on_this_pc_a_path_that_is_not_on_a_disk_here_is_not_looked_at() {
        let system = SystemContext::live();
        for path in [
            r"\\attacker\share\x",
            r"\\localhost\C$\Users\bob",
            r"\??\UNC\attacker\share",
        ] {
            let path = Path::new(path);
            assert!(!system.may_read(path));
            assert!(!system.is_dir(path));
            assert!(!system.is_file(path));
            assert_eq!(system.resolve_source_path(path), None);
            assert!(system.is_in_another_profile(path));
        }
        assert!(system.may_read(Path::new(r"C:\Windows")));
        assert!(
            system
                .warnings()
                .iter()
                .any(|warning| warning.code == "network_paths_not_followed")
        );
    }

    #[test]
    fn a_drive_has_to_have_real_folders_to_be_taken_for_a_windows_install() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        assert!(!looks_like_a_windows_drive(work.path()));
        fs::create_dir(work.path().join("Users"))?;
        assert!(!looks_like_a_windows_drive(work.path()));
        let elsewhere = work.path().join("elsewhere");
        fs::create_dir(&elsewhere)?;
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(work.path().join("Windows"))
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());
        assert!(!looks_like_a_windows_drive(work.path()), "that is a link");
        fs::remove_dir(work.path().join("Windows"))?;
        fs::create_dir(work.path().join("Windows"))?;
        assert!(looks_like_a_windows_drive(work.path()));
        Ok(())
    }

    #[test]
    fn only_a_mount_this_app_named_is_taken_for_one() {
        for ours in [
            "WAM_abc123_SOFTWARE",
            "WAM_abc123_SYSTEM",
            "WAM_abc123_USER_0",
            "WAM_abc123_CLASSES_12",
            // What older versions named them, which a crash of one can have left.
            "WAM_4242_SOFTWARE",
        ] {
            assert!(run_of_mount(ours).is_some(), "{ours}");
        }
        assert_eq!(run_of_mount("WAM_abc123_USER_0"), Some("abc123"));
        for other in [
            "SOFTWARE",
            "SYSTEM",
            "WAM",
            "WAM_",
            "WAM_abc123",
            "WAM_abc123_",
            "WAM_abc123_OTHER",
            "WAM_abc123_USER_",
            "WAM_abc123_USER_x",
            "WAM_abc 123_SOFTWARE",
            "WAM__SOFTWARE",
            "WAM_a_b_SOFTWARE",
            "wam_abc123_SOFTWARE",
            "OTHER_abc123_SOFTWARE",
        ] {
            assert_eq!(run_of_mount(other), None, "{other}");
        }
    }

    #[test]
    fn a_lease_says_whether_its_run_is_alive() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let lease = work.path().join(LEASE_FILE);
        assert!(!lease_is_held(&lease), "nothing there");
        let held = OpenOptions::new()
            .write(true)
            .create_new(true)
            .share_mode(0)
            .open(&lease)?;
        assert!(lease_is_held(&lease));
        drop(held);
        assert!(!lease_is_held(&lease), "its run has gone");
        Ok(())
    }

    #[test]
    fn a_profile_record_is_read_as_a_path_on_its_pc() {
        assert_eq!(
            expand_profile_path(r"%SystemDrive%\Users\alice", 'D'),
            Some(PathBuf::from(r"D:\Users\alice"))
        );
        assert_eq!(
            expand_profile_path(r"C:\Users\bob", 'D'),
            Some(PathBuf::from(r"C:\Users\bob"))
        );
        assert_eq!(expand_profile_path(r"%USERPROFILE%\x", 'C'), None);
        assert_eq!(expand_profile_path("   ", 'C'), None);
    }

    #[test]
    fn the_windows_drive_is_read_from_what_it_says_and_nothing_else() {
        assert_eq!(system_drive_of(r"D:\WINDOWS"), 'D');
        assert_eq!(system_drive_of(r"c:\Windows"), 'C');
        for odd in ["", r"\\server\share", "1:", "Windows", r"\Windows"] {
            assert_eq!(system_drive_of(odd), DEFAULT_SYSTEM_DRIVE, "{odd}");
        }
    }

    #[test]
    fn a_drives_windows_version_reads_as_a_builds_does() {
        assert_eq!(
            windows_version_text(10, 0, "26200", 7171),
            "Microsoft Windows [Version 10.0.26200.7171]"
        );
    }

    #[test]
    fn what_a_drive_calls_itself_is_kept_to_a_line() {
        assert_eq!(bounded_text("OLD-PC\r\nmore"), "OLD-PCmore");
        assert_eq!(
            bounded_text(&"x".repeat(1000)).chars().count(),
            MAX_IDENTITY_CHARS
        );
    }

    #[test]
    fn maps_software_logical_paths_onto_the_software_mount() {
        assert_eq!(
            hives("ControlSet001").map_hklm(r"Software\Microsoft\Windows\CurrentVersion\Uninstall"),
            Some(r"WAM_1_SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall".to_owned())
        );
    }

    #[test]
    fn maps_current_control_set_to_the_active_set() {
        assert_eq!(
            hives("ControlSet002").map_hklm(r"SYSTEM\CurrentControlSet\Services"),
            Some(r"WAM_1_SYSTEM\ControlSet002\Services".to_owned())
        );
    }

    #[test]
    fn a_users_classes_are_read_from_where_they_really_are() {
        let mapped =
            |logical: &str| mounted_user_key("WAM_1_USER_0", Some("WAM_1_CLASSES_0"), logical);
        assert_eq!(mapped(r"Software\Classes"), "WAM_1_CLASSES_0");
        assert_eq!(
            mapped(r"SOFTWARE\CLASSES\CLSID\{X}"),
            r"WAM_1_CLASSES_0\CLSID\{X}"
        );
        assert_eq!(mapped(r"Software\Vendor"), r"WAM_1_USER_0\Software\Vendor");
        assert_eq!(
            mapped(r"Software\ClassesOther"),
            r"WAM_1_USER_0\Software\ClassesOther"
        );
        // With none of its own to read, it is only what the hive itself has.
        assert_eq!(
            mounted_user_key("WAM_1_USER_0", None, r"Software\Classes\.x"),
            r"WAM_1_USER_0\Software\Classes\.x"
        );
    }

    #[test]
    fn strip_prefix_requires_whole_segment() {
        assert_eq!(
            strip_prefix_ci("Software\\X", "Software"),
            Some("\\X".to_owned())
        );
        assert_eq!(strip_prefix_ci("Software", "Software"), Some(String::new()));
        // Must not match a partial segment like "Soft".
        assert_eq!(strip_prefix_ci("SoftwareX\\Y", "Software"), None);
    }

    #[test]
    fn a_context_for_this_pc_needs_nothing_to_be_run_to_exist() {
        let system = SystemContext::live();
        assert!(!system.is_offline());
        assert!(system.volume_map().is_none());
        assert!(system.warnings().is_empty());
        assert!(system.may_read(Path::new(r"C:\Windows")));
        assert_eq!(
            system.logical_path(Path::new(r"C:\Windows")),
            PathBuf::from(r"C:\Windows")
        );
        assert_eq!(
            system.resolve_source_path(Path::new(r"C:\Windows")),
            Some(PathBuf::from(r"C:\Windows"))
        );
    }

    /// The scan of a drive uses a stand-in here for its registry's mount, so this needs no
    /// elevation. What it can see is what its path rules allow.
    #[test]
    fn a_drive_names_only_what_is_on_it_and_says_what_it_would_not_follow() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let mut offline = hives("ControlSet001");
        offline.map.volume_root = work.path().to_path_buf();
        let system = SystemContext::with_offline(offline);
        let on_volume = work.path().join(r"Program Files\App");
        assert_eq!(
            system.resolve_source_path(Path::new(r"C:\Program Files\App")),
            Some(on_volume.clone())
        );
        assert_eq!(
            system.logical_path(&on_volume),
            PathBuf::from(r"C:\Program Files\App")
        );
        assert_eq!(system.resolve_source_path(Path::new(r"\Windows\x")), None);
        assert_eq!(
            system.resolve_source_path(Path::new(r"\\attacker\x\y")),
            None
        );
        let warnings = system.warnings();
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert!(warnings[0].message.contains(r"\\attacker\x\y"));
        assert_eq!(warnings[0].source, None, "a value does not go in a log");
        Ok(())
    }

    #[test]
    fn variables_on_a_drive_are_its_own_and_not_this_pcs() {
        let system = SystemContext::with_offline(hives("ControlSet001"));
        assert_eq!(
            system.expand_source_variables(r"%ProgramFiles%\App\app.exe"),
            r"C:\Program Files\App\app.exe"
        );
        assert_eq!(
            system.expand_source_variables(r"%SystemRoot%\System32\x.exe"),
            r"C:\Windows\System32\x.exe"
        );
        assert_eq!(system.expand_source_variables("%TEMP%"), "%TEMP%");
        assert_eq!(
            system.expand_source_variables("%USERPROFILE%"),
            "%USERPROFILE%"
        );
    }
}
