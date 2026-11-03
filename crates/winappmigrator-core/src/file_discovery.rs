use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use winreg::RegKey;
use winreg::enums::{HKEY_USERS, KEY_READ};

use crate::errors::{Result, WinAppMigratorError};
use crate::hashing::sha256_file;
use crate::models::{
    FileClassification, FileEntry, FootprintKind, FootprintLocation, InstallScope, UserFolderPath,
    Warning, WarningSeverity,
};
use crate::path_utils::{
    DEEPEST_WALK, UserAccount, comparable_path, expand_vars_with, file_name_string,
    is_auxiliary_executable, is_drive_root, is_really_within, path_has_extension,
    path_is_strictly_inside, path_is_within, program_data_dir, public_profile_dir,
    resolved_location, signed_in_user, strip_prefix_ignore_case, walk_asking, windows_build,
};
use crate::privilege::{hold_folder, hold_real_folder, pin_held_folder};
use crate::product_match::{MatchScope, ProductNames};
use crate::system_context::{SystemContext, UserHive, VolumeMap};

/// How many of what a walk left out to name, of however many there are.
const MOST_NAMED_IN_A_WARNING: usize = 5;
/// More than any app has, and a package could otherwise ask for any number. Nothing of them is stored
/// but their names.
pub const MAX_EMPTY_FOLDERS: usize = 10_000;
/// The first build of Windows 11. From here on, a device's name with an extension, such as
/// `aux.h`, is an ordinary file.
const FIRST_BUILD_WITH_ORDINARY_DEVICE_EXTENSIONS: u32 = 22000;
/// `User Shell Folders` holds the (possibly redirected) per-user known-folder
/// paths as `REG_EXPAND_SZ` values such as `%USERPROFILE%\Documents`.
const USER_SHELL_FOLDERS_KEY: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders";
const DOCUMENTS_VALUE: &str = "Personal";
/// `User Shell Folders` value name (a known-folder GUID) for Saved Games.
const SAVED_GAMES_VALUE: &str = "{4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}";
const ROAMING_VALUE: &str = "AppData";
const LOCAL_VALUE: &str = "Local AppData";
/// `User Shell Folders` value name (a known-folder GUID) for LocalLow.
const LOCAL_LOW_VALUE: &str = "{A520A1A4-1780-4FF6-BD18-167343C5AF16}";
pub(crate) const DESKTOP_VALUE: &str = "Desktop";
pub(crate) const START_MENU_VALUE: &str = "Start Menu";
const USER_ENVIRONMENT_KEY: &str = "Environment";
const DOCUMENTS_FALLBACK: &str = "Documents";
const SAVED_GAMES_FALLBACK: &str = "Saved Games";
pub(crate) const DESKTOP_FALLBACK: &str = "Desktop";
pub(crate) const START_MENU_FALLBACK: &str = r"AppData\Roaming\Microsoft\Windows\Start Menu";
/// File data is not locally present (cloud online-only). Copying would trigger
/// hydration or fail, so such placeholders are reported but not copied.
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x0000_1000;
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x0040_0000;
const PACKAGE_FILES_ROOT: &str = "files";
const EXTERNAL_FILES_DIR: &str = "_external";
const DRIVER_EXTENSION: &str = "sys";
const APPDATA_SEGMENT: &str = "AppData";
const LOCAL_PROGRAMS_SEGMENT: &str = r"AppData\Local\Programs";
const COMMON_FILES_SEGMENT: &str = "Common Files";
const APPDATA_ROAMING_SEGMENT: &str = r"AppData\Roaming";
const APPDATA_LOCAL_SEGMENT: &str = r"AppData\Local";
const APPDATA_LOCAL_LOW_SEGMENT: &str = r"AppData\LocalLow";
/// Directory names whose contents are disposable (caches, crash dumps, logs)
/// and are pruned from copy candidates to keep packages lean.
const VOLATILE_DIR_NAMES: &[&str] = &[
    "cache",
    "cache2",
    "code cache",
    "gpucache",
    "dawncache",
    "shadercache",
    "crashpad",
    "crashdumps",
    "minidump",
    "minidumps",
    "temp",
    "tmp",
    "logs",
];

#[derive(Debug, Clone)]
pub struct FileDiscoveryContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    pub app_name: Option<String>,
    pub publisher: Option<String>,
    pub include_hashes: bool,
    pub include_user_data: bool,
    pub include_program_data: bool,
    /// Nothing machine-wide goes by the name of such an app. It could not have written to most of
    /// those places, its name is whatever it says in its own registry, and in `ProgramData` a
    /// folder of that name can be whatever any account or an administrator made it.
    pub installed_for_one_account: bool,
    /// Additional roots discovered elsewhere (service binaries, shortcut
    /// targets, scheduled task actions, App Paths) to fold into the footprint.
    pub extra_roots: Vec<PathBuf>,
    /// Files an install record names that have no folder of the app's own to walk, such
    /// as one directly under `Common Files`. Captured one by one.
    pub extra_files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct DiscoveryRoot {
    pub(crate) path: PathBuf,
    kind: FootprintKind,
    scope: InstallScope,
    /// The user's folder it is under, with where that is, when that has a place of its own on
    /// each PC.
    user_folder: Option<(UserDataAnchor, PathBuf)>,
}

/// What a look through an app's folders found.
#[derive(Debug, Default)]
pub(crate) struct DiscoveredFiles {
    pub(crate) files: Vec<FileEntry>,
    pub(crate) empty_folders: Vec<FileEntry>,
    pub(crate) locations: Vec<FootprintLocation>,
    pub(crate) warnings: Vec<Warning>,
}

pub fn discover_files(
    context: &FileDiscoveryContext,
    system: &SystemContext,
) -> (Vec<FileEntry>, Vec<FootprintLocation>, Vec<Warning>) {
    let found = discover_files_unless(context, system, &|| false);
    (found.files, found.locations, found.warnings)
}

/// [`discover_files`], going no further once `is_cancelled` says so, in which case what it found so
/// far is of no use to anyone. A large app takes as long to walk as anything in a scan.
pub(crate) fn discover_files_unless(
    context: &FileDiscoveryContext,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> DiscoveredFiles {
    let mut warnings = Vec::new();
    let mut files = Vec::new();
    let mut empty_folders = Vec::new();
    let mut seen = BTreeSet::new();
    let mut locations = Vec::new();

    let install_root = context.install_location.as_deref();

    // Always capture the selected executable even when its directory is not
    // walkable (for example a system-hosted helper executable).
    add_file(
        &mut files,
        &mut seen,
        &context.executable_path,
        install_root,
        FootprintKind::InstallDirectory,
        context.include_hashes,
        context.include_user_data,
        context.include_program_data,
        "Selected main executable",
        None,
        None,
        system,
        &mut warnings,
    );

    for extra in &context.extra_files {
        if !may_take_from(extra, install_root, system) {
            warnings.push(left_out_warning(extra));
            continue;
        }
        add_file(
            &mut files,
            &mut seen,
            extra,
            install_root,
            kind_for_path(extra),
            context.include_hashes,
            context.include_user_data,
            context.include_program_data,
            "File named by the install record",
            None,
            None,
            system,
            &mut warnings,
        );
    }

    let names = product_names(context);
    for root in collect_roots(context, &names, system, &mut warnings) {
        let start = files.len();
        discover_tree(
            &root,
            install_root,
            context.include_hashes,
            context.include_user_data,
            context.include_program_data,
            system,
            is_cancelled,
            &mut files,
            &mut empty_folders,
            &mut seen,
            &mut warnings,
        );
        let added = &files[start..];
        if added.is_empty() {
            continue;
        }
        let total_bytes: u64 = added.iter().filter_map(|file| file.size_bytes).sum();
        let copyable_bytes: u64 = added
            .iter()
            .filter(|file| file.should_copy)
            .filter_map(|file| file.size_bytes)
            .sum();
        locations.push(FootprintLocation {
            path: root.path.clone(),
            kind: root.kind.clone(),
            scope: root.scope.clone(),
            file_count: added.len(),
            total_bytes,
            copyable_bytes,
            included: copyable_bytes > 0,
        });
    }

    files.sort_by_cached_key(|file| file.original_path.to_string_lossy().to_ascii_lowercase());
    locations.sort_by_key(|location| std::cmp::Reverse(location.total_bytes));
    empty_folders.sort_by_cached_key(|folder| comparable_path(&folder.original_path));
    empty_folders.dedup_by_key(|folder| comparable_path(&folder.original_path));

    DiscoveredFiles {
        files,
        empty_folders,
        locations,
        warnings,
    }
}

pub fn package_relative_path(original_path: &Path, install_root: Option<&Path>) -> PathBuf {
    let relative = install_root
        .and_then(|root| strip_prefix_ignore_case(original_path, root))
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| external_relative_path(original_path));

    PathBuf::from(PACKAGE_FILES_ROOT).join(safe_relative_path(&relative))
}

pub fn safe_relative_path(path: &Path) -> PathBuf {
    let mut safe = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => safe.push(sanitize_component(&value.to_string_lossy())),
            Component::CurDir => {}
            Component::Prefix(prefix) => {
                safe.push(sanitize_component(&prefix.as_os_str().to_string_lossy()))
            }
            Component::RootDir | Component::ParentDir => safe.push("_"),
        }
    }
    if safe.as_os_str().is_empty() {
        PathBuf::from("_")
    } else {
        safe
    }
}

pub fn package_destination_path(
    package_root: &Path,
    package_relative_path: &Path,
) -> Result<PathBuf> {
    let relative = safe_relative_path(package_relative_path);
    reject_reserved_device_names(&relative)?;
    Ok(package_root.join(relative))
}

/// Check a path just before writing to it. `root` must already exist and be absolute,
/// `candidate` must lie inside it by its components alone, and nothing from `root` down
/// to `candidate` may be a link. Windows reports junctions, symbolic links, and mount
/// points the same way, and each would carry the write somewhere else. A cloud
/// placeholder is a reparse point too, but it is not reported as a link and redirects
/// nothing, so a folder synced by OneDrive stays usable.
///
/// A link at or above `root` is the caller's business: the install location is checked
/// once by [`ensure_target_root`], and an ancestor can legitimately be a mounted volume.
pub fn ensure_within_root(root: &Path, candidate: &Path) -> Result<()> {
    if !root.is_absolute() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the folder to write under must be a full path: {}",
            root.display()
        )));
    }
    if !fs::metadata(root)
        .map_err(|source| WinAppMigratorError::io(root, source))?
        .is_dir()
    {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the folder to write under is not a folder: {}",
            root.display()
        )));
    }
    let relative = strip_prefix_ignore_case(candidate, root)
        .filter(|relative| !relative.as_os_str().is_empty())
        .ok_or_else(|| {
            WinAppMigratorError::InvalidInput(format!(
                "path escapes intended root: {}",
                candidate.display()
            ))
        })?;
    reject_unsafe_components(&relative)?;

    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(WinAppMigratorError::InvalidInput(format!(
                    "refusing to write through a link: {}",
                    current.display()
                )));
            }
            Ok(_) => {}
            // Nothing below a missing folder can exist.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => return Err(WinAppMigratorError::io(&current, error)),
        }
    }
    Ok(())
}

/// Hold every folder from below `root` to the one `candidate` is in, then check the path
/// with [`ensure_within_root`]. Used to delete or put back a file with full rights, where a
/// folder swapped for a link in the moment after the check would carry that somewhere else.
/// Each folder is opened as what has that name, not what a link points at, and while held it
/// cannot be swapped, so what this checked is what the caller acts on. Keep the result until
/// done. Holding `root` and above is the caller's business, since it can be a mounted volume.
pub(crate) fn hold_path_below_root(root: &Path, candidate: &Path) -> Result<Vec<fs::File>> {
    let relative = strip_prefix_ignore_case(candidate, root)
        .filter(|relative| !relative.as_os_str().is_empty())
        .ok_or_else(|| {
            WinAppMigratorError::InvalidInput(format!(
                "path escapes intended root: {}",
                candidate.display()
            ))
        })?;
    let mut holds = Vec::new();
    let mut current = root.to_path_buf();
    if let Some(folders) = relative.parent() {
        for component in folders.components() {
            current.push(component);
            match hold_real_folder(&current) {
                Ok(hold) => holds.push(hold),
                // Nothing below a missing folder can exist.
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
                Err(_) => {
                    return Err(WinAppMigratorError::InvalidInput(format!(
                        "refusing to go through a link: {}",
                        current.display()
                    )));
                }
            }
        }
    }
    ensure_within_root(root, candidate)?;
    Ok(holds)
}

/// Whether a link leads back to the folder it is, as a volume mounted on a folder does. That
/// does not lead anywhere else.
fn leads_to_itself(path: &Path) -> bool {
    fs::canonicalize(path)
        .is_ok_and(|real| path_is_within(&real, path) && path_is_within(path, &real))
}

/// The outermost link on the way to `path`, `path` included. Whoever can write beside a link can
/// change where it leads at any moment, and with no need for it to be empty, so what is done
/// through one with full rights can be sent anywhere that way.
pub fn first_link_to(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .filter(|ancestor| {
            fs::symlink_metadata(ancestor).is_ok_and(|metadata| metadata.file_type().is_symlink())
                && !leads_to_itself(ancestor)
        })
        .last()
        .map(Path::to_path_buf)
}

/// What to say about [`first_link_to`].
pub(crate) fn link_refusal(link: &Path) -> String {
    format!(
        "{} is a link to somewhere else, and others may be able to change where it leads. Use \
         the place it leads to",
        link.display()
    )
}

/// Hold a folder paths are checked against, which must be what has that name and not a link to
/// something else. This is asked each time, because whoever can write to it can empty it and
/// turn it into a link, and a hold from before does not stop that.
pub(crate) fn hold_root(root: &Path) -> std::io::Result<fs::File> {
    match hold_real_folder(root) {
        Err(error) if error.kind() == std::io::ErrorKind::Other && leads_to_itself(root) => {
            hold_folder(root)
        }
        held => held,
    }
}

/// [`std::io::ErrorKind::Other`] is what holding or pinning says of a folder that turned out to
/// be a link, or to have become one. Anything else is only something that went wrong.
fn refusal_or_io(path: &Path, error: std::io::Error) -> WinAppMigratorError {
    if error.kind() == std::io::ErrorKind::Other {
        WinAppMigratorError::InvalidInput(format!("refusing to go through a link: {error}"))
    } else {
        WinAppMigratorError::io(path, error)
    }
}

/// Make a missing folder on the way down, and hold it. Once held, it is what keeps its parent
/// from being emptied, so until then, that is the pin's job.
fn make_and_hold_folder(folder: &Path, parent_hold: &fs::File) -> Result<fs::File> {
    let parent = folder.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!("{} has no parent folder", folder.display()))
    })?;
    let pin = pin_held_folder(parent, parent_hold).map_err(|error| refusal_or_io(parent, error))?;
    if let Err(error) = fs::create_dir(folder)
        && error.kind() != std::io::ErrorKind::AlreadyExists
    {
        return Err(WinAppMigratorError::io(folder, error));
    }
    let hold = hold_real_folder(folder).map_err(|error| refusal_or_io(folder, error))?;
    drop(pin);
    Ok(hold)
}

/// A path that stays where it leads for as long as this is alive. See [`pin_path_below_root`].
pub(crate) struct PinnedPath {
    _holds: Vec<fs::File>,
    _pin: fs::File,
}

/// Before writing, moving, or deleting at `candidate` with full rights. Holds `root` and every
/// folder down to the one `candidate` is in, making any that are missing, one at a time, so
/// that none is made through a link. Each folder holds the next, which cannot be moved out, so
/// none of them can be emptied, and the last has a file held open in it for the same reason, so
/// none can be turned into a link. Only then is the path checked, with
/// [`ensure_within_root`], so that what it found is still so when what the caller does has
/// been made to wait, which whoever can write to these folders can arrange. `root` must not be
/// a link. What is above it has to be kept from changing by holding it for longer than this,
/// as [`ensure_target_root`] and `Journal::note_root` do, or checked again after this.
pub(crate) fn pin_path_below_root(root: &Path, candidate: &Path) -> Result<PinnedPath> {
    let relative = strip_prefix_ignore_case(candidate, root)
        .filter(|relative| !relative.as_os_str().is_empty())
        .ok_or_else(|| {
            WinAppMigratorError::InvalidInput(format!(
                "path escapes intended root: {}",
                candidate.display()
            ))
        })?;
    let mut last_hold = hold_root(root).map_err(|error| refusal_or_io(root, error))?;
    let mut holds = Vec::new();
    let mut current = root.to_path_buf();
    if let Some(folders) = relative.parent() {
        for component in folders.components() {
            current.push(component);
            let held = match hold_real_folder(&current) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    make_and_hold_folder(&current, &last_hold)?
                }
                Err(_) => {
                    return Err(WinAppMigratorError::InvalidInput(format!(
                        "refusing to go through a link: {}",
                        current.display()
                    )));
                }
                Ok(held) => held,
            };
            holds.push(std::mem::replace(&mut last_hold, held));
        }
    }
    let pin =
        pin_held_folder(&current, &last_hold).map_err(|error| refusal_or_io(&current, error))?;
    holds.push(last_hold);
    ensure_within_root(root, candidate)?;
    Ok(PinnedPath {
        _holds: holds,
        _pin: pin,
    })
}

/// Create the install location, hold it open, and confirm that it is a real folder with no link
/// above it. Run once, before anything is written: a junction planted there would carry the
/// whole install somewhere else. Once held, it cannot be renamed, and with it in them, the
/// folders above it cannot be emptied and turned into links, so keep the result until the install
/// is finished. It can still be emptied and turned into a link itself, which
/// [`pin_path_below_root`] refuses, for each file.
pub fn ensure_target_root(target_root: &Path) -> Result<fs::File> {
    if !target_root.is_absolute() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the install location must be a full path: \"{}\"",
            target_root.display()
        )));
    }
    fs::create_dir_all(target_root)
        .map_err(|source| WinAppMigratorError::io(target_root, source))?;
    let hold = hold_root(target_root).map_err(|source| {
        if source.kind() == std::io::ErrorKind::Other {
            WinAppMigratorError::InvalidInput(format!(
                "the install location is a link to somewhere else: {}",
                target_root.display()
            ))
        } else {
            WinAppMigratorError::io(target_root, source)
        }
    })?;
    if let Some(link) = first_link_to(target_root) {
        return Err(WinAppMigratorError::InvalidInput(link_refusal(&link)));
    }
    Ok(hold)
}

/// The folder a restored file's location is checked against: the install location, or
/// the real user-data folder it was re-anchored to. `None` for anything else, which
/// nothing should be writing to.
pub fn write_root_for(destination: &Path, target_root: &Path) -> Option<PathBuf> {
    if path_is_within(destination, target_root) {
        return Some(target_root.to_path_buf());
    }
    USER_DATA_ANCHORS
        .iter()
        .filter_map(|(anchor, _)| user_data_base(*anchor))
        .find(|base| path_is_within(destination, base))
}

/// True for the names Windows opens as a device instead of a file. A trailing dot or space
/// changes nothing. With an extension, as in `aux.h`, Windows 11 treats it as an ordinary
/// file and Windows 10 does not, and [`opens_as_device_on`] is what knows that.
pub fn is_reserved_device_name(component: &str) -> bool {
    let upper = component.trim_end_matches(['.', ' ']).to_ascii_uppercase();
    if matches!(
        upper.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
    ) {
        return true;
    }
    let (Some(prefix), Some(suffix)) = (upper.get(..3), upper.get(3..)) else {
        return false;
    };
    matches!(prefix, "COM" | "LPT")
        && matches!(
            suffix,
            "0" | "1"
                | "2"
                | "3"
                | "4"
                | "5"
                | "6"
                | "7"
                | "8"
                | "9"
                | "\u{b9}"
                | "\u{b2}"
                | "\u{b3}"
        )
}

/// True when any folder or file in `path` has a name Windows opens as a device.
pub fn has_reserved_device_name(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(value) => is_reserved_device_name(&value.to_string_lossy()),
        _ => false,
    })
}

/// True when this build of Windows would open a file or folder with this name as a device.
/// Before Windows 11, `aux.h` reaches `AUX` and `nul.txt` reaches `NUL`. Unpacking one there
/// would throw its data away, or hang writing to a real serial port. CI's move between PCs
/// runs this where it matters: on build 20348 `aux.h` opens as `\\.\aux`, and on 26100 and
/// 26200 it is a file. Where the line falls between them is Microsoft's documented rule.
pub(crate) fn opens_as_device_on(component: &str, windows_build: u32) -> bool {
    is_reserved_device_name(component)
        || (windows_build < FIRST_BUILD_WITH_ORDINARY_DEVICE_EXTENSIONS
            && component
                .split_once('.')
                .is_some_and(|(stem, _)| is_reserved_device_name(stem)))
}

fn reject_device_names_on(path: &Path, windows_build: u32) -> Result<()> {
    if has_reserved_device_name(path) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "path uses a name reserved for a Windows device: {}",
            path.display()
        )));
    }
    let older_windows_device = path.components().any(|component| match component {
        Component::Normal(value) => opens_as_device_on(&value.to_string_lossy(), windows_build),
        _ => false,
    });
    if older_windows_device {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "{} has a name that this version of Windows opens as a device and not as a file, so \
             nothing was written to it. It can be installed on Windows 11",
            path.display()
        )));
    }
    Ok(())
}

fn reject_reserved_device_names(path: &Path) -> Result<()> {
    reject_device_names_on(path, windows_build())
}

/// Refuse a path relative to some root unless every part of it is an ordinary name.
/// `:` would address an alternate data stream, and Windows silently drops a trailing
/// dot or space, so `..` could be spelled `.. .`.
pub(crate) fn reject_unsafe_components(relative: &Path) -> Result<()> {
    reject_reserved_device_names(relative)?;
    for component in relative.components() {
        let safe = match component {
            Component::Normal(value) => {
                let value = value.to_string_lossy();
                !value.contains(':') && !value.ends_with(['.', ' '])
            }
            _ => false,
        };
        if !safe {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "unsafe path: {}",
                relative.display()
            )));
        }
    }
    Ok(())
}

/// The per-user/per-machine data root an archived file originally lived under.
/// Used to route restored user data back to the equivalent base on the target
/// machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserDataAnchor {
    RoamingAppData,
    LocalAppData,
    LocalLowAppData,
    ProgramData,
    /// The shared Public profile (`C:\Users\Public`). Machine-wide and fixed
    /// across machines, so it re-anchors to the same absolute location.
    PublicProfile,
    /// A user's Documents, wherever that is on each PC. The registry says where.
    Documents,
    /// A user's Saved Games, wherever that is on each PC. The registry says where.
    SavedGames,
}

/// Anchor markers, longest/most-specific first so `AppData\LocalLow` is matched
/// before `AppData\Local` (and `\users\public\` before it could be confused with
/// a per-user profile path).
const USER_DATA_ANCHORS: &[(UserDataAnchor, &str)] = &[
    (UserDataAnchor::RoamingAppData, r"\appdata\roaming\"),
    (UserDataAnchor::LocalLowAppData, r"\appdata\locallow\"),
    (UserDataAnchor::LocalAppData, r"\appdata\local\"),
    (UserDataAnchor::ProgramData, r"\programdata\"),
    (UserDataAnchor::PublicProfile, r"\users\public\"),
    // Common words, so only directly in a profile, which [`split_user_data_relative`] checks. A
    // redirected one is told by [`FileEntry::user_folder`].
    (UserDataAnchor::Documents, r"\documents\"),
    (UserDataAnchor::SavedGames, r"\saved games\"),
];

/// Sandbox restore destination: strip the package `files` root and join the
/// remaining relative path onto `target_root`. This is the conservative default
/// for every archived file, and the fallback for user data that cannot be
/// re-anchored. Working on names alone, it touches no disk, so a preview can use it for
/// a location that does not exist yet: [`safe_relative_path`] leaves no way out of
/// `target_root`, and [`ensure_within_root`] checks the real path before anything is
/// written.
pub fn restore_destination(target_root: &Path, package_relative_path: &Path) -> Result<PathBuf> {
    let relative = safe_relative_path(package_relative_path);
    reject_reserved_device_names(&relative)?;
    let without_files = relative
        .strip_prefix(PACKAGE_FILES_ROOT)
        .unwrap_or(&relative);
    Ok(target_root.join(without_files))
}

/// Resolve where a packaged file should be written on restore.
///
/// App-owned install-directory files always land under `target_root`. When
/// `restore_user_data` is set, files archived from the source machine's
/// `%AppData%`/`%ProgramData%` (staged under `files\_external\…`) are re-anchored
/// to the target machine's equivalent real base. Anything that cannot be
/// re-anchored falls back to the sandbox path under `target_root`.
pub fn restore_destination_for_entry(
    entry: &FileEntry,
    target_root: &Path,
    restore_user_data: bool,
) -> Result<PathBuf> {
    let relative = entry.package_relative_path.as_ref().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "file has no package relative path: {}",
            entry.original_path.display()
        ))
    })?;
    if restore_user_data
        && is_external_package_path(relative)
        && let Some((anchor, tail)) = data_location_of(entry)
        && let Some(base) = user_data_base(anchor)
    {
        reject_reserved_device_names(&tail)?;
        return Ok(base.join(tail));
    }
    restore_destination(target_root, relative)
}

/// Which of a user's or the PC's data folders a file came from, and where in it. A file from a
/// redirected Documents says so itself, since its path cannot. The relative part of that comes
/// from a package, which is not trusted, so it gets the same cleaning as any other.
pub fn data_location_of(entry: &FileEntry) -> Option<(UserDataAnchor, PathBuf)> {
    match entry.user_folder.as_ref() {
        Some(folder) => Some((folder.anchor, safe_relative_path(&folder.relative))),
        None => split_user_data_relative(&entry.original_path),
    }
}

/// True when a package-relative path was staged under `files\_external\…`
/// (data captured from outside the install directory, i.e. user/ProgramData).
pub(crate) fn is_external_package_path(package_relative_path: &Path) -> bool {
    let relative = safe_relative_path(package_relative_path);
    relative
        .strip_prefix(PACKAGE_FILES_ROOT)
        .unwrap_or(&relative)
        .components()
        .next()
        .is_some_and(|component| match component {
            Component::Normal(value) => value.eq_ignore_ascii_case(EXTERNAL_FILES_DIR),
            _ => false,
        })
}

/// Split a source path into its user-data anchor and the path relative to that
/// anchor (sanitized for safe rejoining). Pure and OS-environment independent.
pub fn split_user_data_relative(original_path: &Path) -> Option<(UserDataAnchor, PathBuf)> {
    let display = original_path.to_string_lossy();
    // ASCII-lowercasing preserves byte length and positions, so a marker offset
    // found here is a valid byte index into `display`.
    let lower = display.to_ascii_lowercase();
    for (anchor, marker) in USER_DATA_ANCHORS {
        if let Some(position) = lower.find(marker) {
            if matches!(
                anchor,
                UserDataAnchor::Documents | UserDataAnchor::SavedGames
            ) && !is_directly_in_a_profile(&lower[..position])
            {
                continue;
            }
            let tail_start = position + marker.len();
            let tail = display[tail_start..].trim();
            if tail.is_empty() {
                continue;
            }
            return Some((*anchor, safe_relative_path(Path::new(tail))));
        }
    }
    None
}

/// True for `C:\Users\alice`, which is what can have a Documents that is a user's. Not
/// `C:\Apps\Documents`, or `C:\Users\alice\Projects\Documents`.
fn is_directly_in_a_profile(lower_text_before: &str) -> bool {
    let mut parts = lower_text_before.split('\\');
    let (Some(drive), Some(users), Some(name), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    drive.ends_with(':') && users == "users" && !name.is_empty()
}

/// Resolve the target-machine base directory for a user-data anchor. The per-user ones are those
/// of whoever is signed in, which may not be the account this process is running as, and `None`
/// if that cannot be told or if Documents or Saved Games are somewhere this will not write. See
/// [`user_folder_place`]. Everything that would have gone there then goes under the install
/// location, and the review of the package says so.
pub(crate) fn user_data_base(anchor: UserDataAnchor) -> Option<PathBuf> {
    match anchor {
        UserDataAnchor::RoamingAppData
        | UserDataAnchor::LocalAppData
        | UserDataAnchor::LocalLowAppData => {
            let account = signed_in_user().account()?;
            let recorded = match user_folder_place(anchor) {
                UserFolderPlace::Usable(folder) => Some(folder),
                _ => None,
            };
            checked_user_folder(recorded, &account.profile, default_of_anchor(anchor))
        }
        UserDataAnchor::Documents | UserDataAnchor::SavedGames => match user_folder_place(anchor) {
            UserFolderPlace::Usable(folder) => Some(folder),
            _ => None,
        },
        // The machine-wide folders come from the admin-only registry, because the
        // environment of an elevated process is partly the user's to set. The marker
        // for the Public profile includes the trailing "public\", so the tail excludes it.
        UserDataAnchor::ProgramData => program_data_dir().map(Path::to_path_buf),
        UserDataAnchor::PublicProfile => public_profile_dir().map(Path::to_path_buf),
    }
}

fn default_of_anchor(anchor: UserDataAnchor) -> &'static str {
    match anchor {
        UserDataAnchor::RoamingAppData => APPDATA_ROAMING_SEGMENT,
        UserDataAnchor::LocalAppData => APPDATA_LOCAL_SEGMENT,
        UserDataAnchor::LocalLowAppData => APPDATA_LOCAL_LOW_SEGMENT,
        UserDataAnchor::Documents => DOCUMENTS_FALLBACK,
        UserDataAnchor::SavedGames => SAVED_GAMES_FALLBACK,
        UserDataAnchor::ProgramData | UserDataAnchor::PublicProfile => "",
    }
}

/// A per-user folder, from where the registry says it is as long as that is inside this
/// account's profile, which the admin-only registry says where to find. The user can set the
/// first with no rights at all, and an elevated process writes wherever it points.
fn checked_user_folder(
    recorded: Option<PathBuf>,
    profile: &Path,
    default: &str,
) -> Option<PathBuf> {
    recorded
        .filter(|folder| path_is_strictly_inside(folder, profile))
        .or_else(|| Some(profile.join(default)))
}

/// Where one of this PC's per-user folders is, for whoever is signed in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserFolderPlace {
    Usable(PathBuf),
    /// It has been moved somewhere this will not write: outside the profile, or under
    /// `AppData`, where a file can run by itself. Whoever is signed in can choose that with no
    /// rights at all, and an elevated process writes wherever it is, so the rule is that it stays
    /// in the profile, which the admin-only registry says where to find.
    Elsewhere(PathBuf),
    /// No one to go by, or a value that cannot be read as a path.
    Unknown,
}

/// Where a per-user folder is, read once, since that does not change during a run.
pub fn user_folder_place(anchor: UserDataAnchor) -> UserFolderPlace {
    static PLACES: OnceLock<Vec<(UserDataAnchor, UserFolderPlace)>> = OnceLock::new();
    PLACES
        .get_or_init(read_user_folder_places)
        .iter()
        .find(|(known, _)| *known == anchor)
        .map_or(UserFolderPlace::Unknown, |(_, place)| place.clone())
}

fn read_user_folder_places() -> Vec<(UserDataAnchor, UserFolderPlace)> {
    let Some(account) = signed_in_user().account() else {
        return Vec::new();
    };
    [
        (UserDataAnchor::RoamingAppData, ROAMING_VALUE),
        (UserDataAnchor::LocalAppData, LOCAL_VALUE),
        (UserDataAnchor::LocalLowAppData, LOCAL_LOW_VALUE),
        (UserDataAnchor::Documents, DOCUMENTS_VALUE),
        (UserDataAnchor::SavedGames, SAVED_GAMES_VALUE),
    ]
    .into_iter()
    .map(|(anchor, value)| {
        let place = account_shell_folder(account, value, default_of_anchor(anchor));
        (anchor, place)
    })
    .collect()
}

/// Where the registry says a shell folder is for an account that is signed in, by its hive under
/// `HKEY_USERS` rather than `HKEY_CURRENT_USER`, which belongs to whoever is running this.
pub(crate) fn account_shell_folder(
    account: &UserAccount,
    value_name: &str,
    default: &str,
) -> UserFolderPlace {
    let users = RegKey::predef(HKEY_USERS);
    let recorded = users
        .open_subkey_with_flags(
            format!(r"{}\{USER_SHELL_FOLDERS_KEY}", account.sid),
            KEY_READ,
        )
        .and_then(|key| key.get_value::<String, _>(value_name))
        .ok();
    let environment = users
        .open_subkey_with_flags(format!(r"{}\{USER_ENVIRONMENT_KEY}", account.sid), KEY_READ)
        .ok();
    let lookup = |name: &str| {
        environment
            .as_ref()
            .and_then(|key| key.get_value::<String, _>(name).ok())
    };
    place_of_user_folder(recorded.as_deref(), &account.profile, default, &lookup)
}

/// See [`UserFolderPlace`]. With nothing recorded, the folder is where Windows puts it.
fn place_of_user_folder(
    recorded: Option<&str>,
    profile: &Path,
    default: &str,
    environment: &dyn Fn(&str) -> Option<String>,
) -> UserFolderPlace {
    let Some(recorded) = recorded else {
        return UserFolderPlace::Usable(profile.join(default));
    };
    let Some(folder) = expand_user_folder(recorded, profile, environment) else {
        return UserFolderPlace::Unknown;
    };
    let belongs_in_app_data = default.starts_with(APPDATA_SEGMENT);
    let is_in_app_data = strip_prefix_ignore_case(&folder, profile).is_some_and(|relative| {
        relative
            .components()
            .next()
            .is_some_and(|first| first.as_os_str().eq_ignore_ascii_case(APPDATA_SEGMENT))
    });
    if path_is_strictly_inside(&folder, profile) && (belongs_in_app_data || !is_in_app_data) {
        UserFolderPlace::Usable(folder)
    } else {
        UserFolderPlace::Elsewhere(folder)
    }
}

/// Expand a value in `User Shell Folders`, which is written `%USERPROFILE%\Documents` or in full.
/// Not with this process's environment, which may be another account's and can be set by anyone.
/// What a variable means comes from the account's profile, and failing that from the account's
/// own registry. `None` if a variable has no known meaning, since a path made with one left in it
/// would be somewhere else.
fn expand_user_folder(
    recorded: &str,
    profile: &Path,
    environment: &dyn Fn(&str) -> Option<String>,
) -> Option<PathBuf> {
    let profile_text = profile.display().to_string();
    let (drive, rest_of_profile) = match profile_text.split_once(':') {
        Some((letter, rest)) => (format!("{letter}:"), rest.to_owned()),
        None => (String::new(), profile_text.clone()),
    };
    let expanded = expand_vars_with(recorded.trim(), &|name| {
        let lower = name.to_ascii_lowercase();
        match lower.as_str() {
            "userprofile" => Some(profile_text.clone()),
            "homedrive" | "systemdrive" => Some(drive.clone()),
            "homepath" => Some(rest_of_profile.clone()),
            "appdata" => Some(format!(r"{profile_text}\{APPDATA_ROAMING_SEGMENT}")),
            "localappdata" => Some(format!(r"{profile_text}\{APPDATA_LOCAL_SEGMENT}")),
            _ => environment(name),
        }
    });
    (!expanded.is_empty() && !expanded.contains('%')).then(|| PathBuf::from(expanded))
}

/// A file that is not to be read for this scan, which only says what was left out: what would be
/// followed on a drive, or what belongs to someone else, at most by a value's say-so.
fn left_out_warning(path: &Path) -> Warning {
    Warning::new(
        "file_discovery_left_out",
        WarningSeverity::Warning,
        format!(
            "{} was left out: it goes through a link on a connected drive, or it belongs to \
             another account than the one whose data is being included.",
            path.display()
        ),
        Some(path.display().to_string()),
    )
}

/// Whether any account can make a folder at the top of this kind of place, and so make it a link to
/// wherever an administrator can read.
pub(crate) fn anyone_can_create_in(kind: &FootprintKind) -> bool {
    matches!(
        kind,
        FootprintKind::ProgramData | FootprintKind::PublicDocuments
    )
}

/// Whether this kind of place is the account's own to say where it is. A link in its `AppData`, or a
/// value for where its Documents are, could otherwise take an app's name there to anything an
/// administrator can read, so what is found there has to really be that account's.
pub(crate) fn is_of_one_account(kind: &FootprintKind) -> bool {
    matches!(
        kind,
        FootprintKind::RoamingAppData
            | FootprintKind::LocalAppData
            | FootprintKind::LocalLowAppData
            | FootprintKind::Documents
            | FootprintKind::SavedGames
    )
}

pub(crate) fn is_a_link(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink())
}

/// Whether what a scan found may be read for it: on a drive, it has to be on the drive with no
/// link on the way. And another account's folder is its own, unless it is where this app is.
fn may_take_from(path: &Path, install_root: Option<&Path>, system: &SystemContext) -> bool {
    system.may_read(path)
        && (install_root.is_some_and(|root| is_really_within(path, root))
            || !system.is_in_another_profile(path))
}

/// The names this app's own folders can go by. The install folder's parent only ever
/// qualifies a nested match as a vendor folder: selecting on it is how an app in
/// `AppData\Local\Programs` used to take everything else installed there.
fn product_names(context: &FileDiscoveryContext) -> ProductNames {
    names_of_app(
        Some(&context.executable_path),
        context.install_location.as_deref(),
        context.app_name.as_deref(),
        context.publisher.as_deref(),
    )
}

pub(crate) fn names_of_app(
    executable: Option<&Path>,
    install_location: Option<&Path>,
    app_name: Option<&str>,
    publisher: Option<&str>,
) -> ProductNames {
    let mut aliases = Vec::new();
    if let Some(executable) = executable.filter(|path| !is_auxiliary_executable(path))
        && let Some(stem) = executable.file_stem().and_then(|s| s.to_str())
    {
        aliases.push(stem);
    }
    let mut vendor_folders = Vec::new();
    if let Some(install) = install_location {
        if let Some(leaf) = install.file_name().and_then(|s| s.to_str()) {
            aliases.push(leaf);
        }
        if let Some(parent) = install
            .parent()
            .and_then(Path::file_name)
            .and_then(|s| s.to_str())
        {
            vendor_folders.push(parent);
        }
    }
    ProductNames::new(app_name, publisher, &aliases, &vendor_folders)
}

fn collect_roots(
    context: &FileDiscoveryContext,
    names: &ProductNames,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> Vec<DiscoveryRoot> {
    let mut raw: Vec<DiscoveryRoot> = Vec::new();

    if let Some(install_location) = context.install_location.as_ref() {
        push_root(
            &mut raw,
            install_location.clone(),
            FootprintKind::InstallDirectory,
        );
    }
    if let Some(parent) = context.executable_path.parent() {
        push_root(
            &mut raw,
            parent.to_path_buf(),
            FootprintKind::InstallDirectory,
        );
    }
    for extra in &context.extra_roots {
        let kind = kind_for_path(extra);
        push_root(&mut raw, extra.clone(), kind);
    }

    if !context.installed_for_one_account {
        collect_program_files_roots(names, &mut raw, system, warnings);
        collect_program_data_roots(names, &mut raw, system, warnings);
    }
    collect_appdata_roots(names, &mut raw, system, warnings);
    collect_known_user_folder_roots(
        names,
        !context.installed_for_one_account,
        &mut raw,
        system,
        warnings,
    );

    let known_base_dirs = known_base_dirs(system);
    let windows_dir = system.windows_dir();
    // Deduplicate while preserving order so the most authoritative root (the
    // install location) owns each shared file, and drop any root nested inside a
    // root already kept so subtrees are not walked twice.
    let mut seen_paths = BTreeSet::new();
    let mut roots: Vec<DiscoveryRoot> = Vec::new();
    for root in raw {
        let is_for_this_scan = system.may_read(&root.path)
            && !(anyone_can_create_in(&root.kind) && is_a_link(&root.path))
            && !(is_of_one_account(&root.kind)
                && root.path.exists()
                && !system.is_the_accounts_own(&root.path))
            && (!root.path.exists()
                || may_take_from(&root.path, context.install_location.as_deref(), system));
        if !is_for_this_scan {
            warnings.push(left_out_warning(&root.path));
            continue;
        }
        if !is_walkable_app_dir(&root.path, &known_base_dirs, &windows_dir) {
            continue;
        }
        let key = root.path.to_string_lossy().to_ascii_lowercase();
        if !seen_paths.insert(key) {
            continue;
        }
        if roots
            .iter()
            .any(|existing| path_is_within(&root.path, &existing.path))
        {
            continue;
        }
        roots.push(root);
    }
    roots
}

fn push_root(roots: &mut Vec<DiscoveryRoot>, path: PathBuf, kind: FootprintKind) {
    let scope = scope_for_path(&path);
    roots.push(DiscoveryRoot {
        path,
        kind,
        scope,
        user_folder: None,
    });
}

fn collect_program_files_roots(
    names: &ProductNames,
    roots: &mut Vec<DiscoveryRoot>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    for base in system.program_files_dirs() {
        collect_matching_dirs(
            &base,
            FootprintKind::InstallDirectory,
            names,
            MatchScope::ProgramFolders,
            system.volume_map(),
            roots,
            warnings,
        );
    }
}

fn collect_program_data_roots(
    names: &ProductNames,
    roots: &mut Vec<DiscoveryRoot>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    if let Some(base) = system.program_data() {
        collect_matching_dirs(
            &base,
            FootprintKind::ProgramData,
            names,
            MatchScope::ProgramFolders,
            system.volume_map(),
            roots,
            warnings,
        );
    }
}

fn collect_appdata_roots(
    names: &ProductNames,
    roots: &mut Vec<DiscoveryRoot>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    for user in system.user_hives() {
        for (base, kind) in appdata_bases_of(&user.profile_dir) {
            collect_matching_dirs(
                &base,
                kind,
                names,
                MatchScope::ProgramFolders,
                system.volume_map(),
                roots,
                warnings,
            );
        }
    }
}

/// Match user-authored data folders the account being scanned keeps outside AppData:
/// `Documents` (including nested `My Games\<App>`) and `Saved Games`, resolving
/// any folder redirection (OneDrive, roaming profiles) recorded in the
/// `User Shell Folders` registry key before falling back to the profile path.
fn collect_known_user_folder_roots(
    names: &ProductNames,
    include_machine_wide: bool,
    roots: &mut Vec<DiscoveryRoot>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    for user in system.user_hives() {
        let folders = user_folders_of(&user, system, warnings);
        for (anchor, base, kind) in [
            (
                UserDataAnchor::Documents,
                folders.documents,
                FootprintKind::Documents,
            ),
            (
                UserDataAnchor::SavedGames,
                folders.saved_games,
                FootprintKind::SavedGames,
            ),
        ] {
            let Some(base) = base else {
                continue;
            };
            let first_new = roots.len();
            collect_matching_dirs(
                &base,
                kind,
                names,
                MatchScope::UserFolders,
                system.volume_map(),
                roots,
                warnings,
            );
            for root in &mut roots[first_new..] {
                root.user_folder = Some((anchor, base.clone()));
            }
        }
    }
    // Shared Public Documents: some apps keep machine-wide data here rather than
    // in ProgramData (e.g. YDIS's diagnostic database at
    // C:\Users\Public\Documents\YDIS_Data).
    if include_machine_wide && let Some(public_documents) = public_documents_dir(system) {
        collect_matching_dirs(
            &public_documents,
            FootprintKind::PublicDocuments,
            names,
            MatchScope::UserFolders,
            system.volume_map(),
            roots,
            warnings,
        );
    }
}

/// Where an account's own folders are, where they can be read. `None` for one that is not there.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct UserFolders {
    pub documents: Option<PathBuf>,
    pub saved_games: Option<PathBuf>,
}

/// Read an account's `User Shell Folders`. What they say is a path like any other a PC records,
/// so on a drive one can only be on the drive, and one that was on another of that PC's disks is
/// said to be out of reach rather than quietly missed.
pub(crate) fn user_folders_of(
    user: &UserHive,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> UserFolders {
    UserFolders {
        documents: user_folder_of(user, DOCUMENTS_VALUE, DOCUMENTS_FALLBACK, system, warnings),
        saved_games: user_folder_of(
            user,
            SAVED_GAMES_VALUE,
            SAVED_GAMES_FALLBACK,
            system,
            warnings,
        ),
    }
}

pub(crate) fn user_folder_of(
    user: &UserHive,
    value_name: &str,
    default: &str,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> Option<PathBuf> {
    let default_folder = user.profile_dir.join(default);
    let key = user.open(USER_SHELL_FOLDERS_KEY, KEY_READ).ok();
    let Some(recorded) = key
        .as_ref()
        .and_then(|key| key.get_value::<String, _>(value_name).ok())
    else {
        return Some(default_folder).filter(|folder| system.is_dir(folder));
    };
    let environment = user.open(USER_ENVIRONMENT_KEY, KEY_READ).ok();
    let lookup = |name: &str| {
        environment
            .as_ref()
            .and_then(|key| key.get_value::<String, _>(name).ok())
    };
    let logical_profile = system.logical_path(&user.profile_dir);
    let logical = expand_user_folder(&recorded, &logical_profile, &lookup);
    let on_a_drive = system.is_offline();
    let out_of_reach = on_a_drive
        || logical
            .as_deref()
            .is_some_and(|folder| !system.may_read(folder));
    let folder = logical.and_then(|logical| system.resolve_source_path(&logical));
    match folder {
        Some(folder) if system.is_dir(&folder) => Some(folder),
        _ if out_of_reach => {
            let not_on = if on_a_drive {
                "this drive"
            } else {
                "a disk of this PC"
            };
            warnings.push(Warning::new(
                "user_folder_out_of_reach",
                WarningSeverity::Warning,
                format!(
                    "{}'s {default} folder was at {recorded}, which is not on {not_on}, so what \
                     is there was not included.",
                    user.name
                ),
                None,
            ));
            None
        }
        _ => Some(default_folder).filter(|folder| system.is_dir(folder)),
    }
}

/// The shared Public Documents directory (`%PUBLIC%\Documents`, i.e.
/// `C:\Users\Public\Documents`), where some apps store machine-wide data.
fn public_documents_dir(system: &SystemContext) -> Option<PathBuf> {
    system
        .public_dir()
        .map(|base| base.join(DOCUMENTS_FALLBACK))
        .filter(|path| system.is_dir(path))
}

/// Find directories under `base` named for the application, at either the top level
/// (`base\App`) or one level deep (`base\Vendor\App`). Many installers nest per-vendor,
/// so checking the second level is essential to capture data folders like
/// `ProgramData\Apple Computer\iTunes`. A vendor's folder itself is never selected. On a drive
/// (`drive`), no link there is followed, since it would lead to this PC. On this PC, a link that
/// will not let itself be listed is what Windows keeps for old programs that use the old names of
/// folders (`Application Data`, `My Music`), and has nothing to report.
pub(crate) fn collect_matching_dirs(
    base: &Path,
    kind: FootprintKind,
    names: &ProductNames,
    scope: MatchScope,
    drive: Option<&VolumeMap>,
    roots: &mut Vec<DiscoveryRoot>,
    warnings: &mut Vec<Warning>,
) {
    if drive.is_some_and(|drive| !drive.may_read(base)) {
        return;
    }
    // Asked of each one by name, on a drive. What a folder's listing calls a folder can open as the
    // link beside it, if its name ends in a dot, and that listing cannot tell.
    let is_not_to_follow = |path: &Path| {
        drive.is_some_and(|drive| !drive.may_read(path))
            || (anyone_can_create_in(&kind) && is_a_link(path))
    };
    let entries = match fs::read_dir(base) {
        Ok(entries) => entries,
        Err(error) => {
            // `NotFound` just means the app keeps nothing under this base — normal
            // and silent. Any other error (notably `PermissionDenied` on an
            // offline/foreign disk owned by another PC's SIDs) means we could not
            // look, so real application data here would be silently missed. Record
            // it rather than dropping the whole footprint category.
            if error.kind() != std::io::ErrorKind::NotFound {
                warnings.push(Warning::new(
                    "file_discovery_base_unreadable",
                    WarningSeverity::Warning,
                    format!(
                        "Could not read {} ({error}); application data stored there was not included.",
                        base.display()
                    ),
                    Some(base.display().to_string()),
                ));
            }
            return;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let not_to_follow = is_not_to_follow(&path);
        let link = !not_to_follow && is_a_link(&path);
        if (!not_to_follow && !path.is_dir()) || is_volatile_dir(&path, true) {
            continue;
        }
        let folder_name = file_name_string(&path);
        if names.matches_top_level(&folder_name, scope) {
            // Matched at the top level; the whole directory is walked later, so
            // there is no need to descend into it here. One that must not be followed is
            // refused there, and said to have been.
            push_root(roots, path, kind.clone());
            continue;
        }
        if not_to_follow {
            continue;
        }
        let children = match fs::read_dir(&path) {
            Ok(children) => children,
            Err(error) => {
                let is_legacy_name = link && error.kind() == std::io::ErrorKind::PermissionDenied;
                if error.kind() != std::io::ErrorKind::NotFound && !is_legacy_name {
                    warnings.push(Warning::new(
                        "file_discovery_dir_unreadable",
                        WarningSeverity::Info,
                        format!(
                            "Could not read {} ({error}); nested application data there may be missing.",
                            path.display()
                        ),
                        Some(path.display().to_string()),
                    ));
                }
                continue;
            }
        };
        for child in children.flatten() {
            let child_path = child.path();
            let not_to_follow = is_not_to_follow(&child_path);
            if (not_to_follow || child_path.is_dir())
                && !is_volatile_dir(&child_path, true)
                && names.matches_nested(&folder_name, &file_name_string(&child_path), scope)
            {
                push_root(roots, child_path, kind.clone());
            }
        }
    }
}

/// Each profile's data folders, which are shared by what is in them. They are only for telling
/// what must not be taken for an app's own. Reading them goes by [`appdata_bases_of`].
fn appdata_bases(system: &SystemContext) -> Vec<(PathBuf, FootprintKind)> {
    system
        .user_profiles()
        .iter()
        .flat_map(|profile| appdata_bases_of(profile))
        .collect()
}

pub(crate) fn appdata_bases_of(profile: &Path) -> [(PathBuf, FootprintKind); 3] {
    [
        (
            profile.join(APPDATA_ROAMING_SEGMENT),
            FootprintKind::RoamingAppData,
        ),
        (
            profile.join(APPDATA_LOCAL_SEGMENT),
            FootprintKind::LocalAppData,
        ),
        (
            profile.join(APPDATA_LOCAL_LOW_SEGMENT),
            FootprintKind::LocalLowAppData,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn discover_tree(
    root: &DiscoveryRoot,
    install_root: Option<&Path>,
    include_hashes: bool,
    include_user_data: bool,
    include_program_data: bool,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
    files: &mut Vec<FileEntry>,
    empty_folders: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    warnings: &mut Vec<Warning>,
) {
    let prune_volatile = holds_disposable_data(&root.kind);
    // The walk has been to everything in a folder by the time it is done, so nothing has to be asked
    // of a folder twice: what is left of these when it ends had nothing come from it.
    let mut folders: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut folders_with_something: BTreeSet<String> = BTreeSet::new();
    if root.kind != FootprintKind::InstallDirectory {
        folders.insert(comparable_path(&root.path), root.path.clone());
    }
    // What a walk does not go through is named at the end of it. A link to somewhere else is, unless
    // somebody says otherwise, somewhere else.
    let links: RefCell<Vec<PathBuf>> = RefCell::new(Vec::new());
    let mut too_deep: Vec<PathBuf> = Vec::new();
    // Asked about by name before each is looked at. See `walk_asking`.
    let walker = walk_asking(&root.path, DEEPEST_WALK, |path, is_dir| {
        if is_cancelled() {
            return false;
        }
        if !system.may_read(path) {
            links.borrow_mut().push(path.to_path_buf());
            return false;
        }
        !prune_volatile || !is_volatile_dir(path, is_dir)
    });

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                folders_with_something.insert(comparable_path(&error.path));
                warnings.push(Warning::new(
                    "file_discovery_walk_error",
                    WarningSeverity::Info,
                    format!("Could not read a filesystem entry: {error}"),
                    Some(root.path.display().to_string()),
                ));
                continue;
            }
        };
        if let Some(folder) = entry.path.parent() {
            folders_with_something.insert(comparable_path(folder));
        }
        if entry.file_type.is_symlink() {
            links.borrow_mut().push(entry.path.clone());
        }
        if entry.too_deep {
            too_deep.push(entry.path.clone());
            folders_with_something.insert(comparable_path(&entry.path));
        }
        if entry.file_type.is_dir()
            && !entry
                .metadata
                .as_ref()
                .is_some_and(|metadata| is_online_only(metadata.file_attributes()))
        {
            folders.insert(comparable_path(&entry.path), entry.path.clone());
        }
        if entry.file_type.is_file() {
            add_file(
                files,
                seen,
                &entry.path,
                install_root,
                root.kind.clone(),
                include_hashes,
                include_user_data,
                include_program_data,
                discovery_reason(&root.kind),
                root.user_folder.as_ref(),
                entry.metadata.as_ref(),
                system,
                warnings,
            );
        }
    }
    if is_cancelled() {
        return;
    }
    let is_kept = |classification: &FileClassification| match classification {
        FileClassification::AppOwned => true,
        FileClassification::UserData => include_user_data,
        FileClassification::MachineAppData => include_program_data,
        _ => false,
    };
    let links = links.into_inner();
    if !links.is_empty() {
        warnings.push(Warning::new(
            "links_not_followed",
            WarningSeverity::Warning,
            format!(
                "{} links, or names that would open something else, were not followed in this \
                 folder, so what they point at is not in the installer: {}.",
                links.len(),
                named_in(&links, &root.path)
            ),
            Some(root.path.display().to_string()),
        ));
    }
    if !too_deep.is_empty() {
        warnings.push(Warning::new(
            "folders_too_deep",
            WarningSeverity::Warning,
            format!(
                "{} folders are more than {DEEPEST_WALK} levels down, so what is in them is not in \
                 the installer: {}.",
                too_deep.len(),
                named_in(&too_deep, &root.path)
            ),
            Some(root.path.display().to_string()),
        ));
    }
    for (key, folder) in folders {
        if folders_with_something.contains(&key) || has_reserved_device_name(&folder) {
            continue;
        }
        let classification = classify_file(
            &system.logical_path(&folder),
            install_root
                .map(|root| system.logical_path(root))
                .as_deref(),
            &root.kind,
        );
        if !is_kept(&classification) {
            continue;
        }
        empty_folders.push(FileEntry {
            path: folder.clone(),
            original_path: folder.clone(),
            package_relative_path: Some(package_relative_path(&folder, install_root)),
            size_bytes: None,
            hash_sha256: None,
            classification,
            should_copy: true,
            reason: "An empty folder of the app's".to_owned(),
            user_folder: root.user_folder.as_ref().and_then(|(anchor, base)| {
                strip_prefix_ignore_case(&folder, base).map(|relative| UserFolderPath {
                    anchor: *anchor,
                    relative,
                })
            }),
        });
    }
}

/// As many of what a walk left out as is worth reading, by where each is in what was walked.
fn named_in(paths: &[PathBuf], root: &Path) -> String {
    let mut names: Vec<String> = paths
        .iter()
        .take(MOST_NAMED_IN_A_WARNING)
        .map(|path| {
            strip_prefix_ignore_case(path, root)
                .unwrap_or_else(|| path.clone())
                .display()
                .to_string()
        })
        .collect();
    if paths.len() > MOST_NAMED_IN_A_WARNING {
        names.push(format!(
            "and {} more",
            paths.len() - MOST_NAMED_IN_A_WARNING
        ));
    }
    names.join(", ")
}

/// True for data folders, where a `Cache` or `Logs` folder is the app's own scratch
/// space and can be left behind. Inside an install folder the same names can be files
/// the app shipped with, so nothing there is skipped: an installer missing them is
/// silently broken.
fn holds_disposable_data(kind: &FootprintKind) -> bool {
    !matches!(
        kind,
        FootprintKind::InstallDirectory | FootprintKind::RegistryReference | FootprintKind::Other
    )
}

fn discovery_reason(kind: &FootprintKind) -> &'static str {
    match kind {
        FootprintKind::InstallDirectory => "File under install directory",
        FootprintKind::ProgramData => "Machine-wide application data (ProgramData)",
        FootprintKind::RoamingAppData => "Roaming user application data",
        FootprintKind::LocalAppData => "Local user application data",
        FootprintKind::LocalLowAppData => "LocalLow user application data",
        FootprintKind::Documents => "User document data",
        FootprintKind::SavedGames => "Saved game data",
        FootprintKind::PublicDocuments => "Shared public document data",
        FootprintKind::RegistryReference => "Referenced application location",
        FootprintKind::Other => "Related application file",
    }
}

#[allow(clippy::too_many_arguments)]
fn add_file(
    files: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    path: &Path,
    install_root: Option<&Path>,
    root_kind: FootprintKind,
    include_hashes: bool,
    include_user_data: bool,
    include_program_data: bool,
    reason: &str,
    user_folder: Option<&(UserDataAnchor, PathBuf)>,
    listed: Option<&fs::Metadata>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    let key = path.to_string_lossy().to_ascii_lowercase();
    if !seen.insert(key) {
        return;
    }

    // What a folder's listing said, if this came from one. Asking again for every file of a large
    // app takes as long as the rest of the walk.
    let metadata = match listed.map_or_else(|| fs::metadata(path), |listed| Ok(listed.clone())) {
        Ok(metadata) => metadata,
        Err(error) => {
            warnings.push(Warning::new(
                "file_metadata_unreadable",
                WarningSeverity::Info,
                format!("Could not read file metadata: {error}"),
                Some(path.display().to_string()),
            ));
            return;
        }
    };
    if !metadata.is_file() {
        return;
    }

    // By what the PC itself called it: a drive can be mounted on a folder with any name.
    let classification = classify_file(
        &system.logical_path(path),
        install_root
            .map(|root| system.logical_path(root))
            .as_deref(),
        &root_kind,
    );
    let online_only = is_online_only(metadata.file_attributes());
    let base_should_copy = match classification {
        FileClassification::AppOwned => true,
        FileClassification::UserData => include_user_data,
        FileClassification::MachineAppData => include_program_data,
        _ => false,
    };
    // A file named like a device can only exist if something bypassed Windows' own
    // rules, and on the target the name would open the device. It stays out of the
    // package, and restoring refuses a manifest that names one anyway.
    let reserved_name = has_reserved_device_name(path);
    if reserved_name && base_should_copy {
        warnings.push(Warning::new(
            "file_reserved_name_skipped",
            WarningSeverity::Warning,
            "This file's name is reserved for a Windows device, so it was not copied.",
            Some(path.display().to_string()),
        ));
    }
    // An online-only cloud placeholder has no local data; copying it would
    // hydrate or fail, so it is reported but never copied.
    let should_copy = base_should_copy && !online_only && !reserved_name;

    let hash_sha256 = if include_hashes && should_copy {
        match sha256_file(path) {
            Ok(hash) => Some(hash),
            Err(error) => {
                warnings.push(Warning::new(
                    "file_hash_failed",
                    WarningSeverity::Info,
                    format!("Could not hash file: {error}"),
                    Some(path.display().to_string()),
                ));
                None
            }
        }
    } else {
        None
    };

    let package_relative_path = should_copy.then(|| package_relative_path(path, install_root));
    let reason = if online_only {
        warnings.push(Warning::new(
            "cloud_placeholder_skipped",
            WarningSeverity::Info,
            "Online-only cloud file (e.g. OneDrive Files On-Demand) has no local data; pin or download it before backup. Reported but not copied.",
            Some(path.display().to_string()),
        ));
        "Online-only cloud placeholder (data not downloaded); not copied"
    } else {
        match classification {
            FileClassification::ManualRuntimeInstall => {
                let file_name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();
                warnings.push(Warning::new(
                    DRIVER_FILE_NOT_COPIED_CODE,
                    WarningSeverity::Info,
                    format!(
                        "Driver/kernel file '{file_name}' is not copied as a plain file. If it backs a registered driver service for this app, the installer bundles it and re-registers the driver automatically; otherwise reinstall it with the vendor's signed driver package."
                    ),
                    Some(path.display().to_string()),
                ));
                "Driver/kernel file: bundled and re-registered when app-owned, otherwise vendor reinstall"
            }
            FileClassification::UserData if !include_user_data => {
                "User data found but excluded by current scan options"
            }
            FileClassification::UserData => "User or application data captured for restore",
            FileClassification::MachineAppData if !include_program_data => {
                "Machine-wide ProgramData found but excluded by current scan options"
            }
            FileClassification::MachineAppData => {
                "Machine-wide ProgramData application data captured for restore"
            }
            _ => reason,
        }
    };

    files.push(FileEntry {
        path: path.to_path_buf(),
        original_path: path.to_path_buf(),
        package_relative_path,
        size_bytes: Some(metadata.len()),
        hash_sha256,
        classification,
        should_copy,
        reason: reason.to_owned(),
        user_folder: user_folder.and_then(|(anchor, base)| {
            strip_prefix_ignore_case(path, base).map(|relative| UserFolderPath {
                anchor: *anchor,
                relative,
            })
        }),
    });
}

fn classify_file(
    path: &Path,
    install_root: Option<&Path>,
    root_kind: &FootprintKind,
) -> FileClassification {
    if path_has_extension(path, DRIVER_EXTENSION) {
        return FileClassification::ManualRuntimeInstall;
    }
    if is_windows_system_path(path) {
        return FileClassification::WindowsSystem;
    }
    if install_root.is_some_and(|root| path_is_within(path, root)) {
        return FileClassification::AppOwned;
    }
    match root_kind {
        FootprintKind::InstallDirectory => FileClassification::AppOwned,
        FootprintKind::RegistryReference | FootprintKind::Other => {
            if is_program_data_path(path) {
                FileClassification::MachineAppData
            } else if is_user_profile_path(path) && !is_program_files_path(path) {
                FileClassification::UserData
            } else {
                // Program Files, or a folder of its own such as C:\CocosDashboard: an
                // install record pointed here, so the app needs it.
                FileClassification::AppOwned
            }
        }
        // ProgramData and shared Public Documents are machine-wide application
        // data shared by all users, not per-user data.
        FootprintKind::ProgramData | FootprintKind::PublicDocuments => {
            FileClassification::MachineAppData
        }
        FootprintKind::RoamingAppData
        | FootprintKind::LocalAppData
        | FootprintKind::LocalLowAppData
        | FootprintKind::Documents
        | FootprintKind::SavedGames => FileClassification::UserData,
    }
}

pub(crate) fn is_volatile_dir(path: &Path, is_dir: bool) -> bool {
    if !is_dir {
        return false;
    }
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if name.is_empty() {
        return false;
    }
    VOLATILE_DIR_NAMES.contains(&name.as_str()) || name.ends_with("cache")
}

pub(crate) fn kind_for_path(path: &Path) -> FootprintKind {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    if lower.contains(r"\appdata\roaming") {
        FootprintKind::RoamingAppData
    } else if lower.contains(r"\appdata\locallow") {
        FootprintKind::LocalLowAppData
    } else if lower.contains(r"\appdata\local") {
        FootprintKind::LocalAppData
    } else if lower.contains(r"\programdata") {
        FootprintKind::ProgramData
    } else if is_program_files_path(path) {
        FootprintKind::InstallDirectory
    } else {
        FootprintKind::RegistryReference
    }
}

fn scope_for_path(path: &Path) -> InstallScope {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    if lower.contains(r"\users\") || lower.contains(r"\appdata\") {
        InstallScope::PerUser
    } else if is_program_files_path(path)
        || lower.contains(r"\programdata")
        || lower.contains(r"\windows")
    {
        InstallScope::PerMachine
    } else {
        InstallScope::Unknown
    }
}

fn is_program_files_path(path: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .contains(r"\program files")
}

fn is_program_data_path(path: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .contains(r"\programdata")
}

/// What is said of a driver's file that stays behind.
pub(crate) const DRIVER_FILE_NOT_COPIED_CODE: &str = "driver_file_not_copied";

pub(crate) fn is_user_profile_path(path: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .contains(r"\users\")
}

/// True for a cloud (e.g. OneDrive Files On-Demand) placeholder whose data is
/// not present locally, so it cannot be copied without hydrating it.
fn is_online_only(attributes: u32) -> bool {
    attributes & (FILE_ATTRIBUTE_OFFLINE | FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS) != 0
}

fn is_windows_system_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.contains(r"\windows\system32\")
        || lower.contains(r"\windows\syswow64\")
        || lower.ends_with(r"\windows")
}

/// True when `path` is a directory we can safely walk: it exists, is not the drive
/// itself, is not in the Windows directory, and is not a known base such as
/// `Program Files` or a user profile. A folder directly on a drive, like
/// `C:\CocosDashboard`, is a normal install location.
pub(crate) fn is_walkable_app_dir(
    path: &Path,
    known_base_dirs: &[PathBuf],
    windows_dir: &Path,
) -> bool {
    path.is_dir()
        && !is_drive_root(path)
        && !path_is_within(path, windows_dir)
        && !holds_a_known_base_dir(path, known_base_dirs)
}

/// Whether a folder is a known base, or has one inside it, which holds everything that one does: a
/// profile can be directly in any folder, if the profile list says so. By where it really is too,
/// since `C:\Users\BOB~1` is someone's whole profile. That looks through a link, so on a drive,
/// ask only about a path that may be read.
pub(crate) fn holds_a_known_base_dir(path: &Path, known_base_dirs: &[PathBuf]) -> bool {
    let real = resolved_location(path);
    known_base_dirs
        .iter()
        .any(|base| path_is_within(base, path) || path_is_within(base, &real))
}

/// Folders that hold other things side by side. None of them can be an app's own install
/// location or a root to walk.
pub(crate) fn known_base_dirs(system: &SystemContext) -> Vec<PathBuf> {
    let mut bases = system.program_files_dirs();
    for program_files in system.program_files_dirs() {
        bases.push(program_files.join(COMMON_FILES_SEGMENT));
    }
    if let Some(program_data) = system.program_data() {
        bases.push(program_data);
    }
    if let Some(public) = system.public_dir() {
        bases.push(public.join(DOCUMENTS_FALLBACK));
        bases.push(public);
    }
    for (base, _) in appdata_bases(system) {
        bases.push(base);
    }
    let profiles = system.user_profiles();
    if let Some(users_root) = profiles.first().and_then(|profile| profile.parent()) {
        bases.push(users_root.to_path_buf());
    }
    for profile in profiles {
        bases.push(profile.join(APPDATA_SEGMENT));
        bases.push(profile.join(LOCAL_PROGRAMS_SEGMENT));
        bases.push(profile.join(DOCUMENTS_FALLBACK));
        bases.push(profile.join(SAVED_GAMES_FALLBACK));
        bases.push(profile);
    }
    bases.push(system.windows_dir());
    // Where each really is too, for comparing with where a folder really is. Finding that out
    // looks through links, so not where a link may not be followed.
    let real: Vec<PathBuf> = bases
        .iter()
        .filter(|base| system.may_read(base))
        .map(|base| resolved_location(base))
        .collect();
    bases.extend(real);
    bases
}

fn external_relative_path(path: &Path) -> PathBuf {
    let mut output = PathBuf::from(EXTERNAL_FILES_DIR);
    for component in safe_relative_path(path).components() {
        if let Component::Normal(value) = component {
            output.push(value);
        }
    }
    output
}

fn sanitize_component(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => output.push('_'),
            value if value.is_control() => output.push('_'),
            value => output.push(value),
        }
    }
    // Dots and spaces in any order: Windows drops them from the end of a name, which
    // would turn `.. .` into `..`.
    let trimmed = output.trim_start().trim_end_matches(['.', ' ']).to_owned();
    if trimmed.is_empty() {
        "_".to_owned()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_unsafe_relative_components() {
        let safe = safe_relative_path(Path::new(r"..\C:\bad:name.txt"));

        assert!(!safe.to_string_lossy().contains(".."));
        assert!(!safe.to_string_lossy().contains(':'));
    }

    #[test]
    fn no_spelling_of_a_parent_folder_survives() {
        for name in [
            r"a\..\b",
            r"a\.. .\b",
            r"a\. .\b",
            r"a\ ..\b",
            r"a\...\b",
            r"a\.. ",
        ] {
            let safe = safe_relative_path(Path::new(name));
            for component in safe.components() {
                let Component::Normal(value) = component else {
                    panic!("{name} kept {component:?}");
                };
                let value = value.to_string_lossy();
                assert_ne!(value, "..", "{name}");
                assert!(!value.ends_with(['.', ' ']), "{name} became {value:?}");
            }
        }
    }

    #[test]
    fn device_names_are_recognized_in_any_case_and_ordinary_files_are_not() {
        for name in [
            "CON",
            "con",
            "NUL",
            "Com1",
            "COM9",
            "lpt1",
            "AUX",
            "PRN",
            "CONIN$",
            "COM\u{b9}",
            "nul ",
            "NUL.",
        ] {
            assert!(is_reserved_device_name(name), "{name}");
        }
        for name in [
            "console",
            "CONTRIBUTING.md",
            "null",
            "COM",
            "COM10",
            "LPT",
            "auxiliary.dll",
            "prn2",
            "a.con",
            // Ordinary files on Windows 11, and shipped by real apps.
            "aux.h",
            "con.txt",
            "nul.c",
            "com1.txt",
        ] {
            assert!(!is_reserved_device_name(name), "{name}");
        }
    }

    #[test]
    fn destinations_refuse_device_names_and_keep_ordinary_files() -> Result<()> {
        let root = Path::new(r"C:\Target");
        assert_eq!(
            restore_destination(root, Path::new(r"files\bin\console.exe"))?,
            PathBuf::from(r"C:\Target\bin\console.exe")
        );
        assert_eq!(
            package_destination_path(root, Path::new(r"files\com.dll"))?,
            PathBuf::from(r"C:\Target\files\com.dll")
        );
        assert!(restore_destination(root, Path::new(r"files\CON")).is_err());
        assert!(restore_destination(root, Path::new(r"files\NUL\x.dat")).is_err());
        assert!(package_destination_path(root, Path::new(r"files\com1")).is_err());
        Ok(())
    }

    /// A junction needs no special privilege, unlike a symbolic link, so this runs in a
    /// normal shell. Temp folders only.
    fn make_junction(link: &Path, target: &Path) -> bool {
        crate::path_utils::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(link)
            .arg(target)
            .output()
            .is_ok_and(|output| output.status.success())
    }

    #[test]
    fn before_windows_11_a_devices_name_with_an_extension_is_refused_with_a_clear_message() {
        const WINDOWS_10: u32 = 19045;
        const WINDOWS_11: u32 = 22621;
        for name in ["aux.h", "NUL.txt", "com1.log", "con.tar.gz", "LPT3.dat"] {
            let path = Path::new("include").join(name);
            assert!(reject_device_names_on(&path, WINDOWS_11).is_ok(), "{name}");
            let refused = reject_device_names_on(&path, WINDOWS_10);
            assert!(
                matches!(&refused, Err(WinAppMigratorError::InvalidInput(message))
                    if message.contains("opens as a device") && message.contains(name)),
                "{name}: {refused:?}"
            );
            // A build that cannot be read counts as old, which fails and says so.
            assert!(reject_device_names_on(&path, 0).is_err(), "{name}");
        }
        for name in [
            "auxiliary.h",
            "console.txt",
            "com10.log",
            "aux_h",
            "readme.aux",
        ] {
            let path = Path::new("include").join(name);
            assert!(reject_device_names_on(&path, WINDOWS_10).is_ok(), "{name}");
        }
        // A folder's name counts too, and with no extension no Windows takes it as a file.
        assert!(reject_device_names_on(Path::new(r"aux.d\file.txt"), WINDOWS_10).is_err());
        assert!(reject_device_names_on(Path::new(r"aux.d\file.txt"), WINDOWS_11).is_ok());
        assert!(reject_device_names_on(Path::new("aux"), WINDOWS_11).is_err());
    }

    #[test]
    fn holding_the_way_to_a_file_refuses_a_link_on_the_way() -> Result<()> {
        let base = temp_base()?;
        let root = base.path().join("root");
        fs::create_dir_all(root.join(r"real\deep"))
            .map_err(|error| WinAppMigratorError::io(&root, error))?;
        let elsewhere = base.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let made = crate::path_utils::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(root.join("linked"))
            .arg(&elsewhere)
            .output()
            .map_err(|error| WinAppMigratorError::io(&root, error))?;
        assert!(made.status.success());

        let holds = hold_path_below_root(&root, &root.join(r"real\deep\file.txt"))?;
        assert_eq!(holds.len(), 2);
        // Held, so it cannot be swapped for a link now.
        assert!(fs::rename(root.join(r"real\deep"), root.join(r"real\moved")).is_err());
        assert!(fs::rename(root.join("real"), root.join("moved")).is_err());
        drop(holds);

        assert!(hold_path_below_root(&root, &root.join(r"linked\file.txt")).is_err());
        assert!(hold_path_below_root(&root, &elsewhere.join("file.txt")).is_err());
        assert!(hold_path_below_root(&root, &root).is_err());
        // A folder that is not there has nothing below it to go through.
        assert!(hold_path_below_root(&root, &root.join(r"gone\file.txt")).is_ok());
        Ok(())
    }

    #[test]
    fn pinning_the_way_to_a_file_makes_what_is_missing_refuses_links_and_leaves_nothing()
    -> Result<()> {
        let base = temp_base()?;
        let root = base.path().join("root");
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let elsewhere = base.path().join("elsewhere");
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let made = crate::path_utils::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(root.join("linked"))
            .arg(&elsewhere)
            .output()
            .map_err(|error| WinAppMigratorError::io(&root, error))?;
        assert!(made.status.success());

        let pinned = pin_path_below_root(&root, &root.join(r"new\deep\file.txt"))?;
        let deep = root.join(r"new\deep");
        assert!(deep.is_dir(), "what was missing was made");
        assert_eq!(fs::read_dir(&deep).map_or(0, Iterator::count), 1);
        assert!(fs::rename(&deep, root.join(r"new\moved")).is_err());
        drop(pinned);
        assert_eq!(fs::read_dir(&deep).map_or(1, Iterator::count), 0);

        // In the folder itself as well.
        drop(pin_path_below_root(&root, &root.join("file.txt"))?);

        // And the folder it is all under must not be a link either.
        assert!(pin_path_below_root(&root.join("linked"), &root.join(r"linked\file.txt")).is_err());
        assert!(pin_path_below_root(&root, &root.join(r"linked\file.txt")).is_err());
        assert!(pin_path_below_root(&root, &root.join(r"linked\new\file.txt")).is_err());
        assert_eq!(fs::read_dir(&elsewhere).map_or(1, Iterator::count), 0);
        assert!(pin_path_below_root(&root, &root.join("linked")).is_err());
        assert!(pin_path_below_root(&root, &root).is_err());
        assert!(pin_path_below_root(&root, &elsewhere.join("file.txt")).is_err());
        Ok(())
    }

    #[test]
    fn a_link_anywhere_on_the_way_is_found_and_what_is_real_is_not() -> Result<()> {
        let base = temp_base()?;
        let real = base.path().join(r"real\deep");
        fs::create_dir_all(&real).map_err(|error| WinAppMigratorError::io(&real, error))?;
        let link = base.path().join("link");
        let made = crate::path_utils::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(base.path().join("real"))
            .output()
            .map_err(|error| WinAppMigratorError::io(&link, error))?;
        assert!(made.status.success());

        assert_eq!(first_link_to(&real), None);
        assert_eq!(first_link_to(&real.join("not there")), None);
        assert_eq!(first_link_to(&link).as_deref(), Some(link.as_path()));
        assert_eq!(
            first_link_to(&link.join("deep")).as_deref(),
            Some(link.as_path())
        );
        assert_eq!(
            first_link_to(&link.join(r"deep\not there")).as_deref(),
            Some(link.as_path())
        );
        assert!(ensure_target_root(&link.join("new")).is_err());
        Ok(())
    }

    fn temp_base() -> Result<tempfile::TempDir> {
        tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp folder", error))
    }

    #[test]
    fn writing_through_a_junction_is_refused() -> Result<()> {
        let base = temp_base()?;
        let root = base.path().join("target");
        let elsewhere = base.path().join("elsewhere");
        fs::create_dir_all(root.join("real"))
            .map_err(|error| WinAppMigratorError::io(&root, error))?;
        fs::create_dir_all(&elsewhere).map_err(|error| WinAppMigratorError::io(&root, error))?;
        assert!(make_junction(&root.join("linked"), &elsewhere));

        assert!(ensure_within_root(&root, &root.join(r"real\app.exe")).is_ok());
        assert!(ensure_within_root(&root, &root.join(r"real\new\deep\app.exe")).is_ok());
        assert!(ensure_within_root(&root, &root.join(r"linked\app.exe")).is_err());
        assert!(ensure_within_root(&root, &root.join(r"linked\new\app.exe")).is_err());
        assert!(ensure_within_root(&root, &root.join("linked")).is_err());
        Ok(())
    }

    #[test]
    fn a_symbolic_link_at_the_destination_is_refused() -> Result<()> {
        let base = temp_base()?;
        let elsewhere = base.path().join("outside.txt");
        fs::write(&elsewhere, b"x").map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        let link = base.path().join("app.exe");
        // Symbolic links need Developer Mode or elevation, so this cannot always run. CI
        // is elevated, so there this fails rather than skipping.
        if let Err(error) = std::os::windows::fs::symlink_file(&elsewhere, &link) {
            assert!(
                std::env::var_os("CI").is_none(),
                "CI should be able to create a symbolic link: {error}"
            );
            eprintln!("skipping: this shell cannot create a symbolic link");
            return Ok(());
        }
        assert!(ensure_within_root(base.path(), &link).is_err());
        Ok(())
    }

    #[test]
    fn write_checks_need_a_real_absolute_root_and_a_path_inside_it() -> Result<()> {
        let base = temp_base()?;
        let root = base.path();
        assert!(ensure_within_root(Path::new("relative"), Path::new(r"relative\a")).is_err());
        assert!(ensure_within_root(&root.join("missing"), &root.join(r"missing\a")).is_err());
        assert!(ensure_within_root(root, &root.join(r"..\a")).is_err());
        assert!(ensure_within_root(root, &root.join(r"a\..\..\b")).is_err());
        assert!(ensure_within_root(root, &root.join("CON")).is_err());
        assert!(ensure_within_root(root, &root.join("a:stream")).is_err());
        assert!(ensure_within_root(root, root).is_err());
        assert!(ensure_within_root(root, Path::new(r"C:\Windows\System32\x.dll")).is_err());
        assert!(ensure_within_root(root, &root.join("a.txt")).is_ok());
        assert!(ensure_within_root(root, &root.join(r"new\folder\a.txt")).is_ok());
        Ok(())
    }

    #[test]
    fn a_target_root_is_created_and_must_be_a_real_folder() -> Result<()> {
        let base = temp_base()?;
        let fresh = base.path().join(r"new\target");
        let hold = ensure_target_root(&fresh)?;
        assert!(fresh.is_dir());
        assert!(fs::rename(&fresh, base.path().join("moved")).is_err());
        drop(hold);
        assert!(ensure_target_root(Path::new("relative")).is_err());

        let linked = base.path().join("linked");
        assert!(make_junction(&linked, &fresh));
        assert!(ensure_target_root(&linked).is_err());
        Ok(())
    }

    #[test]
    fn a_per_user_folder_outside_the_profile_is_ignored() {
        let profile = Path::new(r"C:\Users\bob");
        let check = |recorded: &str| {
            checked_user_folder(
                Some(PathBuf::from(recorded)),
                profile,
                APPDATA_ROAMING_SEGMENT,
            )
        };
        let usual = Some(PathBuf::from(r"C:\Users\bob\AppData\Roaming"));
        assert_eq!(check(r"C:\Users\bob\AppData\Roaming"), usual);
        assert_eq!(
            check(r"C:\Users\bob\Moved"),
            Some(PathBuf::from(r"C:\Users\bob\Moved"))
        );
        assert_eq!(check(r"C:\Windows\System32"), usual);
        assert_eq!(check(r"C:\Users\bobby\AppData"), usual);
        assert_eq!(check(r"C:\Users\bob\..\..\Windows\System32"), usual);
        assert_eq!(
            checked_user_folder(None, profile, APPDATA_ROAMING_SEGMENT),
            usual
        );
    }

    fn no_environment(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn a_user_folder_is_read_for_that_account_and_not_with_this_processs_variables() {
        let profile = Path::new(r"C:\Users\bob");
        let expand = |recorded: &str| expand_user_folder(recorded, profile, &no_environment);
        assert_eq!(
            expand(r"%USERPROFILE%\Documents"),
            Some(PathBuf::from(r"C:\Users\bob\Documents"))
        );
        assert_eq!(
            expand(r"%HOMEDRIVE%%HOMEPATH%\Saved Games"),
            Some(PathBuf::from(r"C:\Users\bob\Saved Games"))
        );
        assert_eq!(
            expand(r"C:\Users\bob\OneDrive\Documents"),
            Some(PathBuf::from(r"C:\Users\bob\OneDrive\Documents"))
        );
        // A variable that only means something in this process, and could be anyone's.
        assert_eq!(expand(r"%TEMP%\Documents"), None);
        assert_eq!(
            expand_user_folder(r"%OneDrive%\Documents", profile, &|name| {
                (name == "OneDrive").then(|| r"C:\Users\bob\OneDrive".to_owned())
            }),
            Some(PathBuf::from(r"C:\Users\bob\OneDrive\Documents"))
        );
    }

    #[test]
    fn a_user_folder_is_used_inside_its_profile_and_said_to_be_elsewhere_otherwise() {
        let profile = Path::new(r"C:\Users\bob");
        let place = |recorded: Option<&str>| {
            place_of_user_folder(recorded, profile, DOCUMENTS_FALLBACK, &no_environment)
        };
        assert_eq!(
            place(None),
            UserFolderPlace::Usable(PathBuf::from(r"C:\Users\bob\Documents"))
        );
        assert_eq!(
            place(Some(r"%USERPROFILE%\OneDrive\Documents")),
            UserFolderPlace::Usable(PathBuf::from(r"C:\Users\bob\OneDrive\Documents"))
        );
        for elsewhere in [
            r"D:\Docs",
            r"\\server\home\bob\Documents",
            r"C:\Users\bobby\Documents",
            r"%USERPROFILE%\..\..\Windows\System32",
            // Inside it, and where what lands in it can run.
            r"%USERPROFILE%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup",
        ] {
            assert!(
                matches!(place(Some(elsewhere)), UserFolderPlace::Elsewhere(_)),
                "{elsewhere}: {:?}",
                place(Some(elsewhere))
            );
        }
        assert_eq!(
            place(Some(r"%NotKnown%\Documents")),
            UserFolderPlace::Unknown
        );
        // A folder that is in AppData by nature is not moved out of it for that.
        assert_eq!(
            place_of_user_folder(
                Some(r"%USERPROFILE%\AppData\Roaming"),
                profile,
                APPDATA_ROAMING_SEGMENT,
                &no_environment
            ),
            UserFolderPlace::Usable(PathBuf::from(r"C:\Users\bob\AppData\Roaming"))
        );
    }

    #[test]
    fn documents_and_saved_games_are_a_users_only_directly_in_a_profile() {
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Users\alice\Documents\App\a.txt")),
            Some((UserDataAnchor::Documents, PathBuf::from(r"App\a.txt")))
        );
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Users\alice\Saved Games\App\slot.sav")),
            Some((UserDataAnchor::SavedGames, PathBuf::from(r"App\slot.sav")))
        );
        for not_theirs in [
            r"C:\Apps\Documents\a.txt",
            r"C:\Program Files\App\Documents\a.txt",
            r"C:\Users\alice\Projects\Documents\a.txt",
            r"D:\Saved Games\a.txt",
        ] {
            assert_eq!(
                split_user_data_relative(Path::new(not_theirs)),
                None,
                "{not_theirs}"
            );
        }
        // What other anchors claim stays theirs, even with these words further on.
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Users\Public\Documents\App\a.txt")),
            Some((
                UserDataAnchor::PublicProfile,
                PathBuf::from(r"Documents\App\a.txt")
            ))
        );
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Users\a\AppData\Local\App\Documents\b.txt")),
            Some((
                UserDataAnchor::LocalAppData,
                PathBuf::from(r"App\Documents\b.txt")
            ))
        );
    }

    #[test]
    fn a_file_from_a_redirected_folder_says_where_it_goes_itself() {
        let mut entry = external_entry(r"D:\Docs\App\a.txt");
        assert_eq!(data_location_of(&entry), None, "its path says nothing");
        entry.user_folder = Some(UserFolderPath {
            anchor: UserDataAnchor::Documents,
            relative: PathBuf::from(r"App\a.txt"),
        });
        assert_eq!(
            data_location_of(&entry),
            Some((UserDataAnchor::Documents, PathBuf::from(r"App\a.txt")))
        );
        // A package is not trusted to stay in the folder.
        entry.user_folder = Some(UserFolderPath {
            anchor: UserDataAnchor::Documents,
            relative: PathBuf::from(r"..\..\..\Windows\x.dll"),
        });
        let Some((_, tail)) = data_location_of(&entry) else {
            panic!("it has a location");
        };
        assert!(
            !tail
                .components()
                .any(|component| matches!(component, Component::ParentDir)),
            "{}",
            tail.display()
        );
    }

    #[test]
    fn the_folders_windows_keeps_for_old_programs_are_not_reported_and_others_are() -> Result<()> {
        // `My Music` in Documents and `Application Data` in AppData\Local are junctions that
        // refuse to be listed. They are not an app's, and every elevated scan used to warn of 14.
        let base = temp_base()?;
        let folder = base.path().join("Vendor");
        fs::create_dir_all(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
        let target = base.path().join("target");
        fs::create_dir_all(target.join("Demo"))
            .map_err(|error| WinAppMigratorError::io(&target, error))?;
        let link = base.path().join("Linked Vendor");
        assert!(make_junction(&link, &target));
        // Denied to everyone, as Windows does for these.
        let denied = crate::path_utils::hidden_command("icacls")
            .arg(&link)
            .args(["/deny", "*S-1-1-0:(RD)"])
            .output()
            .map_err(|error| WinAppMigratorError::io(&link, error))?;
        assert!(denied.status.success());

        let names = ProductNames::new(Some("Other Product"), None, &[], &[]);
        let mut roots = Vec::new();
        let mut warnings = Vec::new();
        collect_matching_dirs(
            base.path(),
            FootprintKind::ProgramData,
            &names,
            MatchScope::ProgramFolders,
            None,
            &mut roots,
            &mut warnings,
        );
        let undone = crate::path_utils::hidden_command("icacls")
            .arg(&link)
            .args(["/remove:d", "*S-1-1-0"])
            .output()
            .map_err(|error| WinAppMigratorError::io(&link, error))?;
        assert!(undone.status.success());
        assert!(roots.is_empty());
        assert!(warnings.is_empty(), "{warnings:?}");
        Ok(())
    }

    #[test]
    fn a_folder_that_refuses_to_be_listed_is_still_reported() -> Result<()> {
        let base = temp_base()?;
        let folder = base.path().join("Vendor");
        fs::create_dir_all(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
        let denied = crate::path_utils::hidden_command("icacls")
            .arg(&folder)
            .args(["/deny", "*S-1-1-0:(RD)"])
            .output()
            .map_err(|error| WinAppMigratorError::io(&folder, error))?;
        assert!(denied.status.success());
        let names = ProductNames::new(Some("Other Product"), None, &[], &[]);
        let mut roots = Vec::new();
        let mut warnings = Vec::new();
        collect_matching_dirs(
            base.path(),
            FootprintKind::ProgramData,
            &names,
            MatchScope::ProgramFolders,
            None,
            &mut roots,
            &mut warnings,
        );
        let undone = crate::path_utils::hidden_command("icacls")
            .arg(&folder)
            .args(["/remove:d", "*S-1-1-0"])
            .output()
            .map_err(|error| WinAppMigratorError::io(&folder, error))?;
        assert!(undone.status.success());
        assert_eq!(
            warnings
                .iter()
                .map(|warning| warning.code.as_str())
                .collect::<Vec<_>>(),
            ["file_discovery_dir_unreadable"]
        );
        Ok(())
    }

    #[test]
    fn a_scan_of_a_drive_takes_its_own_accounts_data_and_nothing_past_a_link() -> Result<()> {
        let base = temp_base()?;
        let volume = base.path().join("volume");
        let alice = volume.join(r"Users\alice");
        let bob = volume.join(r"Users\bob");
        let host = base.path().join("this PC");
        let install = volume.join(r"Program Files\Demo App");
        for (path, content) in [
            (install.join("demo.exe"), "MZ"),
            (install.join(r"bin\tool.dll"), "x"),
            (alice.join(r"NTUSER.DAT"), "x"),
            (alice.join(r"AppData\Roaming\Demo App\mine.txt"), "x"),
            (alice.join(r"Documents\Demo App\mine-doc.txt"), "x"),
            (alice.join(r"Saved Games\Demo App\mine.sav"), "x"),
            (bob.join(r"NTUSER.DAT"), "x"),
            (bob.join(r"AppData\Roaming\Demo App\bobs.txt"), "x"),
            (bob.join(r"AppData\Roaming\Taxes\return.txt"), "x"),
            (bob.join(r"Documents\letter.txt"), "x"),
            (host.join(r"Demo App\secret.txt"), "x"),
        ] {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| WinAppMigratorError::io(parent, error))?;
            }
            fs::write(&path, content).map_err(|error| WinAppMigratorError::io(&path, error))?;
        }
        for folder in [alice.join(r"AppData\Local"), volume.join("ProgramData")] {
            fs::create_dir_all(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
        }
        // A real folder and a real file whose names open the link beside them, which what a
        // listing says about them cannot show.
        let twin_of = |link: &Path| format!(r"\\?\{}.", link.display());
        assert!(make_junction(&install.join("twin"), &host.join("Demo App")));
        fs::create_dir(twin_of(&install.join("twin")))
            .map_err(|error| WinAppMigratorError::io(&install, error))?;
        // Inside what is walked, named for the app, on the way to a vendor's folder, and given as
        // a root by something on the drive.
        assert!(make_junction(
            &install.join("linked"),
            &host.join("Demo App")
        ));
        assert!(make_junction(
            &alice.join(r"AppData\Local\Demo App"),
            &host.join("Demo App")
        ));
        assert!(make_junction(&volume.join(r"ProgramData\Vendor"), &host));
        assert!(make_junction(
            &volume.join("Elsewhere"),
            &host.join("Demo App")
        ));

        let system = SystemContext::offline_for_tests(&volume, &alice);
        let (files, _, warnings) = discover_files(
            &FileDiscoveryContext {
                executable_path: install.join("demo.exe"),
                install_location: Some(install.clone()),
                app_name: Some("Demo App".to_owned()),
                publisher: Some("Vendor".to_owned()),
                include_hashes: false,
                include_user_data: true,
                include_program_data: true,
                installed_for_one_account: false,
                extra_roots: vec![bob.join(r"AppData\Roaming\Taxes"), volume.join("Elsewhere")],
                extra_files: vec![bob.join(r"Documents\letter.txt")],
            },
            &system,
        );

        let mut names: Vec<String> = files
            .iter()
            .map(|file| file_name_string(&file.path))
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "demo.exe",
                "mine-doc.txt",
                "mine.sav",
                "mine.txt",
                "tool.dll"
            ],
            "not bob's, however they were named, and not what is past a link"
        );
        let in_documents = files
            .iter()
            .find(|file| file_name_string(&file.path) == "mine-doc.txt");
        assert_eq!(
            in_documents.and_then(|file| file.user_folder.clone()),
            Some(UserFolderPath {
                anchor: UserDataAnchor::Documents,
                relative: PathBuf::from(r"Demo App\mine-doc.txt"),
            }),
            "it says where it goes"
        );
        let left_out: Vec<String> = warnings
            .iter()
            .filter(|warning| warning.code == "file_discovery_left_out")
            .filter_map(|warning| warning.source.clone())
            .collect();
        for told in [
            bob.join(r"AppData\Roaming\Taxes"),
            bob.join(r"Documents\letter.txt"),
            volume.join("Elsewhere"),
            alice.join(r"AppData\Local\Demo App"),
        ] {
            assert!(
                left_out.contains(&told.display().to_string()),
                "{} should be said to be left out: {left_out:?}",
                told.display()
            );
        }
        Ok(())
    }

    #[test]
    fn a_short_name_or_a_folder_above_one_does_not_make_a_shared_folder_an_apps() -> Result<()> {
        let base = temp_base()?;
        let profiles = base.path().join("Profiles");
        let bob = profiles.join("bob");
        fs::create_dir_all(&bob).map_err(|error| WinAppMigratorError::io(&bob, error))?;
        // A link stands in for a short name here. Each is another way to write the same place.
        let other_name = base.path().join("BOB1");
        assert!(make_junction(&other_name, &bob));
        let bases = vec![bob.clone(), resolved_location(&bob)];
        assert!(holds_a_known_base_dir(&bob, &bases));
        assert!(holds_a_known_base_dir(&profiles, &bases));
        assert!(holds_a_known_base_dir(&other_name, &bases));
        assert!(!holds_a_known_base_dir(&bob.join("Vendor"), &bases));
        assert!(!is_walkable_app_dir(
            &profiles,
            &bases,
            Path::new(r"C:\Windows")
        ));
        Ok(())
    }

    #[test]
    fn nothing_past_a_link_on_a_drive_is_looked_at() -> Result<()> {
        let base = temp_base()?;
        let volume = base.path().join("volume");
        let data = volume.join("ProgramData");
        fs::create_dir_all(data.join(r"Vendor\Demo"))
            .map_err(|error| WinAppMigratorError::io(&data, error))?;
        let host = base.path().join("this PC");
        fs::create_dir_all(host.join("Demo"))
            .map_err(|error| WinAppMigratorError::io(&host, error))?;
        assert!(make_junction(&data.join("Linked Vendor"), &host));
        assert!(make_junction(&data.join("Demo Link"), &host));

        let names = ProductNames::new(Some("Demo"), Some("Vendor"), &[], &[]);
        let drive = VolumeMap {
            volume_root: volume.clone(),
            system_drive: 'C',
        };
        let mut roots = Vec::new();
        let mut warnings = Vec::new();
        collect_matching_dirs(
            &data,
            FootprintKind::ProgramData,
            &names,
            MatchScope::ProgramFolders,
            Some(&drive),
            &mut roots,
            &mut warnings,
        );
        let found: Vec<PathBuf> = roots.iter().map(|root| root.path.clone()).collect();
        assert!(found.contains(&data.join(r"Vendor\Demo")), "{found:?}");
        assert!(
            !found.contains(&data.join(r"Linked Vendor\Demo")),
            "that is on this PC: {found:?}"
        );
        for root in &roots {
            if root.path != data.join(r"Vendor\Demo") {
                assert!(!drive.may_read(&root.path), "{}", root.path.display());
            }
        }

        // And the walk of what is real does not leave by a link inside it.
        let real = data.join(r"Vendor\Demo");
        fs::write(real.join("own.txt"), b"x")
            .map_err(|error| WinAppMigratorError::io(&real, error))?;
        fs::write(host.join("secret.txt"), b"x")
            .map_err(|error| WinAppMigratorError::io(&host, error))?;
        assert!(make_junction(&real.join("linked"), &host));
        assert_eq!(
            walk_tree(&real, FootprintKind::ProgramData),
            vec!["own.txt"]
        );
        Ok(())
    }

    #[test]
    fn computes_install_relative_package_path() {
        let path = package_relative_path(
            Path::new(r"C:\Apps\Demo\bin\demo.exe"),
            Some(Path::new(r"C:\Apps\Demo")),
        );

        assert_eq!(path, PathBuf::from(r"files\bin\demo.exe"));
    }

    #[test]
    fn classifies_appdata_files_as_user_data() {
        let classification = classify_file(
            Path::new(r"C:\Users\sample\AppData\Roaming\Demo\settings.json"),
            Some(Path::new(r"C:\Program Files\Demo")),
            &FootprintKind::RoamingAppData,
        );

        assert_eq!(classification, FileClassification::UserData);
    }

    #[test]
    fn install_directory_files_are_app_owned() {
        let classification = classify_file(
            Path::new(r"C:\Program Files\Vendor\Demo\demo.dll"),
            Some(Path::new(r"C:\Program Files\Demo")),
            &FootprintKind::InstallDirectory,
        );

        assert_eq!(classification, FileClassification::AppOwned);
    }

    #[test]
    fn program_data_is_machine_app_data_not_user_data() {
        let classification = classify_file(
            Path::new(r"C:\ProgramData\Demo\machine.dat"),
            Some(Path::new(r"C:\Program Files\Demo")),
            &FootprintKind::ProgramData,
        );
        assert_eq!(classification, FileClassification::MachineAppData);
    }

    #[test]
    fn documents_and_saved_games_are_user_data() {
        let documents = classify_file(
            Path::new(r"C:\Users\sample\Documents\Demo\save.json"),
            None,
            &FootprintKind::Documents,
        );
        let saved = classify_file(
            Path::new(r"C:\Users\sample\Saved Games\Demo\slot1.sav"),
            None,
            &FootprintKind::SavedGames,
        );
        assert_eq!(documents, FileClassification::UserData);
        assert_eq!(saved, FileClassification::UserData);
    }

    #[test]
    fn public_documents_are_machine_app_data() {
        let classification = classify_file(
            Path::new(r"C:\Users\Public\Documents\YDIS_Data\Config.xml"),
            None,
            &FootprintKind::PublicDocuments,
        );
        assert_eq!(classification, FileClassification::MachineAppData);
    }

    #[test]
    fn public_profile_reanchors_to_public_base() {
        // A shared Public path splits on the "\users\public\" marker so the tail
        // starts after it, and re-anchors to %PUBLIC% (fixed at C:\Users\Public),
        // i.e. back to the same absolute location on the target.
        let path = Path::new(r"C:\Users\Public\Documents\YDIS_Data\Config.xml");
        let Some((anchor, tail)) = split_user_data_relative(path) else {
            panic!("public path should split on the marker");
        };
        assert_eq!(anchor, UserDataAnchor::PublicProfile);
        assert_eq!(tail, PathBuf::from(r"Documents\YDIS_Data\Config.xml"));

        let Some(base) = user_data_base(anchor) else {
            panic!("public base should resolve to a destination");
        };
        let destination = base.join(&tail);
        assert!(
            destination.ends_with(r"Documents\YDIS_Data\Config.xml"),
            "unexpected destination: {}",
            destination.display()
        );
        assert!(
            destination
                .to_string_lossy()
                .to_ascii_lowercase()
                .contains(r"\public\")
        );
    }

    #[test]
    fn detects_online_only_cloud_placeholders() {
        assert!(is_online_only(FILE_ATTRIBUTE_OFFLINE));
        assert!(is_online_only(FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS));
        assert!(is_online_only(
            FILE_ATTRIBUTE_OFFLINE | 0x20 /* archive */
        ));
        assert!(!is_online_only(0x20));
    }

    #[test]
    fn prunes_cache_directories() {
        assert!(is_volatile_dir(
            Path::new(r"C:\Users\me\AppData\Local\Demo\Cache"),
            true
        ));
        assert!(is_volatile_dir(
            Path::new(r"C:\Users\me\AppData\Local\Demo\GPUCache"),
            true
        ));
        assert!(!is_volatile_dir(
            Path::new(r"C:\Users\me\AppData\Local\Demo\Profiles"),
            true
        ));
    }

    fn unique_temp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "wam-discovery-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ))
    }

    fn matching_dirs(base: &Path, names: &ProductNames, scope: MatchScope) -> Vec<PathBuf> {
        let mut roots = Vec::new();
        let mut warnings = Vec::new();
        collect_matching_dirs(
            base,
            FootprintKind::ProgramData,
            names,
            scope,
            None,
            &mut roots,
            &mut warnings,
        );
        let mut paths: Vec<PathBuf> = roots.into_iter().map(|root| root.path).collect();
        paths.sort();
        paths
    }

    #[test]
    fn matches_nested_vendor_app_directories() {
        let base = unique_temp("nested");
        let nested = base.join("Apple Computer").join("iTunes");
        let unrelated = base.join("AnyDesk");
        let _ = fs::create_dir_all(&nested);
        let _ = fs::create_dir_all(&unrelated);

        let names = ProductNames::new(Some("iTunes"), Some("Apple Inc."), &["iTunes"], &[]);
        let found = matching_dirs(&base, &names, MatchScope::ProgramFolders);
        let _ = fs::remove_dir_all(&base);

        assert_eq!(
            found,
            vec![nested],
            "the nested product folder matches at depth two, and nothing else does"
        );
    }

    #[test]
    fn selects_the_product_and_never_the_vendor_or_its_other_products() {
        let base = unique_temp("vendor");
        for folder in [
            r"Mozilla\Firefox",
            r"Mozilla\Thunderbird",
            r"Mozilla\Extensions",
            "Mozilla Thunderbird",
            r"ms-playwright\firefox-1543",
            r"grok\git",
        ] {
            let _ = fs::create_dir_all(base.join(folder));
        }

        let names = ProductNames::new(
            Some("Mozilla Firefox (x64 en-US)"),
            Some("Mozilla"),
            &["firefox", "Mozilla Firefox"],
            &[],
        );
        let found = matching_dirs(&base, &names, MatchScope::ProgramFolders);
        let _ = fs::remove_dir_all(&base);

        assert_eq!(found, vec![base.join(r"Mozilla\Firefox")]);
    }

    #[test]
    fn product_names_never_select_on_the_install_parent() {
        let context = FileDiscoveryContext {
            executable_path: PathBuf::from(
                r"C:\Users\me\AppData\Local\Programs\Microsoft VS Code\Code.exe",
            ),
            install_location: Some(PathBuf::from(
                r"C:\Users\me\AppData\Local\Programs\Microsoft VS Code",
            )),
            app_name: Some("Microsoft Visual Studio Code (User)".to_owned()),
            publisher: Some("Microsoft Corporation".to_owned()),
            include_hashes: false,
            include_user_data: true,
            include_program_data: true,
            installed_for_one_account: false,
            extra_roots: Vec::new(),
            extra_files: Vec::new(),
        };
        let names = product_names(&context);

        assert!(!names.matches_top_level("Programs", MatchScope::ProgramFolders));
        assert!(names.matches_top_level("Code", MatchScope::ProgramFolders));
        assert!(names.matches_top_level("Microsoft VS Code", MatchScope::ProgramFolders));
        assert!(!names.matches_top_level("Microsoft Visual Studio", MatchScope::ProgramFolders));
    }

    #[test]
    fn user_folders_are_not_taken_by_an_executable_name() {
        let base = unique_temp("documents");
        for folder in ["Code", "Git", "PowerShell", "WindowsPowerShell"] {
            let _ = fs::create_dir_all(base.join(folder));
        }
        let names = ProductNames::new(
            Some("PowerShell 7.6.9.0-x64"),
            Some("Microsoft Corporation"),
            &["Code", "Git"],
            &[],
        );
        let found = matching_dirs(&base, &names, MatchScope::UserFolders);
        let _ = fs::remove_dir_all(&base);

        assert_eq!(found, vec![base.join("PowerShell")]);
    }

    /// The folders below `root` with nothing in them, as a walk with these options finds them.
    fn empty_folders_of(
        root: &Path,
        kind: FootprintKind,
        install_root: Option<&Path>,
        include_user_data: bool,
    ) -> Vec<(String, FileClassification)> {
        let mut files = Vec::new();
        let mut empty_folders = Vec::new();
        let mut seen = BTreeSet::new();
        let mut warnings = Vec::new();
        discover_tree(
            &DiscoveryRoot {
                path: root.to_path_buf(),
                kind,
                scope: InstallScope::PerUser,
                user_folder: None,
            },
            install_root,
            false,
            include_user_data,
            false,
            &SystemContext::live(),
            &|| false,
            &mut files,
            &mut empty_folders,
            &mut seen,
            &mut warnings,
        );
        let mut found: Vec<(String, FileClassification)> = empty_folders
            .into_iter()
            .map(|folder| {
                assert!(folder.should_copy);
                assert!(folder.package_relative_path.is_some());
                let relative = strip_prefix_ignore_case(&folder.original_path, root)
                    .unwrap_or_default()
                    .display()
                    .to_string();
                (relative, folder.classification)
            })
            .collect();
        found.sort_by(|left, right| left.0.cmp(&right.0));
        found
    }

    fn warnings_of_walking(root: &Path) -> (Vec<FileEntry>, Vec<FileEntry>, Vec<Warning>) {
        let mut files = Vec::new();
        let mut empty_folders = Vec::new();
        let mut seen = BTreeSet::new();
        let mut warnings = Vec::new();
        discover_tree(
            &DiscoveryRoot {
                path: root.to_path_buf(),
                kind: FootprintKind::InstallDirectory,
                scope: InstallScope::PerMachine,
                user_folder: None,
            },
            Some(root),
            false,
            false,
            false,
            &SystemContext::live(),
            &|| false,
            &mut files,
            &mut empty_folders,
            &mut seen,
            &mut warnings,
        );
        (files, empty_folders, warnings)
    }

    #[test]
    fn a_link_in_an_apps_folder_is_named_as_it_is_left_out() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let install = work.path().join("Demo");
        let elsewhere = work.path().join("Elsewhere");
        fs::create_dir_all(&install).map_err(|error| WinAppMigratorError::io(&install, error))?;
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        fs::write(elsewhere.join("a.dll"), "x")
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        fs::write(install.join("app.exe"), "x")
            .map_err(|error| WinAppMigratorError::io(&install, error))?;
        assert!(make_junction(&install.join("current"), &elsewhere));
        let (files, _, warnings) = warnings_of_walking(&install);
        assert!(
            !files
                .iter()
                .any(|file| file.original_path.ends_with("a.dll")),
            "{files:?}"
        );
        let warning = warnings
            .iter()
            .find(|warning| warning.code == "links_not_followed")
            .ok_or_else(|| WinAppMigratorError::Archive(format!("nothing said: {warnings:?}")))?;
        assert!(warning.message.contains("current"), "{warning:?}");
        Ok(())
    }

    #[test]
    fn what_is_too_deep_is_said_to_be_and_is_not_an_empty_folder() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let install = work.path().join("Demo");
        let mut deep = install.clone();
        for level in 0..DEEPEST_WALK + 3 {
            deep = deep.join(format!("{}", level % 10));
        }
        fs::create_dir_all(&deep).map_err(|error| WinAppMigratorError::io(&deep, error))?;
        fs::write(deep.join("too-deep.dll"), "x")
            .map_err(|error| WinAppMigratorError::io(&deep, error))?;
        let mut within_reach = install.clone();
        for level in 0..DEEPEST_WALK - 2 {
            within_reach = within_reach.join(format!("{}", level % 10));
        }
        fs::write(within_reach.join("deep.dll"), "x")
            .map_err(|error| WinAppMigratorError::io(&within_reach, error))?;
        let (files, empty_folders, warnings) = warnings_of_walking(&install);
        assert!(
            files
                .iter()
                .any(|file| file.original_path.ends_with("deep.dll")),
            "as far down as anything real is"
        );
        assert!(
            !files
                .iter()
                .any(|file| file.original_path.ends_with("too-deep.dll")),
        );
        assert!(empty_folders.is_empty(), "{empty_folders:?}");
        assert!(
            warnings
                .iter()
                .any(|warning| warning.code == "folders_too_deep"),
            "{warnings:?}"
        );
        Ok(())
    }

    #[test]
    fn a_folder_with_nothing_in_it_is_found_and_one_on_the_way_to_it_is_not() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let install = work.path().join("Demo");
        let made = |relative: &str| -> Result<()> {
            let path = install.join(relative);
            fs::create_dir_all(&path).map_err(|error| WinAppMigratorError::io(&path, error))
        };
        made(r"textfile_inputs")?;
        made(r"a\b\deep")?;
        made(r"has_a_file")?;
        let file = install.join(r"has_a_file\x.txt");
        fs::write(&file, b"x").map_err(|error| WinAppMigratorError::io(&file, error))?;

        let found = empty_folders_of(
            &install,
            FootprintKind::InstallDirectory,
            Some(&install),
            false,
        );

        let names: Vec<&str> = found.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, [r"a\b\deep", "textfile_inputs"]);
        assert!(
            found
                .iter()
                .all(|(_, classification)| *classification == FileClassification::AppOwned)
        );
        Ok(())
    }

    #[test]
    fn an_empty_folder_of_data_goes_by_the_option_as_a_file_there_would() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let data = work.path().join("Demo");
        let inside = data.join("saves");
        fs::create_dir_all(&inside).map_err(|error| WinAppMigratorError::io(&inside, error))?;

        assert!(empty_folders_of(&data, FootprintKind::RoamingAppData, None, false).is_empty());
        let found = empty_folders_of(&data, FootprintKind::RoamingAppData, None, true);
        assert_eq!(
            found,
            [("saves".to_owned(), FileClassification::UserData)],
            "and the folder it is in has something in it, so it is not one"
        );
        Ok(())
    }

    #[test]
    fn a_data_folder_with_nothing_in_it_is_one_and_the_install_folder_is_not() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let folder = work.path().join("Demo");
        fs::create_dir_all(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;

        let found = empty_folders_of(&folder, FootprintKind::RoamingAppData, None, true);
        assert_eq!(found.len(), 1);
        assert!(
            empty_folders_of(
                &folder,
                FootprintKind::InstallDirectory,
                Some(&folder),
                false
            )
            .is_empty()
        );
        Ok(())
    }

    #[test]
    fn a_link_to_a_folder_is_not_an_empty_folder() -> Result<()> {
        let work =
            tempfile::tempdir().map_err(|error| WinAppMigratorError::io(Path::new("."), error))?;
        let install = work.path().join("Demo");
        let elsewhere = work.path().join("Elsewhere");
        fs::create_dir_all(&install).map_err(|error| WinAppMigratorError::io(&install, error))?;
        fs::create_dir_all(&elsewhere)
            .map_err(|error| WinAppMigratorError::io(&elsewhere, error))?;
        assert!(make_junction(&install.join("link"), &elsewhere));

        assert!(
            empty_folders_of(
                &install,
                FootprintKind::InstallDirectory,
                Some(&install),
                false
            )
            .is_empty()
        );
        Ok(())
    }

    fn walk_tree(root: &Path, kind: FootprintKind) -> Vec<String> {
        let mut files = Vec::new();
        let mut empty_folders = Vec::new();
        let mut seen = BTreeSet::new();
        let mut warnings = Vec::new();
        discover_tree(
            &DiscoveryRoot {
                path: root.to_path_buf(),
                kind,
                scope: InstallScope::PerUser,
                user_folder: None,
            },
            None,
            false,
            true,
            true,
            &SystemContext::live(),
            &|| false,
            &mut files,
            &mut empty_folders,
            &mut seen,
            &mut warnings,
        );
        let mut names: Vec<String> = files
            .iter()
            .filter_map(|file| {
                file.original_path
                    .strip_prefix(root)
                    .ok()
                    .map(|relative| relative.display().to_string())
            })
            .collect();
        names.sort();
        names
    }

    #[test]
    fn install_folders_keep_files_under_volatile_names_and_data_folders_do_not() {
        let root = unique_temp("volatile");
        let _ = fs::create_dir_all(root.join(r"node-compile-cache\v24"));
        let _ = fs::create_dir_all(root.join("Logs"));
        let _ = fs::write(root.join(r"node-compile-cache\v24\00b791e3"), b"x");
        let _ = fs::write(root.join(r"Logs\shipped.txt"), b"x");
        let _ = fs::write(root.join("app.dll"), b"x");

        let install = walk_tree(&root, FootprintKind::InstallDirectory);
        let data = walk_tree(&root, FootprintKind::LocalAppData);
        let _ = fs::remove_dir_all(&root);

        assert_eq!(
            install,
            vec![
                r"Logs\shipped.txt".to_owned(),
                "app.dll".to_owned(),
                r"node-compile-cache\v24\00b791e3".to_owned(),
            ]
        );
        assert_eq!(data, vec!["app.dll".to_owned()]);
    }

    #[test]
    fn walks_a_folder_on_a_drive_but_not_the_drive_or_a_known_base() {
        let windows = Path::new(r"C:\Windows");
        let temp = std::env::temp_dir();
        let top_level = temp
            .ancestors()
            .find(|ancestor| ancestor.parent().is_some_and(is_drive_root));
        if let Some(top_level) = top_level {
            assert!(
                is_walkable_app_dir(top_level, &[], windows),
                "a folder directly on a drive is a normal install location"
            );
            assert!(!is_walkable_app_dir(
                top_level,
                &[top_level.to_path_buf()],
                windows
            ));
        }
        assert!(!is_walkable_app_dir(Path::new(r"C:\"), &[], windows));
        assert!(!is_walkable_app_dir(windows, &[], windows));
        assert!(!is_walkable_app_dir(
            Path::new(r"C:\Windows\System32"),
            &[],
            windows
        ));
    }

    #[test]
    fn a_windows_folder_inside_an_app_is_not_the_windows_directory() {
        let parent = unique_temp("windows-segment");
        let root = parent.join("windows");
        let _ = fs::create_dir_all(&root);
        let walkable = is_walkable_app_dir(&root, &[], Path::new(r"C:\Windows"));
        let _ = fs::remove_dir_all(&parent);

        assert!(walkable);
    }

    #[test]
    fn known_bases_include_the_shared_per_user_programs_folder() {
        let system = SystemContext::live();
        let bases = known_base_dirs(&system);
        for profile in system.user_profiles() {
            assert!(holds_a_known_base_dir(
                &profile.join(r"AppData\Local\Programs\"),
                &bases
            ));
        }
        for program_files in system.program_files_dirs() {
            assert!(holds_a_known_base_dir(&program_files, &bases));
            assert!(holds_a_known_base_dir(
                &program_files.join("Common Files"),
                &bases
            ));
        }
    }

    #[test]
    fn a_folder_of_its_own_outside_program_files_is_app_owned() {
        assert_eq!(
            classify_file(
                Path::new(r"C:\CocosDashboard\ffmpeg.dll"),
                None,
                &FootprintKind::RegistryReference
            ),
            FileClassification::AppOwned
        );
        assert_eq!(
            classify_file(
                Path::new(r"C:\Users\me\Documents\Demo\a.txt"),
                None,
                &FootprintKind::RegistryReference
            ),
            FileClassification::UserData
        );
    }

    #[test]
    fn captures_recorded_roots_and_loose_files_outside_the_install_folder() {
        let base = unique_temp("recorded");
        let install = base.join("WAM Recorded Demo App");
        let recorded = base.join("Elsewhere Recorded");
        let loose = base.join("Shared").join("loose.dll");
        let unrelated = base.join("Shared").join("unrelated.dll");
        let _ = fs::create_dir_all(&install);
        let _ = fs::create_dir_all(&recorded);
        let _ = fs::create_dir_all(base.join("Shared"));
        let exe = install.join("wam-recorded-demo.exe");
        let _ = fs::write(&exe, b"MZ");
        let _ = fs::write(recorded.join("data.bin"), b"x");
        let _ = fs::write(&loose, b"x");
        let _ = fs::write(&unrelated, b"x");

        let (files, _, _) = discover_files(
            &FileDiscoveryContext {
                executable_path: exe.clone(),
                install_location: Some(install),
                app_name: Some("WAM Recorded Demo App".to_owned()),
                publisher: None,
                include_hashes: false,
                include_user_data: true,
                include_program_data: true,
                installed_for_one_account: false,
                extra_roots: vec![recorded.clone()],
                extra_files: vec![loose.clone()],
            },
            &SystemContext::live(),
        );
        let _ = fs::remove_dir_all(&base);

        let captured: Vec<&Path> = files.iter().map(|file| file.path.as_path()).collect();
        assert!(captured.contains(&exe.as_path()));
        assert!(captured.contains(&recorded.join("data.bin").as_path()));
        assert!(captured.contains(&loose.as_path()));
        assert!(
            !captured.contains(&unrelated.as_path()),
            "a loose file does not bring its neighbors"
        );
    }

    #[test]
    fn package_paths_ignore_case_differences_in_the_install_root() {
        assert_eq!(
            package_relative_path(
                Path::new(r"C:\Program Files\NVIDIA Corporation\NVIDIA app\7z.dll"),
                Some(Path::new(r"C:\Program Files\NVIDIA Corporation\NVIDIA App")),
            ),
            PathBuf::from(r"files\7z.dll")
        );
    }

    fn external_entry(original: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(original),
            original_path: PathBuf::from(original),
            package_relative_path: Some(
                PathBuf::from(PACKAGE_FILES_ROOT)
                    .join(EXTERNAL_FILES_DIR)
                    .join("payload.bin"),
            ),
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    #[test]
    fn splits_user_data_anchors() {
        assert_eq!(
            split_user_data_relative(Path::new(
                r"C:\Users\alice\AppData\Roaming\Vendor\App\settings.json"
            )),
            Some((
                UserDataAnchor::RoamingAppData,
                PathBuf::from(r"Vendor\App\settings.json")
            ))
        );
        assert_eq!(
            split_user_data_relative(Path::new(
                r"C:\Users\alice\AppData\LocalLow\Sun\Java\deployment.bin"
            )),
            Some((
                UserDataAnchor::LocalLowAppData,
                PathBuf::from(r"Sun\Java\deployment.bin")
            )),
            "LocalLow must win over the Local prefix"
        );
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Users\alice\AppData\Local\App\cache.db")),
            Some((UserDataAnchor::LocalAppData, PathBuf::from(r"App\cache.db")))
        );
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\ProgramData\App\machine.dat")),
            Some((
                UserDataAnchor::ProgramData,
                PathBuf::from(r"App\machine.dat")
            ))
        );
        assert_eq!(
            split_user_data_relative(Path::new(r"C:\Program Files\App\app.exe")),
            None
        );
    }

    #[test]
    fn restore_destination_falls_back_to_sandbox_when_disabled() -> Result<()> {
        let entry = external_entry(r"C:\Users\alice\AppData\Roaming\Vendor\App\settings.json");
        let target = std::env::temp_dir();
        let destination = restore_destination_for_entry(&entry, &target, false)?;

        assert_eq!(
            destination,
            target.join(EXTERNAL_FILES_DIR).join("payload.bin")
        );
        Ok(())
    }

    #[test]
    fn restore_destination_reanchors_user_data_when_enabled() -> Result<()> {
        let Some(appdata) = std::env::var_os("APPDATA").map(PathBuf::from) else {
            return Ok(());
        };
        let entry = external_entry(r"C:\Users\alice\AppData\Roaming\Vendor\App\settings.json");
        let target = std::env::temp_dir();
        let destination = restore_destination_for_entry(&entry, &target, true)?;

        assert_eq!(destination, appdata.join(r"Vendor\App\settings.json"));
        assert!(!destination.starts_with(&target));
        Ok(())
    }

    #[test]
    fn app_owned_files_ignore_user_data_routing() -> Result<()> {
        let entry = FileEntry {
            path: PathBuf::from(r"C:\Program Files\App\bin\app.exe"),
            original_path: PathBuf::from(r"C:\Program Files\App\bin\app.exe"),
            package_relative_path: Some(PathBuf::from(r"files\bin\app.exe")),
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        };
        let target = std::env::temp_dir();
        let destination = restore_destination_for_entry(&entry, &target, true)?;

        assert_eq!(destination, target.join(r"bin\app.exe"));
        Ok(())
    }
}
