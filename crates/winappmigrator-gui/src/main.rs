#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod theme;
mod views;
mod widgets;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui::{self, Align, Layout, Margin, RichText, Stroke};

use winappmigrator_core::{
    InstalledApp, InventorySnapshotOptions, InventorySnapshotReport, PackageArchiveFormat,
    PackageOptions, PackageResult, RestoreOptions, RestoreResult, ScanOptions, ScanReport,
    create_package, list_installed_apps, restore_package, scan_and_write_reports,
    write_inventory_snapshot,
};

use theme::{NAV_WIDTH, Role, Theme, ThemeMode};
use widgets::{ButtonKind, Icon};

const DEFAULT_WINDOW_WIDTH: f32 = 1240.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 820.0;
const WINDOW_TITLE: &str = "WinAppMigrator";

const NAV_ITEMS: [(View, Icon, &str); 5] = [
    (View::Dashboard, Icon::Dashboard, "Dashboard"),
    (View::Applications, Icon::Apps, "Applications"),
    (View::Analyze, Icon::Analyze, "Analyze"),
    (View::Package, Icon::Package, "Package"),
    (View::Restore, Icon::Restore, "Restore"),
];

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .with_min_inner_size([960.0, 640.0])
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
pub enum View {
    Dashboard,
    Applications,
    Analyze,
    Package,
    Restore,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScopeFilter {
    All,
    PerUser,
    PerMachine,
}

pub struct WinAppMigratorApp {
    theme: Theme,
    active_view: View,
    inventory_state: InventoryState,
    inventory_receiver: Option<Receiver<InventoryMessage>>,
    apps: Vec<InstalledApp>,
    selected_app_index: Option<usize>,
    app_search: String,
    app_scope_filter: ScopeFilter,
    executable_path: String,
    output_dir: String,
    restore_package_dir: String,
    restore_target_dir: String,
    scan_recursive_dependencies: bool,
    scan_include_hashes: bool,
    scan_include_user_data: bool,
    package_zip: bool,
    package_sfx: bool,
    restore_apply: bool,
    restore_force: bool,
    scan_state: ScanState,
    package_state: OperationState,
    restore_state: OperationState,
    snapshot_state: OperationState,
    scan_receiver: Option<Receiver<ScanMessage>>,
    package_receiver: Option<Receiver<PackageMessage>>,
    restore_receiver: Option<Receiver<RestoreMessage>>,
    snapshot_receiver: Option<Receiver<SnapshotMessage>>,
    scan_detail: String,
    package_detail: String,
    restore_detail: String,
    snapshot_detail: String,
    last_report: Option<ScanReport>,
    last_package: Option<PackageResult>,
    last_restore: Option<RestoreResult>,
    last_snapshot: Option<InventorySnapshotReport>,
    last_error: Option<String>,
}

impl WinAppMigratorApp {
    fn new() -> Self {
        let mut app = Self {
            theme: Theme::new(ThemeMode::Dark),
            active_view: View::Dashboard,
            inventory_state: InventoryState::Loading,
            inventory_receiver: None,
            apps: Vec::new(),
            selected_app_index: None,
            app_search: String::new(),
            app_scope_filter: ScopeFilter::All,
            executable_path: String::new(),
            output_dir: String::new(),
            restore_package_dir: String::new(),
            restore_target_dir: String::new(),
            scan_recursive_dependencies: true,
            scan_include_hashes: true,
            scan_include_user_data: true,
            package_zip: false,
            package_sfx: false,
            restore_apply: false,
            restore_force: false,
            scan_state: ScanState::Idle,
            package_state: OperationState::Idle,
            restore_state: OperationState::Idle,
            snapshot_state: OperationState::Idle,
            scan_receiver: None,
            package_receiver: None,
            restore_receiver: None,
            snapshot_receiver: None,
            scan_detail: String::new(),
            package_detail: String::new(),
            restore_detail: String::new(),
            snapshot_detail: String::new(),
            last_report: None,
            last_package: None,
            last_restore: None,
            last_snapshot: None,
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

    fn start_scan(&mut self) {
        let executable_path = self.executable_path.trim().to_owned();
        let output_dir = self.output_dir.trim().to_owned();
        if executable_path.is_empty() || output_dir.is_empty() {
            self.last_error = Some("Executable path and output directory are required.".to_owned());
            return;
        }

        let (sender, receiver) = mpsc::channel();
        self.scan_receiver = Some(receiver);
        self.scan_state = ScanState::Running;
        self.scan_detail =
            "Reading PE imports, resolving DLLs, and discovering integrations…".to_owned();
        self.last_error = None;
        self.last_report = None;
        let recursive_dependencies = self.scan_recursive_dependencies;
        let include_hashes = self.scan_include_hashes;
        let include_user_data = self.scan_include_user_data;

        thread::spawn(move || {
            let mut options = ScanOptions::for_executable(executable_path, output_dir);
            options.recursive_dependencies = recursive_dependencies;
            options.include_hashes = include_hashes;
            options.include_user_data = include_user_data;
            let message = match scan_and_write_reports(&options) {
                Ok(report) => ScanMessage::Completed(Box::new(report)),
                Err(error) => ScanMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
        });
    }

    fn start_package(&mut self) {
        let executable_path = self.executable_path.trim().to_owned();
        let output_dir = self.output_dir.trim().to_owned();
        if executable_path.is_empty() || output_dir.is_empty() {
            self.last_error =
                Some("Executable path and package output directory are required.".to_owned());
            return;
        }

        let (sender, receiver) = mpsc::channel();
        self.package_receiver = Some(receiver);
        self.package_state = OperationState::Running;
        self.package_detail =
            "Scanning, copying app-owned files, and writing package artifacts…".to_owned();
        self.last_error = None;
        self.last_package = None;
        let package_zip = self.package_zip;
        let package_sfx = self.package_sfx;
        let include_user_data = self.scan_include_user_data;
        let sfx_stub = if package_sfx { cli_stub_path() } else { None };

        thread::spawn(move || {
            let mut options = PackageOptions::for_executable(executable_path, output_dir);
            options.current_directory = std::env::current_dir().ok();
            options.include_user_data = include_user_data;
            options.archive_format = if package_zip || package_sfx {
                Some(PackageArchiveFormat::Zip)
            } else {
                None
            };
            options.create_self_extracting_archive = package_sfx;
            options.self_extracting_stub = sfx_stub;
            let message = match create_package(&options) {
                Ok(result) => PackageMessage::Completed(Box::new(result)),
                Err(error) => PackageMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
        });
    }

    fn start_restore(&mut self) {
        let package_root = self.restore_package_dir.trim().to_owned();
        let target_root = self.restore_target_dir.trim().to_owned();
        if package_root.is_empty() || target_root.is_empty() {
            self.last_error = Some("Package root and restore target root are required.".to_owned());
            return;
        }

        let (sender, receiver) = mpsc::channel();
        self.restore_receiver = Some(receiver);
        self.restore_state = OperationState::Running;
        let apply = self.restore_apply;
        let force = self.restore_force;
        self.restore_detail = if apply {
            "Applying restore plan and running post-restore verification…".to_owned()
        } else {
            "Building dry-run restore plan…".to_owned()
        };
        self.last_error = None;
        self.last_restore = None;

        thread::spawn(move || {
            let options = RestoreOptions {
                package_root: PathBuf::from(package_root),
                target_root: PathBuf::from(target_root),
                dry_run: !apply,
                apply,
                force,
            };
            let message = match restore_package(&options) {
                Ok(result) => RestoreMessage::Completed(Box::new(result)),
                Err(error) => RestoreMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
        });
    }

    fn start_snapshot(&mut self) {
        let output_dir = self.output_dir.trim().to_owned();
        if output_dir.is_empty() {
            self.last_error =
                Some("Output directory is required for inventory snapshot.".to_owned());
            return;
        }

        let (sender, receiver) = mpsc::channel();
        self.snapshot_receiver = Some(receiver);
        self.snapshot_state = OperationState::Running;
        self.snapshot_detail = "Reading installed application inventory…".to_owned();
        self.last_error = None;
        self.last_snapshot = None;

        thread::spawn(move || {
            let options = InventorySnapshotOptions {
                output_dir: PathBuf::from(output_dir),
            };
            let message = match write_inventory_snapshot(&options) {
                Ok(result) => SnapshotMessage::Completed(Box::new(result)),
                Err(error) => SnapshotMessage::Failed(error.to_string()),
            };
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

        if let Some(receiver) = self.scan_receiver.take() {
            match receiver.try_recv() {
                Ok(ScanMessage::Completed(report)) => {
                    self.scan_state = if report.warnings.is_empty() {
                        ScanState::Completed
                    } else {
                        ScanState::Warning
                    };
                    self.scan_detail = format!(
                        "Completed with {} dependencies, {} files, and {} warnings.",
                        report.dependencies.len(),
                        report.file_inventory.len(),
                        report.warnings.len()
                    );
                    self.last_report = Some(*report);
                }
                Ok(ScanMessage::Failed(error)) => {
                    self.scan_state = ScanState::Failed;
                    self.scan_detail = "Failed.".to_owned();
                    self.last_error = Some(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.scan_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.scan_state = ScanState::Failed;
                    self.last_error = Some("Scan worker disconnected.".to_owned());
                }
            }
        }

        if let Some(receiver) = self.package_receiver.take() {
            match receiver.try_recv() {
                Ok(PackageMessage::Completed(result)) => {
                    self.package_state = if result.warnings.is_empty() {
                        OperationState::Completed
                    } else {
                        OperationState::Warning
                    };
                    self.package_detail = format!(
                        "Completed with {} copied files and {} warnings.",
                        result.files_copied,
                        result.warnings.len()
                    );
                    self.restore_package_dir = result
                        .archive_path
                        .as_ref()
                        .unwrap_or(&result.package_root)
                        .display()
                        .to_string();
                    self.last_package = Some(*result);
                }
                Ok(PackageMessage::Failed(error)) => {
                    self.package_state = OperationState::Failed;
                    self.package_detail = "Failed.".to_owned();
                    self.last_error = Some(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.package_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.package_state = OperationState::Failed;
                    self.last_error = Some("Package worker disconnected.".to_owned());
                }
            }
        }

        if let Some(receiver) = self.restore_receiver.take() {
            match receiver.try_recv() {
                Ok(RestoreMessage::Completed(result)) => {
                    self.restore_state = if result.errors.is_empty() {
                        OperationState::Completed
                    } else {
                        OperationState::Warning
                    };
                    self.restore_detail = format!(
                        "Completed with {} logged changes, {} warnings, and {} errors.",
                        result.changes_logged.len(),
                        result.warnings.len(),
                        result.errors.len()
                    );
                    self.last_restore = Some(*result);
                }
                Ok(RestoreMessage::Failed(error)) => {
                    self.restore_state = OperationState::Failed;
                    self.restore_detail = "Failed.".to_owned();
                    self.last_error = Some(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.restore_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.restore_state = OperationState::Failed;
                    self.last_error = Some("Restore worker disconnected.".to_owned());
                }
            }
        }

        if let Some(receiver) = self.snapshot_receiver.take() {
            match receiver.try_recv() {
                Ok(SnapshotMessage::Completed(snapshot)) => {
                    self.snapshot_state = OperationState::Completed;
                    self.snapshot_detail = format!(
                        "Completed with {} installed applications.",
                        snapshot.applications.len()
                    );
                    self.last_snapshot = Some(*snapshot);
                }
                Ok(SnapshotMessage::Failed(error)) => {
                    self.snapshot_state = OperationState::Failed;
                    self.snapshot_detail = "Failed.".to_owned();
                    self.last_error = Some(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.snapshot_receiver = Some(receiver);
                    ctx.request_repaint();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.snapshot_state = OperationState::Failed;
                    self.snapshot_detail = "Snapshot worker disconnected.".to_owned();
                    self.last_error = Some("Snapshot worker disconnected.".to_owned());
                }
            }
        }
    }

    fn draw_nav(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        ui.add_space(2.0);
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(34.0, 34.0), egui::Sense::hover());
            widgets::paint_logo(ui.painter(), rect, &theme);
            ui.add_space(6.0);
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("WinAppMigrator")
                        .size(15.0)
                        .strong()
                        .color(theme.text),
                );
                ui.label(
                    RichText::new("Migration console")
                        .size(11.0)
                        .color(theme.text_subtle),
                );
            });
        });

        ui.add_space(18.0);
        widgets::eyebrow(ui, &theme, "Workspace");
        ui.add_space(8.0);

        for (view, icon, label) in NAV_ITEMS {
            if widgets::nav_item(ui, &theme, icon, label, self.active_view == view) {
                self.active_view = view;
            }
            ui.add_space(2.0);
        }

        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            ui.add_space(2.0);
            ui.label(
                RichText::new(format!("v{} · local build", env!("CARGO_PKG_VERSION")))
                    .size(11.0)
                    .color(theme.text_subtle),
            );
            ui.add_space(8.0);
            ui.separator();
        });
    }

    fn draw_topbar(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let (title, subtitle) = view_meta(self.active_view);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                widgets::eyebrow(ui, &theme, "WinAppMigrator");
                ui.label(RichText::new(title).heading().color(theme.text));
                ui.label(RichText::new(subtitle).size(13.0).color(theme.text_muted));
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
                if widgets::button(ui, &theme, ButtonKind::Secondary, "Reload apps", true).clicked()
                {
                    self.start_inventory_load();
                }
            });
        });

        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            let (inventory_label, inventory_role) = match &self.inventory_state {
                InventoryState::Loading => ("Inventory loading".to_owned(), Role::Neutral),
                InventoryState::Loaded => (format!("{} apps", self.apps.len()), Role::Success),
                InventoryState::Failed(_) => ("Inventory failed".to_owned(), Role::Danger),
            };
            widgets::status_pill(ui, &theme, &inventory_label, inventory_role);
            widgets::status_pill(
                ui,
                &theme,
                &format!("Scan · {}", scan_text(self.scan_state)),
                scan_role(self.scan_state),
            );
            widgets::status_pill(
                ui,
                &theme,
                &op_text("Package", self.package_state),
                op_role(self.package_state),
            );
            widgets::status_pill(
                ui,
                &theme,
                &op_text("Restore", self.restore_state),
                op_role(self.restore_state),
            );
            widgets::status_pill(
                ui,
                &theme,
                &op_text("Snapshot", self.snapshot_state),
                op_role(self.snapshot_state),
            );
        });
    }
}

impl eframe::App for WinAppMigratorApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_workers(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = self.theme.clone();

        let nav_frame = egui::Frame::new()
            .fill(theme.sidebar)
            .stroke(Stroke::new(1.0, theme.border))
            .inner_margin(Margin::symmetric(16, 18));
        egui::Panel::left("nav")
            .exact_size(NAV_WIDTH)
            .resizable(false)
            .frame(nav_frame)
            .show_inside(ui, |ui| self.draw_nav(ui));

        let top_frame = egui::Frame::new()
            .fill(theme.surface)
            .inner_margin(Margin::symmetric(22, 16));
        egui::Panel::top("topbar")
            .frame(top_frame)
            .show_inside(ui, |ui| self.draw_topbar(ui));

        let central_frame = egui::Frame::new()
            .fill(theme.bg)
            .inner_margin(Margin::symmetric(22, 18));
        egui::CentralPanel::default()
            .frame(central_frame)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        self.draw_active_view(ui);
                    });
            });
    }
}

pub(crate) fn open_folder(path: &Path) -> std::io::Result<()> {
    Command::new("explorer").arg(path).spawn().map(|_child| ())
}

/// A sensible default package/report output directory for an application,
/// so the common path does not require the user to choose a folder manually.
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

pub(crate) fn cli_stub_path() -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    let sibling = current.parent()?.join("winappmigrator.exe");
    sibling.is_file().then_some(sibling)
}

fn view_meta(view: View) -> (&'static str, &'static str) {
    match view {
        View::Dashboard => ("Dashboard", "Overview of inventory and migration progress"),
        View::Applications => ("Applications", "Browse and select installed applications"),
        View::Analyze => ("Analyze", "Resolve dependencies and Windows integrations"),
        View::Package => ("Package", "Build a portable migration package"),
        View::Restore => ("Restore", "Preview or apply a package on the target"),
    }
}

pub(crate) fn scan_text(state: ScanState) -> &'static str {
    match state {
        ScanState::Idle => "Idle",
        ScanState::Running => "Running",
        ScanState::Completed => "Complete",
        ScanState::Warning => "Warnings",
        ScanState::Failed => "Failed",
    }
}

pub(crate) fn scan_role(state: ScanState) -> Role {
    match state {
        ScanState::Idle => Role::Neutral,
        ScanState::Running => Role::Accent,
        ScanState::Completed => Role::Success,
        ScanState::Warning => Role::Warning,
        ScanState::Failed => Role::Danger,
    }
}

pub(crate) fn op_word(state: OperationState) -> &'static str {
    match state {
        OperationState::Idle => "Idle",
        OperationState::Running => "Running",
        OperationState::Completed => "Complete",
        OperationState::Warning => "Warnings",
        OperationState::Failed => "Failed",
    }
}

pub(crate) fn op_text(prefix: &str, state: OperationState) -> String {
    format!("{prefix} · {}", op_word(state))
}

pub(crate) fn op_role(state: OperationState) -> Role {
    match state {
        OperationState::Idle => Role::Neutral,
        OperationState::Running => Role::Accent,
        OperationState::Completed => Role::Success,
        OperationState::Warning => Role::Warning,
        OperationState::Failed => Role::Danger,
    }
}

enum InventoryMessage {
    Loaded(Vec<InstalledApp>),
    Failed(String),
}

enum ScanMessage {
    Completed(Box<ScanReport>),
    Failed(String),
}

enum PackageMessage {
    Completed(Box<PackageResult>),
    Failed(String),
}

enum RestoreMessage {
    Completed(Box<RestoreResult>),
    Failed(String),
}

enum SnapshotMessage {
    Completed(Box<InventorySnapshotReport>),
    Failed(String),
}

pub(crate) enum InventoryState {
    Loading,
    Loaded,
    Failed(String),
}

#[derive(Clone, Copy)]
pub(crate) enum ScanState {
    Idle,
    Running,
    Completed,
    Warning,
    Failed,
}

#[derive(Clone, Copy)]
pub(crate) enum OperationState {
    Idle,
    Running,
    Completed,
    Warning,
    Failed,
}
