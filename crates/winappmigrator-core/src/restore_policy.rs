//! What a package is allowed to make an elevated process do.
//!
//! A package can come from a hostile or merely odd source drive, through the genuine
//! builder, and nothing authenticates it. So everything it asks for is checked here when
//! restoring, not just when scanning, and the rules are pure functions so each can be
//! tested with a realistic case on both sides. A refusal is a sentence for the person
//! installing, saying what was left out and why.
//!
//! The rules are written to let a real app's own items through: a service or Run key that
//! belongs to the app must keep working.

use std::collections::HashSet;
use std::path::{Component, Path, PathBuf, Prefix};

use crate::errors::Result;
use crate::file_discovery::{
    has_reserved_device_name, is_reserved_device_name, restore_destination, safe_relative_path,
};
use crate::models::{DriverEntry, RegistryEntry, RegistryHive, RegistryValueKind};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{
    is_drive_root, path_has_extension, path_is_strictly_inside, path_is_within, sanitize_filename,
    trim_icon_suffix,
};
use crate::registry_target::hex_decode;
use crate::service_scan::parse_image_path;

/// The longest service name the Service Control Manager accepts.
const MAX_SERVICE_NAME_CHARS: usize = 256;
const LOCAL_SERVICE_ACCOUNT: &str = r"NT AUTHORITY\LocalService";
const NETWORK_SERVICE_ACCOUNT: &str = r"NT AUTHORITY\NetworkService";
const LOCAL_SYSTEM_ACCOUNT: &str = "LocalSystem";
const VIRTUAL_ACCOUNT_PREFIX: &str = r"NT SERVICE\";

/// True when `name` can be passed to `sc.exe` and joined to a registry path as one
/// service's name. Real names hold spaces, `$`, `.`, `(`, and `{`, as in
/// `MSSQL$SQLEXPRESS`, so this refuses only what could change the meaning of a command
/// or a key: separators, control characters, quotes, and `=`, which `sc.exe` reads as the
/// start of an option.
pub fn is_valid_service_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().count() <= MAX_SERVICE_NAME_CHARS
        && name.trim() == name
        && !name.starts_with('-')
        && !name
            .chars()
            .any(|character| character.is_control() || matches!(character, '/' | '\\' | '"' | '='))
}

/// The account a restored service runs as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceAccount {
    /// The package names none, so Windows applies its default, LocalSystem.
    Unspecified,
    LocalSystem,
    LocalService,
    NetworkService,
    /// `NT SERVICE\<service>`, an account Windows manages for that service alone.
    Virtual(String),
    /// A real user or managed account. Its password is not in the package and cannot be.
    Custom(String),
}

impl ServiceAccount {
    /// Work out the account from what the source's registry recorded. Built-in accounts
    /// have several spellings and need no password. `NT SERVICE\` only counts for the
    /// service's own name, the only one Windows would accept.
    pub fn from_recorded(recorded: Option<&str>, service_name: &str) -> Self {
        let Some(recorded) = recorded.map(str::trim).filter(|value| !value.is_empty()) else {
            return Self::Unspecified;
        };
        let lower = recorded.to_ascii_lowercase();
        let lower = lower.strip_prefix(r".\").unwrap_or(&lower);
        match lower {
            "localsystem" | r"nt authority\system" | "system" => Self::LocalSystem,
            "localservice"
            | r"nt authority\localservice"
            | r"nt authority\local service"
            | "local service" => Self::LocalService,
            "networkservice"
            | r"nt authority\networkservice"
            | r"nt authority\network service"
            | "network service" => Self::NetworkService,
            _ => {
                let virtual_name = recorded
                    .get(..VIRTUAL_ACCOUNT_PREFIX.len())
                    .filter(|prefix| prefix.eq_ignore_ascii_case(VIRTUAL_ACCOUNT_PREFIX))
                    .and_then(|prefix| recorded.strip_prefix(prefix));
                if virtual_name.is_some_and(|name| name.eq_ignore_ascii_case(service_name)) {
                    Self::Virtual(recorded.to_owned())
                } else {
                    Self::Custom(recorded.to_owned())
                }
            }
        }
    }

    /// The value for `sc.exe obj=`. A custom account has no password to go with it, so
    /// it becomes LocalService, which has few rights. Running it as LocalSystem instead
    /// would give it more than it had on the source.
    pub fn sc_object_name(&self) -> Option<&str> {
        match self {
            Self::Unspecified => None,
            Self::LocalSystem => Some(LOCAL_SYSTEM_ACCOUNT),
            Self::LocalService | Self::Custom(_) => Some(LOCAL_SERVICE_ACCOUNT),
            Self::NetworkService => Some(NETWORK_SERVICE_ACCOUNT),
            Self::Virtual(name) => Some(name),
        }
    }

    /// LocalService and NetworkService take an empty password when set. A virtual
    /// account must not be given one at all, and LocalSystem ignores it.
    pub fn sc_takes_empty_password(&self) -> bool {
        matches!(
            self,
            Self::LocalService | Self::NetworkService | Self::Custom(_)
        )
    }

    /// What to call this account in the installer's window.
    pub fn label(&self) -> String {
        match self {
            Self::Unspecified | Self::LocalSystem => LOCAL_SYSTEM_ACCOUNT.to_owned(),
            Self::LocalService => LOCAL_SERVICE_ACCOUNT.to_owned(),
            Self::NetworkService => NETWORK_SERVICE_ACCOUNT.to_owned(),
            Self::Virtual(name) => name.clone(),
            Self::Custom(_) => format!("{LOCAL_SERVICE_ACCOUNT} (custom account not restored)"),
        }
    }
}

/// The value to store as a service's `ImagePath`. Quoted, so that for
/// `C:\My Apps\Foo Bar\svc.exe` Windows cannot start `C:\My Apps\Foo.exe` instead.
///
/// `std::process::Command` removes only its own layer of quoting, and `sc.exe` stores what
/// it gets verbatim, so these quotes are what ends up in the registry. A kernel driver's
/// `ImagePath` is a path rather than a command line and must not get them.
pub fn service_image_path_value(binary: &Path, arguments: Option<&str>) -> String {
    let mut value = format!("\"{}\"", binary.display());
    if let Some(arguments) = arguments.map(str::trim).filter(|value| !value.is_empty()) {
        value.push(' ');
        value.push_str(arguments);
    }
    value
}

/// Why an install location must not be used, if so. The rules treat what is inside the
/// install location as the app's own, so that cannot be somewhere everything else lives,
/// and a package gets to suggest the location. `shared_folders` are those that hold other
/// things side by side: Program Files, ProgramData, user profiles, and so on.
pub fn install_location_refusal(
    target_root: &Path,
    windows_dir: &Path,
    shared_folders: &[PathBuf],
) -> Option<String> {
    if !target_root.is_absolute() {
        return Some("it must be a full path".to_owned());
    }
    if target_root
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Some("it must not contain \"..\"".to_owned());
    }
    // Windows would read these as something other than what they say, and so what they say
    // cannot be compared with anything.
    let reads_as_something_else = target_root.components().any(|component| match component {
        // Including a share, which can be this PC's own drives under another name.
        Component::Prefix(prefix) => !matches!(prefix.kind(), Prefix::Disk(_)),
        Component::Normal(name) => {
            let name = name.to_string_lossy();
            name.ends_with(['.', ' ']) || name.contains(':') || is_reserved_device_name(&name)
        }
        _ => false,
    });
    if reads_as_something_else {
        return Some("it has to be a plain path on a drive of this PC".to_owned());
    }
    install_placement_refusal(target_root, windows_dir, shared_folders)
}

/// Why an install location is somewhere an app cannot go, however it was written.
pub(crate) fn install_placement_refusal(
    target_root: &Path,
    windows_dir: &Path,
    shared_folders: &[PathBuf],
) -> Option<String> {
    if is_drive_root(target_root) {
        return Some("it cannot be a whole drive".to_owned());
    }
    if path_is_within(target_root, windows_dir) || path_is_within(windows_dir, target_root) {
        return Some("it cannot be the Windows folder".to_owned());
    }
    let lower = format!(
        "{}\\",
        target_root
            .to_string_lossy()
            .to_ascii_lowercase()
            .replace('/', "\\")
    );
    let runs_by_itself = lower.contains(STARTUP_FOLDER)
        || REFUSED_DATA_FOLDERS
            .iter()
            .filter(|folder| !FINE_FOR_AN_INSTALL_LOCATION.contains(folder))
            .any(|folder| lower.contains(&format!("\\{folder}")));
    if runs_by_itself {
        return Some(
            "it is a folder that belongs to Windows, or whose contents run or load by themselves"
                .to_owned(),
        );
    }
    shared_folders
        .iter()
        .any(|folder| paths_equal(folder, target_root))
        .then(|| {
            "it holds other things side by side, so the app needs a folder inside it".to_owned()
        })
}

/// Why a service already on this PC must not be taken over, if so. `--force` does not
/// change this. A package that names `WinDefend` must not be able to repoint it, however
/// the rest of the package looks. A service already running from the install location is
/// an earlier install of this app, which is what `--force` is for.
pub fn existing_service_refusal(
    service_name: &str,
    current_image_path: &str,
    target_root: &Path,
    windows_dir: &Path,
) -> Option<String> {
    let current = parse_image_path(current_image_path, windows_dir);
    (!path_is_strictly_inside(&current.path, target_root)).then(|| {
        format!(
            "{service_name} already exists on this PC and runs {}, outside the install location. \
             If it is an older copy of this app, uninstall that first",
            current.path.display()
        )
    })
}

/// Where a bundled driver image goes, always inside the install location. The driver image
/// is not a copyable file in the inventory, so its original path is re-anchored, with
/// `<target_root>\drivers\<leaf>` for one that was not under the install location.
pub fn driver_destination(driver: &DriverEntry, map: &RestoreMap) -> Result<PathBuf> {
    let target_root = map.target_root();
    let rewritten = PathBuf::from(map.rewrite(&driver.image_path.to_string_lossy()));
    if path_is_strictly_inside(&rewritten, target_root) {
        return Ok(rewritten);
    }
    let leaf = driver.image_path.file_name().map_or_else(
        || format!("{}.sys", sanitize_filename(&driver.service_name)),
        |name| name.to_string_lossy().into_owned(),
    );
    restore_destination(
        target_root,
        &Path::new("drivers").join(safe_relative_path(Path::new(&leaf))),
    )
}

/// True when the `ImagePath` Windows reports runs exactly this binary with these arguments.
pub fn image_path_matches(
    actual: &str,
    expected_binary: &Path,
    expected_arguments: Option<&str>,
    windows_dir: &Path,
) -> bool {
    let actual = parse_image_path(actual, windows_dir);
    paths_equal(&actual.path, expected_binary)
        && actual
            .arguments
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            == expected_arguments.map(str::trim).unwrap_or_default()
}

/// Paths compared as Windows would, ignoring case and a trailing separator, and no looser.
pub fn paths_equal(left: &Path, right: &Path) -> bool {
    path_is_within(left, right) && path_is_within(right, left)
}

/// Keys under `Software` that nothing an app install does should change. Each decides what
/// runs at sign-in, what loads into other programs, or what the machine trusts. Compared
/// whole, per component, after dropping `WOW6432Node`, so both registry views are covered.
const REFUSED_SOFTWARE_KEYS: &[&str] = &[
    r"software\microsoft\windows nt\currentversion\image file execution options",
    r"software\microsoft\windows nt\currentversion\silentprocessexit",
    r"software\microsoft\windows nt\currentversion\winlogon",
    // AppInit_DLLs, Load, and Run.
    r"software\microsoft\windows nt\currentversion\windows",
    r"software\microsoft\windows nt\currentversion\aedebug",
    r"software\microsoft\windows nt\currentversion\svchost",
    // The Task Scheduler's own store. Tasks go through their own rules.
    r"software\microsoft\windows nt\currentversion\schedule",
    // Runs at sign-in on a terminal server, like the Run keys.
    r"software\microsoft\windows nt\currentversion\terminal server",
    r"software\microsoft\windows nt\currentversion\drivers32",
    r"software\microsoft\windows nt\currentversion\appcompatflags\custom",
    r"software\microsoft\windows nt\currentversion\appcompatflags\installedsdb",
    r"software\microsoft\windows\currentversion\policies",
    r"software\microsoft\windows\currentversion\authentication",
    r"software\microsoft\windows\currentversion\shellserviceobjectdelayload",
    r"software\microsoft\windows\currentversion\explorer\shellserviceobjects",
    // Where the Startup folder is. Pointing it at the app makes every file there run.
    r"software\microsoft\windows\currentversion\explorer\user shell folders",
    r"software\microsoft\windows\currentversion\explorer\shell folders",
    r"software\microsoft\windows\windows error reporting",
    r"software\microsoft\windows\currentversion\explorer\shellexecutehooks",
    r"software\microsoft\windows\currentversion\explorer\browser helper objects",
    r"software\microsoft\windows\currentversion\explorer\sharedtaskscheduler",
    r"software\microsoft\command processor",
    r"software\microsoft\netsh",
    r"software\microsoft\windows defender",
    r"software\microsoft\cryptography",
    r"software\microsoft\systemcertificates",
    r"software\microsoft\enterprisecertificates",
    r"software\policies",
];
/// File types that decide how programs and scripts start, for every file of that type.
const REFUSED_CLASSES: &[&str] = &[
    "exefile",
    "batfile",
    "cmdfile",
    "comfile",
    "piffile",
    "scrfile",
    "htafile",
    "lnkfile",
    "mscfile",
    "regfile",
    "jsfile",
    "jsefile",
    "vbsfile",
    "vbefile",
    "wsffile",
    "wshfile",
    "ms-settings",
    "msi.package",
    ".exe",
    ".bat",
    ".cmd",
    ".com",
    ".pif",
    ".scr",
    ".hta",
    ".lnk",
    ".msc",
    ".reg",
    ".msi",
    ".js",
    ".jse",
    ".vbs",
    ".vbe",
    ".wsf",
    ".wsh",
];
/// Values that name a debugger for every .NET program.
const NET_DEBUGGER_KEY: &str = r"software\microsoft\.netframework";
const NET_DEBUGGER_VALUES: &[&str] = &["dbgmanageddebugger", "dbgjitdebuglaunchsetting"];
const CLASSES_PREFIX: &str = r"software\classes\";
/// File types and protocols every program shares. An app can add a command to them, which
/// is how "Git Bash Here" works, so long as the command runs the app's own file. Changing
/// which command is the default changes what happens for everyone.
const SHARED_CLASSES: &[&str] = &[
    "*",
    "allfilesystemobjects",
    "desktopbackground",
    "directory",
    r"directory\background",
    "drive",
    "folder",
    "http",
    "https",
];
const OPEN_WITH_PREFIX: &str = "openwith";
/// `Folder\shell\open` decides what opening any folder runs.
const REFUSED_CLASS_VERBS: &[&str] = &[r"folder\shell\open"];
/// Keys whose values Windows runs at sign-in. A value is allowed when it runs the app's
/// own restored file.
const AUTOSTART_KEYS: &[&str] = &[
    r"software\microsoft\windows\currentversion\run",
    r"software\microsoft\windows\currentversion\runonce",
    r"software\microsoft\windows\currentversion\runonceex",
    r"software\microsoft\windows\currentversion\runservices",
    r"software\microsoft\windows\currentversion\runservicesonce",
];
const ACTIVE_SETUP_KEY: &str = r"software\microsoft\active setup\installed components";
const ACTIVE_SETUP_COMMAND: &str = "stubpath";
const APP_PATHS_KEY: &str = r"software\microsoft\windows\currentversion\app paths";
/// Under a CLSID, the keys that name what COM loads.
pub(crate) const COM_SERVER_KEYS: &[&str] = &[
    "inprocserver32",
    "localserver32",
    "inprochandler32",
    "treatas",
];
const CLSID_PREFIX: &str = r"software\classes\clsid\";
pub(crate) const SERVICES_PREFIX: &str = r"system\currentcontrolset\services\";
/// What a package may write for a service it restores. The rest of a service's key, such
/// as its `ImagePath` and account, is set through `sc.exe` and these rules.
const SERVICE_SUBKEYS: &[&str] = &["parameters", "instances"];
const SERVICE_DLL_VALUE: &str = "servicedll";
/// Per-user keys that hold more than settings. Environment variables go through their own
/// rules, which merge `Path` rather than replacing it.
const REFUSED_USER_KEYS: &[&str] = &[
    "environment",
    "volatile environment",
    r"control panel\desktop",
];
const ENVIRONMENT_KEY: &str = r"system\currentcontrolset\control\session manager\environment";
/// Variables Windows and other programs treat as telling them where to find themselves, or
/// what to load. A package has no business changing them.
const REFUSED_ENVIRONMENT_VARIABLES: &[&str] = &[
    "allusersprofile",
    "appdata",
    "commonprogramfiles",
    "commonprogramfiles(x86)",
    "commonprogramw6432",
    "comspec",
    "driverdata",
    "homedrive",
    "homepath",
    "localappdata",
    "os",
    "pathext",
    "programdata",
    "programfiles",
    "programfiles(x86)",
    "programw6432",
    "public",
    "systemdrive",
    "systemroot",
    "temp",
    "tmp",
    "userprofile",
    "windir",
    "__compat_layer",
    "__pslockdownpolicy",
    "dotnet_startup_hooks",
    "dotnet_additional_deps",
    "dotnet_shared_store",
    "java_tool_options",
    "_java_options",
    "jdk_java_options",
    "node_options",
    "perl5opt",
    "pythonstartup",
    "rubyopt",
];
/// A .NET profiler is a DLL loaded into every .NET program, and these others can load a
/// different runtime altogether.
const REFUSED_ENVIRONMENT_PREFIXES: &[&str] =
    &["cor_", "coreclr_", "complus_", "appdomain_manager_"];
const START_MENU_MARKER: &str = r"\start menu\";
const DESKTOP_MARKER: &str = r"\desktop\";
const STARTUP_TAIL: &str = r"programs\startup\";
const STARTUP_FOLDER: &str = r"start menu\programs\startup\";
/// Of [`REFUSED_DATA_FOLDERS`], what only matters for where data goes. A whole app can be in a
/// folder of that name anywhere else, including one that is this program.
const FINE_FOR_AN_INSTALL_LOCATION: &[&str] = &[r"desktop\", r"start menu\", r"winappmigrator\"];
/// The handler COM itself provides, which any class can name. Nothing about whose a class is
/// follows from it.
const COM_DEFAULT_HANDLER: &str = "ole32.dll";
const SHORTCUT_EXTENSION: &str = "lnk";
/// A real data location an app's user data has no business in, whichever base it is under.
const REFUSED_DATA_FOLDERS: &[&str] = &[
    r"microsoft\windows\",
    r"microsoft\windowsapps\",
    r"microsoft\windows defender\",
    r"microsoft\crypto\",
    r"microsoft\protect\",
    r"microsoft\credentials\",
    r"microsoft\systemcertificates\",
    // Office runs what is in these when it starts.
    r"microsoft\word\startup\",
    r"microsoft\excel\xlstart\",
    // On `PATH` on most PCs.
    r"oracle\java\javapath\",
    // In the Public profile, where a file would appear on everyone's desktop.
    r"desktop\",
    r"start menu\",
    // Where installs keep their records and what an uninstall trusts. A file a package put
    // there could be taken for a record of an install, and acted on with full rights.
    r"winappmigrator\",
];
/// Where Settings finds what it lists under installed apps.
const UNINSTALL_KEY: &str = r"software\microsoft\windows\currentversion\uninstall";
/// What a captured entry says about how to change or remove the app on the PC it came from.
/// None of it means anything here, and `WindowsInstaller` and `NoRemove` would stop Windows
/// from using the uninstaller this entry is given.
const SOURCE_UNINSTALL_VALUES: &[&str] = &[
    "uninstallstring",
    "quietuninstallstring",
    "modifypath",
    "uninstallpath",
    "windowsinstaller",
    "noremove",
    "nomodify",
    "norepair",
    // Hides it from the list, where the only way to uninstall it is.
    "systemcomponent",
];
const DISPLAY_ICON_VALUE: &str = "displayicon";
const UNC_PREFIX: &str = r"\\";
/// Windows' own scheduled tasks, which run with high rights and are expected to be there.
const WINDOWS_TASK_FOLDER: &str = r"\microsoft\windows\";
const SYSTEM_ACCOUNTS: &[&str] = &["s-1-5-18", "system", "localsystem"];

/// A key path to compare against the rules: lowercase, single separators, and no
/// `WOW6432Node`, which is the same key seen by a 32-bit program.
fn normalized_key(key_path: &str) -> String {
    key_path
        .split('\\')
        .filter(|part| !part.is_empty() && !part.eq_ignore_ascii_case("wow6432node"))
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("\\")
}

/// True when `key` is `parent` or inside it. Compared per component.
fn key_is_within(key: &str, parent: &str) -> bool {
    key.strip_prefix(parent)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
}

fn is_default_value(value_name: Option<&str>) -> bool {
    match value_name {
        None => true,
        Some(name) => name.is_empty() || name.eq_ignore_ascii_case("(Default)"),
    }
}

/// True for a value Windows runs by itself at sign-in or logon.
pub fn is_autostart_entry(entry: &RegistryEntry) -> bool {
    let key = normalized_key(&entry.key_path);
    if AUTOSTART_KEYS
        .iter()
        .any(|parent| key_is_within(&key, parent))
    {
        return entry.value_kind != Some(RegistryValueKind::None);
    }
    key_is_within(&key, ACTIVE_SETUP_KEY)
        && entry
            .value_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(ACTIVE_SETUP_COMMAND))
}

/// True when a key is an app's own entry in Settings' list of installed apps.
pub fn is_uninstall_entry_key(key_path: &str) -> bool {
    normalized_key(key_path)
        .strip_prefix(UNINSTALL_KEY)
        .and_then(|rest| rest.strip_prefix('\\'))
        .is_some_and(|name| !name.is_empty() && !name.contains('\\'))
}

/// True for a value that only meant something on the source: how to remove the app from
/// that PC. It is left out rather than refused, because the installer sets its own.
pub fn is_source_uninstall_value(entry: &RegistryEntry) -> bool {
    is_uninstall_entry_key(&entry.key_path)
        && entry.value_name.as_deref().is_some_and(|name| {
            let name = name.to_ascii_lowercase();
            SOURCE_UNINSTALL_VALUES.contains(&name.as_str())
                // Settings opens an icon just by showing it, so one on a network share would
                // hand that server the name and password hash of whoever looks at the list.
                || (name == DISPLAY_ICON_VALUE
                    && entry.value_data.as_deref().is_some_and(|data| {
                        data.trim_start_matches(['"', ' '])
                            .replace('/', "\\")
                            .starts_with(UNC_PREFIX)
                    }))
        })
}

/// True when `command` starts a file this package restores. A quoted path means exactly
/// that path. An unquoted one could end at any space, and each place it could end is a
/// file Windows might run, so it counts if the whole first word or more names one.
/// `cmd.exe /c C:\App\a.exe` does not count: it runs `cmd.exe`.
pub fn command_runs_restored_file(command: &str, map: &RestoreMap) -> bool {
    let trimmed = trim_icon_suffix(command.trim());
    if let Some(rest) = trimmed.strip_prefix('"') {
        return rest
            .split_once('"')
            .is_some_and(|(path, _)| map.is_restored_file(Path::new(path)));
    }
    let ends = trimmed
        .char_indices()
        .filter(|(_, character)| character.is_whitespace())
        .map(|(index, _)| index)
        .chain([trimmed.len()]);
    for end in ends {
        let candidate = &trimmed[..end];
        if map.is_restored_file(Path::new(candidate)) {
            return true;
        }
        // Past an `.exe`, the words are arguments. Without this `cmd.exe /c C:\App\a.exe`
        // would count, if anything could make the whole text a file name.
        if candidate.to_ascii_lowercase().ends_with(".exe") {
            return false;
        }
    }
    false
}

/// Why a registry value must not be restored, if so. `expand` turns `%VAR%` markers into
/// what they mean on this PC, and `restored_services` holds the lowercase names of the
/// services and drivers this package restores.
pub fn registry_refusal(
    entry: &RegistryEntry,
    map: &RestoreMap,
    restored_services: &HashSet<String>,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if entry.key_path.chars().any(char::is_control) {
        return Some("its key name holds control characters".to_owned());
    }
    // A link value would redirect the key to any other key on the machine.
    if entry.value_kind == Some(RegistryValueKind::Link) {
        return Some("it is a registry link".to_owned());
    }
    let key = normalized_key(&entry.key_path);
    let first = key.split('\\').next().unwrap_or_default();
    match entry.hive {
        RegistryHive::LocalMachine if first == "system" => {
            return system_key_refusal(&key, entry, map, restored_services, expand);
        }
        RegistryHive::LocalMachine if first != "software" => {
            return Some("no app install should write to this part of the registry".to_owned());
        }
        RegistryHive::CurrentUser
            if REFUSED_USER_KEYS
                .iter()
                .any(|refused| key_is_within(&key, refused)) =>
        {
            return Some("no app install should change this key".to_owned());
        }
        _ => {}
    }
    if REFUSED_SOFTWARE_KEYS
        .iter()
        .any(|refused| key_is_within(&key, refused))
    {
        return Some("no app install should change this key".to_owned());
    }
    if key == NET_DEBUGGER_KEY
        && entry
            .value_name
            .as_deref()
            .is_some_and(|name| NET_DEBUGGER_VALUES.contains(&name.to_ascii_lowercase().as_str()))
    {
        return Some("it would name a debugger for every .NET program".to_owned());
    }
    if let Some(class) = key.strip_prefix(CLASSES_PREFIX) {
        if class_key_redirects_programs(class) {
            return Some("it would change how a kind of program or folder is opened".to_owned());
        }
        if let Some(refusal) = shared_class_refusal(class, entry, map, expand) {
            return Some(refusal);
        }
    }
    if is_autostart_entry(entry) {
        return autostart_refusal(entry, map, expand);
    }
    if key_is_within(&key, APP_PATHS_KEY)
        && key.len() > APP_PATHS_KEY.len()
        && is_default_value(entry.value_name.as_deref())
        && entry.value_kind != Some(RegistryValueKind::None)
    {
        return autostart_refusal(entry, map, expand).map(|_| {
            "it would register a program name for a file this package does not install".to_owned()
        });
    }
    None
}

/// True for a key under `Software\Classes` that decides what starts a program, a script,
/// or a folder. `.bat\OpenWithProgids` only adds to a menu, and editors add themselves there,
/// so that stays allowed.
fn class_key_redirects_programs(class: &str) -> bool {
    if REFUSED_CLASS_VERBS
        .iter()
        .any(|refused| key_is_within(class, refused))
    {
        return true;
    }
    let (name, rest) = class.split_once('\\').unwrap_or((class, ""));
    REFUSED_CLASSES.contains(&name) && !rest.starts_with(OPEN_WITH_PREFIX)
}

/// A shared file type's default verb, and any command the app adds to one.
fn shared_class_refusal(
    class: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if !is_default_value(entry.value_name.as_deref())
        || entry.value_kind == Some(RegistryValueKind::None)
    {
        return None;
    }
    let shell = SHARED_CLASSES
        .iter()
        .find_map(|shared| class.strip_prefix(shared)?.strip_prefix(r"\shell"))?;
    if shell.is_empty() {
        return Some("it would change what opening every file or folder of a kind does".to_owned());
    }
    shell
        .strip_prefix('\\')?
        .strip_suffix(r"\command")
        .and_then(|_| autostart_refusal(entry, map, expand))
        .map(|_| "it adds a command that runs something this package does not install".to_owned())
}

/// What a registry value's text will be on the target. A command that runs at sign-in gets
/// its path quoted if it needs it, because this is where `C:\App\a.exe --x` becomes
/// `D:\My Apps\a.exe --x`, and Windows would try `D:\My.exe` first.
pub fn restored_value_text(entry: &RegistryEntry, map: &RestoreMap) -> String {
    let rewritten = map.rewrite(entry.value_data.as_deref().unwrap_or_default());
    if is_autostart_entry(entry) {
        quote_command_path(&rewritten, map)
    } else {
        rewritten
    }
}

/// What a registry value's raw data will be on the target, for what is not plain text. A
/// `REG_MULTI_SZ` is text too, and often a list of paths.
pub fn restored_value_bytes(
    entry: &RegistryEntry,
    map: &RestoreMap,
) -> std::result::Result<Vec<u8>, &'static str> {
    let bytes = hex_decode(entry.value_bytes_hex.as_deref().unwrap_or_default())?;
    Ok(
        if entry.value_kind == Some(RegistryValueKind::MultiString) {
            map.rewrite_multi_string(&bytes)
        } else {
            bytes
        },
    )
}

/// Quote the path of an unquoted command that runs a restored file, when the path has a
/// space in it. Anything else is returned as it was.
fn quote_command_path(command: &str, map: &RestoreMap) -> String {
    let trimmed = command.trim();
    if trimmed.starts_with('"') {
        return command.to_owned();
    }
    let ends = trimmed
        .char_indices()
        .filter(|(_, character)| character.is_whitespace())
        .map(|(index, _)| index)
        .chain([trimmed.len()]);
    for end in ends {
        let (path, rest) = trimmed.split_at(end);
        if map.is_restored_file(Path::new(path)) {
            return if path.contains(char::is_whitespace) {
                format!("\"{path}\"{rest}")
            } else {
                command.to_owned()
            };
        }
    }
    command.to_owned()
}

/// An autostart value must be text, and must run the app's own restored file.
fn autostart_refusal(
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if !matches!(
        entry.value_kind,
        Some(RegistryValueKind::String | RegistryValueKind::ExpandString)
    ) {
        return Some("it starts automatically, and its value is not a command".to_owned());
    }
    let command = expand(&map.rewrite(entry.value_data.as_deref().unwrap_or_default()));
    (!command_runs_restored_file(&command, map)).then(|| {
        format!(
            "it starts automatically, and runs something this package does not install: {command}"
        )
    })
}

/// Under `SYSTEM`, only what belongs to a service this package restores.
fn system_key_refusal(
    key: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    restored_services: &HashSet<String>,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if key_is_within(key, ENVIRONMENT_KEY) {
        return Some("environment variables are restored by their own rules".to_owned());
    }
    let Some(rest) = key.strip_prefix(SERVICES_PREFIX) else {
        return Some("no app install should change this key".to_owned());
    };
    let mut parts = rest.splitn(3, '\\');
    let service = parts.next().unwrap_or_default();
    let subkey = parts.next().unwrap_or_default();
    if !restored_services.contains(service) {
        return Some("it belongs to a service this package does not restore".to_owned());
    }
    if !SERVICE_SUBKEYS.contains(&subkey) {
        return Some("a service's own settings are only restored through its rules".to_owned());
    }
    let names_dll = entry
        .value_name
        .as_deref()
        .is_some_and(|name| name.eq_ignore_ascii_case(SERVICE_DLL_VALUE));
    if names_dll {
        let dll = expand(&map.rewrite(entry.value_data.as_deref().unwrap_or_default()));
        if !map.is_restored_file(Path::new(dll.trim().trim_matches('"'))) {
            return Some("its ServiceDll is not a file this package installs".to_owned());
        }
    }
    None
}

/// Why overwriting a COM server already registered on this PC must be refused, if so.
/// Registering a class of the app's own is normal, and its server may well be in Windows,
/// like `mscoree.dll` for .NET. Taking over a class that is already something else's
/// is how code gets loaded into Explorer, and `--force` does not make it right.
pub fn com_server_refusal(
    existing: &str,
    new_value: &str,
    target_root: &Path,
    windows_dir: &Path,
) -> Option<String> {
    let existing = existing.trim();
    if existing.is_empty()
        || existing.eq_ignore_ascii_case(new_value.trim())
        || existing.eq_ignore_ascii_case(COM_DEFAULT_HANDLER)
    {
        return None;
    }
    let server = parse_image_path(existing, windows_dir).path;
    (!path_is_strictly_inside(&server, target_root)).then(|| {
        format!(
            "this COM class already belongs to {existing}, which is outside the install location"
        )
    })
}

/// The key of the COM class a registry key is part of, if it is part of one, in lowercase and as
/// it is in the registry, since a 32-bit class is not the 64-bit one of that name.
pub fn com_class_key(key_path: &str) -> Option<String> {
    normalized_key(key_path).strip_prefix(CLSID_PREFIX)?;
    let lower = key_path.to_ascii_lowercase();
    let parts: Vec<&str> = lower.split('\\').filter(|part| !part.is_empty()).collect();
    let class = parts.iter().position(|part| *part == "clsid")? + 1;
    parts
        .get(class)
        .is_some()
        .then(|| parts[..=class].join("\\"))
}

/// True for a value naming what COM loads for a class.
pub fn is_com_server_entry(entry: &RegistryEntry) -> bool {
    let key = normalized_key(&entry.key_path);
    let Some(rest) = key.strip_prefix(CLSID_PREFIX) else {
        return false;
    };
    let mut parts = rest.split('\\');
    let _class = parts.next();
    parts
        .next()
        .is_some_and(|subkey| COM_SERVER_KEYS.contains(&subkey))
        && parts.next().is_none()
        && is_default_value(entry.value_name.as_deref())
}

/// Why an environment variable must not be restored, if so.
pub fn environment_refusal(name: &str) -> Option<String> {
    if name.is_empty() || name.contains('=') || name.chars().any(char::is_control) {
        return Some("its name is not a valid variable name".to_owned());
    }
    let lower = name.to_ascii_lowercase();
    (REFUSED_ENVIRONMENT_VARIABLES.contains(&lower.as_str())
        || REFUSED_ENVIRONMENT_PREFIXES
            .iter()
            .any(|prefix| lower.starts_with(prefix)))
    .then(|| "Windows and other programs rely on this variable".to_owned())
}

/// What a scheduled task's definition says it does.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskDefinition {
    /// The command of every program action, with XML escaping removed.
    pub commands: Vec<String>,
    /// An action that is not a program, such as a COM handler, which no path can vouch for.
    pub has_other_actions: bool,
    pub runs_as_system: bool,
    /// The account it says it runs as.
    pub account: Option<String>,
}

/// All of an element's text. A comment splits it, and the Task Scheduler reads straight
/// across, so stopping at the first piece would read something else.
fn node_text(node: roxmltree::Node<'_, '_>) -> String {
    node.children()
        .filter(roxmltree::Node::is_text)
        .filter_map(|child| child.text())
        .collect::<String>()
        .trim()
        .to_owned()
}

/// True for the names SYSTEM can be given in a task: by SID, or by name, alone or after any
/// authority (`NT AUTHORITY\\SYSTEM`, `.\\SYSTEM`, `PCNAME\\SYSTEM`).
fn is_system_account(account: &str) -> bool {
    let lower = account.to_ascii_lowercase();
    let name = lower.rsplit('\\').next().unwrap_or_default();
    SYSTEM_ACCOUNTS.contains(&name)
}

/// Read a task's definition. The XML is what Windows acts on, so this is what is checked,
/// and not the manifest's separate description of it.
pub fn read_task_definition(xml: &str) -> std::result::Result<TaskDefinition, String> {
    let document = roxmltree::Document::parse(xml).map_err(|error| error.to_string())?;
    let mut definition = TaskDefinition::default();
    for node in document.descendants().filter(roxmltree::Node::is_element) {
        match node.tag_name().name() {
            "Actions" => {
                for action in node.children().filter(roxmltree::Node::is_element) {
                    if action.tag_name().name() != "Exec" {
                        definition.has_other_actions = true;
                    }
                    definition.commands.extend(
                        action
                            .descendants()
                            .filter(|child| child.tag_name().name() == "Command")
                            .map(node_text),
                    );
                }
            }
            "UserId" | "GroupId" => {
                let account = node_text(node);
                definition.runs_as_system |= is_system_account(&account);
                if !account.is_empty() {
                    definition.account.get_or_insert(account);
                }
            }
            _ => {}
        }
    }
    Ok(definition)
}

/// Why a scheduled task must not be restored, if so. `xml` has already been rewritten for
/// the target.
pub fn task_refusal(
    task_path: &str,
    task_name: &str,
    definition: &TaskDefinition,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if task_name.is_empty()
        || task_name.chars().any(|character| {
            character.is_control()
                || matches!(
                    character,
                    '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'
                )
        })
    {
        return Some("its name is not one Windows accepts".to_owned());
    }
    if !task_path.starts_with('\\')
        || !task_path.ends_with('\\')
        || task_path.split('\\').any(|part| part == "..")
    {
        return Some("its folder is not one Windows accepts".to_owned());
    }
    if task_path
        .to_ascii_lowercase()
        .starts_with(WINDOWS_TASK_FOLDER)
    {
        return Some("it would replace one of Windows' own tasks".to_owned());
    }
    if definition.has_other_actions {
        return Some("it does something other than run a program".to_owned());
    }
    if definition.commands.is_empty() {
        return Some("it names no program to run".to_owned());
    }
    definition.commands.iter().find_map(|command| {
        let expanded = expand(command);
        let path = expanded.trim().trim_matches('"');
        (!map.is_restored_file(Path::new(path)))
            .then(|| format!("it runs something this package does not install: {path}"))
    })
}

/// Why a task already on this PC must not be replaced, if so: one of its programs is
/// outside the install location. `--force` does not change this.
pub fn existing_task_refusal(existing_xml: &str, target_root: &Path) -> Option<String> {
    let Ok(definition) = read_task_definition(existing_xml) else {
        return Some("a task with this name already exists and could not be read".to_owned());
    };
    if definition.has_other_actions || definition.commands.is_empty() {
        return Some("a task with this name already exists and does not run the app".to_owned());
    }
    definition.commands.iter().find_map(|command| {
        let path = command.trim().trim_matches('"');
        (!path_is_strictly_inside(Path::new(path), target_root)).then(|| {
            format!(
                "a task with this name already exists and runs {path}, outside the install location"
            )
        })
    })
}

/// The folders shortcuts may be put in, on this PC.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnownFolders {
    pub user_desktop: Option<PathBuf>,
    pub common_desktop: Option<PathBuf>,
    pub user_start_menu: Option<PathBuf>,
    pub common_start_menu: Option<PathBuf>,
}

/// Where a shortcut goes on this PC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutPlacement {
    /// The known folder it goes under, which links are checked below.
    pub root: PathBuf,
    pub destination: PathBuf,
    /// In a Startup folder, so Windows runs it at sign-in.
    pub autostart: bool,
    /// In a folder shared by everyone on the PC.
    pub for_all_users: bool,
}

/// Work out where a shortcut goes. The source's path says which kind of place it was: this
/// PC's own folder for that kind is used, and nothing else. So a shortcut can only land in a
/// Start Menu or Desktop, and `C:\Users\alice\Desktop` does not get created on a PC with
/// no alice.
pub fn shortcut_placement(
    original: &Path,
    folders: &KnownFolders,
) -> std::result::Result<ShortcutPlacement, String> {
    if !path_has_extension(original, SHORTCUT_EXTENSION) {
        return Err("it is not a .lnk shortcut".to_owned());
    }
    let text = original.to_string_lossy().replace('/', "\\");
    let lower = text.to_ascii_lowercase();
    let in_start_menu = lower.find(START_MENU_MARKER);
    let on_desktop = lower.find(DESKTOP_MARKER);
    let (marker_at, marker_len, is_start_menu) = match (in_start_menu, on_desktop) {
        (Some(position), _) => (position, START_MENU_MARKER.len(), true),
        (None, Some(position)) => (position, DESKTOP_MARKER.len(), false),
        (None, None) => return Err("it is not in a Start Menu or on a Desktop".to_owned()),
    };
    let tail = safe_relative_path(Path::new(&text[marker_at + marker_len..]));
    if has_reserved_device_name(&tail) || has_short_name_alias(&tail) {
        return Err("its path uses a name that cannot be checked".to_owned());
    }
    let for_all_users = if is_start_menu {
        lower[..marker_at].contains(r"\programdata\")
    } else {
        lower[..marker_at].ends_with(r"\users\public")
    };
    let root = match (is_start_menu, for_all_users) {
        (true, true) => folders.common_start_menu.as_ref(),
        (true, false) => folders.user_start_menu.as_ref(),
        (false, true) => folders.common_desktop.as_ref(),
        (false, false) => folders.user_desktop.as_ref(),
    }
    .ok_or_else(|| "this PC has no such folder for it".to_owned())?;
    let tail_text = tail.to_string_lossy().to_ascii_lowercase();
    Ok(ShortcutPlacement {
        root: root.clone(),
        destination: root.join(&tail),
        autostart: is_start_menu && tail_text.starts_with(STARTUP_TAIL),
        for_all_users,
    })
}

/// True for a folder in `path` written the way Windows abbreviates long names, as
/// `STARTU~1` for `Startup`, which would pass the rules for the long name. A file can
/// have such a name: it is only folders that the rules look at.
fn has_short_name_alias(path: &Path) -> bool {
    let Some(folders) = path.parent() else {
        return false;
    };
    folders.components().any(|component| {
        let Component::Normal(name) = component else {
            return false;
        };
        let name = name.to_string_lossy();
        let stem = name.split('.').next().unwrap_or_default();
        stem.rsplit_once('~').is_some_and(|(base, digits)| {
            !base.is_empty()
                && base.len() <= 6
                && !digits.is_empty()
                && digits.chars().all(|character| character.is_ascii_digit())
        })
    })
}

/// Why a file must not go to its real data location, if so. Everything here is under a user
/// data base such as `%APPDATA%`, by the path after that base. A Startup folder there is
/// autostart in everything but name, so a copied file would run at sign-in. Under the install
/// location the same relative path is harmless, and this does not apply.
pub fn data_location_refusal(tail: &Path) -> Option<String> {
    if has_short_name_alias(tail) {
        return Some("its path uses a name that cannot be checked".to_owned());
    }
    let lower = format!(
        "{}\\",
        tail.to_string_lossy()
            .to_ascii_lowercase()
            .replace('/', "\\")
    );
    if lower.contains(STARTUP_FOLDER) {
        return Some("it would go in a Startup folder, and run at every sign-in".to_owned());
    }
    REFUSED_DATA_FOLDERS
        .iter()
        .any(|folder| lower.starts_with(folder))
        .then(|| "it would go in a folder that belongs to Windows".to_owned())
}

/// Where a license file or store goes. Under the install location when its path maps there,
/// and otherwise only to a known data folder on this PC, by the part of the path after that
/// folder. `base_of` finds this PC's folder for a source path. Any other absolute path a
/// manifest names is refused rather than copied to.
pub fn license_destination(
    original: &Path,
    map: &RestoreMap,
    restore_user_data: bool,
    real_destination: &dyn Fn(&Path) -> Option<(PathBuf, PathBuf)>,
) -> std::result::Result<PathBuf, String> {
    let real = || {
        real_destination(original).map(|(destination, tail)| match data_location_refusal(&tail) {
            Some(refusal) => Err(refusal),
            None => Ok(destination),
        })
    };
    if restore_user_data && let Some(result) = real() {
        return result;
    }
    let rewritten = PathBuf::from(map.rewrite(&original.to_string_lossy()));
    if path_is_strictly_inside(&rewritten, map.target_root()) {
        return Ok(rewritten);
    }
    real().unwrap_or_else(|| {
        Err("it is outside the install location and every known data folder".to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MatchConfidence;

    const WINDOWS: &str = r"C:\Windows";

    #[test]
    fn what_a_source_said_about_removing_an_app_is_left_out() {
        let value = |key: &str, name: &str| RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: key.to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some("x".to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        };
        let entry = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Acme";
        let wow = r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\{A-B}";
        for name in [
            "UninstallString",
            "QuietUninstallString",
            "ModifyPath",
            "UninstallPath",
            "WindowsInstaller",
            "NoRemove",
            "SystemComponent",
        ] {
            assert!(is_source_uninstall_value(&value(entry, name)), "{name}");
            assert!(is_source_uninstall_value(&value(wow, name)), "{name}");
        }
        let mut icon = value(entry, "DisplayIcon");
        for network in [
            r"\\server\share\app.ico",
            r"//server/share/app.ico",
            r#""\\server\share\app.ico",0"#,
            r" \\server\share\app.ico",
        ] {
            icon.value_data = Some(network.to_owned());
            assert!(is_source_uninstall_value(&icon), "{network}");
        }
        icon.value_data = Some(r"C:\Program Files\Acme\app.exe,0".to_owned());
        assert!(!is_source_uninstall_value(&icon));
        for (key, name) in [
            (entry, "DisplayName"),
            (entry, "InstallLocation"),
            // Not an entry, and not the same value.
            (
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
                "UninstallString",
            ),
            (r"SOFTWARE\Acme", "UninstallString"),
            (
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Acme\Sub",
                "UninstallString",
            ),
            (
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\UninstallOther\Acme",
                "UninstallString",
            ),
        ] {
            assert!(
                !is_source_uninstall_value(&value(key, name)),
                "{key} {name}"
            );
        }
    }

    #[test]
    fn an_install_location_windows_would_read_as_something_else_is_refused() {
        for spelled in [
            r"\\?\C:\Apps\Acme",
            r"\\.\C:\Apps\Acme",
            r"\\?\UNC\server\share\Acme",
            r"\\server\share\Acme",
            r"\\localhost\C$\Windows\Acme",
            r"C:\Windows.\Acme",
            r"C:\Windows \Acme",
            r"C:\Apps\Acme:stream",
            r"C:\Apps\CON\Acme",
        ] {
            let refused = install_location_refusal(Path::new(spelled), Path::new(WINDOWS), &[]);
            assert!(refused.is_some(), "{spelled}");
        }
        for spelled in [
            r"C:\.\Windows\Acme",
            r"C:\\Windows\Acme",
            r"C:/Windows/Acme",
        ] {
            let refused = install_location_refusal(Path::new(spelled), Path::new(WINDOWS), &[]);
            assert_eq!(
                refused.as_deref(),
                Some("it cannot be the Windows folder"),
                "{spelled}"
            );
        }
        assert_eq!(
            install_location_refusal(Path::new(r"C:\Apps\Acme"), Path::new(WINDOWS), &[]),
            None
        );
        assert_eq!(
            install_location_refusal(Path::new(r"C:\Apps\Acme 2.0"), Path::new(WINDOWS), &[]),
            None
        );
    }

    #[test]
    fn a_class_is_the_key_it_is_in_the_registry() {
        const CLASS: &str = r"software\classes\clsid\{a}";
        assert_eq!(
            com_class_key(r"SOFTWARE\Classes\CLSID\{A}\InprocServer32").as_deref(),
            Some(CLASS)
        );
        assert_eq!(
            com_class_key(r"SOFTWARE\Classes\CLSID\{A}").as_deref(),
            Some(CLASS)
        );
        assert_eq!(
            com_class_key(r"SOFTWARE\WOW6432Node\Classes\CLSID\{A}\ProgID").as_deref(),
            Some(r"software\wow6432node\classes\clsid\{a}")
        );
        assert_eq!(com_class_key(r"SOFTWARE\Classes\CLSID"), None);
        assert_eq!(com_class_key(r"SOFTWARE\Acme\CLSID\{A}"), None);
    }

    #[test]
    fn what_com_gives_every_class_says_nothing_about_whose_it_is() {
        let target = Path::new(r"C:\Apps\Acme");
        for own in ["ole32.dll", "OLE32.DLL", " ole32.dll "] {
            assert_eq!(
                com_server_refusal(own, r"C:\Apps\Acme\a.dll", target, Path::new(WINDOWS)),
                None
            );
        }
        assert!(
            com_server_refusal(
                r"C:\Other\b.dll",
                r"C:\Apps\Acme\a.dll",
                target,
                Path::new(WINDOWS)
            )
            .is_some()
        );
    }

    #[test]
    fn a_folder_whose_contents_run_by_themselves_is_not_an_install_location() {
        let windows = Path::new(WINDOWS);
        for bad in [
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup",
            r"C:\Users\me\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\Acme",
            r"C:\Users\me\AppData\Local\Microsoft\WindowsApps",
            r"C:\Users\me\AppData\Roaming\Microsoft\Word\STARTUP\Acme",
            r"C:\Users\me\AppData\Roaming\Microsoft\Excel\XLSTART",
            r"C:\Program Files\Common Files\Oracle\Java\javapath",
        ] {
            assert!(
                install_location_refusal(Path::new(bad), windows, &[]).is_some(),
                "{bad}"
            );
        }
        for good in [
            r"C:\Users\me\AppData\Local\Microsoft\OneDrive",
            r"C:\Users\me\AppData\Local\Programs\Acme",
            r"C:\Program Files\WinAppMigrator",
            r"D:\Desktop\Acme",
            r"C:\Program Files\Start Menu Fixer",
        ] {
            assert_eq!(
                install_location_refusal(Path::new(good), windows, &[]),
                None,
                "{good}"
            );
        }
    }

    #[test]
    fn nothing_a_package_installs_goes_where_installs_are_recorded() {
        for tail in [
            r"WinAppMigrator\installs\wam-evil\journal.jsonl",
            r"winappmigrator\installs\wam-evil\backups\00000001.moved",
            r"WinAppMigrator\busy.lock",
        ] {
            assert!(data_location_refusal(Path::new(tail)).is_some(), "{tail}");
        }
        assert!(data_location_refusal(Path::new(r"WinAppMigrator Extras\settings.json")).is_none());
    }

    #[test]
    fn ordinary_service_names_are_accepted() {
        for name in [
            "AcmeSync",
            "MSSQL$SQLEXPRESS",
            "Apache2.4",
            "Steam Client Service",
            "Intel(R) SUR QC SAM",
            "{B5F3A1D2-0000-4000-8000-000000000000}",
            "vmware-converter-agent",
            "NVDisplay.ContainerLocalSystem",
            "Überwachung",
        ] {
            assert!(is_valid_service_name(name), "{name}");
        }
    }

    #[test]
    fn names_that_change_a_command_or_a_key_are_refused() {
        let too_long = "x".repeat(MAX_SERVICE_NAME_CHARS + 1);
        for name in [
            "",
            " ",
            " padded",
            "padded ",
            r"..\Other",
            "a/b",
            "start=",
            "-flag",
            "a\"b",
            "line\nbreak",
            too_long.as_str(),
        ] {
            assert!(!is_valid_service_name(name), "{name:?}");
        }
    }

    #[test]
    fn built_in_accounts_are_recognized_in_every_spelling() {
        for recorded in ["LocalSystem", r".\LocalSystem", r"NT AUTHORITY\SYSTEM"] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::LocalSystem,
                "{recorded}"
            );
        }
        for recorded in [r"NT AUTHORITY\LocalService", r"nt authority\LOCAL SERVICE"] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::LocalService,
                "{recorded}"
            );
        }
        for recorded in [
            r"NT AUTHORITY\NetworkService",
            r"NT AUTHORITY\Network Service",
        ] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::NetworkService,
                "{recorded}"
            );
        }
        assert_eq!(
            ServiceAccount::from_recorded(None, "App"),
            ServiceAccount::Unspecified
        );
    }

    #[test]
    fn low_rights_accounts_are_kept_and_not_turned_into_local_system() {
        let local = ServiceAccount::from_recorded(Some(r"NT AUTHORITY\LocalService"), "App");
        assert_eq!(local.sc_object_name(), Some(r"NT AUTHORITY\LocalService"));
        assert!(local.sc_takes_empty_password());

        let network = ServiceAccount::from_recorded(Some(r"NT AUTHORITY\NETWORK SERVICE"), "App");
        assert_eq!(
            network.sc_object_name(),
            Some(r"NT AUTHORITY\NetworkService")
        );

        let system = ServiceAccount::from_recorded(Some("LocalSystem"), "App");
        assert_eq!(system.sc_object_name(), Some("LocalSystem"));
        assert!(!system.sc_takes_empty_password());
    }

    #[test]
    fn a_virtual_account_is_only_the_services_own() {
        let own = ServiceAccount::from_recorded(Some(r"NT SERVICE\AcmeSync"), "AcmeSync");
        assert_eq!(own.sc_object_name(), Some(r"NT SERVICE\AcmeSync"));
        assert!(!own.sc_takes_empty_password());

        let other = ServiceAccount::from_recorded(Some(r"NT SERVICE\TrustedInstaller"), "AcmeSync");
        assert!(matches!(other, ServiceAccount::Custom(_)));
    }

    #[test]
    fn a_custom_account_loses_rights_rather_than_gaining_them() {
        let custom = ServiceAccount::from_recorded(Some(r"CONTOSO\svc-acme"), "AcmeSync");
        assert_eq!(
            custom,
            ServiceAccount::Custom(r"CONTOSO\svc-acme".to_owned())
        );
        assert_eq!(custom.sc_object_name(), Some(r"NT AUTHORITY\LocalService"));
        assert!(custom.label().contains("not restored"));
    }

    #[test]
    fn the_stored_image_path_is_quoted_and_keeps_its_arguments() {
        assert_eq!(
            service_image_path_value(Path::new(r"C:\My Apps\Foo Bar\svc.exe"), None),
            r#""C:\My Apps\Foo Bar\svc.exe""#
        );
        assert_eq!(
            service_image_path_value(
                Path::new(r"C:\My Apps\Foo Bar\svc.exe"),
                Some(r#"--config "C:\My Apps\Foo Bar\app.conf" -v"#)
            ),
            r#""C:\My Apps\Foo Bar\svc.exe" --config "C:\My Apps\Foo Bar\app.conf" -v"#
        );
        assert_eq!(
            service_image_path_value(Path::new(r"C:\App\svc.exe"), Some("  ")),
            r#""C:\App\svc.exe""#
        );
    }

    #[test]
    fn an_install_location_cannot_be_where_everything_else_lives() {
        let windows = Path::new(WINDOWS);
        let shared = [
            PathBuf::from(r"C:\Program Files"),
            PathBuf::from(r"C:\ProgramData"),
            PathBuf::from(r"C:\Users\bob"),
        ];
        for bad in [
            r"C:\",
            r"D:\",
            r"C:\Windows",
            r"c:\windows\system32",
            r"C:\Windows\App",
            r"C:\Program Files",
            r"C:\program files\",
            r"C:\ProgramData",
            r"C:\Users\bob",
            r"C:\Program Files\..\Windows",
            "relative",
        ] {
            assert!(
                install_location_refusal(Path::new(bad), windows, &shared).is_some(),
                "{bad}"
            );
        }
        for good in [
            r"C:\Program Files\Acme",
            r"C:\Program Files\Vendor\Acme",
            r"C:\CocosDashboard",
            r"D:\Apps\Acme",
            r"C:\Users\bob\AppData\Local\Programs\Acme",
            r"C:\WindowsApps2\Acme",
        ] {
            assert_eq!(
                install_location_refusal(Path::new(good), windows, &shared),
                None,
                "{good}"
            );
        }
    }

    #[test]
    fn a_service_outside_the_install_location_is_never_taken_over() {
        let target = Path::new(r"C:\Program Files\Acme");
        let windows = Path::new(WINDOWS);
        for current in [
            r#""C:\ProgramData\Microsoft\Windows Defender\Platform\4.18\MsMpEng.exe""#,
            r"%SystemRoot%\System32\svchost.exe -k netsvcs",
            r"C:\Program Files\Acme Other\svc.exe",
            r"C:\Program Files\Acme\..\Other\svc.exe",
            "",
        ] {
            let refusal = existing_service_refusal("WinDefend", current, target, windows);
            assert!(refusal.is_some(), "{current}");
        }
    }

    #[test]
    fn an_earlier_install_in_the_same_place_can_be_replaced() {
        let target = Path::new(r"C:\Program Files\Acme");
        let windows = Path::new(WINDOWS);
        for current in [
            r#""C:\Program Files\Acme\svc.exe" --run"#,
            r"C:\Program Files\Acme\bin\svc.exe",
            r"c:\program files\acme\SVC.EXE",
        ] {
            assert_eq!(
                existing_service_refusal("AcmeSync", current, target, windows),
                None,
                "{current}"
            );
        }
    }

    #[test]
    fn checking_an_image_path_compares_the_binary_and_arguments_exactly() {
        let windows = Path::new(WINDOWS);
        let binary = Path::new(r"C:\Program Files\Git\svc.exe");
        assert!(image_path_matches(
            r#""C:\Program Files\Git\svc.exe" --run"#,
            binary,
            Some("--run"),
            windows
        ));
        assert!(image_path_matches(
            r#""c:\program files\git\SVC.exe""#,
            binary,
            None,
            windows
        ));
        // A path that merely contains the expected one used to pass.
        assert!(!image_path_matches(
            r#""D:\Evil\C:\Program Files\Git\svc.exe""#,
            binary,
            None,
            windows
        ));
        assert!(!image_path_matches(
            r"C:\Program Files\Git\svc.exe.bak",
            binary,
            None,
            windows
        ));
        assert!(!image_path_matches(
            r#""C:\Program Files\Git\svc.exe" --other"#,
            binary,
            Some("--run"),
            windows
        ));
    }

    fn app_map() -> RestoreMap {
        let mut map = RestoreMap::for_tests(r"D:\Apps\Acme");
        map.add_test_root(r"C:\Program Files\Acme", r"D:\Apps\Acme");
        map.add_test_file(r"C:\Program Files\Acme\acme.exe", r"D:\Apps\Acme\acme.exe");
        map.add_test_file(
            r"C:\Program Files\Acme\My Tools\run.cmd",
            r"D:\Apps\Acme\My Tools\run.cmd",
        );
        map.add_test_file(
            r"C:\Program Files\Acme\bin\host.dll",
            r"D:\Apps\Acme\bin\host.dll",
        );
        map
    }

    fn entry(hive: RegistryHive, key: &str, name: Option<&str>, data: &str) -> RegistryEntry {
        RegistryEntry {
            hive,
            key_path: key.to_owned(),
            value_name: name.map(ToOwned::to_owned),
            value_data: Some(data.to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: false,
        }
    }

    fn refusal(entry: &RegistryEntry) -> Option<String> {
        let services: HashSet<String> = ["acmesync".to_owned()].into_iter().collect();
        registry_refusal(entry, &app_map(), &services, &|text| text.to_owned())
    }

    #[test]
    fn keys_no_app_install_should_write_are_refused_in_every_form() {
        use RegistryHive::{CurrentUser, LocalMachine};
        for (hive, key) in [
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\sethc.exe",
            ),
            (
                LocalMachine,
                r"Software\WOW6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\sethc.exe",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            ),
            (
                CurrentUser,
                r"Software\Microsoft\Windows NT\CurrentVersion\Winlogon",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
            ),
            (
                CurrentUser,
                r"software\microsoft\windows nt\currentversion\WINDOWS",
            ),
            (
                LocalMachine,
                r"SOFTWARE\\Microsoft\Windows NT\\CurrentVersion\\Winlogon\",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SilentProcessExit\x.exe",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Svchost",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\SystemCertificates\Root\Certificates\AB",
            ),
            (CurrentUser, r"Software\Microsoft\Command Processor"),
            (
                LocalMachine,
                r"SOFTWARE\Policies\Microsoft\Windows Defender",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\Run",
            ),
            (LocalMachine, r"SYSTEM\CurrentControlSet\Control\Lsa"),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\SecurityProviders\WDigest",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\Session Manager",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\KnownDLLs",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
            ),
            (LocalMachine, r"SYSTEM\ControlSet001\Control\Lsa"),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Terminal Server\Install\Software\Microsoft\Windows\CurrentVersion\Run",
            ),
            (
                CurrentUser,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellServiceObjects\{A}",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting\Hangs",
            ),
            (LocalMachine, r"SECURITY\Policy"),
            (LocalMachine, r"SAM\SAM"),
            (LocalMachine, r"BCD00000000\Objects"),
            (CurrentUser, "Environment"),
            (CurrentUser, r"Control Panel\Desktop"),
        ] {
            assert!(
                refusal(&entry(hive, key, Some("x"), "y")).is_some(),
                "{key} should be refused"
            );
        }
    }

    #[test]
    fn an_apps_own_keys_are_left_alone() {
        use RegistryHive::{CurrentUser, LocalMachine};
        for (hive, key) in [
            (LocalMachine, r"SOFTWARE\Acme\Sync"),
            (LocalMachine, r"SOFTWARE\WOW6432Node\Acme\Sync"),
            (CurrentUser, r"Software\Acme"),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Acme",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows Search\Acme",
            ),
            (
                LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\WinlogonNotes",
            ),
            (LocalMachine, r"SOFTWARE\Microsoft\VisualStudio\Acme"),
            (LocalMachine, r"SOFTWARE\Classes\.acme"),
            (
                LocalMachine,
                r"SOFTWARE\Classes\Acme.Document\shell\open\command",
            ),
            (LocalMachine, r"SOFTWARE\Classes\.exefile2"),
            (
                CurrentUser,
                r"Software\Classes\Directory\shell\acme\command",
            ),
            (CurrentUser, r"Software\Classes\.bat\OpenWithProgids"),
            (CurrentUser, r"Software\Classes\.js\OpenWithProgids"),
            (
                LocalMachine,
                r"SOFTWARE\Classes\CLSID\{12345678-0000-0000-0000-000000000000}\InprocServer32",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AcmeSync\Parameters",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\acmesync\Parameters\Sub",
            ),
            (
                LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AcmeSync\Instances\Default",
            ),
            (CurrentUser, r"AppEvents\Schemes\Apps\Acme"),
        ] {
            assert_eq!(
                refusal(&entry(hive, key, Some("x"), "y")),
                None,
                "{key} should be allowed"
            );
        }
    }

    #[test]
    fn a_shared_file_type_can_gain_the_apps_command_but_nothing_else() {
        let verb = |class: &str, data: &str| {
            entry(
                RegistryHive::CurrentUser,
                &format!(r"Software\Classes\{class}\shell\acme\command"),
                None,
                data,
            )
        };
        let own = r#""C:\Program Files\Acme\acme.exe" "%V""#;
        for class in [
            "Directory",
            r"Directory\Background",
            "*",
            "Folder",
            "Drive",
            "http",
        ] {
            assert_eq!(refusal(&verb(class, own)), None, "{class}");
            assert!(
                refusal(&verb(class, r"C:\Windows\System32\cmd.exe /c calc")).is_some(),
                "{class}"
            );
        }
        // A file type of the app's own can open with whatever it likes.
        assert_eq!(refusal(&verb("Acme.Document", "notepad.exe %1")), None);

        let default_verb = entry(
            RegistryHive::CurrentUser,
            r"Software\Classes\Folder\shell",
            None,
            "evil",
        );
        assert!(refusal(&default_verb).is_some());
    }

    #[test]
    fn a_debugger_for_every_net_program_is_refused_and_other_net_settings_are_not() {
        let key = r"SOFTWARE\Microsoft\.NETFramework";
        let debugger = entry(
            RegistryHive::LocalMachine,
            key,
            Some("DbgManagedDebugger"),
            "x",
        );
        let other = entry(RegistryHive::LocalMachine, key, Some("InstallRoot"), "x");
        assert!(refusal(&debugger).is_some());
        assert_eq!(refusal(&other), None);
    }

    #[test]
    fn a_service_key_is_only_open_for_a_restored_service_and_its_settings() {
        use RegistryHive::LocalMachine;
        for key in [
            r"SYSTEM\CurrentControlSet\Services\WinDefend\Parameters",
            r"SYSTEM\CurrentControlSet\Services\AcmeSync",
            r"SYSTEM\CurrentControlSet\Services\AcmeSync\Security",
            r"SYSTEM\CurrentControlSet\Services\AcmeSyncOther\Parameters",
        ] {
            assert!(
                refusal(&entry(LocalMachine, key, Some("x"), "y")).is_some(),
                "{key}"
            );
        }
    }

    #[test]
    fn a_service_dll_must_be_a_restored_file() {
        let key = r"SYSTEM\CurrentControlSet\Services\AcmeSync\Parameters";
        let dll = |data| entry(RegistryHive::LocalMachine, key, Some("ServiceDll"), data);
        assert_eq!(refusal(&dll(r"C:\Program Files\Acme\bin\host.dll")), None);
        assert!(refusal(&dll(r"C:\Windows\System32\evil.dll")).is_some());
        assert!(refusal(&dll(r"C:\Program Files\Acme\bin\missing.dll")).is_some());
    }

    #[test]
    fn program_file_types_cannot_be_redirected() {
        let key_of = |class: &str| format!(r"Software\Classes\{class}");
        for class in [
            r"exefile\shell\open\command",
            r".exe",
            r".lnk",
            r"batfile\shell\open\command",
            r"ms-settings\shell\open\command",
            r"mscfile\shell\open\command",
            r"Folder\shell\open\command",
            r".bat",
            r".js\shell\open",
        ] {
            let found = entry(RegistryHive::CurrentUser, &key_of(class), None, "x");
            assert!(refusal(&found).is_some(), "{class}");
        }
    }

    #[test]
    fn a_run_value_needs_to_run_the_apps_own_file() {
        for key in [
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce",
            r"Software\Microsoft\Windows\CurrentVersion\RunOnceEx\0001",
        ] {
            let run = |data| entry(RegistryHive::LocalMachine, key, Some("Acme"), data);
            // Rewritten for the target first, then checked.
            assert_eq!(
                refusal(&run(r"C:\Program Files\Acme\acme.exe")),
                None,
                "{key}"
            );
            assert_eq!(
                refusal(&run(r#""C:\Program Files\Acme\acme.exe" --tray"#)),
                None
            );
            assert_eq!(
                refusal(&run(r"C:\Program Files\Acme\acme.exe --tray")),
                None
            );
            assert_eq!(
                refusal(&run(r"C:\Program Files\Acme\My Tools\run.cmd")),
                None
            );

            for bad in [
                r"C:\Windows\System32\cmd.exe /c C:\Program Files\Acme\acme.exe",
                r#"rundll32.exe "C:\Program Files\Acme\bin\host.dll",Start"#,
                r"C:\Program Files\Acme\missing.exe",
                r"powershell -enc AAAA",
                r#""C:\Program Files\Acme\acme.exe"#,
                r"",
            ] {
                assert!(refusal(&run(bad)).is_some(), "{bad}");
            }
        }
    }

    #[test]
    fn a_command_that_runs_at_sign_in_is_quoted_when_its_new_path_has_a_space() {
        let mut map = RestoreMap::for_tests(r"D:\My Apps");
        map.add_test_root(r"C:\App", r"D:\My Apps");
        map.add_test_file(r"C:\App\a.exe", r"D:\My Apps\a.exe");
        let run = |data: &str| {
            entry(
                RegistryHive::CurrentUser,
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                Some("Acme"),
                data,
            )
        };
        assert_eq!(
            restored_value_text(&run(r"C:\App\a.exe --x"), &map),
            r#""D:\My Apps\a.exe" --x"#
        );
        assert_eq!(
            restored_value_text(&run(r#""C:\App\a.exe" --x"#), &map),
            r#""D:\My Apps\a.exe" --x"#
        );

        let settings = entry(
            RegistryHive::CurrentUser,
            r"Software\Acme",
            Some("Cmd"),
            r"C:\App\a.exe --x",
        );
        assert_eq!(
            restored_value_text(&settings, &map),
            r"D:\My Apps\a.exe --x",
            "only a command Windows runs by itself is changed"
        );

        let mut no_space = RestoreMap::for_tests(r"D:\Apps");
        no_space.add_test_root(r"C:\App", r"D:\Apps");
        no_space.add_test_file(r"C:\App\a.exe", r"D:\Apps\a.exe");
        assert_eq!(
            restored_value_text(&run(r"C:\App\a.exe --x"), &no_space),
            r"D:\Apps\a.exe --x"
        );
    }

    #[test]
    fn only_a_multi_string_value_has_paths_in_its_raw_data_rewritten() {
        let mut map = RestoreMap::for_tests(r"D:\Apps");
        map.add_test_root(r"C:\App", r"D:\Apps");
        let hex = |text: &str| {
            text.encode_utf16()
                .flat_map(u16::to_le_bytes)
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        };
        let source = hex("C:\\App\\a\0\0");
        let mut value = entry(
            RegistryHive::CurrentUser,
            r"Software\Acme",
            Some("Dirs"),
            "",
        );
        value.value_bytes_hex = Some(source.clone());

        value.value_kind = Some(RegistryValueKind::MultiString);
        assert_eq!(
            restored_value_bytes(&value, &map),
            hex_decode(&hex("D:\\Apps\\a\0\0"))
        );
        value.value_kind = Some(RegistryValueKind::Binary);
        assert_eq!(restored_value_bytes(&value, &map), hex_decode(&source));
    }

    #[test]
    fn a_run_value_that_is_not_text_is_refused_and_an_empty_key_is_fine() {
        let key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
        let mut binary = entry(RegistryHive::LocalMachine, key, Some("Acme"), "");
        binary.value_kind = Some(RegistryValueKind::Binary);
        assert!(refusal(&binary).is_some());

        let mut key_only = entry(RegistryHive::LocalMachine, key, None, "");
        key_only.value_kind = Some(RegistryValueKind::None);
        assert_eq!(refusal(&key_only), None);
    }

    #[test]
    fn an_environment_marker_in_a_run_value_is_expanded_before_checking() {
        let services = HashSet::new();
        let key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
        let run = entry(
            RegistryHive::CurrentUser,
            key,
            Some("Acme"),
            r"%ACME%\acme.exe",
        );
        let expand = |text: &str| text.replace("%ACME%", r"D:\Apps\Acme");
        assert_eq!(registry_refusal(&run, &app_map(), &services, &expand), None);
        assert!(registry_refusal(&run, &app_map(), &services, &|text| text.to_owned()).is_some());
    }

    #[test]
    fn active_setup_only_matters_for_the_value_it_runs() {
        let key = r"SOFTWARE\Microsoft\Active Setup\Installed Components\{ACME}";
        let stub = |data| entry(RegistryHive::LocalMachine, key, Some("StubPath"), data);
        assert!(is_autostart_entry(&stub("x")));
        assert_eq!(
            refusal(&stub(r"C:\Program Files\Acme\acme.exe /setup")),
            None
        );
        assert!(refusal(&stub(r"C:\Windows\System32\cmd.exe /c calc")).is_some());

        let version = entry(RegistryHive::LocalMachine, key, Some("Version"), "1,0");
        assert!(!is_autostart_entry(&version));
        assert_eq!(refusal(&version), None);
    }

    #[test]
    fn app_paths_must_point_at_the_apps_own_file() {
        let key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\acme.exe";
        let default = |data| entry(RegistryHive::LocalMachine, key, Some("(Default)"), data);
        assert_eq!(refusal(&default(r"C:\Program Files\Acme\acme.exe")), None);
        assert!(refusal(&default(r"C:\Windows\System32\cmd.exe")).is_some());

        let path = entry(RegistryHive::LocalMachine, key, Some("Path"), r"C:\Windows");
        assert_eq!(
            refusal(&path),
            None,
            "only the default value names a program"
        );
    }

    #[test]
    fn a_registry_link_or_a_key_with_control_characters_is_refused() {
        let mut link = entry(
            RegistryHive::CurrentUser,
            r"Software\Acme",
            Some("SymbolicLinkValue"),
            "",
        );
        link.value_kind = Some(RegistryValueKind::Link);
        assert!(refusal(&link).is_some());
        assert!(
            refusal(&entry(
                RegistryHive::CurrentUser,
                "Software\\Acme\0x",
                Some("x"),
                ""
            ))
            .is_some()
        );
    }

    #[test]
    fn a_com_class_can_be_registered_but_not_taken_over() {
        let target = Path::new(r"D:\Apps\Acme");
        let windows = Path::new(WINDOWS);
        assert_eq!(
            com_server_refusal("", r"D:\Apps\Acme\a.dll", target, windows),
            None
        );
        assert_eq!(
            com_server_refusal(
                r"D:\Apps\Acme\old.dll",
                r"D:\Apps\Acme\a.dll",
                target,
                windows
            ),
            None,
            "an earlier install of the app"
        );
        assert_eq!(
            com_server_refusal(
                r"C:\Windows\System32\mscoree.dll",
                r"c:\windows\system32\MSCOREE.DLL",
                target,
                windows
            ),
            None,
            "already what the package asks for"
        );
        assert!(
            com_server_refusal(
                r"%SystemRoot%\System32\shell32.dll",
                r"D:\Apps\Acme\a.dll",
                target,
                windows
            )
            .is_some()
        );
        assert_eq!(
            com_server_refusal(
                r"D:\Apps\Acme\My Tools\old.dll",
                r"D:\Apps\Acme\a.dll",
                target,
                windows
            ),
            None,
            "a path with spaces is read whole"
        );
        assert!(
            com_server_refusal(
                r"D:\Apps\Other\b.dll",
                r"D:\Apps\Acme\a.dll",
                target,
                windows
            )
            .is_some()
        );
    }

    #[test]
    fn com_server_values_are_recognized_in_both_views() {
        let server = |key: &str, name: Option<&str>| {
            is_com_server_entry(&entry(RegistryHive::LocalMachine, key, name, ""))
        };
        assert!(server(r"SOFTWARE\Classes\CLSID\{A}\InprocServer32", None));
        assert!(server(
            r"Software\Classes\WOW6432Node\CLSID\{A}\LocalServer32",
            Some("(Default)")
        ));
        assert!(server(
            r"SOFTWARE\WOW6432Node\Classes\CLSID\{A}\TreatAs",
            None
        ));
        assert!(!server(
            r"SOFTWARE\Classes\CLSID\{A}\InprocServer32",
            Some("ThreadingModel")
        ));
        assert!(!server(r"SOFTWARE\Classes\CLSID\{A}\ProgID", None));
        assert!(!server(r"SOFTWARE\Acme\CLSID\{A}\InprocServer32", None));
    }

    #[test]
    fn environment_variables_are_refused_by_name_and_only_by_name() {
        for name in [
            "COMPLUS_InstallRoot",
            "COMPLUS_Version",
            "APPDOMAIN_MANAGER_ASM",
            "ComSpec",
            "PATHEXT",
            "windir",
            "SystemRoot",
            "TEMP",
            "ProgramFiles",
            "COR_ENABLE_PROFILING",
            "cor_profiler_path",
            "CORECLR_PROFILER",
            "__COMPAT_LAYER",
            "JAVA_TOOL_OPTIONS",
            "NODE_OPTIONS",
            "DOTNET_STARTUP_HOOKS",
            "",
            "A=B",
        ] {
            assert!(environment_refusal(name).is_some(), "{name}");
        }
        for name in [
            "Path",
            "PSModulePath",
            "ACME_HOME",
            "JAVA_HOME",
            "COREHOME",
            "PYTHONPATH",
        ] {
            assert_eq!(environment_refusal(name), None, "{name}");
        }
    }

    fn task_xml(actions: &str, principal: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-16"?>
            <Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
              <Principals><Principal id="Author">{principal}</Principal></Principals>
              <Actions Context="Author">{actions}</Actions>
            </Task>"#
        )
    }

    fn exec(command: &str) -> String {
        format!("<Exec><Command>{command}</Command><Arguments>--run</Arguments></Exec>")
    }

    fn definition(xml: &str) -> TaskDefinition {
        read_task_definition(xml).unwrap_or_default()
    }

    #[test]
    fn a_task_definition_is_read_from_its_xml() {
        let parsed = definition(&task_xml(
            &format!(
                "{}{}",
                exec(r"D:\A&amp;B\a.exe"),
                exec(r#""D:\Apps\Acme\acme.exe""#)
            ),
            "<UserId>S-1-5-18</UserId><RunLevel>HighestAvailable</RunLevel>",
        ));
        assert_eq!(
            parsed.commands,
            [r"D:\A&B\a.exe", r#""D:\Apps\Acme\acme.exe""#]
        );
        assert!(parsed.runs_as_system);
        assert!(!parsed.has_other_actions);

        for user in [
            "SYSTEM",
            r"NT AUTHORITY\SYSTEM",
            "s-1-5-18",
            "LocalSystem",
            r".\SYSTEM",
            r"PCNAME\SYSTEM",
            "<!-- hidden -->S-1-5-18",
            "S-1-5<!-- hidden -->-18",
        ] {
            let xml = task_xml(&exec("x"), &format!("<UserId>{user}</UserId>"));
            assert!(definition(&xml).runs_as_system, "{user}");
        }
        for user in ["S-1-5-21-1-2-3-1001", "bob", "S-1-5-19"] {
            let xml = task_xml(&exec("x"), &format!("<UserId>{user}</UserId>"));
            assert!(!definition(&xml).runs_as_system, "{user}");
        }
        assert!(read_task_definition("<Task>").is_err());
        assert!(
            read_task_definition(r#"<!DOCTYPE Task [<!ENTITY x "y">]><Task>&x;</Task>"#).is_err(),
            "entities are refused rather than expanded"
        );
    }

    fn task_verdict(path: &str, name: &str, xml: &str) -> Option<String> {
        task_refusal(path, name, &definition(xml), &app_map(), &|text| {
            text.to_owned()
        })
    }

    #[test]
    fn a_tasks_programs_must_all_be_the_apps_own() {
        let good = exec(r"D:\Apps\Acme\acme.exe");
        let quoted = exec(r#""D:\Apps\Acme\acme.exe""#);
        let evil = exec(r"C:\Windows\System32\cmd.exe");
        assert_eq!(task_verdict(r"\", "Acme", &task_xml(&good, "")), None);
        assert_eq!(task_verdict(r"\", "Acme", &task_xml(&quoted, "")), None);
        assert_eq!(
            task_verdict(r"\Acme\Sub\", "Acme", &task_xml(&good, "")),
            None
        );
        assert_eq!(
            task_verdict(r"\Microsoft\VisualStudio\", "Acme", &task_xml(&good, "")),
            None,
            "only Windows' own folder is off limits"
        );

        assert!(task_verdict(r"\", "Acme", &task_xml(&evil, "")).is_some());
        assert!(
            task_verdict(r"\", "Acme", &task_xml(&format!("{good}{evil}"), "")).is_some(),
            "every action counts"
        );
        assert!(
            task_verdict(
                r"\",
                "Acme",
                &task_xml(
                    &format!("{good}<ComHandler><ClassId>{{A}}</ClassId></ComHandler>"),
                    ""
                )
            )
            .is_some()
        );
        assert!(task_verdict(r"\", "Acme", &task_xml("", "")).is_some());
    }

    #[test]
    fn a_tasks_name_and_folder_must_be_safe() {
        let good = task_xml(&exec(r"D:\Apps\Acme\acme.exe"), "");
        for (path, name) in [
            (r"\", ""),
            (r"\", r"a\b"),
            (r"\", "a/b"),
            (r"\", "a:b"),
            ("Acme", "x"),
            (r"\..\", "x"),
            (
                r"\Microsoft\Windows\Defender\",
                "Windows Defender Scheduled Scan",
            ),
            (r"\microsoft\windows\", "x"),
        ] {
            assert!(task_verdict(path, name, &good).is_some(), "{path}{name}");
        }
    }

    #[test]
    fn an_existing_task_outside_the_install_location_is_left_alone() {
        let target = Path::new(r"D:\Apps\Acme");
        let task = |command: &str| task_xml(&exec(command), "");
        assert_eq!(
            existing_task_refusal(&task(r"D:\Apps\Acme\acme.exe"), target),
            None
        );
        assert!(existing_task_refusal(&task(r"%windir%\system32\defrag.exe"), target).is_some());
        assert!(existing_task_refusal(&task(r"C:\Program Files\Other\x.exe"), target).is_some());
        assert!(existing_task_refusal("<Task>", target).is_some());
        let com_handler = task_xml("<ComHandler><ClassId>{A}</ClassId></ComHandler>", "");
        assert!(existing_task_refusal(&com_handler, target).is_some());
    }

    fn shortcut_folders() -> KnownFolders {
        KnownFolders {
            user_desktop: Some(PathBuf::from(r"C:\Users\bob\Desktop")),
            common_desktop: Some(PathBuf::from(r"C:\Users\Public\Desktop")),
            user_start_menu: Some(PathBuf::from(r"C:\Users\bob\Start")),
            common_start_menu: Some(PathBuf::from(r"C:\ProgramData\Start")),
        }
    }

    #[test]
    fn shortcuts_are_re_anchored_to_the_same_kind_of_folder() {
        let placed = |source: &str| shortcut_placement(Path::new(source), &shortcut_folders());
        for (source, destination, all_users) in [
            (
                r"C:\Users\alice\Desktop\Acme.lnk",
                r"C:\Users\bob\Desktop\Acme.lnk",
                false,
            ),
            (
                r"C:\Users\alice\OneDrive - Contoso\Desktop\Tools\Acme.lnk",
                r"C:\Users\bob\Desktop\Tools\Acme.lnk",
                false,
            ),
            (
                r"C:\Users\Public\Desktop\Acme.lnk",
                r"C:\Users\Public\Desktop\Acme.lnk",
                true,
            ),
            (
                r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Acme\Acme.lnk",
                r"C:\Users\bob\Start\Programs\Acme\Acme.lnk",
                false,
            ),
            (
                r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Acme.lnk",
                r"C:\ProgramData\Start\Programs\Acme.lnk",
                true,
            ),
        ] {
            let Ok(placement) = placed(source) else {
                panic!("{source} should be allowed");
            };
            assert_eq!(placement.destination, PathBuf::from(destination));
            assert_eq!(placement.for_all_users, all_users, "{source}");
            assert!(!placement.autostart, "{source}");
        }
    }

    #[test]
    fn a_startup_shortcut_is_marked_as_autostart() {
        for source in [
            r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\Acme.lnk",
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\StartUp\Acme.lnk",
        ] {
            let placement = shortcut_placement(Path::new(source), &shortcut_folders());
            assert!(
                placement.is_ok_and(|placement| placement.autostart),
                "{source}"
            );
        }
    }

    #[test]
    fn a_shortcut_cannot_land_anywhere_else() {
        for source in [
            r"C:\Windows\System32\evil.lnk",
            r"C:\Program Files\Acme\Acme.lnk",
            r"C:\Users\alice\Desktop\Acme.exe",
            r"C:\Users\alice\Desktop\Acme.url",
            r"C:\Users\alice\Desktop\NUL\Acme.lnk",
            r"C:\Users\alice\Desktop\CON\Acme.lnk",
            r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\PROGRA~1\STARTU~1\Acme.lnk",
        ] {
            assert!(
                shortcut_placement(Path::new(source), &shortcut_folders()).is_err(),
                "{source}"
            );
        }
        let placed = shortcut_placement(
            Path::new(r"C:\Users\alice\Desktop\..\..\..\Windows\evil.lnk"),
            &shortcut_folders(),
        );
        assert!(
            placed.is_ok_and(|placement| placement
                .destination
                .starts_with(r"C:\Users\bob\Desktop")
                && !placement.destination.to_string_lossy().contains("..")),
            "a parent step stays inside the folder"
        );
        assert!(
            shortcut_placement(
                Path::new(r"C:\Users\alice\Desktop\A.lnk"),
                &KnownFolders::default()
            )
            .is_err()
        );
    }

    #[test]
    fn user_data_cannot_land_in_a_startup_folder_or_windows_own_folders() {
        for tail in [
            r"Microsoft\Windows\Start Menu\Programs\Startup\x.exe",
            r"microsoft\windows\start menu\programs\startup\sub\x.exe",
            r"Microsoft\Windows\Themes\x.dll",
            r"Microsoft\WindowsApps\winget.exe",
            r"Microsoft\Windows Defender\Platform\x.dll",
            r"Microsoft\Word\STARTUP\x.dotm",
            r"Microsoft\Excel\XLSTART\x.xlsm",
            r"Microsoft\Crypto\RSA\x",
            r"Oracle\Java\javapath\java.exe",
            r"Desktop\Evil.lnk",
            r"Start Menu\Programs\Evil.lnk",
            r"Vendor\Start Menu\Programs\Startup\x.exe",
            r"Microsoft\Windows\STARTM~1\Programs\Startup\x.exe",
            r"PROGRA~1\x.exe",
        ] {
            assert!(data_location_refusal(Path::new(tail)).is_some(), "{tail}");
        }
        for tail in [
            r"Acme\settings.json",
            r"Vendor\Acme\Startup\settings.json",
            r"Microsoft\Teams\settings.json",
            r"Microsoft\WindowsTerminal\x.json",
            r"Acme\data~backup.txt",
            r"Acme\file~1.txt.bak~",
        ] {
            assert_eq!(data_location_refusal(Path::new(tail)), None, "{tail}");
        }
    }

    fn real_location(original: &Path) -> Option<(PathBuf, PathBuf)> {
        crate::file_discovery::split_user_data_relative(original)
            .map(|(_, tail)| (Path::new(r"C:\Real").join(&tail), tail))
    }

    #[test]
    fn license_files_go_only_to_the_install_location_or_a_known_folder() {
        let map = app_map();
        let place = |path: &str, user_data: bool| {
            license_destination(Path::new(path), &map, user_data, &real_location)
        };
        assert_eq!(
            place(r"C:\Program Files\Acme\acme.lic", false),
            Ok(PathBuf::from(r"D:\Apps\Acme\acme.lic"))
        );
        assert_eq!(
            place(r"C:\ProgramData\FlexLM", false),
            Ok(PathBuf::from(r"C:\Real\FlexLM")),
            "a store re-anchors to this PC's folder even when the source drive was different"
        );
        assert_eq!(
            place(r"E:\Users\alice\AppData\Roaming\Acme\a.lic", true),
            Ok(PathBuf::from(r"C:\Real\Acme\a.lic"))
        );
        for bad in [
            r"C:\Windows\System32\evil.dll",
            r"C:\Program Files\Other\x.lic",
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup\x.exe",
            r"C:\Program Files\Acme\..\..\Windows\evil.dll",
        ] {
            assert!(place(bad, false).is_err(), "{bad}");
            assert!(place(bad, true).is_err(), "{bad}");
        }
    }
}
