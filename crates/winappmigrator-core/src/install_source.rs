//! Authoritative install-source reverse engineering.
//!
//! Rather than guessing an application's footprint by scanning folders, this
//! module first determines *how* the application was installed and then, where
//! Windows records it, reads the authoritative list of installed paths.
//!
//! For Windows Installer (MSI) products the installer keeps a component
//! database under
//! `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\<SID>\Components`.
//! Each component subkey is named with a "squished" (packed) component GUID and
//! holds one value per consuming product (also a squished GUID) whose data is
//! the component's *key path* — the concrete file or registry key the installer
//! placed on disk. Reading those values for a product code yields the exact set
//! of files an MSI installed, which we collapse into a minimal set of install
//! roots.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::models::{InstallMethod, InstalledApp};
use crate::path_utils::split_command_executable_path;
use crate::system_context::SystemContext;

const USERDATA_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData";
const APP_PATHS_PREFIX: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths";
const SIGNATURE_SCAN_BYTES: u64 = 2_000_000;
const SQUIRREL_APP_PREFIX: &str = "app-";
const APPX_MANIFEST: &str = "AppxManifest.xml";
const MSIX_PACKAGES_DIR: &str = "Packages";

/// The result of reverse-engineering how and where an application was installed.
#[derive(Debug, Clone)]
pub struct InstallSource {
    pub method: InstallMethod,
    pub product_code: Option<String>,
    pub authoritative_file_count: usize,
    pub authoritative_registry_count: usize,
    /// Install directories derived from authoritative or registered sources.
    pub roots: Vec<PathBuf>,
    /// Human-readable signals used to reverse-engineer the install.
    pub evidence: Vec<String>,
}

/// Authoritative key paths recorded for a product in the MSI component database.
struct MsiComponentPaths {
    files: Vec<PathBuf>,
    registry_count: usize,
}

pub fn detect_install_source(app: &InstalledApp, system: &SystemContext) -> InstallSource {
    let product_code = extract_product_code(app);
    let method = classify_method(app, product_code.as_deref());

    let mut roots = Vec::new();
    let mut evidence = Vec::new();
    let mut authoritative_file_count = 0;
    let mut authoritative_registry_count = 0;

    match method {
        InstallMethod::WindowsInstaller => {
            if let Some(code) = product_code.as_deref()
                && let Some(paths) = msi_component_paths(code, system)
            {
                authoritative_file_count = paths.files.len();
                authoritative_registry_count = paths.registry_count;
                roots.extend(minimal_install_roots(&paths.files));
                evidence.push(format!(
                    "Windows Installer database: {} component key paths",
                    authoritative_file_count + authoritative_registry_count
                ));
            }
        }
        InstallMethod::InnoSetup => {
            if let Some(dir) = primary_install_dir(app)
                && let Some(log) = find_inno_log(&dir)
            {
                let files = inno_manifest_files(&log, system);
                if files.is_empty() {
                    evidence.push("Inno Setup uninstall log (unins000.dat)".to_owned());
                } else {
                    authoritative_file_count = files.len();
                    let footprint: Vec<PathBuf> = files
                        .into_iter()
                        .filter(|path| is_app_footprint_path(path))
                        .collect();
                    roots.extend(minimal_install_roots(&footprint));
                    evidence.push(format!(
                        "Inno Setup uninstall log: {authoritative_file_count} recorded paths"
                    ));
                }
            }
        }
        InstallMethod::Nsis => {
            evidence.push("NSIS (Nullsoft) uninstaller signature".to_owned());
            // NSIS keeps no manifest by default, but the AdvUninstLog macro
            // writes an `uninstall.log` of installed paths in the install dir.
            if let Some(dir) = primary_install_dir(app)
                && let Some(log) = find_nsis_log(&dir)
            {
                let files = nsis_manifest_files(&log, system);
                if !files.is_empty() {
                    authoritative_file_count = files.len();
                    let footprint: Vec<PathBuf> = files
                        .into_iter()
                        .filter(|path| is_app_footprint_path(path))
                        .collect();
                    roots.extend(minimal_install_roots(&footprint));
                    evidence.push(format!(
                        "NSIS AdvUninstLog uninstall.log: {authoritative_file_count} recorded paths"
                    ));
                }
            }
        }
        InstallMethod::Squirrel => {
            evidence.push("Squirrel per-user layout (Update.exe / app-*)".to_owned());
            if let Some(dir) = primary_install_dir(app) {
                roots.push(dir.clone());
                if let Some(app_dir) = latest_squirrel_app_dir(&dir) {
                    if let Some(version) = app_dir.file_name().and_then(|name| name.to_str()) {
                        evidence.push(format!("Squirrel current version: {version}"));
                    }
                    roots.push(app_dir);
                }
            }
        }
        InstallMethod::Chocolatey => {
            evidence.push("Chocolatey package metadata".to_owned());
        }
        InstallMethod::Msix => {
            evidence.push("MSIX / Appx package".to_owned());
            // The install payload under WindowsApps is authoritative, but the
            // app's mutable per-user state lives at
            // %LOCALAPPDATA%\Packages\<PackageFamilyName>, whose folder name is
            // the package identity (not the display name) and is therefore
            // missed by name-token heuristics. Read the manifest identity to
            // anchor it directly.
            if let Some(dir) = primary_install_dir(app)
                && let Some(identity) = appx_identity_name(&dir)
            {
                evidence.push(format!("AppxManifest identity: {identity}"));
                let package_roots = msix_package_data_roots(&identity, system);
                if !package_roots.is_empty() {
                    evidence.push(format!(
                        "MSIX per-user package data: {} location(s)",
                        package_roots.len()
                    ));
                    roots.extend(package_roots);
                }
            }
        }
        InstallMethod::Generic => {}
    }

    // App Paths registration is an authoritative, documented source of the
    // launcher location and any extra DLL search directories.
    if let Some(executable) = app.main_executable.as_ref()
        && let Some(name) = executable.file_name().and_then(|value| value.to_str())
    {
        let app_paths = app_paths_roots(name, system);
        if !app_paths.is_empty() {
            evidence.push("App Paths registration".to_owned());
            roots.extend(app_paths);
        }
    }

    // The uninstaller almost always lives in the install tree, so its directory
    // is a reliable anchor even when InstallLocation is absent.
    if let Some(uninstall) = app.uninstall_string.as_deref()
        && let Some(executable) = split_command_executable_path(uninstall)
        && let Some(parent) = executable.parent()
    {
        roots.push(system.resolve_source_path(parent));
    }

    InstallSource {
        method,
        product_code,
        authoritative_file_count,
        authoritative_registry_count,
        roots: dedup_existing_dirs(roots),
        evidence,
    }
}

/// Best-effort resolution of an application's main executable when the uninstall
/// registry entry recorded none. Many MSI products omit both `InstallLocation`
/// and a usable `DisplayIcon`, so the inventory-time heuristic finds nothing.
/// Here we consult the authoritative install source — the Windows Installer
/// component database for MSI products, otherwise the resolved install roots —
/// and pick a representative non-auxiliary executable. Used as a fallback so a
/// scan or package by application name still succeeds for such apps.
pub fn primary_executable(app: &InstalledApp, system: &SystemContext) -> Option<PathBuf> {
    let source = detect_install_source(app, system);
    let mut candidates: Vec<PathBuf> = Vec::new();

    if source.method == InstallMethod::WindowsInstaller
        && let Some(code) = source.product_code.as_deref()
        && let Some(paths) = msi_component_paths(code, system)
    {
        candidates.extend(
            paths
                .files
                .into_iter()
                .filter(|path| is_executable_path(path)),
        );
    }

    if candidates.is_empty() {
        for dir in &source.roots {
            collect_dir_executables(dir, &mut candidates);
        }
    }

    pick_primary_executable(candidates, &app.display_name)
}

fn is_executable_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("exe"))
        .unwrap_or(false)
        && path.is_file()
}

fn collect_dir_executables(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_executable_path(&path) {
            out.push(path);
        }
    }
}

/// Pick the most representative main executable from a candidate set: drop
/// uninstallers/helpers, prefer one whose file name matches the display name,
/// and otherwise fall back to the largest remaining executable.
fn pick_primary_executable(candidates: Vec<PathBuf>, display_name: &str) -> Option<PathBuf> {
    let mut filtered: Vec<PathBuf> = candidates
        .into_iter()
        .filter(|path| !is_auxiliary_executable_name(path))
        .collect();
    filtered.sort();
    filtered.dedup();
    if filtered.is_empty() {
        return None;
    }

    let normalized_display = normalize_for_match(display_name);
    if !normalized_display.is_empty()
        && let Some(matched) = filtered.iter().find(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(normalize_for_match)
                .map(|stem| {
                    !stem.is_empty()
                        && (normalized_display.contains(&stem)
                            || stem.contains(&normalized_display))
                })
                .unwrap_or(false)
        })
    {
        return Some(matched.clone());
    }

    filtered
        .into_iter()
        .max_by_key(|path| std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0))
}

fn is_auxiliary_executable_name(path: &Path) -> bool {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    const AUXILIARY_MARKERS: &[&str] = &[
        "unins",
        "uninstall",
        "setup",
        "install",
        "update",
        "updater",
        "crashpad",
        "crashreporter",
        "vcredist",
        "vc_redist",
        "dotnet",
        "helper",
    ];
    AUXILIARY_MARKERS.iter().any(|marker| stem.contains(marker))
}

fn normalize_for_match(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn classify_method(app: &InstalledApp, product_code: Option<&str>) -> InstallMethod {
    let location = app
        .install_location
        .as_ref()
        .map(|path| path.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    let uninstall = app
        .uninstall_string
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();

    if location.contains(r"\windowsapps\") {
        return InstallMethod::Msix;
    }
    if location.contains(r"\chocolatey\lib\") || uninstall.contains(r"\chocolatey\") {
        return InstallMethod::Chocolatey;
    }
    if location.contains(r"\appdata\local\") && is_squirrel_layout(app.install_location.as_deref())
    {
        return InstallMethod::Squirrel;
    }
    if product_code.is_some() || uninstall.contains("msiexec") {
        return InstallMethod::WindowsInstaller;
    }

    // EXE installers: identify the toolkit from concrete on-disk evidence.
    let primary_dir = primary_install_dir(app);
    if let Some(dir) = primary_dir.as_deref() {
        if find_inno_log(dir).is_some() {
            return InstallMethod::InnoSetup;
        }
        if is_nsis_install(dir, app.uninstall_string.as_deref()) {
            return InstallMethod::Nsis;
        }
    }
    InstallMethod::Generic
}

fn primary_install_dir(app: &InstalledApp) -> Option<PathBuf> {
    if let Some(location) = app.install_location.as_ref()
        && location.is_dir()
    {
        return Some(location.clone());
    }
    app.main_executable
        .as_ref()
        .and_then(|executable| executable.parent())
        .map(Path::to_path_buf)
}

/// Inno Setup records every installed file and registry key in an uninstall log
/// named `unins000.dat` in the install directory; locate it.
fn find_inno_log(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if is_inno_log_name(&name) {
            return Some(entry.path());
        }
    }
    None
}

/// True for an Inno uninstall log name such as `unins000.dat` (three digits).
/// Deliberately does not match a plain `uninstall.exe`, which other installers
/// also use.
fn is_inno_log_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    name.len() == 12
        && name.starts_with("unins")
        && name.ends_with(".dat")
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
        && bytes[7].is_ascii_digit()
}

/// Extract the authoritative list of installed file paths from an Inno Setup
/// uninstall log. The log stores each installed path as a resolved absolute
/// string (UTF-16 in modern logs, ANSI in older ones); scanning those strings
/// is version-independent and avoids decoding Inno's record framing.
fn inno_manifest_files(log_path: &Path, system: &SystemContext) -> Vec<PathBuf> {
    let Ok(data) = std::fs::read(log_path) else {
        return Vec::new();
    };
    if !is_inno_log(&data) {
        return Vec::new();
    }
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    for candidate in extract_utf16_paths(&data)
        .into_iter()
        .chain(extract_ansi_paths(&data))
    {
        let path = system.resolve_source_path(&PathBuf::from(&candidate));
        if path.is_file() && seen.insert(candidate.to_ascii_lowercase()) {
            files.push(path);
        }
    }
    files
}

fn is_inno_log(data: &[u8]) -> bool {
    let head = &data[..64.min(data.len())];
    String::from_utf8_lossy(head).contains("Inno Setup Uninstall Log")
}

/// Locate an NSIS AdvUninstLog `uninstall.log` in the install directory.
fn find_nsis_log(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if is_nsis_log_name(&name) {
            return Some(entry.path());
        }
    }
    None
}

/// The default file the NSIS AdvUninstLog macro writes (`uninstall.log`).
fn is_nsis_log_name(name: &str) -> bool {
    name == "uninstall.log"
}

/// Extract installed file paths from an NSIS AdvUninstLog `uninstall.log`.
/// The log has no documented stable framing, so (like the Inno reader) this
/// scans for resolved absolute path strings and keeps the ones that exist,
/// which is robust across AdvUninstLog versions and UTF-16/ANSI encodings.
fn nsis_manifest_files(log_path: &Path, system: &SystemContext) -> Vec<PathBuf> {
    let Ok(data) = std::fs::read(log_path) else {
        return Vec::new();
    };
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    for candidate in extract_utf16_paths(&data)
        .into_iter()
        .chain(extract_ansi_paths(&data))
    {
        let path = system.resolve_source_path(&PathBuf::from(&candidate));
        if path.is_file() && seen.insert(candidate.to_ascii_lowercase()) {
            files.push(path);
        }
    }
    files
}

fn extract_utf16_paths(data: &[u8]) -> Vec<String> {
    let mut paths = Vec::new();
    let mut index = 0;
    while index + 6 <= data.len() {
        if data[index].is_ascii_alphabetic()
            && data[index + 1] == 0
            && data[index + 2] == b':'
            && data[index + 3] == 0
            && data[index + 4] == b'\\'
            && data[index + 5] == 0
        {
            let mut units = Vec::new();
            let mut cursor = index;
            while cursor + 2 <= data.len() {
                let unit = u16::from_le_bytes([data[cursor], data[cursor + 1]]);
                if unit == 0 {
                    break;
                }
                units.push(unit);
                cursor += 2;
            }
            let decoded: String = char::decode_utf16(units)
                .map(|result| result.unwrap_or(char::REPLACEMENT_CHARACTER))
                .take_while(|character| is_path_char(*character))
                .collect();
            push_path(&mut paths, decoded);
            index = cursor.max(index + 2);
        } else {
            index += 1;
        }
    }
    paths
}

fn extract_ansi_paths(data: &[u8]) -> Vec<String> {
    let mut paths = Vec::new();
    let mut index = 0;
    while index + 3 <= data.len() {
        if data[index].is_ascii_alphabetic() && data[index + 1] == b':' && data[index + 2] == b'\\'
        {
            let mut text = String::new();
            let mut cursor = index;
            while cursor < data.len() {
                let character = data[cursor] as char;
                if data[cursor] >= 0x20 && is_path_char(character) {
                    text.push(character);
                    cursor += 1;
                } else {
                    break;
                }
            }
            push_path(&mut paths, text);
            index = cursor.max(index + 1);
        } else {
            index += 1;
        }
    }
    paths
}

fn push_path(paths: &mut Vec<String>, candidate: String) {
    let trimmed = candidate.trim_end().to_owned();
    if trimmed.len() >= 4 {
        paths.push(trimmed);
    }
}

fn is_path_char(character: char) -> bool {
    character.is_alphanumeric()
        || character == ' '
        || r"\:./-_()[]{}+,'~!@#$%^&=;`".contains(character)
}

/// Exclude shortcut, Start Menu, Desktop and system paths so install-manifest
/// entries do not turn an entire shared system folder into a footprint root.
fn is_app_footprint_path(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    !(lower.ends_with(".lnk")
        || lower.contains(r"\start menu\")
        || lower.contains(r"\desktop\")
        || lower.contains(r"\microsoft\windows\")
        || lower.contains(r"\windows\"))
}

/// NSIS embeds the "Nullsoft.NSIS" / "Nullsoft Install System" marker in its
/// (un)installer; detect it from the uninstaller binary.
fn is_nsis_install(dir: &Path, uninstall_string: Option<&str>) -> bool {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(uninstall) = uninstall_string
        && let Some(executable) = split_command_executable_path(uninstall)
    {
        candidates.push(executable);
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if name.starts_with("uninst") && name.ends_with(".exe") {
                candidates.push(entry.path());
            }
        }
    }
    candidates
        .iter()
        .any(|candidate| file_contains_signature(candidate, b"Nullsoft"))
}

fn file_contains_signature(path: &Path, signature: &[u8]) -> bool {
    let Ok(file) = std::fs::File::open(path) else {
        return false;
    };
    let mut buffer = Vec::new();
    if file
        .take(SIGNATURE_SCAN_BYTES)
        .read_to_end(&mut buffer)
        .is_err()
    {
        return false;
    }
    buffer
        .windows(signature.len())
        .any(|window| window == signature)
}

fn app_paths_roots(executable_name: &str, system: &SystemContext) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let subkey = format!("{APP_PATHS_PREFIX}\\{executable_name}");
    for flags in [KEY_READ | KEY_WOW64_64KEY, KEY_READ | KEY_WOW64_32KEY] {
        let Ok(key) = system.open_hklm(&subkey, flags) else {
            continue;
        };
        if let Ok(default) = key.get_value::<String, _>("")
            && let Some(parent) = PathBuf::from(default.trim_matches('"')).parent()
        {
            roots.push(system.resolve_source_path(parent));
        }
        if let Ok(path_value) = key.get_value::<String, _>("Path") {
            for segment in path_value.split(';') {
                let trimmed = segment.trim().trim_matches('"');
                if !trimmed.is_empty() {
                    roots.push(system.resolve_source_path(&PathBuf::from(trimmed)));
                }
            }
        }
    }
    roots
}

fn dedup_existing_dirs(roots: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut output = Vec::new();
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        if seen.insert(root.to_string_lossy().to_ascii_lowercase()) {
            output.push(root);
        }
    }
    output
}

fn is_squirrel_layout(install_location: Option<&Path>) -> bool {
    let Some(location) = install_location else {
        return false;
    };
    if location.join("Update.exe").is_file() {
        return true;
    }
    let Ok(entries) = std::fs::read_dir(location) else {
        return false;
    };
    entries
        .flatten()
        .any(|entry| entry.path().is_dir() && is_squirrel_app_dir(&entry.path()))
}

fn is_squirrel_app_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.to_ascii_lowercase().starts_with(SQUIRREL_APP_PREFIX))
}

/// The current Squirrel `app-<version>` directory (highest version) in an
/// install location, if any. Versions are compared by their dotted numeric
/// segments so `app-1.0.10` ranks above `app-1.0.9`.
fn latest_squirrel_app_dir(install_dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(install_dir).ok()?;
    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && is_squirrel_app_dir(path))
        .max_by(|left, right| squirrel_version_key(left).cmp(&squirrel_version_key(right)))
}

fn squirrel_version_key(app_dir: &Path) -> Vec<u64> {
    app_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            name.trim_start_matches(|character: char| !character.is_ascii_digit())
                .split('.')
                .map(|segment| segment.parse::<u64>().unwrap_or(0))
                .collect()
        })
        .unwrap_or_default()
}

/// Read the `Identity` element `Name` from a package's `AppxManifest.xml`
/// (for example `Microsoft.WindowsTerminal`). A targeted attribute scan avoids
/// pulling in an XML dependency for a single well-known element.
fn appx_identity_name(install_dir: &Path) -> Option<String> {
    let data = std::fs::read(install_dir.join(APPX_MANIFEST)).ok()?;
    let text = String::from_utf8_lossy(&data);
    let identity = &text[text.find("<Identity")?..];
    let after_name = &identity[identity.find("Name=\"")? + "Name=\"".len()..];
    let name = after_name[..after_name.find('"')?].trim();
    (!name.is_empty()).then(|| name.to_owned())
}

/// Per-user MSIX package-data directories for an identity name. The folder is
/// named `<IdentityName>_<publisherHash>`, so a prefix match avoids computing
/// the publisher hash. Scoped to the current user's `%LOCALAPPDATA%\Packages`.
fn msix_package_data_roots(identity_name: &str, system: &SystemContext) -> Vec<PathBuf> {
    let prefix = format!("{}_", identity_name.to_ascii_lowercase());
    let mut roots = Vec::new();
    for local_app_data in local_app_data_dirs(system) {
        let Ok(entries) = std::fs::read_dir(local_app_data.join(MSIX_PACKAGES_DIR)) else {
            continue;
        };
        roots.extend(entries.flatten().map(|entry| entry.path()).filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
        }));
    }
    roots
}

/// `%LOCALAPPDATA%` bases to search. Live: the current user; offline: each
/// profile's `AppData\Local` under the scanned volume.
fn local_app_data_dirs(system: &SystemContext) -> Vec<PathBuf> {
    if system.is_offline() {
        system
            .user_profiles()
            .into_iter()
            .map(|profile| profile.join(r"AppData\Local"))
            .collect()
    } else {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .into_iter()
            .collect()
    }
}

fn extract_product_code(app: &InstalledApp) -> Option<String> {
    if let Some(last) = app.registry_source_path.rsplit(['\\', '/']).next()
        && is_guid(last)
    {
        return Some(last.to_ascii_uppercase());
    }
    if let Some(uninstall) = app.uninstall_string.as_deref() {
        return find_guid(uninstall);
    }
    None
}

fn is_guid(value: &str) -> bool {
    let value = value.trim();
    value.len() == 38
        && value.starts_with('{')
        && value.ends_with('}')
        && value[1..value.len() - 1]
            .chars()
            .all(|character| character.is_ascii_hexdigit() || character == '-')
}

fn find_guid(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'{' && index + 38 <= value.len() && is_guid(&value[index..index + 38]) {
            return Some(value[index..index + 38].to_ascii_uppercase());
        }
    }
    None
}

/// Convert a product/component GUID into the "squished" (packed) form used as
/// registry value and key names in the Installer database.
pub fn squish_guid(guid: &str) -> Option<String> {
    let hex: Vec<char> = guid
        .chars()
        .filter(|character| character.is_ascii_hexdigit())
        .collect();
    if hex.len() != 32 {
        return None;
    }
    let mut squished = String::with_capacity(32);
    squished.extend(hex[0..8].iter().rev());
    squished.extend(hex[8..12].iter().rev());
    squished.extend(hex[12..16].iter().rev());
    for pair in hex[16..32].chunks(2) {
        squished.push(pair[1]);
        squished.push(pair[0]);
    }
    Some(squished.to_ascii_uppercase())
}

fn msi_component_paths(product_code: &str, system: &SystemContext) -> Option<MsiComponentPaths> {
    let squished_product = squish_guid(product_code)?;
    let user_data = system
        .open_hklm(USERDATA_PATH, KEY_READ | KEY_WOW64_64KEY)
        .ok()?;

    let mut files = Vec::new();
    let mut registry_count = 0;
    let mut seen = BTreeSet::new();

    for sid in component_sweep_sids(&user_data, &squished_product) {
        let Ok(sid_key) = user_data.open_subkey_with_flags(&sid, KEY_READ | KEY_WOW64_64KEY) else {
            continue;
        };
        let Ok(components) =
            sid_key.open_subkey_with_flags("Components", KEY_READ | KEY_WOW64_64KEY)
        else {
            continue;
        };
        for component in components.enum_keys().flatten() {
            let Ok(component_key) =
                components.open_subkey_with_flags(&component, KEY_READ | KEY_WOW64_64KEY)
            else {
                continue;
            };
            let Ok(key_path) = component_key.get_value::<String, _>(&squished_product) else {
                continue;
            };
            if key_path.is_empty() {
                continue;
            }
            if is_registry_key_path(&key_path) {
                registry_count += 1;
            } else if seen.insert(key_path.to_ascii_lowercase()) {
                // MSI records absolute source-machine paths; re-root onto the
                // scanned volume so downstream reads hit the right disk.
                files.push(system.resolve_source_path(&PathBuf::from(key_path)));
            }
        }
    }

    if files.is_empty() && registry_count == 0 {
        return None;
    }
    Some(MsiComponentPaths {
        files,
        registry_count,
    })
}

/// The SIDs whose `Components` subtree to sweep for a product's key paths.
///
/// A component only records a product's key path under the SID where that
/// product is registered (`UserData\<SID>\Products\<sqProduct>`), so scanning
/// only those SIDs yields identical results while skipping unrelated per-user
/// component trees on multi-user machines. This is a safe narrowing, not a
/// `Features`-based component scoping: it never decodes feature descriptors, so
/// it cannot under-report. If no SID is found to register the product (an
/// unexpected registry state), every SID is swept as a fallback so results are
/// never worse than the exhaustive scan.
fn component_sweep_sids(user_data: &RegKey, squished_product: &str) -> Vec<String> {
    let all_sids: Vec<String> = user_data.enum_keys().flatten().collect();
    let registering: Vec<String> = all_sids
        .iter()
        .filter(|sid| {
            user_data
                .open_subkey_with_flags(
                    format!(r"{sid}\Products\{squished_product}"),
                    KEY_READ | KEY_WOW64_64KEY,
                )
                .is_ok()
        })
        .cloned()
        .collect();
    if registering.is_empty() {
        all_sids
    } else {
        registering
    }
}

/// MSI encodes registry key paths as `<root>:<path>` (for example `02:\Software\…`).
fn is_registry_key_path(value: &str) -> bool {
    let mut chars = value.chars();
    let first = chars.next();
    match first {
        Some(c) if c.is_ascii_digit() => {
            // One or two leading digits then a colon.
            matches!(chars.next(), Some(':'))
                || matches!((chars.next(), chars.next()), (Some(':'), _))
        }
        _ => false,
    }
}

/// Markers that identify a well-known base directory; the application's install
/// root is the first path segment that follows one of these. Ordered
/// most-specific first: the shared `Common Files` directories must be matched
/// before the plain `Program Files` markers so a vendor's files under
/// `Common Files\<Vendor>` resolve to that vendor sub-directory instead of
/// collapsing the entire (multi-vendor) `Common Files` tree into one root.
const ROOT_MARKERS: &[&str] = &[
    r"\program files (x86)\common files\",
    r"\program files\common files\",
    r"\program files (x86)\",
    r"\program files\",
    r"\programdata\",
    r"\appdata\roaming\",
    r"\appdata\locallow\",
    r"\appdata\local\",
];

/// Collapse a set of installed file paths into the distinct top-level install
/// roots that contain them (for example every file under
/// `C:\Program Files\Inkscape\…` resolves to `C:\Program Files\Inkscape`).
fn minimal_install_roots(files: &[PathBuf]) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut roots = Vec::new();
    for file in files {
        let root = install_root_for_file(file);
        if seen.insert(root.to_string_lossy().to_ascii_lowercase()) {
            roots.push(root);
        }
    }
    roots
}

fn install_root_for_file(file: &Path) -> PathBuf {
    let display = file.to_string_lossy();
    let lower = display.to_ascii_lowercase();
    for marker in ROOT_MARKERS {
        if let Some(position) = lower.find(marker) {
            let after = position + marker.len();
            let segment_end = display[after..]
                .find('\\')
                .map(|index| after + index)
                .unwrap_or(display.len());
            return PathBuf::from(&display[..segment_end]);
        }
    }
    file.parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| file.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squishes_known_product_guid() {
        let squished = squish_guid("{C653D4A8-392F-476F-A1B5-C64F217938D7}");
        assert_eq!(
            squished.as_deref(),
            Some("8A4D356CF293F6741A5B6CF41297837D")
        );
    }

    #[test]
    fn recognizes_guid_strings() {
        assert!(is_guid("{C653D4A8-392F-476F-A1B5-C64F217938D7}"));
        assert!(!is_guid("Inkscape"));
        assert!(!is_guid("{too-short}"));
    }

    #[test]
    fn finds_guid_in_uninstall_string() {
        let code = find_guid(r"MsiExec.exe /X{C653D4A8-392F-476F-A1B5-C64F217938D7}");
        assert_eq!(
            code.as_deref(),
            Some("{C653D4A8-392F-476F-A1B5-C64F217938D7}")
        );
    }

    #[test]
    fn detects_registry_key_paths() {
        assert!(is_registry_key_path(r"21:\Software\Inkscape\Inkscape"));
        assert!(is_registry_key_path(r"2:\Software\Demo"));
        assert!(!is_registry_key_path(
            r"C:\Program Files\Inkscape\bin\inkscape.exe"
        ));
    }

    #[test]
    fn recognizes_inno_log_names() {
        assert!(is_inno_log_name("unins000.dat"));
        assert!(is_inno_log_name("unins042.dat"));
        assert!(!is_inno_log_name("uninstall.exe"));
        assert!(!is_inno_log_name("unins000.exe"));
        assert!(!is_inno_log_name("uninsXYZ.dat"));
    }

    #[test]
    fn extracts_utf16_paths_from_log_bytes() {
        let mut data = b"Inno Setup Uninstall Log (b) 64-bit".to_vec();
        data.resize(64, 0);
        let path = r"C:\Program Files\Demo\app.exe";
        for unit in path.encode_utf16() {
            data.extend_from_slice(&unit.to_le_bytes());
        }
        data.extend_from_slice(&[0, 0]);

        let found = extract_utf16_paths(&data);
        assert!(
            found.iter().any(|candidate| candidate == path),
            "expected the embedded path, got {found:?}"
        );
    }

    #[test]
    fn excludes_shortcut_and_system_paths() {
        assert!(is_app_footprint_path(Path::new(
            r"C:\Program Files\Demo\app.exe"
        )));
        assert!(!is_app_footprint_path(Path::new(
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Demo\Demo.lnk"
        )));
    }

    #[test]
    fn collapses_nested_install_roots() {
        let files = vec![
            PathBuf::from(r"C:\Program Files\Inkscape\bin\inkscape.exe"),
            PathBuf::from(r"C:\Program Files\Inkscape\share\icons\a.svg"),
            PathBuf::from(r"C:\ProgramData\Inkscape\config.ini"),
        ];
        let mut roots = minimal_install_roots(&files);
        roots.sort();
        assert_eq!(
            roots,
            vec![
                PathBuf::from(r"C:\Program Files\Inkscape"),
                PathBuf::from(r"C:\ProgramData\Inkscape"),
            ]
        );
    }

    #[test]
    fn common_files_root_is_vendor_scoped_not_the_whole_shared_dir() {
        // A vendor file under the shared Common Files tree must resolve to the
        // vendor sub-directory, never the multi-vendor Common Files root.
        let files = vec![
            PathBuf::from(r"C:\Program Files (x86)\Common Files\Esoteric Technologies\shared.dll"),
            PathBuf::from(r"C:\Program Files (x86)\SolarFire9\SolarFire.exe"),
        ];
        let mut roots = minimal_install_roots(&files);
        roots.sort();
        assert_eq!(
            roots,
            vec![
                PathBuf::from(r"C:\Program Files (x86)\Common Files\Esoteric Technologies"),
                PathBuf::from(r"C:\Program Files (x86)\SolarFire9"),
            ]
        );
    }

    #[test]
    fn recognizes_nsis_log_name() {
        assert!(is_nsis_log_name("uninstall.log"));
        assert!(!is_nsis_log_name("install.log"));
        assert!(!is_nsis_log_name("unins000.dat"));
        assert!(!is_nsis_log_name("uninstall.exe"));
    }

    #[test]
    fn extracts_existing_paths_from_nsis_log() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let base = std::env::temp_dir().join(format!(
            "wam-nsis-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&base)?;
        let installed = base.join("app.exe");
        std::fs::write(&installed, b"binary")?;
        let log = base.join("uninstall.log");
        // AdvUninstLog-style line: a header comment plus the resolved path.
        let content = format!(";AdvUninstLog\r\n{}\r\n", installed.display());
        std::fs::write(&log, content.as_bytes())?;

        let found = find_nsis_log(&base);
        let files = nsis_manifest_files(&log, &SystemContext::live());
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found.as_deref(), Some(log.as_path()));
        assert!(
            files.iter().any(|path| path == &installed),
            "expected the installed path, got {files:?}"
        );
        Ok(())
    }

    fn unique_temp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ))
    }

    #[test]
    fn parses_appx_identity_name() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-appx-test");
        std::fs::create_dir_all(&base)?;
        std::fs::write(
            base.join(APPX_MANIFEST),
            r#"<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10">
  <Identity Name="Microsoft.WindowsTerminal" Publisher="CN=Microsoft Corporation" Version="1.18.0.0" />
</Package>"#,
        )?;

        let name = appx_identity_name(&base);
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(name.as_deref(), Some("Microsoft.WindowsTerminal"));
        Ok(())
    }

    #[test]
    fn selects_latest_squirrel_app_dir() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-squirrel-test");
        for version in ["app-1.0.2", "app-1.0.9", "app-1.0.10"] {
            std::fs::create_dir_all(base.join(version))?;
        }
        std::fs::write(base.join("Update.exe"), b"x")?;

        let latest = latest_squirrel_app_dir(&base);
        let expected = base.join("app-1.0.10");
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(
            latest.as_deref(),
            Some(expected.as_path()),
            "numeric version segments must rank 1.0.10 above 1.0.9"
        );
        Ok(())
    }
}
