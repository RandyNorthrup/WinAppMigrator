use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;

use crate::errors::{Result, WinAppMigratorError};
use crate::hashing::sha256_file;
use crate::models::{FileClassification, FileEntry, Warning, WarningSeverity};
use crate::path_utils::{file_name_string, normalize_name_for_match, path_has_extension};

const MAX_WALK_DEPTH: usize = 24;
const PACKAGE_FILES_ROOT: &str = "files";
const EXTERNAL_FILES_DIR: &str = "_external";
const MANIFEST_EXTENSION: &str = "manifest";
const CONFIG_EXTENSIONS: &[&str] = &[
    "cfg", "conf", "config", "ini", "json", "toml", "xml", "yaml", "yml",
];
const APPDATA_ROAMING_SEGMENT: &str = r"AppData\Roaming";
const APPDATA_LOCAL_SEGMENT: &str = r"AppData\Local";
const APPDATA_LOCAL_LOW_SEGMENT: &str = r"AppData\LocalLow";
const USER_PROFILE_NAMES_TO_SKIP: &[&str] = &["All Users", "Default", "Default User", "Public"];

#[derive(Debug, Clone)]
pub struct FileDiscoveryContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    pub app_name: Option<String>,
    pub publisher: Option<String>,
    pub include_hashes: bool,
}

pub fn discover_files(context: &FileDiscoveryContext) -> (Vec<FileEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let mut files = Vec::new();
    let mut seen = BTreeSet::new();

    add_file(
        &mut files,
        &mut seen,
        &context.executable_path,
        context.install_location.as_deref(),
        context.include_hashes,
        "Selected main executable",
        &mut warnings,
    );

    if let Some(install_location) = context.install_location.as_ref() {
        discover_tree(
            install_location,
            context.install_location.as_deref(),
            context.include_hashes,
            "File under install directory",
            &mut files,
            &mut seen,
            &mut warnings,
        );
    }

    discover_related_data_dirs(context, &mut files, &mut seen, &mut warnings);

    files.sort_by(|left, right| {
        left.original_path
            .to_string_lossy()
            .to_ascii_lowercase()
            .cmp(&right.original_path.to_string_lossy().to_ascii_lowercase())
    });

    (files, warnings)
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

fn discover_tree(
    root: &Path,
    install_root: Option<&Path>,
    include_hashes: bool,
    reason: &str,
    files: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    warnings: &mut Vec<Warning>,
) {
    for entry in WalkDir::new(root)
        .max_depth(MAX_WALK_DEPTH)
        .follow_links(false)
        .into_iter()
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(Warning::new(
                    "file_discovery_walk_error",
                    WarningSeverity::Warning,
                    format!("Could not read a filesystem entry: {error}"),
                    Some(root.display().to_string()),
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
                include_hashes,
                reason,
                warnings,
            );
        }
    }
}

fn discover_related_data_dirs(
    context: &FileDiscoveryContext,
    files: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    warnings: &mut Vec<Warning>,
) {
    let tokens = related_tokens(context);
    if tokens.is_empty() {
        return;
    }

    for root in candidate_data_roots() {
        let entries = match fs::read_dir(&root) {
            Ok(entries) => entries,
            Err(error) => {
                warnings.push(Warning::new(
                    "related_data_root_unreadable",
                    WarningSeverity::Info,
                    format!("Could not read application data root: {error}"),
                    Some(root.display().to_string()),
                ));
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = file_name_string(&path);
            let normalized = normalize_name_for_match(&name);
            if tokens.iter().any(|token| normalized.contains(token)) {
                discover_tree(
                    &path,
                    context.install_location.as_deref(),
                    context.include_hashes,
                    "Related application data directory",
                    files,
                    seen,
                    warnings,
                );
            }
        }
    }
}

fn candidate_data_roots() -> Vec<PathBuf> {
    let mut roots = BTreeSet::new();
    for env_var in ["APPDATA", "LOCALAPPDATA", "ProgramData"] {
        if let Some(path) = std::env::var_os(env_var).map(PathBuf::from) {
            roots.insert(path);
        }
    }

    if let Some(user_profile) = std::env::var_os("USERPROFILE").map(PathBuf::from)
        && let Some(users_root) = user_profile.parent()
        && let Ok(entries) = fs::read_dir(users_root)
    {
        for entry in entries.flatten() {
            let profile_path = entry.path();
            if !profile_path.is_dir() || is_skipped_user_profile(&profile_path) {
                continue;
            }
            roots.insert(profile_path.join(APPDATA_ROAMING_SEGMENT));
            roots.insert(profile_path.join(APPDATA_LOCAL_SEGMENT));
            roots.insert(profile_path.join(APPDATA_LOCAL_LOW_SEGMENT));
        }
    }

    roots.into_iter().collect()
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

fn related_tokens(context: &FileDiscoveryContext) -> Vec<String> {
    let mut tokens = Vec::new();
    for value in [
        context.app_name.as_deref(),
        context.publisher.as_deref(),
        context
            .executable_path
            .file_stem()
            .and_then(|value| value.to_str()),
    ]
    .into_iter()
    .flatten()
    {
        let token = normalize_name_for_match(value);
        if token.len() >= 3 && !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens
}

fn add_file(
    files: &mut Vec<FileEntry>,
    seen: &mut BTreeSet<String>,
    path: &Path,
    install_root: Option<&Path>,
    include_hashes: bool,
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
                WarningSeverity::Warning,
                format!("Could not read file metadata: {error}"),
                Some(path.display().to_string()),
            ));
            return;
        }
    };
    if !metadata.is_file() {
        return;
    }

    let hash_sha256 = if include_hashes {
        match sha256_file(path) {
            Ok(hash) => Some(hash),
            Err(error) => {
                warnings.push(Warning::new(
                    "file_hash_failed",
                    WarningSeverity::Warning,
                    format!("Could not hash file: {error}"),
                    Some(path.display().to_string()),
                ));
                None
            }
        }
    } else {
        None
    };

    let classification = classify_file(path, install_root);
    let should_copy = matches!(classification, FileClassification::AppOwned);
    let package_relative_path = should_copy.then(|| package_relative_path(path, install_root));

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

fn classify_file(path: &Path, install_root: Option<&Path>) -> FileClassification {
    if install_root.is_some_and(|root| path.starts_with(root)) {
        return FileClassification::AppOwned;
    }
    if is_windows_system_path(path) {
        return FileClassification::WindowsSystem;
    }
    if path_has_extension(path, MANIFEST_EXTENSION)
        || CONFIG_EXTENSIONS
            .iter()
            .any(|extension| path_has_extension(path, extension))
    {
        return FileClassification::AppOwned;
    }
    FileClassification::Unknown
}

fn is_windows_system_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.contains(r"\windows\system32\")
        || lower.contains(r"\windows\syswow64\")
        || lower.ends_with(r"\windows")
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
}
