//! The graphical installer shown when a produced installer is double-clicked.
//!
//! It reads the application package embedded in its own executable, presents the
//! app, the install location, and — only when they were captured at build time —
//! checkboxes to restore user data and license data, lists the runtime
//! dependencies it will install, then reinstalls and hash-verifies on click.

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui;
use tempfile::TempDir;

use winappmigrator_core::package::read_manifest;
use winappmigrator_core::{
    PackageManifest, RestoreOptions, RestoreResult, RuntimePrerequisite, RuntimePrerequisiteKind,
    WarningSeverity, extract_embedded_self_archive, has_embedded_self_archive, restore_package,
};
use winappmigrator_ui::theme::{Role, Theme, ThemeMode};
use winappmigrator_ui::widgets::{self, ButtonKind, metric};

const WINDOW_TITLE: &str = "WinAppMigrator Installer";

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 700.0])
            .with_min_inner_size([620.0, 520.0])
            .with_app_id("winappmigrator-installer"),
        ..Default::default()
    };
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| {
            let app = InstallerApp::new();
            app.theme.apply(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

enum Screen {
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
    changes: usize,
    warnings: usize,
    errors: usize,
    package_valid: bool,
    restore_valid: Option<bool>,
    reboot: bool,
    messages: Vec<(Role, String)>,
}

enum Msg {
    Done(Box<RestoreResult>),
    Failed(String),
}

struct InstallerApp {
    theme: Theme,
    screen: Screen,
    // Kept alive so the extracted package outlives the restore.
    _temp: Option<TempDir>,
    package_root: PathBuf,

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
    receiver: Option<Receiver<Msg>>,
    outcome: Option<Outcome>,
    error: Option<String>,
}

impl InstallerApp {
    fn new() -> Self {
        let mut app = Self {
            theme: Theme::new(ThemeMode::Dark),
            screen: Screen::NoPackage,
            _temp: None,
            package_root: PathBuf::new(),
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
            receiver: None,
            outcome: None,
            error: None,
        };
        app.load();
        app
    }

    fn load(&mut self) {
        let exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(error) => {
                self.screen = Screen::Failed(format!("Could not locate the installer: {error}"));
                return;
            }
        };
        match has_embedded_self_archive(&exe) {
            Ok(true) => {}
            Ok(false) => {
                self.screen = Screen::NoPackage;
                return;
            }
            Err(error) => {
                self.screen = Screen::Failed(format!("Could not read the installer: {error}"));
                return;
            }
        }
        // Remove extraction directories from previous or crashed runs first, so
        // nothing accumulates even if a prior window was killed before its own
        // cleanup could run.
        crate::cleanup::sweep_stale_extractions(&exe);
        // Stage the extracted package on the same drive as the installer itself
        // (where the user placed it to run on the target), not the system temp
        // drive, so a large package never fails to extract for lack of space on
        // C:. Falls back to the system temp directory if that is not writable.
        let temp = match crate::cleanup::create_staging_dir(None, &exe) {
            Ok(temp) => temp,
            Err(error) => {
                self.screen =
                    Screen::Failed(format!("Could not create a staging directory: {error}"));
                return;
            }
        };
        if let Err(error) = extract_embedded_self_archive(&exe, temp.path()) {
            self.screen = Screen::Failed(format!("Could not extract the package: {error}"));
            return;
        }
        match read_manifest(temp.path()) {
            Ok(manifest) => self.apply_manifest(&manifest),
            Err(error) => {
                self.screen = Screen::Failed(format!("Could not read the manifest: {error}"));
                return;
            }
        }
        self.package_root = temp.path().to_path_buf();
        self._temp = Some(temp);
        self.screen = Screen::Ready;
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

    fn start(&mut self, apply: bool, ctx: &egui::Context) {
        if self.phase == Phase::Running {
            return;
        }
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        self.phase = Phase::Running;
        self.applied_now = apply;
        self.outcome = None;
        self.error = None;
        self.detail = if apply {
            "Installing the application, its data, and its dependencies…".to_owned()
        } else {
            "Previewing the install…".to_owned()
        };
        let options = RestoreOptions {
            package_root: self.package_root.clone(),
            target_root: PathBuf::from(self.target_root.trim()),
            dry_run: !apply,
            apply,
            force: true,
            restore_user_data: self.show_user_data && self.restore_user_data,
            restore_license: self.show_license && self.restore_license,
        };
        let ctx = ctx.clone();
        thread::spawn(move || {
            let message = match restore_package(&options) {
                Ok(result) => Msg::Done(Box::new(result)),
                Err(error) => Msg::Failed(error.to_string()),
            };
            let _ = sender.send(message);
            ctx.request_repaint();
        });
    }

    fn poll(&mut self) {
        let Some(receiver) = self.receiver.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(Msg::Done(result)) => {
                self.outcome = Some(outcome_from(&result, self.applied_now));
                self.phase = Phase::Done;
                self.receiver = None;
            }
            Ok(Msg::Failed(error)) => {
                self.error = Some(error);
                self.phase = Phase::Done;
                self.receiver = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.receiver = None;
                self.phase = Phase::Done;
            }
        }
    }
}

impl eframe::App for InstallerApp {
    /// Clear the framebuffer to the theme background so the window matches the
    /// active theme (the installer draws directly, without a filled panel, so the
    /// default near-black clear color would otherwise show through in light mode).
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.theme.bg.to_normalized_gamma_f32()
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Clone the theme so widget calls (which borrow it) don't conflict with
        // the mutable field access the form controls need.
        let theme = self.theme.clone();
        let ctx = ui.ctx().clone();
        let mut do_preview = false;
        let mut do_install = false;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Inset the content so cards never sit flush against the window
                // edge (the scrollbar stays at the edge, the content is guttered).
                egui::Frame::new()
                    .inner_margin(egui::Margin {
                        left: 22,
                        right: 22,
                        top: 16,
                        bottom: 18,
                    })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let (rect, _) = ui
                                .allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::hover());
                            widgets::paint_logo(ui.painter(), rect, &theme);
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(WINDOW_TITLE)
                                    .size(18.0)
                                    .strong()
                                    .color(theme.text),
                            );
                        });
                        ui.add_space(12.0);

                        // Read the screen into an owned value so the immutable borrow
                        // of `self.screen` is released before `ready_ui` borrows
                        // `self` mutably.
                        enum Kind {
                            NoPackage,
                            Failed,
                            Ready,
                        }
                        let (kind, failed) = match &self.screen {
                            Screen::NoPackage => (Kind::NoPackage, String::new()),
                            Screen::Failed(reason) => (Kind::Failed, reason.clone()),
                            Screen::Ready => (Kind::Ready, String::new()),
                        };
                        match kind {
                            Kind::NoPackage => widgets::notice(
                                ui,
                                &theme,
                                Role::Accent,
                                "This is a WinAppMigrator installer stub — it carries no \
                                 application package. Build an installer with WinAppMigrator, \
                                 then run the produced .exe here.",
                            ),
                            Kind::Failed => widgets::notice(ui, &theme, Role::Danger, &failed),
                            Kind::Ready => {
                                self.ready_ui(ui, &theme, &mut do_preview, &mut do_install);
                            }
                        }
                        ui.add_space(8.0);
                    });
            });

        if do_preview {
            self.start(false, &ctx);
        }
        if do_install {
            self.start(true, &ctx);
        }
    }
}

impl InstallerApp {
    fn ready_ui(
        &mut self,
        ui: &mut egui::Ui,
        theme: &Theme,
        do_preview: &mut bool,
        do_install: &mut bool,
    ) {
        let subtitle = [self.publisher.as_str(), self.version.as_str()]
            .iter()
            .filter(|part| !part.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" · ");
        let subtitle = (!subtitle.is_empty()).then_some(subtitle);
        widgets::card_titled(ui, theme, &self.title, subtitle.as_deref(), |ui| {
            if !self.source_computer.is_empty() {
                widgets::kv_row(ui, theme, "Captured from", &self.source_computer);
            }
            ui.add_space(8.0);
            widgets::path_field(
                ui,
                theme,
                "INSTALL LOCATION",
                &mut self.target_root,
                r"C:\Program Files\App",
                || {
                    rfd::FileDialog::new()
                        .set_title("Choose the install location")
                        .pick_folder()
                },
            );
        });
        ui.add_space(10.0);

        if self.show_user_data || self.show_license {
            widgets::card_titled(ui, theme, "Options", None, |ui| {
                if self.show_user_data {
                    ui.checkbox(
                        &mut self.restore_user_data,
                        "Restore user data (settings and per-user/ProgramData files)",
                    );
                }
                if self.show_license {
                    ui.checkbox(
                        &mut self.restore_license,
                        "Restore license (activation / license files and keys)",
                    );
                }
            });
            ui.add_space(10.0);
        }

        widgets::card_titled(
            ui,
            theme,
            "Dependencies",
            Some("Installed automatically as part of this installer."),
            |ui| {
                if self.prereqs.is_empty() {
                    ui.label(
                        egui::RichText::new("No additional runtime dependencies were detected.")
                            .size(13.0)
                            .color(theme.text_muted),
                    );
                }
                for prereq in &self.prereqs {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(&prereq.name)
                                .size(13.5)
                                .color(theme.text),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            widgets::status_pill(ui, theme, &prereq.handling, prereq.role);
                        });
                    });
                    ui.add_space(4.0);
                }
            },
        );
        ui.add_space(12.0);

        let running = self.phase == Phase::Running;
        ui.horizontal(|ui| {
            if widgets::button(ui, theme, ButtonKind::Primary, "Install", !running).clicked() {
                *do_install = true;
            }
            ui.add_space(8.0);
            if widgets::button(ui, theme, ButtonKind::Secondary, "Preview", !running).clicked() {
                *do_preview = true;
            }
            if running {
                ui.add_space(10.0);
                ui.add(egui::Spinner::new());
                ui.label(
                    egui::RichText::new(&self.detail)
                        .size(12.5)
                        .color(theme.text_muted),
                );
            }
        });

        if let Some(error) = self.error.as_ref() {
            ui.add_space(10.0);
            let error = error.clone();
            widgets::notice(ui, theme, Role::Danger, &error);
        }

        if let Some(outcome) = self.outcome.as_ref() {
            ui.add_space(12.0);
            self.outcome_ui(ui, theme, outcome);
        }
    }

    fn outcome_ui(&self, ui: &mut egui::Ui, theme: &Theme, outcome: &Outcome) {
        let heading = if outcome.applied {
            "Install result"
        } else {
            "Preview result"
        };
        widgets::card_titled(ui, theme, heading, None, |ui| {
            widgets::metric_tiles(
                ui,
                theme,
                &[
                    metric("Changes", outcome.changes.to_string(), Role::Accent),
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
            ui.horizontal(|ui| {
                widgets::status_pill(
                    ui,
                    theme,
                    if outcome.package_valid {
                        "Package hashes valid"
                    } else {
                        "Package hashes INVALID"
                    },
                    if outcome.package_valid {
                        Role::Success
                    } else {
                        Role::Danger
                    },
                );
                if let Some(valid) = outcome.restore_valid {
                    ui.add_space(6.0);
                    widgets::status_pill(
                        ui,
                        theme,
                        if valid {
                            "Install hashes valid"
                        } else {
                            "Install hashes INVALID"
                        },
                        if valid { Role::Success } else { Role::Danger },
                    );
                }
            });
            if outcome.reboot {
                ui.add_space(8.0);
                widgets::notice(
                    ui,
                    theme,
                    Role::Warning,
                    "A dependency needs a reboot to finish. Restart Windows, then launch the app.",
                );
            }
            if outcome.applied && outcome.errors == 0 && outcome.package_valid {
                ui.add_space(8.0);
                widgets::notice(
                    ui,
                    theme,
                    Role::Success,
                    "The application was installed and verified. You can launch it now.",
                );
            }
            for (role, message) in &outcome.messages {
                ui.add_space(6.0);
                widgets::notice(ui, theme, *role, message);
            }
        });
    }
}

fn prereq_row(prereq: &RuntimePrerequisite) -> PrereqRow {
    let (handling, role) = if prereq.present_on_source == Some(true) {
        ("already present".to_owned(), Role::Success)
    } else if let Some(bundle) = prereq.bundle.as_ref() {
        if let Some(feature) = bundle.windows_feature.as_deref() {
            (format!("Windows feature: {feature}"), Role::Accent)
        } else if bundle.package_relative_path.is_some() {
            ("bundled installer".to_owned(), Role::Accent)
        } else if let Some(id) = bundle.winget_id.as_deref() {
            (format!("winget: {id}"), Role::Accent)
        } else {
            ("download & install".to_owned(), Role::Accent)
        }
    } else if matches!(prereq.kind, RuntimePrerequisiteKind::DotNetFramework4x) {
        ("built into Windows 10/11".to_owned(), Role::Success)
    } else if !prereq.install_manually {
        ("ships with the app".to_owned(), Role::Success)
    } else {
        ("install manually".to_owned(), Role::Warning)
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
    let (package_valid, restore_valid) = result
        .verification
        .as_ref()
        .map(|report| (report.package_valid, report.restore_valid))
        .unwrap_or((true, None));
    Outcome {
        applied,
        changes: result.changes_logged.len(),
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
