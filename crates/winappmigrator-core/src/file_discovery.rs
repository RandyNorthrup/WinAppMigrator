use std::collections::BTreeSet;
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;
use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;

use crate::errors::{Result, WinAppMigratorError};
use crate::hashing::sha256_file;
use crate::models::{
    FileClassification, FileEntry, FootprintKind, FootprintLocation, InstallScope, Warning,
    WarningSeverity,
};
use crate::path_utils::{
    file_name_string, is_auxiliary_executable, is_drive_root, path_has_extension, path_is_within,
    program_data_dir, public_profile_dir, strip_prefix_ignore_case, user_profile_dir,
};
use crate::privilege::hold_folder;
use crate::product_match::{MatchScope, ProductNames};
use crate::system_context::SystemContext;

const MAX_WALK_DEPTH: usize = 24;
/// `User Shell Folders` holds the (possibly redirected) per-user known-folder
/// paths as `REG_EXPAND_SZ` values such as `%USERPROFILE%\Documents`.
const USER_SHELL_FOLDERS_KEY: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders";
const DOCUMENTS_VALUE: &str = "Personal";
/// `User Shell Folders` value name (a known-folder GUID) for Saved Games.
const SAVED_GAMES_VALUE: &str = "{4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}";
const DOCUMENTS_FALLBACK: &str = "Documents";
const SAVED_GAMES_FALLBACK: &str = "Saved Games";
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
    /// Additional roots discovered elsewhere (service binaries, shortcut
    /// targets, scheduled task actions, App Paths) to fold into the footprint.
    pub extra_roots: Vec<PathBuf>,
    /// Files an install record names that have no folder of the app's own to walk, such
    /// as one directly under `Common Files`. Captured one by one.
    pub extra_files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
struct DiscoveryRoot {
    path: PathBuf,
    kind: FootprintKind,
    scope: InstallScope,
}

pub fn discover_files(
    context: &FileDiscoveryContext,
    system: &SystemContext,
) -> (Vec<FileEntry>, Vec<FootprintLocation>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let mut files = Vec::new();
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
        &mut warnings,
    );

    for extra in &context.extra_files {
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
            &mut files,
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

    files.sort_by(|left, right| {
        left.original_path
            .to_string_lossy()
            .to_ascii_lowercase()
            .cmp(&right.original_path.to_string_lossy().to_ascii_lowercase())
    });
    locations.sort_by_key(|location| std::cmp::Reverse(location.total_bytes));

    (files, locations, warnings)
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

/// Create the install location, hold it open, and confirm it is a real folder. Run once,
/// before anything is written: a junction planted there would carry the whole install
/// somewhere else. The result keeps it and the folders above it from being renamed and
/// replaced, so keep it until the install is finished.
pub fn ensure_target_root(target_root: &Path) -> Result<fs::File> {
    if !target_root.is_absolute() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the install location must be a full path: \"{}\"",
            target_root.display()
        )));
    }
    fs::create_dir_all(target_root)
        .map_err(|source| WinAppMigratorError::io(target_root, source))?;
    let hold =
        hold_folder(target_root).map_err(|source| WinAppMigratorError::io(target_root, source))?;
    let metadata = fs::symlink_metadata(target_root)
        .map_err(|source| WinAppMigratorError::io(target_root, source))?;
    if metadata.file_type().is_symlink() {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "the install location is a link to somewhere else: {}",
            target_root.display()
        )));
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
/// file, and Windows 10 does not. Opening one there reaches the device or fails, which
/// harms nothing and shows up as an error, so only a real app's file is at stake either way.
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

fn reject_reserved_device_names(path: &Path) -> Result<()> {
    if has_reserved_device_name(path) {
        return Err(WinAppMigratorError::InvalidInput(format!(
            "path uses a name reserved for a Windows device: {}",
            path.display()
        )));
    }
    Ok(())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserDataAnchor {
    RoamingAppData,
    LocalAppData,
    LocalLowAppData,
    ProgramData,
    /// The shared Public profile (`C:\Users\Public`). Machine-wide and fixed
    /// across machines, so it re-anchors to the same absolute location.
    PublicProfile,
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
        && let Some(destination) = user_data_real_destination(&entry.original_path)?
    {
        return Ok(destination);
    }
    restore_destination(target_root, relative)
}

/// True when a package-relative path was staged under `files\_external\…`
/// (data captured from outside the install directory, i.e. user/ProgramData).
fn is_external_package_path(package_relative_path: &Path) -> bool {
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

/// Map an archived user-data source path to its real location on the target
/// machine, e.g. `C:\Users\alice\AppData\Roaming\App\x` →
/// `%APPDATA%\App\x`. Returns `Ok(None)` when the path is not under a known
/// user-data anchor or the target base cannot be resolved.
pub fn user_data_real_destination(original_path: &Path) -> Result<Option<PathBuf>> {
    let Some((anchor, tail)) = split_user_data_relative(original_path) else {
        return Ok(None);
    };
    let Some(base) = user_data_base(anchor) else {
        return Ok(None);
    };
    reject_reserved_device_names(&tail)?;
    Ok(Some(base.join(&tail)))
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

/// Resolve the target-machine base directory for a user-data anchor from the
/// current process environment. LocalLow has no dedicated variable, so it is
/// derived from the user profile.
pub(crate) fn user_data_base(anchor: UserDataAnchor) -> Option<PathBuf> {
    match anchor {
        UserDataAnchor::RoamingAppData => user_profile_folder("APPDATA", APPDATA_ROAMING_SEGMENT),
        UserDataAnchor::LocalAppData => user_profile_folder("LOCALAPPDATA", APPDATA_LOCAL_SEGMENT),
        UserDataAnchor::LocalLowAppData => user_profile_dir()
            .map(|profile| profile.join(APPDATA_LOCAL_LOW_SEGMENT))
            .or_else(|| {
                std::env::var_os("USERPROFILE")
                    .map(|profile| PathBuf::from(profile).join(APPDATA_LOCAL_LOW_SEGMENT))
            }),
        // The machine-wide folders come from the admin-only registry, because the
        // environment of an elevated process is partly the user's to set. The marker
        // for the Public profile includes the trailing "public\", so the tail excludes it.
        UserDataAnchor::ProgramData => program_data_dir()
            .map(Path::to_path_buf)
            .or_else(|| std::env::var_os("ProgramData").map(PathBuf::from)),
        UserDataAnchor::PublicProfile => public_profile_dir()
            .map(Path::to_path_buf)
            .or_else(|| std::env::var_os("PUBLIC").map(PathBuf::from)),
    }
}

/// A per-user folder, from its variable as long as that is inside this account's profile,
/// which the admin-only registry says where to find. The variable can be set by any
/// program, and an elevated process writes wherever it points. With no profile on record,
/// as for a system account, the variable is all there is.
fn user_profile_folder(variable: &str, default: &str) -> Option<PathBuf> {
    let recorded = std::env::var_os(variable).map(PathBuf::from);
    checked_user_folder(recorded, user_profile_dir(), default)
}

fn checked_user_folder(
    recorded: Option<PathBuf>,
    profile: Option<&Path>,
    default: &str,
) -> Option<PathBuf> {
    let Some(profile) = profile else {
        return recorded;
    };
    recorded
        .filter(|folder| path_is_within(folder, profile))
        .or_else(|| Some(profile.join(default)))
}

/// The names this app's own folders can go by. The install folder's parent only ever
/// qualifies a nested match as a vendor folder: selecting on it is how an app in
/// `AppData\Local\Programs` used to take everything else installed there.
fn product_names(context: &FileDiscoveryContext) -> ProductNames {
    let mut aliases = Vec::new();
    if !is_auxiliary_executable(&context.executable_path)
        && let Some(stem) = context.executable_path.file_stem().and_then(|s| s.to_str())
    {
        aliases.push(stem);
    }
    let mut vendor_folders = Vec::new();
    if let Some(install) = context.install_location.as_ref() {
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
    ProductNames::new(
        context.app_name.as_deref(),
        context.publisher.as_deref(),
        &aliases,
        &vendor_folders,
    )
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

    collect_program_files_roots(names, &mut raw, system, warnings);
    collect_program_data_roots(names, &mut raw, system, warnings);
    collect_appdata_roots(names, &mut raw, system, warnings);
    collect_known_user_folder_roots(names, &mut raw, system, warnings);

    let known_base_dirs = known_base_dirs(system);
    let windows_dir = system.windows_dir();
    // Deduplicate while preserving order so the most authoritative root (the
    // install location) owns each shared file, and drop any root nested inside a
    // root already kept so subtrees are not walked twice.
    let mut seen_paths = BTreeSet::new();
    let mut roots: Vec<DiscoveryRoot> = Vec::new();
    for root in raw {
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
    roots.push(DiscoveryRoot { path, kind, scope });
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
    for (base, kind) in appdata_bases(system) {
        collect_matching_dirs(
            &base,
            kind,
            names,
            MatchScope::ProgramFolders,
            roots,
            warnings,
        );
    }
}

/// Match user-authored data folders the current user keeps outside AppData:
/// `Documents` (including nested `My Games\<App>`) and `Saved Games`, resolving
/// any folder redirection (OneDrive, roaming profiles) recorded in the
/// `User Shell Folders` registry key before falling back to the profile path.
fn collect_known_user_folder_roots(
    names: &ProductNames,
    roots: &mut Vec<DiscoveryRoot>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) {
    if system.is_offline() {
        // Redirection lives in each user's NTUSER hive; the common case is
        // unredirected, so use the profile-relative folders for every user.
        for profile in system.user_profiles() {
            collect_matching_dirs(
                &profile.join(DOCUMENTS_FALLBACK),
                FootprintKind::Documents,
                names,
                MatchScope::UserFolders,
                roots,
                warnings,
            );
            collect_matching_dirs(
                &profile.join(SAVED_GAMES_FALLBACK),
                FootprintKind::SavedGames,
                names,
                MatchScope::UserFolders,
                roots,
                warnings,
            );
        }
    } else {
        if let Some(documents) = documents_dir() {
            collect_matching_dirs(
                &documents,
                FootprintKind::Documents,
                names,
                MatchScope::UserFolders,
                roots,
                warnings,
            );
        }
        if let Some(saved_games) = saved_games_dir() {
            collect_matching_dirs(
                &saved_games,
                FootprintKind::SavedGames,
                names,
                MatchScope::UserFolders,
                roots,
                warnings,
            );
        }
    }
    // Shared Public Documents: some apps keep machine-wide data here rather than
    // in ProgramData (e.g. YDIS's diagnostic database at
    // C:\Users\Public\Documents\YDIS_Data).
    if let Some(public_documents) = public_documents_dir(system) {
        collect_matching_dirs(
            &public_documents,
            FootprintKind::PublicDocuments,
            names,
            MatchScope::UserFolders,
            roots,
            warnings,
        );
    }
}

fn documents_dir() -> Option<PathBuf> {
    user_shell_folder(DOCUMENTS_VALUE).or_else(|| user_profile_join(DOCUMENTS_FALLBACK))
}

fn saved_games_dir() -> Option<PathBuf> {
    user_shell_folder(SAVED_GAMES_VALUE).or_else(|| user_profile_join(SAVED_GAMES_FALLBACK))
}

/// The shared Public Documents directory (`%PUBLIC%\Documents`, i.e.
/// `C:\Users\Public\Documents`), where some apps store machine-wide data.
fn public_documents_dir(system: &SystemContext) -> Option<PathBuf> {
    system
        .public_dir()
        .map(|base| base.join(DOCUMENTS_FALLBACK))
        .filter(|path| path.is_dir())
}

fn user_profile_join(leaf: &str) -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(|profile| PathBuf::from(profile).join(leaf))
        .filter(|path| path.is_dir())
}

/// Read a (possibly redirected) known-folder path from `User Shell Folders`,
/// expanding the `%USERPROFILE%`-style `REG_EXPAND_SZ` value.
pub(crate) fn user_shell_folder(value_name: &str) -> Option<PathBuf> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(USER_SHELL_FOLDERS_KEY)
        .ok()?;
    let raw: String = key.get_value(value_name).ok()?;
    let path = PathBuf::from(expand_env_vars(&raw));
    path.is_dir().then_some(path)
}

/// Expand `%VAR%` references using the current process environment. Unknown
/// variables are left intact.
pub(crate) fn expand_env_vars(value: &str) -> String {
    let mut output = String::new();
    let mut rest = value;
    while let Some(start) = rest.find('%') {
        output.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find('%') else {
            output.push('%');
            output.push_str(after);
            return output;
        };
        let name = &after[..end];
        match std::env::var_os(name).and_then(|value| value.into_string().ok()) {
            Some(resolved) => output.push_str(&resolved),
            None => {
                output.push('%');
                output.push_str(name);
                output.push('%');
            }
        }
        rest = &after[end + 1..];
    }
    output.push_str(rest);
    output
}

/// Find directories under `base` named for the application, at either the top level
/// (`base\App`) or one level deep (`base\Vendor\App`). Many installers nest per-vendor,
/// so checking the second level is essential to capture data folders like
/// `ProgramData\Apple Computer\iTunes`. A vendor's folder itself is never selected.
fn collect_matching_dirs(
    base: &Path,
    kind: FootprintKind,
    names: &ProductNames,
    scope: MatchScope,
    roots: &mut Vec<DiscoveryRoot>,
    warnings: &mut Vec<Warning>,
) {
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
        if !path.is_dir() || is_volatile_dir(&path, true) {
            continue;
        }
        let folder_name = file_name_string(&path);
        if names.matches_top_level(&folder_name, scope) {
            // Matched at the top level; the whole directory is walked later, so
            // there is no need to descend into it here.
            push_root(roots, path, kind.clone());
            continue;
        }
        let children = match fs::read_dir(&path) {
            Ok(children) => children,
            Err(error) => {
                if error.kind() != std::io::ErrorKind::NotFound {
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
            if child_path.is_dir()
                && !is_volatile_dir(&child_path, true)
                && names.matches_nested(&folder_name, &file_name_string(&child_path), scope)
            {
                push_root(roots, child_path, kind.clone());
            }
        }
    }
}

fn appdata_bases(system: &SystemContext) -> Vec<(PathBuf, FootprintKind)> {
    let mut bases = Vec::new();
    for profile in system.user_profiles() {
        bases.push((
            profile.join(APPDATA_ROAMING_SEGMENT),
            FootprintKind::RoamingAppData,
        ));
        bases.push((
            profile.join(APPDATA_LOCAL_SEGMENT),
            FootprintKind::LocalAppData,
        ));
        bases.push((
            profile.join(APPDATA_LOCAL_LOW_SEGMENT),
            FootprintKind::LocalLowAppData,
        ));
    }
    bases
}

#[allow(clippy::too_many_arguments)]
fn discover_tree(
    root: &DiscoveryRoot,
    install_root: Option<&Path>,
    include_hashes: bool,
    include_user_data: bool,
    include_program_data: bool,
    files: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    warnings: &mut Vec<Warning>,
) {
    let prune_volatile = holds_disposable_data(&root.kind);
    let walker = WalkDir::new(&root.path)
        .max_depth(MAX_WALK_DEPTH)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            !prune_volatile || !is_volatile_dir(entry.path(), entry.file_type().is_dir())
        });

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(Warning::new(
                    "file_discovery_walk_error",
                    WarningSeverity::Info,
                    format!("Could not read a filesystem entry: {error}"),
                    Some(root.path.display().to_string()),
                ));
                continue;
            }
        };
        if entry.file_type().is_file() {
            add_file(
                files,
                seen,
                entry.path(),
                install_root,
                root.kind.clone(),
                include_hashes,
                include_user_data,
                include_program_data,
                discovery_reason(&root.kind),
                warnings,
            );
        }
    }
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
    warnings: &mut Vec<Warning>,
) {
    let key = path.to_string_lossy().to_ascii_lowercase();
    if !seen.insert(key) {
        return;
    }

    let metadata = match fs::metadata(path) {
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

    let classification = classify_file(path, install_root, &root_kind);
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
                    "driver_file_not_copied",
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
        suggested_destination_path: None,
        size_bytes: Some(metadata.len()),
        hash_sha256,
        classification,
        should_copy,
        reason: reason.to_owned(),
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

fn is_volatile_dir(path: &Path, is_dir: bool) -> bool {
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

fn kind_for_path(path: &Path) -> FootprintKind {
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

fn is_user_profile_path(path: &Path) -> bool {
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
fn is_walkable_app_dir(path: &Path, known_base_dirs: &[PathBuf], windows_dir: &Path) -> bool {
    path.is_dir()
        && !is_drive_root(path)
        && !path_is_within(path, windows_dir)
        && !is_known_base_dir(path, known_base_dirs)
}

pub(crate) fn is_known_base_dir(path: &Path, known_base_dirs: &[PathBuf]) -> bool {
    let normalized = path.to_string_lossy().to_ascii_lowercase();
    let normalized = normalized.trim_end_matches('\\');
    known_base_dirs.iter().any(|base| {
        base.to_string_lossy()
            .to_ascii_lowercase()
            .trim_end_matches('\\')
            == normalized
    })
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
                Some(profile),
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
        assert_eq!(
            checked_user_folder(None, Some(profile), APPDATA_ROAMING_SEGMENT),
            usual
        );
        assert_eq!(
            checked_user_folder(
                Some(PathBuf::from(r"D:\Anywhere")),
                None,
                APPDATA_ROAMING_SEGMENT
            ),
            Some(PathBuf::from(r"D:\Anywhere"))
        );
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

        let Ok(Some(destination)) = user_data_real_destination(path) else {
            panic!("public base should resolve to a destination");
        };
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
    fn expands_known_env_vars_and_keeps_unknown() {
        // USERPROFILE is always present on Windows; avoid mutating the env.
        if let Some(profile) = std::env::var_os("USERPROFILE").and_then(|v| v.into_string().ok()) {
            assert_eq!(
                expand_env_vars(r"%USERPROFILE%\Documents"),
                format!(r"{profile}\Documents")
            );
        }
        assert_eq!(
            expand_env_vars(r"%WAM_DEFINITELY_MISSING%\x"),
            r"%WAM_DEFINITELY_MISSING%\x"
        );
        assert_eq!(expand_env_vars("no-vars-here"), "no-vars-here");
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

    fn walk_tree(root: &Path, kind: FootprintKind) -> Vec<String> {
        let mut files = Vec::new();
        let mut seen = BTreeSet::new();
        let mut warnings = Vec::new();
        discover_tree(
            &DiscoveryRoot {
                path: root.to_path_buf(),
                kind,
                scope: InstallScope::PerUser,
            },
            None,
            false,
            true,
            true,
            &mut files,
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
            assert!(is_known_base_dir(
                &profile.join(r"AppData\Local\Programs\"),
                &bases
            ));
        }
        for program_files in system.program_files_dirs() {
            assert!(is_known_base_dir(&program_files, &bases));
            assert!(is_known_base_dir(
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
            suggested_destination_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
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
            suggested_destination_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
        };
        let target = std::env::temp_dir();
        let destination = restore_destination_for_entry(&entry, &target, true)?;

        assert_eq!(destination, target.join(r"bin\app.exe"));
        Ok(())
    }
}
