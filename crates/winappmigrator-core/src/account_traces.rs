//! What a package says of whose PC it came from, with none of an account's data in it.
//!
//! An installer can be copied and shared. An account's key under `HKEY_CURRENT_USER` is only one of the
//! places that can say who it was: a scheduled task records who wrote it, a shortcut on someone's Desktop
//! says what they opened with it, and each of those has gone along by a route nothing was looking at. So
//! this goes by what each thing is, and is one function, which runs when a scan is asked for none of
//! an account's data, and again when a person has left all of that data out of one that was.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::account_settings::say_no_more_of_licenses_than_the_apps_key;
use crate::models::{
    DependencyEntry, EnvironmentScope, EnvironmentVariableEntry, FileEntry, InstallScope,
    LeftOutItem, PackageManifest, RegistryHive, ScanReport, ScheduledTaskEntry, ServiceEntry,
    ShortcutEntry, Warning, WarningSeverity,
};
use crate::path_utils::{
    comparable_path, current_user_sid, is_person_sid, path_is_within, signed_in_user,
};
use crate::restore_policy::ServiceAccount;
use crate::secret_rules::LeftOut;
use crate::shortcut_scan::{folders_of_the_accounts_shortcuts, shared_shortcut_folders};
use crate::system_context::SystemContext;
use crate::task_scan::{
    TaskThatNames, task_for_whoever_installs_it, without_what_names_an_account,
};

const USERS_FOLDER: &str = r"\users\";
const PUBLIC_NAME: &str = "public";
const START_MENU_PART: &str = r"\start menu\";
const DESKTOP_PART: &str = r"\desktop\";
/// Where a shortcut in an account's own folders goes, wherever that account's folders are. A restore
/// goes by the folder's name, and puts it in the Desktop or Start Menu of whoever is installing.
const ACCOUNTS_START_MENU: &str = r"%USERPROFILE%\Start Menu";
const ACCOUNTS_DESKTOP: &str = r"%USERPROFILE%\Desktop";
const SHORTCUTS_LEFT_OUT_NAME: &str = "Shortcuts in this account's own folders";
const VARIABLES_LEFT_OUT_NAME: &str = "Environment variables of this account's";
/// What a service that ran as someone is said to have run as. A restore goes by there being a name it
/// has no password for, and makes it `LocalService` whichever it is.
const AN_ACCOUNT_OF_ITS_OWN: &str = "an account of the PC it came from";
/// What stands for each in what a package says.
const AN_ACCOUNTS_FOLDER: &str = "<an account's folder>";
const AN_ACCOUNT: &str = "<an account>";
const A_PC: &str = "<a PC>";
const A_NETWORK_FOLDER: &str = "<a network folder>";
/// Too short to be said of anything but itself.
const SHORTEST_NAME_TO_LOOK_FOR: usize = 3;
/// What an account is often called that means something else in a sentence. These only count in a
/// path, where they are a folder of that name.
const NAMES_THAT_ARE_WORDS: [&str; 14] = [
    "admin",
    "administrator",
    "app",
    "default",
    "guest",
    "home",
    "owner",
    "pc",
    "public",
    "system",
    "test",
    "user",
    "users",
    "windows",
];
const NETWORK_PATH_START: &str = r"\\";
const NOT_A_NETWORK_PATH: [&str; 2] = [r"\\?\", r"\\.\"];
const MENTIONS_CODE: &str = "package_mentions_an_account";
const ENDS_A_PATH_ON_A_COMMAND_LINE: [char; 6] = ['"', '<', '>', '|', ';', ','];
/// The shortest run of hex that can be a value's bytes, rather than a number.
const SHORTEST_HEX_VALUE: usize = 6;

/// Who a scan was of and who is building, for what a package with none of an account's data has no
/// reason to say. Only ever in memory: nothing here goes in a package.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccountTraces {
    pub account_names: Vec<String>,
    pub profile_folders: Vec<String>,
    pub computer_names: Vec<String>,
    pub sids: Vec<String>,
}

impl AccountTraces {
    /// Whoever is at this PC and running this, and that is all a package of a drive can say of them.
    pub(crate) fn of_this_pc() -> Self {
        let mut traces = Self::default();
        if let Some(account) = signed_in_user().account() {
            traces.add_account(&account.name(), Some(&account.sid), Some(&account.profile));
        }
        if let Some(sid) = current_user_sid() {
            traces.add_sid(sid);
        }
        if let Ok(name) = std::env::var("USERNAME") {
            traces.add_account(&name, None, None);
        }
        if let Some(folder) = std::env::var_os("USERPROFILE") {
            traces.add_folder(Path::new(&folder));
        }
        if let Ok(computer) = std::env::var("COMPUTERNAME") {
            traces.add_computer(&computer);
        }
        traces
    }

    /// What a build looks for: who the scan was of, and whoever is building now.
    pub(crate) fn with_this_pc(&self) -> Self {
        let mut all = self.clone();
        let here = Self::of_this_pc();
        for name in &here.account_names {
            add_once(&mut all.account_names, name);
        }
        for folder in &here.profile_folders {
            add_once(&mut all.profile_folders, folder);
        }
        for computer in &here.computer_names {
            add_once(&mut all.computer_names, computer);
        }
        for sid in &here.sids {
            add_once(&mut all.sids, sid);
        }
        all
    }

    /// Text with nothing in it of who this is, for what is only ever read. A person's folder and a
    /// network folder go whoever they are, since what names them is not always known here.
    pub(crate) fn scrub(&self, text: &str) -> String {
        let mut said = text.to_owned();
        for folder in &self.profile_folders {
            said = replace_ignoring_case(&said, folder, AN_ACCOUNTS_FOLDER);
        }
        said = without_network_paths(&said);
        said = without_accounts_folders(&said);
        said = without_person_sids(&said);
        for sid in &self.sids {
            said = replace_ignoring_case(&said, sid, AN_ACCOUNT);
        }
        for name in &self.computer_names {
            said = replace_name(&said, name, A_PC);
        }
        for name in &self.account_names {
            said = replace_name(&said, name, AN_ACCOUNT);
        }
        said
    }

    /// Everything to look for, as what it is called on a screen.
    fn to_look_for(&self) -> Vec<(TraceKind, &str)> {
        let mut all = Vec::new();
        all.extend(
            self.profile_folders
                .iter()
                .map(|text| (TraceKind::Folder, text.as_str())),
        );
        all.extend(self.sids.iter().map(|text| (TraceKind::Sid, text.as_str())));
        all.extend(
            self.computer_names
                .iter()
                .map(|text| (TraceKind::Computer, text.as_str())),
        );
        all.extend(
            self.account_names
                .iter()
                .map(|text| (TraceKind::Account, text.as_str())),
        );
        all.retain(|(_, text)| text.chars().count() >= SHORTEST_NAME_TO_LOOK_FOR);
        all
    }

    /// Those, and whoever a scan was of, and everyone with a folder where it was.
    pub(crate) fn of_scan(system: &SystemContext) -> Self {
        let mut traces = Self::of_this_pc();
        for user in system.user_hives() {
            traces.add_account(
                &user.name,
                user.sid.as_deref(),
                Some(&system.logical_path(&user.profile_dir)),
            );
        }
        for folder in system.user_profiles() {
            let folder = system.logical_path(&folder);
            if let Some(name) = folder.file_name() {
                traces.add_account(&name.to_string_lossy(), None, Some(&folder));
            }
        }
        // Where its own settings say its Desktop and Start Menu are, which can be anywhere.
        let shared = shared_shortcut_folders(system);
        for folder in folders_of_the_accounts_shortcuts(system) {
            if !shared
                .iter()
                .any(|base| comparable_path(&folder) == comparable_path(base))
            {
                traces.add_folder(&system.logical_path(&folder));
            }
        }
        if let Some(computer) = system.source_identity().computer_name {
            traces.add_computer(&computer);
        }
        traces
    }

    fn add_account(&mut self, name: &str, sid: Option<&str>, folder: Option<&Path>) {
        add_once(&mut self.account_names, name);
        if let Some(sid) = sid {
            self.add_sid(sid);
        }
        if let Some(folder) = folder {
            self.add_folder(folder);
        }
    }

    fn add_sid(&mut self, sid: &str) {
        add_once(&mut self.sids, sid);
    }

    fn add_folder(&mut self, folder: &Path) {
        add_once(&mut self.profile_folders, &folder.to_string_lossy());
    }

    fn add_computer(&mut self, name: &str) {
        add_once(&mut self.computer_names, name);
    }
}

fn add_once(list: &mut Vec<String>, text: &str) {
    let text = text.trim();
    if !text.is_empty() && !list.iter().any(|known| known.eq_ignore_ascii_case(text)) {
        list.push(text.to_owned());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TraceKind {
    Account,
    Folder,
    Computer,
    Sid,
}

impl TraceKind {
    fn called(self) -> &'static str {
        match self {
            Self::Account => "an account's name",
            Self::Folder => "where an account's folder is",
            Self::Computer => "the PC's name",
            Self::Sid => "an account's id",
        }
    }
}

fn replace_ignoring_case(text: &str, what: &str, with: &str) -> String {
    if what.is_empty() {
        return text.to_owned();
    }
    let lower = text.to_ascii_lowercase();
    let what = what.to_ascii_lowercase();
    let mut output = String::with_capacity(text.len());
    let mut from = 0;
    while let Some(found) = lower[from..].find(&what) {
        let at = from + found;
        output.push_str(&text[from..at]);
        output.push_str(with);
        from = at + what.len();
    }
    output.push_str(&text[from..]);
    output
}

fn is_part_of_a_word(character: Option<char>) -> bool {
    character.is_some_and(|character| character.is_alphanumeric() || character == '_')
}

/// Where a name is in text as itself, not as part of something longer. One that is also a word only
/// counts as a folder of that name, or after an authority.
fn places_of_name(lower: &str, name: &str) -> Vec<usize> {
    let is_a_word = NAMES_THAT_ARE_WORDS.contains(&name);
    let mut places = Vec::new();
    let mut from = 0;
    while let Some(found) = lower[from..].find(name) {
        let at = from + found;
        from = at + name.len();
        let before = lower[..at].chars().next_back();
        let after = lower[at + name.len()..].chars().next();
        if is_part_of_a_word(before) || is_part_of_a_word(after) {
            continue;
        }
        if is_a_word && before != Some('\\') && after != Some('\\') {
            continue;
        }
        places.push(at);
    }
    places
}

fn replace_name(text: &str, name: &str, with: &str) -> String {
    if name.chars().count() < SHORTEST_NAME_TO_LOOK_FOR {
        return text.to_owned();
    }
    let lower = text.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();
    let mut output = String::with_capacity(text.len());
    let mut from = 0;
    for at in places_of_name(&lower, &name) {
        output.push_str(&text[from..at]);
        output.push_str(with);
        from = at + name.len();
    }
    output.push_str(&text[from..]);
    output
}

/// Where a path in a sentence ends, if nothing else says so.
fn end_of_a_path(text: &str) -> usize {
    text.find(['"', '\'', ',', ';', '\r', '\n', '<', '>', '|'])
        .unwrap_or(text.len())
}

fn without_network_paths(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(NETWORK_PATH_START) {
        let path = &rest[at..];
        output.push_str(&rest[..at]);
        if NOT_A_NETWORK_PATH
            .iter()
            .any(|start| path.starts_with(start))
        {
            output.push_str(NETWORK_PATH_START);
            rest = &path[NETWORK_PATH_START.len()..];
            continue;
        }
        output.push_str(A_NETWORK_FOLDER);
        rest = &path[end_of_a_path(path)..];
    }
    output.push_str(rest);
    output
}

/// `C:\Users\someone` of a path that says so, up to the folder after it. `Public` is nobody's.
fn without_accounts_folders(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    let mut output = String::with_capacity(text.len());
    let mut from = 0;
    let mut look_from = 0;
    while let Some(found) = lower[look_from..].find(USERS_FOLDER) {
        let at = look_from + found;
        let name_at = at + USERS_FOLDER.len();
        look_from = name_at;
        if !is_in_a_users_folder(&lower[at..]) {
            continue;
        }
        let rest = &text[name_at..];
        let name_length = rest
            .find('\\')
            .unwrap_or_else(|| end_of_a_path(rest))
            .min(end_of_a_path(rest));
        if name_length == 0 {
            continue;
        }
        let start = if at >= 2 && lower.as_bytes()[at - 1] == b':' {
            at - 2
        } else {
            at
        };
        if start < from {
            continue;
        }
        output.push_str(&text[from..start]);
        output.push_str(AN_ACCOUNTS_FOLDER);
        from = name_at + name_length;
        look_from = from;
    }
    output.push_str(&text[from..]);
    output
}

fn without_person_sids(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("S-1-") {
        let tail = &rest[at..];
        let length = tail
            .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
            .unwrap_or(tail.len());
        output.push_str(&rest[..at]);
        if is_person_sid(&tail[..length]) {
            output.push_str(AN_ACCOUNT);
        } else {
            output.push_str(&tail[..length]);
        }
        rest = &tail[length..];
    }
    output.push_str(rest);
    output
}

/// The text a value can hold under another form: hex, as a binary registry value is written, of
/// bytes that are text in either encoding. `None` for what is not that.
fn text_in_hex(text: &str) -> Option<String> {
    let digits: String = text
        .chars()
        .filter(|character| !matches!(character, ',' | ' ' | '\\' | '\r' | '\n'))
        .collect();
    if digits.len() < SHORTEST_HEX_VALUE
        || !digits.len().is_multiple_of(2)
        || !digits.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return None;
    }
    let bytes: Vec<u8> = (0..digits.len())
        .step_by(2)
        .filter_map(|at| u8::from_str_radix(&digits[at..at + 2], 16).ok())
        .collect();
    Some(text_of_bytes(&bytes))
}

/// Bytes read each way a value's can be, side by side.
fn text_of_bytes(bytes: &[u8]) -> String {
    let (pairs, _) = bytes.as_chunks::<2>();
    let wide: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
    format!(
        "{}\n{}",
        String::from_utf8_lossy(bytes),
        String::from_utf16_lossy(&wide)
    )
}

fn mentions(text: &str, kind: TraceKind, what: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let what = what.to_ascii_lowercase();
    match kind {
        TraceKind::Folder | TraceKind::Sid => lower.contains(&what),
        TraceKind::Account | TraceKind::Computer => !places_of_name(&lower, &what).is_empty(),
    }
}

fn text_mentions(text: &str, kind: TraceKind, what: &str) -> bool {
    mentions(text, kind, what)
        || text_in_hex(text).is_some_and(|decoded| mentions(&decoded, kind, what))
}

/// How many of the values under a part of a manifest mention this.
fn count_mentions(value: &Value, kind: TraceKind, what: &str) -> usize {
    match value {
        Value::String(text) => usize::from(text_mentions(text, kind, what)),
        Value::Array(items) if items.iter().all(Value::is_u64) && !items.is_empty() => {
            let bytes: Vec<u8> = items
                .iter()
                .filter_map(Value::as_u64)
                .filter_map(|number| u8::try_from(number).ok())
                .collect();
            usize::from(mentions(&text_of_bytes(&bytes), kind, what))
        }
        Value::Array(items) => items
            .iter()
            .map(|item| count_mentions(item, kind, what))
            .sum(),
        Value::Object(fields) => fields
            .iter()
            .map(|(name, item)| {
                usize::from(mentions(name, kind, what)) + count_mentions(item, kind, what)
            })
            .sum(),
        _ => 0,
    }
}

/// What a part of a manifest is called on a screen.
fn part_called(field: &str) -> String {
    match field {
        "file_inventory" | "empty_folders" => "the paths of the app's own files".to_owned(),
        "registry_keys_exported" | "license_registry_entries" => "registry values".to_owned(),
        "application_metadata" => "the app's entry in Settings".to_owned(),
        "main_executable_path" => "where its main program is".to_owned(),
        "shortcuts_detected" => "shortcuts".to_owned(),
        "scheduled_tasks_detected" => "scheduled tasks".to_owned(),
        "services_detected" => "services".to_owned(),
        "environment_variables_detected" => "environment variables".to_owned(),
        other => other.replace('_', " "),
    }
}

/// After a build with none of an account's data: anything in what it says that still says who this
/// was, as a warning for whoever is building, which names where and never what. Nothing is changed by
/// this, and it stops nothing, since what is left can be what makes the app work. It is here because
/// every way a name has got into a package so far was one way nobody had thought of, and the next one
/// should at least be seen. Files other than the manifest repeat it, so they are named if it does not.
pub(crate) fn what_a_package_still_says(
    manifest: &PackageManifest,
    written: &[(String, Vec<u8>)],
    traces: &AccountTraces,
) -> Vec<Warning> {
    let Ok(Value::Object(fields)) = serde_json::to_value(manifest) else {
        return Vec::new();
    };
    let mut said: BTreeMap<TraceKind, BTreeMap<String, usize>> = BTreeMap::new();
    for (kind, what) in traces.to_look_for() {
        let mut in_the_manifest = false;
        for (field, value) in &fields {
            let count = count_mentions(value, kind, what);
            if count > 0 {
                in_the_manifest = true;
                *said
                    .entry(kind)
                    .or_default()
                    .entry(part_called(field))
                    .or_default() += count;
            }
        }
        if in_the_manifest {
            continue;
        }
        for (name, bytes) in written {
            let decoded = text_of_bytes(bytes);
            if text_mentions(&decoded, kind, what) {
                *said
                    .entry(kind)
                    .or_default()
                    .entry(name.clone())
                    .or_default() += 1;
            }
        }
    }
    said.into_iter()
        .map(|(kind, places)| {
            let places: Vec<String> = places
                .into_iter()
                .map(|(place, count)| format!("{place} ({count})"))
                .collect();
            Warning::new(
                MENTIONS_CODE,
                WarningSeverity::Warning,
                format!(
                    "This package mentions {} in {}. It was built with none of an account's data, \
                     and anyone who has the installer can read this.",
                    kind.called(),
                    places.join(", ")
                ),
                None,
            )
        })
        .collect()
}

/// Whether a path is under `Users` and past the `Public` there, whose name has to be all of it.
fn is_in_a_users_folder(lower: &str) -> bool {
    lower.find(USERS_FOLDER).is_some_and(|at| {
        let name = &lower[at + USERS_FOLDER.len()..];
        let name = name.split('\\').next().unwrap_or_default();
        !name.is_empty() && name != PUBLIC_NAME
    })
}

/// Whether a path is in someone's own folders: under `Users`, or where this PC or drive says an
/// account's folder is, which can be anywhere.
fn is_in_an_accounts_folders(path: &Path, traces: &AccountTraces) -> bool {
    let lower = path
        .to_string_lossy()
        .replace('/', "\\")
        .to_ascii_lowercase();
    is_in_a_users_folder(&lower)
        || traces
            .profile_folders
            .iter()
            .any(|folder| path_is_within(path, Path::new(folder)))
}

/// Whether text names a place in someone's own folders that is not one of the app's.
fn names_a_place_of_an_account(
    text: &str,
    app_folders: &[PathBuf],
    traces: &AccountTraces,
) -> bool {
    let lower = text.replace('/', "\\").to_ascii_lowercase();
    let is_the_apps = |path: &str| {
        app_folders
            .iter()
            .any(|folder| path_is_within(Path::new(path), folder))
    };
    // A folder known to be someone's, which need not be anywhere called `Users`.
    for folder in &traces.profile_folders {
        let folder = folder.replace('/', "\\").to_ascii_lowercase();
        let mut from = 0;
        while let Some(found) = lower[from..].find(&folder) {
            let at = from + found;
            from = at + folder.len();
            // `D:\\Profiles\\AliceProject` is not Alice's.
            let goes_on = lower[from..]
                .chars()
                .next()
                .is_some_and(|next| !next.is_whitespace() && !"\\\"'<>|;,".contains(next));
            if goes_on {
                continue;
            }
            if !is_the_apps(path_on_a_command_line(&lower, at)) {
                return true;
            }
        }
    }
    let mut from = 0;
    while let Some(found) = lower[from..].find(USERS_FOLDER) {
        let at = from + found;
        from = at + USERS_FOLDER.len();
        if !is_in_a_users_folder(&lower[at..]) {
            continue;
        }
        // From the drive letter, if it has one, to where a command line would end the path.
        let start = lower[..at]
            .rfind(['"', '\'', ' ', '=', ',', ';'])
            .map_or(0, |at| at + 1);
        if !is_the_apps(path_on_a_command_line(&lower, start)) {
            return true;
        }
    }
    false
}

/// The path starting here. An apostrophe can be in a name, as `O'Neil` has, so it only ends one that
/// it also started.
fn path_on_a_command_line(text: &str, start: usize) -> &str {
    let path = text[start..]
        .split(ENDS_A_PATH_ON_A_COMMAND_LINE)
        .next()
        .unwrap_or_default();
    if text[..start].ends_with('\'') {
        return path.split('\'').next().unwrap_or_default();
    }
    path
}

/// What a shortcut in an account's own folders is called wherever it goes, with no account in it. What
/// is left past the folder it is in can still say who, as `D:\\Desktop\\Alice` does.
fn location_with_no_account(path: &Path, traces: &AccountTraces) -> Option<PathBuf> {
    let text = path.to_string_lossy().replace('/', "\\");
    let lower = text.to_ascii_lowercase();
    let (part, place) = match lower.find(START_MENU_PART) {
        Some(at) => (at + START_MENU_PART.len(), ACCOUNTS_START_MENU),
        None => (
            lower.find(DESKTOP_PART)? + DESKTOP_PART.len(),
            ACCOUNTS_DESKTOP,
        ),
    };
    let rest = &text[part..];
    (traces.scrub(rest) == rest).then(|| Path::new(place).join(rest))
}

/// With none of an account's data going: a shortcut in an account's own folders is what that account
/// made it, or was given, and what it opens and how is theirs. One that only opens the app can go, to
/// the Desktop or Start Menu of whoever installs it. An app installed for one account has its own
/// there, which can start it with arguments of its own and go on naming nothing else of theirs.
fn withhold_shortcuts(
    shortcuts: &mut Vec<ShortcutEntry>,
    app_folders: &[PathBuf],
    main_program: &Path,
    installed_for_one_account: bool,
    traces: &AccountTraces,
    left_out: &mut LeftOut,
) {
    let is_the_apps = |path: &Path| {
        path == main_program
            || app_folders
                .iter()
                .any(|folder| path_is_within(path, folder))
    };
    let mut withheld = 0_usize;
    shortcuts.retain_mut(|shortcut| {
        let arguments = shortcut.arguments.as_deref().unwrap_or_default();
        if names_a_place_of_an_account(arguments, app_folders, traces) {
            withheld += 1;
            return false;
        }
        if !is_in_an_accounts_folders(&shortcut.original_lnk_path, traces) {
            return true;
        }
        let its_own = is_the_apps(&shortcut.target_path)
            && (installed_for_one_account || arguments.trim().is_empty());
        if !its_own {
            withheld += 1;
            return false;
        }
        // Somewhere only this account's own settings say is its Desktop, which is nowhere on another
        // PC, and a restore would have nowhere to put it.
        let Some(location) = location_with_no_account(&shortcut.original_lnk_path, traces) else {
            withheld += 1;
            return false;
        };
        shortcut.original_lnk_path = location;
        let keep = |path: &PathBuf| !is_in_an_accounts_folders(path, traces) || is_the_apps(path);
        shortcut.working_directory = shortcut.working_directory.take().filter(keep);
        shortcut.icon_path = shortcut.icon_path.take().filter(keep);
        true
    });
    if withheld > 0 {
        let why = if withheld == 1 {
            "1 shortcut, which only goes with the User data option".to_owned()
        } else {
            format!("{withheld} shortcuts, which only go with the User data option")
        };
        left_out.settings.push(crate::models::LeftOutItem {
            name: SHORTCUTS_LEFT_OUT_NAME.to_owned(),
            why,
        });
    }
}

/// With none of an account's data going: a task keeps what runs the app, and goes without whatever
/// else it was told to do in someone's folders. One that was given a place of theirs to work on, or
/// is named for them, is theirs, and goes with the rest of what is.
fn withhold_tasks(
    tasks: &mut Vec<ScheduledTaskEntry>,
    app_folders: &[PathBuf],
    traces: &AccountTraces,
    left_out: &mut LeftOut,
) {
    let names_them = |text: &str| names_a_place_of_an_account(text, app_folders, traces);
    let is_named_for_them = |task: &ScheduledTaskEntry| {
        let name = format!("{}{}", task.task_path, task.task_name).to_ascii_lowercase();
        traces
            .sids
            .iter()
            .any(|sid| name.contains(&sid.to_ascii_lowercase()))
    };
    tasks.retain_mut(|task| {
        let item = format!("The scheduled task {}{}", task.task_path, task.task_name);
        let goes_whole =
            is_named_for_them(task) || task.arguments.as_deref().is_some_and(names_them);
        let cut = if goes_whole {
            TaskThatNames::GoesWhole
        } else {
            match task.definition_xml.as_deref() {
                Some(xml) => without_what_names_an_account(xml, &names_them),
                None => TaskThatNames::NothingOfTheirs,
            }
        };
        match cut {
            TaskThatNames::NothingOfTheirs => {}
            TaskThatNames::GoesWhole => {
                left_out.setting_of(
                    item,
                    "It works on a place in an account's folders, or is named for one, and only \
                     goes with the User data option",
                );
                return false;
            }
            TaskThatNames::LostSome { xml, actions } => {
                task.definition_xml = Some(xml);
                if actions > 0 {
                    left_out.setting_of(
                        item,
                        "What it also runs from an account's folders only goes with the User data \
                         option",
                    );
                }
            }
        }
        if task
            .working_directory
            .as_deref()
            .is_some_and(|folder| names_them(&folder.to_string_lossy()))
        {
            task.working_directory = None;
        }
        true
    });
}

/// What of a scan's settings can name an account, and what tells what is theirs from the app's.
pub(crate) struct Withholding<'a> {
    pub shortcuts: &'a mut Vec<ShortcutEntry>,
    pub tasks: &'a mut Vec<ScheduledTaskEntry>,
    pub app_folders: &'a [PathBuf],
    pub main_program: &'a Path,
    pub installed_for_one_account: bool,
    pub traces: &'a AccountTraces,
}

/// With the User data option off: what of an app's settings is an account's, by what each is.
pub(crate) fn withhold_settings(from: &mut Withholding<'_>, left_out: &mut LeftOut) {
    withhold_shortcuts(
        from.shortcuts,
        from.app_folders,
        from.main_program,
        from.installed_for_one_account,
        from.traces,
        left_out,
    );
    withhold_tasks(from.tasks, from.app_folders, from.traces, left_out);
}

/// A variable of an account's goes if all it says is where the app is, and a list of folders keeps
/// the app's. Anything else it holds is that account's, as anything naming one of their folders is.
fn withhold_variables(
    variables: &mut Vec<EnvironmentVariableEntry>,
    app_folders: &[PathBuf],
    expand: &dyn Fn(&str) -> String,
    traces: &AccountTraces,
    left_out: &mut LeftOut,
) {
    let is_the_apps = |entry: &str| {
        let expanded = expand(entry.trim().trim_matches('"'));
        Path::new(&expanded).is_absolute()
            && app_folders
                .iter()
                .any(|folder| path_is_within(Path::new(&expanded), folder))
    };
    let mut withheld = 0_usize;
    variables.retain_mut(|variable| {
        let entries: Vec<&str> = variable
            .value
            .split(';')
            .filter(|entry| !entry.trim().is_empty())
            .collect();
        let of_an_account = variable.scope == EnvironmentScope::User;
        let kept: Vec<&str> = entries
            .iter()
            .copied()
            .filter(|entry| is_the_apps(entry))
            .collect();
        if of_an_account && kept.is_empty() {
            withheld += 1;
            return false;
        }
        if of_an_account
            || (entries.len() > 1
                && names_a_place_of_an_account(&variable.value, app_folders, traces))
        {
            if kept.is_empty() {
                withheld += 1;
                return false;
            }
            variable.value = kept.join(";");
            return true;
        }
        if names_a_place_of_an_account(&variable.value, app_folders, traces) {
            withheld += 1;
            return false;
        }
        true
    });
    if withheld > 0 {
        let why = if withheld == 1 {
            "1 variable, which only goes with the User data option".to_owned()
        } else {
            format!("{withheld} variables, which only go with the User data option")
        };
        left_out.settings.push(LeftOutItem {
            name: VARIABLES_LEFT_OUT_NAME.to_owned(),
            why,
        });
    }
}

/// Who a service ran as, if that is one of the accounts every Windows has. A restore goes by there
/// having been some other one, whichever it was.
fn withhold_service_account(account: &mut Option<String>, service_name: &str) {
    if matches!(
        ServiceAccount::from_recorded(account.as_deref(), service_name),
        ServiceAccount::Custom(_)
    ) {
        *account = Some(AN_ACCOUNT_OF_ITS_OWN.to_owned());
    }
}

fn withhold_services(services: &mut [ServiceEntry]) {
    for service in services {
        withhold_service_account(&mut service.account_name, &service.service_name);
    }
}

/// A DLL found in someone's folders by what their `Path` said is known by its name.
fn withhold_dependency_paths(
    dependencies: &mut [DependencyEntry],
    app_folders: &[PathBuf],
    traces: &AccountTraces,
) {
    for dependency in dependencies {
        let names_them = dependency.resolved_path.as_deref().is_some_and(|path| {
            names_a_place_of_an_account(&path.to_string_lossy(), app_folders, traces)
        });
        if names_them {
            dependency.resolved_path = Some(PathBuf::from(&dependency.name));
        }
    }
}

/// What is in someone's folders and not going says nothing here, whatever kind of file it is.
fn withhold_files_staying(
    files: &mut Vec<FileEntry>,
    app_folders: &[PathBuf],
    traces: &AccountTraces,
) {
    files.retain(|file| {
        file.should_copy
            || !names_a_place_of_an_account(
                &file.original_path.to_string_lossy(),
                app_folders,
                traces,
            )
    });
}

/// The folders of a report that are the app's own.
fn folders_of_the_app_in(report: &ScanReport) -> Vec<PathBuf> {
    report
        .install_location
        .clone()
        .or_else(|| report.main_executable.path.parent().map(Path::to_path_buf))
        .into_iter()
        .chain(
            report
                .near_matches
                .iter()
                .filter(|near| near.picked)
                .filter_map(|near| near.path.clone()),
        )
        .collect()
}

/// Everything in a scan's settings that goes only with the User data option, by what it is. Nothing
/// that has gone once is said twice, so this can run again on what it has run on.
pub(crate) fn withhold_what_is_an_accounts(report: &mut ScanReport, include_license_data: bool) {
    for task in &mut report.scheduled_tasks {
        let generalised = task
            .definition_xml
            .as_deref()
            .and_then(task_for_whoever_installs_it);
        // What cannot be made to say nobody goes as a definition no one can read, which a restore
        // refuses, rather than as it was.
        match generalised {
            Some((xml, runs_as_a_person)) => {
                task.definition_xml = Some(xml);
                if runs_as_a_person {
                    task.account = Some(crate::restore_policy::WHOEVER_INSTALLS_IT.to_owned());
                }
            }
            None => {
                task.definition_xml = None;
                task.account = None;
            }
        }
    }
    let folders = folders_of_the_app_in(report);
    let installed_for_one_account = report
        .source_app
        .as_ref()
        .is_some_and(|app| app.install_scope == InstallScope::PerUser);
    let main_program = report.main_executable.path.clone();
    let traces = report.account_traces.clone();
    let mut left_out = LeftOut::default();
    withhold_settings(
        &mut Withholding {
            shortcuts: &mut report.shortcuts,
            tasks: &mut report.scheduled_tasks,
            app_folders: &folders,
            main_program: &main_program,
            installed_for_one_account,
            traces: &traces,
        },
        &mut left_out,
    );
    withhold_variables(
        &mut report.environment_variables,
        &folders,
        &crate::path_utils::expand_env_vars,
        &traces,
        &mut left_out,
    );
    withhold_services(&mut report.services);
    for driver in &mut report.drivers {
        withhold_service_account(&mut driver.account_name, &driver.service_name);
    }
    withhold_dependency_paths(&mut report.dependencies, &folders, &traces);
    withhold_files_staying(&mut report.file_inventory, &folders, &traces);
    report
        .certificates
        .retain(|certificate| certificate.hive != RegistryHive::CurrentUser);
    if !include_license_data {
        left_out.licenses.append(&mut report.licenses_left_out);
        say_no_more_of_licenses_than_the_apps_key(
            &mut report.license_artifacts,
            &mut left_out,
            &|path| is_in_an_accounts_folders(path, &traces),
        );
        report.licenses_left_out.append(&mut left_out.licenses);
    }
    report.settings_left_out.append(&mut left_out.settings);
}

/// What is only ever read of a package, with no one in it. What makes something work is left to
/// [`what_a_package_still_says`] to point out, since changing it would stop it working.
pub(crate) fn say_nothing_of_who_it_was(manifest: &mut PackageManifest, traces: &AccountTraces) {
    scrub_warnings(&mut manifest.warnings, traces);
    for item in manifest
        .secrets_left_out
        .iter_mut()
        .chain(&mut manifest.licenses_left_out)
        .chain(&mut manifest.settings_left_out)
        .chain(&mut manifest.left_out_by_choice)
    {
        item.name = traces.scrub(&item.name);
        item.why = traces.scrub(&item.why);
    }
    for text in manifest
        .added_by_choice
        .iter_mut()
        .chain(&mut manifest.items_intentionally_excluded)
    {
        *text = traces.scrub(text);
    }
}

fn scrub_warnings(warnings: &mut [Warning], traces: &AccountTraces) {
    for warning in warnings {
        warning.message = traces.scrub(&warning.message);
        warning.source = warning.source.as_deref().map(|source| traces.scrub(source));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const APP: &str = r"C:\Program Files\Acme";
    const MAIN: &str = r"C:\Program Files\Acme\acme.exe";

    fn shortcut(location: &str, target: &str, arguments: Option<&str>) -> ShortcutEntry {
        ShortcutEntry {
            original_lnk_path: PathBuf::from(location),
            target_path: PathBuf::from(target),
            arguments: arguments.map(str::to_owned),
            working_directory: None,
            icon_path: None,
            target_is_selected_app: true,
        }
    }

    fn withheld(
        shortcuts: &mut Vec<ShortcutEntry>,
        folders: &[PathBuf],
        one_account: bool,
    ) -> Vec<String> {
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            shortcuts,
            folders,
            Path::new(MAIN),
            one_account,
            &AccountTraces::default(),
            &mut left_out,
        );
        left_out.settings.into_iter().map(|item| item.why).collect()
    }

    #[test]
    fn a_shortcut_on_someones_desktop_with_arguments_is_theirs() {
        let folders = [PathBuf::from(APP)];
        let mut shortcuts = vec![
            shortcut(
                r"C:\Users\randy\Desktop\Bank.lnk",
                MAIN,
                Some(r"-load bank-prod"),
            ),
            shortcut(r"C:\Users\randy\Desktop\Acme.lnk", MAIN, None),
            shortcut(
                r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Acme.lnk",
                MAIN,
                Some("--safe"),
            ),
            shortcut(
                r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Report.lnk",
                MAIN,
                Some(r"C:\Users\randy\Documents\private.txt"),
            ),
        ];
        let said = withheld(&mut shortcuts, &folders, false);
        assert_eq!(
            said,
            ["2 shortcuts, which only go with the User data option"]
        );
        assert_eq!(shortcuts.len(), 2);
        assert_eq!(
            shortcuts[0].original_lnk_path,
            Path::new(r"%USERPROFILE%\Desktop\Acme.lnk"),
            "with no account in it"
        );
        assert!(
            shortcuts[1]
                .original_lnk_path
                .starts_with(r"C:\ProgramData"),
            "everyone's stays as it was"
        );
    }

    #[test]
    fn a_shortcut_of_theirs_to_something_else_goes_and_what_it_points_at_goes_with_it() {
        let folders = [PathBuf::from(APP)];
        let mut other = shortcut(
            r"C:\Users\randy\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Sub\Acme.lnk",
            MAIN,
            None,
        );
        other.working_directory = Some(PathBuf::from(r"C:\Users\randy\Documents"));
        other.icon_path = Some(PathBuf::from(r"C:\Users\randy\icons\a.ico"));
        let mut shortcuts = vec![
            other,
            shortcut(
                r"C:\Users\randy\Desktop\Tool.lnk",
                r"C:\Users\randy\tools\tool.exe",
                None,
            ),
        ];
        withheld(&mut shortcuts, &folders, false);
        assert_eq!(shortcuts.len(), 1);
        assert_eq!(
            shortcuts[0].original_lnk_path,
            Path::new(r"%USERPROFILE%\Start Menu\Programs\Sub\Acme.lnk")
        );
        assert_eq!(shortcuts[0].working_directory, None);
        assert_eq!(shortcuts[0].icon_path, None);
    }

    #[test]
    fn an_app_of_one_accounts_own_shortcuts_keep_working() {
        let app = r"C:\Users\randy\AppData\Local\Discord";
        let main = format!(r"{app}\Update.exe");
        let folders = [PathBuf::from(app)];
        let mut shortcuts = vec![
            shortcut(
                r"C:\Users\randy\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Discord.lnk",
                &main,
                Some("--processStart Discord.exe"),
            ),
            shortcut(
                r"C:\Users\randy\Desktop\Discord.lnk",
                &main,
                Some(&format!(r#"--processStart "{app}\Discord.exe""#)),
            ),
            shortcut(
                r"C:\Users\randy\Desktop\Other.lnk",
                &main,
                Some(r"--open C:\Users\randy\Documents\a.txt"),
            ),
        ];
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            &mut shortcuts,
            &folders,
            Path::new(&main),
            true,
            &AccountTraces::default(),
            &mut left_out,
        );
        assert_eq!(shortcuts.len(), 2, "{shortcuts:?}");
        assert_eq!(
            shortcuts[0].original_lnk_path,
            Path::new(r"%USERPROFILE%\Start Menu\Programs\Discord.lnk")
        );
        assert_eq!(left_out.settings.len(), 1);
    }

    #[test]
    fn a_shortcut_somewhere_only_its_account_calls_its_desktop_goes_with_that_account() {
        let folders = [PathBuf::from(APP)];
        let traces = AccountTraces {
            profile_folders: vec![r"D:\PrivateDesk".to_owned()],
            ..AccountTraces::default()
        };
        let mut shortcuts = vec![shortcut(r"D:\PrivateDesk\Acme.lnk", MAIN, None)];
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            &mut shortcuts,
            &folders,
            Path::new(MAIN),
            false,
            &traces,
            &mut left_out,
        );
        assert!(shortcuts.is_empty(), "{shortcuts:?}");
        assert_eq!(left_out.settings.len(), 1);
    }

    #[test]
    fn what_is_left_of_where_a_shortcut_is_can_say_who_too() {
        let folders = [PathBuf::from(APP)];
        let traces = AccountTraces {
            account_names: vec!["Alice".to_owned()],
            profile_folders: vec![r"D:\Desktop\Alice".to_owned()],
            ..AccountTraces::default()
        };
        let mut shortcuts = vec![
            shortcut(r"D:\Desktop\Alice\Acme.lnk", MAIN, None),
            shortcut(r"C:\Users\Alice\Desktop\Acme.lnk", MAIN, None),
        ];
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            &mut shortcuts,
            &folders,
            Path::new(MAIN),
            false,
            &traces,
            &mut left_out,
        );
        assert_eq!(shortcuts.len(), 1, "{shortcuts:?}");
        assert_eq!(
            shortcuts[0].original_lnk_path,
            Path::new(r"%USERPROFILE%\Desktop\Acme.lnk")
        );
    }

    #[test]
    fn running_again_says_nothing_again() {
        let folders = [PathBuf::from(APP)];
        let mut shortcuts = vec![
            shortcut(r"C:\Users\randy\Desktop\Acme.lnk", MAIN, None),
            shortcut(r"C:\Users\randy\Desktop\B.lnk", MAIN, Some("-x")),
        ];
        assert_eq!(withheld(&mut shortcuts, &folders, false).len(), 1);
        let again = shortcuts.clone();
        assert!(withheld(&mut shortcuts, &folders, false).is_empty());
        assert_eq!(shortcuts.len(), again.len());
        assert_eq!(shortcuts[0].original_lnk_path, again[0].original_lnk_path);
    }

    fn traces() -> AccountTraces {
        AccountTraces {
            account_names: vec!["alice".to_owned(), "admin".to_owned()],
            profile_folders: vec![r"D:\Homes\alice".to_owned()],
            computer_names: vec!["OLD-LAPTOP".to_owned()],
            sids: vec!["S-1-5-21-9-9-9-1001".to_owned()],
        }
    }

    #[test]
    fn what_is_said_goes_on_saying_it_without_whose_it_was() {
        let traces = traces();
        let scrubbed = traces.scrub(
            r"alice's Documents folder was at \\server\home$\alice\Documents, which is not on a disk",
        );
        assert_eq!(
            scrubbed,
            r"<an account>'s Documents folder was at <a network folder>, which is not on a disk"
        );
        assert_eq!(
            traces.scrub(r"Could not copy D:\Homes\alice\a.txt on OLD-LAPTOP"),
            r"Could not copy <an account's folder>\a.txt on <a PC>"
        );
        assert_eq!(
            traces.scrub(r"Read C:\Users\bob smith\AppData\x, and C:\Users\Public\y"),
            r"Read <an account's folder>\AppData\x, and C:\Users\Public\y"
        );
        assert_eq!(
            traces.scrub("Could not tell who: S-1-5-21-1-2-3-1002 or S-1-5-18"),
            "Could not tell who: <an account> or S-1-5-18"
        );
        assert_eq!(
            traces.scrub(r"\\?\C:\App is here, malice aforethought"),
            r"\\?\C:\App is here, malice aforethought",
            "not a network path, and not her name"
        );
    }

    #[test]
    fn an_account_called_a_word_only_counts_as_a_folder() {
        let traces = traces();
        assert_eq!(traces.scrub("Run as admin first"), "Run as admin first");
        assert_eq!(
            traces.scrub(r"OLD\admin ran it"),
            r"OLD\<an account> ran it"
        );
    }

    fn manifest_saying(value: &str) -> PackageManifest {
        let mut manifest: PackageManifest = serde_json::from_value(serde_json::json!({
            "schema_version": "1",
            "scan_timestamp_utc": "",
            "main_executable_path": r"C:\App\a.exe",
            "file_inventory": [],
            "registry_keys_exported": [],
            "services_detected": [],
            "scheduled_tasks_detected": [],
            "shortcuts_detected": [],
            "environment_variables_detected": [],
            "runtime_prerequisites": [],
            "warnings": [],
            "items_intentionally_excluded": [],
            "included_user_data": false,
            "included_license_data": false,
            "included_program_data": false,
        }))
        .unwrap_or_else(|error| panic!("{error}"));
        manifest.items_intentionally_excluded = vec![value.to_owned()];
        manifest
    }

    #[test]
    fn a_last_look_finds_a_name_however_it_is_written_and_says_where_and_not_what() {
        let forms = [
            "made by alice".to_owned(),
            r"C:\Users\ALICE\a.txt".to_owned(),
            // As a binary registry value holds it, in each encoding.
            "616c6963652068657265".to_owned(),
            "61,00,6c,00,69,00,63,00,65,00".to_owned(),
        ];
        for form in forms {
            let warnings = what_a_package_still_says(&manifest_saying(&form), &[], &traces());
            assert_eq!(warnings.len(), 1, "{form}: {warnings:?}");
            assert_eq!(warnings[0].code, MENTIONS_CODE);
            assert!(!warnings[0].message.contains("alice"), "{warnings:?}");
            assert!(
                warnings[0]
                    .message
                    .contains("items intentionally excluded (1)"),
                "{warnings:?}"
            );
        }
    }

    #[test]
    fn a_last_look_is_quiet_about_a_package_that_says_nothing() {
        let quiet = manifest_saying("malice and an admin tool");
        assert!(what_a_package_still_says(&quiet, &[], &traces()).is_empty());
    }

    #[test]
    fn a_last_look_goes_through_the_other_files_when_the_manifest_says_nothing() {
        let quiet = manifest_saying("nothing");
        let in_both = AccountTraces {
            account_names: vec!["alice".to_owned(), "bob".to_owned()],
            ..AccountTraces::default()
        };
        let warnings = what_a_package_still_says(
            &manifest_saying("by alice"),
            &[("report.md".to_owned(), b"and by bob".to_vec())],
            &in_both,
        );
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert!(warnings[0].message.contains("report.md"), "{warnings:?}");
        let three = AccountTraces {
            computer_names: vec!["Bob".to_owned()],
            ..AccountTraces::default()
        };
        assert_eq!(
            what_a_package_still_says(&manifest_saying("426f62"), &[], &three).len(),
            1,
            "three letters, as a binary value holds them"
        );
        let written = vec![(
            r"reports\warnings.md".to_owned(),
            "on OLD-LAPTOP"
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect(),
        )];
        let warnings = what_a_package_still_says(&quiet, &written, &traces());
        assert_eq!(warnings.len(), 1, "{warnings:?}");
        assert!(warnings[0].message.contains("warnings.md"), "{warnings:?}");
        assert!(
            warnings[0].message.contains("the PC's name"),
            "{warnings:?}"
        );
    }

    #[test]
    fn a_variable_of_an_accounts_goes_unless_all_it_says_is_where_the_app_is() {
        let folders = [PathBuf::from(APP)];
        let variable =
            |scope: EnvironmentScope, name: &str, value: &str| EnvironmentVariableEntry {
                scope,
                name: name.to_owned(),
                value: value.to_owned(),
                value_kind: crate::models::RegistryValueKind::String,
                requires_admin: false,
                confidence: crate::models::MatchConfidence::High,
            };
        let mut variables = vec![
            variable(
                EnvironmentScope::Machine,
                "PYTHONPATH",
                r"C:\Program Files\Acme\lib;C:\Users\randy\work",
            ),
            variable(EnvironmentScope::User, "ACME_HOME", APP),
            variable(EnvironmentScope::User, "ACME_MODE", "fast"),
            variable(
                EnvironmentScope::User,
                "Path",
                r"C:\Program Files\Acme\bin;C:\Users\randy\bin",
            ),
            variable(EnvironmentScope::Machine, "ACME_MODE", "fast"),
            variable(
                EnvironmentScope::Machine,
                "ACME_DATA",
                r"C:\Users\randy\data",
            ),
        ];
        let mut left_out = LeftOut::default();
        withhold_variables(
            &mut variables,
            &folders,
            &|text| text.to_owned(),
            &AccountTraces::default(),
            &mut left_out,
        );
        let kept: Vec<(&str, &str)> = variables
            .iter()
            .map(|variable| (variable.name.as_str(), variable.value.as_str()))
            .collect();
        assert_eq!(
            kept,
            [
                ("PYTHONPATH", r"C:\Program Files\Acme\lib"),
                ("ACME_HOME", APP),
                ("Path", r"C:\Program Files\Acme\bin"),
                ("ACME_MODE", "fast"),
            ]
        );
        assert_eq!(left_out.settings.len(), 1);
        assert!(
            left_out.settings[0].why.starts_with("2 variables"),
            "{:?}",
            left_out.settings
        );
    }

    #[test]
    fn a_service_that_ran_as_someone_says_so_and_not_who() {
        for (recorded, kept) in [
            (r"PC\alice", false),
            (r"CONTOSO\svc-acme", false),
            (r"NT SERVICE\Other", false),
            ("LocalSystem", true),
            (r"NT AUTHORITY\LocalService", true),
            (r"NT SERVICE\Acme", true),
        ] {
            let mut account = Some(recorded.to_owned());
            withhold_service_account(&mut account, "Acme");
            assert_eq!(account.as_deref() == Some(recorded), kept, "{recorded}");
        }
        let mut account = Some(r"PC\alice".to_owned());
        withhold_service_account(&mut account, "Acme");
        assert!(matches!(
            ServiceAccount::from_recorded(account.as_deref(), "Acme"),
            ServiceAccount::Custom(_)
        ));
    }

    #[test]
    fn what_names_someones_folder_is_told_from_the_apps_own() {
        let app = r"C:\Users\randy\AppData\Local\Acme";
        let folders = [PathBuf::from(app)];
        let none = AccountTraces::default();
        assert!(names_a_place_of_an_account(
            r"--config C:\Users\randy\work\a.ini",
            &folders,
            &none
        ));
        assert!(names_a_place_of_an_account(
            r#"--config="C:\Users\Bob Smith\a.ini""#,
            &folders,
            &none
        ));
        assert!(!names_a_place_of_an_account(
            &format!(r#"--config "{app}\a.ini""#),
            &folders,
            &none
        ));
        assert!(!names_a_place_of_an_account(
            r"--shared C:\Users\Public\Documents\a.ini",
            &folders,
            &none
        ));
        assert!(!names_a_place_of_an_account("--port 80", &folders, &none));
        assert!(names_a_place_of_an_account(
            r"--shared C:\Users\Publicity\a.ini",
            &folders,
            &none
        ));
        let elsewhere = AccountTraces {
            profile_folders: vec![r"D:\Profiles\Alice".to_owned()],
            ..AccountTraces::default()
        };
        assert!(!names_a_place_of_an_account(
            r"D:\Profiles\Alice\a.ini",
            &folders,
            &none
        ));
        assert!(!names_a_place_of_an_account(
            r"D:\Profiles\AliceProject\a.ini",
            &folders,
            &elsewhere
        ));
        assert!(names_a_place_of_an_account(
            r#""D:\Profiles\Alice" --x"#,
            &folders,
            &elsewhere
        ));
        assert!(names_a_place_of_an_account(
            r"--home 'D:\Profiles\Alice'",
            &folders,
            &elsewhere
        ));
        assert!(names_a_place_of_an_account(
            r"--home 'C:\Users\Bob\a.ini'",
            &folders,
            &none
        ));
        let apostrophe = [PathBuf::from(r"C:\Users\O'Neil\App")];
        assert!(!names_a_place_of_an_account(
            r"C:\Users\O'Neil\App\app.exe",
            &apostrophe,
            &none
        ));
        assert!(!names_a_place_of_an_account(
            r"'C:\Users\O'Neil\App\app.exe'",
            &[PathBuf::from(r"C:\Users\O")],
            &none
        ));
        assert!(names_a_place_of_an_account(
            r"D:\Profiles\Alice\a.ini",
            &folders,
            &elsewhere
        ));
        assert!(is_in_an_accounts_folders(
            Path::new(r"D:\Profiles\Alice\Desktop\a.lnk"),
            &elsewhere
        ));
        assert!(!is_in_an_accounts_folders(
            Path::new(r"C:\Users\Public\Desktop\a.lnk"),
            &none
        ));
        assert!(is_in_an_accounts_folders(
            Path::new(r"C:\Users\Publicity\Desktop\a.lnk"),
            &none
        ));
    }
}
