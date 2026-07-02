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
use crate::path_utils::{file_name_string, normalize_name_for_match, path_has_extension};

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
const MIN_TOKEN_LEN: usize = 3;
/// Tokens too generic to identify an application; ignored when matching
/// directory names so a publisher like "data" never matches every `data` dir.
const GENERIC_TOKENS: &[&str] = &[
    "bin",
    "lib",
    "app",
    "apps",
    "data",
    "core",
    "src",
    "etc",
    "share",
    "usr",
    "var",
    "common",
    "resources",
    "plugins",
    "program",
    "programfiles",
    "programfilesx86",
    "programfilesx64",
    "programdata",
    "files",
    "x86",
    "x64",
    "win32",
    "win64",
    "windows",
    "microsoft",
    "inc",
    "llc",
    "ltd",
    "corp",
    "corporation",
    "software",
    "company",
    "limited",
];
const APPDATA_ROAMING_SEGMENT: &str = r"AppData\Roaming";
const APPDATA_LOCAL_SEGMENT: &str = r"AppData\Local";
const APPDATA_LOCAL_LOW_SEGMENT: &str = r"AppData\LocalLow";
const USER_PROFILE_NAMES_TO_SKIP: &[&str] = &["All Users", "Default", "Default User", "Public"];
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
}

#[derive(Debug, Clone)]
struct DiscoveryRoot {
    path: PathBuf,
    kind: FootprintKind,
    scope: InstallScope,
}

struct Tokens {
    values: Vec<String>,
}

pub fn discover_files(
    context: &FileDiscoveryContext,
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

    let tokens = Tokens::from_context(context);
    for root in collect_roots(context, &tokens) {
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
        .and_then(|root| original_path.strip_prefix(root).ok())
        .filter(|path| !path.as_os_str().is_empty())
        .map(Path::to_path_buf)
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
    let destination = package_root.join(relative);
    ensure_within_root(package_root, &destination)?;
    Ok(destination)
}

pub fn ensure_within_root(root: &Path, candidate: &Path) -> Result<()> {
    let canonical_root = canonical_or_current(root)?;
    let canonical_parent = candidate
        .parent()
        .map(canonical_or_current)
        .transpose()?
        .unwrap_or_else(|| canonical_root.clone());
    if canonical_parent.starts_with(&canonical_root) {
        Ok(())
    } else {
        Err(WinAppMigratorError::InvalidInput(format!(
            "path escapes intended root: {}",
            candidate.display()
        )))
    }
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
/// re-anchored. Guards against path traversal out of `target_root`.
pub fn restore_destination(target_root: &Path, package_relative_path: &Path) -> Result<PathBuf> {
    let relative = safe_relative_path(package_relative_path);
    let without_files = relative
        .strip_prefix(PACKAGE_FILES_ROOT)
        .unwrap_or(&relative);
    let destination = target_root.join(without_files);
    ensure_within_root(target_root, &destination)?;
    Ok(destination)
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
    let destination = base.join(&tail);
    ensure_within_root(&base, &destination)?;
    Ok(Some(destination))
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
fn user_data_base(anchor: UserDataAnchor) -> Option<PathBuf> {
    match anchor {
        UserDataAnchor::RoamingAppData => std::env::var_os("APPDATA").map(PathBuf::from),
        UserDataAnchor::LocalAppData => std::env::var_os("LOCALAPPDATA").map(PathBuf::from),
        UserDataAnchor::LocalLowAppData => std::env::var_os("USERPROFILE")
            .map(|profile| PathBuf::from(profile).join(APPDATA_LOCAL_LOW_SEGMENT)),
        UserDataAnchor::ProgramData => std::env::var_os("ProgramData").map(PathBuf::from),
        // %PUBLIC% is C:\Users\Public on every Windows install; the marker
        // includes the trailing "public\", so the tail already excludes it.
        UserDataAnchor::PublicProfile => std::env::var_os("PUBLIC")
            .map(PathBuf::from)
            .or_else(|| Some(PathBuf::from(r"C:\Users\Public"))),
    }
}

fn collect_roots(context: &FileDiscoveryContext, tokens: &Tokens) -> Vec<DiscoveryRoot> {
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

    collect_program_files_roots(tokens, &mut raw);
    collect_program_data_roots(tokens, &mut raw);
    collect_appdata_roots(tokens, &mut raw);
    collect_known_user_folder_roots(tokens, &mut raw);

    // Deduplicate while preserving order so the most authoritative root (the
    // install location) owns each shared file, and drop any root nested inside a
    // root already kept so subtrees are not walked twice.
    let mut seen_paths = BTreeSet::new();
    let mut roots: Vec<DiscoveryRoot> = Vec::new();
    for root in raw {
        if !is_walkable_app_dir(&root.path) {
            continue;
        }
        let key = root.path.to_string_lossy().to_ascii_lowercase();
        if !seen_paths.insert(key) {
            continue;
        }
        if roots
            .iter()
            .any(|existing| root.path.starts_with(&existing.path))
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

fn collect_program_files_roots(tokens: &Tokens, roots: &mut Vec<DiscoveryRoot>) {
    for env_var in ["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"] {
        if let Some(base) = std::env::var_os(env_var).map(PathBuf::from) {
            collect_matching_dirs(&base, FootprintKind::InstallDirectory, tokens, roots);
        }
    }
}

fn collect_program_data_roots(tokens: &Tokens, roots: &mut Vec<DiscoveryRoot>) {
    if let Some(base) = std::env::var_os("ProgramData").map(PathBuf::from) {
        collect_matching_dirs(&base, FootprintKind::ProgramData, tokens, roots);
    }
}

fn collect_appdata_roots(tokens: &Tokens, roots: &mut Vec<DiscoveryRoot>) {
    for (base, kind) in appdata_bases() {
        collect_matching_dirs(&base, kind, tokens, roots);
    }
}

/// Match user-authored data folders the current user keeps outside AppData:
/// `Documents` (including nested `My Games\<App>`) and `Saved Games`, resolving
/// any folder redirection (OneDrive, roaming profiles) recorded in the
/// `User Shell Folders` registry key before falling back to the profile path.
fn collect_known_user_folder_roots(tokens: &Tokens, roots: &mut Vec<DiscoveryRoot>) {
    if let Some(documents) = documents_dir() {
        collect_matching_dirs(&documents, FootprintKind::Documents, tokens, roots);
    }
    if let Some(saved_games) = saved_games_dir() {
        collect_matching_dirs(&saved_games, FootprintKind::SavedGames, tokens, roots);
    }
    // Shared Public Documents: some apps keep machine-wide data here rather than
    // in ProgramData (e.g. YDIS's diagnostic database at
    // C:\Users\Public\Documents\YDIS_Data).
    if let Some(public_documents) = public_documents_dir() {
        collect_matching_dirs(&public_documents, FootprintKind::PublicDocuments, tokens, roots);
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
fn public_documents_dir() -> Option<PathBuf> {
    std::env::var_os("PUBLIC")
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from(r"C:\Users\Public")))
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
fn user_shell_folder(value_name: &str) -> Option<PathBuf> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(USER_SHELL_FOLDERS_KEY)
        .ok()?;
    let raw: String = key.get_value(value_name).ok()?;
    let path = PathBuf::from(expand_env_vars(&raw));
    path.is_dir().then_some(path)
}

/// Expand `%VAR%` references using the current process environment. Unknown
/// variables are left intact.
fn expand_env_vars(value: &str) -> String {
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

/// Find directories under `base` whose name matches the application tokens, at
/// either the top level (`base\App`) or one level deep (`base\Vendor\App`).
/// Many installers nest per-vendor, so checking the second level is essential
/// to capture data folders like `ProgramData\Apple Computer\iTunes`.
fn collect_matching_dirs(
    base: &Path,
    kind: FootprintKind,
    tokens: &Tokens,
    roots: &mut Vec<DiscoveryRoot>,
) {
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || is_volatile_dir(&path, true) {
            continue;
        }
        if tokens.matches(&file_name_string(&path)) {
            // Matched at the top level; the whole directory is walked later, so
            // there is no need to descend into it here.
            push_root(roots, path, kind.clone());
            continue;
        }
        let Ok(children) = fs::read_dir(&path) else {
            continue;
        };
        for child in children.flatten() {
            let child_path = child.path();
            if child_path.is_dir()
                && !is_volatile_dir(&child_path, true)
                && tokens.matches(&file_name_string(&child_path))
            {
                push_root(roots, child_path, kind.clone());
            }
        }
    }
}

fn appdata_bases() -> Vec<(PathBuf, FootprintKind)> {
    let mut bases = Vec::new();
    let mut profiles = BTreeSet::new();

    if let Some(user_profile) = std::env::var_os("USERPROFILE").map(PathBuf::from) {
        profiles.insert(user_profile);
    }
    if let Some(user_profile) = std::env::var_os("USERPROFILE").map(PathBuf::from)
        && let Some(users_root) = user_profile.parent()
        && let Ok(entries) = fs::read_dir(users_root)
    {
        for entry in entries.flatten() {
            let profile_path = entry.path();
            if profile_path.is_dir() && !is_skipped_user_profile(&profile_path) {
                profiles.insert(profile_path);
            }
        }
    }

    for profile in profiles {
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

fn is_skipped_user_profile(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    USER_PROFILE_NAMES_TO_SKIP
        .iter()
        .any(|skipped| name.eq_ignore_ascii_case(skipped))
}

impl Tokens {
    fn from_context(context: &FileDiscoveryContext) -> Self {
        let mut values = Vec::new();
        let mut candidates: Vec<String> = Vec::new();
        if let Some(name) = context.app_name.as_deref() {
            candidates.push(primary_name(name));
        }
        if let Some(publisher) = context.publisher.as_deref() {
            candidates.push(publisher.to_owned());
        }
        if let Some(stem) = context.executable_path.file_stem().and_then(|s| s.to_str()) {
            candidates.push(stem.to_owned());
        }
        if let Some(install) = context.install_location.as_ref() {
            if let Some(leaf) = install.file_name().and_then(|s| s.to_str()) {
                candidates.push(leaf.to_owned());
            }
            if let Some(vendor) = install
                .parent()
                .and_then(Path::file_name)
                .and_then(|s| s.to_str())
            {
                candidates.push(vendor.to_owned());
            }
        }

        for candidate in candidates {
            let token = normalize_name_for_match(&candidate);
            if token.len() >= MIN_TOKEN_LEN
                && !GENERIC_TOKENS.contains(&token.as_str())
                && !values.contains(&token)
            {
                values.push(token);
            }
        }
        Self { values }
    }

    fn matches(&self, dir_name: &str) -> bool {
        let normalized = normalize_name_for_match(dir_name);
        if normalized.len() < MIN_TOKEN_LEN || GENERIC_TOKENS.contains(&normalized.as_str()) {
            return false;
        }
        self.values.iter().any(|token| {
            // Exact match, or the directory name embeds a sufficiently specific
            // token (e.g. "Mozilla Firefox" contains "firefox"), or the token
            // embeds a longer directory name (a vendor folder shorter than the
            // product token). The length floors avoid generic fragments such as
            // "x64" or "bin" producing spurious matches.
            *token == normalized
                || (token.len() >= 4 && normalized.contains(token.as_str()))
                || (normalized.len() >= 5
                    && token.len() > normalized.len()
                    && token.contains(normalized.as_str()))
        })
    }
}

/// Reduce a Display Name to its core product name by dropping trailing version
/// numbers and parenthetical qualifiers (architecture, edition, etc.).
fn primary_name(display_name: &str) -> String {
    let base = display_name.split('(').next().unwrap_or(display_name);
    base.split_whitespace()
        .filter(|word| {
            !word
                .chars()
                .all(|character| character.is_ascii_digit() || matches!(character, '.' | ',' | '-'))
        })
        .collect::<Vec<_>>()
        .join(" ")
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
    let walker = WalkDir::new(&root.path)
        .max_depth(MAX_WALK_DEPTH)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| !is_volatile_dir(entry.path(), entry.file_type().is_dir()));

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
    // An online-only cloud placeholder has no local data; copying it would
    // hydrate or fail, so it is reported but never copied.
    let should_copy = base_should_copy && !online_only;

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
    if install_root.is_some_and(|root| path.starts_with(root)) {
        return FileClassification::AppOwned;
    }
    match root_kind {
        FootprintKind::InstallDirectory => FileClassification::AppOwned,
        FootprintKind::RegistryReference | FootprintKind::Other => {
            if is_program_files_path(path) {
                FileClassification::AppOwned
            } else if is_program_data_path(path) {
                FileClassification::MachineAppData
            } else {
                FileClassification::UserData
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

/// True when `path` is a directory we can safely walk: it exists, is not a
/// Windows system tree, and is a subdirectory of (not equal to) a known base
/// such as `Program Files` or a user profile.
fn is_walkable_app_dir(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    let lower = path.to_string_lossy().to_ascii_lowercase();
    if lower.contains(r"\windows\") || lower.ends_with(r"\windows") {
        return false;
    }
    let component_count = path
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count();
    if component_count < 2 {
        return false;
    }
    !is_known_base_dir(path)
}

fn is_known_base_dir(path: &Path) -> bool {
    let normalized = path.to_string_lossy().to_ascii_lowercase();
    let normalized = normalized.trim_end_matches('\\');
    known_base_dirs().iter().any(|base| {
        base.to_string_lossy()
            .to_ascii_lowercase()
            .trim_end_matches('\\')
            == normalized
    })
}

fn known_base_dirs() -> Vec<PathBuf> {
    let mut bases = Vec::new();
    for env_var in [
        "ProgramFiles",
        "ProgramFiles(x86)",
        "ProgramW6432",
        "ProgramData",
    ] {
        if let Some(path) = std::env::var_os(env_var).map(PathBuf::from) {
            bases.push(path);
        }
    }
    for (base, _) in appdata_bases() {
        bases.push(base);
    }
    if let Some(user_profile) = std::env::var_os("USERPROFILE").map(PathBuf::from) {
        bases.push(user_profile.clone());
        if let Some(users_root) = user_profile.parent() {
            bases.push(users_root.to_path_buf());
        }
    }
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
    let trimmed = output.trim().trim_end_matches('.').to_owned();
    if trimmed.is_empty() {
        "_".to_owned()
    } else {
        trimmed
    }
}

fn canonical_or_current(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        path.canonicalize()
            .map_err(|source| WinAppMigratorError::io(path, source))
    } else if let Some(parent) = path.parent() {
        canonical_or_current(parent)
    } else {
        Ok(path.to_path_buf())
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

    #[test]
    fn matches_nested_vendor_app_directories() {
        let base = std::env::temp_dir().join(format!(
            "wam-discovery-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));
        let nested = base.join("Apple Computer").join("iTunes");
        let top_level = base.join("AnyDesk");
        let _ = fs::create_dir_all(&nested);
        let _ = fs::create_dir_all(&top_level);

        let context = FileDiscoveryContext {
            executable_path: PathBuf::from(r"C:\Program Files\iTunes\iTunes.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\iTunes")),
            app_name: Some("iTunes".to_owned()),
            publisher: Some("Apple Inc.".to_owned()),
            include_hashes: false,
            include_user_data: true,
            include_program_data: true,
            extra_roots: Vec::new(),
        };
        let tokens = Tokens::from_context(&context);
        let mut roots = Vec::new();
        collect_matching_dirs(&base, FootprintKind::ProgramData, &tokens, &mut roots);

        let _ = fs::remove_dir_all(&base);
        assert!(
            roots.iter().any(|root| root.path == nested),
            "nested vendor/app directory should be matched at depth two"
        );
    }

    #[test]
    fn rejects_generic_and_cross_app_matches() {
        let context = FileDiscoveryContext {
            executable_path: PathBuf::from(r"C:\Program Files\7-Zip\7zFM.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\7-Zip")),
            app_name: Some("7-Zip 25.01 (x64)".to_owned()),
            publisher: Some("Igor Pavlov".to_owned()),
            include_hashes: false,
            include_user_data: true,
            include_program_data: true,
            extra_roots: Vec::new(),
        };
        let tokens = Tokens::from_context(&context);
        assert!(
            tokens.matches("7-Zip"),
            "the product directory should match"
        );
        assert!(
            !tokens.matches("x64"),
            "architecture fragment must not match"
        );
        assert!(
            !tokens.matches("TeamViewer"),
            "unrelated vendor must not match"
        );
        assert!(!tokens.matches("files"), "generic directory must not match");

        let anydesk = FileDiscoveryContext {
            executable_path: PathBuf::from(r"C:\Program Files (x86)\AnyDesk\AnyDesk.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files (x86)\AnyDesk")),
            app_name: Some("AnyDesk".to_owned()),
            publisher: None,
            include_hashes: false,
            include_user_data: true,
            include_program_data: true,
            extra_roots: Vec::new(),
        };
        let anydesk_tokens = Tokens::from_context(&anydesk);
        assert!(anydesk_tokens.matches("AnyDesk"));
        assert!(!anydesk_tokens.matches("files"));
        assert!(!anydesk_tokens.matches("3uTools9"));
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

    #[test]
    fn tokens_match_vendor_directories() {
        let context = FileDiscoveryContext {
            executable_path: PathBuf::from(r"C:\Program Files\Mozilla Firefox\firefox.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Mozilla Firefox")),
            app_name: Some("Mozilla Firefox".to_owned()),
            publisher: Some("Mozilla".to_owned()),
            include_hashes: false,
            include_user_data: true,
            include_program_data: true,
            extra_roots: Vec::new(),
        };
        let tokens = Tokens::from_context(&context);

        assert!(tokens.matches("Mozilla"));
        assert!(tokens.matches("Mozilla Firefox"));
        assert!(!tokens.matches("Google"));
    }
}
