//! Reusable, theme-aware UI building blocks shared by the app and the installer.

use std::path::PathBuf;

use eframe::egui::Role as A11yRole;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, CursorIcon, FontId, Galley, Margin, Rect, Response,
    RichText, Sense, Shape, Stroke, StrokeKind, TextStyle, TextWrapMode, Ui, WidgetInfo, pos2,
    text::LayoutJob, vec2,
};

use crate::theme::{
    self, CONTROL_HEIGHT, RADIUS_LG, RADIUS_MD, RADIUS_SM, Role, SPACE_LG, SPACE_MD, SPACE_SM,
    SPACE_XS, Theme,
};

/// Visual weight for action buttons.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Ghost,
}

/// Vector glyphs, drawn with the painter so no image assets are required.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Apps,
    Search,
    Refresh,
    Sun,
    Moon,
    Folder,
    Copy,
    Close,
    Check,
    Warning,
    Error,
    Info,
    Drive,
    Monitor,
    ChevronUp,
    ChevronDown,
    ChevronRight,
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

/// A flat elevated container that fills the available width.
pub fn card<R>(ui: &mut Ui, theme: &Theme, contents: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .fill(theme.card)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_LG)
        .inner_margin(Margin::same(SPACE_LG as i8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            contents(ui)
        })
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
        heading(ui, theme, title);
        if let Some(subtitle) = subtitle {
            ui.add_space(2.0);
            ui.label(RichText::new(subtitle).size(12.5).color(theme.text_muted));
        }
        ui.add_space(SPACE_MD);
        contents(ui)
    })
}

/// A left-to-right row centered on the shared control height, so labels line up with
/// the buttons and fields beside them.
pub fn control_row<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), CONTROL_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        contents,
    )
    .inner
}

/// A semibold heading.
pub fn heading(ui: &mut Ui, theme: &Theme, text: &str) {
    ui.add(
        egui::Label::new(
            RichText::new(text)
                .font(theme::semibold(15.0))
                .color(theme.text),
        )
        .wrap_mode(TextWrapMode::Wrap),
    );
}

/// A small label above a field or a group of controls.
pub fn field_label(ui: &mut Ui, theme: &Theme, text: &str) {
    ui.label(
        RichText::new(text)
            .font(theme::semibold(12.5))
            .color(theme.text_muted),
    );
}

/// Muted supporting text that wraps to the available width.
pub fn hint(ui: &mut Ui, theme: &Theme, text: &str) {
    ui.add(
        egui::Label::new(RichText::new(text).size(12.5).color(theme.text_muted))
            .wrap_mode(TextWrapMode::Wrap),
    );
}

/// Lay a single line of text out, cutting it with `…` at `max_width` when it does
/// not fit.
fn elided_galley(
    ui: &Ui,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> std::sync::Arc<Galley> {
    let mut job = LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap = egui::text::TextWrapping::truncate_at_width(max_width.max(0.0));
    ui.painter().layout_job(job)
}

/// Paint one line of text left-aligned at `left`, centered vertically on `center_y`,
/// cut with `…` at `max_width`.
pub fn paint_elided(
    ui: &Ui,
    text: &str,
    font: FontId,
    color: Color32,
    left: f32,
    center_y: f32,
    max_width: f32,
) {
    let galley = elided_galley(ui, text, font, color, max_width);
    let top = center_y - galley.size().y / 2.0;
    ui.painter().galley(pos2(left, top), galley, color);
}

/// A button of the given weight.
pub fn button(
    ui: &mut Ui,
    theme: &Theme,
    kind: ButtonKind,
    label: &str,
    enabled: bool,
) -> Response {
    draw_button(ui, theme, kind, None, Some(label), enabled, None)
}

/// A button with a leading icon.
pub fn icon_text_button(
    ui: &mut Ui,
    theme: &Theme,
    kind: ButtonKind,
    icon: Icon,
    label: &str,
    enabled: bool,
) -> Response {
    draw_button(ui, theme, kind, Some(icon), Some(label), enabled, None)
}

/// A button that fills the available width.
pub fn wide_button(
    ui: &mut Ui,
    theme: &Theme,
    kind: ButtonKind,
    label: &str,
    enabled: bool,
) -> Response {
    let width = ui.available_width();
    draw_button(ui, theme, kind, None, Some(label), enabled, Some(width))
}

/// A square icon-only button. `label` becomes its tooltip and accessible name.
pub fn icon_button(ui: &mut Ui, theme: &Theme, icon: Icon, label: &str, enabled: bool) -> Response {
    let (rect, response) = allocate_control(ui, vec2(CONTROL_HEIGHT, CONTROL_HEIGHT), enabled);
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Button, enabled, label));
    paint_button_background(ui, theme, ButtonKind::Ghost, rect, &response, enabled);
    let color = button_ink(theme, ButtonKind::Ghost, enabled);
    paint_icon(
        ui.painter(),
        icon,
        Rect::from_center_size(rect.center(), vec2(16.0, 16.0)),
        color,
    );
    response.on_hover_text(label)
}

fn allocate_control(ui: &mut Ui, size: egui::Vec2, enabled: bool) -> (Rect, Response) {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    let response = if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    };
    (rect, response)
}

fn button_ink(theme: &Theme, kind: ButtonKind, enabled: bool) -> Color32 {
    match (kind, enabled) {
        (_, false) => theme.text_subtle,
        (ButtonKind::Primary, true) => theme.on_accent,
        (ButtonKind::Secondary, true) => theme.text,
        (ButtonKind::Ghost, true) => theme.text_muted,
    }
}

fn paint_button_background(
    ui: &Ui,
    theme: &Theme,
    kind: ButtonKind,
    rect: Rect,
    response: &Response,
    enabled: bool,
) {
    let painter = ui.painter();
    let lit = enabled && (response.hovered() || response.is_pointer_button_down_on());
    match kind {
        ButtonKind::Primary => {
            let fill = match (enabled, lit) {
                (false, _) => theme.border,
                (true, true) => theme.accent_fill_hover,
                (true, false) => theme.accent_fill,
            };
            painter.rect_filled(rect, RADIUS_SM, fill);
        }
        ButtonKind::Secondary => {
            let fill = if lit { theme.card_hover } else { theme.card };
            let stroke = if enabled {
                theme.border_strong
            } else {
                theme.border
            };
            painter.rect(
                rect,
                RADIUS_SM,
                fill,
                Stroke::new(1.0, stroke),
                StrokeKind::Inside,
            );
        }
        ButtonKind::Ghost => {
            if lit {
                painter.rect_filled(rect, RADIUS_SM, theme.card_hover);
            }
        }
    }
    paint_focus_ring(ui, theme, rect, response);
}

fn paint_focus_ring(ui: &Ui, theme: &Theme, rect: Rect, response: &Response) {
    if response.has_focus() {
        ui.painter().rect_stroke(
            rect.expand(2.0),
            RADIUS_SM + 2,
            Stroke::new(2.0, theme.accent),
            StrokeKind::Outside,
        );
    }
}

fn draw_button(
    ui: &mut Ui,
    theme: &Theme,
    kind: ButtonKind,
    icon: Option<Icon>,
    label: Option<&str>,
    enabled: bool,
    width: Option<f32>,
) -> Response {
    const ICON_SIZE: f32 = 15.0;
    const ICON_GAP: f32 = 7.0;
    const PADDING: f32 = 14.0;

    let font = if kind == ButtonKind::Primary {
        theme::semibold(14.0)
    } else {
        theme::regular(14.0)
    };
    let ink = button_ink(theme, kind, enabled);
    let text = label.unwrap_or_default();
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, ink);
    let icon_width = icon.map_or(0.0, |_| ICON_SIZE + ICON_GAP);
    let content_width = icon_width + galley.size().x;
    let desired_width = width.unwrap_or(content_width + PADDING * 2.0);

    let (rect, response) = allocate_control(ui, vec2(desired_width, CONTROL_HEIGHT), enabled);
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Button, enabled, text));
    paint_button_background(ui, theme, kind, rect, &response, enabled);

    let mut x = rect.center().x - content_width / 2.0;
    if let Some(icon) = icon {
        let icon_rect = Rect::from_min_size(
            pos2(x, rect.center().y - ICON_SIZE / 2.0),
            vec2(ICON_SIZE, ICON_SIZE),
        );
        paint_icon(ui.painter(), icon, icon_rect, ink);
        x += ICON_SIZE + ICON_GAP;
    }
    let top = rect.center().y - galley.size().y / 2.0;
    ui.painter().galley(pos2(x, top), galley, ink);
    response
}

/// A compact segmented control. Returns `true` when the selection changed. The whole
/// control is measured first, so it stays centered in whatever row it sits in.
pub fn segmented<T: Copy + PartialEq>(
    ui: &mut Ui,
    theme: &Theme,
    current: &mut T,
    options: &[(&str, T)],
    enabled: bool,
) -> bool {
    const INSET: f32 = 3.0;
    const GAP: f32 = 2.0;
    const SEGMENT_PADDING: f32 = 22.0;

    let segments: Vec<_> = options
        .iter()
        .map(|(label, value)| {
            let selected = *current == *value;
            let ink = match (enabled, selected) {
                (false, _) => theme.text_subtle,
                (true, true) => theme.accent,
                (true, false) => theme.text_muted,
            };
            let font = if selected {
                theme::semibold(13.0)
            } else {
                theme::regular(13.0)
            };
            let galley = ui.painter().layout_no_wrap((*label).to_owned(), font, ink);
            (*label, *value, selected, ink, galley)
        })
        .collect();
    let width = segments
        .iter()
        .map(|(.., galley)| galley.size().x + SEGMENT_PADDING)
        .sum::<f32>()
        + GAP * segments.len().saturating_sub(1) as f32
        + INSET * 2.0;

    let (rect, _) = ui.allocate_exact_size(vec2(width, CONTROL_HEIGHT), Sense::hover());
    ui.painter().rect(
        rect,
        RADIUS_SM,
        theme.field_bg,
        Stroke::new(1.0, theme.border_strong),
        StrokeKind::Inside,
    );

    let mut changed = false;
    let mut left = rect.left() + INSET;
    for (label, value, selected, ink, galley) in segments {
        let segment_width = galley.size().x + SEGMENT_PADDING;
        let segment = Rect::from_min_size(
            pos2(left, rect.top() + INSET),
            vec2(segment_width, CONTROL_HEIGHT - INSET * 2.0),
        );
        left += segment_width + GAP;

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let response = ui.interact(segment, ui.make_persistent_id(label), sense);
        let response = if enabled {
            response.on_hover_cursor(CursorIcon::PointingHand)
        } else {
            response
        };
        response
            .widget_info(|| WidgetInfo::selected(A11yRole::RadioButton, enabled, selected, label));
        if selected {
            ui.painter()
                .rect_filled(segment, RADIUS_SM - 2, theme.accent_soft);
        } else if enabled && response.hovered() {
            ui.painter()
                .rect_filled(segment, RADIUS_SM - 2, theme.card_hover);
        }
        paint_focus_ring(ui, theme, segment, &response);
        let position = segment.center() - galley.size() / 2.0;
        ui.painter().galley(position, galley, ink);
        if response.clicked() && !selected {
            *current = value;
            changed = true;
        }
    }
    changed
}

/// A soft-filled capsule with a leading status dot, sized to its text.
pub fn status_pill(ui: &mut Ui, theme: &Theme, text: &str, role: Role) {
    let color = theme.role(role);
    capsule(ui, theme, text, color, true);
}

/// A compact category tag without a dot.
pub fn badge(ui: &mut Ui, theme: &Theme, text: &str, color: Color32) {
    capsule(ui, theme, text, color, false);
}

fn capsule(ui: &mut Ui, theme: &Theme, text: &str, color: Color32, dot: bool) {
    const HEIGHT: f32 = 22.0;
    const PADDING: f32 = 9.0;
    const DOT_SPACE: f32 = 12.0;

    let dark = theme.mode == theme::ThemeMode::Dark;
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), theme::regular(12.0), theme.text);
    let dot_space = if dot { DOT_SPACE } else { 0.0 };
    let size = vec2(galley.size().x + PADDING * 2.0 + dot_space, HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Label, true, text));
    let painter = ui.painter();
    painter.rect(
        rect,
        CornerRadius::same(HEIGHT as u8 / 2),
        theme::alpha(color, if dark { 34 } else { 24 }),
        Stroke::new(1.0, theme::alpha(color, 110)),
        StrokeKind::Inside,
    );
    let mut x = rect.left() + PADDING;
    if dot {
        painter.circle_filled(pos2(x + 3.5, rect.center().y), 3.5, color);
        x += DOT_SPACE;
    }
    let top = rect.center().y - galley.size().y / 2.0;
    painter.galley(pos2(x, top), galley, theme.text);
}

/// Lay metric tiles out in one row of equal widths.
pub fn metric_tiles(ui: &mut Ui, theme: &Theme, metrics: &[Metric]) {
    if metrics.is_empty() {
        return;
    }
    ui.columns(metrics.len(), |columns| {
        for (column, metric) in columns.iter_mut().zip(metrics) {
            metric_tile(column, theme, metric);
        }
    });
}

fn metric_tile(ui: &mut Ui, theme: &Theme, metric: &Metric) {
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_MD)
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                RichText::new(&metric.value)
                    .font(theme::semibold(17.0))
                    .color(theme.role(metric.role)),
            );
            ui.label(
                RichText::new(&metric.label)
                    .size(12.0)
                    .color(theme.text_muted),
            );
        });
}

/// A key column and a value column, for short facts in a detail panel.
pub fn kv_row(ui: &mut Ui, theme: &Theme, key: &str, value: &str) {
    const KEY_WIDTH: f32 = 96.0;
    ui.horizontal_top(|ui| {
        let (rect, response) = ui.allocate_exact_size(vec2(KEY_WIDTH, 18.0), Sense::hover());
        // Painted, so it would otherwise not be there for a screen reader.
        response.widget_info(|| WidgetInfo::labeled(A11yRole::Label, true, key));
        ui.painter().text(
            pos2(rect.left(), rect.center().y),
            Align2::LEFT_CENTER,
            key,
            theme::regular(13.0),
            theme.text_muted,
        );
        ui.add(
            egui::Label::new(RichText::new(value).size(13.0).color(theme.text))
                .wrap_mode(TextWrapMode::Wrap),
        );
    });
}

/// A labeled monospace path that wraps instead of overflowing.
pub fn mono_path(ui: &mut Ui, theme: &Theme, label: &str, value: &str) {
    field_label(ui, theme, label);
    ui.add_space(SPACE_XS);
    egui::Frame::new()
        .fill(theme.field_bg)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_SM)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    RichText::new(value)
                        .text_style(TextStyle::Monospace)
                        .color(theme.text),
                )
                .wrap_mode(TextWrapMode::Wrap),
            );
        });
}

/// A full-width notice with a role icon.
pub fn notice(ui: &mut Ui, theme: &Theme, role: Role, message: &str) {
    let color = theme.role(role);
    let dark = theme.mode == theme::ThemeMode::Dark;
    egui::Frame::new()
        .inner_margin(Margin::symmetric(11, 9))
        .fill(theme::alpha(color, if dark { 26 } else { 18 }))
        .stroke(Stroke::new(1.0, theme::alpha(color, 110)))
        .corner_radius(RADIUS_MD)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal_top(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(16.0, 18.0), Sense::hover());
                let icon_rect = Rect::from_center_size(rect.center(), vec2(15.0, 15.0));
                paint_icon(ui.painter(), role_icon(role), icon_rect, color);
                ui.add(
                    egui::Label::new(RichText::new(message).size(13.0).color(theme.text))
                        .wrap_mode(TextWrapMode::Wrap),
                );
            });
        });
}

/// The icon that goes with a status role.
pub fn role_icon(role: Role) -> Icon {
    match role {
        Role::Accent | Role::Neutral => Icon::Info,
        Role::Success => Icon::Check,
        Role::Warning => Icon::Warning,
        Role::Danger => Icon::Error,
    }
}

/// A centered placeholder shown when there is nothing to display yet.
pub fn empty_state(ui: &mut Ui, theme: &Theme, icon: Icon, title: &str, body: &str) {
    ui.vertical_centered(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(48.0, 48.0), Sense::hover());
        let painter = ui.painter();
        painter.circle_filled(rect.center(), 24.0, theme.accent_soft);
        let icon_rect = Rect::from_center_size(rect.center(), vec2(22.0, 22.0));
        paint_icon(painter, icon, icon_rect, theme.accent);
        ui.add_space(SPACE_MD);
        heading(ui, theme, title);
        ui.add_space(2.0);
        ui.add(
            egui::Label::new(RichText::new(body).size(13.0).color(theme.text_muted))
                .wrap_mode(TextWrapMode::Wrap),
        );
    });
}

/// A checkbox row with a title and a line of description; the whole row is the
/// click target. A disabled row reads as a fixed, always-on inclusion.
pub fn option_row(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    title: &str,
    description: &str,
    enabled: bool,
) -> Response {
    const BOX_SIZE: f32 = 18.0;
    const BOX_GAP: f32 = 10.0;
    const PADDING: f32 = SPACE_SM;

    let width = ui.available_width();
    let text_width = (width - BOX_SIZE - BOX_GAP - PADDING * 2.0).max(40.0);
    let title_galley = ui.painter().layout(
        title.to_owned(),
        theme::semibold(13.5),
        theme.text,
        text_width,
    );
    let description_galley = ui.painter().layout(
        description.to_owned(),
        theme::regular(12.5),
        theme.text_muted,
        text_width,
    );
    let height = title_galley.size().y + description_galley.size().y + 2.0 + PADDING * 2.0;
    let (rect, mut response) = allocate_control(ui, vec2(width, height), enabled);
    if enabled && response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }
    let selected = *checked;
    response.widget_info(|| WidgetInfo::selected(A11yRole::CheckBox, enabled, selected, title));

    let painter = ui.painter();
    if enabled && response.hovered() {
        painter.rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    paint_focus_ring(ui, theme, rect, &response);

    let box_rect = Rect::from_min_size(
        pos2(rect.left() + PADDING, rect.top() + PADDING + 1.0),
        vec2(BOX_SIZE, BOX_SIZE),
    );
    let accent = if enabled {
        theme.accent_fill
    } else {
        theme.text_subtle
    };
    if selected {
        painter.rect_filled(box_rect, RADIUS_SM - 2, accent);
        paint_icon(painter, Icon::Check, box_rect.shrink(3.0), theme.on_accent);
    } else {
        painter.rect(
            box_rect,
            RADIUS_SM - 2,
            theme.field_bg,
            Stroke::new(1.5, theme.border_strong),
            StrokeKind::Inside,
        );
    }

    let text_left = box_rect.right() + BOX_GAP;
    let title_height = title_galley.size().y;
    painter.galley(
        pos2(text_left, rect.top() + PADDING),
        title_galley,
        theme.text,
    );
    painter.galley(
        pos2(text_left, rect.top() + PADDING + title_height + 2.0),
        description_galley,
        theme.text_muted,
    );
    response
}

/// How tall a row of a long list is. They are all the same, so that a list of any length can be
/// scrolled with only the rows in view drawn.
pub const LIST_ROW_HEIGHT: f32 = 44.0;

fn paint_list_row_text(
    ui: &Ui,
    theme: &Theme,
    rect: Rect,
    left: f32,
    title: &str,
    detail: &str,
    trailing: Option<&str>,
) {
    const TRAILING_GAP: f32 = SPACE_SM;
    let right = rect.right() - SPACE_SM;
    let trailing_left = trailing.map_or(right, |text| {
        let galley =
            ui.painter()
                .layout_no_wrap(text.to_owned(), theme::regular(12.5), theme.text_muted);
        let top = rect.top() + 7.0;
        let left = right - galley.size().x;
        ui.painter()
            .galley(pos2(left, top), galley, theme.text_muted);
        left
    });
    paint_elided(
        ui,
        title,
        theme::semibold(13.5),
        theme.text,
        left,
        rect.top() + 14.0,
        (trailing_left - TRAILING_GAP - left).max(0.0),
    );
    paint_elided(
        ui,
        detail,
        theme::regular(12.5),
        theme.text_muted,
        left,
        rect.top() + 31.0,
        (right - left).max(0.0),
    );
}

/// A checkbox row of one height, whose title and description are cut with `…` to fit and whose
/// tooltip has all of it, for a list that scrolls by rows. The whole row is the click target, and
/// the space bar works it when it has the focus. A disabled row reads as what it says.
pub fn choice_row(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    title: &str,
    detail: &str,
    trailing: Option<&str>,
    enabled: bool,
) -> Response {
    const BOX_SIZE: f32 = 16.0;
    const BOX_GAP: f32 = 9.0;
    let width = ui.available_width();
    let (rect, mut response) = allocate_control(ui, vec2(width, LIST_ROW_HEIGHT), enabled);
    if enabled && response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }
    let selected = *checked;
    let name = format!("{title}, {detail}");
    response.widget_info(|| WidgetInfo::selected(A11yRole::CheckBox, enabled, selected, &name));
    if enabled && response.hovered() {
        ui.painter().rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    if response.has_focus() {
        ui.painter().rect_stroke(
            rect.shrink(1.0),
            RADIUS_SM,
            Stroke::new(1.5, theme.accent),
            StrokeKind::Inside,
        );
    }
    let box_rect = Rect::from_min_size(
        pos2(rect.left() + SPACE_XS, rect.top() + 6.0),
        vec2(BOX_SIZE, BOX_SIZE),
    );
    paint_checkbox(ui, theme, box_rect, selected, enabled);
    paint_list_row_text(
        ui,
        theme,
        rect,
        box_rect.right() + BOX_GAP,
        title,
        detail,
        trailing,
    );
    response.on_hover_text(format!("{title}\n{detail}"))
}

/// A checkbox with one line of text, sized to it, to sit beside others in a row. What it does is
/// its tooltip and part of its accessible name.
pub fn inline_choice(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    title: &str,
    description: &str,
    enabled: bool,
) -> Response {
    const BOX_SIZE: f32 = 16.0;
    const BOX_GAP: f32 = 7.0;
    let galley = ui
        .painter()
        .layout_no_wrap(title.to_owned(), theme::semibold(13.0), theme.text);
    let size = vec2(
        BOX_SIZE + BOX_GAP + galley.size().x + SPACE_XS,
        CONTROL_HEIGHT,
    );
    let (rect, mut response) = allocate_control(ui, size, enabled);
    if enabled && response.clicked() {
        *checked = !*checked;
        response.mark_changed();
    }
    let selected = *checked;
    let name = format!("{title}, {description}");
    response.widget_info(|| WidgetInfo::selected(A11yRole::CheckBox, enabled, selected, &name));
    if enabled && response.hovered() {
        ui.painter().rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    paint_focus_ring(ui, theme, rect, &response);
    let box_rect = Rect::from_min_size(
        pos2(rect.left() + 2.0, rect.center().y - BOX_SIZE / 2.0),
        vec2(BOX_SIZE, BOX_SIZE),
    );
    paint_checkbox(ui, theme, box_rect, selected, enabled);
    let top = rect.center().y - galley.size().y / 2.0;
    ui.painter()
        .galley(pos2(box_rect.right() + BOX_GAP, top), galley, theme.text);
    response.on_hover_text(description)
}

fn paint_checkbox(ui: &Ui, theme: &Theme, box_rect: Rect, selected: bool, enabled: bool) {
    let accent = if enabled {
        theme.accent_fill
    } else {
        theme.text_subtle
    };
    if selected {
        ui.painter().rect_filled(box_rect, RADIUS_SM - 2, accent);
        paint_icon(
            ui.painter(),
            Icon::Check,
            box_rect.shrink(3.0),
            theme.on_accent,
        );
    } else {
        ui.painter().rect(
            box_rect,
            RADIUS_SM - 2,
            theme.field_bg,
            Stroke::new(
                1.5,
                if enabled {
                    theme.border_strong
                } else {
                    theme.border
                },
            ),
            StrokeKind::Inside,
        );
    }
}

/// A row of a list that has nothing to choose, in the same height and place as [`choice_row`].
pub fn plain_row(ui: &mut Ui, theme: &Theme, title: &str, detail: &str, trailing: Option<&str>) {
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, LIST_ROW_HEIGHT), Sense::hover());
    let name = format!("{title}, {detail}");
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Label, true, &name));
    paint_list_row_text(
        ui,
        theme,
        rect,
        rect.left() + SPACE_XS,
        title,
        detail,
        trailing,
    );
    response.on_hover_text(format!("{title}\n{detail}"));
}

/// A heading that opens and closes what is below it, with a summary at its right. Returns whether
/// it was clicked. The Enter key and the space bar work it when it has the focus.
pub fn disclosure_row(
    ui: &mut Ui,
    theme: &Theme,
    open: bool,
    title: &str,
    summary: &str,
) -> Response {
    const HEIGHT: f32 = 34.0;
    let width = ui.available_width();
    let (rect, response) = allocate_control(ui, vec2(width, HEIGHT), true);
    let name = format!(
        "{title}, {summary}, {}",
        if open { "open" } else { "closed" }
    );
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Button, true, &name));
    if response.hovered() {
        ui.painter().rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    paint_focus_ring(ui, theme, rect, &response);
    let icon_rect =
        Rect::from_center_size(pos2(rect.left() + 11.0, rect.center().y), vec2(11.0, 11.0));
    paint_icon(
        ui.painter(),
        if open {
            Icon::ChevronDown
        } else {
            Icon::ChevronRight
        },
        icon_rect,
        theme.text_muted,
    );
    let summary_galley =
        ui.painter()
            .layout_no_wrap(summary.to_owned(), theme::regular(12.5), theme.text_muted);
    let summary_left = rect.right() - SPACE_XS - summary_galley.size().x;
    let top = rect.center().y - summary_galley.size().y / 2.0;
    ui.painter()
        .galley(pos2(summary_left, top), summary_galley, theme.text_muted);
    let title_left = icon_rect.right() + SPACE_XS;
    paint_elided(
        ui,
        title,
        theme::semibold(13.5),
        theme.text,
        title_left,
        rect.center().y,
        (summary_left - SPACE_SM - title_left).max(0.0),
    );
    response
}

/// A single-line text field at the shared control height.
pub fn text_field(ui: &mut Ui, value: &mut String, hint: &str, width: f32) -> Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(hint)
            .desired_width(width)
            .min_size(vec2(width, CONTROL_HEIGHT))
            .margin(Margin::symmetric(10, 8)),
    )
}

/// A text field with a leading search icon and a clear button.
pub fn search_field(
    ui: &mut Ui,
    theme: &Theme,
    id: egui::Id,
    value: &mut String,
    hint: &str,
    width: f32,
) -> Response {
    const ICON_SIZE: f32 = 14.0;
    let response = ui.add(
        egui::TextEdit::singleline(value)
            .id(id)
            .hint_text(hint)
            .desired_width(width)
            .min_size(vec2(width, CONTROL_HEIGHT))
            .margin(Margin {
                left: 32,
                right: 30,
                top: 8,
                bottom: 8,
            }),
    );
    let center_y = response.rect.center().y;
    let icon_rect = Rect::from_center_size(
        pos2(response.rect.left() + 17.0, center_y),
        vec2(ICON_SIZE, ICON_SIZE),
    );
    paint_icon(ui.painter(), Icon::Search, icon_rect, theme.text_subtle);

    if !value.is_empty() {
        let clear_rect = Rect::from_center_size(
            pos2(response.rect.right() - 16.0, center_y),
            vec2(20.0, 20.0),
        );
        let clear = ui
            .interact(clear_rect, id.with("clear"), Sense::click())
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_hover_text("Clear search");
        clear.widget_info(|| WidgetInfo::labeled(A11yRole::Button, true, "Clear search"));
        if clear.hovered() {
            ui.painter()
                .rect_filled(clear_rect, RADIUS_SM - 2, theme.card_hover);
        }
        paint_icon(
            ui.painter(),
            Icon::Close,
            clear_rect.shrink(5.0),
            theme.text_muted,
        );
        if clear.clicked() {
            value.clear();
        }
    }
    response
}

/// A labeled path input with a Browse button.
pub fn path_field(
    ui: &mut Ui,
    theme: &Theme,
    label: &str,
    value: &mut String,
    hint: &str,
    enabled: bool,
    pick: impl FnOnce() -> Option<PathBuf>,
) {
    const BROWSE_WIDTH: f32 = 86.0;
    field_label(ui, theme, label);
    ui.add_space(SPACE_XS);
    control_row(ui, |ui| {
        let width = (ui.available_width() - BROWSE_WIDTH - SPACE_SM).max(120.0);
        let current = value.clone();
        ui.add_enabled_ui(enabled, |ui| {
            let field = text_field(ui, value, hint, width);
            if !current.is_empty() {
                field.on_hover_text(current);
            }
        });
        if draw_button(
            ui,
            theme,
            ButtonKind::Secondary,
            None,
            Some("Browse"),
            enabled,
            Some(BROWSE_WIDTH),
        )
        .on_hover_text(format!("Choose {}", label.to_lowercase()))
        .clicked()
            && let Some(path) = pick()
        {
            *value = path.display().to_string();
        }
    });
}

/// A slim progress bar. `None` draws an indeterminate sweep for a step whose
/// length is unknown.
pub fn progress_bar(ui: &mut Ui, theme: &Theme, fraction: Option<f32>) {
    const HEIGHT: f32 = 6.0;
    const SWEEP: f32 = 0.3;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), HEIGHT), Sense::hover());
    let painter = ui.painter();
    let radius = CornerRadius::same(HEIGHT as u8 / 2);
    painter.rect_filled(rect, radius, theme.border);
    let (start, end) = match fraction {
        Some(fraction) => (0.0, fraction.clamp(0.0, 1.0)),
        None => {
            let phase = (ui.input(|input| input.time) * 0.7).fract() as f32;
            let start = phase * (1.0 + SWEEP) - SWEEP;
            (start.max(0.0), (start + SWEEP).min(1.0))
        }
    };
    if end > start {
        let fill = Rect::from_min_max(
            pos2(rect.left() + rect.width() * start, rect.top()),
            pos2(rect.left() + rect.width() * end, rect.bottom()),
        );
        painter.rect_filled(fill, radius, theme.accent_fill);
    }
}

/// The application icon for a window's title bar.
pub fn window_icon() -> Option<std::sync::Arc<egui::IconData>> {
    eframe::icon_data::from_png_bytes(include_bytes!("../assets/winappmigrator.png"))
        .ok()
        .map(std::sync::Arc::new)
}

/// Paint the app brand mark inside `rect`.
pub fn paint_logo(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    painter.rect_filled(rect, RADIUS_SM, theme.accent_fill);
    let inset = rect.shrink(rect.width() * 0.28);
    let stroke = Stroke::new(2.0, theme.on_accent);
    // A right-pointing migration chevron pair.
    let mid_y = inset.center().y;
    painter.add(Shape::line(
        vec![
            pos2(inset.left(), inset.top()),
            pos2(inset.center().x, mid_y),
            pos2(inset.left(), inset.bottom()),
        ],
        stroke,
    ));
    painter.add(Shape::line(
        vec![
            pos2(inset.center().x, inset.top()),
            pos2(inset.right(), mid_y),
            pos2(inset.center().x, inset.bottom()),
        ],
        stroke,
    ));
}

/// Draw a glyph inside `rect` using vector primitives.
pub fn paint_icon(painter: &egui::Painter, icon: Icon, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.6, color);
    let at = |x: f32, y: f32| {
        pos2(
            rect.left() + rect.width() * x,
            rect.top() + rect.height() * y,
        )
    };
    let line = |points: &[(f32, f32)]| {
        let points = points.iter().map(|(x, y)| at(*x, *y)).collect();
        painter.add(Shape::line(points, stroke));
    };
    let arc = |center: egui::Pos2, radius: f32, from: f32, to: f32| {
        const STEPS: usize = 24;
        let points = (0..=STEPS)
            .map(|step| {
                let angle = from + (to - from) * step as f32 / STEPS as f32;
                center + egui::Vec2::angled(angle) * radius
            })
            .collect();
        painter.add(Shape::line(points, stroke));
    };
    let unit = rect.width().min(rect.height());
    let tau = std::f32::consts::TAU;

    match icon {
        Icon::Apps => {
            painter.rect_stroke(rect, 3, stroke, StrokeKind::Inside);
            line(&[(0.0, 0.34), (1.0, 0.34)]);
            line(&[(0.18, 0.56), (0.82, 0.56)]);
            line(&[(0.18, 0.76), (0.82, 0.76)]);
        }
        Icon::Search => {
            painter.circle_stroke(at(0.42, 0.42), unit * 0.36, stroke);
            line(&[(0.68, 0.68), (0.98, 0.98)]);
        }
        Icon::Refresh => {
            // A clockwise arc ending in an arrowhead that follows the curve.
            let end = tau * 0.88;
            arc(rect.center(), unit * 0.40, tau * 0.05, end);
            let tip = rect.center() + egui::Vec2::angled(end) * (unit * 0.40);
            let along = egui::Vec2::angled(end + tau * 0.25);
            let across = egui::Vec2::angled(end);
            let size = unit * 0.30;
            painter.add(Shape::line(
                vec![
                    tip - along * size + across * size * 0.75,
                    tip,
                    tip - along * size - across * size * 0.75,
                ],
                stroke,
            ));
        }
        Icon::Sun => {
            painter.circle_stroke(rect.center(), unit * 0.22, stroke);
            for ray in 0..8 {
                let direction = egui::Vec2::angled(tau * ray as f32 / 8.0);
                painter.line_segment(
                    [
                        rect.center() + direction * (unit * 0.36),
                        rect.center() + direction * (unit * 0.50),
                    ],
                    stroke,
                );
            }
        }
        Icon::Moon => {
            arc(rect.center(), unit * 0.44, 0.55, tau * 0.84);
            arc(at(0.68, 0.36), unit * 0.34, 1.05, tau * 0.61);
        }
        Icon::Folder => {
            line(&[
                (0.0, 0.88),
                (0.0, 0.14),
                (0.38, 0.14),
                (0.50, 0.30),
                (1.0, 0.30),
                (1.0, 0.88),
                (0.0, 0.88),
            ]);
        }
        Icon::Copy => {
            let back = Rect::from_min_max(at(0.32, 0.0), at(1.0, 0.68));
            let front = Rect::from_min_max(at(0.0, 0.32), at(0.68, 1.0));
            painter.rect_stroke(back, 2, stroke, StrokeKind::Inside);
            painter.rect_stroke(front, 2, stroke, StrokeKind::Inside);
        }
        Icon::Close => {
            line(&[(0.1, 0.1), (0.9, 0.9)]);
            line(&[(0.9, 0.1), (0.1, 0.9)]);
        }
        Icon::Check => {
            line(&[(0.08, 0.54), (0.38, 0.84), (0.94, 0.18)]);
        }
        Icon::Warning => {
            line(&[(0.5, 0.06), (1.0, 0.94), (0.0, 0.94), (0.5, 0.06)]);
            line(&[(0.5, 0.38), (0.5, 0.64)]);
            painter.circle_filled(at(0.5, 0.78), unit * 0.05, color);
        }
        Icon::Error => {
            painter.circle_stroke(rect.center(), unit * 0.47, stroke);
            line(&[(0.34, 0.34), (0.66, 0.66)]);
            line(&[(0.66, 0.34), (0.34, 0.66)]);
        }
        Icon::Info => {
            painter.circle_stroke(rect.center(), unit * 0.47, stroke);
            line(&[(0.5, 0.46), (0.5, 0.74)]);
            painter.circle_filled(at(0.5, 0.29), unit * 0.05, color);
        }
        Icon::Drive => {
            painter.rect_stroke(
                Rect::from_min_max(at(0.0, 0.2), at(1.0, 0.8)),
                3,
                stroke,
                StrokeKind::Inside,
            );
            line(&[(0.14, 0.5), (0.5, 0.5)]);
            painter.circle_filled(at(0.8, 0.5), unit * 0.06, color);
        }
        Icon::Monitor => {
            painter.rect_stroke(
                Rect::from_min_max(at(0.0, 0.08), at(1.0, 0.72)),
                2,
                stroke,
                StrokeKind::Inside,
            );
            line(&[(0.5, 0.72), (0.5, 0.92)]);
            line(&[(0.26, 0.92), (0.74, 0.92)]);
        }
        Icon::ChevronUp => {
            line(&[(0.14, 0.66), (0.5, 0.3), (0.86, 0.66)]);
        }
        Icon::ChevronDown => {
            line(&[(0.14, 0.34), (0.5, 0.7), (0.86, 0.34)]);
        }
        Icon::ChevronRight => {
            line(&[(0.34, 0.14), (0.7, 0.5), (0.34, 0.86)]);
        }
    }
}

/// Format a byte count for people.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// Format a duration as `mm:ss`, or `h:mm:ss` past an hour.
pub fn format_elapsed(seconds: u64) -> String {
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

/// Space between stacked sections.
pub fn section_gap(ui: &mut Ui) {
    ui.add_space(SPACE_MD);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_read_in_the_largest_whole_unit() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(format_bytes(3 * 1024 * 1024 * 1024 / 2), "1.5 GB");
    }

    #[test]
    fn elapsed_time_adds_hours_only_when_needed() {
        assert_eq!(format_elapsed(0), "00:00");
        assert_eq!(format_elapsed(65), "01:05");
        assert_eq!(format_elapsed(3661), "1:01:01");
    }

    #[test]
    fn window_icon_decodes() {
        assert!(window_icon().is_some());
    }
}
