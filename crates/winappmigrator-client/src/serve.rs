//! Answering a paired builder: describe, list and read paths below the snapshot's root, never
//! following a link.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use winappmigrator_link::channel::SecureChannel;
use winappmigrator_link::paths::names_of;
use winappmigrator_link::protocol::{
    About, Entry, EntryKind, FailureKind, MAX_LISTED, MAX_READ, Request, Response, decode, encode,
};
use winappmigrator_link::{LinkError, Result};

/// What a request could not be answered with.
type Refusal = (FailureKind, String);

/// How much of a message a page of a listing may take, well inside what the link carries, since what
/// a name takes depends on its letters as much as on how many there are.
const MAX_LISTED_BYTES: usize = 512 * 1024;

/// The hives of `System32\config` that hold this PC's passwords and secrets. A build never needs them,
/// so no builder is given them, with each file Windows keeps beside one for its log or a backup.
const SECRET_HIVES: &[&str] = &["sam", "security"];

/// Answer a builder until it closes the link. Each folder found not to be a link is remembered for
/// the session, as a scan asks about many files in the same few folders.
pub fn serve<S: Read + Write>(
    channel: &mut SecureChannel<S>,
    root: &Path,
    about: &About,
) -> Result<u64> {
    let mut checked = HashSet::new();
    let mut answered = 0;
    loop {
        let message = match channel.receive() {
            Ok(message) => message,
            Err(LinkError::Closed) => return Ok(answered),
            // How a builder that closed, or was closed, ends a connection on Windows.
            Err(LinkError::Io(error))
                if matches!(
                    error.kind(),
                    io::ErrorKind::ConnectionReset | io::ErrorKind::ConnectionAborted
                ) =>
            {
                return Ok(answered);
            }
            Err(error) => return Err(error),
        };
        let (request, _) = decode::<Request>(&message)?;
        let (response, body) = answer(request, root, about, &mut checked);
        channel.send(&encode(&response, &body)?)?;
        answered += 1;
    }
}

/// The response to one request, and the bytes after it.
pub fn answer(
    request: Request,
    root: &Path,
    about: &About,
    checked: &mut HashSet<PathBuf>,
) -> (Response, Vec<u8>) {
    let result = match request {
        Request::About => Ok((Response::About(about.clone()), Vec::new())),
        Request::Describe { path } => describe(root, &path, checked),
        Request::List { path, from } => list(root, &path, from, checked),
        Request::Read {
            path,
            offset,
            length,
        } => read(root, &path, offset, length, checked),
    };
    result.unwrap_or_else(|(kind, message)| (Response::Failed { kind, message }, Vec::new()))
}

fn describe(
    root: &Path,
    path: &str,
    checked: &mut HashSet<PathBuf>,
) -> std::result::Result<(Response, Vec<u8>), Refusal> {
    let full = locate(root, path, checked)?;
    let metadata = fs::symlink_metadata(&full).map_err(|error| failure(&error))?;
    let name = path.rsplit('\\').next().unwrap_or_default().to_owned();
    Ok((
        Response::Described {
            entry: entry_of(name, &metadata),
        },
        Vec::new(),
    ))
}

fn list(
    root: &Path,
    path: &str,
    from: usize,
    checked: &mut HashSet<PathBuf>,
) -> std::result::Result<(Response, Vec<u8>), Refusal> {
    let full = locate(root, path, checked)?;
    not_a_link(&full)?;
    let mut entries: Vec<Entry> = Vec::new();
    for item in fs::read_dir(&full).map_err(|error| failure(&error))? {
        let Ok(item) = item else {
            continue;
        };
        // A name that is not text cannot be asked about by name, and the builder leaves it out.
        let Ok(name) = item.file_name().into_string() else {
            continue;
        };
        // On Windows this describes the entry itself, without following a link.
        if let Ok(metadata) = item.metadata() {
            entries.push(entry_of(name, &metadata));
        }
    }
    entries.sort_by_cached_key(|entry| entry.name.to_lowercase());
    let total = entries.len();
    let mut page = Vec::new();
    let mut bytes = 0;
    for entry in entries.into_iter().skip(from).take(MAX_LISTED) {
        let size = encode(&entry, &[]).map_or(MAX_LISTED_BYTES, |message| message.len());
        // At least one, so that a listing always gets somewhere.
        if !page.is_empty() && bytes + size > MAX_LISTED_BYTES {
            break;
        }
        bytes += size;
        page.push(entry);
    }
    let more = total > from.saturating_add(page.len());
    Ok((
        Response::Listed {
            entries: page,
            more,
        },
        Vec::new(),
    ))
}

fn read(
    root: &Path,
    path: &str,
    offset: u64,
    length: u32,
    checked: &mut HashSet<PathBuf>,
) -> std::result::Result<(Response, Vec<u8>), Refusal> {
    let full = locate(root, path, checked)?;
    not_a_link(&full)?;
    let mut file = File::open(&full).map_err(|error| failure(&error))?;
    file.seek(SeekFrom::Start(offset))
        .map_err(|error| failure(&error))?;
    let wanted = length.min(MAX_READ) as usize;
    let mut bytes = vec![0_u8; wanted];
    let mut filled = 0;
    while filled < wanted {
        match file.read(&mut bytes[filled..]) {
            Ok(0) => break,
            Ok(count) => filled += count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(failure(&error)),
        }
    }
    bytes.truncate(filled);
    Ok((Response::Read, bytes))
}

/// Where a path asked about is, if it is one to ask about and nothing on the way to it from the
/// root is a link. The path itself may be one, which describing it says.
fn locate(
    root: &Path,
    path: &str,
    checked: &mut HashSet<PathBuf>,
) -> std::result::Result<PathBuf, Refusal> {
    let names = names_of(path).ok_or_else(|| {
        (
            FailureKind::Refused,
            "that is not a path below the drive's root in ordinary names".to_owned(),
        )
    })?;
    if is_secret_hive(&names) {
        return Err((
            FailureKind::Refused,
            "that holds this PC's passwords, which no build needs".to_owned(),
        ));
    }
    let mut full = root.to_path_buf();
    let last = names.len().saturating_sub(1);
    for (index, name) in names.iter().enumerate() {
        full.push(name);
        if index == last || checked.contains(&full) {
            continue;
        }
        match fs::symlink_metadata(&full) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err((
                    FailureKind::Refused,
                    "the way there goes through a link".to_owned(),
                ));
            }
            Ok(_) => {
                checked.insert(full.clone());
            }
            Err(error) => return Err(failure(&error)),
        }
    }
    Ok(full)
}

/// Whether a path is one of [`SECRET_HIVES`] in Windows' `System32\config`, or below it, as its log, a
/// backup in `RegBack`, or anything else named for it. The first name is the Windows folder, whatever it
/// is called.
fn is_secret_hive(names: &[&str]) -> bool {
    names.len() > 3
        && names[1].eq_ignore_ascii_case("System32")
        && names[2].eq_ignore_ascii_case("config")
        && names[3..].iter().any(|name| {
            let stem = name
                .split('.')
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            SECRET_HIVES.contains(&stem.as_str())
        })
}

fn not_a_link(full: &Path) -> std::result::Result<(), Refusal> {
    match fs::symlink_metadata(full) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err((
            FailureKind::Refused,
            "that is a link, which is never followed".to_owned(),
        )),
        Ok(_) => Ok(()),
        Err(error) => Err(failure(&error)),
    }
}

fn entry_of(name: String, metadata: &fs::Metadata) -> Entry {
    let kind = metadata.file_type();
    Entry {
        name,
        kind: if kind.is_symlink() {
            EntryKind::Link
        } else if kind.is_dir() {
            EntryKind::Directory
        } else if kind.is_file() {
            EntryKind::File
        } else {
            EntryKind::Other
        },
        len: metadata.len(),
        attributes: metadata.file_attributes(),
        modified: metadata.last_write_time(),
        created: metadata.creation_time(),
        accessed: metadata.last_access_time(),
    }
}

fn failure(error: &io::Error) -> Refusal {
    let kind = match error.kind() {
        io::ErrorKind::NotFound => FailureKind::NotFound,
        io::ErrorKind::PermissionDenied => FailureKind::Denied,
        _ => FailureKind::Other,
    };
    (kind, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn about() -> About {
        About {
            client_version: crate::VERSION.to_owned(),
            computer_name: "OLD-PC".to_owned(),
            windows_name: "Windows 7".to_owned(),
            architecture: "x86".to_owned(),
            snapshot_taken: String::new(),
        }
    }

    fn root() -> Option<tempfile_free::Root> {
        tempfile_free::Root::new()
    }

    /// A folder of its own under the system's temp folder, removed afterward.
    mod tempfile_free {
        use std::path::PathBuf;

        pub struct Root(pub PathBuf);

        impl Root {
            pub fn new() -> Option<Self> {
                let path = std::env::temp_dir().join(format!(
                    "wam-client-test-{}-{:?}",
                    std::process::id(),
                    std::thread::current().id()
                ));
                let _ = std::fs::remove_dir_all(&path);
                std::fs::create_dir_all(path.join(r"Program Files\Acme")).ok()?;
                std::fs::write(path.join(r"Program Files\Acme\app.exe"), b"MZ0123456789").ok()?;
                Some(Self(path))
            }
        }

        impl Drop for Root {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }

    #[test]
    fn a_file_is_described_listed_and_read_in_part() {
        let Some(root) = root() else {
            return;
        };
        let mut checked = HashSet::new();
        let (described, _) = answer(
            Request::Describe {
                path: r"Program Files\Acme\app.exe".to_owned(),
            },
            &root.0,
            &about(),
            &mut checked,
        );
        assert!(
            matches!(&described, Response::Described { entry } if entry.kind == EntryKind::File && entry.len == 12),
            "{described:?}"
        );
        let (listed, _) = answer(
            Request::List {
                path: r"Program Files\Acme".to_owned(),
                from: 0,
            },
            &root.0,
            &about(),
            &mut checked,
        );
        assert!(
            matches!(&listed, Response::Listed { entries, more: false } if entries.len() == 1 && entries[0].name == "app.exe"),
            "{listed:?}"
        );
        let (read, body) = answer(
            Request::Read {
                path: r"Program Files\Acme\app.exe".to_owned(),
                offset: 2,
                length: 4,
            },
            &root.0,
            &about(),
            &mut checked,
        );
        assert_eq!((read, body.as_slice()), (Response::Read, &b"0123"[..]));
        let (past_the_end, body) = answer(
            Request::Read {
                path: r"Program Files\Acme\app.exe".to_owned(),
                offset: 10,
                length: 100,
            },
            &root.0,
            &about(),
            &mut checked,
        );
        assert_eq!(
            (past_the_end, body.as_slice()),
            (Response::Read, &b"89"[..])
        );
    }

    #[test]
    fn nothing_outside_the_root_or_through_a_link_is_answered() {
        let Some(root) = root() else {
            return;
        };
        let mut checked = HashSet::new();
        for path in [
            r"..\x",
            r"C:\Windows",
            r"Program Files\..\..\x",
            r"Program Files\Acme\app.exe:s",
        ] {
            let (response, _) = answer(
                Request::Read {
                    path: path.to_owned(),
                    offset: 0,
                    length: 10,
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(
                matches!(
                    response,
                    Response::Failed {
                        kind: FailureKind::Refused,
                        ..
                    }
                ),
                "{path}: {response:?}"
            );
        }
        // A junction to somewhere else, made the way any account can make one in its own folder.
        let junction = root.0.join("Elsewhere");
        let made = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/j"])
            .arg(&junction)
            .arg(std::env::temp_dir())
            .output()
            .is_ok_and(|output| output.status.success());
        if made {
            let (through, _) = answer(
                Request::List {
                    path: r"Elsewhere".to_owned(),
                    from: 0,
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(matches!(
                through,
                Response::Failed {
                    kind: FailureKind::Refused,
                    ..
                }
            ));
            let (described, _) = answer(
                Request::Describe {
                    path: "Elsewhere".to_owned(),
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(
                matches!(&described, Response::Described { entry } if entry.kind == EntryKind::Link),
                "{described:?}"
            );
            let (below, _) = answer(
                Request::Describe {
                    path: r"Elsewhere\x".to_owned(),
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(matches!(
                below,
                Response::Failed {
                    kind: FailureKind::Refused,
                    ..
                }
            ));
            let _ = std::fs::remove_dir(&junction);
        }
    }

    #[test]
    fn the_hives_that_hold_passwords_are_never_given() {
        let Some(root) = root() else {
            return;
        };
        let mut checked = HashSet::new();
        let refused = |response: &Response| {
            matches!(
                response,
                Response::Failed {
                    kind: FailureKind::Refused,
                    ..
                }
            )
        };
        for path in [
            r"Windows\System32\config\SAM",
            r"WINDOWS\system32\CONFIG\sam.LOG1",
            r"Windows\System32\config\SECURITY",
            r"Windows\System32\config\RegBack\SECURITY",
            r"WINNT\System32\config\SAM.sav",
        ] {
            let (response, _) = answer(
                Request::Read {
                    path: path.to_owned(),
                    offset: 0,
                    length: 10,
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(refused(&response), "{path}: {response:?}");
        }
        // What a build reads, and an app's own file that happens to have the name, are asked about.
        for path in [
            r"Windows\System32\config\SOFTWARE",
            r"Windows\System32\config\SYSTEM",
            r"Program Files\Acme\config\SAM.dat",
        ] {
            let (response, _) = answer(
                Request::Describe {
                    path: path.to_owned(),
                },
                &root.0,
                &about(),
                &mut checked,
            );
            assert!(!refused(&response), "{path}: {response:?}");
        }
    }

    #[test]
    fn a_page_of_long_names_still_fits_in_a_message() {
        let Some(root) = root() else {
            return;
        };
        let folder = root.0.join("Long");
        if std::fs::create_dir(&folder).is_err() {
            return;
        }
        // Each name takes three bytes a letter, so a page of as many as a page can list would not fit.
        const FILES: usize = 900;
        let long = "字".repeat(200);
        for index in 0..FILES {
            if std::fs::write(folder.join(format!("{index:04}{long}")), b"").is_err() {
                return;
            }
        }
        let mut checked = HashSet::new();
        let mut names = Vec::new();
        let mut pages = 0;
        loop {
            let (response, body) = answer(
                Request::List {
                    path: "Long".to_owned(),
                    from: names.len(),
                },
                &root.0,
                &about(),
                &mut checked,
            );
            let size = encode(&response, &body).map_or(usize::MAX, |message| message.len());
            assert!(
                size <= winappmigrator_link::channel::MAX_MESSAGE,
                "a page of {size} bytes"
            );
            let Response::Listed { entries, more } = response else {
                panic!("{response:?}");
            };
            pages += 1;
            names.extend(entries.into_iter().map(|entry| entry.name));
            if !more {
                break;
            }
        }
        assert!(pages > 1, "it took more than one page");
        assert_eq!(names.len(), FILES);
        assert!(
            names.windows(2).all(|pair| pair[0] < pair[1]),
            "in order, once each"
        );
    }
}
