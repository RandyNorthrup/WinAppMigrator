//! How a window of these programs opens, the same for all of them: it fits the screen it is on, and
//! it is in the mode a person chose in the app, or else the one Windows is in.
//!
//! Neither can be known before there is a window, since that is who egui asks, and a window is only
//! shown once its first frame is painted. So the first frame finds out, and what that changes is done
//! before anything else can be drawn.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use eframe::egui::{self, Rect, Vec2, ViewportCommand, vec2};

use crate::theme::{Theme, ThemeMode};
use crate::widgets;

/// What a window's title bar and borders add to what is inside it, in points.
const WINDOW_FRAME: Vec2 = vec2(16.0, 39.0);
/// How much of a screen to leave for the taskbar, which can be on any edge. Windows 11's is this tall.
const TASKBAR: f32 = 48.0;
const SETTINGS_DIR: &str = "WinAppMigrator";
const MODE_FILE: &str = "theme.txt";
/// More than either word it can hold.
const MOST_MODE_FILE_BYTES: u64 = 16;

fn file_of_the_chosen_mode() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from)?;
    Some(base.join(SETTINGS_DIR).join(MODE_FILE))
}

/// The mode a person chose in the app, if they have, on this account.
fn chosen_mode() -> Option<ThemeMode> {
    mode_in(&file_of_the_chosen_mode()?)
}

/// These programs run elevated, and this file is in a folder the account they were started from can
/// change. So it is only ever one of two words, and only if it is really a file there, as something
/// else could be made to keep a window from opening.
fn mode_in(path: &Path) -> Option<ThemeMode> {
    if !fs::symlink_metadata(path).ok()?.is_file() {
        return None;
    }
    let mut tag = String::new();
    File::open(path)
        .ok()?
        .take(MOST_MODE_FILE_BYTES)
        .read_to_string(&mut tag)
        .ok()?;
    ThemeMode::from_tag(tag.trim())
}

/// Remember the mode a person chose. Losing this is harmless, so a failure to write it is not worth
/// interrupting them for.
pub fn remember_chosen_mode(mode: ThemeMode) {
    if let Some(path) = file_of_the_chosen_mode() {
        write_mode(&path, mode);
    }
}

/// With the app elevated, writing to what is already there would write through a link to it, to any
/// file an administrator can change. A new file put in its place replaces the name, and leaves alone
/// whatever else that name led to. And a link in the way of the folder is not written through at all.
fn write_mode(path: &Path, mode: ThemeMode) {
    let Some(folder) = path.parent() else {
        return;
    };
    let _ = fs::create_dir_all(folder);
    let is_a_link = |place: &Path| {
        fs::symlink_metadata(place).is_ok_and(|metadata| metadata.file_type().is_symlink())
    };
    if folder.ancestors().any(is_a_link) {
        return;
    }
    let beside = folder.join(format!("{MODE_FILE}.{}", std::process::id()));
    // Not over anything already there, which is not to be written through either.
    let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&beside)
    else {
        return;
    };
    let written = file.write_all(mode.as_tag().as_bytes());
    drop(file);
    if written.is_err() || fs::rename(&beside, path).is_err() {
        let _ = fs::remove_file(&beside);
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
}

impl Opening {
    /// How a window opens on this account, with the theme to start it in.
    pub fn on_this_account(sizes: WindowSizes) -> (Self, Theme) {
        match chosen_mode() {
            Some(mode) => (Self::in_a_chosen_mode(sizes), Theme::new(mode)),
            None => (Self::new(sizes), Theme::new(ThemeMode::Dark)),
        }
    }

    /// A window in the mode Windows is in, until a person chooses.
    pub fn new(sizes: WindowSizes) -> Self {
        Self {
            sizes,
            fitted: false,
            follows_windows: true,
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
        self.take_up_the_mode_of_windows(ctx, theme);
        self.fit_the_screen(ctx);
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
        assert!(fs::remove_file(&file).is_ok());
        assert!(fs::create_dir(&file).is_ok());
        assert_eq!(mode_in(&file), None, "a folder of that name");
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
        let scratch = place.0.join(format!("{MODE_FILE}.{}", std::process::id()));
        assert!(fs::write(&scratch, "someone else's").is_ok());
        write_mode(&file, ThemeMode::Light);
        assert_eq!(
            fs::read_to_string(&scratch).ok().as_deref(),
            Some("someone else's")
        );
        assert_eq!(mode_in(&file), None);
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
