//! Checks a window's accessibility tree the way a screen reader and a keyboard user meet it, for the UI tests of the app,
//! the installer, and the uninstaller. Each returns what is wrong, as sentences, so that a test can show all of it.
//!
//! - [`interactive_problems`]: every control has a name, a role, and its state (AC-A11Y-NAMES).
//! - [`small_targets`]: nothing clickable is smaller than 24 by 24 points without that much room around it
//!   (WCAG 2.5.8, AC-A11Y-TARGET).
//! - [`tab_order`]: the controls the Tab key reaches, in order (AC-A11Y-KEYBOARD).
//! - [`unmarked`]: in a test language, the words that are not from the catalog (AC-I18N-NOHARDCODE).
//!
//! Test code only. The app's and the installer's tests include this file by path, each with its own dev-dependency on
//! egui_kittest, so that no program that ships depends on egui_kittest, even optionally.

use eframe::egui::accesskit::{Action, Role};
use eframe::egui::{self, Rect};
use egui_kittest::kittest::NodeT;
use egui_kittest::{Harness, Node};
use winappmigrator_i18n::{PSEUDO_END, PSEUDO_START, is_isolation_mark};

/// The least a target may measure across, in points, by WCAG 2.5.8: `theme::SMALLEST_TARGET`, which a test holds
/// them to.
pub const SMALLEST_TARGET: f32 = 24.0;

/// Roles a control has that a person works.
const CONTROL_ROLES: &[Role] = &[
    Role::Button,
    Role::CheckBox,
    Role::RadioButton,
    Role::ComboBox,
    Role::TextInput,
    Role::SearchInput,
    Role::Link,
    Role::Slider,
    Role::Tab,
    Role::MenuItem,
    Role::ListBoxOption,
    Role::Switch,
    Role::ScrollBar,
    Role::DisclosureTriangle,
];

/// Roles a screen reader names by themselves: a scroll bar, by what it scrolls.
const NAMED_BY_ROLE: &[Role] = &[Role::ScrollBar];

/// A node's name, as a screen reader would say it: its label, else its value.
pub fn name_of(node: &Node<'_>) -> String {
    let data = node.accesskit_node();
    data.label()
        .filter(|label| !label.trim().is_empty())
        .or_else(|| data.value())
        .unwrap_or_default()
}

/// Whether a person can act on the node: click it, or move the focus to it. Text is not, though egui lets its words
/// be selected and copied.
pub fn is_interactive(node: &Node<'_>) -> bool {
    let data = node.accesskit_node();
    let role = data.role();
    !data.is_hidden()
        && !matches!(role, Role::Label | Role::TextRun | Role::Heading)
        && (data.data().supports_action(Action::Click)
            || data.data().supports_action(Action::Focus)
            || CONTROL_ROLES.contains(&role))
}

/// Every node of the window, in tree order.
pub fn all_nodes<'tree>(root: &Node<'tree>) -> Vec<Node<'tree>> {
    root.children_recursive().collect()
}

/// AC-A11Y-NAMES: every control has a name and a role that says what it is, and a checkbox says whether it is ticked.
pub fn interactive_problems(root: &Node<'_>) -> Vec<String> {
    let mut problems = Vec::new();
    for node in all_nodes(root) {
        let data = node.accesskit_node();
        if data.role() == Role::Pane && data.bounding_box().is_none() {
            problems.push("a pane has no bounds".to_owned());
        }
    }
    for node in all_nodes(root).iter().filter(|node| is_interactive(node)) {
        let data = node.accesskit_node();
        let name = name_of(node);
        let role = data.role();
        if name.trim().is_empty() && !NAMED_BY_ROLE.contains(&role) {
            problems.push(format!("a {role:?} at {:?} has no name", node.rect()));
        }
        if !CONTROL_ROLES.contains(&role) {
            problems.push(format!("{name:?} is clickable but its role is {role:?}"));
        }
        if matches!(role, Role::CheckBox | Role::Switch) && data.toggled().is_none() {
            problems.push(format!("{name:?} does not say whether it is ticked"));
        }
    }
    problems
}

/// AC-A11Y-TARGET: clickable nodes smaller than 24 by 24 points, unless a 24-point circle on each touches no other
/// target (WCAG 2.5.8's spacing exception). A button inside a text field counts the field as a neighbour.
pub fn small_targets(root: &Node<'_>) -> Vec<String> {
    let targets: Vec<(String, Rect)> = all_nodes(root)
        .iter()
        .filter(|node| is_interactive(node))
        .map(|node| (name_of(node), node.rect()))
        .filter(|(_, rect)| rect.is_positive())
        .collect();
    let mut problems = Vec::new();
    for (index, (name, rect)) in targets.iter().enumerate() {
        if rect.width() >= SMALLEST_TARGET - 0.5 && rect.height() >= SMALLEST_TARGET - 0.5 {
            continue;
        }
        let circle = Rect::from_center_size(rect.center(), egui::Vec2::splat(SMALLEST_TARGET));
        let crowded = targets
            .iter()
            .enumerate()
            .any(|(other, (_, its))| other != index && its.intersects(circle));
        if crowded {
            problems.push(format!(
                "{name:?} is {:.0} by {:.0} points, with other targets within reach",
                rect.width(),
                rect.height()
            ));
        }
    }
    problems
}

/// AC-A11Y-KEYBOARD: the names of the controls the Tab key moves the focus to, in order, from the first until it comes
/// round again or `most` presses have been made.
pub fn tab_order<State>(harness: &mut Harness<'_, State>, most: usize) -> Vec<String> {
    focus_stops(harness, most)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

/// [`tab_order`], with where each control is when it has the focus.
pub fn focus_stops<State>(harness: &mut Harness<'_, State>, most: usize) -> Vec<(String, Rect)> {
    let mut stops: Vec<(String, Rect)> = Vec::new();
    for _ in 0..most {
        harness.key_press(egui::Key::Tab);
        harness.run();
        let focused = all_nodes(&harness.root())
            .into_iter()
            .find(|node| node.accesskit_node().is_focused())
            .map(|node| (name_of(&node), node.rect()));
        let Some(focused) = focused else {
            break;
        };
        if stops
            .iter()
            .any(|(name, rect)| *name == focused.0 && rect.min == focused.1.min)
        {
            break;
        }
        stops.push(focused);
    }
    stops
}

/// WCAG 2.4.11: the controls that, when the Tab key gives them the focus, cannot be seen at all in `window`.
pub fn hidden_when_focused<State>(
    harness: &mut Harness<'_, State>,
    window: Rect,
    most: usize,
) -> Vec<String> {
    focus_stops(harness, most)
        .into_iter()
        .filter(|(_, rect)| !window.intersects(*rect))
        .map(|(name, rect)| format!("{name:?} at {rect:?}"))
        .collect()
}

/// AC-A11Y-LIVE: the names of the nodes a screen reader says again when they change, and whether each is urgent.
pub fn live_regions(root: &Node<'_>) -> Vec<(String, bool)> {
    all_nodes(root)
        .iter()
        .filter_map(|node| match node.accesskit_node().live() {
            egui::accesskit::Live::Off => None,
            live => Some((name_of(node), live == egui::accesskit::Live::Assertive)),
        })
        .collect()
}

/// AC-I18N-NOHARDCODE: in a test language every message is marked, so any text left once each marked part and the
/// test's own `data` (names, paths, versions it put on screen) are taken out came from the code, not the catalog.
/// Returns each name or value that has such text, with the text.
pub fn unmarked(root: &Node<'_>, data: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();
    let mut data = data.to_vec();
    // The Language menu shows each language by its own name, which is not translated.
    data.push(winappmigrator_i18n::current().native_name);
    for node in all_nodes(root) {
        let accesskit = node.accesskit_node();
        // A run of a label's text, which the label has whole.
        if accesskit.role() == Role::TextRun {
            continue;
        }
        for text in [
            accesskit.label(),
            accesskit.value(),
            accesskit.description(),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(left) = unmarked_part(&text, &data) {
                problems.push(format!("{:?} {text:?} has {left:?}", accesskit.role()));
            }
        }
    }
    problems
}

/// The letters of `text` that are neither inside a test language's marks nor one of `data`, if any.
pub fn unmarked_part(text: &str, data: &[&str]) -> Option<String> {
    let mut left = String::new();
    let mut depth = 0_usize;
    for character in text.chars() {
        if character == PSEUDO_START {
            depth += 1;
        } else if character == PSEUDO_END {
            depth = depth.saturating_sub(1);
        } else if depth == 0 && !is_isolation_mark(character) {
            left.push(character);
        }
    }
    let mut longest_first: Vec<&str> = data.to_vec();
    longest_first.sort_by_key(|value| std::cmp::Reverse(value.len()));
    for value in longest_first.into_iter().filter(|value| !value.is_empty()) {
        left = left.replace(value, " ");
    }
    left.chars()
        .any(char::is_alphabetic)
        .then(|| left.split_whitespace().collect::<Vec<_>>().join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_words_outside_the_marks_and_the_data_count_as_unmarked() {
        assert_eq!(unmarked_part("«Ïñštåłł» Acme 1.2", &["Acme"]), None);
        assert_eq!(
            unmarked_part("«Ŝéàŕçĥ», 12 of 40", &[]),
            Some(", 12 of 40".to_owned())
        );
        assert_eq!(
            unmarked_part("Install «Ŵĥéŕé»: C:\\Apps", &["C:\\Apps"]),
            Some("Install :".to_owned())
        );
        assert_eq!(unmarked_part("«a «nested» b»", &[]), None);
        assert_eq!(unmarked_part("\u{2068}Acme\u{2069} «x»", &["Acme"]), None);
    }
}
