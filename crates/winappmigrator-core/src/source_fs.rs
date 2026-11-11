//! Reading what a scan and a build read: a file or folder of this PC, of a drive connected to it, or of
//! another PC through its client ([`crate::remote`]).
//!
//! For a path of this PC each of these is what `std::fs` does, and gives what it gives. For a path
//! below a paired PC's root, the client answers, and it never follows a link: [`metadata`] of a link
//! there is an error, as nothing on another PC can be followed from this one.

use std::ffi::OsString;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use winappmigrator_link::protocol::{Entry, EntryKind, MAX_READ};

use crate::remote::{RemotePc, remote_of};

/// Seconds from 1601, when `FILETIME`s start, to 1970.
const FILETIME_TO_UNIX_SECONDS: u64 = 11_644_473_600;
const FILETIME_STEPS_PER_SECOND: u64 = 10_000_000;

/// What a file, a folder or a link is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    File,
    Dir,
    Link,
    Other,
}

/// What a listing or a look at a path says of it.
#[derive(Debug, Clone)]
pub struct Metadata {
    kind: Kind,
    len: u64,
    attributes: u32,
    modified: Option<SystemTime>,
}

/// The kind alone, as `std::fs::FileType` answers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileType(Kind);

impl FileType {
    pub fn is_dir(self) -> bool {
        self.0 == Kind::Dir
    }
    pub fn is_file(self) -> bool {
        self.0 == Kind::File
    }
    pub fn is_symlink(self) -> bool {
        self.0 == Kind::Link
    }
}

impl Metadata {
    pub fn file_type(&self) -> FileType {
        FileType(self.kind)
    }
    pub fn is_dir(&self) -> bool {
        self.kind == Kind::Dir
    }
    pub fn is_file(&self) -> bool {
        self.kind == Kind::File
    }
    pub fn is_symlink(&self) -> bool {
        self.kind == Kind::Link
    }
    pub fn len(&self) -> u64 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// `FILE_ATTRIBUTE_*`.
    pub fn file_attributes(&self) -> u32 {
        self.attributes
    }
    pub fn modified(&self) -> io::Result<SystemTime> {
        self.modified
            .ok_or_else(|| io::Error::other("when it was changed is not known"))
    }
}

impl From<&fs::Metadata> for Metadata {
    fn from(metadata: &fs::Metadata) -> Self {
        let kind = metadata.file_type();
        Self {
            kind: if kind.is_symlink() {
                Kind::Link
            } else if kind.is_dir() {
                Kind::Dir
            } else if kind.is_file() {
                Kind::File
            } else {
                Kind::Other
            },
            len: metadata.len(),
            attributes: metadata.file_attributes(),
            modified: metadata.modified().ok(),
        }
    }
}

impl From<&Entry> for Metadata {
    fn from(entry: &Entry) -> Self {
        Self {
            kind: match entry.kind {
                EntryKind::File => Kind::File,
                EntryKind::Directory => Kind::Dir,
                EntryKind::Link => Kind::Link,
                EntryKind::Other => Kind::Other,
            },
            len: entry.len,
            attributes: entry.attributes,
            modified: time_of_filetime(entry.modified),
        }
    }
}

fn time_of_filetime(filetime: u64) -> Option<SystemTime> {
    let seconds = (filetime / FILETIME_STEPS_PER_SECOND).checked_sub(FILETIME_TO_UNIX_SECONDS)?;
    let nanos = (filetime % FILETIME_STEPS_PER_SECOND) * 100;
    UNIX_EPOCH.checked_add(Duration::new(seconds, u32::try_from(nanos).ok()?))
}

/// What a path is, without following a link.
pub fn symlink_metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
    let path = path.as_ref();
    match remote_of(path) {
        None => fs::symlink_metadata(path).map(|metadata| Metadata::from(&metadata)),
        Some(found) => {
            let (remote, relative) = found?;
            remote
                .describe(&relative)
                .map(|entry| Metadata::from(&entry))
        }
    }
}

/// What a path is, following a link on this PC. On another PC a link is not followed, and is an
/// error here.
pub fn metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
    let path = path.as_ref();
    match remote_of(path) {
        None => fs::metadata(path).map(|metadata| Metadata::from(&metadata)),
        Some(_) => {
            let metadata = symlink_metadata(path)?;
            if metadata.is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "a link on another PC is not followed",
                ));
            }
            Ok(metadata)
        }
    }
}

pub fn exists(path: impl AsRef<Path>) -> bool {
    metadata(path).is_ok()
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
    metadata(path).is_ok_and(|metadata| metadata.is_file())
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
    metadata(path).is_ok_and(|metadata| metadata.is_dir())
}

/// One thing in a folder, with what its listing said of it.
#[derive(Debug, Clone)]
pub struct DirEntry {
    path: PathBuf,
    name: OsString,
    metadata: Option<Metadata>,
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn file_name(&self) -> OsString {
        self.name.clone()
    }
    /// What the listing said, without following a link.
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.metadata
            .clone()
            .ok_or_else(|| io::Error::other("the listing said nothing of it"))
    }
    pub fn file_type(&self) -> io::Result<FileType> {
        self.metadata().map(|metadata| metadata.file_type())
    }
}

/// A folder's contents, as `std::fs::read_dir` gives them.
pub struct ReadDir(std::vec::IntoIter<io::Result<DirEntry>>);

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// A folder's contents. Another PC's come all at once.
pub fn read_dir(path: impl AsRef<Path>) -> io::Result<ReadDir> {
    let path = path.as_ref();
    let entries: Vec<io::Result<DirEntry>> = match remote_of(path) {
        None => fs::read_dir(path)?
            .map(|entry| {
                entry.map(|entry| DirEntry {
                    path: entry.path(),
                    name: entry.file_name(),
                    metadata: entry
                        .metadata()
                        .ok()
                        .map(|metadata| Metadata::from(&metadata)),
                })
            })
            .collect(),
        Some(found) => {
            let (remote, relative) = found?;
            remote
                .list(&relative)?
                .iter()
                .map(|entry| {
                    Ok(DirEntry {
                        path: path.join(&entry.name),
                        name: OsString::from(&entry.name),
                        metadata: Some(Metadata::from(entry)),
                    })
                })
                .collect()
        }
    };
    Ok(ReadDir(entries.into_iter()))
}

/// A file opened for reading, on this PC or another.
pub enum File {
    Local(fs::File),
    Remote(RemoteFile),
}

/// A file on another PC, read in parts as it is asked for.
pub struct RemoteFile {
    remote: Arc<RemotePc>,
    path: String,
    position: u64,
    len: u64,
    metadata: Metadata,
    buffer: Vec<u8>,
    buffer_start: u64,
}

impl File {
    /// What the file is, as it was when it was opened.
    pub fn metadata(&self) -> io::Result<Metadata> {
        match self {
            Self::Local(file) => file.metadata().map(|metadata| Metadata::from(&metadata)),
            Self::Remote(file) => Ok(file.metadata.clone()),
        }
    }

    /// Read from `offset`, as `FileExt::seek_read` does: where reading goes on from afterward is
    /// not to be relied on.
    pub fn seek_read(&self, buffer: &mut [u8], offset: u64) -> io::Result<usize> {
        match self {
            Self::Local(file) => std::os::windows::fs::FileExt::seek_read(file, buffer, offset),
            Self::Remote(file) => {
                let wanted = file.left_from(offset).min(buffer.len() as u64);
                if wanted == 0 {
                    return Ok(0);
                }
                let bytes = file.remote.read(&file.path, offset, read_size(wanted))?;
                let Some(into) = buffer.get_mut(..bytes.len()) else {
                    return Err(more_than_asked_for());
                };
                into.copy_from_slice(&bytes);
                Ok(bytes.len())
            }
        }
    }
}

/// Open a file to read it. On another PC a link is never opened.
pub fn open(path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();
    match remote_of(path) {
        None => fs::File::open(path).map(File::Local),
        Some(found) => {
            let (remote, relative) = found?;
            let entry = remote.describe(&relative)?;
            if entry.kind != EntryKind::File {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "only a file is opened on another PC",
                ));
            }
            Ok(File::Remote(RemoteFile {
                remote,
                path: relative,
                position: 0,
                len: entry.len,
                metadata: Metadata::from(&entry),
                buffer: Vec::new(),
                buffer_start: 0,
            }))
        }
    }
}

/// All of a file. Another PC's is never more than it was first said to be, which can be anything.
/// What is expected to be small is read with [`read_at_most`].
pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = path.as_ref();
    match remote_of(path) {
        None => fs::read(path),
        Some(_) => {
            let mut bytes = Vec::new();
            open(path)?.read_to_end(&mut bytes)?;
            Ok(bytes)
        }
    }
}

/// All of a file that is no more than `limit` bytes, or an error. What it says of its size is
/// checked first, and no more than that is read whatever it says.
pub fn read_at_most(path: impl AsRef<Path>, limit: u64) -> io::Result<Vec<u8>> {
    let path = path.as_ref();
    let file = open(path)?;
    let size = file.metadata()?.len();
    if size > limit {
        return Err(too_large(size, limit));
    }
    let mut bytes = Vec::new();
    file.take(limit.saturating_add(1)).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(too_large(bytes.len() as u64, limit));
    }
    Ok(bytes)
}

fn too_large(size: u64, limit: u64) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("it is {size} bytes, and no more than {limit} were expected"),
    )
}

fn more_than_asked_for() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "the other PC sent more of a file than was asked for",
    )
}

/// What one read asks for, of `wanted` bytes.
fn read_size(wanted: u64) -> u32 {
    u32::try_from(wanted).unwrap_or(MAX_READ).min(MAX_READ)
}

pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    String::from_utf8(read(path)?)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// Copy a file to a new file of this PC, as `std::fs::copy` does.
pub fn copy_to_this_pc(source: &Path, destination: &Path) -> io::Result<u64> {
    match remote_of(source) {
        None => fs::copy(source, destination),
        Some(_) => {
            let mut from = open(source)?;
            let mut to = fs::File::create(destination)?;
            io::copy(&mut from, &mut to)
        }
    }
}

/// Whether a path is below the root of another PC read through its client.
pub fn is_on_another_pc(path: impl AsRef<Path>) -> bool {
    remote_of(path.as_ref()).is_some()
}

impl Read for File {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Local(file) => file.read(buffer),
            Self::Remote(file) => file.read(buffer),
        }
    }
}

impl Seek for File {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        match self {
            Self::Local(file) => file.seek(position),
            Self::Remote(file) => file.seek(position),
        }
    }
}

impl RemoteFile {
    /// How much of the file there is from `offset`, by the size it was first said to be.
    fn left_from(&self, offset: u64) -> u64 {
        self.len.saturating_sub(offset)
    }
}

impl Read for RemoteFile {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let left = self.left_from(self.position);
        if buffer.is_empty() || left == 0 {
            return Ok(0);
        }
        let buffered_end = self.buffer_start + self.buffer.len() as u64;
        if self.position < self.buffer_start || self.position >= buffered_end {
            // Never past its end, and the client may send no more than is asked for.
            self.buffer = self
                .remote
                .read(&self.path, self.position, read_size(left))?;
            self.buffer_start = self.position;
            if self.buffer.is_empty() {
                // It is shorter than when it was opened. A snapshot does not change, so this is a
                // file the client could not read to its end.
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "the other PC's file ended early",
                ));
            }
        }
        let start = (self.position - self.buffer_start) as usize;
        let count = buffer.len().min(self.buffer.len() - start);
        buffer[..count].copy_from_slice(&self.buffer[start..start + count]);
        self.position += count as u64;
        Ok(count)
    }
}

impl Seek for RemoteFile {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let target = match position {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::End(offset) => self.len.checked_add_signed(offset),
            SeekFrom::Current(offset) => self.position.checked_add_signed(offset),
        };
        let target = target.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "a seek to before the start")
        })?;
        self.position = target;
        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_filetime_is_the_time_it_says() {
        // 2026-11-09 17:53:13 UTC is 134,072,239,930,000,000 hundred-nanoseconds after 1601.
        let seconds_after_1970 = 1_794_246_793;
        let filetime = (seconds_after_1970 + FILETIME_TO_UNIX_SECONDS) * FILETIME_STEPS_PER_SECOND;
        assert_eq!(
            time_of_filetime(filetime),
            UNIX_EPOCH.checked_add(Duration::from_secs(seconds_after_1970))
        );
        assert_eq!(time_of_filetime(0), None);
    }

    #[test]
    fn this_pcs_paths_read_as_std_reads_them() {
        let folder = std::env::temp_dir().join(format!("wam-source-fs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        let made =
            fs::create_dir_all(&folder).and_then(|()| fs::write(folder.join("a.txt"), b"abc"));
        if made.is_err() {
            return;
        }
        assert!(is_dir(&folder));
        assert!(is_file(folder.join("a.txt")));
        assert_eq!(
            metadata(folder.join("a.txt")).map(|m| m.len()).ok(),
            Some(3)
        );
        assert_eq!(
            read_at_most(folder.join("a.txt"), 3).ok().as_deref(),
            Some(&b"abc"[..])
        );
        assert!(read_at_most(folder.join("a.txt"), 2).is_err());
        assert_eq!(
            read(folder.join("a.txt")).ok().as_deref(),
            Some(&b"abc"[..])
        );
        let listed = read_dir(&folder).map(Iterator::count).ok();
        assert_eq!(listed, Some(1));
        assert!(!is_on_another_pc(&folder));
        let _ = fs::remove_dir_all(&folder);
    }
}
