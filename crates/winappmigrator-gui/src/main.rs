// Always a windowed GUI app — never allocate a console window, in any build.
#![windows_subsystem = "windows"]

mod views;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui::{self, Align, Layout, Margin, RichText, Stroke};

use winappmigrator_core::{
    AppSelection, InstalledApp, PackageOptions, PackageResult, ScanSource, SystemContext,
    VerificationOptions, VerificationReport, create_package, list_installed_apps_from,
    verify_package,
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
    /// Where apps are read from: the live OS, or a connected offline drive.
    scan_source: ScanSource,
    /// A problem selecting the offline drive (not a Windows volume, etc.).
    source_error: Option<String>,
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
            scan_source: ScanSource::Live,
            source_error: None,
            migrate_state: OperationState::Idle,
            migrate_receiver: None,
            migrate_detail: String::new(),
            last_outcome: None,
            last_error: None,
        };
        // Remove any stub temp directories left by previous or crashed runs.
        sweep_stale_stub_temp_dirs();
        app.start_inventory_load();
        app
    }

    fn start_inventory_load(&mut self) {
        self.inventory_state = InventoryState::Loading;
        self.selected_app_index = None;
        let (sender, receiver) = mpsc::channel();
        self.inventory_receiver = Some(receiver);
        // Build the source context (mounting offline hives if needed) on the
        // worker thread so mounts are short-lived and never block the UI.
        let source = self.scan_source.clone();
        thread::spawn(move || {
            let message = match SystemContext::open(&source)
                .and_then(|system| list_installed_apps_from(&system))
            {
                Ok(apps) => InventoryMessage::Loaded(apps),
                Err(error) => InventoryMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
        });
    }

    /// Scan the live machine.
    fn select_live_source(&mut self) {
        self.source_error = None;
        if self.scan_source.is_offline() {
            self.scan_source = ScanSource::Live;
            self.start_inventory_load();
        }
    }

    /// Pick a connected drive (a disk from another PC) as the scan source.
    fn pick_offline_drive(&mut self) {
        let Some(root) = rfd::FileDialog::new()
            .set_title("Select the connected drive's root (e.g. E:\\)")
            .pick_folder()
        else {
            return;
        };
        // A Windows system volume has \Windows and \Users. Check those (which are
        // listable) rather than the registry hive file, whose ACL denies even an
        // existence check until we are elevated and reading it via backup mode.
        if !(root.join("Windows").is_dir() && root.join("Users").is_dir()) {
            self.source_error = Some(format!(
                "{} does not look like a Windows system drive (no \\Windows and \\Users). \
                 Pick the root of the drive from the other PC.",
                root.display()
            ));
            return;
        }
        self.source_error = None;
        self.scan_source = ScanSource::offline(root);
        self.start_inventory_load();
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
        // Name the installer after the application, not the chosen folder, so the
        // deliverable is self-describing (e.g. `YAMAHA DIAGNOSTIC SYSTEM2_installer.exe`).
        let base_name = safe_file_stem(&app.display_name);
        let app_name = app.display_name.clone();
        let include_user_data = self.backup_user_data;
        let include_license_data = self.backup_license;
        let source = self.scan_source.clone();

        let (sender, receiver) = mpsc::channel();
        self.migrate_receiver = Some(receiver);
        self.migrate_state = OperationState::Running;
        self.migrate_detail =
            "Reverse-engineering the app, copying its files, and building the installer…"
                .to_owned();
        self.last_error = None;
        self.last_outcome = None;

        thread::spawn(move || {
            // Stage the (potentially large) package on the SAME drive as the
            // chosen output folder, not the system temp drive, so a full-footprint
            // build never fails for lack of space on C:. The chosen folder still
            // ends up holding only the single self-extracting installer.
            let staging_parent = build_staging_parent(&installer_dir);
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
                // Read the app from the live OS or the connected offline drive.
                source,
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

/// Sanitize an application's display name into a safe, human-readable label:
/// keep letters, digits, spaces, hyphens, and underscores; replace anything else
/// with `_`; trim surrounding whitespace. Unlike the core's filesystem sanitizer,
/// spaces are preserved so installer names read naturally (e.g.
/// `YAMAHA DIAGNOSTIC SYSTEM2`).
fn sanitized_app_label(app_name: &str) -> String {
    app_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, ' ' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_owned()
}

/// A sensible default installer output directory for an application, so the
/// common path does not require the user to choose a folder manually.
pub(crate) fn default_output_for(app_name: &str) -> String {
    let safe = sanitized_app_label(app_name);
    let base = std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("WinAppMigrator")
        .join(if safe.is_empty() {
            "Package"
        } else {
            safe.as_str()
        })
        .display()
        .to_string()
}

/// Sanitize an application's display name into a safe file stem, used to name the
/// produced installer after the app (e.g. `YAMAHA DIAGNOSTIC SYSTEM2`).
pub(crate) fn safe_file_stem(app_name: &str) -> String {
    let safe = sanitized_app_label(app_name);
    if safe.is_empty() {
        "App".to_owned()
    } else {
        safe
    }
}

/// A staging directory on the SAME drive as the chosen output folder, so the
/// temporary full-footprint package never has to fit on the system temp drive.
pub(crate) fn build_staging_parent(installer_dir: &Path) -> PathBuf {
    let base = installer_dir
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(installer_dir);
    base.join(format!(".winappmigrator-build-{}", std::process::id()))
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

/// Prefix for this app's per-process temp directory (holds the extracted stub).
const STUB_TEMP_PREFIX: &str = "winappmigrator-stub-";

/// This process's temp directory for the extracted installer stub. Removed on
/// exit by `Drop for WinAppMigratorApp`.
fn stub_temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!("{STUB_TEMP_PREFIX}{}", std::process::id()))
}

/// Write the embedded installer stub to a per-process temp file (once) and
/// return its path, so it can be used as the self-extracting archive stub.
fn write_embedded_stub() -> std::io::Result<PathBuf> {
    let dir = stub_temp_dir();
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

/// Remove leftover stub temp directories from previous (or crashed) runs, so a
/// crash never leaves the ~18 MB stub behind. The current process's own
/// directory is kept and cleaned on exit (`Drop`).
fn sweep_stale_stub_temp_dirs() {
    let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else {
        return;
    };
    let own = stub_temp_dir();
    for entry in entries.flatten() {
        let path = entry.path();
        let is_ours = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(STUB_TEMP_PREFIX));
        if is_ours && path != own && path.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        }
    }
}

impl Drop for WinAppMigratorApp {
    fn drop(&mut self) {
        // eframe drops the app when the window closes; remove this process's
        // extracted installer stub so nothing is left in %TEMP%.
        let _ = std::fs::remove_dir_all(stub_temp_dir());
    }
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
