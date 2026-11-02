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
//! A rollback in the same run takes back everything the install wrote, whatever is in it: it
//! only just wrote it, and it may be what failed. An uninstall, or undoing an install that never
//! finished, can come long after, so it deletes a file only if it is still what the install
//! wrote. Either way, a registry value or environment variable that no longer holds what the
//! install wrote belongs to someone else now, and is left alone.
//!
//! Whoever can write to a folder can act between a check and what follows it, and can make what
//! follows it wait. So a delete or a restore is done with the way there pinned, in
//! [`pin_path_below_root`], which keeps it from becoming a link for as long as that takes. If
//! pinning finds a folder changing under it, nothing more is done under that root in this run,
//! because trying again and again is what a race needs.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use tracing::{info, warn};
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_READ, KEY_SET_VALUE, KEY_WRITE,
};
use winreg::{RegKey, RegValue};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    PinnedPath, expand_env_vars, hold_path_below_root, pin_path_below_root,
};
use crate::hashing::sha256_file;
use crate::journal::{
    Backup, Change, ItemKind, Journal, JournalLock, ReadJournal, STAGING_PREFIX, ServiceConfig,
    StoredValue, checked_install_folder, find_backup, has_no_journal, install_folders, is_in_use,
    lock_journal, lock_state, read_journal, real_path,
};
use crate::models::{EnvironmentScope, RegistryHive, UndoReport};
use crate::path_utils::{
    POWERSHELL_IMPORT_UTILITY, current_user_sid, escape_powershell_single, hidden_command,
    path_is_within, system_root, text_mentions_path,
};
use crate::privilege::{confirm_protected_folder, hold_folder, hold_real_folder, is_pin_file_name};
use crate::progress::{InstallMonitor, InstallPhase, InstallStep};
use crate::registry_target::{
    environment_location, registry_hive_name, remove_path_entries, service_image_path,
    stored_firewall_rules,
};
use crate::restore::{
    SC_EXE, encode_utf16_nul, make_replaceable, register_task_script, run_checked,
    run_powershell_script, sc_start_type, write_partial_copy, write_task_definition,
};
use crate::restore_policy::{ServiceAccount, existing_task_refusal, image_path_matches};
use crate::restore_policy::{
    firewall_fingerprint, netsh_delete_rule_arguments, stored_firewall_rule_matches,
};
use crate::task_scan::live_task_xml;

const STOP_SERVICE_TIMEOUT: Duration = Duration::from_secs(30);
const STOP_SERVICE_POLL: Duration = Duration::from_millis(250);
const STOPPED_STATE: &str = "STOPPED";
const STOPPED_STATE_NUMBER: &str = "1";
/// What `sc.exe delete` says of one that has already been asked for and is waiting for a restart.
const SC_ALREADY_MARKED_FOR_DELETE: &str = "1072";
const SC_EMPTY_DEPENDENCIES: &str = "/";
const ERROR_DIR_NOT_EMPTY: i32 = 145;
const PIN_LINGER_TRIES: u32 = 10;
const PIN_LINGER_POLL: Duration = Duration::from_millis(50);
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
    /// The install has just failed, so all of what it wrote goes.
    Rollback,
    /// An earlier install never finished. All of it goes, but this may be long after, so what
    /// has changed since is not its to remove.
    Recovery,
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
        _ if facts.unchanged == Some(true) => replace_or_delete,
        _ => FileAction::KeepChanged,
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

/// `runs_only_the_app` says the task has nothing to run outside the install location.
fn task_action(
    live: Option<&str>,
    written: Option<&str>,
    prior: Option<&str>,
    runs_only_the_app: bool,
) -> TaskAction {
    let Some(live) = live else {
        return TaskAction::Nothing;
    };
    let same = |left: &str, right: &str| left.trim() == right.trim();
    let is_written = written.is_some_and(|written| same(written, live));
    if !is_written && prior.is_some_and(|prior| same(prior, live)) {
        return TaskAction::Nothing;
    }
    if written.is_some() && !is_written {
        return TaskAction::Keep;
    }
    // What the install registered is unknown if it was cut off right after doing so, or never
    // got that far, in which case this one is someone else's unless it runs what was installed.
    if written.is_none() && !runs_only_the_app {
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
    /// The whole drive has gone, and may come back.
    DriveGone,
    /// The record of the folder it was all under could not be read.
    RootUnknown,
}

struct Undoer<'a> {
    folder: &'a Path,
    target_root: &'a Path,
    mode: UndoMode,
    user_sid: Option<&'a str>,
    roots: HashMap<String, String>,
    kept_folders: Vec<PathBuf>,
    damaged: bool,
    roots_left_unseen: HashSet<String>,
    /// Under these, a folder changed while it was being made safe to act in.
    roots_being_changed: HashSet<String>,
    /// What stays, as text, because it has changed since: a service's command, a task, a value.
    /// A file any of it names stays too, or it would be left calling for a file that is gone,
    /// from a folder others may be able to write to.
    kept_mentions: Vec<String>,
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
    let mut undoer = undoer_for(journal, &changes, mode);
    if journal.damaged {
        // Said, and not held against it. It cannot get better by trying again, and it must
        // not stop every later install.
        undoer.kept(format!(
            "Part of the record of {} could not be read, so anything it described is still in \
             place, and a file it replaced may not come back.",
            journal.header.app_name
        ));
    }
    // The app's entry in Settings goes last and only if all else went, so that an uninstall
    // that could not finish can be run again from where it was started.
    let (entry_changes, other_changes): (Vec<&Change>, Vec<&Change>) = changes
        .iter()
        .partition(|change| change.is_uninstall_entry());
    let total = changes.len();
    let mut done = 0;
    for change in other_changes.into_iter().rev() {
        monitor.report(InstallStep::new(phase, done, total));
        undoer.undo(change);
        done += 1;
    }
    // An install that never finished is finished by the next install, not from Settings, and what
    // it wrote there must not be left for it to take for its own.
    let run_again_from_settings = matches!(mode, UndoMode::Uninstall { .. });
    if undoer.report.failed.is_empty()
        && !(undoer.report.waits_for_a_drive() && run_again_from_settings)
    {
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

/// What to add to a record after undoing all of it that could be reached, if some could not. It
/// stays on record for that, and the rest must not be undone again when it is back.
fn what_is_left_of(report: &UndoReport, mode: UndoMode) -> Option<Change> {
    (report.failed.is_empty() && report.waits_for_a_drive()).then(|| Change::OnlyLeftOn {
        roots: report.drives_waited_for.clone(),
        keeps_entry: matches!(mode, UndoMode::Uninstall { .. }),
    })
}

fn undoer_for<'a>(journal: &'a ReadJournal, changes: &[Change], mode: UndoMode) -> Undoer<'a> {
    Undoer {
        folder: &journal.folder,
        target_root: &journal.header.target_root,
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
        damaged: journal.damaged,
        roots_left_unseen: HashSet::new(),
        roots_being_changed: HashSet::new(),
        kept_mentions: Vec::new(),
        report: UndoReport::default(),
    }
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
            Change::TaskWritten { .. }
            | Change::PrerequisiteSkipped { .. }
            | Change::OnlyLeftOn { .. } => {}
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
                    let replaced = if prior.is_some() {
                        " What it replaced is not being put back."
                    } else {
                        ""
                    };
                    self.kept(format!(
                        "{label} stays, since it was not asked to be removed.{replaced}"
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
            Change::FirewallRule {
                name,
                direction,
                program,
                fingerprint,
            } => self.undo_firewall_rule(name, direction, program, fingerprint),
            Change::Prerequisite { name } => self.report.left_behind.push(format!(
                "{name} was installed, or may have been, and stays, because other programs may \
                 be using it."
            )),
        }
    }

    /// Hold the way to `path` and check nothing on it is a link. The folder it is under must
    /// still be where it was, which catches a link anywhere above that folder. Holding it
    /// keeps it and the folders above it from being renamed, and [`hold_path_below_root`] does
    /// the same for what is below. A folder that is not there means what was in it has gone,
    /// unless it is a whole drive that is not there, in which case it may come back.
    fn guard(&self, root: &Path, path: &Path) -> std::result::Result<Guard, String> {
        if self.roots_being_changed.contains(&root_key(root)) {
            return Err(format!(
                "something was changing {} while this ran, so nothing more was done in it. Close \
                 what is doing that, and run this again",
                root.display()
            ));
        }
        let hold = match hold_folder(root) {
            Ok(hold) => hold,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(if drive_is_connected(root) {
                    Guard::RootGone
                } else {
                    Guard::DriveGone
                });
            }
            Err(error) => return Err(format!("{} could not be opened: {error}", root.display())),
        };
        if self.damaged && !self.roots.contains_key(&root_key(root)) {
            return Ok(Guard::RootUnknown);
        }
        self.confirm_root_is_where_it_was(root)?;
        let mut holds = hold_path_below_root(root, path).map_err(|refusal| refusal.to_string())?;
        holds.push(hold);
        Ok(Guard::Held(holds))
    }

    fn confirm_root_is_where_it_was(&self, root: &Path) -> std::result::Result<(), String> {
        let expected = self
            .roots
            .get(&root_key(root))
            .ok_or_else(|| format!("there is no record of where {} was", root.display()))?;
        let actual = real_path(root)
            .map_err(|error| format!("{} could not be opened: {error}", root.display()))?;
        if actual.eq_ignore_ascii_case(expected) {
            return Ok(());
        }
        Err(format!(
            "{} now leads to {actual}, not where it was during the install",
            root.display()
        ))
    }

    /// Pin the way to `path` for what is about to be done there. With everything on the way
    /// held, the folder it is all under cannot move either, so that is asked last. [`Self::guard`]
    /// found no link here a moment ago, so one found now means someone is changing this as it
    /// runs, and it is not given another try.
    fn pin(&mut self, root: &Path, path: &Path) -> std::result::Result<PinnedPath, String> {
        let pinned = pin_path_below_root(root, path).map_err(|refusal| {
            if matches!(refusal, WinAppMigratorError::InvalidInput(_)) {
                self.roots_being_changed.insert(root_key(root));
            }
            refusal.to_string()
        })?;
        self.confirm_root_is_where_it_was(root)?;
        Ok(pinned)
    }

    /// Note what stays, as it would be read when it runs.
    fn note_kept_text(&mut self, text: &str) {
        self.kept_mentions
            .push(expand_env_vars(text).replace("&amp;", "&"));
    }

    /// Whether what stays still names this file.
    fn is_still_named(&self, path: &Path) -> bool {
        !matches!(self.mode, UndoMode::Rollback)
            && self
                .kept_mentions
                .iter()
                .any(|text| text_mentions_path(text, path, true))
    }

    /// Whether what is under `root` has to be left without looking at it. Said once for each
    /// folder, because it can be every file of an install.
    fn left_unseen(&mut self, guard: &Guard, root: &Path) -> bool {
        let text = match guard {
            Guard::DriveGone => {
                if !self
                    .report
                    .drives_waited_for
                    .iter()
                    .any(|left| left == root)
                {
                    self.report.drives_waited_for.push(root.to_path_buf());
                }
                format!(
                    "What this install put in {} is on a drive that is not connected. Connect \
                     it, and this can be finished.",
                    root.display()
                )
            }
            Guard::RootUnknown => format!(
                "What this install put in {} stays, because that part of the record could not \
                 be read.",
                root.display()
            ),
            Guard::RootGone | Guard::Held(_) => return false,
        };
        if self.roots_left_unseen.insert(root_key(root)) {
            self.kept(text);
        }
        true
    }

    fn undo_folder(&mut self, path: &Path, root: &Path, item: ItemKind) {
        let what = format!("The folder {}", path.display());
        let guard = match self.guard(root, path) {
            Ok(guard) => guard,
            Err(reason) => {
                self.failed(format!("{what} was left: {reason}."));
                return;
            }
        };
        if self.left_unseen(&guard, root) || matches!(guard, Guard::RootGone) {
            return;
        }
        if matches!(fs::symlink_metadata(path), Err(error) if error.kind() == io::ErrorKind::NotFound)
        {
            return;
        }
        let pinned = match self.pin(root, path) {
            Ok(pinned) => pinned,
            Err(reason) => {
                self.failed(format!("{what} was left: {reason}."));
                return;
            }
        };
        let removed = remove_folder_if_nothing_real_is_in_it(path);
        drop(pinned);
        match removed {
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
        let what = format!("The file {}", path.display());
        let guard = match self.guard(root, path) {
            Ok(guard) => guard,
            Err(reason) => {
                self.failed(format!("{what} was left: {reason}."));
                return;
            }
        };
        if self.left_unseen(&guard, root) {
            return;
        }
        let holds = match guard {
            Guard::Held(holds) => holds,
            _ => {
                if found_backup.is_some() {
                    self.kept(format!(
                        "{} is not being put back, because the folder it was under has gone.",
                        path.display()
                    ));
                }
                return;
            }
        };
        if let Some(partial) = partial.filter(|partial| fs::symlink_metadata(partial).is_ok()) {
            let removed = self.pin(root, partial).and_then(|_pinned| {
                remove_regular_file(partial).map_err(|error| error.to_string())
            });
            if let Err(error) = removed {
                self.failed(format!(
                    "A partial file could not be removed: {} ({error}).",
                    partial.display()
                ));
            }
        }
        let exists = fs::symlink_metadata(path).is_ok();
        let wants_hash = exists && !matches!(self.mode, UndoMode::Rollback);
        let unchanged = if wants_hash && !self.keeps(item) {
            match sha256.map(|expected| {
                sha256_file(path).map(|actual| actual.eq_ignore_ascii_case(expected))
            }) {
                Some(Ok(same)) => Some(same),
                // Only a shortcut has no hash until it is made. Without one in an install that
                // never finished, it stopped in the moment after, so this is the install's. In
                // one that did, there is nothing to say it is unchanged.
                None => Some(matches!(self.mode, UndoMode::Recovery)),
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
        if matches!(action, FileAction::Delete | FileAction::PutBackBackup)
            && self.is_still_named(path)
        {
            self.kept(format!(
                "{} stays, because something that stays still uses it.{}",
                path.display(),
                if found_backup.is_some() {
                    " The file it replaced is not being put back."
                } else {
                    ""
                }
            ));
            return;
        }
        match action {
            FileAction::Nothing => {}
            FileAction::Delete => {
                let pinned = match self.pin(root, path) {
                    Ok(pinned) => pinned,
                    Err(reason) => {
                        self.failed(format!("{what} was left: {reason}."));
                        return;
                    }
                };
                let deleted = make_replaceable(path).and_then(|()| {
                    fs::remove_file(path).map_err(|error| WinAppMigratorError::io(path, error))
                });
                drop(pinned);
                match deleted {
                    Ok(()) => self.undone(format!("Removed {}", path.display())),
                    Err(error) => {
                        self.failed(format!("{} could not be removed: {error}.", path.display()))
                    }
                }
            }
            FileAction::PutBackBackup => {
                let pinned = match self.pin(root, path) {
                    Ok(pinned) => pinned,
                    Err(reason) => {
                        self.failed(format!("{what} was left: {reason}."));
                        return;
                    }
                };
                let put_back = found_backup
                    .as_ref()
                    .ok_or_else(|| {
                        WinAppMigratorError::InvalidInput("the backup has gone".to_owned())
                    })
                    .and_then(|found| put_back_backup(found, path));
                drop(pinned);
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
            FileAction::KeepByRequest => {
                let replaced = if found_backup.is_some() {
                    " The file it replaced is not being put back."
                } else {
                    ""
                };
                self.kept(format!(
                    "{} stays, since it was not asked to be removed.{replaced}",
                    path.display()
                ));
            }
        }
        drop(holds);
    }

    /// Open a hive for a change made during the install. An install writes to the hive of
    /// whoever was signed in, which is not this one if someone else is undoing it, or if it was
    /// run as another administrator. Theirs is only there while they are signed in, and if they
    /// are not, what is in it stays, and this says so once `None` comes back. A record with no
    /// account on it is from whoever is running this.
    fn hive_root(&self, hive: &RegistryHive) -> io::Result<Option<RegKey>> {
        match (hive, self.user_sid) {
            (RegistryHive::LocalMachine, _) => Ok(Some(RegKey::predef(HKEY_LOCAL_MACHINE))),
            (RegistryHive::CurrentUser, None) => Ok(Some(RegKey::predef(HKEY_CURRENT_USER))),
            (RegistryHive::CurrentUser, Some(sid)) => {
                match RegKey::predef(HKEY_USERS).open_subkey_with_flags(sid, KEY_READ | KEY_WRITE) {
                    Ok(root) => Ok(Some(root)),
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        Ok((current_user_sid() == Some(sid))
                            .then(|| RegKey::predef(HKEY_CURRENT_USER)))
                    }
                    Err(error) => Err(error),
                }
            }
        }
    }

    fn kept_for_another_account(&mut self, label: &str) {
        self.kept(format!(
            "{label} stays, because it belongs to the account that installed this, which is not \
             signed in."
        ));
    }

    fn undo_new_key(&mut self, hive: &RegistryHive, key: &str, item: ItemKind) {
        let label = format!("{}\\{key}", registry_hive_name(hive));
        if self.keeps(item) {
            return;
        }
        let root = match self.hive_root(hive) {
            Ok(Some(root)) => root,
            Ok(None) => {
                self.kept_for_another_account(&format!("The key {label}"));
                return;
            }
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
        let root = match self.hive_root(hive) {
            Ok(Some(root)) => root,
            Ok(None) => {
                self.kept_for_another_account(label);
                return;
            }
            Err(error) => {
                self.failed(format!("{label} could not be reached: {error}."));
                return;
            }
        };
        let handle = match root.open_subkey_with_flags(key, KEY_READ | KEY_SET_VALUE) {
            Ok(handle) => Some(handle),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => {
                self.failed(format!("{label} could not be reached: {error}."));
                return;
            }
        };
        let current = match handle.as_ref().map(|handle| handle.get_raw_value(name)) {
            Some(Ok(value)) => Some(StoredValue::from_reg(&value)),
            None => None,
            Some(Err(error)) if error.kind() == io::ErrorKind::NotFound => None,
            // Not knowing is not the same as it not being there.
            Some(Err(error)) => {
                self.failed(format!("{label} could not be read: {error}."));
                return;
            }
        };
        match value_action(current.as_ref(), written, prior) {
            ValueAction::Nothing => {}
            ValueAction::Keep => {
                if let Some(Ok(value)) = current.as_ref().map(StoredValue::to_reg) {
                    self.note_kept_text(&value.to_string());
                }
                self.kept(format!(
                    "{label} stays, because it has changed since it was installed."
                ));
            }
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
        let root = match self.hive_root(&hive) {
            Ok(Some(root)) => root,
            Ok(None) => {
                self.kept_for_another_account(&label);
                return;
            }
            Err(error) => {
                self.failed(format!("{label} could not be reached: {error}."));
                return;
            }
        };
        let handle = match root.open_subkey_with_flags(key, KEY_READ | KEY_SET_VALUE) {
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
                self.note_kept_text(&current);
                self.kept(format!(
                    "The {kind} {name} stays, because it has changed since it was installed."
                ));
            }
            return;
        }
        // One that will not stop is still dealt with, in the way Windows allows: it is removed
        // when the PC restarts, or goes back to how it was when it next starts. There may be
        // nothing that makes it stop, so this must not be what keeps an uninstall from finishing.
        let was_running = match stop_service(name) {
            Ok(was_running) => was_running,
            Err(reason) => {
                self.kept(format!(
                    "The {kind} {name} could not be stopped ({reason}), so what it is using may \
                     stay until the PC restarts."
                ));
                false
            }
        };
        let failed_before = self.report.failed.len();
        match prior {
            Some(prior) => {
                self.restore_service(name, kind, driver, prior);
                if was_running {
                    self.kept(format!(
                        "The {kind} {name} was stopped to do this. Start it again when you need it."
                    ));
                }
            }
            None => self.delete_service(name, kind),
        }
        // Still set to run that, until this is tried again.
        if self.report.failed.len() > failed_before {
            self.note_kept_text(&current);
        }
    }

    fn delete_service(&mut self, name: &str, kind: &str) {
        let mut command = hidden_command(SC_EXE);
        command.args(["delete", name]);
        if let Err(warning) = run_checked(&mut command, "service_delete_failed", name)
            && !warning.message.contains(SC_ALREADY_MARKED_FOR_DELETE)
        {
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

    /// Take out a rule this install added. One that says something else by now is someone's, unless
    /// the install failed just now, and a rule of the same name for anything else is not touched.
    fn undo_firewall_rule(
        &mut self,
        name: &str,
        direction: &str,
        program: &Path,
        fingerprint: &str,
    ) {
        let label = format!("The firewall rule {name}");
        let stored = match stored_firewall_rules() {
            Ok(stored) => stored,
            Err(error) if error.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(error) => {
                self.failed(format!("{label} could not be read: {error}."));
                return;
            }
        };
        let found: Vec<&String> = stored
            .iter()
            .filter(|rule| stored_firewall_rule_matches(rule, name, direction, program))
            .collect();
        if found.is_empty() {
            return;
        }
        let as_it_was = found.iter().any(|rule| {
            crate::integration_scan::parse_firewall_rule(rule)
                .is_some_and(|rule| firewall_fingerprint(&rule) == fingerprint)
        });
        if !as_it_was && !matches!(self.mode, UndoMode::Rollback) {
            self.note_kept_text(&program.display().to_string());
            self.kept(format!(
                "{label} stays, because it has changed since it was installed."
            ));
            return;
        }
        let deleted = hidden_command("netsh.exe")
            .args(netsh_delete_rule_arguments(name, direction, program))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        match deleted {
            Ok(status) if status.success() => self.undone(format!("Removed {label}")),
            Ok(status) => self.failed(format!("{label} could not be removed: {status}.")),
            Err(error) => self.failed(format!("{label} could not be removed: {error}.")),
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
        let runs_only_the_app = live
            .as_deref()
            .is_some_and(|xml| existing_task_refusal(xml, self.target_root).is_none());
        match task_action(live.as_deref(), written, prior, runs_only_the_app) {
            TaskAction::Nothing => {}
            TaskAction::Keep => {
                if let Some(xml) = live.as_deref() {
                    // A path in a task's definition is followed by the tag that ends it.
                    self.note_kept_text(&xml.replace('<', "\""));
                }
                self.kept(format!(
                    "The task {label} stays, because it has changed since it was installed."
                ));
            }
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

/// Whether the drive a path is on is there at all.
fn drive_is_connected(path: &Path) -> bool {
    path.components()
        .next()
        .is_some_and(|drive| Path::new(&format!("{}\\", drive.as_os_str().display())).exists())
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
/// The caller has pinned the way there.
pub(crate) fn put_back_backup(backup: &Backup, path: &Path) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!("{} has no parent folder", path.display()))
    })?;
    make_replaceable(path)?;
    match backup {
        Backup::Moved(from) => {
            fs::rename(from, path).map_err(|error| WinAppMigratorError::io(path, error))
        }
        Backup::Copied(from) => {
            // Under one name for each backup, so a run that was cut off leaves nothing behind
            // once this runs again.
            let partial = parent.join(format!(
                ".wam-putback-{}.partial",
                from.file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_default()
            ));
            remove_regular_file(&partial)?;
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

/// Remove a folder that has nothing in it but what pinning it may have left, which can linger
/// for a moment if something else had it open, or for good after a power cut.
fn remove_folder_if_nothing_real_is_in_it(path: &Path) -> io::Result<()> {
    for _ in 0..PIN_LINGER_TRIES {
        match fs::remove_dir(path) {
            Err(error) if error.raw_os_error() == Some(ERROR_DIR_NOT_EMPTY) => {
                let Ok(hold) = hold_real_folder(path) else {
                    return Err(error);
                };
                let pins: Vec<PathBuf> = fs::read_dir(path)?
                    .flatten()
                    .map(|entry| entry.path())
                    .collect();
                let only_pins = pins.iter().all(|file| {
                    file.file_name()
                        .is_some_and(|name| is_pin_file_name(&name.to_string_lossy()))
                });
                if !only_pins {
                    return Err(error);
                }
                for pin in pins {
                    let _ = fs::remove_file(pin);
                }
                drop(hold);
                std::thread::sleep(PIN_LINGER_POLL);
            }
            done => return done,
        }
    }
    fs::remove_dir(path)
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

/// Whether `sc.exe query` says a service has stopped. The state's name and number are not
/// translated, though what labels them is, and what comes before them can be its name.
fn query_shows_stopped(output: &str) -> bool {
    let after_the_name = output
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .skip(1);
    after_the_name.into_iter().any(|line| {
        let mut words = line
            .split_once(':')
            .map_or("", |(_, value)| value)
            .split_whitespace();
        words.next() == Some(STOPPED_STATE_NUMBER) && words.next() == Some(STOPPED_STATE)
    })
}

/// Stop a service if it is running, and say whether it was.
fn stop_service(name: &str) -> std::result::Result<bool, String> {
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
        return Ok(false);
    }
    let _ = hidden_command(SC_EXE).args(["stop", name]).output();
    let started = Instant::now();
    while started.elapsed() < STOP_SERVICE_TIMEOUT {
        if is_stopped()? {
            return Ok(true);
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
    if let Some(left) = what_is_left_of(&report, UndoMode::Rollback)
        && let Err(error) = journal.note(left)
    {
        warn!(%error, "could not record what is left of a rollback");
    }
    journal.close();
    if report.failed.is_empty() && !report.waits_for_a_drive() {
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

/// What came of looking for installs that never finished.
pub(crate) struct Recovery {
    pub recovered: Vec<RecoveredInstall>,
    /// Records that were left alone, because their folders could be changed by others.
    pub not_trusted: Vec<PathBuf>,
}

/// The apps with an install that never finished, which the next install will undo. This only
/// reads, and what it reads is only ever repeated, so it is not checked as what is undone is.
/// A journal in use belongs to an installer that is still running.
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
/// the next install over the top could not be told from it. The state folders have already
/// been checked, and each record is checked before anything it says is done.
pub(crate) fn recover_interrupted_installs(
    state_root: &Path,
    monitor: &InstallMonitor,
) -> Result<Recovery> {
    let mut interrupted: Vec<(JournalLock, ReadJournal)> = Vec::new();
    let mut not_trusted = Vec::new();
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
            Ok(journal) => {
                // What this says would be done elevated. A folder of the right name only counts
                // if it was made the way an install's is.
                if let Err(error) = confirm_protected_folder(&folder) {
                    warn!(folder = %folder.display(), %error, "skipping a record that cannot be trusted");
                    not_trusted.push(folder);
                    continue;
                }
                interrupted.push((lock, journal));
            }
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
    for (mut lock, journal) in interrupted {
        info!(id = %journal.header.id, app = %journal.header.app_name, "undoing an install that never finished");
        let report = undo_journal(
            &journal,
            UndoMode::Recovery,
            monitor,
            InstallPhase::RecoveringEarlierInstall,
        );
        remove_staging_left_behind(&journal);
        if !report.failed.is_empty() {
            stuck.push(format!(
                "{}: {} If it can never be undone, an administrator can give up on it by \
                 removing {}.",
                journal.header.app_name,
                report.failed.join(" "),
                journal.folder.display()
            ));
        } else if let Some(left) = what_is_left_of(&report, UndoMode::Recovery) {
            // Waiting for a drive stops nothing else, and what is there is looked at when it is
            // back. Only that stays on record.
            if let Err(error) = lock.note(left) {
                warn!(folder = %journal.folder.display(), %error, "could not record what is left of an install");
            }
        } else {
            drop(lock);
            remove_install_folder(&journal.folder);
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
    Ok(Recovery {
        recovered,
        not_trusted,
    })
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
    state_root: PathBuf,
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
    Ok(OpenInstall {
        journal,
        lock,
        state_root: state_root.to_path_buf(),
    })
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
        let Self {
            journal,
            mut lock,
            state_root,
        } = self;
        let _busy = match lock_state(&state_root) {
            Ok(busy) => busy,
            Err(error) => {
                return UninstallOutcome {
                    report: UndoReport {
                        failed: vec![format!("Nothing was undone: {error}.")],
                        ..UndoReport::default()
                    },
                    ..UninstallOutcome::default()
                };
            }
        };
        let mode = if journal.committed {
            UndoMode::Uninstall { remove_user_data }
        } else {
            UndoMode::Recovery
        };
        info!(id = %journal.header.id, app = %journal.header.app_name, "uninstalling");
        let report = undo_journal(&journal, mode, monitor, InstallPhase::Uninstalling);
        remove_staging_left_behind(&journal);
        let finished = report.failed.is_empty() && !report.waits_for_a_drive();
        if let Some(left) = what_is_left_of(&report, mode)
            && let Err(error) = lock.note(left)
        {
            warn!(%error, "could not record what is left of an uninstall");
        }
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
/// path, since this runs before any `PATH` could be trusted. The brackets matter: without them
/// `cmd.exe` takes all that follows an `if` to be part of it. `None` if the path holds what
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
         if exist \"{folder}\" (\"{ping}\" -n 2 127.0.0.1 >nul) else (exit /b 0))\""
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
    use std::os::windows::fs::OpenOptionsExt;

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
        // Whatever it holds, because the install has only just written it.
        assert_eq!(
            file_action(
                UndoMode::Rollback,
                program,
                &facts(false, false, true, Some(false))
            ),
            FileAction::Delete
        );
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
    fn undoing_an_install_that_never_finished_leaves_what_has_changed_since() {
        let program = ItemKind::Program;
        for (facts, expected) in [
            (facts(false, false, true, Some(true)), FileAction::Delete),
            // Months later, the user may have put this there themselves.
            (
                facts(false, false, true, Some(false)),
                FileAction::KeepChanged,
            ),
            (
                facts(true, true, true, Some(true)),
                FileAction::PutBackBackup,
            ),
            (
                facts(true, true, true, Some(false)),
                FileAction::KeepChanged,
            ),
            // It stopped between keeping the old one and placing the new one.
            (facts(true, true, false, None), FileAction::PutBackBackup),
        ] {
            assert_eq!(file_action(UndoMode::Recovery, program, &facts), expected);
        }
        // User data was part of what never finished, so nobody has asked for it to stay.
        assert_eq!(
            file_action(
                UndoMode::Recovery,
                ItemKind::UserData,
                &facts(false, false, true, Some(true))
            ),
            FileAction::Delete
        );
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
                task_action(live, written, prior, true),
                expected,
                "{live:?} {written:?} {prior:?}"
            );
        }
        // Cut off before it was registered, and this one runs something else, so it is not
        // what that install was making.
        assert_eq!(
            task_action(Some("another program's"), None, None, false),
            TaskAction::Keep
        );
        assert_eq!(
            task_action(Some("another program's"), None, Some("old"), false),
            TaskAction::Keep
        );
    }

    #[test]
    fn a_stopped_service_is_recognized_from_what_sc_says() {
        assert!(query_shows_stopped(
            "SERVICE_NAME: X\r\n        STATE              : 1  STOPPED\r\n"
        ));
        assert!(!query_shows_stopped(
            "SERVICE_NAME: X\r\n        STATE              : 4  RUNNING\r\n"
        ));
        // Its name can say anything.
        assert!(!query_shows_stopped(
            "\r\nSERVICE_NAME: 1 STOPPED\r\n        STATE              : 4  RUNNING\r\n"
        ));
        // And what labels the state is translated.
        assert!(query_shows_stopped(
            "SERVICE_NAME: X\r\n        STATUS             : 1  STOPPED\r\n"
        ));
    }

    fn journal_with(changes: Vec<Change>, damaged: bool) -> ReadJournal {
        ReadJournal {
            header: crate::journal::JournalHeader {
                schema: 1,
                id: "wam-a".to_owned(),
                app_name: "Acme".to_owned(),
                target_root: PathBuf::from(r"C:\A"),
                user_sid: None,
                started_at_utc: String::new(),
                staging: None,
                ui_theme: None,
            },
            changes,
            committed: false,
            damaged,
            folder: PathBuf::from(r"C:\Not\There"),
        }
    }

    #[test]
    fn a_drive_that_is_not_there_is_waited_for_and_not_called_a_failure() {
        let Some(drive) = ('D'..='Z')
            .map(|letter| format!(r"{letter}:\"))
            .find(|drive| !Path::new(drive).exists())
        else {
            return;
        };
        let root = Path::new(&drive).join("Apps");
        let journal = journal_with(
            vec![
                Change::Root {
                    path: root.clone(),
                    real: root.display().to_string(),
                },
                Change::File {
                    path: root.join("a.dat"),
                    root: root.clone(),
                    item: ItemKind::Program,
                    backup: None,
                    sha256: None,
                    partial: None,
                },
            ],
            false,
        );
        let report = undo_journal(
            &journal,
            UndoMode::Recovery,
            &InstallMonitor::default(),
            InstallPhase::RollingBack,
        );
        assert!(report.waits_for_a_drive());
        assert_eq!(report.drives_waited_for, [root]);
        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert!(report.kept.join(" ").contains("not connected"));
    }

    /// Or it would be left set to run a file that has gone, from a folder someone may be able
    /// to put another one in.
    #[test]
    fn a_file_that_what_stays_still_uses_stays_too() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let root = work.path().to_path_buf();
        let used = root.join("service.exe");
        let unused = root.join("other.dat");
        for file in [&used, &unused] {
            fs::write(file, b"x").map_err(|error| WinAppMigratorError::io(file, error))?;
        }
        let hash = sha256_file(&used)?;
        let journal = journal_with(
            vec![Change::Root {
                path: root.clone(),
                real: real_path(&root).map_err(|error| WinAppMigratorError::io(&root, error))?,
            }],
            false,
        );
        let changes = journal.settled_changes();
        for (mode, stays) in [
            (KEEP, true),
            (UndoMode::Recovery, true),
            (UndoMode::Rollback, false),
        ] {
            let mut undoer = undoer_for(&journal, &changes, mode);
            undoer
                .kept_mentions
                .push(format!("\"{}\" --changed", used.display()));
            undoer.undo_file(&used, &root, ItemKind::Program, None, Some(&hash), None);
            undoer.undo_file(&unused, &root, ItemKind::Program, None, Some(&hash), None);
            assert_eq!(used.exists(), stays, "{mode:?}");
            assert!(!unused.exists(), "{mode:?}");
            assert_eq!(undoer.report.kept.len(), usize::from(stays), "{mode:?}");
            for file in [&used, &unused] {
                fs::write(file, b"x").map_err(|error| WinAppMigratorError::io(file, error))?;
            }
        }
        Ok(())
    }

    #[test]
    fn what_stays_is_read_as_it_would_be_when_it_runs() {
        let journal = journal_with(Vec::new(), false);
        let mut undoer = undoer_for(&journal, &[], KEEP);
        undoer.note_kept_text(r#""%SystemRoot%\Acme\a&amp;b.exe" --changed"#);
        assert!(undoer.is_still_named(&system_root().join(r"Acme\a&b.exe")));
        assert!(!undoer.is_still_named(&system_root().join(r"Acme\other.exe")));
    }

    /// Once it has been seen to change, there is no second try in this run.
    #[test]
    fn a_folder_changing_as_this_runs_stops_it_there() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let root = work.path().to_path_buf();
        let file = root.join("a.dat");
        fs::write(&file, b"x").map_err(|error| WinAppMigratorError::io(&file, error))?;
        let hash = sha256_file(&file)?;
        let journal = journal_with(
            vec![Change::Root {
                path: root.clone(),
                real: real_path(&root).map_err(|error| WinAppMigratorError::io(&root, error))?,
            }],
            false,
        );
        let changes = journal.settled_changes();
        let mut undoer = undoer_for(&journal, &changes, KEEP);
        undoer.roots_being_changed.insert(root_key(&root));
        undoer.undo_file(&file, &root, ItemKind::Program, None, Some(&hash), None);
        assert!(file.exists());
        assert!(undoer.report.failed.join(" ").contains("was changing"));
        Ok(())
    }

    /// It cannot get better by trying again, and it must not stop every later install.
    #[test]
    fn a_folder_whose_record_was_damaged_is_left_and_said_not_failed() -> io::Result<()> {
        let work = tempfile::tempdir()?;
        let file = work.path().join("a.dat");
        fs::write(&file, b"x")?;
        let journal = journal_with(
            vec![Change::File {
                path: file.clone(),
                root: work.path().to_path_buf(),
                item: ItemKind::Program,
                backup: None,
                sha256: None,
                partial: None,
            }],
            true,
        );
        let report = undo_journal(
            &journal,
            UndoMode::Recovery,
            &InstallMonitor::default(),
            InstallPhase::RollingBack,
        );
        assert!(report.failed.is_empty(), "{:?}", report.failed);
        assert!(file.exists());
        assert!(report.kept.join(" ").contains("could not be read"));
        Ok(())
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

    /// Run for real on a temp folder, which is all it ever touches.
    #[test]
    fn a_folder_really_goes_once_the_command_has_run() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = parent.path().join("wam-abc");
        fs::create_dir_all(folder.join("backups"))?;
        fs::write(folder.join("uninstall.exe"), b"x")?;
        fs::write(folder.join(r"backups\00000001.moved"), b"x")?;
        // As the running uninstaller is, so the first tries have to fail and it has to wait.
        let in_use = fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(folder.join("uninstall.exe"))?;

        schedule_removal_of_running_uninstaller(&folder)?;

        std::thread::sleep(Duration::from_secs(3));
        assert!(folder.exists(), "it cannot have gone while that was held");
        drop(in_use);
        let started = Instant::now();
        while folder.exists() && started.elapsed() < Duration::from_secs(20) {
            std::thread::sleep(Duration::from_millis(100));
        }
        assert!(!folder.exists());
        Ok(())
    }

    #[test]
    fn a_drive_that_is_not_there_is_told_from_a_folder_that_is_not() {
        assert!(drive_is_connected(system_root()));
        assert!(drive_is_connected(&system_root().join("no such folder")));
        let missing = ('D'..='Z')
            .map(|letter| format!(r"{letter}:\"))
            .find(|drive| !Path::new(drive).exists());
        if let Some(drive) = missing {
            assert!(!drive_is_connected(&Path::new(&drive).join("Apps")));
        }
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
