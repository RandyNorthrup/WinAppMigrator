//! How a window of these programs opens, the same for all of them: it fits the screen it is on, and
//! it is in the mode a person chose in the app, or else the one Windows is in.
//!
//! Neither can be known before there is a window, since that is who egui asks, and a window is only
//! shown once its first frame is painted. So the first frame finds out, and what that changes is done
//! before anything else can be drawn.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use eframe::egui::{self, Rect, Vec2, ViewportCommand, vec2};

use crate::system_settings::{self, SystemSettings};
use crate::theme::{Theme, ThemeMode};
use crate::widgets;

/// What a window's title bar and borders add to what is inside it, in points.
const WINDOW_FRAME: Vec2 = vec2(16.0, 39.0);
/// How much of a screen to leave for the taskbar, which can be on any edge. Windows 11's is this tall.
const TASKBAR: f32 = 48.0;
const SETTINGS_DIR: &str = "WinAppMigrator";
const MODE_FILE: &str = "theme.txt";
const LANGUAGE_FILE: &str = "language.txt";
const OUTPUT_FOLDER_FILE: &str = "output-folder.txt";
/// More than any word or language tag a setting holds.
const MOST_SETTING_BYTES: u64 = 16;
/// More than any folder a person picks, in UTF-8. Windows' own limit for a path is 32,767 UTF-16
/// characters, but nobody types that, and a folder past this is just not remembered.
const MOST_FOLDER_BYTES: u64 = 4096;

fn settings_file(name: &str) -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from)?;
    Some(base.join(SETTINGS_DIR).join(name))
}

/// The mode a person chose in the app, if they have, on this account.
fn chosen_mode() -> Option<ThemeMode> {
    mode_in(&settings_file(MODE_FILE)?)
}

/// The language a person chose in the app, if they have, on this account, as its tag. Whether it still ships is for
/// the caller to ask.
pub fn chosen_language() -> Option<String> {
    setting_in(&settings_file(LANGUAGE_FILE)?)
}

/// The folder a person last chose for installers, on this account, if they chose one of their own.
/// Only a full path, with nothing in it that could not be in a name, is taken for one: it is only
/// ever put in the Output Folder field, for them to see before anything is written there.
pub fn chosen_output_folder() -> Option<PathBuf> {
    output_folder_in(&settings_file(OUTPUT_FOLDER_FILE)?)
}

fn output_folder_in(path: &Path) -> Option<PathBuf> {
    let text = setting_of_at_most(path, MOST_FOLDER_BYTES)?;
    let folder = PathBuf::from(&text);
    (folder.is_absolute() && !text.chars().any(char::is_control)).then_some(folder)
}

/// Remember the folder a person chose for installers, the same way as the mode. `None` forgets it,
/// so each app gets its own folder again.
pub fn remember_output_folder(folder: Option<&Path>) {
    if let Some(path) = settings_file(OUTPUT_FOLDER_FILE) {
        remember_output_folder_in(&path, folder);
    }
}

fn remember_output_folder_in(path: &Path, folder: Option<&Path>) {
    match folder.and_then(Path::to_str) {
        Some(folder) => write_setting(path, folder),
        // Only if it is a file. Through a link, nothing is removed but the link.
        None if fs::symlink_metadata(path).is_ok_and(|metadata| !metadata.is_dir()) => {
            let _ = fs::remove_file(path);
        }
        None => {}
    }
}

fn mode_in(path: &Path) -> Option<ThemeMode> {
    ThemeMode::from_tag(&setting_in(path)?)
}

/// These programs run elevated, and a setting is in a folder the account they were started from can
/// change. So it is only ever a short word, and only if it is really a file there, as something
/// else could be made to keep a window from opening.
fn setting_in(path: &Path) -> Option<String> {
    setting_of_at_most(path, MOST_SETTING_BYTES)
}

fn setting_of_at_most(path: &Path, most: u64) -> Option<String> {
    if !fs::symlink_metadata(path).ok()?.is_file() {
        return None;
    }
    let mut text = String::new();
    File::open(path)
        .ok()?
        .take(most + 1)
        .read_to_string(&mut text)
        .ok()?;
    if text.len() as u64 > most {
        return None;
    }
    Some(text.trim().to_owned())
}

/// Remember the mode a person chose. Losing this is harmless, so a failure to write it is not worth
/// interrupting them for.
pub fn remember_chosen_mode(mode: ThemeMode) {
    if let Some(path) = settings_file(MODE_FILE) {
        write_mode(&path, mode);
    }
}

/// Remember the language a person chose, the same way.
pub fn remember_chosen_language(tag: &str) {
    if let Some(path) = settings_file(LANGUAGE_FILE) {
        write_setting(&path, tag);
    }
}

fn write_mode(path: &Path, mode: ThemeMode) {
    write_setting(path, mode.as_tag());
}

/// With the app elevated, writing to what is already there would write through a link to it, to any
/// file an administrator can change. A new file put in its place replaces the name, and leaves alone
/// whatever else that name led to. And a link in the way of the folder is not written through at all.
fn write_setting(path: &Path, text: &str) {
    static ATTEMPTS: AtomicU32 = AtomicU32::new(0);
    let Some(folder) = path.parent() else {
        return;
    };
    let is_a_link = |place: &Path| {
        fs::symlink_metadata(place).is_ok_and(|metadata| metadata.file_type().is_symlink())
    };
    // Before making the folder as well, which would make it wherever a link leads.
    if folder.ancestors().any(is_a_link) || fs::create_dir_all(folder).is_err() {
        return;
    }
    let attempt = ATTEMPTS.fetch_add(1, Ordering::Relaxed);
    let Some(file_name) = path.file_name() else {
        return;
    };
    let name = format!(
        "{}.{}.{attempt}",
        file_name.to_string_lossy(),
        std::process::id()
    );
    put_in_place(path, text, &folder.join(name));
}

#[cfg(test)]
fn put_mode_in_place(path: &Path, mode: ThemeMode, beside: &Path) {
    put_in_place(path, mode.as_tag(), beside);
}

fn put_in_place(path: &Path, text: &str, beside: &Path) {
    // Not over anything already there, which is not to be written through either.
    let Ok(mut file) = OpenOptions::new().write(true).create_new(true).open(beside) else {
        return;
    };
    let written = file.write_all(text.as_bytes());
    drop(file);
    if written.is_err() || fs::rename(beside, path).is_err() {
        let _ = fs::remove_file(beside);
    }
}

/// How big a window would like to be, in points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowSizes {
    pub opens_at: Vec2,
    pub at_least: Vec2,
}

impl WindowSizes {
    /// What to open a window of these sizes with.
    pub fn viewport(self, app_id: &str) -> egui::ViewportBuilder {
        let viewport = egui::ViewportBuilder::default()
            .with_inner_size(self.opens_at)
            .with_min_inner_size(self.at_least)
            .with_app_id(app_id);
        match widgets::window_icon() {
            Some(icon) => viewport.with_icon(icon),
            None => viewport,
        }
    }

    /// What these come to on a screen they do not fit, if they do not. Windows chooses where a window
    /// goes, so it can fit and still be put where the bottom of it is behind the taskbar. Where a screen
    /// other than the main one starts cannot be told from here, so that only goes by a window that
    /// starts inside what would be the main one, and being wrong about it only means it is maximized.
    fn on_a_screen_of(self, screen: Vec2, placed_at: Option<Rect>) -> Option<Self> {
        let room = (screen - WINDOW_FRAME - vec2(TASKBAR, TASKBAR)).max(Vec2::splat(1.0));
        let too_big = self.opens_at.x > room.x || self.opens_at.y > room.y;
        let placed_off_it = placed_at.is_some_and(|placed| {
            let starts_on_it = placed.min.x >= 0.0
                && placed.min.y >= 0.0
                && placed.min.x < screen.x
                && placed.min.y < screen.y;
            starts_on_it && (placed.max.x > screen.x || placed.max.y > screen.y - TASKBAR)
        });
        (too_big || placed_off_it).then(|| Self {
            opens_at: self.opens_at.min(room),
            at_least: self.at_least.min(room),
        })
    }
}

/// What is left to do of opening a window.
#[derive(Debug)]
pub struct Opening {
    sizes: WindowSizes,
    fitted: bool,
    follows_windows: bool,
    /// What a person chose in Windows' accessibility settings, which every window follows.
    settings: SystemSettings,
    settings_applied: bool,
}

impl Opening {
    /// How a window opens on this account, with the theme to start it in.
    pub fn on_this_account(sizes: WindowSizes) -> (Self, Theme) {
        Self::with_settings(sizes, SystemSettings::read())
    }

    /// How a window opens with these accessibility settings. A contrast theme is drawn in its own colors, whatever
    /// mode was chosen in the app.
    pub fn with_settings(sizes: WindowSizes, settings: SystemSettings) -> (Self, Theme) {
        let (opening, theme) = match (settings.contrast, chosen_mode()) {
            (Some(colors), _) => (Self::in_a_chosen_mode(sizes), Theme::contrast(colors)),
            (None, Some(mode)) => (Self::in_a_chosen_mode(sizes), Theme::new(mode)),
            (None, None) => (Self::new(sizes), Theme::new(ThemeMode::Dark)),
        };
        (
            Self {
                settings,
                ..opening
            },
            theme,
        )
    }

    /// Whether the window is in a Windows contrast theme, in which a person's light or dark choice does not apply.
    pub fn in_contrast_theme(&self) -> bool {
        self.settings.contrast.is_some()
    }

    /// A window in the mode Windows is in, until a person chooses.
    pub fn new(sizes: WindowSizes) -> Self {
        Self {
            sizes,
            fitted: false,
            follows_windows: true,
            settings: SystemSettings::default(),
            settings_applied: false,
        }
    }

    /// A window in a mode a person chose earlier.
    pub fn in_a_chosen_mode(sizes: WindowSizes) -> Self {
        Self {
            follows_windows: false,
            ..Self::new(sizes)
        }
    }

    pub fn person_chose_a_mode(&mut self) {
        self.follows_windows = false;
    }

    /// Call at the start of each frame, before anything is drawn.
    pub fn settle(&mut self, ctx: &egui::Context, theme: &mut Theme) {
        self.apply_the_settings(ctx);
        self.take_up_the_mode_of_windows(ctx, theme);
        self.fit_the_screen(ctx);
    }

    /// Text size makes everything larger by its factor, as zooming does, so nothing outgrows what holds it, and what no
    /// longer fits scrolls. With Animation effects off, nothing moves on its own.
    fn apply_the_settings(&mut self, ctx: &egui::Context) {
        if self.settings_applied {
            return;
        }
        self.settings_applied = true;
        if self.settings.text_scale > 1.0 {
            ctx.set_zoom_factor(self.settings.text_scale);
        }
        system_settings::allow_animations(ctx, self.settings.animations);
    }

    fn take_up_the_mode_of_windows(&mut self, ctx: &egui::Context, theme: &mut Theme) {
        if !self.follows_windows {
            return;
        }
        let Some(system) = ctx.system_theme() else {
            return;
        };
        self.follows_windows = false;
        let mode = match system {
            egui::Theme::Dark => ThemeMode::Dark,
            egui::Theme::Light => ThemeMode::Light,
        };
        if mode != theme.mode {
            *theme = Theme::new(mode);
            theme.apply(ctx);
        }
    }

    /// Windows knows exactly what of a screen a window can use, wherever the taskbar is and however big,
    /// and nothing here can ask without code the rest of these programs are kept free of. Maximizing is
    /// asking, so a window that would not fit does that, and comes back to a size that does.
    fn fit_the_screen(&mut self, ctx: &egui::Context) {
        if self.fitted {
            return;
        }
        let (screen, placed_at) = ctx.input(|input| {
            let viewport = input.viewport();
            (viewport.monitor_size, viewport.outer_rect)
        });
        let Some(screen) = screen else {
            return;
        };
        self.fitted = true;
        let Some(fitting) = self.sizes.on_a_screen_of(screen, placed_at) else {
            return;
        };
        hide_fit_and_show(fitting)
            .into_iter()
            .for_each(|command| ctx.send_viewport_cmd(command));
    }
}

/// In this order, since eframe shows a window just before it does what is asked of it in the first
/// frame. Hidden again straight after being shown, there is as little to see of it before its final
/// place as can be had without knowing the screen before there is a window. Nothing has measured how
/// little that is, on a real screen.
fn hide_fit_and_show(fitting: WindowSizes) -> [ViewportCommand; 5] {
    [
        ViewportCommand::Visible(false),
        ViewportCommand::MinInnerSize(fitting.at_least),
        ViewportCommand::InnerSize(fitting.opens_at),
        ViewportCommand::Maximized(true),
        ViewportCommand::Visible(true),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::{Pos2, RawInput, ViewportId, pos2};

    const APP: WindowSizes = WindowSizes {
        opens_at: vec2(1240.0, 780.0),
        at_least: vec2(1000.0, 640.0),
    };
    const INSTALLER: WindowSizes = WindowSizes {
        opens_at: vec2(760.0, 720.0),
        at_least: vec2(620.0, 520.0),
    };
    const UNINSTALLER: WindowSizes = WindowSizes {
        opens_at: vec2(620.0, 560.0),
        at_least: vec2(520.0, 420.0),
    };

    fn placed(at: Pos2, sizes: WindowSizes) -> Option<Rect> {
        Some(Rect::from_min_size(at, sizes.opens_at + WINDOW_FRAME))
    }

    #[test]
    fn a_window_that_fits_a_big_screen_is_left_where_windows_put_it() {
        let screen = vec2(1920.0, 1080.0);
        for sizes in [APP, INSTALLER, UNINSTALLER] {
            assert_eq!(
                sizes.on_a_screen_of(screen, placed(pos2(26.0, 26.0), sizes)),
                None
            );
        }
    }

    #[test]
    fn on_the_small_screens_people_have_a_window_comes_to_what_there_is() {
        // 1024x768, 1366x768, the same at 125% and 150%, and 1280x720 at 150%.
        for screen in [
            vec2(1024.0, 768.0),
            vec2(1366.0, 768.0),
            vec2(1092.8, 614.4),
            vec2(910.7, 512.0),
            vec2(853.3, 480.0),
        ] {
            let room = screen - WINDOW_FRAME - vec2(TASKBAR, TASKBAR);
            let Some(fitting) = APP.on_a_screen_of(screen, None) else {
                panic!("the app does not fit {screen:?}");
            };
            assert!(fitting.opens_at.x <= room.x && fitting.opens_at.y <= room.y);
            assert!(fitting.at_least.x <= room.x && fitting.at_least.y <= room.y);
        }
    }

    #[test]
    fn an_installer_does_not_fit_1024_by_768_and_an_uninstaller_does() {
        let screen = vec2(1024.0, 768.0);
        let fitting = INSTALLER.on_a_screen_of(screen, placed(pos2(0.0, 0.0), INSTALLER));
        assert_eq!(
            fitting.map(|fitting| fitting.opens_at),
            Some(vec2(760.0, 681.0))
        );
        assert_eq!(
            UNINSTALLER.on_a_screen_of(screen, placed(pos2(26.0, 26.0), UNINSTALLER)),
            None
        );
    }

    #[test]
    fn a_window_that_fits_can_still_be_put_with_its_bottom_behind_the_taskbar() {
        let screen = vec2(1920.0, 1000.0);
        assert_eq!(
            APP.on_a_screen_of(screen, placed(pos2(26.0, 26.0), APP)),
            None
        );
        assert!(
            APP.on_a_screen_of(screen, placed(pos2(26.0, 200.0), APP))
                .is_some()
        );
        assert!(
            APP.on_a_screen_of(screen, placed(pos2(800.0, 26.0), APP))
                .is_some()
        );
    }

    #[test]
    fn a_window_on_another_screen_is_not_taken_for_one_hanging_off_the_main_one() {
        let screen = vec2(1920.0, 1080.0);
        for at in [
            pos2(1950.0, 26.0),
            pos2(-1900.0, 26.0),
            pos2(26.0, -1050.0),
            pos2(26.0, 1100.0),
        ] {
            assert_eq!(APP.on_a_screen_of(screen, placed(at, APP)), None, "{at:?}");
        }
    }

    /// 1280x720 at 300% and 400%, where what is left is less than any of them would ask for.
    #[test]
    fn the_smallest_a_window_can_be_made_is_never_more_than_a_screen_has() {
        for screen in [vec2(426.7, 240.0), vec2(320.0, 180.0)] {
            let room = screen - WINDOW_FRAME - vec2(TASKBAR, TASKBAR);
            for sizes in [APP, INSTALLER, UNINSTALLER] {
                let Some(fitting) = sizes.on_a_screen_of(screen, None) else {
                    panic!("none of them fit");
                };
                assert!(fitting.at_least.x <= room.x && fitting.at_least.y <= room.y);
                assert!(fitting.at_least.x <= fitting.opens_at.x);
                assert!(fitting.at_least.y <= fitting.opens_at.y);
            }
        }
    }

    #[test]
    fn a_screen_with_no_room_at_all_still_asks_for_a_size_there_can_be() {
        let fitting = APP.on_a_screen_of(vec2(50.0, 50.0), None);
        assert_eq!(
            fitting.map(|fitting| (fitting.opens_at, fitting.at_least)),
            Some((Vec2::splat(1.0), Vec2::splat(1.0)))
        );
    }

    /// A folder of its own, gone at the end of the test.
    struct Place(PathBuf);

    impl Place {
        fn new(name: &str) -> Self {
            let folder =
                std::env::temp_dir().join(format!("wam-window-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&folder);
            assert!(fs::create_dir_all(&folder).is_ok());
            Self(folder)
        }
    }

    impl Drop for Place {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_mode_that_was_chosen_is_read_back_and_nothing_else_is_taken_for_one() {
        let place = Place::new("read");
        let file = place.0.join(MODE_FILE);
        assert_eq!(mode_in(&file), None, "nothing is there");
        for (mode, text) in [(ThemeMode::Light, "light"), (ThemeMode::Dark, "dark")] {
            write_mode(&file, mode);
            assert_eq!(fs::read_to_string(&file).ok().as_deref(), Some(text));
            assert_eq!(mode_in(&file), Some(mode));
        }
        assert!(fs::write(&file, " light\r\n").is_ok());
        assert_eq!(mode_in(&file), Some(ThemeMode::Light));
        for text in ["", "nonsense", "lightly"] {
            assert!(fs::write(&file, text).is_ok());
            assert_eq!(mode_in(&file), None, "{text}");
        }
        // However much of it there is, only as much as could be a word is read.
        assert!(fs::write(&file, "light".repeat(1000)).is_ok());
        assert_eq!(mode_in(&file), None);
        // And what starts with a word and goes on past that is not that word.
        assert!(fs::write(&file, format!("light{}x", " ".repeat(20))).is_ok());
        assert_eq!(mode_in(&file), None);
        assert!(fs::remove_file(&file).is_ok());
        assert!(fs::create_dir(&file).is_ok());
        assert_eq!(mode_in(&file), None, "a folder of that name");
    }

    #[test]
    fn a_language_that_was_chosen_is_read_back_as_its_tag() {
        let place = Place::new("language");
        let file = place.0.join(LANGUAGE_FILE);
        assert_eq!(setting_in(&file), None, "nothing is there");
        write_setting(&file, "it-IT");
        assert_eq!(fs::read_to_string(&file).ok().as_deref(), Some("it-IT"));
        assert_eq!(setting_in(&file).as_deref(), Some("it-IT"));
        assert!(fs::write(&file, "it-IT".repeat(100)).is_ok());
        assert_eq!(setting_in(&file), None, "too long to be a tag");
    }

    #[test]
    fn a_chosen_output_folder_is_read_back_and_forgotten_and_nothing_else_is_taken_for_one() {
        let place = Place::new("output-folder");
        let file = place.0.join(OUTPUT_FOLDER_FILE);
        assert_eq!(output_folder_in(&file), None, "nothing is there");
        let folder = Path::new(r"D:\Installers\Über 日本");
        remember_output_folder_in(&file, Some(folder));
        assert_eq!(output_folder_in(&file).as_deref(), Some(folder));
        remember_output_folder_in(&file, None);
        assert!(!file.exists(), "forgotten");
        assert_eq!(output_folder_in(&file), None);
        remember_output_folder_in(&file, None);
        let with_a_bell = format!(r"D:\a{}b", '\u{7}');
        for text in [r"relative\folder", "", with_a_bell.as_str()] {
            assert!(fs::write(&file, text).is_ok());
            assert_eq!(output_folder_in(&file), None, "{text:?}");
        }
        assert!(fs::write(&file, format!(r"D:\{}", "a".repeat(5000))).is_ok());
        assert_eq!(output_folder_in(&file), None, "longer than anyone picks");
        assert!(fs::write(&file, format!("{}\r\n", r"E:\Out")).is_ok());
        assert_eq!(
            output_folder_in(&file).as_deref(),
            Some(Path::new(r"E:\Out"))
        );
    }

    #[test]
    fn forgetting_an_output_folder_removes_no_folder_of_that_name() {
        let place = Place::new("output-folder-dir");
        let file = place.0.join(OUTPUT_FOLDER_FILE);
        assert!(fs::create_dir(&file).is_ok());
        remember_output_folder_in(&file, None);
        assert!(file.is_dir());
    }

    #[test]
    fn choosing_a_mode_does_not_write_through_a_link_to_another_file() {
        let place = Place::new("hard-link");
        let protected = place.0.join("protected.txt");
        let file = place.0.join(MODE_FILE);
        assert!(fs::write(&protected, "keep this").is_ok());
        assert!(fs::hard_link(&protected, &file).is_ok());
        write_mode(&file, ThemeMode::Light);
        assert_eq!(
            fs::read_to_string(&protected).ok().as_deref(),
            Some("keep this")
        );
        assert_eq!(mode_in(&file), Some(ThemeMode::Light));
    }

    #[test]
    fn choosing_a_mode_writes_nothing_through_a_link_in_the_way_of_its_folder() {
        let place = Place::new("junction");
        let protected = place.0.join("protected");
        let linked = place.0.join("settings");
        assert!(fs::create_dir(&protected).is_ok());
        let made = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&linked)
            .arg(&protected)
            .output();
        assert!(made.is_ok_and(|output| output.status.success()));
        write_mode(&linked.join(MODE_FILE), ThemeMode::Light);
        assert_eq!(fs::read_dir(&protected).map(Iterator::count).ok(), Some(0));
    }

    #[test]
    fn choosing_a_mode_goes_no_further_if_its_scratch_file_is_there_already() {
        let place = Place::new("scratch");
        let file = place.0.join(MODE_FILE);
        let scratch = place.0.join("scratch");
        assert!(fs::write(&scratch, "someone else's").is_ok());
        put_mode_in_place(&file, ThemeMode::Light, &scratch);
        assert_eq!(
            fs::read_to_string(&scratch).ok().as_deref(),
            Some("someone else's")
        );
        assert_eq!(mode_in(&file), None);
        // The next time it goes by another name.
        write_mode(&file, ThemeMode::Light);
        assert_eq!(mode_in(&file), Some(ThemeMode::Light));
        let left: Vec<_> = fs::read_dir(&place.0)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.file_name())
            .collect();
        assert_eq!(left.len(), 2, "nothing of its own is left behind: {left:?}");
    }

    fn input_with(screen: Option<Vec2>, system: Option<egui::Theme>) -> RawInput {
        let mut input = RawInput {
            system_theme: system,
            ..RawInput::default()
        };
        input
            .viewports
            .entry(ViewportId::ROOT)
            .or_default()
            .monitor_size = screen;
        input
    }

    /// One frame of a window, and what it asked of the window about fitting.
    fn frame(
        ctx: &egui::Context,
        opening: &mut Opening,
        theme: &mut Theme,
        input: RawInput,
    ) -> Vec<ViewportCommand> {
        let mut output = ctx.run_ui(input, |ui| opening.settle(ui.ctx(), theme));
        output.textures_delta.clear();
        output.viewport_output[&ViewportId::ROOT]
            .commands
            .iter()
            .filter(|command| {
                matches!(
                    command,
                    ViewportCommand::Visible(_)
                        | ViewportCommand::MinInnerSize(_)
                        | ViewportCommand::InnerSize(_)
                        | ViewportCommand::Maximized(_)
                )
            })
            .cloned()
            .collect()
    }

    #[test]
    fn it_hides_what_it_changes_and_only_does_it_once() {
        let ctx = egui::Context::default();
        let mut opening = Opening::new(APP);
        let mut theme = Theme::new(ThemeMode::Dark);
        let small = Some(vec2(1024.0, 768.0));

        assert!(
            frame(&ctx, &mut opening, &mut theme, input_with(None, None)).is_empty(),
            "with no screen known yet, it waits"
        );
        let commands = frame(&ctx, &mut opening, &mut theme, input_with(small, None));
        assert_eq!(commands.first(), Some(&ViewportCommand::Visible(false)));
        assert_eq!(commands.last(), Some(&ViewportCommand::Visible(true)));
        assert!(commands.contains(&ViewportCommand::Maximized(true)));
        assert!(frame(&ctx, &mut opening, &mut theme, input_with(small, None)).is_empty());
    }

    #[test]
    fn a_window_on_a_big_screen_asks_nothing_of_it() {
        let ctx = egui::Context::default();
        let mut opening = Opening::new(APP);
        let mut theme = Theme::new(ThemeMode::Dark);
        let big = Some(vec2(1920.0, 1080.0));
        assert!(frame(&ctx, &mut opening, &mut theme, input_with(big, None)).is_empty());
    }

    fn mode_after(mut opening: Opening, start: ThemeMode, system: egui::Theme) -> ThemeMode {
        let ctx = egui::Context::default();
        let mut theme = Theme::new(start);
        // Windows' mode is only known once there is a window, and it changes nothing until then.
        frame(&ctx, &mut opening, &mut theme, input_with(None, None));
        assert_eq!(theme.mode, start);
        frame(
            &ctx,
            &mut opening,
            &mut theme,
            input_with(None, Some(system)),
        );
        theme.mode
    }

    #[test]
    fn a_window_is_in_the_mode_windows_is_in() {
        for (system, mode) in [
            (egui::Theme::Light, ThemeMode::Light),
            (egui::Theme::Dark, ThemeMode::Dark),
        ] {
            for start in [ThemeMode::Dark, ThemeMode::Light] {
                assert_eq!(mode_after(Opening::new(INSTALLER), start, system), mode);
                assert_eq!(mode_after(Opening::new(UNINSTALLER), start, system), mode);
                assert_eq!(mode_after(Opening::new(APP), start, system), mode);
            }
        }
    }

    #[test]
    fn what_a_person_chose_goes_ahead_of_what_windows_is_in() {
        assert_eq!(
            mode_after(
                Opening::in_a_chosen_mode(APP),
                ThemeMode::Light,
                egui::Theme::Dark
            ),
            ThemeMode::Light
        );
        let mut chose = Opening::new(APP);
        chose.person_chose_a_mode();
        assert_eq!(
            mode_after(chose, ThemeMode::Dark, egui::Theme::Light),
            ThemeMode::Dark
        );
    }

    #[test]
    fn once_it_has_taken_up_a_mode_a_change_of_windows_does_not_move_it() {
        let ctx = egui::Context::default();
        let mut opening = Opening::new(INSTALLER);
        let mut theme = Theme::new(ThemeMode::Dark);
        for system in [egui::Theme::Light, egui::Theme::Dark] {
            frame(
                &ctx,
                &mut opening,
                &mut theme,
                input_with(None, Some(system)),
            );
        }
        assert_eq!(theme.mode, ThemeMode::Light);
    }
}
