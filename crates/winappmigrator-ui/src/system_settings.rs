//! Windows' accessibility settings that these windows follow: a contrast theme, Text size, and Animation effects.
//!
//! They are read from Windows' own UI settings, which say what a person chose in Settings, Accessibility, and which a
//! desktop program can ask without anything of its own running unsafe code.

use eframe::egui::{self, Color32};
use windows::UI::ViewManagement::{AccessibilitySettings, UIElementType, UISettings};

/// What a person chose in Windows for how windows look and move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemSettings {
    /// The colors of the contrast theme in use, if one is.
    pub contrast: Option<ContrastColors>,
    /// How much larger text is to be than Windows' standard, from 1 to 2.25.
    pub text_scale: f32,
    /// Whether things may move on their own, such as a progress bar whose length is not known.
    pub animations: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            contrast: None,
            text_scale: 1.0,
            animations: true,
        }
    }
}

/// The system colors of a Windows contrast theme, which a window in that theme is drawn in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContrastColors {
    pub window: Color32,
    pub text: Color32,
    pub highlight: Color32,
    pub highlight_text: Color32,
    pub button_face: Color32,
    pub button_text: Color32,
    pub disabled_text: Color32,
    pub link: Color32,
}

/// Windows lets Text size go up to 225%.
const MOST_TEXT_SCALE: f32 = 2.25;

impl SystemSettings {
    /// What Windows says now. Anything it cannot say is taken as Windows' standard.
    pub fn read() -> Self {
        let ui = UISettings::new().ok();
        let text_scale = ui
            .as_ref()
            .and_then(|ui| ui.TextScaleFactor().ok())
            .map_or(1.0, |scale| clamp_text_scale(scale as f32));
        let animations = ui
            .as_ref()
            .and_then(|ui| ui.AnimationsEnabled().ok())
            .unwrap_or(true);
        let contrast_on = AccessibilitySettings::new()
            .and_then(|settings| settings.HighContrast())
            .unwrap_or(false);
        let contrast = contrast_on
            .then(|| ui.as_ref().and_then(contrast_colors))
            .flatten();
        Self {
            contrast,
            text_scale,
            animations,
        }
    }
}

fn clamp_text_scale(scale: f32) -> f32 {
    if scale.is_finite() {
        scale.clamp(1.0, MOST_TEXT_SCALE)
    } else {
        1.0
    }
}

fn contrast_colors(ui: &UISettings) -> Option<ContrastColors> {
    let color = |element: UIElementType| {
        ui.UIElementColor(element)
            .ok()
            .map(|c| Color32::from_rgb(c.R, c.G, c.B))
    };
    Some(ContrastColors {
        window: color(UIElementType::Window)?,
        text: color(UIElementType::WindowText)?,
        highlight: color(UIElementType::Highlight)?,
        highlight_text: color(UIElementType::HighlightText)?,
        button_face: color(UIElementType::ButtonFace)?,
        button_text: color(UIElementType::ButtonText)?,
        disabled_text: color(UIElementType::GrayText)?,
        link: color(UIElementType::Hotlight)?,
    })
}

/// Whether things may move on their own in this window, as Animation effects says.
#[derive(Clone, Copy)]
struct AnimationsAllowed(bool);

fn animations_id() -> egui::Id {
    egui::Id::unique("winappmigrator-animations-allowed")
}

/// Let things in this window move on their own, or not: egui's own animations and this crate's widgets both follow it.
pub fn allow_animations(ctx: &egui::Context, allowed: bool) {
    ctx.data_mut(|data| data.insert_temp(animations_id(), AnimationsAllowed(allowed)));
    if !allowed {
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
    }
}

/// Whether things may move on their own in this window. They may unless Animation effects was found off.
pub fn animations_allowed(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp::<AnimationsAllowed>(animations_id()))
        .is_none_or(|allowed| allowed.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_scale_stays_within_what_windows_allows() {
        assert_eq!(clamp_text_scale(1.5), 1.5);
        assert_eq!(clamp_text_scale(0.5), 1.0);
        assert_eq!(clamp_text_scale(9.0), MOST_TEXT_SCALE);
        assert_eq!(clamp_text_scale(f32::NAN), 1.0);
    }

    #[test]
    fn a_window_remembers_whether_things_may_move() {
        let ctx = egui::Context::default();
        assert!(
            animations_allowed(&ctx),
            "until Windows is found to say otherwise"
        );
        allow_animations(&ctx, false);
        assert!(!animations_allowed(&ctx));
        assert_eq!(ctx.global_style().animation_time, 0.0);
    }

    /// Whatever this PC is set to, reading it works and gives something within range.
    #[test]
    fn windows_settings_can_be_read() {
        let settings = SystemSettings::read();
        assert!((1.0..=MOST_TEXT_SCALE).contains(&settings.text_scale));
    }
}
