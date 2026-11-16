//! The words WinAppMigrator shows, in every language it ships, and how numbers and sizes are written in each.
//!
//! A program chooses its language once, at start, with [`use_preferred_language`] or [`use_language`], and from then on
//! [`t!`] gives each message in it. A message a language lacks, or a language whose catalog cannot be read, is given in
//! English, never as its ID. Tests, and code that must write in a language other than the program's, use
//! [`with_language`], which holds for the thread it is called on.
//!
//! In a right-to-left language, what a message is given to show (a path, a version) is set off with Unicode isolation
//! marks, so that it keeps its own direction inside the sentence; the UI's bidirectional layout reads them, and
//! [`without_isolation`] removes them where text is not laid out that way.

mod format;
mod languages;
mod pseudo;

use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};

use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::types::FluentNumber;
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::LanguageIdentifier;

pub use fluent_bundle::{FluentArgs, FluentValue};
pub use format::{Separators, decimal, integer};
pub use languages::{
    Direction, ENGLISH, Grouping, LANGUAGES, Language, NumberStyle, Script, language, negotiate,
    windows_display_languages,
};
pub use pseudo::{END as PSEUDO_END, PSEUDO_LONG, PSEUDO_RTL, START as PSEUDO_START, is_marked};

include!(concat!(env!("OUT_DIR"), "/catalogs.rs"));

/// Show a message from the catalog, in the current language: `t!("size-mb", value = "824.0")`. Each named argument
/// becomes the message's `{ $name }`; a number stays a number, so the language can choose its plural form by it.
#[macro_export]
macro_rules! t {
    ($id:literal $(,)?) => {
        $crate::tr($id, None)
    };
    ($id:literal, $($name:ident = $value:expr),+ $(,)?) => {{
        let mut args = $crate::FluentArgs::new();
        $(args.set(stringify!($name), $crate::FluentValue::from($value));)+
        $crate::tr($id, Some(&args))
    }};
}

/// The same as [`t!`], in English whatever the language in use: for what a package records to be shown later on another
/// PC, which shows it by its name in that PC's language, and for logs.
#[macro_export]
macro_rules! t_english {
    ($id:literal $(,)?) => {
        $crate::tr_english($id, None)
    };
    ($id:literal, $($name:ident = $value:expr),+ $(,)?) => {{
        let mut args = $crate::FluentArgs::new();
        $(args.set(stringify!($name), $crate::FluentValue::from($value));)+
        $crate::tr_english($id, Some(&args))
    }};
}

type Bundle = FluentBundle<FluentResource>;

struct Catalog {
    bundle: Bundle,
}

/// A language in use, with how its numbers are written and its catalog, if one could be read.
struct Active {
    language: &'static Language,
    separators: Separators,
    catalog: Option<Arc<Catalog>>,
}

static CHOSEN: RwLock<Option<Arc<Active>>> = RwLock::new(None);
/// A language's tag, and its catalog, or `None` if it could not be read.
type Loaded = (&'static str, Option<Arc<Catalog>>);

/// Each catalog read so far.
static LOADED: Mutex<Vec<Loaded>> = Mutex::new(Vec::new());

thread_local! {
    static ON_THIS_THREAD: RefCell<Option<Arc<Active>>> = const { RefCell::new(None) };
    /// How numbers are written while a message is formatted, for the formatter Fluent calls, which takes no context.
    static FORMATTING: RefCell<Option<Separators>> = const { RefCell::new(None) };
}

/// Speak `language` from now on, everywhere in this program, writing numbers as Windows' Region settings say.
pub fn use_language(language: &'static Language) {
    use_language_and_separators(language, Separators::windows_over(language));
}

/// Speak English from now on, writing numbers as English does whatever Windows' Region settings
/// say, for what a script reads: `1,234.5` in a log is the same on every PC.
pub fn use_english_for_scripts() {
    use_language_and_separators(ENGLISH, Separators::of(ENGLISH));
}

/// Speak `language` from now on, everywhere in this program, writing numbers with `separators`.
pub fn use_language_and_separators(language: &'static Language, separators: Separators) {
    let active = Arc::new(Active {
        language,
        separators,
        catalog: catalog(language),
    });
    match CHOSEN.write() {
        Ok(mut chosen) => *chosen = Some(active),
        Err(poisoned) => *poisoned.into_inner() = Some(active),
    }
}

/// Choose the language a person gets, from what they chose before if it still ships, else from Windows' display
/// languages, and use it. Returns it.
pub fn use_preferred_language(saved: Option<&str>) -> &'static Language {
    let chosen = negotiate(saved, &windows_display_languages(), ships);
    use_language(chosen);
    chosen
}

/// The language in use on this thread.
pub fn current() -> &'static Language {
    active().language
}

/// Which way the language in use runs.
pub fn direction() -> Direction {
    current().direction
}

/// Whether a language's catalog is built in.
pub fn ships(language: &Language) -> bool {
    CATALOGS.iter().any(|(tag, _)| *tag == language.tag)
}

/// The languages that ship, in the Language menu's order.
pub fn shipped() -> impl Iterator<Item = &'static Language> {
    LANGUAGES.iter().filter(|language| ships(language))
}

/// Run `work` with this thread speaking `language`, writing numbers the language's own way.
pub fn with_language<R>(language: &'static Language, work: impl FnOnce() -> R) -> R {
    with_language_and_separators(language, Separators::of(language), work)
}

/// Run `work` with this thread speaking `language` and writing numbers with `separators`.
pub fn with_language_and_separators<R>(
    language: &'static Language,
    separators: Separators,
    work: impl FnOnce() -> R,
) -> R {
    let active = Arc::new(Active {
        language,
        separators,
        catalog: catalog(language),
    });
    on_this_thread(active, work)
}

fn on_this_thread<R>(active: Arc<Active>, work: impl FnOnce() -> R) -> R {
    /// Puts back what the thread spoke before, however `work` ends.
    struct Restore(Option<Arc<Active>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            let previous = self.0.take();
            ON_THIS_THREAD.with(|cell| *cell.borrow_mut() = previous);
        }
    }
    let _restore = Restore(ON_THIS_THREAD.with(|cell| cell.replace(Some(active))));
    work()
}

fn active() -> Arc<Active> {
    if let Some(active) = ON_THIS_THREAD.with(|cell| cell.borrow().clone()) {
        return active;
    }
    let chosen = match CHOSEN.read() {
        Ok(chosen) => chosen.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    chosen.unwrap_or_else(|| {
        Arc::new(Active {
            language: ENGLISH,
            separators: Separators::of(ENGLISH),
            catalog: catalog(ENGLISH),
        })
    })
}

/// A message in the language in use, or in English if that language lacks it.
pub fn tr(id: &str, args: Option<&FluentArgs>) -> String {
    message(id, None, args)
}

/// A message in English, with numbers written the English way, whatever the language in use.
pub fn tr_english(id: &str, args: Option<&FluentArgs>) -> String {
    with_language(ENGLISH, || tr(id, args))
}

/// One of a message's attributes, such as a control's `.tooltip`, the same way.
pub fn tr_attribute(id: &str, attribute: &str, args: Option<&FluentArgs>) -> String {
    message(id, Some(attribute), args)
}

/// Whether English has a message, for code that chooses a message by name at run time.
pub fn has_message(id: &str) -> bool {
    catalog(ENGLISH).is_some_and(|english| english.bundle.has_message(id))
}

fn message(id: &str, attribute: Option<&str>, args: Option<&FluentArgs>) -> String {
    let active = active();
    let kept_right_to_left;
    let args = match args {
        Some(given) if active.language.direction == Direction::RightToLeft => {
            kept_right_to_left = right_to_left_kept_whole(given);
            Some(&kept_right_to_left)
        }
        other => other,
    };
    active
        .catalog
        .as_deref()
        .and_then(|catalog| format(catalog, &active.separators, id, attribute, args))
        .or_else(|| {
            catalog(ENGLISH)
                .and_then(|english| format(&english, &active.separators, id, attribute, args))
        })
        .unwrap_or_else(|| id.to_owned())
}

/// Right-to-left isolate, and the end of an isolate.
const RIGHT_TO_LEFT_ISOLATE: char = '\u{2067}';
const POP_DIRECTIONAL_ISOLATE: char = '\u{2069}';

/// What a right-to-left sentence is given. Fluent isolates each value by its first strong letter, so a translated phrase
/// that starts with a Latin word, such as "Windows ...", would be laid out left to right and its words turned around.
/// A value that is mostly right-to-left letters is kept right to left as a whole; a path or a name, mostly Latin, is
/// left to Fluent's isolation.
fn right_to_left_kept_whole(args: &FluentArgs<'_>) -> FluentArgs<'static> {
    let mut kept = FluentArgs::new();
    for (name, value) in args.iter() {
        let value = match value {
            FluentValue::String(text) if reads_right_to_left(text) => FluentValue::from(format!(
                "{RIGHT_TO_LEFT_ISOLATE}{text}{POP_DIRECTIONAL_ISOLATE}"
            )),
            other => other.into_owned(),
        };
        kept.set(name.to_owned(), value);
    }
    kept
}

/// Whether more of a text's letters are of right-to-left scripts (Hebrew, and Arabic and the scripts written with it)
/// than of any other.
fn reads_right_to_left(text: &str) -> bool {
    let (right, left) =
        text.chars()
            .filter(|c| c.is_alphabetic())
            .fold((0_usize, 0_usize), |(right, left), c| {
                if is_right_to_left_letter(c) {
                    (right + 1, left)
                } else {
                    (right, left + 1)
                }
            });
    right > left
}

/// A letter of a right-to-left script.
fn is_right_to_left_letter(c: char) -> bool {
    matches!(c,
        '\u{0590}'..='\u{08FF}' | '\u{FB1D}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}'
            | '\u{10800}'..='\u{10FFF}' | '\u{1E800}'..='\u{1EFFF}')
}

fn format(
    catalog: &Catalog,
    separators: &Separators,
    id: &str,
    attribute: Option<&str>,
    args: Option<&FluentArgs>,
) -> Option<String> {
    let message = catalog.bundle.get_message(id)?;
    let pattern = match attribute {
        Some(attribute) => message.get_attribute(attribute)?.value(),
        None => message.value()?,
    };
    let previous = FORMATTING.with(|cell| cell.replace(Some(separators.clone())));
    // A missing argument is written as its name in braces, which says what is missing without losing the rest.
    let mut errors = Vec::new();
    let text = catalog
        .bundle
        .format_pattern(pattern, args, &mut errors)
        .into_owned();
    FORMATTING.with(|cell| *cell.borrow_mut() = previous);
    Some(text)
}

/// Fluent's hook for writing a number in a message, so `{ $count }` reads 12.345 in Italian.
fn format_value(value: &FluentValue, _: &IntlLangMemoizer) -> Option<String> {
    let FluentValue::Number(number) = value else {
        return None;
    };
    FORMATTING.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|separators| write_number(number, separators))
    })
}

/// The most fraction digits a number is written with when its message does not say.
const DEFAULT_MAX_FRACTION_DIGITS: usize = 3;

fn write_number(number: &FluentNumber, separators: &Separators) -> String {
    let least = number.options.minimum_fraction_digits.unwrap_or(0);
    let most = number
        .options
        .maximum_fraction_digits
        .unwrap_or(DEFAULT_MAX_FRACTION_DIGITS.max(least))
        .max(least);
    // As few digits as show the value to `most` places: 2.50 is 2.5, and 824.000 is 824.
    let scaled = |digits: usize| (number.value.abs() * 10f64.powi(digits as i32)).round();
    let mut digits = most;
    while digits > least && scaled(digits) % 10.0 == 0.0 {
        digits -= 1;
    }
    decimal(number.value, digits, separators)
}

/// A size in bytes, in the unit that suits it, as `824.0 MB` reads in English and `824,0 Mo` in French.
pub fn size(bytes: u64) -> String {
    let (unit, value) = format::size_parts(bytes);
    let separators = &active().separators;
    let written = if unit == format::SizeUnit::Bytes {
        integer(bytes, separators)
    } else {
        decimal(value, 1, separators)
    };
    let mut args = FluentArgs::new();
    args.set("value", written);
    args.set("count", bytes);
    tr(unit.message(), Some(&args))
}

/// Items written as a list in the language in use: `services, tasks, shortcuts`, or `服务、任务` in Chinese.
pub fn join_list<S: AsRef<str>>(items: &[S]) -> String {
    let separator = tr("list-separator", None);
    items
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(&separator)
}

/// A whole number, grouped the way the language in use and Windows' Region settings write it.
pub fn number(value: u64) -> String {
    integer(value, &active().separators)
}

/// `text` without the Unicode isolation marks a right-to-left catalog puts around what a message is given, for text
/// that is not laid out bidirectionally, such as a log line.
pub fn without_isolation(text: &str) -> Cow<'_, str> {
    if text.chars().any(is_isolation_mark) {
        Cow::Owned(text.chars().filter(|c| !is_isolation_mark(*c)).collect())
    } else {
        Cow::Borrowed(text)
    }
}

/// U+2066 to U+2069: left-to-right, right-to-left, and first-strong isolates, and their end.
pub fn is_isolation_mark(c: char) -> bool {
    ('\u{2066}'..='\u{2069}').contains(&c)
}

fn catalog(language: &'static Language) -> Option<Arc<Catalog>> {
    let mut loaded = match LOADED.lock() {
        Ok(loaded) => loaded,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some((_, catalog)) = loaded.iter().find(|(tag, _)| *tag == language.tag) {
        return catalog.clone();
    }
    let source = if language == &PSEUDO_LONG || language == &PSEUDO_RTL {
        catalog_source(ENGLISH.tag).and_then(|english| pseudo::pseudo_source(&english))
    } else {
        catalog_source(language.tag)
    };
    let catalog = source
        .and_then(|source| build(language, source))
        .map(Arc::new);
    loaded.push((language.tag, catalog.clone()));
    catalog
}

/// A language's catalog source, as built in, also for checks such as whether its fonts draw every letter in it.
pub fn catalog_source(tag: &str) -> Option<String> {
    let (_, compressed) = CATALOGS.iter().find(|(its, _)| *its == tag)?;
    let mut source = String::new();
    flate2::read::DeflateDecoder::new(*compressed)
        .read_to_string(&mut source)
        .ok()?;
    Some(source)
}

/// A catalog from its source. One that does not parse whole, or names a message twice, is not used at all, so the
/// language falls back to English rather than showing part of a broken file.
fn build(language: &Language, source: String) -> Option<Catalog> {
    let resource = FluentResource::try_new(source).ok()?;
    let id: LanguageIdentifier = language.tag.parse().ok()?;
    let mut bundle = Bundle::new_concurrent(vec![id]);
    bundle.set_use_isolating(language.direction == Direction::RightToLeft);
    bundle.set_formatter(Some(format_value));
    bundle.add_resource(resource).ok()?;
    Some(Catalog { bundle })
}

#[cfg(test)]
mod tests;
