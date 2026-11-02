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

use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf, Prefix};

use crate::errors::Result;
use crate::file_discovery::{
    has_reserved_device_name, is_reserved_device_name, restore_destination, safe_relative_path,
};
use crate::integration_scan::{
    EVENT_LOG_FILE_VALUES, EVENT_LOG_OTHER_VALUES, ODBC_DRIVER_LIST, ODBC_SOURCE_LIST,
    first_path_in, parse_firewall_rule,
};
use crate::models::{
    DriverEntry, FirewallRuleEntry, RegistryEntry, RegistryHive, RegistryValueKind,
};
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
/// the rest of the package looks. A service already running from `target_root`, where a record
/// says an install was, is an earlier install of this app, which is what `--force` is for.
pub fn existing_service_refusal(
    service_name: &str,
    current_image_path: &str,
    target_root: &Path,
    windows_dir: &Path,
) -> Option<String> {
    let current = parse_image_path(current_image_path, windows_dir);
    (!path_is_strictly_inside(&current.path, target_root)).then(|| {
        format!(
            "{service_name} already exists on this PC and runs {}, which is not from an earlier \
             install here. If it is an older copy of this app, uninstall that first",
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
    // What a menu's `SubCommands` names, which is outside what the rules for a verb read.
    r"software\microsoft\windows\currentversion\explorer\commandstore",
    r"software\microsoft\windows\currentversion\explorer\browser helper objects",
    r"software\microsoft\windows\currentversion\explorer\sharedtaskscheduler",
    // What stops an extension loading, and what Explorer remembers about those it has.
    r"software\microsoft\windows\currentversion\shell extensions\blocked",
    r"software\microsoft\windows\currentversion\shell extensions\cached",
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
    "libraryfolder",
    r"libraryfolder\background",
];
const OPEN_WITH_PREFIX: &str = "openwith";
/// `Folder\shell\open` decides what opening any folder runs.
const REFUSED_CLASS_VERBS: &[&str] = &[
    r"folder\shell\open",
    r"directory\shell\open",
    r"directory\background\shell\open",
    r"drive\shell\open",
    r"*\shell\open",
    r"allfilesystemobjects\shell\open",
];
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
pub(crate) const EVENT_LOG_PREFIX: &str = r"system\currentcontrolset\services\eventlog\";
/// Who wrote to this one is what an auditor goes by. Windows would not let an app do it anyway.
const SECURITY_LOG: &str = "security";
const INTERFACE_PREFIX: &str = r"software\classes\interface\";
const TYPE_LIBRARY_PREFIX: &str = r"software\classes\typelib\";
const APP_ID_PREFIX: &str = r"software\classes\appid\";
const APPLICATIONS_PREFIX: &str = r"software\classes\applications\";
const APP_PATHS_PREFIX: &str = r"software\microsoft\windows\currentversion\app paths\";
const ODBC_DRIVERS_KEY: &str = r"software\odbc\odbcinst.ini";
const ODBC_SOURCES_KEY: &str = r"software\odbc\odbc.ini";
const FONTS_KEY: &str = r"software\microsoft\windows nt\currentversion\fonts";
const COMPATIBILITY_LAYERS_KEY: &str =
    r"software\microsoft\windows nt\currentversion\appcompatflags\layers";
const ICON_OVERLAYS_KEY: &str =
    r"software\microsoft\windows\currentversion\explorer\shelliconoverlayidentifiers";
const APPROVED_EXTENSIONS_KEY: &str =
    r"software\microsoft\windows\currentversion\shell extensions\approved";
const NAMESPACE_KEYS: &[&str] = &[
    r"software\microsoft\windows\currentversion\explorer\desktop\namespace",
    r"software\microsoft\windows\currentversion\explorer\mycomputer\namespace",
];
const REGISTERED_APPLICATIONS_KEY: &str = r"software\registeredapplications";
const CLIENTS_KEY: &str = r"software\clients";
const SHELL_EXTENSIONS_SEGMENT: &str = r"\shellex";
const OPEN_COMMAND_TAIL: &str = r"\command";
const SHELL_SEGMENT: &str = r"\shell\";
const URL_PROTOCOL_VALUE: &str = "url protocol";
/// A class's own values that can make it something else's.
const COM_REDIRECT_KEYS: &[&str] = &["treatas", "autotreatas"];
/// What makes a call to an interface go through a type library, which any app's can use.
const AUTOMATION_MARSHALERS: &[&str] = &[
    "{00020420-0000-0000-c000-000000000046}",
    "{00020424-0000-0000-c000-000000000046}",
];
const PROXY_STUB_KEYS: &[&str] = &["proxystubclsid32", "proxystubclsid"];
const TYPE_LIBRARY_BUILDS: &[&str] = &["win32", "win64", "arm64"];
const VERB_HANDLER_VALUES: &[&str] = &["explorercommandhandler", "delegateexecute"];
/// What makes a verb a menu of others, which is on the list of what is set up.
const VERB_MENU_VALUES: &[&str] = &["subcommands", "extendedsubcommandskey"];
/// What runs in place of a verb's command when it is run with full rights.
const ELEVATED_COMMAND_VALUE: &str = "isolatedcommand";
/// What Windows Installer puts beside a verb's command, for it to find a product by, and which is
/// run in place of the command. It is no path, and means nothing where that product is not installed.
const INSTALLER_COMMAND_VALUE: &str = "command";
/// Values under a registered thing that are read as a path or a class's id, like a default value is.
const TEXT_ONLY_VALUES: &[&str] = &[
    "appid",
    "codebase",
    "delegateexecute",
    "dllsurrogate",
    "driver",
    "eventmessagefile",
    "categorymessagefile",
    "parametermessagefile",
    "explorercommandhandler",
    "localservice",
    "runas",
    "serverexecutable",
    "setup",
    "tracedll",
    "translationdll",
];
const NETWORK_PATH_START: &str = r"\\";
const APP_ID_ACCESS_VALUES: &[&str] = &[
    "accesspermission",
    "authenticationlevel",
    "launchpermission",
    "runas",
];
/// Protocols that are not any one app's. Any app can be the one that opens them, and what a page
/// on the web can ask for is what these open. See [`class_command_refusal`].
const SHARED_PROTOCOLS: &[&str] = &[
    "about",
    "callto",
    "file",
    "ftp",
    "ftps",
    "http",
    "https",
    "javascript",
    "ldap",
    "mailto",
    "news",
    "nntp",
    "res",
    "sms",
    "snews",
    "ssh",
    "tel",
    "telnet",
];
const INTERACTIVE_USER: &str = "interactive user";
const APP_ID_RUN_AS_VALUE: &str = "runas";
const APP_ID_SURROGATE_VALUE: &str = "dllsurrogate";
const APP_ID_SERVICE_VALUE: &str = "localservice";
const APP_ID_LINK_VALUE: &str = "appid";
/// What makes a class run somewhere else than this PC.
const APP_ID_REMOTE_VALUES: &[&str] = &["remoteservername", "activateatstorage"];
const ODBC_DRIVER_FILE_VALUES: &[&str] = &["driver", "setup"];
/// What a data source can load besides its driver: a translator, and what traces every call.
const ODBC_SOURCE_FILE_VALUES: &[&str] = &["setup", "tracedll", "translationdll"];
/// Keys under `ODBC.INI` and `ODBCINST.INI` that are not one driver or source. `ODBC.INI\ODBC` says
/// what traces every program's calls, and it loads whatever that says.
const ODBC_RESERVED_NAMES: &[&str] = &["odbc", "odbc core", "odbc file dsn", "odbc translators"];
/// What of Windows' own can be the server of an app's class: what hosts a .NET class, a folder in
/// Explorer, or a proxy. Not `scrobj.dll`, which runs any script it is given, nor any program, since
/// `cmd.exe` is also a bare name in Windows.
const COM_WINDOWS_SERVERS: &[&str] = &[
    "actxprxy.dll",
    "combase.dll",
    "mscoree.dll",
    "ole32.dll",
    "oleaut32.dll",
    "shell32.dll",
];
/// Where Windows keeps what can host something of an app's, below its own folder.
const WINDOWS_PROGRAM_FOLDERS: &[&str] = &["", "System32", "SysWOW64"];
/// What of Windows' own a file type can run a script or a library of its own through. Not `cmd.exe`
/// or `ssh.exe`, which run what they are told as well as what they are given.
const WINDOWS_FILE_HOSTS: &[&str] = &[
    "cscript.exe",
    "explorer.exe",
    "mshta.exe",
    "notepad.exe",
    "rundll32.exe",
    "wscript.exe",
];
const COM_LOCAL_SERVER_KEY: &str = "localserver32";
/// Files that run, or that what runs loads.
const RUNNABLE_EXTENSIONS: &[&str] = &[
    "asar", "ax", "bat", "bpl", "cmd", "com", "cpl", "dll", "drv", "exe", "hta", "jar", "js",
    "jse", "lnk", "manifest", "msc", "msi", "node", "ocx", "pif", "ps1", "psd1", "psm1", "py",
    "pyc", "pyd", "pyw", "reg", "scr", "sys", "vbe", "vbs", "wll", "wsf", "wsh", "xll",
];
/// What says how the program of that name starts, which can name what it loads. A `user.config` is
/// only someone's settings.
const RUNNABLE_NAME_ENDS: &[&str] = &[
    ".exe.config",
    ".dll.config",
    ".runtimeconfig.json",
    ".deps.json",
    "._pth",
];
const COM_INPROC_SERVER_KEY: &str = "inprocserver32";
/// What is run or loaded for a class besides what its server's default value says.
const COM_SERVER_PROGRAM_VALUE: &str = "ServerExecutable";
const COM_CODE_BASE_VALUE: &str = "CodeBase";
/// A rule asks for nothing more than this of what is in it, and what is passed to `netsh` as one
/// word goes by it, so that what a package says cannot be read as another option.
const FIREWALL_DIRECTIONS: &[&str] = &["in", "out"];
const FIREWALL_ACTIONS: &[&str] = &["allow", "block"];
const FIREWALL_PROFILES: &[&str] = &["domain", "private", "public"];
const FIREWALL_RULE_NAME_LIMIT: usize = 200;
/// `netsh` reads this name as every rule on the PC, for a delete.
const FIREWALL_RESERVED_NAME: &str = "all";
const FIREWALL_DESCRIPTION_LIMIT: usize = 1024;
const FIREWALL_ANY: &str = "any";
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
/// The handler COM itself provides, which any class can name. As a handler, nothing about whose
/// a class is follows from it. As a server it does: there are classes that have it as their only
/// one.
const COM_DEFAULT_HANDLER: &str = "ole32.dll";
const COM_HANDLER_KEY: &str = "inprochandler32";
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
const UNINSTALL_PREFIX: &str = r"software\microsoft\windows\currentversion\uninstall\";
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
/// In a user's Documents. PowerShell runs what is in these whenever it starts, as a Startup folder
/// is run at sign-in, and they mean nothing anywhere else.
const REFUSED_DOCUMENTS_FOLDERS: &[&str] = &[r"windowspowershell\", r"powershell\"];

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
    // The registry ends a name at a NUL, so this would go by one name and be written as another.
    if entry
        .value_name
        .as_deref()
        .is_some_and(|name| name.chars().any(char::is_control))
    {
        return Some("its value's name holds control characters".to_owned());
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
    // It is only ever a file's name, which looking it up would otherwise read as a path.
    if key
        .strip_prefix(APP_PATHS_PREFIX)
        .is_some_and(|rest| rest.contains(['/', ':']))
    {
        return Some("it names a program by something other than a file's name".to_owned());
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

/// True for text whose path starts with a variable, which could be hiding the app's own path.
fn starts_with_a_variable(text: &str) -> bool {
    text.trim_start_matches(['"', ' ']).starts_with('%')
}

/// What text means on the target. A path can be written with a variable, as `%ProgramFiles%\A\a.dll`,
/// which only reads as the app's own place once the variable is read, and is then moved like any.
/// Anything else is moved once: a new place can be under the old one.
pub(crate) fn text_on_target(
    text: &str,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> String {
    let rewritten = map.rewrite(text);
    let expanded = expand(&rewritten);
    if starts_with_a_variable(&rewritten) {
        map.rewrite(&expanded)
    } else {
        expanded
    }
}

/// True for a value holding a command Windows runs as it is written, where a path needs its quotes.
fn holds_a_command(entry: &RegistryEntry) -> bool {
    if is_autostart_entry(entry) {
        return true;
    }
    let key = normalized_key(&entry.key_path);
    is_default_value(entry.value_name.as_deref())
        && (key.ends_with(r"\localserver32")
            || (key.ends_with(OPEN_COMMAND_TAIL) && key.contains(SHELL_SEGMENT)))
}

/// What a registry value's text will be on the target. A command that runs by itself gets
/// its path quoted if it needs it, because this is where `C:\App\a.exe --x` becomes
/// `D:\My Apps\a.exe --x`, and Windows would try `D:\My.exe` first. A path written with a variable
/// that moved with the app is written as where it went.
pub fn restored_value_text(entry: &RegistryEntry, map: &RestoreMap) -> String {
    let rewritten = map.rewrite(entry.value_data.as_deref().unwrap_or_default());
    let rewritten = if starts_with_a_variable(&rewritten) {
        let expanded = crate::file_discovery::expand_env_vars(&rewritten);
        let moved = map.rewrite(&expanded);
        if moved == expanded { rewritten } else { moved }
    } else {
        rewritten
    };
    if holds_a_command(entry) {
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
        "it starts automatically, and runs something this package does not install".to_owned()
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
    if let Some(rest) = key.strip_prefix(EVENT_LOG_PREFIX) {
        return event_log_value_refusal(rest, entry, map, expand);
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

/// Under `EventLog`, only a source's own values. A log's key says where that log is kept and who
/// may read it. Every file a source takes its messages from has to be one this package installs,
/// since whatever reads the log loads them. Whether it has one at all, and whether one of that name
/// is on this PC, is asked of the whole source.
fn event_log_value_refusal(
    rest: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    let mut parts = rest.split('\\');
    let (Some(log), Some(_source), None) = (parts.next(), parts.next(), parts.next()) else {
        return Some("only an event log source's own values can be restored".to_owned());
    };
    if log == SECURITY_LOG {
        return Some("no app writes to the Security log".to_owned());
    }
    let name = entry
        .value_name
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if EVENT_LOG_FILE_VALUES
        .iter()
        .any(|known| known.eq_ignore_ascii_case(&name))
    {
        let files = text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
        return files
            .split(';')
            .map(str::trim)
            .any(|file| !map.is_restored_file(Path::new(file)))
            .then(|| {
                "its messages would come from a file this package does not install".to_owned()
            });
    }
    (!EVENT_LOG_OTHER_VALUES
        .iter()
        .any(|known| known.eq_ignore_ascii_case(&name)))
    .then(|| "only an event log source's own values can be restored".to_owned())
}

/// What a whole package says about what it registers, for what one entry has to go by.
#[derive(Debug, Clone, Default)]
pub struct PackageRegistrations {
    /// COM classes, by lowercase id, whose server is a file this package installs.
    classes: HashSet<String>,
    /// Which of its files each runs, for saying so.
    class_servers: HashMap<String, String>,
    type_libraries: HashSet<String>,
    odbc_drivers: HashSet<String>,
    odbc_sources: HashSet<String>,
    /// The lowercase name of every file this package installs, which `AppID\<name>` goes by.
    file_names: HashSet<String>,
    /// Classes that say they are a URL protocol, by lowercase name.
    url_protocols: HashSet<String>,
    /// Classes that open with a file this package installs, by lowercase name.
    classes_that_run_it: HashSet<String>,
    /// Every key the package writes to, and all those above them, for what names one of them.
    keys: HashSet<String>,
    /// What `RegisteredApplications` says the package's capabilities are under, by normalized key.
    capabilities: Vec<String>,
    /// Where Windows is on this PC, for what a command may run through.
    windows_dir: PathBuf,
}

impl PackageRegistrations {
    pub fn of(
        entries: &[&RegistryEntry],
        file_destinations: &[&Path],
        map: &RestoreMap,
        expand: &dyn Fn(&str) -> String,
        windows_dir: &Path,
    ) -> Self {
        let mut known = Self {
            file_names: file_destinations
                .iter()
                .filter_map(|path| path.file_name())
                .map(|name| name.to_string_lossy().to_ascii_lowercase())
                .collect(),
            windows_dir: windows_dir.to_path_buf(),
            ..Self::default()
        };
        let mut classes_with_another_server: HashSet<String> = HashSet::new();
        for entry in entries {
            let key = normalized_key(&entry.key_path);
            let mut above = key.as_str();
            loop {
                known.keys.insert(above.to_owned());
                match above.rsplit_once('\\') {
                    Some((parent, _)) => above = parent,
                    None => break,
                }
            }
            let name = entry.value_name.as_deref().unwrap_or_default();
            if key == REGISTERED_APPLICATIONS_KEY {
                known.capabilities.push(normalized_key(
                    entry.value_data.as_deref().unwrap_or_default(),
                ));
            }
            // It says nothing of what it holds, however it came to have some text.
            let holds_something = entry.value_kind != Some(RegistryValueKind::None);
            if let Some(class) = key.strip_prefix(CLASSES_PREFIX)
                && name.eq_ignore_ascii_case(URL_PROTOCOL_VALUE)
                && !class.contains('\\')
            {
                known.url_protocols.insert(class.to_owned());
            }
            if holds_something
                && let Some(class) = key.strip_prefix(CLASSES_PREFIX)
                && class.contains(SHELL_SEGMENT)
                && class.ends_with(OPEN_COMMAND_TAIL)
                && is_default_value(entry.value_name.as_deref())
                && command_runs_restored_file(
                    &text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand),
                    map,
                )
            {
                known
                    .classes_that_run_it
                    .insert(class.split('\\').next().unwrap_or_default().to_owned());
            }
            if holds_something && let Some(rest) = key.strip_prefix(CLSID_PREFIX) {
                let class = rest.split('\\').next().unwrap_or_default().to_owned();
                match class_server_says(rest, entry, map, expand, windows_dir) {
                    ClassServerSays::ItsOwn(file) => {
                        known.classes.insert(class.clone());
                        known.class_servers.entry(class).or_insert(file);
                    }
                    ClassServerSays::SomethingElse => {
                        classes_with_another_server.insert(class);
                    }
                    ClassServerSays::Nothing => {}
                }
            }
            if holds_something
                && let Some(rest) = key.strip_prefix(TYPE_LIBRARY_PREFIX)
                && restored_type_library_file(rest, entry, map, expand)
            {
                known
                    .type_libraries
                    .insert(rest.split('\\').next().unwrap_or_default().to_owned());
            }
            if holds_something
                && let Some(rest) = key.strip_prefix(&format!(r"{ODBC_DRIVERS_KEY}\"))
                && !rest.contains('\\')
                && name.eq_ignore_ascii_case(ODBC_DRIVER_FILE_VALUES[0])
                && odbc_file_is_restored(entry, map, expand)
            {
                known.odbc_drivers.insert(rest.to_owned());
            }
            if let Some(rest) = key.strip_prefix(&format!(r"{ODBC_SOURCES_KEY}\"))
                && !rest.contains('\\')
                && !rest.eq_ignore_ascii_case(ODBC_SOURCE_LIST)
            {
                known.odbc_sources.insert(rest.to_owned());
            }
        }
        // One server of a class's that is something else is what would be loaded just as well.
        for class in &classes_with_another_server {
            known.classes.remove(class);
            known.class_servers.remove(class);
        }
        known
    }

    /// The file a class in this package runs, if it runs one of its own.
    pub(crate) fn server_of_class(&self, class: &str) -> Option<&str> {
        self.class_servers
            .get(&class.to_ascii_lowercase())
            .map(String::as_str)
    }
}

/// What a value says of a class's server.
enum ClassServerSays {
    /// It is this file, which this package installs.
    ItsOwn(String),
    /// It is something else, which is not a host of Windows' for it.
    SomethingElse,
    /// It does not say.
    Nothing,
}

/// What a value of a class says of its server. `rest` is the key below `CLSID`.
fn class_server_says(
    rest: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
    windows_dir: &Path,
) -> ClassServerSays {
    let mut parts = rest.split('\\');
    let _class = parts.next();
    let Some(server) = parts.next() else {
        return ClassServerSays::Nothing;
    };
    if !matches!(server, COM_INPROC_SERVER_KEY | COM_LOCAL_SERVER_KEY) {
        return ClassServerSays::Nothing;
    }
    let is_directly_under = parts.next().is_none();
    let name = entry.value_name.as_deref();
    let text = text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
    let is_the_server = is_directly_under
        && (is_default_value(name)
            || name.is_some_and(|name| name.eq_ignore_ascii_case(COM_SERVER_PROGRAM_VALUE)));
    let is_code_base = name.is_some_and(|name| name.eq_ignore_ascii_case(COM_CODE_BASE_VALUE));
    let path = if is_code_base {
        let Some(path) = text
            .get(FILE_URL_PREFIX.len()..)
            .filter(|_| text.to_ascii_lowercase().starts_with(FILE_URL_PREFIX))
        else {
            return ClassServerSays::SomethingElse;
        };
        map.rewrite(&path.replace('/', "\\"))
    } else if is_the_server {
        text
    } else {
        return ClassServerSays::Nothing;
    };
    if path.trim().is_empty() {
        return ClassServerSays::Nothing;
    }
    if command_runs_restored_file(&path, map) {
        return first_path_in(&path).map_or(ClassServerSays::Nothing, |file| {
            ClassServerSays::ItsOwn(file_name_of(&file))
        });
    }
    if server == COM_INPROC_SERVER_KEY && is_a_com_host_of_windows(&path, windows_dir) {
        return ClassServerSays::Nothing;
    }
    ClassServerSays::SomethingElse
}

/// Whether a server is one of Windows' own that an app's class can be hosted by. As a plain name,
/// or by its whole path in the Windows folder, and with nothing after it to run.
fn is_a_com_host_of_windows(text: &str, windows_dir: &Path) -> bool {
    let Some(path) = first_path_in(text) else {
        return false;
    };
    let as_written = text.trim().trim_matches('"');
    let is_named_alone = path.as_os_str().eq_ignore_ascii_case(as_written);
    is_named_alone
        && is_a_program_of_windows(&path, windows_dir)
        && COM_WINDOWS_SERVERS.contains(&file_name_of(&path).to_ascii_lowercase().as_str())
}

/// Whether a path is what Windows keeps of its own: named alone, or in one of its program folders.
/// `C:\Windows\Temp` is in the Windows folder, and anyone can write there.
fn is_a_program_of_windows(path: &Path, windows_dir: &Path) -> bool {
    if !path.is_absolute() {
        return path.components().count() == 1;
    }
    path.parent().is_some_and(|folder| {
        WINDOWS_PROGRAM_FOLDERS
            .iter()
            .any(|below| paths_equal(folder, &windows_dir.join(below)))
    })
}

const FILE_URL_PREFIX: &str = "file:///";

fn file_name_of(path: &Path) -> String {
    path.file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
}

/// A key below `TypeLib`: `{guid}\<version>\<language>\<build>`.
fn restored_type_library_file(
    rest: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> bool {
    let parts: Vec<&str> = rest.split('\\').collect();
    parts.len() == 4
        && TYPE_LIBRARY_BUILDS.contains(&parts[3])
        && is_default_value(entry.value_name.as_deref())
        && map.is_restored_file(Path::new(without_resource_index(
            text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand)
                .trim()
                .trim_matches('"'),
        )))
}

/// A type library inside a program is named by its path and which of them it is: `C:\App\a.dll\2`.
fn without_resource_index(path: &str) -> &str {
    path.rsplit_once('\\')
        .filter(|(_, last)| !last.is_empty() && last.chars().all(|digit| digit.is_ascii_digit()))
        .map_or(path, |(file, _)| file)
}

fn odbc_file_is_restored(
    entry: &RegistryEntry,
    map: &RestoreMap,
    expand: &dyn Fn(&str) -> String,
) -> bool {
    map.is_restored_file(Path::new(
        text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand)
            .trim()
            .trim_matches('"'),
    ))
}

/// A registry value that is a CLSID, in lowercase, whatever it holds besides.
fn class_named_by(text: &str) -> String {
    text.trim().to_ascii_lowercase()
}

fn is_braced_id(text: &str) -> bool {
    text.len() == 38 && text.starts_with('{') && text.ends_with('}')
}

/// What can be said about one thing a package registers, which is all of it or none of it. A
/// class's values mean nothing without its server, and one wrong value is a reason not to take the
/// rest of what came with it.
pub fn registration_unit_of(entry: &RegistryEntry) -> Option<String> {
    let key = normalized_key(&entry.key_path);
    if key.strip_prefix(CLSID_PREFIX).is_some() {
        return com_class_key(&entry.key_path);
    }
    // A program's name, what `Open with` goes by for it, and its row in Settings are each one thing
    // for one program.
    for prefix in [
        INTERFACE_PREFIX,
        TYPE_LIBRARY_PREFIX,
        APP_ID_PREFIX,
        APP_PATHS_PREFIX,
        APPLICATIONS_PREFIX,
        UNINSTALL_PREFIX,
    ] {
        if let Some(rest) = key.strip_prefix(prefix) {
            return Some(format!(
                "{prefix}{}",
                rest.split('\\').next().unwrap_or_default()
            ));
        }
    }
    if let Some(rest) = key.strip_prefix(EVENT_LOG_PREFIX) {
        let mut parts = rest.split('\\');
        let (log, source) = (parts.next()?, parts.next()?);
        return Some(format!("{EVENT_LOG_PREFIX}{log}\\{source}"));
    }
    if let Some(rest) = key.strip_prefix(&format!(r"{CLIENTS_KEY}\")) {
        let mut parts = rest.split('\\');
        let (kind, program) = (parts.next()?, parts.next()?);
        return Some(format!(r"{CLIENTS_KEY}\{kind}\{program}"));
    }
    for (keys, list) in [
        (ODBC_DRIVERS_KEY, ODBC_DRIVER_LIST),
        (ODBC_SOURCES_KEY, ODBC_SOURCE_LIST),
    ] {
        if let Some(rest) = key.strip_prefix(&format!(r"{keys}\")) {
            let first = rest.split('\\').next().unwrap_or_default();
            let name = if first.eq_ignore_ascii_case(list) {
                entry
                    .value_name
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
            } else {
                first.to_owned()
            };
            return Some(format!("{keys}\\{name}"));
        }
    }
    None
}

/// Why all of one thing a package registers must be left out, if so. `entries` are all of it.
pub fn registration_refusal(
    unit: &str,
    entries: &[&RegistryEntry],
    map: &RestoreMap,
    known: &PackageRegistrations,
    restored_services: &HashSet<String>,
    expand: &dyn Fn(&str) -> String,
    windows_dir: &Path,
) -> Option<String> {
    let of_unit = |prefix: &str| unit.strip_prefix(prefix).map(str::to_owned);
    let value_is = |entry: &RegistryEntry, name: &str| {
        entry
            .value_name
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case(name))
    };
    if normalized_key(unit).strip_prefix(CLSID_PREFIX).is_some() {
        return com_class_refusal(entries, map, known, expand, windows_dir);
    }
    if let Some(interface) = of_unit(INTERFACE_PREFIX) {
        return interface_refusal(&interface, entries, known);
    }
    if of_unit(TYPE_LIBRARY_PREFIX).is_some() {
        return entries.iter().find_map(|entry| {
            let key = normalized_key(&entry.key_path);
            let rest = key.strip_prefix(TYPE_LIBRARY_PREFIX)?;
            let parts: Vec<&str> = rest.split('\\').collect();
            let names_a_file = parts.len() == 4 && TYPE_LIBRARY_BUILDS.contains(&parts[3]);
            (names_a_file && !restored_type_library_file(rest, entry, map, expand))
                .then(|| "its type library is not a file this package installs".to_owned())
        });
    }
    if let Some(name) = of_unit(APP_ID_PREFIX) {
        return app_id_refusal(&name, entries, map, known, restored_services, expand);
    }
    if let Some(name) = of_unit(APPLICATIONS_PREFIX)
        && !known.file_names.contains(&name)
    {
        return Some(format!(
            "it would change how {name} is opened, which this package does not install"
        ));
    }
    if of_unit(EVENT_LOG_PREFIX).is_some() {
        let message_files: Vec<&&RegistryEntry> = entries
            .iter()
            .filter(|entry| {
                EVENT_LOG_FILE_VALUES
                    .iter()
                    .any(|name| value_is(entry, name))
            })
            .collect();
        if message_files.is_empty() {
            return Some("an event log source has to take its messages from a file".to_owned());
        }
        return message_files
            .iter()
            .find_map(|entry| registry_refusal(entry, map, restored_services, expand));
    }
    if let Some(name) = of_unit(&format!(r"{ODBC_DRIVERS_KEY}\")) {
        return odbc_driver_refusal(&name, entries, map, known, expand);
    }
    if let Some(name) = of_unit(&format!(r"{ODBC_SOURCES_KEY}\")) {
        return odbc_source_refusal(&name, entries, map, known, expand, windows_dir);
    }
    None
}

/// A class's server has to be a file this package installs, or be Windows' own, such as
/// `mscoree.dll` for a .NET class. A share, or anywhere else on the PC, is neither. What takes a
/// class over from something else on this PC can only be seen when installing.
fn com_class_refusal(
    entries: &[&RegistryEntry],
    map: &RestoreMap,
    known: &PackageRegistrations,
    expand: &dyn Fn(&str) -> String,
    windows_dir: &Path,
) -> Option<String> {
    entries.iter().find_map(|entry| {
        let key = normalized_key(&entry.key_path);
        let rest = key.strip_prefix(CLSID_PREFIX)?;
        let subkey = rest.split('\\').nth(1)?;
        if entry.value_kind == Some(RegistryValueKind::None) {
            return None;
        }
        let says_what_runs = entry.value_name.as_deref().is_some_and(|name| {
            name.eq_ignore_ascii_case(COM_CODE_BASE_VALUE)
                || name.eq_ignore_ascii_case(COM_SERVER_PROGRAM_VALUE)
        });
        if says_what_runs {
            return matches!(
                class_server_says(rest, entry, map, expand, windows_dir),
                ClassServerSays::SomethingElse
            )
            .then(|| "it runs something that is not a file this package installs".to_owned());
        }
        if !is_default_value(entry.value_name.as_deref()) {
            return None;
        }
        let text = text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
        if COM_REDIRECT_KEYS.contains(&subkey) {
            return (!known.classes.contains(&class_named_by(&text))).then(|| {
                "it would make this class something this package does not install".to_owned()
            });
        }
        if !COM_SERVER_KEYS.contains(&subkey) || text.trim().is_empty() {
            return None;
        }
        let restored = command_runs_restored_file(&text, map);
        let windows_own =
            subkey != COM_LOCAL_SERVER_KEY && is_a_com_host_of_windows(&text, windows_dir);
        (!restored && !windows_own).then(|| {
            "it runs something that is neither a file this package installs nor part of Windows"
                .to_owned()
        })
    })
}

/// How calls to an interface cross processes is decided here for every program that uses it, so it
/// has to be by a class this package installs, or by Windows' marshaler and a type library this
/// package installs.
fn interface_refusal(
    interface: &str,
    entries: &[&RegistryEntry],
    known: &PackageRegistrations,
) -> Option<String> {
    let defaults_of = |subkeys: &[&str]| -> Vec<String> {
        entries
            .iter()
            .filter_map(|entry| {
                let key = normalized_key(&entry.key_path);
                let rest = key.strip_prefix(INTERFACE_PREFIX)?;
                // Directly under the interface, which is the only place Windows looks.
                let (_, leaf) = rest.split_once('\\')?;
                (subkeys.contains(&leaf) && is_default_value(entry.value_name.as_deref()))
                    .then(|| class_named_by(entry.value_data.as_deref().unwrap_or_default()))
            })
            .collect()
    };
    // Each of them, since which one is asked for depends on who asks.
    let stubs = defaults_of(PROXY_STUB_KEYS);
    let libraries = defaults_of(&["typelib"]);
    let by_marshaler = |stub: &str| AUTOMATION_MARSHALERS.contains(&stub);
    let library_is_restored = !libraries.is_empty()
        && libraries
            .iter()
            .all(|library| known.type_libraries.contains(library));
    let every_stub_is_fine = !stubs.is_empty()
        && stubs.iter().all(|stub| {
            known.classes.contains(stub) || (by_marshaler(stub) && library_is_restored)
        });
    (!every_stub_is_fine && !interface.is_empty()).then(|| {
        "it would decide how calls to an interface are made, with something this package does not \
         install"
            .to_owned()
    })
}

/// An AppID is how a class is started and who may ask. It can name a surrogate and a service to
/// host it, and either has to be this package's. `AppID\<program>` applies to a program by its file
/// name alone, so that has to be a file of this package. It cannot run as an account, which would
/// take a password, and cannot run on another PC.
fn app_id_refusal(
    name: &str,
    entries: &[&RegistryEntry],
    map: &RestoreMap,
    known: &PackageRegistrations,
    restored_services: &HashSet<String>,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if !is_braced_id(name) && !known.file_names.contains(name) {
        return Some(format!(
            "it would change how {name} is started, which this package does not install"
        ));
    }
    entries.iter().find_map(|entry| {
        let value = entry
            .value_name
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let text = entry.value_data.as_deref().unwrap_or_default().trim();
        match value.as_str() {
            APP_ID_SURROGATE_VALUE => (!text.is_empty()
                && !map.is_restored_file(Path::new(
                    text_on_target(text, map, expand).trim_matches('"'),
                )))
            .then(|| "it would be hosted by a program this package does not install".to_owned()),
            APP_ID_SERVICE_VALUE => (!restored_services.contains(&text.to_ascii_lowercase()))
                .then(|| "it would run in a service this package does not restore".to_owned()),
            APP_ID_RUN_AS_VALUE => (!text.eq_ignore_ascii_case(INTERACTIVE_USER))
                .then(|| "it would run as an account of its own".to_owned()),
            APP_ID_LINK_VALUE => {
                (!is_braced_id(text)).then(|| "it does not name an AppID".to_owned())
            }
            other if APP_ID_REMOTE_VALUES.contains(&other) => {
                Some("it would run somewhere other than this PC".to_owned())
            }
            _ => None,
        }
    })
}

/// An ODBC driver loads into anything that uses ODBC, so its driver and setup have to be files this
/// package installs.
fn odbc_driver_refusal(
    name: &str,
    entries: &[&RegistryEntry],
    map: &RestoreMap,
    known: &PackageRegistrations,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    if ODBC_RESERVED_NAMES.contains(&name) {
        return Some("that is ODBC's own setting and not a driver".to_owned());
    }
    if !known.odbc_drivers.contains(name) {
        return Some("its driver is not a file this package installs".to_owned());
    }
    entries.iter().find_map(|entry| {
        let is_a_file = entry.value_name.as_deref().is_some_and(|value| {
            ODBC_DRIVER_FILE_VALUES.contains(&value.to_ascii_lowercase().as_str())
        }) && normalized_key(&entry.key_path).ends_with(&format!("\\{name}"));
        (is_a_file && !odbc_file_is_restored(entry, map, expand))
            .then(|| "it would load a file this package does not install".to_owned())
    })
}

/// A data source names the driver it uses. That can be Windows' own, or one this package installs,
/// by its path or by name.
fn odbc_source_refusal(
    name: &str,
    entries: &[&RegistryEntry],
    map: &RestoreMap,
    known: &PackageRegistrations,
    expand: &dyn Fn(&str) -> String,
    windows_dir: &Path,
) -> Option<String> {
    if ODBC_RESERVED_NAMES.contains(&name) {
        return Some("that is ODBC's own setting and not a data source".to_owned());
    }
    if !known.odbc_sources.contains(name) {
        return Some("it lists a data source this package does not have".to_owned());
    }
    entries.iter().find_map(|entry| {
        let key = normalized_key(&entry.key_path);
        if !key.ends_with(&format!("\\{name}")) {
            return None;
        }
        let value = entry
            .value_name
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let text = text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
        let path = Path::new(text.trim().trim_matches('"'));
        if value == ODBC_DRIVER_FILE_VALUES[0] {
            let fine = map.is_restored_file(path)
                || path_is_strictly_inside(path, windows_dir)
                || known
                    .odbc_drivers
                    .contains(&text.trim().to_ascii_lowercase());
            return (!fine).then(|| {
                "its driver is neither a file this package installs nor part of Windows".to_owned()
            });
        }
        (ODBC_SOURCE_FILE_VALUES.contains(&value.as_str()) && !map.is_restored_file(path))
            .then(|| "it would load a file this package does not install".to_owned())
    })
}

/// Why one value of something that goes into how Windows finds and starts things must be left out,
/// if so. What goes for all of a class or the like is [`registration_refusal`]. What depends on
/// what is on this PC is only asked when installing.
pub fn registered_value_refusal(
    entry: &RegistryEntry,
    map: &RestoreMap,
    known: &PackageRegistrations,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    let key = normalized_key(&entry.key_path);
    let default_text =
        || text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
    let is_default = is_default_value(entry.value_name.as_deref());
    let holds_something = entry.value_kind != Some(RegistryValueKind::None);
    let extension_refusal = || {
        "it would load a shell extension that is not a file this package installs into Explorer"
            .to_owned()
    };
    let is_about_how_things_start = key.starts_with(CLASSES_PREFIX)
        || key.starts_with(&format!("{CLIENTS_KEY}\\"))
        || key.starts_with(EVENT_LOG_PREFIX)
        || key.starts_with(ODBC_DRIVERS_KEY)
        || key.starts_with(ODBC_SOURCES_KEY)
        || key_is_within(&key, ICON_OVERLAYS_KEY)
        || key_is_within(&key, APP_PATHS_KEY)
        || key == FONTS_KEY
        || key == COMPATIBILITY_LAYERS_KEY
        || known
            .capabilities
            .iter()
            .any(|root| key_is_within(&key, root));
    if is_about_how_things_start {
        // These are read as a path or a class's id. What is written can be something else when the
        // rules read the text of a value of another kind.
        let is_read_as_text = is_default
            || key == FONTS_KEY
            || key == COMPATIBILITY_LAYERS_KEY
            || entry
                .value_name
                .as_deref()
                .is_some_and(|name| TEXT_ONLY_VALUES.contains(&name.to_ascii_lowercase().as_str()));
        let is_text = matches!(
            entry.value_kind,
            None | Some(RegistryValueKind::String | RegistryValueKind::ExpandString)
        );
        if is_read_as_text && holds_something && !is_text {
            return Some("what it holds is not the text it would be read as".to_owned());
        }
        // Windows opens an icon just by showing it, which hands a server who is looking.
        if is_text
            && default_text()
                .trim_start_matches(['"', ' ', '@'])
                .replace('/', "\\")
                .starts_with(NETWORK_PATH_START)
        {
            return Some("it names a place on the network".to_owned());
        }
    }

    if let Some(rest) = key.strip_prefix(CLASSES_PREFIX)
        && let Some(at) = rest.find(SHELL_EXTENSIONS_SEGMENT)
        && (rest[at + SHELL_EXTENSIONS_SEGMENT.len()..].is_empty()
            || rest[at + SHELL_EXTENSIONS_SEGMENT.len()..].starts_with('\\'))
    {
        let leaf = rest.rsplit('\\').next().unwrap_or_default();
        let by_value = class_named_by(entry.value_data.as_deref().unwrap_or_default());
        let class = if is_braced_id(&by_value) {
            by_value
        } else {
            leaf.to_owned()
        };
        return (!known.classes.contains(&class)).then(extension_refusal);
    }
    if key_is_within(&key, ICON_OVERLAYS_KEY) && key != ICON_OVERLAYS_KEY {
        return (is_default
            && holds_something
            && !known.classes.contains(&class_named_by(
                entry.value_data.as_deref().unwrap_or_default(),
            )))
        .then(extension_refusal);
    }
    if key == APPROVED_EXTENSIONS_KEY {
        return (!entry
            .value_name
            .as_deref()
            .is_some_and(|name| known.classes.contains(&name.to_ascii_lowercase())))
        .then(extension_refusal);
    }
    for namespace in NAMESPACE_KEYS {
        if let Some(rest) = key.strip_prefix(&format!("{namespace}\\")) {
            let class = rest.split('\\').next().unwrap_or_default();
            return (!known.classes.contains(class)).then(extension_refusal);
        }
    }
    if key == FONTS_KEY {
        return (!map.is_restored_file(Path::new(default_text().trim()))).then(|| {
            "it would register a font this package does not install, or one in the Windows folder"
                .to_owned()
        });
    }
    if key == COMPATIBILITY_LAYERS_KEY {
        let program = text_on_target(entry.value_name.as_deref().unwrap_or_default(), map, expand);
        return (!map.is_restored_file(Path::new(program.trim()))).then(|| {
            "it would change how a program runs that this package does not install".to_owned()
        });
    }
    if key == REGISTERED_APPLICATIONS_KEY {
        let capabilities = normalized_key(entry.value_data.as_deref().unwrap_or_default());
        return (!known.keys.contains(&capabilities))
            .then(|| "it lists an app whose details are not in this package".to_owned());
    }
    if let Some(rest) = key.strip_prefix(&format!("{CLIENTS_KEY}\\"))
        && !rest.contains('\\')
        && is_default
        && holds_something
    {
        return Some("it would change which program opens things by default".to_owned());
    }
    if let Some(class) = key.strip_prefix(CLASSES_PREFIX)
        && !class.contains('\\')
        && entry
            .value_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(URL_PROTOCOL_VALUE))
    {
        return (!known.classes_that_run_it.contains(class)).then(|| {
            "it says it is a link type, and opens with nothing this package installs".to_owned()
        });
    }
    if key.starts_with(CLASSES_PREFIX) || key.starts_with(&format!("{CLIENTS_KEY}\\")) {
        return class_command_refusal(&key, entry, map, known, expand);
    }
    None
}

/// A verb's command, or the class that handles it. Under a URL protocol, or one every app shares,
/// it has to run a file this package installs: a page on the web can ask for one. For a file type
/// of its own, an app can run something through Windows, a script of its own through the host for
/// it, which has to be given that file straight away. A class it names has to be one this package
/// installs.
fn class_command_refusal(
    key: &str,
    entry: &RegistryEntry,
    map: &RestoreMap,
    known: &PackageRegistrations,
    expand: &dyn Fn(&str) -> String,
) -> Option<String> {
    let text = text_on_target(entry.value_data.as_deref().unwrap_or_default(), map, expand);
    let name = entry
        .value_name
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if key.contains(SHELL_SEGMENT) && VERB_HANDLER_VALUES.contains(&name.as_str()) {
        return (!known.classes.contains(&class_named_by(&text))).then(|| {
            "it adds a command handled by something this package does not install".to_owned()
        });
    }
    if key.contains(SHELL_SEGMENT) && key.ends_with(OPEN_COMMAND_TAIL) {
        if name == ELEVATED_COMMAND_VALUE {
            return (!command_runs_restored_file(&text, map)).then(|| {
                "it adds a command that runs something this package does not install".to_owned()
            });
        }
        if name == INSTALLER_COMMAND_VALUE {
            return Some("it would run what only Windows Installer can read".to_owned());
        }
    }
    let class = key.strip_prefix(CLASSES_PREFIX).unwrap_or_default();
    if class
        .strip_suffix(r"\clsid")
        .is_some_and(|prog_id| !prog_id.contains('\\'))
        && is_default_value(entry.value_name.as_deref())
        && entry.value_kind != Some(RegistryValueKind::None)
    {
        return (!known.classes.contains(&class_named_by(&text)))
            .then(|| "it would create a class this package does not install".to_owned());
    }
    if !key.contains(SHELL_SEGMENT)
        || !key.ends_with(OPEN_COMMAND_TAIL)
        || !is_default_value(entry.value_name.as_deref())
        || entry.value_kind == Some(RegistryValueKind::None)
    {
        return None;
    }
    let top = class.split('\\').next().unwrap_or_default();
    let must_run_it = known.url_protocols.contains(top) || SHARED_PROTOCOLS.contains(&top);
    if command_runs_restored_file(&text, map) {
        return None;
    }
    if must_run_it {
        return Some("it opens links with something this package does not install".to_owned());
    }
    (!command_gives_a_restored_file_to_windows(&text, map, &known.windows_dir))
        .then(|| "it opens files with something this package does not install".to_owned())
}

/// Whether a command is a program of Windows' given a file this package installs, straight after
/// its name, as `C:\Windows\System32\wscript.exe "C:\App\a.vbs"` is. Not
/// `cmd.exe /c anything "C:\App\a.txt"`, whose file only comes along, and by its whole path, since
/// a name by itself can be found in the folder of the file being opened.
fn command_gives_a_restored_file_to_windows(
    command: &str,
    map: &RestoreMap,
    windows_dir: &Path,
) -> bool {
    let Some(program) = first_path_in(command) else {
        return false;
    };
    let mut words = command_words(command);
    let _program = words.next();
    program.is_absolute()
        && is_a_program_of_windows(&program, windows_dir)
        && WINDOWS_FILE_HOSTS.contains(&file_name_of(&program).to_ascii_lowercase().as_str())
        && words.next().is_some_and(|word| {
            let file = word.split_once(',').map_or(word, |(file, _)| file);
            map.is_restored_file(Path::new(file))
        })
}

/// The words of a command, without the quotes around any.
fn command_words(command: &str) -> impl Iterator<Item = &str> {
    command.split('"').enumerate().flat_map(|(index, part)| {
        if index % 2 == 1 {
            vec![part]
        } else {
            part.split_whitespace().collect()
        }
    })
}

/// The name a value has on the target. A compatibility setting's name is the program it is for.
pub fn restored_value_name<'a>(
    entry: &'a RegistryEntry,
    map: &RestoreMap,
) -> std::borrow::Cow<'a, str> {
    let name = crate::registry_target::registry_value_name(entry.value_name.as_deref());
    if normalized_key(&entry.key_path) == COMPATIBILITY_LAYERS_KEY {
        std::borrow::Cow::Owned(map.rewrite(name))
    } else {
        std::borrow::Cow::Borrowed(name)
    }
}

/// What a value registers, for the list of what an install will set up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Registration {
    /// A shell extension, by what it is called and the class that it is.
    ShellExtension {
        name: String,
        class: Option<String>,
    },
    /// A command on the menu of a file type or folder.
    ShellCommand {
        on: String,
        verb: String,
    },
    /// What opening a type does, whatever verbs it has.
    DefaultVerb {
        of: String,
    },
    /// Being listed as a browser or mail program.
    ClientEntry {
        name: String,
    },
    /// Who may start a class, or what it runs as.
    AppIdAccess,
    UrlProtocol {
        name: String,
    },
    FileType {
        extension: String,
    },
    ComClass {
        class: String,
    },
    TypeLibrary,
    Interface,
    AppId,
    EventLogSource {
        log: String,
        source: String,
    },
    Font {
        name: String,
    },
    OdbcDriver {
        name: String,
    },
    OdbcSource {
        name: String,
    },
    CompatibilitySetting,
    DefaultAppsEntry {
        name: String,
    },
    ProgramName {
        name: String,
    },
}

/// What one value is part of, if it is part of something an install sets up beyond its files.
pub fn registration_of(entry: &RegistryEntry) -> Option<Registration> {
    let key = normalized_key(&entry.key_path);
    // What it is called, as it was written. The rules go by the lowercase.
    let original_key: String = entry
        .key_path
        .split('\\')
        .filter(|part| !part.is_empty() && !part.eq_ignore_ascii_case("wow6432node"))
        .collect::<Vec<_>>()
        .join("\\");
    let part_after = |prefix: &str| -> String {
        original_key
            .get(prefix.len()..)
            .unwrap_or_default()
            .trim_start_matches('\\')
            .split('\\')
            .next()
            .unwrap_or_default()
            .to_owned()
    };
    let leaf = original_key
        .rsplit('\\')
        .next()
        .unwrap_or_default()
        .to_owned();
    let value_name = entry.value_name.clone().unwrap_or_default();
    if let Some(rest) = key.strip_prefix(CLSID_PREFIX) {
        return Some(Registration::ComClass {
            class: rest.split('\\').next().unwrap_or_default().to_owned(),
        });
    }
    if key.starts_with(INTERFACE_PREFIX) {
        return Some(Registration::Interface);
    }
    if key.starts_with(TYPE_LIBRARY_PREFIX) {
        return Some(Registration::TypeLibrary);
    }
    if key.starts_with(APP_ID_PREFIX) {
        let name = value_name.to_ascii_lowercase();
        return Some(if APP_ID_ACCESS_VALUES.contains(&name.as_str()) {
            Registration::AppIdAccess
        } else {
            Registration::AppId
        });
    }
    if key.starts_with(EVENT_LOG_PREFIX) {
        let below = original_key
            .get(EVENT_LOG_PREFIX.len()..)
            .unwrap_or_default();
        let mut parts = below.split('\\');
        return Some(Registration::EventLogSource {
            log: parts.next().unwrap_or_default().to_owned(),
            source: parts.next().unwrap_or_default().to_owned(),
        });
    }
    if key == FONTS_KEY {
        return Some(Registration::Font { name: value_name });
    }
    if key == COMPATIBILITY_LAYERS_KEY {
        return Some(Registration::CompatibilitySetting);
    }
    if key == REGISTERED_APPLICATIONS_KEY {
        return Some(Registration::DefaultAppsEntry { name: value_name });
    }
    if key_is_within(&key, ICON_OVERLAYS_KEY) && key != ICON_OVERLAYS_KEY {
        return Some(Registration::ShellExtension {
            name: leaf,
            class: Some(class_named_by(
                entry.value_data.as_deref().unwrap_or_default(),
            )),
        });
    }
    if key == APPROVED_EXTENSIONS_KEY {
        return Some(Registration::ShellExtension {
            name: value_name.clone(),
            class: Some(class_named_by(&value_name)),
        });
    }
    for namespace in NAMESPACE_KEYS {
        if let Some(rest) = key.strip_prefix(&format!("{namespace}\\")) {
            let class = rest.split('\\').next().unwrap_or_default().to_owned();
            return Some(Registration::ShellExtension {
                name: class.clone(),
                class: Some(class),
            });
        }
    }
    if key_is_within(&key, ODBC_DRIVERS_KEY) && key != ODBC_DRIVERS_KEY {
        let rest = key.strip_prefix(&format!("{ODBC_DRIVERS_KEY}\\"))?;
        let name = if rest.eq_ignore_ascii_case(ODBC_DRIVER_LIST) {
            value_name
        } else {
            part_after(ODBC_DRIVERS_KEY)
        };
        return Some(Registration::OdbcDriver { name });
    }
    if key_is_within(&key, ODBC_SOURCES_KEY) && key != ODBC_SOURCES_KEY {
        let rest = key.strip_prefix(&format!("{ODBC_SOURCES_KEY}\\"))?;
        let name = if rest.eq_ignore_ascii_case(ODBC_SOURCE_LIST) {
            value_name
        } else {
            part_after(ODBC_SOURCES_KEY)
        };
        return Some(Registration::OdbcSource { name });
    }
    if key.starts_with(&format!("{APP_PATHS_KEY}\\")) {
        return Some(Registration::ProgramName {
            name: part_after(APP_PATHS_KEY),
        });
    }
    if let Some(rest) = key.strip_prefix(&format!("{CLIENTS_KEY}\\")) {
        let below = original_key
            .get(CLIENTS_KEY.len() + 1..)
            .unwrap_or_default();
        return rest.contains('\\').then(|| Registration::ClientEntry {
            name: below.split('\\').take(2).collect::<Vec<_>>().join(": "),
        });
    }
    let class = key.strip_prefix(CLASSES_PREFIX)?;
    let original_class = original_key.get(CLASSES_PREFIX.len()..).unwrap_or_default();
    if let Some(at) = class.find(SHELL_EXTENSIONS_SEGMENT) {
        let by_value = class_named_by(entry.value_data.as_deref().unwrap_or_default());
        return Some(Registration::ShellExtension {
            name: format!("{} on {}", leaf, &original_class[..at]),
            class: Some(if is_braced_id(&by_value) {
                by_value
            } else {
                leaf.to_ascii_lowercase()
            }),
        });
    }
    if value_name.eq_ignore_ascii_case(URL_PROTOCOL_VALUE) && !class.contains('\\') {
        return Some(Registration::UrlProtocol {
            name: original_class.to_owned(),
        });
    }
    if class.starts_with('.')
        && !class.contains('\\')
        && is_default_value(entry.value_name.as_deref())
    {
        return Some(Registration::FileType {
            extension: original_class.to_owned(),
        });
    }
    if let Some(of) = class.strip_suffix(r"\shell")
        && is_default_value(entry.value_name.as_deref())
        && entry.value_kind != Some(RegistryValueKind::None)
    {
        return Some(Registration::DefaultVerb {
            of: original_class[..of.len()].to_owned(),
        });
    }
    let lower_value_name = value_name.to_ascii_lowercase();
    let handled = VERB_HANDLER_VALUES.contains(&lower_value_name.as_str())
        || VERB_MENU_VALUES.contains(&lower_value_name.as_str());
    if let Some(at) = class.find(SHELL_SEGMENT)
        && (class.ends_with(OPEN_COMMAND_TAIL) || handled)
    {
        let on = original_class[..at].to_owned();
        let verb = original_class[at + SHELL_SEGMENT.len()..]
            .split('\\')
            .next()
            .unwrap_or_default()
            .to_owned();
        return Some(Registration::ShellCommand { on, verb });
    }
    None
}

/// Why a firewall rule must not be restored, if so. It has to be for one program, which is a file
/// this package installs: never every program, which a rule with none is. It cannot override other
/// rules: what a rule with `AllowBypass` does. And it has to be something `netsh` can set again as
/// it was, or it would mean something else: `Edge` lets things in through NAT, and `Security` and the
/// rest change who it applies to. Every part goes to `netsh` as a word, so each is held to what such
/// a part can be.
pub fn firewall_refusal(
    rule: &FirewallRuleEntry,
    map: &RestoreMap,
    restored_services: &HashSet<String>,
) -> Option<String> {
    let name = rule.name.trim();
    if name.is_empty()
        || name.eq_ignore_ascii_case(FIREWALL_RESERVED_NAME)
        || name.len() > FIREWALL_RULE_NAME_LIMIT
        || name
            .chars()
            .any(|character| character.is_control() || character == '"')
    {
        return Some("its name is not one this can pass on".to_owned());
    }
    if rule.description.as_deref().is_some_and(|text| {
        text.len() > FIREWALL_DESCRIPTION_LIMIT
            || text
                .chars()
                .any(|character| character.is_control() || character == '"')
    }) {
        return Some("its description is not one this can pass on".to_owned());
    }
    if rule.program.as_os_str().is_empty() {
        return Some("it applies to every program, and a rule can only be for one".to_owned());
    }
    if map.destination_of(&rule.program).is_none() {
        return Some(format!(
            "it is for {}, which this package does not install",
            rule.program.display()
        ));
    }
    if !FIREWALL_DIRECTIONS.contains(&rule.direction.to_ascii_lowercase().as_str()) {
        return Some("it does not say a direction this can set".to_owned());
    }
    if !FIREWALL_ACTIONS.contains(&rule.action.to_ascii_lowercase().as_str()) {
        return Some("it would override other rules, which no app's rule should".to_owned());
    }
    if let Some(reach) = rule.not_carried.first() {
        return Some(format!(
            "it says {reach}, which could not be set as it was, or would weaken what other rules \
             do"
        ));
    }
    if rule
        .service
        .as_deref()
        .is_some_and(|service| !restored_services.contains(&service.to_ascii_lowercase()))
    {
        return Some("it is for a service this package does not restore".to_owned());
    }
    let is_word = |text: &str| {
        !text.is_empty()
            && text
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || "-.:/".contains(character))
    };
    let parts_fine = rule.protocol.iter().all(|protocol| is_word(protocol))
        && rule
            .local_ports
            .iter()
            .chain(&rule.remote_ports)
            .chain(&rule.local_addresses)
            .chain(&rule.remote_addresses)
            .all(|part| is_word(part))
        && rule
            .profiles
            .iter()
            .all(|profile| FIREWALL_PROFILES.contains(&profile.to_ascii_lowercase().as_str()));
    (!parts_fine).then(|| "part of it is not what a firewall rule can say".to_owned())
}

/// What tells rules of one name, way, and program apart: an app can have one for TCP and one for
/// UDP, or one for each port.
pub fn firewall_identity(
    protocol: Option<&str>,
    local_ports: &[String],
    remote_ports: &[String],
) -> String {
    // A port goes by its number however it was written, as Windows keeps it.
    let sorted = |parts: &[String]| {
        let mut parts: Vec<String> = parts
            .iter()
            .map(|part| {
                let part = part.trim().to_ascii_lowercase();
                let without_zeros = part.trim_start_matches('0');
                if without_zeros.is_empty() {
                    part
                } else {
                    without_zeros.to_owned()
                }
            })
            .collect();
        parts.sort_unstable();
        parts.join(",")
    };
    format!(
        "{}|{}|{}",
        stored_protocol(protocol.unwrap_or_default()),
        sorted(local_ports),
        sorted(remote_ports)
    )
}

/// What a rule does, in the terms Windows has for it, to tell whether a rule is still as written.
/// The same rule set again, or read back, comes out the same.
pub fn firewall_fingerprint(rule: &FirewallRuleEntry) -> String {
    format!(
        "{}|{}|{}",
        rule.action.to_ascii_lowercase(),
        rule.enabled,
        firewall_identity(
            rule.protocol.as_deref(),
            &rule.local_ports,
            &rule.remote_ports
        )
    )
}

/// A protocol as `netsh` takes one with ports: it goes by the name for these two, and Windows stores
/// the number.
fn netsh_protocol(protocol: &str) -> String {
    match protocol.trim() {
        "6" => "tcp".to_owned(),
        "17" => "udp".to_owned(),
        other => other.to_ascii_lowercase(),
    }
}

/// A protocol as Windows stores one, which is what a rule read back says. Any is not said at all.
fn stored_protocol(protocol: &str) -> String {
    match netsh_protocol(protocol).as_str() {
        "" | FIREWALL_ANY => String::new(),
        "tcp" => "6".to_owned(),
        "udp" => "17".to_owned(),
        "icmpv4" => "1".to_owned(),
        "icmpv6" => "58".to_owned(),
        other => other.to_owned(),
    }
}

/// What to give `netsh advfirewall firewall add rule`. Each is one argument, which is how it reads
/// them whatever they hold, and [`firewall_refusal`] has held them to what a rule can say.
pub fn netsh_add_rule_arguments(rule: &FirewallRuleEntry, program: &Path) -> Vec<String> {
    let list = |parts: &[String]| parts.join(",").to_ascii_lowercase();
    let mut arguments = vec![
        "advfirewall".to_owned(),
        "firewall".to_owned(),
        "add".to_owned(),
        "rule".to_owned(),
        format!("name={}", rule.name.trim()),
        format!("dir={}", rule.direction.to_ascii_lowercase()),
        format!("action={}", rule.action.to_ascii_lowercase()),
        format!("program={}", program.display()),
        format!("enable={}", if rule.enabled { "yes" } else { "no" }),
        format!(
            "profile={}",
            if rule.profiles.is_empty() {
                "any".to_owned()
            } else {
                list(&rule.profiles)
            }
        ),
        format!(
            "protocol={}",
            netsh_protocol(rule.protocol.as_deref().unwrap_or("any"))
        ),
    ];
    for (name, parts) in [
        ("localport", &rule.local_ports),
        ("remoteport", &rule.remote_ports),
        ("localip", &rule.local_addresses),
        ("remoteip", &rule.remote_addresses),
    ] {
        if !parts.is_empty() {
            arguments.push(format!("{name}={}", list(parts)));
        }
    }
    if let Some(service) = rule.service.as_deref() {
        arguments.push(format!("service={service}"));
    }
    if let Some(description) = rule.description.as_deref() {
        arguments.push(format!("description={description}"));
    }
    arguments
}

/// What to give `netsh` to take out what an install added, and nothing else: a rule of the same name
/// for another program, the other way, or another protocol or port stays.
pub fn netsh_delete_rule_arguments(
    name: &str,
    direction: &str,
    program: &Path,
    protocol: Option<&str>,
    local_ports: &[String],
    remote_ports: &[String],
) -> Vec<String> {
    let mut arguments = vec![
        "advfirewall".to_owned(),
        "firewall".to_owned(),
        "delete".to_owned(),
        "rule".to_owned(),
        format!("name={name}"),
        format!("dir={}", direction.to_ascii_lowercase()),
        format!("program={}", program.display()),
    ];
    if let Some(protocol) = protocol {
        arguments.push(format!("protocol={}", netsh_protocol(protocol)));
    }
    for (option, ports) in [("localport", local_ports), ("remoteport", remote_ports)] {
        if !ports.is_empty() {
            arguments.push(format!("{option}={}", ports.join(",").to_ascii_lowercase()));
        }
    }
    arguments
}

/// Whether what Windows has stored is the same rule: the same name, direction, and program.
pub fn stored_firewall_rule_matches(
    stored: &str,
    name: &str,
    direction: &str,
    program: &Path,
) -> bool {
    parse_firewall_rule(stored).is_some_and(|rule| {
        rule.name.trim().eq_ignore_ascii_case(name.trim())
            && rule.direction.eq_ignore_ascii_case(direction)
            && paths_equal(&rule.program, program)
    })
}

/// Why overwriting a COM server already registered on this PC must be refused, if so.
/// Registering a class of the app's own is normal, and its server may well be in Windows,
/// like `mscoree.dll` for .NET. Taking over a class that is already something else's
/// is how code gets loaded into Explorer, and `--force` does not make it right.
pub fn com_server_refusal(
    server_key: &str,
    existing: &str,
    new_value: &str,
    target_root: &Path,
    windows_dir: &Path,
) -> Option<String> {
    let existing = existing.trim();
    let is_default_handler = server_key.eq_ignore_ascii_case(COM_HANDLER_KEY)
        && existing.eq_ignore_ascii_case(COM_DEFAULT_HANDLER);
    if existing.is_empty() || existing.eq_ignore_ascii_case(new_value.trim()) || is_default_handler
    {
        return None;
    }
    let server = parse_image_path(existing, windows_dir).path;
    (!path_is_strictly_inside(&server, target_root)).then(|| {
        format!(
            "this COM class already belongs to {existing}, which is not from an earlier install \
             here"
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

/// Whether a file runs, or decides what runs in a program beside it.
fn is_runnable_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let name = name.to_ascii_lowercase();
    RUNNABLE_NAME_ENDS.iter().any(|end| name.ends_with(end))
        || path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                RUNNABLE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
            })
}

/// Why a program must not go where it would, if so. Outside the install location a package says
/// where a file goes, and nothing on the list a person agrees to says so. A program there would run in
/// place of the one it replaced, or be loaded by one beside it, as another app's service might. So
/// none goes in a folder that was already there, unless a record says an install was at this
/// location. `was_there` says whether the folder it goes under, first below the data folder, was
/// there before this install began. A fresh PC has none of them. A person's data is what goes
/// in those folders, and it can be replaced, and comes back on uninstall.
pub fn program_placement_refusal(
    destination: &Path,
    target_root: &Path,
    earlier_install_here: bool,
    was_there: bool,
) -> Option<String> {
    (was_there
        && !earlier_install_here
        && is_runnable_file(destination)
        && !path_is_strictly_inside(destination, target_root))
    .then(|| {
        "that folder was already there, outside the install location, and nothing says it is this \
         app's, so no program goes in it"
            .to_owned()
    })
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
pub(crate) fn node_text(node: roxmltree::Node<'_, '_>) -> String {
    node.children()
        .filter(roxmltree::Node::is_text)
        .filter_map(|child| child.text())
        .collect::<String>()
        .trim()
        .to_owned()
}

/// True for the names SYSTEM can be given in a task: by SID, or by name, alone or after any
/// authority (`NT AUTHORITY\\SYSTEM`, `.\\SYSTEM`, `PCNAME\\SYSTEM`).
pub(crate) fn is_system_account(account: &str) -> bool {
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
                "a task with this name already exists and runs {path}, which is not from an \
                 earlier install here"
            )
        })
    })
}

/// What restoring needs to know about the account it restores to, and the folders shortcuts may
/// be put in, on this PC.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnownFolders {
    pub user_desktop: Option<PathBuf>,
    pub common_desktop: Option<PathBuf>,
    pub user_start_menu: Option<PathBuf>,
    pub common_start_menu: Option<PathBuf>,
    /// Whether there is an account to put per-user settings in.
    pub user_account_known: bool,
    /// What to say about whose it is, when that is not simply whoever is running this.
    pub user_account_note: Option<String>,
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
    if REFUSED_DOCUMENTS_FOLDERS
        .iter()
        .any(|folder| lower.starts_with(folder))
    {
        return Some("it would go where PowerShell runs what it finds when it starts".to_owned());
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
    fn what_com_gives_every_class_says_nothing_about_whose_it_is_as_a_handler_only() {
        let target = Path::new(r"C:\Apps\Acme");
        let windows = Path::new(WINDOWS);
        let new = r"C:\Apps\Acme\a.dll";
        for own in ["ole32.dll", "OLE32.DLL", " ole32.dll "] {
            assert_eq!(
                com_server_refusal("inprochandler32", own, new, target, windows),
                None
            );
            assert!(
                com_server_refusal("inprocserver32", own, new, target, windows).is_some(),
                "a class can have it as its only server"
            );
        }
        assert!(
            com_server_refusal("inprochandler32", r"C:\Other\b.dll", new, target, windows)
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
            com_server_refusal("inprocserver32", "", r"D:\Apps\Acme\a.dll", target, windows),
            None
        );
        assert_eq!(
            com_server_refusal(
                "inprocserver32",
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
                "inprocserver32",
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
                "inprocserver32",
                r"%SystemRoot%\System32\shell32.dll",
                r"D:\Apps\Acme\a.dll",
                target,
                windows
            )
            .is_some()
        );
        assert_eq!(
            com_server_refusal(
                "inprocserver32",
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
                "inprocserver32",
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

    #[test]
    fn what_powershell_runs_by_itself_is_refused_in_a_users_documents() {
        for tail in [
            r"WindowsPowerShell\Microsoft.PowerShell_profile.ps1",
            r"PowerShell\profile.ps1",
            r"powershell\Modules\Evil\Evil.psm1",
        ] {
            assert!(data_location_refusal(Path::new(tail)).is_some(), "{tail}");
        }
        for tail in [
            r"Acme\settings.json",
            r"My Games\Acme\save.dat",
            r"Acme\PowerShell\x",
        ] {
            assert_eq!(data_location_refusal(Path::new(tail)), None, "{tail}");
        }
    }

    fn shortcut_folders() -> KnownFolders {
        KnownFolders {
            user_desktop: Some(PathBuf::from(r"C:\Users\bob\Desktop")),
            common_desktop: Some(PathBuf::from(r"C:\Users\Public\Desktop")),
            user_start_menu: Some(PathBuf::from(r"C:\Users\bob\Start")),
            common_start_menu: Some(PathBuf::from(r"C:\ProgramData\Start")),
            ..KnownFolders::default()
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

    // ---- What goes into how Windows finds and starts things -------------------------------

    const SERVER: &str = r"C:\Program Files\Acme\bin\host.dll";
    const CLASS: &str = "{5a7e6e0c-e2e0-4000-8000-000000000001}";
    const OTHER_CLASS: &str = "{5a7e6e0c-e2e0-4000-8000-00000000dead}";

    fn no_expanding(text: &str) -> String {
        text.to_owned()
    }

    fn string_value(
        hive: RegistryHive,
        key: &str,
        name: Option<&str>,
        data: &str,
    ) -> RegistryEntry {
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

    fn machine(key: &str, name: Option<&str>, data: &str) -> RegistryEntry {
        string_value(RegistryHive::LocalMachine, key, name, data)
    }

    /// A package with one class, whose server is a file it installs.
    fn known_of(extra: &[RegistryEntry]) -> PackageRegistrations {
        let class = machine(
            &format!(r"Software\Classes\CLSID\{CLASS}\InprocServer32"),
            None,
            SERVER,
        );
        let mut entries: Vec<&RegistryEntry> = vec![&class];
        entries.extend(extra);
        let destinations = [Path::new(r"D:\Apps\Acme\acme.exe")];
        PackageRegistrations::of(
            &entries,
            &destinations,
            &app_map(),
            &no_expanding,
            Path::new(WINDOWS),
        )
    }

    fn value_refusal(entry: &RegistryEntry) -> Option<String> {
        registered_value_refusal(entry, &app_map(), &known_of(&[]), &no_expanding)
    }

    fn unit_refusal(entries: &[RegistryEntry]) -> Option<String> {
        let of_it: Vec<&RegistryEntry> = entries.iter().collect();
        let known = known_of(entries);
        let unit = registration_unit_of(entries.first()?)?;
        registration_refusal(
            &unit,
            &of_it,
            &app_map(),
            &known,
            &HashSet::from(["acmesvc".to_owned()]),
            &no_expanding,
            Path::new(WINDOWS),
        )
    }

    #[test]
    fn a_class_runs_a_file_of_the_package_or_something_of_windows_own() {
        let server = |key: &str, data: &str| {
            machine(
                &format!(r"Software\Classes\CLSID\{CLASS}\{key}"),
                None,
                data,
            )
        };
        for fine in [
            server("InprocServer32", SERVER),
            server("InprocServer32", &format!("\"{SERVER}\"")),
            server(
                "LocalServer32",
                r#""C:\Program Files\Acme\acme.exe" /automation"#,
            ),
            // What .NET has as the server of every class it hosts.
            server("InprocServer32", "mscoree.dll"),
            server("InprocServer32", r"C:\Windows\System32\mscoree.dll"),
            server("InprocHandler32", "ole32.dll"),
        ] {
            assert_eq!(unit_refusal(std::slice::from_ref(&fine)), None, "{fine:?}");
        }
        for refused in [
            server("InprocServer32", r"\\server\share\evil.dll"),
            server("InprocServer32", r"C:\Elsewhere\evil.dll"),
            server("LocalServer32", r"C:\ProgramData\evil.exe"),
            server("TreatAs", OTHER_CLASS),
            // It looks like it is in the Windows folder, and it is not.
            server("InprocServer32", r"C:\Windows\..\Users\Public\evil.dll"),
            // Part of Windows, and it runs whatever it is told.
            server("InprocServer32", r"C:\Windows\System32\scrobj.dll"),
            server("LocalServer32", "cmd.exe /c anything"),
            server("LocalServer32", r"C:\Windows\System32\cmd.exe"),
            server("InprocServer32", "mscoree.dll, and then"),
        ] {
            assert!(
                unit_refusal(std::slice::from_ref(&refused)).is_some(),
                "{refused:?}"
            );
        }
        assert_eq!(
            unit_refusal(&[server("TreatAs", CLASS)]),
            None,
            "it can be another of its own"
        );
    }

    #[test]
    fn a_class_counts_as_the_packages_only_if_all_its_servers_are() {
        let key = |server: &str| format!(r"Software\Classes\CLSID\{OTHER_CLASS}\{server}");
        let handler = machine(
            r"Software\Classes\*\shellex\ContextMenuHandlers\Acme",
            None,
            OTHER_CLASS,
        );
        let own = machine(
            &key("LocalServer32"),
            None,
            r"C:\Program Files\Acme\acme.exe",
        );
        let refusal_with = |also: &[RegistryEntry]| {
            let mut entries = vec![own.clone()];
            entries.extend_from_slice(also);
            registered_value_refusal(&handler, &app_map(), &known_of(&entries), &no_expanding)
        };
        assert_eq!(refusal_with(&[]), None);
        assert!(
            refusal_with(&[machine(
                &key("InprocServer32"),
                None,
                r"C:\Windows\System32\scrobj.dll"
            )])
            .is_some(),
            "Explorer would load the one that is not the package's"
        );
        let none_kind = RegistryEntry {
            value_kind: Some(RegistryValueKind::None),
            ..machine(
                &format!(r"Software\Classes\CLSID\{CLASS}2\InprocServer32"),
                None,
                r"C:\Program Files\Acme\acme.exe",
            )
        };
        let known = known_of(std::slice::from_ref(&none_kind));
        assert!(
            !known.classes.contains(&format!("{CLASS}2")),
            "a key with no value says nothing, however it came to have text"
        );
    }

    #[test]
    fn what_a_dot_net_class_loads_has_to_be_the_packages_too() {
        let of_class = |name: &str, data: &str| {
            machine(
                &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32"),
                Some(name),
                data,
            )
        };
        let host = machine(
            &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32"),
            None,
            "mscoree.dll",
        );
        assert_eq!(
            unit_refusal(&[
                host.clone(),
                of_class("CodeBase", "file:///C:/Program Files/Acme/bin/host.dll")
            ]),
            None
        );
        for refused in [
            of_class("CodeBase", "file:///C:/Elsewhere/evil.dll"),
            of_class("CodeBase", "http://evil.example/evil.dll"),
            of_class("CodeBase", r"file://///evil/share/evil.dll"),
        ] {
            assert!(
                unit_refusal(&[host.clone(), refused.clone()]).is_some(),
                "{refused:?}"
            );
        }
        let versioned = machine(
            &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32\1.0.0.0"),
            Some("CodeBase"),
            "file:///C:/Elsewhere/evil.dll",
        );
        assert!(unit_refusal(&[host, versioned]).is_some());
        let local = machine(
            &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\LocalServer32"),
            Some("ServerExecutable"),
            r"C:\Elsewhere\evil.exe",
        );
        assert!(unit_refusal(std::slice::from_ref(&local)).is_some());
    }

    #[test]
    fn a_verb_runs_nothing_else_beside_its_command() {
        let command = r"Software\Classes\Directory\shell\acme\command";
        let run_it = r#""C:\Program Files\Acme\acme.exe" "%V""#;
        assert_eq!(
            value_refusal(&machine(command, Some("IsolatedCommand"), run_it)),
            None
        );
        assert!(
            value_refusal(&machine(
                command,
                Some("IsolatedCommand"),
                r"C:\Windows\System32\cmd.exe /c anything"
            ))
            .is_some()
        );
        assert!(value_refusal(&machine(command, Some("command"), "a]darwin>descriptor")).is_some());
        let store = machine(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\CommandStore\shell\acme.x\command",
            None,
            r"C:\Windows\System32\cmd.exe /c anything",
        );
        assert!(registry_refusal(&store, &app_map(), &HashSet::new(), &no_expanding).is_some());
        assert_eq!(
            registration_of(&machine(
                r"Software\Classes\Directory\shell\acme",
                Some("SubCommands"),
                "acme.x"
            )),
            Some(Registration::ShellCommand {
                on: "Directory".to_owned(),
                verb: "acme".to_owned()
            })
        );
    }

    #[test]
    fn no_program_goes_in_a_data_folder_that_was_already_there() {
        let target = Path::new(r"C:\Program Files\Acme");
        let refusal = |path: &str, earlier: bool, was_there: bool| {
            program_placement_refusal(Path::new(path), target, earlier, was_there)
        };
        for program in [
            r"C:\ProgramData\Other\svc.exe",
            r"C:\ProgramData\Other\VERSION.DLL",
            r"C:\ProgramData\Other\svc.exe.config",
            r"C:\ProgramData\Other\svc.runtimeconfig.json",
            r"C:\ProgramData\Other\svc.deps.json",
            r"C:\ProgramData\Other\python39._pth",
            r"C:\Users\me\AppData\Local\Programs\Other\resources\app.asar",
            r"C:\ProgramData\Other\native.node",
            r"C:\ProgramData\Other\mod.pyd",
        ] {
            assert!(refusal(program, false, true).is_some(), "{program}");
            assert_eq!(
                refusal(program, false, false),
                None,
                "a folder that was not there has no program in it to load this: {program}"
            );
            assert_eq!(
                refusal(program, true, true),
                None,
                "a record says what is there is an install's: {program}"
            );
        }
        assert_eq!(
            refusal(r"C:\Program Files\Acme\a.exe", false, true),
            None,
            "the install location is asked about by itself"
        );
        for data in [
            r"C:\ProgramData\Other\settings.ini",
            r"C:\Users\me\AppData\Local\Other\user.config",
        ] {
            assert_eq!(refusal(data, false, true), None, "{data}");
        }
    }

    #[test]
    fn a_program_is_named_by_a_files_name() {
        let named = |name: &str| {
            let key = format!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\{name}");
            machine(&key, None, r"C:\Program Files\Acme\acme.exe")
        };
        let refusal = |entry: &RegistryEntry| {
            registry_refusal(entry, &app_map(), &HashSet::new(), &no_expanding)
        };
        assert_eq!(refusal(&named("acme.exe")), None);
        for name in ["//evil/share/x.exe", "C:x.exe", "a/b.exe"] {
            assert!(refusal(&named(name)).is_some(), "{name}");
        }
    }

    #[test]
    fn what_opens_a_program_is_for_a_program_of_the_package() {
        let of = |program: &str| {
            machine(
                &format!(r"Software\Classes\Applications\{program}\shell\open\command"),
                None,
                r#""C:\Program Files\Acme\acme.exe" "%1""#,
            )
        };
        assert_eq!(unit_refusal(&[of("acme.exe")]), None);
        assert!(
            unit_refusal(&[of("winword.exe")]).is_some(),
            "that would change what files Word was chosen for open with"
        );
    }

    #[test]
    fn a_program_of_windows_is_where_windows_keeps_them_and_nowhere_else_in_it() {
        let windows = Path::new(WINDOWS);
        for yes in [
            "wscript.exe",
            r"C:\Windows\System32\wscript.exe",
            r"C:\Windows\SysWOW64\wscript.exe",
            r"C:\Windows\explorer.exe",
        ] {
            assert!(is_a_program_of_windows(Path::new(yes), windows), "{yes}");
        }
        for no in [
            r"C:\Windows\Temp\wscript.exe",
            r"C:\Windows\..\Users\Public\wscript.exe",
            r"..\wscript.exe",
            r"\\evil\share\wscript.exe",
        ] {
            assert!(!is_a_program_of_windows(Path::new(no), windows), "{no}");
        }
        let server = |data: &str| {
            machine(
                &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32"),
                None,
                data,
            )
        };
        assert!(unit_refusal(&[server(r"C:\Windows\Temp\shell32.dll")]).is_some());
    }

    #[test]
    fn every_way_an_interface_is_asked_for_has_to_be_the_packages() {
        let key = |tail: &str| {
            format!(r"Software\Classes\Interface\{{5a7e6e0c-e2e0-4000-8000-0000000000cc}}\{tail}")
        };
        let good = machine(&key("ProxyStubClsid"), None, CLASS);
        let bad = machine(&key("ProxyStubClsid32"), None, OTHER_CLASS);
        assert_eq!(unit_refusal(std::slice::from_ref(&good)), None);
        assert!(
            unit_refusal(&[good, bad]).is_some(),
            "the one a program goes by is the other one"
        );
    }

    #[test]
    fn a_new_place_under_the_old_one_is_moved_once() {
        let mut map = RestoreMap::for_tests(r"C:\App\New");
        map.add_test_root(r"C:\App", r"C:\App\New");
        map.add_test_file(r"C:\App\a.exe", r"C:\App\New\a.exe");
        let command = machine(r"Software\Acme", None, r"C:\App\a.exe --d %APPDATA%\x");
        let expand = |text: &str| text.replace("%APPDATA%", r"C:\Users\me\AppData\Roaming");
        assert_eq!(
            text_on_target(
                command.value_data.as_deref().unwrap_or_default(),
                &map,
                &expand
            ),
            r"C:\App\New\a.exe --d C:\Users\me\AppData\Roaming\x"
        );
        assert_eq!(
            restored_value_text(&command, &map),
            r"C:\App\New\a.exe --d %APPDATA%\x",
            "what is not the app's path keeps its variable"
        );
    }

    #[test]
    fn a_value_is_not_read_as_text_when_it_would_be_written_as_something_else() {
        let as_bytes = RegistryEntry {
            value_kind: Some(RegistryValueKind::MultiString),
            value_bytes_hex: Some("00".to_owned()),
            ..machine(
                r"Software\Classes\Acme.File\shell\open\command",
                None,
                r#""C:\Program Files\Acme\acme.exe""#,
            )
        };
        assert!(value_refusal(&as_bytes).is_some());
        let flags = RegistryEntry {
            value_kind: Some(RegistryValueKind::Binary),
            ..machine(r"Software\Classes\Acme.File", Some("EditFlags"), "x")
        };
        assert_eq!(
            value_refusal(&flags),
            None,
            "this one is only read as what it is"
        );
    }

    #[test]
    fn nothing_registered_names_a_place_on_the_network() {
        for (key, name) in [
            (r"Software\Classes\Acme.File\DefaultIcon", None),
            (r"Software\Classes\CLSID\{a}\DefaultIcon", None),
            (r"Software\Classes\Acme.File\shell\open", Some("Icon")),
            (r"Software\Clients\Media\Acme\DefaultIcon", None),
            (
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\acme.exe",
                Some("Path"),
            ),
        ] {
            let entry = machine(key, name, r"\\evil\s\i.ico");
            assert!(value_refusal(&entry).is_some(), "{key}");
            let quoted = machine(key, name, r#""\\evil\s\i.ico",0"#);
            assert!(value_refusal(&quoted).is_some(), "{key}");
        }
        assert!(
            value_refusal(&machine(
                r"Software\Classes\Acme.File",
                Some("FriendlyTypeName"),
                r"@\\evil\s\x.dll,-1"
            ))
            .is_some()
        );
    }

    #[test]
    fn a_value_name_that_is_not_the_name_it_would_be_written_as_is_refused() {
        for name in ["StubPath\0x", "\0", "DbgManagedDebugger\0", "a\nb"] {
            let entry = machine(r"Software\Acme", Some(name), "x");
            assert!(
                registry_refusal(&entry, &app_map(), &HashSet::new(), &no_expanding).is_some(),
                "{name:?}"
            );
        }
    }

    #[test]
    fn a_path_with_a_variable_is_read_and_moved_like_any_other() {
        let expand = |text: &str| text.replace("%ProgramFiles%", r"C:\Program Files");
        assert_eq!(
            text_on_target(r"%ProgramFiles%\Acme\acme.exe", &app_map(), &expand),
            r"D:\Apps\Acme\acme.exe"
        );
        assert_eq!(
            text_on_target(r"%ProgramFiles%\Other\x.exe", &app_map(), &expand),
            r"C:\Program Files\Other\x.exe"
        );
    }

    #[test]
    fn a_command_windows_runs_as_written_gets_its_quotes_where_the_app_moved() {
        let moved = {
            let mut map = RestoreMap::for_tests(r"D:\My Apps\Acme");
            map.add_test_root(r"C:\Program Files\Acme", r"D:\My Apps\Acme");
            map.add_test_file(
                r"C:\Program Files\Acme\acme.exe",
                r"D:\My Apps\Acme\acme.exe",
            );
            map
        };
        let command = machine(
            r"Software\Classes\Acme.File\shell\open\command",
            None,
            r"C:\Program Files\Acme\acme.exe %1",
        );
        assert_eq!(
            restored_value_text(&command, &moved),
            r#""D:\My Apps\Acme\acme.exe" %1"#
        );
        let server = machine(
            r"Software\Classes\CLSID\{a}\LocalServer32",
            None,
            r"C:\Program Files\Acme\acme.exe /automation",
        );
        assert_eq!(
            restored_value_text(&server, &moved),
            r#""D:\My Apps\Acme\acme.exe" /automation"#
        );
    }

    #[test]
    fn all_of_a_class_goes_with_the_part_that_cannot_be_had() {
        let bad = machine(
            &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32"),
            None,
            r"\\server\share\evil.dll",
        );
        let beside_it = machine(
            &format!(r"Software\Classes\CLSID\{OTHER_CLASS}\InprocServer32"),
            Some("ThreadingModel"),
            "Apartment",
        );
        let unit = registration_unit_of(&bad);
        assert_eq!(unit, registration_unit_of(&beside_it));
        assert!(unit_refusal(&[bad, beside_it]).is_some());
    }

    #[test]
    fn a_32_bit_class_is_its_own_and_goes_by_the_same_rules() {
        let wow = machine(
            &format!(r"Software\Classes\WOW6432Node\CLSID\{OTHER_CLASS}\InprocServer32"),
            None,
            r"C:\Elsewhere\evil.dll",
        );
        assert!(unit_refusal(&[wow]).is_some());
    }

    #[test]
    fn how_calls_to_an_interface_are_made_has_to_be_the_packages() {
        let iid = "{5a7e6e0c-e2e0-4000-8000-0000000000aa}";
        let library = "{5a7e6e0c-e2e0-4000-8000-0000000000bb}";
        let key = |tail: &str| format!(r"Software\Classes\Interface\{iid}\{tail}");
        let stub = |class: &str| machine(&key("ProxyStubClsid32"), None, class);
        assert_eq!(unit_refusal(&[stub(CLASS)]), None);
        assert!(
            unit_refusal(&[stub(OTHER_CLASS)]).is_some(),
            "this is how a DLL gets loaded into every program using a Windows interface"
        );
        assert!(
            unit_refusal(&[stub("{00020424-0000-0000-C000-000000000046}")]).is_some(),
            "Windows' marshaler, and no type library of its own"
        );
        let with_library = [
            stub("{00020424-0000-0000-C000-000000000046}"),
            machine(&key("TypeLib"), None, library),
            machine(
                &format!(r"Software\Classes\TypeLib\{library}\1.0\0\win64"),
                None,
                SERVER,
            ),
        ];
        assert_eq!(unit_refusal(&with_library), None);
    }

    #[test]
    fn a_type_library_is_a_file_of_the_package() {
        let library = |path: &str| {
            machine(
                &format!(r"Software\Classes\TypeLib\{CLASS}\1.0\0\win64"),
                None,
                path,
            )
        };
        assert_eq!(unit_refusal(&[library(SERVER)]), None);
        assert!(unit_refusal(&[library(r"C:\Windows\System32\stdole2.tlb")]).is_some());
        assert_eq!(
            unit_refusal(&[machine(
                &format!(r"Software\Classes\TypeLib\{CLASS}\1.0\HELPDIR"),
                None,
                r"C:\Anywhere",
            )]),
            None,
            "this names a folder for help, and nothing loads from it"
        );
    }

    #[test]
    fn an_app_id_cannot_reach_past_the_package() {
        let of_id = |name: &str, data: &str| {
            machine(
                &format!(r"Software\Classes\AppID\{CLASS}"),
                Some(name),
                data,
            )
        };
        for fine in [
            of_id("DllSurrogate", ""),
            of_id("DllSurrogate", r"C:\Program Files\Acme\acme.exe"),
            of_id("LocalService", "AcmeSvc"),
            of_id("RunAs", "Interactive User"),
            of_id("AccessPermission", "x"),
        ] {
            assert_eq!(unit_refusal(std::slice::from_ref(&fine)), None, "{fine:?}");
        }
        for refused in [
            of_id("DllSurrogate", r"C:\Windows\System32\cmd.exe"),
            of_id("LocalService", "Spooler"),
            of_id("RunAs", "nt authority\\system"),
            of_id("RunAs", r"CONTOSO\admin"),
            of_id("RemoteServerName", "evil.example"),
            of_id("ActivateAtStorage", "Y"),
        ] {
            assert!(
                unit_refusal(std::slice::from_ref(&refused)).is_some(),
                "{refused:?}"
            );
        }
        let by_program = |program: &str| {
            machine(
                &format!(r"Software\Classes\AppID\{program}"),
                Some("AppID"),
                CLASS,
            )
        };
        assert_eq!(unit_refusal(&[by_program("acme.exe")]), None);
        assert!(
            unit_refusal(&[by_program("explorer.exe")]).is_some(),
            "that would change how Explorer is started, by its file name alone"
        );
    }

    #[test]
    fn an_event_log_source_is_only_a_new_one_with_its_own_message_files() {
        let source = r"SYSTEM\CurrentControlSet\Services\EventLog\Application\Acme";
        let services = HashSet::new();
        let refusal =
            |entry: &RegistryEntry| registry_refusal(entry, &app_map(), &services, &no_expanding);
        assert_eq!(
            refusal(&machine(source, Some("EventMessageFile"), SERVER)),
            None
        );
        assert_eq!(
            refusal(&machine(
                source,
                Some("CategoryMessageFile"),
                &format!(r"{SERVER};C:\Program Files\Acme\acme.exe")
            )),
            None
        );
        for refused in [
            machine(
                source,
                Some("EventMessageFile"),
                r"C:\Windows\System32\evil.dll",
            ),
            machine(
                source,
                Some("EventMessageFile"),
                &format!(r"{SERVER};C:\Elsewhere\evil.dll"),
            ),
            machine(source, Some("ImagePath"), r"C:\Windows\System32\cmd.exe"),
            machine(source, Some("CustomSD"), "O:BAG:SYD:(A;;0x3;;;WD)"),
            // The log itself says where it is kept and who may read it.
            machine(
                r"SYSTEM\CurrentControlSet\Services\EventLog\Application",
                Some("File"),
                r"C:\Evil.evtx",
            ),
            machine(
                r"SYSTEM\CurrentControlSet\Services\EventLog\Security\Acme",
                Some("EventMessageFile"),
                SERVER,
            ),
            machine(
                &format!(r"{source}\Deeper"),
                Some("EventMessageFile"),
                SERVER,
            ),
        ] {
            assert!(refusal(&refused).is_some(), "{refused:?}");
        }
        assert!(
            unit_refusal(&[machine(source, Some("TypesSupported"), "7")]).is_some(),
            "a source with no message file of its own is not one"
        );
        assert_eq!(
            unit_refusal(&[
                machine(source, Some("TypesSupported"), "7"),
                machine(source, Some("EventMessageFile"), SERVER),
            ]),
            None
        );
    }

    #[test]
    fn a_protocol_runs_a_file_of_the_package_and_a_file_type_at_least_names_one() {
        let refusal = |name: &str, tail: &str, data: &str, is_protocol: bool| {
            let entry = machine(&format!(r"Software\Classes\{name}\{tail}"), None, data);
            let mark = machine(
                &format!(r"Software\Classes\{name}"),
                Some("URL Protocol"),
                "",
            );
            let extra = if is_protocol { vec![mark] } else { Vec::new() };
            registered_value_refusal(&entry, &app_map(), &known_of(&extra), &no_expanding)
        };
        const COMMAND: &str = r"shell\open\command";
        let run_it = r#""C:\Program Files\Acme\acme.exe" "%1""#;
        let through_windows =
            r#"C:\Windows\System32\wscript.exe "C:\Program Files\Acme\My Tools\run.cmd" "%1""#;
        let through_cmd =
            r#"C:\Windows\System32\cmd.exe /c anything "C:\Program Files\Acme\acme.exe""#;
        let from_a_share = r#"\\evil\share\x.exe "C:\Program Files\Acme\acme.exe""#;
        assert_eq!(refusal("acme", COMMAND, run_it, true), None);
        assert_eq!(refusal("http", COMMAND, run_it, false), None);
        assert!(refusal("acme", COMMAND, r"C:\Windows\System32\cmd.exe /c %1", true).is_some());
        assert!(
            refusal("acme", COMMAND, through_windows, true).is_some(),
            "what a page on the web can ask for runs the app itself or nothing"
        );
        assert!(refusal("http", COMMAND, r"C:\Windows\System32\cmd.exe /c %1", false).is_some());
        assert!(refusal("mailto", COMMAND, "cmd.exe /c %1", false).is_some());
        let mark = machine(r"Software\Classes\acme", Some("URL Protocol"), "");
        assert!(
            value_refusal(&mark).is_some(),
            "it says it is a protocol, with nothing of the package to open it"
        );
        let opens = machine(r"Software\Classes\acme\shell\open\command", None, run_it);
        assert_eq!(
            registered_value_refusal(
                &mark,
                &app_map(),
                &known_of(std::slice::from_ref(&opens)),
                &no_expanding
            ),
            None
        );
        assert_eq!(refusal("Acme.File", COMMAND, run_it, false), None);
        assert_eq!(
            refusal("Acme.File", COMMAND, through_windows, false),
            None,
            "a type of its own can run through Windows, given a file of its own"
        );
        assert!(
            refusal(
                "Acme.File",
                COMMAND,
                r"C:\Windows\System32\cmd.exe /c del %1",
                false
            )
            .is_some()
        );
        assert!(
            refusal("Acme.File", COMMAND, through_cmd, false).is_some(),
            "the file only comes along, and it runs whatever it is told"
        );
        assert!(refusal("Acme.File", COMMAND, from_a_share, false).is_some());
        assert!(
            refusal(
                "Acme.File",
                COMMAND,
                r#"wscript.exe "C:\Program Files\Acme\My Tools\run.cmd""#,
                false
            )
            .is_some(),
            "a name by itself can be found beside the file being opened"
        );
        assert!(
            refusal(
                "Acme.File",
                COMMAND,
                r#"C:\Windows\System32\ssh.exe "C:\Program Files\Acme\acme.exe""#,
                false
            )
            .is_some(),
            "it runs what it is told as well as what it is given"
        );
        assert!(
            refusal("Acme.File", "CLSID", OTHER_CLASS, false).is_some(),
            "it would create what this package does not install"
        );
        assert_eq!(refusal("Acme.File", "CLSID", CLASS, false), None);
    }

    #[test]
    fn a_shell_extension_points_at_a_file_of_the_package() {
        let handlers = r"Software\Classes\*\shellex\ContextMenuHandlers";
        for fine in [
            machine(&format!(r"{handlers}\Acme"), None, CLASS),
            machine(&format!(r"{handlers}\{CLASS}"), None, ""),
            machine(
                r"Software\Classes\.acme\shellex\{e357fccd-a995-4576-b01f-234630154e96}",
                None,
                CLASS,
            ),
            machine(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\ Acme",
                None,
                CLASS,
            ),
            machine(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved",
                Some(CLASS),
                "Acme menu",
            ),
            machine(
                &format!(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{CLASS}"
                ),
                None,
                "Acme",
            ),
        ] {
            assert_eq!(value_refusal(&fine), None, "{fine:?}");
        }
        for refused in [
            machine(&format!(r"{handlers}\Evil"), None, OTHER_CLASS),
            machine(&format!(r"{handlers}\{OTHER_CLASS}"), None, ""),
            machine(&format!(r"{handlers}\Evil"), None, "not a class"),
            machine(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\ Evil",
                None,
                OTHER_CLASS,
            ),
            machine(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved",
                Some(OTHER_CLASS),
                "Evil",
            ),
            machine(
                &format!(
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{OTHER_CLASS}"
                ),
                None,
                "Evil",
            ),
        ] {
            assert!(value_refusal(&refused).is_some(), "{refused:?}");
        }
        for blocked in [
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Cached",
        ] {
            let entry = machine(blocked, Some(OTHER_CLASS), "x");
            assert!(
                registry_refusal(&entry, &app_map(), &HashSet::new(), &no_expanding).is_some(),
                "{blocked}"
            );
        }
    }

    #[test]
    fn a_font_is_registered_by_a_path_the_package_installs() {
        const FONTS: &str = r"Software\Microsoft\Windows NT\CurrentVersion\Fonts";
        assert_eq!(
            value_refusal(&machine(
                FONTS,
                Some("Acme (TrueType)"),
                r"C:\Program Files\Acme\acme.exe"
            )),
            None
        );
        for refused in [
            // Relative to the Windows folder, which this never writes to.
            "arial.ttf",
            r"C:\Windows\Fonts\arial.ttf",
            r"\\server\share\evil.ttf",
            r"C:\Elsewhere\evil.ttf",
        ] {
            assert!(
                value_refusal(&machine(FONTS, Some("Arial (TrueType)"), refused)).is_some(),
                "{refused}"
            );
        }
    }

    #[test]
    fn a_compatibility_setting_is_for_a_program_of_the_package_under_the_name_it_has_there() {
        const LAYERS: &str = r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers";
        let setting = |program: &str| machine(LAYERS, Some(program), "~ RUNASADMIN");
        assert_eq!(
            value_refusal(&setting(r"C:\Program Files\Acme\acme.exe")),
            None
        );
        assert!(value_refusal(&setting(r"C:\Windows\System32\cmd.exe")).is_some());
        assert_eq!(
            restored_value_name(&setting(r"C:\Program Files\Acme\acme.exe"), &app_map()),
            r"D:\Apps\Acme\acme.exe"
        );
        assert_eq!(
            restored_value_name(
                &machine(
                    r"Software\Acme",
                    Some(r"C:\Program Files\Acme\acme.exe"),
                    "x"
                ),
                &app_map()
            ),
            r"C:\Program Files\Acme\acme.exe",
            "any other value's name is just a name"
        );
    }

    #[test]
    fn being_listed_in_default_apps_takes_what_it_says_being_in_the_package() {
        let listed = machine(
            r"Software\RegisteredApplications",
            Some("Acme"),
            r"Software\Acme\Capabilities",
        );
        let capability = machine(
            r"Software\Acme\Capabilities",
            Some("ApplicationName"),
            "Acme",
        );
        let known = known_of(std::slice::from_ref(&capability));
        assert_eq!(
            registered_value_refusal(&listed, &app_map(), &known, &no_expanding),
            None
        );
        assert!(value_refusal(&listed).is_some());
        assert!(
            value_refusal(&machine(
                r"Software\Clients\StartMenuInternet",
                None,
                "Evil"
            ))
            .is_some(),
            "that says which browser is the default"
        );
    }

    #[test]
    fn an_odbc_driver_loads_what_the_package_installs_and_a_source_uses_it_or_windows() {
        let driver_key = r"Software\ODBC\ODBCINST.INI\Acme Driver";
        assert_eq!(
            unit_refusal(&[
                machine(driver_key, Some("Driver"), SERVER),
                machine(driver_key, Some("Setup"), SERVER),
                machine(driver_key, Some("APILevel"), "2"),
                machine(
                    r"Software\ODBC\ODBCINST.INI\ODBC Drivers",
                    Some("Acme Driver"),
                    "Installed"
                ),
            ]),
            None
        );
        assert!(
            unit_refusal(&[machine(
                driver_key,
                Some("Driver"),
                r"C:\Elsewhere\evil.dll"
            )])
            .is_some()
        );
        assert!(
            unit_refusal(&[
                machine(driver_key, Some("Driver"), SERVER),
                machine(driver_key, Some("Setup"), r"C:\Elsewhere\evil.dll"),
            ])
            .is_some()
        );
        assert!(
            unit_refusal(&[machine(
                r"Software\ODBC\ODBCINST.INI\ODBC Drivers",
                Some("Other Driver"),
                "Installed"
            )])
            .is_some(),
            "it lists one it does not have"
        );
        let source = r"Software\ODBC\ODBC.INI\Acme DB";
        for fine in [SERVER, r"C:\Windows\System32\sqlsrv32.dll"] {
            assert_eq!(unit_refusal(&[machine(source, Some("Driver"), fine)]), None);
        }
        assert!(
            unit_refusal(&[machine(source, Some("Driver"), r"C:\Elsewhere\evil.dll")]).is_some()
        );
    }

    fn firewall_rule(program: &str) -> FirewallRuleEntry {
        FirewallRuleEntry {
            name: "Acme Web".to_owned(),
            description: None,
            direction: "In".to_owned(),
            action: "Allow".to_owned(),
            enabled: true,
            program: PathBuf::from(program),
            service: None,
            protocol: Some("6".to_owned()),
            local_ports: vec!["80".to_owned(), "443".to_owned()],
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: vec!["LocalSubnet".to_owned()],
            profiles: vec!["Private".to_owned()],
            not_carried: Vec::new(),
        }
    }

    #[test]
    fn what_is_odbcs_own_is_not_a_driver_or_a_data_source() {
        let tracing = [
            machine(r"Software\ODBC\ODBC.INI\ODBC", Some("Trace"), "1"),
            machine(
                r"Software\ODBC\ODBC.INI\ODBC",
                Some("TraceDll"),
                r"\\evil\share\evil.dll",
            ),
        ];
        assert!(unit_refusal(&tracing).is_some());
        let translator = machine(
            r"Software\ODBC\ODBC.INI\Acme DB",
            Some("TranslationDLL"),
            r"C:\Elsewhere\evil.dll",
        );
        assert!(unit_refusal(std::slice::from_ref(&translator)).is_some());
        assert!(
            unit_refusal(&[machine(
                r"Software\ODBC\ODBC.INI\Acme DB",
                Some("Driver"),
                r"C:\Windows\..\Elsewhere\evil.dll"
            )])
            .is_some()
        );
    }

    #[test]
    fn a_firewall_rule_is_for_one_program_of_the_package_and_touches_nothing_else() {
        const PROGRAM: &str = r"C:\Program Files\Acme\acme.exe";
        let services = HashSet::new();
        let refusal = |rule: &FirewallRuleEntry| firewall_refusal(rule, &app_map(), &services);
        assert_eq!(refusal(&firewall_rule(PROGRAM)), None);
        let block = FirewallRuleEntry {
            action: "Block".to_owned(),
            ..firewall_rule(PROGRAM)
        };
        assert_eq!(refusal(&block), None, "it is only about its own program");

        for (why, rule) in [
            ("every program", firewall_rule("")),
            (
                "not installed",
                firewall_rule(r"C:\Windows\System32\svchost.exe"),
            ),
            (
                "overrides other rules",
                FirewallRuleEntry {
                    action: "AllowBypass".to_owned(),
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "through NAT",
                FirewallRuleEntry {
                    not_carried: vec!["Edge".to_owned()],
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "a name that means every rule",
                FirewallRuleEntry {
                    name: "ALL".to_owned(),
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "a name that is another option",
                FirewallRuleEntry {
                    name: "x\" dir=in".to_owned(),
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "a port that is another option",
                FirewallRuleEntry {
                    local_ports: vec!["80 program=any".to_owned()],
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "a service it does not restore",
                FirewallRuleEntry {
                    service: Some("Spooler".to_owned()),
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "no direction",
                FirewallRuleEntry {
                    direction: "both".to_owned(),
                    ..firewall_rule(PROGRAM)
                },
            ),
            (
                "not a profile",
                FirewallRuleEntry {
                    profiles: vec!["everywhere".to_owned()],
                    ..firewall_rule(PROGRAM)
                },
            ),
        ] {
            assert!(refusal(&rule).is_some(), "{why}");
        }
    }

    #[test]
    fn a_rule_reaches_netsh_as_it_was_and_comes_back_the_same() {
        let rule = firewall_rule(r"C:\Program Files\Acme\acme.exe");
        let program = Path::new(r"D:\My Apps\acme.exe");
        let arguments = netsh_add_rule_arguments(&rule, program);
        for wanted in [
            "name=Acme Web",
            "dir=in",
            "action=allow",
            r"program=D:\My Apps\acme.exe",
            "enable=yes",
            "profile=private",
            "protocol=tcp",
            "localport=80,443",
            "remoteip=localsubnet",
        ] {
            assert!(
                arguments.iter().any(|argument| argument == wanted),
                "{arguments:?}"
            );
        }
        assert_eq!(
            netsh_delete_rule_arguments(
                "Acme Web",
                "In",
                program,
                Some("6"),
                &["80".to_owned(), "443".to_owned()],
                &[]
            )[4..],
            [
                "name=Acme Web".to_owned(),
                "dir=in".to_owned(),
                r"program=D:\My Apps\acme.exe".to_owned(),
                "protocol=tcp".to_owned(),
                "localport=80,443".to_owned(),
            ]
        );
        let stored = r"v2.33|Action=Allow|Active=TRUE|Dir=In|Protocol=6|Profile=Private|LPort=443|LPort=80|RA4=LocalSubnet|App=D:\My Apps\acme.exe|Name=Acme Web|";
        assert!(stored_firewall_rule_matches(
            stored, "acme web", "in", program
        ));
        assert!(!stored_firewall_rule_matches(
            stored, "Acme Web", "out", program
        ));
        assert!(!stored_firewall_rule_matches(
            stored, "Other", "in", program
        ));
        assert!(
            stored_firewall_rule_matches(stored, " Acme Web ", "in", program),
            "it goes by the name that was set, which has nothing around it"
        );
        assert_ne!(
            firewall_identity(Some("6"), &["80".to_owned()], &[]),
            firewall_identity(Some("17"), &["80".to_owned()], &[]),
            "one for TCP and one for UDP are two rules"
        );
        assert_eq!(
            firewall_identity(Some("6"), &["80".to_owned(), "443".to_owned()], &[]),
            firewall_identity(Some("TCP"), &["443".to_owned(), "080".to_owned()], &[]),
        );
        assert_eq!(
            firewall_identity(Some("any"), &[], &[]),
            firewall_identity(None, &[], &[]),
            "a rule for any says nothing about it when it is read back"
        );
        assert_eq!(
            firewall_identity(Some("icmpv4"), &[], &[]),
            firewall_identity(Some("1"), &[], &[]),
        );
        assert!(!stored_firewall_rule_matches(
            stored,
            "Acme Web",
            "in",
            Path::new(r"C:\Other\acme.exe")
        ));
        let read_back = parse_firewall_rule(stored);
        assert_eq!(
            read_back.as_ref().map(firewall_fingerprint),
            Some(firewall_fingerprint(&rule)),
            "the order Windows lists its ports in says nothing"
        );
    }

    #[test]
    fn what_a_value_registers_is_told_by_where_it_is() {
        let of =
            |key: &str, name: Option<&str>, data: &str| registration_of(&machine(key, name, data));
        assert_eq!(
            of(r"Software\Classes\vscode", Some("URL Protocol"), ""),
            Some(Registration::UrlProtocol {
                name: "vscode".to_owned()
            })
        );
        assert_eq!(
            of(r"Software\Classes\.acme", None, "Acme.File"),
            Some(Registration::FileType {
                extension: ".acme".to_owned()
            })
        );
        assert_eq!(
            of(
                r"Software\Classes\Directory\shell\git_shell\command",
                None,
                "x"
            ),
            Some(Registration::ShellCommand {
                on: "Directory".to_owned(),
                verb: "git_shell".to_owned()
            })
        );
        assert_eq!(
            of(
                r"SYSTEM\CurrentControlSet\Services\EventLog\Application\Acme",
                Some("EventMessageFile"),
                "x"
            ),
            Some(Registration::EventLogSource {
                log: "Application".to_owned(),
                source: "Acme".to_owned()
            })
        );
        assert_eq!(
            of(r"Software\Classes\Word.Document\shell", None, "acme"),
            Some(Registration::DefaultVerb {
                of: "Word.Document".to_owned()
            })
        );
        assert_eq!(
            of(r"Software\Classes\AppID\{a}", Some("LaunchPermission"), "x"),
            Some(Registration::AppIdAccess)
        );
        assert_eq!(
            of(r"Software\Clients\Media\Acme\shell\open\command", None, "x"),
            Some(Registration::ClientEntry {
                name: "Media: Acme".to_owned()
            })
        );
        assert_eq!(
            of(
                r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths\acme.exe",
                None,
                "x"
            ),
            Some(Registration::ProgramName {
                name: "acme.exe".to_owned()
            })
        );
        assert_eq!(of(r"Software\Acme", Some("Theme"), "dark"), None);
    }
}
