//! Design system for the WinAppMigrator GUI.
//!
//! Defines a cohesive token-based palette for both a refined dark theme and a
//! clean light theme, plus the routine that maps those tokens onto the global
//! `egui` style so every widget inherits the same look.

use std::collections::BTreeMap;

use eframe::egui::{
    self, Color32, CornerRadius, FontFamily, FontId, Margin, Shadow, Stroke, TextStyle,
};

/// Corner radius tokens, in points.
pub const RADIUS_SM: u8 = 7;
pub const RADIUS_MD: u8 = 11;
pub const RADIUS_LG: u8 = 16;

/// The two presentation modes the application supports.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggled(self) -> Self {
        match self {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        }
    }
}

/// A fully resolved set of colors for the active presentation mode.
#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub on_accent: Color32,
    pub bg: Color32,
    pub surface: Color32,
    pub card: Color32,
    pub card_hover: Color32,
    pub border: Color32,
    pub border_strong: Color32,
    pub field_bg: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub text_subtle: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
    pub shadow: Color32,
}

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Straight-alpha tint of a base color; useful for soft backgrounds.
pub fn alpha(color: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a)
}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }

    fn dark() -> Self {
        let accent = rgb(110, 122, 255);
        Self {
            mode: ThemeMode::Dark,
            accent,
            accent_soft: alpha(accent, 38),
            on_accent: rgb(255, 255, 255),
            bg: rgb(13, 15, 20),
            surface: rgb(20, 23, 31),
            card: rgb(25, 29, 39),
            card_hover: rgb(32, 37, 49),
            border: rgb(38, 43, 56),
            border_strong: rgb(54, 61, 79),
            field_bg: rgb(16, 18, 25),
            text: rgb(232, 235, 242),
            text_muted: rgb(150, 158, 173),
            text_subtle: rgb(104, 112, 128),
            success: rgb(53, 199, 129),
            warning: rgb(240, 182, 68),
            danger: rgb(244, 102, 102),
            shadow: Color32::from_black_alpha(150),
        }
    }

    fn light() -> Self {
        let accent = rgb(45, 99, 224);
        Self {
            mode: ThemeMode::Light,
            accent,
            accent_soft: alpha(accent, 28),
            on_accent: rgb(255, 255, 255),
            bg: rgb(244, 246, 250),
            surface: rgb(249, 250, 252),
            card: rgb(255, 255, 255),
            card_hover: rgb(243, 246, 250),
            border: rgb(225, 229, 236),
            border_strong: rgb(201, 208, 219),
            field_bg: rgb(255, 255, 255),
            text: rgb(22, 29, 41),
            text_muted: rgb(92, 103, 119),
            text_subtle: rgb(138, 147, 161),
            success: rgb(22, 148, 90),
            warning: rgb(176, 112, 16),
            danger: rgb(199, 60, 60),
            shadow: Color32::from_black_alpha(30),
        }
    }

    /// Shadow used for floating surfaces (cards, popups, windows).
    pub fn drop_shadow(&self) -> Shadow {
        Shadow {
            offset: [0, 10],
            blur: 28,
            spread: 0,
            color: self.shadow,
        }
    }

    pub fn soft_shadow(&self) -> Shadow {
        Shadow {
            offset: [0, 4],
            blur: 14,
            spread: 0,
            color: alpha(self.shadow, 90),
        }
    }

    /// Resolve a status color from a semantic role name.
    pub fn role(&self, role: Role) -> Color32 {
        match role {
            Role::Accent => self.accent,
            Role::Success => self.success,
            Role::Warning => self.warning,
            Role::Danger => self.danger,
            Role::Neutral => self.text_subtle,
        }
    }

    /// Apply this theme to the global `egui` style.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();

        let mut visuals = match self.mode {
            ThemeMode::Dark => egui::Visuals::dark(),
            ThemeMode::Light => egui::Visuals::light(),
        };

        let radius: CornerRadius = RADIUS_SM.into();

        visuals.override_text_color = Some(self.text);
        visuals.panel_fill = self.bg;
        visuals.window_fill = self.surface;
        visuals.window_stroke = Stroke::new(1.0, self.border);
        visuals.window_corner_radius = RADIUS_LG.into();
        visuals.window_shadow = self.drop_shadow();
        visuals.popup_shadow = self.drop_shadow();
        visuals.menu_corner_radius = RADIUS_MD.into();
        visuals.extreme_bg_color = self.field_bg;
        visuals.faint_bg_color = alpha(self.text_subtle, 20);
        visuals.code_bg_color = self.field_bg;
        visuals.hyperlink_color = self.accent;
        visuals.warn_fg_color = self.warning;
        visuals.error_fg_color = self.danger;
        visuals.selection.bg_fill = alpha(self.accent, 96);
        visuals.selection.stroke = Stroke::new(1.0, self.accent);

        visuals.widgets.noninteractive.bg_fill = self.card;
        visuals.widgets.noninteractive.weak_bg_fill = self.card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_muted);
        visuals.widgets.noninteractive.corner_radius = radius;

        visuals.widgets.inactive.bg_fill = self.card;
        visuals.widgets.inactive.weak_bg_fill = self.card;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.inactive.corner_radius = radius;

        visuals.widgets.hovered.bg_fill = self.card_hover;
        visuals.widgets.hovered.weak_bg_fill = self.card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, self.border_strong);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.hovered.corner_radius = radius;

        visuals.widgets.active.bg_fill = self.card_hover;
        visuals.widgets.active.weak_bg_fill = self.card_hover;
        visuals.widgets.active.bg_stroke = Stroke::new(1.4, self.accent);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.active.corner_radius = radius;

        visuals.widgets.open.bg_fill = self.card_hover;
        visuals.widgets.open.weak_bg_fill = self.card_hover;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, self.border_strong);
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.open.corner_radius = radius;

        style.visuals = visuals;

        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(14.0, 7.0);
        style.spacing.window_margin = Margin::same(0);
        style.spacing.menu_margin = Margin::same(6);
        style.spacing.interact_size.y = 30.0;
        style.spacing.scroll.bar_width = 9.0;

        let mut text_styles = BTreeMap::new();
        text_styles.insert(
            TextStyle::Heading,
            FontId::new(23.0, FontFamily::Proportional),
        );
        text_styles.insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
        text_styles.insert(
            TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Monospace,
            FontId::new(12.5, FontFamily::Monospace),
        );
        style.text_styles = text_styles;

        ctx.set_global_style(style);
    }
}

/// Semantic status roles used across badges, chips, and metrics.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Accent,
    Success,
    Warning,
    Danger,
    Neutral,
}
