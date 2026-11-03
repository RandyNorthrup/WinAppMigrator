//! Applies the rules in [`crate::restore_policy`] to a whole package.
//!
//! Restoring and verifying work from the one result, so verification expects exactly what
//! was allowed. The installer's window shows the same rules before an install location has
//! been chosen, so a few checks that depend on what is already at that location can only
//! happen once one has: a task or COM class that belongs to something else is refused then.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{HKEY_USERS, KEY_READ};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::{
    DESKTOP_FALLBACK, DESKTOP_VALUE, MAX_EMPTY_FOLDERS, START_MENU_FALLBACK, START_MENU_VALUE,
    UserDataAnchor, UserFolderPlace, account_shell_folder, data_location_of, first_link_to,
    has_reserved_device_name, known_base_dirs, link_refusal, safe_relative_path,
    split_user_data_relative, user_data_base, user_folder_place,
};
use crate::footprint::ChoiceId;
use crate::journal::{install_on_record_at, machine_state_root};
use crate::license_scan::is_license_env_var;
use crate::models::{
    AppArchitecture, CertificateEntry, DriverEntry, EnvironmentScope, EnvironmentVariableEntry,
    FileClassification, FileEntry, FirewallRuleEntry, LeftOutItem, LicenseArtifact,
    LicenseStoreFile, PackageManifest, RegistryEntry, RegistryHive, ScheduledTaskEntry,
    ServiceEntry, Warning, WarningSeverity,
};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{
    SignedInUser, current_user_sid, expand_env_vars, is_network_path, path_is_within,
    resolved_location, signed_in_user, strip_prefix_ignore_case, system_root,
    windows_architecture_name, windows_build, windows_version_numbers,
};
use crate::registry_target::{app_path_entries, is_path_variable, service_image_path};
use crate::restore_policy::{
    KnownFolders, PackageRegistrations, PlatformFit, Registration, SERVICES_PREFIX, ServiceAccount,
    ShortcutPlacement, WHOEVER_INSTALLS_IT, WindowsHere, architecture_from_name,
    data_location_refusal, driver_destination, environment_refusal, existing_service_refusal,
    firewall_refusal, install_location_refusal, install_placement_refusal, is_autostart_entry,
    is_source_uninstall_value, is_uninstall_entry_key, is_valid_service_name, license_destination,
    normalized_key, platform_fit, read_task_definition, registered_value_refusal, registration_of,
    registration_refusal, registration_unit_of, registry_refusal, shortcut_placement, task_refusal,
};
use crate::space::{
    DRIVER_ALLOWANCE_BYTES, PlaceNeed, PlannedFile, RoomCheck, install_needs, on_same_volume,
};
use crate::system_context::SystemContext;

/// What a task that ran as a person is said to run as here.
const THE_PERSON_INSTALLING: &str = "you, whoever is signed in";
const DESKTOP_FOLDER: &str = "Desktop";
const START_MENU_SUBFOLDER: &str = r"Microsoft\Windows\Start Menu";
const NO_USER_ACCOUNT_REFUSAL: &str =
    "it belongs to one account's settings, and who is signed in could not be told";
/// What a person installing is told about whose folders and settings these are, or where they
/// went instead. Shown before they install.
const USER_ACCOUNT_NOTE_CODE: &str = "user_account_note";
const USER_FOLDER_ELSEWHERE_CODE: &str = "user_folder_elsewhere";
const PARAMETERS_SUBKEY: &str = "parameters";
const REFUSED_CODE: &str = "restore_refused";
const DISPLAY_NAME_VALUE: &str = "DisplayName";
pub(crate) const STATE_ROOT_REFUSAL: &str = "it is where installs keep their records";
/// A stand-in target, for describing a package before an install location is chosen.
pub(crate) const NO_TARGET_YET: &str = r"C:\WinAppMigrator install location";

/// How to look up a service already on this PC by name.
type ExistingService<'a> = &'a dyn Fn(&str) -> std::io::Result<Option<String>>;

/// A service that is allowed, with what it will run.
pub(crate) struct ReviewedService<'a> {
    pub entry: &'a ServiceEntry,
    pub binary: PathBuf,
    pub arguments: Option<String>,
    pub account: ServiceAccount,
}

/// A bundled driver that is allowed, with where its image goes.
pub(crate) struct ReviewedDriver<'a> {
    pub entry: &'a DriverEntry,
    pub package_relative_path: &'a Path,
    pub destination: PathBuf,
}

/// A scheduled task that is allowed, with its definition rewritten for the target.
pub(crate) struct ReviewedTask<'a> {
    pub entry: &'a ScheduledTaskEntry,
    pub xml: String,
    pub commands: Vec<String>,
    /// What those commands are given.
    pub arguments: Vec<String>,
    pub runs_as_system: bool,
    /// The account its definition names, which is what it will run as.
    pub account: Option<String>,
    /// Asks for the highest rights that account can have.
    pub highest_available: bool,
}

/// A shortcut that is allowed, with its paths rewritten for the target.
pub(crate) struct ReviewedShortcut {
    /// Where it was on its PC, which is what it goes by.
    pub original_lnk_path: PathBuf,
    pub placement: ShortcutPlacement,
    pub target: PathBuf,
    pub arguments: Option<String>,
    pub working_directory: Option<String>,
    pub icon: Option<String>,
}

/// License data that is allowed, with where it goes.
pub(crate) struct ReviewedLicense<'a> {
    pub artifact: &'a LicenseArtifact,
    pub package_relative_path: &'a Path,
    pub destination: PathBuf,
}

/// An app's entry in Settings' list of installed apps, which a package brought with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UninstallKey {
    pub hive: RegistryHive,
    pub key_path: String,
    pub has_display_name: bool,
}

/// A firewall rule that is allowed, with the program it is for on this PC.
pub(crate) struct ReviewedFirewallRule<'a> {
    pub entry: &'a FirewallRuleEntry,
    pub program: PathBuf,
}

/// What a package may do on this PC, and what it was refused.
pub(crate) struct ReviewedPackage<'a> {
    pub map: RestoreMap,
    pub files: Vec<&'a FileEntry>,
    /// Folders of the app's with nothing in them.
    pub empty_folders: Vec<&'a FileEntry>,
    pub registry: Vec<&'a RegistryEntry>,
    /// The entries in `registry` that make an app's entry in Settings, which an install points
    /// at its own uninstaller.
    pub uninstall_keys: Vec<UninstallKey>,
    pub license_registry: Vec<&'a RegistryEntry>,
    pub license: Vec<ReviewedLicense<'a>>,
    pub services: Vec<ReviewedService<'a>>,
    pub drivers: Vec<ReviewedDriver<'a>>,
    /// Drivers that were not bundled, for which there are only instructions.
    pub manual_drivers: Vec<&'a DriverEntry>,
    pub tasks: Vec<ReviewedTask<'a>>,
    pub shortcuts: Vec<ReviewedShortcut>,
    pub environment: Vec<&'a EnvironmentVariableEntry>,
    pub firewall_rules: Vec<ReviewedFirewallRule<'a>>,
    /// What the package registers, as a whole.
    pub registrations: PackageRegistrations,
    /// What the package asked for and will not be given.
    pub refused: Vec<Warning>,
    /// Problems with the package short of a refusal.
    pub warnings: Vec<Warning>,
    user_account_known: bool,
    manifest: &'a PackageManifest,
}

impl<'a> ReviewedPackage<'a> {
    /// Check a package against this PC. `Err` means it names an unsafe path for one of its
    /// files, which is refused whole, before anything happens.
    pub(crate) fn new(
        manifest: &'a PackageManifest,
        target_root: &Path,
        restore_user_data: bool,
        restore_license: bool,
    ) -> Result<Self> {
        Self::with_folders(
            manifest,
            target_root,
            restore_user_data,
            restore_license,
            &current_known_folders(),
            &expand_env_vars,
            &service_image_path,
        )
    }

    /// What a package would be allowed, for showing before an install location has been chosen, as
    /// the installer's window does and as the app does before anything is built. Whether an item is
    /// allowed does not depend on where the app is installed, and it does depend on these two
    /// options, which are the ones an install will be given.
    pub(crate) fn before_a_location_is_chosen(
        manifest: &'a PackageManifest,
        restore_user_data: bool,
        restore_license: bool,
    ) -> Result<Self> {
        Self::with_folders(
            manifest,
            Path::new(NO_TARGET_YET),
            restore_user_data,
            restore_license,
            &current_known_folders(),
            &expand_env_vars,
            // Whether an existing service may be replaced depends on where it is being installed.
            &|_| Ok(None),
        )
    }

    /// [`Self::new`], with what comes from this PC passed in, so a test can give its own.
    fn with_folders(
        manifest: &'a PackageManifest,
        target_root: &Path,
        restore_user_data: bool,
        restore_license: bool,
        folders: &KnownFolders,
        expand: &dyn Fn(&str) -> String,
        existing_service: ExistingService<'_>,
    ) -> Result<Self> {
        let map = RestoreMap::new(manifest, target_root, restore_user_data)?;
        let mut review = Self {
            files: Vec::new(),
            empty_folders: Vec::new(),
            registry: Vec::new(),
            uninstall_keys: Vec::new(),
            license_registry: Vec::new(),
            license: Vec::new(),
            services: Vec::new(),
            drivers: Vec::new(),
            manual_drivers: Vec::new(),
            tasks: Vec::new(),
            shortcuts: Vec::new(),
            environment: Vec::new(),
            firewall_rules: Vec::new(),
            registrations: PackageRegistrations::default(),
            refused: Vec::new(),
            warnings: Vec::new(),
            user_account_known: folders.user_account_known,
            map,
            manifest,
        };
        if let Some(note) = folders.user_account_note.as_deref()
            && package_has_something_per_user(manifest)
        {
            review.warnings.push(Warning::new(
                USER_ACCOUNT_NOTE_CODE,
                WarningSeverity::Warning,
                note,
                None,
            ));
        }
        review.review_files(manifest);
        review.review_empty_folders(manifest);
        if restore_user_data {
            review.review_user_folders(target_root, &user_folder_place);
        }
        review.review_services(manifest, expand, existing_service);
        review.review_drivers(manifest, existing_service);
        let restored_services = review.restored_service_names();
        let refused_registrations = review.review_registrations(
            manifest,
            restored_license_entries(manifest, restore_license),
            &restored_services,
            expand,
        );
        review.review_registry(manifest, &restored_services, &refused_registrations, expand);
        review.review_firewall_rules(manifest, &restored_services);
        review.review_tasks(manifest, expand);
        review.review_shortcuts(manifest, folders);
        review.review_environment(manifest, restore_license);
        if restore_license {
            review.review_license(
                manifest,
                restore_user_data,
                &restored_services,
                &refused_registrations,
                expand,
            );
        }
        Ok(review)
    }

    fn refuse(&mut self, item: impl Into<String>, reason: impl std::fmt::Display) {
        let item = item.into();
        self.refused.push(Warning::new(
            REFUSED_CODE,
            WarningSeverity::Warning,
            format!("{item} was left out: {reason}."),
            Some(item),
        ));
    }

    fn review_files(&mut self, manifest: &'a PackageManifest) {
        let refused: Vec<(PathBuf, String)> = self.map.refused_files().to_vec();
        for (path, reason) in refused {
            self.refuse(format!("The file {}", path.display()), reason);
        }
        for file in manifest
            .file_inventory
            .iter()
            .filter(|entry| entry.should_copy)
        {
            if file.package_relative_path.is_none() {
                self.warnings.push(Warning::new(
                    "restore_package_path_missing",
                    WarningSeverity::Warning,
                    "A copyable file has no package relative path.",
                    Some(file.original_path.display().to_string()),
                ));
            } else if self.map.destination_of(&file.original_path).is_some() {
                self.files.push(file);
            }
        }
    }

    fn review_empty_folders(&mut self, manifest: &'a PackageManifest) {
        let refused: Vec<(PathBuf, String)> = self.map.refused_folders().to_vec();
        for (path, reason) in refused {
            self.refuse(format!("The folder {}", path.display()), reason);
        }
        if manifest.empty_folders.len() > MAX_EMPTY_FOLDERS {
            self.warnings.push(Warning::new(
                "restore_too_many_empty_folders",
                WarningSeverity::Warning,
                format!("This package lists more than {MAX_EMPTY_FOLDERS} empty folders, and only that many are made."),
                None,
            ));
        }
        // However many times a package lists the one folder, the same as however many it lists.
        self.empty_folders.extend(
            manifest
                .empty_folders
                .iter()
                .filter(|folder| {
                    self.map
                        .folder_destination_of(&folder.original_path)
                        .is_some()
                })
                .take(MAX_EMPTY_FOLDERS),
        );
    }

    /// Say where what should go in Documents or Saved Games goes instead, when those folders are
    /// not somewhere this writes. Nothing is lost, and it should not be a surprise.
    fn review_user_folders(
        &mut self,
        target_root: &Path,
        place_of: &dyn Fn(UserDataAnchor) -> UserFolderPlace,
    ) {
        for (anchor, label) in [
            (UserDataAnchor::Documents, "Documents"),
            (UserDataAnchor::SavedGames, "Saved Games"),
        ] {
            let count = self
                .files
                .iter()
                .filter(|file| {
                    data_location_of(file).is_some_and(|(found, _)| found == anchor)
                        && file.classification == FileClassification::UserData
                })
                .count();
            if count == 0 {
                continue;
            }
            let why = match place_of(anchor) {
                UserFolderPlace::Usable(_) => continue,
                UserFolderPlace::Elsewhere(folder) => format!(
                    "this PC's {label} folder is at {}, and only a folder inside your profile is \
                     written to",
                    folder.display()
                ),
                UserFolderPlace::Unknown => {
                    format!("where this PC's {label} folder is could not be told")
                }
            };
            self.warnings.push(Warning::new(
                USER_FOLDER_ELSEWHERE_CODE,
                WarningSeverity::Warning,
                format!(
                    "{count} files from {label} go under {} instead, in a folder named _external, \
                     because {why}. To use them, move them to your {label} folder yourself.",
                    target_root.display()
                ),
                None,
            ));
        }
    }

    /// Whether this is to be left out for going to an account that could not be told, saying so.
    fn left_out_for_no_account(&mut self, is_per_user: bool, item: impl Into<String>) -> bool {
        if is_per_user && !self.user_account_known {
            self.refuse(item, NO_USER_ACCOUNT_REFUSAL);
            return true;
        }
        false
    }

    /// Why a name already taken on this PC must be left alone, if so. This has to be known
    /// before anything is written, since a restored service's name opens its registry key.
    fn existing_service_problem(
        &self,
        name: &str,
        existing_service: ExistingService<'_>,
    ) -> Option<String> {
        match existing_service(name) {
            Ok(None) => None,
            Ok(Some(current)) => {
                existing_service_refusal(name, &current, self.map.target_root(), system_root())
            }
            Err(error) => Some(format!("its current settings could not be read: {error}")),
        }
    }

    fn review_services(
        &mut self,
        manifest: &'a PackageManifest,
        expand: &dyn Fn(&str) -> String,
        existing_service: ExistingService<'_>,
    ) {
        for service in &manifest.services_detected {
            let item = format!("The service {}", service.service_name);
            if !is_valid_service_name(&service.service_name) {
                self.refuse(item, "Windows would not accept its name");
                continue;
            }
            if let Some(problem) =
                self.existing_service_problem(&service.service_name, existing_service)
            {
                self.refuse(item, problem);
                continue;
            }
            if !service
                .dependencies
                .iter()
                .all(|name| is_valid_service_name(name))
            {
                self.refuse(item, "it depends on a service with an invalid name");
                continue;
            }
            // With no fallback to the manifest's own path: a service runs elevated, and the
            // only thing it may run is a file this package puts in place.
            let Some(binary) = self.map.runnable_destination_of(&service.binary_path) else {
                self.refuse(
                    item,
                    format!(
                        "it runs {}, which this package does not install",
                        service.binary_path.display()
                    ),
                );
                continue;
            };
            let binary = binary.to_path_buf();
            let arguments = service
                .arguments
                .as_deref()
                .map(|text| self.map.rewrite(text));
            let account = ServiceAccount::from_recorded(
                service.account_name.as_deref(),
                &service.service_name,
            );
            self.services.push(ReviewedService {
                entry: service,
                binary,
                arguments,
                account,
            });
        }
        // A service's own parameters are checked once the set of restored names is known.
        let names = self.restored_service_names();
        let mut bad = Vec::new();
        for service in &self.services {
            for entry in &service.entry.parameters {
                if !is_own_parameter(entry, &service.entry.service_name)
                    || registry_refusal(entry, &self.map, &names, expand).is_some()
                {
                    bad.push(service.entry.service_name.clone());
                    break;
                }
            }
        }
        for name in bad {
            self.services
                .retain(|service| service.entry.service_name != name);
            self.refuse(
                format!("The service {name}"),
                "its settings name something this package does not install, or a key that is not \
                 its own",
            );
        }
    }

    fn review_drivers(
        &mut self,
        manifest: &'a PackageManifest,
        existing_service: ExistingService<'_>,
    ) {
        for driver in &manifest.drivers_detected {
            let Some(relative) = driver.package_relative_path.as_deref() else {
                self.manual_drivers.push(driver);
                continue;
            };
            let item = format!("The driver {}", driver.service_name);
            if !is_valid_service_name(&driver.service_name) {
                self.refuse(item, "Windows would not accept its name");
                continue;
            }
            if driver.hash_sha256.is_none() {
                self.refuse(item, "it has no hash to check it by");
                continue;
            }
            if let Some(problem) =
                self.existing_service_problem(&driver.service_name, existing_service)
            {
                self.refuse(item, problem);
                continue;
            }
            match driver_destination(driver, &self.map) {
                Ok(destination) => self.drivers.push(ReviewedDriver {
                    entry: driver,
                    package_relative_path: relative,
                    destination,
                }),
                Err(error) => self.refuse(item, error),
            }
        }
    }

    fn restored_service_names(&self) -> HashSet<String> {
        self.services
            .iter()
            .map(|service| service.entry.service_name.to_ascii_lowercase())
            .chain(
                self.drivers
                    .iter()
                    .map(|driver| driver.entry.service_name.to_ascii_lowercase()),
            )
            .collect()
    }

    /// What the package registers, as a whole, with what of it must all be left out and why.
    /// A class means nothing without its server, and so on, and where a package asks for one part
    /// of something it cannot have, it gets none of it.
    fn review_registrations(
        &mut self,
        manifest: &'a PackageManifest,
        license_entries: &'a [RegistryEntry],
        restored_services: &HashSet<String>,
        expand: &dyn Fn(&str) -> String,
    ) -> HashMap<String, String> {
        let entries: Vec<&RegistryEntry> = manifest
            .registry_keys_exported
            .iter()
            .chain(license_entries)
            .collect();
        let destinations: Vec<&Path> = self
            .files
            .iter()
            .filter_map(|file| self.map.runnable_destination_of(&file.original_path))
            .collect();
        self.registrations =
            PackageRegistrations::of(&entries, &destinations, &self.map, expand, system_root());
        let mut units: HashMap<String, Vec<&RegistryEntry>> = HashMap::new();
        for entry in &entries {
            if let Some(unit) = registration_unit_of(entry) {
                units.entry(unit).or_default().push(entry);
            }
        }
        units
            .into_iter()
            .filter_map(|(unit, of_it)| {
                registration_refusal(
                    &unit,
                    &of_it,
                    &self.map,
                    &self.registrations,
                    restored_services,
                    expand,
                    system_root(),
                )
                .map(|reason| (unit, reason))
            })
            .collect()
    }

    /// Why this must be left out, if so, by every rule there is. All that is restored as a value
    /// comes through here, by whichever list it came in.
    fn registry_problem(
        &self,
        entry: &RegistryEntry,
        restored_services: &HashSet<String>,
        refused_registrations: &HashMap<String, String>,
        expand: &dyn Fn(&str) -> String,
    ) -> Option<String> {
        registry_refusal(entry, &self.map, restored_services, expand)
            .or_else(|| {
                registration_unit_of(entry)
                    .and_then(|unit| refused_registrations.get(&unit).cloned())
            })
            .or_else(|| registered_value_refusal(entry, &self.map, &self.registrations, expand))
    }

    fn review_firewall_rules(
        &mut self,
        manifest: &'a PackageManifest,
        restored_services: &HashSet<String>,
    ) {
        for rule in &manifest.firewall_rules_detected {
            let item = format!("The firewall rule {}", rule.name);
            if let Some(reason) = firewall_refusal(rule, &self.map, restored_services) {
                self.refuse(item, reason);
                continue;
            }
            let Some(program) = self.map.runnable_destination_of(&rule.program) else {
                continue;
            };
            let program = program.to_path_buf();
            self.firewall_rules.push(ReviewedFirewallRule {
                entry: rule,
                program,
            });
        }
    }

    fn review_registry(
        &mut self,
        manifest: &'a PackageManifest,
        restored_services: &HashSet<String>,
        refused_registrations: &HashMap<String, String>,
        expand: &dyn Fn(&str) -> String,
    ) {
        for entry in &manifest.registry_keys_exported {
            if self.left_out_for_no_account(
                entry.hive == RegistryHive::CurrentUser,
                registry_item(entry),
            ) {
                continue;
            }
            match self.registry_problem(entry, restored_services, refused_registrations, expand) {
                None => {
                    self.note_uninstall_entry(entry);
                    // How to remove it from the PC it came from. The install sets its own.
                    if !is_source_uninstall_value(entry) {
                        self.registry.push(entry);
                    }
                }
                Some(reason) => self.refuse(registry_item(entry), reason),
            }
        }
    }

    fn note_uninstall_entry(&mut self, entry: &RegistryEntry) {
        if !is_uninstall_entry_key(&entry.key_path) {
            return;
        }
        let is_display_name = entry
            .value_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(DISPLAY_NAME_VALUE));
        let known = self.uninstall_keys.iter_mut().find(|key| {
            key.hive == entry.hive && key.key_path.eq_ignore_ascii_case(&entry.key_path)
        });
        match known {
            Some(key) => key.has_display_name |= is_display_name,
            None => self.uninstall_keys.push(UninstallKey {
                hive: entry.hive.clone(),
                key_path: entry.key_path.clone(),
                has_display_name: is_display_name,
            }),
        }
    }

    fn review_license(
        &mut self,
        manifest: &'a PackageManifest,
        restore_user_data: bool,
        restored_services: &HashSet<String>,
        refused_registrations: &HashMap<String, String>,
        expand: &dyn Fn(&str) -> String,
    ) {
        for artifact in manifest
            .license_artifacts_detected
            .iter()
            .filter(|artifact| artifact.should_copy)
        {
            let (Some(relative), Some(original)) = (
                artifact.package_relative_path.as_deref(),
                artifact.path.as_deref(),
            ) else {
                continue;
            };
            let destination =
                license_destination(original, &self.map, restore_user_data, &real_location)
                    .and_then(|destination| {
                        match store_file_refusal(artifact, original, &destination, &self.map) {
                            Some(reason) => Err(reason),
                            None => Ok(destination),
                        }
                    });
            match destination {
                Ok(destination) => self.license.push(ReviewedLicense {
                    artifact,
                    package_relative_path: relative,
                    destination,
                }),
                Err(reason) => self.refuse(
                    format!("The license data at {}", original.display()),
                    reason,
                ),
            }
        }
        for entry in &manifest.license_registry_entries {
            // License values for the whole machine are only ever reported.
            if entry.hive == RegistryHive::LocalMachine {
                self.refuse(
                    format!("The license value {}", registry_item(entry)),
                    "a license stored for the whole machine is only reported",
                );
                continue;
            }
            if is_autostart_entry(entry) {
                self.refuse(
                    format!("The license value {}", registry_item(entry)),
                    "a license has no reason to start anything",
                );
                continue;
            }
            if self.left_out_for_no_account(
                entry.hive == RegistryHive::CurrentUser,
                registry_item(entry),
            ) {
                continue;
            }
            match self.registry_problem(entry, restored_services, refused_registrations, expand) {
                None if is_source_uninstall_value(entry) => {}
                None => self.license_registry.push(entry),
                Some(reason) => self.refuse(registry_item(entry), reason),
            }
        }
    }

    fn review_tasks(&mut self, manifest: &'a PackageManifest, expand: &dyn Fn(&str) -> String) {
        for task in &manifest.scheduled_tasks_detected {
            let item = format!("The scheduled task {}{}", task.task_path, task.task_name);
            let Some(xml) = task.definition_xml.as_deref() else {
                self.refuse(item, "its definition was not captured");
                continue;
            };
            let rewritten = self.map.rewrite_xml(xml).and_then(|rewritten| {
                read_task_definition(&rewritten).map(|definition| (rewritten, definition))
            });
            let (xml, mut definition) = match rewritten {
                Ok(parsed) => parsed,
                Err(error) => {
                    self.refuse(item, format!("its definition could not be read: {error}"));
                    continue;
                }
            };
            let xml = if xml.contains(WHOEVER_INSTALLS_IT) {
                let Some(account) = signed_in_user().account() else {
                    self.refuse(
                        item,
                        "it runs as whoever installs it, and who is signed in here could not be told",
                    );
                    continue;
                };
                if definition.account.as_deref() == Some(WHOEVER_INSTALLS_IT) {
                    definition.account = Some(THE_PERSON_INSTALLING.to_owned());
                }
                xml.replace(WHOEVER_INSTALLS_IT, &account.sid)
            } else {
                xml
            };
            if let Some(reason) = task_refusal(
                &task.task_path,
                &task.task_name,
                &definition,
                &self.map,
                expand,
            ) {
                self.refuse(item, reason);
                continue;
            }
            self.tasks.push(ReviewedTask {
                entry: task,
                xml,
                commands: definition.commands,
                arguments: definition.arguments,
                runs_as_system: definition.runs_as_system,
                highest_available: definition.highest_available,
                account: definition.account,
            });
        }
    }

    fn review_shortcuts(&mut self, manifest: &'a PackageManifest, folders: &KnownFolders) {
        for shortcut in &manifest.shortcuts_detected {
            let item = format!("The shortcut {}", shortcut.original_lnk_path.display());
            let placement = match shortcut_placement(&shortcut.original_lnk_path, folders) {
                Ok(placement) => placement,
                Err(reason) => {
                    self.refuse(item, reason);
                    continue;
                }
            };
            let Some(target) = self.map.runnable_destination_of(&shortcut.target_path) else {
                self.refuse(
                    item,
                    format!(
                        "it opens {}, which this package does not install",
                        shortcut.target_path.display()
                    ),
                );
                continue;
            };
            let target = target.to_path_buf();
            // Windows opens a shortcut's icon just by showing it, so one on a network share
            // would hand that server the name and password hash of whoever looks at it.
            let local_path = |path: &PathBuf| {
                Some(self.map.rewrite(&path.to_string_lossy()))
                    .filter(|path| !is_network_path(path))
            };
            let working_directory = shortcut.working_directory.as_ref().and_then(local_path);
            let icon = shortcut.icon_path.as_ref().and_then(local_path);
            let arguments = shortcut
                .arguments
                .as_deref()
                .map(|text| self.map.rewrite(text));
            self.shortcuts.push(ReviewedShortcut {
                original_lnk_path: shortcut.original_lnk_path.clone(),
                placement,
                target,
                arguments,
                working_directory,
                icon,
            });
        }
    }

    fn review_environment(&mut self, manifest: &'a PackageManifest, restore_license: bool) {
        for variable in &manifest.environment_variables_detected {
            // It says where a license is, so it goes by what the license goes by.
            if !restore_license && is_license_env_var(&variable.name) {
                continue;
            }
            if self.left_out_for_no_account(
                variable.scope == EnvironmentScope::User,
                format!("The environment variable {}", variable.name),
            ) {
                continue;
            }
            match environment_refusal(&variable.name) {
                None => self.environment.push(variable),
                Some(reason) => {
                    self.refuse(
                        format!("The environment variable {}", variable.name),
                        reason,
                    );
                }
            }
        }
    }
}

/// What of a package is restored as license values, for what the rest goes by.
fn restored_license_entries(manifest: &PackageManifest, restore_license: bool) -> &[RegistryEntry] {
    if restore_license {
        &manifest.license_registry_entries
    } else {
        &[]
    }
}

/// True when a captured value sits in this service's own `Parameters` key.
fn is_own_parameter(entry: &RegistryEntry, service_name: &str) -> bool {
    if entry.hive != RegistryHive::LocalMachine {
        return false;
    }
    let lower = entry.key_path.to_ascii_lowercase();
    let parameters = format!(
        "{SERVICES_PREFIX}{}\\{PARAMETERS_SUBKEY}",
        service_name.to_ascii_lowercase()
    );
    lower == parameters || lower.starts_with(&format!("{parameters}\\"))
}

pub(crate) fn registry_item(entry: &RegistryEntry) -> String {
    let hive = match entry.hive {
        RegistryHive::LocalMachine => "HKLM",
        RegistryHive::CurrentUser => "HKCU",
    };
    format!(
        "{hive}\\{}\\{}",
        entry.key_path,
        entry.value_name.as_deref().unwrap_or("(Default)")
    )
}

/// This PC's own location for a path under a known data folder, with the part below it.
fn real_location(original: &Path) -> Option<(PathBuf, PathBuf)> {
    let (anchor, tail) = split_user_data_relative(original)?;
    if has_reserved_device_name(&tail) {
        return None;
    }
    let base = user_data_base(anchor)?;
    Some((base.join(&tail), tail))
}

/// Why a file of a license store cannot go where the store does, if so. A store's root can be
/// somewhere fine, and what it holds can be named to land in a Startup folder or where Office loads
/// from. Under the install location a file's name means nothing to Windows. The whole store goes
/// or stays, since a part of one is not a license, and one that asks for this is not one.
fn store_file_refusal(
    artifact: &LicenseArtifact,
    original: &Path,
    destination: &Path,
    map: &RestoreMap,
) -> Option<String> {
    if path_is_within(destination, map.target_root()) {
        return None;
    }
    artifact.store_files.iter().find_map(|file| {
        let source = original.join(safe_relative_path(&file.relative_path));
        split_user_data_relative(&source).and_then(|(_, tail)| data_location_refusal(&tail))
    })
}

/// The Desktop and Start Menu folders of whoever is signed in, whoever this is running as, and the
/// ones shared by everyone on the PC.
fn current_known_folders() -> KnownFolders {
    let signed_in = signed_in_user();
    let shell_folder = |value: &str, default: &str| {
        signed_in.account().and_then(|account| {
            match account_shell_folder(account, value, default) {
                UserFolderPlace::Usable(folder) => Some(folder),
                _ => None,
            }
        })
    };
    let program_data = user_data_base(UserDataAnchor::ProgramData);
    let public = user_data_base(UserDataAnchor::PublicProfile);
    KnownFolders {
        user_desktop: shell_folder(DESKTOP_VALUE, DESKTOP_FALLBACK),
        common_desktop: public.map(|public| public.join(DESKTOP_FOLDER)),
        user_start_menu: shell_folder(START_MENU_VALUE, START_MENU_FALLBACK),
        common_start_menu: program_data.map(|base| base.join(START_MENU_SUBFOLDER)),
        // Under `HKEY_USERS` while they are signed in, which with no desktop they may not be.
        user_account_known: signed_in.account().is_some_and(|account| {
            RegKey::predef(HKEY_USERS)
                .open_subkey_with_flags(&account.sid, KEY_READ)
                .is_ok()
        }),
        user_account_note: user_account_note(signed_in, current_user_sid()),
    }
}

/// What a person ought to know about whose per-user folders and settings these are, when it is not
/// simply the account they are using.
fn user_account_note(signed_in: &SignedInUser, running_as: Option<&str>) -> Option<String> {
    match signed_in {
        SignedInUser::Desktop(account) if running_as == Some(account.sid.as_str()) => None,
        SignedInUser::Desktop(account) => Some(format!(
            "User data and per-user settings go to {}, who is signed in here, and not to the \
             administrator this is running as.",
            account.name()
        )),
        SignedInUser::NoDesktop(account) => Some(format!(
            "No one is signed in to this session, so user data and per-user settings go to {}, \
             the account this is running as.",
            account.name()
        )),
        SignedInUser::Unknown(reason) => Some(format!(
            "Could not tell who is signed in ({reason}), so per-user settings are left out, and \
             user data goes under the install location."
        )),
    }
}

/// Whether anything in a package belongs in an account rather than on the PC.
fn package_has_something_per_user(manifest: &PackageManifest) -> bool {
    manifest
        .registry_keys_exported
        .iter()
        .chain(&manifest.license_registry_entries)
        .any(|entry| entry.hive == RegistryHive::CurrentUser)
        || manifest
            .environment_variables_detected
            .iter()
            .any(|variable| variable.scope == EnvironmentScope::User)
        || manifest
            .file_inventory
            .iter()
            .any(|file| file.should_copy && file.classification == FileClassification::UserData)
}

/// What kind of change a package makes beyond its files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemChangeKind {
    Service,
    Driver,
    ScheduledTask,
    /// Runs by itself at sign-in: a Run value, Active Setup, or a Startup shortcut.
    Autostart,
    Shortcut,
    PathEntry,
    EnvironmentVariable,
    /// Loads into Explorer, for everyone who uses it.
    ShellExtension,
    /// A command on the menu of files or folders.
    ShellCommand,
    /// Links of a kind open with the app.
    UrlProtocol,
    FileType,
    /// Windows starts it when another program asks for one of its classes.
    ComServer,
    FirewallRule,
    EventLogSource,
    Font,
    OdbcDriver,
    OdbcSource,
    CompatibilitySetting,
    DefaultApps,
    /// A name for the program that Run and other programs can find it by.
    ProgramName,
    /// Values somewhere other than the app's own keys and the kinds above.
    OtherSettings,
}

impl SystemChangeKind {
    /// What to call this kind of change in a list.
    pub fn label(self) -> &'static str {
        match self {
            Self::Service => "Service",
            Self::Driver => "Driver",
            Self::ScheduledTask => "Scheduled task",
            Self::Autostart => "Starts automatically",
            Self::Shortcut => "Shortcut",
            Self::PathEntry => "PATH",
            Self::EnvironmentVariable => "Environment variable",
            Self::ShellExtension => "Shell extension",
            Self::ShellCommand => "Menu command",
            Self::UrlProtocol => "Link type",
            Self::FileType => "File type",
            Self::ComServer => "COM",
            Self::FirewallRule => "Firewall rule",
            Self::EventLogSource => "Event log source",
            Self::Font => "Font",
            Self::OdbcDriver => "ODBC driver",
            Self::OdbcSource => "ODBC data source",
            Self::CompatibilitySetting => "Compatibility setting",
            Self::DefaultApps => "Default apps",
            Self::ProgramName => "Program name",
            Self::OtherSettings => "Other settings",
        }
    }
}

/// One thing a package sets up beyond its files, described for the person installing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemChange {
    pub kind: SystemChangeKind,
    pub title: String,
    pub detail: String,
    /// Runs with full rights, so it deserves a second look.
    pub high_rights: bool,
}

/// What a package does to the PC beyond copying files.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SystemChanges {
    pub changes: Vec<SystemChange>,
    /// What the package asked for and will not be given, as sentences.
    pub refused: Vec<String>,
    /// What to know about whose folders and settings these will be, as sentences.
    pub notes: Vec<String>,
}

/// Describe what a package would do with the options an installer's window starts on. See
/// [`system_changes_restoring`].
pub fn system_changes(manifest: &PackageManifest) -> Result<SystemChanges> {
    system_changes_restoring(
        manifest,
        manifest.included_user_data || manifest.included_program_data,
        manifest.included_license_data,
    )
}

/// Describe what an install with these options would do, for showing before anything is installed.
/// It goes by the same review an install does, given the same two options, so what this says was
/// left out is left out. Whether an item is allowed does not depend on where the app is installed,
/// so this is right for any install location chosen later. `Err` means the package would be refused
/// whole.
pub fn system_changes_restoring(
    manifest: &PackageManifest,
    restore_user_data: bool,
    restore_license: bool,
) -> Result<SystemChanges> {
    let fit = refuse_what_cannot_run_here(manifest)?;
    let review =
        ReviewedPackage::before_a_location_is_chosen(manifest, restore_user_data, restore_license)?;
    let mut changes = describe(&review, Path::new(NO_TARGET_YET));
    changes.notes.extend(fit.warnings);
    Ok(changes)
}

/// The Windows this is running on, from what it records about itself, which only an administrator
/// can change. What a process's environment says of its architecture is what it is emulated as.
pub fn windows_here() -> WindowsHere {
    WindowsHere {
        build: Some(windows_build()).filter(|build| *build != 0),
        version: windows_version_numbers(),
        architecture: windows_architecture_name()
            .as_deref()
            .map_or(AppArchitecture::Unknown, architecture_from_name),
    }
}

/// Whether what this package holds can run on this PC, as far as can be told. An app that cannot is
/// refused here, before anything of it is unpacked, and what is left is what may not work, for the
/// person installing to read.
pub fn refuse_what_cannot_run_here(manifest: &PackageManifest) -> Result<PlatformFit> {
    let fit = platform_fit(
        manifest.source_platform.as_ref(),
        &manifest.program_needs,
        &windows_here(),
    );
    match fit.refusal.as_deref() {
        Some(refusal) => Err(WinAppMigratorError::InvalidInput(format!(
            "this app cannot run on this PC, so nothing was changed: {refusal}"
        ))),
        None => Ok(fit),
    }
}

/// What an install of this package would need and where, with what it would replace as things stand.
/// A file that is refused takes no room. Nothing is looked at beyond whether each destination is
/// there and how big it is, and it is the same review the install goes by.
#[allow(clippy::too_many_arguments)]
pub fn install_places(
    manifest: &PackageManifest,
    target_root: &Path,
    restore_user_data: bool,
    restore_license: bool,
    staging: &Path,
    unpacked_bytes: u64,
    state_root: &Path,
    stub_bytes: u64,
) -> Result<Vec<PlaceNeed>> {
    let review = ReviewedPackage::new(manifest, target_root, restore_user_data, restore_license)?;
    let mut planned: Vec<PlannedFile> = review
        .files
        .iter()
        .filter_map(|file| {
            let destination = review.map.destination_of(&file.original_path)?;
            Some(PlannedFile {
                destination: destination.to_path_buf(),
                new_bytes: file.size_bytes.unwrap_or(0),
                replaced_bytes: replaced_bytes(destination),
            })
        })
        .collect();
    for driver in &review.drivers {
        planned.push(PlannedFile {
            destination: driver.destination.clone(),
            new_bytes: DRIVER_ALLOWANCE_BYTES,
            replaced_bytes: replaced_bytes(&driver.destination),
        });
    }
    for license in &review.license {
        if license.artifact.store_files.is_empty() {
            planned.push(PlannedFile {
                destination: license.destination.clone(),
                new_bytes: license.artifact.size_bytes.unwrap_or(0),
                replaced_bytes: replaced_bytes(&license.destination),
            });
        }
        for file in &license.artifact.store_files {
            let destination = store_file_destination(&license.destination, file);
            planned.push(PlannedFile {
                new_bytes: file.size_bytes,
                replaced_bytes: replaced_bytes(&destination),
                destination,
            });
        }
    }
    // By its folder, since a large app replaces many files in few folders.
    let asked: RefCell<HashMap<PathBuf, bool>> = RefCell::new(HashMap::new());
    let on_records_volume = |destination: &Path| {
        let folder = destination.parent().unwrap_or(destination).to_path_buf();
        *asked
            .borrow_mut()
            .entry(folder)
            .or_insert_with(|| on_same_volume(destination, state_root))
    };
    Ok(install_needs(
        staging,
        unpacked_bytes,
        &planned,
        state_root,
        stub_bytes,
        &on_records_volume,
    ))
}

/// Where a file of a license store goes. What a package says of it is cleaned the way the path it
/// is written to is, so that what is worked out about it beforehand is about the same place. A
/// path with a drive in it would otherwise be somewhere else altogether.
pub(crate) fn store_file_destination(store: &Path, file: &LicenseStoreFile) -> PathBuf {
    store.join(safe_relative_path(&file.relative_path))
}

/// How big a file already at a destination is, and nothing is followed to find out.
fn replaced_bytes(destination: &Path) -> u64 {
    std::fs::symlink_metadata(destination).map_or(0, |metadata| metadata.len())
}

/// Stop an install or a preview that does not have the room it needs, before anything is unpacked.
/// What could not be checked is returned, for saying.
pub fn refuse_without_room_to_install(needs: &[PlaceNeed]) -> Result<RoomCheck> {
    let room = RoomCheck::on_this_pc(needs);
    let shortfalls = room.shortfalls();
    if shortfalls.is_empty() {
        return Ok(room);
    }
    Err(WinAppMigratorError::InvalidInput(format!(
        "there is not enough room for this, so nothing was changed. {}. Free up some space there, \
         or choose an install location on another drive.",
        shortfalls.join(". ")
    )))
}

/// Why this folder is not one to install to without being told to: it has files in it, and no
/// install on record put them there. They would be another app's or a person's. It holds for a file
/// that is new as much as for one that replaces something, since a DLL added beside another app's
/// programs is loaded by them.
pub fn files_no_install_put_there(folder: &Path, state_root: Option<&Path>) -> Option<String> {
    let on_record =
        state_root.is_some_and(|state_root| install_on_record_at(state_root, folder, None));
    (folder_has_files(folder) && !on_record).then(|| {
        "it already has files, and no install is on record here, so use an empty folder".to_owned()
    })
}

/// Whether anything is in a folder. One that is there and will not say is not taken to be empty.
pub fn folder_has_files(folder: &Path) -> bool {
    match std::fs::read_dir(folder) {
        Ok(mut entries) => entries.next().is_some(),
        Err(error) => error.kind() != std::io::ErrorKind::NotFound,
    }
}

/// Why this install location must not be used on this PC, if so.
pub fn install_location_problem(target_root: &Path) -> Option<String> {
    let state_root = machine_state_root();
    if state_root
        .as_deref()
        .is_some_and(|state_root| overlaps_by_how_written(target_root, state_root))
    {
        return Some(STATE_ROOT_REFUSAL.to_owned());
    }
    // What can be told from how it is written comes first, so that a path on a share is not sent
    // to the network, which a window would have to wait for.
    let shared_folders = known_base_dirs(&SystemContext::live());
    if let Some(refusal) = install_location_refusal(target_root, system_root(), &shared_folders) {
        return Some(refusal);
    }
    if state_root.is_some_and(|state_root| overlaps_state_root(target_root, &state_root)) {
        return Some(STATE_ROOT_REFUSAL.to_owned());
    }
    // And where it really is, past any link or short name.
    install_placement_refusal(
        &resolved_location(target_root),
        system_root(),
        &shared_folders,
    )
    .or_else(|| first_link_to(target_root).map(|link| link_refusal(&link)))
}

fn overlaps_by_how_written(target: &Path, state: &Path) -> bool {
    path_is_within(target, state) || path_is_within(state, target)
}

/// True when an install location would put an app's files where installs keep their records.
/// What an uninstall trusts is there, and a package gets to suggest the location.
pub(crate) fn overlaps_state_root(target_root: &Path, state_root: &Path) -> bool {
    overlaps_by_how_written(target_root, state_root)
        || overlaps_by_how_written(
            &resolved_location(target_root),
            &resolved_location(state_root),
        )
}

fn describe(review: &ReviewedPackage<'_>, target_root: &Path) -> SystemChanges {
    SystemChanges {
        changes: described_rows(review, target_root)
            .into_iter()
            .map(|row| row.change)
            .collect(),
        refused: review
            .refused
            .iter()
            .map(|warning| warning.message.clone())
            .collect(),
        notes: notes_of(review),
    }
}

/// What a person ought to know of a review, as sentences.
pub(crate) fn notes_of(review: &ReviewedPackage<'_>) -> Vec<String> {
    let mut notes: Vec<String> = review
        .warnings
        .iter()
        .filter(|warning| {
            warning.code == USER_ACCOUNT_NOTE_CODE || warning.code == USER_FOLDER_ELSEWHERE_CODE
        })
        .map(|warning| warning.message.clone())
        .collect();
    notes.extend(notes_on_what_is_left_out(review.manifest));
    notes
}

/// One thing a package sets up, and what a person can leave it out by.
pub(crate) struct ChangeRow {
    pub id: ChoiceId,
    pub change: SystemChange,
}

/// What a service is known by in a choice.
pub(crate) fn service_choice(name: &str) -> ChoiceId {
    ChoiceId::Service(name.to_ascii_lowercase())
}

pub(crate) fn driver_choice(name: &str) -> ChoiceId {
    ChoiceId::Driver(name.to_ascii_lowercase())
}

pub(crate) fn task_choice(task_path: &str, task_name: &str) -> ChoiceId {
    ChoiceId::Task(format!("{task_path}{task_name}").to_ascii_lowercase())
}

pub(crate) fn shortcut_choice(original: &Path) -> ChoiceId {
    ChoiceId::Shortcut(original.to_string_lossy().to_ascii_lowercase())
}

pub(crate) fn autostart_choice(entry: &RegistryEntry) -> ChoiceId {
    ChoiceId::Autostart(registry_item(entry).to_ascii_lowercase())
}

/// The words a package's list of what it sets up uses, with each thing's own way to leave it out.
/// Everything on that list comes through here, so what the app and an installer say cannot differ.
pub(crate) fn described_rows(review: &ReviewedPackage<'_>, target_root: &Path) -> Vec<ChangeRow> {
    let mut changes = Vec::new();
    for service in &review.services {
        let starts = service
            .entry
            .start_type
            .as_deref()
            .map(service_start_text)
            .unwrap_or_default();
        let runs_as = service.account.label();
        changes.push(ChangeRow {
            id: service_choice(&service.entry.service_name),
            change: SystemChange {
                kind: SystemChangeKind::Service,
                title: service
                    .entry
                    .display_name
                    .clone()
                    .unwrap_or_else(|| service.entry.service_name.clone()),
                detail: format!(
                    "Runs {} as {runs_as}{starts}",
                    file_name_text(&service.binary)
                ),
                high_rights: matches!(
                    service.account,
                    ServiceAccount::Unspecified | ServiceAccount::LocalSystem
                ),
            },
        });
    }
    for driver in &review.drivers {
        let starts = driver
            .entry
            .start_type
            .as_deref()
            .map(|start| format!(", starts {start}"))
            .unwrap_or_default();
        changes.push(ChangeRow {
            id: driver_choice(&driver.entry.service_name),
            change: SystemChange {
                kind: SystemChangeKind::Driver,
                title: driver.entry.service_name.clone(),
                detail: format!(
                    "Loads {} into Windows itself{starts}",
                    file_name_text(&driver.destination)
                ),
                high_rights: true,
            },
        });
    }
    for task in &review.tasks {
        let runs_as = if task.runs_as_system {
            "SYSTEM".to_owned()
        } else {
            task.account
                .clone()
                .unwrap_or_else(|| "the user who created it".to_owned())
        };
        changes.push(ChangeRow {
            id: task_choice(&task.entry.task_path, &task.entry.task_name),
            change: SystemChange {
                kind: SystemChangeKind::ScheduledTask,
                title: format!("{}{}", task.entry.task_path, task.entry.task_name),
                detail: if task.highest_available {
                    format!("Runs as {runs_as}, with the highest rights it can have")
                } else {
                    format!("Runs as {runs_as}")
                },
                high_rights: task.runs_as_system || task.highest_available,
            },
        });
    }
    for entry in review
        .registry
        .iter()
        .filter(|entry| is_autostart_entry(entry))
    {
        let scope = match entry.hive {
            RegistryHive::LocalMachine => "for everyone who signs in",
            RegistryHive::CurrentUser => "when you sign in",
        };
        changes.push(ChangeRow {
            id: autostart_choice(entry),
            change: SystemChange {
                kind: SystemChangeKind::Autostart,
                title: entry
                    .value_name
                    .clone()
                    .unwrap_or_else(|| entry.key_path.clone()),
                detail: format!("Runs {scope}"),
                high_rights: false,
            },
        });
    }
    for shortcut in &review.shortcuts {
        let name = shortcut
            .placement
            .destination
            .file_stem()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        let place = if shortcut.placement.for_all_users {
            "for everyone on this PC"
        } else {
            "for you"
        };
        let id = shortcut_choice(&shortcut.original_lnk_path);
        if shortcut.placement.autostart {
            changes.push(ChangeRow {
                id,
                change: SystemChange {
                    kind: SystemChangeKind::Autostart,
                    title: name,
                    detail: format!("A shortcut in the Startup folder, {place}"),
                    high_rights: false,
                },
            });
        } else {
            changes.push(ChangeRow {
                id,
                change: SystemChange {
                    kind: SystemChangeKind::Shortcut,
                    title: name,
                    detail: format!("Opens {}, {place}", file_name_text(&shortcut.target)),
                    high_rights: false,
                },
            });
        }
    }
    for variable in &review.environment {
        if is_path_variable(&variable.name) {
            let rewritten = review.map.rewrite(&variable.value);
            for entry in app_path_entries(&rewritten, target_root) {
                changes.push(ChangeRow {
                    id: ChoiceId::Integration(SystemChangeKind::PathEntry),
                    change: SystemChange {
                        kind: SystemChangeKind::PathEntry,
                        title: format!("Adds to {}", variable.name),
                        detail: strip_prefix_ignore_case(Path::new(&entry), target_root)
                            .map_or_else(
                                || "The install folder".to_owned(),
                                |relative| {
                                    if relative.as_os_str().is_empty() {
                                        "The install folder".to_owned()
                                    } else {
                                        format!(r"The install folder\{}", relative.display())
                                    }
                                },
                            ),
                        high_rights: false,
                    },
                });
            }
        } else {
            changes.push(ChangeRow {
                id: ChoiceId::Integration(SystemChangeKind::EnvironmentVariable),
                change: SystemChange {
                    kind: SystemChangeKind::EnvironmentVariable,
                    title: variable.name.clone(),
                    detail: format!("Set to {}", variable.value),
                    high_rights: false,
                },
            });
        }
    }
    let by_kind = |change: SystemChange| ChangeRow {
        id: ChoiceId::Integration(change.kind),
        change,
    };
    changes.extend(
        describe_firewall_rules(&review.firewall_rules)
            .into_iter()
            .map(by_kind),
    );
    changes.extend(describe_registrations(review).into_iter().map(by_kind));
    changes.extend(describe_other_settings(review).map(by_kind));
    changes
}

/// How many of the keys are named before saying how many more there are.
const OTHER_SETTINGS_KEYS_NAMED: usize = 3;

/// Whatever a package writes that is not the app's own and not a kind this list has words for. The
/// refusals can never name every key Windows acts on, so what is outside them is at least said.
fn describe_other_settings(review: &ReviewedPackage<'_>) -> Option<SystemChange> {
    let app = AppKeys::of(review.manifest);
    let entries: Vec<&RegistryEntry> = review
        .registry
        .iter()
        .chain(&review.license_registry)
        .copied()
        .filter(|entry| is_other_setting(entry, &app))
        .collect();
    if entries.is_empty() {
        return None;
    }
    let keys: BTreeSet<String> = entries
        .iter()
        .map(|entry| {
            let hive = match entry.hive {
                RegistryHive::LocalMachine => "HKLM",
                RegistryHive::CurrentUser => "HKCU",
            };
            format!(r"{hive}\{}", entry.key_path)
        })
        .collect();
    let more = keys.len().saturating_sub(OTHER_SETTINGS_KEYS_NAMED);
    let named: Vec<String> = keys.into_iter().take(OTHER_SETTINGS_KEYS_NAMED).collect();
    let values = entries.len();
    Some(SystemChange {
        kind: SystemChangeKind::OtherSettings,
        title: format!(
            "{values} {} outside the app's own keys",
            if values == 1 { "setting" } else { "settings" }
        ),
        detail: if more > 0 {
            format!("In {}, and {more} more", named.join(", "))
        } else {
            format!("In {}", named.join(", "))
        },
        high_rights: entries.iter().any(|entry| is_a_windows_key(entry)),
    })
}

/// What a package says an app goes by, for telling its own keys. It comes from the package, and this
/// only decides what a person is told about, so the worst a package can do by lying is go unmentioned
/// in a vendor's keys it claims. Windows' own are never taken for an app's, whatever it claims.
pub(crate) struct AppKeys {
    names: Vec<String>,
}

/// Fewer letters than this say nothing about whose a key is.
const SHORTEST_NAME_FOR_A_KEY: usize = 3;
const WINDOWS_VENDOR_KEY: &str = "microsoft";

impl AppKeys {
    pub(crate) fn of(manifest: &PackageManifest) -> Self {
        let app = manifest.application_metadata.as_ref();
        let of_path = |path: Option<&Path>, stem: bool| {
            path.and_then(|path| {
                if stem {
                    path.file_stem()
                } else {
                    path.file_name()
                }
            })
            .map(|name| name.to_string_lossy().into_owned())
        };
        let names = [
            app.map(|app| app.display_name.clone()),
            app.and_then(|app| app.publisher.clone()),
            of_path(app.and_then(|app| app.install_location.as_deref()), false),
            of_path(Some(&manifest.main_executable_path), true),
        ]
        .into_iter()
        .flatten()
        .map(|name| letters_of(&name))
        .filter(|name| name.len() >= SHORTEST_NAME_FOR_A_KEY)
        .collect();
        Self { names }
    }

    /// Whether a key under `Software` is the app's or its publisher's, going by the key below it.
    fn holds(&self, key: &str) -> bool {
        let Some(below) = key.strip_prefix(r"software\") else {
            return false;
        };
        let vendor = below.split('\\').next().unwrap_or_default();
        if vendor == WINDOWS_VENDOR_KEY {
            return false;
        }
        let vendor = letters_of(vendor);
        vendor.len() >= SHORTEST_NAME_FOR_A_KEY
            && self
                .names
                .iter()
                .any(|name| name.contains(&vendor) || vendor.contains(name.as_str()))
    }
}

fn letters_of(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

/// Whether a value is written somewhere other than the app's own keys and what the list already has
/// a kind for. A service's own values, and what makes the entry in Settings, are the service and the
/// entry. `Software\Classes` is what file types and COM are made of, which go by rules of their own,
/// and a key under `Software` named by an id is one no vendor's and nothing of Windows' goes by.
pub(crate) fn is_other_setting(entry: &RegistryEntry, app: &AppKeys) -> bool {
    let key = normalized_key(&entry.key_path);
    change_kind_of(entry).is_none()
        && !is_autostart_entry(entry)
        && !is_uninstall_entry_key(&entry.key_path)
        && !key.starts_with(r"system\")
        && !key.starts_with(r"software\classes\")
        && !key.starts_with(r"software\{")
        && !app.holds(&key)
}

/// Whether a key is one of Windows' own, which it may act on.
fn is_a_windows_key(entry: &RegistryEntry) -> bool {
    normalized_key(&entry.key_path).starts_with(r"software\microsoft\")
}

/// What a service's row says of when it runs. One that starts by itself is started as soon as the
/// install has checked what it installed, as well as at each boot.
fn service_start_text(start: &str) -> String {
    match start {
        "automatic" => ", starts now and each time this PC starts".to_owned(),
        crate::service_scan::DELAYED_AUTOMATIC_START_LABEL => {
            ", starts now and soon after each time this PC starts".to_owned()
        }
        "manual" => ", starts only when something asks it to".to_owned(),
        "disabled" => ", is disabled".to_owned(),
        other => format!(", starts {other}"),
    }
}

/// Which of the kinds on the list of what is set up a value is part of, if it is part of one. All of
/// a kind can be left out together.
pub(crate) fn change_kind_of(entry: &RegistryEntry) -> Option<SystemChangeKind> {
    Some(match registration_of(entry)? {
        Registration::ShellExtension { .. } => SystemChangeKind::ShellExtension,
        Registration::ShellCommand { .. } | Registration::DefaultVerb { .. } => {
            SystemChangeKind::ShellCommand
        }
        Registration::ClientEntry { .. } | Registration::DefaultAppsEntry { .. } => {
            SystemChangeKind::DefaultApps
        }
        Registration::AppIdAccess
        | Registration::ComClass { .. }
        | Registration::TypeLibrary
        | Registration::Interface
        | Registration::AppId => SystemChangeKind::ComServer,
        Registration::UrlProtocol { .. } => SystemChangeKind::UrlProtocol,
        Registration::FileType { .. } => SystemChangeKind::FileType,
        Registration::EventLogSource { .. } => SystemChangeKind::EventLogSource,
        Registration::Font { .. } => SystemChangeKind::Font,
        Registration::OdbcDriver { .. } => SystemChangeKind::OdbcDriver,
        Registration::OdbcSource { .. } => SystemChangeKind::OdbcSource,
        Registration::CompatibilitySetting => SystemChangeKind::CompatibilitySetting,
        Registration::ProgramName { .. } => SystemChangeKind::ProgramName,
    })
}

fn describe_firewall_rules(rules: &[ReviewedFirewallRule<'_>]) -> Vec<SystemChange> {
    rules
        .iter()
        .map(|rule| {
            let entry = rule.entry;
            let is_in = entry.direction.eq_ignore_ascii_case("in");
            let allows = entry.action.eq_ignore_ascii_case("allow");
            let verb = if allows { "Allows" } else { "Blocks" };
            let way = if is_in { "incoming" } else { "outgoing" };
            let ports = if entry.local_ports.is_empty() {
                "connections".to_owned()
            } else {
                format!("connections on {}", entry.local_ports.join(", "))
            };
            let profiles = if entry.profiles.is_empty() {
                "every network".to_owned()
            } else {
                format!("{} networks", entry.profiles.join(", "))
            };
            SystemChange {
                kind: SystemChangeKind::FirewallRule,
                title: entry.name.clone(),
                detail: format!(
                    "{verb} {way} {ports} for {}, on {profiles}",
                    file_name_text(&rule.program)
                ),
                high_rights: is_in && allows,
            }
        })
        .collect()
}

/// What the package registers, with no more than a line for each thing, however many values or
/// classes it takes.
fn describe_registrations(review: &ReviewedPackage<'_>) -> Vec<SystemChange> {
    let mut changes = Vec::new();
    let mut listed: BTreeSet<(String, String, String)> = BTreeSet::new();
    let mut classes_by_server: BTreeMap<String, usize> = BTreeMap::new();
    let mut support = (0_usize, 0_usize, 0_usize);
    let mut settings = 0_usize;
    let server_of = |class: Option<&str>| {
        class
            .and_then(|class| review.registrations.server_of_class(class))
            .map_or_else(|| "a file it installs".to_owned(), str::to_owned)
    };
    let everyone_or_you = |entry: &RegistryEntry| match entry.hive {
        RegistryHive::LocalMachine => "for everyone on this PC",
        RegistryHive::CurrentUser => "for you",
    };
    for entry in review.registry.iter().chain(&review.license_registry) {
        let Some(registration) = registration_of(entry) else {
            continue;
        };
        let (kind, title, detail, high_rights) = match registration {
            Registration::ShellExtension { name, class } => (
                SystemChangeKind::ShellExtension,
                name,
                format!(
                    "Loads {} into Explorer, {}",
                    server_of(class.as_deref()),
                    everyone_or_you(entry)
                ),
                true,
            ),
            Registration::ShellCommand { on, verb } => (
                SystemChangeKind::ShellCommand,
                verb,
                format!(
                    "Adds a command to the menu of {on}, {}",
                    everyone_or_you(entry)
                ),
                false,
            ),
            Registration::DefaultVerb { of } => (
                SystemChangeKind::ShellCommand,
                of,
                format!(
                    "Changes what opening one of these does, {}",
                    everyone_or_you(entry)
                ),
                true,
            ),
            Registration::ClientEntry { name } => (
                SystemChangeKind::DefaultApps,
                name,
                "Can be chosen as the program for this. Nothing is changed to it".to_owned(),
                false,
            ),
            Registration::AppIdAccess => (
                SystemChangeKind::ComServer,
                "COM permissions".to_owned(),
                "Says who may start its COM classes, or what they run as".to_owned(),
                true,
            ),
            Registration::UrlProtocol { name } => (
                SystemChangeKind::UrlProtocol,
                format!("{name}:"),
                format!(
                    "Links of this kind open with the app, {}",
                    everyone_or_you(entry)
                ),
                false,
            ),
            Registration::FileType { extension } => (
                SystemChangeKind::FileType,
                extension,
                format!(
                    "Files of this type open with the app, {}",
                    everyone_or_you(entry)
                ),
                false,
            ),
            Registration::ComClass { class } => {
                *classes_by_server
                    .entry(server_of(Some(&class)))
                    .or_default() += 1;
                continue;
            }
            Registration::TypeLibrary => {
                support.0 += 1;
                continue;
            }
            Registration::Interface => {
                support.1 += 1;
                continue;
            }
            Registration::AppId => {
                support.2 += 1;
                continue;
            }
            Registration::EventLogSource { log, source } => (
                SystemChangeKind::EventLogSource,
                source,
                format!("Can write to the {log} log, with messages from its own files"),
                false,
            ),
            Registration::Font { name } => (
                SystemChangeKind::Font,
                name,
                format!(
                    "Loads from the install folder when you sign in, {}",
                    everyone_or_you(entry)
                ),
                false,
            ),
            Registration::OdbcDriver { name } => (
                SystemChangeKind::OdbcDriver,
                name,
                "Loads into programs that use ODBC to reach a database".to_owned(),
                true,
            ),
            Registration::OdbcSource { name } => (
                SystemChangeKind::OdbcSource,
                name,
                format!("A database connection, {}", everyone_or_you(entry)),
                false,
            ),
            Registration::CompatibilitySetting => {
                settings += 1;
                continue;
            }
            Registration::DefaultAppsEntry { name } => (
                SystemChangeKind::DefaultApps,
                name,
                "Can be chosen in Default apps. Nothing is changed to it".to_owned(),
                false,
            ),
            Registration::ProgramName { name } => (
                SystemChangeKind::ProgramName,
                name,
                format!(
                    "Run and other programs can find it by this name, {}",
                    everyone_or_you(entry)
                ),
                false,
            ),
        };
        // A verb called `open` on two types is two lines, and how a name is written is not.
        if listed.insert((
            kind.label().to_owned(),
            title.to_lowercase(),
            detail.to_lowercase(),
        )) {
            changes.push(SystemChange {
                kind,
                title,
                detail,
                high_rights,
            });
        }
    }
    for (server, count) in classes_by_server {
        changes.push(SystemChange {
            kind: SystemChangeKind::ComServer,
            title: format!("COM in {server}"),
            detail: format!("{count} settings of the classes other programs can start"),
            high_rights: false,
        });
    }
    let (libraries, interfaces, app_ids) = support;
    if libraries + interfaces + app_ids > 0 {
        changes.push(SystemChange {
            kind: SystemChangeKind::ComServer,
            title: "COM type libraries, interfaces, and AppIDs".to_owned(),
            detail: format!(
                "{libraries} and {interfaces} and {app_ids} settings of each, which go with its classes"
            ),
            high_rights: false,
        });
    }
    if settings > 0 {
        changes.push(SystemChange {
            kind: SystemChangeKind::CompatibilitySetting,
            title: "How its programs start".to_owned(),
            detail: format!("{settings} compatibility settings for programs it installs"),
            high_rights: false,
        });
    }
    changes
}

/// How many names to put in a sentence. The rest are in the report in the package.
const MOST_NAMES_IN_A_NOTE: usize = 8;

fn some_names(names: Vec<String>) -> String {
    let total = names.len();
    let mut shown: Vec<String> = names.into_iter().take(MOST_NAMES_IN_A_NOTE).collect();
    if total > shown.len() {
        shown.push(format!("and {} more", total - shown.len()));
    }
    shown.join(", ")
}

fn names_of_left_out(items: &[LeftOutItem]) -> String {
    some_names(items.iter().map(|item| item.name.clone()).collect())
}

/// What the app and an installer both call what is left out, so that the two screens agree.
pub const CREDENTIALS_LEFT_OUT_TEXT: &str =
    "Not in this installer, because they look like credentials";
pub const SETTINGS_LEFT_OUT_TEXT: &str =
    "Settings and history an account keeps for the app, which only go in with the User data option";
pub const CERTIFICATES_LEFT_OUT_TEXT: &str =
    "The app seems to rely on certificates that were trusted on its PC, and they are not installed";
pub const FONTS_LEFT_OUT_TEXT: &str = "These fonts go by the app's name, and nothing says they are its own, so they are not installed";
const LEFT_OUT_BY_CHOICE_TEXT: &str = "Left out by whoever built this installer";
const ADDED_BY_CHOICE_TEXT: &str =
    "Added by whoever built this installer, because they took it to be part of the app";

/// What a person ought to know is not coming, and why. Certificates are only ever this: which to
/// trust is for them to decide.
fn notes_on_what_is_left_out(manifest: &PackageManifest) -> Vec<String> {
    let mut notes = Vec::new();
    if !manifest.secrets_left_out.is_empty() {
        notes.push(format!(
            "{CREDENTIALS_LEFT_OUT_TEXT}: {}. What is on this PC of them stays as it is, and you may \
             have to enter them again.",
            names_of_left_out(&manifest.secrets_left_out)
        ));
    }
    if !manifest.licenses_left_out.is_empty() {
        notes.push(format!(
            "Not in this installer, because the License option was off when it was built: {}.",
            names_of_left_out(&manifest.licenses_left_out)
        ));
    }
    if !manifest.settings_left_out.is_empty() {
        notes.push(format!(
            "Not in this installer, because the User data option was off when it was built: the \
             settings and history an account keeps for the app, in {}. The app starts here as it \
             would for someone new to it.",
            names_of_left_out(&manifest.settings_left_out)
        ));
    }
    if !manifest.certificates_reported.is_empty() {
        notes.push(format!(
            "{CERTIFICATES_LEFT_OUT_TEXT}: {}. Whether to trust them here is for you to decide.",
            some_names(
                manifest
                    .certificates_reported
                    .iter()
                    .map(certificate_label)
                    .collect()
            )
        ));
    }
    if !manifest.fonts_reported.is_empty() {
        notes.push(format!(
            "{FONTS_LEFT_OUT_TEXT}: {}.",
            some_names(manifest.fonts_reported.clone())
        ));
    }
    if !manifest.left_out_by_choice.is_empty() {
        notes.push(format!(
            "{LEFT_OUT_BY_CHOICE_TEXT}: {}.",
            names_of_left_out(&manifest.left_out_by_choice)
        ));
    }
    if !manifest.added_by_choice.is_empty() {
        notes.push(format!(
            "{ADDED_BY_CHOICE_TEXT}: {}.",
            some_names(manifest.added_by_choice.clone())
        ));
    }
    notes
}

pub(crate) fn certificate_label(certificate: &CertificateEntry) -> String {
    let name = if certificate.subject.is_empty() {
        &certificate.thumbprint
    } else {
        &certificate.subject
    };
    format!("{name} in {}", certificate.store)
}

fn file_name_text(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        DriverKind, EnvironmentScope, FileClassification, FirewallRuleEntry, LicenseArtifactKind,
        LicenseFramework, MatchConfidence, RegistryValueKind, ShortcutEntry, TransferableHint,
    };

    const SOURCE: &str = r"C:\Program Files\Acme";
    const TARGET: &str = r"D:\Apps\Acme";

    fn empty_manifest() -> PackageManifest {
        PackageManifest {
            schema_version: String::new(),
            source_computer_name: None,
            source_windows_version: None,
            source_platform: None,
            program_needs: Vec::new(),
            scan_timestamp_utc: String::new(),
            main_executable_path: PathBuf::from(SOURCE).join("acme.exe"),
            application_metadata: None,
            file_inventory: Vec::new(),
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
            included_license_data: true,
            user_data_profile: None,
        }
    }

    fn file(relative: &str) -> FileEntry {
        let original = PathBuf::from(SOURCE).join(relative);
        FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(PathBuf::from("files").join(relative)),
            size_bytes: Some(1),
            hash_sha256: Some("00".repeat(32)),
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    /// Reads only.
    #[test]
    fn no_way_of_writing_it_gets_past_where_records_are_kept_or_the_windows_folder() {
        let Some(state_root) = machine_state_root() else {
            return;
        };
        let state_text = state_root.display().to_string();
        let windows_text = system_root().display().to_string();
        for spelled in [
            state_text.replacen(r"\", r"\.\", 1),
            state_text.replacen(r"\", r"\\", 1),
            state_text.replace('\\', "/"),
            format!(r"{state_text}\installs\wam-abc"),
            state_text.replacen(r"\", r"\.\", 1) + r"\installs\wam-abc",
            format!(r"\\?\{state_text}\installs\wam-abc"),
            windows_text.replacen(r"\", r"\.\", 1) + r"\Acme",
            windows_text.replacen(r"\", r"\\", 1) + r"\Acme",
            format!(r"\\?\{windows_text}\Acme"),
            format!(r"{windows_text}.\Acme"),
        ] {
            assert!(
                install_location_problem(Path::new(&spelled)).is_some(),
                "{spelled}"
            );
        }
    }

    #[test]
    fn a_file_of_a_store_is_where_it_is_written_whatever_a_package_says_of_it() {
        let store = Path::new(r"C:\ProgramData\Acme\store");
        for said in [
            r"C:\Windows\x.exe",
            r"..\..\Other\x.exe",
            r"\\server\share\x.exe",
            r"\Windows\x.exe",
            r"sub\x.exe",
        ] {
            let file = LicenseStoreFile {
                relative_path: PathBuf::from(said),
                size_bytes: 1,
                hash_sha256: String::new(),
            };
            let destination = store_file_destination(store, &file);
            assert!(
                crate::path_utils::path_is_strictly_inside(&destination, store),
                "{said} came to {}",
                destination.display()
            );
        }
    }

    #[test]
    fn what_a_source_said_about_removing_it_does_not_survive_but_the_entry_does() -> Result<()> {
        let value = |name: &str| RegistryEntry {
            hive: RegistryHive::LocalMachine,
            key_path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Acme".to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some(format!(r"{SOURCE}\unins000.exe")),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: true,
        };
        let mut manifest = acme_manifest();
        manifest.registry_keys_exported = vec![
            value("DisplayName"),
            value("UninstallString"),
            value("QuietUninstallString"),
            value("ModifyPath"),
        ];
        let reviewed = review(&manifest)?;

        let kept: Vec<_> = reviewed
            .registry
            .iter()
            .filter_map(|entry| entry.value_name.as_deref())
            .collect();
        assert_eq!(kept, ["DisplayName"]);
        assert!(reviewed.refused.is_empty(), "it is replaced, not refused");
        assert_eq!(
            reviewed.uninstall_keys,
            [UninstallKey {
                hive: RegistryHive::LocalMachine,
                key_path: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Acme".to_owned(),
                has_display_name: true,
            }]
        );
        Ok(())
    }

    fn acme_manifest() -> PackageManifest {
        let mut manifest = empty_manifest();
        manifest.file_inventory = vec![file("acme.exe"), file(r"bin\svc.exe")];
        manifest
    }

    #[test]
    fn with_no_account_to_go_by_per_user_settings_are_left_out_and_said_to_be() -> Result<()> {
        let mut manifest = empty_manifest();
        manifest.registry_keys_exported = vec![
            value(RegistryHive::CurrentUser, r"Software\Acme", "Theme", "dark"),
            value(
                RegistryHive::LocalMachine,
                r"Software\Acme",
                "Theme",
                "dark",
            ),
        ];
        manifest.environment_variables_detected = vec![EnvironmentVariableEntry {
            scope: EnvironmentScope::User,
            name: "ACME_HOME".to_owned(),
            value: "x".to_owned(),
            value_kind: crate::models::RegistryValueKind::String,
            requires_admin: false,
            confidence: crate::models::MatchConfidence::High,
        }];
        let unknown = KnownFolders {
            user_account_known: false,
            user_account_note: Some("Could not tell who is signed in.".to_owned()),
            ..folders()
        };
        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            true,
            true,
            &unknown,
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        assert_eq!(reviewed.registry.len(), 1);
        assert_eq!(reviewed.registry[0].hive, RegistryHive::LocalMachine);
        assert!(reviewed.environment.is_empty());
        assert_eq!(reviewed.refused.len(), 2, "{:?}", reviewed.refused);
        assert!(
            reviewed
                .warnings
                .iter()
                .any(|warning| warning.message.contains("Could not tell who is signed in"))
        );
        Ok(())
    }

    #[test]
    fn a_note_about_whose_folders_these_are_is_only_for_a_package_with_something_per_user()
    -> Result<()> {
        let told = KnownFolders {
            user_account_note: Some("It goes to alice.".to_owned()),
            ..folders()
        };
        let review = |manifest: &PackageManifest| -> Result<Vec<String>> {
            Ok(ReviewedPackage::with_folders(
                manifest,
                Path::new(TARGET),
                true,
                true,
                &told,
                &|text| text.to_owned(),
                &|_| Ok(None),
            )?
            .warnings
            .iter()
            .map(|warning| warning.message.clone())
            .collect())
        };
        assert!(review(&empty_manifest())?.is_empty());
        let mut with_a_setting = empty_manifest();
        with_a_setting.registry_keys_exported = vec![value(
            RegistryHive::CurrentUser,
            r"Software\Acme",
            "Theme",
            "dark",
        )];
        assert_eq!(review(&with_a_setting)?, ["It goes to alice."]);
        Ok(())
    }

    #[test]
    fn what_goes_to_a_folder_this_will_not_write_to_is_said_to_go_elsewhere() -> Result<()> {
        let mut manifest = empty_manifest();
        manifest.file_inventory = vec![external_data(r"C:\Users\alice\Documents\Acme\a.txt")];
        let mut review = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            true,
            true,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        review.warnings.clear();
        review.review_user_folders(Path::new(TARGET), &|anchor| {
            assert!(matches!(
                anchor,
                UserDataAnchor::Documents | UserDataAnchor::SavedGames
            ));
            UserFolderPlace::Elsewhere(PathBuf::from(r"D:\Docs"))
        });
        assert_eq!(review.warnings.len(), 1, "only Documents has anything");
        let message = &review.warnings[0].message;
        assert!(message.contains(r"D:\Docs"), "{message}");
        assert!(message.contains(TARGET), "{message}");
        assert!(message.contains("1 files from Documents"), "{message}");
        review.warnings.clear();
        review.review_user_folders(Path::new(TARGET), &|_| {
            UserFolderPlace::Usable(PathBuf::from(r"C:\Users\alice\Documents"))
        });
        assert!(review.warnings.is_empty());
        Ok(())
    }

    #[test]
    fn who_gets_the_data_is_said_when_it_is_not_who_is_running_this() {
        let account = crate::path_utils::UserAccount {
            sid: "S-1-5-21-1".to_owned(),
            profile: PathBuf::from(r"C:\Users\alice"),
        };
        assert_eq!(
            user_account_note(&SignedInUser::Desktop(account.clone()), Some("S-1-5-21-1")),
            None
        );
        let Some(other_admin) =
            user_account_note(&SignedInUser::Desktop(account.clone()), Some("S-1-5-21-2"))
        else {
            panic!("it should say");
        };
        assert!(other_admin.contains("alice"), "{other_admin}");
        let Some(no_desktop) =
            user_account_note(&SignedInUser::NoDesktop(account), Some("S-1-5-21-1"))
        else {
            panic!("it should say");
        };
        assert!(no_desktop.contains("alice"), "{no_desktop}");
        let Some(unknown) = user_account_note(&SignedInUser::Unknown("no reason".to_owned()), None)
        else {
            panic!("it should say");
        };
        assert!(unknown.contains("no reason"), "{unknown}");
    }

    fn folders() -> KnownFolders {
        KnownFolders {
            user_desktop: Some(PathBuf::from(r"C:\Users\bob\Desktop")),
            common_desktop: Some(PathBuf::from(r"C:\Users\Public\Desktop")),
            user_start_menu: Some(PathBuf::from(
                r"C:\Users\bob\AppData\Roaming\Microsoft\Windows\Start Menu",
            )),
            common_start_menu: Some(PathBuf::from(
                r"C:\ProgramData\Microsoft\Windows\Start Menu",
            )),
            user_account_known: true,
            user_account_note: None,
        }
    }

    fn review(manifest: &PackageManifest) -> Result<ReviewedPackage<'_>> {
        ReviewedPackage::with_folders(
            manifest,
            Path::new(TARGET),
            false,
            true,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )
    }

    fn service(name: &str, binary: &str) -> ServiceEntry {
        ServiceEntry {
            service_name: name.to_owned(),
            display_name: None,
            binary_path: PathBuf::from(binary),
            arguments: None,
            parameters: Vec::new(),
            start_type: Some("automatic".to_owned()),
            account_name: Some(r"NT AUTHORITY\LocalService".to_owned()),
            dependencies: Vec::new(),
            description: None,
            requires_admin: true,
            restore_command_suggestion: String::new(),
        }
    }

    fn value(hive: RegistryHive, key: &str, name: &str, data: &str) -> RegistryEntry {
        RegistryEntry {
            hive,
            key_path: key.to_owned(),
            value_name: Some(name.to_owned()),
            value_data: Some(data.to_owned()),
            value_kind: Some(RegistryValueKind::String),
            value_bytes_hex: None,
            confidence: MatchConfidence::High,
            requires_admin: false,
        }
    }

    #[test]
    fn a_services_binary_must_be_in_the_inventory_with_no_fallback() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.services_detected = vec![
            service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe"),
            service("Evil", r"C:\Windows\System32\cmd.exe"),
        ];
        let reviewed = review(&manifest)?;

        assert_eq!(reviewed.services.len(), 1);
        assert_eq!(
            reviewed.services[0].binary,
            PathBuf::from(r"D:\Apps\Acme\bin\svc.exe")
        );
        assert_eq!(reviewed.refused.len(), 1);
        assert!(reviewed.refused[0].message.contains("Evil"));
        Ok(())
    }

    /// Naming a service already on the PC must not open its registry key. The window shows
    /// a service by its display name, so this could look like anything.
    #[test]
    fn a_service_already_elsewhere_is_refused_along_with_its_registry_keys() -> Result<()> {
        let existing = |name: &str| {
            Ok(match name {
                "Schedule" => Some(r"%SystemRoot%\system32\svchost.exe -k netsvcs".to_owned()),
                "Earlier" => Some(format!(r#""{TARGET}\bin\svc.exe" --run"#)),
                "WinSock2" => Some(String::new()),
                _ => None,
            })
        };
        let hash = "00".repeat(32);
        let mut manifest = acme_manifest();
        let mut disguised = service("Schedule", r"C:\Program Files\Acme\bin\svc.exe");
        disguised.display_name = Some("Acme Sync".to_owned());
        manifest.services_detected = vec![
            disguised,
            service("Earlier", r"C:\Program Files\Acme\bin\svc.exe"),
            service("WinSock2", r"C:\Program Files\Acme\bin\svc.exe"),
            service("New", r"C:\Program Files\Acme\bin\svc.exe"),
        ];
        manifest.drivers_detected = vec![driver("WdFilter", Some(&hash))];
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\Schedule\Parameters",
            "ServiceDll",
            r"C:\Program Files\Acme\bin\svc.exe",
        )];
        let existing_driver = |name: &str| {
            if name == "WdFilter" {
                Ok(Some(r"system32\drivers\wd\WdFilter.sys".to_owned()))
            } else {
                existing(name)
            }
        };

        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            false,
            true,
            &folders(),
            &|text| text.to_owned(),
            &existing_driver,
        )?;

        let kept: Vec<_> = reviewed
            .services
            .iter()
            .map(|service| service.entry.service_name.as_str())
            .collect();
        assert_eq!(kept, ["Earlier", "New"]);
        assert!(reviewed.drivers.is_empty());
        assert!(reviewed.registry.is_empty());
        assert_eq!(reviewed.refused.len(), 4);
        Ok(())
    }

    #[test]
    fn a_service_whose_settings_cannot_be_read_is_refused() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.services_detected = vec![service("New", r"C:\Program Files\Acme\bin\svc.exe")];
        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            false,
            true,
            &folders(),
            &|text| text.to_owned(),
            &|_| Err(std::io::Error::other("denied")),
        )?;
        assert!(reviewed.services.is_empty());
        assert_eq!(reviewed.refused.len(), 1);
        Ok(())
    }

    #[test]
    fn a_service_with_a_bad_name_or_dependency_is_refused() -> Result<()> {
        let mut bad_dependency = service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe");
        bad_dependency.dependencies = vec!["a/b".to_owned()];
        let mut manifest = acme_manifest();
        manifest.services_detected = vec![
            service(r"..\Other", r"C:\Program Files\Acme\bin\svc.exe"),
            bad_dependency,
        ];
        let reviewed = review(&manifest)?;
        assert!(reviewed.services.is_empty());
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    #[test]
    fn a_services_settings_must_be_its_own() -> Result<()> {
        let key = r"SYSTEM\CurrentControlSet\Services\AcmeSync\Parameters";
        let mut good = service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe");
        good.parameters = vec![
            value(RegistryHive::LocalMachine, key, "Port", "8080"),
            value(
                RegistryHive::LocalMachine,
                key,
                "ServiceDll",
                r"C:\Program Files\Acme\bin\svc.exe",
            ),
        ];
        let mut outside_key = service("Second", r"C:\Program Files\Acme\bin\svc.exe");
        outside_key.parameters = vec![value(
            RegistryHive::LocalMachine,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            "Evil",
            r"C:\Program Files\Acme\bin\svc.exe",
        )];
        let mut outside_dll = service("Third", r"C:\Program Files\Acme\bin\svc.exe");
        outside_dll.parameters = vec![value(
            RegistryHive::LocalMachine,
            r"SYSTEM\CurrentControlSet\Services\Third\Parameters",
            "ServiceDll",
            r"C:\Windows\System32\evil.dll",
        )];

        let mut manifest = acme_manifest();
        manifest.services_detected = vec![good, outside_key, outside_dll];
        let reviewed = review(&manifest)?;

        let kept: Vec<_> = reviewed
            .services
            .iter()
            .map(|service| service.entry.service_name.as_str())
            .collect();
        assert_eq!(kept, ["AcmeSync"]);
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    fn driver(name: &str, hash: Option<&str>) -> DriverEntry {
        DriverEntry {
            service_name: name.to_owned(),
            display_name: None,
            image_path: PathBuf::from(r"C:\Program Files\Acme\drivers\flt.sys"),
            driver_kind: DriverKind::Kernel,
            start_type: Some("manual".to_owned()),
            account_name: None,
            description: None,
            requires_admin: true,
            restore_instructions: "Reinstall it.".to_owned(),
            app_owned: true,
            signer: None,
            hash_sha256: hash.map(ToOwned::to_owned),
            package_relative_path: Some(PathBuf::from(r"drivers\flt.sys")),
        }
    }

    #[test]
    fn a_bundled_driver_needs_a_hash_and_a_valid_name() -> Result<()> {
        let hash = "00".repeat(32);
        let mut manual = driver("Vendor", None);
        manual.package_relative_path = None;
        let mut manifest = acme_manifest();
        manifest.drivers_detected = vec![
            driver("AcmeFlt", Some(&hash)),
            driver("NoHash", None),
            driver("a/b", Some(&hash)),
            manual,
        ];
        let reviewed = review(&manifest)?;

        assert_eq!(reviewed.drivers.len(), 1);
        assert_eq!(
            reviewed.drivers[0].destination,
            PathBuf::from(r"D:\Apps\Acme\drivers\flt.sys")
        );
        assert_eq!(reviewed.manual_drivers.len(), 1);
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    #[test]
    fn registry_under_a_restored_service_is_allowed_and_any_other_refused() -> Result<()> {
        let hash = "00".repeat(32);
        let mut manifest = acme_manifest();
        manifest.services_detected =
            vec![service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe")];
        manifest.drivers_detected = vec![driver("AcmeFlt", Some(&hash))];
        manifest.registry_keys_exported = vec![
            value(
                RegistryHive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AcmeSync\Parameters",
                "Port",
                "1",
            ),
            value(
                RegistryHive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AcmeFlt\Instances",
                "DefaultInstance",
                "AcmeFlt",
            ),
            value(
                RegistryHive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\WinDefend\Parameters",
                "ServiceDll",
                "evil",
            ),
            value(
                RegistryHive::LocalMachine,
                r"SYSTEM\CurrentControlSet\Services\AcmeSync",
                "ImagePath",
                r"C:\evil.exe",
            ),
        ];
        let reviewed = review(&manifest)?;
        assert_eq!(reviewed.registry.len(), 2);
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    fn task(path: &str, name: &str, xml: Option<String>) -> ScheduledTaskEntry {
        ScheduledTaskEntry {
            task_name: name.to_owned(),
            task_path: path.to_owned(),
            action_command: PathBuf::from(r"C:\Program Files\Acme\acme.exe"),
            arguments: None,
            working_directory: None,
            trigger_summary: None,
            run_level: None,
            account: Some("bob".to_owned()),
            requires_admin: false,
            restore_instructions: String::new(),
            definition_xml: xml,
        }
    }

    fn task_xml(command: &str, user: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-16"?><Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task"><Principals><Principal><UserId>{user}</UserId></Principal></Principals><Actions><Exec><Command>{command}</Command></Exec></Actions></Task>"#
        )
    }

    #[test]
    fn the_task_xml_is_what_gets_checked_not_the_manifests_description() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.scheduled_tasks_detected = vec![
            task(
                r"\",
                "Good",
                Some(task_xml(r"C:\Program Files\Acme\acme.exe", "bob")),
            ),
            // `action_command` says acme.exe, but this is what Windows would run.
            task(
                r"\",
                "Lying",
                Some(task_xml(r"C:\Windows\System32\cmd.exe", "bob")),
            ),
            task(r"\", "NoDefinition", None),
            task(r"\", "NotXml", Some("<Task>".to_owned())),
        ];
        let reviewed = review(&manifest)?;

        assert_eq!(reviewed.tasks.len(), 1);
        assert_eq!(reviewed.tasks[0].entry.task_name, "Good");
        assert_eq!(reviewed.tasks[0].commands, [r"D:\Apps\Acme\acme.exe"]);
        assert!(reviewed.tasks[0].xml.contains(r"D:\Apps\Acme\acme.exe"));
        assert!(!reviewed.tasks[0].runs_as_system);
        assert_eq!(reviewed.refused.len(), 3);
        Ok(())
    }

    #[test]
    fn a_task_that_runs_as_system_is_flagged_and_kept() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.scheduled_tasks_detected = vec![task(
            r"\Acme\",
            "Updater",
            Some(task_xml(r"C:\Program Files\Acme\acme.exe", "S-1-5-18")),
        )];
        let changes = describe(&review(&manifest)?, Path::new(TARGET));

        assert_eq!(changes.changes.len(), 1);
        assert_eq!(changes.changes[0].kind, SystemChangeKind::ScheduledTask);
        assert!(changes.changes[0].high_rights);
        assert!(changes.changes[0].detail.contains("SYSTEM"));
        Ok(())
    }

    fn shortcut(lnk: &str, target: &str) -> ShortcutEntry {
        ShortcutEntry {
            original_lnk_path: PathBuf::from(lnk),
            target_path: PathBuf::from(target),
            arguments: Some(r"--data C:\Program Files\Acme\data".to_owned()),
            working_directory: Some(PathBuf::from(SOURCE)),
            icon_path: Some(PathBuf::from(r"C:\Program Files\Acme\acme.exe,0")),
            target_is_selected_app: true,
        }
    }

    #[test]
    fn a_shortcut_lands_in_this_pcs_folder_with_its_paths_rewritten() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.shortcuts_detected = vec![shortcut(
            r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Acme\Acme.lnk",
            r"C:\Program Files\Acme\acme.exe",
        )];
        let reviewed = review(&manifest)?;

        let reviewed_shortcut = &reviewed.shortcuts[0];
        assert_eq!(
            reviewed_shortcut.placement.destination,
            PathBuf::from(
                r"C:\Users\bob\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Acme\Acme.lnk"
            )
        );
        assert_eq!(
            reviewed_shortcut.target,
            PathBuf::from(r"D:\Apps\Acme\acme.exe")
        );
        assert_eq!(
            reviewed_shortcut.arguments.as_deref(),
            Some(r"--data D:\Apps\Acme\data")
        );
        assert_eq!(reviewed_shortcut.working_directory.as_deref(), Some(TARGET));
        assert_eq!(
            reviewed_shortcut.icon.as_deref(),
            Some(r"D:\Apps\Acme\acme.exe,0")
        );
        Ok(())
    }

    #[test]
    fn a_shortcut_elsewhere_or_to_something_else_is_refused() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.shortcuts_detected = vec![
            shortcut(
                r"C:\Windows\System32\evil.lnk",
                r"C:\Program Files\Acme\acme.exe",
            ),
            shortcut(
                r"C:\Users\alice\Desktop\Evil.lnk",
                r"C:\Windows\System32\cmd.exe",
            ),
            shortcut(
                r"C:\Users\alice\Desktop\Evil.exe",
                r"C:\Program Files\Acme\acme.exe",
            ),
        ];
        let reviewed = review(&manifest)?;
        assert!(reviewed.shortcuts.is_empty());
        assert_eq!(reviewed.refused.len(), 3);
        Ok(())
    }

    #[test]
    fn a_startup_shortcut_is_surfaced_as_autostart() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.shortcuts_detected = vec![
            shortcut(
                r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\Acme.lnk",
                r"C:\Program Files\Acme\acme.exe",
            ),
            shortcut(
                r"C:\Users\Public\Desktop\Acme.lnk",
                r"C:\Program Files\Acme\acme.exe",
            ),
        ];
        let changes = describe(&review(&manifest)?, Path::new(TARGET));
        let kinds: Vec<_> = changes.changes.iter().map(|change| change.kind).collect();
        assert_eq!(
            kinds,
            [SystemChangeKind::Autostart, SystemChangeKind::Shortcut]
        );
        assert!(changes.changes[0].detail.contains("Startup"));
        assert!(changes.changes[1].detail.contains("everyone"));
        Ok(())
    }

    fn license_file(path: &str) -> LicenseArtifact {
        LicenseArtifact {
            kind: LicenseArtifactKind::File,
            framework: LicenseFramework::FileBased,
            path: Some(PathBuf::from(path)),
            registry_location: None,
            detail: String::new(),
            machine_bound: false,
            requires_admin: false,
            transferable: TransferableHint::Likely,
            should_copy: true,
            package_relative_path: Some(PathBuf::from(r"license\a.lic")),
            size_bytes: Some(1),
            hash_sha256: Some("00".repeat(32)),
            store_files: Vec::new(),
        }
    }

    #[test]
    fn license_files_only_reach_known_roots() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.license_artifacts_detected = vec![
            license_file(r"C:\Program Files\Acme\acme.lic"),
            license_file(r"C:\Windows\System32\evil.dll"),
            license_file(
                r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\x.exe",
            ),
        ];
        let reviewed = review(&manifest)?;

        assert_eq!(reviewed.license.len(), 1);
        assert_eq!(
            reviewed.license[0].destination,
            PathBuf::from(r"D:\Apps\Acme\acme.lic")
        );
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    #[test]
    fn machine_wide_license_values_are_never_written() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.license_registry_entries = vec![
            value(RegistryHive::LocalMachine, r"SOFTWARE\Acme", "Serial", "x"),
            value(RegistryHive::CurrentUser, r"Software\Acme", "Serial", "x"),
        ];
        let reviewed = review(&manifest)?;
        assert_eq!(reviewed.license_registry.len(), 1);
        assert_eq!(reviewed.license_registry[0].hive, RegistryHive::CurrentUser);
        assert!(reviewed.refused[0].message.contains("only reported"));
        Ok(())
    }

    /// As the review found it: a store rooted at a plain folder, holding what Windows or Office
    /// would run. A plain license file there was refused, and a store's file was not looked at.
    #[test]
    fn each_file_of_a_license_store_goes_by_the_rule_for_where_it_lands() -> Result<()> {
        let store_of = |relative: &str| {
            let mut store = license_file(r"C:\Users\bob\AppData\Roaming\Microsoft");
            store.store_files = vec![
                LicenseStoreFile {
                    relative_path: PathBuf::from("license.dat"),
                    size_bytes: 1,
                    hash_sha256: String::new(),
                },
                LicenseStoreFile {
                    relative_path: PathBuf::from(relative),
                    size_bytes: 1,
                    hash_sha256: String::new(),
                },
            ];
            store
        };
        for relative in [
            r"Windows\Start Menu\Programs\Startup\evil.url",
            r"Word\STARTUP\evil.dotm",
            r"Excel\XLSTART\evil.xlsm",
        ] {
            let mut manifest = acme_manifest();
            manifest.license_artifacts_detected = vec![store_of(relative)];
            let reviewed = ReviewedPackage::with_folders(
                &manifest,
                Path::new(TARGET),
                true,
                true,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )?;
            assert!(reviewed.license.is_empty(), "{relative}");
            assert_eq!(reviewed.refused.len(), 1, "{relative}");
        }
        let mut manifest = acme_manifest();
        manifest.license_artifacts_detected = vec![store_of(r"Acme\more.dat")];
        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            true,
            true,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        assert_eq!(reviewed.license.len(), 1, "{:?}", reviewed.refused);
        Ok(())
    }

    #[test]
    fn a_license_value_cannot_start_anything() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.license_registry_entries = vec![value(
            RegistryHive::CurrentUser,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "Serial",
            r"C:\Program Files\Acme\acme.exe",
        )];
        let reviewed = review(&manifest)?;
        assert!(reviewed.license_registry.is_empty());
        assert_eq!(reviewed.refused.len(), 1);
        Ok(())
    }

    #[test]
    fn a_shortcut_cannot_point_its_icon_or_folder_at_a_network_share() -> Result<()> {
        let mut manifest = acme_manifest();
        let mut remote = shortcut(
            r"C:\Users\alice\Desktop\Acme.lnk",
            r"C:\Program Files\Acme\acme.exe",
        );
        for share in [
            r"\\evil.example\share\a.ico,0",
            "//evil.example/share/a.ico,0",
            r"\/evil.example/share\a.ico,0",
            r"\\?\UNC\evil.example\share\a.ico,0",
            r"\??\UNC\evil.example\share\a.ico,0",
        ] {
            remote.icon_path = Some(PathBuf::from(share));
            remote.working_directory = Some(PathBuf::from(share));
            manifest.shortcuts_detected = vec![remote.clone()];
            let reviewed = review(&manifest)?;
            assert_eq!(reviewed.shortcuts.len(), 1);
            assert_eq!(reviewed.shortcuts[0].icon, None, "{share}");
            assert_eq!(reviewed.shortcuts[0].working_directory, None, "{share}");
        }
        Ok(())
    }

    /// Who a trigger waits for is not who a task runs as, and a package can name anyone there.
    #[test]
    fn a_task_runs_as_its_principal_and_what_a_trigger_names_says_nothing() -> Result<()> {
        let xml = format!(
            r#"<?xml version="1.0"?><Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task"><Triggers><LogonTrigger><UserId>LOWLY\guest</UserId></LogonTrigger></Triggers><Principals><Principal id="Author"><UserId>S-1-5-21-1-2-3-500</UserId><LogonType>S4U</LogonType><RunLevel>HighestAvailable</RunLevel></Principal></Principals><Actions Context="Author"><Exec><Command>{SOURCE}\acme.exe</Command></Exec></Actions></Task>"#
        );
        let mut manifest = acme_manifest();
        manifest.scheduled_tasks_detected = vec![task(r"\Acme\", "Updater", Some(xml))];
        let changes = describe(&review(&manifest)?, Path::new(TARGET));
        assert_eq!(changes.changes.len(), 1, "{:?}", changes.refused);
        let row = &changes.changes[0];
        assert!(row.detail.contains("S-1-5-21-1-2-3-500"), "{}", row.detail);
        assert!(!row.detail.contains("guest"), "{}", row.detail);
        assert!(row.detail.contains("highest rights"), "{}", row.detail);
        assert!(row.high_rights);
        Ok(())
    }

    /// A person on another PC is nobody here, and registering a task for them fails.
    #[test]
    fn a_task_that_ran_as_a_person_runs_as_whoever_installs_it() -> Result<()> {
        let xml = format!(
            r#"<?xml version="1.0"?><Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task"><Triggers><LogonTrigger><UserId>{WHOEVER_INSTALLS_IT}</UserId></LogonTrigger></Triggers><Principals><Principal id="Author"><UserId>{WHOEVER_INSTALLS_IT}</UserId><LogonType>InteractiveToken</LogonType></Principal></Principals><Actions Context="Author"><Exec><Command>{SOURCE}\acme.exe</Command></Exec></Actions></Task>"#
        );
        let mut manifest = acme_manifest();
        manifest.scheduled_tasks_detected = vec![task(r"\Acme\", "Updater", Some(xml))];
        let reviewed = review(&manifest)?;
        let Some(account) = signed_in_user().account() else {
            assert_eq!(
                reviewed.tasks.len(),
                0,
                "with no one to run as, it does not go"
            );
            assert_eq!(reviewed.refused.len(), 1);
            return Ok(());
        };
        assert_eq!(reviewed.tasks.len(), 1, "{:?}", reviewed.refused);
        assert!(!reviewed.tasks[0].xml.contains(WHOEVER_INSTALLS_IT));
        assert_eq!(
            reviewed.tasks[0].xml.matches(account.sid.as_str()).count(),
            2
        );
        let changes = describe(&reviewed, Path::new(TARGET));
        assert!(
            changes.changes[0].detail.contains(THE_PERSON_INSTALLING),
            "{:?}",
            changes.changes
        );
        assert!(!changes.changes[0].high_rights);
        Ok(())
    }

    /// With no account in what a package says of it, a shortcut still goes to whoever installs it.
    #[test]
    fn a_shortcut_said_with_no_account_goes_to_whoever_installs_it() {
        let folders = KnownFolders {
            user_desktop: Some(PathBuf::from(r"C:\Users\here\Desktop")),
            common_desktop: Some(PathBuf::from(r"C:\Users\Public\Desktop")),
            user_start_menu: Some(PathBuf::from(
                r"C:\Users\here\AppData\Roaming\Microsoft\Windows\Start Menu",
            )),
            common_start_menu: Some(PathBuf::from(
                r"C:\ProgramData\Microsoft\Windows\Start Menu",
            )),
            user_account_known: true,
            user_account_note: None,
        };
        let desktop = shortcut_placement(Path::new(r"%USERPROFILE%\Desktop\Acme.lnk"), &folders)
            .expect("it goes");
        assert_eq!(
            desktop.destination,
            Path::new(r"C:\Users\here\Desktop\Acme.lnk")
        );
        assert!(!desktop.for_all_users);
        let menu = shortcut_placement(
            Path::new(r"%USERPROFILE%\Start Menu\Programs\Acme\Acme.lnk"),
            &folders,
        )
        .expect("it goes");
        assert_eq!(
            menu.destination,
            Path::new(
                r"C:\Users\here\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Acme\Acme.lnk"
            )
        );
        assert!(!menu.for_all_users);
        assert!(!menu.autostart);
    }

    #[test]
    fn a_trigger_naming_system_does_not_make_a_task_run_as_it() -> Result<()> {
        let definition = crate::restore_policy::read_task_definition(
            r#"<Task><Triggers><LogonTrigger><UserId>S-1-5-18</UserId></LogonTrigger></Triggers><Principals><Principal><GroupId>S-1-5-32-545</GroupId></Principal></Principals><Actions><Exec><Command>a.exe</Command></Exec></Actions></Task>"#,
        )
        .map_err(WinAppMigratorError::InvalidInput)?;
        assert!(!definition.runs_as_system);
        assert_eq!(definition.account.as_deref(), Some("S-1-5-32-545"));
        assert!(!definition.highest_available);
        Ok(())
    }

    #[test]
    fn with_more_than_one_principal_it_goes_by_the_one_its_actions_name_and_the_worst_of_them()
    -> Result<()> {
        let definition = crate::restore_policy::read_task_definition(
            r#"<Task><Principals><Principal id="A"><UserId>S-1-5-18</UserId></Principal><Principal id="B"><UserId>bob</UserId></Principal></Principals><Actions Context="B"><Exec><Command>a.exe</Command></Exec></Actions></Task>"#,
        )
        .map_err(WinAppMigratorError::InvalidInput)?;
        assert_eq!(definition.account.as_deref(), Some("bob"));
        assert!(definition.runs_as_system);
        Ok(())
    }

    fn other_settings_row(manifest: &PackageManifest) -> Result<Option<SystemChange>> {
        Ok(describe(&review(manifest)?, Path::new(TARGET))
            .changes
            .into_iter()
            .find(|change| change.kind == SystemChangeKind::OtherSettings))
    }

    fn acme_app(manifest: &mut PackageManifest) {
        manifest.application_metadata = Some(crate::models::InstalledApp {
            display_name: "Acme Widget".to_owned(),
            publisher: Some("Acme Corporation".to_owned()),
            version: None,
            install_location: Some(PathBuf::from(SOURCE)),
            uninstall_string: None,
            main_executable: None,
            registry_source_path: String::new(),
            architecture: AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        });
    }

    #[test]
    fn a_value_outside_the_apps_own_keys_is_said_to_be_there() -> Result<()> {
        let mut manifest = acme_manifest();
        acme_app(&mut manifest);
        let setting = |key: &str, name: &str| value(RegistryHive::LocalMachine, key, name, "x");
        manifest.registry_keys_exported = vec![setting(r"Software\Acme\Widget", "Theme")];
        assert_eq!(other_settings_row(&manifest)?, None, "its own key");

        manifest.registry_keys_exported = vec![
            setting(r"Software\Acme\Widget", "Theme"),
            setting(r"Software\Microsoft\Whatever", "Hook"),
            setting(r"Software\Fabrikam\Thing", "Hook"),
        ];
        let row = other_settings_row(&manifest)?;
        let row = row.as_ref();
        assert!(
            row.is_some_and(|row| row.title.starts_with("2 settings") && row.high_rights),
            "{row:?}"
        );
        assert!(row.is_some_and(|row| row.detail.contains(r"HKLM\Software\Microsoft\Whatever")));
        Ok(())
    }

    /// A package says what an app is called, and Windows' own keys are not an app's by any name.
    #[test]
    fn calling_itself_microsoft_does_not_make_windows_keys_its_own() -> Result<()> {
        let mut manifest = acme_manifest();
        acme_app(&mut manifest);
        if let Some(app) = manifest.application_metadata.as_mut() {
            app.display_name = "Windows".to_owned();
            app.publisher = Some("Microsoft Corporation".to_owned());
        }
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"Software\Microsoft\Windows\CurrentVersion\Whatever",
            "Hook",
            "cmd.exe",
        )];
        assert!(other_settings_row(&manifest)?.is_some());
        Ok(())
    }

    /// A file type is made of keys under `Classes`, which have rules of their own, and a row saying
    /// so of nearly every app that has a file type would be a row nobody reads.
    #[test]
    fn what_makes_a_file_type_is_not_counted_as_anything_else() -> Result<()> {
        let mut manifest = acme_manifest();
        acme_app(&mut manifest);
        manifest.registry_keys_exported = vec![
            value(
                RegistryHive::LocalMachine,
                r"Software\Classes\.acme",
                "x",
                "y",
            ),
            value(
                RegistryHive::LocalMachine,
                r"Software\{F86AE060-B25C}",
                "x",
                "y",
            ),
        ];
        assert_eq!(other_settings_row(&manifest)?, None);
        Ok(())
    }

    #[test]
    fn only_windows_own_keys_are_marked_for_a_second_look() -> Result<()> {
        let mut manifest = acme_manifest();
        acme_app(&mut manifest);
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"Software\JavaSoft\JDK",
            "x",
            "y",
        )];
        let row = other_settings_row(&manifest)?;
        assert!(row.as_ref().is_some_and(|row| !row.high_rights), "{row:?}");
        Ok(())
    }

    #[test]
    fn what_has_a_kind_of_its_own_is_not_counted_again() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "Acme",
            &format!(r"{SOURCE}\acme.exe"),
        )];
        assert_eq!(other_settings_row(&manifest)?, None);
        Ok(())
    }

    #[test]
    fn a_tasks_account_is_read_from_its_definition_not_the_manifest() -> Result<()> {
        let mut manifest = acme_manifest();
        let hidden = task_xml(
            r"C:\Program Files\Acme\acme.exe",
            "<!-- looks empty -->S-1-5-18",
        );
        manifest.scheduled_tasks_detected = vec![task(r"\Acme\", "Updater", Some(hidden))];
        let changes = describe(&review(&manifest)?, Path::new(TARGET));
        assert!(changes.changes[0].high_rights);
        assert!(changes.changes[0].detail.contains("SYSTEM"));
        Ok(())
    }

    #[test]
    fn this_pc_refuses_its_own_windows_folder_as_an_install_location() {
        assert!(install_location_problem(system_root()).is_some());
        assert!(install_location_problem(&system_root().join("System32")).is_some());
        assert_eq!(install_location_problem(Path::new(TARGET)), None);
    }

    #[test]
    fn license_data_stays_out_of_the_review_when_it_is_not_being_restored() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.license_artifacts_detected = vec![license_file(r"C:\Windows\evil.dll")];
        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            false,
            false,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        assert!(reviewed.license.is_empty());
        assert!(reviewed.refused.is_empty());
        Ok(())
    }

    fn variable(name: &str, value: &str) -> EnvironmentVariableEntry {
        EnvironmentVariableEntry {
            scope: EnvironmentScope::Machine,
            name: name.to_owned(),
            value: value.to_owned(),
            value_kind: RegistryValueKind::String,
            requires_admin: true,
            confidence: MatchConfidence::High,
        }
    }

    #[test]
    fn environment_variables_that_steer_other_programs_are_refused() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.environment_variables_detected = vec![
            variable("ACME_HOME", SOURCE),
            variable("Path", r"C:\Windows;C:\Program Files\Acme\bin"),
            variable("COR_PROFILER_PATH", r"C:\Program Files\Acme\evil.dll"),
            variable("ComSpec", r"C:\Program Files\Acme\acme.exe"),
        ];
        let reviewed = review(&manifest)?;
        assert_eq!(reviewed.environment.len(), 2);
        assert_eq!(reviewed.refused.len(), 2);
        Ok(())
    }

    #[test]
    fn a_description_lists_what_is_set_up_beyond_files() -> Result<()> {
        let hash = "00".repeat(32);
        let mut manifest = acme_manifest();
        let mut acme_service = service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe");
        acme_service.display_name = Some("Acme Sync".to_owned());
        acme_service.account_name = Some("LocalSystem".to_owned());
        manifest.services_detected = vec![acme_service];
        manifest.drivers_detected = vec![driver("AcmeFlt", Some(&hash))];
        manifest.registry_keys_exported = vec![
            value(
                RegistryHive::LocalMachine,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                "AcmeTray",
                r#""C:\Program Files\Acme\acme.exe" --tray"#,
            ),
            value(RegistryHive::CurrentUser, r"Software\Acme", "Theme", "dark"),
            value(
                RegistryHive::CurrentUser,
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                "Evil",
                r"C:\Windows\System32\cmd.exe /c calc",
            ),
        ];
        manifest.environment_variables_detected = vec![
            variable(
                "Path",
                r"C:\Windows;C:\Program Files\Acme\bin;C:\Program Files\Acme",
            ),
            variable("ACME_HOME", SOURCE),
        ];
        let changes = describe(&review(&manifest)?, Path::new(TARGET));

        let listed: Vec<_> = changes
            .changes
            .iter()
            .map(|change| (change.kind, change.title.as_str()))
            .collect();
        assert_eq!(
            listed,
            [
                (SystemChangeKind::Service, "Acme Sync"),
                (SystemChangeKind::Driver, "AcmeFlt"),
                (SystemChangeKind::Autostart, "AcmeTray"),
                (SystemChangeKind::PathEntry, "Adds to Path"),
                (SystemChangeKind::PathEntry, "Adds to Path"),
                (SystemChangeKind::EnvironmentVariable, "ACME_HOME"),
            ]
        );
        assert!(changes.changes[0].high_rights);
        assert!(changes.changes[0].detail.contains("LocalSystem"));
        assert_eq!(changes.changes[3].detail, r"The install folder\bin");
        assert_eq!(changes.changes[4].detail, "The install folder");
        assert_eq!(changes.refused.len(), 1);
        assert!(changes.refused[0].contains("Evil"));
        Ok(())
    }

    fn every_choice() -> [(bool, bool); 4] {
        [(false, false), (false, true), (true, false), (true, true)]
    }

    /// Where a file from `C:\ProgramData` is, with what would run it if the rules let it: as the
    /// review of a hostile package found it.
    fn package_that_runs_something_from_a_data_folder(data: &str) -> PackageManifest {
        let mut manifest = acme_manifest();
        manifest.file_inventory.push(external_data(data));
        manifest.services_detected = vec![service("AcmeHidden", data)];
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "AcmeHidden",
            data,
        )];
        manifest.scheduled_tasks_detected = vec![task(
            "\\Acme\\",
            "Hidden",
            Some(format!(
                r#"<?xml version="1.0"?><Task><Principals><Principal><UserId>S-1-5-18</UserId></Principal></Principals><Actions><Exec><Command>{data}</Command></Exec></Actions></Task>"#
            )),
        )];
        manifest
    }

    fn rows_of(changes: &SystemChanges) -> Vec<(SystemChangeKind, String)> {
        let mut rows: Vec<_> = changes
            .changes
            .iter()
            .map(|change| (change.kind, change.title.clone()))
            .collect();
        rows.sort();
        rows
    }

    /// What a person is shown is worked out for no location, and an install for a real one.
    fn what_is_shown_and_what_is_done(
        manifest: &PackageManifest,
        restore_user_data: bool,
        restore_license: bool,
    ) -> Result<(SystemChanges, SystemChanges)> {
        let shown = ReviewedPackage::with_folders(
            manifest,
            Path::new(NO_TARGET_YET),
            restore_user_data,
            restore_license,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        let done = ReviewedPackage::with_folders(
            manifest,
            Path::new(TARGET),
            restore_user_data,
            restore_license,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        Ok((
            describe(&shown, Path::new(NO_TARGET_YET)),
            describe(&done, Path::new(TARGET)),
        ))
    }

    #[test]
    fn what_is_shown_is_what_is_done_for_every_choice() -> Result<()> {
        for data in [
            r"C:\ProgramData\Microsoft\Windows\Acme\evil.exe",
            r"C:\ProgramData\Acme\helper.exe",
        ] {
            let manifest = package_that_runs_something_from_a_data_folder(data);
            for (user_data, license) in every_choice() {
                let (shown, done) = what_is_shown_and_what_is_done(&manifest, user_data, license)?;
                assert_eq!(
                    rows_of(&shown),
                    rows_of(&done),
                    "{data}, user data {user_data}, license {license}"
                );
                assert_eq!(
                    shown.refused.len(),
                    done.refused.len(),
                    "{data}, user data {user_data}, license {license}: {:?} {:?}",
                    shown.refused,
                    done.refused
                );
            }
        }
        Ok(())
    }

    #[test]
    fn nothing_is_set_up_to_run_a_file_put_aside_under_the_install_location() -> Result<()> {
        let manifest = package_that_runs_something_from_a_data_folder(
            r"C:\ProgramData\Microsoft\Windows\Acme\evil.exe",
        );
        for license in [false, true] {
            let reviewed = ReviewedPackage::with_folders(
                &manifest,
                Path::new(TARGET),
                false,
                license,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )?;
            assert!(reviewed.services.is_empty(), "{:?}", reviewed.refused);
            assert!(reviewed.tasks.is_empty(), "{:?}", reviewed.refused);
            assert!(
                reviewed.registry.is_empty(),
                "{:?}",
                reviewed
                    .registry
                    .iter()
                    .map(|entry| &entry.value_name)
                    .collect::<Vec<_>>()
            );
            assert_eq!(reviewed.refused.len(), 3, "{:?}", reviewed.refused);
            // The file itself still comes, under the install location, where it runs nothing.
            assert_eq!(reviewed.files.len(), 3);
        }
        Ok(())
    }

    #[test]
    fn a_setting_that_names_a_file_put_aside_still_follows_it() -> Result<()> {
        let data = r"C:\ProgramData\Acme\settings.json";
        let mut manifest = acme_manifest();
        manifest.file_inventory.push(external_data(data));
        manifest.registry_keys_exported = vec![value(
            RegistryHive::LocalMachine,
            r"Software\Acme",
            "Settings",
            data,
        )];
        let reviewed = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            false,
            true,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        assert_eq!(reviewed.registry.len(), 1, "{:?}", reviewed.refused);
        assert_eq!(
            reviewed.map.rewrite(data),
            format!(r"{TARGET}\_external\x.exe")
        );
        Ok(())
    }

    #[test]
    fn a_program_of_a_part_outside_the_install_location_can_still_be_run() -> Result<()> {
        // Not from a data folder, so its place under the install location is simply where it goes.
        let part = r"C:\Program Files\Common Files\Acme\agent.exe";
        let mut manifest = acme_manifest();
        manifest.file_inventory.push(external_data(part));
        manifest.services_detected = vec![service("AcmeAgent", part)];
        for user_data in [false, true] {
            let reviewed = ReviewedPackage::with_folders(
                &manifest,
                Path::new(TARGET),
                user_data,
                true,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )?;
            assert_eq!(reviewed.services.len(), 1, "{:?}", reviewed.refused);
        }
        Ok(())
    }

    fn external_data(original: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(original),
            original_path: PathBuf::from(original),
            package_relative_path: Some(PathBuf::from(r"files\_external\x.exe")),
            size_bytes: Some(1),
            hash_sha256: Some("00".repeat(32)),
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    #[test]
    fn an_empty_folder_cannot_reach_a_startup_folder_any_more_than_a_file_can() -> Result<()> {
        const STARTUP: &str = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup\x";
        const ORDINARY: &str = r"C:\ProgramData\Acme\inputs";
        let empty = |path: &str| {
            let mut folder = external_data(path);
            folder.size_bytes = None;
            folder.hash_sha256 = None;
            folder
        };
        let mut manifest = acme_manifest();
        manifest.empty_folders = vec![empty(STARTUP), empty(ORDINARY)];
        let review_with = |restore_user_data| {
            ReviewedPackage::with_folders(
                &manifest,
                Path::new(TARGET),
                restore_user_data,
                true,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )
        };

        let real_locations = review_with(true)?;
        assert_eq!(real_locations.empty_folders.len(), 1);
        assert_eq!(real_locations.refused.len(), 1);
        assert!(real_locations.refused[0].message.contains("Startup"));
        assert!(
            real_locations.refused[0]
                .source
                .as_deref()
                .is_some_and(|item| item.starts_with("The folder"))
        );
        assert_eq!(
            real_locations.map.folder_destination_of(Path::new(STARTUP)),
            None
        );
        assert!(
            real_locations
                .map
                .folder_destination_of(Path::new(ORDINARY))
                .is_some()
        );
        // What makes a program is known by the files, and no file is at one of these.
        let destination = real_locations
            .map
            .folder_destination_of(Path::new(ORDINARY));
        assert!(destination.is_some_and(|path| !real_locations.map.is_restored_file(path)));
        Ok(())
    }

    #[test]
    fn a_package_from_before_these_were_kept_still_loads() -> Result<()> {
        let mut value = serde_json::to_value(acme_manifest())
            .map_err(|error| crate::errors::WinAppMigratorError::InvalidInput(error.to_string()))?;
        let fields = value.as_object_mut().ok_or_else(|| {
            crate::errors::WinAppMigratorError::InvalidInput("not an object".to_owned())
        })?;
        assert!(fields.remove("empty_folders").is_some());
        assert!(fields.remove("settings_left_out").is_some());
        let manifest: PackageManifest = serde_json::from_value(value)
            .map_err(|error| crate::errors::WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert!(manifest.empty_folders.is_empty());
        assert!(manifest.settings_left_out.is_empty());
        Ok(())
    }

    #[test]
    fn a_package_cannot_ask_for_any_number_of_empty_folders() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.empty_folders = (0..MAX_EMPTY_FOLDERS + 5)
            .map(|number| {
                let path = format!(r"C:\ProgramData\Acme\{number}");
                let mut folder = external_data(&path);
                folder.package_relative_path =
                    Some(PathBuf::from(format!(r"files\_external\{number}")));
                folder
            })
            .collect();
        let how_many_and_said = |manifest: &PackageManifest| -> Result<(usize, bool)> {
            let review = ReviewedPackage::with_folders(
                manifest,
                Path::new(TARGET),
                false,
                true,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )?;
            Ok((
                review.empty_folders.len(),
                review
                    .warnings
                    .iter()
                    .any(|warning| warning.code == "restore_too_many_empty_folders"),
            ))
        };
        assert_eq!(how_many_and_said(&manifest)?, (MAX_EMPTY_FOLDERS, true));
        // The one folder, any number of times over, gets no further.
        manifest.empty_folders = vec![manifest.empty_folders[0].clone(); MAX_EMPTY_FOLDERS + 5];
        assert_eq!(how_many_and_said(&manifest)?, (MAX_EMPTY_FOLDERS, true));
        Ok(())
    }

    #[test]
    fn user_data_cannot_reach_a_startup_folder_but_a_normal_folder_is_fine() -> Result<()> {
        const STARTUP: &str = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup\x.exe";
        let mut manifest = acme_manifest();
        manifest.file_inventory.push(external_data(STARTUP));
        manifest
            .file_inventory
            .push(external_data(r"C:\ProgramData\Acme\settings.json"));
        let review_with = |restore_user_data| {
            ReviewedPackage::with_folders(
                &manifest,
                Path::new(TARGET),
                restore_user_data,
                true,
                &folders(),
                &|text| text.to_owned(),
                &|_| Ok(None),
            )
        };

        let real_locations = review_with(true)?;
        assert_eq!(real_locations.files.len(), 3);
        assert_eq!(real_locations.refused.len(), 1);
        assert!(real_locations.refused[0].message.contains("Startup"));
        assert_eq!(real_locations.map.destination_of(Path::new(STARTUP)), None);
        assert!(
            real_locations
                .map
                .destination_of(Path::new(r"C:\ProgramData\Acme\settings.json"))
                .is_some()
        );

        // Under the install location the same file is only data.
        let under_target = review_with(false)?;
        assert_eq!(under_target.files.len(), 4);
        assert!(under_target.refused.is_empty());
        Ok(())
    }

    #[test]
    fn a_package_naming_a_device_is_refused_whole() {
        let mut manifest = acme_manifest();
        let mut device = file("acme.exe");
        device.package_relative_path = Some(PathBuf::from(r"files\CON"));
        manifest.file_inventory.push(device);
        assert!(review(&manifest).is_err());
    }

    #[test]
    fn describing_a_package_needs_no_install_location() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.services_detected =
            vec![service("AcmeSync", r"C:\Program Files\Acme\bin\svc.exe")];
        let changes = system_changes(&manifest)?;
        assert_eq!(changes.changes.len(), 1);
        Ok(())
    }

    // ---- What goes into how Windows finds and starts things -------------------------------

    const ACME_CLASS: &str = "{5a7e6e0c-e2e0-4000-8000-000000000001}";
    const MACHINE: RegistryHive = RegistryHive::LocalMachine;

    fn with_integrations() -> PackageManifest {
        let mut manifest = acme_manifest();
        manifest.file_inventory.push(file(r"bin\shell.dll"));
        let server = format!(r"{SOURCE}\bin\shell.dll");
        let exe = format!(r"{SOURCE}\acme.exe");
        manifest.registry_keys_exported = vec![
            value(
                MACHINE,
                &format!(r"Software\Classes\CLSID\{ACME_CLASS}\InprocServer32"),
                "(Default)",
                &server,
            ),
            value(
                MACHINE,
                r"Software\Classes\*\shellex\ContextMenuHandlers\Acme",
                "(Default)",
                ACME_CLASS,
            ),
            value(
                MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved",
                ACME_CLASS,
                "Acme menu",
            ),
            value(MACHINE, r"Software\Classes\acme", "URL Protocol", ""),
            value(
                MACHINE,
                r"Software\Classes\acme\shell\open\command",
                "(Default)",
                &format!("\"{exe}\" \"%1\""),
            ),
            value(MACHINE, r"Software\Classes\.acme", "(Default)", "Acme.File"),
            value(
                MACHINE,
                r"SYSTEM\CurrentControlSet\Services\EventLog\Application\Acme",
                "EventMessageFile",
                &server,
            ),
            value(
                MACHINE,
                r"Software\Microsoft\Windows NT\CurrentVersion\Fonts",
                "Acme (TrueType)",
                &server,
            ),
            value(
                MACHINE,
                r"Software\ODBC\ODBCINST.INI\Acme Driver",
                "Driver",
                &server,
            ),
            value(
                MACHINE,
                r"Software\ODBC\ODBC.INI\Acme DB",
                "Driver",
                &server,
            ),
            value(
                MACHINE,
                r"Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
                &exe,
                "~ RUNASADMIN",
            ),
            value(
                MACHINE,
                r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths\acme.exe",
                "(Default)",
                &exe,
            ),
        ];
        manifest.firewall_rules_detected = vec![FirewallRuleEntry {
            name: "Acme Web".to_owned(),
            description: None,
            direction: "In".to_owned(),
            action: "Allow".to_owned(),
            enabled: true,
            program: PathBuf::from(exe),
            service: None,
            protocol: Some("6".to_owned()),
            local_ports: vec!["80".to_owned()],
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: Vec::new(),
            profiles: Vec::new(),
            not_carried: Vec::new(),
        }];
        manifest
    }

    #[test]
    fn everything_a_package_registers_is_in_what_a_person_is_asked_about() -> Result<()> {
        let manifest = with_integrations();
        let reviewed = review(&manifest)?;
        assert!(reviewed.refused.is_empty(), "{:?}", reviewed.refused);
        let changes = describe(&reviewed, Path::new(TARGET)).changes;
        let kinds: Vec<SystemChangeKind> = changes.iter().map(|change| change.kind).collect();
        for wanted in [
            SystemChangeKind::ShellExtension,
            SystemChangeKind::UrlProtocol,
            SystemChangeKind::FileType,
            SystemChangeKind::ComServer,
            SystemChangeKind::FirewallRule,
            SystemChangeKind::EventLogSource,
            SystemChangeKind::Font,
            SystemChangeKind::OdbcDriver,
            SystemChangeKind::OdbcSource,
            SystemChangeKind::CompatibilitySetting,
            SystemChangeKind::ProgramName,
        ] {
            assert!(kinds.contains(&wanted), "{wanted:?} in {changes:?}");
        }
        let extension = changes
            .iter()
            .find(|change| change.kind == SystemChangeKind::ShellExtension);
        assert!(
            extension
                .is_some_and(|change| change.high_rights && change.detail.contains("shell.dll")),
            "{extension:?}"
        );
        let rule = changes
            .iter()
            .find(|change| change.kind == SystemChangeKind::FirewallRule);
        assert!(
            rule.is_some_and(|change| change.high_rights && change.detail.contains("80")),
            "{rule:?}"
        );
        Ok(())
    }

    #[test]
    fn a_person_is_also_asked_about_what_opening_a_type_does_and_who_may_start_a_class()
    -> Result<()> {
        let mut manifest = with_integrations();
        let exe = PathBuf::from(SOURCE).join("acme.exe").display().to_string();
        let command = format!("\"{exe}\" \"%1\"");
        manifest.registry_keys_exported.extend([
            value(
                MACHINE,
                r"Software\Classes\Acme.File\shell",
                "(Default)",
                "edit",
            ),
            value(
                MACHINE,
                r"Software\Classes\Acme.File\shell\edit\command",
                "(Default)",
                &command,
            ),
            value(
                MACHINE,
                r"Software\Classes\Acme.File\shell\open\command",
                "(Default)",
                &command,
            ),
            value(
                MACHINE,
                r"Software\Classes\Other.File\shell\acme",
                "ExplorerCommandHandler",
                ACME_CLASS,
            ),
            value(
                MACHINE,
                r"Software\Clients\Media\Acme\shell\open\command",
                "(Default)",
                &command,
            ),
            value(
                MACHINE,
                &format!(r"Software\Classes\AppID\{ACME_CLASS}"),
                "LaunchPermission",
                "x",
            ),
        ]);
        let reviewed = review(&manifest)?;
        assert!(reviewed.refused.is_empty(), "{:?}", reviewed.refused);
        let changes = describe(&reviewed, Path::new(TARGET)).changes;
        let row = |kind: SystemChangeKind, title: &str| {
            changes
                .iter()
                .find(|change| change.kind == kind && change.title == title)
        };
        assert!(
            row(SystemChangeKind::ShellCommand, "Acme.File")
                .is_some_and(|change| change.high_rights),
            "as it is written, and that it changes what opening one does: {changes:?}"
        );
        assert!(
            row(SystemChangeKind::ShellCommand, "acme")
                .is_some_and(|change| change.detail.contains("Other.File")),
            "a verb only a class handles is a verb: {changes:?}"
        );
        assert!(
            row(SystemChangeKind::DefaultApps, "Media: Acme").is_some(),
            "{changes:?}"
        );
        assert!(
            row(SystemChangeKind::ComServer, "COM permissions")
                .is_some_and(|change| change.high_rights),
            "{changes:?}"
        );
        let open_verbs = changes
            .iter()
            .filter(|change| {
                change.kind == SystemChangeKind::ShellCommand && change.title == "open"
            })
            .count();
        assert_eq!(open_verbs, 2, "one on each type: {changes:?}");
        Ok(())
    }

    #[test]
    fn the_part_of_something_that_cannot_be_had_takes_all_of_it_and_what_names_it() -> Result<()> {
        let mut manifest = with_integrations();
        // Its server is somewhere else, so all of this class goes, and the extension that loads it.
        for entry in &mut manifest.registry_keys_exported {
            if entry.key_path.ends_with("InprocServer32") {
                entry.value_data = Some(r"\\server\share\evil.dll".to_owned());
            }
        }
        let reviewed = review(&manifest)?;
        let kept: Vec<&str> = reviewed
            .registry
            .iter()
            .map(|entry| entry.key_path.as_str())
            .collect();
        assert!(!kept.iter().any(|key| key.contains("CLSID")), "{kept:?}");
        assert!(!kept.iter().any(|key| key.contains("shellex")), "{kept:?}");
        assert!(!kept.iter().any(|key| key.contains("Approved")), "{kept:?}");
        assert!(
            kept.iter().any(|key| key.contains(r"Classes\acme")),
            "{kept:?}"
        );
        assert_eq!(reviewed.refused.len(), 3, "{:?}", reviewed.refused);
        Ok(())
    }

    #[test]
    fn a_license_value_cannot_get_past_what_any_other_value_goes_by() -> Result<()> {
        let mut manifest = acme_manifest();
        manifest.license_registry_entries = vec![value(
            RegistryHive::CurrentUser,
            r"Software\Classes\*\shellex\ContextMenuHandlers\Evil",
            "(Default)",
            "{5a7e6e0c-e2e0-4000-8000-00000000dead}",
        )];
        let reviewed = review(&manifest)?;
        assert!(reviewed.license_registry.is_empty());
        assert_eq!(reviewed.refused.len(), 1, "{:?}", reviewed.refused);
        Ok(())
    }

    #[test]
    fn a_firewall_rule_that_cannot_be_had_is_said_and_others_go_ahead() -> Result<()> {
        let mut manifest = with_integrations();
        let good = manifest.firewall_rules_detected[0].clone();
        manifest.firewall_rules_detected.push(FirewallRuleEntry {
            name: "Every program".to_owned(),
            program: PathBuf::new(),
            ..good.clone()
        });
        manifest.firewall_rules_detected.push(FirewallRuleEntry {
            name: "Not installed".to_owned(),
            program: PathBuf::from(r"C:\Windows\System32\svchost.exe"),
            ..good
        });
        let reviewed = review(&manifest)?;
        assert_eq!(reviewed.firewall_rules.len(), 1);
        assert_eq!(
            reviewed.firewall_rules[0].program,
            PathBuf::from(r"D:\Apps\Acme\acme.exe")
        );
        let refused: Vec<&str> = reviewed
            .refused
            .iter()
            .map(|item| item.message.as_str())
            .collect();
        assert_eq!(refused.len(), 2, "{refused:?}");
        assert!(refused.iter().any(|item| item.contains("Every program")));
        assert!(refused.iter().any(|item| item.contains("Not installed")));
        Ok(())
    }

    #[test]
    fn a_person_is_told_what_was_left_out_by_name_and_never_what_it_held() {
        let mut manifest = empty_manifest();
        manifest.secrets_left_out = vec![LeftOutItem {
            name: r"HKLM\Software\Acme\ApiKey".to_owned(),
            why: "it looks like a credential".to_owned(),
        }];
        manifest.licenses_left_out = vec![LeftOutItem {
            name: r"C:\Program Files\Acme\license.lic".to_owned(),
            why: "the License option is off".to_owned(),
        }];
        manifest.certificates_reported = vec![CertificateEntry {
            store: "ROOT".to_owned(),
            hive: RegistryHive::LocalMachine,
            thumbprint: "ABC".to_owned(),
            subject: "CN=Acme Root".to_owned(),
            why: "named for it".to_owned(),
        }];
        manifest.fonts_reported = vec!["Acme Icons (TrueType)".to_owned()];
        let notes = notes_on_what_is_left_out(&manifest);
        assert_eq!(notes.len(), 4, "{notes:?}");
        assert!(notes[0].contains("ApiKey"), "{}", notes[0]);
        assert!(notes[1].contains("license.lic"), "{}", notes[1]);
        assert!(notes[2].contains("CN=Acme Root in ROOT"), "{}", notes[2]);
        assert!(notes[3].contains("Acme Icons"), "{}", notes[3]);
    }

    #[test]
    fn a_long_list_of_names_says_how_many_more_there_are() {
        let names: Vec<String> = (0..MOST_NAMES_IN_A_NOTE + 3)
            .map(|number| number.to_string())
            .collect();
        assert!(some_names(names).ends_with("and 3 more"));
        assert_eq!(some_names(vec!["a".to_owned(), "b".to_owned()]), "a, b");
    }

    #[test]
    fn with_no_license_to_be_restored_the_variable_that_says_where_it_is_stays_out() -> Result<()> {
        let mut manifest = empty_manifest();
        manifest.environment_variables_detected = vec![EnvironmentVariableEntry {
            scope: EnvironmentScope::Machine,
            name: "ACME_LICENSE_FILE".to_owned(),
            value: format!(r"{SOURCE}\license.lic"),
            value_kind: crate::models::RegistryValueKind::String,
            requires_admin: true,
            confidence: crate::models::MatchConfidence::High,
        }];
        let with_it = ReviewedPackage::with_folders(
            &manifest,
            Path::new(TARGET),
            false,
            false,
            &folders(),
            &|text| text.to_owned(),
            &|_| Ok(None),
        )?;
        assert!(with_it.environment.is_empty());
        assert_eq!(review(&manifest)?.environment.len(), 1);
        Ok(())
    }
}
