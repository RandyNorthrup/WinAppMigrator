//! The screen as it shows what goes in an installer, from a fake scan and the real view code. The
//! review behind what it says is the real one, so its words are the installer's.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

use winappmigrator_core::{
    AppArchitecture, CREDENTIALS_LEFT_OUT_TEXT, ChoiceId, ExecutableMetadata, FileClassification,
    FileEntry, FirewallRuleEntry, Footprint, FootprintKind, FootprintLocation, InstallScope,
    InstalledApp, LeftOutItem, LicenseArtifact, LicenseArtifactKind, LicenseFramework,
    MatchConfidence, NearMatch, NearMatchKind, RegistryEntry, RegistryHive, RegistryValueKind,
    RuntimePrerequisite, RuntimePrerequisiteKind, SCAN_SCHEMA_VERSION, SETTINGS_LEFT_OUT_TEXT,
    ScanFlags, ScanMonitor, ScanOptions, ScanReport, ScheduledTaskEntry, ServiceEntry,
    ShortcutEntry, SignatureStatus, SystemChangeKind, TransferableHint, VolumeStats,
    WinAppMigratorError,
};

use crate::theme::ThemeMode;
use crate::ui_tests::{harness_sized, harness_with, preview, select_first};
use crate::{ReadyScan, ScanRunner, ScanState, WinAppMigratorApp};

const MIB: u64 = 1024 * 1024;
const SCROLL_ANIMATION_FRAMES: usize = 60;
const GIB: u64 = 1024 * MIB;
const INSTALL: &str = r"C:\Program Files\Fabrikam Data Sync";
const TASK_XML: &str = r#"<?xml version="1.0"?>
<Task xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <Principals><Principal id="Author"><UserId>S-1-5-18</UserId></Principal></Principals>
  <Actions Context="Author"><Exec><Command>COMMAND</Command></Exec></Actions>
</Task>"#;

fn files_in(
    folder: &str,
    count: usize,
    each: u64,
    classification: FileClassification,
) -> Vec<FileEntry> {
    (0..count)
        .map(|index| {
            file(
                &format!(r"{folder}\file{index}.dat"),
                each,
                classification.clone(),
            )
        })
        .collect()
}

fn file(path: &str, size: u64, classification: FileClassification) -> FileEntry {
    let original = PathBuf::from(path);
    let relative = original.strip_prefix(INSTALL).map_or_else(
        |_| PathBuf::from(r"files\_external").join(path.replace(':', "_")),
        |relative| PathBuf::from("files").join(relative),
    );
    FileEntry {
        path: original.clone(),
        original_path: original,
        package_relative_path: Some(relative),
        size_bytes: Some(size),
        hash_sha256: None,
        classification,
        should_copy: true,
        reason: String::new(),
        user_folder: None,
    }
}

fn value(hive: RegistryHive, key: &str, name: Option<&str>, data: &str) -> RegistryEntry {
    RegistryEntry {
        hive,
        key_path: key.to_owned(),
        value_name: name.map(ToOwned::to_owned),
        value_data: Some(data.to_owned()),
        value_kind: Some(RegistryValueKind::String),
        value_bytes_hex: None,
        confidence: MatchConfidence::High,
        requires_admin: true,
    }
}

fn service(name: &str, display: &str, relative: &str) -> ServiceEntry {
    ServiceEntry {
        service_name: name.to_owned(),
        display_name: Some(display.to_owned()),
        binary_path: PathBuf::from(INSTALL).join(relative),
        arguments: None,
        parameters: Vec::new(),
        start_type: Some("automatic".to_owned()),
        account_name: Some("LocalSystem".to_owned()),
        dependencies: Vec::new(),
        description: None,
        requires_admin: true,
        restore_command_suggestion: String::new(),
    }
}

fn task(name: &str, relative: &str) -> ScheduledTaskEntry {
    let command = PathBuf::from(INSTALL).join(relative);
    ScheduledTaskEntry {
        task_name: name.to_owned(),
        task_path: r"\Fabrikam\".to_owned(),
        action_command: command.clone(),
        arguments: None,
        working_directory: None,
        trigger_summary: None,
        run_level: None,
        account: None,
        requires_admin: true,
        restore_instructions: String::new(),
        definition_xml: Some(TASK_XML.replace("COMMAND", &command.display().to_string())),
    }
}

fn near_match(name: &str, why: &str, follows_user_data: bool, kind: NearMatchKind) -> NearMatch {
    NearMatch {
        id: format!("folder:{}", name.to_ascii_lowercase()),
        kind,
        name: name.to_owned(),
        path: Some(PathBuf::from(r"C:\Program Files").join(name)),
        why: why.to_owned(),
        follows_user_data,
        file_count: 12,
        total_bytes: 8 * MIB,
        measured_all: true,
        picked: false,
    }
}

/// What a scan of the app the tests build might find, with nothing of any real PC in it.
pub(crate) fn sample_report(app: &InstalledApp) -> ScanReport {
    let main = PathBuf::from(INSTALL).join(r"bin\fabrikam.exe");
    let mut files = vec![
        file(
            &main.display().to_string(),
            40 * MIB,
            FileClassification::AppOwned,
        ),
        file(
            &format!(r"{INSTALL}\bin\fabsync.exe"),
            20 * MIB,
            FileClassification::AppOwned,
        ),
        file(
            &format!(r"{INSTALL}\bin\tray.exe"),
            MIB,
            FileClassification::AppOwned,
        ),
        file(
            &format!(r"{INSTALL}\plugins\host.exe"),
            30 * MIB,
            FileClassification::AppOwned,
        ),
        file(
            &format!(r"{INSTALL}\plugins\refresh.exe"),
            MIB,
            FileClassification::AppOwned,
        ),
        file(
            &format!(r"{INSTALL}\readme.txt"),
            4096,
            FileClassification::AppOwned,
        ),
    ];
    files.extend(files_in(
        &format!(r"{INSTALL}\bin"),
        640,
        512 * 1024,
        FileClassification::AppOwned,
    ));
    files.extend(files_in(
        &format!(r"{INSTALL}\plugins"),
        412,
        220 * 1024,
        FileClassification::AppOwned,
    ));
    files.extend(files_in(
        &format!(r"{INSTALL}\samples"),
        232,
        96 * 1024,
        FileClassification::AppOwned,
    ));
    let data = r"C:\ProgramData\Fabrikam Data Sync";
    files.extend(files_in(
        data,
        90,
        300 * 1024,
        FileClassification::MachineAppData,
    ));
    let mut not_copied = files_in(
        r"C:\Users\You\AppData\Roaming\Fabrikam Data Sync",
        48,
        64 * 1024,
        FileClassification::UserData,
    );
    for entry in &mut not_copied {
        entry.should_copy = false;
        entry.package_relative_path = None;
    }
    files.extend(not_copied);
    ScanReport {
        schema_version: SCAN_SCHEMA_VERSION.to_owned(),
        generated_at_utc: String::new(),
        source_volume: None,
        other_accounts: Default::default(),
        source_computer_name: None,
        source_windows_version: None,
        source_platform: None,
        program_needs: Vec::new(),
        user_data_profile: None,
        source_app: Some(app.clone()),
        main_executable: ExecutableMetadata {
            path: main.clone(),
            file_name: "fabrikam.exe".to_owned(),
            file_size_bytes: 40 * MIB,
            modified_utc: None,
            version_info: None,
            digital_signature: SignatureStatus::NotPresent,
            external_manifest: None,
            is_dotnet: false,
            dotnet_runtime_version: None,
            dotnet_target_framework: None,
            embedded_manifest: false,
            requested_execution_level: None,
            sxs_dependencies: Vec::new(),
        },
        architecture: AppArchitecture::X64,
        install_location: Some(PathBuf::from(INSTALL)),
        install_method: None,
        authoritative_file_count: 0,
        install_evidence: Vec::new(),
        footprint_locations: vec![
            FootprintLocation {
                path: PathBuf::from(INSTALL),
                kind: FootprintKind::InstallDirectory,
                scope: InstallScope::PerMachine,
                file_count: 0,
                total_bytes: 0,
                copyable_bytes: 0,
                included: true,
            },
            FootprintLocation {
                path: PathBuf::from(data),
                kind: FootprintKind::ProgramData,
                scope: InstallScope::PerMachine,
                file_count: 0,
                total_bytes: 0,
                copyable_bytes: 0,
                included: true,
            },
        ],
        file_inventory: files,
        dependencies: Vec::new(),
        app_local_dlls: Vec::new(),
        registry_entries: vec![
            value(
                RegistryHive::LocalMachine,
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                Some("Fabrikam Tray"),
                &format!(r#""{INSTALL}\bin\tray.exe""#),
            ),
            value(
                RegistryHive::LocalMachine,
                r"Software\Classes\.fab",
                None,
                "Fabrikam.Document",
            ),
            value(
                RegistryHive::LocalMachine,
                r"Software\Classes\Fabrikam.Document\shell\open\command",
                None,
                &format!(r#""{INSTALL}\bin\fabrikam.exe" "%1""#),
            ),
        ],
        com_registrations: Vec::new(),
        file_associations: Vec::new(),
        services: vec![
            service("FabrikamSync", "Fabrikam Sync Service", r"bin\fabsync.exe"),
            service(
                "FabrikamPlugins",
                "Fabrikam Plugin Host",
                r"plugins\host.exe",
            ),
        ],
        drivers: Vec::new(),
        scheduled_tasks: vec![
            task("Nightly sync", r"bin\fabrikam.exe"),
            task("Plugin refresh", r"plugins\refresh.exe"),
        ],
        shortcuts: vec![ShortcutEntry {
            original_lnk_path: PathBuf::from(
                r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Fabrikam Data Sync.lnk",
            ),
            target_path: main,
            arguments: None,
            working_directory: None,
            icon_path: None,
            target_is_selected_app: true,
        }],
        environment_variables: Vec::new(),
        runtime_prerequisites: vec![RuntimePrerequisite {
            name: "Microsoft Visual C++ 2015+ Redistributable (x64)".to_owned(),
            kind: RuntimePrerequisiteKind::VisualCppRedistributable,
            evidence: String::new(),
            install_manually: true,
            present_on_source: None,
            bundle: None,
            presence_probe: None,
            architecture: Some(AppArchitecture::X64),
        }],
        license_artifacts: Vec::new(),
        license_registry_entries: Vec::new(),
        secrets_left_out: vec![LeftOutItem {
            name: format!(r"{INSTALL}\config\credentials.json"),
            why: "its name says it holds credentials".to_owned(),
        }],
        licenses_left_out: vec![LeftOutItem {
            name: format!(r"{INSTALL}\license.lic"),
            why: "the License option is off".to_owned(),
        }],
        settings_left_out: vec![LeftOutItem {
            name: r"HKCU\Software\Fabrikam".to_owned(),
            why: "12 values, which only go with the User data option".to_owned(),
        }],
        empty_folders: Vec::new(),
        firewall_rules: vec![FirewallRuleEntry {
            name: "Fabrikam Data Sync".to_owned(),
            description: None,
            direction: "In".to_owned(),
            action: "Allow".to_owned(),
            enabled: true,
            program: PathBuf::from(INSTALL).join(r"bin\fabsync.exe"),
            service: None,
            protocol: Some("6".to_owned()),
            local_ports: vec!["7777".to_owned()],
            remote_ports: Vec::new(),
            local_addresses: Vec::new(),
            remote_addresses: Vec::new(),
            profiles: vec!["Private".to_owned()],
            not_carried: Vec::new(),
        }],
        certificates: Vec::new(),
        fonts_reported: Vec::new(),
        near_matches: vec![
            near_match(
                "Fabrikam Data Sync Tools",
                "Its name starts with this app's name.",
                false,
                NearMatchKind::Folder,
            ),
            near_match(
                "Fabrikam Updater",
                "A part hidden from the list of apps, from the same publisher, Fabrikam Inc.",
                false,
                NearMatchKind::HiddenEntry,
            ),
            near_match(
                "Fabrikam-1de4eec8",
                "Its name starts with the publisher's name, Fabrikam.",
                true,
                NearMatchKind::Folder,
            ),
        ],
        warnings: Vec::new(),
        items_excluded: Vec::new(),
        confidence_score: 90,
        generated_artifacts: None,
    }
}

const FLAGS: ScanFlags = ScanFlags {
    include_user_data: false,
    include_program_data: false,
    include_license_data: false,
};

/// The app with a scan that has finished, as it would be if it was just looked at. Nothing of the PC
/// the test runs on is in what is shown, whoever is running it.
pub(crate) fn with_ready_scan(app: &mut WinAppMigratorApp) {
    with_ready_scan_of(app, |_| {});
}

/// The same, of a scan that found something else as well.
fn with_ready_scan_of(app: &mut WinAppMigratorApp, change: impl FnOnce(&mut ScanReport)) {
    let selected = app.selected_app().cloned();
    let (Some(selected), Some(key)) = (selected, app.wanted_scan_key()) else {
        panic!("nothing selected to have looked at");
    };
    let mut report = sample_report(&selected);
    change(&mut report);
    let mut footprint = Footprint::of(&report, FLAGS).unwrap_or_else(|error| panic!("{error}"));
    footprint.notes.clear();
    app.scan = ScanState::Ready(Box::new(ReadyScan {
        key,
        report: Arc::new(report),
        footprint: Arc::new(footprint),
        finished: Instant::now(),
    }));
    set_room(app, 356 * GIB);
}

/// Say how much room the drive of the output folder has, so that none of this asks the PC it runs on.
fn set_room(app: &mut WinAppMigratorApp, free_bytes: u64) {
    let folder = app.output_dir.trim().to_owned();
    app.output_stats = Some((
        folder.clone(),
        Some(VolumeStats {
            free_bytes,
            total_bytes: 1024 * GIB,
        }),
    ));
    app.set_stats_asked_for(folder);
}

fn ready_app(app: &mut WinAppMigratorApp) {
    select_first(app);
    with_ready_scan(app);
}

/// A scan that tells a test when it starts, and goes on for as long as the test says, or until it
/// is cancelled. What it was asked is kept.
struct GatedScan {
    started: Receiver<ScanOptions>,
    release: Sender<()>,
}

fn gated_scan(app: &mut WinAppMigratorApp) -> GatedScan {
    let (started_sender, started) = mpsc::channel();
    let (release, wait_for_release) = mpsc::channel::<()>();
    let wait_for_release = Mutex::new(wait_for_release);
    let runner: ScanRunner = Arc::new(move |options: &ScanOptions, monitor: &ScanMonitor| {
        let _ = started_sender.send(options.clone());
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            if monitor.is_cancelled() {
                return Err(WinAppMigratorError::Cancelled);
            }
            let released = wait_for_release
                .lock()
                .ok()
                .and_then(|receiver| receiver.recv_timeout(Duration::from_millis(5)).ok());
            if released.is_some() {
                break;
            }
            assert!(Instant::now() < deadline, "no test released this scan");
        }
        let crate::AppSelection::InstalledApp(app) = &options.selection else {
            panic!("a scan of something else");
        };
        Ok(sample_report(app))
    });
    app.set_scan_runner(runner);
    GatedScan { started, release }
}

/// A window with an app selected, whose scan starts as soon as the window looks, and is held.
fn scanning_harness(mode: ThemeMode) -> (Harness<'static, WinAppMigratorApp>, GatedScan) {
    let mut scan = None;
    let harness = harness_with(mode, |app| {
        select_first(app);
        scan = Some(gated_scan(app));
    });
    let Some(scan) = scan else {
        panic!("the window was set up");
    };
    (harness, scan)
}

fn started(scan: &GatedScan) -> ScanOptions {
    scan.started
        .recv_timeout(Duration::from_secs(30))
        .unwrap_or_else(|error| panic!("no scan started: {error}"))
}

/// Click a row, wherever in its list and the panel it has got to.
fn click_row(harness: &mut Harness<'_, WinAppMigratorApp>, label: &str) {
    scroll_to_row(harness, label);
    harness.get_by_label_contains(label).click();
    harness.run_steps(3);
}

/// Bring a row into view, and wait for it to get there, since scrolling is animated.
fn scroll_to_row(harness: &mut Harness<'_, WinAppMigratorApp>, label: &str) {
    harness.get_by_label_contains(label).scroll_to_me();
    harness.run_steps(SCROLL_ANIMATION_FRAMES);
}

/// Run frames until something is so, which on a worker thread takes as long as it takes.
fn wait_until(
    harness: &mut Harness<'_, WinAppMigratorApp>,
    what: &str,
    is_so: impl Fn(&WinAppMigratorApp) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !is_so(harness.state()) {
        assert!(Instant::now() < deadline, "never came to pass: {what}");
        harness.run_steps(1);
        std::thread::sleep(Duration::from_millis(2));
    }
    harness.run_steps(2);
}

fn is_ready(app: &WinAppMigratorApp) -> bool {
    app.ready_scan().is_some()
}

fn app_name_of(options: &ScanOptions) -> String {
    match &options.selection {
        crate::AppSelection::InstalledApp(app) => app.display_name.clone(),
        crate::AppSelection::ExecutablePath(path) => path.display().to_string(),
    }
}

fn folder_id(path: &str) -> ChoiceId {
    ChoiceId::Folder(path.to_ascii_lowercase())
}

fn open_all(app: &mut WinAppMigratorApp) {
    use crate::ListGroup as Group;
    app.open_groups.extend([
        Group::Program,
        Group::Data,
        Group::Services,
        Group::Startup,
        Group::Tasks,
        Group::Shortcuts,
        Group::Integrations,
        Group::Prerequisites,
        Group::NearMatches,
        Group::LeftOut,
    ]);
}

#[test]
fn size_and_room_are_there_before_anything_is_clicked_and_first() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, ready_app);
        harness.run_steps(2);
        assert!(harness.query_by_label("Files").is_some());
        assert!(harness.query_by_label("Up to").is_some());
        assert!(harness.query_by_label("Free on C:").is_some());
        assert!(harness.query_by_label("356.0 GB").is_some());
        assert!(
            !harness
                .get_by_label("Create installer")
                .accesskit_node()
                .is_disabled(),
            "it fits, and it has been looked at"
        );
        let size_at = harness.get_by_label("Up to").rect().top();
        let list_at = harness.get_by_label_contains("Program files").rect().top();
        assert!(
            size_at < list_at,
            "how big it is comes before what is in it"
        );
        preview(&mut harness, &format!("footprint-{name}"));
    }
}

#[test]
fn it_reads_the_same_at_the_smallest_window() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_sized(mode, egui::vec2(1000.0, 640.0), ready_app);
        harness.run_steps(2);
        assert!(harness.query_by_label("Create installer").is_some());
        assert!(harness.query_by_label("Up to").is_some());
        let button = harness.get_by_label("Create installer").rect();
        assert!(
            button.bottom() <= 640.0,
            "the button stays in view: {button:?}"
        );
        // Not just the top of the panel: what goes in is what the panel is about.
        let panel_top = harness.get_by_label("Output folder").rect().top();
        let row = harness.get_by_label_contains("plugins").rect();
        assert!(
            row.bottom() <= panel_top,
            "{row:?} is behind what starts at {panel_top}"
        );
        preview(&mut harness, &format!("footprint-small-{name}"));
    }
    let mut harness = harness_sized(ThemeMode::Dark, egui::vec2(1000.0, 640.0), |app| {
        ready_app(app);
        open_all(app);
    });
    harness.run_steps(2);
    preview(&mut harness, "footprint-small-open-dark");
}

/// What a maximized window has, on what people have, by the size of the screen and how much it is
/// scaled: 1024x768, then 1366x768 at 100%, 125% and 150%, and 1280x720 at 150%.
const WHAT_SMALL_SCREENS_LEAVE: [(f32, f32, &str); 5] = [
    (1024.0, 697.0, "1024x768"),
    (1366.0, 697.0, "1366x768"),
    (1093.0, 543.0, "1366x768-125"),
    (911.0, 441.0, "1366x768-150"),
    (853.0, 409.0, "1280x720-150"),
];

#[test]
fn on_a_small_screen_what_matters_can_be_reached() {
    for (width, height, name) in WHAT_SMALL_SCREENS_LEAVE {
        let mut harness = harness_sized(ThemeMode::Dark, egui::vec2(width, height), ready_app);
        harness.run_steps(2);
        let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, height));
        for label in [
            "Create installer",
            "Browse",
            "Fabrikam Data Sync, Fabrikam Inc., version 1.7.3",
        ] {
            let at = harness.get_by_label(label).rect();
            assert!(window.contains_rect(at), "{name}: {label} is at {at:?}");
        }
        assert!(harness.query_by_label("Up to").is_some(), "{name}");
        // It scrolls, and whatever it scrolls to is not behind what is pinned below it.
        let panel_top = harness.get_by_label("Output folder").rect().top();
        assert!(
            panel_top > 120.0,
            "{name}: only {panel_top} above the panel"
        );
        preview(&mut harness, &format!("footprint-on-{name}-dark"));
    }
}

/// What is in the columns with a width of their own is all there, however little there is of the rest.
#[test]
fn a_small_window_does_not_cut_what_a_row_says_of_how_it_was_installed() {
    let ctx = egui::Context::default();
    crate::theme::install_fonts(&ctx, &crate::fonts_dir());
    let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
        let width_of = |text: &str, font: egui::FontId| {
            ui.painter()
                .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
                .size()
                .x
        };
        let cells = crate::views::column_rects(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(400.0, 36.0),
        ));
        let font = crate::theme::regular(13.0);
        assert!(width_of("Per-machine", font.clone()) <= cells[3].width());
        assert!(width_of("Per-user", font.clone()) <= cells[3].width());
        assert!(width_of("ARM64", font.clone()) <= cells[4].width());
        assert!(width_of("10.0.10007", font) <= cells[2].width());
        assert!(cells[0].width() > cells[1].width());
    });
    output.textures_delta.clear();
}

#[test]
fn every_group_reads_in_plain_words_and_in_the_installers_own() {
    let mut harness = harness_with(ThemeMode::Light, |app| {
        ready_app(app);
        app.open_groups.clear();
    });
    harness.run_steps(2);
    for heading in [
        "Program files",
        "Data folders",
        "Services",
        "Starts automatically",
        "Scheduled tasks",
        "Shortcuts",
        "What ties it into Windows",
        "What it needs to run",
        "Might also be part of it",
        "Left out, and why",
    ] {
        assert!(
            harness.query_by_label_contains(heading).is_some(),
            "{heading}"
        );
    }
    preview(&mut harness, "footprint-closed-light");

    // Each of them, opened one at a time, says what the installer's list says of the same thing.
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    click_row(&mut harness, "Services,");
    assert!(
        harness
            .query_by_label_contains("Fabrikam Sync Service, Service. Runs fabsync.exe as")
            .is_some()
    );
    preview(&mut harness, "footprint-services-dark");
    click_row(&mut harness, "Services,");
    click_row(&mut harness, "Scheduled tasks,");
    assert!(
        harness
            .query_by_label_contains(r"\Fabrikam\Nightly sync, Scheduled task. Runs as SYSTEM")
            .is_some()
    );
    click_row(&mut harness, "Scheduled tasks,");
    click_row(&mut harness, "What ties it into Windows,");
    assert!(harness.query_by_label_contains("File type,").is_some());
    assert!(
        harness
            .query_by_label_contains(".fab, Files of this type open with the app")
            .is_some()
    );
    assert!(harness.query_by_label_contains("Firewall rule,").is_some());
    preview(&mut harness, "footprint-integrations-dark");
    click_row(&mut harness, "What ties it into Windows,");
    click_row(&mut harness, "What it needs to run,");
    assert!(
        harness
            .query_by_label_contains("Microsoft Visual C++ 2015+ Redistributable (x64), Included")
            .is_some()
    );
}

#[test]
fn what_stays_out_is_named_and_cannot_be_ticked() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        app.open_groups.clear();
        app.open_groups.insert(crate::ListGroup::LeftOut);
    });
    harness.run_steps(2);
    assert!(
        harness
            .query_by_label_contains(CREDENTIALS_LEFT_OUT_TEXT)
            .is_some(),
        "in the installer's words"
    );
    let secret = harness.get_by_label_contains("credentials.json");
    assert_eq!(
        secret.accesskit_node().role(),
        egui::accesskit::Role::Label,
        "there is nothing to tick"
    );
    let license = harness.get_by_label_contains("license.lic");
    assert_eq!(
        license.accesskit_node().role(),
        egui::accesskit::Role::Label
    );
    assert!(
        harness
            .query_by_label_contains(SETTINGS_LEFT_OUT_TEXT)
            .is_some(),
        "what an account keeps is said to be left out"
    );
    let settings = harness.get_by_label_contains(r"HKCU\Software\Fabrikam");
    assert_eq!(
        settings.accesskit_node().role(),
        egui::accesskit::Role::Label,
        "and the way to bring it is the User data option, not a box here"
    );
    scroll_to_row(&mut harness, "license.lic");
    preview(&mut harness, "footprint-left-out-dark");
    let mut light = harness_with(ThemeMode::Light, |app| {
        ready_app(app);
        app.open_groups.clear();
        app.open_groups.insert(crate::ListGroup::LeftOut);
    });
    light.run_steps(2);
    scroll_to_row(&mut light, "license.lic");
    preview(&mut light, "footprint-left-out-light");
}

#[test]
fn leaving_a_folder_out_takes_effect_at_once_and_says_what_goes_with_it() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        leaving_a_folder_out(mode, name);
    }
}

fn leaving_a_folder_out(mode: ThemeMode, name: &str) {
    let mut harness = harness_sized(mode, egui::vec2(1240.0, 900.0), ready_app);
    harness.run_steps(2);
    let before = harness
        .state()
        .build_inputs()
        .map(|inputs| inputs.payload_bytes);
    click_row(&mut harness, "plugins,");
    let plugins = folder_id(&format!(r"{INSTALL}\plugins"));
    assert!(harness.state().choices.is_left_out(&plugins));
    let after = harness
        .state()
        .build_inputs()
        .map(|inputs| inputs.payload_bytes);
    assert!(
        after < before,
        "{after:?} {before:?}: it counts straight away"
    );

    wait_until(&mut harness, "what goes with it is known", |app| {
        app.also_is_current()
    });
    let also: Vec<String> = harness
        .state()
        .also
        .items
        .iter()
        .map(|item| item.title.clone())
        .collect();
    assert!(
        also.iter().any(|title| title == "Fabrikam Plugin Host"),
        "{also:?}"
    );
    assert!(
        also.iter().any(|title| title.ends_with("Plugin refresh")),
        "{also:?}"
    );
    assert!(
        !also.iter().any(|title| title == "Fabrikam Sync Service"),
        "this one runs a file that is staying: {also:?}"
    );
    assert!(
        harness
            .query_by_label_contains("That also leaves out")
            .is_some(),
        "it says so plainly"
    );
    // This one is in the README, without a pointer on it.
    harness.event(egui::Event::PointerGone);
    harness.run_steps(2);
    preview(&mut harness, &format!("footprint-left-out-folder-{name}"));
    assert!(
        !harness
            .get_by_label("Create installer")
            .accesskit_node()
            .is_disabled(),
        "and it can be built like that"
    );

    // And ticking it again brings all of it back.
    click_row(&mut harness, "plugins,");
    wait_until(&mut harness, "what goes with it is known", |app| {
        app.also_is_current() && app.also.items.is_empty()
    });
    assert_eq!(
        harness
            .state()
            .build_inputs()
            .map(|inputs| inputs.payload_bytes),
        before
    );
}

#[test]
fn what_ties_it_into_windows_is_said_as_what_was_left_out_leaves_it() {
    let mut harness = harness_sized(ThemeMode::Dark, egui::vec2(1240.0, 900.0), |app| {
        select_first(app);
        with_ready_scan_of(app, |report| {
            report.firewall_rules[0].program = PathBuf::from(INSTALL).join(r"plugins\host.exe");
        });
        app.open_groups.insert(crate::ListGroup::Integrations);
    });
    harness.run_steps(2);
    click_row(&mut harness, "plugins,");
    wait_until(&mut harness, "what goes with it is known", |app| {
        app.also_is_current()
    });
    let integrations = harness
        .state()
        .also
        .integrations
        .clone()
        .unwrap_or_default();
    assert!(
        !integrations
            .iter()
            .any(|row| row.kind == SystemChangeKind::FirewallRule),
        "{integrations:?}"
    );
    assert!(
        harness
            .state()
            .also
            .items
            .iter()
            .any(|item| item.title == "Fabrikam Data Sync"),
        "and it says it went: {:?}",
        harness.state().also.items
    );
    scroll_to_row(&mut harness, "Firewall rule");
    assert!(
        harness
            .query_by_label_contains("Nothing left of it, because of what you left out")
            .is_some(),
        "the kind stays, with nothing left of it"
    );
}

#[test]
fn a_license_in_a_folder_left_out_is_said_to_still_go() {
    let license = PathBuf::from(INSTALL).join(r"plugins\fabrikam.lic");
    let mut harness = harness_sized(ThemeMode::Dark, egui::vec2(1240.0, 900.0), |app| {
        select_first(app);
        with_ready_scan_of(app, |report| {
            report.license_artifacts.push(LicenseArtifact {
                kind: LicenseArtifactKind::File,
                framework: LicenseFramework::FileBased,
                path: Some(license.clone()),
                registry_location: None,
                detail: String::new(),
                machine_bound: false,
                requires_admin: false,
                transferable: TransferableHint::Likely,
                should_copy: true,
                package_relative_path: None,
                size_bytes: Some(5),
                hash_sha256: None,
                store_files: Vec::new(),
            });
        });
    });
    harness.run_steps(2);
    assert!(
        harness
            .query_by_label_contains("still goes in, because that goes by the License")
            .is_none(),
        "nothing is left out yet"
    );
    click_row(&mut harness, "plugins,");
    assert!(
        harness
            .query_by_label_contains("still goes in, because that goes by the License")
            .is_some(),
        "it is told, by name"
    );
    assert!(harness.query_by_label_contains("fabrikam.lic").is_some());
}

#[test]
fn a_build_waits_until_what_goes_with_a_choice_is_known() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let id = folder_id(&format!(r"{INSTALL}\plugins"));
    harness.state_mut().set_included(&id, false);
    assert!(!harness.state().also_is_current());
    assert!(!harness.state().can_build());
    harness.state_mut().start_migration();
    assert!(!harness.state().is_building());
}

#[test]
fn the_main_program_cannot_be_left_out() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let bin = harness.get_by_label_contains("Holds the main program");
    assert!(bin.accesskit_node().is_disabled());
    let root = harness.get_by_label_contains("These always go with the main program");
    assert!(root.accesskit_node().is_disabled());
    // Whatever is asked, a row that holds it is in.
    let id = folder_id(&format!(r"{INSTALL}\bin"));
    harness.state_mut().set_included(&id, false);
    let inputs = harness.state().build_inputs();
    let footprint = harness.state().shown_footprint();
    let (Some(inputs), Some(footprint)) = (inputs, footprint) else {
        panic!("it has been looked at");
    };
    let all: u64 = footprint.folders.iter().map(|row| row.bytes).sum();
    assert_eq!(inputs.payload_bytes, all);
}

#[test]
fn a_service_a_task_and_a_group_can_each_be_left_out() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        app.open_groups.clear();
        app.open_groups.insert(crate::ListGroup::Services);
        app.open_groups.insert(crate::ListGroup::Tasks);
    });
    harness.run_steps(2);
    click_row(&mut harness, "Fabrikam Plugin Host,");
    click_row(&mut harness, "Nightly sync,");
    let state = harness.state();
    assert!(
        state
            .choices
            .is_left_out(&ChoiceId::Service("fabrikamplugins".to_owned()))
    );
    assert!(
        state
            .choices
            .is_left_out(&ChoiceId::Task(r"\fabrikam\nightly sync".to_owned()))
    );
}

#[test]
fn a_row_can_be_worked_from_the_keyboard_and_says_what_it_is() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    scroll_to_row(&mut harness, "samples,");
    let row = harness.get_by_label_contains("samples,");
    assert_eq!(row.accesskit_node().role(), egui::accesskit::Role::CheckBox);
    row.focus();
    harness.run_steps(1);
    harness.key_press(egui::Key::Space);
    harness.run_steps(3);
    assert!(
        harness
            .state()
            .choices
            .is_left_out(&folder_id(&format!(r"{INSTALL}\samples")))
    );
    let heading = harness.get_by_label_contains("Program files,");
    assert_eq!(
        heading.accesskit_node().role(),
        egui::accesskit::Role::Button
    );
    heading.focus();
    harness.run_steps(1);
    harness.key_press(egui::Key::Enter);
    // What goes with the choice above comes from a worker, and until it does the frames are busy.
    let closed = (0..500).any(|_| {
        harness.run_steps(1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        harness.query_by_label_contains("samples,").is_none()
    });
    assert!(closed);
}

#[test]
fn near_matches_start_unticked_say_why_and_ticking_one_looks_again_for_it() {
    let mut harness = harness_with(ThemeMode::Light, ready_app);
    harness.run_steps(2);
    scroll_to_row(&mut harness, "Fabrikam Updater, A part hidden");
    let row = harness.get_by_label_contains("Fabrikam Updater, A part hidden");
    assert_eq!(
        row.accesskit_node().toggled(),
        Some(egui::accesskit::Toggled::False)
    );
    assert!(
        harness
            .query_by_label_contains("Its name starts with this app's name")
            .is_some()
    );
    preview(&mut harness, "footprint-near-matches-light");

    let scan = gated_scan(harness.state_mut());
    click_row(&mut harness, "Fabrikam Updater, A part hidden");
    let asked = started(&scan);
    assert_eq!(asked.near_matches, ["folder:fabrikam updater"]);
    assert!(!harness.state().can_build());
    assert!(
        harness
            .query_by_label_contains("Create installer")
            .is_some_and(|button| button.accesskit_node().is_disabled())
    );
    assert!(
        harness
            .query_by_label_contains("Fabrikam Updater,")
            .is_some(),
        "what it found stays there while it looks again"
    );
    preview(&mut harness, "footprint-looking-again-light");
    let _ = scan.release.send(());
    wait_until(&mut harness, "it has looked again", is_ready);
    // That scan did not say it took it, so it is not ticked, and not asked for again.
    assert!(harness.state().near_picks.is_empty());
    assert!(harness.state().ready_scan().is_some());
}

#[test]
fn a_near_match_in_a_data_folder_comes_with_the_user_data_option() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let row = harness.get_by_label_contains("Comes with the User data option");
    assert!(row.accesskit_node().is_disabled());
    assert!(
        row.accesskit_node()
            .label()
            .is_some_and(|label| label.contains("Fabrikam-1de4eec8"))
    );
}

#[test]
fn one_waiting_for_the_user_data_option_is_not_counted_as_added() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let Some(id) = harness.state().ready_scan().and_then(|ready| {
        ready
            .footprint
            .near_matches
            .iter()
            .find(|near| near.follows_user_data)
            .map(|near| near.id.clone())
    }) else {
        panic!("the sample has one");
    };
    harness.state_mut().near_picks.insert(id, true);
    harness.run_steps(2);
    assert!(
        harness.query_by_label_contains("0 of 3 added").is_some(),
        "it is not in while that option is off"
    );
}

#[test]
fn scanning_says_so_and_the_build_waits_for_it() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let (mut harness, scan) = scanning_harness(mode);
        started(&scan);
        harness.run_steps(2);
        assert!(harness.query_by_label("Stop").is_some());
        assert!(
            harness
                .get_by_label("Create installer")
                .accesskit_node()
                .is_disabled()
        );
        preview(&mut harness, &format!("footprint-scanning-{name}"));
        let _ = scan.release.send(());
        wait_until(&mut harness, "the scan finished", is_ready);
        assert!(harness.query_by_label("Stop").is_none());
        assert!(harness.query_by_label("Up to").is_some());
    }
}

#[test]
fn selecting_another_app_stops_the_scan_and_its_result_never_turns_up() {
    let (mut harness, scan) = scanning_harness(ThemeMode::Dark);
    let first = started(&scan);
    assert_eq!(app_name_of(&first), "Fabrikam Data Sync");
    let ScanState::Running { monitor, .. } = &harness.state().scan else {
        panic!("it should be running");
    };
    let first_monitor = monitor.clone();

    harness.state_mut().select_app(3);
    harness.run_steps(1);
    assert!(first_monitor.is_cancelled(), "it is told to stop");
    let second = started(&scan);
    assert_eq!(app_name_of(&second), "Tailspin Toolbar");
    assert!(second.near_matches.is_empty());

    let _ = scan.release.send(());
    wait_until(&mut harness, "the second scan finished", is_ready);
    let Some(ready) = harness.state().ready_scan() else {
        panic!("it finished");
    };
    assert_eq!(
        ready
            .report
            .source_app
            .as_ref()
            .map(|app| app.display_name.as_str()),
        Some("Tailspin Toolbar"),
        "the scan a build would go by is of the app selected"
    );
}

#[test]
fn a_result_from_a_scan_of_something_else_is_not_taken() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let state = harness.state_mut();
    let wrong_id = state.next_scan_id_for_tests() + 40;
    state.deliver_scan_for_tests(wrong_id);
    harness.run_steps(2);
    assert!(
        harness.state().ready_scan().is_some_and(|ready| ready
            .report
            .source_app
            .as_ref()
            .is_some_and(|app| app.display_name == "Fabrikam Data Sync")),
        "what it had stays"
    );
}

#[test]
fn a_choice_that_changes_what_a_scan_reads_looks_again_and_keeps_the_rest() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let plugins = folder_id(&format!(r"{INSTALL}\plugins"));
    harness.state_mut().set_included(&plugins, false);
    wait_until(&mut harness, "what goes with it is known", |app| {
        app.also_is_current()
    });

    for (label, expect) in [
        ("License and activation data,", (false, true)),
        ("User data,", (true, true)),
    ] {
        let scan = gated_scan(harness.state_mut());
        click_row(&mut harness, label);
        let asked = started(&scan);
        assert_eq!(
            (asked.include_user_data, asked.include_license_data),
            expect
        );
        assert_eq!(asked.include_program_data, expect.0);
        assert!(
            !harness.state().can_build(),
            "that scan is not of this any more"
        );
        let _ = scan.release.send(());
        wait_until(&mut harness, "it has looked again", is_ready);
        wait_until(&mut harness, "what goes with it is known", |app| {
            app.also_is_current()
        });
        assert!(
            harness.state().choices.is_left_out(&plugins),
            "what is chosen goes by what it is called"
        );
    }
}

#[test]
fn another_account_or_another_source_is_another_scan() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    let Some(before) = harness.state().wanted_scan_key() else {
        panic!("something is selected");
    };
    harness
        .state_mut()
        .set_chosen_profile_for_tests("id of bob");
    assert_ne!(harness.state().wanted_scan_key(), Some(before.clone()));
    assert!(
        harness.state().ready_scan().is_none(),
        "a build cannot go by the last one"
    );
    assert!(!harness.state().can_build());

    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    harness.run_steps(2);
    harness.state_mut().set_source_for_tests(Path::new(r"E:\"));
    harness.run_steps(2);
    assert!(
        matches!(harness.state().scan, ScanState::None),
        "nothing is selected on it yet"
    );
    assert!(harness.state().shown_footprint().is_none());
}

#[test]
fn a_failed_scan_says_why_and_can_be_tried_again() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |app| {
            select_first(app);
            let runner: ScanRunner = Arc::new(|_: &ScanOptions, _: &ScanMonitor| {
                Err(WinAppMigratorError::InvalidInput(
                    "could not find an executable for Fabrikam Data Sync. A driver, runtime, or \
                     add-in may have no program of its own to package."
                        .to_owned(),
                ))
            });
            app.set_scan_runner(runner);
        });
        wait_until(&mut harness, "it failed", |app| {
            matches!(app.scan, ScanState::Failed { .. })
        });
        assert!(
            harness
                .query_by_label_contains("could not find an executable")
                .is_some()
        );
        assert!(harness.query_by_label("Try again").is_some());
        assert!(
            harness
                .get_by_label("Create installer")
                .accesskit_node()
                .is_disabled()
        );
        preview(&mut harness, &format!("footprint-failed-{name}"));
    }
}

#[test]
fn stopping_a_scan_stays_stopped_until_asked() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let (mut harness, scan) = scanning_harness(mode);
        started(&scan);
        harness.run_steps(1);
        harness.get_by_label("Stop").click();
        harness.run_steps(3);
        assert!(matches!(harness.state().scan, ScanState::Stopped { .. }));
        assert!(harness.query_by_label_contains("Stopped.").is_some());
        assert!(
            harness
                .get_by_label("Create installer")
                .accesskit_node()
                .is_disabled()
        );
        preview(&mut harness, &format!("footprint-stopped-{name}"));
        // No other scan starts by itself.
        assert!(
            scan.started
                .recv_timeout(Duration::from_millis(200))
                .is_err()
        );
        harness.get_by_label("Look again").click();
        harness.run_steps(3);
        started(&scan);
        let _ = scan.release.send(());
        wait_until(&mut harness, "it finished", is_ready);
    }
}

#[test]
fn looking_again_takes_a_new_scan_and_a_stale_one_says_how_old_it_is() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        if let ScanState::Ready(ready) = &mut app.scan
            && let Some(long_ago) = Instant::now().checked_sub(Duration::from_secs(45 * 60))
        {
            ready.finished = long_ago;
        }
    });
    harness.run_steps(2);
    assert!(
        harness
            .query_by_label_contains("This was looked at 45:00 ago")
            .is_some()
    );
    preview(&mut harness, "footprint-old-dark");

    let scan = gated_scan(harness.state_mut());
    harness.get_by_label("Look again").click();
    harness.run_steps(3);
    started(&scan);
    assert!(harness.state().ready_scan().is_none());
    let _ = scan.release.send(());
    wait_until(&mut harness, "it finished", is_ready);
}

#[test]
fn without_the_room_it_says_where_and_how_much_and_will_not_start() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        set_room(app, 5 * MIB);
    });
    harness.run_steps(2);
    assert!(
        harness
            .query_by_label_contains("There is not enough room. C: needs up to")
            .is_some()
    );
    assert!(harness.query_by_label_contains("another drive").is_some());
    assert!(
        harness
            .get_by_label("Create installer")
            .accesskit_node()
            .is_disabled()
    );
    preview(&mut harness, "footprint-no-room-dark");
    assert!(!harness.state().can_build());
}

#[test]
fn a_drive_that_will_not_say_does_not_stop_a_build_and_is_said_not_to_have_said() {
    let mut harness = harness_with(ThemeMode::Dark, ready_app);
    let folder = harness.state().output_dir.trim().to_owned();
    harness.state_mut().output_stats = Some((folder, None));
    harness.run_steps(2);
    assert!(
        harness
            .query_by_label_contains("would not say how much room")
            .is_some()
    );
    assert!(harness.query_by_label("Unknown").is_some());
    assert!(harness.state().can_build());
}

#[test]
fn a_running_build_locks_every_choice() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        app.set_building_for_tests();
    });
    harness.run_steps(2);
    scroll_to_row(&mut harness, "samples,");
    let row = harness.get_by_label_contains("samples,");
    assert!(row.accesskit_node().is_disabled());
    assert!(
        harness
            .query_by_label_contains("License and activation data")
            .is_some_and(|option| option.accesskit_node().is_disabled())
    );
}

#[test]
fn a_list_of_any_length_only_draws_what_is_in_view() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        with_ready_scan(app);
        if let ScanState::Ready(ready) = &mut app.scan {
            let mut footprint = (*ready.footprint).clone();
            let template = footprint.folders.last().cloned();
            if let Some(template) = template {
                footprint.folders.extend((0..200_000).map(|index| {
                    let mut row = template.clone();
                    row.name = format!("folder {index}");
                    row.id = folder_id(&format!(r"{INSTALL}\folder {index}"));
                    row
                }));
            }
            ready.footprint = Arc::new(footprint);
        }
    });
    let started = Instant::now();
    harness.run_steps(3);
    let rows = harness.query_all_by_label_contains("folder ").count();
    assert!(rows < 40, "{rows} rows on screen");
    assert!(rows > 0);
    assert!(
        started.elapsed() < Duration::from_secs(30),
        "{:?}",
        started.elapsed()
    );
}

#[test]
fn a_long_list_keeps_a_row_with_the_focus_in_view() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        if let ScanState::Ready(ready) = &mut app.scan {
            let mut footprint = (*ready.footprint).clone();
            let template = footprint.folders.last().cloned();
            if let Some(template) = template {
                footprint.folders.extend((0..40).map(|index| {
                    let mut row = template.clone();
                    row.name = format!("many {index}");
                    row.id = folder_id(&format!(r"{INSTALL}\many {index}"));
                    row
                }));
            }
            ready.footprint = Arc::new(footprint);
        }
    });
    harness.run_steps(2);
    let panel_top = harness.get_by_label("Output folder").rect().top();
    harness.get_by_label_contains("many 0,").focus();
    harness.run_steps(SCROLL_ANIMATION_FRAMES);
    let row = harness.get_by_label_contains("many 0,").rect();
    assert!(
        row.bottom() <= panel_top,
        "{row:?} is behind what starts at {panel_top}"
    );
}

#[test]
fn nothing_of_a_real_pc_is_in_what_these_show() {
    let harness = harness_with(ThemeMode::Dark, |app| {
        ready_app(app);
        open_all(app);
    });
    let user = std::env::var("USERNAME").unwrap_or_default();
    if user.is_empty() {
        return;
    }
    let named: Vec<_> = harness
        .query_all_by_label_contains(&user)
        .map(|node| node.accesskit_node().label())
        .collect();
    assert!(named.is_empty(), "{named:?}");
}
