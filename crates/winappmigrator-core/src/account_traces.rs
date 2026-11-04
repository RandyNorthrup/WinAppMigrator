//! What a package says of whose PC it came from, with none of an account's data in it.
//!
//! An installer can be copied and shared. An account's key under `HKEY_CURRENT_USER` is only one of the
//! places that can say who it was: a scheduled task records who wrote it, a shortcut on someone's Desktop
//! says what they opened with it, and each of those has gone along by a route nothing was looking at. So
//! this goes by what each thing is, and is one function, which runs when a scan is asked for none of
//! an account's data, and again when a person has left all of that data out of one that was.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::account_settings::{
    say_no_more_of_licenses_than_the_apps_key, say_no_more_than_the_apps_key,
};
use crate::errors::Result;
use crate::file_discovery::folders_an_account_has_moved;
use crate::models::{
    DependencyEntry, EnvironmentScope, EnvironmentVariableEntry, FileEntry, InstallScope,
    LeftOutItem, PackageManifest, RegistryHive, ScanReport, ScheduledTaskEntry, ServiceEntry,
    ShortcutEntry, Warning, WarningSeverity,
};
use crate::path_utils::{
    comparable_path, current_user_sid, expand_env_vars, is_person_sid, path_is_within,
    signed_in_user,
};
use crate::restore_policy::ServiceAccount;
use crate::secret_rules::LeftOut;
use crate::shortcut_scan::{folders_of_the_accounts_shortcuts, shared_shortcut_folders};
use crate::system_context::SystemContext;
use crate::task_scan::{
    TaskThatNames, task_for_whoever_installs_it, without_what_names_an_account,
};

const USERS_FOLDER: &str = r"\users\";
/// What is under `Users` and nobody's.
const NOBODYS_FOLDERS: [&str; 4] = ["public", "default", "default user", "all users"];
/// What a path can be written with that is wherever an account's folder is, by what is below that.
const ACCOUNTS_FOLDER_VARIABLES: [(&str, &str); 3] = [
    ("%userprofile%", ""),
    ("%appdata%", r"\AppData\Roaming"),
    ("%localappdata%", r"\AppData\Local"),
];
/// What can only be said of whoever reads it, wherever the app is.
const ONLY_AN_ACCOUNTS_VARIABLES: [&str; 5] =
    ["%username%", "%homepath%", "%homedrive%", "%temp%", "%tmp%"];
/// The words a name comes after when that is what it is, in what a package says of someone.
const WORDS_BEFORE_A_NAME: [&str; 9] = [
    "account", "as", "by", "for", "from", "of", "on", "owner", "user",
];
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
/// What a warning from the last look over a package is known by, which a screen has to put where it
/// will be read: it is about what strangers can read in an installer that looks finished.
pub const MENTIONS_CODE: &str = "package_mentions_an_account";
const ENDS_A_PATH_ON_A_COMMAND_LINE: [char; 6] = ['"', '<', '>', '|', ';', ','];
/// The shortest run of hex that can be a value's bytes, rather than a number.
const SHORTEST_HEX_VALUE: usize = 6;
/// What Windows-1252 has where Latin-1 has control characters, from `0x80`.
const WINDOWS_1252_FROM_0X80: [char; 32] = [
    '€', '\u{81}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{8D}', 'Ž', '\u{8F}',
    '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\u{9D}', 'ž', 'Ÿ',
];
const SID_PREFIX: &str = "S-1-";
const TWO_BYTES_A_CHARACTER_WITH_NO_ZEROS_UP_TO: usize = 64 * 1024;
const FOLDER_UNREADABLE_CODE: &str = "file_discovery_dir_unreadable";

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
        // Where its own settings say its folders are, which can be anywhere.
        let shared = shared_shortcut_folders(system);
        for folder in folders_of_the_accounts_shortcuts(system) {
            if !shared
                .iter()
                .any(|base| comparable_path(&folder) == comparable_path(base))
            {
                traces.add_folder(&system.logical_path(&folder));
            }
        }
        for user in system.user_hives() {
            for folder in folders_an_account_has_moved(&user, system) {
                traces.add_folder(&folder);
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

/// Whether a name at this place in text is being used as one, by what is around it: part of a path,
/// someone's, or after a word that comes before a name. An account can be called what a word is, and
/// a PC what a product is, and neither of those is to be taken out of a sentence.
fn is_said_as_a_name(lower: &str, at: usize, length: usize) -> bool {
    let before = &lower[..at];
    let after = &lower[at + length..];
    let word_before = before
        .trim_end()
        .rsplit(|character: char| !character.is_alphanumeric())
        .next()
        .unwrap_or_default();
    (before.is_empty() && !after.starts_with('.'))
        || before.ends_with(['\\', '/', '=', ':', '"', '\'', '<', '>'])
        || after.starts_with(['\\', '/', '@'])
        || after.starts_with("'s")
        || WORDS_BEFORE_A_NAME.contains(&word_before)
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
        if !is_said_as_a_name(lower, at, name.len()) {
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
        // A drive's letter, if it is one: what comes before the colon can be anything.
        let start = if at >= 2
            && lower.as_bytes()[at - 1] == b':'
            && lower.as_bytes()[at - 2].is_ascii_alphabetic()
        {
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

/// The bytes a value can hold under another form: hex, as a binary registry value is written. `None`
/// for what is not that.
fn bytes_in_hex(text: &str) -> Option<Vec<u8>> {
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
    Some(
        (0..digits.len())
            .step_by(2)
            .filter_map(|at| u8::from_str_radix(&digits[at..at + 2], 16).ok())
            .collect(),
    )
}

fn windows_1252(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| match byte {
            0x80..=0x9F => WINDOWS_1252_FROM_0X80[usize::from(byte - 0x80)],
            other => char::from(other),
        })
        .collect()
}

fn wide_text(bytes: &[u8], from: fn([u8; 2]) -> u16) -> String {
    let (pairs, _) = bytes.as_chunks::<2>();
    let wide: Vec<u16> = pairs.iter().map(|pair| from(*pair)).collect();
    String::from_utf16_lossy(&wide)
}

/// Bytes read each way a value's or a file's can be, one string for each. Each way of two bytes to a
/// character goes from both places it could start, as a file need not hold only text. Those only go for
/// something large if it has a zero in it, which text in those has unless none of it is Latin at all.
fn text_forms_of(bytes: &[u8]) -> Vec<String> {
    let mut forms = vec![
        String::from_utf8_lossy(bytes).into_owned(),
        windows_1252(bytes),
    ];
    if bytes.len() <= TWO_BYTES_A_CHARACTER_WITH_NO_ZEROS_UP_TO || bytes.contains(&0) {
        for start in [0, 1] {
            let part = bytes.get(start..).unwrap_or_default();
            forms.push(wide_text(part, u16::from_le_bytes));
            forms.push(wide_text(part, u16::from_be_bytes));
        }
    }
    forms
}

/// An account's id as Windows holds it, which is how a binary value would.
fn sid_as_bytes(sid: &str) -> Option<Vec<u8>> {
    let mut parts = sid.strip_prefix(SID_PREFIX)?.split('-');
    let authority: u64 = parts.next()?.parse().ok()?;
    let below: Vec<u32> = parts
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()
        .ok()?;
    let mut bytes = vec![1, u8::try_from(below.len()).ok()?];
    bytes.extend(authority.to_be_bytes().get(2..)?);
    for part in below {
        bytes.extend(part.to_le_bytes());
    }
    Some(bytes)
}

/// What has been seen of who someone is, by its kind and which of what was looked for it is.
type Found = BTreeSet<(TraceKind, usize)>;
/// What is seen by what it looks like, and not as any one thing that was looked for.
const BY_ITS_SHAPE: usize = usize::MAX;

/// What to look for in a package, each once.
struct LastLook {
    words: Vec<(TraceKind, String)>,
    ids: Vec<Vec<u8>>,
}

impl LastLook {
    fn for_traces(traces: &AccountTraces) -> Self {
        Self {
            words: traces
                .to_look_for()
                .into_iter()
                .map(|(kind, text)| (kind, text.to_lowercase()))
                .collect(),
            ids: traces
                .sids
                .iter()
                .filter_map(|sid| sid_as_bytes(sid))
                .collect(),
        }
    }

    /// What of who someone is this says. Anyone's folder and anyone's id say so by their shape, as an
    /// account can have gone from a PC, and a folder can be called by its short name.
    fn in_text(&self, text: &str, found: &mut Found) {
        let lower = text.to_lowercase();
        for (which, (kind, what)) in self.words.iter().enumerate() {
            if !found.contains(&(*kind, which)) && mentions(&lower, *kind, what) {
                found.insert((*kind, which));
            }
        }
        if without_accounts_folders(&lower) != lower {
            found.insert((TraceKind::Folder, BY_ITS_SHAPE));
        }
        if mentions_a_person_id(text) {
            found.insert((TraceKind::Sid, BY_ITS_SHAPE));
        }
    }

    fn in_bytes(&self, bytes: &[u8], found: &mut Found) {
        for form in text_forms_of(bytes) {
            self.in_text(&form, found);
        }
        let has_id = self
            .ids
            .iter()
            .any(|id| bytes.windows(id.len()).any(|window| window == id));
        if has_id {
            found.insert((TraceKind::Sid, BY_ITS_SHAPE));
        }
    }

    /// As a manifest holds it: text, or what is written as hex or as a list of numbers.
    fn in_value_text(&self, text: &str, found: &mut Found) {
        self.in_text(text, found);
        if let Some(bytes) = bytes_in_hex(text) {
            self.in_bytes(&bytes, found);
        }
    }

    /// Each value under a part of a manifest, by what each says.
    fn in_value(&self, value: &Value, said: &mut dyn FnMut(Found)) {
        match value {
            Value::String(text) => {
                let mut found = Found::new();
                self.in_value_text(text, &mut found);
                said(found);
            }
            Value::Array(items) if items.iter().all(Value::is_u64) && !items.is_empty() => {
                let bytes: Vec<u8> = items
                    .iter()
                    .filter_map(Value::as_u64)
                    .filter_map(|number| u8::try_from(number).ok())
                    .collect();
                let mut found = Found::new();
                self.in_bytes(&bytes, &mut found);
                said(found);
            }
            Value::Array(items) => {
                for item in items {
                    self.in_value(item, said);
                }
            }
            Value::Object(fields) => {
                for (name, item) in fields {
                    let mut found = Found::new();
                    self.in_text(name, &mut found);
                    said(found);
                    self.in_value(item, said);
                }
            }
            _ => {}
        }
    }
}

/// Whether text holds an id of the kind only a person has, however it is written.
fn mentions_a_person_id(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    without_person_sids(&upper) != upper
}

fn mentions(lower: &str, kind: TraceKind, what: &str) -> bool {
    match kind {
        TraceKind::Folder | TraceKind::Sid => lower.contains(what),
        TraceKind::Account | TraceKind::Computer => !places_of_name(lower, what).is_empty(),
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
///
/// It goes by each thing that says who someone is in each form text is held in, and anyone's folder
/// and anyone's id by their shape. Nothing encoded any other way is seen. `looked_at` is told how far
/// it has got, and is how it is stopped.
pub(crate) fn what_a_package_still_says(
    manifest: &PackageManifest,
    written: &[(String, Vec<u8>)],
    traces: &AccountTraces,
    looked_at: &dyn Fn(usize, usize) -> Result<()>,
) -> Result<Vec<Warning>> {
    let Ok(Value::Object(fields)) = serde_json::to_value(manifest) else {
        return Ok(Vec::new());
    };
    let look = LastLook::for_traces(traces);
    let total = fields.len() + written.len();
    let mut done = 0;
    let mut said: BTreeMap<TraceKind, BTreeMap<String, usize>> = BTreeMap::new();
    let mut in_the_manifest = Found::new();
    let note = |found: &Found, place: &str, said: &mut BTreeMap<_, BTreeMap<String, usize>>| {
        let kinds: BTreeSet<TraceKind> = found.iter().map(|(kind, _)| *kind).collect();
        for kind in kinds {
            *said
                .entry(kind)
                .or_default()
                .entry(place.to_owned())
                .or_default() += 1;
        }
    };
    for (field, value) in &fields {
        looked_at(done, total)?;
        done += 1;
        let place = part_called(field);
        look.in_value(value, &mut |found| {
            note(&found, &place, &mut said);
            in_the_manifest.extend(found);
        });
    }
    for (name, bytes) in written {
        looked_at(done, total)?;
        done += 1;
        let mut found = Found::new();
        look.in_bytes(bytes, &mut found);
        let only_here: Found = found.difference(&in_the_manifest).copied().collect();
        note(&only_here, name, &mut said);
    }
    Ok(said
        .into_iter()
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
        .collect())
}

/// Whether a path is under `Users` and past the `Public` there, whose name has to be all of it.
fn is_in_a_users_folder(lower: &str) -> bool {
    lower.find(USERS_FOLDER).is_some_and(|at| {
        let name = &lower[at + USERS_FOLDER.len()..];
        let name = name.split('\\').next().unwrap_or_default();
        !name.is_empty() && !NOBODYS_FOLDERS.contains(&name)
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
    if ONLY_AN_ACCOUNTS_VARIABLES
        .iter()
        .any(|variable| lower.contains(variable))
    {
        return true;
    }
    // Written as whoever reads it would have it, which only the app's own folder can be let off.
    for (variable, _) in ACCOUNTS_FOLDER_VARIABLES {
        let mut from = 0;
        while let Some(found) = lower[from..].find(variable) {
            let at = from + found;
            from = at + variable.len();
            if !reads_as_the_apps(path_on_a_command_line(&lower, at), app_folders, traces) {
                return true;
            }
        }
    }
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

/// Whether a path written with a variable of an account's is in the app's folder, for any account
/// whose folder is known. Not by this process's environment, which is whoever is running this.
fn reads_as_the_apps(path: &str, app_folders: &[PathBuf], traces: &AccountTraces) -> bool {
    traces.profile_folders.iter().any(|profile| {
        let mut read = path.to_owned();
        for (variable, below) in ACCOUNTS_FOLDER_VARIABLES {
            read = replace_ignoring_case(&read, variable, &format!("{profile}{below}"));
        }
        app_folders
            .iter()
            .any(|folder| path_is_within(Path::new(&read), folder))
    })
}

/// The words of a command line, as whatever it runs would be given them: split by spaces, apart from
/// in quotes.
fn words_of(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_quotes = false;
    for character in text.chars() {
        match character {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            other => word.push(other),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// What a word says, apart from being an option: all of it, or what is given to an option in it, as in
/// `--open=C:\a.txt` and `/open:C:\a.txt`. An option on its own says nothing.
fn what_a_word_says(word: &str) -> Option<&str> {
    if word.starts_with("//") {
        return Some(word);
    }
    let Some(option) = word.strip_prefix('-').or_else(|| word.strip_prefix('/')) else {
        return Some(word);
    };
    let at = option.find(['=', ':'])?;
    option.get(at + 1..).filter(|value| !value.is_empty())
}

/// Whether something is written as a place or as someone, rather than as a word.
fn is_written_as_a_place(value: &str) -> bool {
    value.contains(['\\', '/', '%', '@', ':'])
}

/// Whether something written as a place is the app's. With no way up and nothing to say where else it
/// is, it is relative to wherever the app is running.
fn is_a_place_of_the_app(value: &str, app_folders: &[PathBuf], traces: &AccountTraces) -> bool {
    if value.contains('@') || value.contains("://") {
        return false;
    }
    if reads_as_the_apps(value, app_folders, traces) {
        return true;
    }
    let expanded = expand_env_vars(value);
    let path = Path::new(&expanded);
    if path.is_absolute() {
        return app_folders
            .iter()
            .any(|folder| path_is_within(path, folder));
    }
    !expanded.contains(['%', ':'])
        && !expanded.starts_with(['\\', '/'])
        && !expanded.split(['\\', '/']).any(|part| part == "..")
}

/// With none of an account's data going: a machine-wide shortcut or task can be given anything to work
/// with, and what is written as a place or as someone and is not the app's is what a person gave it.
/// Plain words are how an app is told what to do.
fn arguments_name_what_is_not_the_apps(
    arguments: &str,
    app_folders: &[PathBuf],
    traces: &AccountTraces,
) -> bool {
    words_of(arguments).iter().any(|word| {
        what_a_word_says(word).is_some_and(|value| {
            is_written_as_a_place(value) && !is_a_place_of_the_app(value, app_folders, traces)
        })
    })
}

/// The names of what an app has in its folders, as what an app of one account's starts itself with
/// goes by them.
pub(crate) fn names_in_the_app(files: &[FileEntry]) -> HashSet<String> {
    let mut names = HashSet::new();
    for file in files.iter().filter(|file| file.should_copy) {
        names.extend(
            file.original_path
                .file_name()
                .map(|name| name.to_string_lossy().to_lowercase()),
        );
    }
    names
}

/// What an app of one account's starts itself with has to be about the app: every word of it says
/// nothing, or names something of the app's. `--processStart Discord.exe` does, and
/// `-load bank-prod` is someone's. A switch can be given a number, or told yes or no.
fn every_word_is_about_the_app(
    arguments: &str,
    app_folders: &[PathBuf],
    names: &HashSet<String>,
    traces: &AccountTraces,
) -> bool {
    words_of(arguments)
        .iter()
        .filter_map(|word| what_a_word_says(word))
        .all(|value| {
            let lower = value.to_lowercase();
            if is_written_as_a_place(value) {
                return is_a_place_of_the_app(value, app_folders, traces);
            }
            names.contains(&lower)
                || lower == "true"
                || lower == "false"
                || lower.bytes().all(|byte| byte.is_ascii_digit())
        })
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
    names: &HashSet<String>,
    traces: &AccountTraces,
    left_out: &mut LeftOut,
) {
    let is_the_apps = |path: &Path| {
        path == main_program
            || app_folders
                .iter()
                .any(|folder| path_is_within(path, folder))
    };
    let says_nothing_of_anyone = |path: &PathBuf| {
        is_the_apps(path)
            || (!is_in_an_accounts_folders(path, traces)
                && !names_a_place_of_an_account(&path.to_string_lossy(), app_folders, traces))
    };
    let mut withheld = 0_usize;
    shortcuts.retain_mut(|shortcut| {
        let arguments = shortcut.arguments.as_deref().unwrap_or_default();
        if names_a_place_of_an_account(arguments, app_folders, traces)
            || arguments_name_what_is_not_the_apps(arguments, app_folders, traces)
            || !says_nothing_of_anyone(&shortcut.target_path)
        {
            withheld += 1;
            return false;
        }
        shortcut.working_directory = shortcut
            .working_directory
            .take()
            .filter(says_nothing_of_anyone);
        shortcut.icon_path = shortcut.icon_path.take().filter(says_nothing_of_anyone);
        if !is_in_an_accounts_folders(&shortcut.original_lnk_path, traces) {
            return true;
        }
        let its_own = is_the_apps(&shortcut.target_path)
            && (arguments.trim().is_empty()
                || (installed_for_one_account
                    && every_word_is_about_the_app(arguments, app_folders, names, traces)));
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
    let arguments_are_theirs = |text: &str| {
        names_them(text) || arguments_name_what_is_not_the_apps(text, app_folders, traces)
    };
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
            is_named_for_them(task) || task.arguments.as_deref().is_some_and(arguments_are_theirs);
        let cut = if goes_whole {
            TaskThatNames::GoesWhole
        } else {
            match task.definition_xml.as_deref() {
                Some(xml) => without_what_names_an_account(xml, &names_them, &arguments_are_theirs),
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
    /// What the app has in its folders, by name.
    pub names_in_the_app: &'a HashSet<String>,
    pub traces: &'a AccountTraces,
}

/// With the User data option off: what of an app's settings is an account's, by what each is.
pub(crate) fn withhold_settings(from: &mut Withholding<'_>, left_out: &mut LeftOut) {
    withhold_shortcuts(
        from.shortcuts,
        from.app_folders,
        from.main_program,
        from.installed_for_one_account,
        from.names_in_the_app,
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
        let entry = entry.trim().trim_matches('"');
        let expanded = expand(entry);
        reads_as_the_apps(entry, app_folders, traces)
            || (Path::new(&expanded).is_absolute()
                && app_folders
                    .iter()
                    .any(|folder| path_is_within(Path::new(&expanded), folder)))
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

/// A service keeps what runs it. Told to work on a place of someone's, what it does is theirs, as for a
/// task, and a setting of its that names one goes without it.
fn withhold_services(
    services: &mut Vec<ServiceEntry>,
    app_folders: &[PathBuf],
    traces: &AccountTraces,
    left_out: &mut LeftOut,
) {
    let names_them = |text: &str| {
        names_a_place_of_an_account(text, app_folders, traces)
            || bytes_in_hex(text).is_some_and(|bytes| {
                text_forms_of(&bytes)
                    .iter()
                    .any(|form| names_a_place_of_an_account(form, app_folders, traces))
            })
    };
    services.retain_mut(|service| {
        withhold_service_account(&mut service.account_name, &service.service_name);
        if names_them(&service.restore_command_suggestion) {
            service.restore_command_suggestion.clear();
        }
        if service.arguments.as_deref().is_some_and(|arguments| {
            names_them(arguments)
                || arguments_name_what_is_not_the_apps(arguments, app_folders, traces)
        }) {
            left_out.setting_of(
                format!("The service {}", service.service_name),
                "It works on a place in an account's folders, and only goes with the User data \
                 option",
            );
            return false;
        }
        service.parameters.retain(|entry| {
            !entry.value_data.as_deref().is_some_and(names_them)
                && !entry.value_bytes_hex.as_deref().is_some_and(names_them)
        });
        true
    });
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

/// What could not be read while looking through `ProgramData` and an account's folders is mostly other
/// software's, and what that is called is no one's to know from an installer.
fn say_only_how_many_folders_of_others_could_not_be_read(
    warnings: &mut Vec<Warning>,
    app_folders: &[PathBuf],
) {
    let before = warnings.len();
    warnings.retain(|warning| {
        warning.code != FOLDER_UNREADABLE_CODE
            || warning.source.as_deref().is_some_and(|source| {
                app_folders
                    .iter()
                    .any(|folder| path_is_within(Path::new(source), folder))
            })
    });
    let gone = before - warnings.len();
    if gone > 0 {
        warnings.push(Warning::new(
            FOLDER_UNREADABLE_CODE,
            WarningSeverity::Info,
            format!(
                "{gone} folders outside the app could not be read, so data of the app's there may be \
                 missing."
            ),
            None,
        ));
    }
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
    say_no_more_than_the_apps_key(&mut report.warnings);
    let folders = folders_of_the_app_in(report);
    say_only_how_many_folders_of_others_could_not_be_read(&mut report.warnings, &folders);
    let names = names_in_the_app(&report.file_inventory);
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
            names_in_the_app: &names,
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
    withhold_services(&mut report.services, &folders, &traces, &mut left_out);
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

    fn still_says(
        manifest: &PackageManifest,
        written: &[(String, Vec<u8>)],
        traces: &AccountTraces,
    ) -> Vec<Warning> {
        what_a_package_still_says(manifest, written, traces, &|_, _| Ok(()))
            .unwrap_or_else(|error| panic!("{error}"))
    }

    fn names_of_the_app() -> HashSet<String> {
        ["discord.exe".to_owned()].into_iter().collect()
    }

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
            &names_of_the_app(),
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
            &names_of_the_app(),
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
            &names_of_the_app(),
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
            &names_of_the_app(),
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

    const PUBLIC_DESKTOP: &str = r"C:\Users\Public\Desktop\Acme.lnk";

    fn on_a_shared_desktop_with(arguments: &str, traces: &AccountTraces) -> bool {
        let mut shortcuts = vec![shortcut(PUBLIC_DESKTOP, MAIN, Some(arguments))];
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            &mut shortcuts,
            &[PathBuf::from(APP)],
            Path::new(MAIN),
            false,
            &names_of_the_app(),
            traces,
            &mut left_out,
        );
        !shortcuts.is_empty()
    }

    /// Who it is for need not be under `Users`, or written as it is on this PC.
    #[test]
    fn a_machine_wide_shortcut_given_a_place_or_a_person_that_is_not_the_apps_goes_with_user_data()
    {
        let traces = AccountTraces {
            profile_folders: vec![r"E:\Data\MyDocs".to_owned()],
            ..AccountTraces::default()
        };
        for arguments in [
            r"--open E:\Data\MyDocs\salary.xlsx",
            r"--open %USERPROFILE%\Documents\bank.xlsx",
            r"--open %LOCALAPPDATA%\Other\a.txt",
            r"--user alice.smith@example.com",
            r#""D:\Private\a.txt""#,
            r"/open:E:\x.txt",
            r"--open=https://example.com/mine",
            "%USERNAME%",
            r"\\server\share\a.txt",
        ] {
            assert!(!on_a_shared_desktop_with(arguments, &traces), "{arguments}");
        }
    }

    #[test]
    fn a_machine_wide_shortcut_told_what_to_do_or_given_a_place_of_the_apps_stays() {
        let traces = AccountTraces::default();
        for arguments in [
            "--minimized",
            "--mode fast",
            "game1",
            r#"--config "C:\Program Files\Acme\a.ini""#,
            r#""--config=C:\Program Files\Acme\a.ini""#,
            r#""%ProgramFiles%\Acme\a.ini""#,
            r"--config data\a.ini",
        ] {
            assert!(on_a_shared_desktop_with(arguments, &traces), "{arguments}");
        }
        assert!(!on_a_shared_desktop_with(r"--config ..\..\a.ini", &traces));
    }

    #[test]
    fn a_machine_wide_shortcut_keeps_no_folder_or_icon_of_someones() {
        let mut one = shortcut(PUBLIC_DESKTOP, MAIN, None);
        one.working_directory = Some(PathBuf::from(r"C:\Users\alice\Work"));
        one.icon_path = Some(PathBuf::from(r"C:\Users\alice\i.ico"));
        let mut two = shortcut(PUBLIC_DESKTOP, MAIN, None);
        two.working_directory = Some(PathBuf::from(APP));
        two.icon_path = Some(PathBuf::from(MAIN));
        let mut shortcuts = vec![one, two];
        let mut left_out = LeftOut::default();
        withhold_shortcuts(
            &mut shortcuts,
            &[PathBuf::from(APP)],
            Path::new(MAIN),
            false,
            &names_of_the_app(),
            &AccountTraces::default(),
            &mut left_out,
        );
        assert_eq!(shortcuts.len(), 2);
        assert_eq!(shortcuts[0].working_directory, None);
        assert_eq!(shortcuts[0].icon_path, None);
        assert_eq!(shortcuts[1].working_directory, Some(PathBuf::from(APP)));
        assert_eq!(shortcuts[1].icon_path, Some(PathBuf::from(MAIN)));
    }

    /// An app of one account's starts itself with arguments of its own, and those stay. What else
    /// its shortcut says is that person's.
    #[test]
    fn an_app_of_one_accounts_shortcut_keeps_only_arguments_about_the_app() {
        let keep = |arguments: &str| {
            every_word_is_about_the_app(
                arguments,
                &[PathBuf::from(APP)],
                &names_of_the_app(),
                &AccountTraces::default(),
            )
        };
        for arguments in [
            "--processStart Discord.exe",
            "--processStart discord.exe --start-minimized",
            "--count=2 --hidden=true",
            r#"--processStart "C:\Program Files\Acme\x.exe""#,
        ] {
            assert!(keep(arguments), "{arguments}");
        }
        for arguments in [
            "-load bank-prod",
            "--user alice.smith@example.com",
            r"--open %USERPROFILE%\Documents\Taxes 2024.xlsx",
            "--profile-directory=Default",
        ] {
            assert!(!keep(arguments), "{arguments}");
        }
    }

    /// A variable that says where the app is, of whichever account's folder it is in, says nothing else
    /// of them, whoever is running this now.
    #[test]
    fn a_variable_written_with_an_accounts_variable_stays_if_it_is_the_apps() {
        let traces = AccountTraces {
            profile_folders: vec![r"E:\Users\bob".to_owned()],
            ..AccountTraces::default()
        };
        let folders = [PathBuf::from(r"E:\Users\bob\AppData\Local\Acme")];
        let variable = |value: &str| EnvironmentVariableEntry {
            scope: EnvironmentScope::User,
            name: "ACME_HOME".to_owned(),
            value: value.to_owned(),
            value_kind: crate::models::RegistryValueKind::ExpandString,
            requires_admin: false,
            confidence: crate::models::MatchConfidence::High,
        };
        let mut variables = vec![
            variable(r"%LOCALAPPDATA%\Acme"),
            variable(r"%LOCALAPPDATA%\Other"),
        ];
        let mut left_out = LeftOut::default();
        withhold_variables(
            &mut variables,
            &folders,
            &|text| text.to_owned(),
            &traces,
            &mut left_out,
        );
        assert_eq!(variables.len(), 1, "{variables:?}");
        assert_eq!(variables[0].value, r"%LOCALAPPDATA%\Acme");
    }

    fn service_named(arguments: Option<&str>) -> ServiceEntry {
        ServiceEntry {
            service_name: "AcmeSvc".to_owned(),
            display_name: None,
            binary_path: PathBuf::from(MAIN),
            arguments: arguments.map(str::to_owned),
            parameters: Vec::new(),
            start_type: None,
            account_name: None,
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: String::new(),
        }
    }

    #[test]
    fn a_service_says_nothing_of_a_place_of_someones() {
        let mut quiet = service_named(Some("--port 80"));
        quiet.restore_command_suggestion = r"sc create AcmeSvc C:\Users\alice\a.exe".to_owned();
        quiet.parameters = vec![
            crate::models::RegistryEntry {
                hive: RegistryHive::LocalMachine,
                key_path: "Parameters".to_owned(),
                value_name: Some("Port".to_owned()),
                value_data: Some("80".to_owned()),
                value_kind: None,
                value_bytes_hex: None,
                confidence: crate::models::MatchConfidence::High,
                requires_admin: true,
            },
            crate::models::RegistryEntry {
                hive: RegistryHive::LocalMachine,
                key_path: "Parameters".to_owned(),
                value_name: Some("Data".to_owned()),
                value_data: Some(r"C:\Users\alice\work".to_owned()),
                value_kind: None,
                value_bytes_hex: None,
                confidence: crate::models::MatchConfidence::High,
                requires_admin: true,
            },
        ];
        let mut services = vec![quiet, service_named(Some(r"--data C:\Users\alice\work"))];
        let mut left_out = LeftOut::default();
        withhold_services(
            &mut services,
            &[PathBuf::from(APP)],
            &AccountTraces::default(),
            &mut left_out,
        );
        assert_eq!(services.len(), 1, "{services:?}");
        assert_eq!(services[0].restore_command_suggestion, "");
        assert_eq!(services[0].parameters.len(), 1);
        assert_eq!(
            services[0].parameters[0].value_name.as_deref(),
            Some("Port")
        );
        assert_eq!(left_out.settings.len(), 1);
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
            let warnings = still_says(&manifest_saying(&form), &[], &traces());
            assert!(!warnings.is_empty(), "{form}");
            for warning in &warnings {
                assert_eq!(warning.code, MENTIONS_CODE);
                assert!(!warning.message.contains("alice"), "{warnings:?}");
                assert!(
                    warning.message.contains("items intentionally excluded (1)"),
                    "{warnings:?}"
                );
            }
        }
    }

    fn utf16_le(text: &str) -> Vec<u8> {
        text.encode_utf16().flat_map(u16::to_le_bytes).collect()
    }

    fn says_of_a_report(bytes: Vec<u8>) -> Vec<Warning> {
        let traces = AccountTraces {
            account_names: vec!["alice".to_owned(), "josé".to_owned()],
            sids: vec!["S-1-5-21-111-222-333-1001".to_owned()],
            ..AccountTraces::default()
        };
        still_says(
            &manifest_saying("nothing"),
            &[("report.bin".to_owned(), bytes)],
            &traces,
        )
    }

    /// Each as the review found one that was missed.
    #[test]
    fn a_last_look_finds_what_a_file_holds_in_any_of_the_forms_text_is_held_in() {
        let mut odd_offset = vec![0x41_u8];
        odd_offset.extend(utf16_le("owner alice here"));
        let big_endian: Vec<u8> = "owner alice here"
            .encode_utf16()
            .flat_map(u16::to_be_bytes)
            .collect();
        let forms: Vec<(&str, Vec<u8>)> = vec![
            ("UTF-16", utf16_le("owner alice here")),
            ("UTF-16 at an odd offset", odd_offset),
            ("UTF-16 big-endian", big_endian),
            ("Windows-1252", b"by Jos\xE9, of here".to_vec()),
            ("another case", "by JOSÉ, of here".as_bytes().to_vec()),
            (
                "a folder of someone no longer on this PC",
                br"C:\Users\oldadmin\a.txt".to_vec(),
            ),
            (
                "a short name of a folder",
                br"C:\Users\RANDYN~1\a.txt".to_vec(),
            ),
            (
                "someone's id that is not known here",
                b"S-1-5-21-999-888-777-1002".to_vec(),
            ),
            (
                "a known id as Windows holds it",
                sid_as_bytes("S-1-5-21-111-222-333-1001")
                    .unwrap_or_default()
                    .into_iter()
                    .chain([0, 1, 2])
                    .collect(),
            ),
        ];
        for (form, bytes) in forms {
            let warnings = says_of_a_report(bytes);
            assert!(!warnings.is_empty(), "{form}");
            assert!(warnings[0].message.contains("report.bin"), "{warnings:?}");
        }
    }

    /// Three characters of a Chinese name are six bytes with no zero among them.
    #[test]
    fn a_last_look_finds_a_name_with_no_zero_in_it_as_two_bytes_a_character() {
        let traces = AccountTraces {
            account_names: vec!["王小明".to_owned()],
            ..AccountTraces::default()
        };
        let hex: String = "王小明"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert!(!hex.contains("00"));
        assert_eq!(still_says(&manifest_saying(&hex), &[], &traces).len(), 1);
    }

    #[test]
    fn a_last_look_can_be_stopped_and_says_how_far_it_has_got() {
        let stopped =
            what_a_package_still_says(&manifest_saying("nothing"), &[], &traces(), &|_, _| {
                Err(crate::errors::WinAppMigratorError::Cancelled)
            });
        assert!(matches!(
            stopped,
            Err(crate::errors::WinAppMigratorError::Cancelled)
        ));
        let furthest = std::cell::Cell::new(0);
        let total_said = std::cell::Cell::new(0);
        let written = [("report.md".to_owned(), b"x".to_vec())];
        assert!(
            what_a_package_still_says(
                &manifest_saying("nothing"),
                &written,
                &traces(),
                &|done, total| {
                    furthest.set(done);
                    total_said.set(total);
                    Ok(())
                }
            )
            .is_ok()
        );
        assert_eq!(furthest.get() + 1, total_said.get());
    }

    #[test]
    fn a_name_that_is_a_word_stays_in_a_sentence_and_goes_from_a_path() {
        let traces = AccountTraces {
            account_names: vec!["Will".to_owned(), "acme".to_owned()],
            computer_names: vec!["OFFICE".to_owned()],
            ..AccountTraces::default()
        };
        for text in [
            "the installer will try winget",
            "Microsoft OFFICE is here",
            "Acme.lnk stays",
            "It is called Acme Tools",
        ] {
            assert_eq!(traces.scrub(text), text);
        }
        assert!(!traces.scrub(r"\\OFFICE\share").contains("OFFICE"));
        assert!(
            !traces
                .scrub("The settings of Will could not be read")
                .contains("Will")
        );
        assert!(!traces.scrub("Will's Documents").contains("Will"));
        assert!(!traces.scrub(r"Z:\Will\a.txt").contains("Will"));
    }

    #[test]
    fn a_folder_of_other_softwares_that_could_not_be_read_is_only_counted() {
        let unreadable = |path: &str| {
            Warning::new(
                FOLDER_UNREADABLE_CODE,
                WarningSeverity::Info,
                format!("Could not read {path}"),
                Some(path.to_owned()),
            )
        };
        let mut warnings = vec![
            unreadable(r"C:\ProgramData\SomeoneElse"),
            unreadable(r"C:\ProgramData\AnotherOne"),
            unreadable(&format!(r"{APP}\data")),
        ];
        say_only_how_many_folders_of_others_could_not_be_read(&mut warnings, &[PathBuf::from(APP)]);
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings[0].message.contains(r"Acme\data"));
        assert!(warnings[1].message.contains("2 folders"));
        assert!(!warnings[1].message.contains("SomeoneElse"));
    }

    /// As one a drive's garbage could have: what comes before a colon need not be a letter of a drive.
    #[test]
    fn a_path_with_something_else_for_a_drive_does_not_stop_a_build() {
        let said = traces().scrub(r"é:\Users\bob\x");
        assert!(!said.contains("bob"), "{said}");
    }

    #[test]
    fn a_last_look_is_quiet_about_a_package_that_says_nothing() {
        let quiet = manifest_saying("malice and an admin tool");
        assert!(still_says(&quiet, &[], &traces()).is_empty());
    }

    #[test]
    fn a_last_look_goes_through_the_other_files_when_the_manifest_says_nothing() {
        let quiet = manifest_saying("nothing");
        let in_both = AccountTraces {
            account_names: vec!["alice".to_owned(), "bob".to_owned()],
            ..AccountTraces::default()
        };
        let warnings = still_says(
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
            still_says(&manifest_saying("426f62"), &[], &three).len(),
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
        let warnings = still_says(&quiet, &written, &traces());
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
