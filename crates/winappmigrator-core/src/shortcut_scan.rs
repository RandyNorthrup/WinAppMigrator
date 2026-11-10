use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::file_discovery::{
    DESKTOP_FALLBACK, DESKTOP_VALUE, START_MENU_FALLBACK, START_MENU_VALUE, user_folder_of,
};
use crate::models::{ShortcutEntry, Warning, WarningSeverity};
use crate::path_utils::{
    DEEPEST_WALK, POWERSHELL_IMPORT_MANAGEMENT, POWERSHELL_IMPORT_UTILITY, POWERSHELL_UTF8_OUTPUT,
    escape_powershell_single, path_has_extension, path_is_strictly_inside, powershell_command,
    trim_icon_suffix, walk_asking,
};
use crate::system_context::SystemContext;

const LNK_HEADER_SIZE: u32 = 0x4C;
const LNK_FLAGS_OFFSET: usize = 0x14;
const LNK_ICON_INDEX_OFFSET: usize = 0x38;
const LNK_HAS_TARGET_ID_LIST: u32 = 0x1;
const LNK_HAS_LINK_INFO: u32 = 0x2;
const LNK_HAS_NAME: u32 = 0x4;
const LNK_HAS_RELATIVE_PATH: u32 = 0x8;
const LNK_HAS_WORKING_DIR: u32 = 0x10;
const LNK_HAS_ARGUMENTS: u32 = 0x20;
const LNK_HAS_ICON_LOCATION: u32 = 0x40;
const LNK_IS_UNICODE: u32 = 0x80;
const LNK_ID_LIST_SIZE_FIELD: usize = 2;
const LNK_INFO_HEADER_SIZE_OFFSET: usize = 0x04;
const LNK_INFO_FLAGS_OFFSET: usize = 0x08;
const LNK_INFO_BASE_OFFSET: usize = 0x10;
const LNK_INFO_SUFFIX_OFFSET: usize = 0x18;
const LNK_INFO_UNICODE_BASE_OFFSET: usize = 0x1C;
const LNK_INFO_UNICODE_SUFFIX_OFFSET: usize = 0x20;
const LNK_INFO_UNICODE_HEADER_SIZE: u32 = 0x24;
const LNK_INFO_HAS_LOCAL_PATH: u32 = 0x1;
const LNK_HAS_EXP_STRING: u32 = 0x200;
/// `EnvironmentVariableDataBlock`: its size and signature, then the target in ANSI in 260 bytes, then
/// in UTF-16 in 520.
const ENVIRONMENT_BLOCK_SIGNATURE: u32 = 0xA000_0001;
const ENVIRONMENT_BLOCK_SIZE: usize = 0x314;
const ENVIRONMENT_ANSI_OFFSET: usize = 8;
const ENVIRONMENT_UNICODE_OFFSET: usize = 8 + 260;
/// Far more than any shortcut. On a drive from elsewhere, one could be made big enough to run this
/// out of memory.
const MAX_SHORTCUT_BYTES: u64 = 1024 * 1024;
const SHORTCUT_EXTENSION: &str = "lnk";
const COMMON_START_MENU: &str = r"Microsoft\Windows\Start Menu";

/// The app's paths as its own PC calls them, which is what a shortcut holds.
#[derive(Debug, Clone)]
pub struct ShortcutScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    /// Other folders that are the app's, because a person added them.
    pub also_the_apps: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PowerShellShortcut {
    path: Option<String>,
    target_path: Option<String>,
    arguments: Option<String>,
    working_directory: Option<String>,
    icon_location: Option<String>,
}

pub fn scan_shortcuts(
    context: &ShortcutScanContext,
    system: &SystemContext,
) -> (Vec<ShortcutEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let folders = shortcut_folders(system, &mut warnings);
    let (entries, more_warnings) = if system.is_offline() {
        read_shortcuts_on_a_drive(context, &folders, system)
    } else {
        scan_live_shortcuts(context, &folders)
    };
    warnings.extend(more_warnings);
    (entries, warnings)
}

/// Where shortcuts for everyone on a PC are, which someone's settings pointing there do not make theirs.
pub(crate) fn shared_shortcut_folders(system: &SystemContext) -> Vec<PathBuf> {
    system
        .public_dir()
        .map(|public| public.join(DESKTOP_FALLBACK))
        .into_iter()
        .chain(
            system
                .program_data()
                .map(|program_data| program_data.join(COMMON_START_MENU)),
        )
        .collect()
}

/// Where the account being scanned keeps shortcuts, by what its own settings say.
pub(crate) fn folders_of_the_accounts_shortcuts(system: &SystemContext) -> Vec<PathBuf> {
    let mut warnings = Vec::new();
    let mut folders = Vec::new();
    for user in system.user_hives() {
        folders.extend(user_folder_of(
            &user,
            DESKTOP_VALUE,
            DESKTOP_FALLBACK,
            system,
            &mut warnings,
        ));
        folders.extend(user_folder_of(
            &user,
            START_MENU_VALUE,
            START_MENU_FALLBACK,
            system,
            &mut warnings,
        ));
    }
    folders
}

/// Where the account being scanned, and everyone on the PC, keep shortcuts.
fn shortcut_folders(system: &SystemContext, warnings: &mut Vec<Warning>) -> Vec<PathBuf> {
    let mut folders = Vec::new();
    for user in system.user_hives() {
        folders.extend(user_folder_of(
            &user,
            DESKTOP_VALUE,
            DESKTOP_FALLBACK,
            system,
            warnings,
        ));
        folders.extend(user_folder_of(
            &user,
            START_MENU_VALUE,
            START_MENU_FALLBACK,
            system,
            warnings,
        ));
    }
    folders.extend(shared_shortcut_folders(system));
    // A value can say an account's Desktop is someone else's.
    folders.retain(|folder| system.may_read(folder) && !system.is_in_another_profile(folder));
    folders
}

/// Read each `.lnk` file on a drive directly. COM would resolve what it found against this PC.
fn read_shortcuts_on_a_drive(
    context: &ShortcutScanContext,
    folders: &[PathBuf],
    system: &SystemContext,
) -> (Vec<ShortcutEntry>, Vec<Warning>) {
    let mut entries = Vec::new();
    let mut unreadable = Vec::new();
    for folder in folders {
        for entry in walk_asking(folder, DEEPEST_WALK, |path, _| system.may_read(path)).flatten() {
            let path = entry.path.as_path();
            if !entry.file_type.is_file() || !path_has_extension(path, SHORTCUT_EXTENSION) {
                continue;
            }
            let data = read_shortcut_file(path);
            let read = data.as_deref().and_then(read_lnk).or_else(|| {
                // What a variable in it means on the PC the drive is from, and nothing on this one.
                data.as_deref()
                    .and_then(read_lnk_written_with_variables)
                    .map(|link| LnkFile {
                        target: PathBuf::from(
                            system.expand_source_variables(&link.target.to_string_lossy()),
                        ),
                        ..link
                    })
                    .filter(|link| !link.target.to_string_lossy().contains('%'))
            });
            let Some(link) = read else {
                unreadable.push(path.display().to_string());
                continue;
            };
            if path_matches(context, &link.target) {
                entries.push(ShortcutEntry {
                    original_lnk_path: system.logical_path(path),
                    target_path: link.target,
                    arguments: link.arguments,
                    working_directory: link.working_directory,
                    icon_path: link.icon_location,
                    target_is_selected_app: true,
                });
            }
        }
    }
    let warnings = if unreadable.is_empty() {
        Vec::new()
    } else {
        // An MSI's "advertised" shortcut names no path, and so says nothing about whose it is.
        vec![Warning::new(
            "shortcuts_unreadable",
            WarningSeverity::Info,
            format!(
                "{} shortcuts on this drive name no file, so whether they open this app could not \
                 be told.",
                unreadable.len()
            ),
            None,
        )]
    };
    (entries, warnings)
}

/// A shortcut's file, if it is small enough to be one.
pub(crate) fn read_shortcut_file(path: &Path) -> Option<Vec<u8>> {
    let size = crate::source_fs::metadata(path).ok()?.len();
    (size <= MAX_SHORTCUT_BYTES)
        .then(|| crate::source_fs::read(path).ok())
        .flatten()
}

fn scan_live_shortcuts(
    context: &ShortcutScanContext,
    folders: &[PathBuf],
) -> (Vec<ShortcutEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let roots = folders
        .iter()
        .map(|folder| {
            format!(
                "'{}'",
                escape_powershell_single(&folder.display().to_string())
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let script = format!(
        r#"
$roots = @({roots}) | Where-Object {{ $_ -and (Test-Path -LiteralPath $_) }} | Select-Object -Unique
$shell = New-Object -ComObject WScript.Shell
$items = foreach ($root in $roots) {{
  Get-ChildItem -LiteralPath $root -Filter *.lnk -Recurse -ErrorAction SilentlyContinue | ForEach-Object {{
    $shortcut = $shell.CreateShortcut($_.FullName)
    [pscustomobject]@{{
      Path = $_.FullName
      TargetPath = $shortcut.TargetPath
      Arguments = $shortcut.Arguments
      WorkingDirectory = $shortcut.WorkingDirectory
      IconLocation = $shortcut.IconLocation
    }}
  }}
}}
$items | ConvertTo-Json -Depth 4
"#
    );
    let output = match run_powershell(&script) {
        Ok(output) => output,
        Err(error) => {
            warnings.push(Warning::new(
                "shortcuts_unreadable",
                WarningSeverity::Warning,
                format!("Could not query shortcuts: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };
    let shortcuts = match parse_json_array::<PowerShellShortcut>(&output) {
        Ok(shortcuts) => shortcuts,
        Err(error) => {
            warnings.push(Warning::new(
                "shortcuts_parse_failed",
                WarningSeverity::Warning,
                format!("Could not parse shortcut query output: {error}"),
                None,
            ));
            return (Vec::new(), warnings);
        }
    };

    let entries = shortcuts
        .into_iter()
        .filter_map(|shortcut| shortcut_entry(context, shortcut))
        .collect();
    (entries, warnings)
}

fn shortcut_entry(
    context: &ShortcutScanContext,
    shortcut: PowerShellShortcut,
) -> Option<ShortcutEntry> {
    let target_path = PathBuf::from(shortcut.target_path?);
    if !path_matches(context, &target_path) {
        return None;
    }

    Some(ShortcutEntry {
        original_lnk_path: PathBuf::from(shortcut.path?),
        target_path,
        arguments: shortcut.arguments.filter(|value| !value.trim().is_empty()),
        working_directory: shortcut
            .working_directory
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from),
        // Windows says `,0` of a shortcut with no icon of its own, which uses its target's.
        icon_path: shortcut
            .icon_location
            .filter(|value| !trim_icon_suffix(value).is_empty())
            .map(PathBuf::from),
        target_is_selected_app: true,
    })
}

fn run_powershell(script: &str) -> std::io::Result<String> {
    let script = format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_MANAGEMENT}{POWERSHELL_IMPORT_UTILITY}{script}"
    );
    let output = powershell_command(&script).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn parse_json_array<T>(json: &str) -> serde_json::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let trimmed = json.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    if trimmed.starts_with('[') {
        serde_json::from_str(trimmed)
    } else {
        serde_json::from_str(trimmed).map(|item| vec![item])
    }
}

fn path_matches(context: &ShortcutScanContext, candidate: &Path) -> bool {
    candidate
        .to_string_lossy()
        .eq_ignore_ascii_case(&context.executable_path.to_string_lossy())
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_strictly_inside(candidate, root))
        || context
            .also_the_apps
            .iter()
            .any(|root| path_is_strictly_inside(candidate, root))
}

/// The target of a `.lnk` file. See [`read_lnk`].
pub(crate) fn lnk_target_path(data: &[u8]) -> Option<PathBuf> {
    read_lnk(data).map(|link| link.target)
}

/// What a `.lnk` file says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LnkFile {
    pub target: PathBuf,
    pub arguments: Option<String>,
    pub working_directory: Option<PathBuf>,
    /// `path,index`, as it reads through COM.
    pub icon_location: Option<PathBuf>,
}

/// Read a `.lnk` file from its `LinkInfo` section and the strings after it, without the COM object
/// `scan_live_shortcuts` uses, so it also works on a shortcut from another PC. A shortcut
/// that only stores a shell item list has no such path, and neither does an MSI
/// "advertised" one: both give `None`. The format is documented in MS-SHLLINK.
pub(crate) fn read_lnk(data: &[u8]) -> Option<LnkFile> {
    if read_u32(data, 0)? != LNK_HEADER_SIZE {
        return None;
    }
    let flags = read_u32(data, LNK_FLAGS_OFFSET)?;
    let mut offset = LNK_HEADER_SIZE as usize;
    if flags & LNK_HAS_TARGET_ID_LIST != 0 {
        let list_size = usize::from(read_u16(data, offset)?);
        offset += LNK_ID_LIST_SIZE_FIELD + list_size;
    }
    if flags & LNK_HAS_LINK_INFO == 0 {
        return None;
    }
    let info = data.get(offset..)?;
    let target = local_path_in_link_info(info)?;
    let info_size = read_u32(info, 0)? as usize;
    let strings = read_lnk_strings(info.get(info_size..).unwrap_or_default(), flags);
    Some(LnkFile {
        target,
        arguments: strings.arguments,
        working_directory: strings.working_directory.map(PathBuf::from),
        icon_location: strings.icon_location.map(|path| {
            let index = read_u32(data, LNK_ICON_INDEX_OFFSET).unwrap_or_default() as i32;
            PathBuf::from(format!("{path},{index}"))
        }),
    })
}

#[derive(Default)]
struct LnkStrings {
    working_directory: Option<String>,
    arguments: Option<String>,
    icon_location: Option<String>,
}

/// A shortcut whose target is only written with a variable, such as `%windir%\system32\calc.exe`, in
/// its `EnvironmentVariableDataBlock`, with no `LinkInfo`. Windows 7's own shortcuts are made this way,
/// and so are some an installer of its time made. The target comes back as it is written, for the
/// caller to read as the PC the shortcut is from would.
pub(crate) fn read_lnk_written_with_variables(data: &[u8]) -> Option<LnkFile> {
    if read_u32(data, 0)? != LNK_HEADER_SIZE {
        return None;
    }
    let flags = read_u32(data, LNK_FLAGS_OFFSET)?;
    if flags & LNK_HAS_EXP_STRING == 0 || flags & LNK_HAS_LINK_INFO != 0 {
        return None;
    }
    let mut offset = LNK_HEADER_SIZE as usize;
    if flags & LNK_HAS_TARGET_ID_LIST != 0 {
        let list_size = usize::from(read_u16(data, offset)?);
        offset = offset.checked_add(LNK_ID_LIST_SIZE_FIELD + list_size)?;
    }
    let (strings, mut extra) = read_lnk_strings_and_rest(data.get(offset..)?, flags);
    // Each block has its size first, and a size under four ends them.
    let target = loop {
        let size = read_u32(extra, 0)? as usize;
        if size < 4 || size > extra.len() {
            return None;
        }
        if read_u32(extra, 4)? == ENVIRONMENT_BLOCK_SIGNATURE && size >= ENVIRONMENT_BLOCK_SIZE {
            let block = &extra[..size];
            let unicode = block
                .get(ENVIRONMENT_UNICODE_OFFSET..ENVIRONMENT_BLOCK_SIZE)
                .and_then(|text| utf16_string(text, 0))
                .filter(|text| !text.trim().is_empty());
            break unicode.or_else(|| ansi_string(block, ENVIRONMENT_ANSI_OFFSET as u32))?;
        }
        extra = &extra[size..];
    };
    let target = target.trim();
    (!target.is_empty()).then(|| LnkFile {
        target: PathBuf::from(target),
        arguments: strings.arguments,
        working_directory: strings.working_directory.map(PathBuf::from),
        icon_location: strings.icon_location.map(|path| {
            let index = read_u32(data, LNK_ICON_INDEX_OFFSET).unwrap_or_default() as i32;
            PathBuf::from(format!("{path},{index}"))
        }),
    })
}

/// The strings after `LinkInfo`, each with its length first, those a shortcut has in a fixed order.
/// Whatever does not fit in the file is simply not there.
fn read_lnk_strings(data: &[u8], flags: u32) -> LnkStrings {
    read_lnk_strings_and_rest(data, flags).0
}

/// [`read_lnk_strings`], and what comes after them, which is where the extra blocks are.
fn read_lnk_strings_and_rest(mut data: &[u8], flags: u32) -> (LnkStrings, &[u8]) {
    let mut strings = LnkStrings::default();
    let unicode = flags & LNK_IS_UNICODE != 0;
    let mut next = |wanted: bool| -> Option<String> {
        if !wanted {
            return None;
        }
        let characters = usize::from(read_u16(data, 0)?);
        let bytes = if unicode { characters * 2 } else { characters };
        let text = data.get(2..2 + bytes)?;
        let value = if unicode {
            let (pairs, _) = text.as_chunks::<2>();
            let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
            String::from_utf16_lossy(&units)
        } else {
            text.iter().map(|&byte| char::from(byte)).collect()
        };
        data = &data[2 + bytes..];
        Some(value).filter(|value| !value.trim().is_empty())
    };
    next(flags & LNK_HAS_NAME != 0);
    next(flags & LNK_HAS_RELATIVE_PATH != 0);
    strings.working_directory = next(flags & LNK_HAS_WORKING_DIR != 0);
    strings.arguments = next(flags & LNK_HAS_ARGUMENTS != 0);
    strings.icon_location = next(flags & LNK_HAS_ICON_LOCATION != 0);
    (strings, data)
}

fn local_path_in_link_info(info: &[u8]) -> Option<PathBuf> {
    if read_u32(info, LNK_INFO_FLAGS_OFFSET)? & LNK_INFO_HAS_LOCAL_PATH == 0 {
        return None;
    }
    let header_size = read_u32(info, LNK_INFO_HEADER_SIZE_OFFSET)?;
    if header_size >= LNK_INFO_UNICODE_HEADER_SIZE
        && let Some(base) = read_u32(info, LNK_INFO_UNICODE_BASE_OFFSET)
            .and_then(|at| utf16_string(info, at))
            .filter(|base| !base.is_empty())
    {
        let suffix = read_u32(info, LNK_INFO_UNICODE_SUFFIX_OFFSET)
            .and_then(|at| utf16_string(info, at))
            .unwrap_or_default();
        return Some(PathBuf::from(format!("{base}{suffix}")));
    }
    let base = ansi_string(info, read_u32(info, LNK_INFO_BASE_OFFSET)?)?;
    let suffix = read_u32(info, LNK_INFO_SUFFIX_OFFSET)
        .and_then(|at| ansi_string(info, at))
        .unwrap_or_default();
    Some(PathBuf::from(format!("{base}{suffix}"))).filter(|path| !path.as_os_str().is_empty())
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// A NUL-terminated string in the local code page, read as Latin-1: a path outside that
/// fails the existence check callers apply, and the shortcut is simply not used.
fn ansi_string(data: &[u8], offset: u32) -> Option<String> {
    let start = data.get(offset as usize..)?;
    let end = start.iter().position(|&byte| byte == 0)?;
    Some(start[..end].iter().map(|&byte| char::from(byte)).collect())
}

fn utf16_string(data: &[u8], offset: u32) -> Option<String> {
    let (pairs, _) = data.get(offset as usize..)?.as_chunks::<2>();
    let units: Vec<u16> = pairs
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|&unit| unit != 0)
        .collect();
    Some(String::from_utf16_lossy(&units))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path_utils::hidden_command;

    /// A shortcut holding the given local path, in the ANSI form or also the Unicode one.
    fn lnk_bytes(base: &str, suffix: &str, unicode: bool, id_list: &[u8]) -> Vec<u8> {
        let mut data = vec![0_u8; LNK_HEADER_SIZE as usize];
        data[0..4].copy_from_slice(&LNK_HEADER_SIZE.to_le_bytes());
        let mut flags = LNK_HAS_LINK_INFO;
        if !id_list.is_empty() {
            flags |= LNK_HAS_TARGET_ID_LIST;
            data.extend_from_slice(&(id_list.len() as u16).to_le_bytes());
            data.extend_from_slice(id_list);
        }
        data[LNK_FLAGS_OFFSET..LNK_FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());

        let header_size: u32 = if unicode {
            LNK_INFO_UNICODE_HEADER_SIZE
        } else {
            0x1C
        };
        let base_at = header_size;
        let suffix_at = base_at + base.len() as u32 + 1;
        let unicode_base_at = suffix_at + suffix.len() as u32 + 1;
        let unicode_suffix_at = unicode_base_at + (base.encode_utf16().count() as u32 + 1) * 2;
        let mut info = vec![0_u8; header_size as usize];
        let mut put = |at: usize, value: u32| {
            info[at..at + 4].copy_from_slice(&value.to_le_bytes());
        };
        let total = if unicode {
            unicode_suffix_at + (suffix.encode_utf16().count() as u32 + 1) * 2
        } else {
            unicode_base_at
        };
        put(0, total);
        put(LNK_INFO_HEADER_SIZE_OFFSET, header_size);
        put(LNK_INFO_FLAGS_OFFSET, LNK_INFO_HAS_LOCAL_PATH);
        put(LNK_INFO_BASE_OFFSET, base_at);
        put(LNK_INFO_SUFFIX_OFFSET, suffix_at);
        if unicode {
            put(LNK_INFO_UNICODE_BASE_OFFSET, unicode_base_at);
            put(LNK_INFO_UNICODE_SUFFIX_OFFSET, unicode_suffix_at);
        }
        info.extend_from_slice(base.as_bytes());
        info.push(0);
        info.extend_from_slice(suffix.as_bytes());
        info.push(0);
        if unicode {
            for text in [base, suffix] {
                info.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
                info.extend_from_slice(&[0, 0]);
            }
        }
        data.extend_from_slice(&info);
        data
    }

    #[test]
    fn reads_a_shortcut_target() {
        let path = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
        assert_eq!(
            lnk_target_path(&lnk_bytes(path, "", false, &[])),
            Some(PathBuf::from(path))
        );
        assert_eq!(
            lnk_target_path(&lnk_bytes(path, "", true, &[1, 2, 3, 4])),
            Some(PathBuf::from(path)),
            "skips the item list, and reads the Unicode form when there is one"
        );
        assert_eq!(
            lnk_target_path(&lnk_bytes(r"C:\Program Files\", "App\\app.exe", false, &[])),
            Some(PathBuf::from(r"C:\Program Files\App\app.exe")),
            "the path can be split into a base and a suffix"
        );
    }

    /// A shortcut as Windows 7 makes its own: an item list, arguments, and the target only in an
    /// `EnvironmentVariableDataBlock`, after another block.
    fn written_with_variables(target: &str, unicode_target: bool) -> Vec<u8> {
        let mut data = vec![0_u8; LNK_HEADER_SIZE as usize];
        data[0..4].copy_from_slice(&LNK_HEADER_SIZE.to_le_bytes());
        let flags =
            LNK_HAS_TARGET_ID_LIST | LNK_HAS_ARGUMENTS | LNK_IS_UNICODE | LNK_HAS_EXP_STRING;
        data[LNK_FLAGS_OFFSET..LNK_FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&[9, 9, 9]);
        let arguments: Vec<u16> = "/x".encode_utf16().collect();
        data.extend_from_slice(&(arguments.len() as u16).to_le_bytes());
        for unit in arguments {
            data.extend_from_slice(&unit.to_le_bytes());
        }
        // Some other block first, which is stepped over by its size.
        data.extend_from_slice(&12_u32.to_le_bytes());
        data.extend_from_slice(&0xA000_0009_u32.to_le_bytes());
        data.extend_from_slice(&[0; 4]);
        let mut block = vec![0_u8; ENVIRONMENT_BLOCK_SIZE];
        block[0..4].copy_from_slice(&(ENVIRONMENT_BLOCK_SIZE as u32).to_le_bytes());
        block[4..8].copy_from_slice(&ENVIRONMENT_BLOCK_SIGNATURE.to_le_bytes());
        block[ENVIRONMENT_ANSI_OFFSET..ENVIRONMENT_ANSI_OFFSET + target.len()]
            .copy_from_slice(target.as_bytes());
        if unicode_target {
            for (index, unit) in target.encode_utf16().enumerate() {
                let at = ENVIRONMENT_UNICODE_OFFSET + index * 2;
                block[at..at + 2].copy_from_slice(&unit.to_le_bytes());
            }
        }
        data.extend_from_slice(&block);
        data.extend_from_slice(&0_u32.to_le_bytes());
        data
    }

    #[test]
    fn a_shortcut_written_with_variables_is_read_as_written() {
        for unicode in [true, false] {
            let data = written_with_variables(r"%windir%\system32\calc.exe", unicode);
            assert_eq!(read_lnk(&data), None, "it has no LinkInfo");
            let link = read_lnk_written_with_variables(&data);
            assert_eq!(
                link.as_ref().map(|link| link.target.clone()),
                Some(PathBuf::from(r"%windir%\system32\calc.exe")),
                "{unicode}"
            );
            assert_eq!(link.and_then(|link| link.arguments).as_deref(), Some("/x"));
        }
        // One with a LinkInfo is read by that, and one with no block, or cut short, by nothing.
        assert_eq!(
            read_lnk_written_with_variables(&lnk_bytes(r"C:\App\app.exe", "", true, &[])),
            None
        );
        let whole = written_with_variables(r"%ProgramFiles%\App\app.exe", true);
        for cut in 0..whole.len() {
            let _ = read_lnk_written_with_variables(&whole[..cut]);
        }
        assert_eq!(
            read_lnk_written_with_variables(&whole[..whole.len() - 300]),
            None
        );
    }

    #[test]
    fn a_shortcuts_strings_are_read_from_after_where_its_target_is() {
        fn with_strings(strings: &[(u32, &str)], unicode: bool) -> Vec<u8> {
            let mut data = lnk_bytes(r"C:\App\app.exe", "", true, &[1, 2]);
            let mut flags = read_u32(&data, LNK_FLAGS_OFFSET).unwrap_or_default();
            if unicode {
                flags |= LNK_IS_UNICODE;
            }
            for (flag, text) in strings {
                flags |= flag;
                let length = text.encode_utf16().count() as u16;
                data.extend_from_slice(&length.to_le_bytes());
                if unicode {
                    data.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
                } else {
                    data.extend_from_slice(text.as_bytes());
                }
            }
            data[LNK_FLAGS_OFFSET..LNK_FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());
            data[LNK_ICON_INDEX_OFFSET..LNK_ICON_INDEX_OFFSET + 4]
                .copy_from_slice(&3_i32.to_le_bytes());
            data
        }
        for unicode in [true, false] {
            let data = with_strings(
                &[
                    (LNK_HAS_NAME, "A description"),
                    (LNK_HAS_RELATIVE_PATH, r"..\app.exe"),
                    (LNK_HAS_WORKING_DIR, r"C:\App"),
                    (LNK_HAS_ARGUMENTS, "--run"),
                    (LNK_HAS_ICON_LOCATION, r"C:\App\icon.ico"),
                ],
                unicode,
            );
            assert_eq!(
                read_lnk(&data),
                Some(LnkFile {
                    target: PathBuf::from(r"C:\App\app.exe"),
                    arguments: Some("--run".to_owned()),
                    working_directory: Some(PathBuf::from(r"C:\App")),
                    icon_location: Some(PathBuf::from(r"C:\App\icon.ico,3")),
                }),
                "unicode: {unicode}"
            );
            // Cut anywhere: what is whole is kept, and nothing panics.
            for cut in 0..data.len() {
                let _ = read_lnk(&data[..cut]);
            }
        }
        // None of the optional ones.
        assert_eq!(
            read_lnk(&with_strings(&[], true)),
            Some(LnkFile {
                target: PathBuf::from(r"C:\App\app.exe"),
                arguments: None,
                working_directory: None,
                icon_location: None,
            })
        );
    }

    #[test]
    fn a_shortcut_on_a_drive_is_kept_for_what_it_opens_and_not_read_through_a_link() {
        let Ok(work) = tempfile::tempdir() else {
            panic!("no temp folder");
        };
        let context = ShortcutScanContext {
            executable_path: PathBuf::from(r"C:\App\app.exe"),
            install_location: Some(PathBuf::from(r"C:\App")),
            also_the_apps: Vec::new(),
        };
        let desktop = work.path().join("Desktop");
        let elsewhere = work.path().join("elsewhere");
        for folder in [&desktop, &elsewhere] {
            assert!(std::fs::create_dir_all(folder).is_ok());
        }
        let own = lnk_bytes(r"C:\App\app.exe", "", true, &[]);
        let other = lnk_bytes(r"C:\Other\other.exe", "", true, &[]);
        assert!(std::fs::write(desktop.join("App.lnk"), &own).is_ok());
        assert!(std::fs::write(desktop.join("Other.lnk"), &other).is_ok());
        assert!(std::fs::write(desktop.join("Damaged.lnk"), b"not one").is_ok());
        assert!(std::fs::write(elsewhere.join("Theirs.lnk"), &own).is_ok());
        let made = hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(desktop.join("Linked"))
            .arg(&elsewhere)
            .output();
        assert!(made.is_ok_and(|output| output.status.success()));

        let (entries, warnings) = read_shortcuts_on_a_drive(
            &context,
            std::slice::from_ref(&desktop),
            &SystemContext::live(),
        );
        let found: Vec<PathBuf> = entries
            .iter()
            .map(|entry| entry.original_lnk_path.clone())
            .collect();
        assert_eq!(found, vec![desktop.join("App.lnk")]);
        assert_eq!(
            warnings.len(),
            1,
            "the damaged one is said to be: {warnings:?}"
        );
    }

    #[test]
    fn a_shortcut_without_a_local_path_gives_nothing() {
        let mut advertised = vec![0_u8; LNK_HEADER_SIZE as usize];
        advertised[0..4].copy_from_slice(&LNK_HEADER_SIZE.to_le_bytes());
        assert_eq!(lnk_target_path(&advertised), None);
        assert_eq!(lnk_target_path(b"not a shortcut"), None);
        assert_eq!(lnk_target_path(&[]), None);
    }

    #[test]
    fn a_truncated_shortcut_does_not_panic() {
        let whole = lnk_bytes(r"C:\App\app.exe", "", true, &[9, 9]);
        for cut in 0..whole.len() {
            let _ = lnk_target_path(&whole[..cut]);
        }
    }

    #[test]
    fn a_shortcut_that_says_more_than_it_holds_gives_nothing_and_does_not_panic() {
        // A list of items said to go on past the end of the file.
        let mut data = lnk_bytes(r"C:\App\app.exe", "", true, &[9, 9]);
        let list_size_at = LNK_HEADER_SIZE as usize;
        data[list_size_at..list_size_at + 2].copy_from_slice(&u16::MAX.to_le_bytes());
        assert_eq!(lnk_target_path(&data), None);

        // Where its path is said to be, past the end of the file.
        let info_at = LNK_HEADER_SIZE as usize;
        let mut data = lnk_bytes(r"C:\App\app.exe", "", false, &[]);
        data[info_at + LNK_INFO_BASE_OFFSET..info_at + LNK_INFO_BASE_OFFSET + 4]
            .copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(lnk_target_path(&data), None);
    }

    #[test]
    fn a_shortcut_goes_by_its_ansi_path_when_the_unicode_one_is_not_there_or_is_empty() {
        let info_at = LNK_HEADER_SIZE as usize;
        let unicode_at = info_at + LNK_INFO_UNICODE_BASE_OFFSET;
        let want = Some(PathBuf::from(r"C:\App\app.exe"));

        let mut past_the_end = lnk_bytes(r"C:\App\app.exe", "", true, &[]);
        past_the_end[unicode_at..unicode_at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(lnk_target_path(&past_the_end), want);

        // Pointing at the two zero bytes that end the Unicode suffix, which is an empty string.
        let mut empty = lnk_bytes(r"C:\App\app.exe", "", true, &[]);
        let suffix_at = u32::from_le_bytes([
            empty[info_at + LNK_INFO_UNICODE_SUFFIX_OFFSET],
            empty[info_at + LNK_INFO_UNICODE_SUFFIX_OFFSET + 1],
            empty[info_at + LNK_INFO_UNICODE_SUFFIX_OFFSET + 2],
            empty[info_at + LNK_INFO_UNICODE_SUFFIX_OFFSET + 3],
        ]);
        empty[unicode_at..unicode_at + 4].copy_from_slice(&suffix_at.to_le_bytes());
        assert_eq!(lnk_target_path(&empty), want);
    }

    #[test]
    fn a_shortcut_with_an_empty_path_has_no_target() {
        for unicode in [false, true] {
            assert_eq!(lnk_target_path(&lnk_bytes("", "", unicode, &[])), None);
        }
    }

    /// A path that is not in the code page this PC reads shortcuts in comes out different, and then
    /// matches nothing. See [`ansi_string`].
    #[test]
    fn an_ansi_path_is_read_a_byte_at_a_time() {
        let mut data = lnk_bytes("C:\\App\\gr?e.exe", "", false, &[]);
        let at = data
            .iter()
            .position(|&byte| byte == b'?')
            .unwrap_or_default();
        data[at] = 0xFC;
        assert_eq!(
            lnk_target_path(&data),
            Some(PathBuf::from("C:\\App\\gr\u{fc}e.exe"))
        );
    }

    /// [`lnk_bytes`], with these strings after it.
    fn lnk_with_strings(flags: u32, strings: &[&str], icon_index: i32) -> Vec<u8> {
        let mut data = lnk_bytes(r"C:\App\app.exe", "", true, &[]);
        let all = LNK_HAS_LINK_INFO | LNK_IS_UNICODE | flags;
        data[LNK_FLAGS_OFFSET..LNK_FLAGS_OFFSET + 4].copy_from_slice(&all.to_le_bytes());
        data[LNK_ICON_INDEX_OFFSET..LNK_ICON_INDEX_OFFSET + 4]
            .copy_from_slice(&icon_index.to_le_bytes());
        for text in strings {
            let units: Vec<u16> = text.encode_utf16().collect();
            data.extend_from_slice(&(units.len() as u16).to_le_bytes());
            data.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
        }
        data
    }

    #[test]
    fn an_icon_says_which_one_in_its_file_even_counting_from_the_end() {
        let data = lnk_with_strings(LNK_HAS_ICON_LOCATION, &[r"C:\App\icons.dll"], -3);
        assert_eq!(
            read_lnk(&data).and_then(|link| link.icon_location),
            Some(PathBuf::from(r"C:\App\icons.dll,-3"))
        );
    }

    #[test]
    fn a_string_cut_short_is_not_there_and_neither_is_what_comes_after_it() {
        let flags = LNK_HAS_WORKING_DIR | LNK_HAS_ARGUMENTS | LNK_HAS_ICON_LOCATION;
        let whole = lnk_with_strings(flags, &[r"C:\App", "--run", r"C:\App\a.ico"], 0);
        let Some(link) = read_lnk(&whole) else {
            panic!("the whole of it reads");
        };
        assert_eq!(link.arguments.as_deref(), Some("--run"));
        let tail = (2 + r"C:\App\a.ico".len() * 2) + 2 + "--run".len() * 2 - 2;

        let Some(cut) = read_lnk(&whole[..whole.len() - tail]) else {
            panic!("its target is before all of this");
        };
        assert_eq!(cut.target, link.target);
        assert_eq!(cut.working_directory, link.working_directory);
        assert_eq!(cut.arguments, None);
        assert_eq!(cut.icon_location, None);
    }

    #[test]
    fn a_shortcut_with_strings_does_not_panic_wherever_it_is_cut() {
        let flags = LNK_HAS_NAME
            | LNK_HAS_RELATIVE_PATH
            | LNK_HAS_WORKING_DIR
            | LNK_HAS_ARGUMENTS
            | LNK_HAS_ICON_LOCATION;
        let whole = lnk_with_strings(flags, &["n", "r", "w", "a", "i"], 1);
        for cut in 0..whole.len() {
            let _ = read_lnk(&whole[..cut]);
        }
    }

    #[test]
    fn a_folder_a_person_added_is_the_apps_like_its_own() {
        let mut context = context_at(r"C:\Tools\acme.exe", None);
        let added = Path::new(r"D:\Acme Extra\tool.exe");
        assert!(!path_matches(&context, added));
        context.also_the_apps.push(PathBuf::from(r"D:\Acme Extra"));
        assert!(path_matches(&context, added));
        assert!(!path_matches(
            &context,
            Path::new(r"D:\Acme Extra2\tool.exe")
        ));
    }

    #[test]
    fn a_target_in_a_folder_with_a_longer_name_is_not_inside_the_install_folder() {
        let context = ShortcutScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\Git\git-bash.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Git")),
            also_the_apps: Vec::new(),
        };
        assert!(path_matches(
            &context,
            Path::new(r"C:\PROGRAM FILES\GIT\cmd\git.exe")
        ));
        assert!(!path_matches(
            &context,
            Path::new(r"C:\Program Files\GitHub CLI\gh.exe")
        ));
    }

    fn context_at(executable: &str, location: Option<&str>) -> ShortcutScanContext {
        ShortcutScanContext {
            executable_path: PathBuf::from(executable),
            install_location: location.map(PathBuf::from),
            also_the_apps: Vec::new(),
        }
    }

    fn powershell_shortcut(target: Option<&str>, path: Option<&str>) -> PowerShellShortcut {
        PowerShellShortcut {
            path: path.map(ToOwned::to_owned),
            target_path: target.map(ToOwned::to_owned),
            arguments: Some("  ".to_owned()),
            working_directory: Some(String::new()),
            icon_location: Some(r"C:\Program Files\Acme\acme.ico,0".to_owned()),
        }
    }

    #[test]
    fn what_windows_says_about_a_shortcut_is_only_kept_if_it_opens_the_app() {
        let context = context_at(
            r"C:\Program Files\Acme\acme.exe",
            Some(r"C:\Program Files\Acme"),
        );
        let link = r"C:\Users\me\Desktop\Acme.lnk";
        let kept = shortcut_entry(
            &context,
            powershell_shortcut(Some(r"C:\Program Files\Acme\acme.exe"), Some(link)),
        );
        let Some(kept) = kept else {
            panic!("it opens the app");
        };
        assert_eq!(kept.original_lnk_path, Path::new(link));
        assert_eq!(kept.arguments, None, "what is only spaces is nothing");
        assert_eq!(kept.working_directory, None);
        assert_eq!(
            kept.icon_path,
            Some(PathBuf::from(r"C:\Program Files\Acme\acme.ico,0"))
        );
        assert!(kept.target_is_selected_app);

        let mut no_icon = powershell_shortcut(Some(r"C:\Program Files\Acme\acme.exe"), Some(link));
        no_icon.icon_location = Some(",0".to_owned());
        assert_eq!(
            shortcut_entry(&context, no_icon).and_then(|kept| kept.icon_path),
            None,
            "as Windows says it of a shortcut with no icon of its own"
        );

        for (target, path) in [
            (Some(r"C:\Windows\System32\cmd.exe"), Some(link)),
            (None, Some(link)),
            (Some(r"C:\Program Files\Acme\acme.exe"), None),
        ] {
            assert!(
                shortcut_entry(&context, powershell_shortcut(target, path)).is_none(),
                "{target:?} {path:?}"
            );
        }
    }

    #[test]
    fn a_shortcut_opens_the_app_by_its_program_or_being_in_its_folder() {
        let with_folder = context_at(r"C:\Tools\acme.exe", Some(r"C:\Program Files\Acme"));
        for ours in [r"C:\Tools\ACME.exe", r"C:\Program Files\Acme\sub\tool.exe"] {
            assert!(path_matches(&with_folder, Path::new(ours)), "{ours}");
        }
        for not in [
            r"C:\Tools\other.exe",
            r"C:\Program Files\Acme",
            r"C:\Program Files\Acme\..\Other\x.exe",
        ] {
            assert!(!path_matches(&with_folder, Path::new(not)), "{not}");
        }
    }

    #[test]
    fn powershell_gives_one_item_or_a_list_or_nothing() -> serde_json::Result<()> {
        assert_eq!(parse_json_array::<u32>("")?, Vec::<u32>::new());
        assert_eq!(parse_json_array::<u32>("  \r\n")?, Vec::<u32>::new());
        assert_eq!(parse_json_array::<u32>("7")?, [7]);
        assert_eq!(parse_json_array::<u32>(" [1,2] ")?, [1, 2]);
        assert!(parse_json_array::<u32>("not json").is_err());
        Ok(())
    }
}
