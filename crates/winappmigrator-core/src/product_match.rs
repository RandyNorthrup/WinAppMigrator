//! Decides from a folder or registry key name whether it belongs to an application.
//!
//! Discovery finds most of an app from records (install logs, the MSI database, App
//! Paths). Names fill in what nothing records, mostly data folders created at run time,
//! so a loose match here copies other products. Matching is therefore on whole names at
//! a segment boundary, never on substrings: `Git` is not `Logitech` or `GitHub`, and
//! `Microsoft Visual Studio Code` is not `Microsoft Visual Studio`.
//!
//! A publisher never selects anything on its own. It can only qualify the parent folder
//! of a product match, as in `Mozilla\Firefox`.
//!
//! [`ProductNames::near_match`] says what is close to a name without being it. That takes nothing
//! by itself: it is only ever offered, and goes by the same rules as anything else once it is ticked.

use crate::path_utils::normalize_name_for_match;

const MIN_NAME_LEN: usize = 3;
/// A name this short is too likely to be someone's own folder under `Documents`.
const MIN_USER_FOLDER_NAME_LEN: usize = 4;
/// Two names that both start with the same word this long are treated as one vendor, so
/// `ASUS` qualifies for `ASUSTeK Computer Inc.`. Shorter ones must be equal: `Git` is not
/// `GitHub`.
const MIN_VENDOR_PREFIX_LEN: usize = 4;
/// A folder can be called the start of a vendor's name, as `ASUS` is of `ASUSTeK` and `Logi` of
/// `Logitech`, if it is this long. Not the other way round, where a vendor's name starts a longer word:
/// `Intel` of `IntelliJ`, `Micro` of `Microsoft`.
const SHORTEST_VENDOR_WORD_TO_GO_BY_THE_START_OF: usize = 4;
/// A multi-word product name this long is specific enough to match under any parent.
const MIN_ANY_PARENT_NAME_LEN: usize = 6;
/// An executable's or folder's one-word name this short is a word anyone could use, as `Code` is, so
/// a folder with that word in its name is nothing like the product.
const MIN_NEAR_ALIAS_LEN: usize = 5;
/// .NET keeps a program's settings in a folder named `<Company>\<Program>.exe_Url_<hash>`, with
/// nothing about the company to go by.
const DOT_NET_SETTINGS_WORD: &str = "exe";
/// Publishers with so much on a PC that being theirs says nothing. Windows itself is one.
const PUBLISHERS_OF_EVERYTHING: &[&str] = &["microsoft", "windows"];

/// Names too generic to identify an application: a folder or executable called this says
/// nothing about which product it belongs to.
const GENERIC_NAMES: &[&str] = &[
    "bin",
    "bin64",
    "lib",
    "app",
    "apps",
    "application",
    "applications",
    "data",
    "core",
    "src",
    "etc",
    "share",
    "shared",
    "usr",
    "var",
    "cmd",
    "exe",
    "common",
    "resources",
    "plugins",
    "tools",
    "program",
    "programs",
    "programfiles",
    "programfilesx86",
    "programfilesx64",
    "programdata",
    "files",
    "x86",
    "x64",
    "win32",
    "win64",
    "amd64",
    "windows",
    "microsoft",
    "inc",
    "llc",
    "ltd",
    "corp",
    "corporation",
    "software",
    "company",
    "limited",
    "client",
    "server",
    "launcher",
    "main",
    "start",
    "run",
    "service",
    "update",
    "updater",
    "install",
    "installer",
    "setup",
    "uninstall",
    "uninstaller",
    "helper",
    "game",
    "games",
    "system",
    "config",
    "temp",
    "tmp",
    "cache",
    "logs",
    "docs",
    "user",
    "users",
    "default",
    "local",
    "public",
];
/// Skipped when looking for the word that identifies a publisher.
const PUBLISHER_NOISE_WORDS: &[&str] = &["the"];
/// Words that add nothing to a vendor's name: `Adobe Systems` is still just Adobe.
const CORPORATE_WORDS: &[&str] = &[
    "co",
    "company",
    "computer",
    "computers",
    "corp",
    "corporation",
    "gmbh",
    "group",
    "inc",
    "incorporated",
    "labs",
    "limited",
    "llc",
    "ltd",
    "software",
    "systems",
    "technologies",
    "technology",
];
/// Folders that hold unrelated products side by side, so they say nothing about the
/// child's vendor but do not rule it out either.
const CONTAINER_FOLDERS: &[&str] = &["programs", "my games"];
const ARCHITECTURE_WORDS: &[&str] = &[
    "x64", "x86", "arm64", "amd64", "win64", "win32", "64bit", "32bit",
];

/// Where a name match is being applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MatchScope {
    /// `Program Files`, `ProgramData`, `AppData`, and the registry: folders that exist
    /// for applications, where an executable's name is a fair signal.
    ProgramFolders,
    /// `Documents` and `Saved Games`: folders a person names. Only the product's own
    /// name counts, so `Code.exe` does not take `Documents\Code`.
    UserFolders,
}

/// The names one application goes by, normalized for comparison.
#[derive(Debug, Clone, Default)]
pub(crate) struct ProductNames {
    /// The display name, without and with its version: `solarfire`, `solarfire9`.
    full: Vec<String>,
    /// The members of `full` specific enough to match under any parent folder.
    any_parent: Vec<String>,
    /// The display name without its publisher, which only counts under a vendor folder:
    /// `firefox` from `Mozilla Firefox`, `cli` from `GitHub CLI`.
    short: Vec<String>,
    /// The executable and install-folder names. `AppData\Roaming\Code` is `Code.exe`.
    aliases: Vec<String>,
    /// What the vendor's folders are called. Only ever a qualifier.
    vendors: Vec<String>,
    /// The product's names as words, for what is close to one without being it.
    near_names: Vec<Vec<String>>,
    /// What the publisher is called, as words, without what adds nothing to a vendor's name.
    publisher_words: Vec<String>,
}

/// What makes a folder's name close to a product's without being it. In the order of how much that
/// says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NearName {
    /// `Stream Deck Plugins` for `Stream Deck`.
    StartsWithProduct,
    /// `Elgato Stream Deck` for `Stream Deck`.
    HasProductInIt,
    /// `Mozilla-1de4eec8` for a Mozilla app, with the publisher's name as it is written there.
    StartsWithPublisher(String),
}

impl ProductNames {
    pub(crate) fn new(
        display_name: Option<&str>,
        publisher: Option<&str>,
        aliases: &[&str],
        vendor_folders: &[&str],
    ) -> Self {
        let mut names = Self::default();
        if let Some(publisher) = publisher {
            names.publisher_words = vendor_words(publisher);
            push_unique(&mut names.vendors, normalize_name_for_match(publisher));
            if let Some(word) = significant_words(publisher).next() {
                push_unique(&mut names.vendors, word);
            }
        }
        for folder in vendor_folders {
            let normalized = normalize_name_for_match(folder);
            if is_usable_name(&normalized) {
                push_unique(&mut names.vendors, normalized);
            }
        }
        if let Some(display_name) = display_name {
            names.add_display_name(display_name);
        }
        for alias in aliases {
            let primary = primary_name(alias);
            let normalized = normalize_name_for_match(&primary);
            if is_usable_name(&normalized) {
                push_unique(&mut names.aliases, normalized.clone());
                let words = camel_case_words(&primary);
                if words.len() > 1 || normalized.len() >= MIN_NEAR_ALIAS_LEN {
                    names.add_near_name(words);
                }
            }
        }
        names
    }

    fn add_near_name(&mut self, words: Vec<String>) {
        if !words.is_empty() && !self.near_names.contains(&words) {
            self.near_names.push(words);
        }
    }

    fn add_display_name(&mut self, display_name: &str) {
        let primary = primary_name(display_name);
        let normalized = normalize_name_for_match(&primary);
        let words: Vec<&str> = primary.split_whitespace().collect();
        if is_usable_name(&normalized) {
            if words.len() > 1 && normalized.len() >= MIN_ANY_PARENT_NAME_LEN {
                self.any_parent.push(normalized.clone());
            }
            self.full.push(normalized);
            self.add_near_name(camel_case_words(&primary));
        }
        let with_version =
            normalize_name_for_match(display_name.split('(').next().unwrap_or(display_name));
        if is_usable_name(&with_version) {
            push_unique(&mut self.full, with_version);
        }
        if let Some((first, rest)) = words.split_first()
            && !rest.is_empty()
            && self.is_vendor_name(first)
        {
            let short = normalize_name_for_match(&rest.join(" "));
            if is_usable_name(&short) {
                // One word by itself is a word anyone could use: `Studio` from `OBS Studio`.
                if rest.len() > 1 {
                    self.add_near_name(camel_case_words(&rest.join(" ")));
                }
                self.short.push(short);
            }
        }
    }

    /// True when a folder directly under a base (`base\Name`) is this product.
    pub(crate) fn matches_top_level(&self, folder: &str, scope: MatchScope) -> bool {
        let keys = name_keys(folder);
        match scope {
            MatchScope::ProgramFolders => {
                has_any(&keys, &self.full) || has_any(&keys, &self.aliases)
            }
            MatchScope::UserFolders => keys
                .iter()
                .any(|key| key.len() >= MIN_USER_FOLDER_NAME_LEN && self.full.contains(key)),
        }
    }

    /// True when a folder one level down (`base\Parent\Name`) is this product: the name
    /// spelled across the two segments (`Mozilla\Firefox`), a product name under a folder
    /// that qualifies as its vendor or as a plain container (`Apple Computer\iTunes`,
    /// `Programs\Git`), or a multi-word product name under any parent.
    pub(crate) fn matches_nested(&self, parent: &str, folder: &str, scope: MatchScope) -> bool {
        let keys = name_keys(folder);
        if keys.is_empty() {
            return false;
        }
        if self.spells_out_product(parent, folder) {
            return true;
        }
        if !self.is_vendor_name(parent) && !is_container_folder(parent) {
            return false;
        }
        has_any(&keys, &self.full)
            || has_any(&keys, &self.short)
            || (scope == MatchScope::ProgramFolders && has_any(&keys, &self.aliases))
    }

    /// The strict half of `matches_nested`: the two segments spell the product's name, or
    /// the child alone is a multi-word product name. For a parent that holds too much to
    /// qualify anything, like `Software\Microsoft`, where `PowerShell` is Windows' own.
    pub(crate) fn spells_out_product(&self, parent: &str, folder: &str) -> bool {
        let keys = name_keys(folder);
        if has_any(&keys, &self.any_parent) {
            return true;
        }
        name_keys_any(parent).iter().any(|parent_key| {
            keys.iter()
                .any(|key| self.full.contains(&format!("{parent_key}{key}")))
        })
    }

    /// True when a name carries a version that this product's display name starts with:
    /// `Python 3.14 (64-bit)` for `Python 3.14.10 (64-bit)`, but not `Python 3.13`.
    pub(crate) fn shares_version(&self, name: &str) -> bool {
        let versioned = normalize_name_for_match(name.split('(').next().unwrap_or(name));
        versioned.ends_with(|c: char| c.is_ascii_digit())
            && self.full.iter().any(|full| full.starts_with(&versioned))
    }

    /// True when an executable or shortcut with this name is the product itself, not
    /// merely something that ships with it.
    pub(crate) fn names_executable(&self, stem: &str) -> bool {
        let keys = name_keys(stem);
        has_any(&keys, &self.full) || has_any(&keys, &self.short) || has_any(&keys, &self.aliases)
    }

    /// True for a folder named after the vendor and nothing else, like `NVIDIA Corporation`
    /// or `Acme`: it may hold the vendor's other products too, so it cannot be claimed
    /// whole. `Microsoft VS Code` starts with a vendor's name but is a product's folder.
    pub(crate) fn is_vendor_only_folder(&self, folder: &str) -> bool {
        if self.matches_top_level(folder, MatchScope::ProgramFolders)
            || has_any(&name_keys(folder), &self.short)
        {
            return false;
        }
        if self.vendors.contains(&normalize_name_for_match(folder)) {
            return true;
        }
        let mut words = significant_words(folder);
        words
            .next()
            .is_some_and(|first| self.is_vendor_word(&first))
            && words.all(|word| CORPORATE_WORDS.contains(&word.as_str()))
    }

    /// Whether a folder's name is close to this product's without being it, and how. That is by
    /// whole words, so `Git` is near `Git LFS` and not `GitHub`, and only ever a reason to ask. A
    /// publisher's name only counts directly in a folder that holds programs or data, all of it, and
    /// for a publisher that is not on everything. See [`PUBLISHERS_OF_EVERYTHING`].
    pub(crate) fn near_match(&self, parent: Option<&str>, folder: &str) -> Option<NearName> {
        if is_generic_name(folder) || self.is_exactly(parent, folder) {
            return None;
        }
        let plain = plain_words(folder);
        let camel_case = camel_case_words(folder);
        // What is under some other folder is that folder's, unless that folder is nothing more than
        // a place to keep things, or the rest of what it is called says more.
        let parent_says_nothing =
            parent.is_none_or(|parent| self.is_vendor_name(parent) || is_container_folder(parent));
        for name in &self.near_names {
            let words = if name.len() > 1 { &camel_case } else { &plain };
            if words.len() > name.len()
                && words.starts_with(name)
                && (parent_says_nothing
                    || name.len() > 1
                    || words
                        .get(name.len())
                        .is_some_and(|next| next == DOT_NET_SETTINGS_WORD))
            {
                return Some(NearName::StartsWithProduct);
            }
        }
        if parent_says_nothing {
            for name in &self.near_names {
                let words = if name.len() > 1 { &camel_case } else { &plain };
                let is_specific = name.len() > 1
                    || name
                        .first()
                        .is_some_and(|word| word.len() >= MIN_ANY_PARENT_NAME_LEN);
                if is_specific && words.windows(name.len()).any(|run| run == name.as_slice()) {
                    return Some(NearName::HasProductInIt);
                }
            }
        }
        let publisher = &self.publisher_words;
        let first = publisher.first()?;
        (parent.is_none()
            && publisher.concat().len() >= MIN_VENDOR_PREFIX_LEN
            && !is_publisher_of_everything(first)
            && plain.len() > publisher.len()
            && plain.starts_with(publisher))
        .then(|| {
            let written: Vec<&str> = folder
                .split(|c: char| !c.is_alphanumeric())
                .filter(|word| !word.is_empty())
                .take(publisher.len())
                .collect();
            NearName::StartsWithPublisher(written.join(" "))
        })
    }

    fn is_exactly(&self, parent: Option<&str>, folder: &str) -> bool {
        self.matches_top_level(folder, MatchScope::ProgramFolders)
            || parent.is_some_and(|parent| {
                self.matches_nested(parent, folder, MatchScope::ProgramFolders)
            })
    }

    /// True for a folder named for the publisher and nothing else, by its whole words, unlike
    /// [`Self::is_vendor_only_folder`], which has to allow for how a registry key is written.
    /// `Mozilla-1de4eec8` is not, and `NVIDIA Corporation` is.
    pub(crate) fn is_named_only_for_publisher(&self, folder: &str) -> bool {
        if self.vendors.contains(&normalize_name_for_match(folder)) {
            return true;
        }
        let words = vendor_words(folder);
        match words.as_slice() {
            [] => false,
            [one] => self.is_vendor_word(one),
            _ => words == self.publisher_words,
        }
    }

    /// Whether this is who published something, by all of what they are called, and whether that is
    /// a publisher to go by at all.
    pub(crate) fn has_publisher_like(&self, publisher: &str) -> bool {
        let Some(first) = self.publisher_words.first() else {
            return false;
        };
        !is_publisher_of_everything(first) && vendor_words(publisher) == self.publisher_words
    }

    /// Whether something can be taken to be a part of an app by what it is called, given what the
    /// part says of itself is only what it says, and it could say anything. That takes it being under
    /// all of what the part's own publisher is called, or starting with all of that and going on, and
    /// no publisher on everything says anything. A folder a part is in says nothing of who published
    /// it. `parent` is what it is in, if that is not the top of where to look.
    pub(crate) fn is_a_parts_own_name(&self, parent: Option<&str>, name: &str) -> bool {
        let publisher = &self.publisher_words;
        if publisher
            .first()
            .is_none_or(|first| is_publisher_of_everything(first))
        {
            return false;
        }
        match parent {
            Some(parent) => vendor_words(parent) == *publisher,
            None => {
                let words = vendor_words(name);
                words.len() > publisher.len() && words.starts_with(publisher)
            }
        }
    }

    fn is_vendor_name(&self, folder: &str) -> bool {
        self.vendors.contains(&normalize_name_for_match(folder))
            || significant_words(folder)
                .next()
                .is_some_and(|first| self.is_vendor_word(&first))
    }

    fn is_vendor_word(&self, word: &str) -> bool {
        self.vendors.iter().any(|vendor| {
            vendor == word
                || (word.len() >= SHORTEST_VENDOR_WORD_TO_GO_BY_THE_START_OF
                    && !is_publisher_of_everything(word)
                    && !is_publisher_of_everything(vendor)
                    && vendor.starts_with(word))
        })
    }
}

/// Reduce a display name to its core product name by dropping parenthetical qualifiers
/// and trailing version and architecture words: `7-Zip 25.01 (x64)` is `7-Zip`, and
/// `PowerShell 7.6.9.0-x64` is `PowerShell`. A name made only of such words, like the
/// game `2048`, is kept whole.
pub(crate) fn primary_name(display_name: &str) -> String {
    let base = display_name.split('(').next().unwrap_or(display_name);
    let kept = base
        .split_whitespace()
        .filter(|word| !is_version_word(word))
        .collect::<Vec<_>>()
        .join(" ");
    if kept.is_empty() {
        base.trim().to_owned()
    } else {
        kept
    }
}

/// True for a word that names a version or an architecture and not a product: `25.01`,
/// `v1.2`, `7-x64`, `64-bit`, a lone `-`.
fn is_version_word(word: &str) -> bool {
    let lower = word.to_ascii_lowercase();
    let trimmed = lower.trim_matches(['-', ',', '.']);
    if trimmed.is_empty() {
        return true;
    }
    let compact: String = trimmed.chars().filter(|c| *c != '-').collect();
    if ARCHITECTURE_WORDS.contains(&compact.as_str()) {
        return true;
    }
    let body = match trimmed.strip_prefix('v') {
        Some(rest) if rest.starts_with(|c: char| c.is_ascii_digit()) => rest,
        _ => trimmed,
    };
    let core = ARCHITECTURE_WORDS
        .iter()
        .find_map(|architecture| body.strip_suffix(architecture))
        .unwrap_or(body);
    core.starts_with(|c: char| c.is_ascii_digit())
        && core
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '.' | ',' | '-'))
}

/// True when a normalized name is long enough and specific enough to identify a product.
fn is_usable_name(normalized: &str) -> bool {
    normalized.len() >= MIN_NAME_LEN && !is_generic_name(normalized)
}

/// True for a name that says nothing about which product owns it, such as `bin`.
pub(crate) fn is_generic_name(name: &str) -> bool {
    GENERIC_NAMES.contains(&normalize_name_for_match(name).as_str())
}

/// What a vendor is called, as words, without the ones that add nothing to its name.
fn vendor_words(name: &str) -> Vec<String> {
    plain_words(name)
        .into_iter()
        .filter(|word| {
            !CORPORATE_WORDS.contains(&word.as_str())
                && !PUBLISHER_NOISE_WORDS.contains(&word.as_str())
        })
        .collect()
}

fn is_publisher_of_everything(word: &str) -> bool {
    PUBLISHERS_OF_EVERYTHING
        .iter()
        .any(|publisher| word.starts_with(publisher))
}

/// A name's words, split where something other than a letter or a digit is, lowercase.
fn plain_words(name: &str) -> Vec<String> {
    name.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// A name's words, split where [`plain_words`] does and also where a capital starts one, so
/// `StreamDeck` is two words. Only for a name of more than one word: `GitHub` is not `Git`.
fn camel_case_words(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in name.split(|c: char| !c.is_alphanumeric()) {
        let mut current = String::new();
        let mut last_was_lower = false;
        for character in word.chars() {
            if character.is_uppercase() && last_was_lower {
                words.push(std::mem::take(&mut current));
            }
            last_was_lower = character.is_lowercase() || character.is_numeric();
            current.extend(character.to_lowercase());
        }
        if !current.is_empty() {
            words.push(current);
        }
    }
    words
}

fn is_container_folder(folder: &str) -> bool {
    CONTAINER_FOLDERS.contains(&folder.trim().to_ascii_lowercase().as_str())
}

/// The forms a folder name can match under, ignoring any generic one: as written, without
/// separated version words (`Adobe Photoshop 2024`), and without trailing digits
/// (`Python311`, `Synapse3`).
fn name_keys(name: &str) -> Vec<String> {
    let mut keys = name_keys_any(name);
    keys.retain(|key| is_usable_name(key));
    keys
}

fn name_keys_any(name: &str) -> Vec<String> {
    let mut keys = Vec::new();
    push_unique(&mut keys, normalize_name_for_match(name));
    let primary = normalize_name_for_match(&primary_name(name));
    let without_digits = primary.trim_end_matches(|c: char| c.is_ascii_digit());
    if without_digits.len() >= MIN_NAME_LEN {
        let without_digits = without_digits.to_owned();
        push_unique(&mut keys, primary);
        push_unique(&mut keys, without_digits);
    } else {
        push_unique(&mut keys, primary);
    }
    keys.retain(|key| !key.is_empty());
    keys
}

/// A publisher's or folder's words, normalized, without the ones that identify nothing.
fn significant_words(name: &str) -> impl Iterator<Item = String> + '_ {
    name.split(|c: char| c.is_whitespace() || c == ',')
        .map(normalize_name_for_match)
        .filter(|word| !word.is_empty() && !PUBLISHER_NOISE_WORDS.contains(&word.as_str()))
}

fn has_any(keys: &[String], names: &[String]) -> bool {
    keys.iter().any(|key| names.contains(key))
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !value.is_empty() && !values.contains(&value) {
        values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: MatchScope = MatchScope::ProgramFolders;
    const USER: MatchScope = MatchScope::UserFolders;

    fn vs_code() -> ProductNames {
        ProductNames::new(
            Some("Microsoft Visual Studio Code (User)"),
            Some("Microsoft Corporation"),
            &["Code", "Microsoft VS Code"],
            &[],
        )
    }

    fn firefox() -> ProductNames {
        ProductNames::new(
            Some("Mozilla Firefox (x64 en-US)"),
            Some("Mozilla"),
            &["firefox", "Mozilla Firefox"],
            &[],
        )
    }

    #[test]
    fn strips_versions_and_architectures_from_display_names() {
        assert_eq!(primary_name("7-Zip 25.01 (x64)"), "7-Zip");
        assert_eq!(primary_name("PowerShell 7-x64"), "PowerShell");
        assert_eq!(primary_name("PowerShell 7.6.9.0-x64"), "PowerShell");
        assert_eq!(primary_name("NVIDIA App 11.0.4.526"), "NVIDIA App");
        assert_eq!(primary_name("Foo v1.2 x64 64-bit"), "Foo");
        assert_eq!(primary_name("Adobe Photoshop 2024"), "Adobe Photoshop");
        assert_eq!(primary_name("2048"), "2048");
    }

    #[test]
    fn visual_studio_code_is_not_visual_studio() {
        let names = vs_code();
        assert!(names.matches_top_level("Code", PROGRAM));
        assert!(names.matches_top_level("Microsoft VS Code", PROGRAM));
        for other in [
            "Microsoft Visual Studio",
            "VisualStudio",
            "Programs",
            "Muse Spark Code",
            "cloud-code",
            "Codecs",
            "@opencode-aidesktop-updater",
        ] {
            assert!(!names.matches_top_level(other, PROGRAM), "{other}");
        }
        assert!(!names.matches_nested("Microsoft", "VisualStudio", PROGRAM));
        assert!(!names.matches_nested("Microsoft", "vscode-cpptools", PROGRAM));
        assert!(!names.matches_nested("OpenAI", "Codex", PROGRAM));
        assert!(!names.matches_nested("Microsoft OneDrive", "Codecs", PROGRAM));
    }

    #[test]
    fn a_publisher_never_selects_on_its_own() {
        let names = firefox();
        assert!(!names.matches_top_level("Mozilla", PROGRAM));
        assert!(!names.matches_top_level("Mozilla Thunderbird", PROGRAM));
        assert!(!names.matches_nested("Mozilla", "Thunderbird", PROGRAM));
        assert!(!names.matches_nested("Mozilla", "MaintenanceService", PROGRAM));

        let nvidia = ProductNames::new(
            Some("NVIDIA App 11.0.4.526"),
            Some("NVIDIA Corporation"),
            &[],
            &[],
        );
        assert!(!nvidia.matches_top_level("NVIDIA Corporation", PROGRAM));
        assert!(!nvidia.matches_top_level("NVIDIA", PROGRAM));
        assert!(!nvidia.matches_nested("NVIDIA Corporation", "NvContainer", PROGRAM));
    }

    #[test]
    fn matches_vendor_and_product_at_a_segment_boundary() {
        let names = firefox();
        assert!(names.matches_top_level("Mozilla Firefox", PROGRAM));
        assert!(names.matches_nested("Mozilla", "Firefox", PROGRAM));
        assert!(names.matches_nested("Mozilla", "Mozilla Firefox", PROGRAM));
        assert!(names.matches_nested("Mozilla", "Mozilla Firefox 159.0", PROGRAM));

        let chrome = ProductNames::new(Some("Google Chrome"), Some("Google LLC"), &[], &[]);
        assert!(chrome.matches_nested("Google", "Chrome", PROGRAM));
        assert!(!chrome.matches_nested("Google", "Chrome for Testing", PROGRAM));
        assert!(!chrome.matches_nested("Google", "Update", PROGRAM));

        let nvidia = ProductNames::new(
            Some("NVIDIA App 11.0.4.526"),
            Some("NVIDIA Corporation"),
            &[],
            &[],
        );
        assert!(nvidia.matches_nested("NVIDIA Corporation", "NVIDIA app", PROGRAM));

        let itunes = ProductNames::new(Some("iTunes"), Some("Apple Inc."), &[], &[]);
        assert!(itunes.matches_nested("Apple Computer", "iTunes", PROGRAM));

        let cocos = ProductNames::new(
            Some("Cocos Dashboard 1.0.14.2515"),
            Some("Xiamen Yaji Software Co., Ltd."),
            &[],
            &[],
        );
        assert!(cocos.matches_nested("Xiamen Yaji Software Co., Ltd.", "Cocos Dashboard", PROGRAM));
        assert!(cocos.matches_nested("Cocos", "Dashboard", PROGRAM));
    }

    #[test]
    fn a_name_is_not_a_substring_match() {
        let git = ProductNames::new(
            Some("Git"),
            Some("The Git Development Community"),
            &["git-bash"],
            &[],
        );
        assert!(git.matches_top_level("Git", PROGRAM));
        assert!(git.matches_nested("Programs", "Git", PROGRAM));
        for other in ["GitHub", "Logitech", "GitHubDesktop", "Gitleaks", "legit"] {
            assert!(!git.matches_top_level(other, PROGRAM), "{other}");
        }
        assert!(
            !git.matches_nested("grok", "git", PROGRAM),
            "a short name under an unrelated parent is someone else's copy"
        );
        assert!(
            !git.matches_nested("GitHub", "CLI", PROGRAM),
            "GitHub is not the Git vendor"
        );

        let firefox = firefox();
        assert!(
            !firefox.matches_nested("ms-playwright", "firefox-1543", PROGRAM),
            "a product's name under an unrelated parent is another tool's copy"
        );

        let powershell = ProductNames::new(
            Some("PowerShell 7.6.9.0-x64"),
            Some("Microsoft Corporation"),
            &["pwsh"],
            &[],
        );
        assert!(powershell.matches_top_level("PowerShell", PROGRAM));
        assert!(powershell.matches_nested("Microsoft", "PowerShell", PROGRAM));
        assert!(!powershell.matches_nested("Microsoft", "Windows PowerShell", PROGRAM));
        assert!(!powershell.matches_top_level("WindowsPowerShell", PROGRAM));
        assert!(
            !powershell
                .matches_top_level("MicrosoftCorporationII.QuickAssist_8wekyb3d8bbwe", PROGRAM)
        );
    }

    #[test]
    fn recognizes_a_name_with_the_same_version() {
        let python = ProductNames::new(Some("Python 3.14.10 (64-bit)"), None, &[], &[]);
        assert!(python.shares_version("Python 3.14 (64-bit)"));
        assert!(!python.shares_version("Python 3.13 (64-bit)"));
        assert!(!python.shares_version("Python"));
    }

    #[test]
    fn a_strict_match_needs_the_whole_name_spelled_out() {
        let powershell = ProductNames::new(
            Some("PowerShell 7.6.9.0-x64"),
            Some("Microsoft Corporation"),
            &["pwsh"],
            &[],
        );
        assert!(!powershell.spells_out_product("Microsoft", "PowerShell"));

        let edge = ProductNames::new(
            Some("Microsoft Edge"),
            Some("Microsoft Corporation"),
            &[],
            &[],
        );
        assert!(edge.spells_out_product("Microsoft", "Edge"));
        assert!(!edge.spells_out_product("Microsoft", "EdgeUpdate"));
    }

    #[test]
    fn folder_names_may_carry_a_version() {
        let photoshop =
            ProductNames::new(Some("Adobe Photoshop 2024"), Some("Adobe Inc."), &[], &[]);
        assert!(photoshop.matches_nested("Adobe", "Adobe Photoshop 2024", PROGRAM));
        assert!(photoshop.matches_nested("Adobe", "Adobe Photoshop 2025", PROGRAM));

        let solar_fire = ProductNames::new(Some("Solar Fire 9"), Some("Esoteric"), &[], &[]);
        assert!(solar_fire.matches_top_level("SolarFire9", PROGRAM));

        let seven_zip = ProductNames::new(Some("7-Zip 25.01 (x64)"), Some("Igor Pavlov"), &[], &[]);
        assert!(seven_zip.matches_top_level("7-Zip", PROGRAM));
        assert!(!seven_zip.matches_top_level("x64", PROGRAM));
        assert!(!seven_zip.matches_top_level("files", PROGRAM));
    }

    #[test]
    fn a_multi_word_name_matches_under_any_parent() {
        let names = ProductNames::new(Some("Stream Deck"), Some("Corsair Memory, Inc."), &[], &[]);
        assert!(names.matches_nested("Elgato", "StreamDeck", PROGRAM));

        let single_word = ProductNames::new(Some("Cursor"), Some("Anysphere"), &[], &[]);
        assert!(single_word.matches_top_level("Cursor", PROGRAM));
        assert!(
            !single_word.matches_nested("Some App", "Cursor", PROGRAM),
            "one common word under an unrelated parent proves nothing"
        );
    }

    #[test]
    fn user_folders_only_match_the_product_name() {
        let code = vs_code();
        assert!(
            !code.matches_top_level("Code", USER),
            "Code.exe does not own Documents\\Code"
        );

        let git = ProductNames::new(Some("Git"), Some("The Git Development Community"), &[], &[]);
        assert!(
            !git.matches_top_level("Git", USER),
            "too short to claim a user's folder"
        );

        let powershell = ProductNames::new(
            Some("PowerShell 7-x64"),
            Some("Microsoft Corporation"),
            &["pwsh"],
            &[],
        );
        assert!(powershell.matches_top_level("PowerShell", USER));
        assert!(!powershell.matches_top_level("WindowsPowerShell", USER));
        assert!(!powershell.matches_top_level("pwsh", USER));

        let game = ProductNames::new(Some("Skyrim"), Some("Bethesda"), &[], &[]);
        assert!(game.matches_nested("My Games", "Skyrim", USER));
    }

    #[test]
    fn a_vendor_is_not_another_one_whose_name_starts_the_same() {
        let focus = ProductNames::new(Some("Credentials"), Some("Micro Focus"), &[], &[]);
        assert!(!focus.matches_nested("Microsoft", "Credentials", PROGRAM));
        let intel = ProductNames::new(Some("Graphics"), Some("Intel Corporation"), &[], &[]);
        assert!(!intel.matches_nested("IntelliJ IDEA", "Graphics", PROGRAM));
        assert!(intel.matches_nested("Intel", "Graphics", PROGRAM));
    }

    #[test]
    fn a_folder_called_the_start_of_a_vendors_name_is_that_vendors() {
        let asus = ProductNames::new(Some("AURA"), Some("ASUSTeK Computer Inc."), &[], &[]);
        assert!(asus.matches_nested("ASUS", "AURA", PROGRAM));
        assert!(asus.matches_nested("ASUSTeK", "AURA", PROGRAM));
        let logitech = ProductNames::new(Some("Options"), Some("Logitech"), &[], &[]);
        assert!(logitech.matches_nested("Logi", "Options", PROGRAM));
        assert!(
            !logitech.matches_nested("Log", "Options", PROGRAM),
            "too short to go by"
        );
    }

    #[test]
    fn recognizes_a_folder_named_only_for_the_vendor() {
        let names = ProductNames::new(Some("Bar Tool"), Some("Acme Corp"), &[], &[]);
        assert!(names.is_vendor_only_folder("Acme"));
        assert!(names.is_vendor_only_folder("Acme Corp"));
        assert!(!names.is_vendor_only_folder("Bar Tool"));
        assert!(!names.is_vendor_only_folder("Other"));
        assert!(
            !names.is_vendor_only_folder("Acme Bar Tool"),
            "a vendor's name plus a product's is the product's folder"
        );

        let inkscape = ProductNames::new(Some("Inkscape"), Some("Inkscape Project"), &[], &[]);
        assert!(
            !inkscape.is_vendor_only_folder("Inkscape"),
            "a folder named for both vendor and product is the product's"
        );
    }

    #[test]
    fn a_near_match_goes_by_whole_words_and_is_never_an_exact_one() {
        let deck = ProductNames::new(Some("Stream Deck"), Some("Corsair Memory, Inc."), &[], &[]);
        assert_eq!(
            deck.near_match(None, "Stream Deck Plugins"),
            Some(NearName::StartsWithProduct)
        );
        assert_eq!(
            deck.near_match(None, "StreamDeckPlugins"),
            Some(NearName::StartsWithProduct),
            "words can be told by their capitals, for a name of more than one"
        );
        assert_eq!(
            deck.near_match(None, "Elgato Stream Deck"),
            Some(NearName::HasProductInIt)
        );
        assert_eq!(
            deck.near_match(None, "StreamDeck"),
            None,
            "that one is exact"
        );
        assert_eq!(deck.near_match(None, "Stream Decks"), None);
        assert_eq!(deck.near_match(None, "Streaming"), None);

        let git = ProductNames::new(Some("Git"), Some("The Git Development Community"), &[], &[]);
        assert_eq!(
            git.near_match(None, "Git LFS"),
            Some(NearName::StartsWithProduct)
        );
        for other in ["GitHub", "GitHubDesktop", "Gitleaks", "legit", "Logitech"] {
            assert_eq!(git.near_match(None, other), None, "{other}");
        }
    }

    #[test]
    fn what_is_under_some_other_folder_is_that_folders() {
        let chrome = ProductNames::new(Some("Google Chrome"), Some("Google LLC"), &["chrome"], &[]);
        assert_eq!(chrome.near_match(Some("muse-shell"), "chrome-prof"), None);
        assert_eq!(
            chrome.near_match(Some("Google"), "Chrome for Testing"),
            Some(NearName::StartsWithProduct)
        );
        assert_eq!(
            chrome.near_match(None, "chrome-devtools"),
            Some(NearName::StartsWithProduct)
        );
        let tools = ProductNames::new(Some("Build Kit"), Some("Fabrikam"), &["BuildTools"], &[]);
        assert_eq!(
            tools.near_match(Some("Elsewhere"), "Microsoft.Windows.SDK.BuildTools"),
            None
        );
        let vpn = ProductNames::new(Some("IPVanish"), Some("IPVanish"), &[], &[]);
        assert_eq!(
            vpn.near_match(Some("Mudhook_Marketing"), "IPVanish.exe_Url_jky3g1v0"),
            Some(NearName::StartsWithProduct),
            "where .NET keeps a program's settings"
        );
    }

    #[test]
    fn a_word_anyone_could_use_is_not_near_anything() {
        let code = vs_code();
        for other in ["Muse Spark Code", "cloud-code", "Code Stuff", "Codecs"] {
            assert_eq!(code.near_match(None, other), None, "{other}");
        }
        assert_eq!(
            code.near_match(None, "Microsoft Visual Studio Code Insiders"),
            Some(NearName::StartsWithProduct)
        );
    }

    #[test]
    fn a_publisher_of_everything_says_nothing() {
        let code = vs_code();
        for other in ["Microsoft Office", "Microsoft SDKs", "Microsoft.NET"] {
            assert_eq!(code.near_match(None, other), None, "{other}");
        }
        assert_eq!(code.near_match(Some("Microsoft"), "EdgeUpdate"), None);
        assert!(!code.has_publisher_like("Microsoft Corporation"));
    }

    #[test]
    fn one_word_of_a_name_is_not_near_anything() {
        let obs = ProductNames::new(Some("OBS Studio"), Some("OBS Project"), &[], &[]);
        for other in ["LM Studio", "Android Studio", "Studio One"] {
            assert_eq!(obs.near_match(None, other), None, "{other}");
        }
        assert_eq!(
            obs.near_match(None, "OBS Studio Plugins"),
            Some(NearName::StartsWithProduct)
        );
    }

    #[test]
    fn a_publisher_counts_by_all_of_what_it_is_called() {
        let windows = ProductNames::new(
            Some("Open the Windows"),
            Some("Open The Windows contributors"),
            &[],
            &[],
        );
        assert_eq!(windows.near_match(None, "Open XML SDK"), None);

        let instruments = ProductNames::new(Some("Traktor"), Some("Native Instruments"), &[], &[]);
        assert_eq!(
            instruments.near_match(None, "Native Instruments Extras"),
            Some(NearName::StartsWithPublisher(
                "Native Instruments".to_owned()
            ))
        );
        assert_eq!(instruments.near_match(None, "Native Sounds"), None);
        assert!(instruments.is_named_only_for_publisher("Native Instruments GmbH"));
        assert!(instruments.is_named_only_for_publisher("Native Instruments"));
        assert!(!instruments.is_named_only_for_publisher("Native Instruments Extras"));
        assert!(instruments.has_publisher_like("Native Instruments GmbH"));
        assert!(!instruments.has_publisher_like("Native Sounds"));

        let part = ProductNames::new(Some("Fabrikam Helper"), Some("Fabrikam Inc."), &[], &[]);
        assert!(part.is_a_parts_own_name(Some("Fabrikam"), "Fabrikam Helper"));
        assert!(part.is_a_parts_own_name(None, "Fabrikam Helper"));
        assert!(
            !part.is_a_parts_own_name(None, "Fabrikam"),
            "that is all of it"
        );
        assert!(!part.is_a_parts_own_name(Some("Contoso"), "Fabrikam Helper"));
        assert!(!part.is_a_parts_own_name(Some("Microsoft"), "Fabrikam Helper"));
        assert!(!part.is_a_parts_own_name(None, "Helper"));
        let of_everyone =
            ProductNames::new(Some("Some Part"), Some("Microsoft Corporation"), &[], &[]);
        assert!(!of_everyone.is_a_parts_own_name(Some("Microsoft"), "Some Part"));
        assert!(!of_everyone.is_a_parts_own_name(None, "Microsoft Some Part"));
        // The first of a publisher's words, the start of one, or the folder it is in are not it.
        let advanced = ProductNames::new(
            Some("Update Service"),
            Some("Advanced Micro Devices, Inc."),
            &[],
            &[],
        );
        assert!(!advanced.is_a_parts_own_name(Some("Advanced Installer"), "Update Service"));
        assert!(advanced.is_a_parts_own_name(Some("Advanced Micro Devices"), "Update Service"));
        let data = ProductNames::new(Some("Update Service"), Some("Data Systems"), &[], &[]);
        assert!(!data.is_a_parts_own_name(Some("Datadog"), "Update Service"));
        let in_a_folder = ProductNames::new(
            Some("Update Service"),
            Some("Fabrikam Inc."),
            &[],
            &["Contoso"],
        );
        assert!(!in_a_folder.is_a_parts_own_name(Some("Contoso"), "Update Service"));

        let short = ProductNames::new(Some("Vive"), Some("HTC Corp."), &[], &[]);
        assert!(
            short.has_publisher_like("HTC Corporation"),
            "all of it is the same"
        );
        assert!(!short.has_publisher_like("Other Corp."));
        assert_eq!(
            short.near_match(None, "HTC Stuff"),
            None,
            "three letters at the start of a folder's name says little"
        );

        let firefox = firefox();
        assert!(!firefox.is_named_only_for_publisher("Mozilla-1de4eec8-1241"));
        assert!(firefox.is_named_only_for_publisher("Mozilla"));
    }

    #[test]
    fn a_publishers_name_is_offered_with_what_it_is_called_and_a_folder_of_theirs_is_not() {
        let firefox = firefox();
        assert_eq!(
            firefox.near_match(None, "Mozilla-1de4eec8-1241-4177-a864-e594e8d1fb38"),
            Some(NearName::StartsWithPublisher("Mozilla".to_owned()))
        );
        assert_eq!(
            firefox.near_match(None, "Mozilla Maintenance Service"),
            Some(NearName::StartsWithPublisher("Mozilla".to_owned()))
        );
        assert_eq!(firefox.near_match(None, "Mozilla"), None);
        assert_eq!(firefox.near_match(None, "Mozillian Stuff"), None);

        let nvidia = ProductNames::new(
            Some("NVIDIA App 11.0.4.526"),
            Some("NVIDIA Corporation"),
            &[],
            &[],
        );
        assert_eq!(
            nvidia.near_match(Some("NVIDIA Corporation"), "NvContainer"),
            None,
            "being in a publisher's folder says nothing. A hidden entry can say more"
        );
        assert_eq!(
            nvidia.near_match(Some("NVIDIA Corporation"), "NVIDIA app"),
            None,
            "that one is exact"
        );
        assert_eq!(
            nvidia.near_match(Some("NVIDIA Corporation"), "NVIDIA App Helper"),
            Some(NearName::StartsWithProduct)
        );
        assert_eq!(nvidia.near_match(Some("NVIDIA Corporation"), "bin"), None);
    }

    #[test]
    fn names_an_executable_only_on_a_whole_name() {
        let chrome = ProductNames::new(Some("Google Chrome"), Some("Google LLC"), &[], &[]);
        assert!(chrome.names_executable("chrome"));
        assert!(chrome.names_executable("Google Chrome"));
        assert!(!chrome.names_executable("chrome_proxy"));
        assert!(!chrome.names_executable("notification_helper"));

        let powershell = ProductNames::new(
            Some("PowerShell 7.6.9.0-x64"),
            Some("Microsoft Corporation"),
            &[],
            &[],
        );
        assert!(powershell.names_executable("PowerShell 7 (x64)"));
    }
}
