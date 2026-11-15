//! Two languages for testing, made from the English catalog and never offered to anyone: one with every letter of the
//! catalog's own text accented, marked at both ends, and 40% longer, which shows text a window cuts off and any English
//! left in code; and the same written right to left, which shows what a window does not mirror.
//!
//! Only the catalog's text is changed. What a message is given to show, a path or an app's name, is left alone, so a
//! test can tell what came from the catalog from what came from the PC.

use fluent_syntax::ast::{Entry, Expression, InlineExpression, Pattern, PatternElement};
use fluent_syntax::parser;
use fluent_syntax::serializer;

use crate::languages::{Direction, Language, POINT_COMMA, Script, lang};

/// Accented, longer English, left to right.
pub const PSEUDO_LONG: Language = lang(
    "en-XA",
    "[Pseudo, Long]",
    "Pseudo (long)",
    Direction::LeftToRight,
    Script::Latin,
    POINT_COMMA,
);

/// The same, right to left.
pub const PSEUDO_RTL: Language = lang(
    "ar-XB",
    "[Pseudo, Right to Left]",
    "Pseudo (right to left)",
    Direction::RightToLeft,
    Script::Latin,
    POINT_COMMA,
);

/// Opens every message in a test language.
pub const START: char = '«';
/// Closes every message in a test language.
pub const END: char = '»';
/// How much longer a test language's text is than the English, as German and Russian often are.
const GROWTH: f64 = 0.4;
const PADDING: char = '~';

/// The test language's catalog, from the English one's source. `None` if the English does not parse, which its own
/// tests rule out.
pub(crate) fn pseudo_source(english: &str) -> Option<String> {
    let mut resource = parser::parse(english.to_owned()).ok()?;
    for entry in &mut resource.body {
        match entry {
            Entry::Message(message) => {
                if let Some(value) = message.value.as_mut() {
                    mark(value);
                }
                for attribute in &mut message.attributes {
                    mark(&mut attribute.value);
                }
            }
            Entry::Term(term) => {
                mark(&mut term.value);
                for attribute in &mut term.attributes {
                    mark(&mut attribute.value);
                }
            }
            _ => {}
        }
    }
    Some(serializer::serialize(&resource))
}

/// Accent a pattern's text, pad it, and put the markers around it.
fn mark(pattern: &mut Pattern<String>) {
    let letters = accent_pattern(pattern);
    let padding = (letters as f64 * GROWTH).ceil() as usize;
    let mut end = String::new();
    end.extend(std::iter::repeat_n(PADDING, padding));
    end.push(END);
    pattern.elements.insert(
        0,
        PatternElement::TextElement {
            value: START.to_string(),
        },
    );
    pattern
        .elements
        .push(PatternElement::TextElement { value: end });
}

/// Accent every text element in a pattern, including inside selections, and count the letters.
fn accent_pattern(pattern: &mut Pattern<String>) -> usize {
    let mut letters = 0;
    for element in &mut pattern.elements {
        match element {
            PatternElement::TextElement { value } => {
                letters += value.chars().filter(|c| c.is_alphabetic()).count();
                *value = value.chars().map(accented).collect();
            }
            PatternElement::Placeable { expression } => letters += accent_expression(expression),
        }
    }
    letters
}

fn accent_expression(expression: &mut Expression<String>) -> usize {
    match expression {
        Expression::Select { variants, .. } => variants
            .iter_mut()
            .map(|variant| accent_pattern(&mut variant.value))
            .max()
            .unwrap_or(0),
        Expression::Inline(InlineExpression::Placeable { expression }) => {
            accent_expression(expression)
        }
        Expression::Inline(_) => 0,
    }
}

fn accented(c: char) -> char {
    match c {
        'a' => 'á',
        'c' => 'ç',
        'e' => 'é',
        'i' => 'í',
        'n' => 'ñ',
        'o' => 'ö',
        's' => 'š',
        'u' => 'ü',
        'y' => 'ý',
        'z' => 'ž',
        'A' => 'Å',
        'C' => 'Ç',
        'E' => 'É',
        'I' => 'Î',
        'N' => 'Ñ',
        'O' => 'Ö',
        'S' => 'Š',
        'U' => 'Ü',
        'Y' => 'Ý',
        'Z' => 'Ž',
        other => other,
    }
}

/// Whether a piece of text came from a test language's catalog.
pub fn is_marked(text: &str) -> bool {
    text.contains(START) && text.contains(END)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_accented_padded_and_marked_and_what_it_is_given_is_left_alone() {
        let source = "greeting = Install { $app } now\nchoice = { $count ->\n    [one] One file\n   *[other] { $count } files\n}\n";
        let Some(pseudo) = pseudo_source(source) else {
            panic!("the pseudo catalog was not made");
        };
        assert!(pseudo.contains("Îñštáll"), "{pseudo}");
        assert!(pseudo.contains("{ $app }"), "{pseudo}");
        assert!(pseudo.contains('«'), "{pseudo}");
        assert!(pseudo.contains("~~~~»"), "{pseudo}");
        assert!(
            pseudo.contains(
                "Öñé fílé
"
            ),
            "{pseudo}"
        );
        assert!(pseudo.contains("{ $count } fíléš"), "{pseudo}");
        assert!(pseudo.contains("[one]"), "selectors are kept: {pseudo}");
        assert!(pseudo_source(&pseudo).is_some(), "it parses again");
    }

    #[test]
    fn text_that_does_not_parse_makes_no_catalog() {
        assert!(pseudo_source("broken = { $\n").is_none());
    }

    #[test]
    fn marked_text_is_told_from_unmarked() {
        assert!(is_marked("«Îñštáll~~»"));
        assert!(!is_marked("C:\\Program Files\\App"));
    }
}
