//! The client: runs on the PC an app is moving off (Windows 7, 8, 10 or 11) and lets WinAppMigrator on
//! the new PC read it over the network, through [`winappmigrator_link`]. It captures nothing itself.
//!
//! It takes a shadow copy of the Windows drive through WMI, so what is read is one moment of it and
//! no file is locked, and answers requests to describe, list and read paths in that copy, read-only.
//! It never follows a link there, and only ever reads below the copy's root. While it runs it lets
//! its own two ports through Windows Firewall from the local network only. All of that is undone
//! when it stops, and by the next run if it could not be.
//!
//! It speaks Windows' display language, or the one `--lang` names, if it ships; see [`choose_language`].

pub mod serve;
pub mod system;

use winappmigrator_i18n::{Direction, ENGLISH, Language, t};

/// The client's version, which the builder is told.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The argument that chooses the language the client speaks, as `--lang it-IT`.
pub const LANG: &str = "--lang";
/// The argument that sets the first pairing code, as `--code 123456`.
pub const CODE: &str = "--code";
/// The arguments that show what the client takes, and nothing else.
pub const HELP: &[&str] = &["--help", "-h", "/?"];

/// What follows `flag` among a program's arguments, as `it-IT` follows `--lang`.
pub fn argument_after(arguments: impl IntoIterator<Item = String>, flag: &str) -> Option<String> {
    arguments
        .into_iter()
        .skip_while(|argument| argument != flag)
        .nth(1)
}

/// The language the client speaks, and the one it speaks in place of, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Speaking {
    pub language: &'static Language,
    /// The right-to-left language chosen, which the console cannot show, so the client speaks English instead.
    pub in_place_of: Option<&'static Language>,
}

/// The language the client speaks: the one `--lang` asked for, if it ships, by its language when not by its region;
/// else the first of Windows' display languages that ships; else English, as the app chooses.
///
/// The console Windows gives the client, on Windows 7 especially, cannot join Arabic letters, nor put a line that runs
/// right to left in order, so a right-to-left language is spoken in English, and [`console_note`] says why.
pub fn choose_language(
    requested: Option<&str>,
    windows: &[String],
    ships: impl Fn(&Language) -> bool,
) -> Speaking {
    let preferred: Vec<String> = requested
        .map(str::to_owned)
        .into_iter()
        .chain(windows.iter().cloned())
        .collect();
    let chosen = winappmigrator_i18n::negotiate(None, &preferred, ships);
    if chosen.direction == Direction::RightToLeft {
        Speaking {
            language: ENGLISH,
            in_place_of: Some(chosen),
        }
    } else {
        Speaking {
            language: chosen,
            in_place_of: None,
        }
    }
}

/// Speak, from now on, the language [`choose_language`] chooses from what `--lang` asked for and this PC's Windows.
pub fn speak(requested: Option<&str>) -> Speaking {
    let speaking = choose_language(
        requested,
        &winappmigrator_i18n::windows_display_languages(),
        winappmigrator_i18n::ships,
    );
    winappmigrator_i18n::use_language(speaking.language);
    speaking
}

/// The line, in English, that says the console cannot show the language chosen, to be printed before anything else.
pub fn console_note(speaking: &Speaking) -> Option<String> {
    speaking.in_place_of.map(|language| {
        t!(
            "client-console-rtl-fallback",
            language = language.english_name
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use winappmigrator_i18n::with_language;

    fn tags(list: &[&str]) -> Vec<String> {
        list.iter().map(|tag| (*tag).to_owned()).collect()
    }

    fn lang(tag: &str) -> &'static Language {
        winappmigrator_i18n::language(tag).unwrap_or_else(|| panic!("{tag} is in the table"))
    }

    /// As if these, and English, were the languages that ship.
    fn shipping(tags: &'static [&'static str]) -> impl Fn(&Language) -> bool {
        move |language| language.tag == "en-US" || tags.contains(&language.tag)
    }

    #[test]
    fn lang_chooses_a_language_that_ships_over_windows() {
        let italian = shipping(&["it-IT", "fr-FR"]);
        for asked in ["it-IT", "IT-it", "it", "it-CH"] {
            let speaking = choose_language(Some(asked), &tags(&["fr-FR"]), &italian);
            assert_eq!(speaking.language.tag, "it-IT", "{asked}");
            assert_eq!(speaking.in_place_of, None);
        }
    }

    #[test]
    fn lang_for_a_language_that_does_not_ship_leaves_windows_then_english() {
        let french = shipping(&["fr-FR"]);
        let speaking = choose_language(Some("it-IT"), &tags(&["fr-CA"]), &french);
        assert_eq!(speaking.language.tag, "fr-FR");
        let speaking = choose_language(Some("it-IT"), &[], &french);
        assert_eq!(speaking.language, ENGLISH);
        let speaking = choose_language(Some("not a language"), &tags(&["xx-YY"]), &french);
        assert_eq!(speaking.language, ENGLISH);
        assert_eq!(speaking.in_place_of, None);
    }

    #[test]
    fn lang_it_it_gives_italian_if_it_ships_and_otherwise_what_windows_says() {
        let windows = tags(&["de-DE", "en-GB"]);
        let speaking = choose_language(Some("it-IT"), &windows, winappmigrator_i18n::ships);
        if winappmigrator_i18n::ships(lang("it-IT")) {
            assert_eq!(speaking.language.tag, "it-IT");
        } else {
            assert_eq!(
                speaking.language,
                winappmigrator_i18n::negotiate(None, &windows, winappmigrator_i18n::ships)
            );
        }
        assert_eq!(speaking.in_place_of, None);
    }

    #[test]
    fn without_lang_windows_display_language_is_spoken() {
        let italian = shipping(&["it-IT"]);
        assert_eq!(
            choose_language(None, &tags(&["it-IT"]), &italian)
                .language
                .tag,
            "it-IT"
        );
        assert_eq!(
            choose_language(None, &tags(&["sr-Latn-RS"]), &italian).language,
            ENGLISH
        );
        assert_eq!(choose_language(None, &[], &italian).language, ENGLISH);
    }

    #[test]
    fn a_right_to_left_language_is_spoken_in_english_with_a_note() {
        let right_to_left = shipping(&["ar-SA", "he-IL", "fa-IR", "ur-PK"]);
        for (asked, windows, named) in [
            (Some("ar-SA"), vec![], "Arabic"),
            (None, tags(&["he-IL"]), "Hebrew"),
            (None, tags(&["fa-IR", "en-US"]), "Persian"),
            (Some("ur"), tags(&["it-IT"]), "Urdu"),
        ] {
            let speaking = choose_language(asked, &windows, &right_to_left);
            assert_eq!(speaking.language, ENGLISH, "{asked:?} {windows:?}");
            assert_eq!(
                speaking.in_place_of.map(|language| language.english_name),
                Some(named)
            );
            let note = with_language(ENGLISH, || console_note(&speaking));
            assert_eq!(
                note.as_deref(),
                Some(
                    format!("This console cannot show {named}, so the client speaks English.")
                        .as_str()
                )
            );
        }
    }

    #[test]
    fn a_right_to_left_language_that_does_not_ship_needs_no_note() {
        let speaking = choose_language(Some("ar-SA"), &tags(&["he-IL"]), shipping(&[]));
        assert_eq!(speaking.language, ENGLISH);
        assert_eq!(speaking.in_place_of, None);
        assert_eq!(console_note(&speaking), None);
    }

    #[test]
    fn the_value_after_a_flag_is_read() {
        let arguments = || tags(&["client.exe", "--code", "123456", "--lang", "it-IT"]);
        assert_eq!(argument_after(arguments(), LANG).as_deref(), Some("it-IT"));
        assert_eq!(argument_after(arguments(), CODE).as_deref(), Some("123456"));
        assert_eq!(argument_after(tags(&["client.exe", "--lang"]), LANG), None);
        assert_eq!(argument_after(tags(&["client.exe"]), LANG), None);
    }

    /// The English of the client's screen, which these messages must keep exactly as the code printed it.
    #[test]
    fn the_screen_reads_as_it_did_in_english() {
        with_language(ENGLISH, || {
            assert_eq!(
                t!("client-intro"),
                "This lets WinAppMigrator on your new PC read the apps on this PC over your\n\
                 network. It changes nothing here: it reads a snapshot of the Windows drive,\n\
                 which it removes when you stop it."
            );
            assert_eq!(
                t!("client-safe-folder-failed", error = "Access is denied."),
                "WinAppMigrator Client could not start from a folder only administrators\n\
                 can change: Access is denied."
            );
            assert_eq!(
                t!("client-caretaker-failed", error = "Access is denied."),
                "If this window is closed rather than stopped, the snapshot and firewall\n\
                 rules stay until the client is next run here (Access is denied.)."
            );
            assert_eq!(
                t!("client-snapshot-taking"),
                "Taking a snapshot of the Windows drive... "
            );
            assert_eq!(
                t!("client-discovery-failed", error = "in use"),
                "This PC will not show up by itself (in use).\n\
                 On the new PC, type this PC's name or address instead."
            );
            assert_eq!(
                t!("client-pairing-steps"),
                "On your new PC, in WinAppMigrator, choose Network PC... and type this code.\n\
                 Close the app you are moving first, so its files are as it left them.\n\
                 To stop, press Enter here or close this window."
            );
            assert_eq!(
                t!("client-too-many-wrong-codes"),
                "Too many wrong codes were tried, so no PC can pair with this one now.\n\
                 Something on your network may be guessing. To pair, stop the client\n\
                 and start it again, and type the new code it shows."
            );
            assert_eq!(
                t!("client-this-pc", computer_name = "OLD-PC"),
                "  This PC:       OLD-PC"
            );
            assert_eq!(
                t!(
                    "client-windows",
                    windows_name = "Windows 7 Professional Service Pack 1",
                    bits = 32
                ),
                "  Windows:       Windows 7 Professional Service Pack 1, 32-bit"
            );
            assert_eq!(
                t!("client-pairing-code", code = "482 913"),
                "  Pairing Code:  482 913"
            );
            assert_eq!(
                t!("client-paired", from = "192.168.1.20"),
                "Paired with 192.168.1.20. It is reading what it needs..."
            );
            assert_eq!(
                t!("client-wrong-code", from = "unknown"),
                "Another PC tried a code that is not this one."
            );
            assert_eq!(
                t!("client-builder-done", from = "192.168.1.20", count = 1_u64),
                "192.168.1.20 is done (1 request answered)."
            );
            assert_eq!(
                t!("client-builder-done", from = "unknown", count = 812_u64),
                "Another PC is done (812 requests answered)."
            );
            assert_eq!(
                t!("client-snapshot-failed", drive = "C:", details = ""),
                "Windows did not make a snapshot of C:: "
            );
            assert_eq!(
                t!("client-usage"),
                "WinAppMigrator Client takes these options:\n\
                 \n\
                 \x20 --lang <language>  Speak this language, such as it-IT, if the client has it,\n\
                 \x20                    rather than Windows' display language.\n\
                 \x20 --code <code>      Start with this six-digit pairing code, for a script that\n\
                 \x20                    starts WinAppMigrator on the new PC too.\n\
                 \x20 --help             Show this."
            );
            for line in t!("client-usage").lines() {
                assert!(line.chars().count() < 80, "{line}");
            }
        });
    }
}
