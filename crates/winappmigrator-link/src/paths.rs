//! Which paths the builder may ask a client about: below the root of its snapshot, in ordinary names,
//! and nothing else. A client checks every path before it touches anything, and the builder checks
//! before it asks.

/// Names Windows opens as a device, whatever follows a dot after them.
const DEVICE_NAMES: &[&str] = &[
    "con", "prn", "aux", "nul", "conin$", "conout$", "com1", "com2", "com3", "com4", "com5",
    "com6", "com7", "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8",
    "lpt9",
];
/// The longest a name on NTFS can be.
const MAX_NAME_CHARS: usize = 255;
/// More than any real path, and short enough that nothing can be made to work through a huge one.
const MAX_PATH_CHARS: usize = 32 * 1024;

/// The names of a path below the snapshot's root, as `Windows\System32` is `Windows` and `System32`,
/// or `None` if it is not one to ask about. The root itself is the empty path.
pub fn names_of(path: &str) -> Option<Vec<&str>> {
    if path.chars().count() > MAX_PATH_CHARS {
        return None;
    }
    if path.is_empty() {
        return Some(Vec::new());
    }
    path.split('\\')
        .map(|name| is_ordinary_name(name).then_some(name))
        .collect()
}

/// A name that is a file's or a folder's, and nothing Windows would read as something else: not
/// `.`, `..`, a stream, a device, a second separator, or what Windows drops from the end of a name.
pub fn is_ordinary_name(name: &str) -> bool {
    if name.is_empty()
        || name.chars().count() > MAX_NAME_CHARS
        || name.ends_with(['.', ' '])
        || name.chars().any(|character| {
            matches!(
                character,
                ':' | '/' | '\\' | '*' | '?' | '"' | '<' | '>' | '|'
            ) || character.is_control()
        })
    {
        return false;
    }
    let stem = name
        .split('.')
        .next()
        .unwrap_or(name)
        .trim_end()
        .to_ascii_lowercase();
    !DEVICE_NAMES.contains(&stem.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_below_the_root_in_ordinary_names_is_asked_about() {
        assert_eq!(names_of(""), Some(vec![]));
        assert_eq!(
            names_of(r"Program Files (x86)\Acme\app.exe"),
            Some(vec!["Program Files (x86)", "Acme", "app.exe"])
        );
        assert_eq!(names_of(".hidden"), Some(vec![".hidden"]));
    }

    #[test]
    fn nothing_that_leaves_the_root_or_reads_as_something_else_is() {
        for path in [
            r"..\x",
            r"a\..\b",
            r"a\.\b",
            r"\Windows",
            r"Windows\",
            r"C:\Windows",
            r"C:x",
            r"a\\b",
            "a/b",
            r"a\b:stream",
            r"a\NUL",
            r"a\nul.txt",
            r"a\COM1.log",
            r"a\b.",
            r"a\b ",
            "a\u{0}b",
            r"a\*",
        ] {
            assert_eq!(names_of(path), None, "{path}");
        }
        assert_eq!(names_of(&"a\\".repeat(20_000)), None);
        assert!(!is_ordinary_name(&"x".repeat(256)));
    }
}
