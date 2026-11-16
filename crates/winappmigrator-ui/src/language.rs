//! The Language menu, and telling screen readers which language a window is in.

use std::path::Path;

use eframe::egui::{self, Ui};
use winappmigrator_i18n::{ENGLISH, Language, t};

use crate::theme;

/// Where a translation problem is reported: the repository's issue form for it.
const TRANSLATION_ISSUE: &str = "https://github.com/RandyNorthrup/WinAppMigrator/issues/new?template=translation-problem.yml&labels=translation";

/// What a person did in the Language menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageAction {
    /// Chose a language.
    Choose(&'static Language),
    /// Asked to report a problem with the translation they are reading, at this address, which the caller opens in the
    /// browser.
    Report(String),
}

/// The address of the issue form for a problem with `language`'s translation, with the language filled in.
pub fn translation_issue_url(language: &Language) -> String {
    format!(
        "{TRANSLATION_ISSUE}&language={tag}&title=%5B{tag}%5D%20",
        tag = language.tag
    )
}

/// The Language menu: every language that ships, by its own name, and, in any but English, a note that the translation
/// is by machine with a way to report a problem with it.
pub fn language_menu(ui: &mut Ui, fonts_dir: &Path) -> Option<LanguageAction> {
    let current = winappmigrator_i18n::current();
    let mut action = None;
    egui::ComboBox::from_label(t!("ui-language"))
        .selected_text(current.native_name)
        .show_ui(ui, |ui| {
            // Each name is in its own script, which needs its font while the list is open. A font
            // loaded now is used from the next pass, so the names wait for it rather than show as boxes.
            if theme::add_fallback_fonts(
                ui.ctx(),
                fonts_dir,
                winappmigrator_i18n::shipped().map(|language| language.native_name),
            ) {
                ui.ctx()
                    .request_discard("the fonts for the languages' names");
                return;
            }
            for language in winappmigrator_i18n::shipped() {
                let row = ui
                    .selectable_label(language == current, language.native_name)
                    .on_hover_text(language.english_name);
                if row.clicked() && language != current {
                    action = Some(LanguageAction::Choose(language));
                }
            }
            if current != ENGLISH {
                ui.separator();
                crate::widgets::wrapped_label(ui, t!("ui-machine-translated"));
                if ui.link(t!("ui-report-translation-problem")).clicked() {
                    action = Some(LanguageAction::Report(translation_issue_url(current)));
                }
            }
        });
    action
}

/// Tell screen readers which language the window is in, so they read it in the right voice. Call it once a frame, as
/// the tree screen readers see is built anew each frame.
pub fn declare_language(ctx: &egui::Context) {
    let tag = winappmigrator_i18n::current().tag;
    let _declared = ctx.accesskit_node_builder(egui::accesskit_root_id(), |node| {
        node.set_language(tag);
    });
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};
    use winappmigrator_i18n::{LANGUAGES, PSEUDO_LONG, is_isolation_mark, with_language};

    use super::*;

    /// Windows' own font folder, where every font a language needs is found on a stock install.
    fn windows_fonts() -> PathBuf {
        PathBuf::from(r"C:\Windows\Fonts")
    }

    /// The characters of `text` that the fonts installed for the language in use cannot draw, in either weight.
    fn missing_letters(text: &str) -> BTreeSet<char> {
        let ctx = egui::Context::default();
        theme::install_fonts(&ctx, &windows_fonts());
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
        output.textures_delta.clear();
        let fonts = [egui::FontId::proportional(14.0), theme::semibold(14.0)];
        ctx.fonts_mut(|installed| {
            text.chars()
                .filter(|c| !c.is_control() && !is_isolation_mark(*c))
                .filter(|c| fonts.iter().any(|font| !installed.has_glyph(font, *c)))
                .collect()
        })
    }

    /// AC-I18N-GLYPHS: every language's own name, and everything in its catalog, draws with the fonts it loads.
    #[test]
    fn every_language_draws_every_letter_with_windows_own_fonts() {
        let mut problems = Vec::new();
        for language in LANGUAGES {
            let mut text = language.native_name.to_owned();
            text.push_str(&winappmigrator_i18n::catalog_source(language.tag).unwrap_or_default());
            let missing = with_language(language, || missing_letters(&text));
            if !missing.is_empty() {
                problems.push(format!("{}: {missing:?}", language.tag));
            }
        }
        assert!(problems.is_empty(), "{problems:#?}");
    }

    #[test]
    fn a_letter_no_font_has_is_reported_missing() {
        let missing = with_language(ENGLISH, || missing_letters("A\u{10FFFD}"));
        assert_eq!(missing, BTreeSet::from(['\u{10FFFD}']));
    }

    #[test]
    fn a_japanese_window_takes_han_from_the_japanese_font() {
        let Some(japanese) = winappmigrator_i18n::language("ja-JP") else {
            panic!("Japanese is in the table");
        };
        let families = with_language(japanese, || {
            let ctx = egui::Context::default();
            theme::install_fonts(&ctx, &windows_fonts());
            let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
            output.textures_delta.clear();
            ctx.fonts(|fonts| {
                fonts
                    .definitions()
                    .families
                    .get(&egui::FontFamily::Proportional)
                    .cloned()
                    .unwrap_or_default()
            })
        });
        assert_eq!(
            families.get(1).map(String::as_str),
            Some("YuGothR.ttc"),
            "{families:?}"
        );
    }

    fn open_menu(language: &'static Language) -> Harness<'static> {
        let mut harness = Harness::new_ui(move |ui| {
            with_language(language, || {
                let _ = language_menu(ui, &windows_fonts());
            });
        });
        // As the app and the installer do before their first frame.
        theme::install_fonts(&harness.ctx, &windows_fonts());
        harness.run();
        harness.get_by_role(egui::accesskit::Role::ComboBox).click();
        harness.run();
        harness
    }

    /// AC-I18N-MT-NOTICE.
    #[test]
    fn any_language_but_english_says_it_is_by_machine_with_a_way_to_report_it() {
        let notice = with_language(&PSEUDO_LONG, || t!("ui-machine-translated"));
        let report = with_language(&PSEUDO_LONG, || t!("ui-report-translation-problem"));
        let harness = open_menu(&PSEUDO_LONG);
        assert!(harness.query_by_label(&notice).is_some(), "{notice}");
        assert!(harness.query_by_label(&report).is_some(), "{report}");

        let english = open_menu(ENGLISH);
        assert!(
            english
                .query_by_label(&t!("ui-machine-translated"))
                .is_none()
        );
        assert!(english.query_by_label("English").is_some());
    }

    #[test]
    fn the_issue_form_is_told_the_language() {
        let Some(italian) = winappmigrator_i18n::language("it-IT") else {
            panic!("Italian is in the table");
        };
        let url = translation_issue_url(italian);
        assert!(url.starts_with("https://github.com/RandyNorthrup/WinAppMigrator/issues/new?"));
        assert!(url.contains("template=translation-problem.yml"));
        assert!(url.contains("&language=it-IT&"));
        assert!(url.ends_with("title=%5Bit-IT%5D%20"));
    }

    // ---- AC-I18N-PICKER ----

    /// The Language menu, open, with what a person chose in it kept as the harness's state.
    fn menu_that_says_what_was_chosen() -> Harness<'static, Option<LanguageAction>> {
        let mut harness = Harness::new_ui_state(
            |ui, chosen: &mut Option<LanguageAction>| {
                with_language(ENGLISH, || {
                    if let Some(action) = language_menu(ui, &windows_fonts()) {
                        *chosen = Some(action);
                    }
                });
            },
            None,
        );
        theme::install_fonts(&harness.ctx, &windows_fonts());
        harness.run();
        harness
    }

    fn italian() -> &'static Language {
        winappmigrator_i18n::language("it-IT").unwrap_or_else(|| panic!("Italian is in the table"))
    }

    #[test]
    fn a_language_is_chosen_by_clicking_it() {
        let mut harness = menu_that_says_what_was_chosen();
        harness.get_by_role(egui::accesskit::Role::ComboBox).click();
        harness.run();
        // A click, as egui is told of one by a mouse, a pen, or a screen reader.
        harness.get_by_label("Italiano").click_accesskit();
        harness.run();
        assert_eq!(harness.state(), &Some(LanguageAction::Choose(italian())));
    }

    #[test]
    fn a_language_is_chosen_with_the_keyboard() {
        let mut harness = menu_that_says_what_was_chosen();
        // Tab to the menu, and open it with the space bar.
        harness.key_press(egui::Key::Tab);
        harness.run();
        harness.key_press(egui::Key::Space);
        harness.run();
        let mut reached = false;
        for _ in 0..40 {
            harness.key_press(egui::Key::Tab);
            harness.run();
            if harness
                .query_by_label("Italiano")
                .is_some_and(|node| node.accesskit_node().is_focused())
            {
                reached = true;
                break;
            }
        }
        assert!(reached, "the Tab key reaches Italiano in the open menu");
        harness.key_press(egui::Key::Enter);
        harness.run();
        assert_eq!(harness.state(), &Some(LanguageAction::Choose(italian())));
    }
}
