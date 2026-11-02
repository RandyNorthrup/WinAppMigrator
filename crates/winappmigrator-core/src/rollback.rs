//! Undoing an install, from its journal.
//!
//! The same code does three jobs: a rollback when an install fails or is cancelled, the same
//! rollback on the next run when an install never finished, and an uninstall. It goes through
//! what the journal says in reverse, and for each change it looks at how things are now,
//! because a journal entry is written before its change, so that change may never have
//! happened, or may have been undone by a run that was itself cut off. That makes running it
//! again always safe.
//!
//! **This runs elevated, on locations other programs may be able to write to.** What it acts
//! on comes only from the journal, and before each delete or restore it holds the folders on
//! the way there and checks that none of them has become a link.
//!
//! A rollback takes back everything the install wrote, whatever is in it, since an install
//! that never finished has no users to lose anything. An uninstall deletes a file only if it
//! is still what the install wrote. Either way, a registry value or environment variable that
//! no longer holds what the install wrote belongs to someone else now, and is left alone.

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use tracing::{info, warn};
use winreg::enums::{HKEY_USERS, KEY_READ, KEY_SET_VALUE};
use winreg::{RegKey, RegValue};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::hold_path_below_root;
use crate::hashing::sha256_file;
use crate::journal::{
    Backup, Change, ItemKind, Journal, JournalLock, ReadJournal, STAGING_PREFIX, ServiceConfig,
    StoredValue, checked_install_folder, find_backup, has_no_journal, install_folders, is_in_use,
    lock_journal, read_journal, real_path,
};
use crate::models::{EnvironmentScope, RegistryHive, UndoReport};
use crate::path_utils::{
    POWERSHELL_IMPORT_UTILITY, current_user_sid, escape_powershell_single, hidden_command,
    path_is_within, system_root,
};
use crate::privilege::{hold_folder, hold_real_folder};
use crate::progress::{InstallMonitor, InstallPhase, InstallStep};
use crate::registry_target::{
    environment_location, registry_hive_name, registry_root, remove_path_entries,
    service_image_path,
};
use crate::restore::{
    SC_EXE, encode_utf16_nul, make_replaceable, partial_file_name, register_task_script,
    run_checked, run_powershell_script, sc_start_type, write_partial_copy, write_task_definition,
};
use crate::restore_policy::{ServiceAccount, image_path_matches};
use crate::task_scan::live_task_xml;

const STOP_SERVICE_TIMEOUT: Duration = Duration::from_secs(30);
const STOP_SERVICE_POLL: Duration = Duration::from_millis(250);
const STOPPED_STATE: &str = "STOPPED";
const SC_EMPTY_DEPENDENCIES: &str = "/";
const ERROR_DIR_NOT_EMPTY: i32 = 145;
const TASK_FOLDER_NOT_EMPTY_EXIT: i32 = 3;
const SERVICE_KERNEL_DRIVER: u32 = 0x1;
const SERVICE_FILE_SYSTEM_DRIVER: u32 = 0x2;
const SERVICE_RECOGNIZER_DRIVER: u32 = 0x8;
/// How long, at about a second a try, the removal of an uninstaller's folder keeps trying
/// after the uninstaller has exited, for whatever is still looking at the file.
const SELF_REMOVAL_TRIES: u32 = 60;

/// How to treat what an install left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoMode {
    /// The install did not finish, so all of it goes.
    Rollback,
    /// The install finished, and what has changed since, or belongs to the user, stays.
    Uninstall { remove_user_data: bool },
}

/// What to do about a file, given how things stand. Kept pure so every case can be tested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    Nothing,
    Delete,
    PutBackBackup,
    /// It has changed since, so it is not this install's to remove.
    KeepChanged,
    /// User data or a license, which stay unless someone asks.
    KeepByRequest,
}

struct FileFacts {
    /// There was a file at this path before the install, so it has a backup or had one.
    replaced: bool,
    /// The backup is still there. Once used it is gone, and that file is back in place.
    has_backup: bool,
    exists: bool,
    /// Whether what is there is what the install wrote, if that was asked.
    unchanged: Option<bool>,
}

fn file_action(mode: UndoMode, item: ItemKind, facts: &FileFacts) -> FileAction {
    let kept_by_request = matches!(
        mode,
        UndoMode::Uninstall {
            remove_user_data: false
        }
    ) && matches!(item, ItemKind::UserData | ItemKind::License);
    if kept_by_request {
        return if facts.exists {
            FileAction::KeepByRequest
        } else {
            FileAction::Nothing
        };
    }
    if facts.replaced && !facts.has_backup {
        return FileAction::Nothing;
    }
    let replace_or_delete = if facts.has_backup {
        FileAction::PutBackBackup
    } else {
        FileAction::Delete
    };
    if !facts.exists {
        return if facts.has_backup {
            FileAction::PutBackBackup
        } else {
            FileAction::Nothing
        };
    }
    match mode {
        UndoMode::Rollback => replace_or_delete,
        UndoMode::Uninstall { .. } if facts.unchanged == Some(true) => replace_or_delete,
        UndoMode::Uninstall { .. } => FileAction::KeepChanged,
    }
}

/// What to do about a registry value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueAction {
    /// It is as it was before, whether the write never happened or was already undone.
    Nothing,
    PutBack,
    /// It holds something the install did not write, which is someone else's to change.
    Keep,
}

fn value_action(
    current: Option<&StoredValue>,
    written: &StoredValue,
    prior: Option<&StoredValue>,
) -> ValueAction {
    if current == Some(written) {
        ValueAction::PutBack
    } else if current == prior {
        ValueAction::Nothing
    } else {
        ValueAction::Keep
    }
}

/// What to do about a scheduled task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskAction {
    Nothing,
    Delete,
    PutBack,
    Keep,
}

fn task_action(live: Option<&str>, written: Option<&str>, prior: Option<&str>) -> TaskAction {
    let Some(live) = live else {
        return TaskAction::Nothing;
    };
    let same = |left: &str, right: &str| left.trim() == right.trim();
    let is_written = written.is_some_and(|written| same(written, live));
    if !is_written && prior.is_some_and(|prior| same(prior, live)) {
        return TaskAction::Nothing;
    }
    // What the install registered is only unknown if it was cut off right after doing so.
    if written.is_some() && !is_written {
        return TaskAction::Keep;
    }
    if prior.is_some() {
        TaskAction::PutBack
    } else {
        TaskAction::Delete
    }
}

/// Whether it is safe to go on to a path, having held what is on the way there.
enum Guard {
    /// Holding these until done.
    Held(Vec<fs::File>),
    /// The folder it was all under has gone, so what was in it has too.
    RootGone,
}

struct Undoer<'a> {
    folder: &'a Path,
    mode: UndoMode,
    user_sid: Option<&'a str>,
    roots: HashMap<String, String>,
    kept_folders: Vec<PathBuf>,
    report: UndoReport,
}

fn root_key(path: &Path) -> String {
    path.to_string_lossy()
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_ascii_lowercase()
}

/// Undo what a journal records. Whatever goes wrong with one change is reported, and the
/// rest still happens, because there is no better state to stop in.
pub(crate) fn undo_journal(
    journal: &ReadJournal,
    mode: UndoMode,
    monitor: &InstallMonitor,
    phase: InstallPhase,
) -> UndoReport {
    let changes = journal.settled_changes();
    let mut undoer = Undoer {
        folder: &journal.folder,
        mode,
        user_sid: journal.header.user_sid.as_deref(),
        roots: changes
            .iter()
            .filter_map(|change| match change {
                Change::Root { path, real } => Some((root_key(path), real.clone())),
                _ => None,
            })
            .collect(),
        kept_folders: Vec::new(),
        report: UndoReport::default(),
    };
    if journal.damaged {
        undoer.failed(format!(
            "Part of the record of {} could not be read, so anything it described is still in \
             place.",
            journal.header.app_name
        ));
    }
    // The app's entry in Settings goes last and only if all else went, so that an uninstall
    // that could not finish can be run again from where it was started.
    let (entry_changes, other_changes): (Vec<&Change>, Vec<&Change>) = changes
        .iter()
        .partition(|change| is_uninstall_entry(change));
    let total = changes.len();
    let mut done = 0;
    for change in other_changes.into_iter().rev() {
        monitor.report(InstallStep::new(phase, done, total));
        undoer.undo(change);
        done += 1;
    }
    if undoer.report.failed.is_empty() {
        for change in entry_changes.into_iter().rev() {
            monitor.report(InstallStep::new(phase, done, total));
            undoer.undo(change);
            done += 1;
        }
    } else if !entry_changes.is_empty() {
        undoer.report.kept.push(
            "The app's entry in Settings stays, so that this can be run again from there."
                .to_owned(),
        );
    }
    monitor.report(InstallStep::new(phase, total, total));
    undoer.report
}

fn is_uninstall_entry(change: &Change) -> bool {
    matches!(
        change,
        Change::RegistryValue {
            item: ItemKind::UninstallEntry,
            ..
        } | Change::RegistryKeyCreated {
            item: ItemKind::UninstallEntry,
            ..
        }
    )
}

impl Undoer<'_> {
    fn undone(&mut self, text: String) {
        info!(what = %text, "undone");
        self.report.undone.push(text);
    }

    fn kept(&mut self, text: String) {
        info!(what = %text, "left in place");
        self.report.kept.push(text);
    }

    fn failed(&mut self, text: String) {
        warn!(what = %text, "could not be undone");
        self.report.failed.push(text);
    }

    /// True when user data and license items stay.
    fn keeps(&self, item: ItemKind) -> bool {
        matches!(
            self.mode,
            UndoMode::Uninstall {
                remove_user_data: false
            }
        ) && matches!(item, ItemKind::UserData | ItemKind::License)
    }

    fn undo(&mut self, change: &Change) {
        match change {
            Change::Root { .. } | Change::FileWritten { .. } | Change::FileUndone { .. } => {}
            Change::TaskWritten { .. } => {}
            Change::FolderCreated { path, root, item } => self.undo_folder(path, root, *item),
            Change::File {
                path,
                root,
                item,
                backup,
                sha256,
                partial,
            } => self.undo_file(
                path,
                root,
                *item,
                *backup,
                sha256.as_deref(),
                partial.as_deref(),
            ),
            Change::RegistryKeyCreated { hive, key, item } => {
                self.undo_new_key(hive, key, *item);
            }
            Change::RegistryValue {
                hive,
                key,
                name,
                prior,
                written,
                item,
            } => {
                let label = value_label(hive, key, name);
                if self.keeps(*item) {
                    self.kept(format!(
                        "{label} stays, since it was not asked to be removed."
                    ));
                    return;
                }
                self.undo_value(hive, key, name, prior.as_ref(), written, &label);
            }
            Change::EnvironmentValue {
                scope,
                name,
                prior,
                written,
            } => {
                let (hive, key) = environment_location(scope);
                let label = format!("The {} environment variable {name}", scope_name(scope));
                self.undo_value(&hive, key, name, prior.as_ref(), written, &label);
            }
            Change::PathEntries {
                scope,
                name,
                added,
                prior,
                written,
            } => self.undo_path(scope, name, added, prior.as_ref(), written),
            Change::Service {
                name,
                driver,
                binary,
                arguments,
                prior,
            } => self.undo_service(name, *driver, binary, arguments.as_deref(), prior.as_ref()),
            Change::TaskFolderCreated { path } => self.undo_task_folder(path),
            Change::Task {
                task_path,
                task_name,
                prior_xml,
                written_xml,
            } => self.undo_task(
                task_path,
                task_name,
                prior_xml.as_deref(),
                written_xml.as_deref(),
            ),
            Change::Prerequisite { name } => self.report.left_behind.push(format!(
                "{name} was installed and stays, because other programs may be using it."
            )),
        }
    }

    /// Hold the way to `path` and check nothing on it is a link. The folder it is under must
    /// still be where it was, which catches a link anywhere above that folder. Holding it
    /// keeps it and the folders above it from moving, and nothing below it can be swapped
    /// once [`hold_path_below_root`] has it.
    fn guard(&self, root: &Path, path: &Path) -> std::result::Result<Guard, String> {
        let hold = match hold_folder(root) {
            Ok(hold) => hold,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Guard::RootGone),
            Err(error) => return Err(format!("{} could not be opened: {error}", root.display())),
        };
        let expected = self
            .roots
            .get(&root_key(root))
            .ok_or_else(|| format!("there is no record of where {} was", root.display()))?;
        let actual = real_path(root)
            .map_err(|error| format!("{} could not be opened: {error}", root.display()))?;
        if !actual.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "{} now leads to {actual}, not where it was during the install",
                root.display()
            ));
        }
        let mut holds = hold_path_below_root(root, path).map_err(|refusal| refusal.to_string())?;
        holds.push(hold);
        Ok(Guard::Held(holds))
    }

    fn undo_folder(&mut self, path: &Path, root: &Path, item: ItemKind) {
        let _holds = match self.guard(root, path) {
            Ok(Guard::Held(holds)) => holds,
            Ok(Guard::RootGone) => return,
            Err(reason) => {
                self.failed(format!("The folder {} was left: {reason}.", path.display()));
                return;
            }
        };
        match fs::remove_dir(path) {
            Ok(()) => self.undone(format!("Removed the folder {}", path.display())),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) if error.raw_os_error() == Some(ERROR_DIR_NOT_EMPTY) => {
                // A folder this was the parent of has already said so.
                let already_said = self
                    .kept_folders
                    .iter()
                    .any(|kept| path_is_within(kept, path));
                self.kept_folders.push(path.to_path_buf());
                if !already_said && !self.keeps(item) {
                    self.kept(format!(
                        "The folder {} stays, because it holds files this install did not \
                         put there.",
                        path.display()
                    ));
                }
            }
            Err(error) => self.failed(format!(
                "The folder {} could not be removed: {error}.",
                path.display()
            )),
        }
    }

    fn undo_file(
        &mut self,
        path: &Path,
        root: &Path,
        item: ItemKind,
        backup: Option<u64>,
        sha256: Option<&str>,
        partial: Option<&Path>,
    ) {
        let found_backup = backup.and_then(|number| find_backup(self.folder, number));
        let holds = match self.guard(root, path) {
            Ok(Guard::Held(holds)) => holds,
            Ok(Guard::RootGone) => {
                if found_backup.is_some() {
                    self.kept(format!(
                        "{} is not being put back, because the folder it was under has gone.",
                        path.display()
                    ));
                }
                return;
            }
            Err(reason) => {
                self.failed(format!("The file {} was left: {reason}.", path.display()));
                return;
            }
        };
        if let Some(partial) = partial
            && let Err(error) = remove_regular_file(partial)
        {
            self.failed(format!(
                "A partial file could not be removed: {} ({error}).",
                partial.display()
            ));
        }
        let exists = fs::symlink_metadata(path).is_ok();
        let wants_hash = exists && matches!(self.mode, UndoMode::Uninstall { .. });
        let unchanged = if wants_hash && !self.keeps(item) {
            match sha256.map(|expected| {
                sha256_file(path).map(|actual| actual.eq_ignore_ascii_case(expected))
            }) {
                Some(Ok(same)) => Some(same),
                None => Some(false),
                // In use, which says nothing about whether it has changed.
                Some(Err(error)) => {
                    self.failed(format!("{} could not be checked: {error}.", path.display()));
                    return;
                }
            }
        } else {
            None
        };
        let action = file_action(
            self.mode,
            item,
            &FileFacts {
                replaced: backup.is_some(),
                has_backup: found_backup.is_some(),
                exists,
                unchanged,
            },
        );
        match action {
            FileAction::Nothing => {}
            FileAction::Delete => {
                let deleted = make_replaceable(path).and_then(|()| {
                    fs::remove_file(path).map_err(|error| WinAppMigratorError::io(path, error))
                });
                match deleted {
                    Ok(()) => self.undone(format!("Removed {}", path.display())),
                    Err(error) => {
                        self.failed(format!("{} could not be removed: {error}.", path.display()))
                    }
                }
            }
            FileAction::PutBackBackup => {
                let put_back = found_backup
                    .as_ref()
                    .ok_or_else(|| {
                        WinAppMigratorError::InvalidInput("the backup has gone".to_owned())
                    })
                    .and_then(|found| put_back_backup(found, path));
                match put_back {
                    Ok(()) => self.undone(format!("Put back {}", path.display())),
                    Err(error) => self.failed(format!(
                        "{} could not be put back: {error}.",
                        path.display()
                    )),
                }
            }
            FileAction::KeepChanged => {
                let replaced = if found_backup.is_some() {
                    " The file it replaced is not being put back."
                } else {
                    ""
                };
                self.kept(format!(
                    "{} stays, because it has changed since it was installed.{replaced}",
                    path.display()
                ));
            }
            FileAction::KeepByRequest => self.kept(format!(
                "{} stays, since it was not asked to be removed.",
                path.display()
            )),
        }
        drop(holds);
    }

    /// Open a hive for a change made during the install. An install writes to the
    /// `HKEY_CURRENT_USER` of whoever ran it, which is not this one if someone else is
    /// undoing it. Theirs is only there while they are signed in.
    fn hive_root(&self, hive: &RegistryHive) -> io::Result<RegKey> {
        match (hive, self.user_sid) {
            (RegistryHive::CurrentUser, Some(sid)) if current_user_sid() != Some(sid) => {
                RegKey::predef(HKEY_USERS).open_subkey_with_flags(sid, KEY_READ | KEY_SET_VALUE)
            }
            _ => Ok(registry_root(hive)),
        }
    }

    fn undo_new_key(&mut self, hive: &RegistryHive, key: &str, item: ItemKind) {
        let label = format!("{}\\{key}", registry_hive_name(hive));
        if self.keeps(item) {
            return;
        }
        let root = match self.hive_root(hive) {
            Ok(root) => root,
            Err(error) => {
                self.failed(format!("The key {label} could not be reached: {error}."));
                return;
            }
        };
        let info = match root.open_subkey(key).and_then(|opened| opened.query_info()) {
            Ok(info) => info,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return,
            Err(error) => {
                self.failed(format!("The key {label} could not be read: {error}."));
                return;
            }
        };
        if info.values > 0 || info.sub_keys > 0 {
            if item == ItemKind::Program {
                self.kept(format!(
                    "The key {label} stays, because it holds what this install did not put \
                     there."
                ));
            }
            return;
        }
        match root.delete_subkey(key) {
            Ok(()) => self.undone(format!("Removed the key {label}")),
            Err(error) => self.failed(format!("The key {label} could not be removed: {error}.")),
        }
    }

    fn undo_value(
        &mut self,
        hive: &RegistryHive,
        key: &str,
        name: &str,
        prior: Option<&StoredValue>,
        written: &StoredValue,
        label: &str,
    ) {
        let opened = self
            .hive_root(hive)
            .and_then(|root| root.open_subkey_with_flags(key, KEY_READ | KEY_SET_VALUE));
        let handle = match opened {
            Ok(handle) => Some(handle),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => {
                self.failed(format!("{label} could not be reached: {error}."));
                return;
            }
        };
        let current = handle.as_ref().and_then(|handle| {
            handle
                .get_raw_value(name)
                .ok()
                .map(|value| StoredValue::from_reg(&value))
        });
        match value_action(current.as_ref(), written, prior) {
            ValueAction::Nothing => {}
            ValueAction::Keep => self.kept(format!(
                "{label} stays, because it has changed since it was installed."
            )),
            ValueAction::PutBack => {
                let Some(handle) = handle else {
                    return;
                };
                let result = match prior {
                    Some(prior) => match prior.to_reg() {
                        Ok(value) => handle.set_raw_value(name, &value),
                        Err(reason) => Err(io::Error::other(reason)),
                    },
                    None => handle.delete_value(name),
                };
                match result {
                    Ok(()) => self.undone(format!("Put back {label}")),
                    Err(error) => self.failed(format!("{label} could not be put back: {error}.")),
                }
            }
        }
    }

    fn undo_path(
        &mut self,
        scope: &EnvironmentScope,
        name: &str,
        added: &[String],
        prior: Option<&StoredValue>,
        written: &StoredValue,
    ) {
        let label = format!("The {} {name}", scope_name(scope));
        let (hive, key) = environment_location(scope);
        let opened = self
            .hive_root(&hive)
            .and_then(|root| root.open_subkey_with_flags(key, KEY_READ | KEY_SET_VALUE));
        let handle = match opened {
            Ok(handle) => handle,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return,
            Err(error) => {
                self.failed(format!("{label} could not be reached: {error}."));
                return;
            }
        };
        let current = match handle.get_raw_value(name) {
            Ok(value) => StoredValue::from_reg(&value),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return,
            Err(error) => {
                self.failed(format!("{label} could not be read: {error}."));
                return;
            }
        };
        // Exactly as written, so it can go back exactly as it was.
        if current == *written {
            self.undo_value(&hive, key, name, prior, written, &label);
            return;
        }
        let current_value = match current.to_reg() {
            Ok(value) => value,
            Err(reason) => {
                self.failed(format!("{label} could not be read: {reason}."));
                return;
            }
        };
        let text = current_value.to_string();
        let remaining = remove_path_entries(&text, added);
        if remaining == text {
            return;
        }
        // It would not have been there at all without this install, or what was added since.
        let result = if remaining.is_empty() && prior.is_none() {
            handle.delete_value(name)
        } else {
            handle.set_raw_value(
                name,
                &RegValue {
                    bytes: Cow::Owned(encode_utf16_nul(&remaining)),
                    vtype: current_value.vtype,
                },
            )
        };
        match result {
            Ok(()) => self.undone(format!("Took this install's entries out of {label}")),
            Err(error) => self.failed(format!(
                "This install's entries could not be taken out of {label}: {error}."
            )),
        }
    }

    fn undo_service(
        &mut self,
        name: &str,
        driver: bool,
        binary: &Path,
        arguments: Option<&str>,
        prior: Option<&ServiceConfig>,
    ) {
        let kind = if driver { "driver" } else { "service" };
        let current = match service_image_path(name) {
            Ok(Some(current)) => current,
            Ok(None) => return,
            Err(error) => {
                self.failed(format!("The {kind} {name} could not be read: {error}."));
                return;
            }
        };
        if !image_path_matches(&current, binary, arguments, system_root()) {
            if prior.is_none_or(|prior| prior.image_path != current) {
                self.kept(format!(
                    "The {kind} {name} stays, because it has changed since it was installed."
                ));
            }
            return;
        }
        if let Err(reason) = stop_service(name) {
            self.failed(format!("The {kind} {name} could not be stopped: {reason}."));
        }
        match prior {
            Some(prior) => self.restore_service(name, kind, driver, prior),
            None => self.delete_service(name, kind),
        }
    }

    fn delete_service(&mut self, name: &str, kind: &str) {
        let mut command = hidden_command(SC_EXE);
        command.args(["delete", name]);
        if let Err(warning) = run_checked(&mut command, "service_delete_failed", name) {
            self.failed(format!(
                "The {kind} {name} could not be removed: {}.",
                warning.message.trim()
            ));
            return;
        }
        if matches!(service_image_path(name), Ok(Some(_))) {
            self.kept(format!(
                "The {kind} {name} is marked for removal, and goes when the PC restarts or \
                 whatever has it open lets go."
            ));
        } else {
            self.undone(format!("Removed the {kind} {name}"));
        }
    }

    fn restore_service(&mut self, name: &str, kind: &str, driver: bool, prior: &ServiceConfig) {
        let account = ServiceAccount::from_recorded(prior.object_name.as_deref(), name);
        let mut commands = vec![sc_restore_arguments(name, driver, prior, &account)];
        if !driver {
            commands.push(vec![
                "config".into(),
                name.into(),
                "depend=".into(),
                if prior.dependencies.is_empty() {
                    SC_EMPTY_DEPENDENCIES.into()
                } else {
                    prior.dependencies.join("/").into()
                },
            ]);
            commands.push(vec![
                "description".into(),
                name.into(),
                prior.description.as_deref().unwrap_or_default().into(),
            ]);
        }
        for arguments in commands {
            let mut command = hidden_command(SC_EXE);
            command.args(arguments);
            if let Err(warning) = run_checked(&mut command, "service_restore_failed", name) {
                self.failed(format!(
                    "The {kind} {name} could not be put back as it was: {}.",
                    warning.message.trim()
                ));
                return;
            }
        }
        self.undone(format!("Put back the settings of the {kind} {name}"));
        if let ServiceAccount::Custom(account) = account {
            self.kept(format!(
                "The service {name} ran as {account}. Its password is not kept, so set its \
                 account again in Services."
            ));
        }
    }

    fn undo_task_folder(&mut self, path: &str) {
        let script = delete_empty_task_folder_script(path);
        let output = hidden_command(crate::restore::POWERSHELL_EXE)
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output();
        match output {
            Ok(output) if output.status.success() => {}
            Ok(output) if output.status.code() == Some(TASK_FOLDER_NOT_EMPTY_EXIT) => {
                self.kept(format!(
                    "The task folder {path} stays, because it holds tasks this install did not \
                     put there."
                ));
            }
            Ok(output) => self.failed(format!(
                "The task folder {path} could not be removed: {}.",
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            Err(error) => self.failed(format!(
                "The task folder {path} could not be removed: {error}."
            )),
        }
    }

    fn undo_task(
        &mut self,
        task_path: &str,
        task_name: &str,
        prior: Option<&str>,
        written: Option<&str>,
    ) {
        let label = format!("{task_path}{task_name}");
        let live = match live_task_xml(task_path, task_name) {
            Ok(live) => live,
            Err(error) => {
                self.failed(format!("The task {label} could not be read: {error}."));
                return;
            }
        };
        match task_action(live.as_deref(), written, prior) {
            TaskAction::Nothing => {}
            TaskAction::Keep => self.kept(format!(
                "The task {label} stays, because it has changed since it was installed."
            )),
            TaskAction::Delete => {
                match run_powershell_script(&unregister_task_script(task_path, task_name)) {
                    Ok(()) => self.undone(format!("Removed the task {label}")),
                    Err(warning) => self.failed(format!(
                        "The task {label} could not be removed: {}.",
                        warning.message.trim()
                    )),
                }
            }
            TaskAction::PutBack => {
                let Some(xml) = prior else {
                    return;
                };
                let result = write_task_definition(self.folder, xml)
                    .map_err(|error| error.to_string())
                    .and_then(|xml_path| {
                        let registered = run_powershell_script(&register_task_script(
                            &xml_path, task_path, task_name, true,
                        ));
                        let _ = fs::remove_file(&xml_path);
                        registered.map_err(|warning| warning.message)
                    });
                match result {
                    Ok(()) => self.undone(format!("Put back the task {label}")),
                    Err(reason) => self.failed(format!(
                        "The task {label} could not be put back: {}.",
                        reason.trim()
                    )),
                }
            }
        }
    }
}

fn scope_name(scope: &EnvironmentScope) -> &'static str {
    match scope {
        EnvironmentScope::User => "user's",
        EnvironmentScope::Machine => "machine's",
    }
}

fn value_label(hive: &RegistryHive, key: &str, name: &str) -> String {
    format!(
        "The registry value {}\\{key}\\{}",
        registry_hive_name(hive),
        if name.is_empty() { "(Default)" } else { name }
    )
}

/// Put a backup back at `path`, whatever is there now. Going through a rename means a link
/// at `path` is replaced and never followed. A moved backup is the original file. A copied
/// one has to be copied beside its destination so it gets that folder's permissions, not
/// those of the admin-only folder it was in.
pub(crate) fn put_back_backup(backup: &Backup, path: &Path) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!("{} has no parent folder", path.display()))
    })?;
    fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    make_replaceable(path)?;
    match backup {
        Backup::Moved(from) => {
            fs::rename(from, path).map_err(|error| WinAppMigratorError::io(path, error))
        }
        Backup::Copied(from) => {
            let partial = parent.join(partial_file_name());
            let copied = write_partial_copy(from, &partial, false).and_then(|()| {
                fs::rename(&partial, path).map_err(|error| WinAppMigratorError::io(path, error))
            });
            if copied.is_err() {
                let _ = fs::remove_file(&partial);
                return copied;
            }
            make_replaceable(from)?;
            fs::remove_file(from).map_err(|error| WinAppMigratorError::io(from, error))
        }
    }
}

/// Remove a partial file an install was writing, if it is there and is a file. A link is
/// removed, not followed.
fn remove_regular_file(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(()),
        Ok(_) => {
            make_replaceable(path)?;
            fs::remove_file(path).map_err(|error| WinAppMigratorError::io(path, error))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(WinAppMigratorError::io(path, error)),
    }
}

/// Whether `sc.exe query` says a service has stopped. The state's name is not translated.
fn query_shows_stopped(output: &str) -> bool {
    output.contains(STOPPED_STATE)
}

fn stop_service(name: &str) -> std::result::Result<(), String> {
    let is_stopped = || {
        hidden_command(SC_EXE)
            .args(["query", name])
            .output()
            .map(|output| {
                !output.status.success()
                    || query_shows_stopped(&String::from_utf8_lossy(&output.stdout))
            })
            .map_err(|error| error.to_string())
    };
    if is_stopped()? {
        return Ok(());
    }
    let _ = hidden_command(SC_EXE).args(["stop", name]).output();
    let started = Instant::now();
    while started.elapsed() < STOP_SERVICE_TIMEOUT {
        if is_stopped()? {
            return Ok(());
        }
        std::thread::sleep(STOP_SERVICE_POLL);
    }
    Err("it did not stop in time".to_owned())
}

/// The `sc.exe config` arguments that set a service back as it was. A custom account's
/// password is not kept anywhere, so that account cannot be asked for without one.
fn sc_restore_arguments(
    name: &str,
    driver: bool,
    prior: &ServiceConfig,
    account: &ServiceAccount,
) -> Vec<OsString> {
    let mut arguments: Vec<OsString> = vec![
        "config".into(),
        name.into(),
        "binPath=".into(),
        prior.image_path.as_str().into(),
    ];
    if let Some(start) = prior
        .start
        .and_then(|start| start_argument(start, prior.delayed))
    {
        arguments.extend(["start=".into(), start.into()]);
    }
    if let Some(display_name) = prior.display_name.as_deref() {
        arguments.extend(["DisplayName=".into(), display_name.into()]);
    }
    if driver {
        if let Some(kind) = prior.service_type.and_then(driver_type_argument) {
            arguments.extend(["type=".into(), kind.into()]);
        }
    } else if !matches!(
        account,
        ServiceAccount::Custom(_) | ServiceAccount::Unspecified
    ) && let Some(object_name) = account.sc_object_name()
    {
        arguments.extend(["obj=".into(), object_name.into()]);
        if account.sc_takes_empty_password() {
            arguments.extend(["password=".into(), "".into()]);
        }
    }
    arguments
}

fn start_argument(start: u32, delayed: bool) -> Option<&'static str> {
    let label = match (start, delayed) {
        (0, _) => "boot",
        (1, _) => "system",
        (2, true) => crate::service_scan::DELAYED_AUTOMATIC_START_LABEL,
        (2, false) => "automatic",
        (3, _) => "manual",
        (4, _) => "disabled",
        _ => return None,
    };
    sc_start_type(label)
}

fn driver_type_argument(service_type: u32) -> Option<&'static str> {
    if service_type & SERVICE_KERNEL_DRIVER != 0 {
        Some("kernel")
    } else if service_type & SERVICE_FILE_SYSTEM_DRIVER != 0 {
        Some("filesys")
    } else if service_type & SERVICE_RECOGNIZER_DRIVER != 0 {
        Some("rec")
    } else {
        None
    }
}

fn unregister_task_script(task_path: &str, task_name: &str) -> String {
    format!(
        "{}Unregister-ScheduledTask -TaskName '{}' -TaskPath '{}' -Confirm:$false",
        crate::path_utils::POWERSHELL_IMPORT_TASKS,
        escape_powershell_single(task_name),
        escape_powershell_single(task_path),
    )
}

/// Remove a task folder if nothing is left in it. Windows makes one as needed and never takes
/// it away. Already gone is fine.
fn delete_empty_task_folder_script(path: &str) -> String {
    let path = escape_powershell_single(path);
    format!(
        "{POWERSHELL_IMPORT_UTILITY}$s = New-Object -ComObject Schedule.Service; $s.Connect(); \
         try {{ $f = $s.GetFolder('{path}') }} catch {{ exit 0 }}; \
         if ($f.GetTasks(1).Count -gt 0 -or $f.GetFolders(0).Count -gt 0) {{ exit {TASK_FOLDER_NOT_EMPTY_EXIT} }}; \
         $s.GetFolder('\\').DeleteFolder('{path}', 0)"
    )
}

/// Put everything back after an install that failed or was cancelled, and if all of it went
/// back, remove the journal. If not, it stays, so that the next run tries again.
pub(crate) fn roll_back(mut journal: Journal, monitor: &InstallMonitor) -> UndoReport {
    let read = match journal.read_back() {
        Ok(read) => read,
        Err(error) => {
            return UndoReport {
                failed: vec![format!(
                    "The record of this install could not be read, so nothing was undone: {error}."
                )],
                ..UndoReport::default()
            };
        }
    };
    let report = undo_journal(
        &read,
        UndoMode::Rollback,
        monitor,
        InstallPhase::RollingBack,
    );
    journal.close();
    if report.failed.is_empty() {
        remove_install_folder(&read.folder);
    }
    report
}

fn remove_install_folder(folder: &Path) {
    if let Err(error) = fs::remove_dir_all(folder) {
        warn!(folder = %folder.display(), %error, "could not remove an install's record");
    }
}

/// What came of undoing an install that never finished.
pub struct RecoveredInstall {
    pub app_name: String,
    pub report: UndoReport,
}

/// The apps with an install that never finished, which the next install will undo. This only
/// reads. A journal in use belongs to an installer that is still running.
pub fn interrupted_installs(state_root: &Path) -> Vec<String> {
    let Ok(folders) = install_folders(state_root) else {
        return Vec::new();
    };
    folders
        .iter()
        .filter_map(|folder| read_journal(folder).ok())
        .filter(|journal| !journal.committed && !journal_is_in_use(&journal.folder))
        .map(|journal| journal.header.app_name)
        .collect()
}

fn journal_is_in_use(folder: &Path) -> bool {
    lock_journal(folder)
        .err()
        .is_some_and(|error| is_in_use(&error))
}

/// Undo every install that never finished, newest first, since one may have been installed
/// over another. If any cannot be fully undone it stays on record and this fails, because
/// the next install over the top could not be told from it.
pub(crate) fn recover_interrupted_installs(
    state_root: &Path,
    monitor: &InstallMonitor,
) -> Result<Vec<RecoveredInstall>> {
    let mut interrupted: Vec<(JournalLock, ReadJournal)> = Vec::new();
    for folder in install_folders(state_root)? {
        if has_no_journal(&folder) {
            // What is left when an uninstaller could not remove its own file. Fails
            // harmlessly if one is still running.
            let _ = fs::remove_dir_all(&folder);
            continue;
        }
        let lock = match lock_journal(&folder) {
            Ok(lock) => lock,
            Err(error) if is_in_use(&error) => continue,
            Err(error) => return Err(WinAppMigratorError::io(&folder, error)),
        };
        match read_journal(&folder) {
            Ok(journal) if journal.committed => {}
            Ok(journal) => interrupted.push((lock, journal)),
            Err(error) => {
                warn!(folder = %folder.display(), %error, "skipping a record that cannot be read");
            }
        }
    }
    interrupted.sort_by(|left, right| {
        right
            .1
            .header
            .started_at_utc
            .cmp(&left.1.header.started_at_utc)
    });
    let mut recovered = Vec::new();
    let mut stuck = Vec::new();
    for (lock, journal) in interrupted {
        info!(id = %journal.header.id, app = %journal.header.app_name, "undoing an install that never finished");
        let report = undo_journal(
            &journal,
            UndoMode::Rollback,
            monitor,
            InstallPhase::RecoveringEarlierInstall,
        );
        remove_staging_left_behind(&journal);
        if report.failed.is_empty() {
            drop(lock);
            remove_install_folder(&journal.folder);
        } else {
            stuck.push(format!(
                "{}: {}",
                journal.header.app_name,
                report.failed.join(" ")
            ));
        }
        recovered.push(RecoveredInstall {
            app_name: journal.header.app_name.clone(),
            report,
        });
    }
    if !stuck.is_empty() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "an earlier install that never finished could not be fully undone, so nothing was \
             installed. Close whatever is using these and try again. {}",
            stuck.join(" ")
        )));
    }
    Ok(recovered)
}

/// A killed installer also leaves the folder it unpacked into, wherever that was.
fn remove_staging_left_behind(journal: &ReadJournal) {
    let Some(staging) = journal.header.staging.as_deref() else {
        return;
    };
    let is_staging = staging
        .file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with(STAGING_PREFIX));
    if is_staging && hold_real_folder(staging).is_ok() {
        // Dropped straight away: it was only asked whether this is a real folder. Removing
        // a folder never follows a link inside it.
        let _ = fs::remove_dir_all(staging);
    }
}

/// An install on record, opened to be uninstalled: checked, and locked so that nothing else
/// can write to or undo it meanwhile.
pub struct OpenInstall {
    journal: ReadJournal,
    lock: JournalLock,
}

/// Open an install by id, only from the state root. A failure means nothing should be done.
pub fn open_install(state_root: &Path, id: &str) -> Result<OpenInstall> {
    let folder = checked_install_folder(state_root, id)?;
    let lock = lock_journal(&folder).map_err(|error| {
        if is_in_use(&error) {
            WinAppMigratorError::InvalidInput(
                "this install is being changed by another installer or uninstaller".to_owned(),
            )
        } else {
            WinAppMigratorError::io(&folder, error)
        }
    })?;
    let journal = read_journal(&folder)?;
    Ok(OpenInstall { journal, lock })
}

/// What came of an uninstall.
#[derive(Debug, Clone, Default)]
pub struct UninstallOutcome {
    pub report: UndoReport,
    /// Everything went, and the install's own record is being removed.
    pub finished: bool,
    /// The program running this is in this folder, and it is all that is left there. A
    /// running program cannot delete its own file, so this goes when it exits: see
    /// [`schedule_removal_of_running_uninstaller`].
    pub remove_after_exit: Option<PathBuf>,
}

impl OpenInstall {
    pub fn summary(&self) -> crate::journal::InstallSummary {
        self.journal.summary()
    }

    /// Undo this install. If all of it went, the record goes too.
    pub fn uninstall(self, remove_user_data: bool, monitor: &InstallMonitor) -> UninstallOutcome {
        let Self { journal, lock } = self;
        let mode = if journal.committed {
            UndoMode::Uninstall { remove_user_data }
        } else {
            UndoMode::Rollback
        };
        info!(id = %journal.header.id, app = %journal.header.app_name, "uninstalling");
        let report = undo_journal(&journal, mode, monitor, InstallPhase::Uninstalling);
        remove_staging_left_behind(&journal);
        let finished = report.failed.is_empty();
        drop(lock);
        let remove_after_exit = if finished {
            remove_record_of_install(&journal.folder)
        } else {
            None
        };
        UninstallOutcome {
            report,
            finished,
            remove_after_exit,
        }
    }
}

/// Remove an install's folder. If the program doing this is running from it, it cannot delete
/// its own file while it runs, so that and the folder are left, and the folder is returned.
fn remove_record_of_install(folder: &Path) -> Option<PathBuf> {
    let running = std::env::current_exe()
        .ok()
        .filter(|exe| path_is_within(exe, folder));
    let Some(running) = running else {
        remove_install_folder(folder);
        return None;
    };
    let entries = fs::read_dir(folder).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if crate::restore_policy::paths_equal(&path, &running) {
            continue;
        }
        let removed = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };
        if let Err(error) = removed {
            warn!(file = %path.display(), %error, "could not remove part of an install's record");
        }
    }
    Some(folder.to_path_buf())
}

/// The command line that removes an uninstaller's folder once it has exited. It stays on the
/// command line, so it runs nothing from any folder but Windows' own, and `/D` keeps `cmd.exe`
/// from also running the `AutoRun` command in `HKEY_CURRENT_USER`, which is the signed-in
/// user's to set and would otherwise run elevated. The waiting is done by `ping`, by its full
/// path, since this runs before any `PATH` could be trusted. `None` if the path holds what
/// would change the meaning of that line, which only an unusual `ProgramData` could.
fn self_removal_command_line(folder: &Path, windows_dir: &Path) -> Option<String> {
    let folder = folder.to_string_lossy();
    let ping = windows_dir.join("System32").join("ping.exe");
    let ping = ping.to_string_lossy();
    if [folder.as_ref(), ping.as_ref()]
        .iter()
        .any(|text| text.contains(['%', '"', '^', '\n', '\r']))
    {
        return None;
    }
    Some(format!(
        "/D /S /C \"for /L %i in (1,1,{SELF_REMOVAL_TRIES}) do @(rd /s /q \"{folder}\" 2>nul & \
         if not exist \"{folder}\" exit /b 0 & \"{ping}\" -n 2 127.0.0.1 >nul)\""
    ))
}

/// Have `cmd.exe` from the Windows folder remove `folder` once this process has exited. This
/// is the one thing that outlives an uninstaller, and it runs from its own folder, on a path
/// only administrators can write to. If it cannot, the next install sweeps the folder away.
pub fn schedule_removal_of_running_uninstaller(folder: &Path) -> io::Result<()> {
    use std::os::windows::process::CommandExt;

    let Some(command_line) = self_removal_command_line(folder, system_root()) else {
        return Err(io::Error::other(format!(
            "{} cannot be put on a command line safely",
            folder.display()
        )));
    };
    let mut command: Command = hidden_command("cmd");
    command
        .raw_arg(command_line)
        .current_dir(system_root().join("System32"))
        // Nothing inherited, or whoever is reading this program's output would wait for it.
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command.spawn().map(drop)
}

/// A summary of a report for the log and for automation.
pub fn report_lines(report: &UndoReport) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Undone: {}", report.undone.len()));
    lines.push(format!("Could not be undone: {}", report.failed.len()));
    lines.extend(report.failed.iter().map(|line| format!("  {line}")));
    lines.push(format!("Left in place: {}", report.kept.len()));
    lines.extend(report.kept.iter().map(|line| format!("  {line}")));
    lines.push(format!("Left behind: {}", report.left_behind.len()));
    lines.extend(report.left_behind.iter().map(|line| format!("  {line}")));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEEP: UndoMode = UndoMode::Uninstall {
        remove_user_data: false,
    };
    const REMOVE: UndoMode = UndoMode::Uninstall {
        remove_user_data: true,
    };

    fn facts(replaced: bool, has_backup: bool, exists: bool, unchanged: Option<bool>) -> FileFacts {
        FileFacts {
            replaced,
            has_backup,
            exists,
            unchanged,
        }
    }

    #[test]
    fn a_rollback_takes_back_everything_it_wrote_whatever_is_in_it() {
        let program = ItemKind::Program;
        let cases = [
            // New, and there.
            (facts(false, false, true, None), FileAction::Delete),
            // New, and it never got that far, or a rollback was cut off after this one.
            (facts(false, false, false, None), FileAction::Nothing),
            // Replaced, and the backup is there, whether or not the new file made it.
            (facts(true, true, true, None), FileAction::PutBackBackup),
            (facts(true, true, false, None), FileAction::PutBackBackup),
            // Replaced, with no backup: never got as far, or was already put back.
            (facts(true, false, true, None), FileAction::Nothing),
            (facts(true, false, false, None), FileAction::Nothing),
        ];
        for (facts, expected) in cases {
            assert_eq!(file_action(UndoMode::Rollback, program, &facts), expected);
        }
        // User data goes as well, since that install never finished.
        assert_eq!(
            file_action(
                UndoMode::Rollback,
                ItemKind::UserData,
                &facts(false, false, true, None)
            ),
            FileAction::Delete
        );
    }

    #[test]
    fn an_uninstall_only_takes_what_is_still_what_it_wrote() {
        let program = ItemKind::Program;
        let cases = [
            (facts(false, false, true, Some(true)), FileAction::Delete),
            (
                facts(false, false, true, Some(false)),
                FileAction::KeepChanged,
            ),
            // The hash is not known, so it cannot be said to be unchanged.
            (facts(false, false, true, None), FileAction::KeepChanged),
            (facts(false, false, false, None), FileAction::Nothing),
            (
                facts(true, true, true, Some(true)),
                FileAction::PutBackBackup,
            ),
            (
                facts(true, true, true, Some(false)),
                FileAction::KeepChanged,
            ),
            // Removed by the user, and what it replaced still comes back.
            (facts(true, true, false, None), FileAction::PutBackBackup),
            // Run a second time, after it was put back, with a different hash from what was
            // installed. It must not call that a change.
            (facts(true, false, true, Some(false)), FileAction::Nothing),
        ];
        for (facts, expected) in cases {
            assert_eq!(file_action(KEEP, program, &facts), expected);
        }
    }

    #[test]
    fn user_data_and_licenses_stay_unless_asked_for() {
        for item in [ItemKind::UserData, ItemKind::License] {
            assert_eq!(
                file_action(KEEP, item, &facts(false, false, true, Some(true))),
                FileAction::KeepByRequest
            );
            assert_eq!(
                file_action(KEEP, item, &facts(true, true, true, Some(true))),
                FileAction::KeepByRequest
            );
            assert_eq!(
                file_action(KEEP, item, &facts(false, false, false, None)),
                FileAction::Nothing
            );
            assert_eq!(
                file_action(REMOVE, item, &facts(false, false, true, Some(true))),
                FileAction::Delete
            );
            assert_eq!(
                file_action(REMOVE, item, &facts(false, false, true, Some(false))),
                FileAction::KeepChanged
            );
        }
    }

    fn stored(text: &str) -> StoredValue {
        StoredValue {
            kind: 1,
            data_hex: text.to_owned(),
        }
    }

    #[test]
    fn a_value_goes_back_only_if_it_still_holds_what_was_written() {
        let (before, written, other) = (stored("aa"), stored("bb"), stored("cc"));
        for (current, prior, expected) in [
            (Some(&written), Some(&before), ValueAction::PutBack),
            (Some(&written), None, ValueAction::PutBack),
            // Never written, or already put back.
            (Some(&before), Some(&before), ValueAction::Nothing),
            (None, None, ValueAction::Nothing),
            // Someone else changed it, or removed it.
            (Some(&other), Some(&before), ValueAction::Keep),
            (Some(&other), None, ValueAction::Keep),
            (None, Some(&before), ValueAction::Keep),
        ] {
            assert_eq!(value_action(current, &written, prior), expected);
        }
    }

    #[test]
    fn a_task_goes_only_if_it_is_still_what_was_registered() {
        for (live, written, prior, expected) in [
            (None, Some("new"), None, TaskAction::Nothing),
            (Some("new"), Some("new"), None, TaskAction::Delete),
            (Some("new\r\n"), Some("new"), None, TaskAction::Delete),
            (Some("new"), Some("new"), Some("old"), TaskAction::PutBack),
            // Registered but cut off before it could be read back.
            (Some("new"), None, None, TaskAction::Delete),
            (Some("new"), None, Some("old"), TaskAction::PutBack),
            // Never got as far, or already put back.
            (Some("old"), None, Some("old"), TaskAction::Nothing),
            (Some("old"), Some("new"), Some("old"), TaskAction::Nothing),
            // Someone changed it since.
            (Some("other"), Some("new"), None, TaskAction::Keep),
            (Some("other"), Some("new"), Some("old"), TaskAction::Keep),
        ] {
            assert_eq!(
                task_action(live, written, prior),
                expected,
                "{live:?} {written:?} {prior:?}"
            );
        }
    }

    #[test]
    fn a_stopped_service_is_recognized_from_what_sc_says() {
        assert!(query_shows_stopped(
            "SERVICE_NAME: X\r\n        STATE              : 1  STOPPED\r\n"
        ));
        assert!(!query_shows_stopped(
            "SERVICE_NAME: X\r\n        STATE              : 4  RUNNING\r\n"
        ));
    }

    #[test]
    fn a_service_goes_back_with_its_delayed_start_and_account() {
        let prior = ServiceConfig {
            image_path: r#""C:\App\svc.exe" --run"#.to_owned(),
            start: Some(2),
            delayed: true,
            object_name: Some(r"NT AUTHORITY\LocalService".to_owned()),
            display_name: Some("Acme".to_owned()),
            description: None,
            dependencies: Vec::new(),
            service_type: Some(0x10),
        };
        let account = ServiceAccount::from_recorded(prior.object_name.as_deref(), "Acme");
        let arguments: Vec<String> = sc_restore_arguments("Acme", false, &prior, &account)
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            arguments,
            [
                "config",
                "Acme",
                "binPath=",
                r#""C:\App\svc.exe" --run"#,
                "start=",
                "delayed-auto",
                "DisplayName=",
                "Acme",
                "obj=",
                r"NT AUTHORITY\LocalService",
                "password=",
                "",
            ]
        );
    }

    #[test]
    fn a_custom_account_is_not_asked_for_without_a_password() {
        let prior = ServiceConfig {
            image_path: r"C:\App\svc.exe".to_owned(),
            object_name: Some(r"CORP\someone".to_owned()),
            ..ServiceConfig::default()
        };
        let account = ServiceAccount::from_recorded(prior.object_name.as_deref(), "Acme");
        let arguments = sc_restore_arguments("Acme", false, &prior, &account);
        assert!(!arguments.iter().any(|argument| argument == "obj="));
    }

    #[test]
    fn a_driver_goes_back_with_its_type_and_no_account() {
        let prior = ServiceConfig {
            image_path: r"\??\C:\App\flt.sys".to_owned(),
            start: Some(3),
            service_type: Some(SERVICE_FILE_SYSTEM_DRIVER),
            ..ServiceConfig::default()
        };
        let arguments: Vec<String> =
            sc_restore_arguments("AcmeFlt", true, &prior, &ServiceAccount::Unspecified)
                .iter()
                .map(|argument| argument.to_string_lossy().into_owned())
                .collect();
        assert_eq!(
            arguments,
            [
                "config",
                "AcmeFlt",
                "binPath=",
                r"\??\C:\App\flt.sys",
                "start=",
                "demand",
                "type=",
                "filesys",
            ]
        );
    }

    #[test]
    fn the_removal_command_runs_nothing_from_a_user_writable_place()
    -> std::result::Result<(), String> {
        let line = self_removal_command_line(
            Path::new(r"C:\ProgramData\WinAppMigrator\installs\wam-abc"),
            Path::new(r"C:\Windows"),
        )
        .ok_or("it should be allowed")?;
        assert!(line.starts_with("/D /S /C "), "{line}");
        assert!(line.contains(r#"rd /s /q "C:\ProgramData\WinAppMigrator\installs\wam-abc""#));
        assert!(line.contains(r#""C:\Windows\System32\ping.exe""#));
        for bad in [
            r"C:\Program%TEMP%Data\wam-abc",
            "C:\\ProgramData\\wam\"&calc&\"",
            "C:\\ProgramData\\wam^",
        ] {
            assert!(
                self_removal_command_line(Path::new(bad), Path::new(r"C:\Windows")).is_none(),
                "{bad}"
            );
        }
        Ok(())
    }

    #[test]
    fn scripts_escape_a_task_name_and_a_task_folder() {
        let script = unregister_task_script(r"\Acme's\", "Sync's");
        assert!(script.contains("-TaskName 'Sync''s'"));
        assert!(script.contains(r"-TaskPath '\Acme''s\'"));
        assert!(script.contains("-Confirm:$false"));
        let script = delete_empty_task_folder_script(r"\Acme's");
        assert!(script.contains(r"GetFolder('\Acme''s')"));
        assert!(script.contains(r"DeleteFolder('\Acme''s', 0)"));
    }
}
