//! The record an install keeps of what it changes, so that it can be undone.
//!
//! Before each change, the state it is about to replace goes in this journal, and files it
//! is about to overwrite go in its backups. If the install fails, is cancelled, or never
//! finishes, what is in here is enough to put everything back, and later the same record is
//! what an uninstall works from. [`crate::rollback`] does the undoing.
//!
//! Each install has a folder at `<state root>\installs\<id>`, and the state root on a real PC
//! is `%ProgramData%\WinAppMigrator`, a fixed place only administrators can write to. What
//! undoing does runs elevated and is decided by what is in here, so nothing is ever read from
//! a path a caller supplies, and each level of the folder is checked before it is trusted. Its
//! base is passed in only so tests can use temp folders.
//!
//! A journal can hold registry and environment data as it was, which can include secrets, so
//! it stays closed to everyone else, and none of it is ever logged.

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use winreg::RegValue;
use winreg::enums::RegType;

use crate::archive::EmbeddedPackage;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{first_link_to, hold_root, link_refusal};
use crate::hashing::hex_lower;
use crate::models::{EnvironmentScope, RegistryHive};
use crate::path_utils::{
    current_user_sid, program_data_dir, signed_in_user, strip_prefix_ignore_case,
    without_verbatim_prefix,
};
use crate::privilege::{
    ProtectedFolder, allow_users_to_start, confirm_protected_folder, ensure_protected_folder,
    is_elevated, refuse_what_others_put_in,
};
use crate::registry_target::{hex_decode, registry_root};
use crate::restore_policy::paths_equal;

const STATE_FOLDER: &str = "WinAppMigrator";
const INSTALLS_FOLDER: &str = "installs";
/// Where a package is unpacked when it can be. Only administrators can get in, from the moment
/// anything is made in it, so nothing can be holding what is made there.
const STAGING_FOLDER: &str = "staging";
const JOURNAL_FILE: &str = "journal.jsonl";
/// Held by whatever is installing or uninstalling, so there is only ever one.
const BUSY_FILE: &str = "busy.lock";
const BACKUPS_FOLDER: &str = "backups";
const UNINSTALLER_FILE: &str = "uninstall.exe";
/// What an install's id starts with. An id is a folder name, and this keeps it from ever
/// spelling a device.
const ID_PREFIX: &str = "wam-";
const MAX_ID_CHARS: usize = 32;
const SCHEMA_VERSION: u32 = 1;
/// How many new files can be recorded before the journal is flushed to disk. A forgotten new
/// file only leaves a stray file behind after a power cut, and flushing for each one would
/// take minutes on a large app. What could not otherwise be put back is flushed every time.
const NEW_ITEMS_BETWEEN_FLUSHES: u32 = 256;
const FILE_SHARE_READ: u32 = 0x0000_0001;
const ERROR_SHARING_VIOLATION: i32 = 32;
const ERROR_NOT_SAME_DEVICE: i32 = 17;
/// A backup that is the file itself, moved here with its permissions and everything else.
const MOVED_BACKUP_SUFFIX: &str = "moved";
/// A backup copied from another drive. It has this folder's permissions, not its own.
const COPIED_BACKUP_SUFFIX: &str = "copied";
const UNFINISHED_BACKUP_SUFFIX: &str = "unfinished";

/// The flag that makes the stub an uninstaller, followed by an install's id.
pub const UNINSTALL_FLAG: &str = "--uninstall";
/// With [`UNINSTALL_FLAG`], run with no window.
pub const QUIET_FLAG: &str = "--quiet";
/// What the folder an installer unpacks its package into is called.
pub const STAGING_PREFIX: &str = "winappmigrator-install-";

/// `%ProgramData%\WinAppMigrator`, by way of the admin-only registry.
pub fn machine_state_root() -> Option<PathBuf> {
    program_data_dir().map(|folder| folder.join(STATE_FOLDER))
}

/// True for what could be an install's id, which is checked before it goes near a path.
pub fn is_valid_install_id(id: &str) -> bool {
    id.len() <= MAX_ID_CHARS
        && id.strip_prefix(ID_PREFIX).is_some_and(|rest| {
            !rest.is_empty()
                && rest
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric())
        })
}

/// What to keep or remove when an app is uninstalled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemKind {
    Program,
    /// Kept unless someone asks for it to be removed.
    UserData,
    /// Kept unless someone asks for it to be removed.
    License,
    /// The app's entry in Settings. It goes last, so that an uninstall that could not finish
    /// can be run again from there.
    UninstallEntry,
}

/// A registry value as it was, in the form it was read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredValue {
    pub kind: u32,
    pub data_hex: String,
}

impl StoredValue {
    pub(crate) fn from_reg(value: &RegValue<'_>) -> Self {
        Self {
            kind: reg_type_number(&value.vtype),
            data_hex: hex_lower(&value.bytes),
        }
    }

    pub(crate) fn to_reg(&self) -> std::result::Result<RegValue<'static>, String> {
        let vtype = reg_type_from_number(self.kind)
            .ok_or_else(|| format!("unknown registry value type {}", self.kind))?;
        let bytes = hex_decode(&self.data_hex).map_err(str::to_owned)?;
        Ok(RegValue {
            bytes: bytes.into(),
            vtype,
        })
    }
}

fn reg_type_number(kind: &RegType) -> u32 {
    match kind {
        RegType::REG_NONE => 0,
        RegType::REG_SZ => 1,
        RegType::REG_EXPAND_SZ => 2,
        RegType::REG_BINARY => 3,
        RegType::REG_DWORD => 4,
        RegType::REG_DWORD_BIG_ENDIAN => 5,
        RegType::REG_LINK => 6,
        RegType::REG_MULTI_SZ => 7,
        RegType::REG_RESOURCE_LIST => 8,
        RegType::REG_FULL_RESOURCE_DESCRIPTOR => 9,
        RegType::REG_RESOURCE_REQUIREMENTS_LIST => 10,
        RegType::REG_QWORD => 11,
    }
}

fn reg_type_from_number(number: u32) -> Option<RegType> {
    Some(match number {
        0 => RegType::REG_NONE,
        1 => RegType::REG_SZ,
        2 => RegType::REG_EXPAND_SZ,
        3 => RegType::REG_BINARY,
        4 => RegType::REG_DWORD,
        5 => RegType::REG_DWORD_BIG_ENDIAN,
        6 => RegType::REG_LINK,
        7 => RegType::REG_MULTI_SZ,
        8 => RegType::REG_RESOURCE_LIST,
        9 => RegType::REG_FULL_RESOURCE_DESCRIPTOR,
        10 => RegType::REG_RESOURCE_REQUIREMENTS_LIST,
        11 => RegType::REG_QWORD,
        _ => return None,
    })
}

/// How a service or driver was set up before an install changed it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub image_path: String,
    pub start: Option<u32>,
    pub delayed: bool,
    pub object_name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub service_type: Option<u32>,
}

/// One change an install is about to make, with what it replaces. Written before the change,
/// so a change that never happened can be in here, and undoing has to expect that.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "snake_case")]
pub enum Change {
    /// A folder that paths below are checked against, and where it really was. A link
    /// planted at or above it later on would change the second of these.
    Root {
        path: PathBuf,
        real: String,
    },
    FolderCreated {
        path: PathBuf,
        root: PathBuf,
        item: ItemKind,
    },
    /// A file about to be written. `backup` is set if one was there.
    File {
        path: PathBuf,
        root: PathBuf,
        item: ItemKind,
        backup: Option<u64>,
        sha256: Option<String>,
        partial: Option<PathBuf>,
    },
    /// What a file written by something else came out as.
    FileWritten {
        path: PathBuf,
        sha256: String,
    },
    /// A file that was not left changed after all.
    FileUndone {
        path: PathBuf,
    },
    RegistryKeyCreated {
        hive: RegistryHive,
        key: String,
        item: ItemKind,
    },
    RegistryValue {
        hive: RegistryHive,
        key: String,
        name: String,
        prior: Option<StoredValue>,
        written: StoredValue,
        item: ItemKind,
    },
    EnvironmentValue {
        scope: EnvironmentScope,
        name: String,
        prior: Option<StoredValue>,
        written: StoredValue,
    },
    /// Entries added to a `Path`. Only these come back out, since the rest belongs to others.
    PathEntries {
        scope: EnvironmentScope,
        name: String,
        added: Vec<String>,
        prior: Option<StoredValue>,
        written: StoredValue,
    },
    Service {
        name: String,
        driver: bool,
        binary: PathBuf,
        arguments: Option<String>,
        prior: Option<ServiceConfig>,
    },
    TaskFolderCreated {
        path: String,
    },
    Task {
        task_path: String,
        task_name: String,
        prior_xml: Option<String>,
        written_xml: Option<String>,
    },
    /// How Windows wrote a task back after registering it.
    TaskWritten {
        task_path: String,
        task_name: String,
        xml: String,
    },
    /// A rule about to be added to Windows Firewall. What identifies it is known before it is
    /// added, so what was cut off right after can still be found.
    FirewallRule {
        name: String,
        direction: String,
        program: PathBuf,
        /// What tells it from another of that name, way, and program.
        #[serde(default)]
        protocol: Option<String>,
        #[serde(default)]
        local_ports: Vec<String>,
        #[serde(default)]
        remote_ports: Vec<String>,
        /// What it does, for telling whether it is still as it was added.
        fingerprint: String,
    },
    /// A runtime about to be installed. It may then be in use by anything, so it is only
    /// reported.
    Prerequisite {
        name: String,
    },
    /// A runtime that was not installed after all.
    PrerequisiteSkipped {
        name: String,
    },
    /// All of it was undone but for what is under these folders, whose drive was not connected,
    /// and the app's entry in Settings if that is to be finished from there. What was undone
    /// must not be undone again: another install may have written the same thing since.
    OnlyLeftOn {
        roots: Vec<PathBuf>,
        keeps_entry: bool,
    },
}

/// What a journal is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalHeader {
    pub schema: u32,
    pub id: String,
    pub app_name: String,
    pub target_root: PathBuf,
    /// The account whose per-user hive this install wrote to, who was signed in at the time.
    pub user_sid: Option<String>,
    pub started_at_utc: String,
    /// Left behind if the installer is killed. Only ever a folder it made.
    pub staging: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Record {
    Header(JournalHeader),
    Change(Change),
    Committed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Durability {
    /// Forgetting this would lose something that cannot be got back.
    Now,
    /// Forgetting this would only leave something new behind.
    Soon,
}

/// A journal being written. It is held open with no writers allowed, and the system closes it
/// however the process ends, which is how another run tells a journal in use from one whose
/// install never finished.
pub struct Journal {
    id: String,
    folder: PathBuf,
    file: File,
    _folder_hold: File,
    next_backup: u64,
    since_flush: u32,
    /// A record could not be written whole, so what comes next would join it and be lost.
    broken: bool,
    known_roots: HashSet<String>,
    /// So that nothing above each of them can change. See [`Journal::note_root`].
    root_holds: Vec<File>,
    known_folders: HashSet<String>,
    known_keys: HashSet<String>,
    known_task_folders: HashSet<String>,
    uninstaller: Option<PathBuf>,
}

fn folded(path: &Path) -> String {
    path.to_string_lossy()
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_ascii_lowercase()
}

/// Where a folder really is, with every link on the way followed.
pub(crate) fn real_path(path: &Path) -> io::Result<String> {
    let actual = fs::canonicalize(path)?;
    Ok(without_verbatim_prefix(&actual.to_string_lossy()).to_owned())
}

/// Where a package can be unpacked in `state_root`, made if it has to be and only if this is
/// elevated. Without elevation there is no boundary to keep, and nowhere safe to make this.
pub fn admin_only_staging_root(state_root: &Path) -> io::Result<PathBuf> {
    if !is_elevated() {
        return Err(io::Error::other("not elevated"));
    }
    let staging = staging_root_in(state_root);
    ensure_protected_folder(state_root)?;
    refuse_what_others_put_in(state_root)?;
    ensure_protected_folder(&staging)?;
    Ok(staging)
}

/// Where [`admin_only_staging_root`] would be, whether or not it is there.
pub fn staging_root_in(state_root: &Path) -> PathBuf {
    state_root.join(STAGING_FOLDER)
}

/// Take away what a preview made to unpack in, as it changes nothing, and it is no harm to anything
/// else: neither can go while anything is in it.
pub fn remove_staging_root_if_unused(state_root: &Path) {
    let _ = fs::remove_dir(staging_root_in(state_root));
    let _ = fs::remove_dir(state_root);
}

/// Make sure the state root and the folder of installs are there and are what they should be.
fn prepare_state_folders(state_root: &Path) -> Result<PathBuf> {
    if !is_elevated()
        && machine_state_root().is_some_and(|machine| paths_equal(&machine, state_root))
    {
        return Err(WinAppMigratorError::InvalidInput(
            "installing needs administrator rights, to keep its record where others cannot \
             change it"
                .to_owned(),
        ));
    }
    let installs = state_root.join(INSTALLS_FOLDER);
    for folder in [state_root, installs.as_path()] {
        ensure_protected_folder(folder).map_err(|error| WinAppMigratorError::io(folder, error))?;
    }
    for folder in [state_root, installs.as_path()] {
        refuse_what_others_put_in(folder)
            .map_err(|error| WinAppMigratorError::io(folder, error))?;
    }
    Ok(installs)
}

impl Journal {
    /// Start a journal in a new folder of its own. A failure means nothing has changed yet,
    /// and that the install should not go ahead, since it could not be undone.
    pub(crate) fn begin(
        state_root: &Path,
        app_name: &str,
        target_root: &Path,
        staging: Option<&Path>,
    ) -> Result<Self> {
        let installs = prepare_state_folders(state_root)?;
        let (folder, hold) = ProtectedFolder::new_in(&installs, ID_PREFIX)
            .map_err(|error| WinAppMigratorError::io(&installs, error))?
            .keep();
        let started = Self::start_in(folder.clone(), hold, app_name, target_root, staging);
        if started.is_err() {
            let _ = fs::remove_dir_all(&folder);
        }
        started
    }

    fn start_in(
        folder: PathBuf,
        hold: File,
        app_name: &str,
        target_root: &Path,
        staging: Option<&Path>,
    ) -> Result<Self> {
        let id = folder
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .filter(|id| is_valid_install_id(id))
            .ok_or_else(|| {
                WinAppMigratorError::InvalidInput(format!(
                    "this is not an install's folder: {}",
                    folder.display()
                ))
            })?;
        let path = folder.join(JOURNAL_FILE);
        let file = open_exclusively(OpenOptions::new().write(true).create_new(true), &path)
            .map_err(|error| WinAppMigratorError::io(&path, error))?;
        let backups = folder.join(BACKUPS_FOLDER);
        fs::create_dir(&backups).map_err(|error| WinAppMigratorError::io(&backups, error))?;
        let mut journal = Self {
            id: id.clone(),
            folder,
            file,
            _folder_hold: hold,
            next_backup: 0,
            since_flush: 0,
            broken: false,
            known_roots: HashSet::new(),
            root_holds: Vec::new(),
            known_folders: HashSet::new(),
            known_keys: HashSet::new(),
            known_task_folders: HashSet::new(),
            uninstaller: None,
        };
        journal.write(
            &Record::Header(JournalHeader {
                schema: SCHEMA_VERSION,
                id,
                app_name: app_name.to_owned(),
                target_root: target_root.to_path_buf(),
                user_sid: signed_in_user()
                    .account()
                    .map(|account| account.sid.as_str())
                    .or_else(current_user_sid)
                    .map(str::to_owned),
                started_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
                staging: staging.map(Path::to_path_buf),
            }),
            Durability::Now,
        )?;
        Ok(journal)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn folder(&self) -> &Path {
        &self.folder
    }

    pub(crate) fn uninstaller_path(&self) -> Option<&Path> {
        self.uninstaller.as_deref()
    }

    /// Write the bare stub here, for the app's entry in Settings to run. It never goes in the
    /// install folder: Windows runs that entry elevated, and non-admins may be able to write
    /// there.
    pub(crate) fn write_uninstaller(&mut self, installer: &EmbeddedPackage) -> Result<()> {
        let path = self.folder.join(UNINSTALLER_FILE);
        installer.write_bare_stub(&path)?;
        allow_users_to_start(&path).map_err(|error| WinAppMigratorError::io(&path, error))?;
        self.uninstaller = Some(path);
        Ok(())
    }

    fn write(&mut self, record: &Record, durability: Durability) -> Result<()> {
        let path = self.folder.join(JOURNAL_FILE);
        if self.broken {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "the record of this install could not be written: {}",
                path.display()
            )));
        }
        let mut line = serde_json::to_vec(record)?;
        line.push(b'\n');
        if let Err(error) = self.file.write_all(&line) {
            self.broken = true;
            return Err(WinAppMigratorError::io(path, error));
        }
        self.since_flush += 1;
        if durability == Durability::Now || self.since_flush >= NEW_ITEMS_BETWEEN_FLUSHES {
            self.file
                .sync_all()
                .map_err(|error| WinAppMigratorError::io(&path, error))?;
            self.since_flush = 0;
        }
        Ok(())
    }

    /// Record a change before it is made.
    pub(crate) fn note(&mut self, change: Change) -> Result<()> {
        self.write(&Record::Change(change), Durability::Now)
    }

    /// Record a change that only leaves something new behind if it is forgotten.
    fn note_new(&mut self, change: Change) -> Result<()> {
        self.write(&Record::Change(change), Durability::Soon)
    }

    /// Record once where a folder paths are checked against really is. It must not be, or be
    /// under, a link, which whoever can write beside it could point elsewhere at any moment. It
    /// is held from here on, and with it in them, the folders above it cannot be emptied and
    /// become links either.
    pub(crate) fn note_root(&mut self, root: &Path) -> Result<()> {
        if self.known_roots.contains(&folded(root)) {
            return Ok(());
        }
        let hold = hold_root(root).map_err(|error| {
            if error.kind() == io::ErrorKind::Other {
                WinAppMigratorError::InvalidInput(link_refusal(root))
            } else {
                WinAppMigratorError::io(root, error)
            }
        })?;
        if let Some(link) = first_link_to(root) {
            return Err(WinAppMigratorError::InvalidInput(link_refusal(&link)));
        }
        let real = real_path(root).map_err(|error| WinAppMigratorError::io(root, error))?;
        self.note_new(Change::Root {
            path: root.to_path_buf(),
            real,
        })?;
        self.known_roots.insert(folded(root));
        self.root_holds.push(hold);
        Ok(())
    }

    /// Record each folder from below `root` to `folder` that is not there yet. The caller
    /// creates them.
    pub(crate) fn note_new_folders(
        &mut self,
        root: &Path,
        folder: &Path,
        item: ItemKind,
    ) -> Result<()> {
        self.note_root(root)?;
        let relative = strip_prefix_ignore_case(folder, root).ok_or_else(|| {
            WinAppMigratorError::InvalidInput(format!(
                "path escapes intended root: {}",
                folder.display()
            ))
        })?;
        let mut current = root.to_path_buf();
        for component in relative.components() {
            current.push(component);
            if !self.known_folders.insert(folded(&current)) {
                continue;
            }
            match fs::symlink_metadata(&current) {
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    self.note_new(Change::FolderCreated {
                        path: current.clone(),
                        root: root.to_path_buf(),
                        item,
                    })?;
                }
                Err(error) => return Err(WinAppMigratorError::io(&current, error)),
            }
        }
        Ok(())
    }

    /// Record the folders it takes to reach `folder`, from whatever is already there.
    pub(crate) fn note_new_folders_to(&mut self, folder: &Path, item: ItemKind) -> Result<()> {
        let existing = folder
            .ancestors()
            .find(|ancestor| fs::symlink_metadata(ancestor).is_ok())
            .ok_or_else(|| {
                WinAppMigratorError::InvalidInput(format!(
                    "none of this exists on this PC: {}",
                    folder.display()
                ))
            })?;
        self.note_new_folders(existing, folder, item)
    }

    /// Record a file about to be written, and reserve a backup for it if one is there.
    pub(crate) fn note_file(
        &mut self,
        path: &Path,
        root: &Path,
        item: ItemKind,
        replaces: bool,
        sha256: Option<&str>,
        partial: Option<&Path>,
    ) -> Result<Option<u64>> {
        self.note_root(root)?;
        let backup = replaces.then(|| {
            self.next_backup += 1;
            self.next_backup
        });
        let change = Change::File {
            path: path.to_path_buf(),
            root: root.to_path_buf(),
            item,
            backup,
            sha256: sha256.map(str::to_owned),
            partial: partial.map(Path::to_path_buf),
        };
        if replaces {
            self.note(change)?;
        } else {
            self.note_new(change)?;
        }
        Ok(backup)
    }

    /// Keep what is at `path` as a backup, leaving nothing there. Moved when it is on the
    /// same drive, so it comes back exactly as it was, permissions and all. From another
    /// drive it is copied, with permissions those of the folder it comes back to, and only
    /// gets its name once complete, so a copy cut short is never taken for a backup.
    pub(crate) fn back_up(&self, path: &Path, number: u64) -> Result<()> {
        let moved = backup_path(&self.folder, number, MOVED_BACKUP_SUFFIX);
        match fs::rename(path, &moved) {
            Ok(()) => return Ok(()),
            Err(error) if error.raw_os_error() != Some(ERROR_NOT_SAME_DEVICE) => {
                return Err(WinAppMigratorError::io(path, error));
            }
            Err(_) => {}
        }
        let unfinished = backup_path(&self.folder, number, UNFINISHED_BACKUP_SUFFIX);
        let copied = backup_path(&self.folder, number, COPIED_BACKUP_SUFFIX);
        crate::restore::write_partial_copy(path, &unfinished, true)?;
        fs::rename(&unfinished, &copied)
            .map_err(|error| WinAppMigratorError::io(&copied, error))?;
        // That rename has to reach the disk before the delete below does, which is on another
        // drive and could get there first. Flushing any file on this drive takes it there.
        self.file
            .sync_all()
            .map_err(|error| WinAppMigratorError::io(&copied, error))?;
        crate::restore::make_replaceable(path)?;
        fs::remove_file(path).map_err(|error| WinAppMigratorError::io(path, error))
    }

    /// Record each registry key on the way to `key` that is not there yet. The caller
    /// creates them.
    pub(crate) fn note_new_keys(
        &mut self,
        hive: &RegistryHive,
        key: &str,
        item: ItemKind,
    ) -> Result<()> {
        let root = registry_root(hive)
            .map_err(|error| WinAppMigratorError::Registry(error.to_string()))?;
        let mut current = String::new();
        for part in key.split('\\').filter(|part| !part.is_empty()) {
            if !current.is_empty() {
                current.push('\\');
            }
            current.push_str(part);
            if !self
                .known_keys
                .insert(format!("{hive:?}\\{}", current.to_ascii_lowercase()))
            {
                continue;
            }
            if let Err(error) = root.open_subkey(&current)
                && error.kind() == io::ErrorKind::NotFound
            {
                self.note_new(Change::RegistryKeyCreated {
                    hive: hive.clone(),
                    key: current.clone(),
                    item,
                })?;
            }
        }
        Ok(())
    }

    /// True the first time a task folder comes up.
    pub(crate) fn first_look_at_task_folder(&mut self, path: &str) -> bool {
        self.known_task_folders.insert(path.to_ascii_lowercase())
    }

    /// Where installs are recorded, which nothing a package installs may ever be under.
    pub(crate) fn state_root(&self) -> Option<&Path> {
        self.folder.parent()?.parent()
    }

    /// Say the install finished. From here on, only an uninstall undoes it.
    pub(crate) fn commit(&mut self) -> Result<()> {
        self.write(&Record::Committed, Durability::Now)
    }

    /// Read back what has been recorded, while still holding it, to undo it. The folders it was
    /// all under are let go of, since some of them may have to be taken back.
    pub(crate) fn read_back(&mut self) -> Result<ReadJournal> {
        self.root_holds.clear();
        self.file
            .sync_all()
            .map_err(|error| WinAppMigratorError::io(self.folder.join(JOURNAL_FILE), error))?;
        read_journal(&self.folder)
    }

    /// Give up the journal, so that its folder can be removed.
    pub(crate) fn close(self) {}
}

impl Change {
    /// Part of the app's entry in Settings, which is what an uninstall is run from.
    pub(crate) fn is_uninstall_entry(&self) -> bool {
        matches!(
            self,
            Change::RegistryValue {
                item: ItemKind::UninstallEntry,
                ..
            } | Change::RegistryKeyCreated {
                item: ItemKind::UninstallEntry,
                ..
            }
        )
    }
}

/// Held by whatever is installing or uninstalling, so that two never change the same things at
/// once: if each kept what the other had just written as what it was replacing, undoing them
/// in the wrong order would lose the original.
pub(crate) struct BusyLock {
    _file: File,
}

/// Take the one lock there is. This is also what makes the state folders, since nothing can be
/// done without them. A failure means nothing has changed, and nothing should.
pub(crate) fn lock_state(state_root: &Path) -> Result<BusyLock> {
    prepare_state_folders(state_root)?;
    let path = state_root.join(BUSY_FILE);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .share_mode(0)
        .open(&path)
        .map_err(|error| {
            if is_in_use(&error) {
                WinAppMigratorError::InvalidInput(
                    "another install or uninstall is running. Wait for it to finish, then try \
                     again"
                        .to_owned(),
                )
            } else {
                WinAppMigratorError::io(&path, error)
            }
        })?;
    Ok(BusyLock { _file: file })
}

/// Open a journal so that no one else can write to it.
fn open_exclusively(options: &mut OpenOptions, path: &Path) -> io::Result<File> {
    options.share_mode(FILE_SHARE_READ).open(path)
}

/// Where a backup with this number and suffix is kept.
pub(crate) fn backup_path(folder: &Path, number: u64, suffix: &str) -> PathBuf {
    folder
        .join(BACKUPS_FOLDER)
        .join(format!("{number:08}.{suffix}"))
}

/// The backup there is for a file, if any.
pub(crate) enum Backup {
    Moved(PathBuf),
    Copied(PathBuf),
}

pub(crate) fn find_backup(folder: &Path, number: u64) -> Option<Backup> {
    let moved = backup_path(folder, number, MOVED_BACKUP_SUFFIX);
    if fs::symlink_metadata(&moved).is_ok() {
        return Some(Backup::Moved(moved));
    }
    let copied = backup_path(folder, number, COPIED_BACKUP_SUFFIX);
    fs::symlink_metadata(&copied)
        .is_ok()
        .then_some(Backup::Copied(copied))
}

/// A journal as it stands on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadJournal {
    pub header: JournalHeader,
    pub changes: Vec<Change>,
    /// The install finished. Without this, it was cut off.
    pub committed: bool,
    /// Part of it could not be read, so what is after that is not known.
    pub damaged: bool,
    pub folder: PathBuf,
}

/// Read a journal. The folder has to be one this code chose, under a checked state root.
pub(crate) fn read_journal(folder: &Path) -> Result<ReadJournal> {
    let path = folder.join(JOURNAL_FILE);
    let bytes = fs::read(&path).map_err(|error| WinAppMigratorError::io(&path, error))?;
    let name = folder
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    parse_journal(&String::from_utf8_lossy(&bytes), &name, folder)
}

/// Each line is one record, and what was written last can be cut off, since a power cut can
/// come at any moment. Whatever a cut-off record described had not started, because a change
/// is only made once its record is whole. A bad line with good ones after it is damage, and
/// everything else in the record is still read. Above all whether the install finished: to take
/// a finished install for one that never did would undo it.
fn parse_journal(text: &str, folder_name: &str, folder: &Path) -> Result<ReadJournal> {
    let unreadable = |detail: &str| {
        WinAppMigratorError::InvalidInput(format!(
            "the record of the install in {} cannot be used: {detail}",
            folder.display()
        ))
    };
    let mut lines = text.split_inclusive('\n');
    let first = lines.next().ok_or_else(|| unreadable("it is empty"))?;
    let header = match serde_json::from_str::<Record>(first) {
        Ok(Record::Header(header)) => header,
        _ => return Err(unreadable("it does not start with what it is about")),
    };
    if header.schema != SCHEMA_VERSION {
        return Err(unreadable("a different version of this program wrote it"));
    }
    if header.id != folder_name || !is_valid_install_id(&header.id) {
        return Err(unreadable("it says it belongs to a different install"));
    }
    let mut changes = Vec::new();
    let mut committed = false;
    let mut damaged = false;
    let mut bad_line = false;
    for line in lines {
        match serde_json::from_str::<Record>(line) {
            Ok(Record::Change(change)) => {
                damaged |= bad_line;
                changes.push(change);
            }
            Ok(Record::Committed) => {
                damaged |= bad_line;
                committed = true;
            }
            Ok(Record::Header(_)) => damaged = true,
            Err(_) => bad_line = true,
        }
    }
    Ok(ReadJournal {
        header,
        changes,
        committed,
        damaged,
        folder: folder.to_path_buf(),
    })
}

impl ReadJournal {
    /// The changes with what was learned after each was made folded into it, in the order
    /// they were made.
    pub fn settled_changes(&self) -> Vec<Change> {
        let mut settled: Vec<Change> = Vec::with_capacity(self.changes.len());
        for change in &self.changes {
            match change {
                Change::FileWritten { path, sha256 } => {
                    for earlier in settled.iter_mut().rev() {
                        if let Change::File {
                            path: earlier_path,
                            sha256: earlier_sha,
                            ..
                        } = earlier
                            && earlier_path == path
                        {
                            *earlier_sha = Some(sha256.clone());
                            break;
                        }
                    }
                }
                Change::FileUndone { path } => {
                    let found = settled.iter().rposition(|earlier| {
                        matches!(earlier, Change::File { path: earlier_path, .. } if earlier_path == path)
                    });
                    if let Some(index) = found {
                        settled.remove(index);
                    }
                }
                Change::TaskWritten {
                    task_path,
                    task_name,
                    xml,
                } => {
                    for earlier in settled.iter_mut().rev() {
                        if let Change::Task {
                            task_path: earlier_path,
                            task_name: earlier_name,
                            written_xml,
                            ..
                        } = earlier
                            && earlier_path == task_path
                            && earlier_name == task_name
                        {
                            *written_xml = Some(xml.clone());
                            break;
                        }
                    }
                }
                Change::PrerequisiteSkipped { name } => {
                    let found = settled.iter().rposition(|earlier| {
                        matches!(earlier, Change::Prerequisite { name: earlier_name } if earlier_name == name)
                    });
                    if let Some(index) = found {
                        settled.remove(index);
                    }
                }
                Change::OnlyLeftOn { roots, keeps_entry } => {
                    let is_left = |root: &Path| roots.iter().any(|left| paths_equal(left, root));
                    settled.retain(|earlier| match earlier {
                        Change::Root { path: root, .. }
                        | Change::File { root, .. }
                        | Change::FolderCreated { root, .. } => is_left(root),
                        other => *keeps_entry && other.is_uninstall_entry(),
                    });
                }
                other => settled.push(other.clone()),
            }
        }
        settled
    }

    /// What this install did, as counts, for saying what an uninstall will remove.
    pub fn summary(&self) -> InstallSummary {
        let mut summary = InstallSummary {
            app_name: self.header.app_name.clone(),
            target_root: self.header.target_root.clone(),
            finished: self.committed,
            ..InstallSummary::default()
        };
        for change in self.settled_changes() {
            match change {
                Change::File {
                    item: ItemKind::UserData,
                    ..
                }
                | Change::RegistryValue {
                    item: ItemKind::UserData,
                    ..
                } => summary.user_data += 1,
                Change::File {
                    item: ItemKind::License,
                    ..
                }
                | Change::RegistryValue {
                    item: ItemKind::License,
                    ..
                } => summary.license += 1,
                Change::File { backup, .. } => {
                    summary.files += 1;
                    if backup.is_some() {
                        summary.files_coming_back += 1;
                    }
                }
                Change::RegistryValue { .. } => summary.registry_values += 1,
                Change::EnvironmentValue { .. } | Change::PathEntries { .. } => {
                    summary.environment += 1;
                }
                Change::Service { driver: false, .. } => summary.services += 1,
                Change::Service { driver: true, .. } => summary.drivers += 1,
                Change::Task { .. } => summary.tasks += 1,
                Change::Prerequisite { name } => summary.runtimes_left.push(name),
                _ => {}
            }
        }
        summary
    }
}

/// What an install did, for someone deciding whether to undo it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallSummary {
    pub app_name: String,
    pub target_root: PathBuf,
    pub finished: bool,
    pub files: usize,
    /// Of those, how many replaced a file that will come back.
    pub files_coming_back: usize,
    pub registry_values: usize,
    pub services: usize,
    pub drivers: usize,
    pub tasks: usize,
    pub environment: usize,
    /// Kept unless someone asks for them to go.
    pub user_data: usize,
    pub license: usize,
    /// Runtimes that were installed, which stay.
    pub runtimes_left: Vec<String>,
}

/// A journal locked against being written to or undone by anyone else. A failure with
/// [`is_in_use`] means another installer or uninstaller has it.
pub(crate) struct JournalLock {
    file: File,
}

impl JournalLock {
    /// Add to a record that is held and being undone. What was last written may have been cut
    /// off. What it described never started, so it goes, and is not left to look like damage
    /// once something follows it.
    pub(crate) fn note(&mut self, change: Change) -> io::Result<()> {
        let mut written = Vec::new();
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_to_end(&mut written)?;
        let whole = written
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(0, |last| last + 1);
        self.file
            .set_len(u64::try_from(whole).map_err(io::Error::other)?)?;
        let mut line = serde_json::to_vec(&Record::Change(change)).map_err(io::Error::other)?;
        line.push(b'\n');
        self.file.seek(SeekFrom::End(0))?;
        self.file.write_all(&line)?;
        self.file.sync_all()
    }
}

pub(crate) fn lock_journal(folder: &Path) -> io::Result<JournalLock> {
    let file = open_exclusively(
        OpenOptions::new().read(true).write(true),
        &folder.join(JOURNAL_FILE),
    )?;
    Ok(JournalLock { file })
}

pub(crate) fn is_in_use(error: &io::Error) -> bool {
    error.raw_os_error() == Some(ERROR_SHARING_VIOLATION)
}

/// The folder of one install, checked level by level. A failure means what is in there is
/// not to be trusted.
pub(crate) fn checked_install_folder(state_root: &Path, id: &str) -> Result<PathBuf> {
    if !is_valid_install_id(id) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "this is not an install's id: {id}"
        )));
    }
    let installs = state_root.join(INSTALLS_FOLDER);
    let folder = installs.join(id);
    for level in [state_root, installs.as_path(), folder.as_path()] {
        confirm_protected_folder(level).map_err(|error| WinAppMigratorError::io(level, error))?;
    }
    Ok(folder)
}

/// The folders of all installs on record, without changing anything. Empty if there has
/// never been one. What this finds is not to be acted on without checking it, and a failure
/// here means there is nothing to read.
pub(crate) fn install_folders(state_root: &Path) -> Result<Vec<PathBuf>> {
    let installs = state_root.join(INSTALLS_FOLDER);
    if fs::symlink_metadata(&installs).is_err() {
        return Ok(Vec::new());
    }
    let entries =
        fs::read_dir(&installs).map_err(|error| WinAppMigratorError::io(&installs, error))?;
    Ok(entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| is_valid_install_id(&name.to_string_lossy()))
        })
        .collect())
}

/// Whether a record says an install was at this location, other than `but_not`, the folder of one
/// going on now. Only an administrator can write where these are kept, and a package cannot name
/// that place, so this is what says a location has been installed to, and a package saying so does
/// not.
pub fn install_on_record_at(state_root: &Path, target_root: &Path, but_not: Option<&Path>) -> bool {
    install_folders(state_root)
        .unwrap_or_default()
        .iter()
        .filter(|folder| but_not.is_none_or(|this_one| folder.file_name() != this_one.file_name()))
        .filter_map(|folder| read_journal(folder).ok())
        .any(|record| {
            let recorded = record.summary().target_root;
            crate::path_utils::path_is_within(&recorded, target_root)
                && crate::path_utils::path_is_within(target_root, &recorded)
        })
}

/// True when a folder holds no journal.
pub(crate) fn has_no_journal(folder: &Path) -> bool {
    matches!(
        fs::symlink_metadata(folder.join(JOURNAL_FILE)),
        Err(error) if error.kind() == io::ErrorKind::NotFound
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_location_is_only_on_record_if_a_record_of_another_install_says_so() -> Result<()> {
        let (_work, state) = temp_state()?;
        let place = Path::new(r"C:\Acme");
        assert!(!install_on_record_at(&state, place, None));
        let journal = Journal::begin(&state, "Acme", place, None)?;
        let folder = journal.folder().to_path_buf();
        drop(journal);
        assert!(install_on_record_at(&state, place, None));
        assert!(install_on_record_at(&state, Path::new(r"c:\acme\"), None));
        assert!(!install_on_record_at(&state, Path::new(r"C:\Acme2"), None));
        assert!(!install_on_record_at(
            &state,
            Path::new(r"C:\Acme\bin"),
            None
        ));
        assert!(
            !install_on_record_at(&state, place, Some(&folder)),
            "the record of this install says nothing of what was there before it"
        );
        Ok(())
    }

    /// Whether the install finished or not, and whether what was last written was whole or not,
    /// what is added is read, and nothing is taken for damage.
    #[test]
    fn what_is_added_to_a_held_record_can_be_read_back() -> Result<()> {
        for (finished, cut_off) in [(false, false), (false, true), (true, false), (true, true)] {
            let (_work, state) = temp_state()?;
            let mut journal = Journal::begin(&state, "Acme", Path::new(r"C:\Acme"), None)?;
            journal.note(Change::Prerequisite {
                name: "Runtime".to_owned(),
            })?;
            if finished {
                journal.commit()?;
            }
            let folder = journal.folder().to_path_buf();
            drop(journal);
            if cut_off {
                OpenOptions::new()
                    .append(true)
                    .open(folder.join(JOURNAL_FILE))
                    .and_then(|mut file| file.write_all(b"{\"change\":{\"cha"))
                    .map_err(|error| WinAppMigratorError::io(&folder, error))?;
            }
            let mut lock =
                lock_journal(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
            lock.note(Change::OnlyLeftOn {
                roots: vec![PathBuf::from(r"E:\Apps")],
                keeps_entry: finished,
            })
            .map_err(|error| WinAppMigratorError::io(&folder, error))?;
            drop(lock);

            let read = read_journal(&folder)?;
            assert_eq!(read.committed, finished, "{finished} {cut_off}");
            assert!(!read.damaged, "{finished} {cut_off}");
            assert!(
                read.settled_changes().is_empty(),
                "what was undone is dropped: {finished} {cut_off}"
            );
        }
        Ok(())
    }

    #[test]
    fn what_is_left_is_only_what_is_on_the_drive_and_an_entry_if_it_is_to_be_finished_from_there() {
        let left = PathBuf::from(r"E:\Apps");
        let here = PathBuf::from(r"C:\Apps");
        let file = |root: &Path| Change::File {
            path: root.join("a.dat"),
            root: root.to_path_buf(),
            item: ItemKind::Program,
            backup: None,
            sha256: None,
            partial: None,
        };
        let entry = Change::RegistryKeyCreated {
            hive: RegistryHive::LocalMachine,
            key: r"Software\Acme".to_owned(),
            item: ItemKind::UninstallEntry,
        };
        let other_key = Change::RegistryKeyCreated {
            hive: RegistryHive::LocalMachine,
            key: r"Software\Other".to_owned(),
            item: ItemKind::Program,
        };
        for keeps_entry in [true, false] {
            let read = ReadJournal {
                header: JournalHeader {
                    schema: SCHEMA_VERSION,
                    id: "wam-abc".to_owned(),
                    app_name: "Acme".to_owned(),
                    target_root: left.clone(),
                    user_sid: None,
                    started_at_utc: String::new(),
                    staging: None,
                },
                changes: vec![
                    file(&left),
                    file(&here),
                    entry.clone(),
                    other_key.clone(),
                    Change::OnlyLeftOn {
                        roots: vec![left.clone()],
                        keeps_entry,
                    },
                ],
                committed: true,
                damaged: false,
                folder: PathBuf::from(r"C:\Not\There"),
            };
            let settled = read.settled_changes();
            let wanted = if keeps_entry { 2 } else { 1 };
            assert_eq!(settled.len(), wanted, "{settled:?}");
            assert!(matches!(&settled[0], Change::File { root, .. } if *root == left));
            assert_eq!(settled.len() > 1, keeps_entry);
        }
    }

    fn temp_state() -> Result<(tempfile::TempDir, PathBuf)> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let state = work.path().join("state");
        Ok((work, state))
    }

    fn journal_in(state: &Path) -> Result<Journal> {
        Journal::begin(state, "Acme", Path::new(r"C:\Program Files\Acme"), None)
    }

    #[test]
    fn only_what_this_code_would_name_counts_as_an_id() {
        for good in ["wam-aB3xYz", "wam-1"] {
            assert!(is_valid_install_id(good), "{good}");
        }
        for bad in [
            "",
            "wam-",
            "aB3xYz",
            "wam-..",
            r"wam-a\b",
            "wam-a/b",
            "wam-a b",
            "wam-a.b",
            "wam-a:b",
            r"..\wam-a",
            "CON",
            "wam-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            assert!(!is_valid_install_id(bad), "{bad}");
        }
    }

    #[test]
    fn a_new_journal_is_in_a_folder_of_its_own_and_reads_back_unfinished() -> Result<()> {
        let (_work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        assert_eq!(
            journal.folder().parent(),
            Some(state.join("installs").as_path())
        );
        assert!(is_valid_install_id(journal.id()));
        let read = journal.read_back()?;
        assert_eq!(read.header.app_name, "Acme");
        assert_eq!(read.header.id, journal.id());
        assert!(read.changes.is_empty());
        assert!(!read.committed);
        assert!(!read.damaged);
        Ok(())
    }

    #[test]
    fn changes_read_back_in_the_order_they_were_recorded_and_commit_shows() -> Result<()> {
        let (_work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        let first = Change::TaskFolderCreated {
            path: r"\Acme".to_owned(),
        };
        let second = Change::Prerequisite {
            name: "Runtime".to_owned(),
        };
        journal.note(first.clone())?;
        journal.note(second.clone())?;
        journal.commit()?;

        let read = read_journal(journal.folder())?;
        assert_eq!(read.changes, vec![first, second]);
        assert!(read.committed);
        Ok(())
    }

    #[test]
    fn a_journal_is_written_at_once_and_a_killed_process_loses_nothing() -> Result<()> {
        let (_work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        // Not flushed by this, but in the system's hands, and so still there if the process
        // dies.
        journal.note_new(Change::FolderCreated {
            path: PathBuf::from(r"C:\A\B"),
            root: PathBuf::from(r"C:\A"),
            item: ItemKind::Program,
        })?;
        assert_eq!(read_journal(journal.folder())?.changes.len(), 1);
        Ok(())
    }

    #[test]
    fn a_cut_off_last_line_is_ignored_and_a_bad_middle_line_is_damage() -> Result<()> {
        let folder = PathBuf::from(r"C:\State\installs\wam-abc");
        let header = serde_json::to_string(&Record::Header(JournalHeader {
            schema: SCHEMA_VERSION,
            id: "wam-abc".to_owned(),
            app_name: "Acme".to_owned(),
            target_root: PathBuf::from(r"C:\Acme"),
            user_sid: None,
            started_at_utc: String::new(),
            staging: None,
        }))?;
        let change = serde_json::to_string(&Record::Change(Change::Prerequisite {
            name: "Runtime".to_owned(),
        }))?;

        let cut_off = format!("{header}\n{change}\n{{\"change\":{{\"chan");
        let read = parse_journal(&cut_off, "wam-abc", &folder)?;
        assert_eq!(read.changes.len(), 1);
        assert!(!read.damaged);
        assert!(!read.committed);

        let zeros_after_power_cut = format!("{header}\n{change}\n\0\0\0\0");
        let read = parse_journal(&zeros_after_power_cut, "wam-abc", &folder)?;
        assert_eq!(read.changes.len(), 1);
        assert!(!read.damaged);

        let damaged = format!("{header}\nnonsense\n{change}\n");
        let read = parse_journal(&damaged, "wam-abc", &folder)?;
        assert_eq!(read.changes.len(), 1, "what can be read still is");
        assert!(read.damaged);
        Ok(())
    }

    #[test]
    fn a_finished_install_stays_finished_with_a_bad_line_in_its_record() -> Result<()> {
        let folder = PathBuf::from(r"C:\State\installs\wam-abc");
        let header = serde_json::to_string(&Record::Header(JournalHeader {
            schema: SCHEMA_VERSION,
            id: "wam-abc".to_owned(),
            app_name: "Acme".to_owned(),
            target_root: PathBuf::from(r"C:\Acme"),
            user_sid: None,
            started_at_utc: String::new(),
            staging: None,
        }))?;
        let change = serde_json::to_string(&Record::Change(Change::Prerequisite {
            name: "Runtime".to_owned(),
        }))?;
        let committed = serde_json::to_string(&Record::Committed)?;

        let text = format!("{header}\nnonsense\n{change}\n{committed}\n");
        let read = parse_journal(&text, "wam-abc", &folder)?;

        assert!(
            read.committed,
            "it must not be taken for one that never finished"
        );
        assert!(read.damaged);
        assert_eq!(read.changes.len(), 1, "what can be read still is");
        Ok(())
    }

    #[test]
    fn a_record_that_could_not_be_written_stops_everything_after_it() -> Result<()> {
        let (_work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        journal.broken = true;
        assert!(
            journal
                .note(Change::Prerequisite {
                    name: "Runtime".to_owned(),
                })
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn only_one_install_or_uninstall_runs_at_a_time() -> Result<()> {
        let (_work, state) = temp_state()?;
        let first = lock_state(&state)?;
        let second = lock_state(&state);
        assert!(
            matches!(&second, Err(WinAppMigratorError::InvalidInput(message))
                if message.contains("another install")),
            "{:?}",
            second.err()
        );
        drop(first);
        assert!(lock_state(&state).is_ok());
        Ok(())
    }

    #[test]
    fn a_journal_that_says_it_is_someone_elses_or_from_another_version_is_refused() -> Result<()> {
        let folder = PathBuf::from(r"C:\State\installs\wam-abc");
        let header = |id: &str, schema: u32| -> Result<String> {
            Ok(serde_json::to_string(&Record::Header(JournalHeader {
                schema,
                id: id.to_owned(),
                app_name: String::new(),
                target_root: PathBuf::new(),
                user_sid: None,
                started_at_utc: String::new(),
                staging: None,
            }))?)
        };
        assert!(parse_journal(&header("wam-abc", SCHEMA_VERSION)?, "wam-abc", &folder).is_ok());
        assert!(parse_journal(&header("wam-other", SCHEMA_VERSION)?, "wam-abc", &folder).is_err());
        assert!(parse_journal(&header("wam-abc", 999)?, "wam-abc", &folder).is_err());
        assert!(parse_journal("", "wam-abc", &folder).is_err());
        assert!(parse_journal("{}", "wam-abc", &folder).is_err());
        Ok(())
    }

    #[test]
    fn a_journal_in_use_cannot_be_locked_and_one_whose_writer_is_gone_can() -> Result<()> {
        let (_work, state) = temp_state()?;
        let journal = journal_in(&state)?;
        let folder = journal.folder().to_path_buf();
        let locked = lock_journal(&folder);
        assert!(
            locked.as_ref().err().is_some_and(is_in_use),
            "{:?}",
            locked.err()
        );
        // Reading is still allowed, which is how a test watches an install from outside.
        assert!(read_journal(&folder).is_ok());
        journal.close();
        let locked =
            lock_journal(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
        assert!(lock_journal(&folder).as_ref().err().is_some_and(is_in_use));
        drop(locked);
        Ok(())
    }

    #[test]
    fn what_comes_after_a_change_is_folded_into_it() -> Result<()> {
        let (_work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        let path = PathBuf::from(r"C:\Links\app.lnk");
        let dropped = PathBuf::from(r"C:\Links\dropped.lnk");
        for file in [&path, &dropped] {
            journal.note(Change::File {
                path: file.clone(),
                root: PathBuf::from(r"C:\Links"),
                item: ItemKind::Program,
                backup: None,
                sha256: None,
                partial: None,
            })?;
        }
        journal.note(Change::FileWritten {
            path: path.clone(),
            sha256: "abc".to_owned(),
        })?;
        journal.note(Change::FileUndone { path: dropped })?;
        journal.note(Change::Prerequisite {
            name: "Runtime".to_owned(),
        })?;
        journal.note(Change::PrerequisiteSkipped {
            name: "Runtime".to_owned(),
        })?;
        journal.note(Change::Task {
            task_path: r"\Acme\".to_owned(),
            task_name: "Sync".to_owned(),
            prior_xml: None,
            written_xml: None,
        })?;
        journal.note(Change::TaskWritten {
            task_path: r"\Acme\".to_owned(),
            task_name: "Sync".to_owned(),
            xml: "<Task/>".to_owned(),
        })?;

        let settled = journal.read_back()?.settled_changes();
        assert_eq!(settled.len(), 2);
        assert!(matches!(
            &settled[0],
            Change::File { sha256: Some(hash), .. } if hash == "abc"
        ));
        assert!(matches!(
            &settled[1],
            Change::Task { written_xml: Some(xml), .. } if xml == "<Task/>"
        ));
        Ok(())
    }

    #[test]
    fn only_the_folders_that_are_not_there_yet_are_recorded_and_only_once() -> Result<()> {
        let (work, state) = temp_state()?;
        let mut journal = journal_in(&state)?;
        let root = work.path().join("root");
        fs::create_dir_all(root.join("there"))
            .map_err(|error| WinAppMigratorError::io(&root, error))?;
        let wanted = root.join(r"there\new\deep");
        journal.note_new_folders(&root, &wanted, ItemKind::Program)?;
        journal.note_new_folders(&root, &wanted, ItemKind::Program)?;

        let created: Vec<PathBuf> = journal
            .read_back()?
            .changes
            .into_iter()
            .filter_map(|change| match change {
                Change::FolderCreated { path, .. } => Some(path),
                _ => None,
            })
            .collect();
        assert_eq!(
            created,
            vec![root.join(r"there\new"), root.join(r"there\new\deep")]
        );
        Ok(())
    }

    #[test]
    fn a_backup_is_the_file_itself_and_nothing_is_left_where_it_was() -> Result<()> {
        let (work, state) = temp_state()?;
        let journal = journal_in(&state)?;
        let file = work.path().join("old.txt");
        fs::write(&file, b"old").map_err(|error| WinAppMigratorError::io(&file, error))?;
        journal.back_up(&file, 7)?;
        assert!(!file.exists());
        let Some(Backup::Moved(backup)) = find_backup(journal.folder(), 7) else {
            return Err(WinAppMigratorError::InvalidInput("no backup".to_owned()));
        };
        assert_eq!(
            fs::read(&backup).map_err(|error| WinAppMigratorError::io(&backup, error))?,
            b"old"
        );
        assert!(find_backup(journal.folder(), 8).is_none());
        Ok(())
    }

    #[test]
    fn a_stored_value_round_trips_every_type() -> std::result::Result<(), String> {
        for number in 0..=11 {
            let stored = StoredValue {
                kind: number,
                data_hex: "00ff10".to_owned(),
            };
            let value = stored.to_reg()?;
            assert_eq!(StoredValue::from_reg(&value), stored);
        }
        assert!(
            StoredValue {
                kind: 12,
                data_hex: String::new()
            }
            .to_reg()
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn a_summary_counts_what_will_go_and_what_stays() {
        let file = |item, backup| Change::File {
            path: PathBuf::from(r"C:\A\x"),
            root: PathBuf::from(r"C:\A"),
            item,
            backup,
            sha256: None,
            partial: None,
        };
        let journal = ReadJournal {
            header: JournalHeader {
                schema: SCHEMA_VERSION,
                id: "wam-a".to_owned(),
                app_name: "Acme".to_owned(),
                target_root: PathBuf::from(r"C:\A"),
                user_sid: None,
                started_at_utc: String::new(),
                staging: None,
            },
            changes: vec![
                file(ItemKind::Program, None),
                file(ItemKind::Program, Some(1)),
                file(ItemKind::UserData, None),
                file(ItemKind::License, None),
                Change::Prerequisite {
                    name: "Runtime".to_owned(),
                },
            ],
            committed: true,
            damaged: false,
            folder: PathBuf::from(r"C:\State\installs\wam-a"),
        };
        let summary = journal.summary();
        assert_eq!(summary.files, 2);
        assert_eq!(summary.files_coming_back, 1);
        assert_eq!(summary.user_data, 1);
        assert_eq!(summary.license, 1);
        assert_eq!(summary.runtimes_left, ["Runtime"]);
        assert!(summary.finished);
    }

    #[test]
    fn install_folders_are_listed_and_a_state_root_that_is_not_there_is_no_error() -> Result<()> {
        let (work, state) = temp_state()?;
        assert!(install_folders(&state)?.is_empty());
        let journal = journal_in(&state)?;
        fs::create_dir(state.join("installs").join("not-one-of-ours"))
            .map_err(|error| WinAppMigratorError::io(work.path(), error))?;
        assert_eq!(
            install_folders(&state)?,
            vec![journal.folder().to_path_buf()]
        );
        assert_eq!(
            checked_install_folder(&state, journal.id())?,
            journal.folder()
        );
        assert!(checked_install_folder(&state, r"..\..\elsewhere").is_err());
        assert!(checked_install_folder(&state, "wam-notthere").is_err());
        Ok(())
    }
}
