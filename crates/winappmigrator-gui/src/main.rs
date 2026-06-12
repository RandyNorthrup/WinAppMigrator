use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui;
use winappmigrator_core::{
    InstalledApp, PackageArchiveFormat, PackageOptions, PackageResult, RestoreOptions,
    RestoreResult, ScanOptions, ScanReport, create_package, list_installed_apps, restore_package,
    scan_and_write_reports,
};

const DEFAULT_WINDOW_WIDTH: f32 = 980.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 700.0;
const WINDOW_TITLE: &str = "WinAppMigrator";

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|_cc| Ok(Box::new(WinAppMigratorApp::new()))),
    )
}

struct WinAppMigratorApp {
    inventory_state: InventoryState,
    inventory_receiver: Option<Receiver<InventoryMessage>>,
    apps: Vec<InstalledApp>,
    selected_app_index: Option<usize>,
    executable_path: String,
    output_dir: String,
    restore_package_dir: String,
    restore_target_dir: String,
    package_zip: bool,
    package_sfx: bool,
    restore_apply: bool,
    restore_force: bool,
    scan_state: ScanState,
    package_state: OperationState,
    restore_state: OperationState,
    scan_receiver: Option<Receiver<ScanMessage>>,
    package_receiver: Option<Receiver<PackageMessage>>,
    restore_receiver: Option<Receiver<RestoreMessage>>,
    last_report: Option<ScanReport>,
    last_package: Option<PackageResult>,
    last_restore: Option<RestoreResult>,
    last_error: Option<String>,
}

impl WinAppMigratorApp {
    fn new() -> Self {
        let mut app = Self {
            inventory_state: InventoryState::Loading,
            inventory_receiver: None,
            apps: Vec::new(),
            selected_app_index: None,
            executable_path: String::new(),
            output_dir: String::new(),
            restore_package_dir: String::new(),
            restore_target_dir: String::new(),
            package_zip: false,
            package_sfx: false,
            restore_apply: false,
            restore_force: false,
            scan_state: ScanState::Idle,
            package_state: OperationState::Idle,
            restore_state: OperationState::Idle,
            scan_receiver: None,
            package_receiver: None,
            restore_receiver: None,
            last_report: None,
            last_package: None,
            last_restore: None,
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
        self.last_error = None;
        self.last_report = None;

        thread::spawn(move || {
            let options = ScanOptions::for_executable(executable_path, output_dir);
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
        self.last_error = None;
        self.last_package = None;
        let package_zip = self.package_zip;
        let package_sfx = self.package_sfx;
        let sfx_stub = if package_sfx { cli_stub_path() } else { None };

        thread::spawn(move || {
            let mut options = PackageOptions::for_executable(executable_path, output_dir);
            options.current_directory = std::env::current_dir().ok();
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
        self.last_error = None;
        self.last_restore = None;
        let apply = self.restore_apply;
        let force = self.restore_force;

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
                    self.last_report = Some(*report);
                }
                Ok(ScanMessage::Failed(error)) => {
                    self.scan_state = ScanState::Failed;
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
                    self.restore_package_dir = result.package_root.display().to_string();
                    self.last_package = Some(*result);
                }
                Ok(PackageMessage::Failed(error)) => {
                    self.package_state = OperationState::Failed;
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
                    self.last_restore = Some(*result);
                }
                Ok(RestoreMessage::Failed(error)) => {
                    self.restore_state = OperationState::Failed;
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
    }

    fn selected_app_label(&self) -> String {
        self.selected_app_index
            .and_then(|index| self.apps.get(index))
            .map(|app| app.display_name.clone())
            .unwrap_or_else(|| "Select installed app".to_owned())
    }
}

impl eframe::App for WinAppMigratorApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_workers(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(WINDOW_TITLE);
                ui.separator();
                ui.label("Scan, package, restore");
                if ui.button("Reload apps").clicked() {
                    self.start_inventory_load();
                }
            });

            ui.separator();
            ui.columns(2, |columns| {
                self.draw_selection_panel(&mut columns[0]);
                self.draw_results_panel(&mut columns[1]);
            });
        });
    }
}

impl WinAppMigratorApp {
    fn draw_selection_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Selection");
        ui.add_space(8.0);

        match &self.inventory_state {
            InventoryState::Loading => {
                ui.spinner();
                ui.label("Loading installed applications...");
            }
            InventoryState::Loaded => {
                ui.label(format!("Installed applications: {}", self.apps.len()));
                egui::ComboBox::from_label("Installed app")
                    .selected_text(self.selected_app_label())
                    .show_ui(ui, |ui| {
                        for index in 0..self.apps.len() {
                            let app_name = self.apps[index].display_name.clone();
                            let selected = self.selected_app_index == Some(index);
                            if ui.selectable_label(selected, app_name).clicked() {
                                self.selected_app_index = Some(index);
                                if let Some(path) =
                                    self.apps[index].main_executable.as_ref().map(PathBuf::from)
                                {
                                    self.executable_path = path.display().to_string();
                                }
                            }
                        }
                    });
            }
            InventoryState::Failed(error) => {
                ui.colored_label(egui::Color32::RED, format!("Inventory failed: {error}"));
            }
        }

        ui.separator();
        ui.label("Executable path");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.executable_path);
            if ui.button("Browse").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("Executable", &["exe"])
                    .pick_file()
            {
                self.executable_path = path.display().to_string();
            }
        });

        ui.label("Output directory");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.output_dir);
            if ui.button("Browse").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                self.output_dir = path.display().to_string();
            }
        });

        ui.add_space(8.0);
        let scan_enabled = !matches!(self.scan_state, ScanState::Running);
        if ui
            .add_enabled(scan_enabled, egui::Button::new("Run scan"))
            .clicked()
        {
            self.start_scan();
        }
        let package_enabled = !matches!(self.package_state, OperationState::Running);
        ui.checkbox(&mut self.package_zip, "Create zip archive");
        ui.checkbox(&mut self.package_sfx, "Create self-extracting package");
        if ui
            .add_enabled(package_enabled, egui::Button::new("Build package"))
            .clicked()
        {
            self.start_package();
        }
        ui.label("Basic list/scan/package does not require admin rights.");

        ui.separator();
        ui.heading("Restore");
        ui.label("Package root");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.restore_package_dir);
            if ui.button("Folder").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                self.restore_package_dir = path.display().to_string();
            }
            if ui.button("Zip").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("Package zip", &["zip"])
                    .pick_file()
            {
                self.restore_package_dir = path.display().to_string();
            }
        });
        ui.label("Target root");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.restore_target_dir);
            if ui.button("Browse").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                self.restore_target_dir = path.display().to_string();
            }
        });
        ui.checkbox(&mut self.restore_apply, "Apply changes");
        ui.checkbox(&mut self.restore_force, "Overwrite existing files");
        let restore_enabled = !matches!(self.restore_state, OperationState::Running);
        let restore_label = if self.restore_apply {
            "Restore apply"
        } else {
            "Restore dry-run"
        };
        if ui
            .add_enabled(restore_enabled, egui::Button::new(restore_label))
            .clicked()
        {
            self.start_restore();
        }
    }

    fn draw_results_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Results");
        ui.add_space(8.0);

        match self.scan_state {
            ScanState::Idle => {
                ui.label("No scan has run.");
            }
            ScanState::Running => {
                ui.spinner();
                ui.label("Scan running...");
            }
            ScanState::Completed | ScanState::Warning => {
                if let Some(report) = self.last_report.as_ref() {
                    ui.label(format!(
                        "Executable: {}",
                        report.main_executable.path.display()
                    ));
                    ui.label(format!("Architecture: {:?}", report.architecture));
                    ui.label(format!("Confidence score: {}", report.confidence_score));
                    ui.label(format!("Dependencies: {}", report.dependencies.len()));
                    ui.label(format!(
                        "Missing dependencies: {}",
                        report
                            .dependencies
                            .iter()
                            .filter(|entry| entry.missing)
                            .count()
                    ));
                    ui.label(format!(
                        "Runtime prerequisites: {}",
                        report.runtime_prerequisites.len()
                    ));
                    ui.label(format!("Warnings: {}", report.warnings.len()));

                    if let Some(artifacts) = report.generated_artifacts.as_ref() {
                        ui.separator();
                        ui.label(format!(
                            "Report: {}",
                            artifacts.scan_report_markdown.display()
                        ));
                        if ui.button("Open report folder").clicked()
                            && let Some(parent) = artifacts.scan_report_markdown.parent()
                            && let Err(error) = open_folder(parent)
                        {
                            self.last_error = Some(error.to_string());
                        }
                    }

                    if !report.warnings.is_empty() {
                        ui.separator();
                        ui.label("Warnings");
                        egui::ScrollArea::vertical()
                            .max_height(180.0)
                            .show(ui, |ui| {
                                for warning in &report.warnings {
                                    ui.label(format!(
                                        "{:?} {}: {}",
                                        warning.severity, warning.code, warning.message
                                    ));
                                }
                            });
                    }
                }
            }
            ScanState::Failed => {
                ui.colored_label(egui::Color32::RED, "Scan failed.");
            }
        }

        ui.separator();
        ui.heading("Package");
        match self.package_state {
            OperationState::Idle => {
                ui.label("No package has run.");
            }
            OperationState::Running => {
                ui.spinner();
                ui.label("Package running...");
            }
            OperationState::Completed | OperationState::Warning => {
                if let Some(package) = self.last_package.as_ref() {
                    ui.label(format!("Package root: {}", package.package_root.display()));
                    ui.label(format!("Files copied: {}", package.files_copied));
                    if let Some(path) = package.archive_path.as_ref() {
                        ui.label(format!("Archive: {}", path.display()));
                    }
                    if let Some(path) = package.self_extracting_archive_path.as_ref() {
                        ui.label(format!("Self-extracting: {}", path.display()));
                    }
                    ui.label(format!("Warnings: {}", package.warnings.len()));
                }
            }
            OperationState::Failed => {
                ui.colored_label(egui::Color32::RED, "Package failed.");
            }
        }

        ui.separator();
        ui.heading("Restore");
        match self.restore_state {
            OperationState::Idle => {
                ui.label("No restore has run.");
            }
            OperationState::Running => {
                ui.spinner();
                ui.label("Restore running...");
            }
            OperationState::Completed | OperationState::Warning => {
                if let Some(restore) = self.last_restore.as_ref() {
                    ui.label(format!("Applied: {}", restore.applied));
                    ui.label(format!("Changes logged: {}", restore.changes_logged.len()));
                    ui.label(format!("Warnings: {}", restore.warnings.len()));
                    ui.label(format!("Errors: {}", restore.errors.len()));
                }
            }
            OperationState::Failed => {
                ui.colored_label(egui::Color32::RED, "Restore failed.");
            }
        }

        if let Some(error) = self.last_error.as_ref() {
            ui.separator();
            ui.colored_label(egui::Color32::RED, error);
        }
    }
}

fn open_folder(path: &Path) -> std::io::Result<()> {
    Command::new("explorer").arg(path).spawn().map(|_child| ())
}

fn cli_stub_path() -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    let sibling = current.parent()?.join("winappmigrator.exe");
    sibling.is_file().then_some(sibling)
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

enum InventoryState {
    Loading,
    Loaded,
    Failed(String),
}

enum ScanState {
    Idle,
    Running,
    Completed,
    Warning,
    Failed,
}

enum OperationState {
    Idle,
    Running,
    Completed,
    Warning,
    Failed,
}
