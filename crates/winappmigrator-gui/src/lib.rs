//! The WinAppMigrator app. It lives in a library so its tests can run without the
//! elevation the `winappmigrator` executable's manifest demands.

mod footprint_view;
mod views;
mod worker;

#[cfg(test)]
mod ui_tests;
#[cfg(test)]
mod ui_tests_footprint;

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui;

use winappmigrator_core::logging;
use winappmigrator_core::path_utils::{is_on_a_local_disk, signed_in_user, system_root};
use winappmigrator_core::privilege::ProtectedFolder;
use winappmigrator_core::{
    AlsoLeftOut, AppSelection, BuildInputs, BuildMonitor, BuildStep, ChoiceId, Footprint,
    FootprintChoices, InstalledApp, InstallerStub, IntegrationRow, PackageOptions, PackageResult,
    ProfileChoice, ProfilePicker, ProfileSurvey, RoomCheck, ScanFlags, ScanMonitor, ScanOptions,
    ScanReport, ScanSource, ScanStep, SystemContext, VerificationReport, VolumeStats, WhatGoes,
    WinAppMigratorError, create_package_from_scan, hidden_command, list_installed_apps_from,
    looks_like_a_windows_drive, panic_text, room_before_a_build, scan_app, scan_options_for,
    survey_of, survey_profiles, sweep_leaked_hive_mounts, verify_installer, volume_stats_at,
    what_goes_with,
};
use winappmigrator_ui::theme::{self, Theme, ThemeMode};
use winappmigrator_ui::widgets;
use winappmigrator_ui::window::{self, Opening, WindowSizes};

use crate::worker::LatestWorker;

const WINDOW_SIZES: WindowSizes = WindowSizes {
    opens_at: egui::vec2(1240.0, 780.0),
    at_least: egui::vec2(1000.0, 640.0),
};
const WINDOW_TITLE: &str = "WinAppMigrator";
const STAGING_PREFIX: &str = ".winappmigrator-build-";
/// How often to redraw while a worker runs, enough for smooth animation without
/// spending a core on repainting during a long build.
const BUSY_REPAINT_INTERVAL: Duration = Duration::from_millis(50);

pub fn run() -> eframe::Result {
    logging::install("app");
    // Setting up the folder runs other programs, and nothing should wait for that.
    let opening_the_log = thread::spawn(logging::open_machine_log);
    // A run that died can have left another PC's registry loaded. And who is signed in takes a
    // moment to find out, and the app is about to want to know.
    thread::spawn(|| {
        sweep_leaked_hive_mounts();
        signed_in_user();
    });
    let native_options = winappmigrator_ui::window_options(WINDOW_SIZES.viewport("winappmigrator"));

    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(move |cc| {
            let adapter = winappmigrator_ui::graphics_adapter_of(cc);
            // What is logged before there is a file goes nowhere.
            thread::spawn(move || {
                let _ = opening_the_log.join();
                tracing::info!(%adapter, "graphics adapter");
            });
            theme::install_fonts(&cc.egui_ctx, &fonts_dir());
            let (opening, theme) = Opening::on_this_account(WINDOW_SIZES);
            let mut app = WinAppMigratorApp::new(cc.egui_ctx.clone(), theme.mode);
            app.opening = opening;
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

/// Everything a scan's result depends on. A build only goes by a scan with the same as what the
/// window says now, and when anything here changes, it is looked at again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScanKey {
    registry_source_path: String,
    display_name: String,
    owner_profile: Option<String>,
    source: ScanSource,
    include_user_data: bool,
    include_license_data: bool,
    user_profile: ProfileChoice,
    near_matches: Vec<String>,
}

impl ScanKey {
    fn is_same_app(&self, other: &Self) -> bool {
        self.registry_source_path == other.registry_source_path
            && self.display_name == other.display_name
            && self.owner_profile == other.owner_profile
            && self.source == other.source
    }
}

/// A scan that finished, and what it found. Both are kept whole for as long as the app is selected,
/// and a build goes by this rather than looking again.
pub(crate) struct ReadyScan {
    pub key: ScanKey,
    pub report: Arc<ScanReport>,
    pub footprint: Arc<Footprint>,
    pub finished: Instant,
}

/// What is known of the selected app.
pub(crate) enum ScanState {
    /// Nothing is selected, or it is too soon to say what a scan should be of.
    None,
    Running {
        id: u64,
        key: ScanKey,
        monitor: ScanMonitor,
        step: Option<ScanStep>,
        /// What was found of this app last time, to keep on screen while this one runs.
        previous: Option<Arc<Footprint>>,
    },
    Ready(Box<ReadyScan>),
    Failed {
        key: ScanKey,
        message: String,
    },
    /// A person stopped it. It is not started again until they ask.
    Stopped {
        key: ScanKey,
        previous: Option<Arc<Footprint>>,
    },
}

impl ScanState {
    fn key(&self) -> Option<&ScanKey> {
        match self {
            Self::None => None,
            Self::Running { key, .. } | Self::Failed { key, .. } | Self::Stopped { key, .. } => {
                Some(key)
            }
            Self::Ready(ready) => Some(&ready.key),
        }
    }
}

/// How a scan runs: the real one, or a test's.
pub(crate) type ScanRunner = Arc<
    dyn Fn(&ScanOptions, &ScanMonitor) -> winappmigrator_core::Result<ScanReport> + Send + Sync,
>;

/// One scan, for the worker that runs them.
struct ScanJob {
    id: u64,
    options: ScanOptions,
    flags: ScanFlags,
    monitor: ScanMonitor,
    runner: ScanRunner,
}

enum ScanMessage {
    Step(u64, ScanStep),
    Finished(u64, Box<(ScanReport, Footprint)>),
    Failed(u64, String),
    Stopped,
}

/// What went with a person's choices. Going through a package's rules takes a moment on a large app,
/// so it is worked out away from the window.
struct AlsoJob {
    version: u64,
    report: Arc<ScanReport>,
    footprint: Arc<Footprint>,
    choices: FootprintChoices,
}

struct AlsoMessage {
    version: u64,
    outcome: Result<WhatGoes, String>,
}

/// What went with a person's choices, and which choices that is the answer for.
#[derive(Default)]
pub(crate) struct AlsoState {
    pub version: u64,
    pub items: Vec<AlsoLeftOut>,
    /// What ties the app into Windows as that leaves it, once there is anything to say about that.
    pub integrations: Option<Vec<IntegrationRow>>,
    pub error: Option<String>,
}

/// Which part of the screen's list. Each opens and closes by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ListGroup {
    Program,
    Data,
    Services,
    Startup,
    Tasks,
    Shortcuts,
    Integrations,
    Prerequisites,
    NearMatches,
    LeftOut,
}

/// The application is a single screen: pick an installed app, choose whether to
/// also back up its license and user data, then build a self-extracting
/// installer that reinstalls it on another device and hash-verifies the result.
pub struct WinAppMigratorApp {
    ctx: egui::Context,
    theme: Theme,
    /// Fitting the screen, and until a person chooses a theme, being in whichever mode Windows is in.
    opening: Opening,
    /// What a test builds with, where there may be no stub to find yet.
    #[cfg(test)]
    stub_for_tests: Option<InstallerStub>,
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
    /// How big it is, for working out how big an installer can be.
    stub_bytes: u64,
    pub(crate) scan: ScanState,
    next_scan_id: u64,
    scan_runner: ScanRunner,
    scan_worker: LatestWorker<ScanJob>,
    scan_sender: Sender<ScanMessage>,
    scan_messages: Receiver<ScanMessage>,
    /// What a person chose to leave out of what the scan found. It goes by what each thing is
    /// called, so it means the same after another scan.
    pub(crate) choices: FootprintChoices,
    /// How many times that has changed, which says whether [`AlsoState`] is about it.
    pub(crate) choices_version: u64,
    pub(crate) also: AlsoState,
    also_worker: LatestWorker<AlsoJob>,
    also_messages: Receiver<AlsoMessage>,
    /// The near matches a person ticked, with whether each comes with the User data option.
    pub(crate) near_picks: BTreeMap<String, bool>,
    /// How much room the drive of the output folder has, for what is in that field now.
    pub(crate) output_stats: Option<(String, Option<VolumeStats>)>,
    stats_asked_for: Option<String>,
    stats_worker: LatestWorker<String>,
    stats_messages: Receiver<(String, Option<VolumeStats>)>,
    pub(crate) open_groups: BTreeSet<ListGroup>,
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
        let (scan_sender, scan_messages) = mpsc::channel();
        let scan_worker = {
            let (sender, ctx) = (scan_sender.clone(), ctx.clone());
            LatestWorker::new(move |job: ScanJob| {
                run_scan(job, &sender);
                ctx.request_repaint();
            })
        };
        let (also_sender, also_messages) = mpsc::channel();
        let also_worker = {
            let ctx = ctx.clone();
            LatestWorker::new(move |job: AlsoJob| {
                let outcome = catch_unwind(AssertUnwindSafe(|| {
                    what_goes_with(&job.report, &job.footprint, &job.choices)
                        .map_err(|error| error.to_string())
                }))
                .unwrap_or_else(|panic| Err(panic_text(panic.as_ref())));
                let _send_result = also_sender.send(AlsoMessage {
                    version: job.version,
                    outcome,
                });
                ctx.request_repaint();
            })
        };
        let (stats_sender, stats_messages) = mpsc::channel();
        let stats_worker = {
            let ctx = ctx.clone();
            LatestWorker::new(move |folder: String| {
                let stats = stats_of_output_folder(&folder);
                let _send_result = stats_sender.send((folder, stats));
                ctx.request_repaint();
            })
        };
        let stub = installer_stub();
        Self {
            ctx,
            theme: Theme::new(mode),
            opening: Opening::in_a_chosen_mode(WINDOW_SIZES),
            #[cfg(test)]
            stub_for_tests: None,
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
            stub_available: stub.is_some(),
            stub_bytes: stub.as_ref().map_or(0, stub_size),
            scan: ScanState::None,
            next_scan_id: 0,
            scan_runner: Arc::new(scan_app),
            scan_worker,
            scan_sender,
            scan_messages,
            choices: FootprintChoices::default(),
            choices_version: 0,
            also: AlsoState::default(),
            also_worker,
            also_messages,
            near_picks: BTreeMap::new(),
            output_stats: None,
            stats_asked_for: None,
            stats_worker,
            stats_messages,
            open_groups: [ListGroup::Program, ListGroup::NearMatches]
                .into_iter()
                .collect(),
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

    pub(crate) fn selected_app(&self) -> Option<&InstalledApp> {
        self.selected_app_index
            .and_then(|index| self.apps.get(index))
    }

    /// What a scan should be of, going by everything the window says now. Nothing yet if whose data
    /// it is about is still being found out, since that is part of what it reads.
    pub(crate) fn wanted_scan_key(&self) -> Option<ScanKey> {
        if matches!(self.inventory_state, InventoryState::Loading) || self.picker_receiver.is_some()
        {
            return None;
        }
        let app = self.selected_app()?;
        Some(ScanKey {
            registry_source_path: app.registry_source_path.clone(),
            display_name: app.display_name.clone(),
            owner_profile: app.owner_profile.clone(),
            source: self.scan_source.clone(),
            include_user_data: self.backup_user_data,
            include_license_data: self.backup_license,
            user_profile: self.profile_choice(),
            near_matches: self
                .near_picks
                .iter()
                // What is in a data folder comes with the User data option.
                .filter(|(_, follows_user_data)| !**follows_user_data || self.backup_user_data)
                .map(|(id, _)| id.clone())
                .collect(),
        })
    }

    /// Have the scan be of what the window says, which is how a choice that changes what a scan has
    /// to read takes effect, whatever it was. One running for anything else is told to stop, and
    /// is not looked at when it does.
    pub(crate) fn keep_scan_current(&mut self) {
        if self.is_building() {
            return;
        }
        let wanted = self.wanted_scan_key();
        if wanted.as_ref() == self.scan.key() {
            return;
        }
        let previous = match (&self.scan, wanted.as_ref()) {
            (ScanState::Ready(ready), Some(wanted)) if ready.key.is_same_app(wanted) => {
                Some(Arc::clone(&ready.footprint))
            }
            (ScanState::Running { key, previous, .. }, Some(wanted))
            | (ScanState::Stopped { key, previous }, Some(wanted))
                if key.is_same_app(wanted) =>
            {
                previous.clone()
            }
            _ => None,
        };
        let replaced = std::mem::replace(&mut self.scan, ScanState::None);
        if let ScanState::Running { monitor, .. } = &replaced {
            monitor.cancel();
        }
        drop_away_from_the_window(replaced);
        if let Some(key) = wanted {
            self.start_scan(key, previous);
        }
    }

    fn start_scan(&mut self, key: ScanKey, previous: Option<Arc<Footprint>>) {
        let Some(app) = self.selected_app().cloned() else {
            return;
        };
        self.next_scan_id += 1;
        let id = self.next_scan_id;
        let sender = self.scan_sender.clone();
        let ctx = self.ctx.clone();
        let monitor = ScanMonitor::new(move |step| {
            let _send_result = sender.send(ScanMessage::Step(id, step));
            ctx.request_repaint();
        });
        let options = scan_options_of(app, &key);
        self.scan_worker.send(ScanJob {
            id,
            options,
            flags: ScanFlags {
                include_user_data: key.include_user_data,
                include_program_data: key.include_user_data,
                include_license_data: key.include_license_data,
            },
            monitor: monitor.clone(),
            runner: Arc::clone(&self.scan_runner),
        });
        self.scan = ScanState::Running {
            id,
            key,
            monitor,
            step: None,
            previous,
        };
        self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
    }

    /// Stop looking at the app, because a person asked to. Nothing is built until it has been
    /// looked at, so that stays until they ask for it again.
    pub(crate) fn stop_scan(&mut self) {
        if let ScanState::Running {
            key,
            monitor,
            previous,
            ..
        } = &self.scan
        {
            monitor.cancel();
            self.scan = ScanState::Stopped {
                key: key.clone(),
                previous: previous.clone(),
            };
        }
    }

    /// Look at the app again, for what is there by now.
    pub(crate) fn scan_again(&mut self) {
        if self.is_building() {
            return;
        }
        let Some(key) = self.wanted_scan_key() else {
            return;
        };
        let previous = match &self.scan {
            ScanState::Ready(ready) => Some(Arc::clone(&ready.footprint)),
            ScanState::Stopped { previous, .. } => previous.clone(),
            _ => None,
        };
        if let ScanState::Running { monitor, .. } = &self.scan {
            monitor.cancel();
        }
        drop_away_from_the_window(std::mem::replace(&mut self.scan, ScanState::None));
        self.start_scan(key, previous);
    }

    /// The scan a build can go by right now, which is one of exactly what the window says.
    pub(crate) fn ready_scan(&self) -> Option<&ReadyScan> {
        match &self.scan {
            ScanState::Ready(ready) if self.wanted_scan_key().as_ref() == Some(&ready.key) => {
                Some(ready)
            }
            _ => None,
        }
    }

    /// What is on the screen as what was found: what is ready, or while another look is taken, the
    /// last one. Only what is ready can be chosen from.
    pub(crate) fn shown_footprint(&self) -> Option<Arc<Footprint>> {
        match &self.scan {
            ScanState::Ready(ready) => Some(Arc::clone(&ready.footprint)),
            ScanState::Running { previous, .. } | ScanState::Stopped { previous, .. } => {
                previous.clone()
            }
            ScanState::None | ScanState::Failed { .. } => None,
        }
    }

    /// A person ticked or unticked something, which takes effect at once. What goes with it follows
    /// when that has been worked out, and a build waits for that.
    pub(crate) fn set_included(&mut self, id: &ChoiceId, included: bool) {
        self.choices.set_included(id, included);
        self.ask_what_goes_with_the_choices();
    }

    /// Start over, for another app.
    pub(crate) fn forget_choices(&mut self) {
        self.choices = FootprintChoices::default();
        self.near_picks.clear();
        self.ask_what_goes_with_the_choices();
    }

    fn ask_what_goes_with_the_choices(&mut self) {
        self.choices_version += 1;
        if self.choices.is_empty() {
            // Nothing to work out, and nothing to wait for.
            self.also = AlsoState {
                version: self.choices_version,
                ..AlsoState::default()
            };
            return;
        }
        let Some(ready) = self.ready_scan() else {
            return;
        };
        let job = AlsoJob {
            version: self.choices_version,
            report: Arc::clone(&ready.report),
            footprint: Arc::clone(&ready.footprint),
            choices: self.choices.clone(),
        };
        self.also_worker.send(job);
    }

    /// Whether what is on the screen is what a build would do with the choices as they are.
    pub(crate) fn also_is_current(&self) -> bool {
        self.also.version == self.choices_version
    }

    /// What can be known of a build from the scan and the choices, as of now.
    pub(crate) fn build_inputs(&self) -> Option<BuildInputs> {
        let ready = self.ready_scan()?;
        Some(
            ready
                .footprint
                .build_inputs(&self.choices, &self.also.items, self.stub_bytes, true),
        )
    }

    /// How much room a build would need and where, before it starts.
    pub(crate) fn room_estimate(&self) -> Option<RoomCheck> {
        let inputs = self.build_inputs()?;
        let folder = self.output_dir.trim();
        if !self.output_dir_is_valid() {
            return None;
        }
        let stats = self
            .output_stats
            .as_ref()
            .filter(|(asked, _)| asked == folder)
            .and_then(|(_, stats)| *stats);
        Some(room_before_a_build(&inputs, Path::new(folder), stats))
    }

    /// Ask how much room the output folder's drive has, when that is a different question. It can
    /// be a drive that takes a long time to answer, so that is not asked from here.
    fn keep_output_stats_current(&mut self) {
        let folder = self.output_dir.trim();
        if self.stats_asked_for.as_deref() == Some(folder) || !self.output_dir_is_valid() {
            return;
        }
        let folder = folder.to_owned();
        self.stats_asked_for = Some(folder.clone());
        self.stats_worker.send(folder);
    }

    /// Whether everything is in place to build, now.
    pub(crate) fn can_build(&self) -> bool {
        self.stub_available
            && self.output_dir_is_valid()
            && self.ready_scan().is_some()
            && self.also_is_current()
            && self.also.error.is_none()
            && self
                .room_estimate()
                .is_some_and(|room| room.shortfalls().is_empty())
    }

    #[cfg(test)]
    pub(crate) fn set_scan_runner(&mut self, runner: ScanRunner) {
        self.scan_runner = runner;
    }

    #[cfg(test)]
    pub(crate) fn set_stats_asked_for(&mut self, folder: String) {
        self.stats_asked_for = Some(folder);
    }

    #[cfg(test)]
    pub(crate) fn next_scan_id_for_tests(&self) -> u64 {
        self.next_scan_id
    }

    /// What the worker would send of a finished scan, as if from one with this number.
    #[cfg(test)]
    pub(crate) fn deliver_scan_for_tests(&mut self, id: u64) {
        let Some(app) = self.apps.first().cloned() else {
            return;
        };
        let report = crate::ui_tests_footprint::sample_report(&app);
        let footprint = Footprint::default();
        let _send_result = self
            .scan_sender
            .send(ScanMessage::Finished(id, Box::new((report, footprint))));
    }

    #[cfg(test)]
    pub(crate) fn set_chosen_profile_for_tests(&mut self, id: &str) {
        self.profile_picker = Some(ProfilePicker {
            rows: Vec::new(),
            selected: Some(id.to_owned()),
            note: None,
        });
        self.chosen_profile = Some(id.to_owned());
    }

    #[cfg(test)]
    pub(crate) fn set_source_for_tests(&mut self, drive: &Path) {
        self.scan_source = ScanSource::offline(drive);
        self.selected_app_index = None;
    }

    #[cfg(test)]
    pub(crate) fn set_building_for_tests(&mut self) {
        self.migrate_state = OperationState::Running;
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

    /// Build a self-extracting installer from the scan on the screen and what a person chose, then
    /// hash-verify the package. That scan has to be of what the window says now, as a build goes
    /// by it and does not look again. What is read from here on is hashed as it is read, so a file
    /// that has changed since is still right, and one that has gone is said to have.
    pub(crate) fn start_migration(&mut self) {
        if self.is_building() {
            return;
        }
        let Some(app) = self.selected_app().cloned() else {
            self.last_error = Some("Select an application to migrate first.".to_owned());
            return;
        };
        let Some(ready) = self.ready_scan() else {
            self.last_error =
                Some("This app has to be looked at again before it can be built.".to_owned());
            return;
        };
        let report = Arc::clone(&ready.report);
        if !self.also_is_current() {
            return;
        }

        // The product is a self-extracting installer, so the stub that becomes
        // that installer must be present. Without it we cannot build anything.
        #[cfg(test)]
        let stub = self.stub_for_tests.clone().or_else(installer_stub);
        #[cfg(not(test))]
        let stub = installer_stub();
        let Some(stub) = stub else {
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
            report,
            choices: self.choices.clone(),
            installer_dir: PathBuf::from(self.output_dir.trim()),
            stub,
            include_user_data: self.backup_user_data,
            include_license_data: self.backup_license,
            source: self.scan_source.clone(),
            user_profile: self.profile_choice(),
        };

        self.migrate_receiver = Some(receiver);
        self.migrate_state = OperationState::Running;
        self.monitor = Some(monitor.clone());
        self.build_progress = Some(BuildProgress::Build(BuildStep::Preparing));
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

    pub(crate) fn poll_workers(&mut self) {
        self.poll_scan();
        while let Ok(message) = self.also_messages.try_recv() {
            // An answer about what the choices were before is about nothing now.
            if message.version == self.choices_version {
                self.also.version = message.version;
                match message.outcome {
                    Ok(goes) => {
                        self.also.items = goes.also;
                        self.also.integrations = Some(goes.integrations);
                        self.also.error = None;
                    }
                    Err(error) => {
                        self.also.items = Vec::new();
                        self.also.integrations = None;
                        self.also.error = Some(error);
                    }
                }
            }
        }
        while let Ok(answer) = self.stats_messages.try_recv() {
            self.output_stats = Some(answer);
        }
        self.keep_output_stats_current();
        if !self.also_is_current() {
            self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
        }

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

    /// Take in what the scan worker has said. What a stopped or replaced scan says goes by its number,
    /// so one app's scan cannot turn up as another's.
    fn poll_scan(&mut self) {
        while let Ok(message) = self.scan_messages.try_recv() {
            let current = match &self.scan {
                ScanState::Running { id, .. } => Some(*id),
                _ => None,
            };
            match message {
                ScanMessage::Step(id, new_step) if Some(id) == current => {
                    if let ScanState::Running { step, .. } = &mut self.scan {
                        *step = Some(new_step);
                    }
                }
                ScanMessage::Finished(id, found) if Some(id) == current => {
                    let (report, footprint) = *found;
                    self.finish_scan(report, footprint);
                }
                ScanMessage::Failed(id, message) if Some(id) == current => {
                    if let ScanState::Running { key, .. } = &self.scan {
                        self.scan = ScanState::Failed {
                            key: key.clone(),
                            message,
                        };
                    }
                }
                // A scan only stops by itself when it was told to, which has been dealt with.
                ScanMessage::Stopped
                | ScanMessage::Step(..)
                | ScanMessage::Finished(..)
                | ScanMessage::Failed(..) => {}
            }
        }
        if matches!(self.scan, ScanState::Running { .. }) {
            self.ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
        }
    }

    fn finish_scan(&mut self, report: ScanReport, footprint: Footprint) {
        let ScanState::Running { key, .. } = &self.scan else {
            return;
        };
        let mut key = key.clone();
        // A pick only stays a pick if the scan still offers it, and it says when one is not. That
        // is what this scan is of, so it takes no other look to be up to date with that.
        let picked_up = |id: &str| {
            footprint
                .near_matches
                .iter()
                .any(|near| near.picked && near.id == id)
        };
        let not_picked_up: Vec<String> = key
            .near_matches
            .iter()
            .filter(|id| !picked_up(id))
            .cloned()
            .collect();
        for id in &not_picked_up {
            self.near_picks.remove(id);
        }
        key.near_matches.retain(|id| !not_picked_up.contains(id));
        self.scan = ScanState::Ready(Box::new(ReadyScan {
            key,
            report: Arc::new(report),
            footprint: Arc::new(footprint),
            finished: Instant::now(),
        }));
        self.ask_what_goes_with_the_choices();
    }

    fn finish_build(&mut self) {
        self.log_file = logging::current_log_file();
        self.monitor = None;
        self.build_progress = None;
        self.build_started = None;
        self.scroll_details_to_top = true;
    }

    fn toggle_theme(&mut self) {
        self.opening.person_chose_a_mode();
        let mode = self.theme.mode.toggled();
        self.theme = Theme::new(mode);
        self.theme.apply(&self.ctx);
        window::remember_chosen_mode(mode);
    }
}

impl eframe::App for WinAppMigratorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.theme.bg.to_normalized_gamma_f32()
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.opening.settle(ctx, &mut self.theme);
        self.poll_workers();
        self.keep_scan_current();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

/// What looking at an app asks for.
fn scan_options_of(app: InstalledApp, key: &ScanKey) -> ScanOptions {
    let mut options = scan_options_for(&PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app)),
        // A scan writes nothing, so it has nowhere to be told of.
        output_dir: PathBuf::new(),
        current_directory: std::env::current_dir().ok(),
        create_self_extracting_archive: true,
        self_extracting_stub: None,
        installer_dir: None,
        include_user_data: key.include_user_data,
        include_program_data: key.include_user_data,
        include_license_data: key.include_license_data,
        bundle_prerequisites: true,
        source: key.source.clone(),
        user_profile: key.user_profile.clone(),
    });
    options.near_matches.clone_from(&key.near_matches);
    options
}

/// Everything a build needs, moved onto its worker thread.
struct BuildJob {
    app: InstalledApp,
    report: Arc<ScanReport>,
    choices: FootprintChoices,
    app_name: String,
    base_name: String,
    installer_dir: PathBuf,
    stub: InstallerStub,
    include_user_data: bool,
    include_license_data: bool,
    source: ScanSource,
    user_profile: ProfileChoice,
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
    };

    let package = match create_package_from_scan(&options, &job.report, &job.choices, monitor) {
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

/// One scan, from start to something to say. On its own thread, and what goes wrong in a way nothing
/// was ready for still has to end in something a person can read.
fn run_scan(job: ScanJob, sender: &Sender<ScanMessage>) {
    let ScanJob {
        id,
        options,
        flags,
        monitor,
        runner,
    } = job;
    if monitor.is_cancelled() {
        let _send_result = sender.send(ScanMessage::Stopped);
        return;
    }
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        let report = runner(&options, &monitor)?;
        let footprint = Footprint::of(&report, flags)?;
        Ok::<_, WinAppMigratorError>((report, footprint))
    }));
    let message = match outcome {
        Ok(Ok(found)) => ScanMessage::Finished(id, Box::new(found)),
        Ok(Err(WinAppMigratorError::Cancelled)) => ScanMessage::Stopped,
        Ok(Err(error)) => {
            tracing::warn!(%error, "an app could not be looked at");
            ScanMessage::Failed(id, error.to_string())
        }
        Err(panic) => {
            let reason = panic_text(panic.as_ref());
            tracing::error!(%reason, "a scan stopped unexpectedly");
            ScanMessage::Failed(id, format!("It stopped unexpectedly: {reason}."))
        }
    };
    let _send_result = sender.send(message);
}

/// Where a large app's scan goes when it is no longer wanted. Freeing all of what it found takes
/// long enough to be seen, if a window does it.
fn drop_away_from_the_window<T: Send + 'static>(value: T) {
    thread::spawn(move || drop(value));
}

/// What a drive says of itself, if it is one to ask. Asking about a share connects to it, as the
/// administrator running this, for something typed in a field.
fn stats_of_output_folder(folder: &str) -> Option<VolumeStats> {
    let path = Path::new(folder);
    (path.is_absolute() && is_on_a_local_disk(path))
        .then(|| volume_stats_at(path))
        .flatten()
}

fn stub_size(stub: &InstallerStub) -> u64 {
    match stub {
        InstallerStub::Embedded(bytes) => bytes.len() as u64,
        InstallerStub::File(path) => std::fs::metadata(path).map_or(0, |metadata| metadata.len()),
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
