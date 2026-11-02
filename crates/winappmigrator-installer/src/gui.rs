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

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, Margin, RichText};

use winappmigrator_core::archive::has_certificate_table;
use winappmigrator_core::authenticode::read_signature;
use winappmigrator_core::journal::machine_state_root;
use winappmigrator_core::logging;
use winappmigrator_core::path_utils::system_root;
use winappmigrator_core::prerequisites::spec_for_bundle;
use winappmigrator_core::rollback::interrupted_installs;
use winappmigrator_core::{
    EmbeddedPackage, InstallMonitor, InstallPhase, InstallStep, PackageManifest, RestoreOptions,
    RestoreResult, RuntimePrerequisite, RuntimePrerequisiteKind, SystemChange, SystemChanges,
    UndoReport, WarningSeverity, WinAppMigratorError, default_install_location, hidden_command,
    install_location_problem, read_embedded_manifest, restore_package_monitored, system_changes,
};
use winappmigrator_ui::theme::{
    self, Role, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS, Theme, ThemeMode,
};
use winappmigrator_ui::widgets::{self, ButtonKind, Icon, metric};

const WINDOW_TITLE: &str = "WinAppMigrator Installer";
/// How often to redraw while installing, enough for smooth animation.
const BUSY_REPAINT_INTERVAL: Duration = Duration::from_millis(50);
/// How many logged changes a preview lists before summarizing the rest.
const MAX_LISTED_CHANGES: usize = 300;
/// How many things left out are listed, so a package that asks for a great deal cannot
/// push everything else off the screen.
const MAX_LISTED_REFUSALS: usize = 20;
const UNSIGNED_TEXT: &str = "Not signed: publisher unknown";
/// What a cancelled install is called in its errors.
const CANCELLED_CODE: &str = "install_cancelled";
/// Added to what the person agrees to when they click Install.
const BEYOND_FILES_TEXT: &str = "All of this is done with administrator rights. If anything \
                                 goes wrong, it is all put back. You can also undo the install \
                                 later from Settings, under Installed apps. Files it replaces \
                                 come back, and runtimes it installs stay.";

pub fn run() -> eframe::Result {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([760.0, 720.0])
        .with_min_inner_size([620.0, 520.0])
        .with_app_id("winappmigrator-installer");
    if let Some(icon) = widgets::window_icon() {
        viewport = viewport.with_icon(icon);
    }
    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(move |cc| {
            theme::install_fonts(&cc.egui_ctx, &system_root().join("Fonts"));
            let mut app = InstallerApp::blank();
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
    Signed(String),
    /// There is a signature, and it does not hold up. The reason is Windows' own status.
    Invalid(String),
}

/// What the check behind the window found.
enum Loaded {
    Ready(Arc<EmbeddedPackage>, Box<PackageManifest>, SystemChanges),
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
    /// Cancelled before anything had changed.
    Cancelled,
}

struct InstallerApp {
    checked_target: std::cell::RefCell<Option<(String, Option<String>)>>,
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
    changes: SystemChanges,
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
            theme: Theme::new(ThemeMode::Dark),
            screen: Screen::Checking,
            package: None,
            load_receiver: None,
            title: String::new(),
            publisher: String::new(),
            version: String::new(),
            source_computer: String::new(),
            checked_target: std::cell::RefCell::new(None),
            target_root: String::new(),
            user_data_profile: String::new(),
            show_user_data: false,
            show_license: false,
            restore_user_data: false,
            restore_license: false,
            prereqs: Vec::new(),
            changes: SystemChanges::default(),
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
                Err(error) => Loaded::Failed(format!("Could not locate the installer: {error}")),
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
                    self.screen = Screen::Failed("The check of the installer stopped.".to_owned());
                }
            }
        }
    }

    fn finish_load(&mut self, loaded: Loaded, ctx: &egui::Context) {
        match loaded {
            Loaded::Ready(package, manifest, changes) => {
                self.package = Some(package);
                self.apply_manifest(&manifest);
                self.changes = changes;
                self.screen = Screen::Ready;
                theme::add_fallback_fonts(
                    ctx,
                    &system_root().join("Fonts"),
                    [self.title.as_str(), self.publisher.as_str()],
                );
                self.theme.apply(ctx);
                ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                    "Install {}",
                    self.title
                )));
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
            .unwrap_or_else(|| "Application".to_owned());
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
        // Present in the theme the app used when this installer was built (older
        // packages carry no tag and keep the default dark theme).
        if let Some(tag) = manifest.ui_theme.as_deref() {
            self.theme = Theme::new(ThemeMode::from_tag(tag));
        }
        self.prereqs = manifest
            .runtime_prerequisites
            .iter()
            .map(prereq_row)
            .collect();
    }

    /// A full path is required: anything else would resolve against whatever folder
    /// the installer happens to run from. And the package suggests this location, so it
    /// cannot be somewhere everything else lives, like the Windows folder.
    fn target_is_valid(&self) -> bool {
        self.target_problem().is_none()
    }

    /// Asked on every frame, and it looks at the disk, so it is only asked again once the text
    /// has changed. What is really done with it is checked again then.
    fn target_problem(&self) -> Option<String> {
        let text = self.target_root.trim();
        let mut checked = self.checked_target.borrow_mut();
        if let Some((checked_text, problem)) = checked.as_ref()
            && checked_text == text
        {
            return problem.clone();
        }
        let problem = install_location_problem(Path::new(text));
        *checked = Some((text.to_owned(), problem.clone()));
        problem
    }

    fn start(&mut self, apply: bool, ctx: &egui::Context) {
        if self.phase == Phase::Running || !self.target_is_valid() {
            return;
        }
        let exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(error) => {
                self.error = Some(format!("Could not locate the installer: {error}"));
                self.phase = Phase::Done;
                return;
            }
        };
        let Some(package) = self.package.clone() else {
            self.error = Some("This installer carries no package.".to_owned());
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
        let choices = RestoreChoices {
            target_root: PathBuf::from(self.target_root.trim()),
            apply,
            restore_user_data: self.show_user_data && self.restore_user_data,
            restore_license: self.show_license && self.restore_license,
        };
        let ctx = ctx.clone();
        thread::spawn(move || {
            let message = run_restore(&exe, &package, choices, &monitor);
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
                    self.error = Some("The install stopped unexpectedly.".to_owned());
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

/// What the person chose in the window.
struct RestoreChoices {
    target_root: PathBuf,
    apply: bool,
    restore_user_data: bool,
    restore_license: bool,
}

/// Hash-check the installer's package, read its manifest, and work out what it would do.
/// The package stays held open from here on, so what Install unpacks later is what this
/// checked. A package naming an unsafe path for a file offers nothing to install.
fn load_package(exe: &Path) -> Loaded {
    let package = match EmbeddedPackage::open(exe) {
        Ok(Some(package)) => package,
        Ok(None) => return Loaded::NoPackage,
        Err(error) => return Loaded::Failed(format!("Could not read the installer: {error}")),
    };
    let manifest = match read_embedded_manifest(&package) {
        Ok(manifest) => manifest,
        Err(error) => return Loaded::Failed(format!("Could not read the installer: {error}")),
    };
    match system_changes(&manifest) {
        Ok(changes) => Loaded::Ready(Arc::new(package), Box::new(manifest), changes),
        Err(error) => Loaded::Failed(format!(
            "This installer's package cannot be used on this PC: {error}"
        )),
    }
}

/// What Windows says about the installer file. Most have no signature, which can be seen
/// at once. A signature takes Windows as long to check as hashing the whole file.
fn installer_signature(exe: &Path) -> SignatureState {
    match has_certificate_table(exe) {
        Ok(false) => SignatureState::Unsigned,
        Ok(true) => match read_signature(exe) {
            Ok(signature) if signature.is_valid() => SignatureState::Signed(
                signature
                    .signer_name()
                    .unwrap_or_else(|| "an unnamed publisher".to_owned()),
            ),
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
    } = choices;
    if apply {
        logging::open_machine_log();
    }
    // Remove extraction directories from previous or crashed runs first, so nothing
    // accumulates even if a prior window was killed before its own cleanup could run.
    crate::cleanup::sweep_stale_extractions(exe);
    // Stage on the install destination's drive, so a large package never fails to
    // extract for lack of space on C:.
    let staging = match crate::cleanup::create_staging_dir(Some(&target_root), exe) {
        Ok(staging) => staging,
        Err(error) => return failed(format!("Could not create a staging folder: {error}")),
    };
    match package.extract_monitored(staging.path(), monitor) {
        Ok(()) => {}
        Err(WinAppMigratorError::Cancelled) => return Msg::Cancelled,
        Err(error) => return failed(format!("Could not unpack the package: {error}")),
    }
    let Some(state_root) = machine_state_root() else {
        return failed(
            "This PC's ProgramData folder could not be found, so the install could not be undone."
                .to_owned(),
        );
    };
    let options = RestoreOptions {
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
        Ok(result) => Msg::Done(Box::new(result)),
        Err(WinAppMigratorError::Cancelled) => Msg::Cancelled,
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

        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
            )
            .show(ui, |ui| self.draw_header(ui, &theme));

        if matches!(self.screen, Screen::Ready) {
            egui::Panel::bottom("actions")
                .frame(
                    egui::Frame::new()
                        .fill(theme.surface)
                        .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
                )
                .show(ui, |ui| self.draw_actions(ui, &theme));
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.bg))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_LG as i8))
                            .show(ui, |ui| {
                                ui.set_max_width(ui.available_width());
                                self.draw_body(ui, &theme);
                            });
                    });
            });
    }

    fn draw_header(&self, ui: &mut egui::Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(32.0, 32.0), egui::Sense::hover());
            widgets::paint_logo(ui.painter(), rect, theme);
            ui.add_space(SPACE_XS);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(
                    RichText::new(WINDOW_TITLE)
                        .font(theme::semibold(16.0))
                        .color(theme.text),
                );
                ui.label(
                    RichText::new("Reinstalls an application captured on another PC")
                        .size(12.5)
                        .color(theme.text_muted),
                );
            });
        });
    }

    fn draw_body(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        match &self.screen {
            Screen::Checking => {
                ui.label(
                    RichText::new("Checking the installer…")
                        .size(13.5)
                        .color(theme.text),
                );
                ui.add_space(SPACE_SM);
                widgets::progress_bar(ui, theme, None);
            }
            Screen::NoPackage => widgets::notice(
                ui,
                theme,
                Role::Accent,
                "This is a WinAppMigrator installer stub, and it carries no application \
                 package. Build an installer with WinAppMigrator, then run the produced .exe \
                 here.",
            ),
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
            widgets::notice(ui, theme, Role::Neutral, "Cancelled. Nothing was changed.");
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
                &format!("An earlier install of {app} never finished. Install undoes it first."),
            );
            widgets::section_gap(ui);
        }

        widgets::card_titled(ui, theme, &title, subtitle.as_deref(), |ui| {
            if !source_computer.is_empty() {
                widgets::kv_row(ui, theme, "Captured from", &source_computer);
            }
            signature_row(ui, theme, &self.signature);
            ui.add_space(SPACE_SM);
            widgets::path_field(
                ui,
                theme,
                "Install location",
                &mut self.target_root,
                r"C:\Program Files\App",
                enabled,
                || {
                    rfd::FileDialog::new()
                        .set_title("Choose the install location")
                        .pick_folder()
                },
            );
            if let Some(problem) = self.target_problem() {
                ui.add_space(SPACE_XS);
                widgets::hint(
                    ui,
                    theme,
                    &format!(r"Enter another folder, such as C:\Program Files\App: {problem}."),
                );
            } else if folder_has_files(Path::new(self.target_root.trim())) {
                ui.add_space(SPACE_SM);
                widgets::notice(
                    ui,
                    theme,
                    Role::Warning,
                    "This folder already has files. Files with the same names are replaced, \
                     and come back if you uninstall.",
                );
            }
        });
        widgets::section_gap(ui);

        // Above the options, so it is in view next to the Install button.
        widgets::card_titled(
            ui,
            theme,
            "What this sets up beyond files",
            Some(BEYOND_FILES_TEXT),
            |ui| system_changes_ui(ui, theme, &self.changes),
        );
        widgets::section_gap(ui);

        if self.show_user_data || self.show_license {
            widgets::card_titled(ui, theme, "Options", None, |ui| {
                if self.show_user_data {
                    widgets::option_row(
                        ui,
                        theme,
                        &mut self.restore_user_data,
                        "User data",
                        "On: settings, Documents, and Saved Games go to the usual folders of \
                         whoever is signed in. Off: all of it goes under the install folder.",
                        enabled,
                    );
                    if !self.user_data_profile.is_empty() {
                        widgets::hint(
                            ui,
                            theme,
                            &format!("This holds the data of {}.", self.user_data_profile),
                        );
                    }
                }
                if self.show_license {
                    widgets::option_row(
                        ui,
                        theme,
                        &mut self.restore_license,
                        "License",
                        "Activation and license files and keys. Some may need reactivation here.",
                        enabled,
                    );
                }
            });
            widgets::section_gap(ui);
        }

        widgets::card_titled(
            ui,
            theme,
            "Dependencies",
            Some("What this app needs, and how each is provided."),
            |ui| {
                if self.prereqs.is_empty() {
                    widgets::hint(
                        ui,
                        theme,
                        "No additional runtime dependencies were detected.",
                    );
                }
                for prereq in &self.prereqs {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&prereq.name).size(13.5).color(theme.text));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                ui.label(RichText::new(label).size(13.0).color(theme.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
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
            if widgets::wide_button(ui, theme, ButtonKind::Secondary, "Cancel", can_cancel)
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
        widgets::control_row(ui, |ui| {
            if installed {
                if widgets::button(ui, theme, ButtonKind::Primary, "Close", true).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    Icon::Folder,
                    "Open install folder",
                    true,
                )
                .clicked()
                {
                    let opened = hidden_command("explorer")
                        .arg(self.target_root.trim())
                        .spawn();
                    if let Err(error) = opened {
                        self.error = Some(format!("Could not open the folder: {error}"));
                    }
                }
                return;
            }
            let valid = self.target_is_valid();
            if widgets::button(ui, theme, ButtonKind::Primary, "Install", valid).clicked() {
                self.start(true, &ctx);
            }
            if widgets::button(ui, theme, ButtonKind::Secondary, "Preview changes", valid).clicked()
            {
                self.start(false, &ctx);
            }
        });
    }
}

/// Whether this installer proves who made it. The name a package gives for its publisher
/// comes from the package and proves nothing, so this is said separately.
fn signature_row(ui: &mut egui::Ui, theme: &Theme, signature: &SignatureState) {
    match signature {
        SignatureState::Checking => {
            widgets::kv_row(ui, theme, "Installer", "Checking its signature…");
        }
        SignatureState::Unsigned => widgets::kv_row(ui, theme, "Installer", UNSIGNED_TEXT),
        SignatureState::Signed(name) => {
            widgets::kv_row(ui, theme, "Installer", &format!("Signed by {name}"));
        }
        SignatureState::Invalid(reason) => {
            ui.add_space(SPACE_XS);
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                &format!(
                    "This installer's signature is not valid ({reason}). It may have been changed \
                     since it was signed."
                ),
            );
        }
    }
}

fn system_changes_ui(ui: &mut egui::Ui, theme: &Theme, changes: &SystemChanges) {
    if changes.changes.is_empty() {
        widgets::hint(
            ui,
            theme,
            "Nothing: it only copies files and registry settings.",
        );
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
        ui.label(RichText::new(&change.title).size(13.5).color(theme.text));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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

fn refused_ui(ui: &mut egui::Ui, theme: &Theme, refused: &[String]) {
    for reason in refused.iter().take(MAX_LISTED_REFUSALS) {
        ui.add_space(SPACE_XS);
        widgets::notice(ui, theme, Role::Warning, reason);
    }
    if refused.len() > MAX_LISTED_REFUSALS {
        ui.add_space(SPACE_XS);
        widgets::hint(
            ui,
            theme,
            &format!(
                "and {} more were left out.",
                refused.len() - MAX_LISTED_REFUSALS
            ),
        );
    }
}

fn folder_has_files(folder: &Path) -> bool {
    std::fs::read_dir(folder).is_ok_and(|mut entries| entries.next().is_some())
}

/// What a running install or preview is doing, and how much of that is done. A cancel takes
/// effect at the next item, and once the install has begun means putting back what it did.
fn step_label(
    step: Option<InstallStep>,
    applying: bool,
    cancelling: bool,
) -> (String, Option<f32>) {
    let counted = |text: &str, step: &InstallStep| {
        if step.total > 0 {
            format!("{text}: {} of {}…", step.done.min(step.total), step.total)
        } else {
            format!("{text}…")
        }
    };
    let Some(step) = step else {
        return ("Getting ready…".to_owned(), None);
    };
    if step.phase == InstallPhase::RollingBack {
        return (
            counted("Putting back what was changed", &step),
            step.fraction(),
        );
    }
    if cancelling {
        return ("Cancelling after the current step…".to_owned(), None);
    }
    let text = match step.phase {
        InstallPhase::Unpacking => "Unpacking",
        InstallPhase::CheckingPackage => "Checking the package",
        InstallPhase::RecoveringEarlierInstall => "Undoing an earlier install that never finished",
        _ if !applying => "Working out what would change",
        InstallPhase::Prerequisites => "Installing runtimes",
        InstallPhase::Files => "Copying files",
        InstallPhase::Services => "Setting up services",
        InstallPhase::Drivers => "Setting up drivers",
        InstallPhase::Registry => "Writing registry settings",
        InstallPhase::License => "Restoring license data",
        InstallPhase::ScheduledTasks => "Setting up scheduled tasks",
        InstallPhase::Shortcuts => "Making shortcuts",
        InstallPhase::Environment => "Setting environment variables",
        InstallPhase::UninstallEntry => "Adding the app to Settings",
        InstallPhase::Verifying => "Checking what was installed",
        InstallPhase::RollingBack | InstallPhase::Uninstalling => "Undoing",
    };
    (counted(text, &step), step.fraction())
}

fn log_location_ui(ui: &mut egui::Ui, theme: &Theme, log_path: Option<&Path>) {
    if let Some(log_path) = log_path {
        ui.add_space(SPACE_SM);
        widgets::mono_path(ui, theme, "Log", &log_path.display().to_string());
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
        "Install result"
    } else {
        "Preview: nothing has been changed yet"
    };
    widgets::card_titled(ui, theme, heading, None, |ui| {
        if outcome.installed_cleanly() {
            widgets::notice(
                ui,
                theme,
                Role::Success,
                if outcome.refused.is_empty() {
                    "The application was installed and passed its hash check."
                } else {
                    "The application was installed and passed its hash check, without what is \
                     listed as left out below."
                },
            );
            ui.add_space(SPACE_SM);
        } else if outcome.applied {
            let (role, text) = match outcome.rollback.as_ref() {
                Some(report) if !report.failed.is_empty() || report.waits_for_a_drive() => (
                    Role::Danger,
                    "The install failed, and not all of what it changed could be put back. What \
                     is left is listed below, and the next install tries again.",
                ),
                Some(_) if outcome.cancelled => (
                    Role::Warning,
                    "The install was cancelled, and what it changed was put back.",
                ),
                Some(_) => (
                    Role::Danger,
                    "The install failed, and what it changed was put back.",
                ),
                None => (
                    Role::Danger,
                    "The install did not finish cleanly. Review the problems below.",
                ),
            };
            widgets::notice(ui, theme, role, text);
            log_location_ui(ui, theme, log_path);
            ui.add_space(SPACE_SM);
        }
        widgets::metric_tiles(
            ui,
            theme,
            &[
                match outcome.rollback.as_ref() {
                    Some(report) => metric(
                        "Changes put back",
                        report.undone.len().to_string(),
                        Role::Accent,
                    ),
                    None => metric(
                        if outcome.applied {
                            "Changes made"
                        } else {
                            "Changes planned"
                        },
                        outcome.changes.len().to_string(),
                        Role::Accent,
                    ),
                },
                metric(
                    "Warnings",
                    outcome.warnings.to_string(),
                    if outcome.warnings > 0 {
                        Role::Warning
                    } else {
                        Role::Neutral
                    },
                ),
                metric(
                    "Errors",
                    outcome.errors.to_string(),
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
                widgets::status_pill(
                    ui,
                    theme,
                    if valid {
                        "Package hashes valid"
                    } else {
                        "Package hashes INVALID"
                    },
                    if valid { Role::Success } else { Role::Danger },
                );
            }
            if let Some(valid) = outcome.restore_valid {
                widgets::status_pill(
                    ui,
                    theme,
                    if valid {
                        "Installed files valid"
                    } else {
                        "Installed files INVALID"
                    },
                    if valid { Role::Success } else { Role::Danger },
                );
            }
        });
        if outcome.reboot {
            ui.add_space(SPACE_SM);
            widgets::notice(
                ui,
                theme,
                Role::Warning,
                "A dependency needs a reboot to finish. Restart Windows, then launch the app.",
            );
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
                RichText::new(format!("Changes ({})", outcome.changes.len()))
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
                        &format!("and {} more.", outcome.changes.len() - MAX_LISTED_CHANGES),
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
            (format!("Windows feature: {feature}"), Role::Accent)
        } else if spec.is_some_and(|spec| spec.bundleable) && bundle.package_relative_path.is_some()
        {
            ("Included".to_owned(), Role::Accent)
        } else if let Some(id) = spec.and_then(|spec| spec.winget_id.as_deref()) {
            (format!("Installed with winget: {id}"), Role::Accent)
        } else {
            ("Install manually".to_owned(), Role::Warning)
        }
    } else if matches!(prereq.kind, RuntimePrerequisiteKind::DotNetFramework4x) {
        ("Built into Windows".to_owned(), Role::Success)
    } else if !prereq.install_manually {
        ("Ships with the app".to_owned(), Role::Success)
    } else {
        ("Install manually".to_owned(), Role::Warning)
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
        messages.push((Role::Danger, error.message.clone()));
    }
    // Surface a few of the most relevant warnings without flooding the panel.
    for warning in result
        .warnings
        .iter()
        .filter(|warning| !matches!(warning.severity, WarningSeverity::Info))
        .take(5)
    {
        messages.push((severity_role(&warning.severity), warning.message.clone()));
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
        refused: result
            .refused
            .iter()
            .map(|refused| refused.message.clone())
            .collect(),
        rollback: result.rollback.clone(),
        cancelled: result
            .errors
            .iter()
            .any(|error| error.code == CANCELLED_CODE),
    }
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

    use super::*;
    use winappmigrator_core::SystemChangeKind;

    const PREVIEW_DIR_VARIABLE: &str = "WAM_UI_PREVIEW_DIR";

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
                    detail: r"Runs FabrikamSync.exe as NT AUTHORITY\LocalService, starts automatic"
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
        app
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

    fn harness_for(app: InstallerApp) -> Harness<'static, InstallerApp> {
        let mode = app.theme.mode;
        let builder = Harness::builder().with_size(egui::vec2(760.0, 720.0));
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
            let mut harness = harness_for(sample_installer(mode));
            assert!(harness.query_by_label("Install").is_some());
            assert!(harness.query_by_label("Preview changes").is_some());
            assert!(harness.query_by_label("User data").is_some());
            preview(&mut harness, &format!("ready-{name}"));
        }
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
            "What this sets up beyond files",
            "Fabrikam Sync Service",
            "Service",
            "Starts automatically",
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
        assert!(harness.query_by_label_contains("and 5 more").is_some());
    }

    #[test]
    fn an_unsigned_installer_says_its_publisher_is_unknown() {
        let harness = harness_for(sample_installer(ThemeMode::Dark));
        assert!(harness.query_by_label(UNSIGNED_TEXT).is_some());
    }

    #[test]
    fn a_signed_installer_says_by_whom() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.signature = SignatureState::Signed("Fabrikam, Inc.".to_owned());
        let harness = harness_for(app);
        assert!(harness.query_by_label("Signed by Fabrikam, Inc.").is_some());
        assert!(harness.query_by_label(UNSIGNED_TEXT).is_none());
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
        assert!(harness.query_by_label("Changes put back").is_some());
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
                .query_by_label_contains("nothing has been changed")
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
}
