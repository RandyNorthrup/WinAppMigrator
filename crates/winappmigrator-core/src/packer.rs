//! Puts files in a package: each read once, hashed from the bytes that were read, and compressed
//! from those same bytes, on as many threads as make sense and in a fixed order.
//!
//! A worker takes the next file, reads it into memory, hashes it, and compresses it. What it hands
//! over is whole, and the one thread writing takes them in order, so a hash cannot describe anything
//! but what goes in the package. A file too big to hold is read by the thread writing, when its turn
//! comes, one chunk at a time. Each chunk is hashed as it is read and then compressed by whichever
//! worker is free, and the results go out in order, as one deflate stream, with no copy of it on disk
//! to need room for. A file that cannot be read to its end is taken back out of the package.

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::os::windows::fs::{FileExt, OpenOptionsExt};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::Duration;

use flate2::{Compress, Compression, FlushCompress, Status};
use sha2::{Digest, Sha256};

use crate::errors::{Result, WinAppMigratorError, panic_text};
use crate::hashing::hex_lower;
use crate::package::copy_file_backup_mode_unless;
use crate::progress::{BuildMonitor, BuildStep};
use crate::zip_stream::{FinishedZip, OpenEntry, WholeEntry, ZipMethod, ZipStream};

/// What is read at a time, and what one worker holds. A file of this size or less is held whole.
const CHUNK_BYTES: usize = 4 * 1024 * 1024;
/// How much finished work can wait for its turn to be written. A worker takes no new file past
/// this, and the file whose turn it is has always been taken already, so this cannot hold up.
const MOST_WAITING_BYTES: u64 = 256 * 1024 * 1024;
/// Past this, more threads only fight over the disk and the bus.
const MOST_WORKERS: usize = 16;
/// How much of a file to try compressing before deciding whether to bother.
const SAMPLE_BYTES: usize = 64 * 1024;
/// How many places in a file too big to hold to take a sample from.
const LARGE_FILE_SAMPLES: u64 = 3;
/// What zlib's own default is, and what the measurements in `CHANGELOG.md` came out best for: 9
/// only took 1 to 2 percent off and 1 added 10 to 25.
const COMPRESSION_LEVEL: u32 = 6;
/// A sample that keeps this much of itself is not worth compressing, whatever its name says. It
/// takes the same level as the real thing to tell: PCM audio will not come down at all at level 1,
/// and comes down by a fifth at level 6.
const WILL_NOT_COMPRESS: f64 = 0.99;
/// A sample that comes down to this much is worth compressing, whatever its name says.
const WILL_COMPRESS: f64 = 0.90;
/// A deflate stream's end: an empty last block, after chunks that were each flushed to a byte.
const DEFLATE_END: [u8; 2] = [0x03, 0x00];
/// How often a thread waiting for a worker looks at whether the build was cancelled.
const CANCEL_LOOK: Duration = Duration::from_millis(100);
/// Opens whatever has the name, rather than what a link with that name points at.
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

/// Named for what they hold, by people who compress things. A sample can still say otherwise.
const COMPRESSED_EXTENSIONS: &[&str] = &[
    "7z", "aac", "apk", "appx", "avif", "br", "bz2", "cab", "crx", "deb", "docx", "flac", "gif",
    "gz", "heic", "heif", "jar", "jpeg", "jpg", "lz", "lz4", "lzma", "m4a", "m4v", "mkv", "mov",
    "mp3", "mp4", "mpeg", "mpg", "msix", "msu", "nupkg", "oga", "ogg", "ogv", "opus", "png",
    "pptx", "rar", "rpm", "tbz2", "tgz", "txz", "vsix", "war", "webm", "webp", "whl", "wma", "wmv",
    "xlsx", "xz", "zip", "zst",
];

/// Whether to compress a file, from its name and from how far a sample of it came down. Neither is
/// taken at its word: a `.png` of plain text is worth compressing, and a `.dat` of compressed data
/// is not. Only what is in between goes by its name.
pub(crate) fn store_or_deflate(name: &str, sample_kept: Option<f64>) -> ZipMethod {
    let Some(kept) = sample_kept else {
        return ZipMethod::Stored;
    };
    if kept >= WILL_NOT_COMPRESS {
        return ZipMethod::Stored;
    }
    if kept <= WILL_COMPRESS {
        return ZipMethod::Deflated;
    }
    if has_compressed_extension(name) {
        ZipMethod::Stored
    } else {
        ZipMethod::Deflated
    }
}

fn has_compressed_extension(name: &str) -> bool {
    name.rsplit(['/', '\\'])
        .next()
        .and_then(|leaf| leaf.rsplit_once('.'))
        .is_some_and(|(_, extension)| {
            COMPRESSED_EXTENSIONS
                .iter()
                .any(|known| extension.eq_ignore_ascii_case(known))
        })
}

/// One file to put in the package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PackItem {
    /// What it is called in the zip.
    pub name: String,
    /// Where to read it.
    pub source: PathBuf,
    /// How big a scan saw it to be, for how to handle it and for progress. It may have changed.
    pub size_hint: u64,
    /// What it has to hash to, for what is already held in the protected folder.
    pub expected_sha256: Option<String>,
}

/// What was read of a file that went in the package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PackedFile {
    pub sha256: String,
    pub size: u64,
}

/// A file that went in, or why it did not.
pub(crate) type PackOutcome = std::result::Result<PackedFile, String>;

/// What a build asks about each file around reading it. Worked out by the caller, and asked by
/// any thread.
pub(crate) struct SourceRules<'a> {
    pub before: &'a (dyn Fn(&Path) -> Result<()> + Sync),
    pub after: &'a (dyn Fn(&Path) -> Result<()> + Sync),
    /// For a drive that may be failing: one read at a time, and no link is opened at all.
    pub connected_drive: bool,
    /// Where a file only a backup-mode copy can read goes for as long as it is being read. Files
    /// from a drive can have permissions that name no one on this PC.
    pub spool: Option<&'a Path>,
}

/// A worker's finished file.
enum Finished {
    Whole {
        method: ZipMethod,
        crc32: u32,
        sha256: String,
        size: u64,
        data: Vec<u8>,
    },
    /// Too big to hold: the thread writing reads it when its turn comes.
    TooBig,
    Failed(String),
}

impl Finished {
    fn held_bytes(&self) -> u64 {
        match self {
            Self::Whole { data, .. } => data.len() as u64,
            Self::TooBig | Self::Failed(_) => 0,
        }
    }
}

struct ChunkJob {
    sequence: u64,
    data: Vec<u8>,
}

#[derive(Default)]
struct Queue {
    next_file: usize,
    finished: BTreeMap<usize, Finished>,
    waiting_bytes: u64,
    chunk_jobs: VecDeque<ChunkJob>,
    chunk_results: BTreeMap<u64, io::Result<Vec<u8>>>,
    stopping: bool,
    worker_died: bool,
}

struct Shared<'a> {
    items: &'a [PackItem],
    order: &'a [usize],
    rules: &'a SourceRules<'a>,
    queue: Mutex<Queue>,
    /// Workers wait here for something to do.
    has_work: Condvar,
    /// The thread writing waits here for something to write.
    has_result: Condvar,
    /// One read at a time, for a drive that may be failing.
    read_gate: Mutex<()>,
    next_spool: AtomicU64,
    is_cancelled: &'a (dyn Fn() -> bool + Sync),
}

impl Shared<'_> {
    fn lock(&self) -> MutexGuard<'_, Queue> {
        // A worker that panicked has no work worth keeping, and the build stops when its turn comes.
        self.queue
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn stop(&self) {
        self.lock().stopping = true;
        self.has_work.notify_all();
        self.has_result.notify_all();
    }
}

/// How many threads to use.
pub(crate) fn worker_count() -> usize {
    std::thread::available_parallelism()
        .map_or(1, std::num::NonZero::get)
        .min(MOST_WORKERS)
}

/// A package on its way out.
pub(crate) struct Packer {
    stream: ZipStream,
    monitor: BuildMonitor,
    names: HashSet<String>,
    files_done: usize,
    files_total: usize,
    bytes_done: u64,
    bytes_total: u64,
    next_chunk: u64,
}

impl Packer {
    /// Start a package at the end of `file`, which already holds `base` bytes.
    pub(crate) fn new(file: File, base: u64, monitor: BuildMonitor) -> Self {
        Self {
            stream: ZipStream::new(file, base),
            monitor,
            names: HashSet::new(),
            files_done: 0,
            files_total: 0,
            bytes_done: 0,
            bytes_total: 0,
            next_chunk: 0,
        }
    }

    /// What is to come, for progress.
    pub(crate) fn expect(&mut self, items: &[PackItem]) {
        self.files_total += items.len();
        for item in items {
            self.bytes_total = self.bytes_total.saturating_add(item.size_hint);
        }
    }

    fn report(&self) {
        self.monitor.report(BuildStep::Packing {
            done: self.files_done,
            total: self.files_total,
            bytes_done: self.bytes_done,
            bytes_total: self.bytes_total,
        });
    }

    /// Put these in the package, in order by name, and say how each went. A file that could not be
    /// read is left out and said to be. `Err` means the package cannot be finished: it could not be
    /// written, or the build was cancelled.
    pub(crate) fn pack(
        &mut self,
        items: &[PackItem],
        rules: &SourceRules<'_>,
    ) -> Result<Vec<PackOutcome>> {
        let mut order: Vec<usize> = (0..items.len()).collect();
        order.sort_by(|left, right| items[*left].name.cmp(&items[*right].name));
        let monitor = self.monitor.clone();
        let is_cancelled = || monitor.is_cancelled();
        let shared = Shared {
            items,
            order: &order,
            rules,
            queue: Mutex::new(Queue::default()),
            has_work: Condvar::new(),
            has_result: Condvar::new(),
            read_gate: Mutex::new(()),
            next_spool: AtomicU64::new(0),
            is_cancelled: &is_cancelled,
        };
        let mut outcomes: Vec<PackOutcome> = vec![Err(String::new()); items.len()];
        std::thread::scope(|scope| {
            for _ in 0..worker_count() {
                scope.spawn(|| run_worker(&shared, &monitor));
            }
            // However this ends, on success too: the workers have nothing left, and wait until they
            // hear so, and this thread cannot leave until they have gone.
            let _stops_them = StopWorkers(&shared);
            self.write_in_order(&shared, &mut outcomes)
        })?;
        Ok(outcomes)
    }

    fn write_in_order(&mut self, shared: &Shared<'_>, outcomes: &mut [PackOutcome]) -> Result<()> {
        for (position, index) in shared.order.iter().copied().enumerate() {
            self.monitor.checkpoint()?;
            self.report();
            let finished = self.wait_for_file(shared, position)?;
            let item = &shared.items[index];
            let duplicate = !self.names.insert(item.name.to_lowercase());
            outcomes[index] = if duplicate {
                Err(format!(
                    "another file has the same place in the package: {}",
                    item.name
                ))
            } else {
                match finished {
                    Finished::Whole {
                        method,
                        crc32,
                        sha256,
                        size,
                        data,
                    } => self
                        .stream
                        .add_whole(&WholeEntry {
                            name: &item.name,
                            method,
                            crc32,
                            uncompressed_bytes: size,
                            data: &data,
                        })
                        .map_err(|error| WinAppMigratorError::io("the installer", error))
                        .map(|()| {
                            self.bytes_done += size;
                            Ok(PackedFile { sha256, size })
                        })?,
                    Finished::TooBig => self.stream_large(shared, item)?,
                    Finished::Failed(reason) => Err(reason),
                }
            };
            if let (Ok(packed), Some(expected)) = (&outcomes[index], &item.expected_sha256)
                && !packed.sha256.eq_ignore_ascii_case(expected)
            {
                return Err(WinAppMigratorError::Archive(format!(
                    "{} changed in the protected folder while the package was being made",
                    item.name
                )));
            }
            self.files_done += 1;
        }
        self.report();
        Ok(())
    }

    fn wait_for_file(&self, shared: &Shared<'_>, position: usize) -> Result<Finished> {
        let mut queue = shared.lock();
        loop {
            if let Some(finished) = queue.finished.remove(&position) {
                queue.waiting_bytes = queue.waiting_bytes.saturating_sub(finished.held_bytes());
                shared.has_work.notify_all();
                return Ok(finished);
            }
            if queue.worker_died {
                return Err(a_worker_died());
            }
            let (next, _) = shared
                .has_result
                .wait_timeout(queue, CANCEL_LOOK)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            queue = next;
            self.monitor.checkpoint()?;
        }
    }

    /// Read a file too big to hold, compress it chunk by chunk, and write it as it comes. What goes
    /// in the hash, the checksum, and the package is what each read returned, so the file changing
    /// underneath can only change what all three say together. A file that cannot be read to its
    /// end comes back out, and only a package that cannot be written stops the build.
    fn stream_large(&mut self, shared: &Shared<'_>, item: &PackItem) -> Result<PackOutcome> {
        if let Err(error) = (shared.rules.before)(&item.source) {
            return Ok(Err(error.to_string()));
        }
        let mut opened = match open_source(shared, &item.source) {
            Ok(opened) => opened,
            Err(error) => return Ok(Err(error.to_string())),
        };
        let kept = sample_of_file(&opened.file);
        // Taking a sample moves a file's place in it, whatever came of it. What is read from here on
        // goes in the package, and has to start at the start.
        if let Err(error) = opened.file.seek(SeekFrom::Start(0)) {
            return Ok(Err(format!("{}: {error}", item.source.display())));
        }
        let method = store_or_deflate(&item.name, kept);
        let mut entry = self
            .stream
            .begin(&item.name, method)
            .map_err(|error| WinAppMigratorError::io("the installer", error))?;
        let started_at = self.bytes_done;
        let mut hasher = Sha256::new();
        let mut crc = crc32fast::Hasher::new();
        let mut size = 0_u64;
        let mut next_to_write = self.next_chunk;
        let most_in_flight = (worker_count() as u64) * 2;
        let mut failure: Option<String> = None;
        loop {
            // A cancel is not a failure of this file: it leaves the whole build.
            self.monitor.checkpoint()?;
            let mut chunk = vec![0_u8; CHUNK_BYTES];
            let read = {
                let _gate = shared
                    .rules
                    .connected_drive
                    .then(|| shared.read_gate.lock());
                read_fully(&mut opened.file, &mut chunk)
            };
            let read = match read {
                Ok(read) => read,
                Err(error) => {
                    failure = Some(format!("{}: {error}", item.source.display()));
                    break;
                }
            };
            if read == 0 {
                break;
            }
            chunk.truncate(read);
            hasher.update(&chunk);
            crc.update(&chunk);
            size += read as u64;
            self.bytes_done = started_at + size;
            if method == ZipMethod::Stored {
                self.stream
                    .write_data(&mut entry, &chunk)
                    .map_err(|error| WinAppMigratorError::io("the installer", error))?;
            } else {
                let sequence = self.next_chunk;
                self.next_chunk += 1;
                {
                    let mut queue = shared.lock();
                    queue.chunk_jobs.push_back(ChunkJob {
                        sequence,
                        data: chunk,
                    });
                }
                shared.has_work.notify_one();
                // Keep what is in flight bounded, and write what is ready.
                let in_flight = self.next_chunk - next_to_write;
                self.write_ready_chunks(
                    shared,
                    &mut entry,
                    &mut next_to_write,
                    in_flight >= most_in_flight,
                )?;
            }
            self.report();
        }
        // Whatever is still out has to come back before anything else can be written.
        while next_to_write < self.next_chunk {
            let Some(result) = self.next_chunk_result(shared, next_to_write, true)? else {
                continue;
            };
            let bytes = result.map_err(|error| WinAppMigratorError::io("the installer", error))?;
            if failure.is_none() {
                self.stream
                    .write_data(&mut entry, &bytes)
                    .map_err(|error| WinAppMigratorError::io("the installer", error))?;
            }
            next_to_write += 1;
        }
        drop(opened);
        if failure.is_none()
            && let Err(error) = (shared.rules.after)(&item.source)
        {
            failure = Some(error.to_string());
        }
        if let Some(reason) = failure {
            self.stream
                .abandon(entry)
                .map_err(|error| WinAppMigratorError::io("the installer", error))?;
            self.bytes_done = started_at;
            return Ok(Err(reason));
        }
        if method == ZipMethod::Deflated {
            self.stream
                .write_data(&mut entry, &DEFLATE_END)
                .map_err(|error| WinAppMigratorError::io("the installer", error))?;
        }
        self.stream
            .end(entry, crc.finalize(), size)
            .map_err(|error| WinAppMigratorError::io("the installer", error))?;
        Ok(Ok(PackedFile {
            sha256: hex_lower(&hasher.finalize()),
            size,
        }))
    }

    /// Write the chunks that are ready, in order, having waited for at least one if asked to.
    fn write_ready_chunks(
        &mut self,
        shared: &Shared<'_>,
        entry: &mut OpenEntry,
        next_to_write: &mut u64,
        must_wait: bool,
    ) -> Result<()> {
        let mut must_wait = must_wait;
        while let Some(result) = self.next_chunk_result(shared, *next_to_write, must_wait)? {
            let bytes = result.map_err(|error| WinAppMigratorError::io("the installer", error))?;
            self.stream
                .write_data(entry, &bytes)
                .map_err(|error| WinAppMigratorError::io("the installer", error))?;
            *next_to_write += 1;
            must_wait = false;
        }
        Ok(())
    }

    fn next_chunk_result(
        &self,
        shared: &Shared<'_>,
        sequence: u64,
        must_wait: bool,
    ) -> Result<Option<io::Result<Vec<u8>>>> {
        let mut queue = shared.lock();
        loop {
            if let Some(result) = queue.chunk_results.remove(&sequence) {
                return Ok(Some(result));
            }
            if !must_wait {
                return Ok(None);
            }
            if queue.worker_died {
                return Err(a_worker_died());
            }
            let (next, _) = shared
                .has_result
                .wait_timeout(queue, CANCEL_LOOK)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            queue = next;
            self.monitor.checkpoint()?;
        }
    }

    /// Finish the package. Nothing more can go in.
    pub(crate) fn finish(self) -> Result<FinishedZip> {
        self.stream
            .finish()
            .map_err(|error| WinAppMigratorError::io("the installer", error))
    }
}

fn a_worker_died() -> WinAppMigratorError {
    WinAppMigratorError::Archive(
        "a thread compressing files stopped unexpectedly, so no installer was written".to_owned(),
    )
}

enum Job {
    Chunk(ChunkJob),
    File(usize),
}

fn next_job(shared: &Shared<'_>, monitor: &BuildMonitor) -> Option<Job> {
    let mut queue = shared.lock();
    loop {
        if queue.stopping || monitor.is_cancelled() {
            return None;
        }
        // The file whose turn it is comes before anything new.
        if let Some(chunk) = queue.chunk_jobs.pop_front() {
            return Some(Job::Chunk(chunk));
        }
        if queue.next_file < shared.order.len() && queue.waiting_bytes < MOST_WAITING_BYTES {
            let position = queue.next_file;
            queue.next_file += 1;
            return Some(Job::File(position));
        }
        let (next, _) = shared
            .has_work
            .wait_timeout(queue, CANCEL_LOOK)
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        queue = next;
    }
}

struct StopWorkers<'a, 'b>(&'a Shared<'b>);

impl Drop for StopWorkers<'_, '_> {
    fn drop(&mut self) {
        self.0.stop();
    }
}

/// A worker that goes down says so, for the thread writing to fail the build by, and goes no further
/// than that. Nothing here is meant to panic, and a dependency could. Left to reach the end of the
/// scope it would take the thread with it, and a window would wait for that thread forever.
fn run_worker(shared: &Shared<'_>, monitor: &BuildMonitor) {
    let finished = catch_unwind(AssertUnwindSafe(|| work_until_stopped(shared, monitor)));
    if let Err(panic) = finished {
        tracing::error!(reason = %panic_text(panic.as_ref()), "a thread packing files stopped");
        shared.lock().worker_died = true;
        shared.stop();
    }
}

fn work_until_stopped(shared: &Shared<'_>, monitor: &BuildMonitor) {
    let mut compressor = Compress::new(Compression::new(COMPRESSION_LEVEL), false);
    while let Some(job) = next_job(shared, monitor) {
        match job {
            Job::Chunk(chunk) => {
                let result = deflate_chunk(&mut compressor, &chunk.data);
                drop(chunk.data);
                shared.lock().chunk_results.insert(chunk.sequence, result);
                shared.has_result.notify_all();
            }
            Job::File(position) => {
                let item = &shared.items[shared.order[position]];
                let finished = read_whole(shared, item, &mut compressor);
                let mut queue = shared.lock();
                queue.waiting_bytes += finished.held_bytes();
                queue.finished.insert(position, finished);
                drop(queue);
                shared.has_result.notify_all();
            }
        }
    }
}

/// A source file, open, and a copy of it to remove when this goes, if that is what it took.
struct OpenedSource {
    file: File,
    spooled: Option<PathBuf>,
}

impl Drop for OpenedSource {
    fn drop(&mut self) {
        if let Some(folder) = self.spooled.take().as_deref().and_then(Path::parent) {
            let _ = fs::remove_dir_all(folder);
        }
    }
}

/// Open what is to be read. On a drive, nothing that is a link is opened at all: whatever has that
/// name is what is looked at, however it came to have it since it was last asked about.
fn open_file(path: &Path, connected_drive: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    if connected_drive {
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options.open(path)?;
    if connected_drive && file.metadata()?.file_type().is_symlink() {
        return Err(io::Error::other(format!(
            "{} is a link, so it was not read",
            path.display()
        )));
    }
    Ok(file)
}

fn open_source(shared: &Shared<'_>, path: &Path) -> Result<OpenedSource> {
    let connected_drive = shared.rules.connected_drive;
    match open_file(path, connected_drive) {
        Ok(file) => Ok(OpenedSource {
            file,
            spooled: None,
        }),
        // A file's permissions can name no one on this PC, on a drive from another. That takes a
        // backup-mode copy to read, which goes in the protected folder and no further.
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied => {
            let (Some(spool), Some(name)) = (shared.rules.spool, path.file_name()) else {
                return Err(WinAppMigratorError::io(path, error));
            };
            let number = shared.next_spool.fetch_add(1, Ordering::Relaxed);
            let folder = spool.join(number.to_string());
            fs::create_dir_all(&folder).map_err(|error| WinAppMigratorError::io(&folder, error))?;
            let copy = folder.join(name);
            let copied =
                copy_file_backup_mode_unless(path, &copy, shared.is_cancelled).and_then(|()| {
                    File::open(&copy).map_err(|error| WinAppMigratorError::io(&copy, error))
                });
            match copied {
                Ok(file) => Ok(OpenedSource {
                    file,
                    spooled: Some(copy),
                }),
                Err(error) => {
                    let _ = fs::remove_dir_all(&folder);
                    Err(error)
                }
            }
        }
        Err(error) => Err(WinAppMigratorError::io(path, error)),
    }
}

/// As much as a read can give of what `buffer` holds, and less only at the end of the file.
fn read_fully(file: &mut File, buffer: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buffer.len() {
        match file.read(&mut buffer[filled..]) {
            Ok(0) => break,
            Ok(read) => filled += read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(filled)
}

/// All of a file, if it fits in a chunk, or one byte past that to say that it does not. Room is made
/// for what it was seen to be, so that a small file does not hold a chunk's worth of memory while it
/// waits its turn, and what it has become is what counts.
fn read_up_to_a_chunk(file: &mut File, size_hint: u64) -> io::Result<Vec<u8>> {
    let most = CHUNK_BYTES + 1;
    let first = usize::try_from(size_hint.saturating_add(1)).map_or(most, |size| size.min(most));
    let mut buffer = vec![0_u8; first];
    let mut filled = read_fully(file, &mut buffer)?;
    if filled == first && first < most {
        buffer.resize(most, 0);
        filled += read_fully(file, &mut buffer[filled..])?;
    }
    buffer.truncate(filled);
    Ok(buffer)
}

/// Read a file into memory, and hash and compress what was read.
fn read_whole(shared: &Shared<'_>, item: &PackItem, compressor: &mut Compress) -> Finished {
    if item.size_hint > CHUNK_BYTES as u64 {
        return Finished::TooBig;
    }
    if let Err(error) = (shared.rules.before)(&item.source) {
        return Finished::Failed(error.to_string());
    }
    let read = {
        let _gate = shared
            .rules
            .connected_drive
            .then(|| shared.read_gate.lock());
        let mut opened = match open_source(shared, &item.source) {
            Ok(opened) => opened,
            Err(error) => return Finished::Failed(error.to_string()),
        };
        read_up_to_a_chunk(&mut opened.file, item.size_hint)
    };
    let buffer = match read {
        Ok(buffer) => buffer,
        Err(error) => return Finished::Failed(format!("{}: {error}", item.source.display())),
    };
    let read = buffer.len();
    // It grew since the scan saw it. Nothing of this read has gone anywhere yet.
    if read > CHUNK_BYTES {
        return Finished::TooBig;
    }
    if let Err(error) = (shared.rules.after)(&item.source) {
        return Finished::Failed(error.to_string());
    }
    let sha256 = hex_lower(&Sha256::digest(&buffer));
    let crc32 = crc32fast::hash(&buffer);
    let sample = &buffer[..read.min(SAMPLE_BYTES)];
    let kept = sample_kept(compressor, sample);
    let (method, data) = match store_or_deflate(&item.name, kept) {
        ZipMethod::Stored => (ZipMethod::Stored, buffer),
        ZipMethod::Deflated => match deflate_whole(compressor, &buffer) {
            // Whatever a sample said, nothing ends up bigger than it started.
            Ok(compressed) if compressed.len() < buffer.len() => (ZipMethod::Deflated, compressed),
            Ok(_) => (ZipMethod::Stored, buffer),
            Err(error) => {
                return Finished::Failed(format!(
                    "{} could not be compressed: {error}",
                    item.source.display()
                ));
            }
        },
    };
    Finished::Whole {
        method,
        crc32,
        sha256,
        size: read as u64,
        data,
    }
}

/// How much of a sample is left after compressing it, as a share of what it was. `None` if there is
/// nothing to go by.
fn sample_kept(compressor: &mut Compress, sample: &[u8]) -> Option<f64> {
    if sample.is_empty() {
        return None;
    }
    let compressed = deflate_whole(compressor, sample).ok()?;
    Some(compressed.len() as f64 / sample.len() as f64)
}

/// A sample of a file too big to hold, from its start, middle, and end, since a package format can
/// start with an index and then be all compressed data. Whatever is read here goes nowhere else, and
/// it leaves the file's place in it wherever it likes.
fn sample_of_file(file: &File) -> Option<f64> {
    let size = file.metadata().ok()?.len();
    let mut compressor = Compress::new(Compression::new(COMPRESSION_LEVEL), false);
    let mut sample = Vec::with_capacity(SAMPLE_BYTES * LARGE_FILE_SAMPLES as usize);
    let mut buffer = vec![0_u8; SAMPLE_BYTES];
    for place in 0..LARGE_FILE_SAMPLES {
        let offset = size.saturating_sub(SAMPLE_BYTES as u64) / (LARGE_FILE_SAMPLES - 1) * place;
        let read = file.seek_read(&mut buffer, offset).ok()?;
        sample.extend_from_slice(&buffer[..read]);
    }
    sample_kept(&mut compressor, &sample)
}

/// One whole deflate stream.
fn deflate_whole(compressor: &mut Compress, data: &[u8]) -> io::Result<Vec<u8>> {
    compressor.reset();
    let mut output = Vec::with_capacity(data.len() / 2 + 1024);
    loop {
        let consumed = usize::try_from(compressor.total_in()).unwrap_or(data.len());
        let status = compressor
            .compress_vec(&data[consumed..], &mut output, FlushCompress::Finish)
            .map_err(io::Error::other)?;
        match status {
            Status::StreamEnd => return Ok(output),
            Status::Ok | Status::BufError => {
                output.reserve(output.len().max(64 * 1024));
            }
        }
    }
}

/// A chunk of a stream, ended on a byte boundary so that chunks can follow one another, with
/// nothing carried over from the one before. It comes out a little bigger for that, which is the
/// price of compressing one file on many threads with no copy of it on disk.
fn deflate_chunk(compressor: &mut Compress, data: &[u8]) -> io::Result<Vec<u8>> {
    compressor.reset();
    let mut output = Vec::with_capacity(data.len() / 2 + 1024);
    loop {
        let consumed = usize::try_from(compressor.total_in()).unwrap_or(data.len());
        compressor
            .compress_vec(&data[consumed..], &mut output, FlushCompress::Sync)
            .map_err(io::Error::other)?;
        let all_in = usize::try_from(compressor.total_in()).is_ok_and(|taken| taken >= data.len());
        // Room left over means it had no more to give.
        if all_in && output.len() < output.capacity() {
            return Ok(output);
        }
        output.reserve(output.len().max(64 * 1024));
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use zip::ZipArchive;

    use super::*;
    use crate::hashing::sha256_file;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn no_check(_: &Path) -> Result<()> {
        Ok(())
    }

    fn rules() -> SourceRules<'static> {
        SourceRules {
            before: &no_check,
            after: &no_check,
            connected_drive: false,
            spool: None,
        }
    }

    /// Bytes that do not compress, from a fixed start, without a crate for it.
    fn noise(len: usize, mut state: u64) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(len + 8);
        while bytes.len() < len {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            bytes.extend_from_slice(&state.to_le_bytes());
        }
        bytes.truncate(len);
        bytes
    }

    fn text(len: usize) -> Vec<u8> {
        b"the quick brown fox jumps over the lazy dog, again and again\n"
            .iter()
            .copied()
            .cycle()
            .take(len)
            .collect()
    }

    fn item(folder: &Path, name: &str, bytes: &[u8]) -> TestResult<PackItem> {
        let source = folder.join(format!("source-{}", name.replace('/', "_")));
        fs::write(&source, bytes)?;
        Ok(PackItem {
            name: name.to_owned(),
            source,
            size_hint: bytes.len() as u64,
            expected_sha256: None,
        })
    }

    fn pack_all(
        output: &Path,
        items: &[PackItem],
        rules: &SourceRules<'_>,
    ) -> Result<(Vec<PackOutcome>, FinishedZip)> {
        let file = File::create(output).map_err(|error| WinAppMigratorError::io(output, error))?;
        let mut packer = Packer::new(file, 0, BuildMonitor::default());
        packer.expect(items);
        let outcomes = packer.pack(items, rules)?;
        Ok((outcomes, packer.finish()?))
    }

    fn entry_of(output: &Path, name: &str) -> TestResult<(Vec<u8>, zip::CompressionMethod, u64)> {
        let mut archive = ZipArchive::new(File::open(output)?)?;
        let mut entry = archive.by_name(name)?;
        let method = entry.compression();
        let compressed = entry.compressed_size();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        Ok((bytes, method, compressed))
    }

    #[test]
    fn what_a_name_says_only_counts_for_what_a_sample_cannot_settle() {
        assert_eq!(store_or_deflate("a.zip", Some(0.99)), ZipMethod::Stored);
        assert_eq!(store_or_deflate("a.dat", Some(0.99)), ZipMethod::Stored);
        assert_eq!(store_or_deflate("a.zip", Some(0.50)), ZipMethod::Deflated);
        assert_eq!(store_or_deflate("a.dat", Some(0.50)), ZipMethod::Deflated);
        assert_eq!(store_or_deflate("a.JPG", Some(0.95)), ZipMethod::Stored);
        assert_eq!(store_or_deflate("a.dat", Some(0.95)), ZipMethod::Deflated);
        assert_eq!(
            store_or_deflate("dir.zip/a", Some(0.95)),
            ZipMethod::Deflated
        );
        assert_eq!(store_or_deflate("empty.txt", None), ZipMethod::Stored);
    }

    #[test]
    fn files_of_every_kind_come_back_as_they_were_with_the_hash_of_what_went_in() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![
            item(work.path(), "files/empty.txt", b"")?,
            item(work.path(), "files/small.txt", &text(1000))?,
            item(work.path(), "files/mislabeled.png", &text(100_000))?,
            item(work.path(), "files/noise.dat", &noise(100_000, 7))?,
            item(work.path(), "files/big.txt", &text(CHUNK_BYTES * 3 + 123))?,
            item(
                work.path(),
                "files/big noise.zip",
                &noise(CHUNK_BYTES * 2 + 5, 9),
            )?,
            item(work.path(), "files/exactly a chunk.txt", &text(CHUNK_BYTES))?,
        ];
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules())?;
        drop(finished.file);
        for (item, outcome) in items.iter().zip(&outcomes) {
            let packed = outcome.as_ref().map_err(|error| error.clone())?;
            assert_eq!(packed.sha256, sha256_file(&item.source)?, "{}", item.name);
            let (bytes, _, _) = entry_of(&output, &item.name)?;
            assert_eq!(bytes, fs::read(&item.source)?, "{}", item.name);
            assert_eq!(packed.size, bytes.len() as u64);
        }
        Ok(())
    }

    #[test]
    fn what_does_not_compress_is_stored_and_what_is_mislabeled_is_not() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![
            item(work.path(), "noise.dat", &noise(200_000, 3))?,
            item(work.path(), "text.png", &text(200_000))?,
            item(work.path(), "big noise.bin", &noise(CHUNK_BYTES * 2, 5))?,
            item(work.path(), "big text.jpg", &text(CHUNK_BYTES * 2))?,
            item(work.path(), "tiny.txt", b"x")?,
        ];
        let output = work.path().join("out.zip");
        let (_, finished) = pack_all(&output, &items, &rules())?;
        drop(finished.file);

        let stored = zip::CompressionMethod::Stored;
        let (_, method, size) = entry_of(&output, "noise.dat")?;
        assert_eq!((method, size), (stored, 200_000));
        let (_, method, _) = entry_of(&output, "big noise.bin")?;
        assert_eq!(method, stored);
        let (_, method, size) = entry_of(&output, "text.png")?;
        assert_ne!(method, stored);
        assert!(size < 10_000, "{size}");
        let (_, method, size) = entry_of(&output, "big text.jpg")?;
        assert_ne!(method, stored);
        assert!(size < (CHUNK_BYTES / 10) as u64, "{size}");
        // A byte compresses to more than a byte, so it stays as it is.
        let (bytes, method, size) = entry_of(&output, "tiny.txt")?;
        assert_eq!(
            (bytes.as_slice(), method, size),
            (b"x".as_slice(), stored, 1)
        );
        Ok(())
    }

    #[test]
    fn entries_go_in_by_name_whatever_order_they_come_in_and_finish_in() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut items = Vec::new();
        // A big file among small ones, so that later files finish well before it.
        for index in (0..200).rev() {
            let bytes = if index == 100 {
                text(CHUNK_BYTES + 10)
            } else {
                text(10 + index)
            };
            items.push(item(work.path(), &format!("f/{index:03}.txt"), &bytes)?);
        }
        let first = work.path().join("first.zip");
        let second = work.path().join("second.zip");
        let (_, one) = pack_all(&first, &items, &rules())?;
        items.reverse();
        let (_, two) = pack_all(&second, &items, &rules())?;
        drop(one.file);
        drop(two.file);

        let archive = ZipArchive::new(File::open(&first)?)?;
        let names: Vec<String> = archive
            .file_names()
            .filter_map(std::result::Result::ok)
            .map(std::borrow::Cow::into_owned)
            .collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
        assert_eq!(names.len(), 200);
        assert_eq!(
            fs::read(&first)?,
            fs::read(&second)?,
            "the same bytes in the same order"
        );
        assert_eq!(one.sha256, two.sha256);
        Ok(())
    }

    #[test]
    fn a_file_that_cannot_be_read_is_left_out_and_the_rest_are_not() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut missing = item(work.path(), "b/missing.txt", b"x")?;
        fs::remove_file(&missing.source)?;
        missing.size_hint = 1;
        let mut missing_big = item(work.path(), "b/missing big.txt", b"x")?;
        fs::remove_file(&missing_big.source)?;
        missing_big.size_hint = CHUNK_BYTES as u64 * 2;
        let items = vec![
            item(work.path(), "a.txt", b"a")?,
            missing,
            missing_big,
            item(work.path(), "c.txt", b"c")?,
        ];
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules())?;
        drop(finished.file);
        assert!(outcomes[0].is_ok());
        assert!(outcomes[1].is_err());
        assert!(outcomes[2].is_err());
        assert!(outcomes[3].is_ok());
        let archive = ZipArchive::new(File::open(&output)?)?;
        assert_eq!(archive.len(), 2);
        Ok(())
    }

    #[test]
    fn what_a_build_asks_after_reading_takes_a_file_back_out_of_any_size() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![
            item(work.path(), "a.txt", b"keep")?,
            item(work.path(), "small link.txt", b"not this")?,
            item(work.path(), "big link.txt", &text(CHUNK_BYTES * 2))?,
            item(work.path(), "z.txt", b"keep too")?,
        ];
        let refuse = |path: &Path| -> Result<()> {
            if path.to_string_lossy().contains("link") {
                return Err(WinAppMigratorError::InvalidInput(
                    "it leads somewhere else by now".to_owned(),
                ));
            }
            Ok(())
        };
        let rules = SourceRules {
            before: &no_check,
            after: &refuse,
            connected_drive: false,
            spool: None,
        };
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules)?;
        drop(finished.file);
        assert!(outcomes[1].is_err());
        assert!(outcomes[2].is_err());
        let mut archive = ZipArchive::new(File::open(&output)?)?;
        let names: Vec<String> = archive
            .file_names()
            .filter_map(std::result::Result::ok)
            .map(std::borrow::Cow::into_owned)
            .collect();
        assert_eq!(names, ["a.txt", "z.txt"]);
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index)?;
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;
        }
        let bytes = fs::read(&output)?;
        assert_eq!(
            finished.sha256.as_slice(),
            Sha256::digest(&bytes).as_slice()
        );
        assert!(
            !bytes.windows(16).any(|window| window == &text(16)[..]),
            "what was taken out is gone from the file"
        );
        Ok(())
    }

    #[test]
    fn two_files_with_the_same_place_in_the_package_are_not_both_put_there() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![
            item(work.path(), "f/a.txt", b"first")?,
            item(work.path(), "F/A.TXT", b"second")?,
        ];
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules())?;
        drop(finished.file);
        assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
        assert_eq!(ZipArchive::new(File::open(&output)?)?.len(), 1);
        Ok(())
    }

    #[test]
    fn a_held_file_that_has_changed_stops_the_build() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut held = item(work.path(), "manifest.json", b"changed")?;
        held.expected_sha256 = Some("00".repeat(32));
        let output = work.path().join("out.zip");
        assert!(pack_all(&output, &[held], &rules()).is_err());
        Ok(())
    }

    /// A panic is not meant to happen. If one does, it must not leave anything waiting forever.
    #[test]
    fn a_worker_that_goes_down_fails_the_build_and_nothing_waits_for_it() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![item(work.path(), "a.txt", b"small")?];
        let panics = |_: &Path| -> Result<()> { panic!("this is a test of it") };
        let rules = SourceRules {
            before: &panics,
            after: &no_check,
            connected_drive: false,
            spool: None,
        };
        let output = work.path().join("out.zip");
        let failed = pack_all(&output, &items, &rules);
        assert!(
            matches!(&failed, Err(WinAppMigratorError::Archive(message))
                if message.contains("stopped unexpectedly")),
            "{:?}",
            failed.map(|_| ())
        );
        Ok(())
    }

    /// The same on the thread writing, whose panic goes on to what called it, with the workers gone.
    #[test]
    fn the_thread_writing_going_down_does_not_leave_the_workers_waiting() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut large = item(work.path(), "large.bin", b"read by the thread writing")?;
        large.size_hint = CHUNK_BYTES as u64 * 2;
        let items = vec![large];
        let panics = |_: &Path| -> Result<()> { panic!("this is a test of it") };
        let rules = SourceRules {
            before: &panics,
            after: &no_check,
            connected_drive: false,
            spool: None,
        };
        let output = work.path().join("out.zip");
        let went_down = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _ = pack_all(&output, &items, &rules);
        }));
        assert!(went_down.is_err());
        Ok(())
    }

    #[test]
    fn a_small_file_only_holds_what_it_needs() -> TestResult {
        let work = tempfile::tempdir()?;
        let path = work.path().join("a");
        fs::write(&path, text(1000))?;
        // As it was seen, smaller than it is, and bigger than it is.
        for hint in [1000, 10, 100_000] {
            let bytes = read_up_to_a_chunk(&mut File::open(&path)?, hint)?;
            assert_eq!(bytes.len(), 1000, "{hint}");
        }
        assert!(read_up_to_a_chunk(&mut File::open(&path)?, 1000)?.capacity() <= 1001);
        // One that has grown past a chunk says so, by one byte more than fits.
        fs::write(&path, text(CHUNK_BYTES * 2))?;
        let bytes = read_up_to_a_chunk(&mut File::open(&path)?, 10)?;
        assert_eq!(bytes.len(), CHUNK_BYTES + 1);
        Ok(())
    }

    /// Whatever a sample did, what goes in the package starts at the file's start. The sample of a
    /// large file comes from its middle and its end last of all, so a mistake here would show.
    #[test]
    fn a_large_file_goes_in_from_its_first_byte() -> TestResult {
        let work = tempfile::tempdir()?;
        let bytes = noise(CHUNK_BYTES + SAMPLE_BYTES * 2, 31);
        let large = item(work.path(), "large.bin", &bytes)?;
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, std::slice::from_ref(&large), &rules())?;
        drop(finished.file);
        let packed = outcomes[0].as_ref().map_err(|error| error.clone())?;
        assert_eq!(packed.size, bytes.len() as u64);
        assert_eq!(packed.sha256, hex_lower(&Sha256::digest(&bytes)));
        assert_eq!(entry_of(&output, "large.bin")?.0, bytes);
        Ok(())
    }

    #[test]
    fn a_file_that_has_grown_past_what_can_be_held_is_read_when_its_turn_comes() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut grown = item(work.path(), "grown.txt", &text(CHUNK_BYTES * 3))?;
        grown.size_hint = 10;
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, std::slice::from_ref(&grown), &rules())?;
        drop(finished.file);
        let packed = outcomes[0].as_ref().map_err(|error| error.clone())?;
        assert_eq!(packed.size, CHUNK_BYTES as u64 * 3);
        assert_eq!(packed.sha256, sha256_file(&grown.source)?);
        assert_eq!(entry_of(&output, "grown.txt")?.0, fs::read(&grown.source)?);
        Ok(())
    }

    #[test]
    fn a_file_changing_while_it_is_read_is_described_by_what_was_read() -> TestResult {
        let work = tempfile::tempdir()?;
        let source = work.path().join("growing.bin");
        fs::write(&source, noise(CHUNK_BYTES * 8, 11))?;
        let stop = Arc::new(AtomicBool::new(false));
        let writer = {
            let source = source.clone();
            let stop = Arc::clone(&stop);
            std::thread::spawn(move || -> io::Result<u64> {
                let mut file = OpenOptions::new().write(true).open(&source)?;
                let mut writes = 0;
                while !stop.load(Ordering::Relaxed) {
                    io::Seek::seek(
                        &mut file,
                        SeekFrom::Start((writes % 8) * CHUNK_BYTES as u64),
                    )?;
                    file.write_all(&noise(CHUNK_BYTES, writes + 100))?;
                    writes += 1;
                }
                Ok(writes)
            })
        };
        let items = [
            PackItem {
                name: "growing.bin".to_owned(),
                source: source.clone(),
                size_hint: CHUNK_BYTES as u64 * 8,
                expected_sha256: None,
            },
            PackItem {
                name: "held in memory.bin".to_owned(),
                source: source.clone(),
                size_hint: 1,
                expected_sha256: None,
            },
        ];
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules())?;
        stop.store(true, Ordering::Relaxed);
        drop(finished.file);
        assert!(writer.join().is_ok_and(|writes| writes.is_ok()));

        let packed = outcomes[0].as_ref().map_err(|error| error.clone())?;
        let (bytes, _, _) = entry_of(&output, "growing.bin")?;
        assert_eq!(hex_lower(&Sha256::digest(&bytes)), packed.sha256);
        assert_eq!(bytes.len() as u64, packed.size);
        assert_eq!(hex_lower(&Sha256::digest(&bytes)).len(), 64);
        Ok(())
    }

    #[test]
    fn cancelling_stops_reading_and_says_so() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![item(work.path(), "a.txt", &text(100))?];
        let monitor = BuildMonitor::default();
        monitor.cancel();
        let output = work.path().join("out.zip");
        let mut packer = Packer::new(File::create(&output)?, 0, monitor);
        assert!(matches!(
            packer.pack(&items, &rules()),
            Err(WinAppMigratorError::Cancelled)
        ));
        Ok(())
    }

    #[test]
    fn cancelling_part_way_through_a_large_file_stops_there() -> TestResult {
        let work = tempfile::tempdir()?;
        let items = vec![item(work.path(), "big.txt", &text(CHUNK_BYTES * 12))?];
        let seen = Arc::new(Mutex::new(0_u64));
        let sink = Arc::clone(&seen);
        let cancel: Arc<Mutex<Option<BuildMonitor>>> = Arc::new(Mutex::new(None));
        let canceller = Arc::clone(&cancel);
        let monitor = BuildMonitor::new(move |step| {
            if let BuildStep::Packing { bytes_done, .. } = step
                && bytes_done > 0
            {
                if let Ok(mut seen) = sink.lock() {
                    *seen = bytes_done;
                }
                if let Ok(canceller) = canceller.lock()
                    && let Some(monitor) = canceller.as_ref()
                {
                    monitor.cancel();
                }
            }
        });
        *cancel.lock().map_err(|error| error.to_string())? = Some(monitor.clone());
        let output = work.path().join("out.zip");
        let mut packer = Packer::new(File::create(&output)?, 0, monitor);
        assert!(matches!(
            packer.pack(&items, &rules()),
            Err(WinAppMigratorError::Cancelled)
        ));
        let seen = *seen.lock().map_err(|error| error.to_string())?;
        assert!(seen < CHUNK_BYTES as u64 * 12, "read all {seen} of it");
        Ok(())
    }

    #[test]
    fn memory_stays_bounded_because_later_files_wait_their_turn() -> TestResult {
        // The file whose turn it is has to finish, whatever is waiting behind it. A package of many
        // files just past what can wait would otherwise hang here.
        let work = tempfile::tempdir()?;
        let chunk = noise(CHUNK_BYTES, 21);
        let source = work.path().join("chunk.bin");
        fs::write(&source, &chunk)?;
        let count = (MOST_WAITING_BYTES / CHUNK_BYTES as u64) as usize + 20;
        let items: Vec<PackItem> = (0..count)
            .map(|index| PackItem {
                name: format!("f/{index:04}"),
                source: source.clone(),
                size_hint: CHUNK_BYTES as u64,
                expected_sha256: None,
            })
            .collect();
        let output = work.path().join("out.zip");
        let (outcomes, finished) = pack_all(&output, &items, &rules())?;
        drop(finished.file);
        assert!(outcomes.iter().all(std::result::Result::is_ok));
        Ok(())
    }

    /// Audio as it is on a disk: at the level that is really used it comes down a good way, and at
    /// the fastest it does not, which is why the sample is not taken at the fastest.
    #[test]
    fn a_sample_is_taken_at_the_level_that_is_really_used() -> TestResult {
        let mut pcm = Vec::with_capacity(SAMPLE_BYTES);
        let mut state = 12345_u64;
        let mut value = 0_i32;
        while pcm.len() < SAMPLE_BYTES {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            // A slow wave, and a little noise in its low byte.
            value = (value + (state % 64) as i32 - 31).clamp(-30000, 30000);
            pcm.extend_from_slice(&(value as i16).to_le_bytes());
        }
        let mut real = Compress::new(Compression::new(COMPRESSION_LEVEL), false);
        let kept = sample_kept(&mut real, &pcm).ok_or("no sample")?;
        assert_eq!(
            store_or_deflate("sound.wav", Some(kept)),
            ZipMethod::Deflated,
            "{kept}"
        );
        Ok(())
    }

    #[test]
    fn a_stream_of_chunks_is_one_deflate_stream_to_a_plain_reader() -> TestResult {
        let data = text(CHUNK_BYTES);
        let mut compressor = Compress::new(Compression::new(COMPRESSION_LEVEL), false);
        let mut stream = deflate_chunk(&mut compressor, &data[..CHUNK_BYTES / 2])?;
        stream.extend_from_slice(&deflate_chunk(&mut compressor, &data[CHUNK_BYTES / 2..])?);
        stream.extend_from_slice(&DEFLATE_END);
        let mut decoder = flate2::read::DeflateDecoder::new(stream.as_slice());
        let mut back = Vec::new();
        decoder.read_to_end(&mut back)?;
        assert_eq!(back, data);
        Ok(())
    }

    #[test]
    fn a_sample_finds_the_compressed_part_of_a_file_that_starts_with_text() -> TestResult {
        let work = tempfile::tempdir()?;
        let path = work.path().join("a.pak");
        let mut bytes = text(SAMPLE_BYTES);
        bytes.extend_from_slice(&noise(CHUNK_BYTES * 2, 13));
        fs::write(&path, &bytes)?;
        let kept = sample_of_file(&File::open(&path)?).ok_or("no sample")?;
        assert!(kept > 0.5, "{kept}");
        Ok(())
    }
}
