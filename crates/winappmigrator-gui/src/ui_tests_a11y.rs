//! Every screen of the app as a screen reader, a keyboard, and another language meet it: AC-A11Y-NAMES,
//! AC-A11Y-TARGET, AC-A11Y-KEYBOARD, AC-I18N-NOHARDCODE, and AC-RTL-MIRROR.

use crate::checks;
use egui_kittest::kittest::{NodeT, Queryable};
use winappmigrator_core::BuildStep;
use winappmigrator_i18n::{ENGLISH, Language, PSEUDO_LONG, PSEUDO_RTL, t, with_language};

use crate::theme::ThemeMode;
use crate::ui_tests::{
    copy_failure, harness_with, sample_outcome, sample_room, select_first_looked_at,
};
use crate::{BuildProgress, OperationState, WinAppMigratorApp};

type Setup = fn(&mut WinAppMigratorApp);

fn building(app: &mut WinAppMigratorApp) {
    select_first_looked_at(app);
    app.migrate_state = OperationState::Running;
    app.build_progress = Some(BuildProgress::Build(BuildStep::Packing {
        done: 412,
        total: 1290,
        bytes_done: 90 * 1024 * 1024,
        bytes_total: 410 * 1024 * 1024,
    }));
    app.build_room = Some(sample_room());
}

fn built(app: &mut WinAppMigratorApp) {
    select_first_looked_at(app);
    app.migrate_state = OperationState::Completed;
    app.last_outcome = Some(sample_outcome(vec![copy_failure()], true));
}

fn failed(app: &mut WinAppMigratorApp) {
    select_first_looked_at(app);
    app.migrate_state = OperationState::Failed;
    app.last_error = Some("It failed.".to_owned());
}

/// The screens the app shows, by name.
fn screens() -> [(&'static str, Setup); 5] {
    [
        ("nothing selected", |_| {}),
        ("ready to build", select_first_looked_at),
        ("building", building),
        ("built", built),
        ("failed", failed),
    ]
}

/// What the tests put on screen themselves: names, paths, versions, and words from outside the catalog, such as an
/// English warning a package from an older version keeps.
const DATA: &[&str] = &[
    "Contoso Notes",
    "Contoso Ltd.",
    "Fabrikam Data Sync",
    "Fabrikam Inc.",
    "Northwind Photo Kit with an unusually long product name that cannot fit in its column",
    "Northwind Media",
    "Tailspin Toolbar",
    "Tailspin Toys",
    "Wide World Importer",
    "Wide World Importers",
    "Fabrikam_Data_Sync_installer.exe",
    r"C:\Users\You\WinAppMigrator\Fabrikam Data Sync",
    r"C:\Program Files\Fabrikam Data Sync",
    r"C:\Program Files\Fabrikam\engine.dll",
    "x64",
    "bin",
    "plugins",
    "samples",
    "Fabrikam Data Sync Tools",
    "Fabrikam Updater",
    "Fabrikam-1de4eec8",
    "Fabrikam",
    "Could not copy file into package: the file is in use.",
    "It failed.",
];

fn arabic() -> &'static Language {
    winappmigrator_i18n::language("ar-SA").unwrap_or_else(|| panic!("Arabic is in the table"))
}

/// AC-A11Y-NAMES.
#[test]
fn every_control_has_a_name_a_role_and_its_state() {
    for language in [ENGLISH, arabic()] {
        for (screen, setup) in screens() {
            with_language(language, || {
                let harness = harness_with(ThemeMode::Dark, setup);
                let problems = checks::interactive_problems(&harness.root());
                assert!(
                    problems.is_empty(),
                    "{} {screen}: {problems:#?}",
                    language.tag
                );
            });
        }
    }
}

/// AC-A11Y-TARGET.
#[test]
fn nothing_clickable_is_too_small_to_hit() {
    for (screen, setup) in screens() {
        let harness = harness_with(ThemeMode::Light, setup);
        let problems = checks::small_targets(&harness.root());
        assert!(problems.is_empty(), "{screen}: {problems:#?}");
    }
}

/// AC-I18N-NOHARDCODE.
#[test]
fn every_word_on_every_screen_is_from_the_catalog() {
    for (screen, setup) in screens() {
        with_language(&PSEUDO_LONG, || {
            let harness = harness_with(ThemeMode::Dark, setup);
            let problems = checks::unmarked(&harness.root(), DATA);
            assert!(problems.is_empty(), "{screen}: {problems:#?}");
        });
    }
}

/// AC-A11Y-KEYBOARD: from the window's start, the Tab key reaches the search box, the list, and the button that builds,
/// in that order, and every button on the screen.
#[test]
fn the_tab_key_reaches_every_control_in_reading_order() {
    let mut harness = harness_with(ThemeMode::Dark, select_first_looked_at);
    let order = checks::tab_order(&mut harness, 300);
    let create = t!("app-build-create");
    let position = |name: &str| order.iter().position(|focused| focused == name);
    assert!(position(&create).is_some(), "{create} in {order:#?}");
    let buttons: Vec<String> = checks::all_nodes(&harness.root())
        .iter()
        .filter(|node| {
            node.accesskit_node().role() == eframe::egui::accesskit::Role::Button
                && !node.accesskit_node().is_disabled()
        })
        .map(checks::name_of)
        .collect();
    for button in &buttons {
        assert!(
            order.contains(button),
            "{button} is never reached: {order:#?}"
        );
    }
}

/// AC-RTL-MIRROR: in a right-to-left language the details are at the left, and the list of apps at the right.
#[test]
fn a_right_to_left_window_is_the_mirror_image() {
    let create_x = |language: &'static Language| -> (f32, f32) {
        with_language(language, || {
            let harness = harness_with(ThemeMode::Dark, select_first_looked_at);
            let create = harness.get_by_label(&t!("app-build-create")).rect();
            let width = harness.ctx.content_rect().width();
            (create.center().x, width)
        })
    };
    let (x, width) = create_x(ENGLISH);
    assert!(x > width / 2.0, "{x} of {width}");
    let (x, width) = create_x(&PSEUDO_RTL);
    assert!(x < width / 2.0, "{x} of {width}");
}

/// AC-A11Y-LIVE: how a build is going is said as it changes, and a failure at once.
#[test]
fn progress_and_problems_are_said_as_they_change() {
    let harness = harness_with(ThemeMode::Dark, building);
    let live = checks::live_regions(&harness.root());
    assert!(
        live.iter()
            .any(|(name, urgent)| name.contains("412") && !urgent),
        "{live:#?}"
    );
    let harness = harness_with(ThemeMode::Dark, failed);
    let live = checks::live_regions(&harness.root());
    assert!(live.iter().any(|(_, urgent)| *urgent), "{live:#?}");
}

/// AC-A11Y-TEXT-SIZE: with Windows' Text size at 150% and 225% on a 1024 by 768 screen, everything is larger by that
/// much, which is the window being that much smaller in points, and Create Installer can still be reached.
#[test]
fn larger_text_leaves_every_control_within_reach() {
    for (scale, name) in [(1.5_f32, "150"), (2.25, "225")] {
        let size = eframe::egui::vec2(1024.0 / scale, 697.0 / scale);
        let mut harness =
            crate::ui_tests::harness_sized(ThemeMode::Dark, size, select_first_looked_at);
        let create_label = t!("app-build-create");
        harness.get_by_label(&create_label).scroll_to_me();
        harness.run();
        let button = harness.get_by_label(&create_label).rect();
        let window = eframe::egui::Rect::from_min_size(eframe::egui::Pos2::ZERO, size);
        assert!(
            window.contains_rect(button),
            "{name}%: Create Installer is at {button:?} in {size:?}"
        );
    }
}

/// AC-A11Y-CONTRAST-THEME: in a Windows contrast theme the window is drawn in its colors, and does not offer to change
/// to the app's own light or dark ones.
#[test]
fn a_contrast_theme_is_kept_and_not_offered_away() {
    use eframe::egui::Color32;
    use winappmigrator_ui::system_settings::{ContrastColors, SystemSettings};
    use winappmigrator_ui::window::Opening;

    let night_sky = ContrastColors {
        window: Color32::from_rgb(0x00, 0x00, 0x00),
        text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        highlight: Color32::from_rgb(0xD6, 0xB4, 0xFD),
        highlight_text: Color32::from_rgb(0x2B, 0x2B, 0x2B),
        button_face: Color32::from_rgb(0x00, 0x00, 0x00),
        button_text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        disabled_text: Color32::from_rgb(0xA6, 0xA6, 0xA6),
        link: Color32::from_rgb(0x80, 0x80, 0xFF),
    };
    let settings = SystemSettings {
        contrast: Some(night_sky),
        ..SystemSettings::default()
    };
    let harness = harness_with(ThemeMode::Dark, |app| {
        let (opening, theme) = Opening::with_settings(crate::WINDOW_SIZES, settings);
        app.opening = opening;
        app.theme = theme;
    });
    assert_eq!(harness.state().theme.bg, night_sky.window);
    assert_eq!(harness.state().theme.text, night_sky.text);
    for offer in [t!("app-theme-to-light"), t!("app-theme-to-dark")] {
        assert!(harness.query_by_label(&offer).is_none(), "{offer}");
    }
    let harness = harness_with(ThemeMode::Dark, |_| {});
    assert!(harness.query_by_label(&t!("app-theme-to-light")).is_some());
}

/// WCAG 2.4.11: on a small screen with large text, the Tab key never gives the focus to something that cannot be seen.
#[test]
fn what_has_the_focus_can_be_seen() {
    let size = eframe::egui::vec2(1024.0 / 1.5, 697.0 / 1.5);
    let mut harness = crate::ui_tests::harness_sized(ThemeMode::Dark, size, select_first_looked_at);
    let window = eframe::egui::Rect::from_min_size(eframe::egui::Pos2::ZERO, size);
    let hidden = checks::hidden_when_focused(&mut harness, window, 300);
    assert!(hidden.is_empty(), "{hidden:#?}");
}
