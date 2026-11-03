//! What an account keeps for an app in its part of the registry, which goes by the User data option.
//!
//! An app's key under `HKEY_CURRENT_USER` is where it remembers things: 7-Zip the folders and archives
//! it last opened, PuTTY the hosts it has been to and who as. None of that is called what it is, so the
//! rules for credentials cannot see it, and an installer can be copied and shared. So with the option
//! off, all that goes of an account's is what makes the app work for it: what Windows itself looks in
//! for an app, and a value that says where the app's own folder is.
//!
//! It goes by where a value is and not by how it was found, so that whatever finds a value next is
//! held to this too. What is left out is said by the key of the app it was in, and no further down,
//! since a session or a host can be what a key is called.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::file_discovery::is_user_profile_path;
use crate::models::{
    LicenseArtifact, LicenseArtifactKind, RegistryEntry, RegistryHive, RegistryValueKind, Warning,
};
use crate::path_utils::path_is_within;
use crate::registry_scan::is_named_like_an_install_path;
use crate::restore_policy::{
    default_apps_capabilities_of, is_where_windows_looks_for_an_app, key_is_within, normalized_key,
};
use crate::secret_rules::LeftOut;

const WOW6432_NODE: &str = "WOW6432Node";
const SOFTWARE: &str = "Software";
/// Everything of Microsoft's is under this, whatever it is, so its name says nothing by itself.
const MICROSOFT: &str = "Microsoft";
/// What cannot be in a path, so a value holding one is holding more than a path. Each is what would
/// let a list or a command line that starts inside the app's folder count as being in it.
const NOT_IN_A_PATH: &[char] = &['"', '*', '?', '<', '>', '|', ';', '\t', '\r', '\n', '\0'];
/// How far below `Software` an app's own key is: `Software\7-Zip`, or `Software\Vendor\App`. A value
/// further down is in something the app keeps, whose name can be what it was keeping.
const MOST_KEYS_BELOW_SOFTWARE: usize = 2;
/// One further down still counts if it is called what an installer calls where it installed, as it
/// does for finding the app's key at all: `Software\Vendor\Products\App`.
const MOST_KEYS_BELOW_SOFTWARE_FOR_AN_INSTALL_PATH: usize = 3;
/// What says a key is an entry in Default apps' list of what an app can do, as it is by convention.
const CAPABILITIES_KEY: &str = "capabilities";
const REGISTRY_HIVE_PREFIX: &str = r"HKCU\";
/// What capturing the registry says when it stops short of a key or a value, which it names.
const CAPTURE_WARNINGS: [&str; 3] = [
    "registry_capture_truncated",
    "registry_key_unreadable",
    "registry_value_too_large",
];

/// Whether this is what an account keeps for an app, rather than how Windows finds the app.
pub(crate) fn is_account_setting(entry: &RegistryEntry) -> bool {
    entry.hive == RegistryHive::CurrentUser && !is_where_windows_looks_for_an_app(entry)
}

/// Whether a value says where something in one of the app's own folders is. All of it has to be that
/// one path: a path at the start of a command line, or a list that has one in it, can hold anything.
/// And it has to be called what such a value is called, in the app's own key, or what goes with it
/// is the name of what it is in, which can be a host, or the file a list of recent ones is by.
fn says_where_the_app_is(
    entry: &RegistryEntry,
    app_folders: &[PathBuf],
    expand: &dyn Fn(&str) -> String,
) -> bool {
    let needs_expanding = match entry.value_kind {
        Some(RegistryValueKind::String) => false,
        Some(RegistryValueKind::ExpandString) => true,
        _ => return false,
    };
    let Some(data) = entry.value_data.as_deref() else {
        return false;
    };
    let Some(name) = entry
        .value_name
        .as_deref()
        .filter(|name| is_a_plain_name(name))
    else {
        return false;
    };
    if !is_in_the_apps_own_key(&entry.key_path, name) {
        return false;
    }
    let path = data.trim();
    let path = path
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(path);
    let expanded = needs_expanding.then(|| expand(path));
    let path = expanded.as_deref().unwrap_or(path);
    is_one_plain_path(path)
        && app_folders
            .iter()
            .any(|folder| path_is_within(Path::new(path), folder))
}

fn is_in_the_apps_own_key(key_path: &str, value_name: &str) -> bool {
    let mut parts = key_path
        .split('\\')
        .filter(|part| !part.is_empty() && !part.eq_ignore_ascii_case(WOW6432_NODE));
    if !parts
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case(SOFTWARE))
    {
        return false;
    }
    let below = parts.count();
    below <= MOST_KEYS_BELOW_SOFTWARE
        || (below <= MOST_KEYS_BELOW_SOFTWARE_FOR_AN_INSTALL_PATH
            && is_named_like_an_install_path(value_name))
}

/// What a setting is called, rather than a path, a host, or anything else a list goes by.
fn is_a_plain_name(name: &str) -> bool {
    name.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, ' ' | '_' | '-' | '(' | ')')
    })
}

fn is_one_plain_path(text: &str) -> bool {
    let after_the_drive = text.get(2..).unwrap_or_default();
    !text.contains(NOT_IN_A_PATH)
        && !after_the_drive.contains(':')
        && Path::new(text).is_absolute()
        && !Path::new(text)
            .components()
            .any(|part| matches!(part, Component::ParentDir))
}

/// The key of the app that a value is in, as far down as says whose it is, and no further.
fn key_of_the_app(key_path: &str) -> String {
    let mut parts = key_path
        .split('\\')
        .filter(|part| !part.is_empty() && !part.eq_ignore_ascii_case(WOW6432_NODE));
    let first = parts.next().unwrap_or_default();
    let mut key = first.to_owned();
    if first.eq_ignore_ascii_case(SOFTWARE)
        && let Some(second) = parts.next()
    {
        key.push('\\');
        key.push_str(second);
        if second.eq_ignore_ascii_case(MICROSOFT)
            && let Some(third) = parts.next()
        {
            key.push('\\');
            key.push_str(third);
        }
    }
    key
}

/// With the User data option off: take what an account keeps for an app out of what was captured,
/// leaving what a value says of where the app is, and say how much went from where.
pub(crate) fn take_account_settings(
    entries: &mut Vec<RegistryEntry>,
    app_folders: &[PathBuf],
    expand: &dyn Fn(&str) -> String,
    left_out: &mut LeftOut,
) {
    // An entry in Default apps is no use without what it points at, and a restore refuses it. It can
    // point anywhere, and only what it points at by that name is what it is for.
    let capabilities: Vec<String> = entries
        .iter()
        .filter(|entry| entry.hive == RegistryHive::CurrentUser)
        .filter_map(default_apps_capabilities_of)
        .filter(|key| key.rsplit('\\').next() == Some(CAPABILITIES_KEY))
        .collect();
    let is_for_default_apps = |entry: &RegistryEntry| {
        let key = normalized_key(&entry.key_path);
        capabilities
            .iter()
            .any(|capabilities| key_is_within(&key, capabilities))
    };
    // By the key's own name, whatever case each value's path had it in.
    let mut gone: BTreeMap<String, (String, usize)> = BTreeMap::new();
    entries.retain(|entry| {
        let goes = is_account_setting(entry)
            && !says_where_the_app_is(entry, app_folders, expand)
            && !is_for_default_apps(entry);
        if goes {
            let key = format!("{REGISTRY_HIVE_PREFIX}{}", key_of_the_app(&entry.key_path));
            gone.entry(key.to_ascii_lowercase())
                .or_insert_with(|| (key, 0))
                .1 += 1;
        }
        !goes
    });
    for (key, count) in gone.into_values() {
        left_out.setting(key, count);
    }
}

/// With the User data option off: what capturing the registry had to say of a key goes no further
/// down than the app's, as what was left out does.
pub(crate) fn say_no_more_than_the_apps_key(warnings: &mut [Warning]) {
    for warning in warnings {
        if CAPTURE_WARNINGS.contains(&warning.code.as_str())
            && let Some(source) = warning.source.as_mut()
        {
            *source = key_of_the_app(source);
        }
    }
}

/// With both options off: nothing of a license is going, and all there is of one in an account's
/// keys is said no further down than the app's, and of one in an account's folders, by what it is
/// called. With the License option on, it has to say where one goes.
pub(crate) fn say_no_more_of_licenses_than_the_apps_key(
    artifacts: &mut [LicenseArtifact],
    left_out: &mut LeftOut,
) {
    let of_an_account = |location: &str| -> Option<String> {
        location
            .strip_prefix(REGISTRY_HIVE_PREFIX)
            .map(|key| format!("{REGISTRY_HIVE_PREFIX}{}", key_of_the_app(key)))
    };
    let name_in_an_account = |path: &Path| -> Option<PathBuf> {
        is_user_profile_path(path)
            .then(|| path.file_name().map(PathBuf::from))
            .flatten()
    };
    for artifact in artifacts.iter_mut() {
        if let Some(name) = artifact.path.as_deref().and_then(name_in_an_account) {
            artifact.path = Some(name);
        }
    }
    for artifact in artifacts
        .iter_mut()
        .filter(|artifact| artifact.kind == LicenseArtifactKind::RegistryValue)
    {
        if let Some(said) = artifact
            .registry_location
            .as_deref()
            .and_then(of_an_account)
        {
            artifact.registry_location = Some(said);
            "Registry value suggests licensing data".clone_into(&mut artifact.detail);
        }
    }
    for item in &mut left_out.licenses {
        if let Some(said) = of_an_account(&item.name) {
            item.name = said;
        } else if let Some(name) = name_in_an_account(Path::new(&item.name)) {
            item.name = name.display().to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        LeftOutItem, LicenseFramework, MatchConfidence, TransferableHint, WarningSeverity,
    };

    const INSTALL: &str = r"C:\Program Files\7-Zip";

    fn setting(key_path: &str, name: &str, kind: RegistryValueKind, data: &str) -> RegistryEntry {
        RegistryEntry {
            hive: RegistryHive::CurrentUser,
            key_path: key_path.to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some(data.to_owned()),
            value_kind: Some(kind),
            value_bytes_hex: None,
            confidence: MatchConfidence::Medium,
            requires_admin: false,
        }
    }

    fn text(key_path: &str, name: &str, data: &str) -> RegistryEntry {
        setting(key_path, name, RegistryValueKind::String, data)
    }

    fn after_taking(entries: Vec<RegistryEntry>) -> (Vec<RegistryEntry>, LeftOut) {
        let mut entries = entries;
        let mut left_out = LeftOut::default();
        take_account_settings(
            &mut entries,
            &[PathBuf::from(INSTALL)],
            &|text| text.replace("%ProgramFiles%", r"C:\Program Files"),
            &mut left_out,
        );
        (entries, left_out)
    }

    fn kept_names(entries: &[RegistryEntry]) -> Vec<&str> {
        entries
            .iter()
            .filter_map(|entry| entry.value_name.as_deref())
            .collect()
    }

    #[test]
    fn what_an_app_remembers_goes_and_where_it_is_stays() {
        let binary = setting(
            r"Software\7-Zip\Compression",
            "ArcHistory",
            RegistryValueKind::Binary,
            r"C:\Users\alice\secret.7z",
        );
        let (kept, left_out) = after_taking(vec![
            text(r"Software\7-Zip", "Path64", r"C:\Program Files\7-Zip\"),
            text(r"Software\7-Zip", "Path", r"C:\Program Files\7-Zip"),
            binary,
            text(r"Software\7-Zip\FM", "PanelPath0", r"D:\Taxes"),
            text(
                r"Software\SimonTatham\PuTTY\Sessions\work",
                "HostName",
                "10.0.0.5",
            ),
            text(
                r"Software\SimonTatham\PuTTY\Sessions\work",
                "UserName",
                "alice",
            ),
            setting(
                r"Software\SimonTatham\PuTTY\SshHostKeys",
                "ssh-ed25519@22:10.0.0.5",
                RegistryValueKind::String,
                "0x1234",
            ),
        ]);
        assert_eq!(kept_names(&kept), ["Path64", "Path"]);
        let names: Vec<(&str, &str)> = left_out
            .settings
            .iter()
            .map(|item| (item.name.as_str(), item.why.as_str()))
            .collect();
        assert_eq!(
            names,
            [
                (
                    r"HKCU\Software\7-Zip",
                    "2 values, which only go with the User data option"
                ),
                (
                    r"HKCU\Software\SimonTatham",
                    "3 values, which only go with the User data option"
                ),
            ]
        );
    }

    #[test]
    fn nothing_said_of_what_was_left_out_goes_below_the_apps_key() {
        let (_, left_out) = after_taking(vec![
            text(
                r"Software\WOW6432Node\SimonTatham\PuTTY\Sessions\the-secret-host",
                "HostName",
                "10.0.0.5",
            ),
            text(r"Software\Microsoft\Office\16.0\Word", "Recent", "x"),
            text(r"Control Panel\Whatever\the-secret-host", "Value", "x"),
        ]);
        let said = format!("{:?}", left_out.settings);
        assert!(!said.contains("the-secret-host"), "{said}");
        assert!(!said.contains("10.0.0.5"), "{said}");
        let names: Vec<&str> = left_out
            .settings
            .iter()
            .map(|item| item.name.as_str())
            .collect();
        assert_eq!(
            names,
            [
                r"HKCU\Control Panel",
                r"HKCU\Software\Microsoft\Office",
                r"HKCU\Software\SimonTatham",
            ]
        );
    }

    #[test]
    fn it_takes_the_whole_of_a_value_to_say_where_the_app_is() {
        for data in [
            // Starts in the app's folder, and is what it was told to open.
            r#""C:\Program Files\7-Zip\7zFM.exe" "C:\Users\alice\secret.7z""#,
            r"C:\Program Files\7-Zip\7zFM.exe C:\Users\alice\secret.7z",
            r"C:\Program Files\7-Zip;C:\Users\alice\Documents",
            r"C:\Program Files\7-Zip,C:\Users\alice\Documents",
            r"C:\Program Files\7-Zip\..\..\Users\alice\secret.7z",
            r"C:\Program Files\7-Zip Extra",
            r"C:\Program Files",
            "7-Zip",
            "",
        ] {
            let (kept, left_out) = after_taking(vec![text(r"Software\7-Zip", "Last", data)]);
            assert!(kept.is_empty(), "{data}");
            assert_eq!(left_out.settings.len(), 1, "{data}");
        }
        for data in [
            r"C:\Program Files\7-Zip",
            r"C:\Program Files\7-Zip\",
            r"c:\program files\7-zip\7zFM.exe",
            r#""C:\Program Files\7-Zip\7zFM.exe""#,
            r"C:\Program Files\7-Zip\Lang, Extra",
        ] {
            let (kept, _) = after_taking(vec![text(r"Software\7-Zip", "Where", data)]);
            assert_eq!(kept.len(), 1, "{data}");
        }
    }

    #[test]
    fn only_text_can_say_where_the_app_is() {
        for kind in [
            RegistryValueKind::Binary,
            RegistryValueKind::MultiString,
            RegistryValueKind::Dword,
            RegistryValueKind::None,
        ] {
            let (kept, _) = after_taking(vec![setting(
                r"Software\7-Zip",
                "Where",
                kind.clone(),
                INSTALL,
            )]);
            assert!(kept.is_empty(), "{kind:?}");
        }
        let (kept, _) = after_taking(vec![setting(
            r"Software\7-Zip",
            "Where",
            RegistryValueKind::ExpandString,
            INSTALL,
        )]);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn a_key_with_nothing_in_it_goes_too_since_its_name_can_be_what_it_remembers() {
        let (kept, left_out) = after_taking(vec![RegistryEntry {
            hive: RegistryHive::CurrentUser,
            key_path: r"Software\SimonTatham\PuTTY\Sessions\the-secret-host".to_owned(),
            value_name: None,
            value_data: None,
            value_kind: Some(RegistryValueKind::None),
            value_bytes_hex: None,
            confidence: MatchConfidence::Medium,
            requires_admin: false,
        }]);
        assert!(kept.is_empty());
        assert_eq!(left_out.settings.len(), 1);
    }

    #[test]
    fn what_windows_looks_in_for_an_app_stays_whatever_it_holds() {
        let wiring = [
            r"Software\Classes\.7z",
            r"Software\Classes\7-Zip.7z\DefaultIcon",
            r"Software\Classes\7-Zip.7z\shell\open\command",
            r"Software\Classes\CLSID\{23170F69-40C1-278A-1000-000100020000}\InprocServer32",
            r"Software\Classes\Applications\7zFM.exe\shell\open\command",
            r"Software\Classes\WOW6432Node\CLSID\{23170F69-40C1-278A-1000-000100020000}",
            r"Software\Microsoft\Windows\CurrentVersion\App Paths\7zFM.exe",
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\7-Zip",
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
            r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\7-Zip",
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{1234}",
            r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved",
            r"Software\Microsoft\Windows NT\CurrentVersion\Fonts",
            r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
            r"Software\RegisteredApplications",
            r"Software\Clients\Media\7-Zip\Capabilities",
            r"Software\ODBC\ODBCINST.INI\7-Zip Driver",
            r"Software\ODBC\ODBC.INI\7-Zip Source",
        ];
        for key in wiring {
            let (kept, left_out) = after_taking(vec![text(key, "Anything", "anything")]);
            assert_eq!(kept.len(), 1, "{key}");
            assert!(left_out.settings.is_empty(), "{key}");
        }
    }

    #[test]
    fn what_is_near_windows_own_places_and_is_not_one_goes() {
        for key in [
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs",
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\RunMRU",
            r"Software\Microsoft\Windows\CurrentVersion\Runner",
            r"Software\Microsoft\Windows\CurrentVersion\Applets\Regedit",
            r"Software\Classics",
            r"Software\Acme\Software\Classes\.7z",
            r"Software\Microsoft\Office\16.0",
            r"Environment",
        ] {
            let (kept, _) = after_taking(vec![text(key, "Anything", "anything")]);
            assert!(kept.is_empty(), "{key}");
        }
    }

    #[test]
    fn a_value_goes_with_the_name_of_what_it_is_in_so_it_has_to_be_the_apps_own_key() {
        const IN_THE_FOLDER: &str = r"C:\Program Files\7-Zip\key.ppk";
        for (key, name) in [
            (
                r"Software\SimonTatham\PuTTY\Sessions\the-secret-host",
                "PublicKeyFile",
            ),
            (r"Software\7-Zip\Recent\the-secret-host", "File"),
            (r"Software\7-Zip\A\B\the-secret-host", "InstallDir"),
            (r"Software\7-Zip", r"C:\Users\alice\secret.7z"),
            (r"Software\7-Zip", "ssh-ed25519@22:the-secret-host"),
            (r"Control Panel\7-Zip", "Path"),
        ] {
            let (kept, _) = after_taking(vec![text(key, name, IN_THE_FOLDER)]);
            assert!(kept.is_empty(), "{key} {name}");
        }
        for (key, name) in [
            (r"Software\7-Zip", "Path64"),
            (r"Software\7-Zip", "(Default)"),
            (r"Software\Igor Pavlov\7-Zip", "Install Dir"),
            (r"Software\WOW6432Node\Igor Pavlov\7-Zip", "install_dir"),
            // One further down, only as what an installer calls that.
            (r"Software\Igor Pavlov\Products\7-Zip", "InstallDir"),
        ] {
            let (kept, _) = after_taking(vec![text(key, name, IN_THE_FOLDER)]);
            assert_eq!(kept.len(), 1, "{key} {name}");
        }
    }

    #[test]
    fn a_value_to_be_expanded_goes_by_where_it_expands_to() {
        let expanding = |data: &str| {
            setting(
                r"Software\7-Zip",
                "Where",
                RegistryValueKind::ExpandString,
                data,
            )
        };
        let (kept, _) = after_taking(vec![expanding(r"%ProgramFiles%\7-Zip")]);
        assert_eq!(kept.len(), 1, "it says where the app is");
        let (kept, _) = after_taking(vec![expanding(r"%ProgramFiles%\Other")]);
        assert!(kept.is_empty());
        let (kept, _) = after_taking(vec![expanding(r"%Unknown%\7-Zip")]);
        assert!(kept.is_empty());
        // Only one of this kind means that. In any other, it is just what it says.
        let (kept, _) = after_taking(vec![text(
            r"Software\7-Zip",
            "Where",
            r"%ProgramFiles%\7-Zip",
        )]);
        assert!(kept.is_empty());
    }

    #[test]
    fn nothing_says_more_of_a_license_in_an_accounts_keys_than_whose_it_is() {
        let mut artifacts = vec![LicenseArtifact {
            kind: LicenseArtifactKind::RegistryValue,
            framework: LicenseFramework::RegistryBased,
            path: None,
            registry_location: Some(
                r"HKCU\Software\Acme\Sessions\the-secret-host\LicenseKey".to_owned(),
            ),
            detail: "Registry key suggests licensing data: the-secret-host".to_owned(),
            machine_bound: false,
            requires_admin: false,
            transferable: TransferableHint::Unknown,
            should_copy: false,
            package_relative_path: None,
            size_bytes: None,
            hash_sha256: None,
            store_files: Vec::new(),
        }];
        let mut left_out = LeftOut {
            licenses: vec![
                LeftOutItem {
                    name: r"HKCU\Software\Acme\Sessions\the-secret-host\LicenseKey".to_owned(),
                    why: "the License option is off".to_owned(),
                },
                LeftOutItem {
                    name: r"C:\Program Files\Acme\license.lic".to_owned(),
                    why: "the License option is off".to_owned(),
                },
                LeftOutItem {
                    name: r"C:\Users\alice\AppData\Roaming\Acme\license.lic".to_owned(),
                    why: "the License option is off".to_owned(),
                },
            ],
            ..LeftOut::default()
        };
        artifacts.push(LicenseArtifact {
            kind: LicenseArtifactKind::File,
            path: Some(PathBuf::from(
                r"C:\Users\alice\AppData\Roaming\Acme\license.lic",
            )),
            registry_location: None,
            detail: "License file detected: license.lic".to_owned(),
            ..artifacts[0].clone()
        });
        say_no_more_of_licenses_than_the_apps_key(&mut artifacts, &mut left_out);
        let said = format!("{artifacts:?} {:?}", left_out.licenses);
        assert!(!said.contains("the-secret-host"), "{said}");
        assert!(!said.contains("alice"), "{said}");
        assert_eq!(artifacts[1].path.as_deref(), Some(Path::new("license.lic")));
        assert_eq!(left_out.licenses[2].name, "license.lic");
        assert_eq!(
            artifacts[0].registry_location.as_deref(),
            Some(r"HKCU\Software\Acme")
        );
        assert_eq!(left_out.licenses[0].name, r"HKCU\Software\Acme");
        assert_eq!(
            left_out.licenses[1].name,
            r"C:\Program Files\Acme\license.lic"
        );
    }

    #[test]
    fn what_an_entry_in_default_apps_points_at_stays_with_it() {
        let (kept, left_out) = after_taking(vec![
            text(
                r"Software\RegisteredApplications",
                "7-Zip",
                r"Software\7-Zip\Capabilities",
            ),
            text(r"Software\7-Zip\Capabilities", "ApplicationName", "7-Zip"),
            text(
                r"Software\7-Zip\Capabilities\FileAssociations",
                ".7z",
                "7-Zip.7z",
            ),
            text(r"Software\7-Zip\FM", "PanelPath0", r"D:\Taxes"),
            text(r"Software\Other\Capabilities", "ApplicationName", "Other"),
        ]);
        assert_eq!(
            kept_names(&kept),
            ["7-Zip", "ApplicationName", ".7z"],
            "and nothing that is only called that"
        );
        assert_eq!(left_out.settings.len(), 2);
        // One pointing at what is kept for someone keeps none of it.
        let (kept, _) = after_taking(vec![
            text(
                r"Software\RegisteredApplications",
                "7-Zip",
                r"Software\7-Zip\FM",
            ),
            text(r"Software\7-Zip\FM", "PanelPath0", r"D:\Taxes"),
        ]);
        assert_eq!(kept_names(&kept), ["7-Zip"]);
    }

    #[test]
    fn what_capturing_says_of_a_key_goes_no_further_down_than_the_apps() {
        let warning = |code: &str, source: &str| {
            Warning::new(code, WarningSeverity::Warning, "x", Some(source.to_owned()))
        };
        let mut warnings = vec![
            warning(
                "registry_value_too_large",
                r"Software\SimonTatham\PuTTY\Sessions\the-secret-host\Blob",
            ),
            warning(
                "registry_key_unreadable",
                r"Software\SimonTatham\PuTTY\Sessions\the-secret-host",
            ),
            warning(
                "registry_capture_truncated",
                r"Software\WOW6432Node\Acme\Recent\the-secret-host",
            ),
            warning("file_hash_failed", r"C:\Program Files\Acme\a.exe"),
        ];
        say_no_more_than_the_apps_key(&mut warnings);
        let sources: Vec<&str> = warnings
            .iter()
            .filter_map(|warning| warning.source.as_deref())
            .collect();
        assert_eq!(
            sources,
            [
                r"Software\SimonTatham",
                r"Software\SimonTatham",
                r"Software\Acme",
                r"C:\Program Files\Acme\a.exe",
            ]
        );
    }

    #[test]
    fn what_is_machine_wide_is_not_an_accounts() {
        let mut entry = text(r"Software\7-Zip\FM", "PanelPath0", r"D:\Taxes");
        entry.hive = RegistryHive::LocalMachine;
        let (kept, left_out) = after_taking(vec![entry]);
        assert_eq!(kept.len(), 1);
        assert!(left_out.settings.is_empty());
    }

    #[test]
    fn a_setting_with_no_folder_to_point_at_goes() {
        let mut entries = vec![text(r"Software\7-Zip", "Path", INSTALL)];
        let mut left_out = LeftOut::default();
        take_account_settings(&mut entries, &[], &|text| text.to_owned(), &mut left_out);
        assert!(entries.is_empty());
    }
}
