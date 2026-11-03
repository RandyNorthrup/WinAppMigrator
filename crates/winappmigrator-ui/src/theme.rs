//! Design system for the WinAppMigrator GUI.
//!
//! Defines a cohesive token-based palette for both a refined dark theme and a
//! clean light theme, plus the routine that maps those tokens onto the global
//! `egui` style so every widget inherits the same look.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Shadow,
    Stroke, TextStyle,
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};

/// Corner radius tokens, in points.
pub const RADIUS_SM: u8 = 6;
pub const RADIUS_MD: u8 = 8;
pub const RADIUS_LG: u8 = 10;

/// Spacing tokens, in points.
pub const SPACE_XS: f32 = 4.0;
pub const SPACE_SM: f32 = 8.0;
pub const SPACE_MD: f32 = 12.0;
pub const SPACE_LG: f32 = 16.0;
pub const SPACE_XL: f32 = 24.0;

/// Height shared by buttons, text fields, and segmented controls, so controls on
/// the same row always line up.
pub const CONTROL_HEIGHT: f32 = 34.0;

/// The heavier text weight. `egui`'s `strong()` only changes color, so headings
/// need a real font of their own to read as headings.
const SEMIBOLD_FAMILY: &str = "semibold";

/// The two presentation modes the application supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// A stable string tag for remembering the mode a person chose.
    pub fn as_tag(self) -> &'static str {
        match self {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }

    /// Parse a tag written by [`ThemeMode::as_tag`]. Anything else says nothing was chosen.
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "dark" => Some(ThemeMode::Dark),
            "light" => Some(ThemeMode::Light),
            _ => None,
        }
    }
}

/// A fully resolved set of colors for the active presentation mode.
#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    /// Accent used for text, icons, and focus rings on top of surfaces.
    pub accent: Color32,
    /// Accent used as a fill behind [`Theme::on_accent`] text.
    pub accent_fill: Color32,
    pub accent_fill_hover: Color32,
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

/// The semibold variant of the interface font.
pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD_FAMILY.into()))
}

/// The regular interface font.
pub fn regular(size: f32) -> FontId {
    FontId::proportional(size)
}

/// Use the native Windows interface fonts, so the app looks at home on the
/// desktop. Nothing is bundled: a missing font keeps `egui`'s built-in default.
pub fn install_fonts(ctx: &egui::Context, fonts_dir: &Path) {
    let mut fonts = FontDefinitions::default();
    let mut proportional = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let mut heavier = proportional.clone();
    let mut monospace = fonts
        .families
        .get(&FontFamily::Monospace)
        .cloned()
        .unwrap_or_default();

    for (file, family) in [
        ("segoeui.ttf", &mut proportional),
        ("seguisb.ttf", &mut heavier),
        ("consola.ttf", &mut monospace),
    ] {
        if let Ok(bytes) = std::fs::read(fonts_dir.join(file)) {
            fonts
                .font_data
                .insert(file.to_owned(), Arc::new(FontData::from_owned(bytes)));
            family.insert(0, file.to_owned());
        }
    }

    fonts
        .families
        .insert(FontFamily::Proportional, proportional);
    fonts
        .families
        .insert(FontFamily::Name(SEMIBOLD_FAMILY.into()), heavier);
    fonts.families.insert(FontFamily::Monospace, monospace);
    ctx.set_fonts(fonts);
}

/// Windows font holding the given script, for text Segoe UI does not cover.
fn fallback_font_file(character: char) -> Option<&'static str> {
    match u32::from(character) {
        0x1100..=0x11FF | 0x3130..=0x318F | 0xAC00..=0xD7AF => Some("malgun.ttf"),
        0x3040..=0x30FF => Some("YuGothR.ttc"),
        0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF => Some("msyh.ttc"),
        _ => None,
    }
}

/// Load only the fallback fonts that `texts` needs. These are tens of megabytes
/// each, and most machines list no application that needs them, but without them
/// a Korean, Japanese, or Chinese application name draws as empty boxes.
pub fn add_fallback_fonts<'a>(
    ctx: &egui::Context,
    fonts_dir: &Path,
    texts: impl IntoIterator<Item = &'a str>,
) {
    let mut needed: Vec<&'static str> = Vec::new();
    for character in texts.into_iter().flat_map(str::chars) {
        if let Some(file) = fallback_font_file(character)
            && !needed.contains(&file)
        {
            needed.push(file);
        }
    }
    for file in needed {
        if let Ok(bytes) = std::fs::read(fonts_dir.join(file)) {
            ctx.add_font(FontInsert::new(
                file,
                FontData::from_owned(bytes),
                vec![
                    InsertFontFamily {
                        family: FontFamily::Proportional,
                        priority: FontPriority::Lowest,
                    },
                    InsertFontFamily {
                        family: FontFamily::Name(SEMIBOLD_FAMILY.into()),
                        priority: FontPriority::Lowest,
                    },
                ],
            ));
        }
    }
}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }

    fn dark() -> Self {
        let accent = rgb(150, 160, 255);
        Self {
            mode: ThemeMode::Dark,
            accent,
            accent_fill: rgb(76, 90, 232),
            accent_fill_hover: rgb(88, 102, 242),
            accent_soft: alpha(accent, 46),
            on_accent: rgb(255, 255, 255),
            // A darker page so raised cards read clearly above it, with cards and
            // borders each a distinct step lighter for visible separation.
            bg: rgb(15, 17, 23),
            surface: rgb(21, 24, 32),
            card: rgb(28, 32, 43),
            card_hover: rgb(38, 43, 57),
            border: rgb(48, 54, 71),
            border_strong: rgb(78, 87, 110),
            field_bg: rgb(17, 19, 26),
            text: rgb(236, 239, 245),
            text_muted: rgb(170, 178, 193),
            text_subtle: rgb(140, 149, 166),
            success: rgb(58, 208, 136),
            warning: rgb(245, 190, 78),
            danger: rgb(250, 128, 128),
            shadow: Color32::from_black_alpha(120),
        }
    }

    fn light() -> Self {
        let accent = rgb(45, 99, 224);
        Self {
            mode: ThemeMode::Light,
            accent,
            accent_fill: accent,
            accent_fill_hover: rgb(34, 84, 201),
            accent_soft: alpha(accent, 28),
            on_accent: rgb(255, 255, 255),
            bg: rgb(241, 243, 248),
            surface: rgb(248, 250, 252),
            card: rgb(255, 255, 255),
            card_hover: rgb(240, 244, 249),
            border: rgb(218, 223, 232),
            border_strong: rgb(160, 170, 186),
            field_bg: rgb(255, 255, 255),
            text: rgb(20, 27, 39),
            text_muted: rgb(76, 87, 104),
            text_subtle: rgb(96, 106, 122),
            success: rgb(16, 120, 72),
            warning: rgb(150, 92, 8),
            danger: rgb(190, 50, 50),
            shadow: Color32::from_black_alpha(24),
        }
    }

    /// Shadow used for floating surfaces (popups, windows).
    pub fn drop_shadow(&self) -> Shadow {
        Shadow {
            offset: [0, 6],
            blur: 20,
            spread: 0,
            color: self.shadow,
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
        visuals.text_edit_bg_color = Some(self.field_bg);
        visuals.faint_bg_color = alpha(self.text_subtle, 20);
        visuals.code_bg_color = self.field_bg;
        visuals.hyperlink_color = self.accent;
        visuals.warn_fg_color = self.warning;
        visuals.error_fg_color = self.danger;
        visuals.selection.bg_fill = alpha(self.accent, 96);
        visuals.selection.stroke = Stroke::new(1.5, self.accent);

        visuals.widgets.noninteractive.bg_fill = self.card;
        visuals.widgets.noninteractive.weak_bg_fill = self.card;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.border);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_muted);
        visuals.widgets.noninteractive.corner_radius = radius;

        visuals.widgets.inactive.bg_fill = self.field_bg;
        visuals.widgets.inactive.weak_bg_fill = self.card;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border_strong);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.inactive.corner_radius = radius;

        visuals.widgets.hovered.bg_fill = self.card_hover;
        visuals.widgets.hovered.weak_bg_fill = self.card_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, self.border_strong);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.hovered.corner_radius = radius;

        visuals.widgets.active.bg_fill = self.card_hover;
        visuals.widgets.active.weak_bg_fill = self.card_hover;
        visuals.widgets.active.bg_stroke = Stroke::new(1.5, self.accent);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.active.corner_radius = radius;

        visuals.widgets.open.bg_fill = self.card_hover;
        visuals.widgets.open.weak_bg_fill = self.card_hover;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, self.border_strong);
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, self.text);
        visuals.widgets.open.corner_radius = radius;

        style.visuals = visuals;

        style.spacing.item_spacing = egui::vec2(SPACE_SM, SPACE_SM);
        style.spacing.button_padding = egui::vec2(SPACE_MD, 6.0);
        style.spacing.window_margin = Margin::same(0);
        style.spacing.menu_margin = Margin::same(6);
        // Rows size themselves from their content; a tall value here would pad every
        // label in a horizontal row. Controls set their own height.
        style.spacing.interact_size.y = 18.0;
        style.spacing.scroll.bar_width = 8.0;

        let mut text_styles = BTreeMap::new();
        text_styles.insert(TextStyle::Heading, semibold(20.0));
        text_styles.insert(TextStyle::Body, regular(14.0));
        text_styles.insert(TextStyle::Button, regular(14.0));
        text_styles.insert(TextStyle::Small, regular(12.0));
        text_styles.insert(
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        );
        style.text_styles = text_styles;

        // egui keeps a style for each mode Windows can be in, and goes by the one it is in. A window
        // here has one look, so with only one of these set, Windows in light mode gave a dark window
        // a white search box and white tooltips.
        let style = std::sync::Arc::new(style);
        ctx.set_style_of(egui::Theme::Dark, std::sync::Arc::clone(&style));
        ctx.set_style_of(egui::Theme::Light, style);
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

#[cfg(test)]
mod tests {
    use super::{Color32, Theme, ThemeMode, alpha, egui, fallback_font_file};

    /// WCAG AA minimum for normal-size text.
    const AA_TEXT: f32 = 4.5;

    #[test]
    fn a_window_stays_in_its_theme_whichever_mode_windows_is_in() {
        for (mode, windows) in [
            (ThemeMode::Dark, egui::Theme::Light),
            (ThemeMode::Light, egui::Theme::Dark),
            (ThemeMode::Dark, egui::Theme::Dark),
            (ThemeMode::Light, egui::Theme::Light),
        ] {
            let ctx = egui::Context::default();
            let theme = Theme::new(mode);
            theme.apply(&ctx);
            // Which of its styles egui is going by, which is otherwise by what Windows says.
            ctx.set_theme(windows);
            assert_eq!(
                ctx.global_style().visuals.text_edit_bg_color,
                Some(theme.field_bg),
                "a text box in {windows:?}"
            );
            assert_eq!(
                ctx.global_style().visuals.window_fill,
                theme.surface,
                "a tooltip in {windows:?}"
            );
        }
    }

    fn luminance(color: Color32) -> f32 {
        let channel = |value: u8| {
            let value = f32::from(value) / 255.0;
            if value <= 0.039_28 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
    }

    fn contrast(foreground: Color32, background: Color32) -> f32 {
        let (first, second) = (luminance(foreground), luminance(background));
        (first.max(second) + 0.05) / (first.min(second) + 0.05)
    }

    /// `over` with `tint` composited on top, for checking text on a soft fill.
    fn blend(tint: Color32, over: Color32) -> Color32 {
        let weight = f32::from(tint.a()) / 255.0;
        let mix = |top: u8, bottom: u8| {
            (f32::from(top) * weight + f32::from(bottom) * (1.0 - weight)).round() as u8
        };
        let plain = tint.to_srgba_unmultiplied();
        Color32::from_rgb(
            mix(plain[0], over.r()),
            mix(plain[1], over.g()),
            mix(plain[2], over.b()),
        )
    }

    #[test]
    fn theme_mode_tag_round_trips() {
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            assert!(ThemeMode::from_tag(mode.as_tag()) == Some(mode));
        }
    }

    #[test]
    fn a_tag_that_is_not_one_says_nothing_was_chosen() {
        assert!(ThemeMode::from_tag("nonsense").is_none());
        assert!(ThemeMode::from_tag("").is_none());
    }

    #[test]
    fn every_text_color_meets_wcag_aa_on_every_surface() {
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            let theme = Theme::new(mode);
            let surfaces = [
                ("bg", theme.bg),
                ("surface", theme.surface),
                ("card", theme.card),
                ("field_bg", theme.field_bg),
            ];
            let inks = [
                ("text", theme.text),
                ("text_muted", theme.text_muted),
                ("text_subtle", theme.text_subtle),
                ("accent", theme.accent),
                ("success", theme.success),
                ("warning", theme.warning),
                ("danger", theme.danger),
            ];
            for (surface_name, surface) in surfaces {
                for (ink_name, ink) in inks {
                    let ratio = contrast(ink, surface);
                    assert!(
                        ratio >= AA_TEXT,
                        "{} {ink_name} on {surface_name} is {ratio:.2}:1",
                        mode.as_tag()
                    );
                }
            }
        }
    }

    #[test]
    fn button_label_meets_wcag_aa_on_the_accent_fill() {
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            let theme = Theme::new(mode);
            for fill in [theme.accent_fill, theme.accent_fill_hover] {
                let ratio = contrast(theme.on_accent, fill);
                assert!(ratio >= AA_TEXT, "{} is {ratio:.2}:1", mode.as_tag());
            }
        }
    }

    #[test]
    fn selected_row_text_stays_readable_on_its_tint() {
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            let theme = Theme::new(mode);
            let selected = blend(alpha(theme.accent, theme.accent_soft.a()), theme.card);
            for ink in [theme.text, theme.text_muted, theme.accent] {
                let ratio = contrast(ink, selected);
                assert!(ratio >= AA_TEXT, "{} is {ratio:.2}:1", mode.as_tag());
            }
        }
    }

    #[test]
    fn fallback_fonts_are_chosen_by_script() {
        assert_eq!(fallback_font_file('A'), None);
        assert_eq!(fallback_font_file('é'), None);
        assert_eq!(fallback_font_file('한'), Some("malgun.ttf"));
        assert_eq!(fallback_font_file('カ'), Some("YuGothR.ttc"));
        assert_eq!(fallback_font_file('微'), Some("msyh.ttc"));
    }
}
