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
    EnvironmentVariableEntry, FileEntry, LeftOutItem, RegistryEntry, RegistryValueKind,
    ScheduledTaskEntry, ServiceEntry, ShortcutEntry,
};
use crate::path_rewrite::escape_xml_text;
use crate::product_match::{MatchScope, ProductNames};
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
const MAX_KEY_FILE_BYTES: u64 = 1024 * 1024;
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

/// Whether a value's name says it holds a credential.
pub fn is_secret_value_name(name: &str) -> bool {
    let words = words_of(name);
    let last_of_it = words.iter().enumerate().find_map(|(index, word)| {
        if SECRET_WORDS.contains(&word.as_str()) {
            return Some(index);
        }
        let next = words.get(index + 1).map(String::as_str);
        if word == "pass" && index > 0 && PASS_QUALIFIERS.contains(&words[index - 1].as_str()) {
            return Some(index);
        }
        next.filter(|next| SECRET_WORD_PAIRS.contains(&(word.as_str(), *next)))
            .map(|_| index + 1)
    });
    let Some(last_of_it) = last_of_it else {
        return false;
    };
    let about_it = last_of_it > 0 && ABOUT_THE_VALUE.contains(&words[0].as_str());
    !about_it
        && words[last_of_it + 1..]
            .iter()
            .all(|word| STILL_THE_VALUE.contains(&word.as_str()))
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
    let named_for_it = entry
        .value_name
        .as_deref()
        .is_some_and(is_secret_value_name)
        && !NOT_A_CREDENTIAL.contains(&text.trim().to_ascii_lowercase().as_str());
    !holds_nothing
        && (named_for_it || value_holds_credentials(text) || (is_binary && is_dpapi_hex(hex)))
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
    let parts = command_line_parts(arguments);
    let mut kept: Vec<&str> = Vec::new();
    let mut taken = Vec::new();
    let mut index = 0;
    while let Some(part) = parts.get(index) {
        index += 1;
        let (name, has_value) = option_name(part);
        if name.is_empty() {
            if value_holds_credentials(part) {
                taken.push("an argument holding a credential".to_owned());
            } else {
                kept.push(part);
            }
            continue;
        }
        if !is_secret_value_name(name) {
            if value_holds_credentials(part) {
                taken.push(name.to_owned());
            } else {
                kept.push(part);
            }
            continue;
        }
        taken.push(name.to_owned());
        // `--password x`. A switch such as `--no-password` is not this, and its name says so.
        if !has_value
            && parts
                .get(index)
                .is_some_and(|next| option_name(next).0.is_empty())
        {
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

/// The text of a task's `Arguments`, without credentials, with each option taken out named. The
/// document stays as it was but for that, so that what is restored is what was read. One that will
/// not parse is left as it is, since a restore refuses it anyway.
fn task_xml_without_secret_arguments(xml: &str) -> (String, Vec<String>) {
    let Ok(document) = roxmltree::Document::parse(xml) else {
        return (xml.to_owned(), Vec::new());
    };
    let mut output = String::with_capacity(xml.len());
    let mut written = 0;
    let mut taken = Vec::new();
    for element in document
        .descendants()
        .filter(|node| node.tag_name().name() == "Arguments")
    {
        for text_node in element.children().filter(roxmltree::Node::is_text) {
            let text = text_node.text().unwrap_or_default();
            let (kept, names) = without_secret_arguments(text);
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
    (output, taken)
}

/// What a scan leaves out, by name, for the person building to be told.
#[derive(Debug, Clone, Default)]
pub(crate) struct LeftOut {
    pub secrets: Vec<LeftOutItem>,
    pub licenses: Vec<LeftOutItem>,
}

impl LeftOut {
    fn secret(&mut self, name: String) {
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

/// Take what looks like a credential out of what a scan found. It never reaches a package.
pub(crate) fn take_secrets_from_settings(
    registry: &mut Vec<RegistryEntry>,
    services: &mut [ServiceEntry],
    tasks: &mut [ScheduledTaskEntry],
    shortcuts: &mut [ShortcutEntry],
    environment: &mut Vec<EnvironmentVariableEntry>,
    left_out: &mut LeftOut,
) {
    registry.retain(|entry| {
        let secret = is_secret_registry_value(entry);
        if secret {
            left_out.secret(registry_entry_name(entry));
        }
        !secret
    });
    environment.retain(|variable| {
        let secret =
            is_secret_value_name(&variable.name) || value_holds_credentials(&variable.value);
        if secret {
            left_out.secret(format!("The environment variable {}", variable.name));
        }
        !secret
    });
    for service in services {
        service.parameters.retain(|entry| {
            let secret = is_secret_registry_value(entry);
            if secret {
                left_out.secret(registry_entry_name(entry));
            }
            !secret
        });
        if let Some(arguments) = service.arguments.as_deref() {
            let (kept, taken) = without_secret_arguments(arguments);
            for option in taken {
                left_out.secret(format!("{option} in how {} is run", service.service_name));
            }
            service.arguments = Some(kept);
        }
    }
    for task in tasks {
        if let Some(arguments) = task.arguments.as_deref() {
            task.arguments = Some(without_secret_arguments(arguments).0);
        }
        if let Some(xml) = task.definition_xml.as_deref() {
            let (kept, taken) = task_xml_without_secret_arguments(xml);
            for option in taken {
                left_out.secret(format!(
                    "{option} in how the task {}{} is run",
                    task.task_path, task.task_name
                ));
            }
            task.definition_xml = Some(kept);
        }
    }
    for shortcut in shortcuts {
        if let Some(arguments) = shortcut.arguments.as_deref() {
            let (kept, taken) = without_secret_arguments(arguments);
            for option in taken {
                left_out.secret(format!(
                    "{option} in the shortcut {}",
                    shortcut.original_lnk_path.display()
                ));
            }
            shortcut.arguments = Some(kept);
        }
    }
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
        ] {
            assert!(!is_secret_registry_value(&not_secret), "{not_secret:?}");
        }
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
        let (kept, taken) = task_xml_without_secret_arguments(xml);
        assert_eq!(taken, ["token"]);
        assert!(!kept.contains("abc"), "{kept}");
        assert!(kept.contains("--port 80 &amp; more"), "{kept}");
        assert!(roxmltree::Document::parse(&kept).is_ok(), "{kept}");

        let plain = r#"<Task><Actions><Exec><Command>C:\A\a.exe</Command><Arguments>--port 80</Arguments></Exec></Actions></Task>"#;
        assert_eq!(task_xml_without_secret_arguments(plain).0, plain);
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
