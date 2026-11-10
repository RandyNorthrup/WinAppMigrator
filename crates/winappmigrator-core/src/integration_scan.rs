//! How an app is tied into Windows, beyond its own keys: COM, protocols and file types, shell
//! extensions, firewall rules, fonts, ODBC, event log sources, and compatibility settings.
//!
//! **An entry is the app's if what it points at is.** Its name says nothing, since these all sit
//! side by side with everyone else's, and another app's class or file type must never come along.
//! So each is read for the file it names, which is compared with the files the app has, as its own
//! PC calls them. On a connected drive that is only ever text, and nothing here opens what a value
//! names. What is linked follows from there: a class leads to its ProgID, AppID, and type library,
//! and an interface to its proxy stub and type library.
//!
//! Every key under `Classes` is looked at once per scan, by name, and only the few subkeys that say
//! what runs are opened. That is one read for each thing on the PC, and not a walk of each of their
//! subtrees, which is what made this slow when it was done for every class by recursion.
//!
//! Certificates are only ever reported. A package must never add to a trust store.

use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::KEY_READ;

use crate::models::{
    CertificateEntry, FileAssociationEntry, FileClassification, FileEntry, FirewallRuleEntry,
    MatchConfidence, RegistryEntry, RegistryHive, RegistryValueKind, Warning,
};
use crate::path_utils::{
    comparable_path, normalize_name_for_match, path_is_strictly_inside, path_is_within,
    text_mentions_path, trim_icon_suffix,
};
use crate::product_match::{MatchScope, ProductNames};
use crate::registry_scan::{
    RegRoot, capture_registry_subtree, entry_from_raw_value, file_associations_from_prog_id,
    raw_value_to_display,
};
use crate::system_context::SystemContext;

const CLASSES: &str = r"Software\Classes";
/// Where a 32-bit program's COM registrations are in both hives. `Software\WOW6432Node\Classes` is
/// a registry link to this in a real hive, and on a connected drive a link is not to be gone through.
const CLASSES_32_BIT: &str = r"Software\Classes\WOW6432Node";
const CLSID: &str = "CLSID";
const INTERFACE: &str = "Interface";
const TYPE_LIB: &str = "TypeLib";
const APP_ID: &str = "AppID";
const DLL_SURROGATE_VALUE: &str = "DllSurrogate";
const INPROC_SERVER: &str = "InprocServer32";
const COM_SERVERS: &[&str] = &[INPROC_SERVER, "LocalServer32", "InprocHandler32"];
/// Where `LocalServer32` is written with a Windows Installer descriptor, the path is this value.
const SERVER_EXECUTABLE_VALUE: &str = "ServerExecutable";
const CODE_BASE_VALUE: &str = "CodeBase";
const FILE_URL_PREFIX: &str = "file:///";
const DEFAULT_ICON: &str = "DefaultIcon";
const INSTANCE_PROPERTY_BAG: &str = r"Instance\InitPropertyBag";
/// The keys under `Classes` that are not file types or protocols.
const NOT_CLASSES: &[&str] = &[
    "appid",
    "applications",
    "clsid",
    "cid",
    "directshow",
    "filter",
    "installer",
    "interface",
    "licenses",
    "media type",
    "mime",
    "record",
    "systemfileassociations",
    "typelib",
    "unknown",
    "wow6432node",
];
/// File types every program shares, which an app can only add to.
const SHARED_CLASSES: &[&str] = &[
    "*",
    "allfilesystemobjects",
    "desktopbackground",
    "directory",
    "drive",
    "folder",
    "libraryfolder",
];
const NESTED_SHARED_CLASSES: &[&str] = &[r"Directory\Background", r"LibraryFolder\Background"];
/// What decides how programs start, and is never an app's. See `restore_policy::REFUSED_CLASSES`.
const NEVER_AN_APPS: &[&str] = &[
    "batfile", "cmdfile", "comfile", "exefile", "htafile", "lnkfile", "mscfile", "piffile",
    "regfile", "scrfile",
];
const SHELL: &str = "shell";
const SHELLEX: &str = "shellex";
const COMMAND: &str = "command";
const OPEN_VERB: &str = "open";
const CLASS_REFERENCE: &str = "CLSID";
const OPEN_WITH_PROGIDS: &str = "OpenWithProgids";
const OPEN_WITH_LIST: &str = "OpenWithList";
const SHELL_NEW: &str = "ShellNew";
const SHELL_NEW_COMMAND_VALUE: &str = "Command";
const VERB_HANDLER_VALUES: &[&str] = &["ExplorerCommandHandler", "DelegateExecute"];
const EXPLORER: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer";
const ICON_OVERLAYS: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers";
const APPROVED: &str = r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved";
const NAMESPACES: &[&str] = &[r"Desktop\NameSpace", r"MyComputer\NameSpace"];
const REGISTERED_APPLICATIONS: &str = r"Software\RegisteredApplications";
const CLIENTS: &str = r"Software\Clients";
const SOFTWARE_PREFIX: &str = r"software\";
const CAPABILITY_FILE_ASSOCIATIONS: &[&str] = &["FileAssociations", "URLAssociations"];
const APPLICATION_ICON_VALUE: &str = "ApplicationIcon";
const EVENT_LOG: &str = r"SYSTEM\CurrentControlSet\Services\EventLog";
pub(crate) const EVENT_LOG_FILE_VALUES: &[&str] = &[
    "EventMessageFile",
    "CategoryMessageFile",
    "ParameterMessageFile",
];
/// What else a source has that means anything. A source can hold nothing else that is restored.
pub(crate) const EVENT_LOG_OTHER_VALUES: &[&str] = &["TypesSupported", "CategoryCount"];
const ODBC_DRIVERS: &[&str] = &[
    r"Software\ODBC\ODBCINST.INI",
    r"Software\WOW6432Node\ODBC\ODBCINST.INI",
];
const ODBC_SOURCES: &[&str] = &[
    r"Software\ODBC\ODBC.INI",
    r"Software\WOW6432Node\ODBC\ODBC.INI",
];
pub(crate) const ODBC_DRIVER_LIST: &str = "ODBC Drivers";
pub(crate) const ODBC_SOURCE_LIST: &str = "ODBC Data Sources";
const ODBC_NOT_DRIVERS: &[&str] = &["odbc core", "odbc drivers", "odbc translators"];
const PROXY_STUB: &str = "ProxyStubClsid32";
/// More than this does not go any faster, since they all read one registry.
const MOST_WALKERS: usize = 8;
const ODBC_NOT_SOURCES: &[&str] = &["odbc", "odbc data sources", "odbc file dsn"];
const DRIVER_VALUE: &str = "Driver";
const SETUP_VALUE: &str = "Setup";
const FONTS: &str = r"Software\Microsoft\Windows NT\CurrentVersion\Fonts";
const WINDOWS_FONTS_FOLDER: &str = "Fonts";
/// Shorter than this, a product's name in a font's name means nothing.
const SHORTEST_NAME_TO_REPORT_BY: usize = 4;
const APP_COMPAT_LAYERS: &str =
    r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers";
const FIREWALL_RULES: &str =
    r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\FirewallRules";
const CERTIFICATE_STORES: &[&str] = &["ROOT", "CA", "TrustedPublisher", "TrustedPeople"];
const SYSTEM_CERTIFICATES: &str = r"Software\Microsoft\SystemCertificates";
const CERTIFICATE_BLOB_VALUE: &str = "Blob";
/// The property in a stored certificate's blob that holds the certificate.
const CERT_PROPERTY_ENCODED: u32 = 32;
const CERT_PROPERTY_HEADER_BYTES: usize = 12;
const OID_COMMON_NAME: &[u8] = &[0x55, 0x04, 0x03];
const OID_ORGANIZATION: &[u8] = &[0x55, 0x04, 0x0a];
/// Windows has these in a store for everyone, so a publisher of this name says nothing about one.
const PUBLISHERS_OF_WINDOWS: &[&str] = &["microsoft"];
const WHY_NAMED_FOR_PUBLISHER: &str = "it is named for the app or who published it";
const WHY_RECORDED: &str = "the app's install record names it";

/// What of a command or a path a value holds is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PointsAt {
    /// A file the app has.
    TheApp,
    /// Something in the Windows folder, or that goes by a bare name, which Windows would find there.
    Windows,
    /// Anything else, which is someone else's.
    Elsewhere,
}

/// The files an app has, and where it is, as its own PC calls them. Everything here is compared
/// as text, so a value can name what it likes and nothing is opened to find out.
#[derive(Debug, Clone)]
pub(crate) struct OwnedFiles {
    executable: PathBuf,
    install_location: Option<PathBuf>,
    windows_dir: PathBuf,
    files: HashSet<String>,
}

impl OwnedFiles {
    pub(crate) fn new(
        executable: &Path,
        install_location: Option<&Path>,
        windows_dir: &Path,
        files: impl IntoIterator<Item = PathBuf>,
    ) -> Self {
        let mut known: HashSet<String> = files
            .into_iter()
            .map(|file| comparable_path(&file))
            .collect();
        known.insert(comparable_path(executable));
        Self {
            executable: executable.to_path_buf(),
            install_location: install_location.map(Path::to_path_buf),
            windows_dir: windows_dir.to_path_buf(),
            files: known,
        }
    }

    pub(crate) fn owns(&self, path: &Path) -> bool {
        self.files.contains(&comparable_path(path))
            || self
                .install_location
                .as_deref()
                .is_some_and(|location| path_is_strictly_inside(path, location))
    }

    /// Whether text names the app, as a whole path. Only for what is run through something else.
    fn mentions(&self, text: &str) -> bool {
        text_mentions_path(text, &self.executable, true)
            || self
                .install_location
                .as_deref()
                .is_some_and(|location| text_mentions_path(text, location, false))
    }

    /// What a command or path a value holds is about. What runs is what counts: another app's
    /// program that is given a file of this app's is that other app's. Only something of Windows'
    /// own, which runs whatever it is told, goes by what it is given.
    pub(crate) fn points_at(&self, text: &str) -> PointsAt {
        let Some(first) = first_path_in(text) else {
            return PointsAt::Elsewhere;
        };
        if first.is_absolute() {
            if self.owns(&first) {
                return PointsAt::TheApp;
            }
            if !path_is_strictly_inside(&first, &self.windows_dir) {
                return PointsAt::Elsewhere;
            }
        }
        if self.mentions(text) {
            PointsAt::TheApp
        } else {
            PointsAt::Windows
        }
    }
}

/// Extensions a command's program or a server can have, for reading a path nothing has quoted.
const PROGRAM_EXTENSIONS: &[&str] = &[
    ".exe", ".dll", ".ocx", ".cpl", ".com", ".bat", ".cmd", ".scr", ".msi", ".ps1", ".py", ".vbs",
    ".js", ".tlb", ".olb", ".ttf", ".otf",
];

/// The first path in a command, however it is written. `C:\Program Files\App\a.dll` has a space and
/// no quotes, as a COM server's path may. A value can say anything, and this only reads it.
pub(crate) fn first_path_in(text: &str) -> Option<PathBuf> {
    let trimmed = trim_icon_suffix(text.trim());
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix('"') {
        return rest.split_once('"').map(|(path, _)| PathBuf::from(path));
    }
    let lower = trimmed.to_ascii_lowercase();
    let ends_at = PROGRAM_EXTENSIONS
        .iter()
        .filter_map(|extension| {
            let mut from = 0;
            while let Some(found) = lower[from..].find(extension) {
                let end = from + found + extension.len();
                match lower[end..].chars().next() {
                    None | Some(' ' | '\t' | ',' | '"') => return Some(end),
                    _ => from = end,
                }
            }
            None
        })
        .min();
    if let Some(end) = ends_at {
        return Some(PathBuf::from(&trimmed[..end]));
    }
    // No extension to go by: a folder, or a file with no extension. Past a drive and with spaces in
    // it, all of it is a better reading than its first word.
    let bytes = lower.as_bytes();
    if bytes.len() > 2 && bytes[1] == b':' {
        return Some(PathBuf::from(trimmed));
    }
    trimmed.split_whitespace().next().map(PathBuf::from)
}

/// What a scan is given to go by, all as the source PC calls things.
pub(crate) struct IntegrationInputs<'a> {
    pub owned: &'a OwnedFiles,
    pub names: &'a ProductNames,
    pub display_name: Option<&'a str>,
    pub publisher: Option<&'a str>,
    /// Every file the app's install record names, for what is not under its own folder.
    pub recorded_files: &'a [PathBuf],
}

/// What was found.
#[derive(Debug, Clone, Default)]
pub(crate) struct Integrations {
    pub entries: Vec<RegistryEntry>,
    pub file_associations: Vec<FileAssociationEntry>,
    pub firewall_rules: Vec<FirewallRuleEntry>,
    pub certificates: Vec<CertificateEntry>,
    pub fonts_reported: Vec<String>,
    /// Fonts in the Windows folder that an install record says are the app's. They go under the
    /// install location, and are registered by path from there.
    pub font_files: Vec<FileEntry>,
    pub warnings: Vec<Warning>,
}

/// What is known to be the app's, by lowercase name, for what links to it.
#[derive(Debug, Clone, Default)]
struct OwnedClasses {
    clsids: HashSet<String>,
    type_libraries: HashSet<String>,
    app_ids: HashSet<String>,
    prog_ids: HashSet<String>,
    /// What classes say they run from, by file name, which is what `AppID\<name>` goes by.
    server_files: BTreeSet<String>,
    odbc_drivers: HashSet<String>,
}

struct Scan<'a> {
    inputs: &'a IntegrationInputs<'a>,
    system: &'a SystemContext,
    owned: OwnedClasses,
    found: Integrations,
}

pub(crate) fn scan_integrations(
    inputs: &IntegrationInputs<'_>,
    system: &SystemContext,
) -> Integrations {
    let mut scan = Scan {
        inputs,
        system,
        owned: OwnedClasses::default(),
        found: Integrations::default(),
    };
    let machine = RegRoot::Hklm(system);
    scan.classes_of(&machine);
    scan.everything_else_of(&machine);
    scan.event_log_sources(&machine);
    scan.firewall_rules(&machine);
    for user in system.user_hives() {
        let hive = RegRoot::User(&user);
        scan.classes_of(&hive);
        scan.everything_else_of(&hive);
    }
    scan.found
}

/// The subkeys of a big key that `is_it` says so of, in the order the key has them. A PC has tens
/// of thousands of these, and an app a few, so the going through is done on a few threads, each with
/// a handle of its own, and only reads. What is then done with those few is done one at a time.
fn subkeys_where<F>(root: &RegRoot<'_>, path: &str, is_it: F) -> Vec<String>
where
    F: Fn(&RegKey, &str) -> bool + Sync,
{
    let Ok(parent) = root.open(path, KEY_READ) else {
        return Vec::new();
    };
    let names: Vec<String> = parent.enum_keys().flatten().collect();
    let walkers = std::thread::available_parallelism()
        .map_or(1, std::num::NonZero::get)
        .min(MOST_WALKERS);
    let each = names.len().div_ceil(walkers).max(1);
    std::thread::scope(|scope| {
        let running: Vec<_> = names
            .chunks(each)
            .map(|some| {
                let is_it = &is_it;
                scope.spawn(move || {
                    let Ok(parent) = root.open(path, KEY_READ) else {
                        return Vec::new();
                    };
                    some.iter()
                        .filter(|name| is_it(&parent, name))
                        .cloned()
                        .collect::<Vec<String>>()
                })
            })
            .collect();
        running
            .into_iter()
            .flat_map(|walker| walker.join().unwrap_or_default())
            .collect()
    })
}

fn default_value(key: &RegKey) -> String {
    key.get_raw_value("")
        .map(|value| raw_value_to_display(&value))
        .unwrap_or_default()
}

fn default_of(parent: &RegKey, path: &str) -> String {
    parent
        .open_subkey_with_flags(path, KEY_READ)
        .map(|key| default_value(&key))
        .unwrap_or_default()
}

/// What `Word.Application` calls a CLSID or AppID, without its braces being anything but its name.
fn lower_trimmed(text: &str) -> String {
    text.trim().to_ascii_lowercase()
}

/// A `file:///C:/x/y.dll` as a path.
fn path_of_file_url(url: &str) -> Option<String> {
    let start = url.get(..FILE_URL_PREFIX.len())?;
    start
        .eq_ignore_ascii_case(FILE_URL_PREFIX)
        .then(|| url[FILE_URL_PREFIX.len()..].replace('/', "\\"))
}

impl Scan<'_> {
    fn expanded(&self, text: &str) -> String {
        if text.contains('%') {
            self.system.expand_source_variables(text)
        } else {
            text.to_owned()
        }
    }

    fn points_at(&self, text: &str) -> PointsAt {
        self.inputs.owned.points_at(&self.expanded(text))
    }

    fn runs_the_app(&self, text: &str) -> bool {
        self.points_at(text) == PointsAt::TheApp
    }

    fn capture(&mut self, root: &RegRoot, key_path: &str) {
        capture_registry_subtree(
            root,
            key_path,
            MatchConfidence::High,
            &mut self.found.entries,
            &mut self.found.warnings,
        );
    }

    fn capture_value(&mut self, root: &RegRoot, key: &RegKey, key_path: &str, name: &str) {
        if let Ok(value) = key.get_raw_value(name) {
            let hive = root.hive();
            let requires_admin = hive == RegistryHive::LocalMachine;
            self.found.entries.push(entry_from_raw_value(
                hive,
                key_path.to_owned(),
                name.to_owned(),
                value,
                MatchConfidence::High,
                requires_admin,
            ));
        }
    }

    // ---- COM, protocols, and file types --------------------------------------------------

    fn classes_of(&mut self, root: &RegRoot) {
        let plain = root.open(CLASSES, KEY_READ);
        let of_32_bit = root.open(CLASSES_32_BIT, KEY_READ);
        // A class and an interface are their own for each view. A type library and an AppID are
        // one thing for both, which Windows shows in each, so only the one is read.
        let mut plain_has_some = false;
        let mut other_has_some = false;
        if let Ok(classes) = plain.as_ref() {
            plain_has_some = self.com_classes(root, CLASSES, classes);
        }
        if let Ok(classes) = of_32_bit.as_ref() {
            other_has_some = self.com_classes(root, CLASSES_32_BIT, classes);
        }
        if let Ok(classes) = plain.as_ref() {
            self.type_libraries(root, CLASSES, classes);
        }
        // This is the biggest key there is, and with nothing of the app's for one to lead to it says
        // nothing about the app. A proxy is in the view its class is in. A type library is for both,
        // and a person's own key has little in it.
        let any_view = !self.owned.type_libraries.is_empty()
            || (matches!(root, RegRoot::User(_)) && !self.owned.clsids.is_empty());
        if any_view || plain_has_some {
            self.interfaces(root, CLASSES);
        }
        if any_view || other_has_some {
            self.interfaces(root, CLASSES_32_BIT);
        }
        if let Ok(classes) = plain.as_ref() {
            self.app_ids(root, CLASSES, classes);
            self.file_types_and_protocols(root, classes);
        }
    }

    /// Each class whose server is the app's, with what it links to. One with a server that is
    /// someone else's is theirs, whatever else in it mentions the app. One with a server of
    /// Windows', such as `mscoree.dll` for a .NET class or `shell32.dll` for a folder in Explorer,
    /// says whose it is by what it gives that.
    fn com_classes(&mut self, root: &RegRoot, view: &str, classes: &RegKey) -> bool {
        let Ok(clsids) = classes.open_subkey_with_flags(CLSID, KEY_READ) else {
            return false;
        };
        let theirs = subkeys_where(root, &format!(r"{view}\{CLSID}"), |clsids, name| {
            clsids
                .open_subkey_with_flags(name, KEY_READ)
                .is_ok_and(|class| self.class_is_the_apps(&class))
        });
        for name in &theirs {
            let Ok(class) = clsids.open_subkey_with_flags(name, KEY_READ) else {
                continue;
            };
            self.owned.clsids.insert(lower_trimmed(name));
            // What a class says it is called is only so if that says it is this class. A class
            // cannot make `Word.Document` the app's by naming it.
            for link in ["ProgID", "VersionIndependentProgID"] {
                let prog_id = default_of(&class, link);
                if !prog_id.trim().is_empty()
                    && lower_trimmed(&default_of(
                        classes,
                        &format!(r"{}\{CLASS_REFERENCE}", prog_id.trim()),
                    )) == lower_trimmed(name)
                {
                    self.owned.prog_ids.insert(lower_trimmed(&prog_id));
                }
            }
            if let Ok(value) = class.get_raw_value(APP_ID) {
                let app_id = lower_trimmed(&raw_value_to_display(&value));
                // An empty name would open the key of them all.
                if !app_id.is_empty() {
                    self.owned.app_ids.insert(app_id);
                }
            }
            for server in COM_SERVERS {
                let text = default_of(&class, server);
                if let Some(file) = first_path_in(&self.expanded(&text))
                    .as_deref()
                    .and_then(Path::file_name)
                {
                    self.owned
                        .server_files
                        .insert(file.to_string_lossy().to_ascii_lowercase());
                }
            }
            self.capture(root, &format!(r"{view}\{CLSID}\{name}"));
        }
        !theirs.is_empty()
    }

    fn class_is_the_apps(&self, class: &RegKey) -> bool {
        let mut someone_elses = false;
        let mut the_apps = false;
        for server in COM_SERVERS {
            let Ok(key) = class.open_subkey_with_flags(server, KEY_READ) else {
                continue;
            };
            let mut named = default_value(&key);
            if named.trim().is_empty()
                && let Ok(value) = key.get_raw_value(SERVER_EXECUTABLE_VALUE)
            {
                named = raw_value_to_display(&value);
            }
            match self.points_at(&named) {
                PointsAt::TheApp if !named.trim().is_empty() => the_apps = true,
                PointsAt::Elsewhere if !named.trim().is_empty() => someone_elses = true,
                _ => {}
            }
            if *server == INPROC_SERVER
                && let Ok(code_base) = key.get_raw_value(CODE_BASE_VALUE)
                && let Some(path) = path_of_file_url(&raw_value_to_display(&code_base))
                && self.runs_the_app(&format!("\"{path}\""))
            {
                the_apps = true;
            }
        }
        // One server of someone else's is what would be loaded just as well.
        if someone_elses || the_apps {
            return the_apps && !someone_elses;
        }
        if self.runs_the_app(&default_of(class, DEFAULT_ICON)) {
            return true;
        }
        class
            .open_subkey_with_flags(INSTANCE_PROPERTY_BAG, KEY_READ)
            .is_ok_and(|bag| {
                bag.enum_values()
                    .flatten()
                    .any(|(_, value)| self.runs_the_app(&raw_value_to_display(&value)))
            })
    }

    /// `TypeLib\{guid}\<version>\<language>\<win32 or win64>` names a file.
    fn type_libraries(&mut self, root: &RegRoot, view: &str, classes: &RegKey) {
        let Ok(libraries) = classes.open_subkey_with_flags(TYPE_LIB, KEY_READ) else {
            return;
        };
        for library in libraries.enum_keys().flatten() {
            let Ok(key) = libraries.open_subkey_with_flags(&library, KEY_READ) else {
                continue;
            };
            let is_the_apps = key
                .enum_keys()
                .flatten()
                .any(|version| self.type_library_version_is_the_apps(&key, &version));
            if is_the_apps {
                self.owned.type_libraries.insert(lower_trimmed(&library));
                self.capture(root, &format!(r"{view}\{TYPE_LIB}\{library}"));
            }
        }
    }

    fn type_library_version_is_the_apps(&self, library: &RegKey, version: &str) -> bool {
        let Ok(version) = library.open_subkey_with_flags(version, KEY_READ) else {
            return false;
        };
        version.enum_keys().flatten().any(|language| {
            // Beside these are `FLAGS` and `HELPDIR`, which say nothing.
            language
                .chars()
                .all(|character| character.is_ascii_hexdigit())
                && version
                    .open_subkey_with_flags(&language, KEY_READ)
                    .is_ok_and(|files| {
                        files.enum_keys().flatten().any(|build| {
                            self.points_at(&default_of(&files, &build)) == PointsAt::TheApp
                        })
                    })
        })
    }

    /// An interface says how calls to it cross processes: by a proxy stub class, or a type library.
    fn interfaces(&mut self, root: &RegRoot, view: &str) {
        let path = format!(r"{view}\{INTERFACE}");
        let theirs = subkeys_where(root, &path, |interfaces, name| {
            let named_by =
                |what: &str| lower_trimmed(&default_of(interfaces, &format!(r"{name}\{what}")));
            (!self.owned.clsids.is_empty() && self.owned.clsids.contains(&named_by(PROXY_STUB)))
                || (!self.owned.type_libraries.is_empty()
                    && self.owned.type_libraries.contains(&named_by(TYPE_LIB)))
        });
        for interface in theirs {
            self.capture(root, &format!(r"{path}\{interface}"));
        }
    }

    /// The AppIDs the app's classes name, and `AppID\<its program>`, which says the same of it.
    fn app_ids(&mut self, root: &RegRoot, view: &str, classes: &RegKey) {
        let Ok(app_ids) = classes.open_subkey_with_flags(APP_ID, KEY_READ) else {
            return;
        };
        let by_id: Vec<String> = self.owned.app_ids.iter().cloned().collect();
        for id in by_id {
            // Windows has AppIDs that any class can name, such as the one each preview handler
            // goes by. What hosts it says whose it is, and with none it is the class's own.
            let is_the_apps = app_ids
                .open_subkey_with_flags(&id, KEY_READ)
                .is_ok_and(|key| {
                    let host = key
                        .get_raw_value(DLL_SURROGATE_VALUE)
                        .map(|value| raw_value_to_display(&value))
                        .unwrap_or_default();
                    host.trim().is_empty() || self.runs_the_app(&host)
                });
            if is_the_apps {
                self.capture(root, &format!(r"{view}\{APP_ID}\{id}"));
            }
        }
        let programs: Vec<String> = self.owned.server_files.iter().cloned().collect();
        for program in programs {
            let Ok(key) = app_ids.open_subkey_with_flags(&program, KEY_READ) else {
                continue;
            };
            let id = key
                .get_raw_value(APP_ID)
                .map(|value| lower_trimmed(&raw_value_to_display(&value)))
                .unwrap_or_default();
            if self.owned.app_ids.contains(&id) {
                self.capture(root, &format!(r"{view}\{APP_ID}\{program}"));
            }
        }
    }

    fn file_types_and_protocols(&mut self, root: &RegRoot, classes: &RegKey) {
        // A file type is found through what it opens with, so those come first.
        let types = subkeys_where(root, CLASSES, |classes, name| {
            !name.starts_with('.') && self.type_has_something_of_the_apps(classes, name)
        });
        for name in &types {
            self.class(root, classes, name);
        }
        for shared in NESTED_SHARED_CLASSES {
            if let Ok(key) = classes.open_subkey_with_flags(shared, KEY_READ) {
                self.what_was_added_to(root, &key, &format!(r"{CLASSES}\{shared}"));
            }
        }
        self.system_file_associations(root, classes);
        let extensions = subkeys_where(root, CLASSES, |classes, name| {
            name.starts_with('.') && self.extension_has_something_of_the_apps(classes, name)
        });
        for extension in &extensions {
            self.extension(root, classes, extension);
        }
    }

    /// Whether [`Self::class`] would find anything in this.
    fn type_has_something_of_the_apps(&self, classes: &RegKey, name: &str) -> bool {
        let lower = name.to_ascii_lowercase();
        if NOT_CLASSES.contains(&lower.as_str()) || NEVER_AN_APPS.contains(&lower.as_str()) {
            return false;
        }
        classes
            .open_subkey_with_flags(name, KEY_READ)
            .is_ok_and(|key| {
                (!SHARED_CLASSES.contains(&lower.as_str()) && self.type_is_the_apps(&key))
                    || !self.verbs_added_to(&key).is_empty()
                    || !self.shell_extensions_added_to(&key).is_empty()
            })
    }

    /// Whether [`Self::extension`] would find anything in this.
    fn extension_has_something_of_the_apps(&self, classes: &RegKey, extension: &str) -> bool {
        let Ok(key) = classes.open_subkey_with_flags(extension, KEY_READ) else {
            return false;
        };
        self.owned
            .prog_ids
            .contains(&lower_trimmed(&default_value(&key)))
            || key
                .open_subkey_with_flags(OPEN_WITH_PROGIDS, KEY_READ)
                .is_ok_and(|list| {
                    list.enum_values()
                        .flatten()
                        .any(|(name, _)| self.owned.prog_ids.contains(&name.to_ascii_lowercase()))
                })
            || key
                .open_subkey_with_flags(OPEN_WITH_LIST, KEY_READ)
                .is_ok_and(|list| {
                    list.enum_keys()
                        .flatten()
                        .any(|program| self.is_opened_with_the_app(&program))
                })
            || !self.shell_extensions_added_to(&key).is_empty()
    }

    fn is_opened_with_the_app(&self, program: &str) -> bool {
        self.owned
            .server_files
            .contains(&program.to_ascii_lowercase())
            || self.is_the_apps_program_name(program)
    }

    /// One top-level key under `Classes`. If what it opens with is the app, or the class it
    /// creates is, all of it is the app's. Otherwise only what the app added to it, so that
    /// restoring cannot overwrite the rest.
    fn class(&mut self, root: &RegRoot, classes: &RegKey, name: &str) {
        let lower = name.to_ascii_lowercase();
        if NOT_CLASSES.contains(&lower.as_str()) || NEVER_AN_APPS.contains(&lower.as_str()) {
            return;
        }
        let Ok(key) = classes.open_subkey_with_flags(name, KEY_READ) else {
            return;
        };
        let key_path = format!(r"{CLASSES}\{name}");
        if !SHARED_CLASSES.contains(&lower.as_str()) && self.type_is_the_apps(&key) {
            self.owned.prog_ids.insert(lower);
            self.capture(root, &key_path);
            return;
        }
        self.what_was_added_to(root, &key, &key_path);
    }

    /// Double-clicking one opens the app, or it creates one of the app's classes. Its icon only
    /// counts if nothing says what opens it: an icon is nothing to go by when a command says
    /// otherwise. PowerShell 7 adds a verb to Windows' `LibraryFolder`, which does not make that
    /// type or its other verbs PowerShell's.
    fn type_is_the_apps(&self, key: &RegKey) -> bool {
        if let Some(opened_with) = self.what_opens(key) {
            return self.runs_the_app(&opened_with);
        }
        let class = default_of(key, CLASS_REFERENCE);
        if !class.trim().is_empty() {
            return self.owned.clsids.contains(&lower_trimmed(&class));
        }
        self.runs_the_app(&default_of(key, DEFAULT_ICON))
    }

    /// The command that opening a file of a type runs, if it says.
    fn what_opens(&self, key: &RegKey) -> Option<String> {
        let shell = key.open_subkey_with_flags(SHELL, KEY_READ).ok()?;
        let default_verb = default_value(&shell);
        let default_verb = default_verb.trim();
        let verbs = [default_verb, OPEN_VERB];
        verbs
            .iter()
            .filter(|verb| !verb.is_empty())
            .find_map(|verb| {
                let command = default_of(&shell, &format!(r"{verb}\{COMMAND}"));
                (!command.trim().is_empty()).then_some(command)
            })
    }

    /// The verbs and shell extensions an app added to a type that is not its own.
    fn what_was_added_to(&mut self, root: &RegRoot, key: &RegKey, key_path: &str) {
        for verb in self.verbs_added_to(key) {
            self.capture(root, &format!(r"{key_path}\{SHELL}\{verb}"));
        }
        self.shell_extensions_of(root, key, key_path);
    }

    fn verbs_added_to(&self, key: &RegKey) -> Vec<String> {
        let Ok(shell) = key.open_subkey_with_flags(SHELL, KEY_READ) else {
            return Vec::new();
        };
        shell
            .enum_keys()
            .flatten()
            .filter(|verb| {
                let Ok(verb_key) = shell.open_subkey_with_flags(verb, KEY_READ) else {
                    return false;
                };
                self.runs_the_app(&default_of(&verb_key, COMMAND))
                    || VERB_HANDLER_VALUES.iter().any(|name| {
                        self.handler_is_the_apps(&verb_key, name)
                            || verb_key
                                .open_subkey_with_flags(COMMAND, KEY_READ)
                                .is_ok_and(|command| self.handler_is_the_apps(&command, name))
                    })
            })
            .collect()
    }

    fn handler_is_the_apps(&self, key: &RegKey, name: &str) -> bool {
        key.get_raw_value(name).is_ok_and(|value| {
            self.owned
                .clsids
                .contains(&lower_trimmed(&raw_value_to_display(&value)))
        })
    }

    fn shell_extensions_of(&mut self, root: &RegRoot, key: &RegKey, key_path: &str) {
        for extension in self.shell_extensions_added_to(key) {
            self.capture(root, &format!(r"{key_path}\{SHELLEX}\{extension}"));
        }
    }

    /// `shellex\<kind>\<name>` holds a class, as its value or its name, and `shellex\{guid}` too.
    fn shell_extensions_added_to(&self, key: &RegKey) -> Vec<String> {
        let mut found = Vec::new();
        if self.owned.clsids.is_empty() {
            return found;
        }
        let Ok(extensions) = key.open_subkey_with_flags(SHELLEX, KEY_READ) else {
            return found;
        };
        for kind in extensions.enum_keys().flatten() {
            let Ok(kind_key) = extensions.open_subkey_with_flags(&kind, KEY_READ) else {
                continue;
            };
            if self
                .owned
                .clsids
                .contains(&lower_trimmed(&default_value(&kind_key)))
            {
                found.push(kind.clone());
            }
            for handler in kind_key.enum_keys().flatten() {
                let named = default_of(&kind_key, &handler);
                if self.owned.clsids.contains(&lower_trimmed(&named))
                    || self.owned.clsids.contains(&lower_trimmed(&handler))
                {
                    found.push(format!(r"{kind}\{handler}"));
                }
            }
        }
        found
    }

    fn system_file_associations(&mut self, root: &RegRoot, classes: &RegKey) {
        const ASSOCIATIONS: &str = "SystemFileAssociations";
        let Ok(associations) = classes.open_subkey_with_flags(ASSOCIATIONS, KEY_READ) else {
            return;
        };
        for kind in associations.enum_keys().flatten() {
            if let Ok(key) = associations.open_subkey_with_flags(&kind, KEY_READ) {
                self.what_was_added_to(root, &key, &format!(r"{CLASSES}\{ASSOCIATIONS}\{kind}"));
            }
        }
    }

    /// One `.ext`. Its own values only if it opens with a type of the app's, since what else is
    /// there can be anyone's. Otherwise only that it can be opened with the app.
    fn extension(&mut self, root: &RegRoot, classes: &RegKey, extension: &str) {
        let Ok(key) = classes.open_subkey_with_flags(extension, KEY_READ) else {
            return;
        };
        let key_path = format!(r"{CLASSES}\{extension}");
        let prog_id = default_value(&key);
        if !prog_id.trim().is_empty() && self.owned.prog_ids.contains(&lower_trimmed(&prog_id)) {
            let names: Vec<String> = key.enum_values().flatten().map(|(name, _)| name).collect();
            for name in names {
                self.capture_value(root, &key, &key_path, &name);
            }
            self.shell_new_of(root, &key, &key_path);
            self.found
                .file_associations
                .extend(file_associations_from_prog_id(
                    root,
                    CLASSES,
                    extension,
                    prog_id.trim(),
                ));
        }
        if let Ok(open_with) = key.open_subkey_with_flags(OPEN_WITH_PROGIDS, KEY_READ) {
            let path = format!(r"{key_path}\{OPEN_WITH_PROGIDS}");
            let names: Vec<String> = open_with
                .enum_values()
                .flatten()
                .map(|(name, _)| name)
                .filter(|name| self.owned.prog_ids.contains(&name.to_ascii_lowercase()))
                .collect();
            for name in names {
                self.capture_value(root, &open_with, &path, &name);
            }
        }
        if let Ok(open_with) = key.open_subkey_with_flags(OPEN_WITH_LIST, KEY_READ) {
            let programs: Vec<String> = open_with
                .enum_keys()
                .flatten()
                .filter(|program| self.is_opened_with_the_app(program))
                .collect();
            for program in programs {
                self.capture(root, &format!(r"{key_path}\{OPEN_WITH_LIST}\{program}"));
            }
        }
        self.shell_extensions_of(root, &key, &key_path);
    }

    fn is_the_apps_program_name(&self, program: &str) -> bool {
        self.inputs
            .owned
            .executable
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case(program))
    }

    /// What `New` in Explorer's menu makes, unless it runs something that is not the app's.
    fn shell_new_of(&mut self, root: &RegRoot, key: &RegKey, key_path: &str) {
        let Ok(new) = key.open_subkey_with_flags(SHELL_NEW, KEY_READ) else {
            return;
        };
        let command = new
            .get_raw_value(SHELL_NEW_COMMAND_VALUE)
            .map(|value| raw_value_to_display(&value))
            .unwrap_or_default();
        if command.trim().is_empty() || self.runs_the_app(&command) {
            self.capture(root, &format!(r"{key_path}\{SHELL_NEW}"));
        }
    }

    // ---- What is outside `Classes` ---------------------------------------------------------

    fn everything_else_of(&mut self, root: &RegRoot) {
        if !self.owned.clsids.is_empty() {
            self.explorer_registrations(root);
        }
        self.registered_applications(root);
        self.clients(root);
        self.odbc(root);
        self.fonts(root);
        self.compatibility_layers(root);
        self.certificates(root);
    }

    /// Overlay icons, what Explorer's list of approved extensions says, and folders it shows.
    fn explorer_registrations(&mut self, root: &RegRoot) {
        if let Ok(overlays) = root.open(ICON_OVERLAYS, KEY_READ) {
            for name in overlays.enum_keys().flatten() {
                if self
                    .owned
                    .clsids
                    .contains(&lower_trimmed(&default_of(&overlays, &name)))
                {
                    self.capture(root, &format!(r"{ICON_OVERLAYS}\{name}"));
                }
            }
        }
        if let Ok(approved) = root.open(APPROVED, KEY_READ) {
            let names: Vec<String> = approved
                .enum_values()
                .flatten()
                .map(|(name, _)| name)
                .filter(|name| self.owned.clsids.contains(&name.to_ascii_lowercase()))
                .collect();
            for name in names {
                self.capture_value(root, &approved, APPROVED, &name);
            }
        }
        for namespace in NAMESPACES {
            let path = format!(r"{EXPLORER}\{namespace}");
            if let Ok(key) = root.open(&path, KEY_READ) {
                for name in key.enum_keys().flatten() {
                    if self.owned.clsids.contains(&name.to_ascii_lowercase()) {
                        self.capture(root, &format!(r"{path}\{name}"));
                    }
                }
            }
        }
    }

    /// `RegisteredApplications` says where an app's capabilities are, which is what Default Apps
    /// offers. That only says it can be chosen: nothing is taken from anything else by it.
    fn registered_applications(&mut self, root: &RegRoot) {
        let Ok(registered) = root.open(REGISTERED_APPLICATIONS, KEY_READ) else {
            return;
        };
        for (name, value) in registered.enum_values().flatten() {
            let capabilities = raw_value_to_display(&value);
            let lower = capabilities.to_ascii_lowercase();
            if !lower.starts_with(SOFTWARE_PREFIX) {
                continue;
            }
            let Ok(key) = root.open(&capabilities, KEY_READ) else {
                continue;
            };
            let has_its_icon = key
                .get_raw_value(APPLICATION_ICON_VALUE)
                .is_ok_and(|icon| self.runs_the_app(&raw_value_to_display(&icon)));
            let opens_its_types = CAPABILITY_FILE_ASSOCIATIONS.iter().any(|group| {
                key.open_subkey_with_flags(group, KEY_READ)
                    .is_ok_and(|types| {
                        types.enum_values().flatten().any(|(_, prog_id)| {
                            self.owned
                                .prog_ids
                                .contains(&lower_trimmed(&raw_value_to_display(&prog_id)))
                        })
                    })
            });
            if has_its_icon || opens_its_types {
                self.capture_value(root, &registered, REGISTERED_APPLICATIONS, &name);
                self.capture(root, &capabilities);
            }
        }
    }

    /// `Software\Clients\<kind>\<app>` lists an app as a browser or mail program.
    fn clients(&mut self, root: &RegRoot) {
        let Ok(clients) = root.open(CLIENTS, KEY_READ) else {
            return;
        };
        for kind in clients.enum_keys().flatten() {
            let Ok(apps) = clients.open_subkey_with_flags(&kind, KEY_READ) else {
                continue;
            };
            for app in apps.enum_keys().flatten() {
                let Ok(key) = apps.open_subkey_with_flags(&app, KEY_READ) else {
                    continue;
                };
                let is_the_apps = self.what_opens(&key).map_or_else(
                    || self.runs_the_app(&default_of(&key, DEFAULT_ICON)),
                    |command| self.runs_the_app(&command),
                );
                if is_the_apps {
                    self.capture(root, &format!(r"{CLIENTS}\{kind}\{app}"));
                }
            }
        }
    }

    /// An ODBC driver is the app's if it names the app's file as its driver or its setup. A data
    /// source is if it uses such a driver, or is named for the app. Its password, if it holds one,
    /// goes the way of any other credential.
    fn odbc(&mut self, root: &RegRoot) {
        for view in ODBC_DRIVERS {
            let Ok(drivers) = root.open(view, KEY_READ) else {
                continue;
            };
            for name in drivers.enum_keys().flatten() {
                if ODBC_NOT_DRIVERS.contains(&name.to_ascii_lowercase().as_str()) {
                    continue;
                }
                let Ok(key) = drivers.open_subkey_with_flags(&name, KEY_READ) else {
                    continue;
                };
                let is_the_apps = [DRIVER_VALUE, SETUP_VALUE].iter().any(|value| {
                    key.get_raw_value(value)
                        .is_ok_and(|text| self.runs_the_app(&raw_value_to_display(&text)))
                });
                if !is_the_apps {
                    continue;
                }
                self.owned.odbc_drivers.insert(name.to_ascii_lowercase());
                self.capture(root, &format!(r"{view}\{name}"));
                let list_path = format!(r"{view}\{ODBC_DRIVER_LIST}");
                if let Ok(list) = root.open(&list_path, KEY_READ) {
                    self.capture_value(root, &list, &list_path, &name);
                }
            }
        }
        for view in ODBC_SOURCES {
            let Ok(sources) = root.open(view, KEY_READ) else {
                continue;
            };
            let list_path = format!(r"{view}\{ODBC_SOURCE_LIST}");
            let list = root.open(&list_path, KEY_READ).ok();
            for name in sources.enum_keys().flatten() {
                if ODBC_NOT_SOURCES.contains(&name.to_ascii_lowercase().as_str()) {
                    continue;
                }
                let Ok(key) = sources.open_subkey_with_flags(&name, KEY_READ) else {
                    continue;
                };
                let uses_its_driver = key
                    .get_raw_value(DRIVER_VALUE)
                    .is_ok_and(|driver| self.runs_the_app(&raw_value_to_display(&driver)))
                    || list.as_ref().is_some_and(|list| {
                        list.get_raw_value(&name).is_ok_and(|driver| {
                            self.owned
                                .odbc_drivers
                                .contains(&lower_trimmed(&raw_value_to_display(&driver)))
                        })
                    });
                if uses_its_driver
                    || self
                        .inputs
                        .names
                        .matches_top_level(&name, MatchScope::ProgramFolders)
                {
                    self.capture(root, &format!(r"{view}\{name}"));
                    if let Some(list) = list.as_ref() {
                        self.capture_value(root, list, &list_path, &name);
                    }
                }
            }
        }
    }

    /// A font registered by its path in the app's folder is the app's. One in the Windows folder is
    /// only if the app's install record names that file. That is carried under the install location
    /// and registered by path from there, so nothing is ever written to `Windows\Fonts`. Others that
    /// go by the app's name are reported, and stay.
    fn fonts(&mut self, root: &RegRoot) {
        let Ok(fonts) = root.open(FONTS, KEY_READ) else {
            return;
        };
        let windows_fonts = self.inputs.owned.windows_dir.join(WINDOWS_FONTS_FOLDER);
        // Only what is in that folder, which is few of what a record names, if any.
        let recorded: HashSet<String> = self
            .inputs
            .recorded_files
            .iter()
            .filter(|file| path_is_within(file, &windows_fonts))
            .map(|file| comparable_path(file))
            .collect();
        for (name, value) in fonts.enum_values().flatten() {
            let text = self.expanded(&raw_value_to_display(&value));
            let path = PathBuf::from(text.trim());
            if path.is_absolute() && !path_is_within(&path, &windows_fonts) {
                if self.inputs.owned.owns(&path) {
                    self.capture_value(root, &fonts, FONTS, &name);
                }
                continue;
            }
            let Some(file_name) = path.file_name() else {
                continue;
            };
            let in_windows = windows_fonts.join(file_name);
            if recorded.contains(&comparable_path(&in_windows)) {
                self.font_from_the_windows_folder(root, &in_windows, &name);
            } else if self.font_goes_by_the_apps_name(&name) {
                self.found.fonts_reported.push(name);
            }
        }
    }

    fn font_from_the_windows_folder(&mut self, root: &RegRoot, logical: &Path, name: &str) {
        let Some(on_disk) = self.system.resolve_source_path(logical) else {
            return;
        };
        let Ok(metadata) = crate::source_fs::metadata(&on_disk) else {
            return;
        };
        if !metadata.is_file() {
            return;
        }
        let text = logical.display().to_string();
        let hive = root.hive();
        let requires_admin = hive == RegistryHive::LocalMachine;
        self.found.entries.push(RegistryEntry {
            hive,
            key_path: FONTS.to_owned(),
            value_name: Some(name.to_owned()),
            value_bytes_hex: None,
            value_data: Some(text),
            value_kind: Some(RegistryValueKind::String),
            confidence: MatchConfidence::High,
            requires_admin,
        });
        self.found.font_files.push(FileEntry {
            path: on_disk.clone(),
            original_path: on_disk,
            package_relative_path: None,
            size_bytes: Some(metadata.len()),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: "A font the app's install record names".to_owned(),
            user_folder: None,
        });
    }

    fn font_goes_by_the_apps_name(&self, name: &str) -> bool {
        let font = normalize_name_for_match(name);
        self.inputs
            .display_name
            .map(normalize_name_for_match)
            .is_some_and(|product| {
                product.len() >= SHORTEST_NAME_TO_REPORT_BY && font.contains(&product)
            })
    }

    /// `Layers` holds a value named for each program, saying how to run it.
    fn compatibility_layers(&mut self, root: &RegRoot) {
        let Ok(layers) = root.open(APP_COMPAT_LAYERS, KEY_READ) else {
            return;
        };
        let names: Vec<String> = layers
            .enum_values()
            .flatten()
            .map(|(name, _)| name)
            .filter(|name| {
                let name = self.expanded(name);
                self.inputs.owned.owns(Path::new(name.trim()))
            })
            .collect();
        for name in names {
            self.capture_value(root, &layers, APP_COMPAT_LAYERS, &name);
        }
    }

    /// The certificates in a store of this account or the PC that go by the app's or its publisher's
    /// name. Their subject, and what they are called, and nothing else: a certificate is never
    /// restored, and which to trust is for the person installing to decide.
    fn certificates(&mut self, root: &RegRoot) {
        let hive = root.hive();
        for store in CERTIFICATE_STORES {
            let path = format!(r"{SYSTEM_CERTIFICATES}\{store}\Certificates");
            let Ok(certificates) = root.open(&path, KEY_READ) else {
                continue;
            };
            for thumbprint in certificates.enum_keys().flatten() {
                let blob = certificates
                    .open_subkey_with_flags(&thumbprint, KEY_READ)
                    .and_then(|key| key.get_raw_value(CERTIFICATE_BLOB_VALUE));
                let Ok(blob) = blob else {
                    continue;
                };
                let Some(subject) = certificate_subject(&blob.bytes) else {
                    continue;
                };
                if subject_names_the_app(&subject, self.inputs.display_name, self.inputs.publisher)
                {
                    self.found.certificates.push(CertificateEntry {
                        store: (*store).to_owned(),
                        hive: hive.clone(),
                        thumbprint,
                        subject: subject.text(),
                        why: WHY_NAMED_FOR_PUBLISHER.to_owned(),
                    });
                }
            }
        }
    }

    // ---- What is only ever machine-wide ------------------------------------------------------

    /// `EventLog\<log>\<source>`, if a file it takes its messages from is the app's. Only the values
    /// that make a source. .NET's own message file is what most managed sources name, and nothing
    /// says whose those are, so they stay.
    fn event_log_sources(&mut self, root: &RegRoot) {
        let Ok(logs) = root.open(EVENT_LOG, KEY_READ) else {
            return;
        };
        for log in logs.enum_keys().flatten() {
            let Ok(log_key) = logs.open_subkey_with_flags(&log, KEY_READ) else {
                continue;
            };
            for source in log_key.enum_keys().flatten() {
                let Ok(key) = log_key.open_subkey_with_flags(&source, KEY_READ) else {
                    continue;
                };
                let is_the_apps = EVENT_LOG_FILE_VALUES.iter().any(|value| {
                    key.get_raw_value(value).is_ok_and(|files| {
                        let text = self.expanded(&raw_value_to_display(&files));
                        text.split(';')
                            .any(|file| self.inputs.owned.owns(Path::new(file.trim())))
                    })
                });
                if !is_the_apps {
                    continue;
                }
                let path = format!(r"{EVENT_LOG}\{log}\{source}");
                for value in EVENT_LOG_FILE_VALUES.iter().chain(EVENT_LOG_OTHER_VALUES) {
                    self.capture_value(root, &key, &path, value);
                }
            }
        }
    }

    fn firewall_rules(&mut self, root: &RegRoot) {
        let Ok(rules) = root.open(FIREWALL_RULES, KEY_READ) else {
            return;
        };
        for (_, value) in rules.enum_values().flatten() {
            let text = raw_value_to_display(&value);
            let Some(mut rule) = parse_firewall_rule(&text) else {
                continue;
            };
            let program = PathBuf::from(self.expanded(&rule.program.to_string_lossy()));
            if self.inputs.owned.owns(&program) {
                rule.program = program;
                self.found.firewall_rules.push(rule);
            }
        }
    }
}

/// A rule as Windows stores it: `v2.33|Action=Allow|Active=TRUE|Dir=In|Protocol=6|LPort=80|App=...|
/// Name=...|`. `None` for one that names no program, which cannot be any one app's. What it says
/// that could not be set again, or must not be, goes in `not_carried`.
pub(crate) fn parse_firewall_rule(text: &str) -> Option<FirewallRuleEntry> {
    let mut rule = FirewallRuleEntry {
        name: String::new(),
        description: None,
        direction: String::new(),
        action: String::new(),
        enabled: false,
        program: PathBuf::new(),
        service: None,
        protocol: None,
        local_ports: Vec::new(),
        remote_ports: Vec::new(),
        local_addresses: Vec::new(),
        remote_addresses: Vec::new(),
        profiles: Vec::new(),
        not_carried: Vec::new(),
    };
    let mut said_once = std::collections::HashSet::new();
    for field in text.split('|').skip(1) {
        let Some((name, value)) = field.split_once('=') else {
            continue;
        };
        match name {
            // Windows writes each of these once. A second would be something a name or a description
            // was made to hold, and the last would otherwise win.
            "App" | "Name" | "Desc" | "Action" | "Dir" if !said_once.insert(name.to_owned()) => {
                rule.not_carried.push(name.to_owned());
            }
            "Action" => value.clone_into(&mut rule.action),
            "Active" => rule.enabled = value.eq_ignore_ascii_case("TRUE"),
            "Dir" => value.clone_into(&mut rule.direction),
            "Protocol" => rule.protocol = Some(value.to_owned()),
            "App" => rule.program = PathBuf::from(value),
            "Svc" => rule.service = Some(value.to_owned()),
            "Name" => value.clone_into(&mut rule.name),
            "Desc" => rule.description = Some(value.to_owned()),
            "Profile" => rule.profiles.push(value.to_owned()),
            "LPort" | "LPort2_10" => rule.local_ports.push(value.to_owned()),
            "RPort" | "RPort2_10" => rule.remote_ports.push(value.to_owned()),
            "LA4" | "LA6" => rule.local_addresses.push(value.to_owned()),
            "RA4" | "RA6" => rule.remote_addresses.push(value.to_owned()),
            // What group it is in, and what Windows Firewall uses to show it. It works without. A
            // `TTK` is a condition, and without it a rule would let in more, so it is not one of these.
            "EmbedCtxt" | "PCat" | "LUOwn" => {}
            // Leaving it to the app whether to come in through NAT, which Windows' prompt for a new
            // program sets. Without it the rule does less, so this is not a reason to refuse it.
            "Defer" => {}
            // This only says anything when it lets that in.
            "Edge" if !value.eq_ignore_ascii_case("TRUE") => {}
            "RA42" | "RA62" => rule.remote_addresses.push(value.to_owned()),
            other => rule.not_carried.push(other.to_owned()),
        }
    }
    (!rule.program.as_os_str().is_empty()).then_some(rule)
}

/// What names a certificate's subject.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CertificateSubject {
    pub common_name: Option<String>,
    pub organization: Option<String>,
}

impl CertificateSubject {
    fn text(&self) -> String {
        [
            self.common_name.as_deref().map(|name| format!("CN={name}")),
            self.organization.as_deref().map(|name| format!("O={name}")),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ")
    }
}

/// Whether a subject goes by the app's or its publisher's name, whole. Windows' own store is full
/// of Microsoft's, which says nothing about one of its apps.
pub(crate) fn subject_names_the_app(
    subject: &CertificateSubject,
    display_name: Option<&str>,
    publisher: Option<&str>,
) -> bool {
    let publisher = publisher
        .map(normalize_name_for_match)
        .filter(|name| name.len() >= SHORTEST_NAME_TO_REPORT_BY)
        .filter(|name| {
            !PUBLISHERS_OF_WINDOWS
                .iter()
                .any(|known| name.starts_with(known))
        });
    let product = display_name
        .map(normalize_name_for_match)
        .filter(|name| name.len() >= SHORTEST_NAME_TO_REPORT_BY);
    [
        subject.common_name.as_deref(),
        subject.organization.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(normalize_name_for_match)
    .any(|name| {
        publisher.as_deref() == Some(name.as_str())
            || product
                .as_deref()
                .is_some_and(|product| name.contains(product))
    })
}

/// The subject of a certificate from a stored blob: a list of properties, each its id, a reserved
/// word, and its length, one of which is the certificate.
pub(crate) fn certificate_subject(blob: &[u8]) -> Option<CertificateSubject> {
    let mut rest = blob;
    while rest.len() >= CERT_PROPERTY_HEADER_BYTES {
        let id = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
        let length = u32::from_le_bytes([rest[8], rest[9], rest[10], rest[11]]) as usize;
        let body =
            rest.get(CERT_PROPERTY_HEADER_BYTES..CERT_PROPERTY_HEADER_BYTES.checked_add(length)?)?;
        if id == CERT_PROPERTY_ENCODED {
            return subject_of_certificate(body);
        }
        rest = &rest[CERT_PROPERTY_HEADER_BYTES + length..];
    }
    None
}

/// One value in DER: its tag, what it holds, and what comes after it.
fn der_value(bytes: &[u8]) -> Option<(u8, &[u8], &[u8])> {
    let tag = *bytes.first()?;
    let first = *bytes.get(1)?;
    let (length, header) = if first < 0x80 {
        (usize::from(first), 2)
    } else {
        let count = usize::from(first & 0x7f);
        if count == 0 || count > 4 {
            return None;
        }
        let digits = bytes.get(2..2 + count)?;
        (
            digits
                .iter()
                .fold(0_usize, |total, digit| (total << 8) | usize::from(*digit)),
            2 + count,
        )
    };
    let end = header.checked_add(length)?;
    Some((tag, bytes.get(header..end)?, bytes.get(end..)?))
}

fn subject_of_certificate(der: &[u8]) -> Option<CertificateSubject> {
    const CONTEXT_VERSION: u8 = 0xa0;
    let (_, certificate, _) = der_value(der)?;
    let (_, to_be_signed, _) = der_value(certificate)?;
    let mut fields = to_be_signed;
    let (tag, _, after) = der_value(fields)?;
    if tag == CONTEXT_VERSION {
        fields = after;
    }
    // The serial number, the algorithm, the issuer, the validity, and then the subject.
    for _ in 0..4 {
        let (_, _, after) = der_value(fields)?;
        fields = after;
    }
    let (_, mut names, _) = der_value(fields)?;
    let mut subject = CertificateSubject::default();
    while !names.is_empty() {
        let (_, set, after) = der_value(names)?;
        names = after;
        let (_, pair, _) = der_value(set)?;
        let (_, oid, value) = der_value(pair)?;
        let (string_tag, text, _) = der_value(value)?;
        let text = der_text(string_tag, text);
        if oid == OID_COMMON_NAME {
            subject.common_name = text;
        } else if oid == OID_ORGANIZATION {
            subject.organization = text;
        }
    }
    Some(subject)
}

fn der_text(tag: u8, bytes: &[u8]) -> Option<String> {
    const BMP_STRING: u8 = 0x1e;
    if tag == BMP_STRING {
        let (pairs, _) = bytes.as_chunks::<2>();
        let units: Vec<u16> = pairs.iter().map(|pair| u16::from_be_bytes(*pair)).collect();
        return String::from_utf16(&units).ok();
    }
    Some(String::from_utf8_lossy(bytes).into_owned())
}

/// A key in a store of certificates, whatever captured it: these are reported, and are not to be
/// restored. `Some` with what names it if it can be told.
pub(crate) fn certificate_named_by(entry: &RegistryEntry) -> Option<CertificateEntry> {
    let lower = entry.key_path.to_ascii_lowercase();
    let plain = lower
        .strip_prefix(r"software\wow6432node\")
        .map_or(lower.clone(), |rest| format!("software\\{rest}"));
    let below = plain.strip_prefix(&format!(r"{}\", SYSTEM_CERTIFICATES.to_ascii_lowercase()))?;
    let mut parts = below.split('\\');
    let store = parts.next()?;
    let kind = parts.next()?;
    let thumbprint = parts.next()?;
    (kind == "certificates").then_some(())?;
    let subject = entry
        .value_bytes_hex
        .as_deref()
        .filter(|_| {
            entry
                .value_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(CERTIFICATE_BLOB_VALUE))
        })
        .and_then(|hex| crate::registry_target::hex_decode(hex).ok())
        .and_then(|bytes| certificate_subject(&bytes))
        .map(|subject| subject.text())
        .unwrap_or_default();
    Some(CertificateEntry {
        store: store.to_owned(),
        hive: entry.hive.clone(),
        thumbprint: thumbprint.to_owned(),
        subject,
        why: WHY_RECORDED.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads only. Every PC has this key, with a lot in it.
    #[test]
    fn going_through_a_big_key_on_threads_finds_what_going_one_at_a_time_would() {
        const BIG_KEY: &str = r"Software\Classes";
        let system = SystemContext::live();
        let root = RegRoot::Hklm(&system);
        let parent = root.open(BIG_KEY, KEY_READ);
        assert!(parent.is_ok(), "{parent:?}");
        let every: Vec<String> = parent
            .iter()
            .flat_map(|parent| parent.enum_keys().flatten())
            .collect();
        let wanted = |name: &str| name.starts_with('.');
        let one_at_a_time: Vec<String> =
            every.iter().filter(|name| wanted(name)).cloned().collect();
        assert!(!one_at_a_time.is_empty());
        assert_eq!(
            subkeys_where(&root, BIG_KEY, |_, name| wanted(name)),
            one_at_a_time
        );
        // Each thread reads through a handle of its own.
        let opened = subkeys_where(&root, BIG_KEY, |parent, name| {
            wanted(name) && parent.open_subkey_with_flags(name, KEY_READ).is_ok()
        });
        assert_eq!(opened, one_at_a_time);
        assert!(subkeys_where(&root, r"Software\WAM No Such Key", |_, _| true).is_empty());
    }

    const WINDOWS: &str = r"C:\Windows";

    fn owned() -> OwnedFiles {
        OwnedFiles::new(
            Path::new(r"C:\Program Files\Acme\acme.exe"),
            Some(Path::new(r"C:\Program Files\Acme")),
            Path::new(WINDOWS),
            [PathBuf::from(
                r"C:\Program Files\Common Files\Acme\shared.dll",
            )],
        )
    }

    #[test]
    fn an_entry_is_the_apps_by_what_it_runs() {
        let files = owned();
        for ours in [
            r"C:\Program Files\Acme\acme.exe",
            r"c:\program files\ACME\Acme.EXE",
            r#""C:\Program Files\Acme\acme.exe" "%1""#,
            r"C:\Program Files\Acme\shell.dll",
            r"C:\Program Files\Acme\sub folder\shell.dll",
            r"C:\Program Files\Acme\acme.exe,0",
            r"C:\Program Files\Common Files\Acme\shared.dll",
            // Run through something of Windows', which goes by what it is given.
            r#"C:\Windows\System32\wscript.exe "C:\Program Files\Acme\x.vbs""#,
            r#"rundll32.exe "C:\Program Files\Acme\shell.dll",Run"#,
        ] {
            assert_eq!(files.points_at(ours), PointsAt::TheApp, "{ours}");
        }
        for windows in [
            r"C:\Windows\System32\mscoree.dll",
            "mscoree.dll",
            "ole32.dll",
            r"rundll32.exe C:\Windows\System32\shell32.dll,Run",
        ] {
            assert_eq!(files.points_at(windows), PointsAt::Windows, "{windows}");
        }
        for elsewhere in [
            r"C:\Program Files\Other\other.exe",
            // Another app's program, given a file of this one's. It is still that app's.
            r#""C:\Program Files\Other\other.exe" --open "C:\Program Files\Acme\acme.exe""#,
            r"C:\Program Files\Acme Other\x.dll",
            r"C:\Program Files\Common Files\Other\x.dll",
            r"\\server\share\x.dll",
        ] {
            assert_eq!(
                files.points_at(elsewhere),
                PointsAt::Elsewhere,
                "{elsewhere}"
            );
        }
    }

    #[test]
    fn the_first_path_is_read_however_it_was_written() {
        for (text, path) in [
            (
                r#""C:\Program Files\A\a.exe" /x"#,
                r"C:\Program Files\A\a.exe",
            ),
            (r"C:\Program Files\A\a.exe /x", r"C:\Program Files\A\a.exe"),
            (r"C:\Program Files\A\a b.dll", r"C:\Program Files\A\a b.dll"),
            (r"C:\Program Files\A\a.exe,0", r"C:\Program Files\A\a.exe"),
            (r"C:\Program Files\A", r"C:\Program Files\A"),
            ("rundll32.exe a.dll,Run", "rundll32.exe"),
            ("mscoree.dll", "mscoree.dll"),
            ("something else", "something"),
        ] {
            assert_eq!(first_path_in(text), Some(PathBuf::from(path)), "{text}");
        }
        assert_eq!(first_path_in(""), None);
        assert_eq!(first_path_in(r#""unterminated"#), None);
    }

    #[test]
    fn a_file_url_is_read_as_the_path_it_is() {
        assert_eq!(
            path_of_file_url("file:///C:/Program Files/Acme/a.dll").as_deref(),
            Some(r"C:\Program Files\Acme\a.dll")
        );
        assert_eq!(path_of_file_url(r"C:\a.dll"), None);
    }

    #[test]
    fn a_firewall_rule_is_read_as_windows_stores_it() {
        let rule = parse_firewall_rule(
            r"v2.33|Action=Allow|Active=TRUE|Dir=In|Protocol=6|Profile=Private|Profile=Public|LPort=80|LPort=443|RA4=LocalSubnet|App=C:\Program Files\Acme\acme.exe|Name=Acme Web|Desc=Lets it in|EmbedCtxt=Acme|",
        );
        let Some(rule) = rule else {
            panic!("it names a program");
        };
        assert_eq!(rule.name, "Acme Web");
        assert_eq!(rule.action, "Allow");
        assert_eq!(rule.direction, "In");
        assert!(rule.enabled);
        assert_eq!(rule.program, Path::new(r"C:\Program Files\Acme\acme.exe"));
        assert_eq!(rule.protocol.as_deref(), Some("6"));
        assert_eq!(rule.local_ports, ["80", "443"]);
        assert_eq!(rule.remote_addresses, ["LocalSubnet"]);
        assert_eq!(rule.profiles, ["Private", "Public"]);
        assert!(rule.not_carried.is_empty(), "{:?}", rule.not_carried);

        let edge = parse_firewall_rule(
            r"v2.33|Action=Allow|Active=TRUE|Dir=In|App=C:\A\a.exe|Edge=TRUE|Defer=App|Name=x|",
        );
        assert_eq!(
            edge.map(|rule| rule.not_carried),
            Some(vec!["Edge".to_owned()])
        );
        assert_eq!(
            parse_firewall_rule(r"v2.33|Action=Allow|Active=TRUE|Dir=In|LPort=80|Name=all|"),
            None,
            "a rule for every program is no one app's"
        );
    }

    /// A certificate with just a subject, as DER has it. Everything before that is a stand-in of
    /// the right shape.
    fn der(tag: u8, content: &[u8]) -> Vec<u8> {
        let mut bytes = vec![tag];
        if content.len() < 0x80 {
            bytes.push(content.len() as u8);
        } else {
            bytes.push(0x82);
            bytes.extend_from_slice(&(content.len() as u16).to_be_bytes());
        }
        bytes.extend_from_slice(content);
        bytes
    }

    fn name_part(oid: &[u8], text: &str) -> Vec<u8> {
        let pair = [der(0x06, oid), der(0x0c, text.as_bytes())].concat();
        der(0x31, &der(0x30, &pair))
    }

    fn certificate_blob(common: &str, organization: &str) -> Vec<u8> {
        let subject = der(
            0x30,
            &[
                name_part(OID_COMMON_NAME, common),
                name_part(OID_ORGANIZATION, organization),
            ]
            .concat(),
        );
        let to_be_signed = der(
            0x30,
            &[
                der(0xa0, &der(0x02, &[2])),
                der(0x02, &[1]),
                der(0x30, &[]),
                der(0x30, &name_part(OID_COMMON_NAME, "The Issuer")),
                der(0x30, &[]),
                subject,
            ]
            .concat(),
        );
        let certificate = der(0x30, &to_be_signed);
        let mut blob = Vec::new();
        for (id, body) in [(3_u32, &[1_u8, 2, 3][..]), (32, &certificate[..])] {
            blob.extend_from_slice(&id.to_le_bytes());
            blob.extend_from_slice(&1_u32.to_le_bytes());
            blob.extend_from_slice(&(body.len() as u32).to_le_bytes());
            blob.extend_from_slice(body);
        }
        blob
    }

    #[test]
    fn a_certificates_subject_is_read_from_how_windows_stores_it() {
        assert_eq!(
            certificate_subject(&certificate_blob("Acme Root", "Acme Inc.")),
            Some(CertificateSubject {
                common_name: Some("Acme Root".to_owned()),
                organization: Some("Acme Inc.".to_owned()),
            })
        );
        for bad in [
            &b""[..],
            &[0xff; 11],
            &[32, 0, 0, 0, 1, 0, 0, 0, 200, 0, 0, 0, 1],
        ] {
            assert_eq!(certificate_subject(bad), None);
        }
    }

    #[test]
    fn a_certificate_is_tied_to_an_app_by_its_whole_name() {
        let subject = |common: &str, organization: &str| CertificateSubject {
            common_name: Some(common.to_owned()),
            organization: Some(organization.to_owned()),
        };
        let acme = (Some("Acme Studio 3"), Some("Acme Inc."));
        for tied in [
            subject("Whatever", "Acme Inc."),
            subject("ACME INC", "x"),
            subject("Acme Studio 3 Root", "x"),
        ] {
            assert!(subject_names_the_app(&tied, acme.0, acme.1), "{tied:?}");
        }
        for not in [
            subject("Acme", "Other"),
            subject("DigiCert Global Root", "DigiCert Inc"),
            subject("Other Acme Inc Thing", "x"),
        ] {
            assert!(!subject_names_the_app(&not, acme.0, acme.1), "{not:?}");
        }
        assert!(
            !subject_names_the_app(
                &subject("Microsoft Root Authority", "Microsoft Corporation"),
                Some("Windows App"),
                Some("Microsoft Corporation")
            ),
            "Windows' own store is full of these, and they say nothing about an app of theirs"
        );
    }

    #[test]
    fn what_a_record_captured_from_a_store_of_certificates_is_only_named() {
        let entry = RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: r"SOFTWARE\Microsoft\SystemCertificates\ROOT\Certificates\ABC123".to_owned(),
            value_name: Some("Blob".to_owned()),
            value_data: None,
            value_kind: Some(RegistryValueKind::Binary),
            value_bytes_hex: Some(crate::hashing::hex_lower(&certificate_blob("A", "B"))),
            confidence: MatchConfidence::High,
            requires_admin: true,
        };
        let named = certificate_named_by(&entry);
        assert_eq!(
            named.as_ref().map(|found| found.store.as_str()),
            Some("root")
        );
        assert_eq!(
            named.as_ref().map(|found| found.thumbprint.as_str()),
            Some("abc123")
        );
        assert_eq!(
            named.map(|found| found.subject),
            Some("CN=A, O=B".to_owned())
        );
        let other = RegistryEntry {
            key_path: r"Software\Acme\Certificates\ABC123".to_owned(),
            ..entry
        };
        assert_eq!(certificate_named_by(&other), None);
    }
}
