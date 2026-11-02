//! What looks like a credential, and leaving it out of a package.
//!
//! An installer can be copied and shared, and an app keeps saved passwords and tokens in the same
//! keys and folders as everything else. A value or file these rules take for a credential is not
//! captured at all, whatever else is asked for, since the License option is about licenses. It is
//! named for the person building, and there is nothing in a package to restore over what the other
//! PC has. So nothing has to be redacted, and nothing here can overwrite a real value there.
//!
//! This goes by names and a few shapes of text. It finds what is called what it is, and no more:
//! a program's own file format, or a value with a name that does not say, is not looked inside.
//! The rules are pure, and false positives are pinned below, since a setting wrongly left out can
//! break an app.

use std::path::Path;

use crate::license_scan::is_license_registry_name;
use crate::models::{
    ComRegistrationEntry, EnvironmentVariableEntry, FileAssociationEntry, FileEntry,
    FirewallRuleEntry, InstalledApp, LeftOutItem, RegistryEntry, RegistryValueKind,
    ScheduledTaskEntry, ServiceEntry, ShortcutEntry,
};
use crate::path_rewrite::escape_xml_text;
use crate::product_match::{MatchScope, ProductNames};
use crate::restore_policy::service_image_path_value;
use crate::system_context::SystemContext;

/// Words that are what they say whatever comes before them.
const SECRET_WORDS: &[&str] = &[
    "apikey",
    "authkey",
    "accesskey",
    "bearer",
    "cookie",
    "cred",
    "credential",
    "credentials",
    "creds",
    "passcode",
    "passphrase",
    "passwd",
    "password",
    "passwords",
    "privatekey",
    "pswd",
    "pwd",
    "secret",
    "secretkey",
    "secrets",
    "token",
    "tokens",
];
/// Two words that mean one of those, such as `API` and `Key`.
const SECRET_WORD_PAIRS: &[(&str, &str)] = &[
    ("access", "key"),
    ("account", "key"),
    ("api", "key"),
    ("auth", "key"),
    ("encryption", "key"),
    ("master", "key"),
    ("private", "key"),
    ("session", "key"),
    ("shared", "key"),
    ("signing", "key"),
    ("ssh", "key"),
];
/// `pass` is only a password after a word that says who or what for. `PassCount` is not one.
const PASS_QUALIFIERS: &[&str] = &[
    "admin", "auth", "db", "ftp", "login", "mail", "mysql", "proxy", "root", "smtp", "sql", "user",
];
/// What can come after one of those words and leave it being what the value holds. Any other word
/// makes it a setting about the credential: `PasswordPolicyVersion`, `TokenCount`, `ApiKeyHeader`.
const STILL_THE_VALUE: &[&str] = &[
    "b64",
    "base64",
    "blob",
    "data",
    "enc",
    "encoded",
    "encrypted",
    "hash",
    "hashed",
    "key",
    "str",
    "string",
    "text",
    "value",
];
/// What can come before one of those words and make it a setting about the credential:
/// `RememberPassword`, `MaxTokens`. Not `Saved`, since `SavedPassword` is the password.
const ABOUT_THE_VALUE: &[&str] = &[
    "allow", "ask", "change", "disable", "enable", "expire", "forgot", "has", "hide", "is", "max",
    "min", "no", "prompt", "remember", "require", "requires", "reset", "save", "show", "use",
];
/// What a name can end in with nothing to show where the words start: `dbpassword`, `accesstoken`.
const RUN_TOGETHER_ENDS: &[&str] = &[
    "accesskey",
    "apikey",
    "passphrase",
    "passwd",
    "password",
    "secret",
    "secretkey",
    "token",
];
/// What makes a password of `pass` with nothing between them: `PassWord`, `pass_phrase`.
const PASS_ENDS: &[&str] = &["word", "words", "phrase", "code"];
const PASS_WORD: &str = "pass";
/// Options that are short for what the rules above go by, which in a value's name could be anything.
const SECRET_OPTIONS: &[&str] = &["pass", "pw"];
/// Names a connection string or a URL's query gives what must not travel.
const CREDENTIAL_FIELDS: &[&str] = &[
    "accesstoken",
    "accountkey",
    "apikey",
    "authtoken",
    "clientsecret",
    "password",
    "passwd",
    "pwd",
    "sharedaccesskey",
    "sharedaccesssignature",
    "token",
];
/// Files that are named for what they hold.
const SECRET_FILE_NAMES: &[&str] = &[
    ".dockercfg",
    ".env",
    ".git-credentials",
    ".htpasswd",
    ".netrc",
    ".npmrc",
    ".pgpass",
    ".pypirc",
    "_netrc",
    "cookies",
    "credentials",
    "credentials.json",
    "credentials.xml",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    "id_rsa",
    "key3.db",
    "key4.db",
    "local state",
    "login data",
    "logins.json",
    "secret.json",
    "secrets.json",
    "secrets.yaml",
    "secrets.yml",
    "token.json",
    "tokens.json",
    "wallet.dat",
    "web data",
];
/// Extensions of what holds a private key or a store of passwords, whatever it is called.
const SECRET_FILE_EXTENSIONS: &[&str] = &["jks", "kdb", "kdbx", "keystore", "p12", "pfx", "ppk"];
/// These can be public, and apps ship them that way: Git and Python each have a `cacert.pem` they
/// cannot work without. So it takes finding a private key in one for it to count.
const KEY_FILE_EXTENSIONS: &[&str] = &["key", "pem"];
const PRIVATE_KEY_MARKER: &[u8] = b"PRIVATE KEY-----";
/// A key file is nowhere near this, and one this size is not read whole for a marker.
pub(crate) const MAX_KEY_FILE_BYTES: u64 = 1024 * 1024;
const ENV_FILE_PREFIX: &str = ".env.";
/// `.env.example` and the like are what a project shows others, not what it uses.
const ENV_FILE_EXAMPLES: &[&str] = &["defaults", "dist", "example", "sample", "template"];
/// What a value holds when its name says password and it holds no password.
const NOT_A_CREDENTIAL: &[&str] = &["0", "1", "false", "no", "none", "true", "yes"];
const DPAPI_PROVIDER_GUID_HEX: &str = "d08c9ddf0115d1118c7a00c04fc297eb";
/// In hex: four bytes of version come before that.
const DPAPI_GUID_OFFSET: usize = 8;
const LEFT_OUT_REASON_SECRET: &str = "it looks like a credential";
const LEFT_OUT_REASON_LICENSE: &str = "the License option is off";
/// A default value is named by its key, but not one under this: `Acme.Token` is a file type.
const NOT_NAMED_BY_THEIR_KEY: &str = r"software\classes";
/// Keys whose names are nobody's license, whatever they are called: a file type or an app's entry
/// in Settings can have that word in its name.
const NOT_LICENSE_KEYS: &[&str] = &[
    r"software\classes",
    r"software\wow6432node\classes",
    r"software\microsoft\windows\currentversion\uninstall",
    r"software\wow6432node\microsoft\windows\currentversion\uninstall",
    r"software\microsoft\windows\currentversion\app paths",
    r"software\wow6432node\microsoft\windows\currentversion\app paths",
    r"system\currentcontrolset\services",
];

/// A value's name, in words: `APIKey` is `api`, `key`, `db_password` is `db`, `password`.
fn words_of(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let characters: Vec<char> = name.chars().collect();
    for (index, character) in characters.iter().enumerate() {
        if !character.is_alphanumeric() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        let previous = index.checked_sub(1).and_then(|at| characters.get(at));
        let next = characters.get(index + 1);
        let starts_a_word = character.is_uppercase()
            && !current.is_empty()
            && (previous.is_some_and(|before| before.is_lowercase() || before.is_numeric())
                || next.is_some_and(|after| after.is_lowercase()));
        if starts_a_word {
            words.push(std::mem::take(&mut current));
        }
        current.extend(character.to_lowercase());
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// The words of a name to go by: a number on the end says nothing, and `pass`, `word` is one word.
fn credential_words(name: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for word in words_of(name) {
        let without_number = word.trim_end_matches(|digit: char| digit.is_ascii_digit());
        let word = if without_number.is_empty() {
            word.as_str()
        } else {
            without_number
        };
        match words.last_mut() {
            Some(last) if last == PASS_WORD && PASS_ENDS.contains(&word) => last.push_str(word),
            _ => words.push(word.to_owned()),
        }
    }
    words
}

/// Whether one word is a name run together, ending in one of those. `SavePassword` in lowercase is
/// still about the password.
fn is_run_together_credential(word: &str) -> bool {
    RUN_TOGETHER_ENDS.iter().any(|end| {
        word.strip_suffix(end)
            .is_some_and(|before| !before.is_empty() && !ABOUT_THE_VALUE.contains(&before))
    })
}

/// Where the part of a name that says it is a credential ends, if it says so at this word.
fn credential_ending_at(words: &[String], index: usize) -> Option<usize> {
    let word = words.get(index)?.as_str();
    if SECRET_WORDS.contains(&word) || is_run_together_credential(word) {
        return Some(index);
    }
    if word == PASS_WORD && index > 0 && PASS_QUALIFIERS.contains(&words[index - 1].as_str()) {
        return Some(index);
    }
    words
        .get(index + 1)
        .filter(|next| SECRET_WORD_PAIRS.contains(&(word, next.as_str())))
        .map(|_| index + 1)
}

/// Whether a value's name says it holds a credential. Each place it could is looked at, since
/// `TokenPassword` is not about the token for being about something after it.
pub fn is_secret_value_name(name: &str) -> bool {
    let words = credential_words(name);
    (0..words.len()).any(|index| {
        let Some(ends_at) = credential_ending_at(&words, index) else {
            return false;
        };
        let about_it = index > 0
            && (ABOUT_THE_VALUE.contains(&words[0].as_str())
                || ABOUT_THE_VALUE.contains(&words[index - 1].as_str()));
        !about_it
            && words[ends_at + 1..]
                .iter()
                .all(|word| STILL_THE_VALUE.contains(&word.as_str()))
    })
}

/// What Windows fills in when it runs a command: `%1`, `%*`, `%L`. It is no one's credential that
/// a protocol's command gives its link to `--token "%1"`.
fn is_a_placeholder(text: &str) -> bool {
    text.trim()
        .trim_matches(['"', '\''])
        .strip_prefix('%')
        .is_some_and(|rest| {
            rest == "*"
                || (!rest.is_empty() && rest.chars().all(|digit| digit.is_ascii_digit()))
                || (rest.len() == 1 && rest.chars().all(|letter| letter.is_ascii_alphabetic()))
        })
}

/// Whether text holds a credential in a shape that says so, whatever the value is called: a
/// connection string with a password in it, or a URL with a user's password or a key.
pub fn value_holds_credentials(text: &str) -> bool {
    url_has_a_password(text)
        || text
            .split([';', '&', '?', ' ', '\t', '\n', '\r'])
            .filter_map(|field| field.split_once('='))
            .any(|(name, value)| {
                let name: String = name
                    .chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .flat_map(char::to_lowercase)
                    .collect();
                let value = value.trim().trim_matches(['"', '\'']);
                CREDENTIAL_FIELDS.contains(&name.as_str())
                    && !value.is_empty()
                    && !is_a_placeholder(value)
                    && !NOT_A_CREDENTIAL.contains(&value.to_ascii_lowercase().as_str())
            })
}

/// `scheme://user:password@host`.
fn url_has_a_password(text: &str) -> bool {
    let mut rest = text;
    while let Some(start) = rest.find("://") {
        rest = &rest[start + 3..];
        let authority = rest
            .split(['/', ' ', '"', '\'', ';', '?', '#'])
            .next()
            .unwrap_or_default();
        if let Some((user, _)) = authority.rsplit_once('@')
            && user
                .split_once(':')
                .is_some_and(|(_, password)| !password.is_empty())
        {
            return true;
        }
    }
    false
}

/// Whether a value's bytes, in hex, are a blob only that account on that PC can read.
pub(crate) fn is_dpapi_hex(hex: &str) -> bool {
    hex.get(DPAPI_GUID_OFFSET..)
        .is_some_and(|rest| rest.starts_with(DPAPI_PROVIDER_GUID_HEX))
}

/// Whether a captured value is, or holds, a credential. A number is not one, and neither is what a
/// flag holds. A blob only DPAPI can read is someone's secret, whatever it is called, and it could
/// not be read on another PC anyway.
pub fn is_secret_registry_value(entry: &RegistryEntry) -> bool {
    if matches!(
        entry.value_kind,
        Some(
            RegistryValueKind::Dword | RegistryValueKind::Qword | RegistryValueKind::DwordBigEndian
        )
    ) {
        return false;
    }
    let text = entry.value_data.as_deref().unwrap_or_default();
    let hex = entry.value_bytes_hex.as_deref().unwrap_or_default();
    let is_binary = entry.value_kind == Some(RegistryValueKind::Binary);
    let holds_nothing = if is_binary {
        hex.is_empty()
    } else {
        text.trim().is_empty()
    };
    let name = entry
        .value_name
        .as_deref()
        .filter(|name| !name.is_empty() && !name.eq_ignore_ascii_case("(Default)"))
        .or_else(|| name_from_key(&entry.key_path));
    let named_for_it = name.is_some_and(is_secret_value_name)
        && !is_a_placeholder(text)
        && !NOT_A_CREDENTIAL.contains(&text.trim().to_ascii_lowercase().as_str());
    let is_text = matches!(
        entry.value_kind,
        None | Some(RegistryValueKind::String | RegistryValueKind::ExpandString)
    );
    !holds_nothing
        && (named_for_it
            || value_holds_credentials(text)
            || (is_text && !without_secret_arguments(text).1.is_empty())
            || (is_binary && is_dpapi_hex(hex)))
}

/// What names a default value: its key, unless that is a type whose name says nothing of what it
/// holds.
fn name_from_key(key_path: &str) -> Option<&str> {
    let under_classes = key_path
        .to_ascii_lowercase()
        .replace(r"wow6432node\", "")
        .starts_with(NOT_NAMED_BY_THEIR_KEY);
    (!under_classes)
        .then(|| key_path.rsplit('\\').next())
        .flatten()
}

/// Whether a file's name says it holds credentials, or a private key. `read_start` gives what it
/// holds, for a kind of file that can go either way.
pub fn is_secret_file(path: &Path, read_start: impl FnOnce() -> Option<Vec<u8>>) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let name = name.to_ascii_lowercase();
    if SECRET_FILE_NAMES.contains(&name.as_str()) {
        return true;
    }
    if let Some(kind) = name.strip_prefix(ENV_FILE_PREFIX) {
        return !ENV_FILE_EXAMPLES.contains(&kind);
    }
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();
    if SECRET_FILE_EXTENSIONS.contains(&extension.as_str()) {
        return true;
    }
    KEY_FILE_EXTENSIONS.contains(&extension.as_str())
        && read_start().is_some_and(|bytes| {
            bytes
                .windows(PRIVATE_KEY_MARKER.len())
                .any(|window| window == PRIVATE_KEY_MARKER)
        })
}

/// A command line without what these rules take for a credential, with the name of each option
/// taken out. It goes by option: `--password=x`, `/token:x`, `--api-key x`, and a connection string
/// or URL as an argument of its own. Quotes hold together, and what is kept stays as it was written.
pub fn without_secret_arguments(arguments: &str) -> (String, Vec<String>) {
    without_options(arguments, &is_secret_option, true)
}

fn is_secret_option(name: &str) -> bool {
    SECRET_OPTIONS.contains(&name.to_ascii_lowercase().as_str()) || is_secret_value_name(name)
}

/// A command line without what is said to be a license, for when the License option is off.
pub(crate) fn without_license_arguments(arguments: &str) -> (String, Vec<String>) {
    without_options(arguments, &is_license_registry_name, false)
}

fn without_options(
    arguments: &str,
    goes: &dyn Fn(&str) -> bool,
    with_credentials: bool,
) -> (String, Vec<String>) {
    let parts = command_line_parts(arguments);
    let holds_credentials = |part: &str| with_credentials && value_holds_credentials(part);
    let mut kept: Vec<&str> = Vec::new();
    let mut taken = Vec::new();
    let mut index = 0;
    while let Some(part) = parts.get(index) {
        index += 1;
        let (name, has_value) = option_name(part);
        if name.is_empty() {
            if holds_credentials(part) {
                taken.push("an argument holding a credential".to_owned());
            } else {
                kept.push(part);
            }
            continue;
        }
        if !goes(name) {
            if holds_credentials(part) {
                taken.push(name.to_owned());
            } else {
                kept.push(part);
            }
            continue;
        }
        // `--password x`. A switch such as `--no-password` is not this, and its name says so.
        let next_is_its_value = !has_value
            && parts
                .get(index)
                .is_some_and(|next| option_name(next).0.is_empty());
        let value_is_filled_in = if has_value {
            part.split_once(['=', ':'])
                .is_some_and(|(_, value)| is_a_placeholder(value))
        } else {
            next_is_its_value && parts.get(index).is_some_and(|next| is_a_placeholder(next))
        };
        if value_is_filled_in {
            kept.push(part);
            continue;
        }
        taken.push(name.to_owned());
        if next_is_its_value {
            index += 1;
        }
    }
    if taken.is_empty() {
        return (arguments.to_owned(), taken);
    }
    (kept.join(" "), taken)
}

/// The words of a command line, each as it was written, with what is in quotes kept as one.
fn command_line_parts(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = None;
    let mut quoted = false;
    for (index, character) in text.char_indices() {
        match character {
            '"' => {
                quoted = !quoted;
                start.get_or_insert(index);
            }
            character if character.is_whitespace() && !quoted => {
                if let Some(from) = start.take() {
                    parts.push(&text[from..index]);
                }
            }
            _ => {
                start.get_or_insert(index);
            }
        }
    }
    if let Some(from) = start {
        parts.push(&text[from..]);
    }
    parts
}

/// An option's name, and whether its value is part of the same word. Empty for what is not one:
/// `/usr/bin/x` starts as one can, and is a path.
fn option_name(part: &str) -> (&str, bool) {
    let trimmed = part.trim_start_matches(['-', '/']);
    if trimmed.len() == part.len() || trimmed.is_empty() {
        return ("", false);
    }
    let end = trimmed.find(['=', ':']);
    let name = end.map_or(trimmed, |at| &trimmed[..at]);
    if name.contains(['/', '\\', '"', ' ']) {
        return ("", false);
    }
    (name, end.is_some())
}

/// The text of a task's `Arguments`, without what `take` takes, with each option taken out named.
/// The document stays as it was but for that, so that what is restored is what was read. `None` for
/// one that will not parse: there is no saying what is in it, and a restore refuses it anyway.
fn task_xml_without_arguments(
    xml: &str,
    take: &dyn Fn(&str) -> (String, Vec<String>),
) -> Option<(String, Vec<String>)> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let mut output = String::with_capacity(xml.len());
    let mut written = 0;
    let mut taken = Vec::new();
    for element in document
        .descendants()
        .filter(|node| node.tag_name().name() == "Arguments")
    {
        for text_node in element.children().filter(roxmltree::Node::is_text) {
            let text = text_node.text().unwrap_or_default();
            let (kept, names) = take(text);
            if names.is_empty() {
                continue;
            }
            let range = text_node.range();
            output.push_str(&xml[written..range.start]);
            output.push_str(&escape_xml_text(&kept));
            written = range.end;
            taken.extend(names);
        }
    }
    output.push_str(&xml[written..]);
    Some((output, taken))
}

/// What a scan leaves out, by name, for the person building to be told.
#[derive(Debug, Clone, Default)]
pub(crate) struct LeftOut {
    pub secrets: Vec<LeftOutItem>,
    pub licenses: Vec<LeftOutItem>,
}

impl LeftOut {
    pub(crate) fn secret(&mut self, name: String) {
        self.secrets.push(LeftOutItem {
            name,
            why: LEFT_OUT_REASON_SECRET.to_owned(),
        });
    }

    pub(crate) fn license(&mut self, name: String) {
        self.licenses.push(LeftOutItem {
            name,
            why: LEFT_OUT_REASON_LICENSE.to_owned(),
        });
    }
}

pub(crate) fn registry_entry_name(entry: &RegistryEntry) -> String {
    let hive = match entry.hive {
        crate::models::RegistryHive::LocalMachine => "HKLM",
        crate::models::RegistryHive::CurrentUser => "HKCU",
    };
    format!(
        r"{hive}\{}\{}",
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    )
}

/// Whether a value under an app's keys is license data: its own name says so, or the name of a key
/// it is under, other than one named for the product itself, or a kind of key that says nothing.
pub(crate) fn is_license_registry_entry(entry: &RegistryEntry, names: &ProductNames) -> bool {
    if entry
        .value_name
        .as_deref()
        .is_some_and(is_license_registry_name)
    {
        return true;
    }
    let lower = entry.key_path.to_ascii_lowercase();
    let says_nothing = NOT_LICENSE_KEYS.iter().any(|key| {
        lower
            .strip_prefix(key)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
    });
    !says_nothing
        && entry.key_path.split('\\').skip(1).any(|segment| {
            is_license_registry_name(segment)
                && !names.matches_top_level(segment, MatchScope::ProgramFolders)
        })
}

/// Everything a scan found that can hold a setting or a command, for taking out what must not go.
pub(crate) struct ScannedSettings<'a> {
    pub registry: &'a mut Vec<RegistryEntry>,
    pub services: &'a mut [ServiceEntry],
    pub tasks: &'a mut [ScheduledTaskEntry],
    pub shortcuts: &'a mut [ShortcutEntry],
    pub environment: &'a mut Vec<EnvironmentVariableEntry>,
    pub file_associations: &'a mut [FileAssociationEntry],
    pub com_registrations: &'a mut [ComRegistrationEntry],
    pub firewall_rules: &'a mut [FirewallRuleEntry],
    pub source_app: Option<&'a mut InstalledApp>,
}

/// One way of taking something out of a command line, and what it goes down as.
struct Taking<'a> {
    take: &'a dyn Fn(&str) -> (String, Vec<String>),
    note: &'a mut dyn FnMut(String),
}

impl Taking<'_> {
    /// `about` says where it was, for a name to go by.
    fn take_from(&mut self, text: &mut String, about: &str) -> bool {
        let (kept, taken) = (self.take)(text);
        for option in &taken {
            (self.note)(format!("{option} in {about}"));
        }
        if taken.is_empty() {
            return false;
        }
        *text = kept;
        true
    }

    fn go_through_commands(&mut self, settings: &mut ScannedSettings<'_>) {
        for service in settings.services.iter_mut() {
            let about = format!("how {} is run", service.service_name);
            if let Some(arguments) = service.arguments.as_mut()
                && self.take_from(arguments, &about)
            {
                // It said the whole command, for someone to run by hand.
                service.restore_command_suggestion = format!(
                    "sc.exe create \"{}\" binPath= {}",
                    service.service_name,
                    service_image_path_value(&service.binary_path, service.arguments.as_deref())
                );
            }
        }
        for task in settings.tasks.iter_mut() {
            let about = format!("how the task {}{} is run", task.task_path, task.task_name);
            if let Some(arguments) = task.arguments.as_mut() {
                self.take_from(arguments, &about);
            }
            if let Some(xml) = task.definition_xml.as_deref() {
                match task_xml_without_arguments(xml, self.take) {
                    Some((kept, taken)) => {
                        for option in taken {
                            (self.note)(format!("{option} in {about}"));
                        }
                        task.definition_xml = Some(kept);
                    }
                    None => {
                        (self.note)(format!(
                            "The definition of the task {}{}, which could not be read to check it",
                            task.task_path, task.task_name
                        ));
                        task.definition_xml = None;
                    }
                }
            }
        }
        for shortcut in settings.shortcuts.iter_mut() {
            let about = format!("the shortcut {}", shortcut.original_lnk_path.display());
            if let Some(arguments) = shortcut.arguments.as_mut() {
                self.take_from(arguments, &about);
            }
        }
        for association in settings.file_associations.iter_mut() {
            let about = format!(
                "what opens {}",
                association
                    .prog_id
                    .as_deref()
                    .or(association.extension.as_deref())
                    .unwrap_or("a file type")
            );
            if let Some(command) = association.command.as_mut() {
                self.take_from(command, &about);
            }
        }
        if let Some(app) = settings.source_app.as_deref_mut()
            && let Some(uninstall) = app.uninstall_string.as_mut()
        {
            let about = format!("how {} is uninstalled", app.display_name);
            self.take_from(uninstall, &about);
        }
    }
}

/// With the License option off, what a scan found that says a license: a service's own settings and
/// what is on a command line. Its values, files, and variables go by themselves.
pub(crate) fn take_licenses_from_settings(
    settings: &mut ScannedSettings<'_>,
    names: &ProductNames,
    left_out: &mut LeftOut,
) {
    for service in settings.services.iter_mut() {
        service.parameters.retain(|entry| {
            let is_a_license = is_license_registry_entry(entry, names);
            if is_a_license {
                left_out.license(registry_entry_name(entry));
            }
            !is_a_license
        });
    }
    let mut note = |name: String| left_out.license(name);
    Taking {
        take: &without_license_arguments,
        note: &mut note,
    }
    .go_through_commands(settings);
}

/// Take what looks like a credential out of what a scan found. It never reaches a package.
pub(crate) fn take_secrets_from_settings(
    settings: &mut ScannedSettings<'_>,
    left_out: &mut LeftOut,
) {
    settings.registry.retain(|entry| {
        let secret = is_secret_registry_value(entry);
        if secret {
            left_out.secret(registry_entry_name(entry));
        }
        !secret
    });
    settings.environment.retain(|variable| {
        let secret =
            is_secret_value_name(&variable.name) || value_holds_credentials(&variable.value);
        if secret {
            left_out.secret(format!("The environment variable {}", variable.name));
        }
        !secret
    });
    for service in settings.services.iter_mut() {
        service.parameters.retain(|entry| {
            let secret = is_secret_registry_value(entry);
            if secret {
                left_out.secret(registry_entry_name(entry));
            }
            !secret
        });
    }
    for class in settings.com_registrations.iter_mut() {
        let holds_it = class
            .server_path
            .as_deref()
            .is_some_and(|path| holds_a_credential(&path.to_string_lossy()));
        if holds_it {
            left_out.secret(format!("What the class {} says it runs", class.clsid));
            class.server_path = None;
        }
    }
    for rule in settings.firewall_rules.iter_mut() {
        if rule.description.as_deref().is_some_and(holds_a_credential) {
            left_out.secret(format!(
                "The description of the firewall rule {}",
                rule.name
            ));
            rule.description = None;
        }
    }
    let mut note = |name: String| left_out.secret(name);
    Taking {
        take: &without_secret_arguments,
        note: &mut note,
    }
    .go_through_commands(settings);
}

/// Whether text that goes in a package holds a credential, by its shape or as a command line.
fn holds_a_credential(text: &str) -> bool {
    value_holds_credentials(text) || !without_secret_arguments(text).1.is_empty()
}

/// Take what looks like a credential out of license values, for when the License option is on: it
/// covers licenses and not credentials.
pub(crate) fn take_secrets_from_license_values(
    values: &mut Vec<RegistryEntry>,
    left_out: &mut LeftOut,
) {
    values.retain(|entry| {
        let secret = is_secret_registry_value(entry);
        if secret {
            left_out.secret(registry_entry_name(entry));
        }
        !secret
    });
}

/// Take credential files out of what is to be copied, whichever of an app's folders they are in.
pub(crate) fn take_secret_files(
    files: &mut Vec<FileEntry>,
    system: &SystemContext,
    left_out: &mut LeftOut,
) {
    files.retain(|file| {
        let secret = is_secret_file(&file.original_path, || read_key_file(file, system));
        if secret {
            left_out.secret(system.logical_path(&file.path).display().to_string());
        }
        !secret
    });
}

fn read_key_file(file: &FileEntry, system: &SystemContext) -> Option<Vec<u8>> {
    if !system.may_read(&file.path) || file.size_bytes.is_none_or(|size| size > MAX_KEY_FILE_BYTES)
    {
        return None;
    }
    std::fs::read(&file.path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{MatchConfidence, RegistryHive};

    #[test]
    fn what_is_called_a_credential_counts_however_it_is_written() {
        for name in [
            "Password",
            "password",
            "PASSWORD",
            "UserPassword",
            "DbPassword",
            "db_password",
            "SMTP-Password",
            "Pwd",
            "PWD",
            "Passwd",
            "ProxyPass",
            "db_pass",
            "Passphrase",
            "ApiKey",
            "APIKey",
            "api_key",
            "x-api-key",
            "AccessKey",
            "SecretKey",
            "PrivateKey",
            "SharedKey",
            "Token",
            "AccessToken",
            "RefreshToken",
            "authToken",
            "oauth_token",
            "ClientSecret",
            "Secret",
            "Credentials",
            "SavedPassword",
            "PasswordHash",
            "EncryptedPassword",
            "PasswordEncrypted",
            "TokenValue",
            "SessionCookie",
            "SigningKey",
            // With nothing to show where the words start.
            "dbpassword",
            "accesstoken",
            "refreshtoken",
            "clientsecret",
            "myapikey",
            // With a number, or the word in two.
            "Password2",
            "password1",
            "Token2",
            "PassWord",
            "pass_word",
            "PASS_PHRASE",
            // Anywhere it says so, whatever else it says first.
            "TokenPassword",
            "PasswordPolicyPassword",
        ] {
            assert!(is_secret_value_name(name), "{name}");
        }
    }

    #[test]
    fn what_is_about_a_credential_is_a_setting() {
        for name in [
            "PasswordPolicyVersion",
            "PasswordLength",
            "PasswordExpiryDays",
            "PasswordRequired",
            "TokenizerPath",
            "Tokenizer",
            "TokenCount",
            "TokenExpiry",
            "TokenUrl",
            "MaxTokens",
            "CredentialProvider",
            "SecretsPath",
            "SecretaryName",
            "ApiKeyHeader",
            "ApiKeyFile",
            "ApiVersion",
            "KeyboardLayout",
            "Key",
            "RememberPassword",
            "UsePassword",
            "ShowPassword",
            "usepassword",
            "rememberpassword",
            "nopassword",
            "Passwordless",
            "Tokens2Used",
            "PassCount",
            "Passthrough",
            "BypassProxy",
            "CompassHeading",
            "Monkey",
            "AccessLevel",
            "CookiePath",
            "CookieConsent",
            "Theme",
            "",
        ] {
            let about = !is_secret_value_name(name);
            assert!(about, "{name}");
        }
    }

    #[test]
    fn a_connection_string_or_a_url_says_it_holds_a_password_by_its_shape() {
        for text in [
            "Server=db;Database=app;User Id=sa;Password=hunter2;",
            "server=db; pwd=hunter2",
            "DSN=App;UID=sa;PWD=hunter2",
            "DefaultEndpointsProtocol=https;AccountName=a;AccountKey=abc==;",
            "Endpoint=sb://x/;SharedAccessKeyName=n;SharedAccessKey=abc",
            "postgres://user:hunter2@host/db",
            "https://user:hunter2@example.com/x",
            "https://example.com/x?token=abc",
            "app.exe --password=hunter2",
        ] {
            assert!(value_holds_credentials(text), "{text}");
        }
        for text in [
            "Server=db;Database=app;Trusted_Connection=yes;",
            "Password=;",
            "https://example.com/x",
            "https://user@example.com/x",
            r"C:\Program Files\App\app.exe --flag",
            "PasswordPolicy=strict",
            "token=1",
            // What Windows gives it when it runs.
            r#""C:\A\a.exe" --token=%1"#,
            "x?token=%1",
            "",
        ] {
            assert!(!value_holds_credentials(text), "{text}");
        }
    }

    fn entry(name: &str, kind: RegistryValueKind, text: &str, hex: &str) -> RegistryEntry {
        RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: r"Software\Acme".to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some(text.to_owned()),
            value_kind: Some(kind),
            value_bytes_hex: Some(hex.to_owned()),
            confidence: MatchConfidence::High,
            requires_admin: true,
        }
    }

    #[test]
    fn a_value_is_a_credential_by_its_name_or_what_it_holds_and_not_for_a_number_or_a_flag() {
        use RegistryValueKind::{Binary, Dword, MultiString, String as Text};
        for secret in [
            entry("Password", Text, "hunter2", ""),
            entry("ApiKey", MultiString, "abc", ""),
            entry("Token", Binary, "", "010203"),
            entry("Server", Text, "Server=db;Password=hunter2;", ""),
            entry(
                "Blob",
                Binary,
                "",
                &format!("01000000{DPAPI_PROVIDER_GUID_HEX}0000"),
            ),
        ] {
            assert!(is_secret_registry_value(&secret), "{secret:?}");
        }
        for not_secret in [
            entry("Password", Text, "", ""),
            entry("Password", Text, "1", ""),
            entry("Token", Text, "true", ""),
            entry("Password", Dword, "1", "01000000"),
            entry("Theme", Text, "dark", ""),
            entry("Blob", Binary, "", "010203"),
            entry("Token", Text, "%1", ""),
            entry("(Default)", Text, r#""C:\A\a.exe" --token "%1""#, ""),
        ] {
            assert!(!is_secret_registry_value(&not_secret), "{not_secret:?}");
        }
        let in_a_command = entry("Run", Text, r#""C:\A\a.exe" --token abc"#, "");
        assert!(
            is_secret_registry_value(&in_a_command),
            "it is not in the shape of a connection string, and it is what the option says"
        );
    }

    #[test]
    fn a_default_value_goes_by_the_name_of_its_key_unless_that_is_a_file_type() {
        let at = |key: &str| RegistryEntry {
            key_path: key.to_owned(),
            value_name: None,
            ..entry("x", RegistryValueKind::String, "hunter2", "")
        };
        assert!(is_secret_registry_value(&at(r"Software\Acme\Password")));
        assert!(is_secret_registry_value(&at(r"Software\Acme\ApiKey")));
        assert!(!is_secret_registry_value(&at(r"Software\Acme\Theme")));
        assert!(!is_secret_registry_value(&at(
            r"Software\Classes\Acme.Token"
        )));
        assert!(!is_secret_registry_value(&at(
            r"Software\WOW6432Node\Classes\Acme.Token"
        )));
    }

    #[test]
    fn a_file_is_a_credential_by_its_name_and_a_key_file_by_what_it_holds() {
        let never_read = || -> Option<Vec<u8>> { panic!("this file need not be read") };
        for name in [
            r"C:\App\.env",
            r"C:\App\.ENV",
            r"C:\App\.env.production",
            r"C:\App\.env.local",
            r"C:\Users\me\.ssh\id_rsa",
            r"C:\Users\me\.ssh\id_ed25519",
            r"C:\App\credentials.json",
            r"C:\App\store.kdbx",
            r"C:\App\cert.pfx",
            r"C:\App\cert.p12",
            r"C:\App\my.ppk",
            r"C:\Users\me\AppData\Local\Vendor\Browser\User Data\Default\Login Data",
            r"C:\App\logins.json",
        ] {
            assert!(is_secret_file(Path::new(name), never_read), "{name}");
        }
        for name in [
            r"C:\App\.env.example",
            r"C:\App\.env.sample",
            r"C:\App\id_rsa.pub",
            r"C:\App\environment.txt",
            r"C:\App\app.exe",
            r"C:\App\settings.json",
            r"C:\App\credentials.txt",
        ] {
            assert!(!is_secret_file(Path::new(name), never_read), "{name}");
        }
        let key = || {
            Some(b"-----BEGIN RSA PRIVATE KEY-----\nabc\n-----END RSA PRIVATE KEY-----\n".to_vec())
        };
        let certificate = || Some(b"-----BEGIN CERTIFICATE-----\nabc\n".to_vec());
        assert!(is_secret_file(Path::new(r"C:\App\server.pem"), key));
        assert!(is_secret_file(Path::new(r"C:\App\server.key"), key));
        // Git and Python each ship one of these, and cannot work without it.
        assert!(!is_secret_file(
            Path::new(r"C:\App\cacert.pem"),
            certificate
        ));
        assert!(!is_secret_file(Path::new(r"C:\App\unread.pem"), || None));
    }

    #[test]
    fn a_credential_in_a_command_line_goes_and_the_rest_stays_as_it_was() {
        for (arguments, kept, taken) in [
            (
                r#"--config "C:\My App\a.conf" --password=hunter2 -v"#,
                r#"--config "C:\My App\a.conf" -v"#,
                vec!["password"],
            ),
            ("--api-key abc --port 80", "--port 80", vec!["api-key"]),
            ("/token:abc /quiet", "/quiet", vec!["token"]),
            ("--pass abc -v", "-v", vec!["pass"]),
            ("-pw abc -v", "-v", vec!["pw"]),
            (r#"--password "two words" -v"#, "-v", vec!["password"]),
            (
                "--db Server=x;Password=y; --run",
                "--db --run",
                vec!["an argument holding a credential"],
            ),
        ] {
            let (left, names) = without_secret_arguments(arguments);
            assert_eq!(left, kept, "{arguments}");
            assert_eq!(names, taken, "{arguments}");
        }
        for unchanged in [
            r#"--config "C:\My App\a.conf""#,
            "--marker service-local",
            "--no-password",
            "--remember-password --port 80",
            r"/c C:\Tools\token.exe",
            // What Windows fills in when it runs this is no one's credential.
            r#"--token "%1""#,
            "--token=%1",
            "/password:%L",
            "",
        ] {
            let (left, names) = without_secret_arguments(unchanged);
            assert_eq!(left, unchanged);
            assert!(names.is_empty(), "{unchanged}: {names:?}");
        }
    }

    #[test]
    fn a_tasks_definition_loses_only_the_credential() {
        let xml = r#"<Task><Actions><Exec><Command>C:\A\a.exe</Command><Arguments>--token=abc --port 80 &amp; more</Arguments></Exec></Actions></Task>"#;
        let (kept, taken) = task_xml(xml).ok_or("it parses").unwrap_or_default();
        assert_eq!(taken, ["token"]);
        assert!(!kept.contains("abc"), "{kept}");
        assert!(kept.contains("--port 80 &amp; more"), "{kept}");
        assert!(roxmltree::Document::parse(&kept).is_ok(), "{kept}");

        let plain = r#"<Task><Actions><Exec><Command>C:\A\a.exe</Command><Arguments>--port 80</Arguments></Exec></Actions></Task>"#;
        assert_eq!(task_xml(plain).map(|kept| kept.0).as_deref(), Some(plain));
        assert_eq!(task_xml("<Task><Arguments>--token=abc"), None);
    }

    fn task_xml(xml: &str) -> Option<(String, Vec<String>)> {
        task_xml_without_arguments(xml, &without_secret_arguments)
    }

    fn service_with(arguments: &str) -> ServiceEntry {
        ServiceEntry {
            service_name: "AcmeSvc".to_owned(),
            display_name: None,
            binary_path: r"C:\A\svc.exe".into(),
            arguments: Some(arguments.to_owned()),
            parameters: vec![entry("LicenseKey", RegistryValueKind::String, "abc", "")],
            start_type: None,
            account_name: None,
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: format!(
                "sc.exe create \"AcmeSvc\" binPath= \"C:\\A\\svc.exe\" {arguments}"
            ),
        }
    }

    fn take_all(
        services: &mut [ServiceEntry],
        associations: &mut [FileAssociationEntry],
        rules: &mut [FirewallRuleEntry],
        app: &mut InstalledApp,
        licenses_too: bool,
    ) -> LeftOut {
        let names = ProductNames::new(Some("Acme"), None, &[], &[]);
        let mut left_out = LeftOut::default();
        let (mut registry, mut environment) = (Vec::new(), Vec::new());
        let mut settings = ScannedSettings {
            registry: &mut registry,
            services,
            tasks: &mut [],
            shortcuts: &mut [],
            environment: &mut environment,
            file_associations: associations,
            com_registrations: &mut [],
            firewall_rules: rules,
            source_app: Some(app),
        };
        if licenses_too {
            take_licenses_from_settings(&mut settings, &names, &mut left_out);
        }
        take_secrets_from_settings(&mut settings, &mut left_out);
        left_out
    }

    fn association_running(command: &str) -> FileAssociationEntry {
        FileAssociationEntry {
            hive: RegistryHive::LocalMachine,
            extension: None,
            prog_id: Some("Acme.File".to_owned()),
            application_name: None,
            verb: Some("open".to_owned()),
            command: Some(command.to_owned()),
            confidence: MatchConfidence::High,
            requires_admin: true,
        }
    }

    fn app_uninstalled_by(command: &str) -> InstalledApp {
        InstalledApp {
            display_name: "Acme".to_owned(),
            publisher: None,
            version: None,
            install_location: None,
            uninstall_string: Some(command.to_owned()),
            main_executable: None,
            registry_source_path: String::new(),
            architecture: crate::models::AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        }
    }

    #[test]
    fn a_credential_goes_from_everywhere_that_says_a_command() {
        const SECRET: &str = "hunter2";
        let mut services = [service_with(&format!("--password={SECRET} --port 80"))];
        let mut associations = [association_running(&format!(
            r#""C:\A\a.exe" --api-key {SECRET} "%1""#
        ))];
        let mut rules = [FirewallRuleEntry {
            name: "Acme".to_owned(),
            description: Some(format!("https://me:{SECRET}@example.com")),
            direction: "In".to_owned(),
            action: "Allow".to_owned(),
            enabled: true,
            program: r"C:\A\a.exe".into(),
            service: None,
            protocol: None,
            local_ports: Vec::new(),
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: Vec::new(),
            profiles: Vec::new(),
            not_carried: Vec::new(),
        }];
        let mut app = app_uninstalled_by(&format!(r#""C:\A\un.exe" --token {SECRET} /S"#));
        let left_out = take_all(
            &mut services,
            &mut associations,
            &mut rules,
            &mut app,
            false,
        );

        assert_eq!(services[0].arguments.as_deref(), Some("--port 80"));
        assert!(services[0].restore_command_suggestion.contains("--port 80"));
        assert!(!services[0].restore_command_suggestion.contains(SECRET));
        assert!(
            !associations[0]
                .command
                .as_deref()
                .unwrap_or_default()
                .contains(SECRET)
        );
        assert_eq!(rules[0].description, None);
        assert!(
            !app.uninstall_string
                .as_deref()
                .unwrap_or_default()
                .contains(SECRET)
        );
        assert_eq!(left_out.secrets.len(), 4, "{:?}", left_out.secrets);
        assert!(
            left_out
                .secrets
                .iter()
                .all(|item| !item.name.contains(SECRET))
        );
    }

    #[test]
    fn with_the_license_option_off_a_service_keeps_nothing_that_says_a_license() {
        let mut services = [service_with("--license=abc --port 80")];
        let mut app = app_uninstalled_by(r#""C:\A\un.exe""#);
        let left_out = take_all(&mut services, &mut [], &mut [], &mut app, true);
        assert_eq!(services[0].arguments.as_deref(), Some("--port 80"));
        assert!(!services[0].restore_command_suggestion.contains("abc"));
        assert!(services[0].parameters.is_empty());
        assert_eq!(left_out.licenses.len(), 2, "{:?}", left_out.licenses);

        let mut services = [service_with("--license=abc --port 80")];
        let left_out = take_all(&mut services, &mut [], &mut [], &mut app, false);
        assert_eq!(
            services[0].arguments.as_deref(),
            Some("--license=abc --port 80"),
            "with it on, that is what was asked for"
        );
        assert_eq!(services[0].parameters.len(), 1);
        assert!(left_out.licenses.is_empty());
    }

    #[test]
    fn a_key_named_for_a_license_is_license_data_unless_it_is_the_products_own_name() {
        let names = ProductNames::new(Some("License Manager Pro"), None, &[], &[]);
        let at = |key: &str, name: &str| RegistryEntry {
            key_path: key.to_owned(),
            ..entry(name, RegistryValueKind::String, "x", "")
        };
        for license in [
            at(r"Software\Acme\App", "LicenseKey"),
            at(r"Software\Acme\App", "SerialNumber"),
            at(r"Software\Acme\App\License", "Key"),
            at(r"Software\Acme\App\Activation", "Code"),
        ] {
            assert!(is_license_registry_entry(&license, &names), "{license:?}");
        }
        for not_license in [
            at(r"Software\Acme\App", "Theme"),
            at(r"Software\Acme\License Manager Pro", "Theme"),
            at(r"Software\Classes\Licensing.File\shell\open\command", "x"),
            at(
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall\License Tool",
                "DisplayName",
            ),
            at(r"Software\Acme\App", "SerialPort"),
            at(r"Software\Acme\App", "Serializer"),
        ] {
            assert!(
                !is_license_registry_entry(&not_license, &names),
                "{not_license:?}"
            );
        }
    }
}
