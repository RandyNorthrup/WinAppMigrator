//! What will go in the installer, shown before anything is built, on the one screen.
//!
//! Lists use `ScrollArea::show_rows`, so a list of any length only draws the rows in view, and no
//! list has a row per file: a folder is as fine as a choice goes.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use eframe::egui::{self, RichText};

use winappmigrator_core::space::build_needs;
use winappmigrator_core::{
    ChoiceId, FolderGroup, FolderRow, Footprint, IntegrationRow, ItemRow, NearMatch, NearMatchKind,
    RoomCheck, ScanStep, SystemChange,
};

use crate::theme::{self, Role, SPACE_LG, SPACE_SM, SPACE_XS, Theme};
use crate::widgets::{self, ButtonKind, LIST_ROW_HEIGHT, metric};
use crate::{ListGroup, ScanState, WinAppMigratorApp};

/// How many rows of a list are in view before it scrolls by itself.
const MOST_ROWS_IN_VIEW: usize = 6;
/// How long ago a look at an app has to have been, before it is worth saying.
const LONG_AGO: Duration = Duration::from_secs(10 * 60);

fn plural(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

fn scan_step_text(step: Option<ScanStep>) -> &'static str {
    match step {
        None => "Getting ready to look at this app…",
        Some(ScanStep::InstallRecord) => "Reading how it was installed…",
        Some(ScanStep::NearMatches) => "Looking for what might also be part of it…",
        Some(ScanStep::SystemItems) => "Looking for its services, tasks, and shortcuts…",
        Some(ScanStep::Files) => "Going through its files…",
        Some(ScanStep::Registry) => "Reading what ties it into Windows…",
        Some(ScanStep::Finishing) => "Finishing up…",
    }
}

/// A list of rows of one height. A short one is simply there, in the panel's own scrolling. A long
/// one scrolls by itself, with only the rows in view drawn, however long it is. `row` draws one and
/// says whether it just got the focus.
fn list_of_rows(
    ui: &mut egui::Ui,
    id: &str,
    count: usize,
    mut row: impl FnMut(&mut egui::Ui, usize) -> bool,
) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        if count <= MOST_ROWS_IN_VIEW {
            for index in 0..count {
                row(ui, index);
            }
            return;
        }
        let mut gained_focus = false;
        // By how many rows, since inside the panel's own scrolling, how much room is left says more
        // about how far that has scrolled than about anything to do with this.
        let height = MOST_ROWS_IN_VIEW as f32 * LIST_ROW_HEIGHT;
        let output = egui::ScrollArea::vertical()
            .id_salt(id)
            .auto_shrink([false, true])
            .min_scrolled_height(height)
            .max_height(height)
            .show_rows(ui, LIST_ROW_HEIGHT, count, |ui, range| {
                for index in range {
                    gained_focus |= row(ui, index);
                }
            });
        // A row brought into view by the keyboard is only in view of this list. The panel takes its
        // own scrolling from what is asked of it after this list has had its turn.
        if gained_focus {
            ui.scroll_to_rect(output.inner_rect, None);
        }
    });
}

impl WinAppMigratorApp {
    fn is_open(&self, group: ListGroup) -> bool {
        self.open_groups.contains(&group)
    }

    /// A heading for part of the list, which says what is in it and opens or closes it.
    fn group_heading(
        &mut self,
        ui: &mut egui::Ui,
        theme: &Theme,
        group: ListGroup,
        title: &str,
        summary: &str,
    ) -> bool {
        let open = self.is_open(group);
        if widgets::disclosure_row(ui, theme, open, title, summary).clicked() {
            if open {
                self.open_groups.remove(&group);
            } else {
                self.open_groups.insert(group);
            }
        }
        self.is_open(group)
    }

    /// Whether what can be chosen can be chosen now.
    fn can_choose(&self) -> bool {
        !self.is_building() && self.ready_scan().is_some()
    }

    /// What went with something left out, if this did.
    fn why_also_left_out(&self, id: &ChoiceId) -> Option<&str> {
        self.also
            .items
            .iter()
            .find(|item| item.id == *id)
            .map(|item| item.because.as_str())
    }

    // ----- How big, and whether it fits ---------------------------------------

    pub(crate) fn draw_footprint(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        self.draw_look_card(ui, theme);
        ui.add_space(SPACE_SM);
        self.draw_options(ui, theme);
        if let Some(footprint) = self.shown_footprint() {
            ui.add_space(SPACE_SM);
            self.draw_groups(ui, theme, &footprint);
        }
    }

    /// First on the screen: how far looking at the app has got, then how big an installer can
    /// be and whether it fits, or why that cannot be said.
    fn draw_look_card(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        enum Look {
            Waiting,
            Running(Option<ScanStep>),
            Ready,
            Failed(String),
            Stopped,
        }
        let look = match &self.scan {
            ScanState::None => Look::Waiting,
            ScanState::Running { step, .. } => Look::Running(*step),
            ScanState::Ready(_) => Look::Ready,
            ScanState::Failed { message, .. } => Look::Failed(message.clone()),
            ScanState::Stopped { .. } => Look::Stopped,
        };
        // Beside its heading, for looking again at what has been looked at.
        let can_look_again = matches!(look, Look::Ready) && !self.is_building();
        widgets::control_row(ui, |ui| {
            widgets::field_label(ui, theme, "What goes in the installer");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if can_look_again
                    && widgets::button(ui, theme, ButtonKind::Ghost, "Look again", true).clicked()
                {
                    self.scan_again();
                }
            });
        });
        match look {
            Look::Waiting => {
                widgets::hint(ui, theme, scan_step_text(None));
            }
            Look::Running(step) => {
                widgets::control_row(ui, |ui| {
                    ui.spinner();
                    ui.add_space(SPACE_XS);
                    ui.label(
                        RichText::new(scan_step_text(step))
                            .size(13.0)
                            .color(theme.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::button(ui, theme, ButtonKind::Ghost, "Stop", true).clicked() {
                            self.stop_scan();
                        }
                    });
                });
                ui.add_space(SPACE_XS);
                widgets::progress_bar(ui, theme, None);
                ui.add_space(SPACE_XS);
                widgets::hint(
                    ui,
                    theme,
                    "You can pick another app or change what is included while this runs.",
                );
            }
            Look::Ready => self.draw_size_and_room(ui, theme),
            Look::Failed(message) => {
                widgets::notice(
                    ui,
                    theme,
                    Role::Danger,
                    &format!("This app could not be looked at. {message}"),
                );
                ui.add_space(SPACE_SM);
                if widgets::button(ui, theme, ButtonKind::Secondary, "Try again", true).clicked() {
                    self.scan_again();
                }
            }
            Look::Stopped => {
                widgets::notice(
                    ui,
                    theme,
                    Role::Neutral,
                    "Stopped. An installer can be built once this app has been looked at.",
                );
                ui.add_space(SPACE_SM);
                if widgets::button(ui, theme, ButtonKind::Secondary, "Look again", true).clicked() {
                    self.scan_again();
                }
            }
        }
    }

    fn draw_size_and_room(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let Some(inputs) = self.build_inputs() else {
            return;
        };
        let room = self
            .build_room
            .clone()
            .filter(|_| self.is_building())
            .or_else(|| self.room_estimate());
        let free = room.as_ref().and_then(|room| {
            room.drives
                .first()
                .map(|drive| (drive.drive.clone(), drive.free_bytes))
        });
        // How big the installer can come to, which is what a person is asking. What it takes to
        // build one is more, and is only worth saying when that is why it will not fit.
        let needed = build_needs(&inputs).installer_bytes;
        let (free_label, free_text) = match free {
            Some((drive, Some(bytes))) => {
                (format!("Free on {drive}"), widgets::format_bytes(bytes))
            }
            Some((drive, None)) => (format!("Free on {drive}"), "Unknown".to_owned()),
            None => ("Free".to_owned(), "Unknown".to_owned()),
        };
        let fits = room
            .as_ref()
            .is_none_or(|room| room.shortfalls().is_empty());
        widgets::metric_tiles(
            ui,
            theme,
            &[
                metric("Files", inputs.payload_files.to_string(), Role::Accent),
                metric("Up to", widgets::format_bytes(needed), Role::Accent),
                metric(
                    free_label,
                    free_text,
                    if fits { Role::Neutral } else { Role::Danger },
                ),
            ],
        );
        ui.add_space(SPACE_XS);
        widgets::hint(ui, theme, "The most it could need, before compressing.");
        if let Some(room) = room.as_ref() {
            self.draw_room_problems(ui, theme, room);
        }
        if !self.also_is_current() {
            ui.add_space(SPACE_XS);
            widgets::hint(ui, theme, "Working out what goes with that…");
        }
        if let Some(error) = self.also.error.clone() {
            ui.add_space(SPACE_SM);
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                &format!("What is left would not make a package: {error}"),
            );
        }
        self.draw_also_left_out(ui, theme);
        self.draw_licenses_that_still_go(ui, theme);
        self.draw_age_of_look(ui, theme);
    }

    fn draw_room_problems(&self, ui: &mut egui::Ui, theme: &Theme, room: &RoomCheck) {
        // In the units of the rest of the screen, which is why this is not what a build says.
        let shortfalls: Vec<String> = room
            .drives
            .iter()
            .filter_map(|drive| {
                let free = drive.free_bytes?;
                (free < drive.needed_bytes).then(|| {
                    format!(
                        "{} needs up to {} free to build this, and has {}",
                        drive.drive,
                        widgets::format_bytes(drive.needed_bytes),
                        widgets::format_bytes(free)
                    )
                })
            })
            .collect();
        if !shortfalls.is_empty() {
            ui.add_space(SPACE_SM);
            widgets::notice(
                ui,
                theme,
                Role::Danger,
                &format!(
                    "There is not enough room. {}. Choose an output folder on another drive, or \
                     leave something out.",
                    shortfalls.join(". ")
                ),
            );
        }
        for note in room.could_not_tell() {
            ui.add_space(SPACE_XS);
            widgets::hint(ui, theme, &format!("{note}."));
        }
    }

    /// Said plainly, in the words of whatever went, as soon as it is known.
    fn draw_also_left_out(&self, ui: &mut egui::Ui, theme: &Theme) {
        if self.also.items.is_empty() {
            return;
        }
        let names: Vec<&str> = self
            .also
            .items
            .iter()
            .take(6)
            .map(|item| item.title.as_str())
            .collect();
        let more = self.also.items.len().saturating_sub(names.len());
        let list = if more > 0 {
            format!("{}, and {more} more", names.join(", "))
        } else {
            names.join(", ")
        };
        ui.add_space(SPACE_SM);
        widgets::notice(
            ui,
            theme,
            Role::Warning,
            &format!(
                "That also leaves out {list}, because what they run or name would not be there. They \
                 will not be set up on the other PC."
            ),
        );
    }

    /// A license goes by the License option, wherever it is, and it should not be a surprise.
    fn draw_licenses_that_still_go(&self, ui: &mut egui::Ui, theme: &Theme) {
        let Some(footprint) = self.shown_footprint() else {
            return;
        };
        let names: Vec<String> = footprint
            .licenses_in_what_is_left_out(&self.choices)
            .iter()
            .take(4)
            .map(|path| {
                path.file_name().map_or_else(
                    || path.display().to_string(),
                    |name| name.to_string_lossy().into_owned(),
                )
            })
            .collect();
        if names.is_empty() {
            return;
        }
        ui.add_space(SPACE_XS);
        widgets::hint(
            ui,
            theme,
            &format!(
                "License data in what you left out still goes in, because that goes by the License \
                 option: {}.",
                names.join(", ")
            ),
        );
    }

    fn draw_age_of_look(&self, ui: &mut egui::Ui, theme: &Theme) {
        let age = match &self.scan {
            ScanState::Ready(ready) => ready.finished.elapsed(),
            _ => return,
        };
        if age >= LONG_AGO {
            ui.add_space(SPACE_XS);
            widgets::hint(
                ui,
                theme,
                &format!(
                    "This was looked at {} ago. Files are read when you build, so a change is \
                     picked up, and anything gone by then is listed. Look again for anything new.",
                    widgets::format_elapsed(age.as_secs())
                ),
            );
        }
    }

    // ----- What it holds ------------------------------------------------------

    fn draw_groups(&mut self, ui: &mut egui::Ui, theme: &Theme, footprint: &Arc<Footprint>) {
        for note in &footprint.notes {
            widgets::notice(ui, theme, Role::Warning, note);
            ui.add_space(SPACE_SM);
        }
        self.draw_folder_group(ui, theme, footprint, FolderGroup::Program);
        self.draw_folder_group(ui, theme, footprint, FolderGroup::Data);
        self.draw_item_group(
            ui,
            theme,
            ListGroup::Services,
            "Services",
            &footprint.services,
        );
        self.draw_item_group(
            ui,
            theme,
            ListGroup::Startup,
            "Starts automatically",
            &footprint.startup,
        );
        self.draw_item_group(
            ui,
            theme,
            ListGroup::Tasks,
            "Scheduled tasks",
            &footprint.tasks,
        );
        self.draw_item_group(
            ui,
            theme,
            ListGroup::Shortcuts,
            "Shortcuts",
            &footprint.shortcuts,
        );
        self.draw_integrations(ui, theme, footprint);
        self.draw_prerequisites(ui, theme, footprint);
        self.draw_near_matches(ui, theme, footprint);
        draw_none_found(ui, theme, footprint);
        ui.add_space(SPACE_SM);
        self.draw_left_out(ui, theme, footprint);
    }

    fn draw_folder_group(
        &mut self,
        ui: &mut egui::Ui,
        theme: &Theme,
        footprint: &Footprint,
        which: FolderGroup,
    ) {
        let rows: Vec<&FolderRow> = footprint
            .folders
            .iter()
            .filter(|row| row.group == which)
            .collect();
        let (group, title) = match which {
            FolderGroup::Program => (ListGroup::Program, "Program files"),
            FolderGroup::Data => (ListGroup::Data, "Data folders"),
        };
        if rows.is_empty()
            && (which == FolderGroup::Program || footprint.data_not_included_files == 0)
        {
            return;
        }
        let bytes = rows
            .iter()
            .filter(|row| footprint.folder_is_in(row, &self.choices))
            .fold(0_u64, |total, row| total.saturating_add(row.bytes));
        let included = rows
            .iter()
            .filter(|row| footprint.folder_is_in(row, &self.choices))
            .count();
        let summary = format!(
            "{} of {}, {}",
            included,
            plural(rows.len(), "folder", "folders"),
            widgets::format_bytes(bytes)
        );
        if !self.group_heading(ui, theme, group, title, &summary) {
            return;
        }
        if which == FolderGroup::Data && footprint.data_not_included_files > 0 {
            widgets::hint(
                ui,
                theme,
                &format!(
                    "{} in {} of this app's data will only go in with the User data option.",
                    widgets::format_bytes(footprint.data_not_included_bytes),
                    plural(footprint.data_not_included_files, "file", "files"),
                ),
            );
        }
        let enabled = self.can_choose();
        list_of_rows(ui, title, rows.len(), |ui, index| {
            let row = rows[index];
            let mut included = footprint.folder_is_in(row, &self.choices);
            let detail = match row.locked {
                Some(why) => format!("{}. {why}", plural(row.files, "file", "files")),
                None => format!("{}. {}", plural(row.files, "file", "files"), row.place),
            };
            let trailing = widgets::format_bytes(row.bytes);
            let response = widgets::choice_row(
                ui,
                theme,
                &mut included,
                &row.name,
                &detail,
                Some(&trailing),
                enabled && row.locked.is_none(),
            )
            .on_hover_text(row.path.display().to_string());
            if response.changed() {
                self.set_included(&row.id, included);
            }
            response.gained_focus()
        });
        ui.add_space(SPACE_XS);
    }

    fn draw_item_group(
        &mut self,
        ui: &mut egui::Ui,
        theme: &Theme,
        group: ListGroup,
        title: &str,
        rows: &[ItemRow],
    ) {
        if rows.is_empty() {
            return;
        }
        let included = rows
            .iter()
            .filter(|row| {
                !self.choices.is_left_out(&row.id) && self.why_also_left_out(&row.id).is_none()
            })
            .count();
        let summary = format!("{included} of {}", rows.len());
        if !self.group_heading(ui, theme, group, title, &summary) {
            return;
        }
        let enabled = self.can_choose();
        list_of_rows(ui, title, rows.len(), |ui, index| {
            let row = &rows[index];
            let also = self.why_also_left_out(&row.id).map(ToOwned::to_owned);
            let mut included = also.is_none() && !self.choices.is_left_out(&row.id);
            let detail = match also.as_deref() {
                Some(why) => format!("Left out too: {why}"),
                None => change_detail(&row.change),
            };
            let response = widgets::choice_row(
                ui,
                theme,
                &mut included,
                &row.change.title,
                &detail,
                None,
                enabled && also.is_none(),
            );
            if response.changed() {
                self.set_included(&row.id, included);
            }
            response.gained_focus()
        });
        ui.add_space(SPACE_XS);
    }

    fn draw_integrations(&mut self, ui: &mut egui::Ui, theme: &Theme, footprint: &Footprint) {
        let groups = &footprint.integrations;
        if groups.is_empty() {
            return;
        }
        let included = groups
            .iter()
            .filter(|group| !self.choices.is_left_out(&group.id))
            .count();
        let summary = format!("{included} of {}", plural(groups.len(), "kind", "kinds"));
        if !self.group_heading(
            ui,
            theme,
            ListGroup::Integrations,
            "What ties it into Windows",
            &summary,
        ) {
            return;
        }
        // A row for each kind, which can be left out, and under it a row for each thing of that kind
        // while it is in, as it stands after what else was left out.
        let current = self.also.integrations.clone();
        let changes_of = |group: &'_ IntegrationRow| -> Vec<SystemChange> {
            current.as_ref().map_or_else(
                || group.changes.clone(),
                |rows| {
                    rows.iter()
                        .find(|row| row.kind == group.kind)
                        .map_or_else(Vec::new, |row| row.changes.clone())
                },
            )
        };
        let mut rows: Vec<(usize, Option<SystemChange>)> = Vec::new();
        let mut counts: Vec<usize> = Vec::with_capacity(groups.len());
        for (index, group) in groups.iter().enumerate() {
            rows.push((index, None));
            let changes = changes_of(group);
            counts.push(changes.len());
            if !self.choices.is_left_out(&group.id) {
                rows.extend(changes.into_iter().map(|change| (index, Some(change))));
            }
        }
        let enabled = self.can_choose();
        list_of_rows(ui, "integrations", rows.len(), |ui, index| {
            let (group_index, change) = &rows[index];
            let group: &IntegrationRow = &groups[*group_index];
            match change {
                Some(change) => {
                    widgets::plain_row(ui, theme, &change.title, &change.detail, None);
                    false
                }
                None => {
                    let mut included = !self.choices.is_left_out(&group.id);
                    let detail = match counts[*group_index] {
                        0 => "Nothing left of it, because of what you left out".to_owned(),
                        count => plural(count, "thing set up", "things set up"),
                    };
                    let response = widgets::choice_row(
                        ui,
                        theme,
                        &mut included,
                        group.kind.label(),
                        &detail,
                        None,
                        enabled,
                    );
                    if response.changed() {
                        self.set_included(&group.id, included);
                    }
                    response.gained_focus()
                }
            }
        });
        ui.add_space(SPACE_XS);
    }

    fn draw_prerequisites(&mut self, ui: &mut egui::Ui, theme: &Theme, footprint: &Footprint) {
        let rows = &footprint.prerequisites;
        if rows.is_empty() {
            return;
        }
        let summary = plural(rows.len(), "runtime", "runtimes");
        if !self.group_heading(
            ui,
            theme,
            ListGroup::Prerequisites,
            "What it needs to run",
            &summary,
        ) {
            return;
        }
        list_of_rows(ui, "prerequisites", rows.len(), |ui, index| {
            let row = &rows[index];
            widgets::plain_row(ui, theme, &row.name, &row.handling, None);
            false
        });
        ui.add_space(SPACE_XS);
    }

    fn draw_near_matches(&mut self, ui: &mut egui::Ui, theme: &Theme, footprint: &Footprint) {
        let rows = &footprint.near_matches;
        if rows.is_empty() {
            return;
        }
        let ticked = rows
            .iter()
            .filter(|near| {
                // One waiting for the User data option is not in, whatever its box says.
                self.near_picks.contains_key(&near.id)
                    && (!near.follows_user_data || self.backup_user_data)
            })
            .count();
        let summary = format!("{ticked} of {} added", rows.len());
        if !self.group_heading(
            ui,
            theme,
            ListGroup::NearMatches,
            "Might also be part of it",
            &summary,
        ) {
            return;
        }
        widgets::hint(
            ui,
            theme,
            "Not in the installer unless you tick them. Ticking one looks at the app again, with \
             the same checks as the rest, so what looks like a password still stays out.",
        );
        let enabled = self.can_choose();
        let with_user_data = self.backup_user_data;
        list_of_rows(ui, "near-matches", rows.len(), |ui, index| {
            let near = &rows[index];
            self.draw_near_match(ui, theme, near, enabled, with_user_data)
        });
        ui.add_space(SPACE_XS);
    }

    fn draw_near_match(
        &mut self,
        ui: &mut egui::Ui,
        theme: &Theme,
        near: &NearMatch,
        enabled: bool,
        with_user_data: bool,
    ) -> bool {
        let mut ticked = self.near_picks.contains_key(&near.id);
        let usable = with_user_data || !near.follows_user_data;
        let detail = if usable {
            near.why.clone()
        } else {
            format!("Comes with the User data option. {}", near.why)
        };
        let size = if near.measured_all {
            widgets::format_bytes(near.total_bytes)
        } else {
            format!("over {}", widgets::format_bytes(near.total_bytes))
        };
        let kind = match near.kind {
            NearMatchKind::Folder => "Folder",
            NearMatchKind::HiddenEntry => "Hidden part of an app",
        };
        let place = near
            .path
            .as_deref()
            .map_or_else(String::new, |path: &Path| path.display().to_string());
        let response = widgets::choice_row(
            ui,
            theme,
            &mut ticked,
            &near.name,
            &detail,
            Some(&size),
            enabled && usable,
        )
        .on_hover_text(format!("{kind}: {place}"));
        if response.changed() {
            if ticked {
                self.near_picks
                    .insert(near.id.clone(), near.follows_user_data);
            } else {
                self.near_picks.remove(&near.id);
            }
        }
        response.gained_focus()
    }

    // ----- What stays out -----------------------------------------------------

    fn draw_left_out(&mut self, ui: &mut egui::Ui, theme: &Theme, footprint: &Footprint) {
        let named: usize = footprint
            .left_out
            .iter()
            .map(|group| group.items.len())
            .sum();
        let refused = footprint.refused.len();
        let warnings = footprint.warnings.len();
        if named + refused + warnings == 0 {
            widgets::hint(
                ui,
                theme,
                "Nothing was left out for looking like a password, a license, or a certificate.",
            );
            return;
        }
        if named + refused > 0 {
            let summary = plural(named + refused, "thing", "things");
            if self.group_heading(ui, theme, ListGroup::LeftOut, "Left out, and why", &summary) {
                for group in &footprint.left_out {
                    widgets::hint(ui, theme, &format!("{}:", group.title));
                    ui.add_space(SPACE_XS);
                    list_of_rows(ui, &group.title, group.items.len(), |ui, index| {
                        let item = &group.items[index];
                        widgets::plain_row(ui, theme, &item.name, &item.why, None);
                        false
                    });
                    ui.add_space(SPACE_SM);
                }
                if refused > 0 {
                    widgets::hint(ui, theme, "No installer would be allowed to set these up:");
                    ui.add_space(SPACE_XS);
                    list_of_rows(ui, "refused", refused, |ui, index| {
                        let sentence = &footprint.refused[index];
                        let (title, why) = sentence
                            .split_once(" was left out: ")
                            .unwrap_or((sentence.as_str(), ""));
                        widgets::plain_row(ui, theme, title, why, None);
                        false
                    });
                }
            }
        }
        if warnings > 0 {
            crate::views::warning_list(ui, theme, "Worth knowing", &footprint.warnings, false);
        }
        ui.add_space(SPACE_LG);
    }
}

/// What a row says of something on the installer's list, in that list's words.
fn change_detail(change: &SystemChange) -> String {
    format!("{}. {}", change.kind.label(), change.detail)
}

/// Saying so, rather than having a person wonder whether they were looked for.
fn draw_none_found(ui: &mut egui::Ui, theme: &Theme, footprint: &Footprint) {
    let mut none = Vec::new();
    if footprint.services.is_empty() {
        none.push("services");
    }
    if footprint.startup.is_empty() {
        none.push("startup items");
    }
    if footprint.tasks.is_empty() {
        none.push("scheduled tasks");
    }
    if footprint.shortcuts.is_empty() {
        none.push("shortcuts");
    }
    if footprint.integrations.is_empty() {
        none.push("anything that ties it into Windows");
    }
    if none.is_empty() {
        return;
    }
    ui.add_space(SPACE_XS);
    ui.label(
        RichText::new(format!("None found: {}.", none.join(", ")))
            .font(theme::regular(12.5))
            .color(theme.text_muted),
    );
}
