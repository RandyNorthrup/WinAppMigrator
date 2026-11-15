//! The languages WinAppMigrator ships, and which of them a person gets.

use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

/// Which way a language's lines run, and so which way its windows are laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

/// The writing a language's text is in, which decides the Windows font that can draw it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    Latin,
    Cyrillic,
    SimplifiedChinese,
    TraditionalChinese,
    Japanese,
    Korean,
    Devanagari,
    Bengali,
    Thai,
    Arabic,
    Hebrew,
}

/// How digits are grouped in a large number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grouping {
    /// 1,234,567
    Thousands,
    /// 12,34,567: the first group of three, then twos, as India and Bangladesh write them.
    Indian,
}

/// How a language writes numbers when Windows does not say otherwise. Digits are 0 to 9 in every language,
/// as they appear beside paths, versions, and sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberStyle {
    pub decimal: &'static str,
    pub group: &'static str,
    pub grouping: Grouping,
    /// The fewest digits a number needs before it is grouped at all: Spanish and Polish write 1234 but 12 345.
    pub min_grouping_digits: usize,
}

/// A language that ships.
#[derive(Debug, PartialEq, Eq)]
pub struct Language {
    /// Its BCP 47 tag, as its catalog's folder is named.
    pub tag: &'static str,
    /// Its name in itself, as the Language menu lists it.
    pub native_name: &'static str,
    /// Its name in English.
    pub english_name: &'static str,
    pub direction: Direction,
    pub script: Script,
    pub numbers: NumberStyle,
}

pub(crate) const POINT_COMMA: NumberStyle = NumberStyle {
    decimal: ".",
    group: ",",
    grouping: Grouping::Thousands,
    min_grouping_digits: 1,
};
const COMMA_POINT: NumberStyle = NumberStyle {
    decimal: ",",
    group: ".",
    grouping: Grouping::Thousands,
    min_grouping_digits: 1,
};
/// A no-break space, so that a number is never split across lines.
const NBSP: &str = "\u{a0}";
/// A narrow no-break space, as French writes thousands.
const NARROW_NBSP: &str = "\u{202f}";

/// Every language that ships, English first. The order is the Language menu's.
pub const LANGUAGES: &[Language] = &[
    lang(
        "en-US",
        "English",
        "English",
        Direction::LeftToRight,
        Script::Latin,
        POINT_COMMA,
    ),
    lang(
        "es-ES",
        "Español",
        "Spanish",
        Direction::LeftToRight,
        Script::Latin,
        NumberStyle {
            min_grouping_digits: 2,
            ..COMMA_POINT
        },
    ),
    lang(
        "pt-BR",
        "Português (Brasil)",
        "Portuguese (Brazil)",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "fr-FR",
        "Français",
        "French",
        Direction::LeftToRight,
        Script::Latin,
        NumberStyle {
            group: NARROW_NBSP,
            ..COMMA_POINT
        },
    ),
    lang(
        "de-DE",
        "Deutsch",
        "German",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "it-IT",
        "Italiano",
        "Italian",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "nl-NL",
        "Nederlands",
        "Dutch",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "pl-PL",
        "Polski",
        "Polish",
        Direction::LeftToRight,
        Script::Latin,
        NumberStyle {
            group: NBSP,
            min_grouping_digits: 2,
            ..COMMA_POINT
        },
    ),
    lang(
        "ru-RU",
        "Русский",
        "Russian",
        Direction::LeftToRight,
        Script::Cyrillic,
        NumberStyle {
            group: NBSP,
            ..COMMA_POINT
        },
    ),
    lang(
        "uk-UA",
        "Українська",
        "Ukrainian",
        Direction::LeftToRight,
        Script::Cyrillic,
        NumberStyle {
            group: NBSP,
            ..COMMA_POINT
        },
    ),
    lang(
        "tr-TR",
        "Türkçe",
        "Turkish",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "zh-CN",
        "中文（简体）",
        "Chinese (Simplified)",
        Direction::LeftToRight,
        Script::SimplifiedChinese,
        POINT_COMMA,
    ),
    lang(
        "zh-TW",
        "中文（繁體）",
        "Chinese (Traditional)",
        Direction::LeftToRight,
        Script::TraditionalChinese,
        POINT_COMMA,
    ),
    lang(
        "ja-JP",
        "日本語",
        "Japanese",
        Direction::LeftToRight,
        Script::Japanese,
        POINT_COMMA,
    ),
    lang(
        "ko-KR",
        "한국어",
        "Korean",
        Direction::LeftToRight,
        Script::Korean,
        POINT_COMMA,
    ),
    lang(
        "hi-IN",
        "हिन्दी",
        "Hindi",
        Direction::LeftToRight,
        Script::Devanagari,
        NumberStyle {
            grouping: Grouping::Indian,
            ..POINT_COMMA
        },
    ),
    lang(
        "bn-BD",
        "বাংলা",
        "Bengali",
        Direction::LeftToRight,
        Script::Bengali,
        NumberStyle {
            grouping: Grouping::Indian,
            ..POINT_COMMA
        },
    ),
    lang(
        "id-ID",
        "Bahasa Indonesia",
        "Indonesian",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "vi-VN",
        "Tiếng Việt",
        "Vietnamese",
        Direction::LeftToRight,
        Script::Latin,
        COMMA_POINT,
    ),
    lang(
        "th-TH",
        "ไทย",
        "Thai",
        Direction::LeftToRight,
        Script::Thai,
        POINT_COMMA,
    ),
    lang(
        "ar-SA",
        "العربية",
        "Arabic",
        Direction::RightToLeft,
        Script::Arabic,
        POINT_COMMA,
    ),
    lang(
        "fa-IR",
        "فارسی",
        "Persian",
        Direction::RightToLeft,
        Script::Arabic,
        POINT_COMMA,
    ),
    lang(
        "ur-PK",
        "اردو",
        "Urdu",
        Direction::RightToLeft,
        Script::Arabic,
        POINT_COMMA,
    ),
    lang(
        "he-IL",
        "עברית",
        "Hebrew",
        Direction::RightToLeft,
        Script::Hebrew,
        POINT_COMMA,
    ),
];

/// English, which every message has, and every other language falls back to.
pub const ENGLISH: &Language = &LANGUAGES[0];

pub(crate) const fn lang(
    tag: &'static str,
    native_name: &'static str,
    english_name: &'static str,
    direction: Direction,
    script: Script,
    numbers: NumberStyle,
) -> Language {
    Language {
        tag,
        native_name,
        english_name,
        direction,
        script,
        numbers,
    }
}

/// The shipped language with exactly this tag, ignoring case.
pub fn language(tag: &str) -> Option<&'static Language> {
    LANGUAGES
        .iter()
        .find(|language| language.tag.eq_ignore_ascii_case(tag.trim()))
}

/// The language a person gets: what they chose, if it still ships; else the first of Windows' display languages that
/// ships, by its language when not by its region; else English.
pub fn negotiate(
    saved: Option<&str>,
    preferred: &[String],
    ships: impl Fn(&Language) -> bool,
) -> &'static Language {
    if let Some(chosen) = saved.and_then(language).filter(|chosen| ships(chosen)) {
        return chosen;
    }
    preferred
        .iter()
        .filter_map(|tag| closest(tag))
        .find(|language| ships(language))
        .unwrap_or(ENGLISH)
}

/// The shipped language closest to a BCP 47 tag: the same tag, else Chinese by its script or region, else the same
/// language anywhere it is spoken.
fn closest(tag: &str) -> Option<&'static Language> {
    if let Some(exact) = language(tag) {
        return Some(exact);
    }
    let lower = tag.trim().to_ascii_lowercase().replace('_', "-");
    let mut parts = lower.split('-');
    let primary = parts.next().filter(|primary| !primary.is_empty())?;
    if primary == "zh" {
        let traditional = parts.any(|part| matches!(part, "hant" | "tw" | "hk" | "mo"));
        return language(if traditional { "zh-TW" } else { "zh-CN" });
    }
    LANGUAGES.iter().find(|language| {
        language
            .tag
            .split('-')
            .next()
            .is_some_and(|its| its.eq_ignore_ascii_case(primary))
    })
}

const USER_PROFILE_KEY: &str = r"Control Panel\International\User Profile";
const DESKTOP_KEY: &str = r"Control Panel\Desktop";
const MUI_CACHED_KEY: &str = r"Control Panel\Desktop\MuiCached";
const NLS_LANGUAGE_KEY: &str = r"SYSTEM\CurrentControlSet\Control\Nls\Language";

/// Windows' display languages for whoever runs this, most preferred first, as BCP 47 tags. Read from the registry,
/// where Windows 8 and later keep the list a person orders in Settings, with Windows 7's places after it. Empty when none
/// can be read, which leaves English.
pub fn windows_display_languages() -> Vec<String> {
    let user = RegKey::predef(HKEY_CURRENT_USER);
    let mut tags = Vec::new();
    for (key, value) in [
        (DESKTOP_KEY, "PreferredUILanguages"),
        (USER_PROFILE_KEY, "Languages"),
        (MUI_CACHED_KEY, "MachinePreferredUILanguages"),
    ] {
        if let Ok(list) = user
            .open_subkey(key)
            .and_then(|key| key.get_value::<Vec<String>, _>(value))
        {
            tags.extend(list.into_iter().filter(|tag| !tag.trim().is_empty()));
        }
    }
    if let Some(tag) = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(NLS_LANGUAGE_KEY)
        .and_then(|key| key.get_value::<String, _>("InstallLanguage"))
        .ok()
        .and_then(|lcid| tag_of_lcid(&lcid))
    {
        tags.push(tag.to_owned());
    }
    tags
}

/// The tag of a Windows language ID written in hex, as Windows 7 records the language it was installed in, for the
/// languages that ship and the regional variants they cover.
fn tag_of_lcid(lcid: &str) -> Option<&'static str> {
    let id = u32::from_str_radix(lcid.trim(), 16).ok()?;
    Some(match id {
        0x0409 => "en-US",
        0x0809 => "en-GB",
        0x0c0a => "es-ES",
        0x080a => "es-MX",
        0x0416 => "pt-BR",
        0x0816 => "pt-PT",
        0x040c => "fr-FR",
        0x0407 => "de-DE",
        0x0410 => "it-IT",
        0x0413 => "nl-NL",
        0x0415 => "pl-PL",
        0x0419 => "ru-RU",
        0x0422 => "uk-UA",
        0x041f => "tr-TR",
        0x0804 => "zh-CN",
        0x0404 => "zh-TW",
        0x0c04 => "zh-HK",
        0x0411 => "ja-JP",
        0x0412 => "ko-KR",
        0x0439 => "hi-IN",
        0x0845 => "bn-BD",
        0x0421 => "id-ID",
        0x042a => "vi-VN",
        0x041e => "th-TH",
        0x0401 => "ar-SA",
        0x0429 => "fa-IR",
        0x0420 => "ur-PK",
        0x040d => "he-IL",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(list: &[&str]) -> Vec<String> {
        list.iter().map(|tag| (*tag).to_owned()).collect()
    }

    fn pick(saved: Option<&str>, preferred: &[String]) -> &'static Language {
        negotiate(saved, preferred, |_| true)
    }

    #[test]
    fn only_a_language_with_a_catalog_is_chosen() {
        let only_english_and_french =
            |language: &Language| matches!(language.tag, "en-US" | "fr-FR");
        assert_eq!(
            negotiate(
                Some("de-DE"),
                &tags(&["it-IT", "fr-CA"]),
                only_english_and_french
            )
            .tag,
            "fr-FR"
        );
        assert_eq!(
            negotiate(None, &tags(&["it-IT"]), only_english_and_french).tag,
            "en-US"
        );
    }

    #[test]
    fn a_saved_choice_that_ships_wins() {
        assert_eq!(pick(Some("de-DE"), &tags(&["it-IT"])).tag, "de-DE");
        assert_eq!(pick(Some("DE-de"), &[]).tag, "de-DE");
    }

    #[test]
    fn a_saved_choice_that_no_longer_ships_is_passed_over() {
        assert_eq!(pick(Some("xx-YY"), &tags(&["it-IT"])).tag, "it-IT");
        assert_eq!(pick(Some(""), &[]).tag, "en-US");
    }

    #[test]
    fn the_first_preferred_language_that_ships_is_chosen() {
        assert_eq!(pick(None, &tags(&["it-IT", "en-US"])).tag, "it-IT");
        assert_eq!(pick(None, &tags(&["sr-Latn-RS", "fr-FR"])).tag, "fr-FR");
        assert_eq!(pick(None, &tags(&["sr-Latn-RS"])).tag, "en-US");
        assert_eq!(pick(None, &[]).tag, "en-US");
    }

    #[test]
    fn a_regional_variant_gets_its_language() {
        for (asked, given) in [
            ("pt-PT", "pt-BR"),
            ("es-MX", "es-ES"),
            ("en-GB", "en-US"),
            ("fr-CA", "fr-FR"),
            ("ar-EG", "ar-SA"),
            ("de", "de-DE"),
            ("zh-SG", "zh-CN"),
            ("zh-Hans-CN", "zh-CN"),
            ("zh-HK", "zh-TW"),
            ("zh-MO", "zh-TW"),
            ("zh-Hant-HK", "zh-TW"),
            ("zh", "zh-CN"),
            ("nb-NO", "en-US"),
        ] {
            assert_eq!(pick(None, &tags(&[asked])).tag, given, "{asked}");
        }
    }

    #[test]
    fn the_test_languages_are_never_chosen_from_windows_list() {
        for asked in ["en-XA", "ar-XB", "qps-ploc", "qps-plocm"] {
            let chosen = pick(Some(asked), &tags(&[asked]));
            assert!(
                chosen.tag == "en-US" || chosen.tag == "ar-SA",
                "{asked}: {}",
                chosen.tag
            );
            assert!(!chosen.tag.ends_with("XA") && !chosen.tag.ends_with("XB"));
        }
    }

    #[test]
    fn windows_7_language_ids_are_read() {
        assert_eq!(tag_of_lcid("0410"), Some("it-IT"));
        assert_eq!(tag_of_lcid("0c04"), Some("zh-HK"));
        assert_eq!(tag_of_lcid("ffff"), None);
        assert_eq!(tag_of_lcid("not hex"), None);
    }

    #[test]
    fn every_language_is_listed_once_with_english_first() {
        assert_eq!(LANGUAGES.len(), 24);
        assert_eq!(ENGLISH.tag, "en-US");
        for (index, language) in LANGUAGES.iter().enumerate() {
            assert!(
                LANGUAGES[index + 1..]
                    .iter()
                    .all(|other| other.tag != language.tag),
                "{} twice",
                language.tag
            );
        }
        let right_to_left: Vec<&str> = LANGUAGES
            .iter()
            .filter(|language| language.direction == Direction::RightToLeft)
            .map(|language| language.tag)
            .collect();
        assert_eq!(right_to_left, ["ar-SA", "fa-IR", "ur-PK", "he-IL"]);
    }
}
