//! Numbers and sizes, written the way the person reading them writes them.

use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;

use crate::languages::{Grouping, Language, NumberStyle};

/// How numbers are written: Windows' own regional settings when it has them, as every Windows program follows those
/// whatever its language, and the language's otherwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Separators {
    pub decimal: String,
    pub group: String,
    pub grouping: Grouping,
    pub min_grouping_digits: usize,
}

impl Separators {
    /// How a language writes numbers.
    pub fn of(language: &Language) -> Self {
        Self::from_style(language.numbers)
    }

    fn from_style(style: NumberStyle) -> Self {
        Self {
            decimal: style.decimal.to_owned(),
            group: style.group.to_owned(),
            grouping: style.grouping,
            min_grouping_digits: style.min_grouping_digits,
        }
    }

    /// The decimal and grouping marks set in Windows' Region settings for whoever runs this, laid over how `language`
    /// writes numbers. Only a mark Windows has is taken.
    pub fn windows_over(language: &Language) -> Self {
        let mut separators = Self::of(language);
        let Ok(international) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(INTERNATIONAL_KEY)
        else {
            return separators;
        };
        let read = |name: &str| {
            international
                .get_value::<String, _>(name)
                .ok()
                .filter(|value| !value.is_empty() && value.chars().count() <= MAX_MARK_CHARS)
        };
        if let Some(decimal) = read("sDecimal") {
            separators.decimal = decimal;
        }
        if let Some(group) = read("sThousand") {
            separators.group = group;
        }
        if let Some(grouping) = international
            .get_value::<String, _>("sGrouping")
            .ok()
            .and_then(|value| grouping_of(&value))
        {
            separators.grouping = grouping;
        }
        separators
    }
}

const INTERNATIONAL_KEY: &str = r"Control Panel\International";
/// Windows allows up to three characters for each mark.
const MAX_MARK_CHARS: usize = 3;

/// Windows writes grouping as group sizes: `3;0` for thousands, `3;2;0` for India's.
fn grouping_of(value: &str) -> Option<Grouping> {
    match value.trim() {
        "3;0" | "3" => Some(Grouping::Thousands),
        "3;2;0" | "3;2" => Some(Grouping::Indian),
        _ => None,
    }
}

/// A whole number, grouped.
pub fn integer(value: u64, separators: &Separators) -> String {
    group_digits(&value.to_string(), separators)
}

/// A number with exactly `fraction_digits` after the decimal mark.
pub fn decimal(value: f64, fraction_digits: usize, separators: &Separators) -> String {
    let written = format!("{:.*}", fraction_digits, value.abs());
    let (whole, fraction) = written.split_once('.').unwrap_or((written.as_str(), ""));
    let mut out = String::new();
    if value.is_sign_negative() && value != 0.0 {
        out.push('-');
    }
    out.push_str(&group_digits(whole, separators));
    if !fraction.is_empty() {
        out.push_str(&separators.decimal);
        out.push_str(fraction);
    }
    out
}

fn group_digits(digits: &str, separators: &Separators) -> String {
    if digits.len() < 3 + separators.min_grouping_digits.max(1) {
        return digits.to_owned();
    }
    let (head, last_three) = digits.split_at(digits.len() - 3);
    let size = match separators.grouping {
        Grouping::Thousands => 3,
        Grouping::Indian => 2,
    };
    let mut groups: Vec<&str> = Vec::new();
    let mut rest = head;
    while rest.len() > size {
        let (left, right) = rest.split_at(rest.len() - size);
        groups.push(right);
        rest = left;
    }
    groups.push(rest);
    groups.reverse();
    groups.push(last_three);
    groups.join(&separators.group)
}

pub(crate) const KIB: u64 = 1024;
pub(crate) const MIB: u64 = KIB * 1024;
pub(crate) const GIB: u64 = MIB * 1024;
pub(crate) const TIB: u64 = GIB * 1024;

/// Which unit a size is written in, with its value in that unit.
pub(crate) fn size_parts(bytes: u64) -> (SizeUnit, f64) {
    for (unit, size) in [
        (SizeUnit::Terabytes, TIB),
        (SizeUnit::Gigabytes, GIB),
        (SizeUnit::Megabytes, MIB),
        (SizeUnit::Kilobytes, KIB),
    ] {
        if bytes >= size {
            return (unit, bytes as f64 / size as f64);
        }
    }
    (SizeUnit::Bytes, bytes as f64)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl SizeUnit {
    /// The catalog message that writes a size in this unit.
    pub(crate) fn message(self) -> &'static str {
        match self {
            Self::Bytes => "size-bytes",
            Self::Kilobytes => "size-kb",
            Self::Megabytes => "size-mb",
            Self::Gigabytes => "size-gb",
            Self::Terabytes => "size-tb",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::language;

    fn of(tag: &str) -> Separators {
        language(tag).map_or_else(|| panic!("{tag} ships"), Separators::of)
    }

    #[test]
    fn whole_numbers_are_grouped_the_language_s_way() {
        assert_eq!(integer(12_345, &of("en-US")), "12,345");
        assert_eq!(integer(12_345, &of("it-IT")), "12.345");
        assert_eq!(integer(1_234_567, &of("fr-FR")), "1\u{202f}234\u{202f}567");
        assert_eq!(integer(1_234_567, &of("hi-IN")), "12,34,567");
        assert_eq!(integer(123, &of("de-DE")), "123");
        assert_eq!(integer(0, &of("en-US")), "0");
    }

    #[test]
    fn spanish_and_polish_leave_four_digits_alone() {
        assert_eq!(integer(1234, &of("es-ES")), "1234");
        assert_eq!(integer(12_345, &of("es-ES")), "12.345");
        assert_eq!(integer(1234, &of("pl-PL")), "1234");
        assert_eq!(integer(1234, &of("en-US")), "1,234");
    }

    #[test]
    fn decimals_use_the_language_s_mark() {
        assert_eq!(decimal(824.0, 1, &of("en-US")), "824.0");
        assert_eq!(decimal(824.0, 1, &of("it-IT")), "824,0");
        assert_eq!(decimal(1234.56, 1, &of("de-DE")), "1.234,6");
        assert_eq!(decimal(-2.5, 1, &of("en-US")), "-2.5");
        assert_eq!(decimal(3.0, 0, &of("en-US")), "3");
    }

    #[test]
    fn windows_marks_override_the_language_s() {
        let mut separators = of("en-US");
        separators.decimal = ",".to_owned();
        separators.group = "'".to_owned();
        assert_eq!(decimal(12_345.0, 1, &separators), "12'345,0");
    }

    #[test]
    fn windows_grouping_is_read() {
        assert_eq!(grouping_of("3;0"), Some(Grouping::Thousands));
        assert_eq!(grouping_of("3;2;0"), Some(Grouping::Indian));
        assert_eq!(grouping_of("0;0"), None);
    }

    #[test]
    fn sizes_pick_their_unit() {
        assert_eq!(size_parts(0), (SizeUnit::Bytes, 0.0));
        assert_eq!(size_parts(1023).0, SizeUnit::Bytes);
        assert_eq!(size_parts(1024).0, SizeUnit::Kilobytes);
        assert_eq!(size_parts(824 * MIB), (SizeUnit::Megabytes, 824.0));
        assert_eq!(size_parts(GIB + GIB / 5).0, SizeUnit::Gigabytes);
        assert_eq!(size_parts(3 * TIB).0, SizeUnit::Terabytes);
    }
}
