//! The graphical installer shown when a produced installer is double-clicked.
//!
//! The window opens at once however large the package is. Behind it, the package is
//! hash-checked and its manifest read, without unpacking anything, and the installer's
//! own file is held open so it cannot be swapped from then on. It then presents
//! the app, the install location, and — only when they were captured at build time —
//! checkboxes to restore user data and license data, lists what it will set up beyond
//! its files, what it will leave out, whether the installer is signed, and the runtime
//! dependencies it will install, then unpacks, reinstalls, and hash-verifies on click, showing
//! how far it has got. If that fails, or it is cancelled, what it changed is put back.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, Margin, RichText};

use winappmigrator_core::archive::has_certificate_table;
use winappmigrator_core::authenticode::read_signature;
use winappmigrator_core::journal::{machine_state_root, remove_staging_root_if_unused};
use winappmigrator_core::logging;
use winappmigrator_core::path_utils::{is_network_path, normalize_name_for_match, system_root};
use winappmigrator_core::prerequisites::spec_for_bundle;
use winappmigrator_core::rollback::interrupted_installs;
use winappmigrator_core::space::PlaceNeed;
use winappmigrator_core::{
    EmbeddedPackage, InstallMonitor, InstallPhase, InstallStep, PackageManifest, RestoreOptions,
    RestoreResult, RoomCheck, RuntimePrerequisite, RuntimePrerequisiteKind, SystemChange,
    SystemChanges, UndoReport, Warning, WarningSeverity, WinAppMigratorError,
    apps_listed_as_installed_in, default_install_location, files_no_install_put_there,
    folder_has_files, hidden_command, install_location_problem, install_places,
    location_a_list_is_for, main_program_already_in, panic_text, preview_needs,
    read_embedded_manifest, refuse_without_room_to_install, restore_package_monitored,
    system_changes_restoring,
};
use winappmigrator_i18n::t;
use winappmigrator_ui::language::{LanguageAction, declare_language, language_menu};
use winappmigrator_ui::theme::{
    self, Role, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS, Theme, ThemeMode,
};
use winappmigrator_ui::widgets::{self, ButtonKind, Icon, metric};
use winappmigrator_ui::window::{Opening, WindowSizes};

const WINDOW_SIZES: WindowSizes = WindowSizes {
    opens_at: egui::vec2(760.0, 720.0),
    at_least: egui::vec2(620.0, 520.0),
};
/// How often to redraw while installing, enough for smooth animation.
const BUSY_REPAINT_INTERVAL: Duration = Duration::from_millis(50);
/// How many logged changes a preview lists before summarizing the rest.
const MAX_LISTED_CHANGES: usize = 300;
/// How many things left out are listed, so a package that asks for a great deal cannot
/// push everything else off the screen.
const MAX_LISTED_REFUSALS: usize = 20;
/// How tall the list of the rest of them gets, in points.
const MORE_LEFT_OUT_HEIGHT: f32 = 240.0;
/// What a cancelled install is called in its errors.
const CANCELLED_CODE: &str = "install_cancelled";

/// Where Windows keeps its fonts, for the language in use and for names in other scripts.
fn fonts_dir() -> PathBuf {
    system_root().join("Fonts")
}

/// Why the install location cannot be used. Install and Preview Changes stay off while there is one,
/// so it is said where it cannot be missed, with what to do about it.
#[derive(Clone, Debug, PartialEq, Eq)]
enum LocationProblem {
    /// A program of the name and place of this package's main one is already there, or Windows lists
    /// an app of this name as installed there: the app, installed on this PC some other way.
    AppAlreadyThere,
    /// Windows lists another app, of this name, as installed there.
    OtherAppThere(String),
    /// It has files in it that no install on record put there.
    HasFiles,
    /// It cannot be used for this reason, whatever is in it.
    Refused(String),
}

impl LocationProblem {
    /// What the window says, for an app of this name.
    fn explained(&self, app: &str) -> String {
        match self {
            Self::AppAlreadyThere => t!("installer-location-app-already-there", app_name = app),
            Self::OtherAppThere(other) => t!(
                "installer-location-other-app-there",
                other_app = other.as_str(),
                app_name = app
            ),
            Self::HasFiles => t!("installer-location-has-files"),
            Self::Refused(reason) => t!("installer-location-refused", reason = reason.as_str()),
        }
    }
}

pub fn run() -> eframe::Result {
    // It runs on the PC the app is moved to, and speaks that PC's language. It saves no choice.
    winappmigrator_i18n::use_preferred_language(None);
    let native_options =
        winappmigrator_ui::window_options(WINDOW_SIZES.viewport("winappmigrator-installer"));
    eframe::run_native(
        &t!("installer-window-title"),
        native_options,
        Box::new(move |cc| {
            theme::install_fonts(&cc.egui_ctx, &fonts_dir());
            let mut app = InstallerApp::blank();
            (app.opening, app.theme) = Opening::on_this_account(WINDOW_SIZES);
            app.theme.apply(&cc.egui_ctx);
            app.start_load(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

enum Screen {
    /// The package is being hash-checked, which reads all of it.
    Checking,
    /// A bare stub with no embedded package (only ever the template the app
    /// copies when building an installer).
    NoPackage,
    /// The package could not be read; carries a human-readable reason.
    Failed(String),
    Ready,
}

#[derive(PartialEq, Eq)]
enum Phase {
    Idle,
    Running,
    Done,
}

struct PrereqRow {
    name: String,
    handling: String,
    role: Role,
}

struct Outcome {
    applied: bool,
    changes: Vec<String>,
    warnings: usize,
    errors: usize,
    /// `None` for a preview: a preview checks no hashes, so it must not claim any.
    package_valid: Option<bool>,
    restore_valid: Option<bool>,
    reboot: bool,
    messages: Vec<(Role, String)>,
    /// What the package asked for and the rules did not allow.
    refused: Vec<String>,
    /// The install failed, or was cancelled, and this is what was put back.
    rollback: Option<UndoReport>,
    cancelled: bool,
}

impl Outcome {
    fn installed_cleanly(&self) -> bool {
        self.applied
            && self.errors == 0
            && self.package_valid == Some(true)
            && self.restore_valid != Some(false)
    }
}

/// What Windows says about the installer's own signature.
#[derive(Clone, PartialEq, Eq)]
enum SignatureState {
    Checking,
    Unsigned,
    /// By whom, as the certificate names them; empty if it names nobody.
    Signed(String),
    /// There is a signature, and it does not hold up. The reason is Windows' own status.
    Invalid(String),
}

/// What the check behind the window found.
enum Loaded {
    Ready(Arc<EmbeddedPackage>, Arc<PackageManifest>, ChangesByChoice),
    NoPackage,
    Failed(String),
    Signature(SignatureState),
    /// Installs that never finished, which installing will undo first.
    Interrupted(Vec<String>),
}

enum Msg {
    Progress(InstallStep),
    Done(Box<RestoreResult>),
    Failed(String),
    /// What an install would do is not what the person was shown, so it did not go ahead.
    ListChanged(Box<SystemChanges>),
    /// Cancelled before anything had changed.
    Cancelled,
}

struct InstallerApp {
    checked_target: std::cell::RefCell<Option<(String, Option<LocationProblem>)>>,
    /// What the install location was last looked at for, and how much room there is.
    checked_room: std::cell::RefCell<Option<(String, RoomCheck)>>,
    /// How much room the package takes unpacked, and what goes to the install location.
    unpacked_bytes: u64,
    opening: Opening,
    theme: Theme,
    screen: Screen,
    /// The package, checked and held open. Every read goes through this.
    package: Option<Arc<EmbeddedPackage>>,
    load_receiver: Option<Receiver<Loaded>>,

    title: String,
    publisher: String,
    version: String,
    source_computer: String,
    target_root: String,
    /// The account on the source PC whose data this holds.
    user_data_profile: String,

    show_user_data: bool,
    show_license: bool,
    restore_user_data: bool,
    restore_license: bool,
    prereqs: Vec<PrereqRow>,
    /// What the package sets up beyond files, and what it asked for and will not get.
    /// What an install with the boxes as they are would set up.
    changes: SystemChanges,
    /// The same for every way the boxes can be set, worked out ahead so that a click is instant.
    changes_by_choice: ChangesByChoice,
    /// What the package says, for working out what it would do at another location.
    manifest: Option<Arc<PackageManifest>>,
    /// The location those lists are for. Nothing can be agreed to for any other.
    listed_for: String,
    /// Why this package cannot go to that location, if so.
    list_problem: Option<String>,
    list_receiver: Option<Receiver<(String, Result<ChangesByChoice, String>)>>,
    /// The last Install did not go ahead, because the list had changed.
    list_changed: bool,
    signature: SignatureState,
    /// Apps with an install that never finished.
    interrupted: Vec<String>,

    phase: Phase,
    applied_now: bool,
    step: Option<InstallStep>,
    monitor: Option<InstallMonitor>,
    started: Option<Instant>,
    receiver: Option<Receiver<Msg>>,
    outcome: Option<Outcome>,
    error: Option<String>,
    cancelled_early: bool,
    log_path: Option<PathBuf>,
}

impl InstallerApp {
    fn blank() -> Self {
        Self {
            opening: Opening::new(WINDOW_SIZES),
            theme: Theme::new(ThemeMode::Dark),
            screen: Screen::Checking,
            package: None,
            load_receiver: None,
            title: String::new(),
            publisher: String::new(),
            version: String::new(),
            source_computer: String::new(),
            checked_target: std::cell::RefCell::new(None),
            checked_room: std::cell::RefCell::new(None),
            unpacked_bytes: 0,
            target_root: String::new(),
            user_data_profile: String::new(),
            show_user_data: false,
            show_license: false,
            restore_user_data: false,
            restore_license: false,
            prereqs: Vec::new(),
            changes: SystemChanges::default(),
            changes_by_choice: ChangesByChoice::new(),
            manifest: None,
            listed_for: String::new(),
            list_problem: None,
            list_receiver: None,
            list_changed: false,
            signature: SignatureState::Checking,
            interrupted: Vec::new(),
            phase: Phase::Idle,
            applied_now: false,
            step: None,
            monitor: None,
            started: None,
            receiver: None,
            outcome: None,
            error: None,
            cancelled_early: false,
            log_path: None,
        }
    }

    /// Check the package behind the window, which stays responsive however large it is.
    fn start_load(&mut self, ctx: &egui::Context) {
        let (sender, receiver) = mpsc::channel();
        self.load_receiver = Some(receiver);
        let ctx = ctx.clone();
        thread::spawn(move || {
            let exe = std::env::current_exe();
            let loaded = match exe.as_deref() {
                Ok(exe) => load_package(exe),
                Err(error) => {
                    Loaded::Failed(t!("installer-locate-failed", error = error.to_string()))
                }
            };
            let _ = sender.send(loaded);
            ctx.request_repaint();
            if let Some(state_root) = machine_state_root() {
                let _ = sender.send(Loaded::Interrupted(interrupted_installs(&state_root)));
                ctx.request_repaint();
            }
            // Checking a signature can take as long as hashing the file, and the window has
            // what it needs without waiting for it.
            if let Ok(exe) = exe.as_deref() {
                let _ = sender.send(Loaded::Signature(installer_signature(exe)));
                ctx.request_repaint();
            }
        });
    }

    fn poll_load(&mut self, ctx: &egui::Context) {
        let Some(receiver) = self.load_receiver.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(loaded) => self.finish_load(loaded, ctx),
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(BUSY_REPAINT_INTERVAL),
            Err(mpsc::TryRecvError::Disconnected) => {
                self.load_receiver = None;
                if matches!(self.screen, Screen::Checking) {
                    self.screen = Screen::Failed(t!("installer-check-stopped"));
                }
            }
        }
    }

    fn finish_load(&mut self, loaded: Loaded, ctx: &egui::Context) {
        match loaded {
            Loaded::Ready(package, manifest, changes_by_choice) => {
                self.unpacked_bytes = package.unpacked_bytes().unwrap_or(0);
                self.package = Some(package);
                self.apply_manifest(&manifest);
                self.listed_for = location_a_list_is_for(&manifest).display().to_string();
                self.manifest = Some(manifest);
                self.changes_by_choice = changes_by_choice;
                self.show_changes_for_the_choices();
                self.screen = Screen::Ready;
                self.add_fonts_for_the_names(ctx);
                self.theme.apply(ctx);
                self.retitle(ctx);
            }
            Loaded::NoPackage => self.screen = Screen::NoPackage,
            Loaded::Failed(reason) => self.screen = Screen::Failed(reason),
            Loaded::Signature(signature) => self.signature = signature,
            Loaded::Interrupted(apps) => self.interrupted = apps,
        }
    }

    fn apply_manifest(&mut self, manifest: &PackageManifest) {
        let meta = manifest.application_metadata.as_ref();
        self.title = meta
            .map(|app| app.display_name.clone())
            .filter(|name| !name.is_empty())
            .or_else(|| {
                manifest
                    .main_executable_path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| t!("installer-unnamed-app"));
        self.publisher = meta
            .and_then(|app| app.publisher.clone())
            .unwrap_or_default();
        self.version = meta.and_then(|app| app.version.clone()).unwrap_or_default();
        self.source_computer = manifest.source_computer_name.clone().unwrap_or_default();
        self.user_data_profile = manifest.user_data_profile.clone().unwrap_or_default();
        self.target_root = default_install_location(manifest)
            .map(|path| path.display().to_string())
            .unwrap_or_default();
        self.show_user_data = manifest.included_user_data || manifest.included_program_data;
        self.show_license = manifest.included_license_data;
        self.restore_user_data = self.show_user_data;
        self.restore_license = self.show_license;
        self.prereqs = manifest
            .runtime_prerequisites
            .iter()
            .map(prereq_row)
            .collect();
    }

    /// The app's name and its publisher's can be in a script the language in use does not need, and
    /// take a font of their own.
    fn add_fonts_for_the_names(&self, ctx: &egui::Context) {
        theme::add_fallback_fonts(
            ctx,
            &fonts_dir(),
            [self.title.as_str(), self.publisher.as_str()],
        );
    }

    /// The window's title in the taskbar, naming the app.
    fn retitle(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(t!(
            "installer-window-title-app",
            app_name = self.title.as_str()
        )));
    }

    /// Speak another language from now on, for as long as this window is open. Nothing is saved: the
    /// next installer opens in this PC's language again. What was worked out in words, such as the
    /// list of what it sets up, is worked out again in the new one, as Install is agreeing to the
    /// list as it was shown.
    fn choose_language(
        &mut self,
        language: &'static winappmigrator_i18n::Language,
        ctx: &egui::Context,
    ) {
        winappmigrator_i18n::use_language(language);
        theme::install_fonts(ctx, &fonts_dir());
        self.add_fonts_for_the_names(ctx);
        self.retitle(ctx);
        if let Some(manifest) = self.manifest.as_deref() {
            self.prereqs = manifest
                .runtime_prerequisites
                .iter()
                .map(prereq_row)
                .collect();
            self.listed_for.clear();
            self.list_receiver = None;
        }
        *self.checked_target.borrow_mut() = None;
    }

    /// What the boxes say, as an install is given it.
    fn chosen(&self) -> (bool, bool) {
        (
            self.show_user_data && self.restore_user_data,
            self.show_license && self.restore_license,
        )
    }

    fn show_changes_for_the_choices(&mut self) {
        if let Some(changes) = self.changes_by_choice.get(&self.chosen()) {
            self.changes = changes.clone();
        }
    }

    /// Whether the list on screen is for the location in the box, which is what Install is agreeing
    /// to. A value written with a variable can be set up at one location and left out at another.
    fn list_is_for_this_location(&self) -> bool {
        self.manifest.is_none()
            || (self.list_problem.is_none() && self.listed_for == self.target_root.trim())
    }

    /// Work out the list again when the location has changed, behind the window, as a large package
    /// takes a moment. One at a time, and whatever is in the box by then is what the next is for, so
    /// typing does not start one for each letter. A location that cannot be used gets none, and
    /// nothing is asked of it.
    fn keep_the_list_for_the_location(&mut self, ctx: &egui::Context) {
        let Some(manifest) = self.manifest.clone() else {
            return;
        };
        if let Some(receiver) = self.list_receiver.as_ref() {
            match receiver.try_recv() {
                Ok((location, Ok(lists))) => {
                    self.changes_by_choice = lists;
                    self.list_problem = None;
                    self.listed_for = location;
                    self.show_changes_for_the_choices();
                }
                Ok((location, Err(problem))) => {
                    self.list_problem = Some(problem);
                    self.listed_for = location;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                    return;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.list_problem = Some(t!("installer-list-stopped"));
                    self.listed_for = self.target_root.trim().to_owned();
                }
            }
            self.list_receiver = None;
        }
        let location = self.target_root.trim().to_owned();
        if location == self.listed_for
            || self.phase == Phase::Running
            || self.target_problem().is_some()
        {
            return;
        }
        let (sender, receiver) = mpsc::channel();
        self.list_receiver = Some(receiver);
        let ctx = ctx.clone();
        thread::spawn(move || {
            let lists = changes_for_every_choice(&manifest, Path::new(&location))
                .map_err(|error| error.to_string());
            let _ = sender.send((location, lists));
            ctx.request_repaint();
        });
    }

    /// A full path is required: anything else would resolve against whatever folder
    /// the installer happens to run from. And the package suggests this location, so it
    /// cannot be somewhere everything else lives, like the Windows folder.
    fn target_is_valid(&self) -> bool {
        self.target_problem().is_none()
    }

    /// Asked on every frame, and it looks at the disk, so it is only asked again once the text
    /// has changed. What is really done with it is checked again then.
    fn target_problem(&self) -> Option<LocationProblem> {
        let text = self.target_root.trim();
        let mut checked = self.checked_target.borrow_mut();
        if let Some((checked_text, problem)) = checked.as_ref()
            && checked_text == text
        {
            return problem.clone();
        }
        let folder = Path::new(text);
        // Only a folder that may be used at all is looked into, and so never one on a share.
        let problem = match install_location_problem(folder) {
            Some(reason) => Some(LocationProblem::Refused(reason)),
            None => files_no_install_put_there(folder, machine_state_root().as_deref())
                .map(|_| self.whose_files(folder)),
        };
        *checked = Some((text.to_owned(), problem.clone()));
        problem
    }

    /// What the files already in a folder look like, for saying why it is not used: this app, as
    /// its main program is there or Windows lists it there, another app Windows lists there, or
    /// something no list names.
    fn whose_files(&self, folder: &Path) -> LocationProblem {
        let program_is_there = self
            .manifest
            .as_deref()
            .is_some_and(|manifest| main_program_already_in(manifest, folder));
        let listed = apps_listed_as_installed_in(folder);
        let this_app = normalize_name_for_match(&self.title);
        let listed_as_this_app = !this_app.is_empty()
            && listed
                .iter()
                .any(|name| normalize_name_for_match(name) == this_app);
        if program_is_there || listed_as_this_app {
            LocationProblem::AppAlreadyThere
        } else if let Some(other) = listed.into_iter().next() {
            LocationProblem::OtherAppThere(other)
        } else {
            LocationProblem::HasFiles
        }
    }

    /// How much room an install to this location would need, for showing while it is chosen. What
    /// it would replace is added when it is really asked to go ahead. Asked on every frame, and it
    /// looks at the disk, so it is only asked again once the text has changed.
    fn room_for_target(&self) -> Option<RoomCheck> {
        let text = self.target_root.trim();
        // A package suggests this before anyone has agreed to anything, and asking a share how much
        // room it has connects to it as whoever is looking.
        if text.is_empty()
            || self.unpacked_bytes == 0
            || !Path::new(text).is_absolute()
            || is_network_path(text)
        {
            return None;
        }
        let mut checked = self.checked_room.borrow_mut();
        if let Some((checked_text, room)) = checked.as_ref()
            && checked_text == text
        {
            return Some(room.clone());
        }
        let room = RoomCheck::on_this_pc(&[PlaceNeed {
            place: PathBuf::from(text),
            bytes: self.unpacked_bytes.saturating_mul(2),
            what: winappmigrator_core::space::FOR_UNPACKING_AND_INSTALLING,
        }]);
        *checked = Some((text.to_owned(), room.clone()));
        Some(room)
    }

    fn start(&mut self, apply: bool, ctx: &egui::Context) {
        // What was found when the text last changed could be a long time ago, and an install is told
        // it may replace what is there.
        *self.checked_target.borrow_mut() = None;
        if self.phase == Phase::Running || !self.target_is_valid() {
            return;
        }
        let exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(error) => {
                self.error = Some(t!("installer-locate-failed", error = error.to_string()));
                self.phase = Phase::Done;
                return;
            }
        };
        let Some(package) = self.package.clone() else {
            self.error = Some(t!("installer-carries-no-package"));
            self.phase = Phase::Done;
            return;
        };
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        self.phase = Phase::Running;
        self.applied_now = apply;
        self.outcome = None;
        self.error = None;
        self.cancelled_early = false;
        self.log_path = None;
        self.step = None;
        self.started = Some(Instant::now());
        let progress = sender.clone();
        let progress_ctx = ctx.clone();
        let monitor = InstallMonitor::new(move |step| {
            let _ = progress.send(Msg::Progress(step));
            progress_ctx.request_repaint();
        });
        self.monitor = Some(monitor.clone());
        self.list_changed = false;
        let (restore_user_data, restore_license) = self.chosen();
        let choices = RestoreChoices {
            target_root: PathBuf::from(self.target_root.trim()),
            apply,
            restore_user_data,
            restore_license,
            agreed_to: apply.then(|| self.changes.clone()),
        };
        let ctx = ctx.clone();
        thread::spawn(move || {
            // What goes wrong in a way nothing was ready for still has to end in something a person
            // can read, or this window would say it was running forever and could not be closed.
            let message = catch_unwind(AssertUnwindSafe(|| {
                let message = run_restore(&exe, &package, choices, &monitor);
                if let Some(state_root) = machine_state_root() {
                    remove_staging_root_if_unused(&state_root);
                }
                message
            }))
            .unwrap_or_else(|panic| {
                let reason = panic_text(panic.as_ref());
                tracing::error!(%reason, "the install stopped unexpectedly");
                Msg::Failed(t!(
                    "installer-stopped-unexpectedly-because",
                    reason = reason
                ))
            });
            let _ = sender.send(message);
            ctx.request_repaint();
        });
    }

    fn poll(&mut self, ctx: &egui::Context) {
        let Some(receiver) = self.receiver.as_ref() else {
            return;
        };
        loop {
            match receiver.try_recv() {
                Ok(Msg::Progress(step)) => self.step = Some(step),
                Ok(Msg::Done(result)) => {
                    self.outcome = Some(outcome_from(&result, self.applied_now));
                    self.finish_run();
                    return;
                }
                Ok(Msg::Failed(error)) => {
                    self.error = Some(error);
                    self.finish_run();
                    return;
                }
                Ok(Msg::ListChanged(now)) => {
                    self.changes = *now;
                    self.list_changed = true;
                    // All of them, for whichever way the boxes go next.
                    self.listed_for.clear();
                    self.finish_run();
                    return;
                }
                Ok(Msg::Cancelled) => {
                    self.cancelled_early = true;
                    self.finish_run();
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                    return;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.error = Some(t!("installer-stopped-unexpectedly"));
                    self.finish_run();
                    return;
                }
            }
        }
    }

    fn finish_run(&mut self) {
        self.receiver = None;
        self.monitor = None;
        self.step = None;
        self.phase = Phase::Done;
        self.log_path = logging::current_log_file();
    }

    fn is_cancelling(&self) -> bool {
        self.monitor
            .as_ref()
            .is_some_and(InstallMonitor::is_cancelled)
    }
}

/// What an install would set up, by what it is asked to do with user data and with a license.
type ChangesByChoice = std::collections::HashMap<(bool, bool), SystemChanges>;

/// Install or preview just as the window's buttons do, for what checks this with no window. `Err` is
/// what the window would have shown.
pub fn install_as_the_window_does(
    exe: &Path,
    target_root: &Path,
    apply: bool,
    (restore_user_data, restore_license): (bool, bool),
    monitor: &InstallMonitor,
) -> std::result::Result<RestoreResult, String> {
    let package = EmbeddedPackage::open(exe)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "it carries no package".to_owned())?;
    // What the window would have on screen, worked out as it works it out.
    let agreed_to = if apply {
        let manifest = read_embedded_manifest(&package).map_err(|error| error.to_string())?;
        Some(
            system_changes_restoring(&manifest, target_root, restore_user_data, restore_license)
                .map_err(|error| error.to_string())?,
        )
    } else {
        None
    };
    let choices = RestoreChoices {
        target_root: target_root.to_path_buf(),
        apply,
        restore_user_data,
        restore_license,
        agreed_to,
    };
    match run_restore(exe, &package, choices, monitor) {
        Msg::Done(result) => Ok(*result),
        Msg::Failed(error) => Err(error),
        Msg::ListChanged(_) => Err("what it would do had changed".to_owned()),
        Msg::Cancelled => Err("it was cancelled".to_owned()),
        Msg::Progress(_) => Err("it stopped part way".to_owned()),
    }
}

/// What the person chose in the window.
struct RestoreChoices {
    target_root: PathBuf,
    apply: bool,
    restore_user_data: bool,
    restore_license: bool,
    /// For an install, what the person was shown when they clicked.
    agreed_to: Option<SystemChanges>,
}

/// Hash-check the installer's package, read its manifest, and work out what it would do.
/// The package stays held open from here on, so what Install unpacks later is what this
/// checked. A package naming an unsafe path for a file offers nothing to install.
fn load_package(exe: &Path) -> Loaded {
    let package = match EmbeddedPackage::open(exe) {
        Ok(Some(package)) => package,
        Ok(None) => return Loaded::NoPackage,
        Err(error) => {
            return Loaded::Failed(t!("installer-read-failed", error = error.to_string()));
        }
    };
    let manifest = match read_embedded_manifest(&package) {
        Ok(manifest) => manifest,
        Err(error) => {
            return Loaded::Failed(t!("installer-read-failed", error = error.to_string()));
        }
    };
    match changes_for_every_choice(&manifest, &location_a_list_is_for(&manifest)) {
        Ok(changes) => Loaded::Ready(Arc::new(package), Arc::new(manifest), changes),
        Err(error) => Loaded::Failed(t!(
            "installer-cannot-be-used-here",
            error = error.to_string()
        )),
    }
}

/// What an install to this location would set up, for every way a package's boxes can be set. It comes
/// from the same review an install goes by, given the same location and the same two options.
fn changes_for_every_choice(
    manifest: &PackageManifest,
    location: &Path,
) -> winappmigrator_core::Result<ChangesByChoice> {
    let offers_user_data = manifest.included_user_data || manifest.included_program_data;
    let mut lists = ChangesByChoice::new();
    for user_data in [false, offers_user_data] {
        for license in [false, manifest.included_license_data] {
            if let std::collections::hash_map::Entry::Vacant(place) =
                lists.entry((user_data, license))
            {
                place.insert(system_changes_restoring(
                    manifest, location, user_data, license,
                )?);
            }
        }
    }
    Ok(lists)
}

/// What Windows says about the installer file. Most have no signature, which can be seen
/// at once. A signature takes Windows as long to check as hashing the whole file.
fn installer_signature(exe: &Path) -> SignatureState {
    match has_certificate_table(exe) {
        Ok(false) => SignatureState::Unsigned,
        Ok(true) => match read_signature(exe) {
            Ok(signature) if signature.is_valid() => {
                SignatureState::Signed(signature.signer_name().unwrap_or_default())
            }
            Ok(signature) => SignatureState::Invalid(signature.status),
            Err(reason) => SignatureState::Invalid(reason),
        },
        Err(error) => SignatureState::Invalid(error.to_string()),
    }
}

/// Unpack the package only now, after the user clicked, so nothing sits on disk
/// waiting to be tampered with while the window is idle, and restore it. The staging
/// folder is removed when this returns. Both can be cancelled, and once the install has
/// begun that puts back what it did. A preview changes nothing, so it does not start a log.
fn run_restore(
    exe: &Path,
    package: &EmbeddedPackage,
    choices: RestoreChoices,
    monitor: &InstallMonitor,
) -> Msg {
    let RestoreChoices {
        target_root,
        apply,
        restore_user_data,
        restore_license,
        agreed_to,
    } = choices;
    if apply {
        logging::open_machine_log();
    }
    let manifest = match read_embedded_manifest(package) {
        Ok(manifest) => manifest,
        Err(error) => {
            return failed(t!(
                "installer-package-read-failed",
                error = error.to_string()
            ));
        }
    };
    // Once more now that Install has been clicked, so that nothing is unpacked for nothing. What is
    // agreed to is what was on screen, and the install holds itself to that as well when it gets there.
    if let Some(agreed_to) = agreed_to.as_ref() {
        match system_changes_restoring(&manifest, &target_root, restore_user_data, restore_license)
        {
            Ok(now) if now == *agreed_to => {}
            Ok(now) => return Msg::ListChanged(Box::new(now)),
            Err(error) => return failed(error.to_string()),
        }
    }
    // Remove extraction directories from previous or crashed runs first, so nothing
    // accumulates even if a prior window was killed before its own cleanup could run.
    let Some(state_root) = machine_state_root() else {
        return failed(t!("installer-no-programdata"));
    };
    crate::cleanup::sweep_stale_extractions(exe, Some(&state_root));
    let unpacked_bytes = match package.unpacked_bytes() {
        Ok(bytes) => bytes,
        Err(error) => {
            return failed(t!(
                "installer-package-read-failed",
                error = error.to_string()
            ));
        }
    };
    // Stage on the install destination's drive, so a large package never fails to
    // extract for lack of space on C:.
    let staging = match crate::cleanup::create_staging_dir(
        Some(&target_root),
        exe,
        unpacked_bytes,
        Some(&state_root),
    ) {
        Ok(staging) => staging,
        Err(error) => {
            return failed(t!("installer-staging-failed", error = error.to_string()));
        }
    };
    let places = if apply {
        install_places(
            &manifest,
            &target_root,
            restore_user_data,
            restore_license,
            staging.path(),
            unpacked_bytes,
            &state_root,
            package.stub_bytes(),
        )
    } else {
        Ok(preview_needs(staging.path(), unpacked_bytes))
    };
    let could_not_tell = match places.and_then(|places| refuse_without_room_to_install(&places)) {
        Ok(room) => room.could_not_tell(),
        Err(error) => return failed(error.to_string()),
    };
    for note in &could_not_tell {
        tracing::warn!(%note, "free space could not be checked");
    }
    match package.extract_monitored(staging.path(), monitor) {
        Ok(()) => {}
        Err(WinAppMigratorError::Cancelled) => return Msg::Cancelled,
        Err(error) => {
            return failed(t!("installer-unpack-failed", error = error.to_string()));
        }
    }
    let options = RestoreOptions {
        agreed_to,
        package_root: staging.path().to_path_buf(),
        target_root,
        dry_run: !apply,
        apply,
        force: true,
        restore_user_data,
        restore_license,
        state_root,
    };
    match restore_package_monitored(&options, monitor, Some(package)) {
        Ok(mut result) => {
            for note in could_not_tell {
                result.warnings.push(Warning::new(
                    "free_space_unknown",
                    WarningSeverity::Warning,
                    note,
                    None,
                ));
            }
            Msg::Done(Box::new(result))
        }
        Err(WinAppMigratorError::Cancelled) => Msg::Cancelled,
        Err(WinAppMigratorError::ListChanged(now)) => Msg::ListChanged(now),
        Err(error) => failed(error.to_string()),
    }
}

fn failed(reason: String) -> Msg {
    tracing::error!(%reason, "nothing was installed");
    Msg::Failed(reason)
}

impl eframe::App for InstallerApp {
    /// Clear the framebuffer to the theme background so the window matches the
    /// active theme.
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.theme.bg.to_normalized_gamma_f32()
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.opening.settle(ctx, &mut self.theme);
        self.poll_load(ctx);
        self.poll(ctx);
        // Closing the window while an install runs would kill it part-way, to be undone only
        // by the next install. It is a cancel, and the window stays until that has finished.
        if self.phase == Phase::Running && ctx.input(|input| input.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            if let Some(monitor) = self.monitor.as_ref() {
                monitor.cancel();
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

impl InstallerApp {
    fn draw(&mut self, ui: &mut egui::Ui) {
        // Clone the theme so widget calls (which borrow it) don't conflict with
        // the mutable field access the form controls need.
        let theme = self.theme.clone();
        let ctx = ui.ctx().clone();
        declare_language(&ctx);
        self.keep_the_list_for_the_location(&ctx);

        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
            )
            .show(ui, |ui| {
                widgets::page(ui, |ui| self.draw_header(ui, &theme))
            });

        if matches!(self.screen, Screen::Ready) {
            egui::Panel::bottom("actions")
                .frame(
                    egui::Frame::new()
                        .fill(theme.surface)
                        .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
                )
                .show(ui, |ui| {
                    widgets::page(ui, |ui| self.draw_actions(ui, &theme))
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.bg))
            .show(ui, |ui| {
                widgets::mark_pane(ui);
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_LG as i8))
                            .show(ui, |ui| {
                                ui.set_max_width(ui.available_width());
                                widgets::page(ui, |ui| self.draw_body(ui, &theme));
                            });
                    });
            });
    }

    fn draw_header(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let chosen = ui
            .horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(32.0, 32.0), egui::Sense::hover());
                widgets::paint_logo(ui.painter(), rect, theme);
                ui.add_space(SPACE_XS);
                widgets::column(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    widgets::label(
                        ui,
                        RichText::new(t!("installer-window-title"))
                            .font(theme::semibold(16.0))
                            .color(theme.text),
                    );
                    widgets::label(
                        ui,
                        RichText::new(t!("installer-tagline"))
                            .size(12.5)
                            .color(theme.text_muted),
                    );
                });
                if !matches!(self.screen, Screen::Ready) {
                    return None;
                }
                widgets::trailing(ui, |ui| {
                    // Not while it installs: the install is held to the list as it was shown, in the
                    // words it was shown in.
                    ui.add_enabled_ui(self.phase != Phase::Running, |ui| {
                        language_menu(ui, &fonts_dir())
                    })
                    .inner
                })
            })
            .inner;
        match chosen {
            Some(LanguageAction::Choose(language)) => {
                let ctx = ui.ctx().clone();
                self.choose_language(language, &ctx);
            }
            Some(LanguageAction::Report(url)) => {
                // The browser opening or not is all there is to it.
                let _ = hidden_command("explorer").arg(url).spawn();
            }
            None => {}
        }
    }

    fn draw_body(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        match &self.screen {
            Screen::Checking => {
                widgets::label(
                    ui,
                    RichText::new(t!("installer-checking"))
                        .size(13.5)
                        .color(theme.text),
                );
                ui.add_space(SPACE_SM);
                widgets::progress_bar(ui, theme, None);
            }
            Screen::NoPackage => {
                widgets::notice(ui, theme, Role::Accent, &t!("installer-stub-no-package"));
            }
            Screen::Failed(reason) => widgets::notice(ui, theme, Role::Danger, reason),
            Screen::Ready => self.ready_ui(ui, theme),
        }
    }

    fn ready_ui(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let enabled = self.phase != Phase::Running;
        let subtitle = [self.publisher.as_str(), self.version.as_str()]
            .iter()
            .filter(|part| !part.is_empty())
            .copied()
            .collect::<Vec<_>>()
            .join(" · ");
        let subtitle = (!subtitle.is_empty()).then_some(subtitle);
        let title = self.title.clone();
        let source_computer = self.source_computer.clone();

        // The result goes first: it is what the user is waiting for.
        if let Some(error) = self.error.as_ref() {
            widgets::notice(ui, theme, Role::Danger, error);
            log_location_ui(ui, theme, self.log_path.as_deref());
            widgets::section_gap(ui);
        }
        if self.cancelled_early {
            widgets::notice(ui, theme, Role::Neutral, &t!("installer-cancelled-early"));
            widgets::section_gap(ui);
        }
        if self.list_changed {
            widgets::notice(ui, theme, Role::Warning, &t!("installer-list-changed"));
            widgets::section_gap(ui);
        }
        if let Some(outcome) = self.outcome.as_ref() {
            outcome_ui(ui, theme, outcome, self.log_path.as_deref());
            widgets::section_gap(ui);
        }
        for app in &self.interrupted {
            widgets::notice(
                ui,
                theme,
                Role::Warning,
                &t!("installer-interrupted", app_name = app.as_str()),
            );
            widgets::section_gap(ui);
        }

        widgets::card_titled(ui, theme, &title, subtitle.as_deref(), |ui| {
            if !source_computer.is_empty() {
                widgets::kv_row(ui, theme, &t!("installer-captured-from"), &source_computer);
            }
            signature_row(ui, theme, &self.signature);
            ui.add_space(SPACE_SM);
            widgets::path_field(
                ui,
                theme,
                &t!("installer-install-location"),
                &mut self.target_root,
                r"C:\Program Files\App",
                enabled,
                || {
                    rfd::FileDialog::new()
                        .set_title(t!("installer-choose-location"))
                        .pick_folder()
                },
            );
            if let Some(problem) = self.target_problem() {
                ui.add_space(SPACE_SM);
                widgets::notice(ui, theme, Role::Danger, &problem.explained(&title));
            } else if folder_has_files(Path::new(self.target_root.trim())) {
                ui.add_space(SPACE_SM);
                widgets::notice(ui, theme, Role::Warning, &t!("installer-folder-has-files"));
            }
            if let Some(room) = self.room_for_target() {
                ui.add_space(SPACE_XS);
                widgets::kv_row(ui, theme, &t!("installer-room-needed"), &room.summary());
                for shortfall in room.shortfalls() {
                    ui.add_space(SPACE_XS);
                    widgets::notice(
                        ui,
                        theme,
                        Role::Warning,
                        &t!("installer-room-short", shortfall = shortfall),
                    );
                }
                for note in room.could_not_tell() {
                    ui.add_space(SPACE_XS);
                    widgets::hint(ui, theme, &note);
                }
            }
        });
        widgets::section_gap(ui);

        // Above the options, so it is in view next to the Install button.
        widgets::card_titled(
            ui,
            theme,
            &t!("installer-beyond-files"),
            Some(t!("installer-beyond-files-agreed").as_str()),
            |ui| {
                if let Some(problem) = self.list_problem.as_ref() {
                    widgets::notice(
                        ui,
                        theme,
                        Role::Danger,
                        &t!("installer-cannot-go-here", problem = problem.as_str()),
                    );
                    ui.add_space(SPACE_SM);
                } else if self.list_receiver.is_some() {
                    widgets::hint(ui, theme, &t!("installer-working-out"));
                    ui.add_space(SPACE_SM);
                }
                system_changes_ui(ui, theme, &self.changes);
            },
        );
        widgets::section_gap(ui);

        let mut changed_a_choice = false;
        if self.show_user_data || self.show_license {
            widgets::card_titled(ui, theme, &t!("installer-options"), None, |ui| {
                if self.show_user_data {
                    changed_a_choice |= widgets::option_row(
                        ui,
                        theme,
                        &mut self.restore_user_data,
                        &t!("installer-user-data"),
                        &t!("installer-user-data-detail"),
                        enabled,
                    )
                    .changed();
                    if !self.user_data_profile.is_empty() {
                        widgets::hint(
                            ui,
                            theme,
                            &t!(
                                "installer-user-data-whose",
                                account = self.user_data_profile.as_str()
                            ),
                        );
                    }
                }
                if self.show_license {
                    changed_a_choice |= widgets::option_row(
                        ui,
                        theme,
                        &mut self.restore_license,
                        &t!("installer-license"),
                        &t!("installer-license-detail"),
                        enabled,
                    )
                    .changed();
                }
            });
            widgets::section_gap(ui);
        }
        if changed_a_choice {
            self.show_changes_for_the_choices();
        }

        widgets::card_titled(
            ui,
            theme,
            &t!("installer-dependencies"),
            Some(t!("installer-dependencies-detail").as_str()),
            |ui| {
                if self.prereqs.is_empty() {
                    widgets::hint(ui, theme, &t!("installer-dependencies-none"));
                }
                for prereq in &self.prereqs {
                    ui.horizontal(|ui| {
                        widgets::label(
                            ui,
                            RichText::new(&prereq.name).size(13.5).color(theme.text),
                        );
                        widgets::trailing(ui, |ui| {
                            widgets::status_pill(ui, theme, &prereq.handling, prereq.role);
                        });
                    });
                }
            },
        );
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        if self.phase == Phase::Running {
            let elapsed = self.started.map_or(0, |start| start.elapsed().as_secs());
            let cancelling = self.is_cancelling();
            let (label, fraction) = step_label(self.step, self.applied_now, cancelling);
            ui.horizontal(|ui| {
                widgets::status_line(ui, RichText::new(label).size(13.0).color(theme.text));
                widgets::trailing(ui, |ui| {
                    widgets::label(
                        ui,
                        RichText::new(widgets::format_elapsed(elapsed))
                            .size(12.5)
                            .color(theme.text_muted),
                    );
                });
            });
            ui.add_space(SPACE_SM);
            widgets::progress_bar(ui, theme, fraction);
            ui.add_space(SPACE_MD);
            let can_cancel = !cancelling
                && !self
                    .step
                    .is_some_and(|step| step.phase == InstallPhase::RollingBack);
            if widgets::wide_button(
                ui,
                theme,
                ButtonKind::Secondary,
                &t!("installer-cancel"),
                can_cancel,
            )
            .clicked()
                && let Some(monitor) = self.monitor.as_ref()
            {
                monitor.cancel();
            }
            return;
        }

        let installed = self
            .outcome
            .as_ref()
            .is_some_and(Outcome::installed_cleanly);
        let ctx = ui.ctx().clone();
        // Beside the buttons too, as that is where a person looks when one will not click.
        if !installed {
            let why = if !self.target_is_valid() {
                Some(t!("installer-location-blocks"))
            } else if self.list_problem.is_some() {
                Some(t!("installer-package-blocks"))
            } else {
                None
            };
            if let Some(why) = why {
                widgets::hint(ui, theme, &why);
                ui.add_space(SPACE_SM);
            }
        }
        widgets::control_row(ui, |ui| {
            if installed {
                if widgets::button(ui, theme, ButtonKind::Primary, &t!("installer-close"), true)
                    .clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    Icon::Folder,
                    &t!("installer-open-folder"),
                    true,
                )
                .clicked()
                {
                    let opened = hidden_command("explorer")
                        .arg(self.target_root.trim())
                        .spawn();
                    if let Err(error) = opened {
                        self.error = Some(t!(
                            "installer-open-folder-failed",
                            error = error.to_string()
                        ));
                    }
                }
                return;
            }
            let valid = self.target_is_valid();
            let can_agree = valid && self.list_is_for_this_location();
            if widgets::button(
                ui,
                theme,
                ButtonKind::Primary,
                &t!("installer-install"),
                can_agree,
            )
            .clicked()
            {
                self.start(true, &ctx);
            }
            if widgets::button(
                ui,
                theme,
                ButtonKind::Secondary,
                &t!("installer-preview"),
                valid,
            )
            .clicked()
            {
                self.start(false, &ctx);
            }
        });
    }
}

/// Whether this installer proves who made it. The name a package gives for its publisher
/// comes from the package and proves nothing, so this is said separately.
fn signature_row(ui: &mut egui::Ui, theme: &Theme, signature: &SignatureState) {
    let installer = t!("installer-signature");
    match signature {
        SignatureState::Checking => {
            widgets::kv_row(ui, theme, &installer, &t!("installer-signature-checking"));
        }
        SignatureState::Unsigned => {
            widgets::kv_row(ui, theme, &installer, &t!("installer-signature-unsigned"));
        }
        SignatureState::Signed(name) if name.is_empty() => {
            widgets::kv_row(ui, theme, &installer, &t!("installer-signature-unnamed"));
        }
        SignatureState::Signed(name) => {
            widgets::kv_row(
                ui,
                theme,
                &installer,
                &t!("installer-signature-signed-by", publisher = name.as_str()),
            );
        }
        SignatureState::Invalid(reason) => {
            ui.add_space(SPACE_XS);
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                &t!("installer-signature-invalid", reason = reason.as_str()),
            );
        }
    }
}

fn system_changes_ui(ui: &mut egui::Ui, theme: &Theme, changes: &SystemChanges) {
    if changes.changes.is_empty() {
        widgets::hint(ui, theme, &t!("installer-beyond-files-none"));
    }
    for change in &changes.changes {
        system_change_row(ui, theme, change);
        ui.add_space(SPACE_XS);
    }
    for note in &changes.notes {
        ui.add_space(SPACE_SM);
        widgets::notice(ui, theme, Role::Warning, note);
    }
    refused_ui(ui, theme, &changes.refused);
}

fn system_change_row(ui: &mut egui::Ui, theme: &Theme, change: &SystemChange) {
    ui.horizontal(|ui| {
        widgets::label(
            ui,
            RichText::new(&change.title).size(13.5).color(theme.text),
        );
        widgets::trailing(ui, |ui| {
            let role = if change.high_rights {
                Role::Warning
            } else {
                Role::Accent
            };
            widgets::status_pill(ui, theme, change.kind.label(), role);
        });
    });
    widgets::hint(ui, theme, &change.detail);
}

/// What was left out. The first few in full, and however many a package manages to ask for, the rest
/// are there to be read, in a list of its own so they cannot push everything else off the screen.
fn refused_ui(ui: &mut egui::Ui, theme: &Theme, refused: &[String]) {
    for reason in refused.iter().take(MAX_LISTED_REFUSALS) {
        ui.add_space(SPACE_XS);
        widgets::notice(ui, theme, Role::Warning, reason);
    }
    let Some(rest) = refused
        .get(MAX_LISTED_REFUSALS..)
        .filter(|rest| !rest.is_empty())
    else {
        return;
    };
    ui.add_space(SPACE_XS);
    egui::CollapsingHeader::new(
        RichText::new(t!("installer-more-left-out", count = rest.len()))
            .size(13.0)
            .color(theme.text),
    )
    .id_salt("more left out")
    .show(ui, |ui| {
        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        egui::ScrollArea::vertical()
            .id_salt("more left out list")
            .max_height(MORE_LEFT_OUT_HEIGHT)
            .show_rows(ui, row_height, rest.len(), |ui, rows| {
                for reason in &rest[rows] {
                    widgets::truncated_label(
                        ui,
                        RichText::new(reason).size(12.5).color(theme.text_muted),
                    );
                }
            });
    });
}

/// What is wrong with a folder that has files in it, unless a record says an install was there. The
/// location is one a package suggests, and this window replaces what it finds, so without this a
/// package could say its home was another app's folder and put its own program in place of one that
/// app's service runs, with nothing on the list to say so.
/// What a running install or preview is doing, and how much of that is done. A cancel takes
/// effect at the next item, and once the install has begun means putting back what it did.
fn step_label(
    step: Option<InstallStep>,
    applying: bool,
    cancelling: bool,
) -> (String, Option<f32>) {
    let counted = |text: String, step: &InstallStep| {
        if step.total > 0 {
            t!(
                "installer-step-counted",
                step = text,
                done = step.done.min(step.total),
                total = step.total
            )
        } else {
            t!("installer-step-uncounted", step = text)
        }
    };
    let Some(step) = step else {
        return (t!("installer-step-getting-ready"), None);
    };
    if step.phase == InstallPhase::RollingBack {
        return (
            counted(t!("installer-step-putting-back"), &step),
            step.fraction(),
        );
    }
    if cancelling {
        return (t!("installer-step-cancelling"), None);
    }
    let text = match step.phase {
        InstallPhase::Unpacking => t!("installer-step-unpacking"),
        InstallPhase::CheckingPackage => t!("installer-step-checking-package"),
        InstallPhase::RecoveringEarlierInstall => t!("installer-step-recovering"),
        _ if !applying => t!("installer-step-previewing"),
        InstallPhase::Prerequisites => t!("installer-step-runtimes"),
        InstallPhase::Files => t!("installer-step-files"),
        InstallPhase::Services => t!("installer-step-services"),
        InstallPhase::Drivers => t!("installer-step-drivers"),
        InstallPhase::Registry => t!("installer-step-registry"),
        InstallPhase::License => t!("installer-step-license"),
        InstallPhase::ScheduledTasks => t!("installer-step-tasks"),
        InstallPhase::Shortcuts => t!("installer-step-shortcuts"),
        InstallPhase::Environment => t!("installer-step-environment"),
        InstallPhase::UninstallEntry => t!("installer-step-settings-entry"),
        InstallPhase::Verifying => t!("installer-step-verifying"),
        InstallPhase::RollingBack | InstallPhase::Uninstalling => t!("installer-step-undoing"),
    };
    (counted(text, &step), step.fraction())
}

fn log_location_ui(ui: &mut egui::Ui, theme: &Theme, log_path: Option<&Path>) {
    if let Some(log_path) = log_path {
        ui.add_space(SPACE_SM);
        widgets::mono_path(
            ui,
            theme,
            &t!("installer-log"),
            &log_path.display().to_string(),
        );
    }
}

fn rollback_ui(ui: &mut egui::Ui, theme: &Theme, report: &UndoReport) {
    for line in &report.failed {
        ui.add_space(SPACE_XS);
        widgets::notice(ui, theme, Role::Danger, line);
    }
    for line in report.kept.iter().chain(&report.left_behind) {
        ui.add_space(SPACE_XS);
        widgets::notice(ui, theme, Role::Warning, line);
    }
}

fn outcome_ui(ui: &mut egui::Ui, theme: &Theme, outcome: &Outcome, log_path: Option<&Path>) {
    let heading = if outcome.applied {
        t!("installer-result")
    } else {
        t!("installer-result-preview")
    };
    widgets::card_titled(ui, theme, &heading, None, |ui| {
        if outcome.installed_cleanly() {
            let text = if outcome.refused.is_empty() {
                t!("installer-result-installed")
            } else {
                t!("installer-result-installed-without-left-out")
            };
            widgets::notice(ui, theme, Role::Success, &text);
            ui.add_space(SPACE_SM);
        } else if outcome.applied {
            let (role, text) = match outcome.rollback.as_ref() {
                Some(report) if !report.failed.is_empty() || report.waits_for_a_drive() => {
                    (Role::Danger, t!("installer-result-not-all-put-back"))
                }
                Some(_) if outcome.cancelled => {
                    (Role::Warning, t!("installer-result-cancelled-put-back"))
                }
                Some(_) => (Role::Danger, t!("installer-result-failed-put-back")),
                None => (Role::Danger, t!("installer-result-not-clean")),
            };
            widgets::notice(ui, theme, role, &text);
            log_location_ui(ui, theme, log_path);
            ui.add_space(SPACE_SM);
        }
        widgets::metric_tiles(
            ui,
            theme,
            &[
                match outcome.rollback.as_ref() {
                    Some(report) => metric(
                        t!("installer-result-changes-put-back"),
                        count_text(report.undone.len()),
                        Role::Accent,
                    ),
                    None => metric(
                        if outcome.applied {
                            t!("installer-result-changes-made")
                        } else {
                            t!("installer-result-changes-planned")
                        },
                        count_text(outcome.changes.len()),
                        Role::Accent,
                    ),
                },
                metric(
                    t!("installer-result-warnings"),
                    count_text(outcome.warnings),
                    if outcome.warnings > 0 {
                        Role::Warning
                    } else {
                        Role::Neutral
                    },
                ),
                metric(
                    t!("installer-result-errors"),
                    count_text(outcome.errors),
                    if outcome.errors > 0 {
                        Role::Danger
                    } else {
                        Role::Success
                    },
                ),
            ],
        );
        ui.add_space(SPACE_SM);
        ui.horizontal(|ui| {
            if let Some(valid) = outcome.package_valid {
                let text = if valid {
                    t!("installer-result-package-valid")
                } else {
                    t!("installer-result-package-invalid")
                };
                widgets::status_pill(
                    ui,
                    theme,
                    &text,
                    if valid { Role::Success } else { Role::Danger },
                );
            }
            if let Some(valid) = outcome.restore_valid {
                let text = if valid {
                    t!("installer-result-files-valid")
                } else {
                    t!("installer-result-files-invalid")
                };
                widgets::status_pill(
                    ui,
                    theme,
                    &text,
                    if valid { Role::Success } else { Role::Danger },
                );
            }
        });
        if outcome.reboot {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Warning, &t!("installer-result-reboot"));
        }
        for (role, message) in &outcome.messages {
            ui.add_space(SPACE_XS);
            widgets::notice(ui, theme, *role, message);
        }
        refused_ui(ui, theme, &outcome.refused);
        if let Some(report) = outcome.rollback.as_ref() {
            rollback_ui(ui, theme, report);
        }
        // What an install made and then took back is not worth listing.
        if !outcome.changes.is_empty() && outcome.rollback.is_none() {
            ui.add_space(SPACE_SM);
            egui::CollapsingHeader::new(
                RichText::new(t!(
                    "installer-result-changes",
                    count = outcome.changes.len()
                ))
                .font(theme::semibold(13.0))
                .color(theme.text),
            )
            .default_open(!outcome.applied)
            .show(ui, |ui| {
                for change in outcome.changes.iter().take(MAX_LISTED_CHANGES) {
                    widgets::hint(ui, theme, change);
                }
                if outcome.changes.len() > MAX_LISTED_CHANGES {
                    widgets::hint(
                        ui,
                        theme,
                        &t!(
                            "installer-result-changes-more",
                            count = outcome.changes.len() - MAX_LISTED_CHANGES
                        ),
                    );
                }
            });
        }
    });
}

fn prereq_row(prereq: &RuntimePrerequisite) -> PrereqRow {
    let (handling, role) = if let Some(bundle) = prereq.bundle.as_ref() {
        let spec = spec_for_bundle(&prereq.kind, bundle);
        let spec = spec.as_ref();
        if let Some(feature) = spec.and_then(|spec| spec.windows_feature.as_deref()) {
            (
                t!("installer-dependency-windows-feature", feature = feature),
                Role::Accent,
            )
        } else if spec.is_some_and(|spec| spec.bundleable) && bundle.package_relative_path.is_some()
        {
            (t!("installer-dependency-included"), Role::Accent)
        } else if let Some(id) = spec.and_then(|spec| spec.winget_id.as_deref()) {
            (t!("installer-dependency-winget", id = id), Role::Accent)
        } else {
            (t!("installer-dependency-manual"), Role::Warning)
        }
    } else if matches!(prereq.kind, RuntimePrerequisiteKind::DotNetFramework4x) {
        (t!("installer-dependency-built-in"), Role::Success)
    } else if !prereq.install_manually {
        (t!("installer-dependency-ships-with-app"), Role::Success)
    } else {
        (t!("installer-dependency-manual"), Role::Warning)
    };
    PrereqRow {
        name: prereq.name.clone(),
        handling,
        role,
    }
}

fn outcome_from(result: &RestoreResult, applied: bool) -> Outcome {
    let reboot = result
        .warnings
        .iter()
        .any(|warning| warning.code.contains("reboot"));
    let mut messages: Vec<(Role, String)> = Vec::new();
    for error in &result.errors {
        messages.push((Role::Danger, error.text()));
    }
    // Surface a few of the most relevant warnings without flooding the panel.
    for warning in result
        .warnings
        .iter()
        .filter(|warning| !matches!(warning.severity, WarningSeverity::Info))
        .take(5)
    {
        messages.push((severity_role(&warning.severity), warning.text()));
    }
    let (package_valid, restore_valid) =
        result.verification.as_ref().map_or((None, None), |report| {
            (Some(report.package_valid), report.restore_valid)
        });
    Outcome {
        applied,
        changes: result.changes_logged.clone(),
        warnings: result.warnings.len(),
        errors: result.errors.len(),
        package_valid,
        restore_valid,
        reboot,
        messages,
        refused: result.refused.iter().map(Warning::text).collect(),
        rollback: result.rollback.clone(),
        cancelled: result
            .errors
            .iter()
            .any(|error| error.code == CANCELLED_CODE),
    }
}

/// A count on its own, as the language in use writes numbers.
fn count_text(count: usize) -> String {
    winappmigrator_i18n::number(count as u64)
}

fn severity_role(severity: &WarningSeverity) -> Role {
    match severity {
        WarningSeverity::Error => Role::Danger,
        WarningSeverity::Warning => Role::Warning,
        WarningSeverity::Info => Role::Accent,
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};
    use winappmigrator_core::Result;
    use winappmigrator_i18n::{PSEUDO_LONG, is_marked, with_language};

    use super::*;
    use winappmigrator_core::SystemChangeKind;

    const PREVIEW_DIR_VARIABLE: &str = "WAM_UI_PREVIEW_DIR";
    const SAMPLE_UNPACKED_BYTES: u64 = 412 * 1024 * 1024;

    fn sample_installer(mode: ThemeMode) -> InstallerApp {
        let mut app = InstallerApp::blank();
        app.theme = Theme::new(mode);
        app.screen = Screen::Ready;
        app.title = "Fabrikam Data Sync".to_owned();
        app.publisher = "Fabrikam Inc.".to_owned();
        app.version = "1.7.3".to_owned();
        app.source_computer = "OLD-LAPTOP".to_owned();
        app.target_root = r"C:\Program Files\Fabrikam Data Sync".to_owned();
        app.show_user_data = true;
        app.show_license = true;
        app.restore_user_data = true;
        app.restore_license = true;
        app.prereqs = vec![
            PrereqRow {
                name: "Microsoft Visual C++ Redistributable".to_owned(),
                handling: "Included".to_owned(),
                role: Role::Accent,
            },
            PrereqRow {
                name: ".NET Framework 4.8".to_owned(),
                handling: "Built into Windows".to_owned(),
                role: Role::Success,
            },
        ];
        app.signature = SignatureState::Unsigned;
        app.changes = SystemChanges {
            changes: vec![
                SystemChange {
                    kind: SystemChangeKind::Service,
                    title: "Fabrikam Sync Service".to_owned(),
                    detail: r"Runs FabrikamSync.exe as NT AUTHORITY\LocalService, starts now and each time this PC starts"
                        .to_owned(),
                    high_rights: false,
                },
                SystemChange {
                    kind: SystemChangeKind::Autostart,
                    title: "Fabrikam Tray".to_owned(),
                    detail: "Runs when you sign in".to_owned(),
                    high_rights: false,
                },
                SystemChange {
                    kind: SystemChangeKind::Shortcut,
                    title: "Fabrikam Data Sync".to_owned(),
                    detail: "Opens Fabrikam.exe, for everyone on this PC".to_owned(),
                    high_rights: false,
                },
                SystemChange {
                    kind: SystemChangeKind::PathEntry,
                    title: "Adds to Path".to_owned(),
                    detail: r"The install folder\bin".to_owned(),
                    high_rights: false,
                },
            ],
            refused: Vec::new(),
            notes: Vec::new(),
        };
        app.unpacked_bytes = SAMPLE_UNPACKED_BYTES;
        *app.checked_room.borrow_mut() = Some((
            app.target_root.clone(),
            sample_room(2 * SAMPLE_UNPACKED_BYTES, Some(356 * 1024 * 1024 * 1024)),
        ));
        app
    }

    /// What a drive says, by a test and not by this PC, so that a picture of it stays the same.
    fn sample_room(needed_bytes: u64, free_bytes: Option<u64>) -> RoomCheck {
        RoomCheck {
            drives: vec![winappmigrator_core::space::DriveRoom {
                drive: "C:".to_owned(),
                needed_bytes,
                free_bytes,
                what_for: vec![winappmigrator_core::space::FOR_UNPACKING_AND_INSTALLING],
            }],
        }
    }

    fn sample_outcome(applied: bool, errors: usize) -> Outcome {
        Outcome {
            applied,
            changes: vec![
                r"copy C:\Program Files\Fabrikam Data Sync\Fabrikam.exe".to_owned(),
                r"set HKLM\SOFTWARE\Fabrikam\InstallPath".to_owned(),
            ],
            warnings: 0,
            errors,
            package_valid: applied.then_some(true),
            restore_valid: applied.then_some(errors == 0),
            reboot: false,
            messages: if errors > 0 {
                vec![(
                    Role::Danger,
                    "Could not write a file: access denied.".to_owned(),
                )]
            } else {
                Vec::new()
            },
            refused: Vec::new(),
            rollback: None,
            cancelled: false,
        }
    }

    fn preview_dir() -> Option<PathBuf> {
        std::env::var_os(PREVIEW_DIR_VARIABLE).map(PathBuf::from)
    }

    /// What a maximized window has on 1024x768, and on 1366x768 at 125% and 150%.
    const WHAT_SMALL_SCREENS_LEAVE: [(f32, f32, &str); 3] = [
        (1024.0, 697.0, "1024x768"),
        (1093.0, 543.0, "1366x768-125"),
        (911.0, 441.0, "1366x768-150"),
    ];

    /// How many things on the screen say this, once whatever is under the pointer has had time to
    /// offer something.
    fn said_with_the_pointer_on_the_location(app: InstallerApp, path: &str) -> usize {
        let mut harness = harness_for(app);
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .hover();
        harness.run_steps(60);
        harness.query_all_by_label_contains(path).count()
    }

    #[test]
    fn the_location_is_only_offered_on_hover_when_some_of_it_cannot_be_seen() {
        let path = r"C:\Program Files\Fabrikam Data Sync";
        assert_eq!(
            said_with_the_pointer_on_the_location(sample_installer(ThemeMode::Dark), path),
            0
        );

        let mut long = sample_installer(ThemeMode::Dark);
        let path = format!(r"C:\Program Files\{}", "Fabrikam Data Sync ".repeat(8));
        long.target_root.clone_from(&path);
        assert!(said_with_the_pointer_on_the_location(long, &path) > 0);
    }

    #[test]
    fn on_a_small_screen_install_can_be_reached() {
        for (width, height, name) in WHAT_SMALL_SCREENS_LEAVE {
            let mut harness =
                harness_sized(sample_installer(ThemeMode::Dark), egui::vec2(width, height));
            let button = harness.get_by_label("Install").rect();
            let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, height));
            assert!(
                window.contains_rect(button),
                "{name}: Install is at {button:?}"
            );
            preview(&mut harness, &format!("on-{name}-dark"));
        }
    }

    #[test]
    fn it_only_says_what_pc_an_app_came_from_if_the_package_does() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(harness.query_by_label_contains("OLD-LAPTOP").is_some());
        assert!(harness.query_by_label_contains("Captured From").is_some());

        let mut app = sample_installer(ThemeMode::Dark);
        app.source_computer = String::new();
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("Captured From").is_none());
    }

    #[test]
    fn the_window_says_how_much_room_it_needs_where() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(
            harness
                .query_by_label_contains("up to 824.0 MB on C: (356.0 GB free)")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("may not be enough room")
                .is_none()
        );
    }

    #[test]
    fn a_location_without_room_says_so_before_anything_is_tried() {
        let app = sample_installer(ThemeMode::Dark);
        *app.checked_room.borrow_mut() = Some((
            app.target_root.clone(),
            sample_room(2 * SAMPLE_UNPACKED_BYTES, Some(1024)),
        ));
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("may not be enough room")
                .is_some()
        );
    }

    #[test]
    fn a_drive_that_will_not_say_is_said_not_to_have_said() {
        let app = sample_installer(ThemeMode::Dark);
        *app.checked_room.borrow_mut() = Some((app.target_root.clone(), sample_room(1024, None)));
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("would not say").is_some());
        assert!(
            harness
                .query_by_label_contains("may not be enough room")
                .is_none()
        );
    }

    #[test]
    fn a_location_that_is_not_a_full_path_has_no_room_to_show() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = r"relative\folder".to_owned();
        assert_eq!(app.room_for_target(), None);
    }

    /// What the window shows is worked out for each way its boxes can be, and one click away.
    #[test]
    fn what_it_says_it_sets_up_goes_with_its_boxes() {
        let list_saying = |title: &str| SystemChanges {
            changes: vec![SystemChange {
                kind: SystemChangeKind::Service,
                title: title.to_owned(),
                detail: String::new(),
                high_rights: false,
            }],
            ..SystemChanges::default()
        };
        let mut app = sample_installer(ThemeMode::Dark);
        app.changes_by_choice = ChangesByChoice::from([
            ((true, true), list_saying("With data and license")),
            ((false, true), list_saying("With a license alone")),
            ((true, false), list_saying("With data alone")),
            ((false, false), list_saying("With neither")),
        ]);
        app.show_changes_for_the_choices();
        let mut harness = harness_sized(app, egui::vec2(760.0, 1600.0));
        harness.run();
        assert!(harness.query_by_label("With data and license").is_some());

        harness.get_by_label("User Data").click();
        harness.run();
        assert!(harness.query_by_label("With a license alone").is_some());
        assert!(harness.query_by_label("With data and license").is_none());
        assert_eq!(harness.state().chosen(), (false, true));

        harness.get_by_label("License").click();
        harness.run();
        assert!(harness.query_by_label("With neither").is_some());
        assert_eq!(harness.state().chosen(), (false, false));
    }

    /// What the boxes cannot offer is not an answer that can be given.
    #[test]
    fn the_lists_are_of_what_a_package_can_offer() -> Result<()> {
        let mut manifest: PackageManifest = serde_json::from_str(
            r#"{
                "schema_version": "",
                "source_computer_name": null,
                "source_windows_version": null,
                "scan_timestamp_utc": "",
                "main_executable_path": "C:\\App\\app.exe",
                "application_metadata": null,
                "file_inventory": [],
                "registry_keys_exported": [],
                "services_detected": [],
                "scheduled_tasks_detected": [],
                "shortcuts_detected": [],
                "environment_variables_detected": [],
                "runtime_prerequisites": [],
                "warnings": [],
                "items_intentionally_excluded": []
            }"#,
        )
        .map_err(WinAppMigratorError::from)?;
        manifest.included_user_data = false;
        manifest.included_program_data = false;
        manifest.included_license_data = false;
        let keys = |lists: &ChangesByChoice| {
            let mut keys: Vec<(bool, bool)> = lists.keys().copied().collect();
            keys.sort_unstable();
            keys
        };
        let location = location_a_list_is_for(&manifest);
        assert_eq!(
            keys(&changes_for_every_choice(&manifest, &location)?),
            [(false, false)]
        );
        manifest.included_license_data = true;
        assert_eq!(
            keys(&changes_for_every_choice(&manifest, &location)?),
            [(false, false), (false, true)]
        );
        manifest.included_program_data = true;
        assert_eq!(
            keys(&changes_for_every_choice(&manifest, &location)?).len(),
            4
        );
        Ok(())
    }

    /// An app and an installer of it, all real but for the stub being this test's own program, which
    /// is never run. What the window does with it is what is being looked at.
    fn real_installer_in(folder: &Path) -> Result<PathBuf> {
        let io = |error| WinAppMigratorError::io("temp", error);
        let this_program = std::env::current_exe().map_err(io)?;
        let app = folder.join("app");
        std::fs::create_dir_all(&app).map_err(io)?;
        let program = app.join("fixture.exe");
        std::fs::copy(&this_program, &program).map_err(io)?;
        std::fs::write(app.join("data.txt"), b"hello").map_err(io)?;
        let options = winappmigrator_core::PackageOptions {
            create_self_extracting_archive: true,
            self_extracting_stub: Some(winappmigrator_core::InstallerStub::File(this_program)),
            installer_dir: Some(folder.join("out")),
            include_user_data: false,
            include_program_data: false,
            ..winappmigrator_core::PackageOptions::for_executable(
                &program,
                folder.join("staging").join("App"),
            )
        };
        winappmigrator_core::create_package(&options)?
            .self_extracting_archive_path
            .ok_or_else(|| WinAppMigratorError::InvalidInput("no installer was made".to_owned()))
    }

    fn run_until_done(harness: &mut Harness<'static, InstallerApp>) {
        let finished = (0..3000).any(|_| {
            harness.run_steps(1);
            std::thread::sleep(Duration::from_millis(10));
            let ctx = harness.ctx.clone();
            harness.state_mut().poll(&ctx);
            harness.state().phase == Phase::Done
        });
        assert!(finished, "the window never came back");
    }

    fn run_until_the_list_is_for_the_location(harness: &mut Harness<'static, InstallerApp>) {
        let caught_up = (0..3000).any(|_| {
            harness.run_steps(1);
            std::thread::sleep(Duration::from_millis(10));
            harness.state().list_is_for_this_location()
        });
        assert!(caught_up, "the list never caught up with the location");
    }

    /// A list is for one location, so nothing can be agreed to until it is the one in the box.
    #[test]
    fn install_waits_for_the_list_of_the_location_in_the_box() -> Result<()> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = real_installer_in(work.path())?;

        let mut app = InstallerApp::blank();
        app.theme = Theme::new(ThemeMode::Dark);
        let ctx = egui::Context::default();
        app.finish_load(load_package(&installer), &ctx);
        assert!(matches!(app.screen, Screen::Ready));
        assert!(
            app.list_is_for_this_location(),
            "it opens on where the package suggests"
        );
        let elsewhere = work.path().join("elsewhere");
        app.target_root = elsewhere.display().to_string();
        assert!(!app.list_is_for_this_location());
        app.signature = SignatureState::Unsigned;

        let mut harness = harness_for(app);
        run_until_the_list_is_for_the_location(&mut harness);
        assert_eq!(harness.state().listed_for, elsewhere.display().to_string());
        assert_eq!(harness.state().list_problem, None);
        Ok(())
    }

    /// What the person agreed to is what was on the screen, so if the install would do something
    /// else, it does not go ahead, whatever that is. They see the new list and can click again.
    #[test]
    fn install_does_not_go_ahead_on_a_list_that_is_out_of_date() -> Result<()> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = real_installer_in(work.path())?;
        let target = work.path().join("target");

        let mut app = InstallerApp::blank();
        app.theme = Theme::new(ThemeMode::Dark);
        let ctx = egui::Context::default();
        app.finish_load(load_package(&installer), &ctx);
        app.target_root = target.display().to_string();
        app.signature = SignatureState::Unsigned;

        let mut harness = harness_for(app);
        run_until_the_list_is_for_the_location(&mut harness);
        harness.state_mut().changes.changes.push(SystemChange {
            kind: SystemChangeKind::Service,
            title: "Not what it would do".to_owned(),
            detail: String::new(),
            high_rights: false,
        });
        harness.run_steps(2);
        harness.get_by_label("Install").click();
        run_until_done(&mut harness);
        run_until_the_list_is_for_the_location(&mut harness);

        let state = harness.state();
        assert!(state.list_changed);
        assert_eq!(state.error, None);
        assert!(state.outcome.is_none());
        assert!(
            !state
                .changes
                .changes
                .iter()
                .any(|change| change.title == "Not what it would do")
        );
        assert!(!target.exists());
        assert!(harness.query_by_label_contains("has changed").is_some());
        Ok(())
    }

    /// The window has a route of its own through staging, room, and unpacking, and everything else
    /// goes by the one for scripts. Here a real package is loaded and worked as a person would, by
    /// what is on the screen, into a folder that is only ever looked at. Installing for real changes
    /// the PC, so that is left to elevated_packaging.rs on a runner, and here it gets as far as what
    /// stops it without rights.
    #[test]
    fn a_person_who_clicks_through_a_real_installer_gets_what_it_says() -> Result<()> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = real_installer_in(work.path())?;
        let target = work.path().join("target");

        let mut app = InstallerApp::blank();
        app.theme = Theme::new(ThemeMode::Dark);
        let ctx = egui::Context::default();
        let loaded = load_package(&installer);
        assert!(matches!(loaded, Loaded::Ready(..)), "it did not load");
        app.finish_load(loaded, &ctx);
        assert!(matches!(app.screen, Screen::Ready));
        app.target_root = target.display().to_string();
        app.signature = SignatureState::Unsigned;

        let mut harness = harness_for(app);
        harness.run();
        harness.get_by_label("Preview Changes").click();
        run_until_done(&mut harness);
        let state = harness.state();
        assert_eq!(state.error, None);
        let outcome = state.outcome.as_ref();
        assert!(outcome.is_some_and(|outcome| !outcome.applied));
        assert!(!target.exists(), "a preview changed something");

        if winappmigrator_core::privilege::is_elevated() {
            return Ok(());
        }
        run_until_the_list_is_for_the_location(&mut harness);
        harness.run_steps(2);
        harness.get_by_label("Install").click();
        run_until_done(&mut harness);
        let said = harness.state().error.clone().unwrap_or_default();
        assert!(said.contains("administrator rights"), "{said}");
        assert!(!target.exists());
        Ok(())
    }

    /// A window can sit open, and what it found when the text last changed is not what is there now.
    #[test]
    fn a_folder_is_looked_at_again_when_install_is_clicked() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = folder.path().display().to_string();
        app.checked_target = std::cell::RefCell::new(None);
        assert_eq!(app.target_problem(), None);

        std::fs::write(folder.path().join("appeared.txt"), b"x")?;
        assert_eq!(
            app.target_problem(),
            None,
            "it is only asked when the text changes"
        );
        app.start(true, &egui::Context::default());

        assert!(app.phase == Phase::Idle);
        assert!(app.error.is_none(), "it went on to look for a package");
        assert!(app.target_problem().is_some());
        Ok(())
    }

    /// A package names where its app was, so this is what a hostile one can make a window ask.
    #[test]
    fn the_window_does_not_ask_a_share_how_much_room_it_has() {
        for share in [
            r"\\evil.example\share\app",
            r"\\?\UNC\evil.example\share\app",
            "//evil.example/share/app",
            r"\\localhost\c$\Windows",
        ] {
            let mut app = sample_installer(ThemeMode::Dark);
            app.unpacked_bytes = 1;
            app.target_root = share.to_owned();
            app.checked_room = std::cell::RefCell::new(None);
            assert!(app.room_for_target().is_none(), "{share}");
            assert!(app.checked_room.borrow().is_none(), "{share}");
            assert!(app.target_problem().is_some(), "{share}");
        }
        let mut app = sample_installer(ThemeMode::Dark);
        app.unpacked_bytes = 1;
        app.target_root = r"C:\Program Files\App".to_owned();
        assert!(app.room_for_target().is_some());
    }

    fn harness_for(app: InstallerApp) -> Harness<'static, InstallerApp> {
        harness_sized(app, egui::vec2(760.0, 720.0))
    }

    fn harness_sized(app: InstallerApp, size: egui::Vec2) -> Harness<'static, InstallerApp> {
        let mode = app.theme.mode;
        let builder = Harness::builder().with_size(size);
        let builder = if preview_dir().is_some() {
            builder.wgpu()
        } else {
            builder
        };
        let mut harness = builder
            .with_context_setup(move |ctx| {
                theme::install_fonts(ctx, &system_root().join("Fonts"));
                Theme::new(mode).apply(ctx);
            })
            .build_ui_state(
                |ui, app: &mut InstallerApp| {
                    // The harness adds a margin around its UI. The real window has none.
                    let window = ui.ctx().content_rect();
                    ui.scope_builder(egui::UiBuilder::new().max_rect(window), |ui| app.draw(ui));
                },
                app,
            );
        harness.run_steps(3);
        harness
    }

    fn preview(harness: &mut Harness<'_, InstallerApp>, name: &str) {
        let Some(directory) = preview_dir() else {
            return;
        };
        harness.run_steps(2);
        let Ok(image) = harness.render() else {
            return;
        };
        let _ = std::fs::create_dir_all(&directory);
        let _ = image.save(directory.join(format!("installer-{name}.png")));
    }

    #[test]
    fn ready_screen_offers_install_and_preview() {
        for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
            let harness = harness_for(sample_installer(mode));
            assert!(harness.query_by_label("Install").is_some());
            assert!(harness.query_by_label("Preview Changes").is_some());
            assert!(harness.query_by_label("User Data").is_some());

            // The README's picture, of the installer it shows being built: nothing of an account's
            // is in it, so there is no PC to name and no data to ask about.
            let mut built_as_shown = sample_installer(mode);
            built_as_shown.source_computer = String::new();
            built_as_shown.show_user_data = false;
            built_as_shown.show_license = false;
            built_as_shown.restore_user_data = false;
            built_as_shown.restore_license = false;
            let mut harness = harness_for(built_as_shown);
            assert!(harness.query_by_label_contains("Captured From").is_none());
            assert!(harness.query_by_label("User Data").is_none());
            preview(&mut harness, &format!("ready-{name}"));
        }
    }

    /// The installer runs on the PC an app is moved to, and its words come from the catalog, in that
    /// PC's language. A word left in the code would be the same in every language, unmarked.
    #[test]
    fn the_ready_screen_takes_its_words_from_the_catalog() {
        with_language(&PSEUDO_LONG, || {
            let mut harness = harness_for(sample_installer(ThemeMode::Dark));
            let install = t!("installer-install");
            assert!(is_marked(&install), "{install}");
            assert_eq!(
                harness.get_by_label(&install).accesskit_node().role(),
                egui::accesskit::Role::Button
            );
            assert!(harness.query_by_label("Install").is_none());
            preview(&mut harness, "ready-pseudo-dark");
        });
    }

    /// Only once there is something to install is there a language to choose for it.
    #[test]
    fn the_ready_screen_offers_a_language_menu() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(
            harness
                .query_by_role(egui::accesskit::Role::ComboBox)
                .is_some()
        );
        let harness = harness_for(InstallerApp::blank());
        assert!(
            harness
                .query_by_role(egui::accesskit::Role::ComboBox)
                .is_none()
        );
    }

    #[test]
    fn the_language_menu_waits_while_it_installs() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Running;
        app.applied_now = true;
        app.step = Some(InstallStep::new(InstallPhase::Files, 1, 2));
        app.monitor = Some(InstallMonitor::default());
        let harness = harness_for(app);
        assert!(
            harness
                .get_by_role(egui::accesskit::Role::ComboBox)
                .accesskit_node()
                .is_disabled()
        );
    }

    #[test]
    fn what_it_sets_up_beyond_files_is_listed_before_installing() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.changes.changes.push(SystemChange {
            kind: SystemChangeKind::Driver,
            title: "FabrikamFlt".to_owned(),
            detail: "Loads flt.sys into Windows itself".to_owned(),
            high_rights: true,
        });
        let harness = harness_for(app);
        for label in [
            "What This Sets Up Beyond Files",
            "Fabrikam Sync Service",
            "Service",
            "Starts Automatically",
            "Shortcut",
            "PATH",
            "Driver",
            "FabrikamFlt",
        ] {
            assert!(harness.query_by_label(label).is_some(), "{label}");
        }
        assert!(
            harness
                .query_by_label_contains(r"The install folder\bin")
                .is_some()
        );
        assert!(harness.query_by_label("Install").is_some());
    }

    #[test]
    fn a_package_with_nothing_beyond_files_says_so() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.changes = SystemChanges::default();
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("only copies files")
                .is_some()
        );
    }

    #[test]
    fn what_a_package_was_refused_is_listed_before_installing() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.changes.refused = (0..25)
            .map(|index| {
                format!("HKLM\\Evil{index} was left out: no app install should change this key.")
            })
            .collect();
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("Evil0 was left out")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("Evil19 was left out")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("Evil20 was left out")
                .is_none()
        );
        assert!(
            harness
                .query_by_label_contains("5 more were left out")
                .is_some(),
            "the rest are to be had"
        );
    }

    #[test]
    fn an_unsigned_installer_says_its_publisher_is_unknown() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(
            harness
                .query_by_label(&t!("installer-signature-unsigned"))
                .is_some()
        );
    }

    #[test]
    fn a_signed_installer_says_by_whom() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.signature = SignatureState::Signed("Fabrikam, Inc.".to_owned());
        let harness = harness_for(app);
        assert!(harness.query_by_label("Signed by Fabrikam, Inc.").is_some());
        assert!(
            harness
                .query_by_label(&t!("installer-signature-unsigned"))
                .is_none()
        );

        let mut app = sample_installer(ThemeMode::Dark);
        app.signature = SignatureState::Signed(String::new());
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label("Signed by an unnamed publisher")
                .is_some()
        );
    }

    #[test]
    fn a_bad_signature_does_not_pass_for_a_good_one() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.signature = SignatureState::Invalid("HashMismatch".to_owned());
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("is not valid (HashMismatch)")
                .is_some()
        );
        assert!(harness.query_by_label_contains("Signed by").is_none());
    }

    /// Reads only: nothing is on record for a folder that was just made.
    #[test]
    fn a_folder_with_files_no_install_put_there_is_not_offered() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        assert_eq!(files_no_install_put_there(folder.path(), None), None);
        std::fs::write(folder.path().join("theirs.exe"), b"theirs")?;
        assert!(files_no_install_put_there(folder.path(), None).is_some());
        Ok(())
    }

    #[test]
    fn a_file_with_no_signature_reads_as_unsigned_without_asking_powershell() -> std::io::Result<()>
    {
        let folder = tempfile::tempdir()?;
        let file = folder.path().join("installer.exe");
        std::fs::write(&file, b"MZ not a real program")?;
        assert!(installer_signature(&file) == SignatureState::Unsigned);
        Ok(())
    }

    #[test]
    fn install_needs_a_full_path() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = "relative".to_owned();
        let harness = harness_for(app);
        assert!(
            harness
                .get_by_label("Install")
                .accesskit_node()
                .is_disabled()
        );
        assert!(
            harness
                .query_by_label_contains("Enter another folder")
                .is_some()
        );
    }

    #[test]
    fn install_will_not_go_to_a_system_folder_the_package_suggested() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = system_root().display().to_string();
        let harness = harness_for(app);
        assert!(
            harness
                .get_by_label("Install")
                .accesskit_node()
                .is_disabled()
        );
        assert!(
            harness
                .query_by_label_contains("cannot be the Windows folder")
                .is_some()
        );
    }

    fn location_harness(
        folder: &Path,
        manifest: Option<PackageManifest>,
    ) -> Harness<'static, InstallerApp> {
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = folder.display().to_string();
        app.manifest = manifest.map(Arc::new);
        app.listed_for = app.target_root.clone();
        harness_for(app)
    }

    /// A package of Fabrikam Data Sync, whose main program was `Fabrikam.exe` in its folder.
    fn fabrikam_manifest() -> std::io::Result<PackageManifest> {
        serde_json::from_str(
            r#"{
                "schema_version": "",
                "source_computer_name": null,
                "source_windows_version": null,
                "scan_timestamp_utc": "",
                "main_executable_path": "C:\\Program Files\\Fabrikam Data Sync\\Fabrikam.exe",
                "application_metadata": null,
                "file_inventory": [],
                "registry_keys_exported": [],
                "services_detected": [],
                "scheduled_tasks_detected": [],
                "shortcuts_detected": [],
                "environment_variables_detected": [],
                "runtime_prerequisites": [],
                "warnings": [],
                "items_intentionally_excluded": []
            }"#,
        )
        .map_err(std::io::Error::other)
    }

    fn both_buttons_are_off(harness: &Harness<'_, InstallerApp>) {
        for label in ["Install", "Preview Changes"] {
            assert!(
                harness.get_by_label(label).accesskit_node().is_disabled(),
                "{label}"
            );
        }
        assert!(
            harness
                .query_by_label(&t!("installer-location-blocks"))
                .is_some()
        );
    }

    /// Issue #2: the buttons were off, and all that said why was a grey line that read like help.
    #[test]
    fn a_folder_with_files_says_plainly_why_nothing_can_be_installed() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        std::fs::write(folder.path().join("theirs.dll"), b"theirs")?;
        let mut harness = location_harness(folder.path(), None);
        both_buttons_are_off(&harness);
        assert!(
            harness
                .query_by_label(&LocationProblem::HasFiles.explained("Fabrikam Data Sync"))
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("is already installed")
                .is_none()
        );
        preview(&mut harness, "location-has-files-dark");
        Ok(())
    }

    /// Issue #2: VLC, Firefox and Office were already installed where their installers suggested.
    #[test]
    fn a_folder_the_app_is_already_installed_in_says_so_and_what_to_do() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        std::fs::write(folder.path().join("Fabrikam.exe"), b"MZ")?;
        let mut harness = location_harness(folder.path(), Some(fabrikam_manifest()?));
        both_buttons_are_off(&harness);
        let said = LocationProblem::AppAlreadyThere.explained("Fabrikam Data Sync");
        assert!(said.starts_with("Fabrikam Data Sync is already installed in this folder"));
        assert!(said.contains("uninstall"), "{said}");
        assert!(harness.query_by_label(&said).is_some());
        preview(&mut harness, "location-app-already-there-dark");

        // Once that copy is gone, so is what was said, and the folder is one like any other.
        std::fs::remove_file(folder.path().join("Fabrikam.exe"))?;
        std::fs::write(folder.path().join("left-over.log"), b"x")?;
        let manifest = harness.state().manifest.as_deref().cloned();
        let harness = location_harness(folder.path(), manifest);
        assert!(harness.query_by_label(&said).is_none());
        assert!(
            harness
                .query_by_label(&LocationProblem::HasFiles.explained("Fabrikam Data Sync"))
                .is_some()
        );
        Ok(())
    }

    /// An app Windows lists in the folder is named, so a person knows what is in the way.
    #[test]
    fn another_app_in_the_folder_is_named_with_what_to_do() {
        let said = LocationProblem::OtherAppThere("Microsoft 365 - it-it".to_owned())
            .explained("Microsoft Office Professional Plus 2019 - en-us");
        assert!(
            said.starts_with(
                "Microsoft 365 - it-it is installed in this folder, so Microsoft Office \
                 Professional Plus 2019 - en-us cannot go here."
            ),
            "{said}"
        );
        assert!(said.contains("Enter a new, empty folder above."), "{said}");
        assert!(said.contains("uninstall it first"), "{said}");
    }

    #[test]
    fn a_folder_that_can_be_used_says_nothing_beside_the_buttons() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        let harness = location_harness(folder.path(), None);
        assert!(
            !harness
                .get_by_label("Preview Changes")
                .accesskit_node()
                .is_disabled()
        );
        assert!(
            harness
                .query_by_label(&t!("installer-location-blocks"))
                .is_none()
        );
        assert!(
            harness
                .query_by_label(&t!("installer-package-blocks"))
                .is_none()
        );
        Ok(())
    }

    #[test]
    fn a_package_that_cannot_go_there_says_so_beside_the_buttons() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        let mut app = sample_installer(ThemeMode::Dark);
        app.target_root = folder.path().display().to_string();
        app.manifest = Some(Arc::new(fabrikam_manifest()?));
        app.listed_for = app.target_root.clone();
        app.list_problem = Some("this app cannot run on this PC".to_owned());
        let harness = harness_for(app);
        assert!(
            harness
                .get_by_label("Install")
                .accesskit_node()
                .is_disabled()
        );
        assert!(
            harness
                .query_by_label(&t!("installer-package-blocks"))
                .is_some()
        );
        assert!(
            harness
                .query_by_label(&t!("installer-location-blocks"))
                .is_none()
        );
        Ok(())
    }

    #[test]
    fn options_appear_only_when_captured() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.show_user_data = false;
        app.show_license = false;
        let harness = harness_for(app);
        assert!(harness.query_by_label("Options").is_none());
    }

    #[test]
    fn running_install_hides_the_buttons_shows_how_far_it_has_got_and_can_be_cancelled() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Running;
        app.applied_now = true;
        app.step = Some(InstallStep::new(InstallPhase::Files, 40, 200));
        app.monitor = Some(InstallMonitor::default());
        let mut harness = harness_for(app);
        assert!(harness.query_by_label("Install").is_none());
        assert!(
            harness
                .query_by_label_contains("Copying files: 40 of 200")
                .is_some()
        );
        assert!(
            !harness
                .get_by_label("Cancel")
                .accesskit_node()
                .is_disabled()
        );
        preview(&mut harness, "running-dark");
    }

    #[test]
    fn clicking_cancel_asks_the_install_to_stop() {
        let monitor = InstallMonitor::default();
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Running;
        app.applied_now = true;
        app.step = Some(InstallStep::new(InstallPhase::Files, 1, 2));
        app.monitor = Some(monitor.clone());
        let mut harness = harness_for(app);
        harness.get_by_label("Cancel").click();
        harness.run_steps(2);
        assert!(monitor.is_cancelled());
        assert!(
            harness
                .query_by_label_contains("Cancelling after the current step")
                .is_some()
        );
        assert!(
            harness
                .get_by_label("Cancel")
                .accesskit_node()
                .is_disabled()
        );
    }

    #[test]
    fn putting_things_back_shows_progress_and_cannot_be_cancelled() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Running;
        app.applied_now = true;
        app.step = Some(InstallStep::new(InstallPhase::RollingBack, 3, 9));
        app.monitor = Some(InstallMonitor::default());
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("Putting back what was changed: 3 of 9")
                .is_some()
        );
        assert!(
            harness
                .get_by_label("Cancel")
                .accesskit_node()
                .is_disabled()
        );
    }

    #[test]
    fn every_step_has_words_for_it_and_a_preview_says_it_changes_nothing() {
        for phase in [
            InstallPhase::Unpacking,
            InstallPhase::CheckingPackage,
            InstallPhase::RecoveringEarlierInstall,
            InstallPhase::Prerequisites,
            InstallPhase::Files,
            InstallPhase::Services,
            InstallPhase::Drivers,
            InstallPhase::Registry,
            InstallPhase::License,
            InstallPhase::ScheduledTasks,
            InstallPhase::Shortcuts,
            InstallPhase::Environment,
            InstallPhase::UninstallEntry,
            InstallPhase::Verifying,
            InstallPhase::RollingBack,
        ] {
            let step = Some(InstallStep::new(phase, 0, 0));
            assert!(!step_label(step, true, false).0.is_empty());
        }
        let (label, _) = step_label(
            Some(InstallStep::new(InstallPhase::Files, 1, 2)),
            false,
            false,
        );
        assert!(
            label.starts_with("Working out what would change"),
            "{label}"
        );
        let (label, fraction) = step_label(
            Some(InstallStep::new(InstallPhase::Unpacking, 1, 4)),
            false,
            false,
        );
        assert!(label.contains("1 of 4"), "{label}");
        assert_eq!(fraction, Some(0.25));
        assert_eq!(step_label(None, true, false).1, None);
    }

    #[test]
    fn the_person_is_told_it_can_be_undone_before_installing() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(
            harness
                .query_by_label_contains("undo the install later from Settings")
                .is_some()
        );
    }

    #[test]
    fn an_install_that_never_finished_is_mentioned_before_installing() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.interrupted = vec!["Contoso Notes".to_owned()];
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("An earlier install of Contoso Notes never finished")
                .is_some()
        );
    }

    fn failed_with_rollback(report: UndoReport) -> InstallerApp {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.log_path = Some(PathBuf::from(
            r"C:\ProgramData\WinAppMigrator\logs\install-1.log",
        ));
        let mut outcome = sample_outcome(true, 1);
        outcome.rollback = Some(report);
        app.outcome = Some(outcome);
        app
    }

    #[test]
    fn a_failed_install_says_it_was_put_back_and_where_the_log_is() {
        let mut harness = harness_for(failed_with_rollback(UndoReport {
            undone: vec!["Removed a file".to_owned()],
            left_behind: vec!["The runtime stays.".to_owned()],
            ..UndoReport::default()
        }));
        assert!(
            harness
                .query_by_label_contains("what it changed was put back")
                .is_some()
        );
        assert!(harness.query_by_label_contains("install-1.log").is_some());
        assert!(
            harness
                .query_by_label_contains("The runtime stays")
                .is_some()
        );
        assert!(harness.query_by_label("Changes Put Back").is_some());
        assert!(
            harness.query_by_label("Install").is_some(),
            "it can be tried again"
        );
        preview(&mut harness, "rolled-back-dark");
    }

    #[test]
    fn what_could_not_be_put_back_is_said_plainly() {
        let harness = harness_for(failed_with_rollback(UndoReport {
            failed: vec![r"C:\App\app.exe could not be removed: access is denied.".to_owned()],
            ..UndoReport::default()
        }));
        assert!(
            harness
                .query_by_label_contains("not all of what it changed could be put back")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("app.exe could not be")
                .is_some()
        );
    }

    #[test]
    fn a_cancelled_install_says_so_rather_than_that_it_failed() {
        let mut app = failed_with_rollback(UndoReport::default());
        if let Some(outcome) = app.outcome.as_mut() {
            outcome.cancelled = true;
        }
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("was cancelled").is_some());
        assert!(harness.query_by_label_contains("install failed").is_none());

        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.cancelled_early = true;
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("Nothing was changed")
                .is_some()
        );
    }

    #[test]
    fn a_clean_install_points_at_no_log() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.outcome = Some(sample_outcome(true, 0));
        app.log_path = Some(PathBuf::from(
            r"C:\ProgramData\WinAppMigrator\logs\install-1.log",
        ));
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("install-1.log").is_none());
    }

    #[test]
    fn an_error_before_anything_changed_also_says_where_the_log_is() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.error = Some("Could not unpack the package.".to_owned());
        app.log_path = Some(PathBuf::from(
            r"C:\ProgramData\WinAppMigrator\logs\install-2.log",
        ));
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("install-2.log").is_some());
    }

    #[test]
    fn clean_install_offers_close() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.outcome = Some(sample_outcome(true, 0));
        let mut harness = harness_for(app);
        assert!(harness.query_by_label("Close").is_some());
        assert!(harness.query_by_label("Install").is_none());
        assert!(
            harness
                .query_by_label_contains("passed its hash check")
                .is_some()
        );
        preview(&mut harness, "done-dark");
    }

    #[test]
    fn a_clean_install_with_refusals_says_so_alongside_the_success() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        let mut outcome = sample_outcome(true, 0);
        outcome.refused = vec!["The service Evil was left out: it runs cmd.exe.".to_owned()];
        app.outcome = Some(outcome);
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("listed as left out")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("The service Evil was left out")
                .is_some()
        );
    }

    #[test]
    fn failed_install_does_not_claim_success_and_can_retry() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Done;
        app.outcome = Some(sample_outcome(true, 1));
        let mut harness = harness_for(app);
        assert!(harness.query_by_label("Close").is_none());
        assert!(harness.query_by_label("Install").is_some());
        assert!(
            harness
                .query_by_label_contains("did not finish cleanly")
                .is_some()
        );
        preview(&mut harness, "failed-dark");
    }

    #[test]
    fn preview_says_nothing_changed_yet() {
        let mut app = sample_installer(ThemeMode::Light);
        app.phase = Phase::Done;
        app.outcome = Some(sample_outcome(false, 0));
        let mut harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("Nothing Has Been Changed")
                .is_some()
        );
        assert!(harness.query_by_label("Install").is_some());
        assert!(
            harness.query_by_label_contains("Package hashes").is_none(),
            "a preview checks no hashes, so it must not claim any"
        );
        preview(&mut harness, "preview-light");
    }

    #[test]
    fn the_window_says_so_while_the_package_is_being_checked() {
        let harness = harness_for(InstallerApp::blank());
        assert!(
            harness
                .query_by_label_contains("Checking the installer")
                .is_some()
        );
        assert!(harness.query_by_label("Install").is_none());
    }

    #[test]
    fn a_damaged_installer_offers_nothing_to_install() {
        let mut app = InstallerApp::blank();
        app.screen = Screen::Failed("this installer is damaged".to_owned());
        let harness = harness_for(app);
        assert!(harness.query_by_label_contains("damaged").is_some());
        assert!(harness.query_by_label("Install").is_none());
    }

    #[test]
    fn bare_stub_explains_itself() {
        let mut app = InstallerApp::blank();
        app.screen = Screen::NoPackage;
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("carries no application")
                .is_some()
        );
    }

    // ---- Every screen as a screen reader, a keyboard, and another language meet it ----

    /// The installer's screens, by name.
    fn screens() -> Vec<(&'static str, InstallerApp)> {
        let ready = sample_installer(ThemeMode::Dark);
        let mut running = sample_installer(ThemeMode::Dark);
        running.phase = Phase::Running;
        running.applied_now = true;
        running.step = Some(InstallStep::new(InstallPhase::Files, 40, 200));
        running.monitor = Some(InstallMonitor::default());
        let mut done = sample_installer(ThemeMode::Dark);
        done.phase = Phase::Done;
        done.outcome = Some(sample_outcome(true, 0));
        let mut failed = sample_installer(ThemeMode::Dark);
        failed.phase = Phase::Done;
        failed.outcome = Some(sample_outcome(true, 1));
        let mut unreadable = InstallerApp::blank();
        unreadable.screen = Screen::Failed(t!("installer-check-stopped"));
        vec![
            ("ready", ready),
            ("running", running),
            ("done", done),
            ("failed", failed),
            ("unreadable", unreadable),
            ("no package", InstallerApp::blank()),
        ]
    }

    /// What the tests put on screen themselves: names, paths, and the words of a sample list of what an install sets
    /// up, which the core writes from the catalog in a real installer.
    const DATA: &[&str] = &[
        "Fabrikam Data Sync",
        "Fabrikam Inc.",
        "1.7.3",
        "OLD-LAPTOP",
        r"C:\Program Files\Fabrikam Data Sync\Fabrikam.exe",
        r"C:\Program Files\Fabrikam Data Sync",
        r"HKLM\SOFTWARE\Fabrikam\InstallPath",
        "Microsoft Visual C++ Redistributable",
        ".NET Framework 4.8",
        "Included",
        "Built into Windows",
        "Fabrikam Sync Service",
        r"Runs FabrikamSync.exe as NT AUTHORITY\LocalService, starts now and each time this PC starts",
        "Fabrikam Tray",
        "Runs when you sign in",
        "Opens Fabrikam.exe, for everyone on this PC",
        "Adds to Path",
        r"The install folder\bin",
        "Could not write a file: access denied.",
        "copy",
        "set",
    ];

    fn arabic() -> &'static winappmigrator_i18n::Language {
        winappmigrator_i18n::language("ar-SA").unwrap_or_else(|| panic!("Arabic is in the table"))
    }

    /// AC-A11Y-NAMES.
    #[test]
    fn every_control_has_a_name_a_role_and_its_state() {
        for language in [winappmigrator_i18n::ENGLISH, arabic()] {
            with_language(language, || {
                for (screen, app) in screens() {
                    let harness = harness_for(app);
                    let problems = winappmigrator_ui::checks::interactive_problems(&harness.root());
                    assert!(
                        problems.is_empty(),
                        "{} {screen}: {problems:#?}",
                        language.tag
                    );
                }
            });
        }
    }

    /// AC-A11Y-TARGET.
    #[test]
    fn nothing_clickable_is_too_small_to_hit() {
        for (screen, app) in screens() {
            let harness = harness_for(app);
            let problems = winappmigrator_ui::checks::small_targets(&harness.root());
            assert!(problems.is_empty(), "{screen}: {problems:#?}");
        }
    }

    /// AC-I18N-NOHARDCODE.
    #[test]
    fn every_word_on_every_screen_is_from_the_catalog() {
        with_language(&PSEUDO_LONG, || {
            for (screen, app) in screens() {
                let harness = harness_for(app);
                let problems = winappmigrator_ui::checks::unmarked(&harness.root(), DATA);
                assert!(problems.is_empty(), "{screen}: {problems:#?}");
            }
        });
    }

    /// AC-A11Y-KEYBOARD: the Tab key reaches every button on the screen, Install among them.
    #[test]
    fn the_tab_key_reaches_every_control() {
        let mut harness = harness_for(sample_installer(ThemeMode::Dark));
        let order = winappmigrator_ui::checks::tab_order(&mut harness, 200);
        let buttons: Vec<String> = winappmigrator_ui::checks::all_nodes(&harness.root())
            .iter()
            .filter(|node| {
                node.accesskit_node().role() == egui::accesskit::Role::Button
                    && !node.accesskit_node().is_disabled()
            })
            .map(winappmigrator_ui::checks::name_of)
            .collect();
        assert!(buttons.contains(&t!("installer-install")), "{buttons:#?}");
        for button in &buttons {
            assert!(
                order.contains(button),
                "{button} is never reached: {order:#?}"
            );
        }
    }

    /// AC-RTL-MIRROR: Install starts its row of actions, at the left, or at the right in a right-to-left language.
    #[test]
    fn a_right_to_left_window_is_the_mirror_image() {
        let install_x = |language: &'static winappmigrator_i18n::Language| -> (f32, f32) {
            with_language(language, || {
                let harness = harness_for(sample_installer(ThemeMode::Dark));
                let install = harness.get_by_label(&t!("installer-install")).rect();
                (install.center().x, harness.ctx.content_rect().width())
            })
        };
        let (x, width) = install_x(winappmigrator_i18n::ENGLISH);
        assert!(x < width / 2.0, "{x} of {width}");
        let (x, width) = install_x(&winappmigrator_i18n::PSEUDO_RTL);
        assert!(x > width / 2.0, "{x} of {width}");
    }

    /// AC-A11Y-LIVE: how an install is going is said as it changes, and a failure at once.
    #[test]
    fn progress_and_problems_are_said_as_they_change() {
        let mut running = sample_installer(ThemeMode::Dark);
        running.phase = Phase::Running;
        running.applied_now = true;
        running.step = Some(InstallStep::new(InstallPhase::Files, 40, 200));
        running.monitor = Some(InstallMonitor::default());
        let harness = harness_for(running);
        let live = winappmigrator_ui::checks::live_regions(&harness.root());
        assert!(
            live.iter()
                .any(|(name, urgent)| name.contains("40") && !urgent),
            "{live:#?}"
        );
        let mut failed = sample_installer(ThemeMode::Dark);
        failed.phase = Phase::Done;
        failed.outcome = Some(sample_outcome(true, 1));
        let harness = harness_for(failed);
        let live = winappmigrator_ui::checks::live_regions(&harness.root());
        assert!(live.iter().any(|(_, urgent)| *urgent), "{live:#?}");
    }

    /// AC-A11Y-TEXT-SIZE: with Windows' Text size at 150% and 225% on a 1024 by 768 screen, Install can still be reached.
    #[test]
    fn larger_text_leaves_every_control_within_reach() {
        for (scale, percent) in [(1.5_f32, "150"), (2.25, "225")] {
            let size = egui::vec2(1024.0 / scale, 697.0 / scale);
            let mut harness = harness_sized(sample_installer(ThemeMode::Dark), size);
            let label = winappmigrator_i18n::t!("installer-install");
            harness.get_by_label(&label).scroll_to_me();
            harness.run();
            let button = harness.get_by_label(&label).rect();
            let window = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
            assert!(
                window.contains_rect(button),
                "{percent}%: Install is at {button:?} in {size:?}"
            );
        }
    }

    /// WCAG 2.4.11: on a small screen with large text, the Tab key never gives the focus to something that cannot be
    /// seen.
    #[test]
    fn what_has_the_focus_can_be_seen() {
        let size = egui::vec2(1024.0 / 1.5, 697.0 / 1.5);
        let mut harness = harness_sized(sample_installer(ThemeMode::Dark), size);
        let window = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
        let hidden = winappmigrator_ui::checks::hidden_when_focused(&mut harness, window, 200);
        assert!(hidden.is_empty(), "{hidden:#?}");
    }
}
