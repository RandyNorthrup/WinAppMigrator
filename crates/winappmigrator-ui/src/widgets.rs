//! Reusable, theme-aware UI building blocks shared across every page.

use std::path::PathBuf;

use eframe::egui::{self, Color32, Margin, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui};

use crate::theme::{self, RADIUS_LG, RADIUS_MD, RADIUS_SM, Role, Theme};

/// Visual weight for action buttons.
#[derive(Clone, Copy)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Ghost,
}

/// Glyph drawn for the empty state.
#[derive(Clone, Copy)]
pub enum Icon {
    Apps,
}

/// A single value tile shown in a metrics row.
pub struct Metric {
    pub label: String,
    pub value: String,
    pub role: Role,
}

pub fn metric(label: impl Into<String>, value: impl Into<String>, role: Role) -> Metric {
    Metric {
        label: label.into(),
        value: value.into(),
        role,
    }
}

/// A flat elevated container with the standard card treatment.
pub fn card<R>(ui: &mut Ui, theme: &Theme, contents: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .fill(theme.card)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_LG)
        .inner_margin(Margin::same(18))
        .shadow(theme.soft_shadow())
        .show(ui, |ui| contents(ui))
        .inner
}

/// A card with a heading row (title + optional subtitle) above its content.
pub fn card_titled<R>(
    ui: &mut Ui,
    theme: &Theme,
    title: &str,
    subtitle: Option<&str>,
    contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    card(ui, theme, |ui| {
        ui.label(
            egui::RichText::new(title)
                .size(16.0)
                .strong()
                .color(theme.text),
        );
        if let Some(subtitle) = subtitle {
            ui.label(
                egui::RichText::new(subtitle)
                    .size(12.5)
                    .color(theme.text_muted),
            );
        }
        ui.add_space(12.0);
        contents(ui)
    })
}

/// Small uppercase tracking label used above sections.
pub fn field_label(ui: &mut Ui, theme: &Theme, text: &str) {
    ui.label(egui::RichText::new(text).size(12.5).color(theme.text_muted));
}

/// A primary/secondary/ghost action button.
pub fn button(
    ui: &mut Ui,
    theme: &Theme,
    kind: ButtonKind,
    label: &str,
    enabled: bool,
) -> Response {
    let widget = match kind {
        ButtonKind::Primary => {
            egui::Button::new(egui::RichText::new(label).strong().color(theme.on_accent))
                .fill(theme.accent)
                .stroke(Stroke::NONE)
                .corner_radius(RADIUS_SM)
                .min_size(egui::vec2(120.0, 36.0))
        }
        ButtonKind::Secondary => egui::Button::new(egui::RichText::new(label).color(theme.text))
            .fill(theme.card_hover)
            .stroke(Stroke::new(1.0, theme.border_strong))
            .corner_radius(RADIUS_SM)
            .min_size(egui::vec2(92.0, 34.0)),
        // A ghost (tertiary) button is still unmistakably a button: an outlined
        // capsule with a legible label — never bare text that reads as a caption.
        ButtonKind::Ghost => egui::Button::new(egui::RichText::new(label).color(theme.text))
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, theme.border_strong))
            .corner_radius(RADIUS_SM)
            .min_size(egui::vec2(92.0, 34.0)),
    };
    ui.add_enabled(enabled, widget)
}

/// A status pill: a soft-filled rounded capsule with a leading status dot.
pub fn status_pill(ui: &mut Ui, theme: &Theme, text: &str, role: Role) {
    let color = theme.role(role);
    let dark = theme.mode == theme::ThemeMode::Dark;
    egui::Frame::new()
        .inner_margin(Margin::symmetric(10, 5))
        .fill(theme::alpha(color, if dark { 32 } else { 26 }))
        .stroke(Stroke::new(1.0, theme::alpha(color, 130)))
        .corner_radius(RADIUS_LG)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(9.0, 9.0), Sense::hover());
                ui.painter().circle_filled(rect.center(), 3.5, color);
                ui.add_space(1.0);
                ui.label(egui::RichText::new(text).size(12.0).color(theme.text));
            });
        });
}

/// A compact category badge (no dot), used for tags inside dense lists.
pub fn badge(ui: &mut Ui, theme: &Theme, text: &str, color: Color32) {
    let dark = theme.mode == theme::ThemeMode::Dark;
    egui::Frame::new()
        .inner_margin(Margin::symmetric(8, 3))
        .fill(theme::alpha(color, if dark { 34 } else { 30 }))
        .corner_radius(RADIUS_SM)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .size(11.5)
                    .color(strengthen(theme, color)),
            );
        });
}

/// Lay metric tiles out in rows of four, aligned to a uniform width.
pub fn metric_tiles(ui: &mut Ui, theme: &Theme, metrics: &[Metric]) {
    const PER_ROW: usize = 4;
    for chunk in metrics.chunks(PER_ROW) {
        ui.columns(PER_ROW, |columns| {
            for (index, metric) in chunk.iter().enumerate() {
                metric_tile(&mut columns[index], theme, metric);
            }
        });
        ui.add_space(8.0);
    }
}

fn metric_tile(ui: &mut Ui, theme: &Theme, metric: &Metric) {
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_MD)
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(56.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(&metric.value)
                        .size(23.0)
                        .strong()
                        .color(theme.role(metric.role)),
                );
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(&metric.label)
                        .size(12.0)
                        .color(theme.text_muted),
                );
            });
        });
}

/// A label/value row used in detail panels.
pub fn kv_row(ui: &mut Ui, theme: &Theme, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(key).size(13.0).color(theme.text_muted));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).size(13.0).color(theme.text));
        });
    });
}

/// A monospace path display row.
pub fn mono_path(ui: &mut Ui, theme: &Theme, label: &str, value: &str) {
    field_label(ui, theme, label);
    egui::Frame::new()
        .fill(theme.field_bg)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_SM)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(value)
                    .monospace()
                    .size(12.0)
                    .color(theme.text),
            );
        });
}

/// A colored notice banner (info / warning / error).
pub fn notice(ui: &mut Ui, theme: &Theme, role: Role, message: &str) {
    let color = theme.role(role);
    let dark = theme.mode == theme::ThemeMode::Dark;
    egui::Frame::new()
        .inner_margin(Margin::same(11))
        .fill(theme::alpha(color, if dark { 26 } else { 22 }))
        .stroke(Stroke::new(1.0, theme::alpha(color, 120)))
        .corner_radius(RADIUS_MD)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 18.0), Sense::hover());
                ui.painter().circle_filled(
                    egui::pos2(rect.center().x, rect.center().y),
                    4.0,
                    color,
                );
                ui.add_space(2.0);
                ui.label(egui::RichText::new(message).color(theme.text));
            });
        });
}

/// A centered placeholder shown when a workflow has no data yet.
pub fn empty_state(ui: &mut Ui, theme: &Theme, icon: Icon, title: &str, body: &str) {
    ui.add_space(8.0);
    ui.vertical_centered(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(46.0, 46.0), Sense::hover());
        let painter = ui.painter();
        painter.circle_filled(rect.center(), 23.0, theme::alpha(theme.accent, 26));
        let icon_rect = Rect::from_center_size(rect.center(), egui::vec2(22.0, 22.0));
        paint_icon(painter, icon, icon_rect, theme.accent);
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(title)
                .size(15.0)
                .strong()
                .color(theme.text),
        );
        ui.add_space(2.0);
        ui.label(egui::RichText::new(body).size(13.0).color(theme.text_muted));
    });
    ui.add_space(8.0);
}

/// A labeled path input with a Browse button.
pub fn path_field(
    ui: &mut Ui,
    theme: &Theme,
    label: &str,
    value: &mut String,
    hint: &str,
    pick: impl FnOnce() -> Option<PathBuf>,
) {
    field_label(ui, theme, label);
    ui.horizontal(|ui| {
        let width = (ui.available_width() - 104.0).max(120.0);
        ui.add(
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .desired_width(width),
        );
        if button(ui, theme, ButtonKind::Secondary, "Browse", true)
            .on_hover_text(format!("Choose {label}"))
            .clicked()
            && let Some(path) = pick()
        {
            *value = path.display().to_string();
        }
    });
}

/// Paint the app brand mark inside `rect`.
pub fn paint_logo(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    painter.rect_filled(rect, RADIUS_SM, theme.accent);
    let inset = rect.shrink(rect.width() * 0.28);
    let stroke = Stroke::new(2.0, theme.on_accent);
    // A right-pointing migration chevron pair.
    let mid_y = inset.center().y;
    painter.add(Shape::line(
        vec![
            egui::pos2(inset.left(), inset.top()),
            egui::pos2(inset.center().x, mid_y),
            egui::pos2(inset.left(), inset.bottom()),
        ],
        stroke,
    ));
    painter.add(Shape::line(
        vec![
            egui::pos2(inset.center().x, inset.top()),
            egui::pos2(inset.right(), mid_y),
            egui::pos2(inset.center().x, inset.bottom()),
        ],
        stroke,
    ));
}

/// Draw one of the navigation glyphs inside `rect` using vector primitives.
pub fn paint_icon(painter: &egui::Painter, icon: Icon, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.7, color);
    match icon {
        Icon::Apps => {
            painter.rect_stroke(rect, 3, stroke, StrokeKind::Inside);
            let top = rect.top() + rect.height() * 0.34;
            painter.line_segment(
                [egui::pos2(rect.left(), top), egui::pos2(rect.right(), top)],
                stroke,
            );
            for k in 1..=2 {
                let y = top + k as f32 * (rect.height() * 0.22);
                painter.line_segment(
                    [
                        egui::pos2(rect.left() + 3.0, y),
                        egui::pos2(rect.right() - 3.0, y),
                    ],
                    Stroke::new(1.4, color),
                );
            }
        }
    }
}

/// Strengthen a soft status color so badge text stays legible on its tint.
fn strengthen(theme: &Theme, color: Color32) -> Color32 {
    if theme.mode == theme::ThemeMode::Dark {
        // Lighten toward white for contrast on dark tints.
        let mix = |c: u8| -> u8 { (c as u16 + (255 - c as u16) * 35 / 100) as u8 };
        Color32::from_rgb(mix(color.r()), mix(color.g()), mix(color.b()))
    } else {
        color
    }
}
