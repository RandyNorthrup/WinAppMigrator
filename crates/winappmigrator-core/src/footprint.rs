//! What a scan found, as a person chooses from it, and what comes of what they choose.
//!
//! A choice only ever takes something away from what a scan found. What is added comes through the
//! scan itself, and [`crate::near_match`] says how, so nothing here can bring in what a scan would
//! leave out, such as what looks like a credential. It is all said in the words of the list an
//! installer shows, from the same review an install goes by, so the two screens cannot differ.
//!
//! Leaving something out can leave something else with nothing to run. So what is left of a package
//! after a choice goes through that review, and what it would now refuse is taken out too, and said
//! to have been: a service that runs a file in a folder left out is not set up to run nothing.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::account_settings::is_account_setting;
use crate::errors::Result;
use crate::models::{
    FileClassification, FileEntry, FootprintKind, FootprintLocation, LeftOutItem, NearMatch,
    PackageManifest, ProgramRole, RegistryEntry, RegistryHive, ScanReport, Warning,
    WarningSeverity,
};
use crate::package::{driver_image_bytes, manifest_for_review};
use crate::path_utils::{
    comparable_path, path_is_within, strip_prefix_ignore_case, text_mentions_path,
};
use crate::prerequisites::handling_before_a_build;
use crate::registry_target::is_path_variable;
use crate::restore_policy::is_autostart_entry;
use crate::restore_review::{
    CERTIFICATES_LEFT_OUT_TEXT, CREDENTIALS_LEFT_OUT_TEXT, ChangeRow, FONTS_LEFT_OUT_TEXT,
    NO_TARGET_YET, ReviewedPackage, SETTINGS_LEFT_OUT_TEXT, SystemChange, SystemChangeKind,
    autostart_choice, certificate_label, change_kind_of, described_rows, driver_choice, notes_of,
    registry_item, service_choice, shortcut_choice, task_choice,
};
use crate::space::BuildInputs;

/// What in a task's definition is one path, however many spaces are in it.
const TASK_TAGS_OF_ONE_PATH: &[&str] = &["Command", "WorkingDirectory"];

/// What the list of what is left out says of licenses, before anything has been built.
pub const LICENSES_LEFT_OUT_TEXT: &str = "Not in this installer, because the License option is off";
/// Where what goes with a service is kept.
const SERVICES_KEY_PREFIX: &str = r"system\currentcontrolset\services\";
/// Where Windows keeps fonts, below the drive it is on. A package holds one of those only if a
/// machine-wide install record names it. A folder of the app's own called this is only a folder.
const WINDOWS_FONTS_BELOW_THE_DRIVE: &str = r"\windows\fonts";
/// How many times to go back over what is left, since what goes with something can take more with
/// it. Each pass only takes things out, so this is far more than any package needs.
const MOST_PASSES: usize = 8;
/// What a package's review calls a thing when it says it was left out.
const LEFT_OUT_MARKER: &str = " was left out: ";

/// One thing a person can tick or untick. Each goes by what it is called and not where it is on
/// a list, so a choice still means the same thing after another scan.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChoiceId {
    /// A folder, by its full path as its PC calls it, lowercase.
    Folder(String),
    Service(String),
    Driver(String),
    Task(String),
    Shortcut(String),
    /// A value that runs something at sign-in.
    Autostart(String),
    /// All of a kind of thing that ties the app into Windows.
    Integration(SystemChangeKind),
}

/// What a person has chosen to leave out. Everything else a scan found goes in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FootprintChoices {
    pub left_out: BTreeSet<ChoiceId>,
}

impl FootprintChoices {
    pub fn is_left_out(&self, id: &ChoiceId) -> bool {
        self.left_out.contains(id)
    }

    pub fn is_empty(&self) -> bool {
        self.left_out.is_empty()
    }

    pub fn set_included(&mut self, id: &ChoiceId, included: bool) {
        if included {
            self.left_out.remove(id);
        } else {
            self.left_out.insert(id.clone());
        }
    }
}

/// What was asked of a scan, which says what it means for something to be missing from it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScanFlags {
    pub include_user_data: bool,
    pub include_program_data: bool,
    pub include_license_data: bool,
}

/// Which list a folder belongs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderGroup {
    Program,
    Data,
}

/// A folder of what goes in, which is as fine as a choice goes. No screen has a row for each file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderRow {
    pub id: ChoiceId,
    pub name: String,
    /// Where it is, in words.
    pub place: String,
    pub path: PathBuf,
    pub group: FolderGroup,
    pub files: usize,
    pub bytes: u64,
    pub largest_file_bytes: u64,
    /// Why it cannot be left out, if so, and it then says why.
    pub locked: Option<&'static str>,
    /// What decides whether it goes in, rather than a choice of its own.
    pub goes_with: Option<ChoiceId>,
}

/// Something set up by itself, in the words the installer's list uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemRow {
    pub id: ChoiceId,
    pub change: SystemChange,
    /// What it adds to a package by itself, as a driver's image does.
    pub bytes: u64,
}

/// All of one kind of what ties an app into Windows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationRow {
    pub kind: SystemChangeKind,
    pub id: ChoiceId,
    pub changes: Vec<SystemChange>,
}

/// A runtime the app needs, and how it is provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrerequisiteRow {
    pub name: String,
    pub handling: String,
    pub needs_attention: bool,
}

/// What is left out for one reason, by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftOutGroup {
    pub title: String,
    pub items: Vec<LeftOutItem>,
}

/// What went with something a person left out, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlsoLeftOut {
    pub id: ChoiceId,
    pub title: String,
    pub because: String,
}

/// What comes of a person's choices, beyond what they chose.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatGoes {
    pub also: Vec<AlsoLeftOut>,
    /// What ties the app into Windows as it now stands, by kind. A kind can have nothing left of it
    /// without having been left out, which is why this is said apart from the rows as they were.
    pub integrations: Vec<IntegrationRow>,
}

/// What a package is allowed, by what each thing goes by.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Allowed {
    services: BTreeSet<String>,
    drivers: BTreeSet<String>,
    tasks: BTreeSet<String>,
    shortcuts: BTreeSet<String>,
    registry: BTreeSet<String>,
    environment: BTreeSet<String>,
    firewall: BTreeSet<String>,
}

/// What a scan found, ready to show and to choose from.
#[derive(Debug, Clone, Default)]
pub struct Footprint {
    pub main_program: PathBuf,
    pub folders: Vec<FolderRow>,
    pub services: Vec<ItemRow>,
    pub startup: Vec<ItemRow>,
    pub tasks: Vec<ItemRow>,
    pub shortcuts: Vec<ItemRow>,
    pub integrations: Vec<IntegrationRow>,
    pub prerequisites: Vec<PrerequisiteRow>,
    pub left_out: Vec<LeftOutGroup>,
    /// What the rules would refuse whatever is chosen, in the sentences an installer uses.
    pub refused: Vec<String>,
    /// What to know of whose folders and settings these will be.
    pub notes: Vec<String>,
    /// What a scan had to say, with what it left out taken off, since that is listed.
    pub warnings: Vec<Warning>,
    /// What a person's data comes to, that only goes in with the User data option.
    pub data_not_included_files: usize,
    pub data_not_included_bytes: u64,
    pub near_matches: Vec<NearMatch>,
    pub license_bytes: u64,
    /// A driver's file goes in even if no install would set it up, for it to say why not.
    refused_driver_bytes: u64,
    /// Where license data that goes in is from. A license goes by the License option, wherever it is.
    pub license_paths: Vec<PathBuf>,
    pub prerequisite_count: usize,
    pub connected_drive: bool,
    pub flags: ScanFlags,
    allowed: Allowed,
}

impl Footprint {
    /// Go over a scan once. This looks at each file, on a large app takes a moment, and belongs on a
    /// worker thread. `Err` means a package of it would be refused whole, which is better said now.
    pub fn of(report: &ScanReport, flags: ScanFlags) -> Result<Self> {
        let manifest = manifest_for_review(report, flags, &|_| true);
        let review =
            ReviewedPackage::before_a_location_is_chosen(&manifest, flags.include_license_data)?;
        let mut footprint = Self {
            main_program: report.main_executable.path.clone(),
            folders: folder_rows(report),
            license_bytes: report
                .license_artifacts
                .iter()
                .filter(|artifact| artifact.should_copy)
                .filter_map(|artifact| artifact.size_bytes)
                .fold(0, u64::saturating_add),
            license_paths: report
                .license_artifacts
                .iter()
                .filter(|artifact| artifact.should_copy)
                .filter_map(|artifact| artifact.path.clone())
                .collect(),
            prerequisite_count: report.runtime_prerequisites.len(),
            connected_drive: report.source_volume.is_some(),
            flags,
            near_matches: report.near_matches.clone(),
            allowed: allowed_by(&review),
            refused: review
                .refused
                .iter()
                .map(|warning| warning.message.clone())
                .collect(),
            notes: notes_of(&review),
            ..Self::default()
        };
        footprint.add_changes(described_rows(&review, Path::new(NO_TARGET_YET)), report);
        footprint.refused_driver_bytes = report
            .drivers
            .iter()
            .filter(|driver| {
                !footprint
                    .allowed
                    .drivers
                    .contains(&driver.service_name.to_ascii_lowercase())
            })
            .map(|driver| driver_image_bytes(report, driver))
            .fold(0, u64::saturating_add);
        footprint.add_what_is_left_out(report, &manifest);
        footprint.prerequisites = report
            .runtime_prerequisites
            .iter()
            .map(|prerequisite| {
                let (handling, needs_attention) =
                    handling_before_a_build(prerequisite, &report.architecture);
                PrerequisiteRow {
                    name: prerequisite.name.clone(),
                    handling,
                    needs_attention,
                }
            })
            .collect();
        for file in report
            .file_inventory
            .iter()
            .filter(|file| !file.should_copy && is_data(&file.classification))
        {
            footprint.data_not_included_files += 1;
            footprint.data_not_included_bytes = footprint
                .data_not_included_bytes
                .saturating_add(file.size_bytes.unwrap_or(0));
        }
        footprint.warnings = report
            .warnings
            .iter()
            .filter(|warning| {
                warning.severity != WarningSeverity::Info
                    && warning.code != "secret_left_out"
                    && warning.code != "license_left_out"
            })
            .cloned()
            .collect();
        Ok(footprint)
    }

    fn add_changes(&mut self, rows: Vec<ChangeRow>, report: &ScanReport) {
        self.integrations = integration_rows(&rows);
        for ChangeRow { id, change } in rows {
            let bytes = match &id {
                ChoiceId::Driver(name) => report
                    .drivers
                    .iter()
                    .find(|driver| driver.service_name.eq_ignore_ascii_case(name))
                    .map_or(0, |driver| driver_image_bytes(report, driver)),
                _ => 0,
            };
            let row = ItemRow { id, change, bytes };
            match row.change.kind {
                SystemChangeKind::Service | SystemChangeKind::Driver => self.services.push(row),
                SystemChangeKind::ScheduledTask => self.tasks.push(row),
                SystemChangeKind::Autostart => self.startup.push(row),
                SystemChangeKind::Shortcut => self.shortcuts.push(row),
                _ => {}
            }
        }
    }

    fn add_what_is_left_out(&mut self, report: &ScanReport, manifest: &PackageManifest) {
        let item_of = |name: String, why: &str| LeftOutItem {
            name,
            why: why.to_owned(),
        };
        let groups = [
            (CREDENTIALS_LEFT_OUT_TEXT, report.secrets_left_out.clone()),
            (LICENSES_LEFT_OUT_TEXT, report.licenses_left_out.clone()),
            (SETTINGS_LEFT_OUT_TEXT, report.settings_left_out.clone()),
            (
                CERTIFICATES_LEFT_OUT_TEXT,
                report
                    .certificates
                    .iter()
                    .map(|certificate| item_of(certificate_label(certificate), &certificate.why))
                    .collect(),
            ),
            (
                FONTS_LEFT_OUT_TEXT,
                report
                    .fonts_reported
                    .iter()
                    .map(|font| item_of(font.clone(), "It stays behind."))
                    .collect(),
            ),
            (
                "Drivers that belong to Windows or a vendor, which cannot be passed on, so \
                 reinstall them on the other PC",
                manifest
                    .drivers_detected
                    .iter()
                    .filter(|driver| driver.package_relative_path.is_none())
                    .map(|driver| {
                        item_of(
                            driver.service_name.clone(),
                            "Reinstall it from its vendor's own package.",
                        )
                    })
                    .collect(),
            ),
        ];
        self.left_out = groups
            .into_iter()
            .filter(|(_, items)| !items.is_empty())
            .map(|(title, items)| LeftOutGroup {
                title: title.to_owned(),
                items,
            })
            .collect();
    }

    /// License data in what a person left out. It still goes in, since a license goes by the License
    /// option and nothing else, which a person ought to be told.
    pub fn licenses_in_what_is_left_out(&self, choices: &FootprintChoices) -> Vec<&Path> {
        let folders = LeftOutFolders::of(self, choices);
        self.license_paths
            .iter()
            .map(PathBuf::as_path)
            .filter(|path| folders.holds(&comparable_path(path)))
            .collect()
    }

    /// The rows a choice can leave out, of every kind but folders.
    fn items(&self) -> impl Iterator<Item = &ItemRow> {
        self.services
            .iter()
            .chain(&self.startup)
            .chain(&self.tasks)
            .chain(&self.shortcuts)
    }

    /// The folders that really are left out. A row that is locked cannot be, whatever is asked, and
    /// one that a font decides goes with that.
    fn folders_left_out(&self, choices: &FootprintChoices) -> Vec<&FolderRow> {
        self.folders
            .iter()
            .filter(|row| {
                row.locked.is_none() && row.goes_with.is_none() && choices.is_left_out(&row.id)
            })
            .collect()
    }

    /// Whether a row is in, after what a person chose and what went with it.
    pub fn folder_is_in(&self, row: &FolderRow, choices: &FootprintChoices) -> bool {
        match &row.goes_with {
            Some(decides) => !choices.is_left_out(decides),
            None => row.locked.is_some() || !choices.is_left_out(&row.id),
        }
    }

    /// What can be known of a build from this and a person's choices, which adds up rows and reads
    /// nothing, so it can be done for each tick. The same as a build then works from. Runtimes stay
    /// as a scan found them: what needs one may be a program that is staying, and it costs only room.
    pub fn build_inputs(
        &self,
        choices: &FootprintChoices,
        also: &[AlsoLeftOut],
        stub_bytes: u64,
        bundle_prerequisites: bool,
    ) -> BuildInputs {
        let mut inputs = BuildInputs {
            stub_bytes,
            connected_drive: self.connected_drive,
            prerequisites: if bundle_prerequisites {
                self.prerequisite_count
            } else {
                0
            },
            held_bytes: self.license_bytes.saturating_add(self.refused_driver_bytes),
            ..BuildInputs::default()
        };
        for row in self
            .folders
            .iter()
            .filter(|row| self.folder_is_in(row, choices))
        {
            inputs.payload_bytes = inputs.payload_bytes.saturating_add(row.bytes);
            inputs.payload_files += row.files;
            inputs.largest_file_bytes = inputs.largest_file_bytes.max(row.largest_file_bytes);
        }
        for row in self.items().filter(|row| {
            !choices.is_left_out(&row.id) && !also.iter().any(|also| also.id == row.id)
        }) {
            inputs.held_bytes = inputs.held_bytes.saturating_add(row.bytes);
        }
        inputs
    }
}

/// What ties an app into Windows, by kind, from a list of what is set up.
fn integration_rows(rows: &[ChangeRow]) -> Vec<IntegrationRow> {
    let mut integrations: BTreeMap<SystemChangeKind, Vec<SystemChange>> = BTreeMap::new();
    for ChangeRow { change, .. } in rows {
        if !matches!(
            change.kind,
            SystemChangeKind::Service
                | SystemChangeKind::Driver
                | SystemChangeKind::ScheduledTask
                | SystemChangeKind::Autostart
                | SystemChangeKind::Shortcut
        ) {
            integrations
                .entry(change.kind)
                .or_default()
                .push(change.clone());
        }
    }
    integrations
        .into_iter()
        .map(|(kind, changes)| IntegrationRow {
            kind,
            id: ChoiceId::Integration(kind),
            changes,
        })
        .collect()
}

/// Whether a folder, as [`comparable_path`] writes it, is where Windows keeps fonts.
fn is_the_windows_fonts_folder(folder: &str) -> bool {
    folder
        .split_once(':')
        .is_some_and(|(_, below)| below == WINDOWS_FONTS_BELOW_THE_DRIVE)
}

fn is_data(classification: &FileClassification) -> bool {
    matches!(
        classification,
        FileClassification::UserData | FileClassification::MachineAppData
    )
}

/// What a kind of data folder is, in words.
fn place_of_kind(kind: &FootprintKind) -> Option<&'static str> {
    Some(match kind {
        FootprintKind::RoamingAppData => "Settings in your profile (AppData)",
        FootprintKind::LocalAppData | FootprintKind::LocalLowAppData => {
            "Settings for this PC in your profile (AppData)"
        }
        FootprintKind::ProgramData => "Shared by everyone on the PC (ProgramData)",
        FootprintKind::Documents => "In your Documents",
        FootprintKind::SavedGames => "In your Saved Games",
        FootprintKind::PublicDocuments => "In Public Documents",
        FootprintKind::InstallDirectory
        | FootprintKind::RegistryReference
        | FootprintKind::Other => return None,
    })
}

fn name_of(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// What a person can choose between: a row for each folder directly in the app's own, and for each
/// other place a scan found it. Everything that goes in is on exactly one row.
fn folder_rows(report: &ScanReport) -> Vec<FolderRow> {
    let main_program = &report.main_executable.path;
    let install = report.install_location.as_deref();
    // Longest first, so that one inside another is what a file goes by.
    let mut locations: Vec<&FootprintLocation> = report.footprint_locations.iter().collect();
    locations.sort_by_key(|location| std::cmp::Reverse(location.path.as_os_str().len()));
    let mut rows: BTreeMap<String, FolderRow> = BTreeMap::new();
    for file in report.file_inventory.iter().filter(|file| file.should_copy) {
        let (path, name, place) = row_of(file, install, &locations);
        let key = comparable_path(&path);
        let row = rows.entry(key.clone()).or_insert_with(|| FolderRow {
            id: ChoiceId::Folder(key.clone()),
            name,
            place,
            group: if is_data(&file.classification) {
                FolderGroup::Data
            } else {
                FolderGroup::Program
            },
            files: 0,
            bytes: 0,
            largest_file_bytes: 0,
            locked: None,
            goes_with: is_the_windows_fonts_folder(&key)
                .then_some(ChoiceId::Integration(SystemChangeKind::Font)),
            path,
        });
        let size = file.size_bytes.unwrap_or(0);
        row.files += 1;
        row.bytes = row.bytes.saturating_add(size);
        row.largest_file_bytes = row.largest_file_bytes.max(size);
    }
    let mut rows: Vec<FolderRow> = rows.into_values().collect();
    // What the main program imports is on the nearest row around it, as a file is.
    let needed_on_rows: Vec<String> = report
        .dependencies
        .iter()
        .filter(|dependency| !dependency.delay_loaded)
        .filter_map(|dependency| dependency.resolved_path.as_deref())
        .filter_map(|path| {
            rows.iter()
                .filter(|row| path_is_within(path, &row.path))
                .max_by_key(|row| row.path.as_os_str().len())
                .map(|row| comparable_path(&row.path))
        })
        .collect();
    for row in &mut rows {
        let is_install_folder =
            install.is_some_and(|install| comparable_path(install) == comparable_path(&row.path));
        if is_install_folder {
            row.locked = Some("These always go with the main program");
        } else if path_is_within(main_program, &row.path) {
            row.locked = Some("Holds the main program, which always goes in");
        } else if needed_on_rows.contains(&comparable_path(&row.path)) {
            row.locked = Some("Holds what the main program cannot start without");
        } else if row.goes_with.is_some() {
            row.locked = Some("Goes with its font, under what ties it into Windows");
        }
    }
    rows.sort_by(|left, right| {
        right
            .locked
            .is_some()
            .cmp(&left.locked.is_some())
            .then_with(|| right.bytes.cmp(&left.bytes))
            .then_with(|| left.name.cmp(&right.name))
    });
    rows
}

/// The folder a file is on a row for, what to call it, and where that is.
fn row_of(
    file: &FileEntry,
    install: Option<&Path>,
    locations: &[&FootprintLocation],
) -> (PathBuf, String, String) {
    let logical = &file.original_path;
    if let Some(install) = install
        && let Some(relative) = strip_prefix_ignore_case(logical, install)
    {
        let mut parts = relative.components();
        if let (Some(first), Some(_)) = (parts.next(), parts.next()) {
            let path = install.join(first.as_os_str());
            let name = name_of(&path);
            return (path, name, "In the app's folder".to_owned());
        }
        return (
            install.to_path_buf(),
            format!("Files directly in {}", name_of(install)),
            "In the app's folder".to_owned(),
        );
    }
    if let Some(location) = locations
        .iter()
        .find(|location| path_is_within(logical, &location.path))
    {
        let place = place_of_kind(&location.kind).map_or_else(
            || {
                location
                    .path
                    .parent()
                    .map_or_else(String::new, |parent| format!("In {}", parent.display()))
            },
            ToOwned::to_owned,
        );
        return (location.path.clone(), name_of(&location.path), place);
    }
    let parent = logical
        .parent()
        .map_or_else(|| logical.clone(), Path::to_path_buf);
    let place = parent
        .parent()
        .map_or_else(String::new, |above| format!("In {}", above.display()));
    (parent.clone(), name_of(&parent), place)
}

fn task_key(task_path: &str, task_name: &str) -> String {
    format!("{task_path}{task_name}").to_ascii_lowercase()
}

fn environment_key(entry: &crate::models::EnvironmentVariableEntry) -> String {
    format!("{:?}|{}", entry.scope, entry.name.to_ascii_lowercase())
}

fn firewall_key(entry: &crate::models::FirewallRuleEntry) -> String {
    format!(
        "{}|{}|{}",
        entry.name.to_ascii_lowercase(),
        entry.direction.to_ascii_lowercase(),
        comparable_path(&entry.program)
    )
}

fn registry_key_of(entry: &RegistryEntry) -> String {
    registry_item(entry).to_ascii_lowercase()
}

fn allowed_by(review: &ReviewedPackage<'_>) -> Allowed {
    Allowed {
        services: review
            .services
            .iter()
            .map(|service| service.entry.service_name.to_ascii_lowercase())
            .collect(),
        drivers: review
            .drivers
            .iter()
            .map(|driver| driver.entry.service_name.to_ascii_lowercase())
            .collect(),
        tasks: review
            .tasks
            .iter()
            .map(|task| task_key(&task.entry.task_path, &task.entry.task_name))
            .collect(),
        shortcuts: review
            .shortcuts
            .iter()
            .map(|shortcut| {
                shortcut
                    .original_lnk_path
                    .to_string_lossy()
                    .to_ascii_lowercase()
            })
            .collect(),
        registry: review
            .registry
            .iter()
            .copied()
            .map(registry_key_of)
            .collect(),
        environment: review
            .environment
            .iter()
            .copied()
            .map(environment_key)
            .collect(),
        firewall: review
            .firewall_rules
            .iter()
            .map(|rule| firewall_key(rule.entry))
            .collect(),
    }
}

/// The reason a review gave for leaving something out, in its own words.
fn reasons_of(review: &ReviewedPackage<'_>) -> HashMap<String, String> {
    review
        .refused
        .iter()
        .filter_map(|warning| {
            let item = warning.source.clone()?;
            let reason = warning
                .message
                .split_once(LEFT_OUT_MARKER)
                .map(|(_, reason)| reason.trim_end_matches('.').to_owned())?;
            Some((item, reason))
        })
        .collect()
}

/// What in a service's settings says it is a program it runs, and not a folder it keeps things in.
const RUNS_WHAT_IS_NAMED: &[&str] = &[
    ".exe", ".dll", ".cmd", ".bat", ".ps1", ".py", ".js", ".vbs", ".jar",
];

/// Whether a service's setting has the name of a program in it. `.js` is not `.json`.
fn names_something_it_runs(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    RUNS_WHAT_IS_NAMED.iter().any(|end| {
        lower.match_indices(end).any(|(at, _)| {
            !lower[at + end.len()..].starts_with(|next: char| next.is_ascii_alphanumeric())
        })
    })
}

/// What is left of a scan after a person's choices, as a package would have it, and what went with
/// what they left out.
struct Decision {
    manifest: PackageManifest,
    also: Vec<AlsoLeftOut>,
    integrations: Vec<IntegrationRow>,
    folders: LeftOutFolders,
}

/// Whether a path, as [`comparable_path`] writes it, is in a folder written that way.
fn is_in_folder(path: &str, folder: &str) -> bool {
    path.strip_prefix(folder)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
}

/// The folders a person left out. One of those can have a row of its own inside it, as a service's
/// folder inside a data folder can, which still says it goes in. So a file goes by the nearest row
/// around it, as it was added up on the screen.
struct LeftOutFolders {
    /// By how long each is, longest first, with whether it is left out.
    rows: Vec<(String, bool)>,
    /// Those that are, for looking for in text.
    left_out: Vec<String>,
}

impl LeftOutFolders {
    fn of(footprint: &Footprint, choices: &FootprintChoices) -> Self {
        let left_out: Vec<String> = footprint
            .folders_left_out(choices)
            .iter()
            .map(|row| comparable_path(&row.path))
            .collect();
        let mut rows: Vec<(String, bool)> = footprint
            .folders
            .iter()
            .map(|row| comparable_path(&row.path))
            .filter(|path| left_out.iter().any(|folder| is_in_folder(path, folder)))
            .map(|path| {
                let is_left_out = left_out.contains(&path);
                (path, is_left_out)
            })
            .collect();
        rows.sort_by_key(|(path, _)| std::cmp::Reverse(path.len()));
        Self { rows, left_out }
    }

    /// Whether a path, as [`comparable_path`] writes it, goes with one of these.
    fn holds(&self, path: &str) -> bool {
        self.rows
            .iter()
            .find(|(folder, _)| is_in_folder(path, folder))
            .is_some_and(|(_, is_left_out)| *is_left_out)
    }

    /// Whether a value naming a path names something in one of these. Only as a whole path, so
    /// `C:\\App Extras` is not `C:\\App`.
    fn are_named_by(&self, text: &str) -> bool {
        self.are_mentioned(text, false)
    }

    /// The same of a command line, where a space ends a path as well.
    fn are_named_in_a_command(&self, text: &str) -> bool {
        self.are_mentioned(text, true)
    }

    /// Whether text has a path in it that goes with one of these, which by [`Self::holds`] one in a
    /// row of its own inside one does not.
    fn are_mentioned(&self, text: &str, followed_by_arguments: bool) -> bool {
        if !self
            .left_out
            .iter()
            .any(|folder| text_mentions_path(text, Path::new(folder), followed_by_arguments))
        {
            return false;
        }
        let ends_a_path = |character: char| {
            matches!(character, '"' | '\'' | ';' | ',' | '\0' | '\r' | '\n')
                || (followed_by_arguments && matches!(character, ' ' | '\t'))
        };
        // Both steps swap single bytes for single bytes, as `text_mentions_path` relies on too.
        let lower = text.to_ascii_lowercase().replace('/', "\\");
        self.left_out.iter().any(|folder| {
            let mut from = 0;
            while let Some(found) = lower[from..].find(folder.as_str()) {
                let start = from + found;
                let end = start + folder.len();
                // In quotes a path goes on to the closing one, spaces and all.
                let quote = lower[..start]
                    .chars()
                    .next_back()
                    .filter(|before| matches!(before, '"' | '\''));
                let ends_this_path = |character: char| match quote {
                    Some(quote) => character == quote || matches!(character, '\0' | '\r' | '\n'),
                    None => ends_a_path(character),
                };
                let is_whole = lower[end..]
                    .chars()
                    .next()
                    .is_none_or(|next| next == '\\' || ends_this_path(next));
                if is_whole {
                    let path_end = lower[end..]
                        .find(ends_this_path)
                        .map_or(lower.len(), |ahead| end + ahead);
                    if self.holds(lower[start..path_end].trim_end_matches('\\')) {
                        return true;
                    }
                }
                from = end;
            }
            false
        })
    }

    /// The same of a task's definition. What it runs and where are one path each, spaces and all,
    /// which is only so of those: its arguments are a command line. Anything else that names one is
    /// counted too, as a tag can hold a path in any way, and so is all of what cannot be read.
    fn are_named_in_xml(&self, xml: &str) -> bool {
        let Ok(document) = roxmltree::Document::parse(xml) else {
            return self.are_named_in_a_command(&xml.replace('<', "\n"));
        };
        document
            .descendants()
            .filter(|node| node.is_text())
            .any(|node| {
                let text = node.text().unwrap_or_default();
                let is_one_path = node
                    .parent_element()
                    .is_some_and(|tag| TASK_TAGS_OF_ONE_PATH.contains(&tag.tag_name().name()));
                if is_one_path {
                    self.are_named_by(text.trim().trim_matches('"'))
                } else {
                    self.are_named_in_a_command(text)
                }
            })
    }
}

fn decide(
    report: &ScanReport,
    footprint: &Footprint,
    choices: &FootprintChoices,
) -> Result<Decision> {
    let folders = LeftOutFolders::of(footprint, choices);
    let fonts_out = choices.is_left_out(&ChoiceId::Integration(SystemChangeKind::Font));
    let keep_file =
        |file: &FileEntry| keeps_path(&comparable_path(&file.original_path), &folders, fonts_out);
    let mut manifest = manifest_for_review(report, footprint.flags, &keep_file);
    let mut also = Vec::new();
    take_out_what_was_chosen(&mut manifest, choices);
    take_out_what_names_a_folder_left_out(&mut manifest, &folders, &mut also);
    // Only what this choice made it refuse. What it refuses anyway is for an installer to say.
    let mut integrations = Vec::new();
    for _ in 0..MOST_PASSES {
        let needing = take_out_what_needs_what_is_out(&mut manifest, report);
        let took_what_needed = !needing.is_empty();
        also.extend(needing);
        let review = ReviewedPackage::before_a_location_is_chosen(
            &manifest,
            footprint.flags.include_license_data,
        )?;
        let now = allowed_by(&review);
        let reasons = reasons_of(&review);
        integrations = integration_rows(&described_rows(&review, Path::new(NO_TARGET_YET)));
        drop(review);
        let taken = take_out_what_is_now_refused(&mut manifest, &footprint.allowed, &now, &reasons);
        if taken.is_empty() && !took_what_needed {
            break;
        }
        also.extend(taken);
    }
    Ok(Decision {
        manifest,
        also,
        integrations,
        folders,
    })
}

/// A service that needs another that will not be there does not start, so it goes too.
fn take_out_what_needs_what_is_out(
    manifest: &mut PackageManifest,
    report: &ScanReport,
) -> Vec<AlsoLeftOut> {
    let lower = |name: &String| name.to_ascii_lowercase();
    let kept: HashSet<String> = manifest
        .services_detected
        .iter()
        .map(|service| lower(&service.service_name))
        .chain(
            manifest
                .drivers_detected
                .iter()
                .map(|driver| lower(&driver.service_name)),
        )
        .collect();
    let out: HashSet<String> = report
        .services
        .iter()
        .map(|service| lower(&service.service_name))
        .chain(
            report
                .drivers
                .iter()
                .map(|driver| lower(&driver.service_name)),
        )
        .filter(|name| !kept.contains(name))
        .collect();
    let mut also = Vec::new();
    if out.is_empty() {
        return also;
    }
    manifest.services_detected.retain(|service| {
        let needed = service
            .dependencies
            .iter()
            .find(|needed| out.contains(&lower(needed)));
        if let Some(needed) = needed {
            also.push(AlsoLeftOut {
                id: service_choice(&service.service_name),
                title: service
                    .display_name
                    .clone()
                    .unwrap_or_else(|| service.service_name.clone()),
                because: format!("it needs {needed}, which is left out"),
            });
        }
        needed.is_none()
    });
    also
}

/// Whether a file at this path, as [`comparable_path`] writes it, goes in.
fn keeps_path(path: &str, folders: &LeftOutFolders, fonts_out: bool) -> bool {
    !folders.holds(path)
        && !(fonts_out
            && path
                .rsplit_once('\\')
                .is_some_and(|(folder, _)| is_the_windows_fonts_folder(folder)))
}

fn take_out_what_was_chosen(manifest: &mut PackageManifest, choices: &FootprintChoices) {
    let has = |id: &ChoiceId| choices.is_left_out(id);
    manifest
        .services_detected
        .retain(|service| !has(&service_choice(&service.service_name)));
    manifest
        .drivers_detected
        .retain(|driver| !has(&driver_choice(&driver.service_name)));
    manifest
        .scheduled_tasks_detected
        .retain(|task| !has(&task_choice(&task.task_path, &task.task_name)));
    manifest
        .shortcuts_detected
        .retain(|shortcut| !has(&shortcut_choice(&shortcut.original_lnk_path)));
    let services_out: Vec<String> = choices
        .left_out
        .iter()
        .filter_map(|id| match id {
            ChoiceId::Service(name) | ChoiceId::Driver(name) => {
                Some(format!("{SERVICES_KEY_PREFIX}{name}"))
            }
            _ => None,
        })
        .collect();
    manifest.registry_keys_exported.retain(|entry| {
        let is_under_a_service = entry.hive == RegistryHive::LocalMachine && {
            let key = entry.key_path.to_ascii_lowercase();
            services_out.iter().any(|service| {
                key.strip_prefix(service.as_str())
                    .is_some_and(|rest| rest.is_empty() || rest.starts_with('\\'))
            })
        };
        !is_under_a_service
            && !(is_autostart_entry(entry) && has(&autostart_choice(entry)))
            && !change_kind_of(entry).is_some_and(|kind| has(&ChoiceId::Integration(kind)))
    });
    if has(&ChoiceId::Integration(SystemChangeKind::FirewallRule)) {
        manifest.firewall_rules_detected.clear();
    }
    if has(&ChoiceId::Integration(SystemChangeKind::ComServer)) {
        manifest.com_registrations_detected.clear();
    }
    if has(&ChoiceId::Integration(SystemChangeKind::FileType)) {
        manifest.file_associations_detected.clear();
    }
    manifest.environment_variables_detected.retain(|variable| {
        let kind = if is_path_variable(&variable.name) {
            SystemChangeKind::PathEntry
        } else {
            SystemChangeKind::EnvironmentVariable
        };
        !has(&ChoiceId::Integration(kind))
    });
}

/// What the rules go by is what runs. So what is given a path as an argument is not refused for
/// that path being gone, and a list of folders in a variable is merged whatever is in it. Neither
/// should be set up naming what was left out. A plain setting of the app's own that names one stays:
/// it is only what the app reads, and there is no telling what it does about that.
fn take_out_what_names_a_folder_left_out(
    manifest: &mut PackageManifest,
    folders: &LeftOutFolders,
    also: &mut Vec<AlsoLeftOut>,
) {
    if folders.left_out.is_empty() {
        return;
    }
    const WHY: &str = "it is told to use something in a folder you left out";
    manifest.services_detected.retain(|service| {
        // What some services run is in their settings, and they run nothing without it. Any other
        // setting that names such a folder stays, as one of the app's own does.
        let names_one = service
            .arguments
            .as_deref()
            .is_some_and(|arguments| folders.are_named_in_a_command(arguments))
            || service.parameters.iter().any(|value| {
                value.value_data.as_deref().is_some_and(|data| {
                    names_something_it_runs(data) && folders.are_named_in_a_command(data)
                })
            });
        if names_one {
            also.push(AlsoLeftOut {
                id: service_choice(&service.service_name),
                title: service
                    .display_name
                    .clone()
                    .unwrap_or_else(|| service.service_name.clone()),
                because: WHY.to_owned(),
            });
        }
        !names_one
    });
    manifest.drivers_detected.retain(|driver| {
        let is_in_one = folders.holds(&comparable_path(&driver.image_path));
        if is_in_one {
            also.push(AlsoLeftOut {
                id: driver_choice(&driver.service_name),
                title: driver.service_name.clone(),
                because: "its file is in a folder you left out".to_owned(),
            });
        }
        !is_in_one
    });
    manifest.scheduled_tasks_detected.retain(|task| {
        let names_one = task
            .arguments
            .as_deref()
            .is_some_and(|arguments| folders.are_named_in_a_command(arguments))
            || task
                .definition_xml
                .as_deref()
                .is_some_and(|xml| folders.are_named_in_xml(xml));
        if names_one {
            also.push(AlsoLeftOut {
                id: task_choice(&task.task_path, &task.task_name),
                title: format!("{}{}", task.task_path, task.task_name),
                because: WHY.to_owned(),
            });
        }
        !names_one
    });
    manifest.shortcuts_detected.retain_mut(|shortcut| {
        let names_one = shortcut
            .arguments
            .as_deref()
            .is_some_and(|arguments| folders.are_named_in_a_command(arguments))
            || shortcut
                .working_directory
                .as_deref()
                .is_some_and(|folder| folders.are_named_by(&folder.to_string_lossy()));
        // Without its icon it is still a shortcut.
        if shortcut
            .icon_path
            .as_deref()
            .is_some_and(|icon| folders.are_named_in_a_command(&icon.to_string_lossy()))
        {
            shortcut.icon_path = None;
        }
        if names_one {
            also.push(AlsoLeftOut {
                id: shortcut_choice(&shortcut.original_lnk_path),
                title: name_of(&shortcut.original_lnk_path),
                because: WHY.to_owned(),
            });
        }
        !names_one
    });
    manifest.registry_keys_exported.retain(|entry| {
        let names_one = (is_autostart_entry(entry) || change_kind_of(entry).is_some())
            && entry
                .value_data
                .as_deref()
                .is_some_and(|data| folders.are_named_in_a_command(data));
        if names_one {
            also.push(left_out_value(entry, WHY));
        }
        !names_one
    });
    manifest.firewall_rules_detected.retain(|rule| {
        let is_in_one = folders.holds(&comparable_path(&rule.program));
        if is_in_one {
            also.push(firewall_rule_left_out(
                rule,
                "the program it is for is in a folder you left out",
            ));
        }
        !is_in_one
    });
    manifest
        .environment_variables_detected
        .retain_mut(|variable| {
            if !folders.are_named_by(&variable.value) {
                return true;
            }
            if is_path_variable(&variable.name) {
                let kept: Vec<&str> = variable
                    .value
                    .split(';')
                    .filter(|entry| !folders.are_named_by(entry))
                    .collect();
                variable.value = kept.join(";");
                return true;
            }
            also.push(AlsoLeftOut {
                id: ChoiceId::Integration(SystemChangeKind::EnvironmentVariable),
                title: variable.name.clone(),
                because: "it names a folder you left out".to_owned(),
            });
            false
        });
}

fn firewall_rule_left_out(rule: &crate::models::FirewallRuleEntry, because: &str) -> AlsoLeftOut {
    AlsoLeftOut {
        id: ChoiceId::Integration(SystemChangeKind::FirewallRule),
        title: rule.name.clone(),
        because: because.to_owned(),
    }
}

/// One value of something set up, by what can leave it out.
fn left_out_value(entry: &RegistryEntry, because: &str) -> AlsoLeftOut {
    let id = if is_autostart_entry(entry) {
        autostart_choice(entry)
    } else {
        ChoiceId::Integration(
            change_kind_of(entry).unwrap_or(SystemChangeKind::EnvironmentVariable),
        )
    };
    AlsoLeftOut {
        id,
        title: entry
            .value_name
            .clone()
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| entry.key_path.clone()),
        because: because.to_owned(),
    }
}

/// Take out what a review allowed before this choice and does not now, saying why. A value that is
/// not on any list only goes with what it was under, so nothing needs saying of it.
fn take_out_what_is_now_refused(
    manifest: &mut PackageManifest,
    before: &Allowed,
    now: &Allowed,
    reasons: &HashMap<String, String>,
) -> Vec<AlsoLeftOut> {
    let mut also = Vec::new();
    let gone = |key: &str, before: &BTreeSet<String>, now: &BTreeSet<String>| {
        before.contains(key) && !now.contains(key)
    };
    manifest.services_detected.retain(|service| {
        let key = service.service_name.to_ascii_lowercase();
        let is_gone = gone(&key, &before.services, &now.services);
        if is_gone {
            let item = format!("The service {}", service.service_name);
            also.push(AlsoLeftOut {
                id: service_choice(&service.service_name),
                title: service
                    .display_name
                    .clone()
                    .unwrap_or_else(|| service.service_name.clone()),
                because: reasons.get(&item).cloned().unwrap_or_default(),
            });
        }
        !is_gone
    });
    manifest.scheduled_tasks_detected.retain(|task| {
        let key = task_key(&task.task_path, &task.task_name);
        let is_gone = gone(&key, &before.tasks, &now.tasks);
        if is_gone {
            let item = format!("The scheduled task {}{}", task.task_path, task.task_name);
            also.push(AlsoLeftOut {
                id: task_choice(&task.task_path, &task.task_name),
                title: format!("{}{}", task.task_path, task.task_name),
                because: reasons.get(&item).cloned().unwrap_or_default(),
            });
        }
        !is_gone
    });
    manifest.shortcuts_detected.retain(|shortcut| {
        let key = shortcut
            .original_lnk_path
            .to_string_lossy()
            .to_ascii_lowercase();
        let is_gone = gone(&key, &before.shortcuts, &now.shortcuts);
        if is_gone {
            let item = format!("The shortcut {}", shortcut.original_lnk_path.display());
            also.push(AlsoLeftOut {
                id: shortcut_choice(&shortcut.original_lnk_path),
                title: name_of(&shortcut.original_lnk_path),
                because: reasons.get(&item).cloned().unwrap_or_default(),
            });
        }
        !is_gone
    });
    manifest.registry_keys_exported.retain(|entry| {
        let key = registry_key_of(entry);
        let is_gone = gone(&key, &before.registry, &now.registry);
        if is_gone && (is_autostart_entry(entry) || change_kind_of(entry).is_some()) {
            let because = reasons
                .get(&registry_item(entry))
                .cloned()
                .unwrap_or_default();
            also.push(left_out_value(entry, &because));
        }
        !is_gone
    });
    manifest.firewall_rules_detected.retain(|rule| {
        let is_gone = gone(&firewall_key(rule), &before.firewall, &now.firewall);
        if is_gone {
            let item = format!("The firewall rule {}", rule.name);
            let because = reasons.get(&item).cloned().unwrap_or_default();
            also.push(firewall_rule_left_out(rule, &because));
        }
        !is_gone
    });
    manifest.environment_variables_detected.retain(|variable| {
        let key = environment_key(variable);
        !gone(&key, &before.environment, &now.environment)
    });
    also
}

/// What went with what a person left out, for saying so as they choose.
pub fn what_goes_with(
    report: &ScanReport,
    footprint: &Footprint,
    choices: &FootprintChoices,
) -> Result<WhatGoes> {
    if choices.is_empty() {
        return Ok(WhatGoes {
            also: Vec::new(),
            integrations: footprint.integrations.clone(),
        });
    }
    let decision = decide(report, footprint, choices)?;
    Ok(WhatGoes {
        also: decision.also,
        integrations: decision.integrations,
    })
}

/// A scan with what a person chose applied, as what a package is built from.
#[derive(Debug, Clone)]
pub struct ChosenScan {
    pub report: ScanReport,
    pub left_out: Vec<LeftOutItem>,
    pub added: Vec<String>,
    pub also: Vec<AlsoLeftOut>,
    /// What there is left of what the options brought in, which is what an installer goes by.
    pub has_user_data: bool,
    pub has_program_data: bool,
}

/// Apply a person's choices to a scan. Nothing that a scan did not find can come of this, since all
/// it does is take away from it.
pub fn apply_choices(
    report: &ScanReport,
    footprint: &Footprint,
    choices: &FootprintChoices,
) -> Result<ChosenScan> {
    let Decision {
        manifest,
        also,
        folders,
        ..
    } = decide(report, footprint, choices)?;
    let mut chosen = report.clone();
    let fonts_out = choices.is_left_out(&ChoiceId::Integration(SystemChangeKind::Font));
    let keep_path = |path: &str| keeps_path(path, &folders, fonts_out);
    chosen
        .file_inventory
        .retain(|file| keep_path(&comparable_path(&file.original_path)));
    chosen
        .empty_folders
        .retain(|folder| keep_path(&comparable_path(&folder.original_path)));
    chosen
        .app_local_dlls
        .retain(|file| keep_path(&comparable_path(&file.original_path)));
    let kept_drivers: HashSet<String> = manifest
        .drivers_detected
        .iter()
        .map(|driver| driver.service_name.to_ascii_lowercase())
        .collect();
    chosen
        .drivers
        .retain(|driver| kept_drivers.contains(&driver.service_name.to_ascii_lowercase()));
    chosen.services = manifest.services_detected;
    chosen.scheduled_tasks = manifest.scheduled_tasks_detected;
    chosen.shortcuts = manifest.shortcuts_detected;
    chosen.registry_entries = manifest.registry_keys_exported;
    chosen.firewall_rules = manifest.firewall_rules_detected;
    chosen.environment_variables = manifest.environment_variables_detected;
    chosen.com_registrations = manifest.com_registrations_detected;
    chosen.com_registrations.retain(|registration| {
        registration
            .server_path
            .as_deref()
            .is_none_or(|server| keep_path(&comparable_path(server)))
    });
    chosen.file_associations = manifest.file_associations_detected;
    chosen.file_associations.retain(|association| {
        association
            .command
            .as_deref()
            .is_none_or(|command| !folders.are_named_in_a_command(command))
    });
    // What a program needs of a PC only matters for a program that is going there.
    let programs: HashSet<String> = chosen
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .map(|file| comparable_path(&file.original_path))
        .chain(
            chosen
                .drivers
                .iter()
                .map(|driver| comparable_path(&driver.image_path)),
        )
        .collect();
    chosen.program_needs.retain(|need| {
        need.role == ProgramRole::Main || programs.contains(&comparable_path(&need.path))
    });
    chosen.warnings.retain(|warning| {
        warning
            .source
            .as_deref()
            .is_none_or(|source| !folders.holds(&comparable_path(Path::new(source))))
    });
    recount_locations(&mut chosen);
    // Not a folder added and then left out, which its row can be. An entry brings more than its
    // folder, which still goes in.
    let added = report
        .near_matches
        .iter()
        .filter(|near| {
            near.picked
                && (near.kind != crate::models::NearMatchKind::Folder
                    || near
                        .path
                        .as_deref()
                        .is_none_or(|path| !folders.holds(&comparable_path(path))))
        })
        .map(|near| near.name.clone())
        .collect();
    let left_out = what_was_left_out(footprint, choices, &also);
    let has = |classification: FileClassification| {
        chosen
            .file_inventory
            .iter()
            .chain(&chosen.empty_folders)
            .any(|file| file.should_copy && file.classification == classification)
    };
    // What an account keeps for the app makes this a package holding that account's data, even with
    // none of its folders left.
    let has_user_data =
        has(FileClassification::UserData) || chosen.registry_entries.iter().any(is_account_setting);
    let has_program_data = has(FileClassification::MachineAppData);
    Ok(ChosenScan {
        report: chosen,
        left_out,
        added,
        also,
        has_user_data,
        has_program_data,
    })
}

/// What a report says of each place, after some of what is in them has gone.
fn recount_locations(report: &mut ScanReport) {
    let mut counts: Vec<(usize, u64, u64)> = vec![(0, 0, 0); report.footprint_locations.len()];
    for file in &report.file_inventory {
        let found = report
            .footprint_locations
            .iter()
            .position(|location| path_is_within(&file.original_path, &location.path));
        if let Some(found) = found {
            let size = file.size_bytes.unwrap_or(0);
            let count = &mut counts[found];
            count.0 += 1;
            count.1 = count.1.saturating_add(size);
            if file.should_copy {
                count.2 = count.2.saturating_add(size);
            }
        }
    }
    let mut counts = counts.into_iter();
    report.footprint_locations.retain_mut(|location| {
        let (files, total, copyable) = counts.next().unwrap_or_default();
        location.file_count = files;
        location.total_bytes = total;
        location.copyable_bytes = copyable;
        location.included = copyable > 0;
        files > 0
    });
}

/// What to tell the person installing of what was chosen, by name and in the same words.
fn what_was_left_out(
    footprint: &Footprint,
    choices: &FootprintChoices,
    also: &[AlsoLeftOut],
) -> Vec<LeftOutItem> {
    const WHY: &str = "whoever built this installer left it out";
    let mut left_out: Vec<LeftOutItem> = footprint
        .folders_left_out(choices)
        .iter()
        .map(|row| LeftOutItem {
            name: row.path.display().to_string(),
            why: WHY.to_owned(),
        })
        .collect();
    left_out.extend(
        footprint
            .items()
            .filter(|row| choices.is_left_out(&row.id))
            .map(|row| LeftOutItem {
                name: format!("{} {}", row.change.kind.label(), row.change.title),
                why: WHY.to_owned(),
            }),
    );
    left_out.extend(
        footprint
            .integrations
            .iter()
            .filter(|row| choices.is_left_out(&row.id))
            .map(|row| LeftOutItem {
                name: row.kind.label().to_owned(),
                why: WHY.to_owned(),
            }),
    );
    left_out.extend(also.iter().map(|item| LeftOutItem {
        name: item.title.clone(),
        why: item.because.clone(),
    }));
    left_out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AppArchitecture, EnvironmentScope, EnvironmentVariableEntry, ExecutableMetadata,
        FirewallRuleEntry, InstallScope, InstalledApp, MatchConfidence, ProgramNeed,
        RegistryValueKind, RuntimePrerequisite, RuntimePrerequisiteKind, SCAN_SCHEMA_VERSION,
        ScheduledTaskEntry, ServiceEntry, ShortcutEntry, SignatureStatus,
    };
    use crate::package::manifest_for_review;
    use crate::restore_review::system_changes;

    const INSTALL: &str = r"C:\Program Files\Acme";
    const FLAGS: ScanFlags = ScanFlags {
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
    };

    fn file(relative: &str, size: u64) -> FileEntry {
        let original = PathBuf::from(INSTALL).join(relative);
        FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(PathBuf::from("files").join(relative)),
            suggested_destination_path: None,
            size_bytes: Some(size),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: "a reason".to_owned(),
            user_folder: None,
        }
    }

    fn value(key: &str, name: Option<&str>, data: &str) -> RegistryEntry {
        RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: key.to_owned(),
            value_name: name.map(ToOwned::to_owned),
            value_data: Some(data.to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        }
    }

    fn service(name: &str, relative: &str, arguments: Option<&str>) -> ServiceEntry {
        ServiceEntry {
            service_name: name.to_owned(),
            display_name: Some(format!("{name} Display")),
            binary_path: PathBuf::from(INSTALL).join(relative),
            arguments: arguments.map(ToOwned::to_owned),
            parameters: Vec::new(),
            start_type: Some("automatic".to_owned()),
            account_name: Some("LocalSystem".to_owned()),
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: String::new(),
        }
    }

    fn task(name: &str, relative: &str) -> ScheduledTaskEntry {
        let command = PathBuf::from(INSTALL).join(relative);
        ScheduledTaskEntry {
            task_name: name.to_owned(),
            task_path: r"\Acme\".to_owned(),
            action_command: command.clone(),
            arguments: None,
            working_directory: None,
            trigger_summary: None,
            run_level: None,
            account: None,
            requires_admin: true,
            restore_instructions: String::new(),
            definition_xml: Some(format!(
                "<?xml version=\"1.0\"?><Task><Principals><Principal><UserId>S-1-5-18</UserId>\
                 </Principal></Principals><Actions><Exec><Command>{}</Command></Exec></Actions>\
                 </Task>",
                command.display()
            )),
        }
    }

    fn report() -> ScanReport {
        let main = PathBuf::from(INSTALL).join(r"bin\acme.exe");
        ScanReport {
            schema_version: SCAN_SCHEMA_VERSION.to_owned(),
            generated_at_utc: String::new(),
            source_volume: None,
            other_accounts: Default::default(),
            source_computer_name: None,
            source_windows_version: None,
            source_platform: None,
            program_needs: vec![
                ProgramNeed {
                    path: main.clone(),
                    role: ProgramRole::Main,
                    machine: 0x8664,
                    min_windows_major: 10,
                    min_windows_minor: 0,
                    is_managed: false,
                },
                ProgramNeed {
                    path: PathBuf::from(INSTALL).join(r"plugins\host.exe"),
                    role: ProgramRole::Other,
                    machine: 0xAA64,
                    min_windows_major: 10,
                    min_windows_minor: 0,
                    is_managed: false,
                },
            ],
            user_data_profile: None,
            source_app: Some(InstalledApp {
                display_name: "Acme".to_owned(),
                publisher: Some("Acme Inc.".to_owned()),
                version: None,
                install_location: Some(PathBuf::from(INSTALL)),
                uninstall_string: None,
                main_executable: Some(main.clone()),
                registry_source_path: r"HKLM\Test\Acme".to_owned(),
                architecture: AppArchitecture::X64,
                install_scope: InstallScope::PerMachine,
                owner_profile: None,
            }),
            main_executable: ExecutableMetadata {
                path: main.clone(),
                file_name: "acme.exe".to_owned(),
                file_size_bytes: 400,
                modified_utc: None,
                version_info: None,
                digital_signature: SignatureStatus::NotChecked,
                external_manifest: None,
                is_dotnet: false,
                dotnet_runtime_version: None,
                dotnet_target_framework: None,
                embedded_manifest: false,
                requested_execution_level: None,
                sxs_dependencies: Vec::new(),
            },
            architecture: AppArchitecture::X64,
            install_location: Some(PathBuf::from(INSTALL)),
            install_method: None,
            authoritative_file_count: 0,
            install_evidence: Vec::new(),
            footprint_locations: vec![FootprintLocation {
                path: PathBuf::from(INSTALL),
                kind: FootprintKind::InstallDirectory,
                scope: InstallScope::PerMachine,
                file_count: 6,
                total_bytes: 0,
                copyable_bytes: 0,
                included: true,
            }],
            file_inventory: vec![
                file(r"bin\acme.exe", 400),
                file(r"bin\svc.exe", 100),
                file(r"plugins\host.exe", 50),
                file(r"plugins\a.dll", 7),
                file(r"samples\one.txt", 3),
                file("readme.txt", 1),
            ],
            dependencies: Vec::new(),
            app_local_dlls: Vec::new(),
            registry_entries: vec![
                value(r"Software\Acme", Some("Theme"), "dark"),
                value(
                    r"Software\Acme",
                    Some("PluginFolder"),
                    &format!(r"{INSTALL}\plugins"),
                ),
                value(
                    r"Software\Microsoft\Windows\CurrentVersion\Run",
                    Some("Acme Tray"),
                    &format!(r#""{INSTALL}\plugins\host.exe" --tray"#),
                ),
                value(
                    r"Software\Microsoft\Windows\CurrentVersion\Run",
                    Some("Acme Main"),
                    &format!(r#""{INSTALL}\bin\acme.exe""#),
                ),
                value(r"Software\Classes\.acme", None, "Acme.Document"),
                value(
                    r"Software\Classes\Acme.Document\shell\open\command",
                    None,
                    &format!(r#""{INSTALL}\bin\acme.exe" "%1""#),
                ),
            ],
            com_registrations: Vec::new(),
            file_associations: Vec::new(),
            services: vec![
                service("AcmeMain", r"bin\svc.exe", None),
                service("AcmePlugins", r"plugins\host.exe", None),
                service(
                    "AcmeConfigured",
                    r"bin\svc.exe",
                    Some(&format!(r#"--config "{INSTALL}\samples\one.txt""#)),
                ),
            ],
            drivers: Vec::new(),
            scheduled_tasks: vec![
                task("Daily", r"bin\acme.exe"),
                task("Plugins", r"plugins\host.exe"),
            ],
            shortcuts: vec![
                ShortcutEntry {
                    original_lnk_path: PathBuf::from(
                        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Acme.lnk",
                    ),
                    target_path: main,
                    arguments: None,
                    working_directory: None,
                    icon_path: None,
                    target_is_selected_app: true,
                },
                ShortcutEntry {
                    original_lnk_path: PathBuf::from(
                        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Acme Plugins.lnk",
                    ),
                    target_path: PathBuf::from(INSTALL).join(r"plugins\host.exe"),
                    arguments: None,
                    working_directory: None,
                    icon_path: None,
                    target_is_selected_app: false,
                },
            ],
            environment_variables: vec![
                EnvironmentVariableEntry {
                    scope: EnvironmentScope::Machine,
                    name: "Path".to_owned(),
                    value: format!(r"C:\Windows;{INSTALL}\bin;{INSTALL}\plugins"),
                    value_kind: RegistryValueKind::String,
                    requires_admin: true,
                    confidence: MatchConfidence::High,
                },
                EnvironmentVariableEntry {
                    scope: EnvironmentScope::Machine,
                    name: "ACME_PLUGINS".to_owned(),
                    value: format!(r"{INSTALL}\plugins"),
                    value_kind: RegistryValueKind::String,
                    requires_admin: true,
                    confidence: MatchConfidence::High,
                },
            ],
            runtime_prerequisites: vec![RuntimePrerequisite {
                name: "A runtime".to_owned(),
                kind: RuntimePrerequisiteKind::VisualCppRedistributable,
                evidence: String::new(),
                install_manually: true,
                present_on_source: None,
                bundle: None,
                presence_probe: None,
                architecture: Some(AppArchitecture::X64),
            }],
            license_artifacts: Vec::new(),
            license_registry_entries: Vec::new(),
            secrets_left_out: vec![LeftOutItem {
                name: format!(r"{INSTALL}\credentials.json"),
                why: "its name says what it holds".to_owned(),
            }],
            licenses_left_out: vec![LeftOutItem {
                name: format!(r"{INSTALL}\acme.lic"),
                why: "the License option is off".to_owned(),
            }],
            settings_left_out: Vec::new(),
            empty_folders: Vec::new(),
            firewall_rules: vec![
                firewall_rule("Acme Main", r"bin\svc.exe"),
                firewall_rule("Acme Plugins", r"plugins\host.exe"),
            ],
            certificates: Vec::new(),
            fonts_reported: Vec::new(),
            near_matches: Vec::new(),
            warnings: vec![Warning::new(
                "driver_file_not_copied",
                WarningSeverity::Info,
                "About a file in a folder.",
                Some(format!(r"{INSTALL}\plugins\a.sys")),
            )],
            items_excluded: Vec::new(),
            confidence_score: 90,
            generated_artifacts: None,
        }
    }

    fn firewall_rule(name: &str, relative: &str) -> FirewallRuleEntry {
        FirewallRuleEntry {
            name: name.to_owned(),
            description: None,
            direction: "In".to_owned(),
            action: "Allow".to_owned(),
            enabled: true,
            program: PathBuf::from(INSTALL).join(relative),
            service: None,
            protocol: None,
            local_ports: Vec::new(),
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: Vec::new(),
            profiles: Vec::new(),
            not_carried: Vec::new(),
        }
    }

    fn folder(relative: &str) -> ChoiceId {
        ChoiceId::Folder(comparable_path(&PathBuf::from(INSTALL).join(relative)))
    }

    fn leaving_out(ids: &[ChoiceId]) -> FootprintChoices {
        FootprintChoices {
            left_out: ids.iter().cloned().collect(),
        }
    }

    fn chosen(ids: &[ChoiceId]) -> Result<ChosenScan> {
        let report = report();
        let footprint = Footprint::of(&report, FLAGS)?;
        apply_choices(&report, &footprint, &leaving_out(ids))
    }

    #[test]
    fn everything_that_goes_in_is_on_exactly_one_row() -> Result<()> {
        let report = report();
        let footprint = Footprint::of(&report, FLAGS)?;
        let rows: BTreeMap<&str, (usize, u64)> = footprint
            .folders
            .iter()
            .map(|row| (row.name.as_str(), (row.files, row.bytes)))
            .collect();
        assert_eq!(rows.get("bin"), Some(&(2, 500)));
        assert_eq!(rows.get("plugins"), Some(&(2, 57)));
        assert_eq!(rows.get("samples"), Some(&(1, 3)));
        assert_eq!(rows.get("Files directly in Acme"), Some(&(1, 1)));
        assert_eq!(rows.len(), 4);
        let inputs = footprint.build_inputs(&FootprintChoices::default(), &[], 0, true);
        assert_eq!(inputs.payload_files, 6);
        assert_eq!(inputs.payload_bytes, 561);
        assert_eq!(inputs.largest_file_bytes, 400);
        assert_eq!(inputs.prerequisites, 1);
        assert_eq!(
            footprint
                .build_inputs(&FootprintChoices::default(), &[], 0, false)
                .prerequisites,
            0
        );
        Ok(())
    }

    #[test]
    fn the_rows_say_what_the_installers_list_says() -> Result<()> {
        let report = report();
        let footprint = Footprint::of(&report, FLAGS)?;
        let manifest = manifest_for_review(&report, FLAGS, &|_| true);
        let installer_says = system_changes(&manifest)?.changes;
        let shown: Vec<&SystemChange> = footprint
            .items()
            .map(|row| &row.change)
            .chain(footprint.integrations.iter().flat_map(|row| &row.changes))
            .collect();
        assert_eq!(shown.len(), installer_says.len(), "{shown:?}");
        for change in &installer_says {
            assert!(shown.contains(&change), "{change:?} is not on the screen");
        }
        assert_eq!(footprint.refused, system_changes(&manifest)?.refused);
        Ok(())
    }

    #[test]
    fn what_holds_the_main_program_goes_in_whatever_is_asked() -> Result<()> {
        let chosen = chosen(&[
            folder("bin"),
            ChoiceId::Folder(comparable_path(Path::new(INSTALL))),
        ])?;
        assert_eq!(chosen.report.file_inventory.len(), 6, "nothing went");
        assert!(chosen.left_out.is_empty(), "{:?}", chosen.left_out);
        Ok(())
    }

    #[test]
    fn what_is_left_out_is_nowhere_and_said_to_be() -> Result<()> {
        let chosen = chosen(&[folder("samples")])?;
        assert!(!chosen.report.file_inventory.iter().any(|file| {
            file.original_path
                .starts_with(format!(r"{INSTALL}\samples"))
        }),);
        assert_eq!(chosen.report.file_inventory.len(), 5);
        assert!(
            chosen
                .left_out
                .iter()
                .any(|item| item.name.ends_with("samples"))
        );
        Ok(())
    }

    #[test]
    fn an_empty_folder_in_what_is_left_out_stays_out_and_the_rest_go() -> Result<()> {
        let mut report = report();
        let empty = |relative: &str| FileEntry {
            size_bytes: None,
            ..file(relative, 0)
        };
        report.empty_folders = vec![empty(r"plugins\inputs"), empty(r"bin\inputs")];
        let footprint = Footprint::of(&report, FLAGS)?;
        let row = footprint
            .folders
            .iter()
            .find(|row| row.name == "plugins")
            .ok_or_else(|| crate::errors::WinAppMigratorError::InvalidInput("no row".to_owned()))?;
        let mut choices = FootprintChoices::default();
        choices.set_included(&row.id, false);

        let chosen = apply_choices(&report, &footprint, &choices)?;

        let kept: Vec<String> = chosen
            .report
            .empty_folders
            .iter()
            .map(|folder| folder.original_path.display().to_string())
            .collect();
        assert_eq!(kept, [format!(r"{INSTALL}\bin\inputs")]);
        Ok(())
    }

    #[test]
    fn an_empty_folder_of_data_makes_a_package_hold_data() -> Result<()> {
        let mut report = report();
        let flags = ScanFlags {
            include_user_data: true,
            ..FLAGS
        };
        report.empty_folders = vec![FileEntry {
            classification: FileClassification::UserData,
            size_bytes: None,
            ..file("saves", 0)
        }];
        let footprint = Footprint::of(&report, flags)?;
        let chosen = apply_choices(&report, &footprint, &FootprintChoices::default())?;
        assert!(chosen.has_user_data);
        Ok(())
    }

    #[test]
    fn what_an_account_keeps_makes_a_package_hold_that_accounts_data() -> Result<()> {
        let mut report = report();
        let flags = ScanFlags {
            include_user_data: true,
            ..FLAGS
        };
        let footprint = Footprint::of(&report, flags)?;
        let without = apply_choices(&report, &footprint, &FootprintChoices::default())?;
        assert!(!without.has_user_data);
        report.registry_entries.push(RegistryEntry {
            hive: RegistryHive::CurrentUser,
            key_path: r"Software\Acme\Recent".to_owned(),
            value_name: Some("Last".to_owned()),
            value_data: Some(r"D:\Taxes".to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::Medium,
            requires_admin: false,
        });
        let footprint = Footprint::of(&report, flags)?;
        let with = apply_choices(&report, &footprint, &FootprintChoices::default())?;
        assert!(with.has_user_data);
        Ok(())
    }

    #[test]
    fn what_an_account_keeps_is_said_to_be_left_out_with_how_much() -> Result<()> {
        let mut report = report();
        report.settings_left_out = vec![LeftOutItem {
            name: r"HKCU\Software\Acme".to_owned(),
            why: "2 values, which only go with the User data option".to_owned(),
        }];
        let footprint = Footprint::of(&report, FLAGS)?;
        let group = footprint
            .left_out
            .iter()
            .find(|group| group.title == SETTINGS_LEFT_OUT_TEXT)
            .ok_or_else(|| {
                crate::errors::WinAppMigratorError::InvalidInput("no group".to_owned())
            })?;
        assert_eq!(group.items.len(), 1);
        assert_eq!(group.items[0].name, r"HKCU\Software\Acme");
        Ok(())
    }

    #[test]
    fn what_runs_a_file_in_a_folder_left_out_goes_with_it_and_is_said_to() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        let report = &chosen.report;
        let service_names: Vec<&str> = report
            .services
            .iter()
            .map(|service| service.service_name.as_str())
            .collect();
        assert_eq!(service_names, ["AcmeMain", "AcmeConfigured"]);
        let task_names: Vec<&str> = report
            .scheduled_tasks
            .iter()
            .map(|task| task.task_name.as_str())
            .collect();
        assert_eq!(task_names, ["Daily"]);
        assert_eq!(report.shortcuts.len(), 1);
        assert_eq!(report.firewall_rules.len(), 1);
        assert_eq!(report.firewall_rules[0].name, "Acme Main");
        let run_values: Vec<&str> = report
            .registry_entries
            .iter()
            .filter(|entry| is_autostart_entry(entry))
            .filter_map(|entry| entry.value_name.as_deref())
            .collect();
        assert_eq!(run_values, ["Acme Main"]);
        let also: Vec<&str> = chosen.also.iter().map(|item| item.title.as_str()).collect();
        for expected in [
            "AcmePlugins Display",
            r"\Acme\Plugins",
            "Acme Tray",
            "ACME_PLUGINS",
        ] {
            assert!(also.contains(&expected), "{expected} in {also:?}");
        }
        assert!(
            chosen.also.iter().all(|item| !item.because.is_empty()),
            "each says why: {:?}",
            chosen.also
        );
        assert!(
            chosen
                .left_out
                .iter()
                .any(|item| item.name == "AcmePlugins Display"),
            "and whoever installs it is told: {:?}",
            chosen.left_out
        );
        Ok(())
    }

    #[test]
    fn nothing_is_set_up_pointing_at_what_was_left_out() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        let footprint = Footprint::of(&chosen.report, FLAGS)?;
        let manifest = manifest_for_review(&chosen.report, FLAGS, &|_| true);
        let review = ReviewedPackage::before_a_location_is_chosen(&manifest, false)?;
        assert!(
            review.refused.is_empty(),
            "the installer has nothing to refuse of it: {:?}",
            review.refused
        );
        let needle = comparable_path(&PathBuf::from(INSTALL).join("plugins"));
        for row in footprint.items() {
            let said = format!("{} {}", row.change.title, row.change.detail).to_ascii_lowercase();
            assert!(!said.contains(&needle), "{said}");
        }
        for variable in &manifest.environment_variables_detected {
            assert!(
                !variable.value.to_ascii_lowercase().contains(&needle),
                "{variable:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn what_is_told_to_use_something_in_a_folder_left_out_goes_too() -> Result<()> {
        let chosen = chosen(&[folder("samples")])?;
        assert!(
            !chosen
                .report
                .services
                .iter()
                .any(|service| service.service_name == "AcmeConfigured"),
        );
        assert!(
            chosen
                .also
                .iter()
                .any(|item| item.title == "AcmeConfigured Display"),
            "{:?}",
            chosen.also
        );
        Ok(())
    }

    #[test]
    fn a_folder_is_taken_out_of_a_list_of_folders_and_the_rest_stays() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        let path = chosen
            .report
            .environment_variables
            .iter()
            .find(|variable| variable.name == "Path");
        assert_eq!(
            path.map(|variable| variable.value.as_str()),
            Some(format!(r"C:\Windows;{INSTALL}\bin").as_str())
        );
        Ok(())
    }

    #[test]
    fn a_plain_setting_that_names_a_folder_left_out_stays() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        assert!(
            chosen
                .report
                .registry_entries
                .iter()
                .any(|entry| entry.value_name.as_deref() == Some("PluginFolder"))
        );
        Ok(())
    }

    #[test]
    fn what_a_program_needs_goes_with_the_program() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        let needs: Vec<ProgramRole> = chosen
            .report
            .program_needs
            .iter()
            .map(|need| need.role)
            .collect();
        assert_eq!(
            needs,
            [ProgramRole::Main],
            "an Arm64 helper that is not going says nothing"
        );
        Ok(())
    }

    #[test]
    fn each_thing_on_a_list_can_be_left_out_by_itself() -> Result<()> {
        let chosen = chosen(&[
            service_choice("AcmePlugins"),
            task_choice(r"\Acme\", "Plugins"),
            shortcut_choice(Path::new(
                r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Acme Plugins.lnk",
            )),
            autostart_choice(&value(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                Some("Acme Tray"),
                "",
            )),
        ])?;
        let report = &chosen.report;
        assert_eq!(report.services.len(), 2);
        assert_eq!(report.scheduled_tasks.len(), 1);
        assert_eq!(report.shortcuts.len(), 1);
        assert_eq!(
            report
                .registry_entries
                .iter()
                .filter(|entry| is_autostart_entry(entry))
                .count(),
            1
        );
        assert_eq!(report.file_inventory.len(), 6, "and every file stays");
        assert!(chosen.also.is_empty(), "{:?}", chosen.also);
        assert_eq!(chosen.left_out.len(), 4, "{:?}", chosen.left_out);
        Ok(())
    }

    #[test]
    fn a_group_left_out_is_all_of_that_kind_and_nothing_else() -> Result<()> {
        let chosen = chosen(&[
            ChoiceId::Integration(SystemChangeKind::FileType),
            ChoiceId::Integration(SystemChangeKind::FirewallRule),
            ChoiceId::Integration(SystemChangeKind::PathEntry),
        ])?;
        let report = &chosen.report;
        assert!(report.firewall_rules.is_empty());
        assert!(!report.registry_entries.iter().any(|entry| {
            entry
                .key_path
                .eq_ignore_ascii_case(r"Software\Classes\.acme")
        }));
        assert!(
            report
                .environment_variables
                .iter()
                .all(|variable| variable.name != "Path")
        );
        assert_eq!(report.services.len(), 3);
        assert_eq!(report.file_inventory.len(), 6);
        Ok(())
    }

    #[test]
    fn a_choice_can_only_take_away() -> Result<()> {
        let chosen = chosen(&[
            ChoiceId::Folder(r"c:\windows".to_owned()),
            ChoiceId::Folder(comparable_path(
                &PathBuf::from(INSTALL).join("credentials.json"),
            )),
            ChoiceId::Service("nosuchservice".to_owned()),
        ])?;
        assert_eq!(chosen.report.file_inventory.len(), 6, "it matched nothing");
        assert_eq!(chosen.report.services.len(), 3);
        assert!(
            !chosen
                .report
                .file_inventory
                .iter()
                .any(|file| file.original_path.ends_with("credentials.json")),
            "and what looks like a secret is not there to be asked for"
        );
        assert_eq!(chosen.report.secrets_left_out.len(), 1, "it is still said");
        Ok(())
    }

    #[test]
    fn a_report_says_what_is_there_after_a_choice() -> Result<()> {
        let chosen = chosen(&[folder("plugins")])?;
        assert_eq!(chosen.report.footprint_locations[0].file_count, 4);
        assert_eq!(chosen.report.footprint_locations[0].copyable_bytes, 504);
        assert!(
            chosen.report.warnings.is_empty(),
            "nothing is said of what is not going: {:?}",
            chosen.report.warnings
        );
        Ok(())
    }

    #[test]
    fn nothing_chosen_has_nothing_to_go_with_it() -> Result<()> {
        let report = report();
        let footprint = Footprint::of(&report, FLAGS)?;
        let goes = what_goes_with(&report, &footprint, &FootprintChoices::default())?;
        assert_eq!(goes.also, []);
        assert_eq!(goes.integrations, footprint.integrations);
        Ok(())
    }

    #[test]
    fn what_ties_it_into_windows_is_said_as_it_now_stands() -> Result<()> {
        let report = report();
        let footprint = Footprint::of(&report, FLAGS)?;
        let firewall = |rows: &[IntegrationRow]| -> Vec<String> {
            rows.iter()
                .filter(|row| row.kind == SystemChangeKind::FirewallRule)
                .flat_map(|row| row.changes.iter().map(|change| change.title.clone()))
                .collect()
        };
        assert_eq!(
            firewall(&footprint.integrations),
            ["Acme Main", "Acme Plugins"]
        );
        let goes = what_goes_with(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert_eq!(firewall(&goes.integrations), ["Acme Main"]);
        assert!(
            goes.also
                .iter()
                .any(|item| item.title == "Acme Plugins" && !item.because.is_empty()),
            "{:?}",
            goes.also
        );
        Ok(())
    }

    #[test]
    fn a_row_inside_one_left_out_still_goes_in_as_it_says() -> Result<()> {
        let mut report = report();
        let (data, _) = with_a_row_inside_a_row(&mut report);
        let footprint = Footprint::of(&report, FLAGS)?;
        let choices = leaving_out(&[ChoiceId::Folder(comparable_path(Path::new(data)))]);
        let chosen = apply_choices(&report, &footprint, &choices)?;
        let names: Vec<String> = chosen
            .report
            .file_inventory
            .iter()
            .map(|file| file.original_path.display().to_string())
            .collect();
        assert!(
            names.iter().any(|name| name.ends_with("agent.exe")),
            "{names:?}"
        );
        assert!(
            !names.iter().any(|name| name.ends_with("data.dat")),
            "{names:?}"
        );
        let inputs = footprint.build_inputs(&choices, &chosen.also, 0, false);
        assert_eq!(inputs.payload_files, chosen.report.file_inventory.len());
        Ok(())
    }

    #[test]
    fn a_license_in_a_folder_left_out_still_goes_and_that_can_be_said() -> Result<()> {
        let mut report = report();
        let license = PathBuf::from(INSTALL).join(r"plugins\acme.lic");
        report
            .license_artifacts
            .push(crate::models::LicenseArtifact {
                kind: crate::models::LicenseArtifactKind::File,
                framework: crate::models::LicenseFramework::FileBased,
                path: Some(license.clone()),
                registry_location: None,
                detail: String::new(),
                machine_bound: false,
                requires_admin: false,
                transferable: crate::models::TransferableHint::Likely,
                should_copy: true,
                package_relative_path: None,
                size_bytes: Some(5),
                hash_sha256: None,
                store_files: Vec::new(),
            });
        let flags = ScanFlags {
            include_license_data: true,
            ..FLAGS
        };
        let footprint = Footprint::of(&report, flags)?;
        let choices = leaving_out(&[folder("plugins")]);
        assert_eq!(
            footprint.licenses_in_what_is_left_out(&choices),
            [license.as_path()]
        );
        assert_eq!(
            footprint.licenses_in_what_is_left_out(&FootprintChoices::default()),
            Vec::<&Path>::new()
        );
        let chosen = apply_choices(&report, &footprint, &choices)?;
        assert_eq!(
            chosen.report.license_artifacts.len(),
            1,
            "it goes by the License option"
        );
        Ok(())
    }

    /// A report with a row for a folder, and one of its own inside it.
    fn with_a_row_inside_a_row(report: &mut ScanReport) -> (&'static str, &'static str) {
        let data = r"C:\ProgramData\Acme";
        let inner = r"C:\ProgramData\Acme\Agent";
        for (location, name) in [(inner, "agent.exe"), (data, "data.dat")] {
            let original = PathBuf::from(location).join(name);
            report.file_inventory.push(FileEntry {
                path: original.clone(),
                original_path: original,
                package_relative_path: Some(PathBuf::from(r"files\_external").join(name)),
                suggested_destination_path: None,
                size_bytes: Some(10),
                hash_sha256: None,
                classification: FileClassification::AppOwned,
                should_copy: true,
                reason: String::new(),
                user_folder: None,
            });
            report.footprint_locations.push(FootprintLocation {
                path: PathBuf::from(location),
                kind: FootprintKind::Other,
                scope: InstallScope::PerMachine,
                file_count: 1,
                total_bytes: 10,
                copyable_bytes: 10,
                included: true,
            });
        }
        (data, inner)
    }

    #[test]
    fn what_names_a_row_inside_one_left_out_stays_as_that_row_does() -> Result<()> {
        let mut report = report();
        let (data, inner) = with_a_row_inside_a_row(&mut report);
        report.services[0].arguments = Some(format!(r#"--config "{inner}\a.ini""#));
        report.services[1].arguments = Some(format!(r#"--config "{data}\a.ini""#));
        let footprint = Footprint::of(&report, FLAGS)?;
        let choices = leaving_out(&[ChoiceId::Folder(comparable_path(Path::new(data)))]);
        let chosen = apply_choices(&report, &footprint, &choices)?;
        let services: Vec<&str> = chosen
            .report
            .services
            .iter()
            .map(|service| service.service_name.as_str())
            .collect();
        assert!(
            services.contains(&report.services[0].service_name.as_str()),
            "{services:?}"
        );
        assert!(
            !services.contains(&report.services[1].service_name.as_str()),
            "{services:?}"
        );
        Ok(())
    }

    #[test]
    fn a_path_in_quotes_goes_on_to_the_closing_one() -> Result<()> {
        let mut report = report();
        let (data, inner) = with_a_row_inside_a_row(&mut report);
        let spaced = r"C:\ProgramData\Acme\Agent Two";
        let original = PathBuf::from(spaced).join("two.exe");
        report.file_inventory.push(FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(PathBuf::from(r"files\_external\two.exe")),
            suggested_destination_path: None,
            size_bytes: Some(10),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        });
        report.footprint_locations.push(FootprintLocation {
            path: PathBuf::from(spaced),
            kind: FootprintKind::Other,
            scope: InstallScope::PerMachine,
            file_count: 1,
            total_bytes: 10,
            copyable_bytes: 10,
            included: true,
        });
        let footprint = Footprint::of(&report, FLAGS)?;
        let choices = leaving_out(&[ChoiceId::Folder(comparable_path(Path::new(data)))]);
        let folders = LeftOutFolders::of(&footprint, &choices);
        for (text, expected) in [
            (format!(r#"--config "{spaced}\a.ini""#), false),
            (format!(r#"--config "{inner}\a.ini""#), false),
            (format!(r#"--config "{data}\other folder\a.ini""#), true),
            (format!(r#"--config "{data} Two\a.ini""#), false),
            (format!(r"--config {inner}\a.ini --port 1"), false),
            (format!(r"--config {data}\a.ini --port 1"), true),
        ] {
            assert_eq!(folders.are_named_in_a_command(&text), expected, "{text}");
        }
        Ok(())
    }

    #[test]
    fn a_setting_that_only_names_a_folder_does_not_take_its_service() -> Result<()> {
        let mut report = report();
        report.services[0].parameters = vec![RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: format!(
                r"SYSTEM\CurrentControlSet\Services\{}\Parameters",
                report.services[0].service_name
            ),
            value_name: Some("LogDir".to_owned()),
            value_data: Some(format!(r"{INSTALL}\plugins\logs")),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        }];
        let mut config = report.services[0].parameters[0].clone();
        config.value_name = Some("Config".to_owned());
        config.value_data = Some(format!(r"{INSTALL}\plugins\settings.json"));
        report.services[0].parameters.push(config);
        assert!(names_something_it_runs(r"C:\App\run.js --port 1"));
        assert!(names_something_it_runs(r"C:\App\HOST.EXE"));
        assert!(!names_something_it_runs(r"C:\App\settings.json"));
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert!(
            chosen
                .report
                .services
                .iter()
                .any(|service| service.service_name == report.services[0].service_name),
            "{:?}",
            chosen.also
        );
        Ok(())
    }

    #[test]
    fn a_service_that_needs_one_left_out_goes_too_and_says_so() -> Result<()> {
        let mut report = report();
        let needed = report.services[1].service_name.clone();
        report.services[0].dependencies = vec![needed.clone(), "Tcpip".to_owned()];
        let footprint = Footprint::of(&report, FLAGS)?;
        let choices = leaving_out(&[service_choice(&needed)]);
        let chosen = apply_choices(&report, &footprint, &choices)?;
        assert!(
            !chosen
                .report
                .services
                .iter()
                .any(|service| service.service_name == report.services[0].service_name),
            "{:?}",
            chosen.report.services
        );
        assert!(
            chosen
                .also
                .iter()
                .any(|item| item.because.contains(&needed)),
            "{:?}",
            chosen.also
        );
        Ok(())
    }

    #[test]
    fn a_service_whose_settings_name_what_it_runs_goes_with_it() -> Result<()> {
        let mut report = report();
        report.services[0].parameters = vec![RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: format!(
                r"SYSTEM\CurrentControlSet\Services\{}\Parameters",
                report.services[0].service_name
            ),
            value_name: Some("Application".to_owned()),
            value_data: Some(format!(r"{INSTALL}\plugins\host.exe")),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        }];
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert!(
            !chosen
                .report
                .services
                .iter()
                .any(|service| service.service_name == report.services[0].service_name),
            "{:?}",
            chosen.report.services
        );
        Ok(())
    }

    #[test]
    fn what_the_main_program_loads_stays_with_it() -> Result<()> {
        let mut report = report();
        report.dependencies.push(crate::models::DependencyEntry {
            name: "needed.dll".to_owned(),
            resolved_path: Some(PathBuf::from(INSTALL).join(r"plugins\needed.dll")),
            architecture: AppArchitecture::X64,
            hash_sha256: None,
            version_info: None,
            publisher: None,
            source_category: crate::models::DependencySourceCategory::AppLocal,
            should_copy: true,
            system_provided: false,
            runtime_prerequisite: false,
            missing: false,
            delay_loaded: false,
        });
        let footprint = Footprint::of(&report, FLAGS)?;
        let row = footprint
            .folders
            .iter()
            .find(|row| row.name == "plugins")
            .ok_or_else(|| crate::errors::WinAppMigratorError::Archive("no row".to_owned()))?;
        assert!(row.locked.is_some());
        assert!(
            footprint.folder_is_in(row, &leaving_out(&[folder("plugins")])),
            "asking changes nothing"
        );
        report.dependencies[0].delay_loaded = true;
        let footprint = Footprint::of(&report, FLAGS)?;
        assert!(
            footprint
                .folders
                .iter()
                .any(|row| row.name == "plugins" && row.locked.is_none()),
            "what it only loads when it needs to is not that"
        );
        Ok(())
    }

    #[test]
    fn only_the_nearest_row_around_what_the_main_program_loads_is_held() -> Result<()> {
        let mut report = report();
        let (data, inner) = with_a_row_inside_a_row(&mut report);
        report.dependencies.push(crate::models::DependencyEntry {
            name: "needed.dll".to_owned(),
            resolved_path: Some(PathBuf::from(inner).join("needed.dll")),
            architecture: AppArchitecture::X64,
            hash_sha256: None,
            version_info: None,
            publisher: None,
            source_category: crate::models::DependencySourceCategory::AppLocal,
            should_copy: true,
            system_provided: false,
            runtime_prerequisite: false,
            missing: false,
            delay_loaded: false,
        });
        let footprint = Footprint::of(&report, FLAGS)?;
        let locked = |path: &str| {
            footprint
                .folders
                .iter()
                .find(|row| comparable_path(&row.path) == comparable_path(Path::new(path)))
                .map(|row| row.locked.is_some())
        };
        assert_eq!(locked(inner), Some(true));
        assert_eq!(locked(data), Some(false));
        Ok(())
    }

    #[test]
    fn what_was_added_and_then_left_out_is_not_said_to_be_added() -> Result<()> {
        let mut report = report();
        report.near_matches.push(NearMatch {
            id: "folder:x".to_owned(),
            kind: crate::models::NearMatchKind::Folder,
            name: "Plugins, as a near match".to_owned(),
            path: Some(PathBuf::from(INSTALL).join("plugins")),
            why: String::new(),
            follows_user_data: false,
            file_count: 1,
            total_bytes: 1,
            measured_all: true,
            picked: true,
        });
        let footprint = Footprint::of(&report, FLAGS)?;
        let kept = apply_choices(&report, &footprint, &FootprintChoices::default())?;
        assert_eq!(kept.added, ["Plugins, as a near match"]);
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert_eq!(chosen.added, Vec::<String>::new());
        // An entry brings its keys and its data as well, which still go in.
        report.near_matches[0].kind = crate::models::NearMatchKind::HiddenEntry;
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert_eq!(chosen.added, ["Plugins, as a near match"]);
        Ok(())
    }

    #[test]
    fn a_task_run_from_a_folder_with_a_space_in_its_name_is_not_taken_for_another() -> Result<()> {
        let mut report = report();
        let kept = PathBuf::from(INSTALL).join("plugins Extra");
        report
            .file_inventory
            .push(file(r"plugins Extra\run.exe", 1));
        let definition = |program: &Path, arguments: &str| {
            format!(
                "<?xml version=\"1.0\"?><Task><Principals><Principal><UserId>S-1-5-18</UserId>\
                 </Principal></Principals><Actions><Exec><Command>{}</Command>\
                 <Arguments>{arguments}</Arguments></Exec></Actions></Task>",
                program.display()
            )
        };
        report.scheduled_tasks[0].definition_xml = Some(definition(&kept.join("run.exe"), ""));
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert_eq!(chosen.report.scheduled_tasks.len(), 1, "{:?}", chosen.also);
        // And one told to use what is left out still goes.
        let plugin = PathBuf::from(INSTALL).join(r"plugins\x.dat");
        report.scheduled_tasks[0].definition_xml = Some(definition(
            &kept.join("run.exe"),
            &format!("--use {}", plugin.display()),
        ));
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert!(chosen.report.scheduled_tasks.is_empty());
        Ok(())
    }

    #[test]
    fn what_a_shortcut_or_task_is_told_to_work_in_counts() -> Result<()> {
        let mut report = report();
        let plugins = PathBuf::from(INSTALL).join("plugins");
        report.shortcuts[0].working_directory = Some(plugins.clone());
        report.shortcuts[1].icon_path = None;
        report.scheduled_tasks[0].definition_xml = Some(format!(
            "<?xml version=\"1.0\"?><Task><Principals><Principal><UserId>S-1-5-18</UserId>\
             </Principal></Principals><Actions><Exec><Command>{}\\bin\\acme.exe</Command>\
             <WorkingDirectory>{}</WorkingDirectory></Exec></Actions></Task>",
            INSTALL,
            plugins.display()
        ));
        report.services[0].arguments = Some(format!("--root {} --port 1", plugins.display()));
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert!(
            chosen.report.shortcuts.is_empty(),
            "{:?}",
            chosen.report.shortcuts
        );
        assert!(chosen.report.scheduled_tasks.is_empty());
        assert!(
            !chosen
                .report
                .services
                .iter()
                .any(|service| service.service_name == "AcmeMain"),
            "a space ends a path in a command"
        );
        Ok(())
    }

    #[test]
    fn an_icon_in_a_folder_left_out_only_loses_the_icon() -> Result<()> {
        let mut report = report();
        report.shortcuts[0].icon_path = Some(PathBuf::from(INSTALL).join(r"plugins\icon.ico"));
        let footprint = Footprint::of(&report, FLAGS)?;
        let chosen = apply_choices(&report, &footprint, &leaving_out(&[folder("plugins")]))?;
        assert_eq!(chosen.report.shortcuts.len(), 1);
        assert_eq!(chosen.report.shortcuts[0].icon_path, None);
        Ok(())
    }

    #[test]
    fn only_where_windows_keeps_fonts_counts_as_that() {
        assert!(is_the_windows_fonts_folder(r"c:\windows\fonts"));
        assert!(!is_the_windows_fonts_folder(
            r"c:\program files\acme\windows\fonts"
        ));
        let mut report = report();
        let own = PathBuf::from(INSTALL).join(r"windows\fonts\a.ttf");
        report.file_inventory.push(FileEntry {
            path: own.clone(),
            original_path: own.clone(),
            package_relative_path: Some(PathBuf::from(r"files\windows\fonts\a.ttf")),
            suggested_destination_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        });
        let footprint = Footprint::of(&report, FLAGS).unwrap_or_else(|error| panic!("{error}"));
        let chosen = apply_choices(
            &report,
            &footprint,
            &leaving_out(&[ChoiceId::Integration(SystemChangeKind::Font)]),
        )
        .unwrap_or_else(|error| panic!("{error}"));
        assert!(
            chosen
                .report
                .file_inventory
                .iter()
                .any(|file| file.original_path == own),
            "that is only a folder of the app's"
        );
    }

    #[test]
    fn what_is_left_out_of_a_scan_is_named() -> Result<()> {
        let footprint = Footprint::of(&report(), FLAGS)?;
        let titles: Vec<&str> = footprint
            .left_out
            .iter()
            .map(|group| group.title.as_str())
            .collect();
        assert_eq!(titles, [CREDENTIALS_LEFT_OUT_TEXT, LICENSES_LEFT_OUT_TEXT]);
        assert_eq!(
            footprint.left_out[0].items[0].why,
            "its name says what it holds"
        );
        Ok(())
    }
}
