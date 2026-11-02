//! The WinAppMigrator app. It lives in a library so its tests can run without the
//! elevation the `winappmigrator` executable's manifest demands.

mod views;

#[cfg(test)]
mod ui_tests;

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui;

use winappmigrator_core::logging;
use winappmigrator_core::path_utils::{signed_in_user, system_root};
use winappmigrator_core::privilege::ProtectedFolder;
use winappmigrator_core::{
    AppSelection, BuildMonitor, BuildStep, InstalledApp, InstallerStub, PackageOptions,
    PackageResult, ProfileChoice, ProfilePicker, ProfileSurvey, RoomCheck, ScanSource,
    SystemContext, VerificationReport, WinAppMigratorError, create_package_monitored,
    hidden_command, list_installed_apps_from, looks_like_a_windows_drive, panic_text, survey_of,
    survey_profiles, sweep_leaked_hive_mounts, verify_installer,
};
use winappmigrator_ui::theme::{self, Theme, ThemeMode};
use winappmigrator_ui::widgets;

const DEFAULT_WINDOW_WIDTH: f32 = 1240.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 780.0;
const MIN_WINDOW_WIDTH: f32 = 1000.0;
const MIN_WINDOW_HEIGHT: f32 = 640.0;
const WINDOW_TITLE: &str = "WinAppMigrator";
const STAGING_PREFIX: &str = ".winappmigrator-build-";
const SETTINGS_DIR: &str = "WinAppMigrator";
const THEME_FILE: &str = "theme.txt";
/// How often to redraw while a worker runs, enough for smooth animation without
/// spending a core on repainting during a long build.
const BUSY_REPAINT_INTERVAL: Duration = Duration::from_millis(50);

pub fn run() -> eframe::Result {
    logging::install("app");
    // Setting up the folder runs other programs, and nothing should wait for that.
    thread::spawn(logging::open_machine_log);
    // A run that died can have left another PC's registry loaded. And who is signed in takes a
    // moment to find out, and the app is about to want to know.
    thread::spawn(|| {
        sweep_leaked_hive_mounts();
        signed_in_user();
    });
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
        .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
        .with_app_id("winappmigrator");
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
        Box::new(|cc| {
            theme::install_fonts(&cc.egui_ctx, &fonts_dir());
            let mode = saved_theme_mode()
                .or_else(|| system_theme_mode(&cc.egui_ctx))
                .unwrap_or(ThemeMode::Dark);
            let mut app = WinAppMigratorApp::new(cc.egui_ctx.clone(), mode);
            app.theme.apply(&cc.egui_ctx);
            // Remove a stub temp directory left behind by an older version.
            sweep_legacy_stub_temp_dirs();
            app.start_inventory_load();
            Ok(Box::new(app))
        }),
    )
}

fn fonts_dir() -> PathBuf {
    system_root().join("Fonts")
}

fn system_theme_mode(ctx: &egui::Context) -> Option<ThemeMode> {
    ctx.system_theme().map(|system| match system {
        egui::Theme::Dark => ThemeMode::Dark,
        egui::Theme::Light => ThemeMode::Light,
    })
}

fn theme_file() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from)?;
    Some(base.join(SETTINGS_DIR).join(THEME_FILE))
}

fn saved_theme_mode() -> Option<ThemeMode> {
    let tag = std::fs::read_to_string(theme_file()?).ok()?;
    Some(ThemeMode::from_tag(tag.trim()))
}

/// Remember the theme the user picked. Losing this is harmless, so a failure to
/// write it is not worth interrupting them for.
fn save_theme_mode(mode: ThemeMode) {
    let Some(path) = theme_file() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, mode.as_tag());
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScopeFilter {
    All,
    PerUser,
    PerMachine,
}

/// Which column the application table is sorted by.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SortColumn {
    Name,
    Publisher,
    Version,
    Scope,
    Architecture,
}

/// Where a running build has got to.
#[derive(Clone, PartialEq, Eq)]
pub(crate) enum BuildProgress {
    Build(BuildStep),
    Verifying,
}

/// The application is a single screen: pick an installed app, choose whether to
/// also back up its license and user data, then build a self-extracting
/// installer that reinstalls it on another device and hash-verifies the result.
pub struct WinAppMigratorApp {
    ctx: egui::Context,
    theme: Theme,
    inventory_state: InventoryState,
    inventory_receiver: Option<Receiver<InventoryMessage>>,
    apps: Vec<InstalledApp>,
    selected_app_index: Option<usize>,
    /// A row to bring into view after the keyboard moved the selection.
    scroll_to_row: Option<usize>,
    app_search: String,
    app_scope_filter: ScopeFilter,
    sort_column: SortColumn,
    sort_ascending: bool,
    output_dir: String,
    backup_user_data: bool,
    backup_license: bool,
    /// Where apps are read from: the live OS, or a connected offline drive.
    scan_source: ScanSource,
    /// A problem selecting the offline drive (not a Windows volume, etc.).
    source_error: Option<String>,
    /// What was found about the accounts on that source, when its apps were listed.
    profile_survey: Option<Arc<ProfileSurvey>>,
    /// Whose data can go in an installer of the selected app, once that has been looked at.
    profile_picker: Option<ProfilePicker>,
    picker_receiver: Option<Receiver<ProfilePicker>>,
    /// Whose data is to go in it, by account.
    chosen_profile: Option<String>,
    /// Whether an installer stub exists to build from, checked once at startup.
    stub_available: bool,
    migrate_state: OperationState,
    migrate_receiver: Option<Receiver<MigrateMessage>>,
    monitor: Option<BuildMonitor>,
    build_progress: Option<BuildProgress>,
    /// How much room this build needs and where, once its scan has found out.
    build_room: Option<RoomCheck>,
    build_started: Option<Instant>,
    last_outcome: Option<MigrationOutcome>,
    last_error: Option<String>,
    /// Where this run is logging, to say when a build fails.
    log_file: Option<PathBuf>,
    scroll_details_to_top: bool,
}

impl WinAppMigratorApp {
    fn new(ctx: egui::Context, mode: ThemeMode) -> Self {
        Self {
            ctx,
            theme: Theme::new(mode),
            inventory_state: InventoryState::Loading,
            inventory_receiver: None,
            apps: Vec::new(),
            selected_app_index: None,
            scroll_to_row: None,
            app_search: String::new(),
            app_scope_filter: ScopeFilter::All,
            sort_column: SortColumn::Name,
            sort_ascending: true,
            output_dir: String::new(),
            backup_user_data: false,
            backup_license: false,
            scan_source: ScanSource::Live,
            source_error: None,
            profile_survey: None,
            profile_picker: None,
            picker_receiver: None,
            chosen_profile: None,
            stub_available: installer_stub().is_some(),
            migrate_state: OperationState::Idle,
            migrate_receiver: None,
            monitor: None,
            build_progress: None,
            build_room: None,
            build_started: None,
            last_outcome: None,
            last_error: None,
            log_file: None,
            scroll_details_to_top: false,
        }
    }

    pub(crate) fn is_building(&self) -> bool {
        matches!(self.migrate_state, OperationState::Running)
    }

    fn start_inventory_load(&mut self) {
        self.inventory_state = InventoryState::Loading;
        self.selected_app_index = None;
        self.profile_survey = None;
        self.profile_picker = None;
        self.picker_receiver = None;
        self.chosen_profile = None;
        let (sender, receiver) = mpsc::channel();
        self.inventory_receiver = Some(receiver);
        // Build the source context (mounting offline hives if needed) on the
        // worker thread so mounts are short-lived and never block the UI.
        let source = self.scan_source.clone();
        let ctx = self.ctx.clone();
        thread::spawn(move || {
            let listed = SystemContext::open(&source).and_then(|system| {
                let apps = list_installed_apps_from(&system)?;
                // A drive's hives are loaded already, and are not to be read twice. Not being able
                // to read this says nothing about the apps: each just goes by whoever it would be
                // by default, and the build says what it could not tell.
                let survey = if source.is_offline() {
                    Some(survey_of(&system))
                } else {
                    survey_profiles(&source).ok()
                };
                Ok((apps, survey))
            });
            let message = match listed {
                Ok((apps, survey)) => InventoryMessage::Loaded(apps, survey.map(Arc::new)),
                Err(error) => InventoryMessage::Failed(error.to_string()),
            };
            let _send_result = sender.send(message);
            ctx.request_repaint();
        });
    }

    /// Find out whose data can go in an installer of this app. It looks at folders, which on a
    /// failing drive can take a while, so a window does not wait for that.
    fn start_profile_lookup(&mut self, app: InstalledApp) {
        self.profile_picker = None;
        self.chosen_profile = None;
        let Some(survey) = self.profile_survey.clone() else {
            self.picker_receiver = None;
            return;
        };
        let (sender, receiver) = mpsc::channel();
        self.picker_receiver = Some(receiver);
        let ctx = self.ctx.clone();
        thread::spawn(move || {
            let _send_result = sender.send(survey.picker_for(&app));
            ctx.request_repaint();
        });
    }

    /// Whose data a build is to include. Until that has been looked at, or if it could not be, a
    /// build goes by the default, which says what it could not tell.
    fn profile_choice(&self) -> ProfileChoice {
        match (self.profile_picker.as_ref(), self.chosen_profile.as_ref()) {
            (Some(_), Some(id)) => ProfileChoice::Profile(id.clone()),
            (Some(_), None) => ProfileChoice::Nobody,
            (None, _) => ProfileChoice::Default,
        }
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
        let picked = rfd::FileDialog::new()
            .set_title("Select the connected drive's root (e.g. E:\\)")
            .pick_folder();
        // The dialog leaves this process in whatever folder was chosen, and a scan searches that.
        let _ = std::env::set_current_dir(system_root());
        let Some(root) = picked else {
            return;
        };
        // A Windows system volume has \Windows and \Users. Check those (which are
        // listable) rather than the registry hive file, whose ACL denies even an
        // existence check until we are elevated and reading it via backup mode.
        if !looks_like_a_windows_drive(&root) {
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
        if self.is_building() {
            return;
        }
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
        let Some(stub) = installer_stub() else {
            self.migrate_state = OperationState::Failed;
            self.last_error = Some(
                "Installer stub (winappmigrator-installer.exe) was not found beside this app, so a \
                 self-extracting installer cannot be built."
                    .to_owned(),
            );
            return;
        };

        let (sender, receiver) = mpsc::channel();
        let progress_sender = sender.clone();
        let progress_ctx = self.ctx.clone();
        let monitor = BuildMonitor::new(move |step| {
            let _send_result =
                progress_sender.send(MigrateMessage::Progress(BuildProgress::Build(step)));
            progress_ctx.request_repaint();
        });

        let job = BuildJob {
            // Name the installer after the application, not the chosen folder, so
            // the deliverable is self-describing (e.g.
            // `YAMAHA_DIAGNOSTIC_SYSTEM2_installer.exe`).
            base_name: safe_file_stem(&app.display_name),
            app_name: app.display_name.clone(),
            app,
            installer_dir: PathBuf::from(self.output_dir.trim()),
            stub,
            include_user_data: self.backup_user_data,
            include_license_data: self.backup_license,
            source: self.scan_source.clone(),
            user_profile: self.profile_choice(),
            // Record the active theme so the produced installer presents to match.
            ui_theme: self.theme.mode.as_tag().to_owned(),
        };

        self.migrate_receiver = Some(receiver);
        self.migrate_state = OperationState::Running;
        self.monitor = Some(monitor.clone());
        self.build_progress = Some(BuildProgress::Build(BuildStep::Scanning));
        self.build_room = None;
        self.build_started = Some(Instant::now());
        self.last_error = None;
        self.last_outcome = None;

        let ctx = self.ctx.clone();
        thread::spawn(move || {
            // What goes wrong in a way nothing was ready for still has to end in something a person
            // can read, or this window would say it was building forever.
            let message = catch_unwind(AssertUnwindSafe(|| run_build(job, &monitor, &sender)))
                .unwrap_or_else(|panic| {
                    let reason = panic_text(panic.as_ref());
                    tracing::error!(%reason, "a build stopped unexpectedly");
                    MigrateMessage::Failed(format!(
                        "The build stopped unexpectedly: {reason}. If it left an installer in the \
                         output folder, do not use it."
                    ))
                });
            let _send_result = sender.send(message);
            ctx.request_repaint();
        });
    }

    fn cancel_migration(&self) {
        if let Some(monitor) = self.monitor.as_ref() {
            monitor.cancel();
        }
    }

    pub(crate) fn is_cancelling(&self) -> bool {
        self.monitor
            .as_ref()
            .is_some_and(BuildMonitor::is_cancelled)
    }

    fn poll_workers(&mut self) {
        if let Some(receiver) = self.picker_receiver.take() {
            match receiver.try_recv() {
                Ok(picker) => {
                    self.chosen_profile = picker.selected.clone();
                    self.profile_picker = Some(picker);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.picker_receiver = Some(receiver);
                    self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                }
                Err(mpsc::TryRecvError::Disconnected) => {}
            }
        }

        if let Some(receiver) = self.inventory_receiver.take() {
            match receiver.try_recv() {
                Ok(InventoryMessage::Loaded(apps, survey)) => {
                    theme::add_fallback_fonts(
                        &self.ctx,
                        &fonts_dir(),
                        apps.iter().flat_map(|app| {
                            std::iter::once(app.display_name.as_str())
                                .chain(app.publisher.as_deref())
                        }),
                    );
                    self.apps = apps;
                    self.profile_survey = survey;
                    self.inventory_state = InventoryState::Loaded;
                }
                Ok(InventoryMessage::Failed(error)) => {
                    self.inventory_state = InventoryState::Failed(error);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.inventory_receiver = Some(receiver);
                    self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.inventory_state =
                        InventoryState::Failed("Inventory worker disconnected.".to_owned());
                }
            }
        }

        if let Some(receiver) = self.migrate_receiver.take() {
            loop {
                match receiver.try_recv() {
                    Ok(MigrateMessage::Progress(progress)) => {
                        if let BuildProgress::Build(BuildStep::RoomChecked(room)) = &progress {
                            self.build_room = Some(room.clone());
                        }
                        self.build_progress = Some(progress);
                    }
                    Ok(MigrateMessage::Completed(outcome)) => {
                        self.migrate_state = OperationState::Completed;
                        self.last_outcome = Some(*outcome);
                        self.finish_build();
                        return;
                    }
                    Ok(MigrateMessage::Failed(error)) => {
                        self.migrate_state = OperationState::Failed;
                        self.last_error = Some(error);
                        self.finish_build();
                        return;
                    }
                    Ok(MigrateMessage::Cancelled) => {
                        self.migrate_state = OperationState::Cancelled;
                        self.finish_build();
                        return;
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        self.migrate_receiver = Some(receiver);
                        self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                        return;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.migrate_state = OperationState::Failed;
                        self.last_error = Some("Migration worker disconnected.".to_owned());
                        self.finish_build();
                        return;
                    }
                }
            }
        }
    }

    fn finish_build(&mut self) {
        self.log_file = logging::current_log_file();
        self.monitor = None;
        self.build_progress = None;
        self.build_started = None;
        self.scroll_details_to_top = true;
    }

    fn toggle_theme(&mut self) {
        let mode = self.theme.mode.toggled();
        self.theme = Theme::new(mode);
        self.theme.apply(&self.ctx);
        save_theme_mode(mode);
    }
}

impl eframe::App for WinAppMigratorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.theme.bg.to_normalized_gamma_f32()
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_workers();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

/// Everything a build needs, moved onto its worker thread.
struct BuildJob {
    app: InstalledApp,
    app_name: String,
    base_name: String,
    installer_dir: PathBuf,
    stub: InstallerStub,
    include_user_data: bool,
    include_license_data: bool,
    source: ScanSource,
    user_profile: ProfileChoice,
    ui_theme: String,
}

fn run_build(
    job: BuildJob,
    monitor: &BuildMonitor,
    progress: &Sender<MigrateMessage>,
) -> MigrateMessage {
    tracing::info!(app = %job.app_name, folder = %job.installer_dir.display(), "building an installer");
    if let Err(error) = std::fs::create_dir_all(&job.installer_dir) {
        return MigrateMessage::Failed(format!(
            "Could not create the output folder {}: {error}",
            job.installer_dir.display()
        ));
    }
    // Stage the (potentially large) package on the SAME drive as the chosen output
    // folder, not the system temp drive, so a full-footprint build never fails for
    // lack of space on C:. The chosen folder still ends up holding only the single
    // self-extracting installer. The name is random and removal is automatic, even
    // if the build fails or is cancelled.
    // Elevated code reads from here to build the installer, so other processes must not
    // be able to change it in the meantime. A drive that keeps no permissions, such as a
    // FAT32 or exFAT USB stick, cannot arrange that, so the system's temp folder is used
    // instead. The build stops if neither can.
    let staging_parent =
        match ProtectedFolder::new_in(staging_base(&job.installer_dir), STAGING_PREFIX)
            .or_else(|_| ProtectedFolder::new_in_temp(STAGING_PREFIX))
        {
            Ok(directory) => directory,
            Err(error) => {
                return MigrateMessage::Failed(format!(
                    "Could not create a protected staging folder, so nothing was built: {error}",
                ));
            }
        };
    let staging = staging_parent.path().join(&job.base_name);

    let options = PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(job.app)),
        output_dir: staging.clone(),
        current_directory: std::env::current_dir().ok(),
        create_self_extracting_archive: true,
        self_extracting_stub: Some(job.stub),
        installer_dir: Some(job.installer_dir),
        include_user_data: job.include_user_data,
        // ProgramData is machine-wide application data; back it up together with
        // per-user data under the one "user data" choice.
        include_program_data: job.include_user_data,
        include_license_data: job.include_license_data,
        // Download and embed runtime prerequisites so the one installer installs
        // everything on the target, even offline.
        bundle_prerequisites: true,
        // Read the app from the live OS or the connected offline drive.
        source: job.source,
        user_profile: job.user_profile,
        ui_theme: Some(job.ui_theme),
    };

    let package = match create_package_monitored(&options, monitor) {
        Ok(package) => package,
        Err(WinAppMigratorError::Cancelled) => return MigrateMessage::Cancelled,
        Err(error) => {
            tracing::error!(%error, "the installer could not be built");
            return MigrateMessage::Failed(format!("The installer could not be built. {error}"));
        }
    };

    tracing::info!(files = package.files_copied, "the package was built");
    let _send_result = progress.send(MigrateMessage::Progress(BuildProgress::Verifying));
    let Some(installer) = package.self_extracting_archive_path.clone() else {
        return MigrateMessage::Failed("The build finished with no installer.".to_owned());
    };
    // Read what was written, as whoever runs it will find it.
    match verify_installer(&installer, monitor) {
        Ok(verification) => {
            let installer_bytes = [Some(installer.as_path()), package.data_file_path.as_deref()]
                .into_iter()
                .flatten()
                .filter_map(|path| std::fs::metadata(path).ok())
                .map(|metadata| metadata.len())
                .sum();
            MigrateMessage::Completed(Box::new(MigrationOutcome {
                app_name: job.app_name,
                package,
                verification,
                installer_bytes,
            }))
        }
        // Nothing is left of a build that was cancelled, even one this far along.
        Err(WinAppMigratorError::Cancelled) => {
            let _ = std::fs::remove_file(&installer);
            if let Some(data_file) = package.data_file_path.as_deref() {
                let _ = std::fs::remove_file(data_file);
            }
            MigrateMessage::Cancelled
        }
        Err(error) => MigrateMessage::Failed(format!(
            "The installer was written, but its hash check could not run: {error}. Do not use it."
        )),
    }
}

/// Open a folder in File Explorer.
pub(crate) fn open_folder(path: &Path) -> std::io::Result<()> {
    hidden_command("explorer")
        .arg(path)
        .spawn()
        .map(|_child| ())
}

/// Sanitize an application's display name into a safe, human-readable label:
/// keep letters, digits, spaces, hyphens, and underscores; replace anything else
/// with `_`; trim surrounding whitespace. Spaces are kept so the default output
/// folder reads naturally (e.g. `YAMAHA DIAGNOSTIC SYSTEM2`).
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
    // Whoever is running this. An installer holds what they chose to put in it, which may be
    // someone else's, and an administrator named at the UAC prompt should not find it left in the
    // folders of whoever is signed in, or write through a link they made there.
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

/// Sanitize an application's display name into a safe file stem, which names the
/// produced installer after the app. Packaging then replaces spaces with `_`.
pub(crate) fn safe_file_stem(app_name: &str) -> String {
    let safe = sanitized_app_label(app_name);
    if safe.is_empty() {
        "App".to_owned()
    } else {
        safe
    }
}

/// The folder staging goes in: beside the chosen output folder, on the same drive,
/// so the temporary full-footprint package never has to fit on the system temp
/// drive.
fn staging_base(installer_dir: &Path) -> &Path {
    installer_dir
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(installer_dir)
}

/// The GUI installer stub, embedded into the app at build time (see `build.rs`).
/// Empty only on a first-pass build where the stub was not yet available; the
/// runtime then falls back to a sibling stub file.
const EMBEDDED_INSTALLER_STUB: &[u8] = include_bytes!(env!("WAM_INSTALLER_STUB"));

/// Resolve the installer stub used to build a self-extracting installer.
///
/// WinAppMigrator is one portable executable, so the stub normally lives inside
/// it. When the stub was not embedded (a bootstrap build), fall back to a sibling
/// `winappmigrator-installer.exe`.
fn installer_stub() -> Option<InstallerStub> {
    if !EMBEDDED_INSTALLER_STUB.is_empty() {
        return Some(InstallerStub::Embedded(EMBEDDED_INSTALLER_STUB));
    }
    let current = std::env::current_exe().ok()?;
    let sibling = current.parent()?.join("winappmigrator-installer.exe");
    sibling.is_file().then_some(InstallerStub::File(sibling))
}

/// Older versions extracted the stub to `%TEMP%\winappmigrator-stub-<pid>`. Current
/// versions write it straight to the installer, so anything left there is stale.
fn sweep_legacy_stub_temp_dirs() {
    let Ok(entries) = std::fs::read_dir(std::env::temp_dir()) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let is_legacy = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("winappmigrator-stub-"));
        if is_legacy && path.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        }
    }
}

/// The combined result of a migration build: the package, its self-extracting
/// installer, and the hash verification that confirms package fidelity.
pub(crate) struct MigrationOutcome {
    pub app_name: String,
    pub package: PackageResult,
    pub verification: VerificationReport,
    pub installer_bytes: u64,
}

enum InventoryMessage {
    Loaded(Vec<InstalledApp>, Option<Arc<ProfileSurvey>>),
    Failed(String),
}

enum MigrateMessage {
    Progress(BuildProgress),
    Completed(Box<MigrationOutcome>),
    Failed(String),
    Cancelled,
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
    Failed,
    Cancelled,
}
