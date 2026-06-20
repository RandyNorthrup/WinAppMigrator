//! Page renderers for each navigation destination.

use eframe::egui::{self, Align2, RichText, Sense};

use winappmigrator_core::{
    AppArchitecture, DependencySourceCategory, FileClassification, FootprintKind,
    FootprintLocation, InstallScope, ScanReport, SignatureStatus, VerificationStatus,
    WarningSeverity,
};

use crate::theme::{self, Role, Theme};
use crate::widgets::{self, ButtonKind, Icon, button, metric};
use crate::{InventoryState, OperationState, ScanState, ScopeFilter, View, WinAppMigratorApp};

impl WinAppMigratorApp {
    pub(crate) fn draw_active_view(&mut self, ui: &mut egui::Ui) {
        match self.active_view {
            View::Dashboard => self.view_dashboard(ui),
            View::Applications => self.view_applications(ui),
            View::Analyze => self.view_analyze(ui),
            View::Package => self.view_package(ui),
            View::Restore => self.view_restore(ui),
        }
    }

    // ----- Dashboard ---------------------------------------------------------

    fn view_dashboard(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        let apps_value = match &self.inventory_state {
            InventoryState::Loaded => self.apps.len().to_string(),
            InventoryState::Loading => "…".to_owned(),
            InventoryState::Failed(_) => "—".to_owned(),
        };
        let scan_value = self
            .last_report
            .as_ref()
            .map(|report| report.dependencies.len().to_string())
            .unwrap_or_else(|| "—".to_owned());
        let package_value = self
            .last_package
            .as_ref()
            .map(|package| package.files_copied.to_string())
            .unwrap_or_else(|| "—".to_owned());
        let restore_value = self
            .last_restore
            .as_ref()
            .map(|restore| restore.changes_logged.len().to_string())
            .unwrap_or_else(|| "—".to_owned());

        widgets::metric_tiles(
            ui,
            &theme,
            &[
                metric("Installed applications", apps_value, Role::Accent),
                metric(
                    "Dependencies (last scan)",
                    scan_value,
                    crate::scan_role(self.scan_state),
                ),
                metric(
                    "Files packaged",
                    package_value,
                    crate::op_role(self.package_state),
                ),
                metric(
                    "Restore changes",
                    restore_value,
                    crate::op_role(self.restore_state),
                ),
            ],
        );

        ui.add_space(4.0);
        ui.columns(2, |columns| {
            self.dashboard_workflow(&mut columns[0], &theme);
            self.dashboard_aside(&mut columns[1], &theme);
        });
    }

    fn dashboard_workflow(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        widgets::card_titled(
            ui,
            theme,
            "Migration workflow",
            Some("Move through each stage to back up and restore an application."),
            |ui| {
                let source_ready = !self.executable_path.trim().is_empty();
                let (source_role, source_status) = if source_ready {
                    (Role::Success, "Source selected")
                } else if matches!(self.inventory_state, InventoryState::Loaded) {
                    (Role::Accent, "Choose an app")
                } else {
                    (Role::Neutral, "Loading inventory")
                };

                let stages = [
                    (
                        Icon::Apps,
                        "1. Select source",
                        "Pick an installed application or executable.",
                        source_role,
                        source_status.to_owned(),
                        View::Applications,
                    ),
                    (
                        Icon::Analyze,
                        "2. Analyze",
                        "Resolve dependencies and Windows integrations.",
                        crate::scan_role(self.scan_state),
                        crate::scan_text(self.scan_state).to_owned(),
                        View::Analyze,
                    ),
                    (
                        Icon::Package,
                        "3. Package",
                        "Copy app-owned files and write the migration package.",
                        crate::op_role(self.package_state),
                        crate::op_word(self.package_state).to_owned(),
                        View::Package,
                    ),
                    (
                        Icon::Restore,
                        "4. Restore",
                        "Preview or apply the package on the target machine.",
                        crate::op_role(self.restore_state),
                        crate::op_word(self.restore_state).to_owned(),
                        View::Restore,
                    ),
                ];

                let mut navigate = None;
                for (index, (icon, title, desc, role, status, target)) in
                    stages.into_iter().enumerate()
                {
                    if index > 0 {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                    }
                    if stage_row(ui, theme, icon, title, desc, role, &status) {
                        navigate = Some(target);
                    }
                }
                if let Some(target) = navigate {
                    self.active_view = target;
                }
            },
        );
    }

    fn dashboard_aside(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        widgets::card_titled(ui, theme, "Quick actions", None, |ui| {
            if button(
                ui,
                theme,
                ButtonKind::Secondary,
                "Reload installed apps",
                true,
            )
            .clicked()
            {
                self.start_inventory_load();
            }
            ui.add_space(6.0);
            let snapshot_running = matches!(self.snapshot_state, OperationState::Running);
            if button(
                ui,
                theme,
                ButtonKind::Secondary,
                "Save inventory snapshot",
                !snapshot_running,
            )
            .clicked()
                && let Some(folder) = rfd::FileDialog::new().pick_folder()
            {
                self.output_dir = folder.display().to_string();
                self.start_snapshot();
            }
            ui.add_space(6.0);
            if button(ui, theme, ButtonKind::Primary, "Start analysis", true).clicked() {
                self.active_view = View::Analyze;
            }
        });

        ui.add_space(12.0);

        widgets::card_titled(ui, theme, "Environment", None, |ui| {
            widgets::kv_row(ui, theme, "Product", "WinAppMigrator");
            ui.add_space(4.0);
            widgets::kv_row(ui, theme, "Version", env!("CARGO_PKG_VERSION"));
            ui.add_space(4.0);
            widgets::kv_row(ui, theme, "Target", "Windows 10 / 11 · x64");
            ui.add_space(4.0);
            let working_dir = std::env::current_dir()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "unknown".to_owned());
            ui.add_space(6.0);
            widgets::mono_path(ui, theme, "Working directory", &working_dir);
        });
    }

    // ----- Applications ------------------------------------------------------

    fn view_applications(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        widgets::card(ui, &theme, |ui| {
            ui.horizontal(|ui| {
                let search_width = (ui.available_width() - 360.0).max(160.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.app_search)
                        .hint_text("Search by name or publisher")
                        .desired_width(search_width),
                );
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let snapshot_running = matches!(self.snapshot_state, OperationState::Running);
                    if button(
                        ui,
                        &theme,
                        ButtonKind::Secondary,
                        "Snapshot",
                        !snapshot_running,
                    )
                    .on_hover_text("Save the installed application inventory to disk")
                    .clicked()
                        && let Some(folder) = rfd::FileDialog::new().pick_folder()
                    {
                        self.output_dir = folder.display().to_string();
                        self.start_snapshot();
                    }
                });
            });
        });

        if let OperationState::Completed = self.snapshot_state
            && let Some(snapshot) = self.last_snapshot.as_ref()
        {
            ui.add_space(8.0);
            widgets::notice(
                ui,
                &theme,
                Role::Success,
                &format!(
                    "Saved snapshot of {} applications to {}",
                    snapshot.applications.len(),
                    snapshot.artifacts.markdown.display()
                ),
            );
        }

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
                .max_height(380.0)
                .show_rows(ui, 32.0, filtered.len(), |ui, range| {
                    for row in range {
                        let app_index = filtered[row];
                        let selected = self.selected_app_index == Some(app_index);
                        if app_table_row(ui, &theme, &self.apps[app_index], selected).clicked() {
                            self.selected_app_index = Some(app_index);
                            if let Some(executable) = self.apps[app_index].main_executable.as_ref()
                            {
                                self.executable_path = executable.display().to_string();
                            }
                            if self.output_dir.trim().is_empty() {
                                self.output_dir =
                                    crate::default_output_for(&self.apps[app_index].display_name);
                            }
                        }
                    }
                });
        });

        if let Some(index) = self.selected_app_index
            && index < self.apps.len()
        {
            ui.add_space(12.0);
            self.application_detail(ui, &theme, index);
        }
    }

    fn application_detail(&mut self, ui: &mut egui::Ui, theme: &Theme, index: usize) {
        let app = self.apps[index].clone();
        let mut go_analyze = false;
        let mut go_package = false;

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
            ui.add_space(8.0);
            if let Some(location) = app.install_location.as_ref() {
                widgets::mono_path(
                    ui,
                    theme,
                    "Install location",
                    &location.display().to_string(),
                );
                ui.add_space(6.0);
            }
            match app.main_executable.as_ref() {
                Some(executable) => {
                    widgets::mono_path(
                        ui,
                        theme,
                        "Main executable",
                        &executable.display().to_string(),
                    );
                }
                None => widgets::notice(
                    ui,
                    theme,
                    Role::Warning,
                    "No main executable was detected. Set an executable path on the Analyze page.",
                ),
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if button(
                    ui,
                    theme,
                    ButtonKind::Primary,
                    "Back up this app",
                    app.main_executable.is_some(),
                )
                .on_hover_text("Prefill the package workflow for this application")
                .clicked()
                {
                    go_package = true;
                }
                if button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    "Analyze",
                    app.main_executable.is_some(),
                )
                .clicked()
                {
                    go_analyze = true;
                }
            });
        });

        if go_analyze || go_package {
            if self.output_dir.trim().is_empty() {
                self.output_dir = crate::default_output_for(&app.display_name);
            }
            self.active_view = if go_package {
                View::Package
            } else {
                View::Analyze
            };
        }
    }

    // ----- Analyze -----------------------------------------------------------

    fn view_analyze(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let scan_running = matches!(self.scan_state, ScanState::Running);
        let snapshot_running = matches!(self.snapshot_state, OperationState::Running);

        ui.columns(2, |columns| {
            widgets::card_titled(&mut columns[0], &theme, "Source", None, |ui| {
                if let Some(index) = self.selected_app_index
                    && index < self.apps.len()
                {
                    widgets::notice(
                        ui,
                        &theme,
                        Role::Accent,
                        &format!("Selected: {}", self.apps[index].display_name),
                    );
                    ui.add_space(8.0);
                }
                widgets::path_field(
                    ui,
                    &theme,
                    "Executable",
                    &mut self.executable_path,
                    "C:\\Path\\To\\App.exe",
                    || {
                        rfd::FileDialog::new()
                            .add_filter("Executable", &["exe"])
                            .pick_file()
                    },
                );
                ui.add_space(8.0);
                widgets::path_field(
                    ui,
                    &theme,
                    "Output directory",
                    &mut self.output_dir,
                    "Where reports are written",
                    || rfd::FileDialog::new().pick_folder(),
                );
            });

            widgets::card_titled(&mut columns[1], &theme, "Scan options", None, |ui| {
                ui.checkbox(
                    &mut self.scan_include_user_data,
                    "Include user data (AppData, ProgramData)",
                );
                ui.label(
                    RichText::new(
                        "Captures settings and data the app stores outside its install folder.",
                    )
                    .size(11.5)
                    .color(theme.text_subtle),
                );
                ui.add_space(6.0);
                ui.checkbox(
                    &mut self.scan_recursive_dependencies,
                    "Recursive app-local DLL scan",
                );
                ui.add_space(2.0);
                ui.checkbox(&mut self.scan_include_hashes, "Compute SHA-256 hashes");
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if button(ui, &theme, ButtonKind::Primary, "Run scan", !scan_running).clicked()
                    {
                        self.start_scan();
                    }
                    if button(
                        ui,
                        &theme,
                        ButtonKind::Secondary,
                        "Save snapshot",
                        !snapshot_running,
                    )
                    .clicked()
                    {
                        self.start_snapshot();
                    }
                });
            });
        });

        ui.add_space(12.0);

        match self.scan_state {
            ScanState::Idle => {
                widgets::card(ui, &theme, |ui| {
                    widgets::empty_state(
                        ui,
                        &theme,
                        Icon::Analyze,
                        "No analysis yet",
                        "Select a source and run a scan to resolve dependencies and integrations.",
                    );
                });
            }
            ScanState::Running => {
                widgets::card(ui, &theme, |ui| {
                    busy_row(ui, &theme, "Scan running");
                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.scan_detail).color(theme.text_muted));
                });
                ui.ctx().request_repaint();
            }
            ScanState::Failed => {
                widgets::card(ui, &theme, |ui| {
                    widgets::notice(
                        ui,
                        &theme,
                        Role::Danger,
                        "Scan failed. See the error below for details.",
                    );
                });
            }
            ScanState::Completed | ScanState::Warning => self.analyze_results(ui, &theme),
        }

        if let Some(error) = self.last_error.clone() {
            ui.add_space(10.0);
            widgets::notice(ui, &theme, Role::Danger, &error);
        }
    }

    fn analyze_results(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folder_error = None;
        if let Some(report) = self.last_report.as_ref() {
            let missing = report
                .dependencies
                .iter()
                .filter(|entry| entry.missing)
                .count();
            let to_copy = report
                .dependencies
                .iter()
                .filter(|entry| entry.should_copy)
                .count();

            widgets::card_titled(ui, theme, "Summary", None, |ui| {
                widgets::metric_tiles(
                    ui,
                    theme,
                    &[
                        metric(
                            "Confidence",
                            format!("{}%", report.confidence_score),
                            confidence_role(report.confidence_score),
                        ),
                        metric(
                            "Dependencies",
                            report.dependencies.len().to_string(),
                            Role::Accent,
                        ),
                        metric("Copy candidates", to_copy.to_string(), Role::Success),
                        metric(
                            "Missing",
                            missing.to_string(),
                            if missing > 0 {
                                Role::Danger
                            } else {
                                Role::Neutral
                            },
                        ),
                    ],
                );
                ui.add_space(6.0);
                widgets::kv_row(ui, theme, "Executable", &report.main_executable.file_name);
                ui.add_space(4.0);
                widgets::kv_row(ui, theme, "Architecture", arch_label(&report.architecture));
                ui.add_space(4.0);
                widgets::kv_row(
                    ui,
                    theme,
                    "Signature",
                    signature_label(&report.main_executable.digital_signature),
                );
                if let Some(version) =
                    report
                        .main_executable
                        .version_info
                        .as_ref()
                        .and_then(|info| {
                            info.product_version
                                .clone()
                                .or_else(|| info.file_version.clone())
                        })
                {
                    ui.add_space(4.0);
                    widgets::kv_row(ui, theme, "Version", &version);
                }
                if let Some(method) = report.install_method.as_ref() {
                    ui.add_space(4.0);
                    widgets::kv_row(ui, theme, "Install method", method.label());
                }
                if report.authoritative_file_count > 0 {
                    ui.add_space(4.0);
                    widgets::kv_row(
                        ui,
                        theme,
                        "Installer-recorded files",
                        &report.authoritative_file_count.to_string(),
                    );
                }
                if !report.install_evidence.is_empty() {
                    ui.add_space(8.0);
                    widgets::field_label(ui, theme, "How this was reverse-engineered");
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        for signal in &report.install_evidence {
                            widgets::badge(ui, theme, signal, theme.accent);
                        }
                    });
                }
                if let Some(artifacts) = report.generated_artifacts.as_ref() {
                    ui.add_space(10.0);
                    if let Some(parent) = artifacts.scan_report_markdown.parent()
                        && button(ui, theme, ButtonKind::Secondary, "Open report folder", true)
                            .clicked()
                        && let Err(error) = crate::open_folder(parent)
                    {
                        folder_error = Some(error.to_string());
                    }
                }
            });

            ui.add_space(12.0);
            footprint_card(ui, theme, report);

            ui.add_space(12.0);
            widgets::card_titled(ui, theme, "Dependency graph", None, |ui| {
                draw_dependency_graph(ui, theme, report);
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    legend_dot(ui, theme, theme.success, "Copy");
                    legend_dot(ui, theme, theme.text_subtle, "System / provided");
                    legend_dot(ui, theme, theme.danger, "Missing");
                });
            });

            ui.add_space(12.0);
            widgets::card_titled(
                ui,
                theme,
                &format!("Dependencies · {}", report.dependencies.len()),
                None,
                |ui| dependency_table(ui, theme, report),
            );

            ui.add_space(12.0);
            self.analyze_integrations(ui, theme, report);

            if !report.runtime_prerequisites.is_empty() {
                ui.add_space(12.0);
                widgets::card_titled(ui, theme, "Runtime prerequisites", None, |ui| {
                    for prerequisite in &report.runtime_prerequisites {
                        line_item(
                            ui,
                            theme,
                            &prerequisite.name,
                            &prerequisite.evidence,
                            if prerequisite.install_manually {
                                Role::Warning
                            } else {
                                Role::Neutral
                            },
                        );
                    }
                });
            }

            if !report.warnings.is_empty() {
                ui.add_space(12.0);
                widgets::card_titled(
                    ui,
                    theme,
                    &format!("Warnings · {}", report.warnings.len()),
                    None,
                    |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("scan-warnings")
                            .max_height(220.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for warning in &report.warnings {
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
                            });
                    },
                );
            }
        }

        if let Some(error) = folder_error {
            self.last_error = Some(error);
        }
    }

    fn analyze_integrations(&self, ui: &mut egui::Ui, theme: &Theme, report: &ScanReport) {
        widgets::card_titled(ui, theme, "Windows integrations", None, |ui| {
            ui.horizontal_wrapped(|ui| {
                integration_badge(ui, theme, "Registry", report.registry_entries.len());
                integration_badge(ui, theme, "COM", report.com_registrations.len());
                integration_badge(ui, theme, "File assoc.", report.file_associations.len());
                integration_badge(ui, theme, "Services", report.services.len());
                integration_badge(ui, theme, "Drivers", report.drivers.len());
                integration_badge(ui, theme, "Tasks", report.scheduled_tasks.len());
                integration_badge(ui, theme, "Shortcuts", report.shortcuts.len());
                integration_badge(ui, theme, "Env vars", report.environment_variables.len());
                integration_badge(ui, theme, "Files", report.file_inventory.len());
            });
            ui.add_space(10.0);

            collapsible(
                ui,
                theme,
                "Registry entries",
                report.registry_entries.len(),
                "int-reg",
                |ui| {
                    for entry in report.registry_entries.iter().take(200) {
                        let hive = match entry.hive {
                            winappmigrator_core::RegistryHive::LocalMachine => "HKLM",
                            winappmigrator_core::RegistryHive::CurrentUser => "HKCU",
                        };
                        let value = entry
                            .value_data
                            .clone()
                            .unwrap_or_else(|| "(key)".to_owned());
                        line_item(
                            ui,
                            theme,
                            &format!("{hive}\\{}", entry.key_path),
                            &value,
                            Role::Neutral,
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "Services",
                report.services.len(),
                "int-svc",
                |ui| {
                    for entry in &report.services {
                        line_item(
                            ui,
                            theme,
                            entry.display_name.as_deref().unwrap_or(&entry.service_name),
                            &entry.binary_path.display().to_string(),
                            if entry.requires_admin {
                                Role::Warning
                            } else {
                                Role::Neutral
                            },
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "Drivers",
                report.drivers.len(),
                "int-drv",
                |ui| {
                    for entry in &report.drivers {
                        line_item(
                            ui,
                            theme,
                            entry.display_name.as_deref().unwrap_or(&entry.service_name),
                            &entry.image_path.display().to_string(),
                            Role::Warning,
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "Scheduled tasks",
                report.scheduled_tasks.len(),
                "int-task",
                |ui| {
                    for entry in &report.scheduled_tasks {
                        line_item(
                            ui,
                            theme,
                            &entry.task_path,
                            &entry.action_command.display().to_string(),
                            if entry.requires_admin {
                                Role::Warning
                            } else {
                                Role::Neutral
                            },
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "Shortcuts",
                report.shortcuts.len(),
                "int-lnk",
                |ui| {
                    for entry in &report.shortcuts {
                        line_item(
                            ui,
                            theme,
                            &entry.original_lnk_path.display().to_string(),
                            &entry.target_path.display().to_string(),
                            Role::Neutral,
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "Environment variables",
                report.environment_variables.len(),
                "int-env",
                |ui| {
                    for entry in &report.environment_variables {
                        line_item(
                            ui,
                            theme,
                            &entry.name,
                            &entry.value,
                            if entry.requires_admin {
                                Role::Warning
                            } else {
                                Role::Neutral
                            },
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "COM registrations",
                report.com_registrations.len(),
                "int-com",
                |ui| {
                    for entry in &report.com_registrations {
                        let server = entry
                            .server_path
                            .as_ref()
                            .map(|path| path.display().to_string());
                        line_item(
                            ui,
                            theme,
                            &entry.clsid,
                            server.as_deref().unwrap_or(&entry.key_path),
                            Role::Neutral,
                        );
                    }
                },
            );
            collapsible(
                ui,
                theme,
                "File associations",
                report.file_associations.len(),
                "int-assoc",
                |ui| {
                    for entry in &report.file_associations {
                        let ext = entry
                            .extension
                            .clone()
                            .unwrap_or_else(|| "(verb)".to_owned());
                        let detail = entry
                            .command
                            .clone()
                            .or_else(|| entry.prog_id.clone())
                            .unwrap_or_default();
                        line_item(ui, theme, &ext, &detail, Role::Neutral);
                    }
                },
            );
        });
    }

    // ----- Package -----------------------------------------------------------

    fn view_package(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let package_running = matches!(self.package_state, OperationState::Running);

        widgets::card_titled(ui, &theme, "Package source", None, |ui| {
            widgets::path_field(
                ui,
                &theme,
                "Executable",
                &mut self.executable_path,
                "C:\\Path\\To\\App.exe",
                || {
                    rfd::FileDialog::new()
                        .add_filter("Executable", &["exe"])
                        .pick_file()
                },
            );
            ui.add_space(8.0);
            widgets::path_field(
                ui,
                &theme,
                "Output directory",
                &mut self.output_dir,
                "Where the package is written",
                || rfd::FileDialog::new().pick_folder(),
            );
            ui.add_space(12.0);
            widgets::field_label(ui, &theme, "Contents");
            ui.checkbox(
                &mut self.scan_include_user_data,
                "Include user data (AppData, ProgramData)",
            );
            ui.add_space(10.0);
            widgets::field_label(ui, &theme, "Archive options");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.package_zip, "Portable .zip");
                ui.add_space(10.0);
                ui.checkbox(&mut self.package_sfx, "Self-extracting .exe");
            });
            if self.package_sfx && crate::cli_stub_path().is_none() {
                ui.add_space(8.0);
                widgets::notice(
                    ui,
                    &theme,
                    Role::Warning,
                    "SFX stub (winappmigrator.exe) was not found beside the GUI executable.",
                );
            }
            ui.add_space(14.0);
            if button(
                ui,
                &theme,
                ButtonKind::Primary,
                "Build package",
                !package_running,
            )
            .clicked()
            {
                self.start_package();
            }
        });

        ui.add_space(12.0);

        match self.package_state {
            OperationState::Idle => {
                widgets::card(ui, &theme, |ui| {
                    widgets::empty_state(
                        ui,
                        &theme,
                        Icon::Package,
                        "No package yet",
                        "Build a package to copy app-owned files and write migration artifacts.",
                    );
                });
            }
            OperationState::Running => {
                widgets::card(ui, &theme, |ui| {
                    busy_row(ui, &theme, "Building package");
                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.package_detail).color(theme.text_muted));
                });
                ui.ctx().request_repaint();
            }
            OperationState::Failed => {
                widgets::card(ui, &theme, |ui| {
                    widgets::notice(ui, &theme, Role::Danger, "Package build failed.")
                });
            }
            OperationState::Completed | OperationState::Warning => self.package_results(ui, &theme),
        }

        if let Some(error) = self.last_error.clone() {
            ui.add_space(10.0);
            widgets::notice(ui, &theme, Role::Danger, &error);
        }
    }

    fn package_results(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folder_error = None;
        if let Some(package) = self.last_package.as_ref() {
            widgets::card_titled(ui, theme, "Package result", None, |ui| {
                widgets::metric_tiles(
                    ui,
                    theme,
                    &[
                        metric(
                            "Files copied",
                            package.files_copied.to_string(),
                            Role::Success,
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
                ui.add_space(6.0);
                widgets::mono_path(
                    ui,
                    theme,
                    "Package root",
                    &package.package_root.display().to_string(),
                );
                if let Some(archive) = package.archive_path.as_ref() {
                    ui.add_space(6.0);
                    widgets::mono_path(ui, theme, "Archive", &archive.display().to_string());
                }
                if let Some(sfx) = package.self_extracting_archive_path.as_ref() {
                    ui.add_space(6.0);
                    widgets::mono_path(ui, theme, "Self-extracting", &sfx.display().to_string());
                }
                ui.add_space(12.0);
                if button(
                    ui,
                    theme,
                    ButtonKind::Secondary,
                    "Open package folder",
                    true,
                )
                .clicked()
                    && let Err(error) = crate::open_folder(&package.package_root)
                {
                    folder_error = Some(error.to_string());
                }
            });
        }
        if let Some(error) = folder_error {
            self.last_error = Some(error);
        }
    }

    // ----- Restore -----------------------------------------------------------

    fn view_restore(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let restore_running = matches!(self.restore_state, OperationState::Running);

        widgets::card_titled(ui, &theme, "Restore source", None, |ui| {
            widgets::field_label(ui, &theme, "Package");
            ui.horizontal(|ui| {
                let width = (ui.available_width() - 160.0).max(120.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.restore_package_dir)
                        .hint_text("Package folder or .zip")
                        .desired_width(width),
                );
                if button(ui, &theme, ButtonKind::Secondary, "Folder", true).clicked()
                    && let Some(folder) = rfd::FileDialog::new().pick_folder()
                {
                    self.restore_package_dir = folder.display().to_string();
                }
                if button(ui, &theme, ButtonKind::Secondary, "Zip", true).clicked()
                    && let Some(file) = rfd::FileDialog::new()
                        .add_filter("Package zip", &["zip"])
                        .pick_file()
                {
                    self.restore_package_dir = file.display().to_string();
                }
            });
            ui.add_space(8.0);
            widgets::path_field(
                ui,
                &theme,
                "Target root",
                &mut self.restore_target_dir,
                "Where files are restored",
                || rfd::FileDialog::new().pick_folder(),
            );

            ui.add_space(12.0);
            widgets::field_label(ui, &theme, "Mode");
            segmented(
                ui,
                &theme,
                &mut self.restore_apply,
                &[("Dry-run", false), ("Apply", true)],
            );

            if !self.restore_apply {
                self.restore_force = false;
            }
            ui.add_space(8.0);
            ui.add_enabled_ui(self.restore_apply, |ui| {
                ui.checkbox(&mut self.restore_force, "Overwrite existing files");
            });

            if self.restore_apply {
                ui.add_space(8.0);
                widgets::notice(
                    ui,
                    &theme,
                    Role::Danger,
                    "Apply mode writes files and Windows integration data to the target system.",
                );
            }

            ui.add_space(14.0);
            let (kind, label) = if self.restore_apply {
                (ButtonKind::Danger, "Apply restore")
            } else {
                (ButtonKind::Primary, "Dry-run restore")
            };
            if button(ui, &theme, kind, label, !restore_running).clicked() {
                self.start_restore();
            }
        });

        ui.add_space(12.0);

        match self.restore_state {
            OperationState::Idle => {
                widgets::card(ui, &theme, |ui| {
                    widgets::empty_state(
                        ui,
                        &theme,
                        Icon::Restore,
                        "No restore yet",
                        "Preview a package with a dry-run before applying it to the target.",
                    );
                });
            }
            OperationState::Running => {
                widgets::card(ui, &theme, |ui| {
                    busy_row(ui, &theme, "Restore running");
                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.restore_detail).color(theme.text_muted));
                });
                ui.ctx().request_repaint();
            }
            OperationState::Failed => {
                widgets::card(ui, &theme, |ui| {
                    widgets::notice(ui, &theme, Role::Danger, "Restore failed.")
                });
            }
            OperationState::Completed | OperationState::Warning => self.restore_results(ui, &theme),
        }

        if let Some(error) = self.last_error.clone() {
            ui.add_space(10.0);
            widgets::notice(ui, &theme, Role::Danger, &error);
        }
    }

    fn restore_results(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut folder_error = None;
        if let Some(restore) = self.last_restore.as_ref() {
            widgets::card_titled(ui, theme, "Restore result", None, |ui| {
                widgets::metric_tiles(
                    ui,
                    theme,
                    &[
                        metric(
                            "Mode",
                            if restore.applied {
                                "Applied"
                            } else {
                                "Dry-run"
                            },
                            if restore.applied {
                                Role::Danger
                            } else {
                                Role::Accent
                            },
                        ),
                        metric(
                            "Changes",
                            restore.changes_logged.len().to_string(),
                            Role::Accent,
                        ),
                        metric(
                            "Warnings",
                            restore.warnings.len().to_string(),
                            if restore.warnings.is_empty() {
                                Role::Neutral
                            } else {
                                Role::Warning
                            },
                        ),
                        metric(
                            "Errors",
                            restore.errors.len().to_string(),
                            if restore.errors.is_empty() {
                                Role::Success
                            } else {
                                Role::Danger
                            },
                        ),
                    ],
                );

                if let Some(verification) = restore.verification.as_ref() {
                    ui.add_space(10.0);
                    widgets::field_label(ui, theme, "Verification");
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        widgets::status_pill(
                            ui,
                            theme,
                            if verification.package_valid {
                                "Package valid"
                            } else {
                                "Package invalid"
                            },
                            if verification.package_valid {
                                Role::Success
                            } else {
                                Role::Danger
                            },
                        );
                        let (restore_text, restore_role) = match verification.restore_valid {
                            Some(true) => ("Restore valid", Role::Success),
                            Some(false) => ("Restore invalid", Role::Danger),
                            None => ("Restore not checked", Role::Neutral),
                        };
                        widgets::status_pill(ui, theme, restore_text, restore_role);
                        widgets::status_pill(
                            ui,
                            theme,
                            &format!("{} checked", verification.items.len()),
                            Role::Accent,
                        );
                        if !verification.errors.is_empty() {
                            widgets::status_pill(
                                ui,
                                theme,
                                &format!("{} errors", verification.errors.len()),
                                Role::Danger,
                            );
                        }
                    });

                    if !verification.items.is_empty() {
                        ui.add_space(10.0);
                        collapsible(
                            ui,
                            theme,
                            "Verification items",
                            verification.items.len(),
                            "verify-items",
                            |ui| {
                                for item in &verification.items {
                                    ui.horizontal(|ui| {
                                        widgets::badge(
                                            ui,
                                            theme,
                                            verification_status_label(&item.status),
                                            theme.role(verification_status_role(&item.status)),
                                        );
                                        ui.label(RichText::new(&item.name).color(theme.text));
                                    });
                                    if !item.message.is_empty() {
                                        ui.label(
                                            RichText::new(&item.message)
                                                .size(12.0)
                                                .color(theme.text_muted),
                                        );
                                    }
                                    ui.add_space(4.0);
                                }
                            },
                        );
                    }
                }

                if !restore.changes_logged.is_empty() {
                    ui.add_space(10.0);
                    collapsible(
                        ui,
                        theme,
                        "Logged changes",
                        restore.changes_logged.len(),
                        "restore-changes",
                        |ui| {
                            for change in restore.changes_logged.iter().take(300) {
                                ui.label(RichText::new(change).size(12.5).color(theme.text));
                                ui.add_space(2.0);
                            }
                        },
                    );
                }

                if let Some(log) = restore.restore_log_markdown.as_ref() {
                    ui.add_space(10.0);
                    widgets::mono_path(ui, theme, "Restore log", &log.display().to_string());
                    if let Some(parent) = log.parent() {
                        ui.add_space(8.0);
                        if button(ui, theme, ButtonKind::Secondary, "Open restore logs", true)
                            .clicked()
                            && let Err(error) = crate::open_folder(parent)
                        {
                            folder_error = Some(error.to_string());
                        }
                    }
                }
            });
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

fn stage_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    icon: Icon,
    title: &str,
    desc: &str,
    role: Role,
    status: &str,
) -> bool {
    let mut clicked = false;
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(34.0, 34.0), Sense::hover());
        ui.painter()
            .rect_filled(rect, crate::theme::RADIUS_SM, theme.accent_soft);
        let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(18.0, 18.0));
        widgets::paint_icon(ui.painter(), icon, icon_rect, theme.accent);
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.label(RichText::new(title).size(14.0).strong().color(theme.text));
            ui.label(RichText::new(desc).size(12.0).color(theme.text_muted));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if button(ui, theme, ButtonKind::Ghost, "Open", true).clicked() {
                clicked = true;
            }
            widgets::status_pill(ui, theme, status, role);
        });
    });
    clicked
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

fn legend_dot(ui: &mut egui::Ui, theme: &Theme, color: egui::Color32, label: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), Sense::hover());
        ui.painter().circle_filled(rect.center(), 4.0, color);
        ui.label(RichText::new(label).size(12.0).color(theme.text_muted));
    });
    ui.add_space(8.0);
}

fn integration_badge(ui: &mut egui::Ui, theme: &Theme, label: &str, count: usize) {
    let color = if count > 0 {
        theme.accent
    } else {
        theme.text_subtle
    };
    widgets::badge(ui, theme, &format!("{label} {count}"), color);
}

fn collapsible(
    ui: &mut egui::Ui,
    theme: &Theme,
    title: &str,
    count: usize,
    id: &str,
    body: impl FnOnce(&mut egui::Ui),
) {
    if count == 0 {
        return;
    }
    egui::CollapsingHeader::new(
        RichText::new(format!("{title}  ·  {count}"))
            .color(theme.text)
            .strong(),
    )
    .id_salt(id)
    .default_open(false)
    .show(ui, |ui| {
        egui::ScrollArea::vertical()
            .id_salt(format!("{id}-scroll"))
            .max_height(240.0)
            .auto_shrink([false, false])
            .show(ui, body);
    });
    ui.add_space(2.0);
}

fn line_item(ui: &mut egui::Ui, theme: &Theme, primary: &str, secondary: &str, role: Role) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0, 16.0), Sense::hover());
        ui.painter()
            .circle_filled(rect.center(), 3.0, theme.role(role));
        ui.label(RichText::new(primary).size(13.0).color(theme.text));
    });
    if !secondary.is_empty() {
        ui.label(
            RichText::new(truncate_tail(secondary, 90))
                .size(11.5)
                .monospace()
                .color(theme.text_muted),
        );
    }
    ui.add_space(6.0);
}

fn footprint_card(ui: &mut egui::Ui, theme: &Theme, report: &ScanReport) {
    let locations = &report.footprint_locations;
    let total_files: usize = locations.iter().map(|location| location.file_count).sum();
    let total_bytes: u64 = locations.iter().map(|location| location.total_bytes).sum();
    let copy_bytes: u64 = locations
        .iter()
        .map(|location| location.copyable_bytes)
        .sum();

    widgets::card_titled(
        ui,
        theme,
        "Data locations",
        Some("Everywhere this application stores files across the PC."),
        |ui| {
            if locations.is_empty() {
                ui.label(
                    RichText::new("No additional data locations were found beyond the executable.")
                        .color(theme.text_muted),
                );
                return;
            }
            widgets::metric_tiles(
                ui,
                theme,
                &[
                    metric("Locations", locations.len().to_string(), Role::Accent),
                    metric("Files", total_files.to_string(), Role::Accent),
                    metric("Total size", human_bytes(total_bytes), Role::Neutral),
                    metric("To back up", human_bytes(copy_bytes), Role::Success),
                ],
            );
            ui.add_space(10.0);
            for location in locations {
                footprint_row(ui, theme, location);
            }
        },
    );
}

fn footprint_row(ui: &mut egui::Ui, theme: &Theme, location: &FootprintLocation) {
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(crate::theme::RADIUS_MD)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), Sense::hover());
                ui.painter().circle_filled(
                    rect.center(),
                    4.0,
                    theme.role(footprint_role(&location.kind)),
                );
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(location.kind.label())
                            .size(13.0)
                            .strong()
                            .color(theme.text),
                    );
                    ui.label(
                        RichText::new(truncate_tail(&location.path.display().to_string(), 70))
                            .size(11.5)
                            .monospace()
                            .color(theme.text_muted),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    widgets::badge(ui, theme, scope_label(&location.scope), theme.text_subtle);
                    ui.label(
                        RichText::new(format!(
                            "{} files · {}",
                            location.file_count,
                            human_bytes(location.total_bytes)
                        ))
                        .size(12.0)
                        .color(theme.text_muted),
                    );
                });
            });
        });
    ui.add_space(8.0);
}

fn footprint_role(kind: &FootprintKind) -> Role {
    match kind {
        FootprintKind::InstallDirectory => Role::Accent,
        FootprintKind::ProgramData => Role::Warning,
        FootprintKind::RoamingAppData
        | FootprintKind::LocalAppData
        | FootprintKind::LocalLowAppData => Role::Success,
        FootprintKind::RegistryReference | FootprintKind::Other => Role::Neutral,
    }
}

fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut index = 0;
    while value >= 1024.0 && index < UNITS.len() - 1 {
        value /= 1024.0;
        index += 1;
    }
    if index == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[index])
    }
}

fn dependency_table(ui: &mut egui::Ui, theme: &Theme, report: &ScanReport) {
    if report.dependencies.is_empty() {
        ui.label(RichText::new("No imported dependencies were detected.").color(theme.text_muted));
        return;
    }
    egui::ScrollArea::vertical()
        .id_salt("dependency-table")
        .max_height(320.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("dependency-grid")
                .num_columns(4)
                .spacing([16.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    for header in ["Name", "Source", "Resolved path", "Status"] {
                        ui.label(
                            RichText::new(header)
                                .size(11.5)
                                .strong()
                                .color(theme.text_subtle),
                        );
                    }
                    ui.end_row();
                    for dependency in &report.dependencies {
                        ui.label(
                            RichText::new(truncate_tail(&dependency.name, 32))
                                .size(12.5)
                                .monospace()
                                .color(theme.text),
                        );
                        widgets::badge(
                            ui,
                            theme,
                            dependency_category_label(&dependency.source_category),
                            theme.accent,
                        );
                        let resolved = dependency
                            .resolved_path
                            .as_ref()
                            .map(|path| truncate_tail(&path.display().to_string(), 44))
                            .unwrap_or_else(|| "—".to_owned());
                        ui.label(
                            RichText::new(resolved)
                                .size(11.5)
                                .monospace()
                                .color(theme.text_muted),
                        );
                        let (status, role) = if dependency.missing {
                            ("Missing", Role::Danger)
                        } else if dependency.should_copy {
                            ("Copy", Role::Success)
                        } else {
                            ("Provided", Role::Neutral)
                        };
                        widgets::badge(ui, theme, status, theme.role(role));
                        ui.end_row();
                    }
                });
        });
}

fn draw_dependency_graph(ui: &mut egui::Ui, theme: &Theme, report: &ScanReport) {
    let width = ui.available_width().max(320.0);
    let height = 210.0;
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, crate::theme::RADIUS_MD, theme.surface);
    painter.rect_stroke(
        rect,
        crate::theme::RADIUS_MD,
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );

    let main = egui::pos2(rect.left() + 96.0, rect.center().y);
    let right_x = rect.right() - 116.0;
    let visible: Vec<_> = report.dependencies.iter().take(9).collect();

    if visible.is_empty() {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "No imported dependencies",
            egui::FontId::proportional(13.0),
            theme.text_muted,
        );
        return;
    }

    let step = height / (visible.len() as f32 + 1.0);
    for (index, dependency) in visible.iter().enumerate() {
        let y = rect.top() + step * (index as f32 + 1.0);
        let node = egui::pos2(right_x, y);
        let color = if dependency.missing {
            theme.danger
        } else if dependency.should_copy {
            theme.success
        } else {
            theme.text_subtle
        };
        painter.line_segment(
            [main, node],
            egui::Stroke::new(1.2, theme::alpha(theme.border_strong, 200)),
        );
        painter.circle_filled(node, 6.0, color);
        painter.text(
            node + egui::vec2(12.0, 0.0),
            Align2::LEFT_CENTER,
            truncate_tail(&dependency.name, 22),
            egui::FontId::proportional(12.0),
            theme.text,
        );
    }

    painter.circle_filled(main, 22.0, theme.accent);
    painter.text(
        main,
        Align2::CENTER_CENTER,
        "APP",
        egui::FontId::proportional(11.0),
        theme.on_accent,
    );
    painter.text(
        main + egui::vec2(0.0, 34.0),
        Align2::CENTER_CENTER,
        truncate_tail(&report.main_executable.file_name, 18),
        egui::FontId::proportional(11.0),
        theme.text_muted,
    );

    if report.dependencies.len() > visible.len() {
        painter.text(
            egui::pos2(rect.right() - 8.0, rect.bottom() - 14.0),
            Align2::RIGHT_CENTER,
            format!("+{} more", report.dependencies.len() - visible.len()),
            egui::FontId::proportional(11.5),
            theme.text_muted,
        );
    }
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

fn dependency_category_label(category: &DependencySourceCategory) -> &'static str {
    match category {
        DependencySourceCategory::KnownDll => "Known DLL",
        DependencySourceCategory::ApiSet => "API set",
        DependencySourceCategory::AppLocal => "App-local",
        DependencySourceCategory::System32 => "System32",
        DependencySourceCategory::SysWow64 => "SysWOW64",
        DependencySourceCategory::Windows => "Windows",
        DependencySourceCategory::CurrentDirectory => "Current dir",
        DependencySourceCategory::Path => "PATH",
        DependencySourceCategory::SideBySide => "Side-by-side",
        DependencySourceCategory::RuntimePrerequisite => "Runtime",
        DependencySourceCategory::Unresolved => "Unresolved",
    }
}

#[allow(dead_code)]
fn classification_label(classification: &FileClassification) -> &'static str {
    match classification {
        FileClassification::AppOwned => "App-owned",
        FileClassification::UserData => "User data",
        FileClassification::SharedRuntime => "Shared runtime",
        FileClassification::WindowsSystem => "Windows system",
        FileClassification::Missing => "Missing",
        FileClassification::DoNotCopy => "Do not copy",
        FileClassification::ManualRuntimeInstall => "Manual runtime",
        FileClassification::Unknown => "Unknown",
    }
}

fn severity_role(severity: &WarningSeverity) -> Role {
    match severity {
        WarningSeverity::Info => Role::Accent,
        WarningSeverity::Warning => Role::Warning,
        WarningSeverity::Error => Role::Danger,
    }
}

fn signature_label(status: &SignatureStatus) -> &'static str {
    match status {
        SignatureStatus::NotPresent => "Unsigned",
        SignatureStatus::EmbeddedSignaturePresent => "Signed (embedded)",
        SignatureStatus::NotChecked => "Not checked",
        SignatureStatus::Unknown => "Unknown",
    }
}

fn verification_status_label(status: &VerificationStatus) -> &'static str {
    match status {
        VerificationStatus::Passed => "Passed",
        VerificationStatus::Warning => "Warning",
        VerificationStatus::Failed => "Failed",
        VerificationStatus::Skipped => "Skipped",
    }
}

fn verification_status_role(status: &VerificationStatus) -> Role {
    match status {
        VerificationStatus::Passed => Role::Success,
        VerificationStatus::Warning => Role::Warning,
        VerificationStatus::Failed => Role::Danger,
        VerificationStatus::Skipped => Role::Neutral,
    }
}

fn confidence_role(score: u8) -> Role {
    if score >= 75 {
        Role::Success
    } else if score >= 45 {
        Role::Warning
    } else {
        Role::Danger
    }
}
