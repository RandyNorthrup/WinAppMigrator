//! Where the app and the installer write down what they did.
//!
//! Logs go to `<state root>\logs`, which on a real PC is `%ProgramData%\WinAppMigrator\logs`:
//! a fixed folder only administrators can change, a new file for each run, and a limit on how
//! many are kept and how large each gets. Nothing is opened until something is about to
//! change, so a preview leaves no trace.
//!
//! **What goes in a log is paths and key names.** What a registry value, a license file, or an
//! environment variable holds never does, and neither does a journal's content, so code
//! writing to this must only pass those names. `tests/log_has_no_secrets.rs` and the elevated
//! round trip both check that.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use time::OffsetDateTime;
use tracing_subscriber::fmt::MakeWriter;

use crate::journal::machine_state_root;
use crate::privilege::{ensure_protected_folder, is_elevated};

const LOGS_FOLDER: &str = "logs";
const LOG_EXTENSION: &str = "log";
/// Enough for a large app's every file, and a limit on what can fill a system drive.
const MAX_LOG_BYTES: u64 = 16 * 1024 * 1024;
/// How many logs are kept, oldest going first.
const MAX_LOG_FILES: usize = 20;
const LOG_TRUNCATED_NOTE: &[u8] = b"-- the log reached its size limit, and stops here --\n";

/// A file being logged to, which stops growing at its limit.
struct BoundedFile {
    file: File,
    written: u64,
    full: bool,
}

impl BoundedFile {
    fn write_line(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.full {
            return Ok(());
        }
        if self.written + bytes.len() as u64 > MAX_LOG_BYTES {
            self.full = true;
            return self.file.write_all(LOG_TRUNCATED_NOTE);
        }
        self.file.write_all(bytes)?;
        self.written += bytes.len() as u64;
        Ok(())
    }
}

#[derive(Default)]
struct Destinations {
    files: Vec<BoundedFile>,
    main_file: Option<PathBuf>,
    /// For a test to read back.
    memory: Option<Arc<Mutex<Vec<u8>>>>,
}

/// Where log lines go. Starts with nowhere, so what is logged before something opens a file
/// is dropped.
#[derive(Clone, Default)]
pub struct LogSink {
    destinations: Arc<Mutex<Destinations>>,
}

impl LogSink {
    /// Start a new file in the logs folder of `state_root`, and forget the oldest. A failure
    /// means there is nowhere safe to log to, and nothing will be written.
    pub fn open_in(&self, state_root: &Path, component: &str) -> io::Result<PathBuf> {
        for folder in [state_root, &state_root.join(LOGS_FOLDER)] {
            ensure_protected_folder(folder)?;
        }
        let folder = state_root.join(LOGS_FOLDER);
        forget_oldest_logs(&folder, MAX_LOG_FILES.saturating_sub(1));
        let path = folder.join(log_file_name(component, std::process::id()));
        let file = new_log_file(&path)?;
        if let Ok(mut destinations) = self.destinations.lock() {
            destinations.files.push(BoundedFile {
                file,
                written: 0,
                full: false,
            });
            destinations.main_file = Some(path.clone());
        }
        Ok(path)
    }

    /// Also log to a file someone asked for by name. It has to be new, which keeps an elevated
    /// process from following a link planted at that name or writing over anything.
    pub fn also_to(&self, path: &Path) -> io::Result<()> {
        let file = new_log_file(path)?;
        if let Ok(mut destinations) = self.destinations.lock() {
            destinations.files.push(BoundedFile {
                file,
                written: 0,
                full: false,
            });
        }
        Ok(())
    }

    /// The file in the logs folder being written to, for saying where to look.
    pub fn location(&self) -> Option<PathBuf> {
        self.destinations
            .lock()
            .ok()
            .and_then(|destinations| destinations.main_file.clone())
    }

    /// Send lines somewhere a test can read them.
    pub fn capture(&self) -> Arc<Mutex<Vec<u8>>> {
        let memory = Arc::new(Mutex::new(Vec::new()));
        if let Ok(mut destinations) = self.destinations.lock() {
            destinations.memory = Some(Arc::clone(&memory));
        }
        memory
    }
}

impl Write for LogSink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if let Ok(mut destinations) = self.destinations.lock() {
            for file in &mut destinations.files {
                // Losing a log line is not worth stopping what it was about.
                let _ = file.write_line(bytes);
            }
            if let Some(memory) = destinations.memory.as_ref()
                && let Ok(mut memory) = memory.lock()
            {
                memory.extend_from_slice(bytes);
            }
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for LogSink {
    type Writer = LogSink;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn new_log_file(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

/// When a run started and which process it is, so names sort by time and never collide.
fn log_file_name(component: &str, process_id: u32) -> String {
    let now = OffsetDateTime::now_utc();
    let stamp = format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    format!("{component}-{stamp}-{process_id}.{LOG_EXTENSION}")
}

/// Keep at most `keep` of the logs here, removing the oldest. Only ordinary files with the
/// right extension are looked at. A link is neither followed nor counted.
fn forget_oldest_logs(folder: &Path, keep: usize) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    let mut logs: Vec<(SystemTime, PathBuf)> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).ok()?;
            let is_log = metadata.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case(LOG_EXTENSION));
            is_log.then(|| (metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH), path))
        })
        .collect();
    logs.sort_by_key(|log| std::cmp::Reverse(log.0));
    for (_, path) in logs.into_iter().skip(keep) {
        let _ = fs::remove_file(path);
    }
}

static SINK: OnceLock<LogSink> = OnceLock::new();
static COMPONENT: OnceLock<String> = OnceLock::new();

/// Route `tracing` to the sink for the rest of this process. Nothing is written until
/// [`open_machine_log`] or [`also_log_to`] gives it somewhere to go. `component` is what the
/// logs of this program are named for. Calling this again changes nothing.
pub fn install(component: &str) {
    let _ = COMPONENT.set(component.to_owned());
    let sink = SINK.get_or_init(LogSink::default).clone();
    // Fails if a subscriber is already there, which only a test could have done.
    let _ = tracing_subscriber::fmt()
        .with_writer(sink)
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .try_init();
}

/// Start logging to the fixed folder. An administrator's process only: anyone else making the
/// folder would leave it where it could not be trusted. `None` means there will be no log,
/// which is not worth stopping anything for.
pub fn open_machine_log() -> Option<PathBuf> {
    let sink = SINK.get()?;
    if let Some(open) = sink.location() {
        return Some(open);
    }
    if !is_elevated() {
        return None;
    }
    let state_root = machine_state_root()?;
    let component = COMPONENT.get().map_or("winappmigrator", String::as_str);
    sink.open_in(&state_root, component).ok()
}

/// Also log to a file named on the command line.
pub fn also_log_to(path: &Path) -> io::Result<()> {
    match SINK.get() {
        Some(sink) => sink.also_to(path),
        None => Err(io::Error::other("logging has not been set up")),
    }
}

/// The log file this run is writing to, if it has one.
pub fn current_log_file() -> Option<PathBuf> {
    SINK.get().and_then(LogSink::location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_log_goes_in_its_folder_under_a_new_name_and_a_second_is_kept_apart() -> io::Result<()> {
        let work = tempfile::tempdir()?;
        let state = work.path().join("state");
        let sink = LogSink::default();
        assert_eq!(sink.location(), None);

        let path = sink.open_in(&state, "installer")?;

        assert_eq!(path.parent(), Some(state.join(LOGS_FOLDER).as_path()));
        assert_eq!(sink.location(), Some(path.clone()));
        let mut writer = sink.clone();
        writer.write_all(b"a line\n")?;
        assert_eq!(fs::read(&path)?, b"a line\n");
        Ok(())
    }

    #[test]
    fn a_log_stops_growing_at_its_limit() -> io::Result<()> {
        let work = tempfile::tempdir()?;
        let path = work.path().join("big.log");
        let mut log = BoundedFile {
            file: new_log_file(&path)?,
            written: MAX_LOG_BYTES - 4,
            full: false,
        };
        log.write_line(b"ok\n")?;
        log.write_line(b"too much\n")?;
        log.write_line(b"and more\n")?;
        let written = fs::read(&path)?;
        assert!(written.starts_with(b"ok\n"));
        assert!(written.ends_with(LOG_TRUNCATED_NOTE));
        assert!(!written.windows(4).any(|window| window == b"more"));
        Ok(())
    }

    #[test]
    fn only_so_many_logs_are_kept_and_nothing_else_is_touched() -> io::Result<()> {
        let work = tempfile::tempdir()?;
        for index in 0..5 {
            let path = work.path().join(format!("app-{index}.log"));
            fs::write(&path, b"x")?;
            let age = filetime::FileTime::from_unix_time(1_000_000 + index, 0);
            filetime::set_file_mtime(&path, age)?;
        }
        fs::write(work.path().join("notes.txt"), b"keep")?;

        forget_oldest_logs(work.path(), 2);

        let mut left: Vec<String> = fs::read_dir(work.path())?
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        assert_eq!(left, ["app-3.log", "app-4.log", "notes.txt"]);
        Ok(())
    }

    /// Run for real against the rename a planted link depends on.
    #[test]
    fn a_log_named_by_hand_must_be_new_so_a_planted_file_or_link_is_not_followed() -> io::Result<()>
    {
        let work = tempfile::tempdir()?;
        let victim = work.path().join("victim.txt");
        fs::write(&victim, b"keep")?;
        let sink = LogSink::default();

        assert!(sink.also_to(&victim).is_err());

        let hard_link = work.path().join("planted.log");
        fs::hard_link(&victim, &hard_link)?;
        assert!(sink.also_to(&hard_link).is_err());
        assert_eq!(fs::read(&victim)?, b"keep");

        let wanted = work.path().join("wanted.log");
        sink.also_to(&wanted)?;
        let mut writer = sink.clone();
        writer.write_all(b"written\n")?;
        assert_eq!(fs::read(&wanted)?, b"written\n");
        Ok(())
    }

    #[test]
    fn what_is_logged_before_anywhere_is_opened_is_dropped() -> io::Result<()> {
        let mut sink = LogSink::default();
        sink.write_all(b"nowhere to go\n")?;
        assert_eq!(sink.location(), None);
        Ok(())
    }
}
