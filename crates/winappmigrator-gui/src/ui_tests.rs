//! Drives the real screen headlessly. Set `WAM_UI_PREVIEW_DIR` to also write a PNG
//! of each state, for checking the design without touching a live window.

use std::cmp::Ordering;
use std::path::PathBuf;

use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

use winappmigrator_core::space::DriveRoom;
use winappmigrator_core::{
    AppArchitecture, BuildStep, InstallScope, InstalledApp, PackageResult, ProfileChoice,
    ProfilePicker, ProfileRow, RoomCheck, VerificationReport, Warning, WarningSeverity,
};

use crate::theme::{Role, ThemeMode};
use crate::views::{compare_versions, outcome_summary};
use crate::{
    BuildProgress, InventoryState, MigrationOutcome, OperationState, WinAppMigratorApp, fonts_dir,
    theme,
};

const PREVIEW_DIR_VARIABLE: &str = "WAM_UI_PREVIEW_DIR";

fn sample_app(name: &str, publisher: &str, version: &str, scope: InstallScope) -> InstalledApp {
    InstalledApp {
        display_name: name.to_owned(),
        publisher: Some(publisher.to_owned()),
        version: Some(version.to_owned()),
        install_location: Some(PathBuf::from(format!(r"C:\Program Files\{name}"))),
        uninstall_string: None,
        main_executable: None,
        registry_source_path: String::new(),
        architecture: AppArchitecture::X64,
        install_scope: scope,
        owner_profile: None,
    }
}

fn sample_apps() -> Vec<InstalledApp> {
    vec![
        sample_app(
            "Contoso Notes",
            "Contoso Ltd.",
            "11.4.2",
            InstallScope::PerUser,
        ),
        sample_app(
            "Fabrikam Data Sync",
            "Fabrikam Inc.",
            "1.7.3",
            InstallScope::PerMachine,
        ),
        sample_app(
            "Northwind Photo Kit with an unusually long product name that cannot fit in its column",
            "Northwind Media",
            "6.1.0",
            InstallScope::PerMachine,
        ),
        sample_app(
            "Tailspin Toolbar",
            "Tailspin Toys",
            "9.11.0",
            InstallScope::PerMachine,
        ),
        sample_app(
            "Wide World Importer",
            "Wide World Importers",
            "10.2.0",
            InstallScope::PerUser,
        ),
    ]
}

fn sample_outcome(warnings: Vec<Warning>, package_valid: bool) -> MigrationOutcome {
    MigrationOutcome {
        app_name: "Fabrikam Data Sync".to_owned(),
        package: PackageResult {
            package_root: PathBuf::from(r"C:\staging"),
            manifest_path: PathBuf::from(r"C:\staging\manifest.json"),
            reports_dir: PathBuf::from(r"C:\staging\reports"),
            files_copied: 1284,
            warnings,
            self_extracting_archive_path: Some(PathBuf::from(
                r"C:\Users\You\WinAppMigrator\Fabrikam Data Sync\Fabrikam_Data_Sync_installer.exe",
            )),
            data_file_path: None,
        },
        verification: VerificationReport {
            schema_version: String::new(),
            verified_at_utc: String::new(),
            package_root: PathBuf::from(r"C:\staging"),
            target_root: None,
            package_valid,
            restore_valid: None,
            items: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            report_json: None,
            report_markdown: None,
        },
        installer_bytes: 412 * 1024 * 1024,
    }
}

fn sample_room() -> RoomCheck {
    RoomCheck {
        drives: vec![DriveRoom {
            drive: "C:".to_owned(),
            needed_bytes: 412 * 1024 * 1024,
            free_bytes: Some(356 * 1024 * 1024 * 1024),
            what_for: vec!["the installer"],
        }],
    }
}

fn copy_failure() -> Warning {
    Warning::new(
        "file_copy_failed",
        WarningSeverity::Warning,
        "Could not copy file into package: the file is in use.",
        Some(r"C:\Program Files\Fabrikam\engine.dll".to_owned()),
    )
}

/// A scan that finds what the sample app has, with nothing of the PC this runs on.
fn fake_scan() -> crate::ScanRunner {
    std::sync::Arc::new(|options, _monitor| match &options.selection {
        winappmigrator_core::AppSelection::InstalledApp(app) => {
            Ok(crate::ui_tests_footprint::sample_report(app))
        }
        winappmigrator_core::AppSelection::ExecutablePath(_) => Err(
            winappmigrator_core::WinAppMigratorError::InvalidInput("no such app".to_owned()),
        ),
    })
}

fn loaded_app(mode: ThemeMode) -> impl FnOnce(&egui::Context) -> WinAppMigratorApp {
    move |ctx| {
        let mut app = WinAppMigratorApp::new(ctx.clone(), mode);
        app.apps = sample_apps();
        app.inventory_state = InventoryState::Loaded;
        app.stub_available = true;
        app.set_scan_runner(fake_scan());
        app
    }
}

pub(crate) fn harness_with(
    mode: ThemeMode,
    setup: impl FnOnce(&mut WinAppMigratorApp),
) -> Harness<'static, WinAppMigratorApp> {
    harness_sized(mode, egui::vec2(1240.0, 780.0), setup)
}

pub(crate) fn harness_sized(
    mode: ThemeMode,
    size: egui::Vec2,
    setup: impl FnOnce(&mut WinAppMigratorApp),
) -> Harness<'static, WinAppMigratorApp> {
    let ctx = egui::Context::default();
    theme::install_fonts(&ctx, &fonts_dir());
    let mut app = loaded_app(mode)(&ctx);
    setup(&mut app);

    let mut builder = Harness::builder();
    builder = builder.with_size(size);
    let builder = if preview_dir().is_some() {
        builder.wgpu()
    } else {
        builder
    };
    let mut harness = builder
        .with_context_setup(move |ctx| {
            theme::install_fonts(ctx, &fonts_dir());
            crate::Theme::new(mode).apply(ctx);
        })
        .build_ui_state(
            |ui, app: &mut WinAppMigratorApp| {
                // What a frame of the real window does before it draws.
                app.take_up_the_theme_of_windows(ui.ctx());
                app.poll_workers();
                app.keep_scan_current();
                // The harness adds a margin around its UI. The real window has none.
                let window = ui.ctx().content_rect();
                ui.scope_builder(egui::UiBuilder::new().max_rect(window), |ui| app.draw(ui));
            },
            app,
        );
    harness.run_steps(3);
    harness
}

fn preview_dir() -> Option<PathBuf> {
    std::env::var_os(PREVIEW_DIR_VARIABLE).map(PathBuf::from)
}

/// Save a PNG of the current state when previews were asked for.
pub(crate) fn preview(harness: &mut Harness<'_, WinAppMigratorApp>, name: &str) {
    let Some(directory) = preview_dir() else {
        return;
    };
    harness.run_steps(2);
    let Ok(image) = harness.render() else {
        return;
    };
    let _ = std::fs::create_dir_all(&directory);
    let _ = image.save(directory.join(format!("{name}.png")));
}

pub(crate) fn select_first(app: &mut WinAppMigratorApp) {
    app.select_app(1);
    // The default comes from the real user profile, which has no place in a screenshot.
    app.output_dir = r"C:\Users\You\WinAppMigrator\Fabrikam Data Sync".to_owned();
}

/// [`select_first`], with the app looked at and found to have what the sample has.
fn select_first_looked_at(app: &mut WinAppMigratorApp) {
    select_first(app);
    crate::ui_tests_footprint::with_ready_scan(app);
}

#[test]
fn nothing_selected_invites_a_pick() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |_| {});
        assert!(harness.query_by_label("Pick an application").is_some());
        assert!(harness.query_by_label("Create installer").is_none());
        preview(&mut harness, &format!("empty-{name}"));
    }
}

#[test]
fn selected_app_offers_the_build() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, select_first_looked_at);
        assert!(harness.query_by_label("Create installer").is_some());
        assert!(
            harness
                .query_by_label_contains("License and activation data")
                .is_some()
        );
        preview(&mut harness, &format!("selected-{name}"));
    }
}

fn profile_row(name: &str, is_default: bool) -> ProfileRow {
    ProfileRow {
        id: format!("id of {name}"),
        name: name.to_owned(),
        has_data: true,
        is_default,
    }
}

fn with_picker(app: &mut WinAppMigratorApp, picker: ProfilePicker) {
    select_first(app);
    app.picker_receiver = None;
    app.chosen_profile = picker.selected.clone();
    app.profile_picker = Some(picker);
}

#[test]
fn with_one_account_there_is_nothing_to_choose() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        with_picker(
            app,
            ProfilePicker {
                rows: Vec::new(),
                selected: Some("id of alice".to_owned()),
                note: None,
            },
        );
        app.backup_user_data = true;
    });
    harness.run_steps(1);
    assert!(harness.query_by_label("Whose data").is_none());
    assert_eq!(
        harness.state().profile_choice(),
        ProfileChoice::Profile("id of alice".to_owned())
    );
}

#[test]
fn accounts_with_data_can_be_chosen_between_and_a_build_goes_by_the_choice() {
    let picker = ProfilePicker {
        rows: vec![profile_row("alice", true), profile_row("bob", false)],
        selected: Some("id of alice".to_owned()),
        note: None,
    };
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        with_picker(app, picker);
        app.backup_user_data = true;
    });
    harness.run_steps(1);
    assert!(harness.query_by_label("Whose data").is_some());
    assert_eq!(
        harness.state().profile_choice(),
        ProfileChoice::Profile("id of alice".to_owned())
    );
    preview(&mut harness, "profiles-dark");

    harness.get_by_label("bob").click();
    harness.run_steps(2);
    assert_eq!(
        harness.state().profile_choice(),
        ProfileChoice::Profile("id of bob".to_owned())
    );
}

#[test]
fn the_choice_is_about_user_data_and_only_shows_with_it() {
    let picker = ProfilePicker {
        rows: vec![profile_row("alice", true), profile_row("bob", false)],
        selected: Some("id of alice".to_owned()),
        note: None,
    };
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        with_picker(app, picker);
        app.backup_user_data = false;
    });
    harness.run_steps(1);
    assert!(harness.query_by_label("Whose data").is_none());
}

#[test]
fn when_who_is_signed_in_cannot_be_told_that_is_said_and_no_one_is_assumed() {
    let picker = ProfilePicker {
        rows: vec![profile_row("alice", false), profile_row("bob", false)],
        selected: None,
        note: Some(
            "Could not tell who is signed in (no reason). Choose whose data to include.".to_owned(),
        ),
    };
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        with_picker(app, picker);
        app.backup_user_data = true;
    });
    harness.run_steps(1);
    assert!(
        harness
            .query_by_label_contains("Could not tell who is signed in")
            .is_some()
    );
    assert_eq!(harness.state().profile_choice(), ProfileChoice::Nobody);
    preview(&mut harness, "profiles-unknown-dark");

    harness.get_by_label("alice").click();
    harness.run_steps(2);
    assert_eq!(
        harness.state().profile_choice(),
        ProfileChoice::Profile("id of alice".to_owned())
    );
}

#[test]
fn before_the_accounts_have_been_looked_at_a_build_goes_by_the_default() {
    let harness = harness_with(ThemeMode::Dark, select_first_looked_at);
    assert_eq!(harness.state().profile_choice(), ProfileChoice::Default);
}

#[test]
fn a_running_build_locks_the_selection() {
    // Selecting another row mid-build used to reset the state and re-enable the build
    // button, starting a second build that deleted the first one's staging folder.
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        select_first_looked_at(app);
        app.migrate_state = OperationState::Running;
        app.build_progress = Some(BuildProgress::Build(BuildStep::Packing {
            done: 412,
            total: 1284,
            bytes_done: 90 * 1024 * 1024,
            bytes_total: 410 * 1024 * 1024,
        }));
        app.build_room = Some(sample_room());
    });
    assert!(harness.query_by_label("Create installer").is_none());
    assert!(harness.query_by_label("Cancel").is_some());
    assert!(
        harness
            .query_by_label_contains("Reading and compressing files: 412 of 1284")
            .is_some()
    );
    assert!(
        harness.query_by_label("356.0 GB").is_some(),
        "how much room there is is in the window, as the build itself found it"
    );
    preview(&mut harness, "building-dark");

    harness.state_mut().select_app(0);
    assert_eq!(harness.state().selected_app_index, Some(1));
}

#[test]
fn a_build_needs_a_full_output_path() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        app.output_dir = r"relative\folder".to_owned();
    });
    harness.run_steps(1);
    assert!(!harness.state().output_dir_is_valid());
    assert!(
        harness
            .query_by_label_contains("Enter a full folder path")
            .is_some()
    );
    let button = harness.get_by_label("Create installer");
    assert!(button.accesskit_node().is_disabled());
}

/// What the window does when a person picks an app and clicks Create installer, without the window.
/// Every other test here gives it a scan or a build of its own, which is how the app came to fail
/// every build it was asked for, for a while, with all of its tests passing.
#[test]
fn what_the_app_looks_at_it_can_build() -> Result<(), Box<dyn std::error::Error>> {
    let work = tempfile::tempdir()?;
    let app_folder = work.path().join("WAM Real Build");
    std::fs::create_dir_all(&app_folder)?;
    let program = app_folder.join("wam-real-build.exe");
    std::fs::copy(std::env::current_exe()?, &program)?;
    let stub = work.path().join("stub.bin");
    std::fs::write(&stub, b"a stub")?;
    let mut app = sample_app(
        "WAM Real Build",
        "WAM Test",
        "1.0.0",
        InstallScope::PerMachine,
    );
    app.install_location = Some(app_folder);
    app.main_executable = Some(program);
    let key = crate::ScanKey {
        registry_source_path: String::new(),
        display_name: app.display_name.clone(),
        owner_profile: None,
        source: winappmigrator_core::ScanSource::Live,
        include_user_data: false,
        include_license_data: false,
        user_profile: ProfileChoice::Default,
        near_matches: Vec::new(),
    };

    let report = winappmigrator_core::scan_app(
        &crate::scan_options_of(app.clone(), &key),
        &winappmigrator_core::ScanMonitor::default(),
    )?;
    let installers = work.path().join("installers");
    let (sender, _receiver) = std::sync::mpsc::channel();
    let message = crate::run_build(
        crate::BuildJob {
            app,
            report: std::sync::Arc::new(report),
            choices: winappmigrator_core::FootprintChoices::default(),
            app_name: "WAM Real Build".to_owned(),
            base_name: "WAM Real Build".to_owned(),
            installer_dir: installers.clone(),
            stub: winappmigrator_core::InstallerStub::File(stub),
            include_user_data: false,
            include_license_data: false,
            source: winappmigrator_core::ScanSource::Live,
            user_profile: ProfileChoice::Default,
            ui_theme: "dark".to_owned(),
        },
        &winappmigrator_core::BuildMonitor::default(),
        &sender,
    );

    match message {
        crate::MigrateMessage::Completed(outcome) => {
            let installer = outcome
                .package
                .self_extracting_archive_path
                .ok_or("the build finished with no installer")?;
            assert!(installer.starts_with(&installers));
            assert!(installer.is_file());
            assert!(outcome.installer_bytes > 0);
        }
        crate::MigrateMessage::Failed(why) => panic!("the build failed: {why}"),
        _ => panic!("the build did not finish"),
    }
    assert!(!std::path::Path::new("reports").exists());
    Ok(())
}

#[test]
fn the_first_time_it_takes_the_theme_windows_is_in_and_a_choice_stays() {
    let mut first = harness_with(ThemeMode::Dark, |app| app.follows_windows_theme = true);
    first.input_mut().system_theme = Some(egui::Theme::Light);
    first.run_steps(2);
    assert_eq!(first.state().theme.mode, ThemeMode::Light);
    assert!(!first.state().follows_windows_theme);

    let mut chosen = harness_with(ThemeMode::Dark, |app| {
        app.follows_windows_theme = true;
        app.toggle_theme();
        app.toggle_theme();
    });
    chosen.input_mut().system_theme = Some(egui::Theme::Light);
    chosen.run_steps(2);
    assert_eq!(chosen.state().theme.mode, ThemeMode::Dark);
}

#[test]
fn search_narrows_the_list() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        app.app_search = "tailspin".to_owned();
    });
    assert!(
        harness
            .query_by_label_contains("Tailspin Toolbar")
            .is_some()
    );
    assert!(harness.query_by_label_contains("Contoso Notes").is_none());
    preview(&mut harness, "search-dark");

    harness.state_mut().app_search = "no such app".to_owned();
    harness.run_steps(2);
    assert!(harness.query_by_label("No matching applications").is_some());
    preview(&mut harness, "no-results-dark");
}

#[test]
fn a_clean_build_reads_as_success() {
    let outcome = sample_outcome(Vec::new(), true);
    assert!(outcome_summary(&outcome).0 == Role::Success);

    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |app| {
            select_first_looked_at(app);
            app.migrate_state = OperationState::Completed;
            app.last_outcome = Some(sample_outcome(Vec::new(), true));
        });
        assert!(harness.query_by_label_contains("Installer ready").is_some());
        preview(&mut harness, &format!("done-{name}"));
    }
}

#[test]
fn a_build_too_big_for_one_file_says_its_two_files_must_stay_together() {
    let two_files = || {
        let mut outcome = sample_outcome(Vec::new(), true);
        outcome.package.data_file_path = Some(PathBuf::from(
            r"C:\Users\You\WinAppMigrator\Fabrikam Data Sync\Fabrikam_Data_Sync_installer.wamdata",
        ));
        outcome.installer_bytes = 6 * 1024 * 1024 * 1024;
        outcome
    };
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |app| {
            select_first_looked_at(app);
            app.migrate_state = OperationState::Completed;
            app.last_outcome = Some(two_files());
        });
        assert!(
            harness
                .query_by_label_contains("have to stay together")
                .is_some()
        );
        assert_eq!(
            harness
                .query_all_by_label_contains("Fabrikam_Data_Sync_installer.wamdata")
                .count(),
            2,
            "in what it says, and as a path of its own"
        );
        preview(&mut harness, &format!("done-two-files-{name}"));
    }
    // And a build of one file does not bring it up.
    let harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        app.migrate_state = OperationState::Completed;
        app.last_outcome = Some(sample_outcome(Vec::new(), true));
    });
    assert!(
        harness
            .query_by_label_contains("have to stay together")
            .is_none()
    );
}

#[test]
fn a_partial_build_does_not_read_as_success() {
    let outcome = sample_outcome(vec![copy_failure(), copy_failure()], true);
    let (role, message) = outcome_summary(&outcome);
    assert!(role == Role::Warning);
    assert!(message.contains("2 files"), "{message}");

    let mut harness = harness_with(ThemeMode::Dark, |app| {
        select_first_looked_at(app);
        app.migrate_state = OperationState::Completed;
        app.last_outcome = Some(sample_outcome(vec![copy_failure()], true));
    });
    assert!(
        harness
            .query_by_label_contains("could not be captured")
            .is_some()
    );
    preview(&mut harness, "partial-dark");
}

#[test]
fn files_gone_since_the_app_was_looked_at_are_said_to_be() {
    let gone = Warning::new(
        "file_gone_since_it_was_looked_at",
        WarningSeverity::Warning,
        "This was there when the app was looked at and is gone now.",
        Some(r"C:\Program Files\Fabrikam\engine.dll".to_owned()),
    );
    let outcome = sample_outcome(vec![gone.clone()], true);
    let (role, message) = outcome_summary(&outcome);
    assert!(role == Role::Warning);
    assert!(message.contains("1 file was gone"), "{message}");

    let outcome = sample_outcome(vec![gone, copy_failure()], true);
    let (role, message) = outcome_summary(&outcome);
    assert!(role == Role::Warning);
    assert!(message.contains("2 files"), "{message}");
}

#[test]
fn a_failed_hash_check_says_not_to_use_the_installer() {
    let outcome = sample_outcome(Vec::new(), false);
    let (role, message) = outcome_summary(&outcome);
    assert!(role == Role::Danger);
    assert!(message.contains("Do not use"), "{message}");
}

#[test]
fn failed_and_cancelled_builds_explain_themselves() {
    let mut harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        app.migrate_state = OperationState::Failed;
        app.last_error = Some("I/O failed for D:\\Out: access is denied".to_owned());
    });
    assert!(
        harness
            .query_by_label_contains("access is denied")
            .is_some()
    );
    assert!(harness.query_by_label("Copy error").is_some());
    assert!(
        harness.query_by_label("Log").is_none(),
        "there is no log to point at"
    );
    preview(&mut harness, "failed-dark");

    let harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        app.migrate_state = OperationState::Failed;
        app.last_error = Some("It failed.".to_owned());
        app.log_file = Some(PathBuf::from(
            r"C:\ProgramData\WinAppMigrator\logs\app-1.log",
        ));
    });
    assert!(harness.query_by_label_contains("app-1.log").is_some());

    let harness = harness_with(ThemeMode::Dark, |app| {
        select_first(app);
        app.migrate_state = OperationState::Cancelled;
    });
    assert!(harness.query_by_label_contains("Build cancelled").is_some());
}

#[test]
fn versions_sort_by_number_not_by_text() {
    assert_eq!(compare_versions("9.11.0", "10.2.0"), Ordering::Less);
    assert_eq!(compare_versions("1.7.3", "1.7.3"), Ordering::Equal);
    assert_eq!(compare_versions("2.0", "1.99"), Ordering::Greater);
}
