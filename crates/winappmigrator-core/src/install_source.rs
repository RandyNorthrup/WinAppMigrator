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
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::models::{InstallMethod, InstalledApp};
use crate::path_utils::split_command_executable_path;

const USERDATA_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData";
const APP_PATHS_PREFIX: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths";
const SIGNATURE_SCAN_BYTES: u64 = 2_000_000;

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

pub fn detect_install_source(app: &InstalledApp) -> InstallSource {
    let product_code = extract_product_code(app);
    let method = classify_method(app, product_code.as_deref());

    let mut roots = Vec::new();
    let mut evidence = Vec::new();
    let mut authoritative_file_count = 0;
    let mut authoritative_registry_count = 0;

    match method {
        InstallMethod::WindowsInstaller => {
            if let Some(code) = product_code.as_deref()
                && let Some(paths) = msi_component_paths(code)
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
                let files = inno_manifest_files(&log);
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
        }
        InstallMethod::Squirrel => {
            evidence.push("Squirrel per-user layout (Update.exe / app-*)".to_owned());
        }
        InstallMethod::Chocolatey => {
            evidence.push("Chocolatey package metadata".to_owned());
        }
        InstallMethod::Msix => {
            evidence.push("MSIX / Appx package".to_owned());
        }
        InstallMethod::Generic => {}
    }

    // App Paths registration is an authoritative, documented source of the
    // launcher location and any extra DLL search directories.
    if let Some(executable) = app.main_executable.as_ref()
        && let Some(name) = executable.file_name().and_then(|value| value.to_str())
    {
        let app_paths = app_paths_roots(name);
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
        roots.push(parent.to_path_buf());
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
fn inno_manifest_files(log_path: &Path) -> Vec<PathBuf> {
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
        let path = PathBuf::from(&candidate);
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

fn app_paths_roots(executable_name: &str) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = format!("{APP_PATHS_PREFIX}\\{executable_name}");
    for flags in [KEY_READ | KEY_WOW64_64KEY, KEY_READ | KEY_WOW64_32KEY] {
        let Ok(key) = hklm.open_subkey_with_flags(&subkey, flags) else {
            continue;
        };
        if let Ok(default) = key.get_value::<String, _>("")
            && let Some(parent) = PathBuf::from(default.trim_matches('"')).parent()
        {
            roots.push(parent.to_path_buf());
        }
        if let Ok(path_value) = key.get_value::<String, _>("Path") {
            for segment in path_value.split(';') {
                let trimmed = segment.trim().trim_matches('"');
                if !trimmed.is_empty() {
                    roots.push(PathBuf::from(trimmed));
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
    entries.flatten().any(|entry| {
        entry.path().is_dir()
            && entry
                .file_name()
                .to_string_lossy()
                .to_ascii_lowercase()
                .starts_with("app-")
    })
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

fn msi_component_paths(product_code: &str) -> Option<MsiComponentPaths> {
    let squished_product = squish_guid(product_code)?;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let user_data = hklm
        .open_subkey_with_flags(USERDATA_PATH, KEY_READ | KEY_WOW64_64KEY)
        .ok()?;

    let mut files = Vec::new();
    let mut registry_count = 0;
    let mut seen = BTreeSet::new();

    for sid in user_data.enum_keys().flatten() {
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
                files.push(PathBuf::from(key_path));
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
/// root is the first path segment that follows one of these.
const ROOT_MARKERS: &[&str] = &[
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
}
