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
use std::io::{self, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use winreg::RegValue;
use winreg::enums::RegType;

use crate::archive::EmbeddedPackage;
use crate::errors::{Result, WinAppMigratorError};
use crate::hashing::hex_lower;
use crate::models::{EnvironmentScope, RegistryHive};
use crate::path_utils::{
    current_user_sid, program_data_dir, strip_prefix_ignore_case, without_verbatim_prefix,
};
use crate::privilege::{
    ProtectedFolder, allow_users_to_start, confirm_protected_folder, ensure_protected_folder,
    is_elevated,
};
use crate::registry_target::{hex_decode, registry_root};
use crate::restore_policy::paths_equal;

const STATE_FOLDER: &str = "WinAppMigrator";
const INSTALLS_FOLDER: &str = "installs";
const JOURNAL_FILE: &str = "journal.jsonl";
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
    /// A runtime that was installed. It may be in use by anything, so it is only reported.
    Prerequisite {
        name: String,
    },
}

/// What a journal is about.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalHeader {
    pub schema: u32,
    pub id: String,
    pub app_name: String,
    pub target_root: PathBuf,
    /// Whose `HKEY_CURRENT_USER` this install wrote to.
    pub user_sid: Option<String>,
    pub started_at_utc: String,
    /// Left behind if the installer is killed. Only ever a folder it made.
    pub staging: Option<PathBuf>,
    /// The theme the installer used, for the uninstaller to match.
    #[serde(default)]
    pub ui_theme: Option<String>,
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
    known_roots: HashSet<String>,
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
        ui_theme: Option<&str>,
    ) -> Result<Self> {
        let installs = prepare_state_folders(state_root)?;
        let (folder, hold) = ProtectedFolder::new_in(&installs, ID_PREFIX)
            .map_err(|error| WinAppMigratorError::io(&installs, error))?
            .keep();
        let started = Self::start_in(
            folder.clone(),
            hold,
            app_name,
            target_root,
            staging,
            ui_theme,
        );
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
        ui_theme: Option<&str>,
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
            known_roots: HashSet::new(),
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
                user_sid: current_user_sid().map(str::to_owned),
                started_at_utc: OffsetDateTime::now_utc().format(&Rfc3339)?,
                staging: staging.map(Path::to_path_buf),
                ui_theme: ui_theme.map(str::to_owned),
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
        let mut line = serde_json::to_vec(record)?;
        line.push(b'\n');
        let path = || self.folder.join(JOURNAL_FILE);
        self.file
            .write_all(&line)
            .map_err(|error| WinAppMigratorError::io(path(), error))?;
        self.since_flush += 1;
        if durability == Durability::Now || self.since_flush >= NEW_ITEMS_BETWEEN_FLUSHES {
            self.file
                .sync_all()
                .map_err(|error| WinAppMigratorError::io(path(), error))?;
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

    /// Record once where a folder paths are checked against really is.
    pub(crate) fn note_root(&mut self, root: &Path) -> Result<()> {
        if self.known_roots.insert(folded(root)) {
            let real = real_path(root).map_err(|error| WinAppMigratorError::io(root, error))?;
            self.note_new(Change::Root {
                path: root.to_path_buf(),
                real,
            })?;
        }
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
        let root = registry_root(hive);
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

    /// Record a registry value about to be written.
    pub(crate) fn note_value(
        &mut self,
        hive: &RegistryHive,
        key: &str,
        name: &str,
        prior: Option<StoredValue>,
        written: StoredValue,
        item: ItemKind,
    ) -> Result<()> {
        let keeps_something = prior.is_some();
        let change = Change::RegistryValue {
            hive: hive.clone(),
            key: key.to_owned(),
            name: name.to_owned(),
            prior,
            written,
            item,
        };
        if keeps_something {
            self.note(change)
        } else {
            self.note_new(change)
        }
    }

    /// True the first time a task folder comes up.
    pub(crate) fn first_look_at_task_folder(&mut self, path: &str) -> bool {
        self.known_task_folders.insert(path.to_ascii_lowercase())
    }

    /// Say the install finished. From here on, only an uninstall undoes it.
    pub(crate) fn commit(&mut self) -> Result<()> {
        self.write(&Record::Committed, Durability::Now)
    }

    /// Read back what has been recorded, while still holding it.
    pub(crate) fn read_back(&mut self) -> Result<ReadJournal> {
        self.file
            .sync_all()
            .map_err(|error| WinAppMigratorError::io(self.folder.join(JOURNAL_FILE), error))?;
        read_journal(&self.folder)
    }

    /// Give up the journal, so that its folder can be removed.
    pub(crate) fn close(self) {}
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
/// is only made once its record is whole. A bad line with good ones after it is damage.
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
    let rest: Vec<&str> = lines.collect();
    for (index, line) in rest.iter().enumerate() {
        match serde_json::from_str::<Record>(line) {
            Ok(Record::Change(change)) => changes.push(change),
            Ok(Record::Committed) => committed = true,
            Ok(Record::Header(_)) => {
                damaged = true;
                break;
            }
            Err(_) => {
                damaged = rest[index + 1..]
                    .iter()
                    .any(|later| serde_json::from_str::<Record>(later).is_ok());
                break;
            }
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
            ui_theme: self.header.ui_theme.clone(),
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
    pub ui_theme: Option<String>,
}

/// A journal locked against being written to or undone by anyone else. A failure with
/// [`is_in_use`] means another installer or uninstaller has it.
pub(crate) struct JournalLock {
    _file: File,
}

pub(crate) fn lock_journal(folder: &Path) -> io::Result<JournalLock> {
    let file = open_exclusively(
        OpenOptions::new().read(true).write(true),
        &folder.join(JOURNAL_FILE),
    )?;
    Ok(JournalLock { _file: file })
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

/// The folders of all installs on record. `Ok` and empty if there has never been one, and an
/// error if what is there cannot be trusted.
pub(crate) fn install_folders(state_root: &Path) -> Result<Vec<PathBuf>> {
    if fs::symlink_metadata(state_root).is_err() {
        return Ok(Vec::new());
    }
    let installs = state_root.join(INSTALLS_FOLDER);
    if fs::symlink_metadata(&installs).is_err() {
        return Ok(Vec::new());
    }
    for level in [state_root, installs.as_path()] {
        confirm_protected_folder(level).map_err(|error| WinAppMigratorError::io(level, error))?;
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

    fn temp_state() -> Result<(tempfile::TempDir, PathBuf)> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let state = work.path().join("state");
        Ok((work, state))
    }

    fn journal_in(state: &Path) -> Result<Journal> {
        Journal::begin(
            state,
            "Acme",
            Path::new(r"C:\Program Files\Acme"),
            None,
            None,
        )
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
            ui_theme: None,
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
        assert!(read.changes.is_empty());
        assert!(read.damaged);
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
                ui_theme: None,
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
                ui_theme: None,
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
