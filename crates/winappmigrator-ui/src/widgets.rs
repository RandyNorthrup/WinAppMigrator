//! Reusable, theme-aware UI building blocks shared by the app and the installer.

use std::path::PathBuf;

use eframe::egui::Role as A11yRole;
use eframe::egui::{
    self, Color32, CornerRadius, CursorIcon, FontId, Margin, Rect, Response, RichText, Sense,
    Shape, Stroke, StrokeKind, TextStyle, TextWrapMode, Ui, WidgetInfo, pos2, vec2,
};

use crate::bidi::{self, TextBlock, right_to_left};
use crate::theme::{
    self, CONTROL_HEIGHT, RADIUS_LG, RADIUS_MD, RADIUS_SM, Role, SPACE_LG, SPACE_MD, SPACE_SM,
    SPACE_XS, Theme,
};
use winappmigrator_i18n::t;

/// Positions inside a widget, worked out as for a left-to-right window, and turned around inside it for a
/// right-to-left one.
#[derive(Clone, Copy, Debug)]
pub struct Mirror {
    left: f32,
    right: f32,
    on: bool,
}

impl Mirror {
    /// Mirroring inside `rect`, when the language in use reads right to left.
    pub fn of(rect: Rect) -> Self {
        Self {
            left: rect.left(),
            right: rect.right(),
            on: right_to_left(),
        }
    }

    /// Where `x` is.
    pub fn x(self, x: f32) -> f32 {
        if self.on {
            self.left + self.right - x
        } else {
            x
        }
    }

    /// Where `rect` is.
    pub fn rect(self, rect: Rect) -> Rect {
        if self.on {
            Rect::from_x_y_ranges(self.x(rect.right())..=self.x(rect.left()), rect.y_range())
        } else {
            rect
        }
    }

    /// The left edge of something `width` wide that starts at `x`.
    pub fn left_of(self, x: f32, width: f32) -> f32 {
        if self.on { self.x(x) - width } else { x }
    }
}

/// A row that runs the way the language reads, centered on its line.
fn reading_row() -> egui::Layout {
    if right_to_left() {
        egui::Layout::right_to_left(egui::Align::Center)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    }
}

/// Tell screen readers where a part of the window is. egui gives a panel a node in the accessibility tree with no
/// bounds, which UI Automation's tools report as an error.
pub fn mark_pane(ui: &Ui) {
    // What is drawn is in a frame inside the panel; the panel's own node is the one with no bounds.
    let Some(panel) = ui
        .stack()
        .iter()
        .find(|frame| frame.kind().is_some_and(|kind| kind.is_panel()))
    else {
        return;
    };
    let rect = panel.max_rect;
    let _marked = ui.ctx().accesskit_node_builder(panel.unique_id, |node| {
        node.set_bounds(egui::accesskit::Rect {
            x0: rect.min.x.into(),
            y0: rect.min.y.into(),
            x1: rect.max.x.into(),
            y1: rect.max.y.into(),
        });
    });
}

/// A text field's hint, laid out in both directions when it needs to be: egui would lay "Search (Ctrl+F)" in Arabic
/// as one right-to-left run, and turn the Latin letters around. It is given laid out and already shortened to `width`:
/// egui shortens a hint that does not fit by cutting its laid-out text, which on a right-to-left one, seen on a real
/// Windows, left the Arabic letters reversed and unjoined.
fn field_hint(ui: &Ui, hint: &str, width: f32) -> egui::WidgetText {
    if !bidi::needs_bidi(hint, winappmigrator_i18n::direction()) {
        return hint.into();
    }
    let font = TextStyle::Body.resolve(ui.style());
    let color = ui.visuals().weak_text_color();
    egui::WidgetText::Galley(bidi::elided_galley(ui.painter(), hint, font, color, width))
}

/// A field whose text, and hint, start at the right edge in a right-to-left window, as Windows' own fields there do.
/// In a left-to-right one egui's own alignment is kept.
fn aligned_for_language<'t>(edit: egui::TextEdit<'t>, ui: &Ui) -> egui::TextEdit<'t> {
    if right_to_left() {
        edit.align(egui::Align2([
            egui::Align::RIGHT,
            ui.layout().vertical_align(),
        ]))
    } else {
        edit
    }
}

/// Give a control the name a screen reader says for it.
fn name_control(ui: &Ui, response: &Response, name: &str) {
    if name.is_empty() {
        return;
    }
    let name = winappmigrator_i18n::without_isolation(name).into_owned();
    let _named = ui
        .ctx()
        .accesskit_node_builder(response.id, |node| node.set_label(name));
}

/// What a page holds, its lines starting at the side the language starts reading from. Rows inside it, made with
/// `ui.horizontal`, run that way too.
pub fn page<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    mark_pane(ui);
    if right_to_left() {
        ui.with_layout(egui::Layout::top_down(egui::Align::Max), contents)
            .inner
    } else {
        contents(ui)
    }
}

/// A column whose lines start at the side the language starts reading from.
pub fn column<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    if right_to_left() {
        ui.with_layout(egui::Layout::top_down(egui::Align::Max), contents)
            .inner
    } else {
        ui.vertical(contents).inner
    }
}

/// What goes at the far end of a row, after everything else in it: at its right, or its left in a right-to-left
/// window. The first thing added is the one furthest along.
pub fn trailing<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    let layout = if right_to_left() {
        egui::Layout::left_to_right(egui::Align::Center)
    } else {
        egui::Layout::right_to_left(egui::Align::Center)
    };
    ui.with_layout(layout, contents).inner
}

/// The font and color `text` is drawn in.
fn font_and_color(ui: &Ui, text: &RichText) -> (FontId, Color32) {
    let job = egui::WidgetText::from(text.clone()).into_layout_job(
        ui.style(),
        egui::FontSelection::Default,
        egui::Align::Min,
    );
    job.sections.first().map_or_else(
        || {
            (
                TextStyle::Body.resolve(ui.style()),
                ui.visuals().text_color(),
            )
        },
        |section| {
            let color = if section.format.color == Color32::PLACEHOLDER {
                ui.visuals().text_color()
            } else {
                section.format.color
            };
            (section.format.font_id.clone(), color)
        },
    )
}

/// A label, read in the direction of the language in use. It wraps where an egui label would.
pub fn label(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    let text = text.into();
    if !bidi::needs_bidi(text.text(), winappmigrator_i18n::direction()) {
        return ui.label(text);
    }
    let wrap = ui.wrap_mode() == TextWrapMode::Wrap;
    let (font, color) = font_and_color(ui, &text);
    bidi::label(ui, text.text(), font, color, wrap)
}

/// A label that wraps to the width there is, read in the direction of the language in use.
pub fn wrapped_label(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    let text = text.into();
    if !bidi::needs_bidi(text.text(), winappmigrator_i18n::direction()) {
        return ui.add(egui::Label::new(text).wrap_mode(TextWrapMode::Wrap));
    }
    let (font, color) = font_and_color(ui, &text);
    bidi::label(ui, text.text(), font, color, true)
}

/// A label of one line, cut short with `…` to the width there is, all of it in its tooltip when it is cut.
pub fn truncated_label(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    let text = text.into();
    if !bidi::needs_bidi(text.text(), winappmigrator_i18n::direction()) {
        return ui.add(egui::Label::new(text).truncate());
    }
    let (font, color) = font_and_color(ui, &text);
    let width = ui.available_width();
    let galley = bidi::elided_galley(ui.painter(), text.text(), font.clone(), color, width);
    let whole = bidi::line_galley(ui.painter(), text.text(), font, color);
    let (rect, response) = ui.allocate_exact_size(vec2(width, galley.size().y), Sense::hover());
    let left = Mirror::of(rect).left_of(rect.left(), galley.size().x);
    ui.painter().galley(pos2(left, rect.top()), galley, color);
    let full = winappmigrator_i18n::without_isolation(text.text()).into_owned();
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Label, true, &full));
    if whole.size().x > width {
        tooltip(response, full)
    } else {
        response
    }
}

/// A line that says how something is going. A screen reader says it again when it changes, once it has finished what
/// it is saying.
pub fn status_line(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    let response = label(ui, text);
    mark_live(ui, &response, false);
    response
}

/// Have a screen reader say what `response` says whenever it changes: at once if `urgent`, else when it is free to.
fn mark_live(ui: &Ui, response: &Response, urgent: bool) {
    let live = if urgent {
        egui::accesskit::Live::Assertive
    } else {
        egui::accesskit::Live::Polite
    };
    let _marked = ui
        .ctx()
        .accesskit_node_builder(response.id, |node| node.set_live(live));
}

/// Whether no tooltip has been shown yet this pass for the control with the focus. A control given two tooltips, its
/// own and one its caller adds, shows the first while it has the focus; on hover egui shows both.
fn first_focus_tooltip_this_pass(response: &Response) -> bool {
    let pass = response.ctx.cumulative_pass_nr();
    let key = response.id.with("winappmigrator-focus-tooltip");
    response.ctx.data_mut(|data| {
        let shown_in = data.get_temp_mut_or_insert_with(key, || u64::MAX);
        let first = *shown_in != pass;
        *shown_in = pass;
        first
    })
}

/// A tooltip, read in the direction of the language in use. It shows on hover, and also while the control has the
/// keyboard's focus, so that what is cut short can be read in full without a pointer; Escape takes the focus, and it,
/// away.
pub fn tooltip(response: Response, text: impl Into<String>) -> Response {
    let text = text.into();
    if response.has_focus() && first_focus_tooltip_this_pass(&response) {
        let shown = text.clone();
        response.show_tooltip_ui(move |ui| {
            let font = TextStyle::Body.resolve(ui.style());
            let color = ui.visuals().text_color();
            bidi::label(ui, &shown, font, color, false);
        });
    }
    if !bidi::needs_bidi(&text, winappmigrator_i18n::direction()) {
        return response.on_hover_text(text);
    }
    response.on_hover_ui(move |ui| {
        let font = TextStyle::Body.resolve(ui.style());
        let color = ui.visuals().text_color();
        let wide = ui
            .painter()
            .layout_no_wrap(text.clone(), font.clone(), color)
            .size()
            .x
            > ui.available_width();
        bidi::label(ui, &text, font, color, wide);
    })
}

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
    ChevronUp,
    ChevronDown,
    ChevronRight,
    ChevronLeft,
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
            label(
                ui,
                RichText::new(subtitle).size(12.5).color(theme.text_muted),
            );
        }
        ui.add_space(SPACE_MD);
        contents(ui)
    })
}

/// A row, in the direction the language reads, centered on the shared control height, so labels line up with
/// the buttons and fields beside them.
pub fn control_row<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), CONTROL_HEIGHT),
        reading_row(),
        contents,
    )
    .inner
}

/// A semibold heading.
pub fn heading(ui: &mut Ui, theme: &Theme, text: &str) {
    let response = wrapped_label(
        ui,
        RichText::new(text)
            .font(theme::semibold(15.0))
            .color(theme.text),
    );
    // A screen reader can move from heading to heading. egui gives a label's text as its value; a heading's is its
    // name.
    let name = winappmigrator_i18n::without_isolation(text).into_owned();
    let _marked = ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(A11yRole::Heading);
        node.set_label(name);
    });
}

/// A small label above a field or a group of controls.
pub fn field_label(ui: &mut Ui, theme: &Theme, text: &str) {
    label(
        ui,
        RichText::new(text)
            .font(theme::semibold(12.5))
            .color(theme.text_muted),
    );
}

/// Muted supporting text that wraps to the available width.
pub fn hint(ui: &mut Ui, theme: &Theme, text: &str) {
    wrapped_label(ui, RichText::new(text).size(12.5).color(theme.text_muted));
}

/// Whether a line of text fits in this much room, which is when there is no need to offer all of it
/// when it is hovered over.
pub fn text_fits(ui: &Ui, text: &str, font: FontId, width: f32) -> bool {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font, Color32::PLACEHOLDER)
        .size()
        .x
        <= width
}

/// Paint one line of text from `start`, which is its left edge, or its right edge in a right-to-left window, centered
/// vertically on `center_y`, cut with `…` at `max_width`.
pub fn paint_elided(
    ui: &Ui,
    text: &str,
    font: FontId,
    color: Color32,
    start: f32,
    center_y: f32,
    max_width: f32,
) {
    let galley = bidi::elided_galley(ui.painter(), text, font, color, max_width);
    let top = center_y - galley.size().y / 2.0;
    let left = if right_to_left() {
        start - galley.size().x
    } else {
        start
    };
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
    tooltip(response, label)
}

fn allocate_control(ui: &mut Ui, size: egui::Vec2, enabled: bool) -> (Rect, Response) {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    bring_into_view_on_focus(&response);
    let response = if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    };
    (rect, response)
}

/// What the Tab key moves to is scrolled into view, so the focus is never somewhere that cannot be seen.
pub fn bring_into_view_on_focus(response: &Response) {
    if response.gained_focus() {
        response.scroll_to_me(None);
    }
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

/// Room between a button's edge and its text.
const BUTTON_PADDING: f32 = 14.0;

fn button_font(kind: ButtonKind) -> FontId {
    if kind == ButtonKind::Primary {
        theme::semibold(14.0)
    } else {
        theme::regular(14.0)
    }
}

/// How wide a [`button`] with this label is, so that what shares its row can be given the room it leaves.
pub fn button_width(ui: &Ui, kind: ButtonKind, label: &str) -> f32 {
    bidi::line_galley(ui.painter(), label, button_font(kind), Color32::PLACEHOLDER)
        .size()
        .x
        + BUTTON_PADDING * 2.0
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

    let ink = button_ink(theme, kind, enabled);
    let text = label.unwrap_or_default();
    let galley = bidi::line_galley(ui.painter(), text, button_font(kind), ink);
    let icon_width = icon.map_or(0.0, |_| ICON_SIZE + ICON_GAP);
    let content_width = icon_width + galley.size().x;
    let desired_width = width.unwrap_or(content_width + BUTTON_PADDING * 2.0);

    let (rect, response) = allocate_control(ui, vec2(desired_width, CONTROL_HEIGHT), enabled);
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Button, enabled, text));
    paint_button_background(ui, theme, kind, rect, &response, enabled);

    let mirror = Mirror::of(rect);
    let mut x = rect.center().x - content_width / 2.0;
    if let Some(icon) = icon {
        let icon_rect = Rect::from_min_size(
            pos2(x, rect.center().y - ICON_SIZE / 2.0),
            vec2(ICON_SIZE, ICON_SIZE),
        );
        paint_icon(ui.painter(), icon, mirror.rect(icon_rect), ink);
        x += ICON_SIZE + ICON_GAP;
    }
    let top = rect.center().y - galley.size().y / 2.0;
    let left = mirror.left_of(x, galley.size().x);
    ui.painter().galley(pos2(left, top), galley, ink);
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
            let galley = bidi::line_galley(ui.painter(), label, font, ink);
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
    let mirror = Mirror::of(rect);
    let mut left = rect.left() + INSET;
    for (label, value, selected, ink, galley) in segments {
        let segment_width = galley.size().x + SEGMENT_PADDING;
        let segment = mirror.rect(Rect::from_min_size(
            pos2(left, rect.top() + INSET),
            vec2(segment_width, CONTROL_HEIGHT - INSET * 2.0),
        ));
        left += segment_width + GAP;

        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let response = ui.interact(segment, ui.make_persistent_id(label), sense);
        bring_into_view_on_focus(&response);
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
    let galley = bidi::line_galley(ui.painter(), text, theme::regular(12.0), theme.text);
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
    let mirror = Mirror::of(rect);
    let mut x = rect.left() + PADDING;
    if dot {
        painter.circle_filled(pos2(mirror.x(x + 3.5), rect.center().y), 3.5, color);
        x += DOT_SPACE;
    }
    let top = rect.center().y - galley.size().y / 2.0;
    let left = mirror.left_of(x, galley.size().x);
    painter.galley(pos2(left, top), galley, theme.text);
}

/// How much room a row of tiles takes up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileSize {
    Normal,
    /// For a window short of room, with what is on either side of them more use than the space.
    Tight,
}

/// Lay metric tiles out in one row of equal widths.
pub fn metric_tiles(ui: &mut Ui, theme: &Theme, metrics: &[Metric]) {
    metric_tiles_sized(ui, theme, metrics, TileSize::Normal);
}

/// [`metric_tiles`], of a given size.
pub fn metric_tiles_sized(ui: &mut Ui, theme: &Theme, metrics: &[Metric], size: TileSize) {
    if metrics.is_empty() {
        return;
    }
    ui.columns(metrics.len(), |columns| {
        // The first is where reading starts.
        if right_to_left() {
            columns.reverse();
        }
        for (column, metric) in columns.iter_mut().zip(metrics) {
            metric_tile(column, theme, metric, size);
        }
    });
}

fn metric_tile(ui: &mut Ui, theme: &Theme, metric: &Metric, size: TileSize) {
    let (margin, value_size) = match size {
        TileSize::Normal => (8, 17.0),
        TileSize::Tight => (3, 14.5),
    };
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(RADIUS_MD)
        .inner_margin(Margin::symmetric(10, margin))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 2.0;
            page(ui, |ui| {
                label(
                    ui,
                    RichText::new(&metric.value)
                        .font(theme::semibold(value_size))
                        .color(theme.role(metric.role)),
                );
                label(
                    ui,
                    RichText::new(&metric.label)
                        .size(12.0)
                        .color(theme.text_muted),
                );
            });
        });
}

/// A key column and a value column, for short facts in a detail panel.
pub fn kv_row(ui: &mut Ui, theme: &Theme, key: &str, value: &str) {
    const KEY_WIDTH: f32 = 96.0;
    ui.horizontal_top(|ui| {
        let (rect, response) = ui.allocate_exact_size(vec2(KEY_WIDTH, 18.0), Sense::hover());
        // Painted, so it would otherwise not be there for a screen reader.
        response.widget_info(|| WidgetInfo::labeled(A11yRole::Label, true, key));
        let key_text = TextBlock::new(
            ui.painter(),
            key,
            theme::regular(13.0),
            theme.text_muted,
            None,
        );
        let start = if right_to_left() {
            rect.right()
        } else {
            rect.left()
        };
        key_text.paint(
            ui.painter(),
            start,
            rect.center().y - key_text.size().y / 2.0,
            theme.text_muted,
        );
        wrapped_label(ui, RichText::new(value).size(13.0).color(theme.text));
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
                let said = wrapped_label(ui, RichText::new(message).size(13.0).color(theme.text));
                // It is news when it appears or changes; a problem is said at once.
                mark_live(ui, &said, role == Role::Danger);
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
        wrapped_label(ui, RichText::new(body).size(13.0).color(theme.text_muted));
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
    let title_text = TextBlock::new(
        ui.painter(),
        title,
        theme::semibold(13.5),
        theme.text,
        Some(text_width),
    );
    let description_text = TextBlock::new(
        ui.painter(),
        description,
        theme::regular(12.5),
        theme.text_muted,
        Some(text_width),
    );
    let height = title_text.size().y + description_text.size().y + 2.0 + PADDING * 2.0;
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

    let mirror = Mirror::of(rect);
    let box_rect = Rect::from_min_size(
        pos2(rect.left() + PADDING, rect.top() + PADDING + 1.0),
        vec2(BOX_SIZE, BOX_SIZE),
    );
    let text_left = box_rect.right() + BOX_GAP;
    let box_rect = mirror.rect(box_rect);
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

    let start = mirror.x(text_left);
    title_text.paint(painter, start, rect.top() + PADDING, theme.text);
    description_text.paint(
        painter,
        start,
        rect.top() + PADDING + title_text.size().y + 2.0,
        theme.text_muted,
    );
    response
}

/// Between a text field's edge and its text.
const TEXT_FIELD_MARGIN_X: i8 = 10;

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
    let mirror = Mirror::of(rect);
    let right = rect.right() - SPACE_SM;
    let trailing_left = trailing.map_or(right, |text| {
        let galley = bidi::line_galley(ui.painter(), text, theme::regular(12.5), theme.text_muted);
        let top = rect.top() + 7.0;
        let left = right - galley.size().x;
        ui.painter().galley(
            pos2(
                mirror
                    .rect(Rect::from_x_y_ranges(left..=right, top..=top))
                    .left(),
                top,
            ),
            galley,
            theme.text_muted,
        );
        left
    });
    paint_elided(
        ui,
        title,
        theme::semibold(13.5),
        theme.text,
        mirror.x(left),
        rect.top() + 14.0,
        (trailing_left - TRAILING_GAP - left).max(0.0),
    );
    paint_elided(
        ui,
        detail,
        theme::regular(12.5),
        theme.text_muted,
        mirror.x(left),
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
    let name = winappmigrator_i18n::join_list(&[title, detail]);
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
    paint_checkbox(
        ui,
        theme,
        Mirror::of(rect).rect(box_rect),
        selected,
        enabled,
    );
    paint_list_row_text(
        ui,
        theme,
        rect,
        box_rect.right() + BOX_GAP,
        title,
        detail,
        trailing,
    );
    tooltip(response, format!("{title}\n{detail}"))
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
    let galley = bidi::line_galley(ui.painter(), title, theme::semibold(13.0), theme.text);
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
    let name = winappmigrator_i18n::join_list(&[title, description]);
    response.widget_info(|| WidgetInfo::selected(A11yRole::CheckBox, enabled, selected, &name));
    if enabled && response.hovered() {
        ui.painter().rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    paint_focus_ring(ui, theme, rect, &response);
    let mirror = Mirror::of(rect);
    let box_rect = Rect::from_min_size(
        pos2(rect.left() + 2.0, rect.center().y - BOX_SIZE / 2.0),
        vec2(BOX_SIZE, BOX_SIZE),
    );
    paint_checkbox(ui, theme, mirror.rect(box_rect), selected, enabled);
    let top = rect.center().y - galley.size().y / 2.0;
    let left = mirror.left_of(box_rect.right() + BOX_GAP, galley.size().x);
    ui.painter().galley(pos2(left, top), galley, theme.text);
    tooltip(response, description)
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
    let name = winappmigrator_i18n::join_list(&[title, detail]);
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
    tooltip(response, format!("{title}\n{detail}"));
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
    let name = t!(
        "ui-disclosure",
        title = title,
        summary = summary,
        state = if open { "open" } else { "closed" }
    );
    response.widget_info(|| WidgetInfo::labeled(A11yRole::Button, true, &name));
    if response.hovered() {
        ui.painter().rect_filled(rect, RADIUS_SM, theme.card_hover);
    }
    paint_focus_ring(ui, theme, rect, &response);
    let mirror = Mirror::of(rect);
    let icon_rect =
        Rect::from_center_size(pos2(rect.left() + 11.0, rect.center().y), vec2(11.0, 11.0));
    // Closed, it points the way the language reads.
    let closed = if right_to_left() {
        Icon::ChevronLeft
    } else {
        Icon::ChevronRight
    };
    paint_icon(
        ui.painter(),
        if open { Icon::ChevronDown } else { closed },
        mirror.rect(icon_rect),
        theme.text_muted,
    );
    let summary_galley = bidi::line_galley(
        ui.painter(),
        summary,
        theme::regular(12.5),
        theme.text_muted,
    );
    let summary_left = rect.right() - SPACE_XS - summary_galley.size().x;
    let top = rect.center().y - summary_galley.size().y / 2.0;
    let summary_rect = Rect::from_min_size(pos2(summary_left, top), summary_galley.size());
    ui.painter().galley(
        mirror.rect(summary_rect).min,
        summary_galley,
        theme.text_muted,
    );
    let title_left = icon_rect.right() + SPACE_XS;
    paint_elided(
        ui,
        title,
        theme::semibold(13.5),
        theme.text,
        mirror.x(title_left),
        rect.center().y,
        (summary_left - SPACE_SM - title_left).max(0.0),
    );
    response
}

/// A single-line text field at the shared control height.
pub fn text_field(ui: &mut Ui, value: &mut String, hint: &str, width: f32) -> Response {
    let edit = egui::TextEdit::singleline(value)
        .hint_text(field_hint(
            ui,
            hint,
            width - 2.0 * f32::from(TEXT_FIELD_MARGIN_X),
        ))
        .desired_width(width)
        .min_size(vec2(width, CONTROL_HEIGHT))
        .margin(Margin::symmetric(TEXT_FIELD_MARGIN_X, 8));
    let response = ui.add(aligned_for_language(edit, ui));
    name_control(ui, &response, hint);
    response
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
    const ICON_SIDE: i8 = 32;
    const CLEAR_SIDE: i8 = 30;
    // The icon goes where reading starts, the clear button at the other end.
    let (left, right) = if right_to_left() {
        (CLEAR_SIDE, ICON_SIDE)
    } else {
        (ICON_SIDE, CLEAR_SIDE)
    };
    let edit = egui::TextEdit::singleline(value)
        .id(id)
        .hint_text(field_hint(ui, hint, width - f32::from(left + right)))
        .desired_width(width)
        .min_size(vec2(width, CONTROL_HEIGHT))
        .margin(Margin {
            left,
            right,
            top: 8,
            bottom: 8,
        });
    let response = ui.add(aligned_for_language(edit, ui));
    name_control(ui, &response, hint);
    let mirror = Mirror::of(response.rect);
    let center_y = response.rect.center().y;
    let icon_rect = mirror.rect(Rect::from_center_size(
        pos2(response.rect.left() + 17.0, center_y),
        vec2(ICON_SIZE, ICON_SIZE),
    ));
    paint_icon(ui.painter(), Icon::Search, icon_rect, theme.text_subtle);

    if !value.is_empty() {
        let clear_rect = mirror.rect(Rect::from_center_size(
            pos2(response.rect.right() - 16.0, center_y),
            vec2(theme::SMALLEST_TARGET, theme::SMALLEST_TARGET),
        ));
        let clear = tooltip(
            ui.interact(clear_rect, id.with("clear"), Sense::click())
                .on_hover_cursor(CursorIcon::PointingHand),
            t!("ui-clear-search"),
        );
        clear.widget_info(|| WidgetInfo::labeled(A11yRole::Button, true, t!("ui-clear-search")));
        if clear.hovered() {
            ui.painter()
                .rect_filled(clear_rect, RADIUS_SM - 2, theme.card_hover);
        }
        paint_icon(
            ui.painter(),
            Icon::Close,
            clear_rect.shrink(7.0),
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
    const FIELD_MARGINS: f32 = 2.0 * TEXT_FIELD_MARGIN_X as f32;
    field_label(ui, theme, label);
    ui.add_space(SPACE_XS);
    control_row(ui, |ui| {
        let width = (ui.available_width() - BROWSE_WIDTH - SPACE_SM).max(120.0);
        let current = value.clone();
        ui.add_enabled_ui(enabled, |ui| {
            let field = text_field(ui, value, hint, width);
            // Named by its label, as a person reads it above the field.
            name_control(ui, &field, label);
            // Only when some of it cannot be seen. It would otherwise only cover what is below.
            let font = egui::TextStyle::Body.resolve(ui.style());
            if !text_fits(ui, &current, font, width - FIELD_MARGINS) {
                tooltip(field, current);
            }
        });
        if tooltip(
            draw_button(
                ui,
                theme,
                ButtonKind::Secondary,
                None,
                Some(&t!("ui-browse")),
                enabled,
                Some(BROWSE_WIDTH),
            ),
            t!("ui-browse-tooltip", field = label),
        )
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
    // With Animation effects off, a step of unknown length shows the whole bar, softer, rather than a sweep.
    if fraction.is_none() && !crate::system_settings::animations_allowed(ui.ctx()) {
        painter.rect_filled(rect, radius, theme::alpha(theme.accent_fill, STILL_ALPHA));
        return;
    }
    let (start, end) = match fraction {
        Some(fraction) => (0.0, fraction.clamp(0.0, 1.0)),
        None => {
            let phase = (ui.input(|input| input.time) * 0.7).fract() as f32;
            let start = phase * (1.0 + SWEEP) - SWEEP;
            (start.max(0.0), (start + SWEEP).min(1.0))
        }
    };
    if end > start {
        // It fills from where the language starts reading.
        let fill = Mirror::of(rect).rect(Rect::from_min_max(
            pos2(rect.left() + rect.width() * start, rect.top()),
            pos2(rect.left() + rect.width() * end, rect.bottom()),
        ));
        painter.rect_filled(fill, radius, theme.accent_fill);
    }
}

/// How strong the still bar is, for a step whose length is unknown while nothing may move.
const STILL_ALPHA: u8 = 110;

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
        Icon::ChevronUp => {
            line(&[(0.14, 0.66), (0.5, 0.3), (0.86, 0.66)]);
        }
        Icon::ChevronDown => {
            line(&[(0.14, 0.34), (0.5, 0.7), (0.86, 0.34)]);
        }
        Icon::ChevronRight => {
            line(&[(0.34, 0.14), (0.7, 0.5), (0.34, 0.86)]);
        }
        Icon::ChevronLeft => {
            line(&[(0.66, 0.14), (0.3, 0.5), (0.66, 0.86)]);
        }
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

    /// The shapes a progress bar of unknown length paints at a moment, in a window that may or may not animate.
    fn sweep_at(time: f64, animations: bool) -> Vec<egui::epaint::ClippedShape> {
        let ctx = egui::Context::default();
        crate::system_settings::allow_animations(&ctx, animations);
        let input = egui::RawInput {
            time: Some(time),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            progress_bar(ui, &Theme::new(theme::ThemeMode::Dark), None);
        });
        output.textures_delta.clear();
        output.shapes
    }

    /// AC-A11Y-MOTION: with Animation effects off, a step of unknown length is shown still.
    #[test]
    fn a_progress_bar_of_unknown_length_holds_still_when_nothing_may_move() {
        let as_text = |shapes: &[egui::epaint::ClippedShape]| format!("{shapes:?}");
        assert_ne!(
            as_text(&sweep_at(0.2, true)),
            as_text(&sweep_at(0.9, true)),
            "it sweeps"
        );
        assert_eq!(
            as_text(&sweep_at(0.2, false)),
            as_text(&sweep_at(0.9, false)),
            "it holds still"
        );
    }

    // ---- AC-RTL-MIRROR: a right-to-left window is the mirror image of a left-to-right one ----

    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use winappmigrator_i18n::{ENGLISH, Language, PSEUDO_RTL, with_language};

    const WIDTH: f32 = 400.0;

    /// The shapes `draw` paints in a window `WIDTH` wide, speaking `language`.
    fn painted(language: &'static Language, draw: impl FnMut(&mut Ui)) -> Vec<Shape> {
        with_language(language, || {
            let ctx = egui::Context::default();
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(WIDTH, 300.0))),
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, draw);
            output.textures_delta.clear();
            output
                .shapes
                .into_iter()
                .map(|clipped| clipped.shape)
                .collect()
        })
    }

    /// The rectangles painted that are `size` across, such as a checkbox.
    fn squares(shapes: &[Shape], size: f32) -> Vec<Rect> {
        shapes
            .iter()
            .filter_map(|shape| match shape {
                Shape::Rect(rect) if (rect.rect.width() - size).abs() < 0.5 => Some(rect.rect),
                _ => None,
            })
            .collect()
    }

    /// Where the buttons named `first` and `second` are, added in that order to a row of a page.
    fn row_of_two(language: &'static Language, trailing_too: bool) -> (Rect, Rect) {
        with_language(language, || {
            let theme = Theme::new(theme::ThemeMode::Light);
            let mut harness = Harness::new_ui(|ui| {
                page(ui, |ui| {
                    ui.horizontal(|ui| {
                        button(ui, &theme, ButtonKind::Secondary, "First", true);
                        if trailing_too {
                            trailing(ui, |ui| {
                                button(ui, &theme, ButtonKind::Secondary, "Second", true)
                            });
                        } else {
                            button(ui, &theme, ButtonKind::Secondary, "Second", true);
                        }
                    });
                });
            });
            harness.run();
            (
                harness.get_by_label("First").rect(),
                harness.get_by_label("Second").rect(),
            )
        })
    }

    #[test]
    fn a_row_runs_the_way_the_language_reads() {
        let (first, second) = row_of_two(ENGLISH, false);
        assert!(first.center().x < second.center().x, "{first:?} {second:?}");
        let (first, second) = row_of_two(&PSEUDO_RTL, false);
        assert!(first.center().x > second.center().x, "{first:?} {second:?}");
    }

    #[test]
    fn what_trails_a_row_is_at_its_far_end() {
        let (first, second) = row_of_two(ENGLISH, true);
        assert!(second.left() > first.right(), "{first:?} {second:?}");
        let (first, second) = row_of_two(&PSEUDO_RTL, true);
        assert!(second.right() < first.left(), "{first:?} {second:?}");
    }

    #[test]
    fn a_checkbox_is_where_reading_starts() {
        let draw = |ui: &mut Ui| {
            let theme = Theme::new(theme::ThemeMode::Light);
            choice_row(
                ui,
                &theme,
                &mut false,
                "Title",
                "Detail",
                Some("1 MB"),
                true,
            );
        };
        let english = squares(&painted(ENGLISH, draw), 16.0);
        let mirrored = squares(&painted(&PSEUDO_RTL, draw), 16.0);
        assert_eq!(
            (english.len(), mirrored.len()),
            (1, 1),
            "{english:?} {mirrored:?}"
        );
        assert!(english[0].center().x < WIDTH / 4.0, "{english:?}");
        assert!(mirrored[0].center().x > WIDTH * 3.0 / 4.0, "{mirrored:?}");
        assert!(
            (english[0].left() + mirrored[0].right() - WIDTH).abs() < 20.0,
            "mirror images: {english:?} {mirrored:?}"
        );
    }

    #[test]
    fn progress_fills_from_where_reading_starts() {
        let draw = |ui: &mut Ui| {
            progress_bar(ui, &Theme::new(theme::ThemeMode::Light), Some(0.25));
        };
        let fill_of = |shapes: Vec<Shape>| -> Vec<Rect> {
            shapes
                .into_iter()
                .filter_map(|shape| match shape {
                    Shape::Rect(rect) if rect.rect.width() < WIDTH / 2.0 => Some(rect.rect),
                    _ => None,
                })
                .collect()
        };
        let english = fill_of(painted(ENGLISH, draw));
        let mirrored = fill_of(painted(&PSEUDO_RTL, draw));
        assert!(english[0].left() < 20.0, "{english:?}");
        assert!(mirrored[0].right() > WIDTH - 20.0, "{mirrored:?}");
    }

    #[test]
    fn a_label_starts_at_the_side_reading_starts() {
        let text_left = |language: &'static Language| -> (f32, f32) {
            let shapes = painted(language, |ui| {
                page(ui, |ui| {
                    label(ui, "Short");
                });
            });
            shapes
                .iter()
                .find_map(|shape| match shape {
                    Shape::Text(text) => Some((text.pos.x, text.pos.x + text.galley.size().x)),
                    _ => None,
                })
                .unwrap_or_default()
        };
        let (left, _) = text_left(ENGLISH);
        assert!(left < 20.0, "{left}");
        let (_, right) = text_left(&PSEUDO_RTL);
        assert!(right > WIDTH - 20.0, "{right}");
    }

    /// Seen on a real Windows: egui shortened an Arabic search hint that did not fit by cutting its laid-out text,
    /// which left the letters reversed and unjoined. A hint with right-to-left letters is now handed over laid out, and
    /// already no wider than the field; one without is left to egui, as before.
    #[test]
    fn a_right_to_left_hint_is_given_laid_out_and_short_enough() {
        let Some(arabic) = winappmigrator_i18n::LANGUAGES
            .iter()
            .find(|language| language.tag == "ar-SA")
        else {
            panic!("no Arabic in the language table");
        };
        with_language(arabic, || {
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                let hint = "ابحث بالاسم أو الناشر  (Ctrl+F)";
                match field_hint(ui, hint, 80.0) {
                    egui::WidgetText::Galley(galley) => {
                        assert!(galley.size().x <= 80.0, "{}", galley.size().x);
                        assert!(galley.text().contains('…'), "shortened: {}", galley.text());
                    }
                    other => panic!("not laid out: {:?}", other.text()),
                }
                assert!(!matches!(
                    field_hint(ui, "Search (Ctrl+F)", 80.0),
                    egui::WidgetText::Galley(_)
                ));
            });
            output.textures_delta.clear();
        });
    }

    #[test]
    fn mirroring_turns_positions_around_inside_a_rect() {
        let rect = Rect::from_min_max(egui::pos2(100.0, 0.0), egui::pos2(300.0, 10.0));
        with_language(&PSEUDO_RTL, || {
            let mirror = Mirror::of(rect);
            assert_eq!(mirror.x(110.0), 290.0);
            assert_eq!(
                mirror.rect(Rect::from_min_max(
                    egui::pos2(100.0, 0.0),
                    egui::pos2(120.0, 10.0)
                )),
                Rect::from_min_max(egui::pos2(280.0, 0.0), egui::pos2(300.0, 10.0))
            );
            assert_eq!(mirror.left_of(110.0, 30.0), 260.0);
        });
        with_language(ENGLISH, || {
            assert_eq!(Mirror::of(rect).x(110.0), 110.0);
        });
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
