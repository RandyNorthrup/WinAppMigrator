#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod views;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui::{self, Align, Layout, Margin, RichText, Stroke};

use winappmigrator_core::{
    AppSelection, InstalledApp, PackageOptions, PackageResult, VerificationOptions,
    VerificationReport, create_package, list_installed_apps, verify_package,
};
use winappmigrator_ui::{theme, widgets};

use theme::{Role, Theme, ThemeMode};
use widgets::ButtonKind;

const DEFAULT_WINDOW_WIDTH: f32 = 1180.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 820.0;
const WINDOW_TITLE: &str = "WinAppMigrator";

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .with_min_inner_size([900.0, 620.0])
            .with_app_id("winappmigrator"),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| {
            let app = WinAppMigratorApp::new();
            app.theme.apply(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScopeFilter {
    All,
    PerUser,
    PerMachine,
}

/// The application is a single screen: pick an installed app, choose whether to
/// also back up its license and user data, then build a self-extracting
/// installer that reinstalls it on another device and hash-verifies the result.
pub struct WinAppMigratorApp {
    theme: Theme,
    inventory_state: InventoryState,
    inventory_receiver: Option<Receiver<InventoryMessage>>,
    apps: Vec<InstalledApp>,
    selected_app_index: Option<usize>,
    app_search: String,
    app_scope_filter: ScopeFilter,
    output_dir: String,
    backup_user_data: bool,
    backup_license: bool,
    migrate_state: OperationState,
    migrate_receiver: Option<Receiver<MigrateMessage>>,
    migrate_detail: String,
    last_outcome: Option<MigrationOutcome>,
    last_error: Option<String>,
}

impl WinAppMigratorApp {
    fn new() -> Self {
        let mut app = Self {
            theme: Theme::new(ThemeMode::Dark),
            inventory_state: InventoryState::Loading,
            inventory_receiver: None,
            apps: Vec::new(),
            selected_app_index: None,
            app_search: String::new(),
            app_scope_filter: ScopeFilter::All,
            output_dir: String::new(),
            backup_user_data: false,
            backup_license: false,
            migrate_state: OperationState::Idle,
            migrate_receiver: None,
            migrate_detail: String::new(),
            last_outcome: None,
            last_error: None,
        };
        app.start_inventory_load();
        app
    }

    fn start_inventory_load(&mut self) {
        self.inventory_state = InventoryState::Loading;
        let (sender, receiver) = mpsc::channel();
        self.inventory_receiver = Some(receiver);
        thread::spawn(move || {
            let message = match list_installed_apps() {
                Ok(apps) => InventoryMessage::Loaded(apps),
                Err(error) => InventoryMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
        });
    }

    /// Reverse-engineer the selected application and build a self-extracting
    /// installer, then hash-verify the package. Analysis is part of this flow —
    /// it is not a separate step the user has to run first.
    fn start_migration(&mut self) {
        let Some(index) = self.selected_app_index else {
            self.last_error = Some("Select an application to migrate first.".to_owned());
            return;
        };
        let Some(app) = self.apps.get(index).cloned() else {
            self.last_error = Some("The selected application is no longer available.".to_owned());
            return;
        };

        // The product is a self-extracting installer, so the stub that becomes
        // that installer must be present. Without it we cannot build anything.
        let Some(stub) = installer_stub_path() else {
            self.last_error = Some(
                "Installer stub (winappmigrator-installer.exe) was not found beside this app, so a \
                 self-extracting installer cannot be built."
                    .to_owned(),
            );
            return;
        };

        if self.output_dir.trim().is_empty() {
            self.output_dir = default_output_for(&app.display_name);
        }
        // The chosen folder receives only the single self-extracting installer.
        let installer_dir = PathBuf::from(self.output_dir.trim());
        let base_name = installer_dir
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "App".to_owned());
        let app_name = app.display_name.clone();
        let include_user_data = self.backup_user_data;
        let include_license_data = self.backup_license;

        let (sender, receiver) = mpsc::channel();
        self.migrate_receiver = Some(receiver);
        self.migrate_state = OperationState::Running;
        self.migrate_detail =
            "Reverse-engineering the app, copying its files, and building the installer…"
                .to_owned();
        self.last_error = None;
        self.last_outcome = None;

        thread::spawn(move || {
            // Build the package in a throwaway staging directory so the chosen
            // output folder ends up holding only the single installer executable.
            let staging_parent =
                std::env::temp_dir().join(format!("winappmigrator-build-{}", std::process::id()));
            let staging = staging_parent.join(&base_name);

            let options = PackageOptions {
                selection: AppSelection::InstalledApp(Box::new(app)),
                output_dir: staging.clone(),
                current_directory: std::env::current_dir().ok(),
                // The deliverable is a single self-contained .exe; no loose zip.
                archive_format: None,
                create_self_extracting_archive: true,
                self_extracting_stub: Some(stub),
                installer_dir: Some(installer_dir),
                include_user_data,
                // ProgramData is machine-wide application data; back it up
                // together with per-user data under the one "user data" choice.
                include_program_data: include_user_data,
                include_license_data,
                // Download and embed runtime prerequisites so the one installer
                // installs everything on the target, even offline.
                bundle_prerequisites: true,
            };

            let message = match create_package(&options) {
                Ok(package) => {
                    let verify_options = VerificationOptions {
                        package_root: package.package_root.clone(),
                        target_root: None,
                        // Keep the verification report in staging (discarded) so
                        // only the installer remains in the chosen folder.
                        output_dir: Some(staging.clone()),
                        restore_user_data: false,
                        restore_license: include_license_data,
                    };
                    match verify_package(&verify_options) {
                        Ok(verification) => MigrateMessage::Completed(Box::new(MigrationOutcome {
                            app_name,
                            package,
                            verification,
                        })),
                        Err(error) => MigrateMessage::Failed(format!(
                            "Installer was built but hash verification failed: {error}"
                        )),
                    }
                }
                Err(error) => MigrateMessage::Failed(error.to_string()),
            };
            let _ = std::fs::remove_dir_all(&staging_parent);
            let _send_result = sender.send(message);
        });
    }

    fn poll_workers(&mut self, ctx: &egui::Context) {
        if let Some(receiver) = self.inventory_receiver.take() {
            match receiver.try_recv() {
                Ok(InventoryMessage::Loaded(apps)) => {
                    self.apps = apps;
                    self.inventory_state = InventoryState::Loaded;
                }
                Ok(InventoryMessage::Failed(error)) => {
                    self.inventory_state = InventoryState::Failed(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.inventory_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.inventory_state =
                        InventoryState::Failed("Inventory worker disconnected.".to_owned());
                }
            }
        }

        if let Some(receiver) = self.migrate_receiver.take() {
            match receiver.try_recv() {
                Ok(MigrateMessage::Completed(outcome)) => {
                    let warnings = outcome.package.warnings.len();
                    self.migrate_state = if warnings == 0 {
                        OperationState::Completed
                    } else {
                        OperationState::Warning
                    };
                    self.migrate_detail = format!(
                        "Built installer with {} files; package hashes {}.",
                        outcome.package.files_copied,
                        if outcome.verification.package_valid {
                            "verified"
                        } else {
                            "did NOT verify"
                        }
                    );
                    self.last_outcome = Some(*outcome);
                }
                Ok(MigrateMessage::Failed(error)) => {
                    self.migrate_state = OperationState::Failed;
                    self.migrate_detail = "Failed.".to_owned();
                    self.last_error = Some(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.migrate_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.migrate_state = OperationState::Failed;
                    self.last_error = Some("Migration worker disconnected.".to_owned());
                }
            }
        }
    }

    fn draw_header(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::hover());
            widgets::paint_logo(ui.painter(), rect, &theme);
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("WinAppMigrator")
                        .size(18.0)
                        .strong()
                        .color(theme.text),
                );
                ui.label(
                    RichText::new("Turn an installed app into a self-extracting installer")
                        .size(12.5)
                        .color(theme.text_muted),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let mode_label = if theme.mode == ThemeMode::Dark {
                    "Light mode"
                } else {
                    "Dark mode"
                };
                if widgets::button(ui, &theme, ButtonKind::Ghost, mode_label, true).clicked() {
                    self.theme = Theme::new(theme.mode.toggled());
                    self.theme.apply(ui.ctx());
                }
                let loading = matches!(self.inventory_state, InventoryState::Loading);
                if widgets::button(ui, &theme, ButtonKind::Secondary, "Reload apps", !loading)
                    .clicked()
                {
                    self.start_inventory_load();
                }
                let (label, role) = match &self.inventory_state {
                    InventoryState::Loading => ("Loading inventory".to_owned(), Role::Neutral),
                    InventoryState::Loaded => (format!("{} apps", self.apps.len()), Role::Success),
                    InventoryState::Failed(_) => ("Inventory failed".to_owned(), Role::Danger),
                };
                widgets::status_pill(ui, &theme, &label, role);
            });
        });
    }
}

impl eframe::App for WinAppMigratorApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_workers(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = self.theme.clone();

        let top_frame = egui::Frame::new()
            .fill(theme.surface)
            .stroke(Stroke::new(1.0, theme.border))
            .inner_margin(Margin::symmetric(24, 16));
        egui::Panel::top("header")
            .frame(top_frame)
            .show_inside(ui, |ui| self.draw_header(ui));

        let central_frame = egui::Frame::new()
            .fill(theme.bg)
            .inner_margin(Margin::symmetric(24, 18));
        egui::CentralPanel::default()
            .frame(central_frame)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        self.view_applications(ui);
                    });
            });
    }
}

pub(crate) fn open_folder(path: &Path) -> std::io::Result<()> {
    Command::new("explorer").arg(path).spawn().map(|_child| ())
}

/// A sensible default installer output directory for an application, so the
/// common path does not require the user to choose a folder manually.
pub(crate) fn default_output_for(app_name: &str) -> String {
    let safe: String = app_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, ' ' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    let safe = safe.trim();
    let base = std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("WinAppMigrator")
        .join(if safe.is_empty() { "Package" } else { safe })
        .display()
        .to_string()
}

/// The GUI installer stub, embedded into the app at build time (see `build.rs`).
/// Empty only on a first-pass build where the stub was not yet available; the
/// runtime then falls back to a sibling stub file.
const EMBEDDED_INSTALLER_STUB: &[u8] = include_bytes!(env!("WAM_INSTALLER_STUB"));

/// Resolve the installer stub used to build a self-extracting installer.
///
/// WinAppMigrator is one portable executable, so the stub normally lives inside
/// it: we write the embedded bytes to a temp file and hand that back. When the
/// stub was not embedded (a bootstrap build), fall back to a sibling
/// `winappmigrator-installer.exe`.
pub(crate) fn installer_stub_path() -> Option<PathBuf> {
    if !EMBEDDED_INSTALLER_STUB.is_empty() {
        return write_embedded_stub().ok();
    }
    let current = std::env::current_exe().ok()?;
    let sibling = current.parent()?.join("winappmigrator-installer.exe");
    sibling.is_file().then_some(sibling)
}

/// Write the embedded installer stub to a per-process temp file (once) and
/// return its path, so it can be used as the self-extracting archive stub.
fn write_embedded_stub() -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("winappmigrator-stub-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("winappmigrator-installer.exe");
    let already_written = std::fs::metadata(&path)
        .map(|meta| meta.len() == EMBEDDED_INSTALLER_STUB.len() as u64)
        .unwrap_or(false);
    if !already_written {
        std::fs::write(&path, EMBEDDED_INSTALLER_STUB)?;
    }
    Ok(path)
}

/// The combined result of a migration build: the package, its self-extracting
/// installer, and the hash verification that confirms package fidelity.
pub(crate) struct MigrationOutcome {
    pub app_name: String,
    pub package: PackageResult,
    pub verification: VerificationReport,
}

enum InventoryMessage {
    Loaded(Vec<InstalledApp>),
    Failed(String),
}

enum MigrateMessage {
    Completed(Box<MigrationOutcome>),
    Failed(String),
}

pub(crate) enum InventoryState {
    Loading,
    Loaded,
    Failed(String),
}

#[derive(Clone, Copy)]
pub(crate) enum OperationState {
    Idle,
    Running,
    Completed,
    Warning,
    Failed,
}
