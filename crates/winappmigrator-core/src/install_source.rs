//! Authoritative install-source reverse engineering.
//!
//! Rather than guessing an application's footprint by scanning folders, this
//! module first determines *how* the application was installed and then, where
//! Windows records it, reads the authoritative list of installed paths.
//!
//! For Windows Installer (MSI) products the installer keeps a component
//! database under
//! `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\<SID>\Components`.
//! Each component subkey is named with a "squished" (packed) component GUID and
//! holds one value per consuming product (also a squished GUID) whose data is
//! the component's *key path* — the concrete file or registry key the installer
//! placed on disk. Reading those values for a product code yields the exact set
//! of files an MSI installed, which we collapse into a minimal set of install
//! roots.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::models::{InstallMethod, InstallScope, InstalledApp};
use crate::path_utils::normalize_name_for_match;
use crate::path_utils::{
    comparable_path, is_auxiliary_executable, is_drive_root, looks_like_executable, path_is_within,
    split_command_executable_path, walk_asking,
};
use crate::privilege::only_administrators_can_change;
use crate::product_match::{MatchScope, ProductNames};
use crate::registry_scan::{RecordedRegistryKey, RecordedRegistryRoot, registry_value_text};
use crate::shortcut_scan::{lnk_target_path, read_shortcut_file};
use crate::system_context::SystemContext;

/// Folders a person keeps things in, which being above a program does not make the app's.
const WHERE_A_PERSON_PUTS_THINGS: &[&str] = &[
    "desktop",
    "documents",
    "downloads",
    "my documents",
    "temp",
    "tmp",
];
const USERDATA_PATH: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData";
const APP_PATHS_PREFIX: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths";
const SIGNATURE_SCAN_BYTES: u64 = 2_000_000;
const SQUIRREL_APP_PREFIX: &str = "app-";
const APPX_MANIFEST: &str = "AppxManifest.xml";
const MSIX_PACKAGES_DIR: &str = "Packages";
const START_MENU_PROGRAMS: &str = r"Microsoft\Windows\Start Menu\Programs";
const USER_START_MENU_PROGRAMS: &str = r"AppData\Roaming\Microsoft\Windows\Start Menu\Programs";
const MAX_START_MENU_DEPTH: usize = 5;
const MAX_EXECUTABLE_SEARCH_DEPTH: usize = 5;
/// Installers keep a copy of themselves in these folders. The copy can be what the
/// uninstall entry points at, but it is not the application.
const INSTALLER_CACHE_MARKERS: &[&str] = &[
    r"\package cache\",
    r"\caphyon\",
    r"\installshield installation information\",
];
const WINDOWS_APPS_MARKER: &str = r"\windowsapps\";
/// Where a Burn bundle's packages record that the bundle depends on them.
const MSI_DEPENDENCIES_PATH: &str = r"Software\Classes\Installer\Dependencies";
/// Runtimes a bundle can chain for its app. They are shared, so they are prerequisites
/// to report and not part of the app, unless the app is that runtime.
const SHARED_RUNTIME_MARKERS: &[&str] = &[".net", "visual c++", "webview2"];
/// Folder names that hold an app's executables and little else, so the app's own folder
/// is the one above.
const EXECUTABLE_FOLDER_NAMES: &[&str] = &[
    "bin",
    "bin64",
    "x64",
    "x86",
    "win32",
    "win64",
    "amd64",
    "cmd",
    "exe",
    "app",
    "application",
    "program",
];

/// Far more than the largest real one, which was tens of megabytes, on a VS Code with years of
/// updates. On a drive from elsewhere, one could be made big enough to run this out of memory.
const MAX_INSTALL_LOG_BYTES: u64 = 128 * 1024 * 1024;
const MAX_APPX_MANIFEST_BYTES: u64 = 16 * 1024 * 1024;

/// Whether what an install log says can be taken at its word. It names files and registry values
/// to package, and it is a file in the app's own folder, which any account may be able to write to
/// if that folder is somewhere like `C:\Tools`. So for a machine-wide app, only administrators can
/// be able to change it. An app of one account's is held to that account's own things anyway, and
/// on a drive none of what it says can lead off the drive.
fn record_can_be_believed(
    app: &InstalledApp,
    log: &Path,
    system: &SystemContext,
    evidence: &mut Vec<String>,
) -> bool {
    let believed = system.is_offline()
        || app.install_scope == InstallScope::PerUser
        || only_administrators_can_change(log);
    if !believed {
        evidence.push(format!(
            "{} was not used: others than administrators can change it",
            log.display()
        ));
    }
    believed
}

/// A file read whole, if it is of a size that makes sense for it.
fn read_within(path: &Path, limit: u64, system: &SystemContext) -> Option<Vec<u8>> {
    if !system.may_read(path) {
        return None;
    }
    let size = std::fs::metadata(path).ok()?.len();
    (size <= limit).then(|| std::fs::read(path).ok()).flatten()
}

/// The result of reverse-engineering how and where an application was installed.
#[derive(Debug, Clone)]
pub struct InstallSource {
    pub method: InstallMethod,
    pub product_code: Option<String>,
    pub authoritative_file_count: usize,
    /// Install directories derived from authoritative or registered sources.
    pub roots: Vec<PathBuf>,
    /// Recorded files with no folder of the app's own to walk, such as one directly under
    /// `Common Files` or on the drive itself. Captured one by one.
    pub loose_files: Vec<PathBuf>,
    /// Every file the install record names, kept so the main executable can be chosen
    /// from them without reading the record again.
    pub recorded_files: Vec<PathBuf>,
    /// Registry keys and values the install record names.
    pub registry_keys: Vec<RecordedRegistryKey>,
    /// Human-readable signals used to reverse-engineer the install.
    pub evidence: Vec<String>,
    /// The folder its uninstaller is in, when that is one of `roots`, and all that says it is.
    pub uninstaller_folder: Option<PathBuf>,
    /// Folders the app's App Paths entry says to look in that are not the app's, which it may need.
    pub folders_left_alone: Vec<PathBuf>,
}

/// Authoritative key paths recorded for a product in the MSI component database.
struct MsiComponentPaths {
    files: Vec<PathBuf>,
    registry_keys: Vec<RecordedRegistryKey>,
}

pub fn detect_install_source(app: &InstalledApp, system: &SystemContext) -> InstallSource {
    detect_install_source_unless(app, system, &|| false)
}

/// [`detect_install_source`], stopping where it is once `is_cancelled` says so, in which case what it
/// found so far is of no use to anyone. The MSI database can take several seconds to go through.
pub(crate) fn detect_install_source_unless(
    app: &InstalledApp,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> InstallSource {
    let product_code = extract_product_code(app);
    let method = classify_method(app, product_code.as_deref(), system);
    let names = app_product_names(app);

    let mut roots = Vec::new();
    let mut loose_files = Vec::new();
    let mut recorded_files = Vec::new();
    let mut registry_keys = Vec::new();
    let mut evidence = Vec::new();
    let mut authoritative_file_count = 0;

    match method {
        InstallMethod::WindowsInstaller => {
            let products = product_code
                .as_deref()
                .map(|code| {
                    msi_products_of(
                        code,
                        &app.display_name,
                        app.install_scope == InstallScope::PerUser,
                        system,
                    )
                })
                .unwrap_or_default();
            if products.len() > 1 {
                evidence.push(format!(
                    "Burn bundle of {} Windows Installer packages",
                    products.len() - 1
                ));
            }
            if let Some(paths) = msi_component_paths(&products, system, is_cancelled) {
                authoritative_file_count = paths.files.len();
                evidence.push(format!(
                    "Windows Installer database: {} component key paths",
                    authoritative_file_count + paths.registry_keys.len()
                ));
                let found = footprint_roots(&paths.files, &names, system, is_cancelled);
                roots.extend(found.dirs);
                loose_files.extend(found.loose_files);
                recorded_files = paths.files;
                registry_keys = paths.registry_keys;
            }
        }
        InstallMethod::InnoSetup => {
            if let Some(log) = inno_log_for(app, system)
                .filter(|log| record_can_be_believed(app, log, system, &mut evidence))
            {
                let manifest = inno_manifest(&log, system);
                if manifest.files.is_empty() {
                    evidence.push("Inno Setup uninstall log (unins000.dat)".to_owned());
                } else {
                    authoritative_file_count = manifest.files.len();
                    evidence.push(format!(
                        "Inno Setup uninstall log: {authoritative_file_count} recorded paths"
                    ));
                    let found = footprint_roots(&manifest.files, &names, system, is_cancelled);
                    roots.extend(found.dirs);
                    loose_files.extend(found.loose_files);
                    recorded_files = manifest.files;
                }
                registry_keys = manifest.registry_keys;
            }
        }
        InstallMethod::Nsis => {
            evidence.push("NSIS (Nullsoft) uninstaller signature".to_owned());
            // NSIS keeps no manifest by default, but the AdvUninstLog macro
            // writes an `uninstall.log` of installed paths in the install dir.
            if let Some(dir) = primary_install_dir(app)
                && let Some(log) = find_nsis_log(&dir).filter(|log| {
                    system.may_read(log) && record_can_be_believed(app, log, system, &mut evidence)
                })
            {
                let files = nsis_manifest_files(&log, system);
                if !files.is_empty() {
                    authoritative_file_count = files.len();
                    evidence.push(format!(
                        "NSIS AdvUninstLog uninstall.log: {authoritative_file_count} recorded paths"
                    ));
                    let found = footprint_roots(&files, &names, system, is_cancelled);
                    roots.extend(found.dirs);
                    loose_files.extend(found.loose_files);
                    recorded_files = files;
                }
            }
        }
        InstallMethod::Squirrel => {
            evidence.push("Squirrel per-user layout (Update.exe / app-*)".to_owned());
            if let Some(dir) = primary_install_dir(app) {
                roots.push(dir.clone());
                if let Some(app_dir) = latest_squirrel_app_dir(&dir, system) {
                    if let Some(version) = app_dir.file_name().and_then(|name| name.to_str()) {
                        evidence.push(format!("Squirrel current version: {version}"));
                    }
                    roots.push(app_dir);
                }
            }
        }
        InstallMethod::Chocolatey => {
            evidence.push("Chocolatey package metadata".to_owned());
        }
        InstallMethod::Msix => {
            evidence.push("MSIX / Appx package".to_owned());
            // The install payload under WindowsApps is authoritative, but the
            // app's mutable per-user state lives at
            // %LOCALAPPDATA%\Packages\<PackageFamilyName>, whose folder name is
            // the package identity (not the display name) and is therefore
            // missed by name-token heuristics. Read the manifest identity to
            // anchor it directly.
            if let Some(dir) = primary_install_dir(app)
                && system.may_read(&dir.join(APPX_MANIFEST))
                && let Some(identity) = appx_identity_name(&dir, system)
            {
                evidence.push(format!("AppxManifest identity: {identity}"));
                let package_roots = msix_package_data_roots(&identity, system);
                if !package_roots.is_empty() {
                    evidence.push(format!(
                        "MSIX per-user package data: {} location(s)",
                        package_roots.len()
                    ));
                    roots.extend(package_roots);
                }
            }
        }
        InstallMethod::Generic => {}
    }

    let mut uninstaller_folder = None;
    let mut folders_left_alone = Vec::new();
    // App Paths registration is an authoritative, documented source of the
    // launcher location and any extra DLL search directories.
    if let Some(executable) = app.main_executable.as_ref() {
        let app_paths = app_paths_roots(
            executable,
            app.install_location.as_deref(),
            &names,
            system,
            &mut folders_left_alone,
        );
        if !app_paths.is_empty() {
            evidence.push("App Paths registration".to_owned());
            roots.extend(app_paths);
        }
    }

    // The uninstaller almost always lives in the install tree, so its directory
    // is a reliable anchor even when InstallLocation is absent. A Windows host such
    // as rundll32 or an installer's cached copy of itself says nothing about the app. Nor does one
    // somewhere else: a setup run from someone's Downloads is that file, and not all that is with it.
    if let Some(uninstall) = app.uninstall_string.as_deref()
        && let Some(executable) = split_command_executable_path(uninstall)
        && let Some(parent) = executable.parent()
        && let Some(parent) = system.resolve_source_path(parent)
        && !is_windows_or_installer_cache_path(&parent, system)
    {
        if uninstaller_folder_is_the_apps(app, &parent, &names, system) {
            uninstaller_folder = Some(parent.clone());
            roots.push(parent);
        } else if is_where_programs_are_put(&parent, system) {
            loose_files.extend(system.resolve_source_path(&executable));
        }
    }

    InstallSource {
        method,
        product_code,
        authoritative_file_count,
        roots: dedup_existing_dirs(roots, system),
        loose_files,
        recorded_files,
        registry_keys,
        evidence,
        uninstaller_folder,
        folders_left_alone,
    }
}

/// Whether a folder says by its name that it is this app's: it is called what the app is called, or
/// is in a folder of the app's vendor by that name.
fn is_named_for_the_app(folder: &Path, names: &ProductNames) -> bool {
    let Some(name) = folder.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let parent = folder
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|parent| parent.to_str());
    names.matches_top_level(name, MatchScope::ProgramFolders)
        || parent
            .is_some_and(|parent| names.matches_nested(parent, name, MatchScope::ProgramFolders))
}

/// Where an installer puts a program, so that one left there alone is the app's to bring. A setup
/// run from someone's Downloads is theirs, on whatever drive that is, and removes the app there and
/// nowhere else.
fn is_where_programs_are_put(folder: &Path, system: &SystemContext) -> bool {
    system
        .program_files_dirs()
        .iter()
        .chain(&system.program_data())
        .any(|base| path_is_within(folder, base))
}

/// Whether a folder is where things are put by whoever uses a PC, and is nobody's whole.
fn is_where_a_person_puts_things(folder: &Path) -> bool {
    folder.file_name().is_some_and(|name| {
        WHERE_A_PERSON_PUTS_THINGS.contains(&name.to_string_lossy().to_ascii_lowercase().as_str())
    })
}

/// Whether a folder holding the uninstaller, and somewhere inside it the app's program, is the app's.
/// That is where an installer was told to put it, and the program being deep in it says nothing of
/// what is beside that, like the `Content` of `D:\Games\MyGame\Binaries\Win64\game.exe`. Not a folder
/// a vendor keeps its products in, or anywhere a person keeps anything.
fn is_above_the_program(folder: &Path, program: &Path, names: &ProductNames) -> bool {
    path_is_within(program, folder)
        && !is_shared_folder(folder)
        && !is_drive_root(folder)
        && !is_where_a_person_puts_things(folder)
        && !folder
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| names.is_named_only_for_publisher(name))
}

/// Whether the folder an uninstaller is in is the app's. Inside where its entry says the app is, it
/// is. With no such place said, one that holds its program is, and one in `Program Files` is what an
/// installer made, unless it is only named for who made it, as it may hold all they make. Anywhere else
/// it has to say so by its name, which is what tells `D:\Tools\Acme` from `D:\Downloads`.
fn uninstaller_folder_is_the_apps(
    app: &InstalledApp,
    folder: &Path,
    names: &ProductNames,
    system: &SystemContext,
) -> bool {
    let location = app
        .install_location
        .as_deref()
        .and_then(|location| system.resolve_source_path(location));
    if location
        .as_deref()
        .is_some_and(|location| path_is_within(folder, location))
    {
        return true;
    }
    let is_only_the_vendors = folder
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| names.is_named_only_for_publisher(name) && !is_app_called(app, name));
    if is_only_the_vendors {
        return false;
    }
    if is_named_for_the_app(folder, names) {
        return true;
    }
    if location.is_some() {
        return false;
    }
    if let Some(program) = app.main_executable.as_deref()
        && (program
            .parent()
            .is_some_and(|parent| path_is_within(folder, parent))
            || is_above_the_program(folder, program, names))
    {
        return true;
    }
    system
        .program_files_dirs()
        .iter()
        .any(|base| path_is_within(folder, base))
}

/// Whether an app is called what this folder is, as `Notepad++` is by `Notepad++ Team`.
fn is_app_called(app: &InstalledApp, folder: &str) -> bool {
    normalize_name_for_match(&app.display_name) == normalize_name_for_match(folder)
}

/// With the program only known now, as nothing in the app's entry said which it was: the folder the
/// uninstaller is in is where the app is if that folder holds it.
pub fn add_the_uninstallers_folder_above_the_program(
    source: &mut InstallSource,
    app: &InstalledApp,
    executable: &Path,
    system: &SystemContext,
) {
    if app.install_location.is_some() || source.uninstaller_folder.is_some() {
        return;
    }
    let names = app_product_names(app);
    let folder = app
        .uninstall_string
        .as_deref()
        .and_then(split_command_executable_path)
        .and_then(|uninstaller| uninstaller.parent().map(Path::to_path_buf))
        .and_then(|folder| system.resolve_source_path(&folder))
        .filter(|folder| !is_windows_or_installer_cache_path(folder, system))
        .filter(|folder| is_above_the_program(folder, executable, &names));
    if let Some(folder) = folder {
        source.uninstaller_folder = Some(folder.clone());
        let mut roots = std::mem::take(&mut source.roots);
        roots.push(folder);
        source.roots = dedup_existing_dirs(roots, system);
    }
}

/// Add the App Paths directories of an executable that was only chosen after
/// `detect_install_source` ran, because the uninstall entry named none.
pub fn add_app_paths_roots(
    source: &mut InstallSource,
    app: &InstalledApp,
    executable: &Path,
    system: &SystemContext,
) {
    let names = app_product_names(app);
    let app_paths = app_paths_roots(
        executable,
        app.install_location.as_deref(),
        &names,
        system,
        &mut source.folders_left_alone,
    );
    if !app_paths.is_empty() {
        source.evidence.push("App Paths registration".to_owned());
        let mut roots = std::mem::take(&mut source.roots);
        roots.extend(app_paths);
        source.roots = dedup_existing_dirs(roots, system);
    }
}

/// The names to match an installed app against before its executable is known.
pub(crate) fn app_product_names(app: &InstalledApp) -> ProductNames {
    ProductNames::new(Some(&app.display_name), app.publisher.as_deref(), &[], &[])
}

/// True when an executable could be the application itself: it exists, is not an
/// uninstaller or updater, and is neither a Windows host nor an installer's cached copy.
pub(crate) fn is_usable_main_executable(path: &Path, system: &SystemContext) -> bool {
    system.may_read(path)
        && is_executable_path(path)
        && !is_auxiliary_executable(path)
        && !is_windows_or_installer_cache_path(path, system)
}

pub(crate) fn is_windows_or_installer_cache_path(path: &Path, system: &SystemContext) -> bool {
    if path_is_within(path, &system.windows_dir()) {
        return true;
    }
    let mut lower = path.to_string_lossy().to_ascii_lowercase();
    lower.push('\\');
    INSTALLER_CACHE_MARKERS
        .iter()
        .any(|marker| lower.contains(marker))
}

/// App Paths registrations and Start Menu shortcuts, which say where an application's
/// executable is when its uninstall entry does not. Read once per listing or scan.
#[derive(Debug, Default)]
pub(crate) struct ExecutableHints {
    app_paths: Vec<PathBuf>,
    shortcuts: Vec<ShortcutHint>,
}

#[derive(Debug)]
struct ShortcutHint {
    name: String,
    target: PathBuf,
}

impl ExecutableHints {
    pub(crate) fn read(system: &SystemContext) -> Self {
        Self::keeping_usable(
            read_app_paths_targets(system, true),
            read_start_menu_shortcuts(system, true),
            system,
        )
    }

    /// Only what takes an administrator to write. An account can write a shortcut or an App Paths
    /// key of its own with no rights at all, naming any folder as a machine-wide app's, which would
    /// then be packaged whole.
    pub(crate) fn read_machine_wide(system: &SystemContext) -> Self {
        Self::keeping_usable(
            read_app_paths_targets(system, false),
            read_start_menu_shortcuts(system, false),
            system,
        )
    }

    /// Checked here, once, because a listing asks about every app in turn and each check
    /// touches the disk.
    fn keeping_usable(
        mut app_paths: Vec<PathBuf>,
        mut shortcuts: Vec<ShortcutHint>,
        system: &SystemContext,
    ) -> Self {
        app_paths.retain(|path| is_usable_main_executable(path, system));
        shortcuts.retain(|hint| is_usable_main_executable(&hint.target, system));
        Self {
            app_paths,
            shortcuts,
        }
    }

    /// The executable these hints name for an app, if any. One that is both named for
    /// the app and inside its install folder wins, then one named for the app, then
    /// anything else inside its install folder. App Paths beats a shortcut at each step.
    pub(crate) fn find(
        &self,
        names: &ProductNames,
        install_location: Option<&Path>,
    ) -> Option<PathBuf> {
        let inside =
            |path: &Path| install_location.is_some_and(|location| path_is_within(path, location));
        // Store apps register App Paths too, and `notepad.exe` there is not Notepad++.
        let usable = |path: &Path| {
            inside(path)
                || !path
                    .to_string_lossy()
                    .to_ascii_lowercase()
                    .contains(WINDOWS_APPS_MARKER)
        };
        // Side-by-side versions share a name: `Python 3.14.10` wants `Python 3.14`'s
        // shortcut and not `Python 3.13`'s.
        let same_version = self
            .shortcuts
            .iter()
            .filter(|hint| usable(&hint.target))
            .find(|hint| names.names_executable(&hint.name) && names.shares_version(&hint.name));
        let app_path_named = |path: &&PathBuf| executable_stem_matches(path, names);
        let shortcut_named = |hint: &&ShortcutHint| {
            names.names_executable(&hint.name) || executable_stem_matches(&hint.target, names)
        };
        let app_paths = || self.app_paths.iter().filter(|path| usable(path));
        let shortcuts = || self.shortcuts.iter().filter(|hint| usable(&hint.target));

        app_paths()
            .filter(app_path_named)
            .find(|path| inside(path))
            .or_else(|| {
                shortcuts()
                    .filter(shortcut_named)
                    .find(|hint| inside(&hint.target))
                    .map(|hint| &hint.target)
            })
            .or_else(|| same_version.map(|hint| &hint.target))
            .or_else(|| app_paths().find(app_path_named))
            .or_else(|| shortcuts().find(shortcut_named).map(|hint| &hint.target))
            .or_else(|| app_paths().find(|path| inside(path)))
            .or_else(|| {
                shortcuts()
                    .find(|hint| inside(&hint.target))
                    .map(|hint| &hint.target)
            })
            .cloned()
    }
}

fn executable_stem_matches(path: &Path, names: &ProductNames) -> bool {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| names.names_executable(stem))
}

fn read_app_paths_targets(system: &SystemContext, of_users_too: bool) -> Vec<PathBuf> {
    let mut keys = Vec::new();
    for flags in [KEY_READ | KEY_WOW64_64KEY, KEY_READ | KEY_WOW64_32KEY] {
        keys.extend(system.open_hklm(APP_PATHS_PREFIX, flags).ok());
    }
    if of_users_too {
        for user in system.user_hives() {
            keys.extend(user.open(APP_PATHS_PREFIX, KEY_READ).ok());
        }
    }
    let mut seen = BTreeSet::new();
    let mut targets = Vec::new();
    for key in keys {
        for name in key.enum_keys().flatten() {
            let Ok(child) = key.open_subkey_with_flags(&name, KEY_READ) else {
                continue;
            };
            let Ok(value) = child.get_value::<String, _>("") else {
                continue;
            };
            let target = value.trim().trim_matches('"');
            // An unexpanded %SystemRoot% entry is a Windows component, never an app's.
            if !target.is_empty()
                && !target.contains('%')
                && seen.insert(target.to_ascii_lowercase())
            {
                targets.extend(system.resolve_source_path(Path::new(target)));
            }
        }
    }
    targets
}

fn read_start_menu_shortcuts(system: &SystemContext, of_users_too: bool) -> Vec<ShortcutHint> {
    let mut folders = Vec::new();
    if let Some(program_data) = system.program_data() {
        folders.push(program_data.join(START_MENU_PROGRAMS));
    }
    // Only of the account this is about: another's shortcut says nothing about this one's apps,
    // and following one would take that person's folder for the app's.
    if of_users_too {
        for user in system.user_hives() {
            folders.push(user.profile_dir.join(USER_START_MENU_PROGRAMS));
        }
    }
    let mut shortcuts = Vec::new();
    for folder in folders.into_iter().filter(|folder| system.may_read(folder)) {
        for entry in walk_asking(&folder, MAX_START_MENU_DEPTH, |path, _| {
            system.may_read(path)
        })
        .flatten()
        {
            let path = entry.path.as_path();
            if !path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("lnk"))
            {
                continue;
            }
            let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            if !entry.file_type.is_file() {
                continue;
            }
            let Some(target) = read_shortcut_file(path)
                .and_then(|data| lnk_target_path(&data))
                .and_then(|target| system.resolve_source_path(&target))
            else {
                continue;
            };
            shortcuts.push(ShortcutHint {
                name: name.to_owned(),
                target,
            });
        }
    }
    shortcuts
}

/// Resolve an application's main executable when the uninstall entry led to none. Many
/// MSI products omit both `InstallLocation` and a usable `DisplayIcon`, so the list
/// has nothing to go on. In order: App Paths and shortcuts, then the files the install
/// record names, then the install folders. When only an uninstaller or an installer's
/// cached copy exists, as for a font pack or a runtime, that is returned as a last
/// resort so the app can still be packaged. It never returns a Windows host.
///
/// Takes the `InstallSource` the scan already has: reading the MSI database is slow, and
/// this must not do it again.
pub fn primary_executable(
    app: &InstalledApp,
    source: &InstallSource,
    system: &SystemContext,
) -> Option<PathBuf> {
    let names = app_product_names(app);
    let location = app.install_location.as_deref();
    let hints = if app.install_scope == InstallScope::PerUser {
        ExecutableHints::read(system)
    } else {
        ExecutableHints::read_machine_wide(system)
    };
    if let Some(found) = hints.find(&names, location) {
        return Some(found);
    }

    let mut candidates: Vec<PathBuf> = source
        .recorded_files
        .iter()
        .filter(|path| is_executable_path(path))
        .cloned()
        .collect();
    if candidates.is_empty() {
        for dir in source.roots.iter().map(PathBuf::as_path).chain(location) {
            collect_dir_executables(dir, &mut candidates, system);
        }
    }
    let usable = |path: &PathBuf| is_usable_main_executable(path, system);
    if let Some(found) = pick_primary_executable(
        candidates
            .iter()
            .filter(|path| usable(path))
            .cloned()
            .collect(),
        &names,
        &app.display_name,
    ) {
        return Some(found);
    }

    for dir in source.roots.iter().map(PathBuf::as_path).chain(location) {
        collect_tree_executables(dir, &mut candidates, system);
    }
    pick_primary_executable(
        candidates
            .iter()
            .filter(|path| usable(path))
            .cloned()
            .collect(),
        &names,
        &app.display_name,
    )
    .or_else(|| last_resort_executable(app, source, candidates, system))
}

/// A Squirrel app's folder holds a small stub that starts the real executable in
/// `app-<version>`, and the stub's architecture and imports say nothing about the app.
/// Returns the real one when `executable` is such a stub.
pub fn squirrel_current_executable(
    executable: &Path,
    source: &InstallSource,
    system: &SystemContext,
) -> Option<PathBuf> {
    if source.method != InstallMethod::Squirrel {
        return None;
    }
    let current = latest_squirrel_app_dir(executable.parent()?, system)?;
    Some(current.join(executable.file_name()?)).filter(|path| system.is_file(path))
}

/// An uninstaller or a cached installer, when that is all there is: architecture and
/// import analysis then describe it and not the app, so the scan warns. Never a Windows
/// host, which would say nothing at all and match every app that uses it. An executable
/// the app's own install record puts in the Windows directory is real, though: that is
/// where the Python launcher's `py.exe` goes.
fn last_resort_executable(
    app: &InstalledApp,
    source: &InstallSource,
    candidates: Vec<PathBuf>,
    system: &SystemContext,
) -> Option<PathBuf> {
    let uninstaller = app
        .uninstall_string
        .as_deref()
        .and_then(split_command_executable_path)
        .and_then(|path| system.resolve_source_path(&path));
    let windows_dir = system.windows_dir();
    let readable = |path: &&PathBuf| system.may_read(path) && is_executable_path(path);
    candidates
        .iter()
        .chain(uninstaller.iter())
        .filter(readable)
        .find(|path| !path_is_within(path, &windows_dir))
        .or_else(|| {
            candidates
                .iter()
                .filter(readable)
                .find(|path| source.recorded_files.contains(path))
        })
        .cloned()
}

/// The folder an app's relative paths hang from, which is also where its installer
/// offers to install. The uninstall entry's location, when the executable is in it.
/// Otherwise the nearest folder above the executable named for the product, so that
/// `Google\Chrome\Application\chrome.exe` installs as `Chrome` and keeps its layout, and
/// NVIDIA App is not anchored to an installer cache it does not run from. Otherwise the
/// shallowest recorded root holding the executable, or its own folder, moving up from a
/// name like `bin`. Never a shared folder: an executable directly in the Windows directory
/// has no install folder of its own, and calling that one its own would claim everything
/// in it.
pub fn resolve_install_location(
    app: &InstalledApp,
    executable: &Path,
    source: &InstallSource,
) -> Option<PathBuf> {
    let recorded = app
        .install_location
        .as_ref()
        .filter(|location| location.is_dir());
    if let Some(location) = recorded
        && path_is_within(executable, location)
    {
        return Some(location.clone());
    }

    let names = app_product_names(app);
    let stem = executable.file_stem().and_then(|stem| stem.to_str());
    let names_with_stem = ProductNames::new(
        Some(&app.display_name),
        app.publisher.as_deref(),
        &stem.into_iter().collect::<Vec<_>>(),
        &[],
    );
    let product_folder = executable
        .ancestors()
        .skip(1)
        .take_while(|ancestor| !is_shared_folder(ancestor))
        .find(|ancestor| {
            ancestor
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    names_with_stem.matches_top_level(name, MatchScope::ProgramFolders)
                })
        });
    if let Some(folder) = product_folder {
        return Some(folder.to_path_buf());
    }
    if let Some(location) = recorded {
        return Some(location.clone());
    }
    let shallowest_root = source
        .roots
        .iter()
        .filter(|root| path_is_within(executable, root) && !is_shared_folder(root))
        .min_by_key(|root| root.components().count());
    if let Some(root) = shallowest_root {
        return Some(root.clone());
    }
    let parent = executable.parent()?;
    let folder = move_up_from_executable_folder(parent, &names);
    (!is_shared_folder(folder)).then(|| folder.to_path_buf())
}

fn move_up_from_executable_folder<'a>(folder: &'a Path, names: &ProductNames) -> &'a Path {
    let holds_executables = folder
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(is_executable_folder_name);
    match folder.parent() {
        Some(parent)
            if holds_executables
                && !is_shared_folder(parent)
                && !parent
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| names.is_vendor_only_folder(name)) =>
        {
            parent
        }
        _ => folder,
    }
}

fn is_executable_folder_name(name: &str) -> bool {
    EXECUTABLE_FOLDER_NAMES.contains(&name.to_ascii_lowercase().as_str())
}

fn is_executable_path(path: &Path) -> bool {
    looks_like_executable(path) && path.is_file()
}

fn collect_dir_executables(dir: &Path, out: &mut Vec<PathBuf>, system: &SystemContext) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if system.may_read(&path) && is_executable_path(&path) {
            out.push(path);
        }
    }
}

fn collect_tree_executables(dir: &Path, out: &mut Vec<PathBuf>, system: &SystemContext) {
    let executables = walk_asking(dir, MAX_EXECUTABLE_SEARCH_DEPTH, |path, _| {
        system.may_read(path)
    })
    .flatten()
    .filter(|entry| entry.file_type.is_file() && looks_like_executable(&entry.path))
    .map(|entry| entry.path);
    out.extend(executables);
}

/// Pick the most representative main executable from a candidate set: prefer one named
/// for the product, then one whose file name overlaps the display name, and otherwise
/// the largest of the shallowest. Depth first: the largest anywhere can be a tool for
/// another platform, like the ARM64 host packed inside a .NET SDK.
fn pick_primary_executable(
    candidates: Vec<PathBuf>,
    names: &ProductNames,
    display_name: &str,
) -> Option<PathBuf> {
    let mut filtered = candidates;
    filtered.sort();
    filtered.dedup();
    if filtered.is_empty() {
        return None;
    }
    if let Some(named) = filtered
        .iter()
        .find(|path| executable_stem_matches(path, names))
    {
        return Some(named.clone());
    }

    let normalized_display = normalize_for_match(display_name);
    if !normalized_display.is_empty()
        && let Some(matched) = filtered.iter().find(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(normalize_for_match)
                .map(|stem| {
                    !stem.is_empty()
                        && (normalized_display.contains(&stem)
                            || stem.contains(&normalized_display))
                })
                .unwrap_or(false)
        })
    {
        return Some(matched.clone());
    }

    filtered.into_iter().min_by_key(|path| {
        let size = std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0);
        (path.components().count(), std::cmp::Reverse(size))
    })
}

fn normalize_for_match(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn classify_method(
    app: &InstalledApp,
    product_code: Option<&str>,
    system: &SystemContext,
) -> InstallMethod {
    let location = app
        .install_location
        .as_ref()
        .map(|path| path.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    let uninstall = app
        .uninstall_string
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();

    if location.contains(r"\windowsapps\") {
        return InstallMethod::Msix;
    }
    if location.contains(r"\chocolatey\lib\") || uninstall.contains(r"\chocolatey\") {
        return InstallMethod::Chocolatey;
    }
    if location.contains(r"\appdata\local\")
        && is_squirrel_layout(app.install_location.as_deref(), system)
    {
        return InstallMethod::Squirrel;
    }
    if product_code.is_some() || uninstall.contains("msiexec") {
        return InstallMethod::WindowsInstaller;
    }

    // EXE installers: identify the toolkit from concrete on-disk evidence.
    if inno_log_for(app, system).is_some() {
        return InstallMethod::InnoSetup;
    }
    if let Some(dir) = primary_install_dir(app)
        && is_nsis_install(&dir, app.uninstall_string.as_deref(), system)
    {
        return InstallMethod::Nsis;
    }
    InstallMethod::Generic
}

fn primary_install_dir(app: &InstalledApp) -> Option<PathBuf> {
    if let Some(location) = app.install_location.as_ref()
        && location.is_dir()
    {
        return Some(location.clone());
    }
    app.main_executable
        .as_ref()
        .and_then(|executable| executable.parent())
        .map(Path::to_path_buf)
}

/// Inno Setup records every installed file and registry key in an uninstall log named
/// `unins000.dat`, beside the uninstaller. That is usually the install directory, but
/// not always: GIMP keeps it in `uninst`.
fn inno_log_for(app: &InstalledApp, system: &SystemContext) -> Option<PathBuf> {
    let beside_uninstaller = app
        .uninstall_string
        .as_deref()
        .and_then(split_command_executable_path)
        .and_then(|path| {
            path.parent()
                .and_then(|parent| system.resolve_source_path(parent))
        });
    beside_uninstaller
        .as_deref()
        .and_then(find_inno_log)
        .or_else(|| primary_install_dir(app).as_deref().and_then(find_inno_log))
        .filter(|log| system.may_read(log))
}

fn find_inno_log(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if is_inno_log_name(&name) {
            return Some(entry.path());
        }
    }
    None
}

/// True for an Inno uninstall log name such as `unins000.dat` (three digits).
/// Deliberately does not match a plain `uninstall.exe`, which other installers
/// also use.
fn is_inno_log_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    name.len() == 12
        && name.starts_with("unins")
        && name.ends_with(".dat")
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
        && bytes[7].is_ascii_digit()
}

/// The files and registry keys an Inno Setup uninstall log records.
#[derive(Debug, Default)]
struct InnoManifest {
    /// Recorded files that still exist. A log grows with every update: on one PC, VS
    /// Code's held 272,656 file records for 6,443 distinct files still on disk.
    files: Vec<PathBuf>,
    registry_keys: Vec<RecordedRegistryKey>,
}

/// Read an Inno Setup uninstall log. Logs in a framing this parser does not recognize
/// fall back to scanning for path strings, which finds most ASCII paths but can return a
/// path corrupted by the byte after it. Such a path fails the existence check, so the
/// result is an undercount and never a wrong file.
fn inno_manifest(log_path: &Path, system: &SystemContext) -> InnoManifest {
    let Some(data) = read_within(log_path, MAX_INSTALL_LOG_BYTES, system) else {
        return InnoManifest::default();
    };
    if !is_inno_log(&data) {
        return InnoManifest::default();
    }
    let (candidates, registry_keys) = match parse_inno_log(&data) {
        Some(log) => (log.files, log.registry_keys),
        None => (scanned_path_strings(&data), Vec::new()),
    };
    InnoManifest {
        files: existing_files(candidates, system),
        registry_keys,
    }
}

/// Size of the fixed header before an Inno log's first block: a 64-byte signature, the
/// 128-byte AppId and AppName, four 32-bit fields, 27 reserved ones, and a CRC.
const INNO_HEADER_LEN: usize = 448;
const INNO_END_OFFSET_FIELD: usize = 328;
/// Each block starts with its 32-bit size, the size's complement, and a CRC.
const INNO_BLOCK_HEADER_LEN: usize = 12;
/// Each record starts with a 16-bit type, 32-bit extra data, and a 32-bit data size.
const INNO_RECORD_HEADER_LEN: usize = 10;
const INNO_RECORD_DELETE_FILE: u16 = 0x82;
const INNO_RECORD_REG_DELETE_ENTIRE_KEY: u16 = 0x86;
const INNO_RECORD_REG_CLEAR_VALUE: u16 = 0x87;
const INNO_RECORD_REG_DELETE_KEY_IF_EMPTY: u16 = 0x88;
const INNO_RECORD_REG_DELETE_VALUE: u16 = 0x89;
const INNO_STRING_END: u8 = 0xFF;
const INNO_STRING_LEN_I32: u8 = 0xFE;
const INNO_STRING_LEN_U16: u8 = 0xFD;
const INNO_REG_VIEW_64_BIT: u32 = 0x0100_0000;
const INNO_REG_ROOT_MASK: u32 = 0x00FF_FFFF;
const INNO_REG_ROOT_CLASSES: u32 = 0;
const INNO_REG_ROOT_CURRENT_USER: u32 = 1;
const INNO_REG_ROOT_LOCAL_MACHINE: u32 = 2;
const LONG_PATH_PREFIX: &str = r"\\?\";

/// What the records of an Inno log name, before checking the disk.
#[derive(Debug, Default, PartialEq)]
struct InnoLogRecords {
    files: Vec<String>,
    registry_keys: Vec<RecordedRegistryKey>,
}

/// Parse an Inno Setup uninstall log. After the header the file is a run of CRC-framed
/// blocks whose bodies, joined, are a run of records, so a record can straddle two
/// blocks. Each record's data is a list of length-prefixed strings. Reading the lengths
/// is what makes this work: on a Unicode log, the end marker and the next record's type
/// sit right after a path and can decode as a CJK letter, which defeated scanning for
/// path text. `None` when the framing is not recognized.
fn parse_inno_log(data: &[u8]) -> Option<InnoLogRecords> {
    let body = inno_log_body(data)?;
    let mut records = InnoLogRecords::default();
    let mut seen_files = BTreeSet::new();
    let mut seen_keys = BTreeSet::new();
    let mut offset = 0;
    while let Some(header) = body.get(offset..offset + INNO_RECORD_HEADER_LEN) {
        let kind = u16::from_le_bytes([header[0], header[1]]);
        let extra = u32::from_le_bytes([header[2], header[3], header[4], header[5]]);
        let size = u32::from_le_bytes([header[6], header[7], header[8], header[9]]) as usize;
        let start = offset + INNO_RECORD_HEADER_LEN;
        let Some(record) = start.checked_add(size).and_then(|end| body.get(start..end)) else {
            break;
        };
        offset = start + size;
        match kind {
            INNO_RECORD_DELETE_FILE => {
                if let Some(path) = inno_strings(record).into_iter().next() {
                    let path = path
                        .strip_prefix(LONG_PATH_PREFIX)
                        .map(ToOwned::to_owned)
                        .unwrap_or(path);
                    if seen_files.insert(path.to_ascii_lowercase()) {
                        records.files.push(path);
                    }
                }
            }
            INNO_RECORD_REG_DELETE_ENTIRE_KEY | INNO_RECORD_REG_DELETE_KEY_IF_EMPTY => {
                let strings = inno_strings(record);
                if let Some(key) = strings
                    .first()
                    .and_then(|path| inno_registry_key(extra, path, None))
                    && seen_keys.insert(key.identity())
                {
                    records.registry_keys.push(key);
                }
            }
            INNO_RECORD_REG_CLEAR_VALUE | INNO_RECORD_REG_DELETE_VALUE => {
                let strings = inno_strings(record);
                if let [path, value, ..] = strings.as_slice()
                    && let Some(key) = inno_registry_key(extra, path, Some(value))
                    && seen_keys.insert(key.identity())
                {
                    records.registry_keys.push(key);
                }
            }
            _ => {}
        }
    }
    (!records.files.is_empty() || !records.registry_keys.is_empty()).then_some(records)
}

/// The joined bodies of an Inno log's blocks, stopping at the first one that fails its
/// size check. `None` when the first block already fails.
fn inno_log_body(data: &[u8]) -> Option<Vec<u8>> {
    let end_field = data.get(INNO_END_OFFSET_FIELD..INNO_END_OFFSET_FIELD + 4)?;
    let end = u32::from_le_bytes([end_field[0], end_field[1], end_field[2], end_field[3]]) as usize;
    let end = end.min(data.len());
    let mut body = Vec::with_capacity(end.saturating_sub(INNO_HEADER_LEN));
    let mut offset = INNO_HEADER_LEN;
    while offset + INNO_BLOCK_HEADER_LEN <= end {
        let header = &data[offset..offset + INNO_BLOCK_HEADER_LEN];
        let size = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let complement = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        let start = offset + INNO_BLOCK_HEADER_LEN;
        let block_end = start.saturating_add(size as usize);
        if size != !complement || block_end > end {
            break;
        }
        body.extend_from_slice(&data[start..block_end]);
        offset = block_end;
    }
    (!body.is_empty()).then_some(body)
}

/// Decode a record's strings. A length below 0xFD is the byte itself, 0xFD and 0xFE
/// introduce a 16-bit and a 32-bit length, and 0xFF ends the list. A negative length is
/// a byte count of UTF-16 text. A positive one is ANSI text, read as Latin-1: a path
/// outside that code page then fails the existence check and falls to the install
/// folder, as it did before. Stops without error at malformed data.
fn inno_strings(record: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut offset = 0;
    while let Some(&tag) = record.get(offset) {
        offset += 1;
        let length = match tag {
            INNO_STRING_END => break,
            INNO_STRING_LEN_I32 => {
                let Some(bytes) = record.get(offset..offset + 4) else {
                    break;
                };
                offset += 4;
                i64::from(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            INNO_STRING_LEN_U16 => {
                let Some(bytes) = record.get(offset..offset + 2) else {
                    break;
                };
                offset += 2;
                i64::from(u16::from_le_bytes([bytes[0], bytes[1]]))
            }
            short => i64::from(short),
        };
        let byte_count = length.unsigned_abs() as usize;
        let Some(bytes) = offset
            .checked_add(byte_count)
            .and_then(|end| record.get(offset..end))
        else {
            break;
        };
        offset += byte_count;
        strings.push(if length < 0 {
            let (pairs, _) = bytes.as_chunks::<2>();
            let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
            String::from_utf16_lossy(&units)
        } else {
            bytes.iter().map(|&byte| char::from(byte)).collect()
        });
    }
    strings
}

/// A registry record's extra data holds the root key, plus a flag for the 64-bit view.
fn inno_registry_key(extra: u32, path: &str, value: Option<&str>) -> Option<RecordedRegistryKey> {
    let root = match extra & INNO_REG_ROOT_MASK {
        INNO_REG_ROOT_CLASSES => RecordedRegistryRoot::ClassesRoot,
        INNO_REG_ROOT_CURRENT_USER => RecordedRegistryRoot::CurrentUser,
        INNO_REG_ROOT_LOCAL_MACHINE => RecordedRegistryRoot::LocalMachine,
        _ => return None,
    };
    let key_path = path.trim_end_matches('\\');
    (!key_path.is_empty()).then(|| RecordedRegistryKey {
        root,
        key_path: key_path.to_owned(),
        value_name: value.map(ToOwned::to_owned),
        wow64_32: extra & INNO_REG_VIEW_64_BIT == 0,
    })
}

/// Paths found by scanning for path text, for logs with no framing to parse.
fn scanned_path_strings(data: &[u8]) -> Vec<String> {
    extract_utf16_paths(data)
        .into_iter()
        .chain(extract_ansi_paths(data))
        .collect()
}

/// Recorded paths that exist as files on the scanned system, without duplicates.
fn existing_files(candidates: Vec<String>, system: &SystemContext) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    for candidate in candidates {
        if !seen.insert(candidate.to_ascii_lowercase()) {
            continue;
        }
        let Some(path) = system.resolve_source_path(&PathBuf::from(&candidate)) else {
            continue;
        };
        if path.is_file() {
            files.push(path);
        }
    }
    files
}

fn is_inno_log(data: &[u8]) -> bool {
    let head = &data[..64.min(data.len())];
    String::from_utf8_lossy(head).contains("Inno Setup Uninstall Log")
}

/// Locate an NSIS AdvUninstLog `uninstall.log` in the install directory.
fn find_nsis_log(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if is_nsis_log_name(&name) {
            return Some(entry.path());
        }
    }
    None
}

/// The default file the NSIS AdvUninstLog macro writes (`uninstall.log`).
fn is_nsis_log_name(name: &str) -> bool {
    name == "uninstall.log"
}

/// Extract installed file paths from an NSIS AdvUninstLog `uninstall.log`.
/// The log has no documented stable framing, so (like the Inno reader) this
/// scans for resolved absolute path strings and keeps the ones that exist,
/// which is robust across AdvUninstLog versions and UTF-16/ANSI encodings.
fn nsis_manifest_files(log_path: &Path, system: &SystemContext) -> Vec<PathBuf> {
    let Some(data) = read_within(log_path, MAX_INSTALL_LOG_BYTES, system) else {
        return Vec::new();
    };
    existing_files(scanned_path_strings(&data), system)
}

fn extract_utf16_paths(data: &[u8]) -> Vec<String> {
    let mut paths = Vec::new();
    let mut index = 0;
    while index + 6 <= data.len() {
        if data[index].is_ascii_alphabetic()
            && data[index + 1] == 0
            && data[index + 2] == b':'
            && data[index + 3] == 0
            && data[index + 4] == b'\\'
            && data[index + 5] == 0
        {
            let mut units = Vec::new();
            let mut cursor = index;
            while cursor + 2 <= data.len() {
                let unit = u16::from_le_bytes([data[cursor], data[cursor + 1]]);
                if unit == 0 {
                    break;
                }
                units.push(unit);
                cursor += 2;
            }
            let decoded: String = char::decode_utf16(units)
                .map(|result| result.unwrap_or(char::REPLACEMENT_CHARACTER))
                .take_while(|character| is_path_char(*character))
                .collect();
            push_path(&mut paths, decoded);
            index = cursor.max(index + 2);
        } else {
            index += 1;
        }
    }
    paths
}

fn extract_ansi_paths(data: &[u8]) -> Vec<String> {
    let mut paths = Vec::new();
    let mut index = 0;
    while index + 3 <= data.len() {
        if data[index].is_ascii_alphabetic() && data[index + 1] == b':' && data[index + 2] == b'\\'
        {
            let mut text = String::new();
            let mut cursor = index;
            while cursor < data.len() {
                let character = data[cursor] as char;
                if data[cursor] >= 0x20 && is_path_char(character) {
                    text.push(character);
                    cursor += 1;
                } else {
                    break;
                }
            }
            push_path(&mut paths, text);
            index = cursor.max(index + 1);
        } else {
            index += 1;
        }
    }
    paths
}

fn push_path(paths: &mut Vec<String>, candidate: String) {
    let trimmed = candidate.trim_end().to_owned();
    if trimmed.len() >= 4 {
        paths.push(trimmed);
    }
}

fn is_path_char(character: char) -> bool {
    character.is_alphanumeric()
        || character == ' '
        || r"\:./-_()[]{}+,'~!@#$%^&=;`".contains(character)
}

/// Exclude shortcut, Start Menu, Desktop and system paths so install-manifest
/// entries do not turn an entire shared system folder into a footprint root. Inno also
/// records a `.pif` beside each shortcut. A `windows` folder inside an app's own tree
/// stays: only the real Windows directory is excluded.
fn is_app_footprint_path(path: &Path, windows_dir: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    !(lower.ends_with(".lnk")
        || lower.ends_with(".pif")
        || lower.contains(r"\start menu\")
        || is_user_desktop_path(&lower)
        || lower.contains(r"\microsoft\windows\")
        || path_is_within(path, windows_dir))
}

/// True for `<drive>\Users\<name>\Desktop\...`, and not an app's own `desktop` folder.
fn is_user_desktop_path(lower: &str) -> bool {
    let mut segments = lower.split('\\').skip(1);
    segments.next() == Some("users") && segments.nth(1) == Some("desktop")
}

/// NSIS embeds the "Nullsoft.NSIS" / "Nullsoft Install System" marker in its
/// (un)installer; detect it from the uninstaller binary.
fn is_nsis_install(dir: &Path, uninstall_string: Option<&str>, system: &SystemContext) -> bool {
    let mut candidates: Vec<PathBuf> = Vec::new();
    // Read as what the source PC meant by it. Left as it is, on a drive, this could open a file
    // on this PC or on any server a value cared to name.
    if let Some(uninstall) = uninstall_string
        && let Some(executable) = split_command_executable_path(uninstall)
        && let Some(executable) = system.resolve_source_path(&executable)
    {
        candidates.push(executable);
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if name.starts_with("uninst") && name.ends_with(".exe") {
                candidates.push(entry.path());
            }
        }
    }
    candidates.iter().any(|candidate| {
        system.may_read(candidate) && file_contains_signature(candidate, b"Nullsoft")
    })
}

fn file_contains_signature(path: &Path, signature: &[u8]) -> bool {
    let Ok(file) = std::fs::File::open(path) else {
        return false;
    };
    let mut buffer = Vec::new();
    if file
        .take(SIGNATURE_SCAN_BYTES)
        .read_to_end(&mut buffer)
        .is_err()
    {
        return false;
    }
    buffer
        .windows(signature.len())
        .any(|window| window == signature)
}

/// What an App Paths entry for this program adds. Its key goes by a file's name, which another program
/// can have, so it has to say it is this program. What it lists as where to look can be anything, and
/// only counts as far as it is the app's own.
fn app_paths_roots(
    executable: &Path,
    install_location: Option<&Path>,
    names: &ProductNames,
    system: &SystemContext,
    left_alone: &mut Vec<PathBuf>,
) -> Vec<PathBuf> {
    let Some(executable_name) = executable.file_name().and_then(|value| value.to_str()) else {
        return Vec::new();
    };
    let is_this_program = |path: &str| {
        let path = Path::new(path.trim().trim_matches('"'));
        system
            .resolve_source_path(path)
            .is_some_and(|path| comparable_path(&path) == comparable_path(executable))
    };
    let is_the_apps = |folder: &Path| {
        executable
            .parent()
            .is_some_and(|parent| path_is_within(folder, parent))
            || install_location.is_some_and(|location| path_is_within(folder, location))
            || is_named_for_the_app(folder, names)
    };
    let mut roots = Vec::new();
    let subkey = format!("{APP_PATHS_PREFIX}\\{executable_name}");
    for flags in [KEY_READ | KEY_WOW64_64KEY, KEY_READ | KEY_WOW64_32KEY] {
        let Ok(key) = system.open_hklm(&subkey, flags) else {
            continue;
        };
        if !key
            .get_value::<String, _>("")
            .is_ok_and(|default| is_this_program(&default))
        {
            continue;
        }
        roots.extend(executable.parent().map(Path::to_path_buf));
        if let Ok(path_value) = key.get_value::<String, _>("Path") {
            for segment in path_value.split(';') {
                let trimmed = segment.trim().trim_matches('"');
                if !trimmed.is_empty()
                    && let Some(folder) = system.resolve_source_path(Path::new(trimmed))
                {
                    if is_the_apps(&folder) {
                        roots.push(folder);
                    } else {
                        left_alone.push(folder);
                    }
                }
            }
        }
    }
    roots
}

fn dedup_existing_dirs(roots: Vec<PathBuf>, system: &SystemContext) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut output = Vec::new();
    for root in roots {
        // An uninstaller at `C:\uninst.exe` says nothing about the rest of the drive.
        if !system.may_read(&root) || !root.is_dir() || is_drive_root(&root) {
            continue;
        }
        if seen.insert(root.to_string_lossy().to_ascii_lowercase()) {
            output.push(root);
        }
    }
    output
}

fn is_squirrel_layout(install_location: Option<&Path>, system: &SystemContext) -> bool {
    let Some(location) = install_location else {
        return false;
    };
    if system.is_file(&location.join("Update.exe")) {
        return true;
    }
    let Ok(entries) = std::fs::read_dir(location) else {
        return false;
    };
    entries
        .flatten()
        .any(|entry| system.is_dir(&entry.path()) && is_squirrel_app_dir(&entry.path()))
}

fn is_squirrel_app_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.to_ascii_lowercase().starts_with(SQUIRREL_APP_PREFIX))
}

/// The current Squirrel `app-<version>` directory (highest version) in an
/// install location, if any. Versions are compared by their dotted numeric
/// segments so `app-1.0.10` ranks above `app-1.0.9`.
fn latest_squirrel_app_dir(install_dir: &Path, system: &SystemContext) -> Option<PathBuf> {
    let entries = std::fs::read_dir(install_dir).ok()?;
    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| system.is_dir(path) && is_squirrel_app_dir(path))
        .max_by(|left, right| squirrel_version_key(left).cmp(&squirrel_version_key(right)))
}

fn squirrel_version_key(app_dir: &Path) -> Vec<u64> {
    app_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            name.trim_start_matches(|character: char| !character.is_ascii_digit())
                .split('.')
                .map(|segment| segment.parse::<u64>().unwrap_or(0))
                .collect()
        })
        .unwrap_or_default()
}

/// Read the `Identity` element `Name` from a package's `AppxManifest.xml`
/// (for example `Microsoft.WindowsTerminal`). A targeted attribute scan avoids
/// pulling in an XML dependency for a single well-known element.
fn appx_identity_name(install_dir: &Path, system: &SystemContext) -> Option<String> {
    let data = read_within(
        &install_dir.join(APPX_MANIFEST),
        MAX_APPX_MANIFEST_BYTES,
        system,
    )?;
    let text = String::from_utf8_lossy(&data);
    let identity = &text[text.find("<Identity")?..];
    let after_name = &identity[identity.find("Name=\"")? + "Name=\"".len()..];
    let name = after_name[..after_name.find('"')?].trim();
    (!name.is_empty()).then(|| name.to_owned())
}

/// Per-user MSIX package-data directories for an identity name. The folder is
/// named `<IdentityName>_<publisherHash>`, so a prefix match avoids computing
/// the publisher hash. Scoped to the `%LOCALAPPDATA%\Packages` of the account being scanned.
fn msix_package_data_roots(identity_name: &str, system: &SystemContext) -> Vec<PathBuf> {
    let prefix = format!("{}_", identity_name.to_ascii_lowercase());
    let mut roots = Vec::new();
    for local_app_data in local_app_data_dirs(system) {
        let packages = local_app_data.join(MSIX_PACKAGES_DIR);
        if !system.may_read(&packages) {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(packages) else {
            continue;
        };
        roots.extend(entries.flatten().map(|entry| entry.path()).filter(|path| {
            system.is_dir(path)
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
        }));
    }
    roots
}

/// `%LOCALAPPDATA%` bases to search: the `AppData\Local` of each account being scanned.
fn local_app_data_dirs(system: &SystemContext) -> Vec<PathBuf> {
    system
        .user_hives()
        .into_iter()
        .map(|user| user.profile_dir.join(r"AppData\Local"))
        .collect()
}

pub(crate) fn extract_product_code(app: &InstalledApp) -> Option<String> {
    if let Some(last) = app.registry_source_path.rsplit(['\\', '/']).next()
        && is_guid(last)
    {
        return Some(last.to_ascii_uppercase());
    }
    if let Some(uninstall) = app.uninstall_string.as_deref() {
        return find_guid(uninstall);
    }
    None
}

fn is_guid(value: &str) -> bool {
    let value = value.trim();
    value.len() == 38
        && value.starts_with('{')
        && value.ends_with('}')
        && value[1..value.len() - 1]
            .chars()
            .all(|character| character.is_ascii_hexdigit() || character == '-')
}

fn find_guid(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        // `get`, because byte 38 can fall inside a non-ASCII character.
        if byte == b'{'
            && let Some(candidate) = value.get(index..index + 38)
            && is_guid(candidate)
        {
            return Some(candidate.to_ascii_uppercase());
        }
    }
    None
}

/// Convert a product/component GUID into the "squished" (packed) form used as
/// registry value and key names in the Installer database.
pub fn squish_guid(guid: &str) -> Option<String> {
    let hex: Vec<char> = guid
        .chars()
        .filter(|character| character.is_ascii_hexdigit())
        .collect();
    if hex.len() != 32 {
        return None;
    }
    let mut squished = String::with_capacity(32);
    squished.extend(hex[0..8].iter().rev());
    squished.extend(hex[8..12].iter().rev());
    squished.extend(hex[12..16].iter().rev());
    for pair in hex[16..32].chunks(2) {
        squished.push(pair[1]);
        squished.push(pair[0]);
    }
    Some(squished.to_ascii_uppercase())
}

/// The products to read from the MSI database for an entry's product code: the code
/// itself, plus, for a Burn bundle, the packages it installed. A bundle is listed under a
/// code of its own that owns no files, and only its hidden packages do. Each package's
/// dependency key names the bundles that depend on it (WiX's dependency extension). A
/// runtime the bundle merely chained is a prerequisite and stays out.
///
/// An account's own hive says only what is so of an app of its own, since it takes no rights to
/// write there, and it could otherwise add any product to a machine-wide app.
fn msi_products_of(
    product_code: &str,
    display_name: &str,
    of_users_too: bool,
    system: &SystemContext,
) -> Vec<String> {
    let mut products = vec![product_code.to_owned()];
    let mut dependency_keys: Vec<RegKey> = Vec::new();
    dependency_keys.extend(system.open_hklm(MSI_DEPENDENCIES_PATH, KEY_READ).ok());
    if of_users_too {
        for user in system.user_hives() {
            dependency_keys.extend(user.open(MSI_DEPENDENCIES_PATH, KEY_READ).ok());
        }
    }
    for key in dependency_keys {
        for provider in key.enum_keys().flatten() {
            if key
                .open_subkey_with_flags(format!(r"{provider}\Dependents\{product_code}"), KEY_READ)
                .is_err()
            {
                continue;
            }
            let Ok(provider_key) = key.open_subkey_with_flags(&provider, KEY_READ) else {
                continue;
            };
            let package_name: String = provider_key.get_value("DisplayName").unwrap_or_default();
            if is_chained_runtime(&package_name, display_name) {
                continue;
            }
            if let Ok(code) = provider_key.get_value::<String, _>("")
                && is_guid(&code)
                && !products
                    .iter()
                    .any(|known| known.eq_ignore_ascii_case(&code))
            {
                products.push(code);
            }
        }
    }
    products
}

/// True for a package that is a shared runtime the app did not set out to be.
pub(crate) fn is_chained_runtime(package_name: &str, app_name: &str) -> bool {
    let package = package_name.to_ascii_lowercase();
    let app = app_name.to_ascii_lowercase();
    SHARED_RUNTIME_MARKERS
        .iter()
        .any(|marker| package.contains(marker) && !app.contains(marker))
}

/// Read everything the MSI component database records for these products, in one pass.
fn msi_component_paths(
    product_codes: &[String],
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> Option<MsiComponentPaths> {
    let squished_products: BTreeSet<String> = product_codes
        .iter()
        .filter_map(|code| squish_guid(code))
        .collect();
    if squished_products.is_empty() {
        return None;
    }
    let user_data = system
        .open_hklm(USERDATA_PATH, KEY_READ | KEY_WOW64_64KEY)
        .ok()?;

    #[cfg(test)]
    MSI_SWEEPS.with(|sweeps| sweeps.set(sweeps.get() + 1));

    let mut files = Vec::new();
    let mut registry_keys = Vec::new();
    let mut seen = BTreeSet::new();

    for sid in component_sweep_sids(&user_data, &squished_products) {
        let Ok(sid_key) = user_data.open_subkey_with_flags(&sid, KEY_READ | KEY_WOW64_64KEY) else {
            continue;
        };
        let Ok(components) =
            sid_key.open_subkey_with_flags("Components", KEY_READ | KEY_WOW64_64KEY)
        else {
            continue;
        };
        for component in components.enum_keys().flatten() {
            if is_cancelled() {
                return None;
            }
            let Ok(component_key) =
                components.open_subkey_with_flags(&component, KEY_READ | KEY_WOW64_64KEY)
            else {
                continue;
            };
            for (product, value) in component_key.enum_values().flatten() {
                if !squished_products.contains(&product) {
                    continue;
                }
                let key_path = registry_value_text(&value);
                if key_path.is_empty() {
                    continue;
                }
                if is_registry_key_path(&key_path) {
                    if seen.insert(key_path.to_ascii_lowercase())
                        && let Some(key) = msi_registry_key(&key_path)
                    {
                        registry_keys.push(key);
                    }
                } else if seen.insert(key_path.to_ascii_lowercase()) {
                    // MSI records absolute source-machine paths; re-root onto the
                    // scanned volume so downstream reads hit the right disk.
                    files.extend(system.resolve_source_path(Path::new(&msi_file_path(&key_path))));
                }
            }
        }
    }

    if files.is_empty() && registry_keys.is_empty() {
        return None;
    }
    Some(MsiComponentPaths {
        files,
        registry_keys,
    })
}

/// Every file the MSI component database records for any of these products, each as
/// [`comparable_path`] writes it, in order. For what an app's entry does not say of where it is, and
/// about nothing being scanned, so what cannot be followed is not said to be.
pub(crate) fn msi_files_of_others(
    product_codes: &[String],
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> Vec<String> {
    let squished_products: BTreeSet<String> = product_codes
        .iter()
        .filter_map(|code| squish_guid(code))
        .collect();
    if squished_products.is_empty() {
        return Vec::new();
    }
    let Ok(user_data) = system.open_hklm(USERDATA_PATH, KEY_READ | KEY_WOW64_64KEY) else {
        return Vec::new();
    };
    let mut files = Vec::new();
    for sid in component_sweep_sids(&user_data, &squished_products) {
        let Ok(components) = user_data
            .open_subkey_with_flags(format!(r"{sid}\Components"), KEY_READ | KEY_WOW64_64KEY)
        else {
            continue;
        };
        for component in components.enum_keys().flatten() {
            if is_cancelled() {
                return Vec::new();
            }
            let Ok(component_key) =
                components.open_subkey_with_flags(&component, KEY_READ | KEY_WOW64_64KEY)
            else {
                continue;
            };
            for (product, value) in component_key.enum_values().flatten() {
                if !squished_products.contains(&product) {
                    continue;
                }
                let key_path = registry_value_text(&value);
                if key_path.is_empty() || is_registry_key_path(&key_path) {
                    continue;
                }
                files.extend(
                    system
                        .resolve_source_path_quietly(Path::new(&msi_file_path(&key_path)))
                        .map(|file| comparable_path(&file)),
                );
            }
        }
    }
    files.sort_unstable();
    files.dedup();
    files
}

/// The SIDs whose `Components` subtree to sweep for a product's key paths.
///
/// A component only records a product's key path under the SID where that
/// product is registered (`UserData\<SID>\Products\<sqProduct>`), so scanning
/// only those SIDs yields identical results while skipping unrelated per-user
/// component trees on multi-user machines. This is a safe narrowing, not a
/// `Features`-based component scoping: it never decodes feature descriptors, so
/// it cannot under-report. If no SID is found to register the product (an
/// unexpected registry state), every SID is swept as a fallback so results are
/// never worse than the exhaustive scan.
fn component_sweep_sids(user_data: &RegKey, squished_products: &BTreeSet<String>) -> Vec<String> {
    let all_sids: Vec<String> = user_data.enum_keys().flatten().collect();
    let registering: Vec<String> = all_sids
        .iter()
        .filter(|sid| {
            squished_products.iter().any(|squished_product| {
                user_data
                    .open_subkey_with_flags(
                        format!(r"{sid}\Products\{squished_product}"),
                        KEY_READ | KEY_WOW64_64KEY,
                    )
                    .is_ok()
            })
        })
        .cloned()
        .collect();
    if registering.is_empty() {
        all_sids
    } else {
        registering
    }
}

/// MSI encodes registry key paths as `<root>:<path>` (for example `02:\Software\…`).
fn is_registry_key_path(value: &str) -> bool {
    let mut chars = value.chars();
    let first = chars.next();
    match first {
        Some(c) if c.is_ascii_digit() => {
            // One or two leading digits then a colon.
            matches!(chars.next(), Some(':'))
                || matches!((chars.next(), chars.next()), (Some(':'), _))
        }
        _ => false,
    }
}

/// A component that counts its references to a shared DLL records its path with a `?`
/// after the drive, as in `C:\?WINDOWS\system32\adal.dll`.
fn msi_file_path(key_path: &str) -> String {
    key_path.replacen(r":\?", r":\", 1)
}

/// Parse an MSI registry key path: `02:\Software\Foo\Bar` names the value `Bar`, and
/// `02:\Software\Foo\` names the key. The tens digit marks the 64-bit view. The ones
/// digit is the root, of which `3` (`HKEY_USERS`) names other accounts and is skipped.
fn msi_registry_key(key_path: &str) -> Option<RecordedRegistryKey> {
    let (code, path) = key_path.split_once(":\\")?;
    let code: u32 = code.parse().ok()?;
    let root = match code % 10 {
        0 => RecordedRegistryRoot::ClassesRoot,
        1 => RecordedRegistryRoot::CurrentUser,
        2 => RecordedRegistryRoot::LocalMachine,
        _ => return None,
    };
    let (key, value) = match path.strip_suffix('\\') {
        Some(key) => (key, None),
        None => {
            let (key, value) = path.rsplit_once('\\')?;
            (key, Some(value.to_owned()))
        }
    };
    (!key.is_empty()).then(|| RecordedRegistryKey {
        root,
        key_path: key.to_owned(),
        value_name: value,
        wow64_32: code < 20,
    })
}

/// Folder names that hold unrelated things side by side. One of these can never be an
/// app's root, however many of its files sit under it.
const SHARED_FOLDER_NAMES: &[&str] = &[
    "program files",
    "program files (x86)",
    "program files (arm)",
    "programdata",
    "common files",
    "microsoft shared",
    "appdata",
    "local",
    "locallow",
    "roaming",
    "programs",
    "users",
    "public",
    "documents",
    "desktop",
    "microsoft",
    "windows",
    "package cache",
];

/// Install roots and the recorded files no root can cover.
#[derive(Debug, Default, PartialEq)]
struct InstallRoots {
    dirs: Vec<PathBuf>,
    loose_files: Vec<PathBuf>,
}

/// A recorded file and the folder segments that hold it, the drive first.
struct RecordedEntry<'a> {
    file: &'a Path,
    folder: Vec<String>,
}

/// The install roots of the recorded files that are part of the app's footprint.
fn footprint_roots(
    files: &[PathBuf],
    names: &ProductNames,
    system: &SystemContext,
    is_cancelled: &dyn Fn() -> bool,
) -> InstallRoots {
    let windows_dir = system.windows_dir();
    let footprint: Vec<PathBuf> = files
        .iter()
        .filter(|path| path.is_absolute() && is_app_footprint_path(path, &windows_dir))
        .cloned()
        .collect();
    let mut roots = minimal_install_roots(&footprint, names);
    // Each of these asks the disk, and a record can name any number. Whatever is left of it when a
    // person stops waiting has nothing to go by, and stopping is all that comes of it.
    roots
        .loose_files
        .retain(|path| !is_cancelled() && path.is_file());
    roots
}

/// Reduce recorded files to the deepest folders that hold them. Files under one folder
/// share a root, so every file under `C:\Program Files\Inkscape\...` gives that folder,
/// but the root stops short of anything shared: `C:\ProgramData\Microsoft` is not an
/// app's root because one of its files is under it, and neither is a vendor's folder that
/// may hold its other products. Those split into one root per child. A folder directly on
/// a drive, like `C:\CocosDashboard`, is a root like any other.
fn minimal_install_roots(files: &[PathBuf], names: &ProductNames) -> InstallRoots {
    let mut entries: Vec<RecordedEntry<'_>> = files
        .iter()
        .map(|file| RecordedEntry {
            file,
            folder: recorded_folder_segments(file),
        })
        .collect();
    // An MSI component keyed on a folder says the app uses that folder, not that it owns
    // everything in it: the ODBC driver keys on `Microsoft SQL Server\`. It only names a
    // root when nothing else is recorded under it.
    let mut ancestors = BTreeSet::new();
    for entry in &entries {
        for count in 1..entry.folder.len() {
            ancestors.insert(entry.folder[..count].join("\\").to_ascii_lowercase());
        }
    }
    entries.retain(|entry| {
        !entry.file.to_string_lossy().ends_with('\\')
            || !ancestors.contains(&entry.folder.join("\\").to_ascii_lowercase())
    });
    let mut roots = InstallRoots::default();
    cover_recorded_files(&entries.iter().collect::<Vec<_>>(), 0, names, &mut roots);
    roots
}

/// The folder a recorded path names: its parent, or itself when it ends in a separator,
/// as an MSI component that keys on a folder does.
fn recorded_folder_segments(path: &Path) -> Vec<String> {
    let text = path.to_string_lossy();
    let mut segments: Vec<String> = text.split('\\').map(ToOwned::to_owned).collect();
    segments.pop();
    segments
}

fn cover_recorded_files(
    entries: &[&RecordedEntry<'_>],
    known_shared: usize,
    names: &ProductNames,
    roots: &mut InstallRoots,
) {
    let Some(first) = entries.first() else {
        return;
    };
    let common = common_segment_count(entries, known_shared);
    let owner = &first.folder[..common];
    let has_direct_files = entries.iter().any(|entry| entry.folder.len() == common);
    let leaf = owner.last().map(String::as_str).unwrap_or_default();
    let is_shared = owner.len() < 2 || is_shared_segments(owner);
    // Below a folder that holds recorded files itself or is named for the product, the
    // app owns everything. A folder that only joins subfolders may be a family of
    // products: `Microsoft SQL Server` has the ODBC driver's subfolders and also LocalDB.
    let is_own_folder =
        has_direct_files || names.matches_top_level(leaf, MatchScope::ProgramFolders);
    if !is_shared && is_own_folder {
        let keep = owned_segment_count(owner, names);
        roots.dirs.push(PathBuf::from(owner[..keep].join("\\")));
        return;
    }
    let mut children: BTreeMap<String, Vec<&RecordedEntry<'_>>> = BTreeMap::new();
    for entry in entries {
        match entry.folder.get(common) {
            Some(child) => children
                .entry(child.to_ascii_lowercase())
                .or_default()
                .push(entry),
            None => roots.loose_files.push(entry.file.to_path_buf()),
        }
    }
    for group in children.values() {
        cover_recorded_files(group, common + 1, names, roots);
    }
}

/// How many leading folder segments all the entries share, at least `known`.
fn common_segment_count(entries: &[&RecordedEntry<'_>], known: usize) -> usize {
    let Some((first, rest)) = entries.split_first() else {
        return known;
    };
    let mut count = known;
    while let Some(segment) = first.folder.get(count) {
        if !rest.iter().all(|entry| {
            entry
                .folder
                .get(count)
                .is_some_and(|other| other.eq_ignore_ascii_case(segment))
        }) {
            break;
        }
        count += 1;
    }
    count.min(first.folder.len())
}

/// A root ending in a folder like `bin` is usually one folder too deep: moving up keeps
/// `Foo\lib` and `Foo\README` with `Foo\bin\foo.exe`. Not past a folder that might hold
/// other products.
fn owned_segment_count(owner: &[String], names: &ProductNames) -> usize {
    let count = owner.len();
    if count >= 3
        && is_executable_folder_name(&owner[count - 1])
        && !is_shared_segments(&owner[..count - 1])
        && !names.is_vendor_only_folder(&owner[count - 2])
    {
        count - 1
    } else {
        count
    }
}

fn is_shared_segments(segments: &[String]) -> bool {
    let Some((leaf, parents)) = segments.split_last() else {
        return true;
    };
    parents.is_empty()
        || parents
            .last()
            .is_some_and(|parent| parent.eq_ignore_ascii_case("users"))
        || SHARED_FOLDER_NAMES.contains(&leaf.to_ascii_lowercase().as_str())
}

/// True for a path that holds unrelated things side by side, or the drive itself.
pub(crate) fn is_shared_folder(path: &Path) -> bool {
    let segments: Vec<String> = path
        .to_string_lossy()
        .trim_end_matches('\\')
        .split('\\')
        .map(ToOwned::to_owned)
        .collect();
    is_shared_segments(&segments)
}

#[cfg(test)]
thread_local! {
    /// How many times this thread read the MSI component database.
    static MSI_SWEEPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squishes_known_product_guid() {
        let squished = squish_guid("{C653D4A8-392F-476F-A1B5-C64F217938D7}");
        assert_eq!(
            squished.as_deref(),
            Some("8A4D356CF293F6741A5B6CF41297837D")
        );
    }

    #[test]
    fn recognizes_guid_strings() {
        assert!(is_guid("{C653D4A8-392F-476F-A1B5-C64F217938D7}"));
        assert!(!is_guid("Inkscape"));
        assert!(!is_guid("{too-short}"));
    }

    #[test]
    fn finds_guid_in_uninstall_string() {
        let code = find_guid(r"MsiExec.exe /X{C653D4A8-392F-476F-A1B5-C64F217938D7}");
        assert_eq!(
            code.as_deref(),
            Some("{C653D4A8-392F-476F-A1B5-C64F217938D7}")
        );
    }

    #[test]
    fn a_non_ascii_uninstall_string_does_not_panic() {
        let value = format!("{{{}\u{e9}", "a".repeat(36));
        assert_eq!(find_guid(&value), None);
        assert_eq!(
            find_guid("D\u{e9}sinstaller /X{C653D4A8-392F-476F-A1B5-C64F217938D7}").as_deref(),
            Some("{C653D4A8-392F-476F-A1B5-C64F217938D7}")
        );
    }

    #[test]
    fn a_drive_is_never_an_install_root() {
        assert!(
            dedup_existing_dirs(vec![PathBuf::from(r"C:\")], &SystemContext::live()).is_empty()
        );

        let mut source = test_source(InstallMethod::Generic);
        source.roots = vec![PathBuf::from(r"C:\")];
        assert_eq!(
            resolve_install_location(
                &test_app("Nothing Like Its Folders", None),
                Path::new(r"C:\tool.exe"),
                &source,
            ),
            None
        );
    }

    #[test]
    fn detects_registry_key_paths() {
        assert!(is_registry_key_path(r"21:\Software\Inkscape\Inkscape"));
        assert!(is_registry_key_path(r"2:\Software\Demo"));
        assert!(!is_registry_key_path(
            r"C:\Program Files\Inkscape\bin\inkscape.exe"
        ));
    }

    #[test]
    fn recognizes_inno_log_names() {
        assert!(is_inno_log_name("unins000.dat"));
        assert!(is_inno_log_name("unins042.dat"));
        assert!(!is_inno_log_name("uninstall.exe"));
        assert!(!is_inno_log_name("unins000.exe"));
        assert!(!is_inno_log_name("uninsXYZ.dat"));
    }

    #[test]
    fn extracts_utf16_paths_from_log_bytes() {
        let mut data = b"Inno Setup Uninstall Log (b) 64-bit".to_vec();
        data.resize(64, 0);
        let path = r"C:\Program Files\Demo\app.exe";
        for unit in path.encode_utf16() {
            data.extend_from_slice(&unit.to_le_bytes());
        }
        data.extend_from_slice(&[0, 0]);

        let found = extract_utf16_paths(&data);
        assert!(
            found.iter().any(|candidate| candidate == path),
            "expected the embedded path, got {found:?}"
        );
    }

    #[test]
    fn excludes_shortcut_and_system_paths() {
        let windows = Path::new(r"C:\Windows");
        for kept in [
            r"C:\Program Files\Demo\app.exe",
            r"C:\Program Files\Demo\windows\helper.dll",
            r"C:\Program Files\GitHub Desktop\desktop\app.exe",
            r"C:\Users\me\AppData\Local\Microsoft\Windows Terminal\Fragments\Git\git-bash.json",
        ] {
            assert!(is_app_footprint_path(Path::new(kept), windows), "{kept}");
        }
        for dropped in [
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Demo\Demo.lnk",
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Demo\Demo.pif",
            r"C:\Users\Public\Desktop\Demo.pif",
            r"C:\Users\me\Desktop\notes.txt",
            r"C:\Windows\System32\demo.dll",
            r"C:\WINDOWS\Installer\{5B686938-99D9-4ED6-94E2-59E5CC7C3D94}\atom.exe",
        ] {
            assert!(
                !is_app_footprint_path(Path::new(dropped), windows),
                "{dropped}"
            );
        }
    }

    // ---- Inno Setup logs ----

    /// One record: a 16-bit type, 32-bit extra data, 32-bit size, then the data.
    fn inno_record(kind: u16, extra: u32, data: &[u8]) -> Vec<u8> {
        let mut record = kind.to_le_bytes().to_vec();
        record.extend_from_slice(&extra.to_le_bytes());
        record.extend_from_slice(&(data.len() as u32).to_le_bytes());
        record.extend_from_slice(data);
        record
    }

    /// The string form a Unicode log uses: 0xFE, a negative byte count, UTF-16 text.
    fn inno_unicode_strings(strings: &[&str]) -> Vec<u8> {
        let mut data = Vec::new();
        for text in strings {
            let bytes: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
            data.push(INNO_STRING_LEN_I32);
            data.extend_from_slice(&(-(bytes.len() as i32)).to_le_bytes());
            data.extend_from_slice(&bytes);
        }
        data.push(INNO_STRING_END);
        data
    }

    /// The string form an old ANSI log uses: a single length byte, then the text.
    fn inno_ansi_strings(strings: &[&str]) -> Vec<u8> {
        let mut data = Vec::new();
        for text in strings {
            data.push(text.len() as u8);
            data.extend_from_slice(text.as_bytes());
        }
        data.push(INNO_STRING_END);
        data
    }

    /// A whole log: the header, then `body` cut into CRC-framed blocks.
    fn inno_log(body: &[u8], block_size: usize) -> Vec<u8> {
        let mut log = b"Inno Setup Uninstall Log (b) 64-bit".to_vec();
        log.resize(INNO_HEADER_LEN, 0);
        for block in body.chunks(block_size) {
            log.extend_from_slice(&(block.len() as u32).to_le_bytes());
            log.extend_from_slice(&(!(block.len() as u32)).to_le_bytes());
            log.extend_from_slice(&[0; 4]);
            log.extend_from_slice(block);
        }
        let end = log.len() as u32;
        log[INNO_END_OFFSET_FIELD..INNO_END_OFFSET_FIELD + 4].copy_from_slice(&end.to_le_bytes());
        log
    }

    #[test]
    fn parses_unicode_inno_logs_past_the_bytes_that_follow_each_path() {
        // The real Git for Windows failure: a path's end marker 0xFF and the next record's
        // type 0x82 read as one UTF-16 unit, U+82FF, a CJK letter that looks like path text.
        let first = r"C:\Program Files\Git\etc\package-versions.txt";
        let second = r"C:\Program Files\Git\git-bash.exe";
        let mut body = inno_record(
            0x10,
            0,
            &inno_unicode_strings(&["PC", "me", r"C:\Program Files\Git"]),
        );
        body.extend(inno_record(
            INNO_RECORD_DELETE_FILE,
            0x800,
            &inno_unicode_strings(&[first]),
        ));
        body.extend(inno_record(
            INNO_RECORD_DELETE_FILE,
            0x800,
            &inno_unicode_strings(&[second]),
        ));
        let log = inno_log(&body, 4096);

        let scanned = extract_utf16_paths(&log);
        assert!(
            !scanned.iter().any(|path| path == first),
            "the fixture must reproduce the corruption, got {scanned:?}"
        );

        let Some(parsed) = parse_inno_log(&log) else {
            panic!("a framed log should parse");
        };
        assert_eq!(parsed.files, vec![first.to_owned(), second.to_owned()]);
    }

    #[test]
    fn parses_old_ansi_inno_logs() {
        // 0x2E is both a valid length and a path character, so a following record
        // would corrupt a scanned path here too.
        let path = r"C:\Program Files (x86)\TouchOSC Bridge\Bridge.exe";
        let mut body = inno_record(
            INNO_RECORD_DELETE_FILE,
            0,
            &inno_ansi_strings(&[path, "", "", ""]),
        );
        body.extend(inno_record(
            INNO_RECORD_DELETE_FILE,
            0,
            &inno_ansi_strings(&[r"C:\Program Files (x86)\TouchOSC Bridge\readme.txt"]),
        ));
        let Some(parsed) = parse_inno_log(&inno_log(&body, 4096)) else {
            panic!("a framed ANSI log should parse");
        };
        assert_eq!(parsed.files[0], path);
        assert_eq!(parsed.files.len(), 2);
    }

    #[test]
    fn inno_records_may_straddle_blocks() {
        let path = r"C:\Program Files\Demo\a-file-with-a-rather-long-name.dll";
        let body = inno_record(INNO_RECORD_DELETE_FILE, 0, &inno_unicode_strings(&[path]));
        let Some(parsed) = parse_inno_log(&inno_log(&body, 16)) else {
            panic!("a log cut into small blocks should parse");
        };
        assert_eq!(parsed.files, vec![path.to_owned()]);
    }

    #[test]
    fn inno_long_paths_lose_their_prefix_and_repeats_are_dropped() {
        // GIMP records \\?\C:\..., and every update of an app records its files again.
        let mut body = Vec::new();
        for _ in 0..2 {
            body.extend(inno_record(
                INNO_RECORD_DELETE_FILE,
                0x80,
                &inno_unicode_strings(&[r"\\?\C:\Program Files\GIMP 3\bin\gimp.exe"]),
            ));
        }
        let Some(parsed) = parse_inno_log(&inno_log(&body, 4096)) else {
            panic!("the log should parse");
        };
        assert_eq!(
            parsed.files,
            vec![r"C:\Program Files\GIMP 3\bin\gimp.exe".to_owned()]
        );
    }

    #[test]
    fn inno_logs_record_registry_keys_with_their_root_and_view() {
        let mut body = inno_record(
            INNO_RECORD_REG_DELETE_KEY_IF_EMPTY,
            0x8100_0002,
            &inno_unicode_strings(&[r"Software\GitForWindows"]),
        );
        body.extend(inno_record(
            INNO_RECORD_REG_DELETE_VALUE,
            0x8100_0001,
            &inno_unicode_strings(&[r"Software\RegisteredApplications", "GIMP 3"]),
        ));
        body.extend(inno_record(
            INNO_RECORD_REG_DELETE_ENTIRE_KEY,
            0x8000_0000,
            &inno_ansi_strings(&[r"*\shell\HxD"]),
        ));
        body.extend(inno_record(
            INNO_RECORD_REG_DELETE_ENTIRE_KEY,
            0x8000_0002,
            &inno_ansi_strings(&[r"Software\Tickr"]),
        ));
        let Some(parsed) = parse_inno_log(&inno_log(&body, 4096)) else {
            panic!("the log should parse");
        };
        assert!(parsed.files.is_empty());
        assert_eq!(
            parsed.registry_keys,
            vec![
                RecordedRegistryKey {
                    root: RecordedRegistryRoot::LocalMachine,
                    key_path: r"Software\GitForWindows".to_owned(),
                    value_name: None,
                    wow64_32: false,
                },
                RecordedRegistryKey {
                    root: RecordedRegistryRoot::CurrentUser,
                    key_path: r"Software\RegisteredApplications".to_owned(),
                    value_name: Some("GIMP 3".to_owned()),
                    wow64_32: false,
                },
                RecordedRegistryKey {
                    root: RecordedRegistryRoot::ClassesRoot,
                    key_path: r"*\shell\HxD".to_owned(),
                    value_name: None,
                    wow64_32: true,
                },
                RecordedRegistryKey {
                    root: RecordedRegistryRoot::LocalMachine,
                    key_path: r"Software\Tickr".to_owned(),
                    value_name: None,
                    wow64_32: true,
                },
            ]
        );
    }

    #[test]
    fn an_inno_record_naming_no_key_is_ignored() {
        // Trimmed, `\` would mean the root of the hive, and capturing that takes everything.
        assert_eq!(inno_registry_key(0x8100_0002, "\\", None), None);
        assert_eq!(inno_registry_key(0x8100_0002, "", None), None);
        assert_eq!(inno_registry_key(0x8100_0003, r"S-1-5-18\Foo", None), None);
    }

    #[test]
    fn unrecognized_inno_framing_is_not_parsed() {
        let mut log = b"Inno Setup Uninstall Log (b)".to_vec();
        log.resize(INNO_HEADER_LEN + 64, 0xAB);
        assert!(parse_inno_log(&log).is_none());
        assert!(parse_inno_log(b"short").is_none());
    }

    #[test]
    fn truncated_inno_records_do_not_panic() {
        let body = inno_record(
            INNO_RECORD_DELETE_FILE,
            0,
            &inno_unicode_strings(&[r"C:\Program Files\Demo\app.exe"]),
        );
        for cut in 0..body.len() {
            let _ = parse_inno_log(&inno_log(&body[..cut], 4096));
            let _ = inno_strings(&body[..cut]);
        }
        assert!(inno_strings(&[INNO_STRING_LEN_I32, 0xFF, 0xFF, 0xFF, 0x7F, 0]).is_empty());
    }

    #[test]
    fn inno_manifest_keeps_only_files_that_exist()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-inno-test");
        std::fs::create_dir_all(&base)?;
        let installed = base.join("app.exe");
        std::fs::write(&installed, b"binary")?;
        let stale = base.join("removed-by-an-update.dll");
        let installed_text = installed.display().to_string();
        let stale_text = stale.display().to_string();
        let mut body = inno_record(
            INNO_RECORD_DELETE_FILE,
            0,
            &inno_unicode_strings(&[installed_text.as_str()]),
        );
        body.extend(inno_record(
            INNO_RECORD_DELETE_FILE,
            0,
            &inno_unicode_strings(&[stale_text.as_str()]),
        ));
        let log_path = base.join("unins000.dat");
        std::fs::write(&log_path, inno_log(&body, 4096))?;

        let manifest = inno_manifest(&log_path, &SystemContext::live());
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(manifest.files, vec![installed]);
        Ok(())
    }

    #[test]
    fn finds_an_inno_log_beside_the_uninstaller_when_it_is_not_in_the_install_folder()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-inno-uninst");
        let uninst = base.join("uninst");
        std::fs::create_dir_all(&uninst)?;
        std::fs::write(uninst.join("unins000.dat"), b"log")?;
        let mut app = test_app("GIMP 3.2.6", None);
        app.install_location = Some(base.clone());
        app.uninstall_string = Some(format!(
            "\"{}\" /SILENT",
            uninst.join("unins000.exe").display()
        ));

        let found = inno_log_for(&app, &SystemContext::live());
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found, Some(uninst.join("unins000.dat")));
        Ok(())
    }

    // ---- The main executable ----

    fn test_app(display_name: &str, publisher: Option<&str>) -> InstalledApp {
        InstalledApp {
            display_name: display_name.to_owned(),
            publisher: publisher.map(ToOwned::to_owned),
            version: None,
            install_location: None,
            uninstall_string: None,
            main_executable: None,
            registry_source_path: r"HKLM\Software\Test".to_owned(),
            architecture: crate::models::AppArchitecture::X64,
            install_scope: crate::models::InstallScope::PerMachine,
            owner_profile: None,
        }
    }

    fn folder_of_an_uninstaller(
        name: &str,
        publisher: &str,
        location: Option<&str>,
        program: Option<&str>,
        folder: &str,
    ) -> bool {
        let mut app = test_app(name, Some(publisher));
        app.install_location = location.map(PathBuf::from);
        app.main_executable = program.map(PathBuf::from);
        uninstaller_folder_is_the_apps(
            &app,
            Path::new(folder),
            &app_product_names(&app),
            &SystemContext::live(),
        )
    }

    /// `Content` is beside `Binaries`, and an app with no location on record would lose it.
    #[test]
    fn the_folder_an_uninstaller_is_in_is_the_apps_if_it_holds_the_program() {
        assert!(folder_of_an_uninstaller(
            "My Game: Deluxe Edition",
            "Indie",
            None,
            Some(r"D:\Games\MyGame\Binaries\Win64\game.exe"),
            r"D:\Games\MyGame",
        ));
        assert!(folder_of_an_uninstaller(
            "Awesome Chat",
            "Acme",
            None,
            Some(r"C:\Users\a\AppData\Local\achat\app-3.0\achat.exe"),
            r"C:\Users\a\AppData\Local\achat",
        ));
    }

    /// A vendor's folder can hold all it makes, and a person's holds anything.
    #[test]
    fn the_folder_an_uninstaller_is_in_is_not_the_apps_if_it_is_a_vendors_or_a_persons() {
        assert!(!folder_of_an_uninstaller(
            "Acme Widget",
            "Acme Corp",
            None,
            Some(r"C:\Program Files\Acme Corp\Widget\widget.exe"),
            r"C:\Program Files\Acme Corp",
        ));
        for folder in [r"D:\Downloads", r"C:\Users\a\Desktop", r"E:\Temp"] {
            assert!(
                !folder_of_an_uninstaller(
                    "Widget Maker",
                    "Acme",
                    None,
                    Some(&format!(r"{folder}\Widget\widget.exe")),
                    folder,
                ),
                "{folder}"
            );
        }
        assert!(!folder_of_an_uninstaller(
            "Widget Maker",
            "Acme",
            None,
            Some(r"C:\Program Files\Widget\widget.exe"),
            r"D:\Downloads",
        ));
    }

    /// One named for itself is still its own, whoever published it.
    #[test]
    fn an_app_named_as_its_publisher_keeps_its_folder() {
        assert!(folder_of_an_uninstaller(
            "Notepad++",
            "Notepad++",
            None,
            Some(r"C:\Program Files\Notepad++\notepad++.exe"),
            r"C:\Program Files\Notepad++",
        ));
        assert!(folder_of_an_uninstaller(
            "Acme Widget",
            "Acme Corp",
            Some(r"C:\Program Files\Acme Corp"),
            None,
            r"C:\Program Files\Acme Corp",
        ));
    }

    /// A setup run from someone's Downloads is theirs, on whatever drive that is. One in a program
    /// folder, or in a cache an installer keeps in `ProgramData`, is what removes the app.
    #[test]
    fn an_uninstaller_alone_comes_only_from_where_programs_are_put() {
        let system = SystemContext::live();
        assert!(!is_where_programs_are_put(
            Path::new(r"E:\alice\Downloads"),
            &system
        ));
        assert!(!is_where_programs_are_put(
            Path::new(r"C:\Users\alice\Downloads"),
            &system
        ));
        for base in system.program_files_dirs() {
            assert!(is_where_programs_are_put(&base.join("Acme"), &system));
        }
        if let Some(data) = system.program_data() {
            assert!(is_where_programs_are_put(&data.join("{GUID}"), &system));
        }
    }

    fn test_source(method: InstallMethod) -> InstallSource {
        InstallSource {
            uninstaller_folder: None,
            folders_left_alone: Vec::new(),
            method,
            product_code: None,
            authoritative_file_count: 0,
            roots: Vec::new(),
            loose_files: Vec::new(),
            recorded_files: Vec::new(),
            registry_keys: Vec::new(),
            evidence: Vec::new(),
        }
    }

    #[test]
    fn choosing_the_executable_never_reads_the_msi_database_again()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-msi-once");
        std::fs::create_dir_all(&base)?;
        let exe = base.join("wam-msi-once-demo.exe");
        std::fs::write(&exe, b"MZ")?;
        let mut source = test_source(InstallMethod::WindowsInstaller);
        source.product_code = Some("{1D1D1768-10D8-3DFF-9AD9-1E26B6323A29}".to_owned());
        source.recorded_files = vec![exe.clone()];

        let before = MSI_SWEEPS.with(std::cell::Cell::get);
        let found = primary_executable(
            &test_app("WAM MSI Once Demo", None),
            &source,
            &SystemContext::live(),
        );
        let after = MSI_SWEEPS.with(std::cell::Cell::get);
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found, Some(exe));
        assert_eq!(after, before, "the sweep must not run again");
        Ok(())
    }

    #[test]
    fn prefers_the_executable_named_for_the_product()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-primary");
        std::fs::create_dir_all(&base)?;
        for name in ["aaa-big-tool.exe", "demo.exe"] {
            std::fs::write(base.join(name), b"MZ")?;
        }
        std::fs::write(base.join("zzz-bigger.exe"), vec![0_u8; 4096])?;
        std::fs::create_dir_all(base.join("packs"))?;
        std::fs::write(base.join(r"packs\biggest.exe"), vec![0_u8; 8192])?;
        let names = ProductNames::new(Some("Demo"), None, &[], &[]);
        let candidates = [
            "aaa-big-tool.exe",
            "demo.exe",
            "zzz-bigger.exe",
            r"packs\biggest.exe",
        ]
        .map(|name| base.join(name))
        .to_vec();

        let named = pick_primary_executable(candidates.clone(), &names, "Demo");
        let unnamed = pick_primary_executable(
            candidates,
            &ProductNames::new(Some("Other"), None, &[], &[]),
            "Other",
        );
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(named, Some(base.join("demo.exe")));
        assert_eq!(
            unnamed,
            Some(base.join("zzz-bigger.exe")),
            "else the largest of the shallowest"
        );
        Ok(())
    }

    #[test]
    fn rejects_uninstallers_windows_hosts_and_installer_caches()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let system = SystemContext::live();
        let host = system.windows_dir().join(r"System32\rundll32.exe");
        assert!(host.is_file(), "every Windows has rundll32");
        assert!(!is_usable_main_executable(&host, &system));

        let base = unique_temp("wam-usable");
        let cache = base.join("Package Cache").join("{guid}");
        std::fs::create_dir_all(&cache)?;
        for path in [
            base.join("app.exe"),
            base.join("unins000.exe"),
            cache.join("bootstrap.exe"),
        ] {
            std::fs::write(path, b"MZ")?;
        }
        let app = is_usable_main_executable(&base.join("app.exe"), &system);
        let uninstaller = is_usable_main_executable(&base.join("unins000.exe"), &system);
        let cached = is_usable_main_executable(&cache.join("bootstrap.exe"), &system);
        let missing = is_usable_main_executable(&base.join("missing.exe"), &system);
        let _ = std::fs::remove_dir_all(&base);

        assert!(app);
        assert!(!uninstaller);
        assert!(!cached);
        assert!(!missing);
        Ok(())
    }

    #[test]
    fn hints_find_an_app_with_no_install_location_from_app_paths_or_shortcuts()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Chrome's MSI records no files and its uninstall entry names no location: only
        // App Paths and a shortcut say where it is.
        let base = unique_temp("wam-hints");
        std::fs::create_dir_all(&base)?;
        for name in [
            "chrome.exe",
            "chrome_proxy.exe",
            "nvapp.exe",
            "unins000.exe",
        ] {
            std::fs::write(base.join(name), b"MZ")?;
        }
        let system = SystemContext::live();
        let hints = ExecutableHints::keeping_usable(
            vec![
                base.join("chrome_proxy.exe"),
                base.join("chrome.exe"),
                base.join("no-longer-installed.exe"),
            ],
            vec![
                ShortcutHint {
                    name: "NVIDIA App Uninstall".to_owned(),
                    target: base.join("unins000.exe"),
                },
                ShortcutHint {
                    name: "NVIDIA App".to_owned(),
                    target: base.join("nvapp.exe"),
                },
            ],
            &system,
        );

        let chrome = ProductNames::new(Some("Google Chrome"), Some("Google LLC"), &[], &[]);
        let nvidia = ProductNames::new(
            Some("NVIDIA App 11.0"),
            Some("NVIDIA Corporation"),
            &[],
            &[],
        );
        let other = ProductNames::new(Some("Unrelated"), Some("Nobody"), &[], &[]);
        let found_chrome = hints.find(&chrome, None);
        let found_nvidia = hints.find(&nvidia, None);
        let found_other = hints.find(&other, None);
        let found_inside = hints.find(&other, Some(&base));
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found_chrome, Some(base.join("chrome.exe")));
        assert_eq!(found_nvidia, Some(base.join("nvapp.exe")));
        assert_eq!(found_other, None);
        assert_eq!(
            found_inside,
            Some(base.join("chrome_proxy.exe")),
            "anything inside the install folder beats nothing"
        );
        Ok(())
    }

    #[test]
    fn hints_skip_a_store_app_with_the_same_name()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-store");
        let store = base.join("WindowsApps").join("Microsoft.Notepad");
        std::fs::create_dir_all(&store)?;
        std::fs::write(store.join("notepad.exe"), b"MZ")?;
        let hints = ExecutableHints::keeping_usable(
            vec![store.join("notepad.exe")],
            Vec::new(),
            &SystemContext::live(),
        );
        let names = ProductNames::new(Some("Notepad++"), Some("Notepad++ Team"), &[], &[]);

        let found = hints.find(&names, None);
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found, None);
        Ok(())
    }

    #[test]
    fn a_squirrel_stub_gives_way_to_the_current_version()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-squirrel-exe");
        for version in ["app-3.5.9", "app-3.5.11"] {
            std::fs::create_dir_all(base.join(version))?;
            std::fs::write(base.join(version).join("Demo.exe"), b"MZ")?;
        }
        let stub = base.join("Demo.exe");
        std::fs::write(&stub, b"MZ")?;

        let system = SystemContext::live();
        let squirrel =
            squirrel_current_executable(&stub, &test_source(InstallMethod::Squirrel), &system);
        let other =
            squirrel_current_executable(&stub, &test_source(InstallMethod::Generic), &system);
        let unmatched = squirrel_current_executable(
            &base.join("Other.exe"),
            &test_source(InstallMethod::Squirrel),
            &system,
        );
        let expected = base.join("app-3.5.11").join("Demo.exe");
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(squirrel, Some(expected));
        assert_eq!(other, None);
        assert_eq!(unmatched, None);
        Ok(())
    }

    #[test]
    fn hints_prefer_the_shortcut_for_the_same_version()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-versions");
        for version in ["313", "314"] {
            std::fs::create_dir_all(base.join(version))?;
            std::fs::write(base.join(version).join("python.exe"), b"MZ")?;
        }
        let hints = ExecutableHints::keeping_usable(
            vec![base.join("313").join("python.exe")],
            vec![
                ShortcutHint {
                    name: "Python 3.13 (64-bit)".to_owned(),
                    target: base.join("313").join("python.exe"),
                },
                ShortcutHint {
                    name: "Python 3.14 (64-bit)".to_owned(),
                    target: base.join("314").join("python.exe"),
                },
            ],
            &SystemContext::live(),
        );
        let python = ProductNames::new(
            Some("Python 3.14.10 (64-bit)"),
            Some("Python Software Foundation"),
            &[],
            &[],
        );

        let found = hints.find(&python, None);
        let expected = base.join("314").join("python.exe");
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found, Some(expected));
        Ok(())
    }

    #[test]
    fn a_runtime_chained_by_a_bundle_stays_out_unless_the_app_is_that_runtime() {
        let app = "Moonlight Game Streaming Client";
        assert!(is_chained_runtime(
            "Microsoft .NET Runtime - 8.0.1 (x64)",
            app
        ));
        assert!(is_chained_runtime(
            "Microsoft Visual C++ 2015-2022 Redistributable",
            app
        ));
        assert!(!is_chained_runtime("Moonlight Game Streaming Client", app));
        assert!(!is_chained_runtime(
            "Microsoft .NET Runtime - 10.0.14 (x64)",
            "Microsoft .NET SDK 10.0.403 (x64)"
        ));
    }

    #[test]
    fn an_uninstaller_is_only_a_last_resort_and_a_windows_host_never_is()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let system = SystemContext::live();
        let base = unique_temp("wam-last-resort");
        std::fs::create_dir_all(&base)?;
        let uninstaller = base.join("unins000.exe");
        std::fs::write(&uninstaller, b"MZ")?;

        let mut font_pack = test_app("WAM Last Resort Font Pack", None);
        font_pack.install_location = Some(base.clone());
        let found = primary_executable(&font_pack, &test_source(InstallMethod::Generic), &system);

        let mut hosted = test_app("WAM Last Resort Hosted", None);
        hosted.uninstall_string = Some(format!(
            "\"{}\" \"C:\\Program Files\\Vendor\\Core.dll\",Uninstall",
            system
                .windows_dir()
                .join(r"System32\rundll32.exe")
                .display()
        ));
        let hosted_found =
            primary_executable(&hosted, &test_source(InstallMethod::Generic), &system);
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found, Some(uninstaller));
        assert_eq!(hosted_found, None);
        Ok(())
    }

    #[test]
    fn an_executable_the_install_record_puts_in_the_windows_directory_counts() {
        // The Python launcher's py.exe lives in C:\Windows, and its MSI says so.
        let system = SystemContext::live();
        let notepad = system.windows_dir().join("notepad.exe");
        assert!(notepad.is_file(), "every Windows has notepad");
        let mut source = test_source(InstallMethod::WindowsInstaller);
        source.recorded_files = vec![notepad.clone()];

        assert_eq!(
            primary_executable(&test_app("WAM Recorded Launcher", None), &source, &system),
            Some(notepad)
        );
    }

    #[test]
    fn install_location_is_the_recorded_one_when_it_holds_the_executable()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-location");
        let location = base.join("Foo Suite");
        std::fs::create_dir_all(location.join("bin"))?;
        let mut app = test_app("Foo", None);
        app.install_location = Some(location.clone());

        let resolved = resolve_install_location(
            &app,
            &location.join(r"bin\foo.exe"),
            &test_source(InstallMethod::Generic),
        );
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(resolved, Some(location));
        Ok(())
    }

    #[test]
    fn install_location_climbs_to_the_product_folder_above_the_executable()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let source = test_source(InstallMethod::WindowsInstaller);
        let chrome = test_app("Google Chrome", Some("Google LLC"));
        assert_eq!(
            resolve_install_location(
                &chrome,
                Path::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
                &source,
            ),
            Some(PathBuf::from(r"C:\Program Files\Google\Chrome"))
        );

        let powershell = test_app("PowerShell 7.6.9.0-x64", Some("Microsoft Corporation"));
        assert_eq!(
            resolve_install_location(
                &powershell,
                Path::new(r"C:\Program Files\PowerShell\7\pwsh.exe"),
                &source,
            ),
            Some(PathBuf::from(r"C:\Program Files\PowerShell"))
        );

        // NVIDIA App's recorded location is an installer cache that does not hold it.
        let base = unique_temp("wam-nvidia-cache");
        std::fs::create_dir_all(&base)?;
        let mut nvidia = test_app("NVIDIA App 11.0.4.526", Some("NVIDIA Corporation"));
        nvidia.install_location = Some(base.clone());
        let resolved = resolve_install_location(
            &nvidia,
            Path::new(r"C:\Program Files\NVIDIA Corporation\NVIDIA app\CEF\NVIDIA App.exe"),
            &source,
        );
        let _ = std::fs::remove_dir_all(&base);
        assert_eq!(
            resolved,
            Some(PathBuf::from(
                r"C:\Program Files\NVIDIA Corporation\NVIDIA app"
            ))
        );
        Ok(())
    }

    #[test]
    fn an_executable_in_a_shared_folder_has_no_install_location() {
        let source = test_source(InstallMethod::WindowsInstaller);
        let launcher = test_app("Python Launcher", Some("Python Software Foundation"));
        assert_eq!(
            resolve_install_location(&launcher, Path::new(r"C:\Windows\py.exe"), &source),
            None,
            "the Windows directory would claim every service and registry value in it"
        );
    }

    #[test]
    fn install_location_stops_at_shared_folders_and_moves_up_from_generic_ones() {
        let source = test_source(InstallMethod::Generic);
        let app = test_app("Nothing Like Its Folders", None);
        assert_eq!(
            resolve_install_location(
                &app,
                Path::new(r"C:\Program Files\Acme Toolkit\bin\tool.exe"),
                &source,
            ),
            Some(PathBuf::from(r"C:\Program Files\Acme Toolkit")),
            "bin is one level too deep"
        );
        assert_eq!(
            resolve_install_location(&app, Path::new(r"C:\Program Files\bin\tool.exe"), &source),
            Some(PathBuf::from(r"C:\Program Files\bin")),
            "never up to a shared folder"
        );
        // A product named like a shared folder must not claim it.
        let programs = test_app("Programs", None);
        assert_eq!(
            resolve_install_location(
                &programs,
                Path::new(r"C:\Users\me\AppData\Local\Programs\Tool\tool.exe"),
                &source,
            ),
            Some(PathBuf::from(r"C:\Users\me\AppData\Local\Programs\Tool"))
        );
    }

    // ---- MSI registry key paths ----

    #[test]
    fn parses_msi_registry_key_paths() {
        assert_eq!(
            msi_registry_key(r"22:\SOFTWARE\GitHub\CLI\InstallDir"),
            Some(RecordedRegistryKey {
                root: RecordedRegistryRoot::LocalMachine,
                key_path: r"SOFTWARE\GitHub\CLI".to_owned(),
                value_name: Some("InstallDir".to_owned()),
                wow64_32: false,
            })
        );
        assert_eq!(
            msi_registry_key(r"02:\Software\Xiamen Yaji Software Co., Ltd.\Cocos Dashboard\"),
            Some(RecordedRegistryKey {
                root: RecordedRegistryRoot::LocalMachine,
                key_path: r"Software\Xiamen Yaji Software Co., Ltd.\Cocos Dashboard".to_owned(),
                value_name: None,
                wow64_32: true,
            })
        );
        assert_eq!(
            msi_registry_key(r"20:\Directory\Background\shell\PowerShell7x64\Icon"),
            Some(RecordedRegistryKey {
                root: RecordedRegistryRoot::ClassesRoot,
                key_path: r"Directory\Background\shell\PowerShell7x64".to_owned(),
                value_name: Some("Icon".to_owned()),
                wow64_32: false,
            })
        );
        assert_eq!(
            msi_registry_key(r"1:\Software\Demo\Value"),
            Some(RecordedRegistryKey {
                root: RecordedRegistryRoot::CurrentUser,
                key_path: r"Software\Demo".to_owned(),
                value_name: Some("Value".to_owned()),
                wow64_32: true,
            })
        );
        assert_eq!(msi_registry_key(r"03:\S-1-5-21\Software\Demo"), None);
        assert_eq!(msi_registry_key(r"C:\Program Files\Demo\app.exe"), None);
    }

    fn roots_of(files: &[&str], publisher: Option<&str>, display: &str) -> InstallRoots {
        let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        let names = ProductNames::new(Some(display), publisher, &[], &[]);
        let mut roots = minimal_install_roots(&files, &names);
        roots.dirs.sort();
        roots.loose_files.sort();
        roots
    }

    fn dirs(paths: &[&str]) -> Vec<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn collapses_nested_install_roots() {
        let roots = roots_of(
            &[
                r"C:\Program Files\Inkscape\bin\inkscape.exe",
                r"C:\Program Files\Inkscape\share\icons\a.svg",
                r"C:\ProgramData\Inkscape\config.ini",
            ],
            Some("Inkscape Project"),
            "Inkscape",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[r"C:\Program Files\Inkscape", r"C:\ProgramData\Inkscape"])
        );
        assert!(roots.loose_files.is_empty());
    }

    #[test]
    fn common_files_root_is_vendor_scoped_not_the_whole_shared_dir() {
        // A vendor file under the shared Common Files tree must resolve to the
        // vendor sub-directory, never the multi-vendor Common Files root.
        let roots = roots_of(
            &[
                r"C:\Program Files (x86)\Common Files\Esoteric Technologies\shared.dll",
                r"C:\Program Files (x86)\SolarFire9\SolarFire.exe",
            ],
            None,
            "Solar Fire 9",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[
                r"C:\Program Files (x86)\Common Files\Esoteric Technologies",
                r"C:\Program Files (x86)\SolarFire9",
            ])
        );
    }

    #[test]
    fn a_file_under_a_shared_folder_never_claims_the_folder() {
        // Cocos Dashboard recorded a file under C:\ProgramData\Microsoft and took all
        // 10.67 GB of it.
        let roots = roots_of(
            &[
                r"C:\CocosDashboard\CocosDashboard.exe",
                r"C:\CocosDashboard\resources\app.asar",
                r"C:\ProgramData\Microsoft\Cocos\state.dat",
                r"C:\ProgramData\Microsoft\Other\more.dat",
            ],
            Some("Xiamen Yaji Software Co., Ltd."),
            "Cocos Dashboard",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[
                r"C:\CocosDashboard",
                r"C:\ProgramData\Microsoft\Cocos",
                r"C:\ProgramData\Microsoft\Other",
            ])
        );
    }

    #[test]
    fn a_folder_directly_on_a_drive_is_a_root_and_the_drive_is_not() {
        let roots = roots_of(
            &[
                r"C:\CocosDashboard\ffmpeg.dll",
                r"C:\CocosDashboard\locales\am.pak",
                r"C:\stray.ini",
            ],
            None,
            "Cocos Dashboard",
        );
        assert_eq!(roots.dirs, dirs(&[r"C:\CocosDashboard"]));
        assert_eq!(roots.loose_files, dirs(&[r"C:\stray.ini"]));
    }

    #[test]
    fn a_file_directly_in_a_shared_folder_is_kept_by_itself() {
        let roots = roots_of(
            &[
                r"C:\Program Files\Common Files\demo.dll",
                r"C:\Program Files\Demo\demo.exe",
            ],
            None,
            "Demo",
        );
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\Demo"]));
        assert_eq!(
            roots.loose_files,
            dirs(&[r"C:\Program Files\Common Files\demo.dll"])
        );
    }

    #[test]
    fn a_folder_that_only_joins_subfolders_is_not_claimed_whole() {
        // The ODBC driver's files share `Microsoft SQL Server` with LocalDB.
        let roots = roots_of(
            &[
                r"C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\odbc.dll",
                r"C:\Program Files\Microsoft SQL Server\170\Shared\x.dll",
            ],
            Some("Microsoft Corporation"),
            "Microsoft ODBC Driver 17 for SQL Server",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[
                r"C:\Program Files\Microsoft SQL Server\170\Shared",
                r"C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170",
            ])
        );

        // Named for the product, the same shape is the app's own folder.
        let roots = roots_of(
            &[
                r"C:\Program Files\Inkscape\share\a.svg",
                r"C:\Program Files\Inkscape\lib\b.dll",
            ],
            None,
            "Inkscape",
        );
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\Inkscape"]));
    }

    #[test]
    fn a_vendor_folder_splits_into_its_recorded_products() {
        let roots = roots_of(
            &[
                r"C:\Program Files\NVIDIA Corporation\NvContainer\a.dll",
                r"C:\Program Files\NVIDIA Corporation\NVIDIA app\b.exe",
            ],
            Some("NVIDIA Corporation"),
            "NVIDIA App 11.0.4.526",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[
                r"C:\Program Files\NVIDIA Corporation\NVIDIA app",
                r"C:\Program Files\NVIDIA Corporation\NvContainer",
            ])
        );
    }

    #[test]
    fn a_vendor_folder_holding_recorded_files_directly_is_the_apps_own() {
        let roots = roots_of(
            &[
                r"C:\Program Files\Acme\bar.exe",
                r"C:\Program Files\Acme\plugins\p.dll",
            ],
            Some("Acme Corp"),
            "Bar",
        );
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\Acme"]));
    }

    #[test]
    fn the_per_user_programs_folder_is_not_a_root() {
        let roots = roots_of(
            &[
                r"C:\Users\me\AppData\Local\Programs\Microsoft VS Code\Code.exe",
                r"C:\Users\me\AppData\Local\Programs\Microsoft VS Code\2c33b50b16\resources\app\main.js",
            ],
            Some("Microsoft Corporation"),
            "Microsoft Visual Studio Code (User)",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[r"C:\Users\me\AppData\Local\Programs\Microsoft VS Code"])
        );
    }

    #[test]
    fn a_root_moves_up_from_a_generic_folder_but_not_into_a_shared_one() {
        let roots = roots_of(&[r"C:\Program Files\Foo\bin\foo.exe"], None, "Foo");
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\Foo"]));

        let roots = roots_of(&[r"C:\Program Files\bin\foo.exe"], None, "Foo");
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\bin"]));

        let roots = roots_of(
            &[r"C:\Program Files\Acme\bin\foo.exe"],
            Some("Acme Corp"),
            "Foo",
        );
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\Acme\bin"]));
    }

    #[test]
    fn an_msi_component_keyed_on_a_folder_names_that_folder() {
        let roots = roots_of(&[r"C:\Program Files\PowerShell\7\"], None, "PowerShell 7");
        assert_eq!(roots.dirs, dirs(&[r"C:\Program Files\PowerShell\7"]));
    }

    #[test]
    fn a_folder_key_above_recorded_files_does_not_claim_the_folder() {
        let roots = roots_of(
            &[
                r"C:\Program Files\Microsoft SQL Server\",
                r"C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\odbc.dll",
            ],
            Some("Microsoft Corporation"),
            "Microsoft ODBC Driver 17 for SQL Server",
        );
        assert_eq!(
            roots.dirs,
            dirs(&[r"C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170"])
        );
    }

    #[test]
    fn a_reference_counted_msi_path_loses_its_marker() {
        assert_eq!(
            msi_file_path(r"C:\?WINDOWS\system32\adal.dll"),
            r"C:\WINDOWS\system32\adal.dll"
        );
        assert_eq!(
            msi_file_path(r"C:\Program Files\Demo\app.exe"),
            r"C:\Program Files\Demo\app.exe"
        );
    }

    #[test]
    fn files_on_several_drives_keep_separate_roots() {
        let roots = roots_of(&[r"C:\Tools\a.exe", r"D:\Data\b.db"], None, "Demo");
        assert_eq!(roots.dirs, dirs(&[r"C:\Tools", r"D:\Data"]));
    }

    #[test]
    fn recognizes_nsis_log_name() {
        assert!(is_nsis_log_name("uninstall.log"));
        assert!(!is_nsis_log_name("install.log"));
        assert!(!is_nsis_log_name("unins000.dat"));
        assert!(!is_nsis_log_name("uninstall.exe"));
    }

    #[test]
    fn extracts_existing_paths_from_nsis_log() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let base = std::env::temp_dir().join(format!(
            "wam-nsis-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&base)?;
        let installed = base.join("app.exe");
        std::fs::write(&installed, b"binary")?;
        let log = base.join("uninstall.log");
        // AdvUninstLog-style line: a header comment plus the resolved path.
        let content = format!(";AdvUninstLog\r\n{}\r\n", installed.display());
        std::fs::write(&log, content.as_bytes())?;

        let found = find_nsis_log(&base);
        let files = nsis_manifest_files(&log, &SystemContext::live());
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(found.as_deref(), Some(log.as_path()));
        assert!(
            files.iter().any(|path| path == &installed),
            "expected the installed path, got {files:?}"
        );
        Ok(())
    }

    fn unique_temp(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0)
        ))
    }

    #[test]
    fn parses_appx_identity_name() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-appx-test");
        std::fs::create_dir_all(&base)?;
        std::fs::write(
            base.join(APPX_MANIFEST),
            r#"<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10">
  <Identity Name="Microsoft.WindowsTerminal" Publisher="CN=Microsoft Corporation" Version="1.18.0.0" />
</Package>"#,
        )?;

        let name = appx_identity_name(&base, &SystemContext::live());
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(name.as_deref(), Some("Microsoft.WindowsTerminal"));
        Ok(())
    }

    #[test]
    fn selects_latest_squirrel_app_dir() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let base = unique_temp("wam-squirrel-test");
        for version in ["app-1.0.2", "app-1.0.9", "app-1.0.10"] {
            std::fs::create_dir_all(base.join(version))?;
        }
        std::fs::write(base.join("Update.exe"), b"x")?;

        let latest = latest_squirrel_app_dir(&base, &SystemContext::live());
        let expected = base.join("app-1.0.10");
        let _ = std::fs::remove_dir_all(&base);

        assert_eq!(
            latest.as_deref(),
            Some(expected.as_path()),
            "numeric version segments must rank 1.0.10 above 1.0.9"
        );
        Ok(())
    }
}
