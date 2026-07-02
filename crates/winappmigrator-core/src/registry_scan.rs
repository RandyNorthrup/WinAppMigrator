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
    looks_like_executable, normalize_name_for_match, split_command_executable_path,
};
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
const HKCU_SOFTWARE: &str = r"Software";
const HKLM_SOFTWARE: &str = r"Software";
const HKCU_CLASSES: &str = r"Software\Classes";
const HKLM_CLASSES: &str = r"Software\Classes";
const HKCU_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKCU_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_CLASSES_APPLICATIONS: &str = r"Software\Classes\Applications";
const HKLM_CLASSES_CLSID: &str = r"Software\Classes\CLSID";
const HKLM_WOW6432_CLASSES_CLSID: &str = r"Software\WOW6432Node\Classes\CLSID";
const MAX_REGISTRY_CAPTURE_DEPTH: usize = 8;
const MAX_REGISTRY_CAPTURE_KEYS: usize = 512;
const MAX_REGISTRY_VALUE_BYTES: usize = 1024 * 1024;
const MAX_REGISTRY_MATCH_SCAN_DEPTH: usize = 4;
const RUN_KEYS: &[&str] = &[
    r"Software\Microsoft\Windows\CurrentVersion\Run",
    r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
];

#[derive(Debug, Clone)]
pub struct RegistryScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
    pub display_name: Option<String>,
    pub publisher: Option<String>,
    pub source_registry_path: Option<String>,
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

    let hklm = RegRoot::Hklm(system);
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
    scan_software_children(context, &hklm, &mut entries, &mut warnings);
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
        scan_app_paths(context, &hkcu, &mut entries, &mut warnings);
        scan_uninstall_entries(context, &hkcu, HKCU_UNINSTALL, &mut entries, &mut warnings);
        scan_run_keys(context, &hkcu, &mut entries, &mut warnings);
        scan_software_children(context, &hkcu, &mut entries, &mut warnings);
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
            && left.key_path == right.key_path
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
        if !registry_tree_matches_context(root, &prog_id_path, context, 0) {
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

fn scan_software_children(
    context: &RegistryScanContext,
    root: &RegRoot,
    entries: &mut Vec<RegistryEntry>,
    _warnings: &mut Vec<Warning>,
) {
    let software_path = match root.hive() {
        RegistryHive::LocalMachine => HKLM_SOFTWARE,
        RegistryHive::CurrentUser => HKCU_SOFTWARE,
    };
    let Ok(software) = root.open(software_path, KEY_READ) else {
        return;
    };
    let tokens = related_tokens(context);
    if tokens.is_empty() {
        return;
    }

    for child_name in software.enum_keys().flatten() {
        let normalized = normalize_name_for_match(&child_name);
        if tokens.iter().any(|token| normalized.contains(token)) {
            capture_registry_subtree(
                root,
                &format!(r"{software_path}\{child_name}"),
                MatchConfidence::Medium,
                entries,
                _warnings,
            );
        }
    }
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
    value.to_string()
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

fn value_matches_context(context: &RegistryScanContext, value: &str) -> bool {
    let value_lower = value.to_ascii_lowercase();
    if path_text_matches(&context.executable_path, &value_lower) {
        return true;
    }
    if context
        .install_location
        .as_ref()
        .is_some_and(|path| path_text_matches(path, &value_lower))
    {
        return true;
    }
    split_command_executable_path(value).is_some_and(|path| command_path_matches(context, &path))
}

fn path_text_matches(path: &Path, lower_text: &str) -> bool {
    lower_text.contains(&path.to_string_lossy().to_ascii_lowercase())
}

fn command_path_matches(context: &RegistryScanContext, candidate: &Path) -> bool {
    if path_eq(candidate, &context.executable_path) {
        return true;
    }
    if candidate.is_absolute()
        && context
            .install_location
            .as_ref()
            .is_some_and(|root| path_starts_with_ignore_case(candidate, root))
    {
        return true;
    }
    if !candidate.is_absolute() && looks_like_executable(candidate) {
        let candidate_name = candidate
            .file_name()
            .map(|value| value.to_string_lossy().to_ascii_lowercase());
        let executable_name = context
            .executable_path
            .file_name()
            .map(|value| value.to_string_lossy().to_ascii_lowercase());
        return candidate_name.is_some() && candidate_name == executable_name;
    }
    false
}

fn path_eq(left: &Path, right: &Path) -> bool {
    left.to_string_lossy()
        .eq_ignore_ascii_case(&right.to_string_lossy())
}

fn path_starts_with_ignore_case(path: &Path, root: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .starts_with(&root.to_string_lossy().to_ascii_lowercase())
}

fn related_tokens(context: &RegistryScanContext) -> Vec<String> {
    [
        context.display_name.as_deref(),
        context.publisher.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(normalize_name_for_match)
    .filter(|token| token.len() >= 3)
    .collect()
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
        };

        assert!(!value_matches_context(&context, "ad 9.6.8"));
    }
}
