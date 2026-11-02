use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use tracing::warn;
use winreg::enums::{
    KEY_READ, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ,
    REG_FULL_RESOURCE_DESCRIPTOR, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_RESOURCE_LIST,
    REG_RESOURCE_REQUIREMENTS_LIST, REG_SZ, RegType,
};
use winreg::{RegKey, RegValue};

use crate::hashing::hex_lower;
use crate::models::{
    ComRegistrationEntry, ComServerKind, FileAssociationEntry, MatchConfidence, RegistryEntry,
    RegistryHive, RegistryValueKind, Warning, WarningSeverity,
};
use crate::path_utils::{
    normalize_name_for_match, path_is_within, split_command_executable_path, text_mentions_path,
};
use crate::product_match::{MatchScope, ProductNames};
use crate::system_context::{SystemContext, UserHive};

/// A registry root to open logical key paths against. Live opens `predef`
/// hives; offline maps each logical path onto the mounted hive while callers
/// keep the logical path for capture/restore.
enum RegRoot<'a> {
    Hklm(&'a SystemContext),
    User(&'a UserHive),
}

impl RegRoot<'_> {
    fn open(&self, logical: &str, flags: u32) -> std::io::Result<RegKey> {
        match self {
            RegRoot::Hklm(system) => system.open_hklm(logical, flags),
            RegRoot::User(user) => user.open(logical, flags),
        }
    }

    fn hive(&self) -> RegistryHive {
        match self {
            RegRoot::Hklm(_) => RegistryHive::LocalMachine,
            RegRoot::User(_) => RegistryHive::CurrentUser,
        }
    }
}

const APP_PATHS: &str = r"Software\Microsoft\Windows\CurrentVersion\App Paths";
const HKLM_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const HKLM_WOW6432_UNINSTALL: &str =
    r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";
const HKCU_UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
const SOFTWARE: &str = r"Software";
const WOW6432_SOFTWARE: &str = r"Software\WOW6432Node";
const SOFTWARE_VIEWS: [&str; 2] = [SOFTWARE, WOW6432_SOFTWARE];
const WOW6432_NODE: &str = "WOW6432Node";
const MICROSOFT: &str = "Microsoft";
const CLASSES: &str = "Classes";
const HKCU_CLASSES: &str = r"Software\Classes";
const HKLM_CLASSES: &str = r"Software\Classes";
const HKCU_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKCU_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKLM_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_WOW6432_CLASSES_CLSID: &str = r"Software\WOW6432Node\Classes\CLSID";
const MAX_REGISTRY_CAPTURE_DEPTH: usize = 8;
const MAX_REGISTRY_CAPTURE_KEYS: usize = 512;
/// Each value is kept as text and hex too, so this is a good deal more in memory.
const MAX_REGISTRY_CAPTURE_BYTES: usize = 64 * 1024 * 1024;
const MAX_REGISTRY_VALUE_BYTES: usize = 1024 * 1024;
const MAX_REGISTRY_MATCH_SCAN_DEPTH: usize = 4;
/// How far under `Software\<Vendor>` to look for a key recording the install folder.
const MAX_INSTALL_PATH_KEY_DEPTH: usize = 3;
/// `Software` children never searched for an install path: they hold everything, so a
/// value there says nothing about who owns the key, and they are large.
const INSTALL_PATH_SEARCH_SKIPPED_KEYS: &[&str] =
    &["classes", "microsoft", "policies", "wow6432node"];
/// Value names an installer uses to record where it installed. Other names can point at
/// the folder too, but `MSysGit` in TortoiseGit's key does not make that key Git's.
const INSTALL_PATH_VALUE_NAMES: &[&str] = &[
    "path",
    "exepath",
    "apppath",
    "installpath",
    "installdir",
    "installdirectory",
    "installfolder",
    "installlocation",
    "installroot",
    "location",
    "home",
    "rootdir",
    "targetdir",
];
/// Keys that hold every app's entries side by side. A record naming one of these keys
/// whole can never mean all of it belongs to one app; a record naming a single value in
/// one still captures just that value.
const SHARED_REGISTRY_KEYS: &[&str] = &[
    r"software",
    r"software\classes",
    r"software\classes\clsid",
    r"software\classes\interface",
    r"software\classes\typelib",
    r"software\classes\appid",
    r"software\classes\applications",
    r"software\classes\directory",
    r"software\classes\directory\background",
    r"software\classes\directory\background\shell",
    r"software\classes\directory\shell",
    r"software\classes\*",
    r"software\classes\*\shell",
    r"software\clients",
    r"software\microsoft",
    r"software\microsoft\windows",
    r"software\microsoft\windows\currentversion",
    r"software\microsoft\windows\currentversion\app paths",
    r"software\microsoft\windows\currentversion\explorer",
    r"software\microsoft\windows\currentversion\run",
    r"software\microsoft\windows\currentversion\runonce",
    r"software\microsoft\windows\currentversion\shareddlls",
    r"software\microsoft\windows\currentversion\uninstall",
    r"software\microsoft\windows nt",
    r"software\microsoft\windows nt\currentversion",
    r"software\policies",
    r"software\registeredapplications",
];
const RUN_KEYS: &[&str] = &[
    r"Software\Microsoft\Windows\CurrentVersion\Run",
    r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
];

/// The app's paths are as its own PC calls them, since that is what the values they are
/// compared with hold, wherever the drive is now.
#[derive(Debug, Clone)]
pub struct RegistryScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    pub display_name: Option<String>,
    pub publisher: Option<String>,
    pub source_registry_path: Option<String>,
    /// Keys and values the app's install record names: exact, so they are captured
    /// whatever they are called.
    pub recorded_keys: Vec<RecordedRegistryKey>,
}

/// A registry root as an install record names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedRegistryRoot {
    ClassesRoot,
    CurrentUser,
    LocalMachine,
}

/// A registry key an install record says belongs to the app (MSI component key paths and
/// the registry records of an Inno Setup log).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRegistryKey {
    pub root: RecordedRegistryRoot,
    pub key_path: String,
    /// One value in the key, or `None` for the key and everything under it.
    pub value_name: Option<String>,
    /// Recorded in the 32-bit view, so on 64-bit Windows it lives under `WOW6432Node`.
    pub wow64_32: bool,
}

impl RecordedRegistryKey {
    /// Identifies a key regardless of case.
    pub(crate) fn identity(&self) -> String {
        format!(
            "{:?}|{}|{}|{}",
            self.root,
            self.wow64_32,
            self.key_path.to_ascii_lowercase(),
            self.value_name
                .as_deref()
                .map(str::to_ascii_lowercase)
                .unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct RegistryScanArtifacts {
    pub entries: Vec<RegistryEntry>,
    pub com_registrations: Vec<ComRegistrationEntry>,
    pub file_associations: Vec<FileAssociationEntry>,
    pub warnings: Vec<Warning>,
}

pub fn scan_registry(
    context: &RegistryScanContext,
    system: &SystemContext,
) -> (Vec<RegistryEntry>, Vec<Warning>) {
    let artifacts = scan_registry_artifacts(context, system);
    (artifacts.entries, artifacts.warnings)
}

pub fn scan_registry_artifacts(
    context: &RegistryScanContext,
    system: &SystemContext,
) -> RegistryScanArtifacts {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let mut file_associations = Vec::new();

    let names = context_product_names(context);
    let hklm = RegRoot::Hklm(system);
    scan_recorded_keys(context, &hklm, &mut entries, &mut warnings);
    scan_app_paths(context, &hklm, &mut entries, &mut warnings);
    scan_uninstall_entries(context, &hklm, HKLM_UNINSTALL, &mut entries, &mut warnings);
    scan_uninstall_entries(
        context,
        &hklm,
        HKLM_WOW6432_UNINSTALL,
        &mut entries,
        &mut warnings,
    );
    scan_run_keys(context, &hklm, &mut entries, &mut warnings);
    scan_software_children(context, &names, &hklm, &mut entries, &mut warnings);
    scan_class_registrations(
        context,
        &hklm,
        HKLM_CLASSES_APPLICATIONS,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        &hklm,
        HKLM_CLASSES_CLSID,
        &mut entries,
        &mut warnings,
    );
    scan_class_registrations(
        context,
        &hklm,
        HKLM_WOW6432_CLASSES_CLSID,
        &mut entries,
        &mut warnings,
    );
    scan_file_extension_associations(
        context,
        &hklm,
        HKLM_CLASSES,
        &mut entries,
        &mut warnings,
        &mut file_associations,
    );

    // Per-user hives: live yields the current user; offline yields every profile.
    for user in system.user_hives() {
        let hkcu = RegRoot::User(&user);
        scan_recorded_keys(context, &hkcu, &mut entries, &mut warnings);
        scan_app_paths(context, &hkcu, &mut entries, &mut warnings);
        scan_uninstall_entries(context, &hkcu, HKCU_UNINSTALL, &mut entries, &mut warnings);
        scan_run_keys(context, &hkcu, &mut entries, &mut warnings);
        scan_software_children(context, &names, &hkcu, &mut entries, &mut warnings);
        scan_class_registrations(
            context,
            &hkcu,
            HKCU_CLASSES_APPLICATIONS,
            &mut entries,
            &mut warnings,
        );
        scan_class_registrations(
            context,
            &hkcu,
            HKCU_CLASSES_CLSID,
            &mut entries,
            &mut warnings,
        );
        scan_file_extension_associations(
            context,
            &hkcu,
            HKCU_CLASSES,
            &mut entries,
            &mut warnings,
            &mut file_associations,
        );
    }

    entries.sort_by(|left, right| {
        left.key_path
            .to_ascii_lowercase()
            .cmp(&right.key_path.to_ascii_lowercase())
            .then_with(|| left.value_name.cmp(&right.value_name))
    });
    entries.dedup_by(|left, right| {
        left.hive == right.hive
            && left.key_path.eq_ignore_ascii_case(&right.key_path)
            && left.value_name == right.value_name
    });

    let com_registrations = classify_com_registrations(&entries);
    file_associations.extend(classify_application_file_associations(&entries));
    file_associations.sort_by(|left, right| {
        format!(
            "{:?}:{:?}:{:?}:{:?}",
            left.hive, left.extension, left.prog_id, left.verb
        )
        .cmp(&format!(
            "{:?}:{:?}:{:?}:{:?}",
            right.hive, right.extension, right.prog_id, right.verb
        ))
    });
    file_associations.dedup_by(|left, right| {
        left.hive == right.hive
            && left.extension == right.extension
            && left.prog_id == right.prog_id
            && left.application_name == right.application_name
            && left.verb == right.verb
            && left.command == right.command
    });

    RegistryScanArtifacts {
        entries,
        com_registrations,
        file_associations,
        warnings,
    }
}

struct RegistryCaptureContext<'a> {
    root: &'a RegRoot<'a>,
    hive: RegistryHive,
    confidence: MatchConfidence,
    entries: &'a mut Vec<RegistryEntry>,
    warnings: &'a mut Vec<Warning>,
    keys_seen: usize,
    bytes_seen: usize,
}

fn scan_class_registrations(
    context: &RegistryScanContext,
    root: &RegRoot,
    base_path: &str,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Ok(base_key) = root.open(base_path, KEY_READ) else {
        return;
    };
    for child_name in base_key.enum_keys().flatten() {
        let key_path = format!(r"{base_path}\{child_name}");
        if registry_tree_matches_context(root, &key_path, context, 0) {
            capture_registry_subtree(root, &key_path, MatchConfidence::High, entries, warnings);
        }
    }
}

fn scan_file_extension_associations(
    context: &RegistryScanContext,
    root: &RegRoot,
    classes_path: &str,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
    file_associations: &mut Vec<FileAssociationEntry>,
) {
    let Ok(classes_key) = root.open(classes_path, KEY_READ) else {
        return;
    };

    for extension_name in classes_key
        .enum_keys()
        .flatten()
        .filter(|name| name.starts_with('.'))
    {
        let extension_path = format!(r"{classes_path}\{extension_name}");
        let Ok(extension_key) = root.open(&extension_path, KEY_READ) else {
            continue;
        };
        let Ok(default_value) = extension_key.get_raw_value("") else {
            continue;
        };
        let prog_id = raw_value_to_display(&default_value);
        if prog_id.trim().is_empty() {
            continue;
        }
        let prog_id_path = format!(r"{classes_path}\{prog_id}");
        if !prog_id_opens_with_app(root, &prog_id_path, context) {
            capture_verbs_pointing_at_app(root, &prog_id_path, context, entries, warnings);
            continue;
        }

        capture_registry_subtree(
            root,
            &extension_path,
            MatchConfidence::High,
            entries,
            warnings,
        );
        capture_registry_subtree(
            root,
            &prog_id_path,
            MatchConfidence::High,
            entries,
            warnings,
        );
        file_associations.extend(file_associations_from_prog_id(
            root,
            classes_path,
            &extension_name,
            &prog_id,
        ));
    }
}

/// True when a file type belongs to this app: double-clicking one opens it, or the app
/// supplies its icon. Mentioning the app somewhere is not enough. PowerShell 7 adds a verb
/// to Windows' `LibraryFolder`, and that does not make the type or its other verbs PowerShell's.
fn prog_id_opens_with_app(
    root: &RegRoot,
    prog_id_path: &str,
    context: &RegistryScanContext,
) -> bool {
    let default_value = |key_path: &str| {
        root.open(key_path, KEY_READ)
            .and_then(|key| key.get_raw_value(""))
            .map(|value| raw_value_to_display(&value))
            .unwrap_or_default()
    };
    let default_verb = default_value(&format!(r"{prog_id_path}\shell"));
    prog_id_ownership_keys(prog_id_path, &default_verb)
        .iter()
        .any(|key_path| value_matches_context(context, &default_value(key_path)))
}

/// The keys that say what a file type opens with and how it looks.
fn prog_id_ownership_keys(prog_id_path: &str, default_verb: &str) -> Vec<String> {
    let mut keys = vec![format!(r"{prog_id_path}\shell\open\command")];
    let default_verb = default_verb.trim();
    if !default_verb.is_empty() && !default_verb.eq_ignore_ascii_case("open") {
        keys.push(format!(r"{prog_id_path}\shell\{default_verb}\command"));
    }
    keys.push(format!(r"{prog_id_path}\DefaultIcon"));
    keys
}

/// Capture just the verbs an app adds to a file type it does not own. Other apps' verbs
/// and the type itself stay out, so restoring cannot overwrite them.
fn capture_verbs_pointing_at_app(
    root: &RegRoot,
    prog_id_path: &str,
    context: &RegistryScanContext,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let mut matching = Vec::new();
    collect_keys_matching_context(root, prog_id_path, context, 0, &mut matching);
    for key_path in matching {
        capture_registry_subtree(
            root,
            verb_key_of(&key_path),
            MatchConfidence::High,
            entries,
            warnings,
        );
    }
}

/// The verb a matching key belongs to: a `command` key's parent, which also holds the
/// verb's label and icon. Any other key stands for itself.
fn verb_key_of(key_path: &str) -> &str {
    key_path
        .rsplit_once('\\')
        .filter(|(_, leaf)| leaf.eq_ignore_ascii_case("command"))
        .map_or(key_path, |(parent, _)| parent)
}

fn collect_keys_matching_context(
    root: &RegRoot,
    key_path: &str,
    context: &RegistryScanContext,
    depth: usize,
    matching: &mut Vec<String>,
) {
    if depth > MAX_REGISTRY_MATCH_SCAN_DEPTH {
        return;
    }
    let Ok(key) = root.open(key_path, KEY_READ) else {
        return;
    };
    if key
        .enum_values()
        .flatten()
        .any(|value| value_matches_context(context, &raw_value_to_display(&value.1)))
    {
        matching.push(key_path.to_owned());
        return;
    }
    for child_name in key.enum_keys().flatten() {
        collect_keys_matching_context(
            root,
            &format!(r"{key_path}\{child_name}"),
            context,
            depth + 1,
            matching,
        );
    }
}

fn file_associations_from_prog_id(
    root: &RegRoot,
    classes_path: &str,
    extension: &str,
    prog_id: &str,
) -> Vec<FileAssociationEntry> {
    let hive = root.hive();
    let shell_path = format!(r"{classes_path}\{prog_id}\shell");
    let Ok(shell_key) = root.open(&shell_path, KEY_READ) else {
        return vec![FileAssociationEntry {
            hive: hive.clone(),
            extension: Some(extension.to_owned()),
            prog_id: Some(prog_id.to_owned()),
            application_name: None,
            verb: None,
            command: None,
            confidence: MatchConfidence::High,
            requires_admin: hive == RegistryHive::LocalMachine,
        }];
    };

    let mut associations = Vec::new();
    for verb in shell_key.enum_keys().flatten() {
        let command_path = format!(r"{shell_path}\{verb}\command");
        let Ok(command_key) = root.open(&command_path, KEY_READ) else {
            continue;
        };
        let command = command_key
            .get_raw_value("")
            .ok()
            .map(|raw| raw_value_to_display(&raw));
        associations.push(FileAssociationEntry {
            hive: hive.clone(),
            extension: Some(extension.to_owned()),
            prog_id: Some(prog_id.to_owned()),
            application_name: None,
            verb: Some(verb),
            command,
            confidence: MatchConfidence::High,
            requires_admin: hive == RegistryHive::LocalMachine,
        });
    }
    if associations.is_empty() {
        associations.push(FileAssociationEntry {
            hive: hive.clone(),
            extension: Some(extension.to_owned()),
            prog_id: Some(prog_id.to_owned()),
            application_name: None,
            verb: None,
            command: None,
            confidence: MatchConfidence::High,
            requires_admin: hive == RegistryHive::LocalMachine,
        });
    }
    associations
}

fn scan_uninstall_entries(
    context: &RegistryScanContext,
    root: &RegRoot,
    uninstall_path: &str,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let key = match root.open(uninstall_path, KEY_READ) {
        Ok(key) => key,
        Err(_) => return,
    };
    let hive = root.hive();

    for child_name in key.enum_keys().flatten() {
        let key_path = format!(r"{uninstall_path}\{child_name}");
        if source_registry_path_matches(context, &hive, &key_path) {
            capture_registry_subtree(root, &key_path, MatchConfidence::High, entries, warnings);
            continue;
        }

        let Ok(child) = root.open(&key_path, KEY_READ) else {
            continue;
        };
        if uninstall_key_matches_context(context, &child) {
            capture_registry_subtree(root, &key_path, MatchConfidence::High, entries, warnings);
        }
    }
}

fn scan_app_paths(
    context: &RegistryScanContext,
    root: &RegRoot,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let key = match root.open(APP_PATHS, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            warn!(%error, "App Paths registry key unavailable");
            return;
        }
    };

    for child_name in key.enum_keys().flatten() {
        let child = match key.open_subkey_with_flags(&child_name, KEY_READ) {
            Ok(child) => child,
            Err(error) => {
                warnings.push(Warning::new(
                    "registry_key_unreadable",
                    WarningSeverity::Info,
                    format!("Could not read App Paths child key: {error}"),
                    Some(child_name),
                ));
                continue;
            }
        };
        let mut matched = false;
        for value_name in ["", "Path"] {
            if let Ok(value) = child.get_raw_value(value_name) {
                let value_display = raw_value_to_display(&value);
                if value_matches_context(context, &value_display) {
                    matched = true;
                }
            }
        }
        if matched {
            capture_registry_subtree(
                root,
                &format!(r"{APP_PATHS}\{child_name}"),
                MatchConfidence::High,
                entries,
                warnings,
            );
        }
    }
}

fn scan_run_keys(
    context: &RegistryScanContext,
    root: &RegRoot,
    entries: &mut Vec<RegistryEntry>,
    _warnings: &mut Vec<Warning>,
) {
    let hive = root.hive();
    for key_path in RUN_KEYS {
        let Ok(key) = root.open(key_path, KEY_READ) else {
            continue;
        };
        for value in key.enum_values().flatten() {
            let value_name = value.0;
            let raw_value = value.1;
            let value_data = raw_value_to_display(&raw_value);
            if value_matches_context(context, &value_data) {
                entries.push(entry_from_raw_value(
                    hive.clone(),
                    (*key_path).to_owned(),
                    value_name,
                    raw_value,
                    MatchConfidence::High,
                    hive == RegistryHive::LocalMachine,
                ));
            }
        }
    }
}

/// The names this app's registry keys can go by, as for its folders.
fn context_product_names(context: &RegistryScanContext) -> ProductNames {
    let mut aliases = Vec::new();
    if let Some(stem) = context.executable_path.file_stem().and_then(|s| s.to_str()) {
        aliases.push(stem);
    }
    let vendor_folders: Vec<&str> = context
        .install_location
        .as_deref()
        .and_then(Path::parent)
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .into_iter()
        .collect();
    ProductNames::new(
        context.display_name.as_deref(),
        context.publisher.as_deref(),
        &aliases,
        &vendor_folders,
    )
}

/// Capture the keys and values the app's install record names. Exact records need no
/// matching, so this also finds keys named nothing like the product, such as Git for
/// Windows' `Software\GitForWindows`. Records of other accounts and keys that are gone
/// are skipped.
fn scan_recorded_keys(
    context: &RegistryScanContext,
    root: &RegRoot,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    for recorded in &context.recorded_keys {
        for key_path in recorded_key_paths(recorded, &root.hive()) {
            let Ok(key) = root.open(&key_path, KEY_READ) else {
                continue;
            };
            match recorded.value_name.as_deref() {
                Some(value_name) => {
                    if let Ok(value) = key.get_raw_value(value_name)
                        && value.bytes.len() <= MAX_REGISTRY_VALUE_BYTES
                    {
                        entries.push(entry_from_raw_value(
                            root.hive(),
                            key_path,
                            value_name.to_owned(),
                            value,
                            MatchConfidence::High,
                            root.hive() == RegistryHive::LocalMachine,
                        ));
                    }
                }
                None if is_shared_registry_key(&key_path) => {}
                None => capture_registry_subtree(
                    root,
                    &key_path,
                    MatchConfidence::High,
                    entries,
                    warnings,
                ),
            }
            break;
        }
    }
}

/// What a record can say about an app installed for one account. Without rights it could not have
/// written anywhere else in the registry, and the record is a file in that account's own folder,
/// which could name a machine's secrets, for an administrator to package. `HKEY_CLASSES_ROOT` is
/// that account's own classes when it writes to it.
pub fn recorded_keys_of_one_account(keys: Vec<RecordedRegistryKey>) -> Vec<RecordedRegistryKey> {
    keys.into_iter()
        .filter_map(|key| match key.root {
            RecordedRegistryRoot::CurrentUser => Some(key),
            RecordedRegistryRoot::ClassesRoot => Some(RecordedRegistryKey {
                root: RecordedRegistryRoot::CurrentUser,
                key_path: format!(r"{HKLM_CLASSES}\{}", key.key_path),
                ..key
            }),
            RecordedRegistryRoot::LocalMachine => None,
        })
        .collect()
}

/// Where a recorded key lives in `hive`, most likely path first. Empty when it belongs
/// to the other hive. `HKEY_CLASSES_ROOT` is a merged view, so it can be either's
/// `Software\Classes`. A key recorded in the 32-bit view is under `WOW6432Node` on 64-bit
/// Windows, and at its plain path on 32-bit Windows.
fn recorded_key_paths(recorded: &RecordedRegistryKey, hive: &RegistryHive) -> Vec<String> {
    match recorded.root {
        RecordedRegistryRoot::ClassesRoot => vec![format!(r"{HKLM_CLASSES}\{}", recorded.key_path)],
        RecordedRegistryRoot::CurrentUser if *hive == RegistryHive::CurrentUser => {
            vec![recorded.key_path.clone()]
        }
        RecordedRegistryRoot::LocalMachine if *hive == RegistryHive::LocalMachine => {
            let plain = recorded.key_path.clone();
            match wow6432_path(&plain).filter(|_| recorded.wow64_32) {
                Some(redirected) => vec![redirected, plain],
                None => vec![plain],
            }
        }
        _ => Vec::new(),
    }
}

/// `Software\Foo` as seen by a 32-bit process on 64-bit Windows.
fn wow6432_path(key_path: &str) -> Option<String> {
    let (first, rest) = key_path.split_once('\\')?;
    (first.eq_ignore_ascii_case(SOFTWARE) && !rest.to_ascii_lowercase().starts_with("wow6432node"))
        .then(|| format!(r"{WOW6432_SOFTWARE}\{rest}"))
}

fn is_shared_registry_key(key_path: &str) -> bool {
    let lower = key_path.to_ascii_lowercase();
    let plain = lower
        .strip_prefix(r"software\wow6432node\")
        .map_or(lower.clone(), |rest| format!(r"software\{rest}"));
    SHARED_REGISTRY_KEYS.contains(&plain.as_str())
}

/// Capture this app's keys under `Software`, in both registry views. A key counts when it
/// is named exactly for the product, either directly or under a vendor's key. A vendor's
/// key by itself never selects anything: that is how `Mozilla` took Thunderbird and
/// `Adobe` took every Adobe product.
fn scan_software_children(
    context: &RegistryScanContext,
    names: &ProductNames,
    root: &RegRoot,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let children_of = |key_path: &str| -> Vec<String> {
        root.open(key_path, KEY_READ)
            .map(|key| key.enum_keys().flatten().collect())
            .unwrap_or_default()
    };
    for software_path in SOFTWARE_VIEWS {
        for key_path in product_keys_in_view(names, software_path, &children_of) {
            capture_registry_subtree(root, &key_path, MatchConfidence::Medium, entries, warnings);
        }
        let Some(install) = context.install_location.as_deref() else {
            continue;
        };
        for child_name in children_of(software_path) {
            if !INSTALL_PATH_SEARCH_SKIPPED_KEYS.contains(&child_name.to_ascii_lowercase().as_str())
            {
                capture_keys_recording_install_path(
                    root,
                    &format!(r"{software_path}\{child_name}"),
                    install,
                    1,
                    entries,
                    warnings,
                );
            }
        }
    }
}

/// The keys in one `Software` view named for the product. `children_of` lists a key's
/// children, which keeps these rules free of the live registry.
pub(crate) fn product_keys_in_view(
    names: &ProductNames,
    software_path: &str,
    children_of: &impl Fn(&str) -> Vec<String>,
) -> Vec<String> {
    let mut keys = Vec::new();
    for child_name in children_of(software_path) {
        if child_name.eq_ignore_ascii_case(WOW6432_NODE) || child_name.eq_ignore_ascii_case(CLASSES)
        {
            continue;
        }
        let child_path = format!(r"{software_path}\{child_name}");
        if names.matches_top_level(&child_name, MatchScope::ProgramFolders) {
            keys.push(child_path);
            continue;
        }
        // Everything on Windows sits under Software\Microsoft, Windows' own components
        // included, so being published by Microsoft qualifies nothing there.
        let holds_everything = child_name.eq_ignore_ascii_case(MICROSOFT);
        for grandchild_name in children_of(&child_path) {
            let matched = if holds_everything {
                names.spells_out_product(&child_name, &grandchild_name)
            } else {
                names.matches_nested(&child_name, &grandchild_name, MatchScope::ProgramFolders)
            };
            if matched {
                keys.push(format!(r"{child_path}\{grandchild_name}"));
            }
        }
    }
    keys
}

/// Capture any key under a vendor's key that records the app's install folder in a value
/// named for that, such as VLC's `Software\VideoLAN\VLC` with its `InstallDir`. This
/// finds the app's own key when its name is not the product's, without taking the
/// vendor's other products.
fn capture_keys_recording_install_path(
    root: &RegRoot,
    key_path: &str,
    install: &Path,
    depth: usize,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let Ok(key) = root.open(key_path, KEY_READ) else {
        return;
    };
    let key_leaf = key_path.rsplit('\\').next().unwrap_or_default();
    let records_install = key.enum_values().flatten().any(|(name, value)| {
        let value_name = if name.is_empty() { key_leaf } else { &name };
        value_records_install_path(value_name, &raw_value_to_display(&value), install)
    });
    if records_install {
        capture_registry_subtree(root, key_path, MatchConfidence::Medium, entries, warnings);
        return;
    }
    if depth >= MAX_INSTALL_PATH_KEY_DEPTH {
        return;
    }
    for child_name in key.enum_keys().flatten() {
        capture_keys_recording_install_path(
            root,
            &format!(r"{key_path}\{child_name}"),
            install,
            depth + 1,
            entries,
            warnings,
        );
    }
}

/// True for a value named like an install path whose data is the install folder or a
/// path inside it.
fn value_records_install_path(value_name: &str, data: &str, install: &Path) -> bool {
    if !INSTALL_PATH_VALUE_NAMES.contains(&normalize_name_for_match(value_name).as_str()) {
        return false;
    }
    // Usually a plain folder, with spaces and no quotes, but sometimes a command line.
    let plain = Path::new(data.trim().trim_matches('"'));
    let command = split_command_executable_path(data);
    std::iter::once(plain)
        .chain(command.as_deref())
        .any(|path| path.is_absolute() && path_is_within(path, install))
}

/// Capture a machine-wide key and everything under it. A key that is not there yields
/// nothing and no warning.
pub(crate) fn capture_hklm_subtree(
    system: &SystemContext,
    key_path: &str,
    warnings: &mut Vec<Warning>,
) -> Vec<RegistryEntry> {
    let root = RegRoot::Hklm(system);
    let mut entries = Vec::new();
    if root.open(key_path, KEY_READ).is_ok() {
        capture_registry_subtree(
            &root,
            key_path,
            MatchConfidence::High,
            &mut entries,
            warnings,
        );
    }
    entries
}

fn capture_registry_subtree(
    root: &RegRoot,
    key_path: &str,
    confidence: MatchConfidence,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let hive = root.hive();
    let mut context = RegistryCaptureContext {
        root,
        hive,
        confidence,
        entries,
        warnings,
        keys_seen: 0,
        bytes_seen: 0,
    };
    capture_registry_subtree_inner(&mut context, key_path, 0);
}

fn capture_registry_subtree_inner(
    context: &mut RegistryCaptureContext<'_>,
    key_path: &str,
    depth: usize,
) {
    if depth > MAX_REGISTRY_CAPTURE_DEPTH || context.keys_seen >= MAX_REGISTRY_CAPTURE_KEYS {
        context.warnings.push(Warning::new(
            "registry_capture_truncated",
            WarningSeverity::Warning,
            "Registry subtree capture reached the configured safety limit.",
            Some(key_path.to_owned()),
        ));
        return;
    }
    context.keys_seen += 1;

    let key = match context.root.open(key_path, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            context.warnings.push(Warning::new(
                "registry_key_unreadable",
                WarningSeverity::Warning,
                format!("Could not read registry key: {error}"),
                Some(key_path.to_owned()),
            ));
            return;
        }
    };

    let mut value_count = 0_usize;
    for value in key.enum_values().flatten() {
        value_count += 1;
        let value_name = value.0;
        let raw_value = value.1;
        if raw_value.bytes.len() > MAX_REGISTRY_VALUE_BYTES {
            context.warnings.push(Warning::new(
                "registry_value_too_large",
                WarningSeverity::Warning,
                "Registry value exceeded the configured capture size limit.",
                Some(format!("{key_path}\\{value_name}")),
            ));
            continue;
        }
        context.bytes_seen += raw_value.bytes.len();
        if context.bytes_seen > MAX_REGISTRY_CAPTURE_BYTES {
            // The same stop as for too many keys, which says so at the next one it comes to.
            context.keys_seen = MAX_REGISTRY_CAPTURE_KEYS;
            context.warnings.push(Warning::new(
                "registry_capture_truncated",
                WarningSeverity::Warning,
                "Registry subtree capture reached the configured safety limit.",
                Some(key_path.to_owned()),
            ));
            return;
        }
        context.entries.push(entry_from_raw_value(
            context.hive.clone(),
            key_path.to_owned(),
            value_name,
            raw_value,
            context.confidence.clone(),
            context.hive == RegistryHive::LocalMachine,
        ));
    }
    if value_count == 0 {
        context.entries.push(RegistryEntry {
            hive: context.hive.clone(),
            key_path: key_path.to_owned(),
            value_name: None,
            value_data: None,
            value_kind: Some(RegistryValueKind::None),
            value_bytes_hex: None,
            confidence: context.confidence.clone(),
            requires_admin: context.hive == RegistryHive::LocalMachine,
        });
    }

    for child_name in key.enum_keys().flatten() {
        capture_registry_subtree_inner(context, &format!(r"{key_path}\{child_name}"), depth + 1);
    }
}

fn entry_from_raw_value(
    hive: RegistryHive,
    key_path: String,
    value_name: String,
    raw_value: RegValue<'_>,
    confidence: MatchConfidence,
    requires_admin: bool,
) -> RegistryEntry {
    RegistryEntry {
        hive,
        key_path,
        value_name: Some(display_value_name(&value_name)),
        value_data: Some(raw_value_to_display(&raw_value)),
        value_kind: Some(registry_value_kind(&raw_value.vtype)),
        value_bytes_hex: Some(hex_lower(raw_value.bytes.as_ref())),
        confidence,
        requires_admin,
    }
}

fn display_value_name(value_name: &str) -> String {
    if value_name.is_empty() {
        "(Default)".to_owned()
    } else {
        value_name.to_owned()
    }
}

fn raw_value_to_display(value: &RegValue<'_>) -> String {
    registry_value_text(value)
}

/// A registry value as text. winreg's `Display` fails on a value whose bytes do not fit
/// its type, such as an empty `REG_DWORD`, and `to_string` turns that into a panic, so a
/// single malformed value anywhere it looks would otherwise end the scan.
pub(crate) fn registry_value_text(value: &RegValue<'_>) -> String {
    let mut text = String::new();
    if write!(text, "{value}").is_err() {
        text.clear();
    }
    text
}

fn registry_value_kind(value_type: &RegType) -> RegistryValueKind {
    if value_type == &REG_NONE {
        RegistryValueKind::None
    } else if value_type == &REG_SZ {
        RegistryValueKind::String
    } else if value_type == &REG_EXPAND_SZ {
        RegistryValueKind::ExpandString
    } else if value_type == &REG_BINARY {
        RegistryValueKind::Binary
    } else if value_type == &REG_DWORD {
        RegistryValueKind::Dword
    } else if value_type == &REG_DWORD_BIG_ENDIAN {
        RegistryValueKind::DwordBigEndian
    } else if value_type == &REG_LINK {
        RegistryValueKind::Link
    } else if value_type == &REG_MULTI_SZ {
        RegistryValueKind::MultiString
    } else if value_type == &REG_RESOURCE_LIST {
        RegistryValueKind::ResourceList
    } else if value_type == &REG_FULL_RESOURCE_DESCRIPTOR {
        RegistryValueKind::FullResourceDescriptor
    } else if value_type == &REG_RESOURCE_REQUIREMENTS_LIST {
        RegistryValueKind::ResourceRequirementsList
    } else if value_type == &REG_QWORD {
        RegistryValueKind::Qword
    } else {
        RegistryValueKind::Binary
    }
}

fn registry_tree_matches_context(
    root: &RegRoot,
    key_path: &str,
    context: &RegistryScanContext,
    depth: usize,
) -> bool {
    if depth > MAX_REGISTRY_MATCH_SCAN_DEPTH {
        return false;
    }
    let Ok(key) = root.open(key_path, KEY_READ) else {
        return false;
    };
    for value in key.enum_values().flatten() {
        if value_matches_context(context, &raw_value_to_display(&value.1)) {
            return true;
        }
    }
    for child_name in key.enum_keys().flatten() {
        if registry_tree_matches_context(
            root,
            &format!(r"{key_path}\{child_name}"),
            context,
            depth + 1,
        ) {
            return true;
        }
    }
    false
}

fn source_registry_path_matches(
    context: &RegistryScanContext,
    hive: &RegistryHive,
    key_path: &str,
) -> bool {
    let Some(source) = context.source_registry_path.as_deref() else {
        return false;
    };
    let expected_prefix = match hive {
        RegistryHive::LocalMachine => "HKLM\\",
        RegistryHive::CurrentUser => "HKCU\\",
    };
    source
        .strip_prefix(expected_prefix)
        .is_some_and(|source_path| source_path.eq_ignore_ascii_case(key_path))
}

fn uninstall_key_matches_context(context: &RegistryScanContext, key: &RegKey) -> bool {
    for value_name in [
        "DisplayName",
        "Publisher",
        "DisplayVersion",
        "InstallLocation",
        "UninstallString",
        "DisplayIcon",
    ] {
        if let Ok(value) = key.get_raw_value(value_name)
            && value_matches_context(context, &raw_value_to_display(&value))
        {
            return true;
        }
    }
    false
}

/// True when a registry value points at the app: its executable or something inside its
/// install folder, as a whole path. A bare file name does not count: `rundll32.exe` in a
/// shell command says nothing about which app it serves.
fn value_matches_context(context: &RegistryScanContext, value: &str) -> bool {
    if text_mentions_path(value, &context.executable_path, true) {
        return true;
    }
    if context
        .install_location
        .as_ref()
        .is_some_and(|path| text_mentions_path(value, path, false))
    {
        return true;
    }
    split_command_executable_path(value).is_some_and(|path| command_path_matches(context, &path))
}

fn command_path_matches(context: &RegistryScanContext, candidate: &Path) -> bool {
    if !candidate.is_absolute() {
        return false;
    }
    path_is_within(candidate, &context.executable_path)
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_within(candidate, root))
}

fn classify_com_registrations(entries: &[RegistryEntry]) -> Vec<ComRegistrationEntry> {
    let mut registrations = Vec::new();
    for entry in entries {
        let lower_key = entry.key_path.to_ascii_lowercase();
        let Some(clsid) = extract_clsid(&entry.key_path) else {
            continue;
        };
        let Some(server_kind) = com_server_kind(&lower_key) else {
            continue;
        };
        if !is_default_value(entry.value_name.as_deref()) {
            continue;
        }
        let server_path = entry.value_data.as_deref().and_then(registry_server_path);
        let threading_model = entries
            .iter()
            .find(|candidate| {
                candidate.hive == entry.hive
                    && candidate.key_path.eq_ignore_ascii_case(&entry.key_path)
                    && candidate
                        .value_name
                        .as_deref()
                        .is_some_and(|value| value.eq_ignore_ascii_case("ThreadingModel"))
            })
            .and_then(|candidate| candidate.value_data.clone());

        registrations.push(ComRegistrationEntry {
            hive: entry.hive.clone(),
            clsid,
            key_path: entry.key_path.clone(),
            server_kind,
            server_path,
            threading_model,
            confidence: entry.confidence.clone(),
            requires_admin: entry.requires_admin,
        });
    }

    registrations.sort_by(|left, right| {
        format!("{:?}:{}:{:?}", left.hive, left.clsid, left.server_kind).cmp(&format!(
            "{:?}:{}:{:?}",
            right.hive, right.clsid, right.server_kind
        ))
    });
    registrations.dedup_by(|left, right| {
        left.hive == right.hive
            && left.clsid.eq_ignore_ascii_case(&right.clsid)
            && left.server_kind == right.server_kind
            && left.key_path.eq_ignore_ascii_case(&right.key_path)
    });
    registrations
}

fn classify_application_file_associations(entries: &[RegistryEntry]) -> Vec<FileAssociationEntry> {
    let mut associations = Vec::new();
    for entry in entries {
        let lower_key = entry.key_path.to_ascii_lowercase();
        if !lower_key.contains(r"\classes\applications\") || !lower_key.ends_with(r"\command") {
            continue;
        }
        if !is_default_value(entry.value_name.as_deref()) {
            continue;
        }

        let components = entry.key_path.split('\\').collect::<Vec<_>>();
        let Some(applications_index) = components
            .iter()
            .position(|component| component.eq_ignore_ascii_case("Applications"))
        else {
            continue;
        };
        let application_name = components
            .get(applications_index + 1)
            .map(|value| (*value).to_owned());
        let verb = components
            .iter()
            .position(|component| component.eq_ignore_ascii_case("shell"))
            .and_then(|index| components.get(index + 1))
            .map(|value| (*value).to_owned());

        associations.push(FileAssociationEntry {
            hive: entry.hive.clone(),
            extension: None,
            prog_id: None,
            application_name,
            verb,
            command: entry.value_data.clone(),
            confidence: entry.confidence.clone(),
            requires_admin: entry.requires_admin,
        });
    }
    associations
}

fn extract_clsid(key_path: &str) -> Option<String> {
    let mut saw_clsid = false;
    for component in key_path.split('\\') {
        if saw_clsid {
            return Some(component.to_owned());
        }
        saw_clsid = component.eq_ignore_ascii_case("CLSID");
    }
    None
}

fn com_server_kind(lower_key: &str) -> Option<ComServerKind> {
    if lower_key.ends_with(r"\inprocserver32") {
        Some(ComServerKind::InProcess)
    } else if lower_key.ends_with(r"\localserver32") {
        Some(ComServerKind::LocalServer)
    } else {
        None
    }
}

fn is_default_value(value_name: Option<&str>) -> bool {
    matches!(value_name, None | Some("(Default)"))
}

fn registry_server_path(value: &str) -> Option<PathBuf> {
    split_command_executable_path(value).or_else(|| {
        let trimmed = value.trim().trim_matches('"');
        (!trimmed.is_empty()).then(|| PathBuf::from(trimmed))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_app_for_one_account_can_only_say_things_about_that_accounts_registry() {
        let key = |root, key_path: &str| RecordedRegistryKey {
            root,
            key_path: key_path.to_owned(),
            value_name: None,
            wow64_32: false,
        };
        let kept = recorded_keys_of_one_account(vec![
            key(RecordedRegistryRoot::CurrentUser, r"Software\Vendor"),
            key(RecordedRegistryRoot::ClassesRoot, r"Vendor.File"),
            key(
                RecordedRegistryRoot::LocalMachine,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
            ),
        ]);
        let shown: Vec<(RecordedRegistryRoot, &str)> = kept
            .iter()
            .map(|key| (key.root, key.key_path.as_str()))
            .collect();
        assert_eq!(
            shown,
            [
                (RecordedRegistryRoot::CurrentUser, r"Software\Vendor"),
                (
                    RecordedRegistryRoot::CurrentUser,
                    r"Software\Classes\Vendor.File"
                ),
            ]
        );
    }

    #[test]
    fn registry_value_match_does_not_match_short_tokens_in_username() {
        let context = RegistryScanContext {
            executable_path: PathBuf::from(
                r"C:\Users\admin\AppData\Local\WinAppMigratorE2E\SumatraPDF\SumatraPDF.exe",
            ),
            install_location: Some(PathBuf::from(
                r"C:\Users\admin\AppData\Local\WinAppMigratorE2E\SumatraPDF",
            )),
            display_name: Some("SumatraPDF".to_owned()),
            publisher: None,
            source_registry_path: None,
            recorded_keys: Vec::new(),
        };

        assert!(!value_matches_context(&context, "ad 9.6.8"));
    }

    fn git_context() -> RegistryScanContext {
        RegistryScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\Git\git-bash.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Git")),
            display_name: Some("Git".to_owned()),
            publisher: Some("The Git Development Community".to_owned()),
            source_registry_path: None,
            recorded_keys: Vec::new(),
        }
    }

    #[test]
    fn values_match_the_install_folder_only_as_a_whole_path() {
        let context = git_context();
        assert!(value_matches_context(&context, r"C:\Program Files\Git"));
        assert!(value_matches_context(
            &context,
            r#""C:\Program Files\Git\bin\git.exe" "%1""#
        ));
        assert!(value_matches_context(
            &context,
            r"C:\Program Files\Git\git-bash.exe,0"
        ));
        assert!(!value_matches_context(
            &context,
            r"C:\Program Files\GitHub CLI\gh.exe"
        ));
        assert!(!value_matches_context(
            &context,
            r"C:\Program Files\Git Extensions\GitExtensions.exe"
        ));
    }

    #[test]
    fn a_malformed_registry_value_does_not_panic() {
        // A REG_DWORD needs four bytes. This shape is on real machines, under vendor keys.
        let value = RegValue {
            bytes: std::borrow::Cow::Borrowed(&[]),
            vtype: REG_DWORD,
        };
        assert_eq!(registry_value_text(&value), "");
    }

    #[test]
    fn a_bare_file_name_does_not_match() {
        // NVIDIA App's "main executable" used to be rundll32.exe, and this match then
        // captured every shell verb on the machine that runs through it, including
        // `Classes\htmlfile`.
        let context = RegistryScanContext {
            executable_path: PathBuf::from(
                r"C:\Program Files\NVIDIA Corporation\NVIDIA app\rundll32.exe",
            ),
            install_location: Some(PathBuf::from(
                r"C:\Program Files\NVIDIA Corporation\NVIDIA app",
            )),
            display_name: Some("NVIDIA App".to_owned()),
            publisher: Some("NVIDIA Corporation".to_owned()),
            source_registry_path: None,
            recorded_keys: Vec::new(),
        };
        assert!(!value_matches_context(
            &context,
            r#"rundll32.exe "C:\Program Files\Common Files\System\wab32.dll",Open"#
        ));
        assert!(!value_matches_context(&context, "rundll32.exe"));
    }

    #[test]
    fn software_keys_follow_the_same_name_rules_as_folders() {
        let names = context_product_names(&git_context());
        assert!(names.matches_top_level("Git", MatchScope::ProgramFolders));
        assert!(!names.matches_top_level("Logitech", MatchScope::ProgramFolders));
        assert!(!names.matches_top_level("GitHub", MatchScope::ProgramFolders));

        let seven_zip = context_product_names(&RegistryScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\7-Zip\7zFM.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\7-Zip")),
            display_name: Some("7-Zip 25.01 (x64)".to_owned()),
            publisher: Some("Igor Pavlov".to_owned()),
            source_registry_path: None,
            recorded_keys: Vec::new(),
        });
        assert!(
            seven_zip.matches_top_level("7-Zip", MatchScope::ProgramFolders),
            "the version in the display name must not hide the app's own key"
        );

        let photoshop = context_product_names(&RegistryScanContext {
            executable_path: PathBuf::from(r"C:\Program Files\Adobe\Photoshop\Photoshop.exe"),
            install_location: Some(PathBuf::from(r"C:\Program Files\Adobe\Photoshop")),
            display_name: Some("Adobe Photoshop 2024".to_owned()),
            publisher: Some("Adobe Inc.".to_owned()),
            source_registry_path: None,
            recorded_keys: Vec::new(),
        });
        assert!(
            !photoshop.matches_top_level("Adobe", MatchScope::ProgramFolders),
            "a publisher never selects a whole tree"
        );
        assert!(photoshop.matches_nested("Adobe", "Photoshop", MatchScope::ProgramFolders));
        assert!(!photoshop.matches_nested("Adobe", "Acrobat", MatchScope::ProgramFolders));
    }

    fn tree_children(tree: &[&str], key_path: &str) -> Vec<String> {
        let prefix = format!(r"{key_path}\");
        let mut children: Vec<String> = tree
            .iter()
            .filter_map(|path| path.strip_prefix(prefix.as_str()))
            .filter_map(|rest| rest.split('\\').next())
            .map(ToOwned::to_owned)
            .collect();
        children.sort();
        children.dedup();
        children
    }

    #[test]
    fn selects_a_products_keys_and_not_its_publishers_in_either_view() {
        let tree = [
            r"Software\7-Zip\Compression",
            r"Software\Adobe\Acrobat",
            r"Software\Adobe\Photoshop\Settings",
            r"Software\Classes\Photoshop",
            r"Software\Logitech\Unifying",
            r"Software\Microsoft\Photoshop",
            r"Software\WOW6432Node\Adobe\Photoshop",
            r"Software\WOW6432Node\Logitech\Unifying",
        ];
        let children_of = |path: &str| tree_children(&tree, path);
        let names = ProductNames::new(
            Some("Adobe Photoshop 2024"),
            Some("Adobe Inc."),
            &["Photoshop"],
            &[],
        );

        assert_eq!(
            product_keys_in_view(&names, SOFTWARE, &children_of),
            vec![r"Software\Adobe\Photoshop".to_owned()],
            "not the publisher's other products, and not another vendor's key of the same name"
        );
        assert_eq!(
            product_keys_in_view(&names, WOW6432_SOFTWARE, &children_of),
            vec![r"Software\WOW6432Node\Adobe\Photoshop".to_owned()]
        );
    }

    #[test]
    fn publishing_through_microsoft_qualifies_nothing_under_its_key() {
        let tree = [
            r"Software\Microsoft\Edge\Extensions",
            r"Software\Microsoft\PowerShell\1\ShellIds",
        ];
        let children_of = |path: &str| tree_children(&tree, path);

        let powershell = ProductNames::new(
            Some("PowerShell 7.6.9.0-x64"),
            Some("Microsoft Corporation"),
            &["pwsh"],
            &[],
        );
        assert!(
            product_keys_in_view(&powershell, SOFTWARE, &children_of).is_empty(),
            "`Software\\Microsoft\\PowerShell` is Windows' own"
        );

        let edge = ProductNames::new(
            Some("Microsoft Edge"),
            Some("Microsoft Corporation"),
            &["msedge"],
            &[],
        );
        assert_eq!(
            product_keys_in_view(&edge, SOFTWARE, &children_of),
            vec![r"Software\Microsoft\Edge".to_owned()]
        );
    }

    #[test]
    fn a_file_type_is_owned_through_its_open_verb_and_icon() {
        assert_eq!(
            prog_id_ownership_keys(r"Software\Classes\Demo.File", ""),
            vec![
                r"Software\Classes\Demo.File\shell\open\command".to_owned(),
                r"Software\Classes\Demo.File\DefaultIcon".to_owned(),
            ]
        );
        assert_eq!(
            prog_id_ownership_keys(r"Software\Classes\Demo.File", " edit "),
            vec![
                r"Software\Classes\Demo.File\shell\open\command".to_owned(),
                r"Software\Classes\Demo.File\shell\edit\command".to_owned(),
                r"Software\Classes\Demo.File\DefaultIcon".to_owned(),
            ]
        );
    }

    #[test]
    fn an_added_verb_is_captured_without_the_type_it_was_added_to() {
        assert_eq!(
            verb_key_of(r"Software\Classes\LibraryFolder\background\shell\pwsh\command"),
            r"Software\Classes\LibraryFolder\background\shell\pwsh"
        );
        assert_eq!(
            verb_key_of(r"Software\Classes\Demo\DefaultIcon"),
            r"Software\Classes\Demo\DefaultIcon"
        );
    }

    #[test]
    fn a_key_recording_the_install_folder_needs_a_value_named_for_that() {
        let install = Path::new(r"C:\Program Files\Git");
        assert!(value_records_install_path(
            "InstallPath",
            r"C:\Program Files\Git",
            install
        ));
        assert!(value_records_install_path(
            "InstallDir",
            r"C:\Program Files\Git\",
            install
        ));
        assert!(value_records_install_path(
            "Path",
            r#""C:\Program Files\Git\bin\git.exe" --flag"#,
            install
        ));
        assert!(
            !value_records_install_path("MSysGit", r"C:\Program Files\Git\bin", install),
            "another app's setting that points at Git does not make its key Git's"
        );
        assert!(!value_records_install_path(
            "InstallPath",
            r"C:\Program Files\GitHub CLI",
            install
        ));
        assert!(!value_records_install_path(
            "InstallPath",
            "git.exe",
            install
        ));
    }

    #[test]
    fn recorded_keys_map_to_the_hive_and_view_they_live_in() {
        let key = |root, key_path: &str, wow64_32| RecordedRegistryKey {
            root,
            key_path: key_path.to_owned(),
            value_name: None,
            wow64_32,
        };
        let machine = RegistryHive::LocalMachine;
        let user = RegistryHive::CurrentUser;

        assert_eq!(
            recorded_key_paths(
                &key(RecordedRegistryRoot::LocalMachine, r"Software\Foo", false),
                &machine
            ),
            vec![r"Software\Foo".to_owned()]
        );
        assert_eq!(
            recorded_key_paths(
                &key(RecordedRegistryRoot::LocalMachine, r"Software\Foo", true),
                &machine
            ),
            vec![
                r"Software\WOW6432Node\Foo".to_owned(),
                r"Software\Foo".to_owned()
            ]
        );
        assert!(
            recorded_key_paths(
                &key(RecordedRegistryRoot::LocalMachine, r"Software\Foo", false),
                &user
            )
            .is_empty()
        );
        assert_eq!(
            recorded_key_paths(
                &key(RecordedRegistryRoot::CurrentUser, r"Software\Foo", true),
                &user
            ),
            vec![r"Software\Foo".to_owned()]
        );
        assert!(
            recorded_key_paths(
                &key(RecordedRegistryRoot::CurrentUser, r"Software\Foo", true),
                &machine
            )
            .is_empty()
        );
        for hive in [&machine, &user] {
            assert_eq!(
                recorded_key_paths(
                    &key(
                        RecordedRegistryRoot::ClassesRoot,
                        r".foo\OpenWithProgids",
                        false
                    ),
                    hive
                ),
                vec![r"Software\Classes\.foo\OpenWithProgids".to_owned()]
            );
        }
    }

    #[test]
    fn a_record_cannot_claim_a_shared_key_whole() {
        for key in [
            r"Software",
            r"Software\Classes",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
            r"Software\RegisteredApplications",
        ] {
            assert!(is_shared_registry_key(key), "{key}");
        }
        for key in [
            r"Software\GitForWindows",
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Git_is1",
            r"Software\Classes\VSCode.ascx",
            r"Software\WOW6432Node\Foo",
        ] {
            assert!(!is_shared_registry_key(key), "{key}");
        }
    }
}
