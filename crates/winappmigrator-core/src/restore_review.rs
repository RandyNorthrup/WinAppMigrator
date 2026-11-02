//! Applies the rules in [`crate::restore_policy`] to a whole package.
//!
//! Restoring and verifying work from the one result, so verification expects exactly what
//! was allowed. The installer's window shows the same rules before an install location has
//! been chosen, so a few checks that depend on what is already at that location can only
//! happen once one has: a task or COM class that belongs to something else is refused then.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::errors::Result;
use crate::file_discovery::{
    expand_env_vars, has_reserved_device_name, known_base_dirs, split_user_data_relative,
    user_data_base, user_shell_folder,
};
use crate::models::{
    DriverEntry, EnvironmentVariableEntry, FileEntry, LicenseArtifact, PackageManifest,
    RegistryEntry, RegistryHive, ScheduledTaskEntry, ServiceEntry, Warning, WarningSeverity,
};
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{strip_prefix_ignore_case, system_root};
use crate::registry_target::{app_path_entries, is_path_variable, service_image_path};
use crate::restore_policy::{
    KnownFolders, SERVICES_PREFIX, ServiceAccount, ShortcutPlacement, driver_destination,
    environment_refusal, existing_service_refusal, install_location_refusal, is_autostart_entry,
    is_source_uninstall_value, is_uninstall_entry_key, is_valid_service_name, license_destination,
    read_task_definition, registry_refusal, shortcut_placement, task_refusal,
};
use crate::system_context::SystemContext;

const DESKTOP_SHELL_FOLDER: &str = "Desktop";
const DESKTOP_FOLDER: &str = "Desktop";
const START_MENU_SUBFOLDER: &str = r"Microsoft\Windows\Start Menu";
const PARAMETERS_SUBKEY: &str = "parameters";
const REFUSED_CODE: &str = "restore_refused";
const DISPLAY_NAME_VALUE: &str = "DisplayName";
const UNC_PREFIX: &str = r"\\";
/// A stand-in target, for describing a package before an install location is chosen.
const NO_TARGET_YET: &str = r"C:\WinAppMigrator install location";

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
    pub runs_as_system: bool,
    /// The account its definition names, which is what it will run as.
    pub account: Option<String>,
}

/// A shortcut that is allowed, with its paths rewritten for the target.
pub(crate) struct ReviewedShortcut {
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

/// What a package may do on this PC, and what it was refused.
pub(crate) struct ReviewedPackage<'a> {
    pub map: RestoreMap,
    pub files: Vec<&'a FileEntry>,
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
    /// What the package asked for and will not be given.
    pub refused: Vec<Warning>,
    /// Problems with the package short of a refusal.
    pub warnings: Vec<Warning>,
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
            refused: Vec::new(),
            warnings: Vec::new(),
            map,
        };
        review.review_files(manifest);
        review.review_services(manifest, expand, existing_service);
        review.review_drivers(manifest, existing_service);
        let restored_services = review.restored_service_names();
        review.review_registry(manifest, &restored_services, expand);
        review.review_tasks(manifest, expand);
        review.review_shortcuts(manifest, folders);
        review.review_environment(manifest);
        if restore_license {
            review.review_license(manifest, restore_user_data, &restored_services, expand);
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
            let Some(binary) = self.map.destination_of(&service.binary_path) else {
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

    fn review_registry(
        &mut self,
        manifest: &'a PackageManifest,
        restored_services: &HashSet<String>,
        expand: &dyn Fn(&str) -> String,
    ) {
        for entry in &manifest.registry_keys_exported {
            match registry_refusal(entry, &self.map, restored_services, expand) {
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
            match license_destination(original, &self.map, restore_user_data, &real_location) {
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
            match registry_refusal(entry, &self.map, restored_services, expand) {
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
            let (xml, definition) = match rewritten {
                Ok(parsed) => parsed,
                Err(error) => {
                    self.refuse(item, format!("its definition could not be read: {error}"));
                    continue;
                }
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
                runs_as_system: definition.runs_as_system,
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
            let Some(target) = self.map.destination_of(&shortcut.target_path) else {
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
                    .filter(|path| !path.starts_with(UNC_PREFIX))
            };
            let working_directory = shortcut.working_directory.as_ref().and_then(local_path);
            let icon = shortcut.icon_path.as_ref().and_then(local_path);
            let arguments = shortcut
                .arguments
                .as_deref()
                .map(|text| self.map.rewrite(text));
            self.shortcuts.push(ReviewedShortcut {
                placement,
                target,
                arguments,
                working_directory,
                icon,
            });
        }
    }

    fn review_environment(&mut self, manifest: &'a PackageManifest) {
        for variable in &manifest.environment_variables_detected {
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

fn registry_item(entry: &RegistryEntry) -> String {
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

/// The Desktop and Start Menu folders of whoever is running the installer, and the ones
/// shared by everyone on the PC.
fn current_known_folders() -> KnownFolders {
    let profile = std::env::var_os("USERPROFILE").map(PathBuf::from);
    let app_data = std::env::var_os("APPDATA").map(PathBuf::from);
    let program_data = user_data_base(crate::file_discovery::UserDataAnchor::ProgramData);
    let public = user_data_base(crate::file_discovery::UserDataAnchor::PublicProfile);
    KnownFolders {
        user_desktop: user_shell_folder(DESKTOP_SHELL_FOLDER)
            .or_else(|| profile.map(|profile| profile.join(DESKTOP_FOLDER))),
        common_desktop: public.map(|public| public.join(DESKTOP_FOLDER)),
        user_start_menu: app_data.map(|base| base.join(START_MENU_SUBFOLDER)),
        common_start_menu: program_data.map(|base| base.join(START_MENU_SUBFOLDER)),
    }
}

/// What kind of change a package makes beyond its files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemChangeKind {
    Service,
    Driver,
    ScheduledTask,
    /// Runs by itself at sign-in: a Run value, Active Setup, or a Startup shortcut.
    Autostart,
    Shortcut,
    PathEntry,
    EnvironmentVariable,
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
}

/// Describe what a package would do, for showing before anything is installed. Whether an
/// item is allowed does not depend on where the app is installed, so this is right for any
/// install location chosen later. `Err` means the package would be refused whole.
pub fn system_changes(manifest: &PackageManifest) -> Result<SystemChanges> {
    let target = PathBuf::from(NO_TARGET_YET);
    let review = ReviewedPackage::with_folders(
        manifest,
        &target,
        true,
        manifest.included_license_data,
        &current_known_folders(),
        &expand_env_vars,
        // Whether an existing service may be replaced depends on where it is being installed.
        &|_| Ok(None),
    )?;
    Ok(describe(&review, &target))
}

/// Why this install location must not be used on this PC, if so.
pub fn install_location_problem(target_root: &Path) -> Option<String> {
    install_location_refusal(
        target_root,
        system_root(),
        &known_base_dirs(&SystemContext::live()),
    )
}

fn describe(review: &ReviewedPackage<'_>, target_root: &Path) -> SystemChanges {
    let mut changes = Vec::new();
    for service in &review.services {
        let starts = service
            .entry
            .start_type
            .as_deref()
            .map(|start| format!(", starts {start}"))
            .unwrap_or_default();
        let runs_as = service.account.label();
        changes.push(SystemChange {
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
        });
    }
    for driver in &review.drivers {
        let starts = driver
            .entry
            .start_type
            .as_deref()
            .map(|start| format!(", starts {start}"))
            .unwrap_or_default();
        changes.push(SystemChange {
            kind: SystemChangeKind::Driver,
            title: driver.entry.service_name.clone(),
            detail: format!(
                "Loads {} into Windows itself{starts}",
                file_name_text(&driver.destination)
            ),
            high_rights: true,
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
        changes.push(SystemChange {
            kind: SystemChangeKind::ScheduledTask,
            title: format!("{}{}", task.entry.task_path, task.entry.task_name),
            detail: format!("Runs as {runs_as}"),
            high_rights: task.runs_as_system,
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
        changes.push(SystemChange {
            kind: SystemChangeKind::Autostart,
            title: entry
                .value_name
                .clone()
                .unwrap_or_else(|| entry.key_path.clone()),
            detail: format!("Runs {scope}"),
            high_rights: false,
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
        if shortcut.placement.autostart {
            changes.push(SystemChange {
                kind: SystemChangeKind::Autostart,
                title: name,
                detail: format!("A shortcut in the Startup folder, {place}"),
                high_rights: false,
            });
        } else {
            changes.push(SystemChange {
                kind: SystemChangeKind::Shortcut,
                title: name,
                detail: format!("Opens {}, {place}", file_name_text(&shortcut.target)),
                high_rights: false,
            });
        }
    }
    for variable in &review.environment {
        if is_path_variable(&variable.name) {
            let rewritten = review.map.rewrite(&variable.value);
            for entry in app_path_entries(&rewritten, target_root) {
                changes.push(SystemChange {
                    kind: SystemChangeKind::PathEntry,
                    title: format!("Adds to {}", variable.name),
                    detail: strip_prefix_ignore_case(Path::new(&entry), target_root).map_or_else(
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
                });
            }
        } else {
            changes.push(SystemChange {
                kind: SystemChangeKind::EnvironmentVariable,
                title: variable.name.clone(),
                detail: format!("Set to {}", variable.value),
                high_rights: false,
            });
        }
    }
    SystemChanges {
        changes,
        refused: review
            .refused
            .iter()
            .map(|warning| warning.message.clone())
            .collect(),
    }
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
        DriverKind, EnvironmentScope, FileClassification, LicenseArtifactKind, LicenseFramework,
        MatchConfidence, RegistryValueKind, ShortcutEntry, TransferableHint,
    };

    const SOURCE: &str = r"C:\Program Files\Acme";
    const TARGET: &str = r"D:\Apps\Acme";

    fn empty_manifest() -> PackageManifest {
        PackageManifest {
            schema_version: String::new(),
            source_computer_name: None,
            source_windows_version: None,
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
            warnings: Vec::new(),
            items_intentionally_excluded: Vec::new(),
            included_user_data: false,
            included_program_data: false,
            included_license_data: true,
            ui_theme: None,
        }
    }

    fn file(relative: &str) -> FileEntry {
        let original = PathBuf::from(SOURCE).join(relative);
        FileEntry {
            path: original.clone(),
            original_path: original,
            package_relative_path: Some(PathBuf::from("files").join(relative)),
            suggested_destination_path: None,
            size_bytes: Some(1),
            hash_sha256: Some("00".repeat(32)),
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
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
        remote.icon_path = Some(PathBuf::from(r"\\evil.example\share\a.ico,0"));
        remote.working_directory = Some(PathBuf::from(r"\\evil.example\share"));
        manifest.shortcuts_detected = vec![remote];
        let reviewed = review(&manifest)?;
        assert_eq!(reviewed.shortcuts.len(), 1);
        assert_eq!(reviewed.shortcuts[0].icon, None);
        assert_eq!(reviewed.shortcuts[0].working_directory, None);
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

    fn external_data(original: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(original),
            original_path: PathBuf::from(original),
            package_relative_path: Some(PathBuf::from(r"files\_external\x.exe")),
            suggested_destination_path: None,
            size_bytes: Some(1),
            hash_sha256: Some("00".repeat(32)),
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
        }
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
}
