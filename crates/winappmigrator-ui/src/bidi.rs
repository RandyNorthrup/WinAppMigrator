//! Text that runs in more than one direction, as a right-to-left language writes it, and as an app's Arabic or Hebrew
//! name reads inside an English window.
//!
//! egui shapes each piece of text in the direction its script runs, but lays the pieces out left to right and has no
//! bidirectional algorithm. So a line is first broken in the order it is read, then the Unicode Bidirectional
//! Algorithm puts each line's runs in the order they are seen, and each run is given to egui as a piece of its own: its
//! shaper then joins and orders an Arabic run right to left, and a path inside it left to right. Screen readers are
//! given the text in the order it is read.

use std::sync::Arc;

use eframe::egui::{
    self, Color32, FontId, Galley, Painter, Response, Role, Sense, Ui, Vec2, WidgetInfo, pos2,
    text::LayoutJob, text::TextFormat, vec2,
};
use unicode_bidi::{BidiClass, BidiInfo, Level, bidi_class};
use winappmigrator_i18n::{Direction, is_isolation_mark, without_isolation};

/// A part of a line that runs one way, with its letters in the order they are written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub text: String,
    pub right_to_left: bool,
}

/// A line's runs, left to right as they are seen.
pub type VisualLine = Vec<Run>;

/// Whether a letter belongs to a right-to-left script, such as Arabic or Hebrew.
pub fn is_right_to_left_letter(c: char) -> bool {
    matches!(bidi_class(c), BidiClass::R | BidiClass::AL)
}

/// Whether text has to be laid out bidirectionally: in a right-to-left window always, and otherwise only when it holds
/// right-to-left letters, so English text keeps egui's own layout.
pub fn needs_bidi(text: &str, base: Direction) -> bool {
    base == Direction::RightToLeft || text.chars().any(is_right_to_left_letter)
}

/// `text` broken into lines no wider than `wrap_width`, at spaces, in the order it is read, with each line's runs in
/// the order they are seen. `width_of` measures a piece of text as it would be drawn. Each paragraph of `text` starts a
/// line of its own.
pub fn visual_lines(
    text: &str,
    base: Direction,
    wrap_width: Option<f32>,
    width_of: impl Fn(&str) -> f32,
) -> Vec<VisualLine> {
    let level = match base {
        Direction::RightToLeft => Level::rtl(),
        Direction::LeftToRight => Level::ltr(),
    };
    let info = BidiInfo::new(text, Some(level));
    let mut lines = Vec::new();
    for paragraph in &info.paragraphs {
        let content = text[paragraph.range.clone()].trim_end_matches(['\n', '\r']);
        let range = paragraph.range.start..paragraph.range.start + content.len();
        for line in break_lines(text, range, wrap_width, &width_of) {
            let (levels, runs) = info.visual_runs(paragraph, line);
            let visual: VisualLine = runs
                .into_iter()
                .filter_map(|run| {
                    let right_to_left = levels.get(run.start).is_some_and(Level::is_rtl);
                    let part = written_run(&text[run], right_to_left);
                    (!part.is_empty()).then_some(Run {
                        text: part,
                        right_to_left,
                    })
                })
                .collect();
            lines.push(visual);
        }
    }
    lines
}

/// A run as it is given to egui: without isolation marks, which draw nothing. egui's shaper reverses and mirrors a run
/// it can tell is right to left by its letters; one with no such letters, such as `(!)` between two Arabic words, it
/// would lay left to right, so it is reversed and mirrored here.
fn written_run(text: &str, right_to_left: bool) -> String {
    let kept = text.chars().filter(|c| !is_isolation_mark(*c));
    if right_to_left && !text.chars().any(is_right_to_left_letter) {
        kept.rev().map(mirrored).collect()
    } else {
        kept.collect()
    }
}

/// The mirror image of a paired mark, as a right-to-left line shows it.
fn mirrored(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        other => other,
    }
}

/// Byte ranges of `range` broken into lines at spaces, so that no line is wider than `wrap_width` unless one word is.
fn break_lines(
    text: &str,
    range: std::ops::Range<usize>,
    wrap_width: Option<f32>,
    width_of: &impl Fn(&str) -> f32,
) -> Vec<std::ops::Range<usize>> {
    let Some(limit) = wrap_width else {
        return vec![range];
    };
    let measure = |from: usize, to: usize| -> f32 {
        let piece: String = text[from..to]
            .chars()
            .filter(|c| !is_isolation_mark(*c))
            .collect();
        width_of(piece.trim_end())
    };
    let mut lines = Vec::new();
    let mut start = range.start;
    let mut last_break = None;
    for (offset, c) in text[range.clone()].char_indices() {
        let at = range.start + offset;
        if c != ' ' {
            continue;
        }
        let end = at + c.len_utf8();
        if measure(start, at) > limit
            && let Some(previous) = last_break
            && previous > start
        {
            lines.push(start..previous);
            start = previous;
        }
        last_break = Some(end);
    }
    if measure(start, range.end) > limit
        && let Some(previous) = last_break
        && previous > start
        && previous < range.end
    {
        lines.push(start..previous);
        start = previous;
    }
    lines.push(start..range.end);
    lines
}

/// One line as egui lays it out: each run a piece of its own, in the order seen.
pub fn line_job(line: &[Run], font: &FontId, color: Color32) -> LayoutJob {
    let mut job = LayoutJob::default();
    for run in line {
        job.append(&run.text, 0.0, TextFormat::simple(font.clone(), color));
    }
    job
}

/// Whether windows are laid out right to left, for the language in use.
pub fn right_to_left() -> bool {
    winappmigrator_i18n::direction() == Direction::RightToLeft
}

/// Text laid out to be drawn: one line, or lines broken to a width, each with its runs in the order they are seen.
/// Text that needs no bidirectional layout is laid out by egui alone, as it always was.
pub struct TextBlock {
    lines: Vec<Arc<Galley>>,
    size: Vec2,
}

impl TextBlock {
    pub fn new(
        painter: &Painter,
        text: &str,
        font: FontId,
        color: Color32,
        wrap_width: Option<f32>,
    ) -> Self {
        let base = winappmigrator_i18n::direction();
        let lines: Vec<Arc<Galley>> = if needs_bidi(text, base) {
            visual_lines(text, base, wrap_width, |piece| {
                painter
                    .layout_no_wrap(piece.to_owned(), font.clone(), color)
                    .size()
                    .x
            })
            .iter()
            .map(|line| painter.layout_job(line_job(line, &font, color)))
            .collect()
        } else {
            vec![match wrap_width {
                Some(width) => painter.layout(text.to_owned(), font, color, width),
                None => painter.layout_no_wrap(text.to_owned(), font, color),
            }]
        };
        let width = lines
            .iter()
            .map(|line| line.size().x)
            .fold(0.0_f32, f32::max);
        let height = lines.iter().map(|line| line.size().y).sum();
        Self {
            lines,
            size: vec2(width, height),
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// Draw it from `top` down, each line starting at `start`: its left edge in a left-to-right window, its right edge
    /// in a right-to-left one.
    pub fn paint(&self, painter: &Painter, start: f32, top: f32, color: Color32) {
        let from_the_right = right_to_left();
        let mut y = top;
        for line in &self.lines {
            let left = if from_the_right {
                start - line.size().x
            } else {
                start
            };
            painter.galley(pos2(left, y), Arc::clone(line), color);
            y += line.size().y;
        }
    }

    /// Draw it from `top` down, each line centered on `center`.
    pub fn paint_centered(&self, painter: &Painter, center: f32, top: f32, color: Color32) {
        let mut y = top;
        for line in &self.lines {
            painter.galley(
                pos2(center - line.size().x / 2.0, y),
                Arc::clone(line),
                color,
            );
            y += line.size().y;
        }
    }
}

/// One line of text as it is drawn, in the order it is seen. Text with line breaks keeps them.
pub fn line_galley(painter: &Painter, text: &str, font: FontId, color: Color32) -> Arc<Galley> {
    let base = winappmigrator_i18n::direction();
    if !needs_bidi(text, base) {
        return painter.layout_no_wrap(text.to_owned(), font, color);
    }
    let lines = visual_lines(text, base, None, |piece| {
        painter
            .layout_no_wrap(piece.to_owned(), font.clone(), color)
            .size()
            .x
    });
    let mut job = LayoutJob::default();
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            job.append("\n", 0.0, TextFormat::simple(font.clone(), color));
        }
        for run in line {
            job.append(&run.text, 0.0, TextFormat::simple(font.clone(), color));
        }
    }
    painter.layout_job(job)
}

/// What ends text that was cut short to fit.
const ELLIPSIS: char = '…';

/// One line of text, cut short with `…` where it is read to end, so as to be no wider than `max_width`.
pub fn elided_galley(
    painter: &Painter,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> Arc<Galley> {
    let max_width = max_width.max(0.0);
    if !needs_bidi(text, winappmigrator_i18n::direction()) {
        let mut job = LayoutJob::simple_singleline(text.to_owned(), font, color);
        job.wrap = egui::text::TextWrapping::truncate_at_width(max_width);
        return painter.layout_job(job);
    }
    let whole = line_galley(painter, text, font.clone(), color);
    if whole.size().x <= max_width {
        return whole;
    }
    let cut = |letters: usize| -> Arc<Galley> {
        let end = text
            .char_indices()
            .nth(letters)
            .map_or(text.len(), |(at, _)| at);
        let shortened = format!("{}{ELLIPSIS}", text[..end].trim_end());
        line_galley(painter, &shortened, font.clone(), color)
    };
    // The most letters that fit, found by halves: a longer piece is never narrower.
    let (mut fits, mut too_many) = (0, text.chars().count());
    while too_many - fits > 1 {
        let middle = usize::midpoint(fits, too_many);
        if cut(middle).size().x <= max_width {
            fits = middle;
        } else {
            too_many = middle;
        }
    }
    cut(fits)
}

/// Text as a label, laid out bidirectionally in the direction of the language in use: lines start at the right edge in
/// a right-to-left window. `wrap` breaks it to the width there is. Screen readers get the text in the order it is read.
pub fn label(ui: &mut Ui, text: &str, font: FontId, color: Color32, wrap: bool) -> Response {
    let wrap_width = wrap.then(|| ui.available_width());
    let block = TextBlock::new(ui.painter(), text, font, color, wrap_width);
    let width = wrap_width.unwrap_or(block.size().x).max(block.size().x);
    let (rect, response) = ui.allocate_exact_size(vec2(width, block.size().y), Sense::hover());
    if ui.layout().horizontal_align() == egui::Align::Center {
        block.paint_centered(ui.painter(), rect.center().x, rect.top(), color);
    } else {
        let start = if right_to_left() {
            rect.right()
        } else {
            rect.left()
        };
        block.paint(ui.painter(), start, rect.top(), color);
    }
    response.widget_info(|| WidgetInfo::labeled(Role::Label, true, without_isolation(text)));
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every letter one unit wide, so lines break by length.
    fn by_letters(piece: &str) -> f32 {
        piece.chars().count() as f32
    }

    fn texts(line: &VisualLine) -> Vec<(&str, bool)> {
        line.iter()
            .map(|run| (run.text.as_str(), run.right_to_left))
            .collect()
    }

    const PATH: &str = r"C:\Program Files\App";

    /// AC-RTL-BIDI: a path inside an Arabic sentence reads left to right, between the sentence's parts in the order
    /// a right-to-left reader sees them.
    #[test]
    fn a_path_inside_an_arabic_sentence_keeps_its_own_direction() {
        let text = format!("ثبّت في {PATH} الآن");
        let lines = visual_lines(&text, Direction::RightToLeft, None, by_letters);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            texts(&lines[0]),
            [(" الآن", true), (PATH, false), ("ثبّت في ", true)],
            "the last words read are seen leftmost"
        );
    }

    #[test]
    fn what_a_catalog_isolates_is_laid_out_the_same_and_its_marks_drawn_nowhere() {
        let text = format!("ثبّت في \u{2068}{PATH}\u{2069} الآن");
        let lines = visual_lines(&text, Direction::RightToLeft, None, by_letters);
        assert_eq!(
            texts(&lines[0]),
            [(" الآن", true), (PATH, false), ("ثبّت في ", true)]
        );
        assert!(
            lines[0]
                .iter()
                .all(|run| !run.text.chars().any(is_isolation_mark))
        );
    }

    #[test]
    fn hebrew_with_a_version_number_keeps_the_number_in_order() {
        let text = "גרסה 3.0.18 מותקנת";
        let lines = visual_lines(text, Direction::RightToLeft, None, by_letters);
        let seen = texts(&lines[0]);
        assert!(seen.contains(&("3.0.18", false)), "{seen:?}");
        assert_eq!(seen.first().map(|run| run.0), Some(" מותקנת"), "{seen:?}");
    }

    #[test]
    fn a_long_sentence_breaks_in_the_order_it_is_read() {
        let text = "واحد اثنان ثلاثة أربعة خمسة";
        let lines = visual_lines(text, Direction::RightToLeft, Some(12.0), by_letters);
        assert!(lines.len() > 1, "{lines:?}");
        let first: String = lines[0].iter().map(|run| run.text.as_str()).collect();
        assert!(
            first.contains("واحد"),
            "the first words read are on the first line: {first}"
        );
        let last: String = lines[lines.len() - 1]
            .iter()
            .map(|run| run.text.as_str())
            .collect();
        assert!(last.contains("خمسة"), "{last}");
    }

    #[test]
    fn an_arabic_name_in_an_english_window_reads_right_to_left_on_its_own() {
        let text = "Install مرحبا";
        assert!(needs_bidi(text, Direction::LeftToRight));
        assert!(!needs_bidi("Install App", Direction::LeftToRight));
        let lines = visual_lines(text, Direction::LeftToRight, None, by_letters);
        assert_eq!(texts(&lines[0]), [("Install ", false), ("مرحبا", true)]);
    }

    #[test]
    fn marks_between_right_to_left_words_are_turned_around_and_mirrored() {
        assert_eq!(written_run("!?", true), "?!");
        assert_eq!(written_run("(a", false), "(a");
        assert_eq!(written_run("[!", true), "!]");
        assert_eq!(
            written_run("مرحبا (", true),
            "مرحبا (",
            "the shaper turns a run of letters"
        );
    }

    #[test]
    fn each_paragraph_starts_a_line() {
        let lines = visual_lines("أ\nب", Direction::RightToLeft, None, by_letters);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn one_word_wider_than_the_line_stays_whole() {
        let lines = visual_lines(
            "abcdefghij kl",
            Direction::LeftToRight,
            Some(4.0),
            by_letters,
        );
        assert_eq!(texts(&lines[0]), [("abcdefghij ", false)]);
        assert_eq!(texts(&lines[1]), [("kl", false)]);
    }
}
