//! The single application screen: browse installed apps on the left, then choose
//! what to include and build a self-extracting installer on the right.

use std::cmp::Ordering;

use eframe::egui::Role as A11yRole;
use eframe::egui::{self, Key, Margin, Modifiers, RichText, Sense, Stroke, WidgetInfo};

use winappmigrator_core::{AppArchitecture, InstallScope, InstalledApp, ScanSource, Warning};
use winappmigrator_core::{BuildStep, WarningSeverity};

use crate::theme::{
    self, Role, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS, Theme, ThemeMode,
};
use crate::widgets::{self, ButtonKind, Icon, metric};
use crate::{
    BuildProgress, InventoryState, MigrationOutcome, OperationState, ScopeFilter, SortColumn,
    WinAppMigratorApp,
};

const DETAIL_PANEL_WIDTH: f32 = 400.0;
const ROW_HEIGHT: f32 = 36.0;
const COLUMN_GUTTER: f32 = 12.0;
const COLUMN_FRACTIONS: [f32; 5] = [0.38, 0.25, 0.14, 0.13, 0.10];
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

impl WinAppMigratorApp {
    pub(crate) fn draw(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::symmetric(SPACE_XL as i8, SPACE_MD as i8)),
            )
            .show(ui, |ui| self.draw_header(ui, &theme));

        egui::Panel::right("details")
            .exact_size(DETAIL_PANEL_WIDTH)
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
                    (Icon::Sun, "Switch to light theme")
                } else {
                    (Icon::Moon, "Switch to dark theme")
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
                    "Reload the application list",
                    can_reload,
                )
                .clicked()
                {
                    self.start_inventory_load();
                }
                ui.add_space(SPACE_SM);
                let (label, role) = match &self.inventory_state {
                    InventoryState::Loading => ("Loading".to_owned(), Role::Neutral),
                    InventoryState::Loaded => (format!("{} apps", self.apps.len()), Role::Success),
                    InventoryState::Failed(_) => ("Load failed".to_owned(), Role::Danger),
                };
                widgets::status_pill(ui, theme, &label, role);
            });
        });
    }

    // ----- Left: the application list ---------------------------------------

    fn draw_applications(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        self.source_selector(ui, theme);
        ui.add_space(SPACE_MD);

        let search_id = ui.make_persistent_id("app-search");
        widgets::control_row(ui, |ui| {
            let filter_width = 262.0;
            let search_width = (ui.available_width() - filter_width - SPACE_SM).max(160.0);
            widgets::search_field(
                ui,
                theme,
                search_id,
                &mut self.app_search,
                "Search by name or publisher  (Ctrl+F)",
                search_width,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                widgets::segmented(
                    ui,
                    theme,
                    &mut self.app_scope_filter,
                    &[
                        ("All", ScopeFilter::All),
                        ("Per-user", ScopeFilter::PerUser),
                        ("Per-machine", ScopeFilter::PerMachine),
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
                        "No matching applications",
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
        }
        let mut chosen = if self.scan_source.is_offline() {
            Source::Drive
        } else {
            Source::ThisPc
        };
        let enabled = !self.is_building();
        widgets::control_row(ui, |ui| {
            widgets::field_label(ui, theme, "Scan source");
            let before = chosen;
            let changed = widgets::segmented(
                ui,
                theme,
                &mut chosen,
                &[
                    ("This PC", Source::ThisPc),
                    ("Connected drive…", Source::Drive),
                ],
                enabled,
            );
            if changed && before != chosen {
                match chosen {
                    Source::ThisPc => self.select_live_source(),
                    Source::Drive => self.pick_offline_drive(),
                }
            }
            if let ScanSource::OfflineVolume { volume_root } = &self.scan_source {
                let drive = volume_root.display().to_string();
                ui.add_space(SPACE_XS);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), Sense::hover());
                widgets::paint_icon(ui.painter(), Icon::Drive, rect, theme.text_muted);
                ui.label(RichText::new(drive).size(13.0).color(theme.text_muted));
                if widgets::button(ui, theme, ButtonKind::Ghost, "Change…", enabled).clicked() {
                    self.pick_offline_drive();
                }
            }
        });
        if let Some(error) = self.source_error.clone() {
            ui.add_space(SPACE_SM);
            widgets::notice(ui, theme, Role::Danger, &error);
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

    /// Select an app and prefill the default output directory for it. Selection is
    /// locked during a build: the build and its result belong to the app it started
    /// with.
    pub(crate) fn select_app(&mut self, index: usize) {
        if self.is_building() {
            return;
        }
        self.selected_app_index = Some(index);
        self.output_dir = crate::default_output_for(&self.apps[index].display_name);
        self.start_profile_lookup(self.apps[index].clone());
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
                    widgets::empty_state(
                        ui,
                        theme,
                        Icon::Apps,
                        "Pick an application",
                        "Select an app from the list to choose what to include and build its \
                         installer.",
                    );
                });
            return;
        };

        egui::Panel::bottom("details-actions")
            .frame(
                egui::Frame::new()
                    .fill(theme.surface)
                    .inner_margin(Margin::same(SPACE_LG as i8)),
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
                            draw_app_title(ui, theme, &app);
                            // The result goes first: it is what the user is waiting
                            // for, and it holds the installer's path and folder button.
                            self.draw_build_result(ui, theme);
                            ui.add_space(SPACE_MD);
                            draw_app_facts(ui, theme, &app);
                            ui.add_space(SPACE_LG);
                            self.draw_build_options(ui, theme);
                        });
                });
            });
    }

    fn draw_build_options(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let enabled = !self.is_building();
        widgets::field_label(ui, theme, "Include in the installer");
        ui.add_space(2.0);
        let mut always = true;
        widgets::option_row(
            ui,
            theme,
            &mut always,
            "Program files and registry",
            "Always included, with services, tasks, shortcuts, and runtimes.",
            false,
        );
        widgets::option_row(
            ui,
            theme,
            &mut self.backup_license,
            "License and activation data",
            "May be tied to this PC and need reactivation on the new one.",
            enabled,
        );
        widgets::option_row(
            ui,
            theme,
            &mut self.backup_user_data,
            "User data",
            "Settings, Documents, Saved Games, and ProgramData.",
            enabled,
        );
        self.draw_profile_choice(ui, theme, enabled);

        ui.add_space(SPACE_MD);
        widgets::path_field(
            ui,
            theme,
            "Output folder",
            &mut self.output_dir,
            r"C:\Users\You\WinAppMigrator",
            enabled,
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
        widgets::field_label(ui, theme, "Whose data");
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
            if let Some(room) = self.build_room.as_ref() {
                ui.add_space(SPACE_XS);
                widgets::hint(ui, theme, &format!("Needs {}", room.summary()));
                for note in room.could_not_tell() {
                    widgets::hint(ui, theme, &note);
                }
            }
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
        let can_build = self.stub_available && self.output_dir_is_valid();
        // Once an installer exists, the result is the next thing to act on.
        let (kind, label) = if matches!(self.migrate_state, OperationState::Completed) {
            (ButtonKind::Secondary, "Create installer again")
        } else {
            (ButtonKind::Primary, "Create installer")
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
                    "Copy error",
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
                metric("Files", package.files_copied.to_string(), Role::Accent),
                metric(
                    "Installer size",
                    widgets::format_bytes(*installer_bytes),
                    Role::Accent,
                ),
                metric(
                    "Warnings",
                    package.warnings.len().to_string(),
                    if package.warnings.is_empty() {
                        Role::Neutral
                    } else {
                        Role::Warning
                    },
                ),
            ],
        );

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
                widgets::mono_path(ui, theme, "Data file", &data_file.display().to_string());
            }
            ui.add_space(SPACE_SM);
            widgets::control_row(ui, |ui| {
                if widgets::icon_text_button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    Icon::Folder,
                    "Open folder",
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
                    "Copy path",
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

        warning_list(ui, theme, "Verification errors", &verification.errors, true);
        // When files were lost, the list of which ones is the next thing to read.
        warning_list(
            ui,
            theme,
            "Warnings",
            &package.warnings,
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

fn draw_app_title(ui: &mut egui::Ui, theme: &Theme, app: &InstalledApp) {
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
    });
}

fn draw_app_facts(ui: &mut egui::Ui, theme: &Theme, app: &InstalledApp) {
    widgets::kv_row(
        ui,
        theme,
        "Publisher",
        app.publisher.as_deref().unwrap_or("Unknown"),
    );
    widgets::kv_row(
        ui,
        theme,
        "Version",
        app.version.as_deref().unwrap_or("Unknown"),
    );
    if let Some(location) = app.install_location.as_ref() {
        ui.add_space(SPACE_SM);
        widgets::mono_path(
            ui,
            theme,
            "Install location",
            &location.display().to_string(),
        );
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
    let missed = outcome
        .package
        .warnings
        .iter()
        .filter(|warning| warning.code == "file_copy_failed")
        .count();
    if !outcome.verification.package_valid {
        (
            Role::Danger,
            "Do not use this installer: it failed its hash check.".to_owned(),
        )
    } else if missed > 0 {
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

fn warning_list(ui: &mut egui::Ui, theme: &Theme, title: &str, warnings: &[Warning], open: bool) {
    if warnings.is_empty() {
        return;
    }
    ui.add_space(SPACE_SM);
    egui::CollapsingHeader::new(
        RichText::new(format!("{title} ({})", warnings.len()))
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
                &format!("and {} more.", warnings.len() - MAX_LISTED_WARNINGS),
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
        None | Some(BuildProgress::Build(BuildStep::Scanning)) => {
            ("Reverse-engineering the app…".to_owned(), None)
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
            format!("Reading and compressing files: {done} of {total}"),
            (*bytes_total > 0).then(|| *bytes_done as f32 / *bytes_total as f32),
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
    for (cell, (text, color, strong)) in column_rects(rect).into_iter().zip(cells) {
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
    response.on_hover_text(app.display_name.as_str())
}

/// The cell rectangles of a table row, with a gutter between columns.
fn column_rects(rect: egui::Rect) -> [egui::Rect; 5] {
    let usable = rect.width() - COLUMN_GUTTER;
    let mut left = rect.left() + COLUMN_GUTTER;
    COLUMN_FRACTIONS.map(|fraction| {
        let width = usable * fraction;
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

fn arch_label(arch: &AppArchitecture) -> &'static str {
    match arch {
        AppArchitecture::X86 => "x86",
        AppArchitecture::X64 => "x64",
        AppArchitecture::Arm64 => "ARM64",
        AppArchitecture::Unknown => "Unknown",
    }
}

fn scope_label(scope: &InstallScope) -> &'static str {
    match scope {
        InstallScope::PerUser => "Per-user",
        InstallScope::PerMachine => "Per-machine",
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
