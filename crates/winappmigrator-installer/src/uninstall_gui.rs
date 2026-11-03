//! The small window an uninstall opens: what will be removed, how it is going, and what came
//! of it.

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, Margin, RichText};

use winappmigrator_core::journal::InstallSummary;
use winappmigrator_core::logging;
use winappmigrator_core::path_utils::system_root;
use winappmigrator_core::rollback::{OpenInstall, UninstallOutcome, open_install};
use winappmigrator_core::{InstallMonitor, InstallStep};
use winappmigrator_ui::theme::{
    self, Role, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS, Theme, ThemeMode,
};
use winappmigrator_ui::widgets::{self, ButtonKind};
use winappmigrator_ui::window::{Opening, WindowSizes};

use crate::uninstall::UninstallCli;

const WINDOW_TITLE: &str = "Uninstall";
const WINDOW_SIZES: WindowSizes = WindowSizes {
    opens_at: egui::vec2(620.0, 560.0),
    at_least: egui::vec2(520.0, 420.0),
};
/// How often to redraw while uninstalling, enough for smooth animation.
const BUSY_REPAINT_INTERVAL: Duration = Duration::from_millis(50);
/// How many things left in place are listed, before saying how many more there are.
const MAX_LISTED_ITEMS: usize = 30;

pub fn run(cli: UninstallCli) -> eframe::Result {
    let finished: Arc<Mutex<Option<UninstallOutcome>>> = Arc::new(Mutex::new(None));
    let for_window = Arc::clone(&finished);
    let result = eframe::run_native(
        WINDOW_TITLE,
        winappmigrator_ui::window_options(WINDOW_SIZES.viewport("winappmigrator-uninstaller")),
        Box::new(move |cc| {
            theme::install_fonts(&cc.egui_ctx, &system_root().join("Fonts"));
            let mut app = UninstallApp::new(cli.id.clone(), cli.remove_user_data, for_window);
            (app.opening, app.theme) = Opening::on_this_account(WINDOW_SIZES);
            app.theme.apply(&cc.egui_ctx);
            app.start_load(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    // Only once the window has closed is this program ready to go.
    if let Ok(mut finished) = finished.lock()
        && let Some(outcome) = finished.take()
    {
        crate::uninstall::finish(&outcome);
    }
    result
}

enum Stage {
    /// The record of the install is being checked and read.
    Opening,
    /// It could not be, with a reason.
    Failed(String),
    Ready,
    Running,
    Done(Box<UninstallOutcome>),
}

enum Msg {
    Opened(Box<OpenInstall>, Box<InstallSummary>),
    OpenFailed(String),
    Progress(InstallStep),
    Done(Box<UninstallOutcome>),
}

struct UninstallApp {
    opening: Opening,
    theme: Theme,
    id: String,
    stage: Stage,
    opened: Option<OpenInstall>,
    summary: InstallSummary,
    remove_user_data: bool,
    step: Option<InstallStep>,
    started: Option<Instant>,
    receiver: Option<Receiver<Msg>>,
    log_path: Option<PathBuf>,
    finished: Arc<Mutex<Option<UninstallOutcome>>>,
}

impl UninstallApp {
    fn new(
        id: String,
        remove_user_data: bool,
        finished: Arc<Mutex<Option<UninstallOutcome>>>,
    ) -> Self {
        Self {
            opening: Opening::new(WINDOW_SIZES),
            theme: Theme::new(ThemeMode::Dark),
            id,
            stage: Stage::Opening,
            opened: None,
            summary: InstallSummary::default(),
            remove_user_data,
            step: None,
            started: None,
            receiver: None,
            log_path: None,
            finished,
        }
    }

    /// Reading the record starts by checking who can change it, which runs other programs,
    /// so the window comes first.
    fn start_load(&mut self, ctx: &egui::Context) {
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        let id = self.id.clone();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let opened = crate::uninstall::state_root().and_then(|root| open_install(&root, &id));
            let message = match opened {
                Ok(opened) => {
                    let summary = opened.summary();
                    Msg::Opened(Box::new(opened), Box::new(summary))
                }
                Err(error) => Msg::OpenFailed(error.to_string()),
            };
            let _ = sender.send(message);
            ctx.request_repaint();
        });
    }

    fn start_uninstall(&mut self, ctx: &egui::Context) {
        let Some(opened) = self.opened.take() else {
            return;
        };
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        self.stage = Stage::Running;
        self.started = Some(Instant::now());
        let remove_user_data = self.remove_user_data;
        let progress = sender.clone();
        let progress_ctx = ctx.clone();
        let monitor = InstallMonitor::new(move |step| {
            let _ = progress.send(Msg::Progress(step));
            progress_ctx.request_repaint();
        });
        let ctx = ctx.clone();
        thread::spawn(move || {
            logging::open_machine_log();
            let outcome = opened.uninstall(remove_user_data, &monitor);
            let _ = sender.send(Msg::Done(Box::new(outcome)));
            ctx.request_repaint();
        });
    }

    fn poll(&mut self, ctx: &egui::Context) {
        let Some(receiver) = self.receiver.take() else {
            return;
        };
        loop {
            match receiver.try_recv() {
                Ok(Msg::Opened(opened, summary)) => {
                    self.opened = Some(*opened);
                    self.summary = *summary;
                    self.stage = Stage::Ready;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                        "Uninstall {}",
                        self.summary.app_name
                    )));
                    return;
                }
                Ok(Msg::OpenFailed(reason)) => {
                    self.stage = Stage::Failed(reason);
                    return;
                }
                Ok(Msg::Progress(step)) => self.step = Some(step),
                Ok(Msg::Done(outcome)) => {
                    self.log_path = logging::current_log_file();
                    if let Ok(mut finished) = self.finished.lock() {
                        *finished = Some((*outcome).clone());
                    }
                    self.stage = Stage::Done(outcome);
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.receiver = Some(receiver);
                    ctx.request_repaint_after(BUSY_REPAINT_INTERVAL);
                    return;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    if matches!(self.stage, Stage::Running | Stage::Opening) {
                        self.stage =
                            Stage::Failed("The uninstall stopped unexpectedly.".to_owned());
                    }
                    return;
                }
            }
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
            )
            .show(ui, |ui| self.draw_header(ui, &theme));
        egui::Panel::bottom("actions")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
            )
            .show(ui, |ui| self.draw_actions(ui, &theme));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.bg))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_LG as i8))
                            .show(ui, |ui| {
                                ui.set_max_width(ui.available_width());
                                self.draw_body(ui, &theme);
                            });
                    });
            });
    }

    fn draw_header(&self, ui: &mut egui::Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(32.0, 32.0), egui::Sense::hover());
            widgets::paint_logo(ui.painter(), rect, theme);
            ui.add_space(SPACE_XS);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let title = if self.summary.app_name.is_empty() {
                    WINDOW_TITLE.to_owned()
                } else {
                    format!("Uninstall {}", self.summary.app_name)
                };
                ui.label(
                    RichText::new(title)
                        .font(theme::semibold(16.0))
                        .color(theme.text),
                );
                ui.label(
                    RichText::new("Undoes what its installer did")
                        .size(12.5)
                        .color(theme.text_muted),
                );
            });
        });
    }

    fn draw_body(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        match &self.stage {
            Stage::Opening => {
                ui.label(
                    RichText::new("Reading the record of this install…")
                        .size(13.5)
                        .color(theme.text),
                );
                ui.add_space(SPACE_SM);
                widgets::progress_bar(ui, theme, None);
            }
            Stage::Failed(reason) => widgets::notice(ui, theme, Role::Danger, reason),
            Stage::Ready | Stage::Running => self.draw_plan(ui, theme),
            Stage::Done(outcome) => draw_outcome(ui, theme, outcome, self.log_path.as_deref()),
        }
    }

    fn draw_plan(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let enabled = matches!(self.stage, Stage::Ready);
        let summary = &self.summary;
        widgets::card_titled(ui, theme, "What will be removed", None, |ui| {
            widgets::kv_row(
                ui,
                theme,
                "Install location",
                &summary.target_root.display().to_string(),
            );
            for (label, count) in [
                ("Files", summary.files),
                ("Registry values", summary.registry_values),
                ("Services", summary.services),
                ("Drivers", summary.drivers),
                ("Scheduled tasks", summary.tasks),
                ("Environment", summary.environment),
            ] {
                if count > 0 {
                    widgets::kv_row(ui, theme, label, &widgets::format_count(count));
                }
            }
            ui.add_space(SPACE_SM);
            widgets::hint(ui, theme, &plan_hint(summary));
        });
        let keepable = summary.user_data + summary.license;
        if keepable > 0 {
            widgets::section_gap(ui);
            let description = format!(
                "{keepable} items of its settings, data, and license. They stay unless this is \
                 on. What they replaced comes back, or goes beside them if they have changed \
                 since."
            );
            widgets::card_titled(ui, theme, "Options", None, |ui| {
                widgets::option_row(
                    ui,
                    theme,
                    &mut self.remove_user_data,
                    "Also remove user data and license data",
                    &description,
                    enabled,
                );
            });
        }
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        if matches!(self.stage, Stage::Running) {
            let (label, fraction) = step_label(self.step);
            let elapsed = self.started.map_or(0, |start| start.elapsed().as_secs());
            ui.horizontal(|ui| {
                ui.label(RichText::new(label).size(13.0).color(theme.text));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(widgets::format_elapsed(elapsed))
                            .size(12.5)
                            .color(theme.text_muted),
                    );
                });
            });
            ui.add_space(SPACE_SM);
            widgets::progress_bar(ui, theme, fraction);
            return;
        }
        let ctx = ui.ctx().clone();
        let ready = matches!(self.stage, Stage::Ready);
        widgets::control_row(ui, |ui| {
            if ready {
                if widgets::button(ui, theme, ButtonKind::Primary, "Uninstall", true).clicked() {
                    self.start_uninstall(&ctx);
                }
                if widgets::button(ui, theme, ButtonKind::Secondary, "Cancel", true).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            } else if widgets::button(ui, theme, ButtonKind::Primary, "Close", true).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}

impl eframe::App for UninstallApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.theme.bg.to_normalized_gamma_f32()
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.opening.settle(ctx, &mut self.theme);
        self.poll(ctx);
        // It can be run again if it is killed, but it has nearly finished by the time anyone
        // would want to, and undoing has no good place to stop.
        if matches!(self.stage, Stage::Running)
            && ctx.input(|input| input.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

fn plan_hint(summary: &InstallSummary) -> String {
    let mut hint = "Files you added or changed stay, with the folders they are in.".to_owned();
    if summary.files_coming_back > 0 {
        hint.push_str(&format!(
            " {} files this install replaced come back, or go beside a file that has changed \
             since.",
            widgets::format_count(summary.files_coming_back)
        ));
    }
    if !summary.runtimes_left.is_empty() {
        hint.push_str(&format!(
            " Runtimes it installed stay, because other programs may use them: {}.",
            summary.runtimes_left.join(", ")
        ));
    }
    hint
}

fn step_label(step: Option<InstallStep>) -> (String, Option<f32>) {
    match step {
        Some(step) if step.total > 0 => (
            format!(
                "Undoing the install: {} of {}…",
                widgets::format_count(step.done),
                widgets::format_count(step.total)
            ),
            step.fraction(),
        ),
        _ => ("Undoing the install…".to_owned(), None),
    }
}

fn draw_outcome(
    ui: &mut egui::Ui,
    theme: &Theme,
    outcome: &UninstallOutcome,
    log_path: Option<&std::path::Path>,
) {
    let report = &outcome.report;
    if outcome.finished {
        widgets::notice(
            ui,
            theme,
            Role::Success,
            if report.kept.is_empty() {
                "The app was uninstalled."
            } else {
                "The app was uninstalled, except for what is listed below."
            },
        );
    } else if report.failed.is_empty() {
        widgets::notice(
            ui,
            theme,
            Role::Warning,
            "Part of it is on a drive that is not connected. Connect it, and uninstall again \
             from Settings.",
        );
    } else {
        widgets::notice(
            ui,
            theme,
            Role::Danger,
            "Not all of it could be undone. Close what is using it, and uninstall again from \
             Settings.",
        );
        for line in &report.failed {
            ui.add_space(SPACE_XS);
            widgets::notice(ui, theme, Role::Danger, line);
        }
        if let Some(log_path) = log_path {
            ui.add_space(SPACE_SM);
            widgets::mono_path(ui, theme, "Log", &log_path.display().to_string());
        }
    }
    widgets::section_gap(ui);
    widgets::metric_tiles(
        ui,
        theme,
        &[
            widgets::metric(
                "Undone",
                widgets::format_count(report.undone.len()),
                Role::Accent,
            ),
            widgets::metric(
                "Left in place",
                widgets::format_count(report.kept.len()),
                if report.kept.is_empty() {
                    Role::Neutral
                } else {
                    Role::Warning
                },
            ),
            widgets::metric(
                "Could not be undone",
                widgets::format_count(report.failed.len()),
                if report.failed.is_empty() {
                    Role::Success
                } else {
                    Role::Danger
                },
            ),
        ],
    );
    let left: Vec<&String> = report.kept.iter().chain(&report.left_behind).collect();
    if !left.is_empty() {
        widgets::section_gap(ui);
        widgets::card_titled(ui, theme, "Left in place", None, |ui| {
            for line in left.iter().take(MAX_LISTED_ITEMS) {
                widgets::hint(ui, theme, line);
            }
            if left.len() > MAX_LISTED_ITEMS {
                widgets::hint(
                    ui,
                    theme,
                    &format!(
                        "and {} more.",
                        widgets::format_count(left.len() - MAX_LISTED_ITEMS)
                    ),
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};
    use winappmigrator_core::{InstallPhase, UndoReport};

    use super::*;

    const PREVIEW_DIR_VARIABLE: &str = "WAM_UI_PREVIEW_DIR";

    fn sample(mode: ThemeMode) -> UninstallApp {
        let mut app = UninstallApp::new("wam-abc".to_owned(), false, Arc::new(Mutex::new(None)));
        app.theme = Theme::new(mode);
        app.stage = Stage::Ready;
        app.summary = InstallSummary {
            app_name: "Fabrikam Data Sync".to_owned(),
            target_root: PathBuf::from(r"C:\Program Files\Fabrikam Data Sync"),
            finished: true,
            files: 214,
            files_coming_back: 3,
            registry_values: 37,
            services: 1,
            tasks: 1,
            environment: 1,
            user_data: 12,
            license: 1,
            runtimes_left: vec!["Microsoft Visual C++ Redistributable".to_owned()],
            ..InstallSummary::default()
        };
        app
    }

    fn preview_dir() -> Option<PathBuf> {
        std::env::var_os(PREVIEW_DIR_VARIABLE).map(PathBuf::from)
    }

    fn harness_for(app: UninstallApp) -> Harness<'static, UninstallApp> {
        harness_sized(app, egui::vec2(620.0, 560.0))
    }

    #[test]
    fn on_a_small_screen_uninstall_can_be_reached() {
        for (width, height, name) in [
            (536.0, 393.0, "smallest-on-1280x720-150"),
            (911.0, 441.0, "1366x768-150"),
        ] {
            let mut harness = harness_sized(sample(ThemeMode::Dark), egui::vec2(width, height));
            let button = harness.get_by_label("Uninstall").rect();
            let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, height));
            assert!(
                window.contains_rect(button),
                "{name}: Uninstall is at {button:?}"
            );
            preview(&mut harness, &format!("on-{name}-dark"));
        }
    }

    fn harness_sized(app: UninstallApp, size: egui::Vec2) -> Harness<'static, UninstallApp> {
        let mode = app.theme.mode;
        let builder = Harness::builder().with_size(size);
        let builder = if preview_dir().is_some() {
            builder.wgpu()
        } else {
            builder
        };
        let mut harness = builder
            .with_context_setup(move |ctx| {
                theme::install_fonts(ctx, &system_root().join("Fonts"));
                Theme::new(mode).apply(ctx);
            })
            .build_ui_state(
                |ui, app: &mut UninstallApp| {
                    let window = ui.ctx().content_rect();
                    ui.scope_builder(egui::UiBuilder::new().max_rect(window), |ui| app.draw(ui));
                },
                app,
            );
        harness.run_steps(3);
        harness
    }

    fn preview(harness: &mut Harness<'_, UninstallApp>, name: &str) {
        let Some(directory) = preview_dir() else {
            return;
        };
        harness.run_steps(2);
        let Ok(image) = harness.render() else {
            return;
        };
        let _ = std::fs::create_dir_all(&directory);
        let _ = image.save(directory.join(format!("uninstall-{name}.png")));
    }

    #[test]
    fn it_says_what_will_be_removed_and_what_stays_before_doing_anything() {
        for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
            let mut harness = harness_for(sample(mode));
            for label in [
                "Uninstall Fabrikam Data Sync",
                "What will be removed",
                "Uninstall",
                "Cancel",
                "Also remove user data and license data",
            ] {
                assert!(harness.query_by_label(label).is_some(), "{label}");
            }
            assert!(harness.query_by_label("214").is_some());
            assert!(
                harness
                    .query_by_label_contains("Files you added or changed stay")
                    .is_some()
            );
            assert!(
                harness
                    .query_by_label_contains("3 files this install replaced come back")
                    .is_some()
            );
            assert!(
                harness
                    .query_by_label_contains("Runtimes it installed stay")
                    .is_some()
            );
            preview(&mut harness, &format!("ready-{name}"));
        }
    }

    #[test]
    fn user_data_is_kept_unless_the_box_is_ticked() {
        let harness = harness_for(sample(ThemeMode::Dark));
        assert!(
            !harness
                .get_by_label("Also remove user data and license data")
                .accesskit_node()
                .is_selected()
                .unwrap_or(false),
            "off until someone turns it on"
        );

        let mut app = sample(ThemeMode::Dark);
        app.summary.user_data = 0;
        app.summary.license = 0;
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label("Also remove user data and license data")
                .is_none()
        );
    }

    #[test]
    fn while_it_runs_it_shows_how_far_it_has_got_and_offers_nothing_to_click() {
        let mut app = sample(ThemeMode::Dark);
        app.stage = Stage::Running;
        app.step = Some(InstallStep::new(InstallPhase::Uninstalling, 40, 200));
        let mut harness = harness_for(app);
        assert!(harness.query_by_label_contains("40 of 200").is_some());
        assert!(harness.query_by_label("Uninstall").is_none());
        assert!(harness.query_by_label("Cancel").is_none());
        preview(&mut harness, "running-dark");
    }

    fn done_with(report: UndoReport) -> UninstallApp {
        let mut app = sample(ThemeMode::Dark);
        let finished = report.failed.is_empty() && !report.waits_for_a_drive();
        app.log_path = Some(PathBuf::from(
            r"C:\ProgramData\WinAppMigrator\logs\uninstall-1.log",
        ));
        app.stage = Stage::Done(Box::new(UninstallOutcome {
            report,
            finished,
            remove_after_exit: None,
        }));
        app
    }

    #[test]
    fn a_drive_that_is_not_connected_is_not_called_a_failure_or_finished() {
        let harness = harness_for(done_with(UndoReport {
            kept: vec![r"What this install put in E:\App is on a drive.".to_owned()],
            drives_waited_for: vec![PathBuf::from(r"E:\App")],
            ..UndoReport::default()
        }));
        assert!(
            harness
                .query_by_label_contains("drive that is not connected")
                .is_some()
        );
        assert!(harness.query_by_label_contains("was uninstalled").is_none());
        assert!(harness.query_by_label_contains("Not all of it").is_none());
    }

    #[test]
    fn a_clean_uninstall_says_so_and_lists_what_it_left() {
        let mut harness = harness_for(done_with(UndoReport {
            undone: vec!["Removed a file".to_owned()],
            kept: vec![r"C:\App\settings.json stays, because it has changed.".to_owned()],
            left_behind: vec!["The runtime stays.".to_owned()],
            ..UndoReport::default()
        }));
        assert!(
            harness
                .query_by_label_contains("except for what is listed below")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("settings.json stays")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("The runtime stays")
                .is_some()
        );
        assert!(harness.query_by_label("Close").is_some());
        assert!(harness.query_by_label("Uninstall").is_none());
        assert!(
            harness.query_by_label("Log").is_none(),
            "nothing went wrong, so nothing to look up"
        );
        preview(&mut harness, "done-dark");
    }

    #[test]
    fn what_could_not_be_undone_is_said_with_where_the_log_is() {
        let mut harness = harness_for(done_with(UndoReport {
            failed: vec![r"C:\App\app.exe could not be removed: access is denied.".to_owned()],
            ..UndoReport::default()
        }));
        assert!(
            harness
                .query_by_label_contains("Not all of it could be undone")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("app.exe could not be")
                .is_some()
        );
        assert!(harness.query_by_label_contains("uninstall-1.log").is_some());
        assert!(harness.query_by_label("Uninstall").is_none());
        preview(&mut harness, "failed-dark");
    }

    #[test]
    fn a_record_that_cannot_be_trusted_offers_nothing_to_do() {
        let mut app = sample(ThemeMode::Dark);
        app.summary = InstallSummary::default();
        app.stage = Stage::Failed("others can change what is in it".to_owned());
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("others can change")
                .is_some()
        );
        assert!(
            harness.query_by_label("Uninstall").is_some(),
            "the window's title"
        );
        assert!(harness.query_by_label("Close").is_some());
        assert!(harness.query_by_label("What will be removed").is_none());
    }

    #[test]
    fn the_window_says_so_while_the_record_is_being_read() {
        let mut app = sample(ThemeMode::Dark);
        app.stage = Stage::Opening;
        let harness = harness_for(app);
        assert!(
            harness
                .query_by_label_contains("Reading the record")
                .is_some()
        );
        assert!(harness.query_by_label("Close").is_some());
    }
}
