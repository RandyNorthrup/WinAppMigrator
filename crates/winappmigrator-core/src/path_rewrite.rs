//! Where a package's paths land on the target, and rewriting text to match.
//!
//! Restoring writes source-machine paths from registry values, environment variables,
//! and scheduled tasks onto a machine where the app may live somewhere else, and
//! verifying has to expect exactly what was written. Both use this one map.
//!
//! There is one rule for the install folder, which covers everything inside it, plus an
//! exact rule for each copied file, since files outside that folder go wherever their
//! own routing says. Text is walked once, left to right: a replacement is never looked
//! at again, which is what stops `C:\App` becoming `C:\App22` for a target of `C:\App2`.
//! Each path found costs a few hash lookups however many files the package has.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    MAX_EMPTY_FOLDERS, data_location_of, is_external_package_path, restore_destination_for_entry,
};
use crate::models::{PackageManifest, RegistryEntry};
use crate::path_utils::{is_drive_root, path_is_within};
use crate::registry_target::hex_decode;
use crate::restore_policy::data_location_refusal;

/// What the source and destination of each rule are known by: lowercase, with forward
/// slashes turned into backslashes and no trailing separator. Every step swaps one
/// byte for one byte, so offsets found in this form are valid in the original text.
fn folded(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    if lower.contains('/') {
        lower.replace('/', "\\")
    } else {
        lower
    }
}

fn folded_path(path: &Path) -> String {
    let folded = folded(&path.to_string_lossy());
    folded.trim_end_matches('\\').to_owned()
}

/// How many bytes of a value are looked at to find what to rewrite in it, which is far past what a
/// path in any real value takes. A package can name a rule and a value of any size, and the cost of
/// looking otherwise grows with one times the other. A package that holds such a value is refused.
const MOST_BYTES_LOOKED_AT_IN_A_VALUE: usize = 32 * 1024 * 1024;
/// The same for all the values of a package together.
const MOST_BYTES_LOOKED_AT: u64 = 2 * 1024 * 1024 * 1024;
const TOO_MUCH_TO_LOOK_THROUGH: &str =
    "this package holds too much in its settings for what they name to be checked";

/// What it would take to look through what a package holds.
#[derive(Default)]
struct LookingCost {
    all: u64,
    most_in_one: u64,
}

impl LookingCost {
    fn too_much(&self) -> bool {
        self.all > MOST_BYTES_LOOKED_AT || self.most_in_one > MOST_BYTES_LOOKED_AT_IN_A_VALUE as u64
    }
}

/// The source machine's paths mapped to where this restore puts them.
#[derive(Debug, Clone, Default)]
pub struct RestoreMap {
    /// Copied files, by their folded source path.
    files: HashMap<String, PathBuf>,
    /// The folded path of everywhere a file that may be run is restored to. What is in
    /// [`Self::set_aside`] is not here.
    destinations: HashSet<String>,
    /// By their folded source path, files from a data folder that are not going back to one, so
    /// they are under the install location, in a folder named `_external`. They are still put
    /// there and what names them follows, but nothing is set up to run them.
    set_aside: HashSet<String>,
    /// Folders, by their folded source path, as text ready to write.
    roots: HashMap<String, String>,
    /// The longest source path of any rule, which bounds the search at each position.
    longest_rule: usize,
    /// The length of each rule's source path, as only a text of one of these can be one.
    rule_lengths: HashSet<usize>,
    target_root: PathBuf,
    /// Files whose destination is not allowed, and why. They are not in this map, so
    /// nothing can count on them being on the target.
    refused_files: Vec<(PathBuf, String)>,
    /// Empty folders, by their folded source path. No file is at any of these, so what says a
    /// program is there is not told that one is.
    empty_folders: HashMap<String, PathBuf>,
    refused_folders: Vec<(PathBuf, String)>,
}

impl RestoreMap {
    /// Build the map for restoring `manifest` to `target_root`.
    ///
    /// Fails if any copied file has an unsafe destination, so a package naming one is
    /// refused before anything happens, and if there is too much in it to go through.
    pub fn new(
        manifest: &PackageManifest,
        target_root: &Path,
        restore_user_data: bool,
    ) -> Result<Self> {
        let mut map = Self {
            target_root: target_root.to_path_buf(),
            ..Self::default()
        };
        for file in &manifest.file_inventory {
            if !file.should_copy || file.package_relative_path.is_none() {
                continue;
            }
            let destination = restore_destination_for_entry(file, target_root, restore_user_data)?;
            // Under the install location a file's name means nothing to Windows. Its real
            // location is what can make it run.
            let refusal = if path_is_within(&destination, target_root) {
                None
            } else {
                data_location_of(file).and_then(|(_, tail)| data_location_refusal(&tail))
            };
            match refusal {
                Some(reason) => map.refused_files.push((file.original_path.clone(), reason)),
                None => {
                    let put_aside = path_is_within(&destination, target_root)
                        && data_location_of(file).is_some()
                        && file
                            .package_relative_path
                            .as_deref()
                            .is_some_and(is_external_package_path);
                    map.add_file(&file.original_path, destination, put_aside);
                }
            }
        }
        for folder in manifest
            .empty_folders
            .iter()
            .filter(|folder| folder.should_copy && folder.package_relative_path.is_some())
            .take(MAX_EMPTY_FOLDERS)
        {
            let destination =
                restore_destination_for_entry(folder, target_root, restore_user_data)?;
            let refusal = if path_is_within(&destination, target_root) {
                None
            } else {
                data_location_of(folder).and_then(|(_, tail)| data_location_refusal(&tail))
            };
            match refusal {
                Some(reason) => map
                    .refused_folders
                    .push((folder.original_path.clone(), reason)),
                None => {
                    // A value can say where this is, and what it says has to follow it.
                    map.add_root(&folder.original_path, &destination);
                    map.empty_folders
                        .insert(folded_path(&folder.original_path), destination);
                }
            }
        }
        // Files are stored relative to the install location, which can be above the
        // executable's own folder (`Chrome` for `Chrome\Application\chrome.exe`).
        let source_root = manifest
            .application_metadata
            .as_ref()
            .and_then(|app| app.install_location.as_deref())
            .or_else(|| manifest.main_executable_path.parent());
        if let Some(source_root) = source_root.filter(|root| !is_drive_root(root)) {
            map.add_root(source_root, target_root);
        }
        map.refuse_too_much_to_look_through(manifest)?;
        Ok(map)
    }

    /// A text is rewritten the same however many times it is asked, so what a review checks is what
    /// is written and what is verified. That leaves no limit that can run out part way through an
    /// install, so what would be too much is found here, before any of it is looked at, by adding up
    /// the most each text could cost. An honest package comes nowhere near.
    fn refuse_too_much_to_look_through(&self, manifest: &PackageManifest) -> Result<()> {
        let mut cost = LookingCost::default();
        for entry in manifest
            .registry_keys_exported
            .iter()
            .chain(&manifest.license_registry_entries)
        {
            self.add_entry_cost(entry, &mut cost);
        }
        for service in &manifest.services_detected {
            for entry in &service.parameters {
                self.add_entry_cost(entry, &mut cost);
            }
            self.add_cost(service.arguments.as_deref().unwrap_or_default(), &mut cost);
            self.add_cost(&service.binary_path.to_string_lossy(), &mut cost);
        }
        for driver in &manifest.drivers_detected {
            self.add_cost(&driver.image_path.to_string_lossy(), &mut cost);
        }
        for task in &manifest.scheduled_tasks_detected {
            self.add_cost(
                task.definition_xml.as_deref().unwrap_or_default(),
                &mut cost,
            );
        }
        for shortcut in &manifest.shortcuts_detected {
            self.add_cost(&shortcut.target_path.to_string_lossy(), &mut cost);
            self.add_cost(shortcut.arguments.as_deref().unwrap_or_default(), &mut cost);
            for path in [&shortcut.working_directory, &shortcut.icon_path]
                .into_iter()
                .flatten()
            {
                self.add_cost(&path.to_string_lossy(), &mut cost);
            }
        }
        for variable in &manifest.environment_variables_detected {
            self.add_cost(&variable.value, &mut cost);
        }
        for rule in &manifest.firewall_rules_detected {
            self.add_cost(&rule.program.to_string_lossy(), &mut cost);
        }
        for artifact in &manifest.license_artifacts_detected {
            if let Some(path) = artifact.path.as_deref() {
                self.add_cost(&path.to_string_lossy(), &mut cost);
            }
        }
        if cost.too_much() {
            return Err(WinAppMigratorError::InvalidInput(
                TOO_MUCH_TO_LOOK_THROUGH.to_owned(),
            ));
        }
        Ok(())
    }

    fn add_entry_cost(&self, entry: &RegistryEntry, cost: &mut LookingCost) {
        self.add_cost(entry.value_name.as_deref().unwrap_or_default(), cost);
        self.add_cost(entry.value_data.as_deref().unwrap_or_default(), cost);
        let Some(bytes) = entry
            .value_bytes_hex
            .as_deref()
            .and_then(|hex| hex_decode(hex).ok())
        else {
            return;
        };
        let (pairs, _) = bytes.as_chunks::<2>();
        let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
        for part in String::from_utf16_lossy(&units).split('\0') {
            self.add_cost(part, cost);
        }
    }

    /// The most [`Self::rewrite`] could look at in this text.
    fn add_cost(&self, text: &str, cost: &mut LookingCost) {
        let search = folded(text);
        let bytes = search.as_bytes();
        let mut this_text = 0u64;
        let mut index = 0;
        while let Some(start) = next_path_start(bytes, index) {
            let limit = start.saturating_add(self.longest_rule).min(bytes.len());
            this_text = this_text.saturating_add((limit - start) as u64);
            index = start + 1;
        }
        cost.all = cost.all.saturating_add(this_text);
        cost.most_in_one = cost.most_in_one.max(this_text);
    }

    fn add_file(&mut self, source: &Path, destination: PathBuf, put_aside: bool) {
        let source = folded_path(source);
        self.note_rule_length(source.len());
        let destination_key = folded_path(&destination);
        if put_aside {
            self.set_aside.insert(source.clone());
        } else {
            self.set_aside.remove(&source);
        }
        // Listed twice in a manifest, a file is only written to where it is finally mapped.
        if let Some(earlier) = self.files.insert(source, destination) {
            self.destinations.remove(&folded_path(&earlier));
        }
        if !put_aside {
            self.destinations.insert(destination_key);
        }
    }

    fn note_rule_length(&mut self, length: usize) {
        self.longest_rule = self.longest_rule.max(length);
        self.rule_lengths.insert(length);
    }

    fn add_root(&mut self, source: &Path, destination: &Path) {
        let source = folded_path(source);
        self.note_rule_length(source.len());
        let text = destination.to_string_lossy();
        self.roots
            .insert(source, text.trim_end_matches(['\\', '/']).to_owned());
    }

    /// Where a copied file lands on the target. `None` for anything that was not copied,
    /// which means no file of this package will be at any location the path could map to.
    pub fn destination_of(&self, source: &Path) -> Option<&Path> {
        self.files.get(&folded_path(source)).map(PathBuf::as_path)
    }

    /// Where a file lands that something may be set up to run. A file from a data folder that was
    /// put aside under the install location is not one: `None` for it, as for what was not copied.
    pub fn runnable_destination_of(&self, source: &Path) -> Option<&Path> {
        let source = folded_path(source);
        if self.set_aside.contains(&source) {
            return None;
        }
        self.files.get(&source).map(PathBuf::as_path)
    }

    /// Where an empty folder goes on the target. `None` for one that is not going anywhere.
    pub fn folder_destination_of(&self, source: &Path) -> Option<&Path> {
        self.empty_folders
            .get(&folded_path(source))
            .map(PathBuf::as_path)
    }

    /// True when `path` is exactly where a copied file lands on the target, and may be run from.
    pub fn is_restored_file(&self, path: &Path) -> bool {
        self.destinations.contains(&folded_path(path))
    }

    pub fn target_root(&self) -> &Path {
        &self.target_root
    }

    /// Files left out because of where they would have gone, with why.
    pub fn refused_files(&self) -> &[(PathBuf, String)] {
        &self.refused_files
    }

    /// Empty folders left out because of where they would have gone, with why.
    pub fn refused_folders(&self) -> &[(PathBuf, String)] {
        &self.refused_folders
    }

    #[cfg(test)]
    pub(crate) fn for_tests(target_root: &str) -> Self {
        Self {
            target_root: PathBuf::from(target_root),
            ..Self::default()
        }
    }

    #[cfg(test)]
    pub(crate) fn add_test_root(&mut self, source: &str, destination: &str) {
        self.add_root(Path::new(source), Path::new(destination));
    }

    #[cfg(test)]
    pub(crate) fn add_test_file(&mut self, source: &str, destination: &str) {
        self.add_file(Path::new(source), PathBuf::from(destination), false);
    }

    /// Replace every path in `text` that this restore moves, leaving everything else as
    /// it was. A path counts only as a whole: `C:\App` is not in `C:\Application Data`.
    pub fn rewrite(&self, text: &str) -> String {
        let search = folded(text);
        let bytes = search.as_bytes();
        let mut output = String::with_capacity(text.len());
        let mut written = 0;
        let mut index = 0;
        let mut budget = MOST_BYTES_LOOKED_AT_IN_A_VALUE;
        while let Some(start) = next_path_start(bytes, index) {
            let Some(looked) = self.look_at(start, bytes.len(), &mut budget) else {
                break;
            };
            let Some((end, destination)) = self.longest_rule_at(&search, start, looked) else {
                index = start + 1;
                continue;
            };
            output.push_str(&text[written..start]);
            let matched = &text[start..end];
            push_destination(&mut output, destination, matched, bytes.get(end).copied());
            written = end;
            index = end;
        }
        output.push_str(&text[written..]);
        output
    }

    /// Rewrite the raw data of a `REG_MULTI_SZ` value: UTF-16 strings, each ended by a NUL.
    /// Data that is not that shape comes back as it was.
    pub fn rewrite_multi_string(&self, bytes: &[u8]) -> Vec<u8> {
        let (pairs, odd_byte) = bytes.as_chunks::<2>();
        let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
        let (true, Ok(text)) = (odd_byte.is_empty(), String::from_utf16(&units)) else {
            return bytes.to_vec();
        };
        text.split('\0')
            .map(|part| self.rewrite(part))
            .collect::<Vec<_>>()
            .join("\0")
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect()
    }

    /// Rewrite the text of an XML document. Paths in XML are escaped, so `C:\A&B` appears
    /// as `C:\A&amp;B`, and a target holding `&` would otherwise break the document.
    /// Fails on anything that does not parse, and on CDATA, whose text would not map back
    /// to one place in the document. Windows does not write it in a task's definition.
    pub fn rewrite_xml(&self, xml: &str) -> std::result::Result<String, String> {
        if xml.contains("<![CDATA[") {
            return Err("it holds a CDATA section".to_owned());
        }
        let document = roxmltree::Document::parse(xml).map_err(|error| error.to_string())?;
        let mut output = String::with_capacity(xml.len());
        let mut written = 0;
        for node in document.descendants().filter(|node| node.is_text()) {
            let text = node.text().unwrap_or_default();
            let rewritten = self.rewrite(text);
            if rewritten != text {
                let range = node.range();
                output.push_str(&xml[written..range.start]);
                output.push_str(&escape_xml_text(&rewritten));
                written = range.end;
            }
        }
        output.push_str(&xml[written..]);
        Ok(output)
    }

    /// How far from `start` there is to look, counted against what can be looked at in one text.
    /// `None` once that is reached, which goes by the text alone, so that it says the same each time.
    fn look_at(&self, start: usize, length: usize, budget: &mut usize) -> Option<usize> {
        let limit = start.saturating_add(self.longest_rule).min(length);
        let cost = limit - start;
        if cost > *budget {
            return None;
        }
        *budget -= cost;
        Some(limit)
    }

    /// The longest rule whose source path starts at `start` and ends at a path boundary.
    fn longest_rule_at(
        &self,
        search: &str,
        start: usize,
        limit: usize,
    ) -> Option<(usize, Destination<'_>)> {
        let bytes = search.as_bytes();
        let mut best = None;
        for end in start + 2..=limit {
            let next = bytes.get(end).copied();
            if !ends_a_path_at(next) {
                if next.is_some_and(cannot_be_in_a_path) {
                    break;
                }
                continue;
            }
            if !self.rule_lengths.contains(&(end - start)) {
                if next.is_some_and(cannot_be_in_a_path) {
                    break;
                }
                continue;
            }
            // `end` is before an ASCII byte or at the end of the text, so it is a boundary
            // between characters.
            let candidate = &search[start..end];
            if let Some(destination) = self.files.get(candidate) {
                best = Some((end, Destination::File(destination)));
            } else if next != Some(b' ')
                && next != Some(b'\t')
                && let Some(destination) = self.roots.get(candidate)
            {
                // A space ends a file's path in a command line. After a folder it far more
                // likely continues a name: `C:\Git Extensions` is not `C:\Git`.
                best = Some((end, Destination::Root(destination)));
            }
            if next.is_some_and(cannot_be_in_a_path) {
                break;
            }
        }
        best
    }
}

enum Destination<'a> {
    File(&'a Path),
    Root(&'a str),
}

fn push_destination(
    output: &mut String,
    destination: Destination<'_>,
    matched: &str,
    next: Option<u8>,
) {
    let mut text = match destination {
        Destination::File(path) => path.to_string_lossy().into_owned(),
        Destination::Root(text) => {
            // A drive root is kept as `D:` so `\rest` can follow it, but alone it needs
            // its separator back, or it would mean the current folder on that drive.
            if next != Some(b'\\') && next != Some(b'/') && text.ends_with(':') {
                format!("{text}\\")
            } else {
                text.to_owned()
            }
        }
    };
    if matched.contains('/') && !matched.contains('\\') {
        text = text.replace('\\', "/");
    }
    output.push_str(&text);
}

/// Find the next place a rooted path can begin: `C:\` or `\\server`, not in the middle of
/// a longer word, in text that has been [`folded`].
fn next_path_start(bytes: &[u8], from: usize) -> Option<usize> {
    let mut index = from;
    while index + 2 < bytes.len() {
        let previous = index.checked_sub(1).and_then(|before| bytes.get(before));
        let starts = match bytes[index] {
            letter if letter.is_ascii_lowercase() => {
                bytes[index + 1] == b':'
                    && bytes[index + 2] == b'\\'
                    && !previous.is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte >= 0x80)
            }
            b'\\' => bytes[index + 1] == b'\\' && previous != Some(&b'\\'),
            _ => false,
        };
        if starts {
            return Some(index);
        }
        index += 1;
    }
    None
}

/// Whether a path can stop just before this byte. Anything else continues a name, which
/// is the boundary check: `C:\Program Files\Git` does not match `...\GitHub CLI`.
fn ends_a_path_at(next: Option<u8>) -> bool {
    match next {
        None => true,
        Some(byte) => {
            matches!(byte, b'\\' | b'\'' | b',' | b' ' | b'\t') || cannot_be_in_a_path(byte)
        }
    }
}

/// Bytes no path continues past. `;` is legal in a file name but it separates paths far
/// more often. The other printable ones are illegal in Windows file names.
fn cannot_be_in_a_path(byte: u8) -> bool {
    matches!(
        byte,
        b'"' | b';' | b'<' | b'>' | b'|' | b'*' | b'?' | b'\0' | b'\r' | b'\n'
    )
}

pub(crate) fn escape_xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::models::{FileClassification, FileEntry};

    fn multi_string(text: &str) -> Vec<u8> {
        text.encode_utf16().flat_map(u16::to_le_bytes).collect()
    }

    #[test]
    fn each_string_in_a_multi_string_value_is_rewritten() {
        let map = map_with_root(r"C:\Program Files\App", r"D:\Restored Here");
        assert_eq!(
            map.rewrite_multi_string(&multi_string(
                "C:\\Program Files\\App\\a\0other\0C:\\Program Files\\App\\b\0\0"
            )),
            multi_string("D:\\Restored Here\\a\0other\0D:\\Restored Here\\b\0\0")
        );
        for unchanged in ["", "\0", "other\0\0", "C:\\Program Files\\Apple\0\0"] {
            assert_eq!(
                map.rewrite_multi_string(&multi_string(unchanged)),
                multi_string(unchanged),
                "{unchanged:?}"
            );
        }
    }

    #[test]
    fn data_that_is_not_a_multi_string_comes_back_as_it_was() {
        let map = map_with_root(r"C:\App", r"D:\Restored");
        for bytes in [
            // An odd length.
            vec![b'C', 0, b':'],
            // A surrogate with no other half.
            vec![0x00, 0xD8, b'C', 0],
        ] {
            assert_eq!(map.rewrite_multi_string(&bytes), bytes);
        }
    }

    fn map_with_root(source: &str, target: &str) -> RestoreMap {
        let mut map = RestoreMap {
            target_root: PathBuf::from(target),
            ..RestoreMap::default()
        };
        map.add_root(Path::new(source), Path::new(target));
        map
    }

    /// As the review found it: one rule of 6 KB and a value of 24 KB took 17.5 seconds, and a value
    /// is rewritten when a package is loaded, before anyone has agreed to anything.
    #[test]
    fn a_long_rule_and_a_long_value_cost_next_to_nothing() {
        let mut map = RestoreMap::for_tests(r"D:\T");
        let long_source = format!(r"C:\{}", "a".repeat(6 * 1024));
        map.add_test_root(&long_source, r"D:\T");
        map.add_test_root(r"C:\App", r"D:\T");
        let value = format!(r"{}C:\App\a.exe", r"C:\a ".repeat(24 * 1024 / 5));
        let started = std::time::Instant::now();
        let rewritten = map.rewrite(&value);
        let took = started.elapsed();
        assert!(took.as_secs() < 2, "{took:?}");
        assert!(rewritten.ends_with(r"D:\T\a.exe"), "it still does its job");
    }

    #[test]
    fn past_what_can_be_looked_at_a_value_is_left_as_it_stands() {
        let mut map = RestoreMap::for_tests(r"D:\T");
        let long_source = format!(r"C:\{}", "a".repeat(32 * 1024));
        map.add_test_root(&long_source, r"D:\T");
        map.add_test_root(r"C:\App", r"D:\T");
        // Each of these starts a path, and costs the length of the longest rule to look along.
        let far_too_much = format!(r"{}C:\App\a.exe", r"C:\a ".repeat(4 * 1024));
        let rewritten = map.rewrite(&far_too_much);
        assert!(rewritten.ends_with(r"C:\App\a.exe"), "it stopped looking");
        assert!(
            map.rewrite(r"C:\App\b.exe").ends_with(r"D:\T\b.exe"),
            "and the next is fine"
        );
    }

    /// What a review checks and what is then written are two asks of this.
    #[test]
    fn however_much_has_been_looked_at_a_text_says_the_same() {
        let mut map = RestoreMap::for_tests(r"D:\T");
        let long_source = format!(r"C:\{}", "a".repeat(32 * 1024));
        map.add_test_root(&long_source, r"D:\T");
        map.add_test_root(r"C:\App", r"D:\T");
        let expensive = r"C:\a ".repeat(4 * 1024);
        for _ in 0..100 {
            map.rewrite(&expensive);
        }
        assert_eq!(map.rewrite(r"C:\App\a.exe"), r"D:\T\a.exe");
    }

    #[test]
    fn a_text_is_found_to_be_too_much_by_what_it_would_cost() {
        let mut map = RestoreMap::for_tests(r"D:\T");
        map.add_test_root(&format!(r"C:\{}", "a".repeat(1024)), r"D:\T");
        let mut cost = LookingCost::default();
        map.add_cost(r"no path at all", &mut cost);
        assert_eq!(cost.all, 0);
        map.add_cost(r"C:\a ", &mut cost);
        assert!(!cost.too_much());
        map.add_cost(&r"C:\a ".repeat(40 * 1024), &mut cost);
        assert!(cost.too_much(), "{} in one", cost.most_in_one);
    }

    #[test]
    fn a_target_beginning_with_the_source_is_not_rewritten_twice() {
        let map = map_with_root(r"C:\App", r"C:\App2");
        assert_eq!(map.rewrite(r"C:\App\a.exe"), r"C:\App2\a.exe");
        assert_eq!(map.rewrite(r"C:\App"), r"C:\App2");
    }

    #[test]
    fn a_longer_name_beginning_with_the_source_is_left_alone() {
        let map = map_with_root(r"C:\App", r"C:\App2");
        assert_eq!(
            map.rewrite(r"C:\Application Data\x"),
            r"C:\Application Data\x"
        );
        assert_eq!(map.rewrite(r"C:\App2\a.exe"), r"C:\App2\a.exe");

        let git = map_with_root(r"C:\Program Files\Git", r"D:\Tools\Git");
        assert_eq!(
            git.rewrite(r"C:\Program Files\GitHub CLI\gh.exe"),
            r"C:\Program Files\GitHub CLI\gh.exe"
        );
        assert_eq!(
            git.rewrite(r"C:\Program Files\Git Extensions\x.exe"),
            r"C:\Program Files\Git Extensions\x.exe"
        );
        assert_eq!(
            git.rewrite(r"C:\Program Files\Git\bin\git.exe"),
            r"D:\Tools\Git\bin\git.exe"
        );
    }

    #[test]
    fn case_is_ignored_and_the_rest_of_the_text_is_kept_exactly() {
        let map = map_with_root(r"C:\App", r"D:\Restored");
        assert_eq!(
            map.rewrite(r"run C:\APP\Bin.exe /Flag"),
            r"run D:\Restored\Bin.exe /Flag"
        );
    }

    #[test]
    fn every_path_in_a_value_is_rewritten() {
        let map = map_with_root(r"C:\App", r"D:\Restored");
        assert_eq!(
            map.rewrite(r#"C:\Windows;C:\App\bin;C:\App;"C:\App\x y.exe" --data C:\App\data"#),
            r#"C:\Windows;D:\Restored\bin;D:\Restored;"D:\Restored\x y.exe" --data D:\Restored\data"#
        );
    }

    #[test]
    fn a_path_inside_a_longer_word_is_not_a_path() {
        let map = map_with_root(r"C:\App", r"D:\Restored");
        assert_eq!(map.rewrite(r"ABC:\App\x"), r"ABC:\App\x");
        assert_eq!(map.rewrite(r"\\?\C:\App\x"), r"\\?\D:\Restored\x");
        assert_eq!(map.rewrite(r"é:\App"), r"é:\App");
    }

    #[test]
    fn forward_slashes_match_and_keep_their_style() {
        let map = map_with_root(r"C:\Program Files\App", r"D:\Apps\App");
        assert_eq!(
            map.rewrite("file:///C:/Program Files/App/help.html"),
            "file:///D:/Apps/App/help.html"
        );
    }

    #[test]
    fn a_trailing_separator_on_a_rule_does_not_matter() {
        let map = map_with_root(r"C:\App\", r"D:\Restored\");
        assert_eq!(map.rewrite(r"C:\App\x"), r"D:\Restored\x");
        assert_eq!(map.rewrite(r"C:\App\"), r"D:\Restored\");
    }

    #[test]
    fn a_drive_root_target_keeps_its_separator() {
        let map = map_with_root(r"C:\App", r"D:\");
        assert_eq!(map.rewrite(r"C:\App\x.exe"), r"D:\x.exe");
        assert_eq!(map.rewrite(r"C:\App"), r"D:\");
    }

    #[test]
    fn unicode_around_a_path_is_kept() {
        let map = map_with_root(r"C:\App", r"D:\Prüfung");
        assert_eq!(
            map.rewrite(r"über C:\App\größe.txt ende"),
            r"über D:\Prüfung\größe.txt ende"
        );
    }

    #[test]
    fn a_file_rule_outranks_its_folder_and_ends_at_arguments() {
        let mut map = map_with_root(r"C:\App", r"D:\Restored");
        map.add_file(
            Path::new(r"C:\App\odd name.\x.exe"),
            PathBuf::from(r"D:\Restored\odd name\x.exe"),
            false,
        );
        assert_eq!(
            map.rewrite(r"C:\App\odd name.\x.exe --flag"),
            r"D:\Restored\odd name\x.exe --flag"
        );
        assert_eq!(map.rewrite(r"C:\App\other.exe"), r"D:\Restored\other.exe");
    }

    #[test]
    fn a_file_rule_is_exact() {
        let mut map = RestoreMap::default();
        map.add_file(
            Path::new(r"C:\ProgramData\App\a.dat"),
            PathBuf::from(r"D:\T\_external\a.dat"),
            false,
        );
        assert_eq!(
            map.rewrite(r"C:\ProgramData\App\a.dat,0"),
            r"D:\T\_external\a.dat,0"
        );
        assert_eq!(
            map.rewrite(r"C:\ProgramData\App\a.dat2"),
            r"C:\ProgramData\App\a.dat2"
        );
        assert_eq!(
            map.rewrite(r"C:\ProgramData\App\b.dat"),
            r"C:\ProgramData\App\b.dat"
        );
    }

    #[test]
    fn a_file_listed_twice_only_counts_where_it_finally_lands() {
        let mut map = RestoreMap::default();
        map.add_file(
            Path::new(r"C:\App\a.exe"),
            PathBuf::from(r"D:\T\first.exe"),
            false,
        );
        map.add_file(
            Path::new(r"C:\App\a.exe"),
            PathBuf::from(r"D:\T\second.exe"),
            false,
        );
        assert!(!map.is_restored_file(Path::new(r"D:\T\first.exe")));
        assert!(map.is_restored_file(Path::new(r"D:\T\second.exe")));
    }

    #[test]
    fn looking_up_a_file_works_from_either_side() {
        let mut map = RestoreMap::default();
        map.add_file(
            Path::new(r"C:\App\A.exe"),
            PathBuf::from(r"D:\T\A.exe"),
            false,
        );
        assert_eq!(
            map.destination_of(Path::new(r"c:\app\a.exe")),
            Some(Path::new(r"D:\T\A.exe"))
        );
        assert_eq!(map.destination_of(Path::new(r"C:\App\b.exe")), None);
        assert_eq!(map.destination_of(Path::new(r"C:\App\..\App\A.exe")), None);
        assert!(map.is_restored_file(Path::new(r"d:\t\a.exe")));
        assert!(!map.is_restored_file(Path::new(r"D:\T\b.exe")));
    }

    #[test]
    fn xml_text_is_rewritten_and_stays_valid() -> std::result::Result<(), String> {
        let map = map_with_root(r"C:\A&B", r"D:\C&D");
        let xml = r#"<?xml version="1.0"?><Task><Exec><Command>C:\A&amp;B\a.exe</Command><Arguments>--x="C:\A&amp;B\data"</Arguments></Exec><Note>C:\Other</Note></Task>"#;
        let rewritten = map.rewrite_xml(xml)?;
        assert_eq!(
            rewritten,
            r#"<?xml version="1.0"?><Task><Exec><Command>D:\C&amp;D\a.exe</Command><Arguments>--x="D:\C&amp;D\data"</Arguments></Exec><Note>C:\Other</Note></Task>"#
        );
        assert!(roxmltree::Document::parse(&rewritten).is_ok());
        assert!(map.rewrite_xml("<Task>").is_err());
        assert!(
            map.rewrite_xml(r"<Task>C:\A&amp;B<![CDATA[\x]]></Task>")
                .is_err()
        );
        Ok(())
    }

    fn inventory(count: usize) -> PackageManifest {
        let mut manifest = PackageManifest {
            schema_version: String::new(),
            source_computer_name: None,
            source_windows_version: None,
            source_platform: None,
            program_needs: Vec::new(),
            scan_timestamp_utc: String::new(),
            main_executable_path: PathBuf::from(r"C:\Program Files\Big App\app.exe"),
            application_metadata: None,
            file_inventory: Vec::with_capacity(count),
            registry_keys_exported: Vec::new(),
            com_registrations_detected: Vec::new(),
            file_associations_detected: Vec::new(),
            services_detected: Vec::new(),
            drivers_detected: Vec::new(),
            scheduled_tasks_detected: Vec::new(),
            shortcuts_detected: Vec::new(),
            environment_variables_detected: Vec::new(),
            runtime_prerequisites: Vec::new(),
            license_artifacts_detected: Vec::new(),
            license_registry_entries: Vec::new(),
            secrets_left_out: Vec::new(),
            licenses_left_out: Vec::new(),
            settings_left_out: Vec::new(),
            empty_folders: Vec::new(),
            firewall_rules_detected: Vec::new(),
            certificates_reported: Vec::new(),
            fonts_reported: Vec::new(),
            left_out_by_choice: Vec::new(),
            added_by_choice: Vec::new(),
            warnings: Vec::new(),
            items_intentionally_excluded: Vec::new(),
            included_user_data: false,
            included_program_data: false,
            included_license_data: false,
            user_data_profile: None,
        };
        for index in 0..count {
            let relative = format!(r"module{}\sub{}\file{index}.dll", index % 400, index % 7);
            let original = PathBuf::from(r"C:\Program Files\Big App").join(&relative);
            manifest.file_inventory.push(FileEntry {
                path: original.clone(),
                original_path: original,
                package_relative_path: Some(PathBuf::from("files").join(relative)),
                size_bytes: Some(1),
                hash_sha256: None,
                classification: FileClassification::AppOwned,
                should_copy: true,
                reason: String::new(),
                user_folder: None,
            });
        }
        manifest
    }

    /// The old loop took about 22 ms per value at 20,000 files, so about 110 ms at
    /// 100,000. A tenth of that per value, on a debug build with other tests running,
    /// leaves plenty of room and still fails if the cost goes back to scaling with the
    /// number of files.
    #[test]
    fn rewriting_stays_cheap_at_a_hundred_thousand_files() -> Result<()> {
        const FILES: usize = 100_000;
        const VALUES: u32 = 2_000;

        let manifest = inventory(FILES);
        let target = PathBuf::from(r"D:\Apps\Big App");

        let started = Instant::now();
        let map = RestoreMap::new(&manifest, &target, false)?;
        let build_time = started.elapsed();

        let value = r#""C:\Program Files\Big App\module7\sub3\file7.dll" --data C:\Program Files\Big App\data;C:\Windows\System32\unrelated.dll"#;
        let started = Instant::now();
        let mut rewritten = String::new();
        for _ in 0..VALUES {
            rewritten = map.rewrite(value);
        }
        let per_value = started.elapsed() / VALUES;
        println!("100,000 files: built in {build_time:?}, {per_value:?} per value");

        assert_eq!(
            rewritten,
            r#""D:\Apps\Big App\module7\sub3\file7.dll" --data D:\Apps\Big App\data;C:\Windows\System32\unrelated.dll"#
        );
        assert!(
            per_value < Duration::from_millis(10),
            "{per_value:?} per value"
        );
        assert!(build_time < Duration::from_secs(20), "{build_time:?}");
        Ok(())
    }
}
