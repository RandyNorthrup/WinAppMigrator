//! The single application screen: browse installed apps, pick one, choose
//! whether to also back up its license and user data, and build a
//! self-extracting installer that reinstalls it on another device.

use eframe::egui::{self, Align2, RichText, Sense};

use winappmigrator_core::{AppArchitecture, InstallScope, Warning, WarningSeverity};

use crate::theme::{Role, Theme};
use crate::widgets::{self, ButtonKind, Icon, button, metric};
use crate::{InventoryState, MigrationOutcome, OperationState, ScopeFilter, WinAppMigratorApp};

impl WinAppMigratorApp {
    pub(crate) fn view_applications(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        // Scan source: this PC or a connected drive from another PC.
        self.source_selector(ui, &theme);
        ui.add_space(12.0);

        // Search + scope filter.
        widgets::card(ui, &theme, |ui| {
            ui.horizontal(|ui| {
                let search_width = (ui.available_width() - 330.0).max(160.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.app_search)
                        .hint_text("Search by name or publisher")
                        .desired_width(search_width),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    segmented(
                        ui,
                        &theme,
                        &mut self.app_scope_filter,
                        &[
                            ("All", ScopeFilter::All),
                            ("Per-user", ScopeFilter::PerUser),
                            ("Per-machine", ScopeFilter::PerMachine),
                        ],
                    );
                });
            });
        });

        ui.add_space(12.0);

        match &self.inventory_state {
            InventoryState::Loading => {
                widgets::card(ui, &theme, |ui| {
                    busy_row(ui, &theme, "Loading installed applications…")
                });
                ui.ctx().request_repaint();
                return;
            }
            InventoryState::Failed(error) => {
                let error = error.clone();
                widgets::card(ui, &theme, |ui| {
                    widgets::notice(
                        ui,
                        &theme,
                        Role::Danger,
                        &format!("Inventory failed: {error}"),
                    );
                });
                return;
            }
            InventoryState::Loaded => {}
        }

        let query = self.app_search.trim().to_lowercase();
        let scope_filter = self.app_scope_filter;
        let filtered: Vec<usize> = self
            .apps
            .iter()
            .enumerate()
            .filter(|(_, app)| scope_matches(scope_filter, &app.install_scope))
            .filter(|(_, app)| {
                if query.is_empty() {
                    return true;
                }
                app.display_name.to_lowercase().contains(&query)
                    || app
                        .publisher
                        .as_deref()
                        .map(|publisher| publisher.to_lowercase().contains(&query))
                        .unwrap_or(false)
            })
            .map(|(index, _)| index)
            .collect();

        widgets::card(ui, &theme, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Installed applications")
                        .size(15.0)
                        .strong()
                        .color(theme.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} shown", filtered.len()))
                            .size(12.0)
                            .color(theme.text_muted),
                    );
                });
            });
            ui.add_space(8.0);
            app_table_header(ui, &theme);
            ui.add_space(2.0);

            if filtered.is_empty() {
                widgets::empty_state(
                    ui,
                    &theme,
                    Icon::Apps,
                    "No matching applications",
                    "Adjust the search text or scope filter.",
                );
                return;
            }

            ui.spacing_mut().item_spacing.y = 3.0;
            egui::ScrollArea::vertical()
                .id_salt("apps-table")
                .auto_shrink([false, false])
                .max_height(360.0)
                .show_rows(ui, 32.0, filtered.len(), |ui, range| {
                    for row in range {
                        let app_index = filtered[row];
                        let selected = self.selected_app_index == Some(app_index);
                        if app_table_row(ui, &theme, &self.apps[app_index], selected).clicked() {
                            self.select_app(app_index);
                        }
                    }
                });
        });

        if let Some(index) = self.selected_app_index
            && index < self.apps.len()
        {
            ui.add_space(12.0);
            self.migrate_panel(ui, &theme, index);
        }
    }

    /// The scan-source selector: read installed apps from this PC, or from a
    /// drive connected from another (often failing) PC.
    fn source_selector(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let offline = self.scan_source.is_offline();
        let offline_root = match &self.scan_source {
            winappmigrator_core::ScanSource::OfflineVolume { volume_root } => {
                Some(volume_root.display().to_string())
            }
            winappmigrator_core::ScanSource::Live => None,
        };
        widgets::card(ui, theme, |ui| {
            ui.horizontal(|ui| {
                widgets::field_label(ui, theme, "Scan source");
                ui.add_space(6.0);
                if button(
                    ui,
                    theme,
                    if offline {
                        ButtonKind::Ghost
                    } else {
                        ButtonKind::Primary
                    },
                    "This PC",
                    true,
                )
                .clicked()
                {
                    self.select_live_source();
                }
                if button(
                    ui,
                    theme,
                    if offline {
                        ButtonKind::Primary
                    } else {
                        ButtonKind::Ghost
                    },
                    "Connected drive…",
                    true,
                )
                .clicked()
                {
                    self.pick_offline_drive();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (label, role) = if offline {
                        ("Offline drive", Role::Warning)
                    } else {
                        ("This PC", Role::Success)
                    };
                    widgets::status_pill(ui, theme, label, role);
                });
            });

            if let Some(root) = offline_root.as_ref() {
                ui.add_space(8.0);
                widgets::mono_path(ui, theme, "Drive root", root);
            }

            if let Some(error) = self.source_error.clone() {
                ui.add_space(8.0);
                widgets::notice(ui, theme, Role::Danger, &error);
            }

            if offline {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(
                        "Reads apps, files, registry, services, and scheduled tasks straight from \
                         the drive; the built installer works exactly like a live scan.",
                    )
                    .size(12.0)
                    .color(theme.text_muted),
                );
            }
        });
    }

    /// Select an app and prefill the default output directory for it.
    fn select_app(&mut self, index: usize) {
        self.selected_app_index = Some(index);
        self.output_dir = crate::default_output_for(&self.apps[index].display_name);
        // A fresh selection starts a fresh migration result.
        self.migrate_state = OperationState::Idle;
        self.migrate_detail.clear();
        self.last_outcome = None;
        self.last_error = None;
    }

    /// The migration controls and result for the selected application.
    fn migrate_panel(&mut self, ui: &mut egui::Ui, theme: &Theme, index: usize) {
        let app = self.apps[index].clone();
        let running = matches!(self.migrate_state, OperationState::Running);

        widgets::card(ui, theme, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&app.display_name)
                        .size(17.0)
                        .strong()
                        .color(theme.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    widgets::badge(ui, theme, arch_label(&app.architecture), theme.text_subtle);
                    widgets::status_pill(
                        ui,
                        theme,
                        scope_label(&app.install_scope),
                        scope_role(&app.install_scope),
                    );
                });
            });

            ui.add_space(10.0);
            widgets::kv_row(
                ui,
                theme,
                "Publisher",
                app.publisher.as_deref().unwrap_or("Unknown"),
            );
            ui.add_space(4.0);
            widgets::kv_row(
                ui,
                theme,
                "Version",
                app.version.as_deref().unwrap_or("Unknown"),
            );
            if let Some(location) = app.install_location.as_ref() {
                ui.add_space(8.0);
                widgets::mono_path(
                    ui,
                    theme,
                    "Install location",
                    &location.display().to_string(),
                );
            }

            ui.add_space(14.0);
            widgets::field_label(ui, theme, "Also back up (optional)");
            ui.checkbox(
                &mut self.backup_license,
                "License / activation data (may be machine-bound and need reactivation)",
            );
            ui.checkbox(
                &mut self.backup_user_data,
                "User data — settings, Documents, Saved Games, and ProgramData",
            );

            ui.add_space(12.0);
            widgets::path_field(
                ui,
                theme,
                "Installer output folder",
                &mut self.output_dir,
                "Where the installer is written",
                || rfd::FileDialog::new().pick_folder(),
            );

            ui.add_space(10.0);
            let mut summary =
                "Builds a self-extracting installer that reinstalls the app".to_owned();
            if self.backup_license {
                summary.push_str(", its license");
            }
            if self.backup_user_data {
                summary.push_str(", its user data");
            }
            summary.push_str(" on another device, then hash-verifies fidelity.");
            widgets::notice(ui, theme, Role::Accent, &summary);

            let stub_present = crate::installer_stub_path().is_some();
            if !stub_present {
                ui.add_space(8.0);
                widgets::notice(
                    ui,
                    theme,
                    Role::Danger,
                    "Installer stub (winappmigrator-installer.exe) was not found beside this app, \
                     so a self-extracting installer cannot be built.",
                );
            }

            ui.add_space(14.0);
            if button(
                ui,
                theme,
                ButtonKind::Primary,
                "Create self-extracting installer",
                !running && stub_present,
            )
            .clicked()
            {
                self.start_migration();
            }
        });

        ui.add_space(12.0);

        match self.migrate_state {
            OperationState::Idle => {}
            OperationState::Running => {
                widgets::card(ui, theme, |ui| {
                    busy_row(ui, theme, "Building installer");
                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.migrate_detail).color(theme.text_muted));
                });
                ui.ctx().request_repaint();
            }
            OperationState::Failed => {
                widgets::card(ui, theme, |ui| {
                    widgets::notice(ui, theme, Role::Danger, "Installer build failed.");
                    if let Some(error) = self.last_error.as_ref() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(error).color(theme.text));
                    }
                });
            }
            OperationState::Completed | OperationState::Warning => self.migrate_result(ui, theme),
        }
    }

    fn migrate_result(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folder_error = None;
        if let Some(outcome) = self.last_outcome.as_ref() {
            let MigrationOutcome {
                app_name,
                package,
                verification,
            } = outcome;

            widgets::card_titled(
                ui,
                theme,
                "Installer ready",
                Some(app_name.as_str()),
                |ui| {
                    widgets::metric_tiles(
                        ui,
                        theme,
                        &[
                            metric(
                                "Files captured",
                                package.files_copied.to_string(),
                                Role::Accent,
                            ),
                            metric(
                                "Hashes",
                                if verification.package_valid {
                                    "Verified"
                                } else {
                                    "Failed"
                                },
                                if verification.package_valid {
                                    Role::Success
                                } else {
                                    Role::Danger
                                },
                            ),
                            metric("Checks", verification.items.len().to_string(), Role::Accent),
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

                    ui.add_space(8.0);
                    widgets::status_pill(
                        ui,
                        theme,
                        if verification.package_valid {
                            "Hashes confirmed"
                        } else {
                            "Hash check failed"
                        },
                        if verification.package_valid {
                            Role::Success
                        } else {
                            Role::Danger
                        },
                    );

                    ui.add_space(10.0);
                    if let Some(sfx) = package.self_extracting_archive_path.as_ref() {
                        widgets::mono_path(
                            ui,
                            theme,
                            "Self-extracting installer",
                            &sfx.display().to_string(),
                        );
                    }

                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(
                            "Run this installer on the target PC to reinstall the app and restore \
                             its captured data, then it verifies every file by SHA-256.",
                        )
                        .size(12.0)
                        .color(theme.text_muted),
                    );

                    ui.add_space(12.0);
                    if button(ui, theme, ButtonKind::Secondary, "Open folder", true).clicked() {
                        let target = package
                            .self_extracting_archive_path
                            .as_deref()
                            .and_then(|path| path.parent())
                            .unwrap_or(package.package_root.as_path());
                        if let Err(error) = crate::open_folder(target) {
                            folder_error = Some(error.to_string());
                        }
                    }
                },
            );

            if !verification.errors.is_empty() {
                ui.add_space(12.0);
                widgets::card_titled(
                    ui,
                    theme,
                    &format!("Verification errors · {}", verification.errors.len()),
                    None,
                    |ui| {
                        for error in &verification.errors {
                            warning_row(ui, theme, error);
                        }
                    },
                );
            }

            if !package.warnings.is_empty() {
                ui.add_space(12.0);
                widgets::card_titled(
                    ui,
                    theme,
                    &format!("Warnings · {}", package.warnings.len()),
                    None,
                    |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("migrate-warnings")
                            .max_height(220.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for warning in &package.warnings {
                                    warning_row(ui, theme, warning);
                                }
                            });
                    },
                );
            }
        }

        if let Some(error) = folder_error {
            self.last_error = Some(error);
        }
    }
}

// ----- Free helpers ----------------------------------------------------------

fn busy_row(ui: &mut egui::Ui, theme: &Theme, label: &str) {
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(RichText::new(label).color(theme.text));
    });
}

fn warning_row(ui: &mut egui::Ui, theme: &Theme, warning: &Warning) {
    ui.horizontal(|ui| {
        widgets::badge(
            ui,
            theme,
            &warning.code,
            theme.role(severity_role(&warning.severity)),
        );
        ui.label(RichText::new(&warning.message).color(theme.text));
    });
    ui.add_space(4.0);
}

fn segmented<T: Copy + PartialEq>(
    ui: &mut egui::Ui,
    theme: &Theme,
    current: &mut T,
    options: &[(&str, T)],
) {
    egui::Frame::new()
        .fill(theme.field_bg)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(crate::theme::RADIUS_SM)
        .inner_margin(egui::Margin::same(3))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                for (label, value) in options {
                    let selected = *current == *value;
                    let widget = egui::Button::new(RichText::new(*label).color(if selected {
                        theme.on_accent
                    } else {
                        theme.text_muted
                    }))
                    .fill(if selected {
                        theme.accent
                    } else {
                        egui::Color32::TRANSPARENT
                    })
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(crate::theme::RADIUS_SM - 1)
                    .min_size(egui::vec2(0.0, 26.0));
                    if ui.add(widget).clicked() {
                        *current = *value;
                    }
                }
            });
        });
}

fn app_table_header(ui: &mut egui::Ui, theme: &Theme) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 22.0), Sense::hover());
    let cells = ["NAME", "PUBLISHER", "VERSION", "SCOPE", "ARCH"];
    let offsets = column_offsets(rect);
    let painter = ui.painter();
    for (text, x) in cells.iter().zip(offsets.iter()) {
        painter.text(
            egui::pos2(*x, rect.center().y),
            Align2::LEFT_CENTER,
            text,
            egui::FontId::proportional(11.0),
            theme.text_subtle,
        );
    }
}

fn app_table_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    app: &winappmigrator_core::InstalledApp,
    selected: bool,
) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, crate::theme::RADIUS_SM, theme.accent_soft);
    } else if response.hovered() {
        painter.rect_filled(rect, crate::theme::RADIUS_SM, theme.card_hover);
    }

    let offsets = column_offsets(rect);
    let widths = column_widths(rect);
    let publisher = app.publisher.as_deref().unwrap_or("—");
    let version = app.version.as_deref().unwrap_or("—");
    let cells = [
        (app.display_name.as_str(), theme.text, true),
        (publisher, theme.text_muted, false),
        (version, theme.text_muted, false),
        (
            scope_label(&app.install_scope),
            theme.role(scope_role(&app.install_scope)),
            false,
        ),
        (arch_label(&app.architecture), theme.text_muted, false),
    ];
    for (index, (text, color, strong)) in cells.into_iter().enumerate() {
        let budget = (widths[index] / 6.7) as usize;
        let font = if strong {
            egui::FontId::proportional(13.0)
        } else {
            egui::FontId::proportional(12.5)
        };
        painter.text(
            egui::pos2(offsets[index], rect.center().y),
            Align2::LEFT_CENTER,
            truncate_tail(text, budget.max(4)),
            font,
            color,
        );
    }
    response
}

fn column_widths(rect: egui::Rect) -> [f32; 5] {
    let usable = rect.width() - 24.0;
    let fractions = [0.36, 0.26, 0.16, 0.12, 0.10];
    let mut widths = [0.0; 5];
    for (slot, fraction) in widths.iter_mut().zip(fractions.iter()) {
        *slot = usable * fraction;
    }
    widths
}

fn column_offsets(rect: egui::Rect) -> [f32; 5] {
    let widths = column_widths(rect);
    let mut offsets = [0.0; 5];
    let mut x = rect.left() + 12.0;
    for (slot, width) in offsets.iter_mut().zip(widths.iter()) {
        *slot = x;
        x += width;
    }
    offsets
}

fn truncate_tail(value: &str, max_chars: usize) -> String {
    let count = value.chars().count();
    if count <= max_chars {
        return value.to_owned();
    }
    let keep = max_chars.saturating_sub(1);
    let skipped = count - keep;
    let tail: String = value.chars().skip(skipped).collect();
    format!("…{tail}")
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
        InstallScope::PerUser => Role::Accent,
        InstallScope::PerMachine => Role::Warning,
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
