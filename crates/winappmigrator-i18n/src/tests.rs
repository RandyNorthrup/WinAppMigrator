use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fluent_syntax::ast::{Entry, Expression, InlineExpression, Pattern, PatternElement};
use fluent_syntax::parser;

use super::*;

/// Run `work` speaking `language` with a catalog made from `source`, for checking what happens with catalogs that do
/// not ship.
fn with_source<R>(language: &'static Language, source: &str, work: impl FnOnce() -> R) -> R {
    let active = Arc::new(Active {
        language,
        separators: Separators::of(language),
        catalog: build(language, source.to_owned()).map(Arc::new),
    });
    on_this_thread(active, work)
}

fn lang(tag: &str) -> &'static Language {
    language(tag).unwrap_or_else(|| panic!("{tag} is in the table"))
}

// ---- The shape of a catalog: its messages, their attributes, and what each part is given ----

/// Message ID, then `""` for its value or the attribute's name, then the variables it uses.
type Shape = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;

fn shape(source: &str) -> Result<Shape, String> {
    let resource = parser::parse(source).map_err(|(_, errors)| format!("{errors:?}"))?;
    let mut shape = Shape::new();
    for entry in &resource.body {
        match entry {
            Entry::Message(message) => {
                let parts = shape.entry(message.id.name.to_owned()).or_default();
                if let Some(value) = &message.value {
                    parts.insert(String::new(), variables(value));
                }
                for attribute in &message.attributes {
                    parts.insert(attribute.id.name.to_owned(), variables(&attribute.value));
                }
            }
            Entry::Junk { content } => return Err(format!("does not parse: {content}")),
            _ => {}
        }
    }
    Ok(shape)
}

fn variables(pattern: &Pattern<&str>) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    collect_pattern(pattern, &mut found);
    found
}

fn collect_pattern(pattern: &Pattern<&str>, found: &mut BTreeSet<String>) {
    for element in &pattern.elements {
        if let PatternElement::Placeable { expression } = element {
            collect_expression(expression, found);
        }
    }
}

fn collect_expression(expression: &Expression<&str>, found: &mut BTreeSet<String>) {
    match expression {
        Expression::Select { selector, variants } => {
            collect_inline(selector, found);
            for variant in variants {
                collect_pattern(&variant.value, found);
            }
        }
        Expression::Inline(inline) => collect_inline(inline, found),
    }
}

fn collect_inline(inline: &InlineExpression<&str>, found: &mut BTreeSet<String>) {
    match inline {
        InlineExpression::VariableReference { id } => {
            found.insert(id.name.to_owned());
        }
        InlineExpression::FunctionReference { arguments, .. } => {
            for argument in &arguments.positional {
                collect_inline(argument, found);
            }
        }
        InlineExpression::Placeable { expression } => collect_expression(expression, found),
        _ => {}
    }
}

/// How a translation's shape differs from the English, as sentences; empty when it matches.
fn differences(english: &Shape, translated: &Shape) -> Vec<String> {
    let mut found = Vec::new();
    for (id, parts) in english {
        let Some(theirs) = translated.get(id) else {
            found.push(format!("{id} is missing"));
            continue;
        };
        for (part, wanted) in parts {
            let name = if part.is_empty() {
                id.clone()
            } else {
                format!("{id}.{part}")
            };
            match theirs.get(part) {
                None => found.push(format!("{name} is missing")),
                Some(given) if given != wanted => {
                    found.push(format!("{name} uses {given:?}, not {wanted:?}"));
                }
                Some(_) => {}
            }
        }
        for part in theirs.keys().filter(|part| !parts.contains_key(*part)) {
            found.push(format!("{id}.{part} is not in English"));
        }
    }
    for id in translated.keys().filter(|id| !english.contains_key(*id)) {
        found.push(format!("{id} is not in English"));
    }
    found
}

fn english_shape() -> Shape {
    let source = catalog_source(ENGLISH.tag).unwrap_or_default();
    shape(&source).unwrap_or_else(|problem| panic!("English: {problem}"))
}

#[test]
fn english_is_built_in_and_parses_whole() {
    let Some(source) = catalog_source(ENGLISH.tag) else {
        panic!("no English catalog is built in");
    };
    assert!(build(ENGLISH, source).is_some());
    assert!(!english_shape().is_empty());
}

/// AC-I18N-CATALOG.
#[test]
fn every_catalog_has_exactly_the_english_messages_with_their_placeholders() {
    let english = english_shape();
    let mut problems = Vec::new();
    for (tag, _) in CATALOGS {
        let source = catalog_source(tag).unwrap_or_default();
        match shape(&source) {
            Ok(translated) => problems.extend(
                differences(&english, &translated)
                    .into_iter()
                    .map(|problem| format!("{tag}: {problem}")),
            ),
            Err(problem) => problems.push(format!("{tag}: {problem}")),
        }
        let Some(language) = language(tag) else {
            problems.push(format!("{tag} is not in the language table"));
            continue;
        };
        if build(language, source).is_none() {
            problems.push(format!("{tag}: the catalog cannot be loaded"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

#[test]
fn a_catalog_that_differs_from_english_is_caught() -> Result<(), String> {
    let english = shape("a = A { $x }\nb = B\n    .tip = Tip\nc = C\n")?;
    let translated = shape("a = A { $y }\nb = B\nd = D\n")?;
    let found = differences(&english, &translated);
    assert!(found.iter().any(|it| it.starts_with("a uses")), "{found:?}");
    assert!(found.contains(&"b.tip is missing".to_owned()), "{found:?}");
    assert!(found.contains(&"c is missing".to_owned()), "{found:?}");
    assert!(
        found.contains(&"d is not in English".to_owned()),
        "{found:?}"
    );
    assert!(shape("broken = { $\n").is_err());
    Ok(())
}

/// Every message the code asks for by name, in every crate, is in English, so a mistyped name cannot reach a person
/// as itself.
#[test]
fn every_message_the_code_asks_for_is_in_english() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    rust_files(&crates, &mut files);
    assert!(files.len() > 20, "found only {} source files", files.len());
    let english = english_shape();
    let mut asked = 0;
    let mut missing = Vec::new();
    for file in files
        .iter()
        .filter(|file| !file.ends_with("winappmigrator-i18n/src/tests.rs"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        for (id, attribute) in asked_for(&text) {
            asked += 1;
            let known = english
                .get(&id)
                .is_some_and(|parts| parts.contains_key(attribute.as_deref().unwrap_or("")));
            if !known {
                missing.push(format!("{}: {id} {attribute:?}", file.display()));
            }
        }
    }
    for unit in [
        format::SizeUnit::Bytes,
        format::SizeUnit::Kilobytes,
        format::SizeUnit::Megabytes,
        format::SizeUnit::Gigabytes,
        format::SizeUnit::Terabytes,
    ] {
        asked += 1;
        if !english.contains_key(unit.message()) {
            missing.push(unit.message().to_owned());
        }
    }
    assert!(asked > 0);
    assert!(missing.is_empty(), "{missing:#?}");
}

fn rust_files(folder: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name != "target") {
                rust_files(&path, files);
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

/// The message names in `t!("…")`, `tr("…")`, and `tr_attribute("…", "…")` calls.
fn asked_for(text: &str) -> Vec<(String, Option<String>)> {
    let mut found = Vec::new();
    for (opening, has_attribute) in [
        ("t!(\"", false),
        ("t_english!(\"", false),
        ("tr(\"", false),
        ("tr_english(\"", false),
        ("tr_attribute(\"", true),
    ] {
        let mut rest = text;
        while let Some(start) = rest.find(opening) {
            // `format!("` ends in `t!("`, and `str("` in `tr("`: a call starts where a name does.
            let part_of_a_name = rest[..start]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            rest = &rest[start + opening.len()..];
            if part_of_a_name {
                continue;
            }
            let Some(end) = rest.find('"') else { break };
            let id = rest[..end].to_owned();
            let attribute = has_attribute
                .then(|| {
                    let after = rest[end + 1..].trim_start().strip_prefix(',')?.trim_start();
                    let after = after.strip_prefix('"')?;
                    after.find('"').map(|close| after[..close].to_owned())
                })
                .flatten();
            found.push((id, attribute));
            rest = &rest[end..];
        }
    }
    // `Warning::localized(code, "message", ...)`: the message is its second string.
    let mut rest = text;
    while let Some(start) = rest.find("Warning::localized(") {
        rest = &rest[start + "Warning::localized(".len()..];
        let mut quoted = rest.split('"').skip(1).step_by(2);
        if let (Some(_code), Some(id)) = (quoted.next(), quoted.next()) {
            found.push((id.to_owned(), None));
        }
    }
    found
}

#[test]
fn calls_are_found_in_source_text() {
    let found = asked_for(
        "format!(\"{x}\"); my_str(\"no\"); let a = t!(\"one\"); let b = t!(\"two\", x = 1); tr(\"three\", None); tr_attribute(\"four\", \"tip\", None);",
    );
    assert!(found.contains(&("one".to_owned(), None)));
    assert!(found.contains(&("two".to_owned(), None)));
    assert!(found.contains(&("three".to_owned(), None)));
    assert!(found.contains(&("four".to_owned(), Some("tip".to_owned()))));
    assert_eq!(found.len(), 4, "{found:?}");
}

// ---- AC-I18N-FALLBACK ----

#[test]
fn a_message_a_language_lacks_is_given_in_english() {
    let text = with_source(lang("it-IT"), "altro = Altro\n", || {
        t!("size-mb", value = "824,0")
    });
    assert_eq!(text, "824,0 MB");
}

#[test]
fn a_catalog_that_does_not_parse_gives_english_not_part_of_itself() {
    let text = with_source(
        lang("it-IT"),
        "size-mb = { $value } Mo\nbroken = { $\n",
        || t!("size-mb", value = "824,0"),
    );
    assert_eq!(text, "824,0 MB");
}

#[test]
fn a_message_nobody_has_is_given_as_its_name_and_a_missing_argument_by_its_name() {
    assert_eq!(
        with_language(ENGLISH, || t!("no-such-message")),
        "no-such-message"
    );
    let text = with_language(ENGLISH, || tr("size-mb", None));
    assert!(text.contains("$value"), "{text}");
    assert!(!text.contains("size-mb"), "{text}");
}

// ---- AC-I18N-PLURAL ----

const PLURALS: &str = "files = { $count ->\n    [zero] zero\n    [one] one\n    [two] two\n    [few] few\n    [many] many\n   *[other] other\n}\n";

#[test]
fn each_language_picks_its_own_plural_form() {
    let counts = [0_u64, 1, 2, 5, 22, 101];
    let forms = |tag: &str| -> Vec<String> {
        with_source(lang(tag), PLURALS, || {
            counts
                .iter()
                .map(|count| t!("files", count = *count))
                .collect()
        })
    };
    assert_eq!(
        forms("en-US"),
        ["other", "one", "other", "other", "other", "other"]
    );
    assert_eq!(
        forms("pl-PL"),
        ["many", "one", "few", "many", "few", "many"]
    );
    assert_eq!(forms("ru-RU"), ["many", "one", "few", "many", "few", "one"]);
    assert_eq!(
        forms("ar-SA"),
        ["zero", "one", "two", "few", "many", "other"]
    );
}

// ---- AC-I18N-SIZE ----

#[test]
fn sizes_and_counts_are_written_the_language_s_way() {
    const MB_824: u64 = 824 * 1024 * 1024;
    assert_eq!(with_language(ENGLISH, || size(MB_824)), "824.0 MB");
    assert_eq!(with_language(lang("it-IT"), || size(MB_824)), "824,0 MB");
    assert_eq!(
        with_language(lang("de-DE"), || size(1_288_490_189)),
        "1,2 GB"
    );
    assert_eq!(with_language(ENGLISH, || size(1)), "1 byte");
    assert_eq!(with_language(ENGLISH, || size(12_345)), "12.1 KB");
    assert_eq!(with_language(ENGLISH, || size(999)), "999 bytes");
    let french = with_source(lang("fr-FR"), "size-mb = { $value } Mo\n", || size(MB_824));
    assert_eq!(french, "824,0 Mo");
    assert_eq!(with_language(ENGLISH, || number(12_345)), "12,345");
    assert_eq!(with_language(lang("it-IT"), || number(12_345)), "12.345");
}

#[test]
fn a_number_in_a_message_uses_the_language_s_marks() {
    let text = with_source(
        lang("it-IT"),
        "files = { $count } file, { $share } %\n",
        || t!("files", count = 12_345_u64, share = 2.5_f64),
    );
    assert_eq!(text, "12.345 file, 2,5 %");
}

#[test]
fn windows_own_marks_win_over_the_language_s() {
    let mut separators = Separators::of(lang("it-IT"));
    separators.decimal = ".".to_owned();
    separators.group = ",".to_owned();
    let text = with_language_and_separators(lang("it-IT"), separators, || size(824 * 1024 * 1024));
    assert_eq!(text, "824.0 MB");
}

// ---- Direction and isolation ----

#[test]
fn a_right_to_left_catalog_isolates_what_it_is_given_and_a_left_to_right_one_does_not() {
    const SOURCE: &str = "to = Install to { $path }\n";
    let path = r"C:\Program Files\App";
    let hebrew = with_source(lang("he-IL"), SOURCE, || t!("to", path = path));
    assert!(
        hebrew.contains(&format!("\u{2068}{path}\u{2069}")),
        "{hebrew:?}"
    );
    assert_eq!(without_isolation(&hebrew), format!("Install to {path}"));
    let italian = with_source(lang("it-IT"), SOURCE, || t!("to", path = path));
    assert_eq!(italian, format!("Install to {path}"));
}

#[test]
fn a_thread_s_language_is_put_back_after() {
    assert_eq!(with_language(lang("it-IT"), || current().tag), "it-IT");
    let nested = with_language(lang("it-IT"), || with_language(lang("he-IL"), direction));
    assert_eq!(nested, Direction::RightToLeft);
    assert_eq!(with_language(ENGLISH, || current().tag), "en-US");
}

// ---- The test languages ----

#[test]
fn the_test_languages_mark_catalog_text_and_leave_what_it_is_given() {
    let text = with_language(&PSEUDO_LONG, || t!("size-mb", value = "824.0"));
    assert!(is_marked(&text), "{text}");
    assert!(text.contains("824.0"), "{text}");
    assert_eq!(
        with_language(&PSEUDO_RTL, direction),
        Direction::RightToLeft
    );
    assert!(is_marked(&with_language(&PSEUDO_RTL, || size(5))));
}

#[test]
fn the_test_languages_never_ship() {
    assert!(!ships(&PSEUDO_LONG));
    assert!(!ships(&PSEUDO_RTL));
    assert!(shipped().all(|language| LANGUAGES.contains(language)));
    assert!(shipped().any(|language| language == ENGLISH));
}

// ---- Size ----

/// Every installer carries every catalog.
#[test]
fn the_catalogs_add_little_to_every_installer() {
    const BUDGET: usize = 2 * 1024 * 1024;
    let total: usize = CATALOGS
        .iter()
        .map(|(_, compressed)| compressed.len())
        .sum();
    assert!(total < BUDGET, "{total} bytes");
}

#[test]
fn english_text_is_english_whatever_the_language() {
    let text = with_source(lang("it-IT"), "size-mb = { $value } Mo\n", || {
        (
            t!("size-mb", value = "1"),
            t_english!("size-mb", value = "1"),
        )
    });
    assert_eq!(text, ("1 Mo".to_owned(), "1 MB".to_owned()));
}

#[test]
fn a_list_is_joined_the_language_s_way() {
    assert_eq!(
        with_language(ENGLISH, || join_list(&["a", "b", "c"])),
        "a, b, c"
    );
    let chinese = with_source(lang("zh-CN"), "list-separator = {\"、\"}\n", || {
        join_list(&["a", "b"])
    });
    assert_eq!(chinese, "a、b");
    assert_eq!(with_language(ENGLISH, || join_list::<&str>(&[])), "");
}
