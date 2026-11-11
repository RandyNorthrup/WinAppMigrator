//! The single application screen: browse installed apps on the left, then choose
//! what to include and build a self-extracting installer on the right.

use std::cmp::Ordering;

use eframe::egui::Role as A11yRole;
use eframe::egui::{self, Key, Margin, Modifiers, RichText, Sense, Stroke, WidgetInfo};

use winappmigrator_core::account_traces::MENTIONS_CODE;
use winappmigrator_core::{
    AppArchitecture, InstallScope, InstalledApp, ScanSource, SourceIdentity, Warning,
};
use winappmigrator_core::{BuildStep, WarningSeverity};

use crate::network_view;
use crate::theme::{
    self, Role, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS, Theme, ThemeMode,
};
use crate::widgets::{self, ButtonKind, Icon, metric};
use crate::{
    BuildProgress, InventoryState, MigrationOutcome, OperationState, ScopeFilter, SortColumn,
    WinAppMigratorApp,
};

/// The details panel takes what share of the window this says, within these, so that a list of what
/// goes in has room to read on a large window and the list of apps has room on a small one.
const DETAIL_PANEL_SHARE: f32 = 0.40;
const MIN_DETAIL_PANEL_WIDTH: f32 = 380.0;
const MAX_DETAIL_PANEL_WIDTH: f32 = 600.0;
/// A window shorter than this has what is at the top of the details panel come closer together, so
/// that what goes in the installer is still what the panel is about.
const SHORT_WINDOW_HEIGHT: f32 = 720.0;
const ROW_HEIGHT: f32 = 36.0;
const COLUMN_GUTTER: f32 = 12.0;
/// Enough for what is in each of these, whatever the window, with a gutter. What is left goes to the
/// names, which take as much as they get.
const VERSION_COLUMN: f32 = 84.0;
const SCOPE_COLUMN: f32 = 88.0;
const ARCH_COLUMN: f32 = 58.0;
/// What the "Change…" button beside a drive or an image takes, and the least its name is given.
const CHANGE_BUTTON_ROOM: f32 = 96.0;
const MIN_SOURCE_TEXT_WIDTH: f32 = 120.0;
const NAME_SHARE: f32 = 0.6;
const LICENSE_OPTION_TEXT: &str = "May be tied to this PC and need reactivation on the new one.";
const USER_DATA_OPTION_TEXT: &str =
    "Settings and history, Documents, Saved Games, and ProgramData.";
/// How many warnings to list before summarizing the rest.
const MAX_LISTED_WARNINGS: usize = 200;
/// How many accounts' names fit across the details panel, of a width anyone would have.
const MAX_PROFILES_IN_A_ROW: usize = 3;

const COLUMNS: [(&str, SortColumn); 5] = [
    ("Name", SortColumn::Name),
    ("Publisher", SortColumn::Publisher),
    ("Version", SortColumn::Version),
    ("Scope", SortColumn::Scope),
    ("Arch", SortColumn::Architecture),
];

/// Whether this window is short of room, going up and down.
pub(crate) fn is_short(ctx: &egui::Context) -> bool {
    ctx.content_rect().height() < SHORT_WINDOW_HEIGHT
}

impl WinAppMigratorApp {
    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let short = is_short(ui.ctx());

        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(
                        SPACE_XL as i8,
                        if short { SPACE_SM } else { SPACE_MD } as i8,
                    )),
            )
            .show(ui, |ui| self.draw_header(ui, &theme));

        let detail_width = (ui.ctx().content_rect().width() * DETAIL_PANEL_SHARE)
            .clamp(MIN_DETAIL_PANEL_WIDTH, MAX_DETAIL_PANEL_WIDTH);
        egui::Panel::right("details")
            .exact_size(detail_width)
            .resizable(false)
            .frame(egui::Frame::new().fill(theme.surface))
            .show(ui, |ui| self.draw_details(ui, &theme));

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme.bg)
                    .inner_margin(Margin::same(SPACE_LG as i8)),
            )
            .show(ui, |ui| self.draw_applications(ui, &theme));
    }

    fn draw_header(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(32.0, 32.0), Sense::hover());
            widgets::paint_logo(ui.painter(), rect, theme);
            ui.add_space(SPACE_XS);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(
                    RichText::new("WinAppMigrator")
                        .font(theme::semibold(16.0))
                        .color(theme.text),
                );
                ui.label(
                    RichText::new("Turn an installed app into a self-extracting installer")
                        .size(12.5)
                        .color(theme.text_muted),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = SPACE_XS;
                let (icon, label) = if theme.mode == ThemeMode::Dark {
                    (Icon::Sun, "Switch to Light Theme")
                } else {
                    (Icon::Moon, "Switch to Dark Theme")
                };
                if widgets::icon_button(ui, theme, icon, label, true).clicked() {
                    self.toggle_theme();
                }
                let can_reload =
                    !matches!(self.inventory_state, InventoryState::Loading) && !self.is_building();
                if widgets::icon_button(
                    ui,
                    theme,
                    Icon::Refresh,
                    "Reload the Application List",
                    can_reload,
                )
                .clicked()
                {
                    self.start_inventory_load();
                }
                ui.add_space(SPACE_SM);
                let (label, role) = match &self.inventory_state {
                    InventoryState::Loading => ("Loading".to_owned(), Role::Neutral),
                    InventoryState::Loaded => (
                        format!("{} Apps", widgets::format_count(self.apps.len())),
                        Role::Success,
                    ),
                    InventoryState::Failed(_) => ("Load Failed".to_owned(), Role::Danger),
                };
                widgets::status_pill(ui, theme, &label, role);
            });
        });
    }

    // ----- Left: the application list ---------------------------------------

    fn draw_applications(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        self.source_selector(ui, theme);
        ui.add_space(SPACE_MD);
        if self.network.is_some() && self.remote.is_none() {
            self.draw_network_wizard(ui, theme);
            return;
        }

        let search_id = ui.make_persistent_id("app-search");
        widgets::control_row(ui, |ui| {
            let filter_width = 262.0;
            let search_width = (ui.available_width() - filter_width - SPACE_SM).max(160.0);
            widgets::search_field(
                ui,
                theme,
                search_id,
                &mut self.app_search,
                "Search by Name or Publisher  (Ctrl+F)",
                search_width,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                widgets::segmented(
                    ui,
                    theme,
                    &mut self.app_scope_filter,
                    &[
                        ("All", ScopeFilter::All),
                        ("Per-User", ScopeFilter::PerUser),
                        ("Per-Machine", ScopeFilter::PerMachine),
                    ],
                    true,
                );
            });
        });
        if ui.input_mut(|input| input.consume_key(Modifiers::COMMAND, Key::F)) {
            ui.memory_mut(|memory| memory.request_focus(search_id));
        }
        ui.add_space(SPACE_MD);

        match &self.inventory_state {
            InventoryState::Loading => {
                widgets::card(ui, theme, |ui| {
                    busy_row(ui, theme, "Loading installed applications…");
                });
                return;
            }
            InventoryState::Failed(error) => {
                let message = format!("Could not list applications: {error}");
                widgets::notice(ui, theme, Role::Danger, &message);
                return;
            }
            InventoryState::Loaded => {}
        }

        let visible = self.visible_apps();
        self.handle_row_keys(ui, search_id, &visible);

        egui::Frame::new()
            .fill(theme.card)
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(theme::RADIUS_LG)
            .inner_margin(Margin::same(SPACE_XS as i8))
            .show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                ui.spacing_mut().item_spacing.y = 0.0;
                self.draw_table_header(ui, theme);
                ui.add_space(SPACE_XS);

                if visible.is_empty() {
                    ui.add_space(SPACE_XL);
                    widgets::empty_state(
                        ui,
                        theme,
                        Icon::Apps,
                        "No Matching Applications",
                        "Adjust the search text or scope filter.",
                    );
                    return;
                }

                let mut area = egui::ScrollArea::vertical()
                    .id_salt("apps-table")
                    .auto_shrink([false, false]);
                if let Some(row) = self.scroll_to_row.take() {
                    area = area.vertical_scroll_offset(scroll_offset_for_row(row, ui));
                }
                let building = self.is_building();
                area.show_rows(ui, ROW_HEIGHT, visible.len(), |ui, range| {
                    for row in range {
                        let app_index = visible[row];
                        let selected = self.selected_app_index == Some(app_index);
                        let clicked =
                            app_row(ui, theme, &self.apps[app_index], selected, !building)
                                .clicked();
                        if clicked {
                            self.select_app(app_index);
                        }
                    }
                });
            });
    }

    /// The scan-source selector: read installed apps from this PC, or from a
    /// drive connected from another (often failing) PC.
    fn source_selector(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        #[derive(Clone, Copy, PartialEq)]
        enum Source {
            ThisPc,
            Drive,
            Image,
            Network,
        }
        let mut chosen = if self.network.is_some() {
            Source::Network
        } else if self.disk_image.is_some() || self.image_receiver.is_some() {
            Source::Image
        } else if self.scan_source.is_offline() {
            Source::Drive
        } else {
            Source::ThisPc
        };
        let enabled = !self.is_building()
            && self.image_receiver.is_none()
            && !self
                .network
                .as_ref()
                .is_some_and(network_view::NetworkWizard::is_connecting);
        widgets::control_row(ui, |ui| {
            widgets::field_label(ui, theme, "Scan Source");
            let before = chosen;
            let changed = widgets::segmented(
                ui,
                theme,
                &mut chosen,
                &[
                    ("This PC", Source::ThisPc),
                    ("Connected Drive…", Source::Drive),
                    ("Disk Image…", Source::Image),
                    ("Network PC…", Source::Network),
                ],
                enabled,
            );
            if changed && before != chosen {
                match chosen {
                    Source::ThisPc => self.select_live_source(),
                    Source::Drive => self.pick_offline_drive(),
                    Source::Image => self.pick_disk_image(),
                    Source::Network => self.select_network_source(),
                }
            }
        });
        // On a line of its own: a path and the name of a Windows can be long.
        if self.image_receiver.is_some() {
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new("Attaching the image, read-only…")
                    .size(13.0)
                    .color(theme.text_muted),
            );
        } else if let ScanSource::OfflineVolume { volume_root } = self.scan_source.clone() {
            ui.add_space(SPACE_XS);
            widgets::control_row(ui, |ui| {
                let place = match (&self.disk_image, &self.remote) {
                    (Some(image), _) => image.image().file_name().map_or_else(
                        || image.image().display().to_string(),
                        |name| name.to_string_lossy().into_owned(),
                    ),
                    (None, Some(remote)) => format!(
                        "{} over the network ({})",
                        remote.about().computer_name,
                        remote.address().ip()
                    ),
                    (None, None) => volume_root.display().to_string(),
                };
                let windows = self
                    .source_windows
                    .as_ref()
                    .and_then(|identity| identity.windows_name());
                let text = match windows {
                    Some(windows) => format!("{place} · {windows}"),
                    None => place,
                };
                let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), Sense::hover());
                widgets::paint_icon(ui.painter(), Icon::Drive, rect, theme.text_muted);
                // Shortened to leave the button room, rather than pushing it out of sight.
                let room = (ui.available_width() - CHANGE_BUTTON_ROOM).max(MIN_SOURCE_TEXT_WIDTH);
                ui.scope(|ui| {
                    ui.set_max_width(room);
                    ui.add(
                        egui::Label::new(RichText::new(text).size(13.0).color(theme.text_muted))
                            .truncate(),
                    );
                });
                let change = if self.remote.is_some() {
                    "Disconnect"
                } else {
                    "Change…"
                };
                if widgets::button(ui, theme, ButtonKind::Ghost, change, enabled).clicked() {
                    if self.remote.is_some() {
                        // Back to the steps, to pair with this PC again or another.
                        self.leave_network_source();
                        self.select_network_source();
                        self.scan_source = ScanSource::Live;
                        self.source_windows = None;
                    } else if self.disk_image.is_some() {
                        self.pick_disk_image();
                    } else {
                        self.pick_offline_drive();
                    }
                }
            });
        }
        if let Some(error) = self.source_error.clone() {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Danger, &error);
        } else if let Some(identity) = self
            .source_windows
            .as_ref()
            .filter(|identity| identity.is_before_windows_10())
        {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Accent, &older_windows_note(identity));
        }
    }

    fn visible_apps(&self) -> Vec<usize> {
        let query = self.app_search.trim().to_lowercase();
        let mut visible: Vec<usize> = self
            .apps
            .iter()
            .enumerate()
            .filter(|(_, app)| scope_matches(self.app_scope_filter, &app.install_scope))
            .filter(|(_, app)| {
                query.is_empty()
                    || app.display_name.to_lowercase().contains(&query)
                    || app
                        .publisher
                        .as_deref()
                        .is_some_and(|publisher| publisher.to_lowercase().contains(&query))
            })
            .map(|(index, _)| index)
            .collect();
        visible.sort_by(|left, right| {
            let ordering = compare_apps(&self.apps[*left], &self.apps[*right], self.sort_column);
            if self.sort_ascending {
                ordering
            } else {
                ordering.reverse()
            }
        });
        visible
    }

    /// Arrow keys move the selection, unless the user is typing somewhere else.
    fn handle_row_keys(&mut self, ui: &egui::Ui, search_id: egui::Id, visible: &[usize]) {
        if self.is_building() || visible.is_empty() {
            return;
        }
        let focused = ui.memory(|memory| memory.focused());
        if focused.is_some_and(|id| id != search_id) {
            return;
        }
        let (down, up) = ui.input(|input| {
            (
                input.key_pressed(Key::ArrowDown),
                input.key_pressed(Key::ArrowUp),
            )
        });
        if !down && !up {
            return;
        }
        let current = self
            .selected_app_index
            .and_then(|selected| visible.iter().position(|index| *index == selected));
        let last = visible.len() - 1;
        let target = match (current, down) {
            (None, true) => 0,
            (None, false) => last,
            (Some(row), true) => (row + 1).min(last),
            (Some(row), false) => row.saturating_sub(1),
        };
        if current != Some(target) {
            self.select_app(visible[target]);
            self.scroll_to_row = Some(target);
        }
    }

    fn draw_table_header(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 28.0), Sense::hover());
        let cells = column_rects(rect);
        for (index, (cell, (label, column))) in cells.into_iter().zip(COLUMNS).enumerate() {
            let id = ui.scope_id().with(("sort", index));
            let response = ui
                .interact(cell, id, Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            let sorted = self.sort_column == column;
            response.widget_info(|| {
                WidgetInfo::labeled(A11yRole::Button, true, format!("Sort by {label}"))
            });
            let color = if sorted || response.hovered() {
                theme.text
            } else {
                theme.text_subtle
            };
            let font = theme::semibold(12.0);
            let galley = ui.painter().layout_no_wrap(label.to_owned(), font, color);
            let text_size = galley.size();
            let top = cell.center().y - text_size.y / 2.0;
            ui.painter()
                .galley(egui::pos2(cell.left(), top), galley, color);
            if sorted {
                let icon = if self.sort_ascending {
                    Icon::ChevronUp
                } else {
                    Icon::ChevronDown
                };
                let icon_rect = egui::Rect::from_center_size(
                    egui::pos2(cell.left() + text_size.x + 10.0, cell.center().y),
                    egui::vec2(10.0, 10.0),
                );
                widgets::paint_icon(ui.painter(), icon, icon_rect, theme.text);
            }
            if response.clicked() {
                if sorted {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_column = column;
                    self.sort_ascending = true;
                }
            }
        }
        let separator = rect.bottom();
        ui.painter()
            .hline(rect.x_range(), separator, Stroke::new(1.0, theme.border));
    }

    /// Select an app and prefill the default output directory for it. What was chosen for another
    /// app means nothing for this one, and any scan of that one stops the next time the window
    /// looks. Selection is locked during a build: the build and its result belong to the app it
    /// started with.
    pub(crate) fn select_app(&mut self, index: usize) {
        if self.is_building() || self.selected_app_index == Some(index) {
            return;
        }
        self.selected_app_index = Some(index);
        self.output_dir = crate::default_output_for(&self.apps[index].display_name);
        self.start_profile_lookup(self.apps[index].clone());
        self.forget_choices();
        // A fresh selection starts a fresh migration result.
        self.migrate_state = OperationState::Idle;
        self.last_outcome = None;
        self.last_error = None;
    }

    // ----- Right: the selected application ----------------------------------

    fn draw_details(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let selected = self
            .selected_app_index
            .and_then(|index| self.apps.get(index).cloned());
        let Some(app) = selected else {
            egui::Frame::new()
                .inner_margin(Margin::same(SPACE_XL as i8))
                .show(ui, |ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    if self.network.is_some() && self.remote.is_none() {
                        widgets::empty_state(
                            ui,
                            theme,
                            Icon::Apps,
                            "Pair with the Old PC",
                            "Once WinAppMigrator Client is running there and you have typed its \
                             code, its apps are listed here, and you build an installer from one \
                             as from this PC.",
                        );
                    } else {
                        widgets::empty_state(
                            ui,
                            theme,
                            Icon::Apps,
                            "Pick an Application",
                            "Select an app from the list to choose what to include and build its \
                             installer.",
                        );
                    }
                });
            return;
        };

        let short = is_short(ui.ctx());
        egui::Panel::bottom("details-actions")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::same(if short { SPACE_MD } else { SPACE_LG } as i8)),
            )
            .show(ui, |ui| self.draw_actions(ui, theme));

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.surface))
            .show(ui, |ui| {
                let mut area = egui::ScrollArea::vertical()
                    .id_salt("details-scroll")
                    .auto_shrink([false, false]);
                // A new result appears at the top, so bring the top into view.
                if std::mem::take(&mut self.scroll_details_to_top) {
                    area = area.vertical_scroll_offset(0.0);
                }
                area.show(ui, |ui| {
                    egui::Frame::new()
                        .inner_margin(Margin::same(SPACE_LG as i8))
                        .show(ui, |ui| {
                            ui.set_max_width(ui.available_width());
                            draw_app_title(ui, theme, &app, short);
                            // The result goes first: it is what the user is waiting
                            // for, and it holds the installer's path and folder button.
                            self.draw_build_result(ui, theme);
                            ui.add_space(if short { SPACE_XS } else { SPACE_SM });
                            draw_app_facts(ui, theme, &app, short);
                            ui.add_space(if short { SPACE_SM } else { SPACE_MD });
                            self.draw_footprint(ui, theme);
                        });
                });
            });
    }

    /// What is asked of a scan, so a change here looks at the app again. What it found can then be
    /// chosen from, which is all a matter of what is left out.
    pub(crate) fn draw_options(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let enabled = !self.is_building();
        widgets::control_row(ui, |ui| {
            ui.spacing_mut().item_spacing.x = SPACE_MD;
            widgets::inline_choice(
                ui,
                theme,
                &mut self.backup_license,
                "License and Activation Data",
                LICENSE_OPTION_TEXT,
                enabled,
            );
            widgets::inline_choice(
                ui,
                theme,
                &mut self.backup_user_data,
                "User Data",
                USER_DATA_OPTION_TEXT,
                enabled,
            );
        });
        if self.backup_license {
            widgets::hint(ui, theme, LICENSE_OPTION_TEXT);
        }
        if self.backup_user_data {
            widgets::hint(ui, theme, USER_DATA_OPTION_TEXT);
        }
        self.draw_profile_choice(ui, theme, enabled);
    }

    /// Whose data goes in, shown only when more than one account has any, or when something about
    /// that is worth saying. An installer holds one account's.
    fn draw_profile_choice(&mut self, ui: &mut egui::Ui, theme: &Theme, enabled: bool) {
        let Some(picker) = self.profile_picker.clone() else {
            return;
        };
        if let Some(note) = picker.note.as_deref() {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Warning, note);
        }
        if !self.backup_user_data || !picker.offers_a_choice() {
            return;
        }
        ui.add_space(SPACE_SM);
        widgets::field_label(ui, theme, "Whose Data");
        ui.add_space(2.0);
        let mut chosen = picker
            .rows
            .iter()
            .position(|row| self.chosen_profile.as_deref() == Some(row.id.as_str()));
        for (row_number, rows) in picker.rows.chunks(MAX_PROFILES_IN_A_ROW).enumerate() {
            let options: Vec<(&str, Option<usize>)> = rows
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    let index = row_number * MAX_PROFILES_IN_A_ROW + index;
                    (row.name.as_str(), Some(index))
                })
                .collect();
            widgets::control_row(ui, |ui| {
                widgets::segmented(ui, theme, &mut chosen, &options, enabled);
            });
        }
        self.chosen_profile = chosen
            .and_then(|index| picker.rows.get(index))
            .map(|row| row.id.clone());
        ui.add_space(SPACE_XS);
        widgets::hint(
            ui,
            theme,
            "An installer holds one account's data. It goes to whoever is signed in where it \
             runs.",
        );
    }

    pub(crate) fn output_dir_is_valid(&self) -> bool {
        std::path::Path::new(self.output_dir.trim()).is_absolute()
    }

    fn draw_actions(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        if self.is_building() {
            let (label, fraction) = progress_label(self.build_progress.as_ref());
            let label = if self.is_cancelling() {
                "Cancelling after the current step…".to_owned()
            } else {
                label
            };
            let elapsed = self
                .build_started
                .map_or(0, |start| start.elapsed().as_secs());
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
            ui.add_space(SPACE_MD);
            let cancelling = self.is_cancelling();
            if widgets::wide_button(ui, theme, ButtonKind::Secondary, "Cancel", !cancelling)
                .clicked()
            {
                self.cancel_migration();
            }
            return;
        }

        if !self.stub_available {
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                "Installer stub (winappmigrator-installer.exe) was not found beside this app, \
                 so a self-extracting installer cannot be built.",
            );
            ui.add_space(SPACE_SM);
        }
        // Beside the button that writes there, so that where it goes is always in view.
        widgets::path_field(
            ui,
            theme,
            "Output Folder",
            &mut self.output_dir,
            r"C:\Users\You\WinAppMigrator",
            true,
            || rfd::FileDialog::new().pick_folder(),
        );
        if !self.output_dir_is_valid() {
            ui.add_space(SPACE_XS);
            widgets::hint(
                ui,
                theme,
                "Enter a full folder path, such as D:\\Installers.",
            );
        }
        ui.add_space(SPACE_SM);
        let can_build = self.can_build();
        // Once an installer exists, the result is the next thing to act on.
        let (kind, label) = if matches!(self.migrate_state, OperationState::Completed) {
            (ButtonKind::Secondary, "Create Installer Again")
        } else {
            (ButtonKind::Primary, "Create Installer")
        };
        if widgets::wide_button(ui, theme, kind, label, can_build).clicked() {
            self.start_migration();
        }
    }

    fn draw_build_result(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        match self.migrate_state {
            OperationState::Idle | OperationState::Running => {}
            OperationState::Cancelled => {
                ui.add_space(SPACE_MD);
                widgets::notice(
                    ui,
                    theme,
                    Role::Neutral,
                    "Build cancelled. No installer was written.",
                );
            }
            OperationState::Failed => {
                ui.add_space(SPACE_MD);
                let error = self.last_error.clone().unwrap_or_default();
                widgets::notice(ui, theme, Role::Danger, &error);
                ui.add_space(SPACE_SM);
                if let Some(log) = self.log_file.as_deref() {
                    widgets::mono_path(ui, theme, "Log", &log.display().to_string());
                    ui.add_space(SPACE_SM);
                }
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    Icon::Copy,
                    "Copy Error",
                    true,
                )
                .clicked()
                {
                    ui.ctx().copy_text(error);
                }
            }
            OperationState::Completed => self.draw_outcome(ui, theme),
        }
    }

    fn draw_outcome(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let Some(outcome) = self.last_outcome.as_ref() else {
            return;
        };
        let MigrationOutcome {
            app_name,
            package,
            verification,
            installer_bytes,
        } = outcome;

        ui.add_space(SPACE_MD);
        let (role, message) = outcome_summary(outcome);
        widgets::notice(ui, theme, role, &message);
        ui.add_space(SPACE_SM);
        if role == Role::Danger
            && let Some(log) = self.log_file.as_deref()
        {
            widgets::mono_path(ui, theme, "Log", &log.display().to_string());
            ui.add_space(SPACE_SM);
        }
        widgets::metric_tiles(
            ui,
            theme,
            &[
                metric(
                    "Files",
                    widgets::format_count(package.files_copied),
                    Role::Accent,
                ),
                metric(
                    "Installer Size",
                    widgets::format_bytes(*installer_bytes),
                    Role::Accent,
                ),
                metric(
                    "Warnings",
                    widgets::format_count(package.warnings.len()),
                    if package.warnings.is_empty() {
                        Role::Neutral
                    } else {
                        Role::Warning
                    },
                ),
            ],
        );

        // Each by itself, first, however many other warnings there are, as it is about what anyone who
        // is given the installer can read.
        let (mentions, other_warnings): (Vec<Warning>, Vec<Warning>) = package
            .warnings
            .iter()
            .cloned()
            .partition(|warning| warning.code == MENTIONS_CODE);
        for warning in &mentions {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Warning, &warning.message);
        }

        let mut folder_error = None;
        if let Some(path) = package.self_extracting_archive_path.as_ref() {
            if let Some(data_file) = package.data_file_path.as_ref() {
                ui.add_space(SPACE_SM);
                widgets::notice(ui, theme, Role::Warning, &two_files_text(path, data_file));
            }
            ui.add_space(SPACE_SM);
            widgets::mono_path(ui, theme, "Installer", &path.display().to_string());
            if let Some(data_file) = package.data_file_path.as_ref() {
                ui.add_space(SPACE_XS);
                widgets::mono_path(ui, theme, "Data File", &data_file.display().to_string());
            }
            ui.add_space(SPACE_SM);
            widgets::control_row(ui, |ui| {
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    Icon::Folder,
                    "Open Folder",
                    true,
                )
                .clicked()
                    && let Some(folder) = path.parent()
                    && let Err(error) = crate::open_folder(folder)
                {
                    folder_error = Some(format!("Could not open the folder: {error}"));
                }
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Ghost,
                    Icon::Copy,
                    "Copy Path",
                    true,
                )
                .clicked()
                {
                    ui.ctx().copy_text(path.display().to_string());
                }
            });
        }
        ui.add_space(SPACE_SM);
        widgets::hint(
            ui,
            theme,
            &format!(
                "Run it on the other PC to reinstall {app_name}. It checks its files by SHA-256. \
                 Windows shows \"Unknown publisher\" because installers you build are not \
                 signed."
            ),
        );

        warning_list(ui, theme, "Verification Errors", &verification.errors, true);
        // When files were lost, the list of which ones is the next thing to read.
        warning_list(
            ui,
            theme,
            "Warnings",
            &other_warnings,
            role == Role::Warning,
        );

        if let Some(error) = folder_error {
            self.last_error = Some(error);
        }
        if let Some(error) = self.last_error.as_ref() {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Danger, error);
        }
    }
}

// ----- Free helpers ----------------------------------------------------------

/// What happens to an app from a Windows older than 10, said once a drive or an image has one.
pub(crate) fn older_windows_note(identity: &SourceIdentity) -> String {
    let windows = identity
        .windows_name()
        .unwrap_or_else(|| "an older Windows".to_owned());
    let mut note = format!(
        "This has {windows}. Installers built from it are made for Windows 10 and 11. With User \
         Data, settings that Windows quietly kept aside for an older app come along, and \
         runtimes Windows 11 does not have, such as DirectX 9, are installed with the app."
    );
    if identity.is_32_bit() {
        note.push_str(
            " Its apps are 32-bit, so they go to Program Files (x86), where 64-bit Windows keeps \
             them.",
        );
    }
    note
}

fn draw_app_title(ui: &mut egui::Ui, theme: &Theme, app: &InstalledApp, short: bool) {
    ui.add(
        egui::Label::new(
            RichText::new(&app.display_name)
                .font(theme::semibold(17.0))
                .color(theme.text),
        )
        .wrap_mode(egui::TextWrapMode::Wrap),
    );
    ui.add_space(SPACE_XS);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = SPACE_XS;
        widgets::status_pill(
            ui,
            theme,
            scope_label(&app.install_scope),
            scope_role(&app.install_scope),
        );
        widgets::badge(ui, theme, arch_label(&app.architecture), theme.text_muted);
        if short {
            ui.add_space(SPACE_XS);
            ui.add(
                egui::Label::new(
                    RichText::new(who_and_which_version(app))
                        .size(12.5)
                        .color(theme.text_muted),
                )
                .truncate(),
            );
        }
    });
}

fn who_and_which_version(app: &InstalledApp) -> String {
    let publisher = app.publisher.as_deref().unwrap_or("Unknown publisher");
    match app.version.as_deref() {
        Some(version) => format!("{publisher}, version {version}"),
        None => publisher.to_owned(),
    }
}

/// Who made it, which version, and where it is, on as few lines as that takes, so that what goes
/// in the installer is what the panel is about. In a short window, the first of those is up beside
/// the app's name.
fn draw_app_facts(ui: &mut egui::Ui, theme: &Theme, app: &InstalledApp, short: bool) {
    if !short {
        widgets::hint(ui, theme, &who_and_which_version(app));
    }
    if let Some(location) = app.install_location.as_ref() {
        widgets::hint(ui, theme, &location.display().to_string());
    }
}

fn busy_row(ui: &mut egui::Ui, theme: &Theme, label: &str) {
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(RichText::new(label).color(theme.text));
    });
}

/// What the build's outcome means for the person holding the installer. A package
/// that lost files still hash-verifies, because a file that was never copied has
/// nothing to check, so that must not read as a clean success.
pub(crate) fn outcome_summary(outcome: &MigrationOutcome) -> (Role, String) {
    let count_of = |code: &str| {
        outcome
            .package
            .warnings
            .iter()
            .filter(|warning| warning.code == code)
            .count()
    };
    let missed = count_of("file_copy_failed");
    let gone = count_of("file_gone_since_it_was_looked_at");
    if !outcome.verification.package_valid {
        (
            Role::Danger,
            "Do not use this installer: it failed its hash check.".to_owned(),
        )
    } else if gone > 0 && missed == 0 {
        (
            Role::Warning,
            format!(
                "Installer built, but {gone} {} gone since the app was looked at, so {} not in it. \
                 See Warnings below.",
                if gone == 1 { "file was" } else { "files were" },
                if gone == 1 { "it is" } else { "they are" },
            ),
        )
    } else if missed + gone > 0 {
        let missed = missed + gone;
        (
            Role::Warning,
            format!(
                "Installer built, but {missed} {} could not be captured, usually because \
                 {} locked or unreadable. The app may not work without {}. See Warnings below.",
                if missed == 1 { "file" } else { "files" },
                if missed == 1 { "it was" } else { "they were" },
                if missed == 1 { "it" } else { "them" },
            ),
        )
    } else if count_of(MENTIONS_CODE) > 0 {
        (
            Role::Warning,
            "Installer built, and it still says something of who it came from. Read what it says \
             below before you give it to anyone."
                .to_owned(),
        )
    } else {
        (
            Role::Success,
            "Installer ready. Its package passed its hash check.".to_owned(),
        )
    }
}

/// What someone with a build too big for one executable has to know, first.
fn two_files_text(installer: &std::path::Path, data_file: &std::path::Path) -> String {
    let name = |path: &std::path::Path| {
        path.file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
    };
    format!(
        "This app is too big for one executable, so its installer is two files, {} and {}. They \
         have to stay together: copy both to the other PC, into the same folder, with these names. \
         If you sign the installer, that signature also covers the data file.",
        name(installer),
        name(data_file)
    )
}

pub(crate) fn warning_list(
    ui: &mut egui::Ui,
    theme: &Theme,
    title: &str,
    warnings: &[Warning],
    open: bool,
) {
    if warnings.is_empty() {
        return;
    }
    ui.add_space(SPACE_SM);
    egui::CollapsingHeader::new(
        RichText::new(format!(
            "{title} ({})",
            widgets::format_count(warnings.len())
        ))
        .font(theme::semibold(13.0))
        .color(theme.text),
    )
    .default_open(open)
    .show(ui, |ui| {
        for warning in warnings.iter().take(MAX_LISTED_WARNINGS) {
            warning_row(ui, theme, warning);
        }
        if warnings.len() > MAX_LISTED_WARNINGS {
            widgets::hint(
                ui,
                theme,
                &format!(
                    "and {} more.",
                    widgets::format_count(warnings.len() - MAX_LISTED_WARNINGS)
                ),
            );
        }
    });
}

fn warning_row(ui: &mut egui::Ui, theme: &Theme, warning: &Warning) {
    let role = severity_role(&warning.severity);
    ui.horizontal_top(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 16.0), Sense::hover());
        let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(13.0, 13.0));
        widgets::paint_icon(
            ui.painter(),
            widgets::role_icon(role),
            icon_rect,
            theme.role(role),
        );
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            ui.add(
                egui::Label::new(RichText::new(&warning.message).size(12.5).color(theme.text))
                    .wrap_mode(egui::TextWrapMode::Wrap),
            );
            if let Some(source) = warning.source.as_deref() {
                ui.add(
                    egui::Label::new(RichText::new(source).size(11.5).color(theme.text_muted))
                        .wrap_mode(egui::TextWrapMode::Wrap),
                );
            }
        });
    });
    ui.add_space(SPACE_XS);
}

fn progress_label(progress: Option<&BuildProgress>) -> (String, Option<f32>) {
    match progress {
        None | Some(BuildProgress::Build(BuildStep::Scanning | BuildStep::Preparing)) => {
            ("Working out what goes in…".to_owned(), None)
        }
        Some(BuildProgress::Build(BuildStep::RoomChecked(_))) => {
            ("Checking there is room…".to_owned(), None)
        }
        Some(BuildProgress::Build(BuildStep::Bundling)) => {
            ("Bundling runtimes and drivers…".to_owned(), None)
        }
        // By how much, since one large file can be most of the time and only one of the files.
        Some(BuildProgress::Build(BuildStep::Packing {
            done,
            total,
            bytes_done,
            bytes_total,
        })) => (
            format!(
                "Reading and compressing files: {} of {}",
                widgets::format_count(*done),
                widgets::format_count(*total)
            ),
            (*bytes_total > 0).then(|| *bytes_done as f32 / *bytes_total as f32),
        ),
        Some(BuildProgress::Build(BuildStep::LookingOverWhatItSays { done, total })) => (
            format!(
                "Looking over what the installer says: {} of {}…",
                widgets::format_count(*done),
                widgets::format_count(*total)
            ),
            None,
        ),
        Some(BuildProgress::Build(BuildStep::WritingInstaller)) => {
            ("Writing the installer…".to_owned(), None)
        }
        Some(BuildProgress::Verifying) => ("Verifying hashes…".to_owned(), None),
    }
}

/// The scroll offset that centers a row in the list.
fn scroll_offset_for_row(row: usize, ui: &egui::Ui) -> f32 {
    (row as f32 * ROW_HEIGHT - ui.available_height() / 2.0).max(0.0)
}

fn app_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    app: &InstalledApp,
    selected: bool,
    enabled: bool,
) -> egui::Response {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_HEIGHT), sense);
    let response = if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    let publisher = app.publisher.as_deref().unwrap_or("—");
    let version = app.version.as_deref().unwrap_or("—");
    response.widget_info(|| {
        WidgetInfo::selected(
            A11yRole::Button,
            enabled,
            selected,
            format!("{}, {publisher}, version {version}", app.display_name),
        )
    });

    let row = rect.shrink2(egui::vec2(0.0, 1.0));
    if selected {
        ui.painter()
            .rect_filled(row, theme::RADIUS_SM, theme.accent_soft);
    } else if enabled && response.hovered() {
        ui.painter()
            .rect_filled(row, theme::RADIUS_SM, theme.card_hover);
    }

    let cells = [
        (app.display_name.as_str(), theme.text, true),
        (publisher, theme.text_muted, false),
        (version, theme.text_muted, false),
        (scope_label(&app.install_scope), theme.text_muted, false),
        (arch_label(&app.architecture), theme.text_muted, false),
    ];
    let columns = column_rects(rect);
    // All of it, if any of it was cut. It would otherwise only cover the rows below.
    let name_font = theme::semibold(13.5);
    let names_fit = widgets::text_fits(ui, &app.display_name, name_font, columns[0].width())
        && widgets::text_fits(ui, publisher, theme::regular(13.0), columns[1].width());
    for (cell, (text, color, strong)) in columns.into_iter().zip(cells) {
        let font = if strong {
            theme::semibold(13.5)
        } else {
            theme::regular(13.0)
        };
        widgets::paint_elided(
            ui,
            text,
            font,
            color,
            cell.left(),
            cell.center().y,
            cell.width(),
        );
    }
    if response.has_focus() {
        ui.painter().rect_stroke(
            row,
            theme::RADIUS_SM,
            Stroke::new(1.5, theme.accent),
            egui::StrokeKind::Inside,
        );
    }
    if names_fit {
        response
    } else {
        response.on_hover_text(format!("{}\n{publisher}", app.display_name))
    }
}

/// The cell rectangles of a table row, with a gutter between columns.
pub(crate) fn column_rects(rect: egui::Rect) -> [egui::Rect; 5] {
    let usable = rect.width() - COLUMN_GUTTER;
    let names = (usable - VERSION_COLUMN - SCOPE_COLUMN - ARCH_COLUMN).max(0.0);
    let mut left = rect.left() + COLUMN_GUTTER;
    [
        names * NAME_SHARE,
        names * (1.0 - NAME_SHARE),
        VERSION_COLUMN,
        SCOPE_COLUMN,
        ARCH_COLUMN,
    ]
    .map(|width| {
        let cell = egui::Rect::from_min_size(
            egui::pos2(left, rect.top()),
            egui::vec2((width - COLUMN_GUTTER).max(0.0), rect.height()),
        );
        left += width;
        cell
    })
}

fn compare_apps(left: &InstalledApp, right: &InstalledApp, column: SortColumn) -> Ordering {
    let by_name = || {
        left.display_name
            .to_lowercase()
            .cmp(&right.display_name.to_lowercase())
    };
    let ordering = match column {
        SortColumn::Name => by_name(),
        SortColumn::Publisher => {
            let lower = |app: &InstalledApp| app.publisher.as_deref().map(str::to_lowercase);
            lower(left).cmp(&lower(right))
        }
        SortColumn::Version => compare_versions(
            left.version.as_deref().unwrap_or_default(),
            right.version.as_deref().unwrap_or_default(),
        ),
        SortColumn::Scope => {
            scope_label(&left.install_scope).cmp(scope_label(&right.install_scope))
        }
        SortColumn::Architecture => {
            arch_label(&left.architecture).cmp(arch_label(&right.architecture))
        }
    };
    ordering.then_with(by_name)
}

/// Compare versions number by number, so 10.2 sorts after 9.11.
pub(crate) fn compare_versions(left: &str, right: &str) -> Ordering {
    let numbers = |version: &str| -> Vec<u64> {
        version
            .split(|character: char| !character.is_ascii_digit())
            .filter(|part| !part.is_empty())
            .map(|part| part.parse().unwrap_or(u64::MAX))
            .collect()
    };
    numbers(left).cmp(&numbers(right))
}

fn scope_matches(filter: ScopeFilter, scope: &InstallScope) -> bool {
    match filter {
        ScopeFilter::All => true,
        ScopeFilter::PerUser => matches!(scope, InstallScope::PerUser),
        ScopeFilter::PerMachine => matches!(scope, InstallScope::PerMachine),
    }
}

pub(crate) fn arch_label(arch: &AppArchitecture) -> &'static str {
    match arch {
        AppArchitecture::X86 => "x86",
        AppArchitecture::X64 => "x64",
        AppArchitecture::Arm64 => "ARM64",
        AppArchitecture::Unknown => "Unknown",
    }
}

pub(crate) fn scope_label(scope: &InstallScope) -> &'static str {
    match scope {
        InstallScope::PerUser => "Per-User",
        InstallScope::PerMachine => "Per-Machine",
        InstallScope::Unknown => "Unknown",
    }
}

fn scope_role(scope: &InstallScope) -> Role {
    match scope {
        InstallScope::PerUser | InstallScope::PerMachine => Role::Accent,
        InstallScope::Unknown => Role::Neutral,
    }
}

fn severity_role(severity: &WarningSeverity) -> Role {
    match severity {
        WarningSeverity::Info => Role::Accent,
        WarningSeverity::Warning => Role::Warning,
        WarningSeverity::Error => Role::Danger,
    }
}
