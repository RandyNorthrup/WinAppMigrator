//! What exact names leave out and may still be an app's, for a person to choose from.
//!
//! A scan takes a folder or a key by the product's whole name, so as not to take another product,
//! and so a folder that is the app's under another name stays behind, and so does a part of it with
//! an uninstall entry of its own that no list of apps shows. This finds those and says why each might
//! be the app's. It never takes anything: what is ticked goes through the scan as one more root, by
//! the same rules as any other, and what looks like a credential or a license stays out as it would.
//!
//! What is offered is what a person could reasonably say yes to, so this is where the rules are
//! stricter than a scan's. Nothing shared is offered, nor anything of another app's, nor what holds
//! either. A scan asks this again for each thing ticked, so a choice can only ever name what this
//! would offer now. See [`NearMatchLook::pick`].

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::app_inventory::{UninstallRecord, read_uninstall_records};
use crate::file_discovery::{
    DiscoveryRoot, anyone_can_create_in, appdata_bases_of, collect_matching_dirs, first_link_to,
    is_a_link, is_of_one_account, is_volatile_dir, is_walkable_app_dir, kind_for_path,
    known_base_dirs, names_of_app,
};
use crate::install_source::{
    detect_install_source_unless, extract_product_code, is_chained_runtime, is_shared_folder,
    msi_files_of_others,
};
use crate::models::{
    FootprintKind, FootprintLocation, InstallScope, InstalledApp, NearMatch, NearMatchKind,
    Warning, WarningSeverity,
};
use crate::path_utils::{
    DEEPEST_WALK, comparable_path, file_name_string, has_parent_step, path_is_strictly_inside,
    path_is_within, resolved_location, walk_asking,
};
use crate::product_match::{MatchScope, NearName, ProductNames, is_generic_name};
use crate::registry_scan::{RecordedRegistryKey, is_named_for_another_app};
use crate::system_context::SystemContext;

/// What a choice calls a folder and an entry, which is all a scan is told of either.
const FOLDER_ID_PREFIX: &str = "folder:";
const ENTRY_ID_PREFIX: &str = "entry:";
/// How much is counted of one thing offered, and of everything offered. On a failing drive this is
/// reading it can do without, so it is only enough to tell a small folder from a large one.
const MOST_ENTRIES_COUNTED_OF_ONE: usize = 20_000;
const MOST_ENTRIES_COUNTED: usize = 100_000;
/// A drive could have any number of folders, and no one is going to read more than this.
const MOST_OFFERED: usize = 200;
/// Folder names that are Windows', or that hold what is anyone's, whatever else might say they are
/// an app's. Compared with the whole name, ignoring case.
const NEVER_OFFERED: &[&str] = &[
    "application data",
    "cache",
    "common files",
    "crashdumps",
    "desktop",
    "documents",
    "dotnet",
    "drivers",
    "iis",
    "iis express",
    "installer",
    "installshield installation information",
    "internet explorer",
    "local",
    "locallow",
    "logs",
    "microsoft",
    "microsoft shared",
    "microsoft.net",
    "modifiablewindowsapps",
    "msbuild",
    "oem",
    "package cache",
    "packages",
    "program files",
    "program files (arm)",
    "program files (x86)",
    "programdata",
    "public",
    "reference assemblies",
    "regid.1991-06.com.microsoft",
    "roaming",
    "softwaredistribution",
    "ssh",
    "start menu",
    "startup",
    "system32",
    "syswow64",
    "temp",
    "templates",
    "tmp",
    "uninstall information",
    "users",
    "usoprivate",
    "usoshared",
    "windows",
    "windowsapps",
    "windowspowershell",
];
/// Windows' own folders go by these, such as `Windows Defender` and `Windows NT`.
const NEVER_OFFERED_PREFIXES: &[&str] = &["windows ", "windows.old"];
/// Nothing below one of these is offered, whatever it is called. Not `Common Files`, which is
/// where a vendor can keep what only its own products use.
const NEVER_OFFERED_BELOW: &[&str] = &[
    "dotnet",
    "installshield installation information",
    "internet explorer",
    "microsoft",
    "microsoft shared",
    "microsoft.net",
    "windows",
    "windowsapps",
    "windowspowershell",
];
/// What is in `Common Files` and of no one vendor, which is all of what is in these.
const NEVER_OFFERED_IN_COMMON_FILES: &[&str] = &[
    "designer",
    "installshield",
    "merge modules",
    "odbc",
    "services",
    "speechengines",
    "system",
];
/// Why an entry is not offered for what it calls itself.
const GOES_BY_ANOTHER_APPS_NAME: &str = "it goes by another app's name";
const IS_ANOTHER_APPS: &str = "it is another app's";
const COMMON_FILES: &str = "common files";
const USERS: &str = "users";

/// What is being scanned, as far as telling what is close to it takes.
pub(crate) struct AppBeingScanned<'a> {
    pub app: &'a InstalledApp,
    pub install_location: Option<&'a Path>,
    pub names: &'a ProductNames,
    pub installed_for_one_account: bool,
}

/// Something that could be offered, and what it takes to bring it in.
#[derive(Debug, Clone)]
struct Offer {
    near: NearMatch,
    /// Where it can be read now, for a folder or an entry that names one.
    readable: Option<PathBuf>,
    /// For an entry, what its entry says, of which there can be several with one folder.
    records: Vec<UninstallRecord>,
    /// The order to offer it in, with what says most first.
    rank: u8,
}

/// Something that looked close, and why it is not offered, for saying what was not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedNearMatch {
    pub name: String,
    pub path: PathBuf,
    pub why: &'static str,
}

/// What is close to an app.
#[derive(Debug, Clone, Default)]
pub struct NearMatchLook {
    offers: Vec<Offer>,
    /// What looked close and is not to be offered, and why.
    pub refused: Vec<RefusedNearMatch>,
    others: Vec<OtherApp>,
    /// What holds other things side by side here, which every folder looked at is asked about.
    known_bases: std::cell::OnceCell<Vec<PathBuf>>,
    /// The product codes of other apps, for an entry that says it is one of those.
    products_of_others: Vec<String>,
    /// What Windows Installer says other apps have, as [`comparable_path`] writes it, in order. An
    /// entry need not say where its app is, so this is what says that of one that does not.
    files_of_others: Vec<String>,
}

/// An app other than the one being scanned, and what it goes by.
#[derive(Debug, Clone)]
struct OtherApp {
    folders: Vec<PathBuf>,
    names: ProductNames,
}

/// What a person ticked, as what a scan goes on to read.
#[derive(Debug, Default)]
pub(crate) struct Picks {
    /// Folders to walk, as they can be read now, by the same rules as any root.
    pub roots: Vec<PathBuf>,
    /// Files an install record names with no folder of the app's own to walk.
    pub loose_files: Vec<PathBuf>,
    /// The folders that are now the app's, as its PC calls them, for what goes by where it is.
    pub also_the_apps: Vec<PathBuf>,
    pub recorded_keys: Vec<RecordedRegistryKey>,
    pub also_named: Vec<ProductNames>,
    /// What other apps go by, which is not to be taken for any of that.
    pub not_named: Vec<ProductNames>,
    /// What each is called, for saying what was added.
    pub ids: BTreeSet<String>,
    pub warnings: Vec<Warning>,
}

/// Look for what is close to an app, as a scan of it would, by what the list of apps says of it.
/// Reads only names, and a little of what is offered, to say how big it is.
pub fn near_matches_of(app: &InstalledApp, system: &SystemContext) -> NearMatchLook {
    let names = names_of_app(
        app.main_executable.as_deref(),
        app.install_location.as_deref(),
        Some(&app.display_name),
        app.publisher.as_deref(),
    );
    look_for_near_matches(
        &AppBeingScanned {
            app,
            install_location: app.install_location.as_deref(),
            names: &names,
            installed_for_one_account: app.install_scope == InstallScope::PerUser,
        },
        system,
        &|| false,
    )
}

impl NearMatchLook {
    /// What is offered, in the order to show it, for saying what is. Without what a scan found
    /// anyway, and with what is ticked said to be.
    pub fn offered(&self) -> impl Iterator<Item = &NearMatch> {
        self.offers.iter().map(|offer| &offer.near)
    }

    /// What is offered once a scan knows what it found, with what was ticked said to be. What a scan
    /// has anyway needs no asking.
    pub(crate) fn rows(
        &self,
        locations: &[FootprintLocation],
        picks: &Picks,
        system: &SystemContext,
    ) -> Vec<NearMatch> {
        self.offers
            .iter()
            .filter_map(|offer| {
                let picked = picks.ids.contains(&offer.near.id);
                let already_found = offer.readable.as_deref().is_some_and(|readable| {
                    locations
                        .iter()
                        .any(|location| path_is_within(readable, &location.path))
                });
                (picked || !already_found).then(|| {
                    let mut near = offer.near.clone();
                    near.picked = picked;
                    near.path = offer
                        .readable
                        .as_deref()
                        .map(|readable| system.logical_path(readable));
                    near
                })
            })
            .collect()
    }

    /// What it takes to bring in what was ticked. It has to be something this would offer now, whatever
    /// is asked for: `ids` come from a person's choice in a window, which may be from an earlier
    /// look, and they are compared with what was found, so they cannot name a folder of their own.
    pub(crate) fn pick(
        &self,
        ids: &[String],
        scan: &AppBeingScanned<'_>,
        system: &SystemContext,
        is_cancelled: &dyn Fn() -> bool,
    ) -> Picks {
        let mut picks = Picks::default();
        if !ids.is_empty() {
            picks.not_named = self
                .others
                .iter()
                .map(|other| other.names.clone())
                .collect();
        }
        for id in ids {
            let Some(offer) = self.offers.iter().find(|offer| offer.near.id == *id) else {
                picks.warnings.push(Warning::new(
                    "near_match_no_longer_offered",
                    WarningSeverity::Warning,
                    "Something ticked in the list of what might also be this app's is not on that \
                     list any more, so it was not added. Look at the list again.",
                    None,
                ));
                continue;
            };
            picks.ids.insert(offer.near.id.clone());
            if let Some(folder) = offer.readable.as_deref() {
                picks.roots.push(folder.to_path_buf());
                picks.also_the_apps.push(system.logical_path(folder));
            }
            for record in &offer.records {
                self.pick_entry(record, scan, system, is_cancelled, &mut picks);
            }
        }
        picks
    }

    /// Bring in what an entry's own install record and names say, each folder of which is asked
    /// about as anything offered would be.
    fn pick_entry(
        &self,
        record: &UninstallRecord,
        scan: &AppBeingScanned<'_>,
        system: &SystemContext,
        is_cancelled: &dyn Fn() -> bool,
        picks: &mut Picks,
    ) {
        let source = detect_install_source_unless(&record.as_installed_app(), system, is_cancelled);
        let names_another_apps = self.record_names_another_apps(&source.roots, &source.loose_files);
        let mut roots = source.roots;
        let names = names_of_app(
            None,
            record.install_location.as_deref(),
            Some(&record.display_name),
            record.publisher.as_deref(),
        );
        // Beside the app says nothing of what it calls itself. Having the same publisher does.
        let has_the_apps_publisher = record
            .publisher
            .as_deref()
            .is_some_and(|publisher| scan.names.has_publisher_like(publisher));
        if has_the_apps_publisher {
            roots.extend(data_folders_named_for(
                &names,
                scan,
                system,
                &mut picks.warnings,
            ));
            picks.also_named.push(names);
        }
        for root in roots {
            let refusal = self.folder_refusal(&root, scan, system);
            match refusal {
                None => {
                    picks.also_the_apps.push(system.logical_path(&root));
                    picks.roots.push(root);
                }
                Some(why) => picks.warnings.push(Warning::new(
                    "near_match_part_left_out",
                    WarningSeverity::Info,
                    format!(
                        "Part of what {} says is its own was left out: {why}.",
                        record.display_name
                    ),
                    Some(root.display().to_string()),
                )),
            }
        }
        // One by one, each with no folder of its own to go by.
        picks.loose_files.extend(
            source
                .loose_files
                .into_iter()
                .filter(|file| self.may_add_loose_file(file, scan, system)),
        );
        if names_another_apps && !source.registry_keys.is_empty() {
            picks.warnings.push(Warning::new(
                "near_match_part_left_out",
                WarningSeverity::Warning,
                format!(
                    "The registry keys {}'s install record lists were left out, since that record \
                     also names what is another app's.",
                    record.display_name
                ),
                None,
            ));
        } else if !scan.installed_for_one_account {
            let not_named = std::mem::take(&mut picks.not_named);
            picks.recorded_keys.extend(
                source
                    .registry_keys
                    .into_iter()
                    .filter(|key| !is_named_for_another_app(&key.key_path, &not_named)),
            );
            picks.not_named = not_named;
        }
    }

    /// Why a hidden entry is not to be offered or added, if so. What it calls itself is only what it
    /// says, and picking it brings in what goes by that name, which would then be another app's.
    fn part_refusal(
        &self,
        record: &UninstallRecord,
        folder: &Path,
        scan: &AppBeingScanned<'_>,
        system: &SystemContext,
    ) -> Option<&'static str> {
        if self.is_named_like_another_app(&record.display_name) {
            return Some(GOES_BY_ANOTHER_APPS_NAME);
        }
        // Which also says what its install record is, and so which keys and files come with it.
        let says_it_is_another_product = extract_product_code(&record.as_installed_app())
            .is_some_and(|code| {
                self.products_of_others
                    .iter()
                    .any(|other| other.eq_ignore_ascii_case(&code))
            });
        if says_it_is_another_product {
            return Some("its install record is another app's");
        }
        // Where its uninstaller or icon is, which is where its install record would be read from.
        if self.is_read_from_another_app(record) {
            return Some("its install record is another app's");
        }
        let names = names_of_app(
            None,
            record.install_location.as_deref(),
            Some(&record.display_name),
            record.publisher.as_deref(),
        );
        if names.is_named_only_for_publisher(&record.display_name)
            || scan.names.is_named_only_for_publisher(&record.display_name)
        {
            return Some("it is named only for the publisher, and may hold its other products");
        }
        self.folder_refusal(folder, scan, system)
    }

    /// Whether an entry's uninstaller or icon is in a folder another entry says is its own. This asks
    /// that of itself, since that folder may be refused anyway, for what it is called, and the record
    /// in it read all the same.
    fn is_read_from_another_app(&self, record: &UninstallRecord) -> bool {
        record.program_folders.iter().any(|folder| {
            self.others.iter().any(|other| {
                other
                    .folders
                    .iter()
                    .any(|others_folder| path_is_within(folder, others_folder))
            })
        })
    }

    /// Whether an install record names anything of another app's, as one chaining that app or read
    /// from where it is does. Then it says nothing about which keys are this one's.
    fn record_names_another_apps(&self, roots: &[PathBuf], loose_files: &[PathBuf]) -> bool {
        roots.iter().any(|root| {
            self.is_another_apps(root, &file_name_string(root))
                || self.holds_a_file_of_another_app(root)
        }) || loose_files.iter().any(|file| {
            self.is_another_apps(file, &file_name_string(file))
                || self
                    .files_of_others
                    .binary_search(&comparable_path(file))
                    .is_ok()
        })
    }

    fn is_named_like_another_app(&self, name: &str) -> bool {
        self.others.iter().any(|other| {
            other
                .names
                .matches_top_level(name, MatchScope::ProgramFolders)
        })
    }

    /// Whether Windows Installer says another app has a file in this folder, however deep.
    fn holds_a_file_of_another_app(&self, folder: &Path) -> bool {
        let folder = comparable_path(folder);
        let below = format!(r"{folder}\");
        let first = self
            .files_of_others
            .partition_point(|file| file.as_str() < below.as_str());
        // A component can be keyed on a folder, which is then what it says.
        self.files_of_others.binary_search(&folder).is_ok()
            || self
                .files_of_others
                .get(first)
                .is_some_and(|file| file.starts_with(&below))
    }

    /// Whether a file an entry's install record names may come along with it. It has no folder of its
    /// own, which is why it is named alone, so it goes by what is around it and whose that is.
    fn may_add_loose_file(
        &self,
        file: &Path,
        scan: &AppBeingScanned<'_>,
        system: &SystemContext,
    ) -> bool {
        system.may_read(file)
            && first_link_to(file).is_none()
            && is_written_as_it_is_called(file)
            && !system.is_in_another_profile(file)
            && (!scan.installed_for_one_account || system.is_the_accounts_own(file))
            && !path_is_within(file, &system.windows_dir())
            && !is_below_what_is_shared(&system.logical_path(file))
            && !self.is_another_apps(file, &file_name_string(file))
            && self
                .files_of_others
                .binary_search(&comparable_path(file))
                .is_err()
    }

    /// Why a folder is not to be offered or added, if so.
    fn folder_refusal(
        &self,
        path: &Path,
        scan: &AppBeingScanned<'_>,
        system: &SystemContext,
    ) -> Option<&'static str> {
        if has_parent_step(path) || !system.may_read(path) {
            return Some("it may not be read");
        }
        // What it is goes by what its PC calls it. A drive can be mounted on a folder of any name.
        let logical = system.logical_path(path);
        let kind = kind_for_path(&logical);
        if (anyone_can_create_in(&kind) && is_a_link(path)) || first_link_to(path).is_some() {
            return Some("it goes through a link");
        }
        if !is_written_as_it_is_called(path) {
            return Some("it is not written the way it is really called");
        }
        if system.is_in_another_profile(path)
            || (is_of_one_account(&kind) && !system.is_the_accounts_own(path))
        {
            return Some("it is in another account's folders");
        }
        if scan.installed_for_one_account && !system.is_the_accounts_own(path) {
            return Some("this app was installed for one account, and this is not that account's");
        }
        let windows_dir = system.windows_dir();
        let known_bases = self.known_bases.get_or_init(|| known_base_dirs(system));
        if !is_walkable_app_dir(path, known_bases, &windows_dir)
            || is_shared_folder(&logical)
            || is_shared_by_name(&logical)
        {
            return Some("it holds what is shared, or is part of Windows");
        }
        let name = file_name_string(path);
        if is_generic_name(&name) || is_volatile_dir(path, true) {
            return Some("its name says nothing of whose it is");
        }
        if scan.names.is_named_only_for_publisher(&name) {
            return Some("it is named only for the publisher, and may hold its other products");
        }
        if scan
            .install_location
            .is_some_and(|location| path_is_within(location, path))
        {
            return Some("it holds this app's own folder, and may hold more");
        }
        if self.is_another_apps(path, &name) || self.holds_a_file_of_another_app(path) {
            return Some(IS_ANOTHER_APPS);
        }
        None
    }

    /// Why a folder something on record names, other than where the app itself is, is not to be
    /// walked as the app's, if so. What holds what is shared, or Windows', is nobody's whole, and
    /// another app's folder is that app's, whatever a record of this one's says. Being somewhere in
    /// another app's folder says less, as a vendor's products can be all in one, with one of them
    /// on record for all of it. Microsoft's own apps are in `Program Files` under the names that say
    /// a folder is everyone's, because it is Microsoft that means it.
    pub(crate) fn root_refusal(
        &self,
        path: &Path,
        is_microsofts_own: bool,
        system: &SystemContext,
    ) -> Option<&'static str> {
        if has_parent_step(path) || !system.may_read(path) {
            return Some("it may not be read");
        }
        if !is_written_as_it_is_called(path) {
            return Some("it is not written the way it is really called");
        }
        let logical = system.logical_path(path);
        let known_bases = self.known_bases.get_or_init(|| known_base_dirs(system));
        let is_where_microsoft_keeps_its_own = is_microsofts_own
            && system
                .program_files_dirs()
                .iter()
                .any(|base| path_is_within(path, base));
        if !is_walkable_app_dir(path, known_bases, &system.windows_dir())
            || is_shared_folder(&logical)
            || (!is_where_microsoft_keeps_its_own && is_shared_by_name(&logical))
        {
            return Some("it holds what is shared, or is part of Windows");
        }
        let holds_another_app = self.others.iter().any(|other| {
            other
                .folders
                .iter()
                .any(|folder| path_is_within(folder, path))
        });
        holds_another_app.then_some(IS_ANOTHER_APPS)
    }

    /// Whether another app has on record that it is somewhere inside this folder.
    pub(crate) fn holds_another_apps_folder(&self, path: &Path) -> bool {
        self.others.iter().any(|other| {
            other
                .folders
                .iter()
                .any(|folder| path_is_strictly_inside(folder, path))
        })
    }

    /// Whether a folder is somewhere inside what another app has on record as its own.
    pub(crate) fn is_in_another_apps_folder(&self, path: &Path) -> bool {
        self.others.iter().any(|other| {
            other
                .folders
                .iter()
                .any(|folder| path_is_within(path, folder))
        })
    }

    /// Whether another app is there, or anywhere in it, or the folder is what another app would
    /// take by its own name.
    fn is_another_apps(&self, path: &Path, name: &str) -> bool {
        let parent = path.parent().map(file_name_string);
        self.others.iter().any(|other| {
            other
                .folders
                .iter()
                .any(|folder| path_is_within(path, folder) || path_is_within(folder, path))
                || other
                    .names
                    .matches_top_level(name, MatchScope::ProgramFolders)
                || parent.as_deref().is_some_and(|parent| {
                    other
                        .names
                        .matches_nested(parent, name, MatchScope::ProgramFolders)
                })
        })
    }
}

/// Whether a folder's name, or one above it, says it is Windows' or everyone's.
fn is_shared_by_name(path: &Path) -> bool {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    NEVER_OFFERED.contains(&name.as_str())
        || NEVER_OFFERED_PREFIXES
            .iter()
            .any(|prefix| name.starts_with(prefix))
        || is_below_what_is_shared(path)
}

/// Whether anything above this says it is Windows' or everyone's.
fn is_below_what_is_shared(path: &Path) -> bool {
    let parts: Vec<String> = path
        .iter()
        .map(|part| part.to_string_lossy().to_ascii_lowercase())
        .collect();
    let is_below_a_name = parts
        .iter()
        .enumerate()
        .take(parts.len().saturating_sub(1))
        .any(|(index, above)| {
            // What an account is called is up to whoever named it.
            let is_an_account = index
                .checked_sub(1)
                .is_some_and(|before| parts[before] == USERS);
            !is_an_account
                && (NEVER_OFFERED_BELOW.contains(&above.as_str())
                    || NEVER_OFFERED_PREFIXES
                        .iter()
                        .any(|prefix| above.starts_with(prefix)))
        });
    is_below_a_name
        || parts.windows(2).any(|pair| {
            pair[0] == COMMON_FILES && NEVER_OFFERED_IN_COMMON_FILES.contains(&pair[1].as_str())
        })
}

/// The data folders named for one thing, as a scan finds its own.
fn data_folders_named_for(
    names: &ProductNames,
    scan: &AppBeingScanned<'_>,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> Vec<PathBuf> {
    let mut folders = Vec::new();
    for (base, kind) in data_bases(scan, system) {
        let mut roots: Vec<DiscoveryRoot> = Vec::new();
        collect_matching_dirs(
            &base,
            kind,
            names,
            MatchScope::ProgramFolders,
            system.volume_map(),
            &mut roots,
            warnings,
        );
        folders.extend(
            roots
                .into_iter()
                .map(|root| root.path)
                .filter(|folder| is_a_parts_own_folder(names, &base, folder)),
        );
    }
    folders
}

/// Whether a folder in `base` can be taken to be a part's by its name, however it matched.
fn is_a_parts_own_folder(names: &ProductNames, base: &Path, folder: &Path) -> bool {
    let Ok(below) = folder.strip_prefix(base) else {
        return false;
    };
    let below: Vec<String> = below
        .iter()
        .map(|part| part.to_string_lossy().into_owned())
        .collect();
    match below.as_slice() {
        [name] => names.is_a_parts_own_name(None, name),
        [parent, name] => names.is_a_parts_own_name(Some(parent), name),
        _ => false,
    }
}

/// Where an app's data can be under a name that is close: a folder a person names is theirs, so
/// `Documents` and `Saved Games` are left to exact names.
fn data_bases(scan: &AppBeingScanned<'_>, system: &SystemContext) -> Vec<(PathBuf, FootprintKind)> {
    let mut bases = Vec::new();
    if !scan.installed_for_one_account
        && let Some(program_data) = system.program_data()
    {
        bases.push((program_data, FootprintKind::ProgramData));
    }
    for user in system.user_hives() {
        bases.extend(appdata_bases_of(&user.profile_dir));
    }
    bases
}

fn near_name_text(near: &NearName) -> (String, u8) {
    match near {
        NearName::StartsWithProduct => ("Its name starts with this app's name.".to_owned(), 0),
        NearName::HasProductInIt => ("Its name has this app's name in it.".to_owned(), 1),
        NearName::StartsWithPublisher(name) => (
            format!("Its name starts with the publisher's name, {name}."),
            3,
        ),
    }
}

/// Look for what is close to an app, stopping where it is once `is_cancelled` says so.
pub(crate) fn look_for_near_matches(
    scan: &AppBeingScanned<'_>,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> NearMatchLook {
    let records = read_uninstall_records(system);
    let is_itself = |record: &UninstallRecord| {
        record
            .registry_source_path
            .eq_ignore_ascii_case(&scan.app.registry_source_path)
    };
    let parts: Vec<(&UninstallRecord, String)> = records
        .iter()
        .filter(|record| !is_itself(record))
        .filter_map(|record| why_a_part(record, scan).map(|why| (record, why)))
        .collect();
    let part_keys: Vec<String> = parts
        .iter()
        .map(|(part, _)| part.registry_source_path.clone())
        .collect();
    let is_a_part = |record: &UninstallRecord| part_keys.contains(&record.registry_source_path);
    let products_of_others: Vec<String> = records
        .iter()
        .filter(|record| !is_itself(record) && !is_a_part(record))
        .filter_map(|record| extract_product_code(&record.as_installed_app()))
        .collect();
    let mut look = NearMatchLook {
        others: records
            .iter()
            .filter(|record| !is_itself(record) && !is_a_part(record))
            .map(|record| OtherApp {
                // As it says, and as it really is, which a short name would not match.
                folders: record
                    .install_location
                    .iter()
                    .chain(&record.program_folders)
                    .flat_map(|folder| [folder.clone(), resolved_location(folder)])
                    .collect(),
                names: names_of_app(
                    None,
                    record.install_location.as_deref(),
                    Some(&record.display_name),
                    record.publisher.as_deref(),
                ),
            })
            .collect(),
        products_of_others,
        ..NearMatchLook::default()
    };
    let mut counted = 0_usize;
    for (record, why) in parts {
        let Some(folder) = own_folder_of(record) else {
            continue;
        };
        match look.part_refusal(record, &folder, scan, system) {
            Some(refusal) => look.refused.push(RefusedNearMatch {
                name: record.display_name.clone(),
                path: system.logical_path(&folder),
                why: refusal,
            }),
            None => look.add_part(record, folder, why, system, &mut counted, is_cancelled),
        }
    }
    let mut folders = Vec::new();
    for base in near_named_bases(scan, system) {
        near_named_folders(&base, scan, system, is_cancelled, &mut folders);
    }
    for (folder, near) in folders {
        if is_cancelled() {
            break;
        }
        let name = file_name_string(&folder);
        let already_offered = look.offers.iter().any(|offer| {
            offer
                .readable
                .as_deref()
                .is_some_and(|readable| path_is_within(readable, &folder))
        });
        // An entry refused for what it calls itself is no less refused for what its folder is called.
        if already_offered || look.was_refused(&folder, system) || look.offers.len() >= MOST_OFFERED
        {
            continue;
        }
        if let Some(refusal) = look.folder_refusal(&folder, scan, system) {
            look.refused.push(RefusedNearMatch {
                name,
                path: system.logical_path(&folder),
                why: refusal,
            });
            continue;
        }
        let (why, rank) = near_name_text(&near);
        let (file_count, total_bytes, measured_all) =
            measure(&folder, system, is_cancelled, &mut counted);
        look.offers.push(Offer {
            near: NearMatch {
                id: format!(
                    "{FOLDER_ID_PREFIX}{}",
                    comparable_path(&system.logical_path(&folder))
                ),
                kind: NearMatchKind::Folder,
                name,
                path: Some(system.logical_path(&folder)),
                why,
                follows_user_data: is_a_data_folder(&folder, system),
                file_count,
                total_bytes,
                measured_all,
                picked: false,
            },
            readable: Some(folder),
            records: Vec::new(),
            rank,
        });
    }
    if !look.offers.is_empty() {
        look.files_of_others = msi_files_of_others(&look.products_of_others, system, is_cancelled);
        look.take_back_what_holds_a_file_of_another_app(system);
    }
    look.offers
        .sort_by(|left, right| (left.rank, &left.near.name).cmp(&(right.rank, &right.near.name)));
    look
}

impl NearMatchLook {
    /// Whether this was refused already. One that called itself another app may be anywhere in that
    /// app's folders, and may be around them, so it goes for anything that holds it or that it holds.
    fn was_refused(&self, folder: &Path, system: &SystemContext) -> bool {
        let logical = system.logical_path(folder);
        self.refused.iter().any(|refused| {
            comparable_path(&refused.path) == comparable_path(&logical)
                || (refused.why == GOES_BY_ANOTHER_APPS_NAME
                    && (path_is_within(&refused.path, &logical)
                        || path_is_within(&logical, &refused.path)))
        })
    }

    fn take_back_what_holds_a_file_of_another_app(&mut self, system: &SystemContext) {
        let offers = std::mem::take(&mut self.offers);
        for offer in offers {
            let is_another_apps = offer
                .readable
                .as_deref()
                .is_some_and(|folder| self.holds_a_file_of_another_app(folder));
            if !is_another_apps {
                self.offers.push(offer);
                continue;
            }
            if let Some(folder) = offer.readable.as_deref() {
                self.refused.push(RefusedNearMatch {
                    name: offer.near.name.clone(),
                    path: system.logical_path(folder),
                    why: IS_ANOTHER_APPS,
                });
            }
        }
    }

    /// Offer a hidden entry, with any that says it is in the same folder as one thing.
    fn add_part(
        &mut self,
        record: &UninstallRecord,
        folder: PathBuf,
        why: String,
        system: &SystemContext,
        counted: &mut usize,
        is_cancelled: &dyn Fn() -> bool,
    ) {
        let found = self.offers.iter_mut().find(|offer| {
            offer
                .readable
                .as_deref()
                .is_some_and(|known| comparable_path(known) == comparable_path(&folder))
        });
        if let Some(offer) = found {
            offer.near.name = format!("{}, {}", offer.near.name, record.display_name);
            offer.records.push(record.clone());
            return;
        }
        if self.offers.len() >= MOST_OFFERED {
            return;
        }
        let (file_count, total_bytes, measured_all) =
            measure(&folder, system, is_cancelled, counted);
        self.offers.push(Offer {
            near: NearMatch {
                id: format!(
                    "{ENTRY_ID_PREFIX}{}",
                    record.registry_source_path.to_ascii_lowercase()
                ),
                kind: NearMatchKind::HiddenEntry,
                name: record.display_name.clone(),
                path: Some(system.logical_path(&folder)),
                why,
                follows_user_data: is_a_data_folder(&folder, system),
                file_count,
                total_bytes,
                measured_all,
                picked: false,
            },
            readable: Some(folder),
            records: vec![record.clone()],
            rank: 2,
        });
    }
}

/// Every rule here goes by how a path is written, and a short name such as `PROGRA~3` is not how any
/// of them would know it. A drive can give a folder any short name, so this asks what it is really
/// called. A drive that has a letter as well as being mounted on a folder is refused by this too,
/// which only ever leaves something out.
fn is_written_as_it_is_called(path: &Path) -> bool {
    comparable_path(&resolved_location(path)) == comparable_path(path)
}

/// Whether what is here comes with the User data option. It is the same test a scan makes of each
/// file, going by the kind of place it is.
fn is_a_data_folder(folder: &Path, system: &SystemContext) -> bool {
    let logical = system.logical_path(folder);
    match kind_for_path(&logical) {
        FootprintKind::InstallDirectory => false,
        FootprintKind::RegistryReference | FootprintKind::Other => logical
            .to_string_lossy()
            .to_ascii_lowercase()
            .contains(r"\users\"),
        _ => true,
    }
}

/// Where an entry is, by what it says of itself.
fn own_folder_of(record: &UninstallRecord) -> Option<PathBuf> {
    record
        .install_location
        .clone()
        .or_else(|| record.program_folders.first().cloned())
}

/// Why a hidden entry might be part of this app, if so. An entry in an account's own registry takes
/// no rights to write, so it says nothing about a machine-wide app, and only counts for an app of that
/// same account's. A runtime the app uses is a prerequisite, and a part of Microsoft's says nothing
/// by its publisher, though it does by where it is.
fn why_a_part(record: &UninstallRecord, scan: &AppBeingScanned<'_>) -> Option<String> {
    if !record.hidden || is_chained_runtime(&record.display_name, &scan.app.display_name) {
        return None;
    }
    let same_account = if scan.installed_for_one_account {
        record.install_scope == InstallScope::PerUser
            && record.owner_profile == scan.app.owner_profile
    } else {
        record.install_scope == InstallScope::PerMachine
    };
    if !same_account {
        return None;
    }
    if let Some(folder) = own_folder_of(record)
        && let Some(beside) = scan
            .install_location
            .and_then(Path::parent)
            .filter(|parent| {
                folder.parent().is_some_and(|its_parent| {
                    comparable_path(its_parent) == comparable_path(parent)
                }) && !is_shared_folder(parent)
                    && !is_shared_by_name(parent)
            })
    {
        return Some(format!(
            "A part hidden from the list of apps, installed beside this app in {}.",
            file_name_string(beside)
        ));
    }
    let publisher = record.publisher.as_deref()?;
    scan.names.has_publisher_like(publisher).then(|| {
        format!("A part hidden from the list of apps, from the same publisher, {publisher}.")
    })
}

/// The folders to look through for names that are close, each of which can hold programs or data.
fn near_named_bases(scan: &AppBeingScanned<'_>, system: &SystemContext) -> Vec<PathBuf> {
    let mut bases = Vec::new();
    if !scan.installed_for_one_account {
        bases.extend(system.program_files_dirs());
    }
    bases.extend(data_bases(scan, system).into_iter().map(|(base, _)| base));
    bases
}

/// What in a folder, and one level down, has a name close to the app's. Nothing that is a link is
/// looked in, and on a drive, nothing that may not be read.
fn near_named_folders(
    base: &Path,
    scan: &AppBeingScanned<'_>,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
    found: &mut Vec<(PathBuf, NearName)>,
) {
    if !system.may_read(base) {
        return;
    }
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        if is_cancelled() {
            return;
        }
        let path = entry.path();
        if !system.may_read(&path) || is_a_link(&path) || !path.is_dir() {
            continue;
        }
        let name = file_name_string(&path);
        // What is in the app's own folder is the app's, and a scan has it.
        if scan
            .names
            .matches_top_level(&name, MatchScope::ProgramFolders)
            || scan
                .install_location
                .is_some_and(|location| path_is_within(&path, location))
        {
            continue;
        }
        // A publisher's own folder is not offered, but what is in it can be.
        let is_the_publishers = scan.names.is_named_only_for_publisher(&name);
        if !is_the_publishers && let Some(near) = scan.names.near_match(None, &name) {
            found.push((path.clone(), near));
            continue;
        }
        if is_shared_by_name(&system.logical_path(&path)) || is_volatile_dir(&path, true) {
            continue;
        }
        let Ok(children) = fs::read_dir(&path) else {
            continue;
        };
        for child in children.flatten() {
            let child_path = child.path();
            if !system.may_read(&child_path)
                || is_a_link(&child_path)
                || !child_path.is_dir()
                || scan
                    .install_location
                    .is_some_and(|location| path_is_within(&child_path, location))
            {
                continue;
            }
            if let Some(near) = scan
                .names
                .near_match(Some(&name), &file_name_string(&child_path))
            {
                found.push((child_path, near));
            }
        }
    }
}

/// How many files a folder holds and how big they are, as far as is worth counting.
fn measure(
    folder: &Path,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
    counted: &mut usize,
) -> (usize, u64, bool) {
    let (mut files, mut bytes, mut looked) = (0_usize, 0_u64, 0_usize);
    let mut all = true;
    let walk = walk_asking(folder, DEEPEST_WALK, |path, _| {
        if looked >= MOST_ENTRIES_COUNTED_OF_ONE
            || counted.saturating_add(looked) >= MOST_ENTRIES_COUNTED
            || is_cancelled()
        {
            all = false;
            return false;
        }
        looked += 1;
        system.may_read(path)
    });
    for entry in walk.flatten() {
        if entry.file_type.is_file() {
            files += 1;
            bytes = bytes.saturating_add(entry.metadata.map_or(0, |metadata| metadata.len()));
        }
    }
    *counted = counted.saturating_add(looked);
    (files, bytes, all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppArchitecture, FootprintLocation, InstallScope};

    fn app(name: &str, publisher: &str, location: &Path) -> InstalledApp {
        InstalledApp {
            display_name: name.to_owned(),
            publisher: Some(publisher.to_owned()),
            version: None,
            install_location: Some(location.to_path_buf()),
            uninstall_string: None,
            main_executable: None,
            registry_source_path: r"HKLM\Test\This App".to_owned(),
            architecture: AppArchitecture::X64,
            install_scope: InstallScope::PerMachine,
            owner_profile: None,
        }
    }

    fn other_app_record(name: &str, folder: Option<&Path>) -> UninstallRecord {
        UninstallRecord {
            display_name: name.to_owned(),
            publisher: Some("Fabrikam Inc.".to_owned()),
            version: None,
            uninstall_string: None,
            registry_source_path: format!(r"HKLM\Test\{name}"),
            architecture: AppArchitecture::X64,
            install_scope: InstallScope::PerMachine,
            owner_profile: None,
            hidden: false,
            install_location: folder.map(Path::to_path_buf),
            program_folders: Vec::new(),
        }
    }

    fn scan_of<'a>(app: &'a InstalledApp, names: &'a ProductNames) -> AppBeingScanned<'a> {
        AppBeingScanned {
            app,
            install_location: app.install_location.as_deref(),
            names,
            installed_for_one_account: false,
        }
    }

    #[test]
    fn a_folder_on_record_is_refused_for_what_it_holds_and_not_for_what_it_is_in() {
        let work = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
        // A runner's temp folder is said with a short name, and this goes by what things are called.
        let base = crate::path_utils::resolved_location(work.path());
        let vendor = base.join("Vendor");
        let dist = vendor.join(r"Client\dist");
        std::fs::create_dir_all(&dist).unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let other = other_app_record("Vendor Agent", Some(&vendor));
        let look = look_with_others(std::slice::from_ref(&other));
        assert_eq!(
            look.root_refusal(&dist, false, &system),
            None,
            "a vendor's products can be all in one folder"
        );
        assert_eq!(
            look.root_refusal(&vendor, false, &system),
            Some(IS_ANOTHER_APPS),
            "that folder is the other app's"
        );
        assert_eq!(
            look.root_refusal(&base, false, &system),
            Some(IS_ANOTHER_APPS),
            "and what holds it holds that"
        );
    }

    #[test]
    fn a_folder_on_record_is_not_shared_by_a_record_saying_so() {
        let system = SystemContext::live();
        let look = NearMatchLook::default();
        for shared in [
            r"C:\Program Files\Common Files\System\ado",
            r"C:\Program Files\Common Files\Microsoft Shared\VC",
            r"C:\Program Files\WindowsApps\Foo_1.0_x64__abc",
            r"C:\ProgramData\Microsoft\Crypto\RSA",
        ] {
            let path = Path::new(shared);
            if path.is_dir() {
                assert!(
                    look.root_refusal(path, false, &system).is_some(),
                    "{shared}"
                );
            }
            assert!(is_shared_by_name(path), "{shared}");
        }
    }

    fn look_with_others(others: &[UninstallRecord]) -> NearMatchLook {
        NearMatchLook {
            others: others
                .iter()
                .map(|record| OtherApp {
                    folders: record.install_location.iter().cloned().collect(),
                    names: names_of_app(
                        None,
                        record.install_location.as_deref(),
                        Some(&record.display_name),
                        record.publisher.as_deref(),
                    ),
                })
                .collect(),
            ..NearMatchLook::default()
        }
    }

    /// A real folder in a real place, by what Windows calls it, which a temp folder's own path may
    /// not be: a runner's has `RUNNER~1` in it.
    fn a_real_folder() -> (tempfile::TempDir, PathBuf) {
        let temp = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
        let real = resolved_location(temp.path());
        let real = real
            .to_string_lossy()
            .trim_start_matches(r"\\?\")
            .to_owned();
        (temp, PathBuf::from(real))
    }

    #[test]
    fn a_folder_by_a_short_name_is_not_taken_at_its_word() {
        use std::os::windows::process::CommandExt as _;

        let (_temp, base) = a_real_folder();
        let long = base.join("Long Folder Name");
        std::fs::create_dir_all(&long).unwrap_or_else(|error| panic!("{error}"));
        // As written, since `cmd` does not read quotes as a program is expected to.
        let listed = crate::path_utils::hidden_command("cmd")
            .raw_arg(format!(
                "/C for %I in (\"{}\") do @echo %~sI",
                long.display()
            ))
            .output()
            .unwrap_or_else(|error| panic!("{error}"));
        let short = String::from_utf8_lossy(&listed.stdout).trim().to_owned();
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook::default();
        assert_eq!(look.folder_refusal(&long, &scan, &system), None);
        // A volume can have short names turned off, and then there is nothing to ask about.
        if comparable_path(Path::new(&short)) != comparable_path(&long) {
            assert_eq!(
                look.folder_refusal(Path::new(&short), &scan, &system),
                Some("it is not written the way it is really called"),
                "{short}"
            );
        }
    }

    #[test]
    fn a_folder_windows_installer_says_has_another_apps_files_is_not_offered() {
        let (_temp, base) = a_real_folder();
        let reports = base.join("Fabrikam Reports");
        let free = base.join("Fabrikam Extras");
        for folder in [&reports, &free] {
            std::fs::create_dir_all(folder.join("Engine"))
                .unwrap_or_else(|error| panic!("{error}"));
        }
        let shared = reports.join(r"Engine\reports.exe");
        let mut look = look_with_others(&[]);
        look.files_of_others = vec![comparable_path(&shared)];
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        assert_eq!(
            look.folder_refusal(&reports, &scan, &system),
            Some(IS_ANOTHER_APPS)
        );
        assert_eq!(
            look.folder_refusal(&reports.join("Engine"), &scan, &system),
            Some(IS_ANOTHER_APPS),
            "or any part of it that has one"
        );
        assert_eq!(look.folder_refusal(&free, &scan, &system), None);
        let mut keyed_on_a_folder = look_with_others(&[]);
        keyed_on_a_folder.files_of_others = vec![comparable_path(&free)];
        assert_eq!(
            keyed_on_a_folder.folder_refusal(&free, &scan, &system),
            Some(IS_ANOTHER_APPS),
            "a component can be keyed on the folder itself"
        );
        assert!(
            !look.may_add_loose_file(&shared, &scan, &system),
            "nor a file of it, by itself"
        );
        assert!(look.may_add_loose_file(&free.join("Engine"), &scan, &system));
    }

    #[test]
    fn what_is_windows_in_common_files_is_not_offered() {
        for shared in [
            r"C:\Program Files\Common Files\System",
            r"C:\Program Files\Common Files\System\Ole DB",
            r"C:\Program Files\Common Files\system\ado\msado15.dll",
            r"C:\Program Files (x86)\Common Files\DESIGNER",
        ] {
            assert!(is_below_what_is_shared(Path::new(shared)), "{shared}");
        }
        for vendors in [
            r"C:\Program Files\Common Files\Adobe\ARM",
            r"C:\Program Files\Fabrikam\System",
        ] {
            assert!(!is_below_what_is_shared(Path::new(vendors)), "{vendors}");
        }
    }

    #[test]
    fn a_data_folder_by_a_parts_name_is_only_under_its_own_publisher() {
        let base = Path::new(r"C:\ProgramData");
        let part = names_of_app(None, None, Some("Fabrikam Helper"), Some("Fabrikam Inc."));
        for (folder, expected) in [
            (r"C:\ProgramData\Fabrikam\Fabrikam Helper", true),
            (r"C:\ProgramData\Fabrikam Helper", true),
            (r"C:\ProgramData\Contoso\Fabrikam Helper", false),
            (r"C:\ProgramData\Microsoft\Fabrikam Helper", false),
            (r"C:\ProgramData\Fabrikam", false),
            (r"C:\Elsewhere\Fabrikam Helper", false),
        ] {
            assert_eq!(
                is_a_parts_own_folder(&part, base, Path::new(folder)),
                expected,
                "{folder}"
            );
        }
    }

    #[test]
    fn an_entry_named_only_for_its_publisher_is_not_offered() {
        let (_temp, base) = a_real_folder();
        let own = base.join("Its Own Folder");
        std::fs::create_dir_all(&own).unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook::default();
        assert_eq!(
            look.part_refusal(
                &other_app_record("Fabrikam", Some(&own)),
                &own,
                &scan,
                &system
            ),
            Some("it is named only for the publisher, and may hold its other products"),
        );
    }

    #[test]
    fn what_was_refused_for_its_name_stays_refused_for_its_folder() {
        let (_temp, base) = a_real_folder();
        let impostor = base.join("Fabrikam Impostor");
        let mut look = NearMatchLook::default();
        let system = SystemContext::live();
        assert!(!look.was_refused(&impostor, &system));
        look.refused.push(RefusedNearMatch {
            name: "Fabrikam Reports".to_owned(),
            path: system.logical_path(&impostor),
            why: "it goes by another app's name",
        });
        assert!(look.was_refused(&impostor, &system));
        assert!(
            look.was_refused(&impostor.join("core"), &system),
            "or what is in it, or what holds it"
        );
        assert!(look.was_refused(&base, &system));
        let other = NearMatchLook {
            refused: vec![RefusedNearMatch {
                name: "Fabrikam".to_owned(),
                path: system.logical_path(&base),
                why: "it is named only for the publisher, and may hold its other products",
            }],
            ..NearMatchLook::default()
        };
        assert!(other.was_refused(&base, &system));
        assert!(
            !other.was_refused(&impostor, &system),
            "what is in a folder refused for what it is called is asked about for itself"
        );
    }

    #[test]
    fn an_entry_that_says_it_is_another_product_is_not_offered() {
        let (_temp, base) = a_real_folder();
        let own = base.join("Fabrikam Part");
        std::fs::create_dir_all(&own).unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook {
            products_of_others: vec!["{11111111-2222-3333-4444-555555555555}".to_owned()],
            ..NearMatchLook::default()
        };
        let claiming = |code: &str| UninstallRecord {
            uninstall_string: Some(format!("MsiExec.exe /X{code}")),
            ..other_app_record("Fabrikam Part", Some(&own))
        };
        assert_eq!(
            look.part_refusal(
                &claiming("{11111111-2222-3333-4444-555555555555}"),
                &own,
                &scan,
                &system
            ),
            Some("its install record is another app's")
        );
        assert_eq!(
            look.part_refusal(
                &claiming("{66666666-2222-3333-4444-555555555555}"),
                &own,
                &scan,
                &system
            ),
            None
        );
    }

    #[test]
    fn an_entry_whose_uninstaller_is_in_another_apps_folder_is_not_offered() {
        let (_temp, base) = a_real_folder();
        let own = base.join("Fabrikam Part");
        let git = base.join("Git Thing");
        for folder in [&own, &git] {
            std::fs::create_dir_all(folder).unwrap_or_else(|error| panic!("{error}"));
        }
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_with_others(&[other_app_record("Git Thing", Some(&git))]);
        let part = |program_folders: Vec<PathBuf>| UninstallRecord {
            program_folders,
            ..other_app_record("Fabrikam Part", Some(&own))
        };
        assert_eq!(
            look.part_refusal(&part(vec![git.clone()]), &own, &scan, &system),
            Some("its install record is another app's")
        );
        assert_eq!(
            look.part_refusal(&part(vec![own.clone()]), &own, &scan, &system),
            None
        );
        // A folder refused for anything else is no one's in particular. A runtime's installer is in
        // `Package Cache`, which says nothing of whose it is.
        let cache = base.join("Package Cache");
        std::fs::create_dir_all(&cache).unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            look.part_refusal(&part(vec![cache]), &own, &scan, &system),
            None
        );
    }

    #[test]
    fn what_is_another_apps_counts_however_else_its_folder_would_be_refused() {
        let (_temp, base) = a_real_folder();
        // Named only for the publisher, and with a name that says nothing, and another app's.
        let publishers = base.join("Fabrikam");
        let launcher = base.join(r"Games\Launcher");
        let own = base.join("Fabrikam Part");
        let shared = base.join(r"Shared\one.dll");
        let look = NearMatchLook {
            files_of_others: vec![comparable_path(&shared)],
            ..look_with_others(&[
                other_app_record("Fabrikam Reports", Some(&publishers)),
                other_app_record("Game Launcher", Some(&launcher)),
            ])
        };
        let part = |program_folders: Vec<PathBuf>| UninstallRecord {
            program_folders,
            ..other_app_record("Fabrikam Part", Some(&own))
        };
        for read_from in [&publishers, &launcher] {
            assert!(
                look.is_read_from_another_app(&part(vec![read_from.clone()])),
                "{}",
                read_from.display()
            );
            assert!(
                look.record_names_another_apps(std::slice::from_ref(read_from), &[]),
                "{}",
                read_from.display()
            );
        }
        assert!(
            look.is_read_from_another_app(&part(vec![publishers.join("Uninstall")])),
            "or anywhere in one"
        );
        assert!(!look.is_read_from_another_app(&part(vec![own.clone()])));
        assert!(!look.record_names_another_apps(std::slice::from_ref(&own), &[]));
        assert!(
            look.record_names_another_apps(&[], std::slice::from_ref(&shared)),
            "a file another app's record lists too"
        );
        assert!(
            look.record_names_another_apps(std::slice::from_ref(&base.join("Shared")), &[]),
            "or a folder with one in it"
        );
        assert!(!look.record_names_another_apps(&[], std::slice::from_ref(&own.join("one.dll"))));
    }

    #[test]
    fn only_an_entry_of_the_apps_publisher_brings_in_what_goes_by_its_name() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let app = drive.app();
        let names = names_of_app(
            None,
            app.install_location.as_deref(),
            Some(&app.display_name),
            app.publisher.as_deref(),
        );
        let scan = scan_of(&app, &names);
        let folder = drive.root.join(r"Program Files\Fabrikam\Data Sync Tools");
        let picks_of = |publisher: &str| {
            let record = UninstallRecord {
                publisher: Some(publisher.to_owned()),
                hidden: true,
                ..other_app_record("Update Service", Some(&folder))
            };
            let look = NearMatchLook {
                offers: vec![Offer {
                    near: NearMatch {
                        id: "entry:x".to_owned(),
                        kind: NearMatchKind::HiddenEntry,
                        name: "Update Service".to_owned(),
                        path: Some(folder.clone()),
                        why: String::new(),
                        follows_user_data: false,
                        file_count: 0,
                        total_bytes: 0,
                        measured_all: true,
                        picked: false,
                    },
                    readable: Some(folder.clone()),
                    records: vec![record],
                    rank: 2,
                }],
                ..NearMatchLook::default()
            };
            look.pick(&["entry:x".to_owned()], &scan, &system, &|| false)
        };
        assert_eq!(picks_of("Fabrikam Inc.").also_named.len(), 1);
        assert_eq!(
            picks_of("Clients").also_named.len(),
            0,
            "beside the app says nothing of what it calls itself"
        );
    }

    #[test]
    fn what_is_inside_what_is_never_offered_is_not_either() {
        for shared in [
            r"C:\Program Files\Windows NT\Accessories",
            r"C:\Program Files\Windows Media Player\Skins",
            r"C:\Program Files\Internet Explorer\Plugins",
            r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App\8.0.0",
            r"C:\Program Files\WindowsPowerShell\Modules",
            r"C:\Program Files\Common Files\InstallShield\Driver",
            r"C:\Program Files\Common Files\Merge Modules",
            r"C:\Windows.old\Users\Bob\Documents",
            r"C:\Windows.old.000\Users\Bob\Documents",
            r"C:\Program Files\Microsoft.NET\Primary Interop Assemblies",
            r"C:\Program Files\InstallShield Installation Information\{A}",
            r"C:\Program Files\Common Files\ODBC\Data Sources",
        ] {
            assert!(is_below_what_is_shared(Path::new(shared)), "{shared}");
        }
        for vendors in [
            r"C:\Program Files\Windows Helper",
            r"C:\Program Files\Fabrikam\Plugins",
            r"C:\Program Files\Common Files\Fabrikam\Merge Modules",
            r"C:\Users\Windows User\AppData\Local\Fabrikam",
            r"C:\Users\dotnet\AppData\Roaming\Fabrikam Extras",
        ] {
            assert!(!is_below_what_is_shared(Path::new(vendors)), "{vendors}");
        }
    }

    #[test]
    fn what_an_entry_calls_itself_cannot_take_another_apps_keys() {
        let look = look_with_others(&[other_app_record("Mozilla Firefox", None)]);
        assert!(look.is_named_like_another_app("Mozilla Firefox"));
        assert!(!look.is_named_like_another_app("Fabrikam Helper"));
        let (_temp, base) = a_real_folder();
        let own = base.join("Its Own Folder");
        std::fs::create_dir_all(&own).unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let install = base.join("Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        assert_eq!(
            look.part_refusal(
                &other_app_record("Mozilla Firefox", Some(&own)),
                &own,
                &scan,
                &system
            ),
            Some(GOES_BY_ANOTHER_APPS_NAME),
            "wherever it says it is"
        );
        assert_eq!(
            look.part_refusal(
                &other_app_record("Fabrikam Helper", Some(&own)),
                &own,
                &scan,
                &system
            ),
            None
        );
        let others: Vec<ProductNames> = look
            .others
            .iter()
            .map(|other| other.names.clone())
            .collect();
        assert!(is_named_for_another_app(
            r"Software\Mozilla\Firefox",
            &others
        ));
        assert!(is_named_for_another_app(
            r"Software\WOW6432Node\Mozilla Firefox\Settings",
            &others
        ));
        assert!(!is_named_for_another_app(
            r"Software\Fabrikam\Helper",
            &others
        ));
    }

    #[test]
    fn nothing_shared_can_be_offered_or_added() {
        let system = SystemContext::live();
        let install = PathBuf::from(r"C:\Program Files\Fabrikam\Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook::default();
        let mut refused: Vec<PathBuf> = vec![
            PathBuf::from(r"C:\"),
            system.windows_dir(),
            system.windows_dir().join("System32"),
            system.windows_dir().join(r"Fabrikam Data Sync Extras"),
        ];
        for program_files in system.program_files_dirs() {
            refused.push(program_files.clone());
            refused.push(program_files.join("Common Files"));
            refused.push(program_files.join(r"Common Files\Microsoft Shared"));
            refused.push(program_files.join("Windows Defender"));
            refused.push(program_files.join("WindowsPowerShell"));
            refused.push(program_files.join("Microsoft"));
            refused.push(program_files.join("dotnet"));
        }
        refused.extend(system.program_data());
        refused.extend(system.user_profiles());
        for path in refused {
            assert!(
                look.folder_refusal(&path, &scan, &system).is_some(),
                "{}",
                path.display()
            );
        }
    }

    #[test]
    fn a_folder_that_holds_this_app_or_is_named_only_for_its_publisher_is_not_offered() {
        let (_temp, base) = a_real_folder();
        let vendor = base.join("Fabrikam");
        let install = vendor.join("Data Sync");
        std::fs::create_dir_all(&install).unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let refusal = NearMatchLook::default().folder_refusal(&vendor, &scan, &system);
        assert!(refusal.is_some(), "{refusal:?}");
    }

    #[test]
    fn another_apps_folder_is_not_offered_by_where_it_is_what_it_holds_or_its_name() {
        let (_temp, base) = a_real_folder();
        let install = base.join(r"Fabrikam\Data Sync");
        let beside = base.join(r"Fabrikam\Data Sync Tools");
        let inside = beside.join("Parts");
        let named = base.join(r"Fabrikam\Data Sync Pro");
        let free = base.join(r"Fabrikam\Data Sync Extras");
        for folder in [&install, &inside, &named, &free] {
            std::fs::create_dir_all(folder).unwrap_or_else(|error| panic!("{error}"));
        }
        let system = SystemContext::live();
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_with_others(&[
            other_app_record("Tools Suite", Some(&inside)),
            other_app_record("Data Sync Pro", None),
        ]);
        assert_eq!(
            look.folder_refusal(&beside, &scan, &system),
            Some(IS_ANOTHER_APPS),
            "it holds where another app is"
        );
        assert_eq!(
            look.folder_refusal(&inside, &scan, &system),
            Some(IS_ANOTHER_APPS),
            "it is where another app is"
        );
        assert_eq!(
            look.folder_refusal(&named, &scan, &system),
            Some(IS_ANOTHER_APPS),
            "that app would take it by its own name"
        );
        assert_eq!(look.folder_refusal(&free, &scan, &system), None);
    }

    #[test]
    fn a_link_is_never_offered_whatever_it_is_called_and_nothing_is_read_through_one() {
        let base = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
        let install = base.path().join("Data Sync");
        let elsewhere = base.path().join("elsewhere");
        for folder in [&install, &elsewhere] {
            std::fs::create_dir_all(folder).unwrap_or_else(|error| panic!("{error}"));
        }
        let link = base.path().join("Data Sync Extras");
        let made = crate::path_utils::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&elsewhere)
            .output()
            .unwrap_or_else(|error| panic!("{error}"));
        assert!(made.status.success());
        let system = SystemContext::live();
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook::default();
        assert_eq!(
            look.folder_refusal(&link, &scan, &system),
            Some("it goes through a link")
        );
        assert_eq!(
            look.folder_refusal(&link.join("below"), &scan, &system),
            Some("it goes through a link")
        );

        let mut found = Vec::new();
        near_named_folders(base.path(), &scan, &system, &|| false, &mut found);
        assert!(found.iter().all(|(path, _)| path != &link), "{found:?}");
    }

    #[test]
    fn only_what_this_would_offer_now_can_be_picked() {
        let system = SystemContext::live();
        let install = PathBuf::from(r"C:\Program Files\Fabrikam\Data Sync");
        let app = app("Data Sync", "Fabrikam Inc.", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = NearMatchLook::default();
        let picks = look.pick(
            &[
                r"folder:c:\windows".to_owned(),
                r"folder:c:\program files".to_owned(),
                r"entry:hklm\anything".to_owned(),
                r"C:\Users".to_owned(),
            ],
            &scan,
            &system,
            &|| false,
        );
        assert!(picks.roots.is_empty(), "{:?}", picks.roots);
        assert!(picks.also_the_apps.is_empty());
        assert!(picks.ids.is_empty());
        assert_eq!(picks.warnings.len(), 4);
    }

    fn hidden_record(
        name: &str,
        publisher: &str,
        folder: &Path,
        scope: InstallScope,
    ) -> UninstallRecord {
        UninstallRecord {
            hidden: true,
            publisher: Some(publisher.to_owned()),
            install_scope: scope,
            ..other_app_record(name, Some(folder))
        }
    }

    #[test]
    fn a_hidden_entry_counts_by_its_publisher_or_by_being_beside_the_app() {
        let vendor = PathBuf::from(r"C:\Vendor\NVIDIA Corporation");
        let install = vendor.join("NVIDIA app");
        let app = app("NVIDIA App 11.0.4.526", "NVIDIA Corporation", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some(&app.display_name),
            app.publisher.as_deref(),
        );
        let scan = scan_of(&app, &names);

        let container = hidden_record(
            "NVIDIA Container",
            "NVIDIA Corporation",
            &vendor.join("NvContainer"),
            InstallScope::PerMachine,
        );
        assert!(why_a_part(&container, &scan).is_some_and(|why| why.contains("beside this app")));

        let elsewhere = hidden_record(
            "NVIDIA Telemetry",
            "NVIDIA Corporation",
            Path::new(r"C:\Other\NvTelemetry"),
            InstallScope::PerMachine,
        );
        assert!(why_a_part(&elsewhere, &scan).is_some_and(|why| why.contains("same publisher")));

        for not_a_part in [
            hidden_record(
                "Someone Else's",
                "Fabrikam Inc.",
                Path::new(r"C:\Other\Thing"),
                InstallScope::PerMachine,
            ),
            hidden_record(
                "Microsoft Visual C++ 2022 X64 Runtime",
                "NVIDIA Corporation",
                &vendor.join("Runtime"),
                InstallScope::PerMachine,
            ),
            // Any account can write this with no rights at all.
            hidden_record(
                "NVIDIA Planted",
                "NVIDIA Corporation",
                &vendor.join("Planted"),
                InstallScope::PerUser,
            ),
            UninstallRecord {
                hidden: false,
                ..hidden_record(
                    "NVIDIA PhysX",
                    "NVIDIA Corporation",
                    &vendor.join("PhysX"),
                    InstallScope::PerMachine,
                )
            },
        ] {
            assert!(
                why_a_part(&not_a_part, &scan).is_none(),
                "{}",
                not_a_part.display_name
            );
        }
    }

    #[test]
    fn being_published_by_microsoft_says_nothing_but_being_beside_the_app_does() {
        let vendor = PathBuf::from(r"C:\Vendor\Contoso Tools");
        let install = vendor.join("Thing");
        let app = app("Thing", "Microsoft Corporation", &install);
        let names = names_of_app(
            None,
            Some(&install),
            Some(&app.display_name),
            app.publisher.as_deref(),
        );
        let scan = scan_of(&app, &names);
        let far = hidden_record(
            "Some Windows Part",
            "Microsoft Corporation",
            Path::new(r"C:\Other\Thing Part"),
            InstallScope::PerMachine,
        );
        assert!(why_a_part(&far, &scan).is_none());
        let near = hidden_record(
            "Thing Part",
            "Microsoft Corporation",
            &vendor.join("Thing Part"),
            InstallScope::PerMachine,
        );
        assert!(why_a_part(&near, &scan).is_some());
    }

    #[test]
    fn a_folder_is_measured_without_going_through_a_link_or_past_a_limit() {
        let base = tempfile::tempdir().unwrap_or_else(|error| panic!("{error}"));
        std::fs::write(base.path().join("a.txt"), b"12345")
            .unwrap_or_else(|error| panic!("{error}"));
        std::fs::create_dir_all(base.path().join("sub")).unwrap_or_else(|error| panic!("{error}"));
        std::fs::write(base.path().join(r"sub\b.txt"), b"123")
            .unwrap_or_else(|error| panic!("{error}"));
        let system = SystemContext::live();
        let mut counted = 0;
        assert_eq!(
            measure(base.path(), &system, &|| false, &mut counted),
            (2, 8, true)
        );
        assert!(counted > 0);
        let mut used_up = MOST_ENTRIES_COUNTED;
        let (files, _, all) = measure(base.path(), &system, &|| false, &mut used_up);
        assert_eq!((files, all), (0, false));
    }

    /// A drive laid out as a PC's, with what might be near the app, and what must not be offered.
    /// Its registry is not loaded, so nothing here comes from an entry.
    struct NearDrive {
        _temp: tempfile::TempDir,
        root: PathBuf,
        account: PathBuf,
        install: PathBuf,
    }

    impl NearDrive {
        fn new() -> Self {
            let (temp, base) = a_real_folder();
            let root = base.join("drive");
            let account = root.join(r"Users\me");
            let install = root.join(r"Program Files\Fabrikam\Data Sync");
            for folder in [
                r"Program Files\Fabrikam\Data Sync\bin",
                r"Program Files\Fabrikam\Data Sync Tools\bin",
                r"Program Files\Fabrikam\Something Else",
                r"Program Files\Fabrikam Inc\Stuff",
                r"Program Files\Data Sync Extras",
                r"Program Files\Windows Data Sync",
                r"Program Files\Common Files\Data Sync Helpers",
                r"Program Files\Common Files\Microsoft Shared\Data Sync Parts",
                r"Program Files\Other Vendor\Data Sync Clone",
                r"ProgramData\Fabrikam-1de4eec8",
                r"ProgramData\Data Sync Cache",
                r"ProgramData\Microsoft\Data Sync Logs",
                r"Users\me\AppData\Roaming\Data Sync Profiles",
                r"Users\me\AppData\Local\ChocolateyHttpCache\Data Sync Stuff",
                r"Users\other\AppData\Roaming\Data Sync Profiles",
                r"Users\me\Documents\Data Sync Notes",
                "Windows",
                "elsewhere",
            ] {
                std::fs::create_dir_all(root.join(folder))
                    .unwrap_or_else(|error| panic!("{folder}: {error}"));
            }
            std::fs::write(install.join(r"bin\app.exe"), b"MZ")
                .unwrap_or_else(|error| panic!("{error}"));
            let extras = root.join(r"Program Files\Data Sync Extras\extra.dat");
            std::fs::write(extras, b"12345").unwrap_or_else(|error| panic!("{error}"));
            let link = root.join(r"Program Files\Data Sync Link");
            let made = crate::path_utils::hidden_command("cmd")
                .arg("/C")
                .arg("mklink")
                .arg("/J")
                .arg(&link)
                .arg(root.join("elsewhere"))
                .output()
                .unwrap_or_else(|error| panic!("{error}"));
            assert!(made.status.success());
            Self {
                _temp: temp,
                root,
                account,
                install,
            }
        }

        fn app(&self) -> InstalledApp {
            app("Data Sync", "Fabrikam Inc.", &self.install)
        }
    }

    fn offered_names(look: &NearMatchLook) -> Vec<String> {
        let mut names: Vec<String> = look.offered().map(|near| near.name.clone()).collect();
        names.sort();
        names
    }

    #[test]
    fn on_a_drive_only_what_is_near_and_nobodys_is_offered() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let app = drive.app();
        let names = names_of_app(
            None,
            Some(&drive.install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_for_near_matches(&scan, &system, &|| false);
        assert_eq!(
            offered_names(&look),
            [
                "Data Sync Clone",
                "Data Sync Extras",
                "Data Sync Profiles",
                "Data Sync Tools",
                "Fabrikam-1de4eec8",
            ],
            "and refused {:?}",
            look.refused
        );
        let by_name = |name: &str| look.offered().find(|near| near.name == name).cloned();
        let extras = by_name("Data Sync Extras").unwrap_or_else(|| panic!("offered"));
        assert_eq!(extras.kind, NearMatchKind::Folder);
        assert_eq!((extras.file_count, extras.total_bytes), (1, 5));
        assert!(!extras.follows_user_data, "it is in Program Files");
        assert!(!extras.why.is_empty());
        assert_eq!(
            extras.path.as_deref(),
            Some(Path::new(r"C:\Program Files\Data Sync Extras")),
            "as its own PC calls it"
        );
        let profile = by_name("Data Sync Profiles").unwrap_or_else(|| panic!("offered"));
        assert!(profile.follows_user_data, "it is in AppData");
        let refused_paths: Vec<String> = look
            .refused
            .iter()
            .map(|refused| refused.path.display().to_string())
            .collect();
        for not_even_looked_at in [
            r"Users\other",
            r"Documents",
            r"Microsoft Shared",
            r"Data Sync Link",
            r"Common Files",
        ] {
            assert!(
                !look.offered().any(|near| near
                    .path
                    .as_deref()
                    .is_some_and(|path| path.to_string_lossy().contains(not_even_looked_at))),
                "{not_even_looked_at} in {refused_paths:?}"
            );
        }
    }

    #[test]
    fn picking_takes_what_is_offered_and_nothing_else() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let app = drive.app();
        let names = names_of_app(
            None,
            Some(&drive.install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_for_near_matches(&scan, &system, &|| false);
        let extras = look
            .offered()
            .find(|near| near.name == "Data Sync Extras")
            .map(|near| near.id.clone())
            .unwrap_or_else(|| panic!("offered"));
        let picks = look.pick(
            &[
                extras.clone(),
                r"folder:c:\windows".to_owned(),
                format!("folder:{}", drive.root.join("Windows").display()),
            ],
            &scan,
            &system,
            &|| false,
        );
        assert_eq!(
            picks.roots,
            [drive.root.join(r"Program Files\Data Sync Extras")]
        );
        assert_eq!(
            picks.also_the_apps,
            [PathBuf::from(r"C:\Program Files\Data Sync Extras")],
            "as its PC calls it, which is what its values say"
        );
        assert_eq!(picks.ids.iter().collect::<Vec<_>>(), [&extras]);
        assert_eq!(picks.warnings.len(), 2, "{:?}", picks.warnings);
        let rows = look.rows(&[], &picks, &system);
        assert!(
            rows.iter().any(|near| near.id == extras && near.picked),
            "{rows:?}"
        );
        assert_eq!(
            rows.iter().filter(|near| near.picked).count(),
            1,
            "{rows:?}"
        );
    }

    #[test]
    fn what_a_scan_has_anyway_is_not_offered_unless_it_was_picked() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let app = drive.app();
        let names = names_of_app(
            None,
            Some(&drive.install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_for_near_matches(&scan, &system, &|| false);
        let location = FootprintLocation {
            path: drive.root.join(r"Program Files\Data Sync Extras"),
            kind: FootprintKind::InstallDirectory,
            scope: InstallScope::PerMachine,
            file_count: 1,
            total_bytes: 5,
            copyable_bytes: 5,
            included: true,
        };
        let rows = look.rows(std::slice::from_ref(&location), &Picks::default(), &system);
        assert!(
            !rows.iter().any(|near| near.name == "Data Sync Extras"),
            "{rows:?}"
        );
        assert!(
            rows.iter().any(|near| near.name == "Data Sync Tools"),
            "{rows:?}"
        );
    }

    #[test]
    fn an_app_of_one_account_is_offered_nothing_machine_wide() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let mut app = drive.app();
        app.install_scope = InstallScope::PerUser;
        let install = drive.account.join(r"AppData\Local\Programs\Data Sync");
        std::fs::create_dir_all(&install).unwrap_or_else(|error| panic!("{error}"));
        app.install_location = Some(install.clone());
        let names = names_of_app(
            None,
            Some(&install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = AppBeingScanned {
            app: &app,
            install_location: Some(&install),
            names: &names,
            installed_for_one_account: true,
        };
        let look = look_for_near_matches(&scan, &system, &|| false);
        assert_eq!(
            offered_names(&look),
            ["Data Sync Profiles"],
            "{:?}",
            look.refused
        );
    }

    #[test]
    fn a_look_stops_when_it_is_told_to() {
        let drive = NearDrive::new();
        let system = SystemContext::offline_for_tests(&drive.root, &drive.account);
        let app = drive.app();
        let names = names_of_app(
            None,
            Some(&drive.install),
            Some("Data Sync"),
            Some("Fabrikam Inc."),
        );
        let scan = scan_of(&app, &names);
        let look = look_for_near_matches(&scan, &system, &|| true);
        assert_eq!(offered_names(&look), Vec::<String>::new());
    }
}
