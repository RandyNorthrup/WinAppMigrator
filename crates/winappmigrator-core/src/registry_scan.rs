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
use crate::integration_scan::{
    IntegrationInputs, Integrations, OwnedFiles, PointsAt, certificate_named_by, scan_integrations,
};
use crate::models::{
    CertificateEntry, ComRegistrationEntry, ComServerKind, FileAssociationEntry, FileEntry,
    FirewallRuleEntry, MatchConfidence, RegistryEntry, RegistryHive, RegistryValueKind, Warning,
    WarningSeverity,
};
use crate::path_utils::{
    is_drive_root, normalize_name_for_match, path_is_within, split_command_executable_path,
    text_mentions_path,
};
use crate::product_match::{MatchScope, ProductNames};
use crate::system_context::{SystemContext, UserHive};

/// A registry root to open logical key paths against. Live opens `predef`
/// hives; offline maps each logical path onto the mounted hive while callers
/// keep the logical path for capture/restore.
pub(crate) enum RegRoot<'a> {
    Hklm(&'a SystemContext),
    User(&'a UserHive),
}

impl RegRoot<'_> {
    pub(crate) fn open(&self, logical: &str, flags: u32) -> std::io::Result<RegKey> {
        match self {
            RegRoot::Hklm(system) => system.open_hklm(logical, flags),
            RegRoot::User(user) => user.open(logical, flags),
        }
    }

    pub(crate) fn hive(&self) -> RegistryHive {
        match self {
            RegRoot::Hklm(_) => RegistryHive::LocalMachine,
            RegRoot::User(_) => RegistryHive::CurrentUser,
        }
    }
}

const APP_PATHS: &str = r"Software\Microsoft\Windows\CurrentVersion\App Paths";
/// On Windows 10 and 11 this is the same key as the other, seen through the 32-bit view, and a
/// mounted hive has no such link to read. So only what the other does not have is its own.
const APP_PATHS_32_BIT: &str = r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths";
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
const HKLM_CLASSES: &str = r"Software\Classes";
const CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const SYSTEM_CERTIFICATES_KEY: &str = r"software\microsoft\systemcertificates";
const MAX_REGISTRY_CAPTURE_DEPTH: usize = 8;
const MAX_REGISTRY_CAPTURE_KEYS: usize = 512;
/// Each value is kept as text and hex too, so this is a good deal more in memory.
const MAX_REGISTRY_CAPTURE_BYTES: usize = 64 * 1024 * 1024;
/// Across everything captured for an app. A hive could otherwise have this many subtrees to take.
const MAX_REGISTRY_ENTRIES: usize = 100_000;
const MAX_REGISTRY_HEX_KEPT: usize = 512 * 1024 * 1024;
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
    r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
    r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce",
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
    /// Nothing machine-wide goes by the name or the record of such an app, since both are that
    /// account's to write. What names its program is still its own.
    pub installed_for_one_account: bool,
    /// Other folders that are the app's, because a person added them.
    pub also_the_apps: Vec<PathBuf>,
    /// The names of what a person added, which its keys go by as this app's go by its own.
    pub(crate) also_named: Vec<ProductNames>,
    /// What other apps go by, which nothing found for a part of this one is to be.
    pub(crate) not_named: Vec<ProductNames>,
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

impl RegistryScanContext {
    /// Where the app's own files are, as its PC calls them.
    pub(crate) fn folders_of_the_app(&self) -> Vec<PathBuf> {
        self.install_location
            .clone()
            .or_else(|| {
                self.executable_path
                    .parent()
                    .filter(|folder| !is_drive_root(folder))
                    .map(Path::to_path_buf)
            })
            .into_iter()
            .chain(self.also_the_apps.iter().cloned())
            .collect()
    }
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
    pub firewall_rules: Vec<FirewallRuleEntry>,
    pub certificates: Vec<CertificateEntry>,
    pub fonts_reported: Vec<String>,
    /// Fonts from the Windows folder that are the app's, which are files to copy as well.
    pub font_files: Vec<FileEntry>,
    pub warnings: Vec<Warning>,
}

pub(crate) fn scan_registry_artifacts(
    context: &RegistryScanContext,
    integrations: &IntegrationInputs<'_>,
    system: &SystemContext,
) -> RegistryScanArtifacts {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();

    let names = context_product_names(context);
    let hklm = RegRoot::Hklm(system);
    if !context.installed_for_one_account {
        scan_recorded_keys(context, &hklm, &mut entries, &mut warnings);
    }
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
    if !context.installed_for_one_account {
        scan_software_children(context, &names, &hklm, &mut entries, &mut warnings);
        for names in &context.also_named {
            scan_part_keys(context, names, &hklm, &mut entries, &mut warnings);
        }
    }
    scan_class_registrations(
        context,
        &hklm,
        CLASSES_APPLICATIONS,
        &mut entries,
        &mut warnings,
    );

    // Per-user hives: live yields the current user; offline yields every profile.
    for user in system.user_hives() {
        let hkcu = RegRoot::User(&user);
        scan_recorded_keys(context, &hkcu, &mut entries, &mut warnings);
        scan_app_paths(context, &hkcu, &mut entries, &mut warnings);
        scan_uninstall_entries(context, &hkcu, HKCU_UNINSTALL, &mut entries, &mut warnings);
        scan_run_keys(context, &hkcu, &mut entries, &mut warnings);
        scan_software_children(context, &names, &hkcu, &mut entries, &mut warnings);
        for names in &context.also_named {
            scan_part_keys(context, names, &hkcu, &mut entries, &mut warnings);
        }
        scan_class_registrations(
            context,
            &hkcu,
            CLASSES_APPLICATIONS,
            &mut entries,
            &mut warnings,
        );
    }

    let Integrations {
        entries: integration_entries,
        mut file_associations,
        firewall_rules,
        mut certificates,
        fonts_reported,
        font_files,
        warnings: integration_warnings,
    } = scan_integrations(integrations, system);
    entries.extend(integration_entries);
    warnings.extend(integration_warnings);
    without_other_apps_verbs(&mut entries, integrations.owned);
    leave_stores_of_certificates_out(&mut entries, &mut certificates);

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
        firewall_rules,
        certificates,
        fonts_reported,
        font_files,
        warnings,
    }
}

/// A store of certificates can come along by a name or an install record, and a package must never
/// add to a trust store. What it was is said, and it goes no further than that.
fn leave_stores_of_certificates_out(
    entries: &mut Vec<RegistryEntry>,
    certificates: &mut Vec<CertificateEntry>,
) {
    entries.retain(|entry| {
        let lower = entry.key_path.to_ascii_lowercase();
        let plain = lower
            .strip_prefix(r"software\wow6432node\")
            .map_or(lower.clone(), |rest| format!(r"software\{rest}"));
        let is_a_store = plain
            .strip_prefix(SYSTEM_CERTIFICATES_KEY)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'));
        if is_a_store
            && let Some(named) = certificate_named_by(entry)
            && !certificates.iter().any(|known| {
                known.thumbprint.eq_ignore_ascii_case(&named.thumbprint)
                    && known.store.eq_ignore_ascii_case(&named.store)
            })
        {
            certificates.push(named);
        }
        !is_a_store
    });
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

pub(crate) fn file_associations_from_prog_id(
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
    scan_app_paths_view(context, root, APP_PATHS, None, entries, warnings);
    scan_app_paths_view(
        context,
        root,
        APP_PATHS_32_BIT,
        Some(APP_PATHS),
        entries,
        warnings,
    );
}

fn scan_app_paths_view(
    context: &RegistryScanContext,
    root: &RegRoot,
    view: &str,
    other_view: Option<&str>,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    let key = match root.open(view, KEY_READ) {
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
        let is_in_the_other_view = other_view.is_some_and(|other| {
            let key = format!(r"{other}\{child_name}");
            entries
                .iter()
                .any(|entry| entry.key_path.eq_ignore_ascii_case(&key))
        });
        if matched && !is_in_the_other_view {
            capture_registry_subtree(
                root,
                &format!(r"{view}\{child_name}"),
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
pub(crate) fn context_product_names(context: &RegistryScanContext) -> ProductNames {
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
        if entries.len() >= MAX_REGISTRY_ENTRIES {
            break;
        }
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
    // A file type's lists and verbs have other apps' lines in them, however many are this app's, and
    // `integration_scan` goes through each by what it runs.
    let of_many_apps = plain
        .strip_prefix(r"software\classes\")
        .map(|rest| rest.strip_prefix(r"wow6432node\").unwrap_or(rest))
        .is_some_and(|rest| {
            let (top, below) = rest.split_once('\\').unwrap_or((rest, ""));
            let first_below = below.split('\\').next().unwrap_or_default();
            top.starts_with('.') || first_below == "shell" || first_below == "shellex"
        });
    of_many_apps || SHARED_REGISTRY_KEYS.contains(&plain.as_str())
}

/// Without a verb another app added to a type, which comes along when all of a type is the app's,
/// or a record names all of it. One that runs something of Windows' can be the type's own.
fn without_other_apps_verbs(entries: &mut Vec<RegistryEntry>, owned: &OwnedFiles) {
    const COMMAND_TAIL: &str = r"\command";
    let others: Vec<String> = entries
        .iter()
        .filter_map(|entry| {
            let key = entry.key_path.to_ascii_lowercase();
            let classes = key.contains(r"\classes\") && key.contains(r"\shell\");
            let is_default = entry
                .value_name
                .as_deref()
                .is_none_or(|name| name.is_empty() || name.eq_ignore_ascii_case("(Default)"));
            let verb = key
                .strip_suffix(COMMAND_TAIL)
                .filter(|_| classes && is_default)?;
            (owned.points_at(entry.value_data.as_deref().unwrap_or_default())
                == PointsAt::Elsewhere
                && entry
                    .value_data
                    .as_deref()
                    .is_some_and(|text| !text.trim().is_empty()))
            .then(|| verb.to_owned())
        })
        .collect();
    if others.is_empty() {
        return;
    }
    entries.retain(|entry| {
        let key = entry.key_path.to_ascii_lowercase();
        !others.iter().any(|verb| {
            key.strip_prefix(verb.as_str())
                .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
        })
    });
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
        let own = context
            .install_location
            .iter()
            .map(|install| (install, false));
        let added = context.also_the_apps.iter().map(|install| (install, true));
        for (install, was_added) in own.chain(added) {
            for child_name in children_of(software_path) {
                if !INSTALL_PATH_SEARCH_SKIPPED_KEYS
                    .contains(&child_name.to_ascii_lowercase().as_str())
                {
                    let first_new = entries.len();
                    capture_keys_recording_install_path(
                        root,
                        &format!(r"{software_path}\{child_name}"),
                        install,
                        1,
                        entries,
                        warnings,
                    );
                    // A key of another app's can say where something a person added is.
                    if was_added {
                        let mut found = entries.split_off(first_new);
                        found.retain(|entry| {
                            !is_named_for_another_app(&entry.key_path, &context.not_named)
                        });
                        entries.append(&mut found);
                    }
                }
            }
        }
    }
}

/// Capture what goes by the name of a part of the app that a person added. Where it is has been
/// looked in already, with the rest of what goes by where the app is.
fn scan_part_keys(
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
        for key_path in part_keys_in_view(names, software_path, &children_of) {
            if !is_shared_registry_key(&key_path)
                && !is_named_for_another_app(&key_path, &context.not_named)
            {
                capture_registry_subtree(
                    root,
                    &key_path,
                    MatchConfidence::Medium,
                    entries,
                    warnings,
                );
            }
        }
    }
}

/// [`product_keys_in_view`], of those that a part can have by what it calls itself, which can be
/// anything: `Windows NT`, or what is another vendor's.
pub(crate) fn part_keys_in_view(
    names: &ProductNames,
    software_path: &str,
    children_of: &impl Fn(&str) -> Vec<String>,
) -> Vec<String> {
    product_keys_in_view(names, software_path, children_of)
        .into_iter()
        .filter(|key| {
            let below = key[software_path.len()..].trim_start_matches('\\');
            match below.split_once('\\') {
                Some((parent, name)) => names.is_a_parts_own_name(Some(parent), name),
                None => names.is_a_parts_own_name(None, below),
            }
        })
        .collect()
}

/// Whether a key is what another app would take by its name, as its folders go by it.
pub(crate) fn is_named_for_another_app(key_path: &str, others: &[ProductNames]) -> bool {
    let parts: Vec<&str> = key_path
        .split('\\')
        .filter(|part| !part.eq_ignore_ascii_case(WOW6432_NODE))
        .collect();
    others.iter().any(|names| {
        parts.iter().enumerate().any(|(index, part)| {
            names.matches_top_level(part, MatchScope::ProgramFolders)
                || index.checked_sub(1).is_some_and(|before| {
                    names.matches_nested(parts[before], part, MatchScope::ProgramFolders)
                })
        })
    })
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

/// True for what an installer calls a value that says where it installed.
pub(crate) fn is_named_like_an_install_path(value_name: &str) -> bool {
    INSTALL_PATH_VALUE_NAMES.contains(&normalize_name_for_match(value_name).as_str())
}

/// True for a value named like an install path whose data is the install folder or a
/// path inside it.
fn value_records_install_path(value_name: &str, data: &str, install: &Path) -> bool {
    if !is_named_like_an_install_path(value_name) {
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

fn registry_capture_is_full(entries: &[RegistryEntry]) -> bool {
    entries.len() >= MAX_REGISTRY_ENTRIES
        || entries
            .iter()
            .map(|entry| entry.value_bytes_hex.as_ref().map_or(0, String::len))
            .sum::<usize>()
            >= MAX_REGISTRY_HEX_KEPT
}

pub(crate) fn capture_registry_subtree(
    root: &RegRoot,
    key_path: &str,
    confidence: MatchConfidence,
    entries: &mut Vec<RegistryEntry>,
    warnings: &mut Vec<Warning>,
) {
    if registry_capture_is_full(entries) {
        if warnings
            .last()
            .is_none_or(|warning| warning.code != "registry_capture_truncated")
        {
            warnings.push(Warning::new(
                "registry_capture_truncated",
                WarningSeverity::Warning,
                "Registry capture reached the configured safety limit.",
                Some(key_path.to_owned()),
            ));
        }
        return;
    }
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

pub(crate) fn entry_from_raw_value(
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

pub(crate) fn raw_value_to_display(value: &RegValue<'_>) -> String {
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
        .iter()
        .chain(&context.also_the_apps)
        .any(|path| text_mentions_path(value, path, false))
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
            .iter()
            .chain(&context.also_the_apps)
            .any(|root| path_is_within(candidate, root))
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
        let server_path = entry
            .value_data
            .as_deref()
            .and_then(|value| registry_server_path(value, &server_kind));
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

/// What a class runs. A DLL's value is a path and nothing else, usually with no quotes around it
/// whatever spaces it has, where a local server's is a command line.
fn registry_server_path(value: &str, kind: &ComServerKind) -> Option<PathBuf> {
    let whole = || {
        let trimmed = value.trim().trim_matches('"');
        (!trimmed.is_empty()).then(|| PathBuf::from(trimmed))
    };
    match kind {
        ComServerKind::InProcess => whole(),
        ComServerKind::LocalServer => split_command_executable_path(value).or_else(whole),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_class_goes_by_the_whole_of_a_dll_path_and_the_program_in_a_command() {
        let in_process = |value| registry_server_path(value, &ComServerKind::InProcess);
        let local = |value| registry_server_path(value, &ComServerKind::LocalServer);
        // As 7-Zip really records its shell extension.
        assert_eq!(
            in_process(r"C:\Program Files\7-Zip\7-zip.dll"),
            Some(PathBuf::from(r"C:\Program Files\7-Zip\7-zip.dll"))
        );
        assert_eq!(
            in_process(r#""C:\Program Files\Acme\a b.dll""#),
            Some(PathBuf::from(r"C:\Program Files\Acme\a b.dll"))
        );
        assert_eq!(
            in_process("mscoree.dll"),
            Some(PathBuf::from("mscoree.dll"))
        );
        assert_eq!(in_process("  "), None);
        assert_eq!(
            local(r#""C:\Program Files\Acme\server.exe" /automation"#),
            Some(PathBuf::from(r"C:\Program Files\Acme\server.exe"))
        );
        assert_eq!(
            local(r"C:\Program Files\Acme\server.exe /automation"),
            Some(PathBuf::from(r"C:\Program Files\Acme\server.exe"))
        );
    }

    #[test]
    fn what_a_part_calls_itself_takes_only_what_is_under_its_own_publisher() {
        let children = |path: &str| -> Vec<String> {
            let names: &[&str] = match path {
                "Software" => &[
                    "Microsoft",
                    "Fabrikam",
                    "Fabrikam Helper",
                    "Contoso",
                    "Policies",
                    "Windows NT",
                ],
                r"Software\Microsoft" => &["Windows NT", "Fabrikam Helper"],
                r"Software\Fabrikam" => &["Fabrikam Helper", "Reports"],
                r"Software\Contoso" => &["Fabrikam Helper", "Update Service"],
                _ => &[],
            };
            names.iter().map(|name| (*name).to_owned()).collect()
        };
        let part = ProductNames::new(Some("Fabrikam Helper"), Some("Fabrikam Inc."), &[], &[]);
        assert_eq!(
            part_keys_in_view(&part, "Software", &children),
            [
                r"Software\Fabrikam\Fabrikam Helper",
                r"Software\Fabrikam Helper",
            ]
        );
        for hostile in ["Windows NT", "Policies", "Update Service", "Microsoft"] {
            let names = ProductNames::new(Some(hostile), Some("Fabrikam Inc."), &[], &[]);
            assert_eq!(
                part_keys_in_view(&names, "Software", &children),
                Vec::<String>::new(),
                "{hostile}"
            );
        }
        let of_the_publisher = ProductNames::new(Some("Fabrikam"), Some("Fabrikam Inc."), &[], &[]);
        assert_eq!(
            part_keys_in_view(&of_the_publisher, "Software", &children),
            Vec::<String>::new(),
            "all of a publisher's key is not a part's"
        );
    }

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
            installed_for_one_account: false,
            also_the_apps: Vec::new(),
            also_named: Vec::new(),
            not_named: Vec::new(),
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
            installed_for_one_account: false,
            also_the_apps: Vec::new(),
            also_named: Vec::new(),
            not_named: Vec::new(),
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
            installed_for_one_account: false,
            also_the_apps: Vec::new(),
            also_named: Vec::new(),
            not_named: Vec::new(),
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
            installed_for_one_account: false,
            also_the_apps: Vec::new(),
            also_named: Vec::new(),
            not_named: Vec::new(),
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
            installed_for_one_account: false,
            also_the_apps: Vec::new(),
            also_named: Vec::new(),
            not_named: Vec::new(),
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
    fn a_store_of_certificates_is_named_and_goes_no_further() {
        let value = |key: &str| RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: key.to_owned(),
            value_name: Some("Blob".to_owned()),
            value_data: None,
            value_kind: Some(RegistryValueKind::Binary),
            value_bytes_hex: Some("00".to_owned()),
            confidence: MatchConfidence::High,
            requires_admin: true,
        };
        let mut entries = vec![
            value(r"SOFTWARE\Microsoft\SystemCertificates\ROOT\Certificates\ABC"),
            value(r"SOFTWARE\WOW6432Node\Microsoft\SystemCertificates\ROOT\Certificates\ABC"),
            value(r"SOFTWARE\Microsoft\SystemCertificates\CA"),
            value(r"SOFTWARE\Acme\SystemCertificates"),
        ];
        let mut certificates = Vec::new();
        leave_stores_of_certificates_out(&mut entries, &mut certificates);
        assert_eq!(entries.len(), 1, "{entries:?}");
        assert_eq!(entries[0].key_path, r"SOFTWARE\Acme\SystemCertificates");
        assert_eq!(certificates.len(), 1, "{certificates:?}");
        assert_eq!(certificates[0].thumbprint.to_ascii_lowercase(), "abc");
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
    fn a_verb_another_app_added_to_a_type_does_not_come_along() {
        let owned = OwnedFiles::new(
            Path::new(r"C:\App\a.exe"),
            Some(Path::new(r"C:\App")),
            Path::new(r"C:\Windows"),
            std::iter::empty(),
        );
        let value = |key: &str, name: Option<&str>, data: &str| RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: key.to_owned(),
            value_name: name.map(ToOwned::to_owned),
            value_data: Some(data.to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        };
        let mut entries = vec![
            value(
                r"Software\Classes\AudioCD\shell\play\command",
                None,
                r#""C:\App\a.exe" "%1""#,
            ),
            value(
                r"Software\Classes\AudioCD\shell\PlayWithVLC",
                None,
                "Play with VLC",
            ),
            value(
                r"Software\Classes\AudioCD\shell\PlayWithVLC\command",
                None,
                r#""C:\Other\vlc.exe" "%1""#,
            ),
            value(
                r"Software\Classes\AudioCD\shell\print\command",
                None,
                "rundll32.exe x",
            ),
            value(r"Software\App\shell\thing\command", None, r"C:\Other\o.exe"),
        ];
        without_other_apps_verbs(&mut entries, &owned);
        let kept: Vec<&str> = entries
            .iter()
            .map(|entry| entry.key_path.as_str())
            .collect();
        assert_eq!(
            kept,
            [
                r"Software\Classes\AudioCD\shell\play\command",
                r"Software\Classes\AudioCD\shell\print\command",
                r"Software\App\shell\thing\command",
            ]
        );
    }

    #[test]
    fn a_record_cannot_claim_a_shared_key_whole() {
        for key in [
            r"Software",
            r"Software\Classes",
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
            r"Software\RegisteredApplications",
            r"Software\Classes\.pdf",
            r"Software\Classes\.pdf\OpenWithProgids",
            r"Software\WOW6432Node\Classes\.pdf",
            r"Software\Classes\AudioCD\shell",
            r"Software\Classes\AudioCD\shell\Play",
            r"Software\Classes\*\shellex\ContextMenuHandlers",
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
