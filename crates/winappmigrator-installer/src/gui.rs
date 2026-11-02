//! The graphical installer shown when a produced installer is double-clicked.
//!
//! The window opens at once however large the package is. Behind it, the package is
//! hash-checked and its manifest read, without unpacking anything, and the installer's
//! own file is held open so it cannot be swapped from then on. It then presents
//! the app, the install location, and — only when they were captured at build time —
//! checkboxes to restore user data and license data, lists the runtime dependencies
//! it will install, then unpacks, reinstalls, and hash-verifies on click.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, Margin, RichText};

use winappmigrator_core::path_utils::system_root;
use winappmigrator_core::prerequisites::spec_for_bundle;
use winappmigrator_core::{
    EmbeddedPackage, PackageManifest, RestoreOptions, RestoreResult, RuntimePrerequisite,
    RuntimePrerequisiteKind, WarningSeverity, hidden_command, read_embedded_manifest,
    restore_package,
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
}

impl Outcome {
    fn installed_cleanly(&self) -> bool {
        self.applied
            && self.errors == 0
            && self.package_valid == Some(true)
            && self.restore_valid != Some(false)
    }
}

/// What the check behind the window found.
enum Loaded {
    Ready(Arc<EmbeddedPackage>, Box<PackageManifest>),
    NoPackage,
    Failed(String),
}

enum Msg {
    Status(&'static str),
    Done(Box<RestoreResult>),
    Failed(String),
}

struct InstallerApp {
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

    show_user_data: bool,
    show_license: bool,
    restore_user_data: bool,
    restore_license: bool,
    prereqs: Vec<PrereqRow>,

    phase: Phase,
    applied_now: bool,
    detail: String,
    started: Option<Instant>,
    receiver: Option<Receiver<Msg>>,
    outcome: Option<Outcome>,
    error: Option<String>,
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
            target_root: String::new(),
            show_user_data: false,
            show_license: false,
            restore_user_data: false,
            restore_license: false,
            prereqs: Vec::new(),
            phase: Phase::Idle,
            applied_now: false,
            detail: String::new(),
            started: None,
            receiver: None,
            outcome: None,
            error: None,
        }
    }

    /// Check the package behind the window, which stays responsive however large it is.
    fn start_load(&mut self, ctx: &egui::Context) {
        let (sender, receiver) = mpsc::channel();
        self.load_receiver = Some(receiver);
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = sender.send(load_package());
            ctx.request_repaint();
        });
    }

    fn poll_load(&mut self, ctx: &egui::Context) {
        let Some(receiver) = self.load_receiver.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(loaded) => {
                self.load_receiver = None;
                self.finish_load(loaded, ctx);
            }
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(BUSY_REPAINT_INTERVAL),
            Err(mpsc::TryRecvError::Disconnected) => {
                self.load_receiver = None;
                self.screen = Screen::Failed("The check of the installer stopped.".to_owned());
            }
        }
    }

    fn finish_load(&mut self, loaded: Loaded, ctx: &egui::Context) {
        match loaded {
            Loaded::Ready(package, manifest) => {
                self.package = Some(package);
                self.apply_manifest(&manifest);
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
        self.target_root = meta
            .and_then(|app| app.install_location.clone())
            .or_else(|| {
                manifest
                    .main_executable_path
                    .parent()
                    .map(std::path::Path::to_path_buf)
            })
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
    /// the installer happens to run from.
    fn target_is_valid(&self) -> bool {
        Path::new(self.target_root.trim()).is_absolute()
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
        self.detail = "Unpacking…".to_owned();
        self.started = Some(Instant::now());
        let choices = RestoreChoices {
            target_root: PathBuf::from(self.target_root.trim()),
            apply,
            restore_user_data: self.show_user_data && self.restore_user_data,
            restore_license: self.show_license && self.restore_license,
        };
        let ctx = ctx.clone();
        thread::spawn(move || {
            let message = run_restore(&exe, &package, choices, &sender, &ctx);
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
                Ok(Msg::Status(status)) => status.clone_into(&mut self.detail),
                Ok(Msg::Done(result)) => {
                    self.outcome = Some(outcome_from(&result, self.applied_now));
                    self.phase = Phase::Done;
                    self.receiver = None;
                    return;
                }
                Ok(Msg::Failed(error)) => {
                    self.error = Some(error);
                    self.phase = Phase::Done;
                    self.receiver = None;
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                    return;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.error = Some("The install stopped unexpectedly.".to_owned());
                    self.receiver = None;
                    self.phase = Phase::Done;
                    return;
                }
            }
        }
    }
}

/// What the person chose in the window.
struct RestoreChoices {
    target_root: PathBuf,
    apply: bool,
    restore_user_data: bool,
    restore_license: bool,
}

/// Hash-check the installer's package and read its manifest. The package stays held open
/// from here on, so what Install unpacks later is what this checked.
fn load_package() -> Loaded {
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(error) => return Loaded::Failed(format!("Could not locate the installer: {error}")),
    };
    let package = match EmbeddedPackage::open(&exe) {
        Ok(Some(package)) => package,
        Ok(None) => return Loaded::NoPackage,
        Err(error) => return Loaded::Failed(format!("Could not read the installer: {error}")),
    };
    match read_embedded_manifest(&package) {
        Ok(manifest) => Loaded::Ready(Arc::new(package), Box::new(manifest)),
        Err(error) => Loaded::Failed(format!("Could not read the installer: {error}")),
    }
}

/// Unpack the package only now, after the user clicked, so nothing sits on disk
/// waiting to be tampered with while the window is idle, and restore it. The staging
/// folder is removed when this returns.
fn run_restore(
    exe: &Path,
    package: &EmbeddedPackage,
    choices: RestoreChoices,
    sender: &Sender<Msg>,
    ctx: &egui::Context,
) -> Msg {
    let RestoreChoices {
        target_root,
        apply,
        restore_user_data,
        restore_license,
    } = choices;
    // Remove extraction directories from previous or crashed runs first, so nothing
    // accumulates even if a prior window was killed before its own cleanup could run.
    crate::cleanup::sweep_stale_extractions(exe);
    // Stage on the install destination's drive, so a large package never fails to
    // extract for lack of space on C:.
    let staging = match crate::cleanup::create_staging_dir(Some(&target_root), exe) {
        Ok(staging) => staging,
        Err(error) => return Msg::Failed(format!("Could not create a staging folder: {error}")),
    };
    if let Err(error) = package.extract_to(staging.path()) {
        return Msg::Failed(format!("Could not unpack the package: {error}"));
    }

    let _ = sender.send(Msg::Status(if apply {
        "Installing the application, its data, and its dependencies…"
    } else {
        "Working out what would change…"
    }));
    ctx.request_repaint();
    let options = RestoreOptions {
        package_root: staging.path().to_path_buf(),
        target_root,
        dry_run: !apply,
        apply,
        force: true,
        restore_user_data,
        restore_license,
    };
    match restore_package(&options) {
        Ok(result) => Msg::Done(Box::new(result)),
        Err(error) => Msg::Failed(error.to_string()),
    }
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
            widgets::section_gap(ui);
        }
        if let Some(outcome) = self.outcome.as_ref() {
            outcome_ui(ui, theme, outcome);
            widgets::section_gap(ui);
        }

        widgets::card_titled(ui, theme, &title, subtitle.as_deref(), |ui| {
            if !source_computer.is_empty() {
                widgets::kv_row(ui, theme, "Captured from", &source_computer);
                ui.add_space(SPACE_SM);
            }
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
            if !self.target_is_valid() {
                ui.add_space(SPACE_XS);
                widgets::hint(
                    ui,
                    theme,
                    r"Enter a full folder path, such as C:\Program Files\App.",
                );
            } else if folder_has_files(Path::new(self.target_root.trim())) {
                ui.add_space(SPACE_SM);
                widgets::notice(
                    ui,
                    theme,
                    Role::Warning,
                    "This folder already has files. Files with the same names are replaced.",
                );
            }
        });
        widgets::section_gap(ui);

        if self.show_user_data || self.show_license {
            widgets::card_titled(ui, theme, "Options", None, |ui| {
                if self.show_user_data {
                    widgets::option_row(
                        ui,
                        theme,
                        &mut self.restore_user_data,
                        "User data",
                        "On: app settings go to their usual folders, and Documents and Saved Games \
                         under the install folder. Off: all of it goes under the install folder.",
                        enabled,
                    );
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
            ui.horizontal(|ui| {
                ui.label(RichText::new(&self.detail).size(13.0).color(theme.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(widgets::format_elapsed(elapsed))
                            .size(12.5)
                            .color(theme.text_muted),
                    );
                });
            });
            ui.add_space(SPACE_SM);
            widgets::progress_bar(ui, theme, None);
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

fn folder_has_files(folder: &Path) -> bool {
    std::fs::read_dir(folder).is_ok_and(|mut entries| entries.next().is_some())
}

fn outcome_ui(ui: &mut egui::Ui, theme: &Theme, outcome: &Outcome) {
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
                "The application was installed and passed its hash check.",
            );
            ui.add_space(SPACE_SM);
        } else if outcome.applied {
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                "The install did not finish cleanly. Review the problems below.",
            );
            ui.add_space(SPACE_SM);
        }
        widgets::metric_tiles(
            ui,
            theme,
            &[
                metric(
                    if outcome.applied {
                        "Changes made"
                    } else {
                        "Changes planned"
                    },
                    outcome.changes.len().to_string(),
                    Role::Accent,
                ),
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
        if !outcome.changes.is_empty() {
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
                .query_by_label_contains("Enter a full folder path")
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
    fn running_install_hides_the_buttons() {
        let mut app = sample_installer(ThemeMode::Dark);
        app.phase = Phase::Running;
        app.detail = "Installing the application, its data, and its dependencies…".to_owned();
        let mut harness = harness_for(app);
        assert!(harness.query_by_label("Install").is_none());
        assert!(
            harness
                .query_by_label_contains("Installing the application")
                .is_some()
        );
        preview(&mut harness, "running-dark");
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
