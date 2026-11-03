use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::os::windows::fs::{FileExt, OpenOptionsExt};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use zip::ZipArchive;

use crate::errors::{Result, WinAppMigratorError, panic_text};
use crate::file_discovery::{reject_unsafe_components, safe_relative_path};
use crate::hashing::hex_lower;
use crate::models::InstallerStub;
use crate::packer::{PackItem, Packer, SourceRules};
use crate::progress::{BuildMonitor, InstallMonitor, InstallPhase, InstallStep};
use crate::space::MOST_INSTALLER_BYTES;
use crate::zip_stream::FinishedZip;

const SFX_MAGIC: &[u8] = b"WINAPPMIGRATOR_SFX_V2";
/// In the same place and of the same length, for a loader, whose package is the data file beside
/// it. The hash and the length before it are that file's, so what signs the loader signs them.
const LOADER_MAGIC: &[u8] = b"WINAPPMIGRATOR_LDR_V3";
const _: () = assert!(SFX_MAGIC.len() == LOADER_MAGIC.len());
const SFX_HASH_BYTES: usize = 32;
const SFX_LENGTH_BYTES: usize = 8;
/// After the package: its SHA-256, its length, then the magic. The stub and its package
/// always ship together, so the layout can change freely between versions.
const SFX_FOOTER_LEN: u64 = (SFX_HASH_BYTES + SFX_LENGTH_BYTES + SFX_MAGIC.len()) as u64;
/// What the data file of a loader is called, beside it, and with its name.
pub const DATA_FILE_EXTENSION: &str = "wamdata";
/// A data file on its way, until it is known whether it is one.
const PARTIAL_DATA_EXTENSION: &str = "wamdata.partial";
const PARTIAL_EXTENSION: &str = "partial";
/// An earlier build's data file, for as long as it takes to put a new one in place.
const EARLIER_DATA_EXTENSION: &str = "wamdata.earlier";
/// Lets other programs read an installer while it is in use, but not change, rename, or
/// delete it.
const FILE_SHARE_READ: u32 = 0x0000_0001;
/// Opens whatever has the name, rather than what a link with that name points at.
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
const HASH_BUFFER_BYTES: usize = 1024 * 1024;
/// Unpacking is mostly waiting for the disk, and past this it mostly waits for itself.
const MOST_UNPACKING_THREADS: usize = 8;

const PE_HEADER_OFFSET_FIELD: u64 = 0x3C;
const PE_SIGNATURE: &[u8] = b"PE\0\0";
const PE_SIGNATURE_LEN: u64 = 4;
const COFF_HEADER_LEN: u64 = 20;
/// Offsets from the PE signature.
const PE_SECTION_COUNT_FIELD: u64 = 6;
const PE_OPTIONAL_HEADER_SIZE_FIELD: u64 = 20;
const SECTION_HEADER_LEN: u64 = 40;
const SECTION_RAW_SIZE_FIELD: u64 = 16;
const SECTION_RAW_POINTER_FIELD: u64 = 20;
const PE32_MAGIC: u16 = 0x10B;
const PE32_PLUS_MAGIC: u16 = 0x20B;
/// Offset of the PE checksum from the start of the optional header, in both PE formats.
const PE_CHECKSUM_FIELD: u64 = 64;
const PE_CHECKSUM_LEN: usize = 4;
const PE_DATA_DIRECTORY_ENTRY_LEN: usize = 8;
/// Offset of the security data directory from the start of the optional header.
const PE32_SECURITY_DIRECTORY: u64 = 128;
const PE32_PLUS_SECURITY_DIRECTORY: u64 = 144;
/// Signing tools align the certificate table to 8 bytes with zero padding.
const MAX_SIGNATURE_PADDING: u64 = 7;
/// Upper bound for a single file read out of a package into memory (the manifest).
const MAX_EMBEDDED_FILE_BYTES: u64 = 1024 * 1024 * 1024;

/// What the hash and size of everything in a package came out as, seen as it was read.
#[derive(Debug, Default)]
pub struct PackageDigests {
    by_name: HashMap<String, EntryDigest>,
}

/// One entry's part of [`PackageDigests`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryDigest {
    pub sha256: String,
    pub size: u64,
}

impl PackageDigests {
    /// What was seen of the file a manifest calls by this path, as it would be placed.
    pub fn get(&self, relative: &Path) -> Option<&EntryDigest> {
        self.by_name.get(&zip_name_of(relative).to_lowercase())
    }
}

/// The name in a zip of a path relative to a package's root.
pub(crate) fn zip_name_of(relative: &Path) -> String {
    let safe = safe_relative_path(relative);
    let parts: Vec<String> = safe
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect();
    parts.join("/")
}

/// What is in a package folder, for putting in a zip. `expected` says what those that were copied
/// in earlier have to hash to, by their lowercased names.
pub(crate) fn package_folder_items(
    package_root: &Path,
    expected: &HashMap<String, String>,
) -> Result<Vec<PackItem>> {
    let mut items = Vec::new();
    for entry in WalkDir::new(package_root).follow_links(false) {
        let entry = entry.map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        if entry.path() == package_root || entry.file_type().is_dir() {
            continue;
        }
        let name = zip_entry_name(package_root, entry.path())?;
        let size_hint = entry
            .metadata()
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
            .len();
        items.push(PackItem {
            expected_sha256: expected.get(&name.to_lowercase()).cloned(),
            name,
            source: entry.path().to_path_buf(),
            size_hint,
        });
    }
    Ok(items)
}

fn nothing_to_ask(_: &Path) -> Result<()> {
    Ok(())
}

/// The rules for what is already in a package's own folder, which nothing else writes to.
pub(crate) const HELD_FILES: SourceRules<'static> = SourceRules {
    before: &nothing_to_ask,
    after: &nothing_to_ask,
    connected_drive: false,
    spool: None,
};

pub fn create_zip_archive(
    package_root: &Path,
    archive_path: &Path,
    monitor: &BuildMonitor,
) -> Result<()> {
    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    let file =
        File::create(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    let written = pack_folder(package_root, file, monitor);
    if written.is_err() {
        let _ = fs::remove_file(archive_path);
    }
    written
}

fn pack_folder(package_root: &Path, file: File, monitor: &BuildMonitor) -> Result<()> {
    let items = package_folder_items(package_root, &HashMap::new())?;
    let mut packer = Packer::new(file, 0, monitor.clone());
    packer.expect(&items);
    for outcome in packer.pack(&items, &HELD_FILES)? {
        outcome.map_err(WinAppMigratorError::Archive)?;
    }
    packer.finish()?;
    Ok(())
}

pub fn extract_zip_archive(archive_path: &Path, destination_root: &Path) -> Result<()> {
    fs::create_dir_all(destination_root)
        .map_err(|error| WinAppMigratorError::io(destination_root, error))?;
    let file =
        File::open(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    let len = file
        .metadata()
        .map_err(|error| WinAppMigratorError::io(archive_path, error))?
        .len();
    extract_zip_reader(
        ArchiveSection::new(Arc::new(file), 0, len),
        destination_root,
        &InstallMonitor::default(),
    )?;
    Ok(())
}

/// Unpack a zip, reporting each file, and hashing each as it goes by. Only plain names below the
/// destination get through.
fn extract_zip_reader<R: Read + Seek + Clone + Send>(
    reader: R,
    destination_root: &Path,
    monitor: &InstallMonitor,
) -> Result<PackageDigests> {
    monitor.checkpoint()?;
    read_every_entry(
        reader,
        Some(destination_root),
        &|done, total| {
            monitor.report(InstallStep::new(InstallPhase::Unpacking, done, total));
        },
        &|| monitor.is_cancelled(),
    )
}

/// Read everything in a zip on as many threads as make sense, each with its own place in it and a
/// buffer of its own, hashing what comes out as it comes out. With `destination`, that is also what
/// is written there, and without, it goes nowhere. It stops at the first thing that fails.
fn read_every_entry<R: Read + Seek + Clone + Send>(
    reader: R,
    destination: Option<&Path>,
    progress: &(dyn Fn(usize, usize) + Sync),
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<PackageDigests> {
    let archive =
        ZipArchive::new(reader).map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
    let total = archive.len();
    // Under its lock, so that what is said comes in order.
    let done = Mutex::new(0_usize);
    progress(0, total);
    let next = AtomicUsize::new(0);
    let stopping = AtomicBool::new(false);
    let digests = Mutex::new(HashMap::with_capacity(total));
    let failure: Mutex<Option<WinAppMigratorError>> = Mutex::new(None);
    let threads = crate::packer::worker_count()
        .min(MOST_UNPACKING_THREADS)
        .min(total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            let mut archive = archive.clone();
            let (next, stopping, digests, failure, done) =
                (&next, &stopping, &digests, &failure, &done);
            scope.spawn(move || {
                while !stopping.load(Ordering::Relaxed) {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    if index >= total {
                        return;
                    }
                    // A panic in reading a package someone else made goes no further than this. It
                    // would take the thread with it, and a window would wait for that thread forever.
                    let outcome = catch_unwind(AssertUnwindSafe(|| {
                        read_one_entry(&mut archive, index, destination, is_cancelled)
                    }))
                    .unwrap_or_else(|panic| {
                        Err(WinAppMigratorError::Archive(format!(
                            "entry {index} of the package could not be read: {}",
                            panic_text(panic.as_ref())
                        )))
                    });
                    match outcome {
                        Ok(Some((name, digest))) => {
                            if let Ok(mut digests) = digests.lock() {
                                digests.insert(name, digest);
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            stopping.store(true, Ordering::Relaxed);
                            if let Ok(mut failure) = failure.lock() {
                                failure.get_or_insert(error);
                            }
                            return;
                        }
                    }
                    if let Ok(mut done) = done.lock() {
                        *done += 1;
                        progress(*done, total);
                    }
                }
            });
        }
    });
    if let Some(error) = failure
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
    {
        return Err(error);
    }
    Ok(PackageDigests {
        by_name: digests
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    })
}

fn read_one_entry<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    index: usize,
    destination_root: Option<&Path>,
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<Option<(String, EntryDigest)>> {
    let mut file = archive
        .by_index(index)
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
    // Not `enclosed_name`, which up to at least zip 9.2.2 panics when a name's third byte
    // falls inside a character, as in `Größe.txt`. It also refuses `aux.h`, which a scan
    // rightly copies, so one such file would stop the whole install. These checks are
    // what it was for: only plain names below the destination get through.
    let name = file
        .name()
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
        .into_owned();
    if name.contains('\0') {
        return Err(WinAppMigratorError::Archive(format!(
            "zip entry has unsafe path: {name}"
        )));
    }
    let relative = Path::new(&name);
    reject_unsafe_components(relative)
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
    let destination = destination_root.map(|root| root.join(relative));

    if file.is_dir() {
        if let Some(destination) = destination.as_deref() {
            fs::create_dir_all(destination)
                .map_err(|error| WinAppMigratorError::io(destination, error))?;
        }
        return Ok(None);
    }
    let mut output = match destination.as_deref() {
        Some(destination) => {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| WinAppMigratorError::io(parent, error))?;
            }
            // Extraction goes into a new, empty folder, so anything already at this path was
            // not put there by this package, and opening it could follow a link.
            Some(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(destination)
                    .map_err(|error| WinAppMigratorError::io(destination, error))?,
            )
        }
        None => None,
    };
    // What the room for this was worked out from is what the entry says it holds, and a package
    // can say anything.
    let declared = file.size();
    let mut hasher = Sha256::new();
    let mut size = 0_u64;
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    loop {
        if is_cancelled() {
            return Err(WinAppMigratorError::Cancelled);
        }
        let read = file
            .read(&mut buffer)
            .map_err(|error| WinAppMigratorError::Archive(format!("{name}: {error}")))?;
        if read == 0 {
            break;
        }
        size += read as u64;
        if size > declared {
            return Err(WinAppMigratorError::Archive(format!(
                "{name} holds more than it says it does"
            )));
        }
        hasher.update(&buffer[..read]);
        if let (Some(output), Some(destination)) = (output.as_mut(), destination.as_deref()) {
            output
                .write_all(&buffer[..read])
                .map_err(|error| WinAppMigratorError::io(destination, error))?;
        }
    }
    Ok(Some((
        name.to_lowercase(),
        EntryDigest {
            sha256: hex_lower(&hasher.finalize()),
            size,
        },
    )))
}

pub fn create_self_extracting_archive(
    stub: &InstallerStub,
    archive_path: &Path,
    output_path: &Path,
) -> Result<()> {
    refuse_signed_stub(stub)?;
    let archive_len = fs::metadata(archive_path)
        .map_err(|error| WinAppMigratorError::io(archive_path, error))?
        .len();
    refuse_oversized_installer(stub_len(stub)? + archive_len + SFX_FOOTER_LEN)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    // Build beside the destination and rename on success, so a failed build never
    // destroys an installer from an earlier run.
    let partial_path = output_path.with_extension(PARTIAL_EXTENSION);
    let result = write_installer(stub, archive_path, &partial_path).and_then(|()| {
        fs::rename(&partial_path, output_path)
            .map_err(|error| WinAppMigratorError::io(output_path, error))
    });
    if result.is_err() {
        let _ = fs::remove_file(&partial_path);
    }
    result
}

/// [`create_self_extracting_archive`], as a small loader and a data file beside it, whatever the
/// size. A build does this by itself when a package has to go that way. Returns the data file.
pub fn create_installer_with_data_file(
    stub: &InstallerStub,
    archive_path: &Path,
    output_path: &Path,
) -> Result<PathBuf> {
    refuse_signed_stub(stub)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    let data_path = data_file_beside(output_path);
    fs::copy(archive_path, &data_path)
        .map_err(|error| WinAppMigratorError::io(&data_path, error))?;
    let mut archive =
        File::open(&data_path).map_err(|error| WinAppMigratorError::io(&data_path, error))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut len = 0_u64;
    loop {
        let read = archive
            .read(&mut buffer)
            .map_err(|error| WinAppMigratorError::io(&data_path, error))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        len += read as u64;
    }
    let mut hash = [0_u8; SFX_HASH_BYTES];
    hash.copy_from_slice(hasher.finalize().as_slice());
    let partial_path = output_path.with_extension(PARTIAL_EXTENSION);
    let _ = fs::remove_file(&partial_path);
    let result = write_loader(stub, &hash, len, &partial_path).and_then(|()| {
        fs::rename(&partial_path, output_path)
            .map_err(|error| WinAppMigratorError::io(output_path, error))
    });
    if result.is_err() {
        let _ = fs::remove_file(&partial_path);
        let _ = fs::remove_file(&data_path);
    }
    result?;
    Ok(data_path)
}

/// Where a loader looks for its data file. Only ever here: beside it, with its name.
pub fn data_file_beside(installer: &Path) -> PathBuf {
    installer.with_extension(DATA_FILE_EXTENSION)
}

fn write_stub_to(stub: &InstallerStub, output: &mut File, output_path: &Path) -> Result<()> {
    match stub {
        InstallerStub::File(path) => {
            let mut source =
                File::open(path).map_err(|error| WinAppMigratorError::io(path, error))?;
            io::copy(&mut source, output)
                .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        }
        InstallerStub::Embedded(bytes) => {
            output
                .write_all(bytes)
                .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        }
    }
    Ok(())
}

fn write_footer(
    output: &mut File,
    output_path: &Path,
    hash: &[u8; SFX_HASH_BYTES],
    len: u64,
    magic: &[u8],
) -> Result<()> {
    for part in [hash.as_slice(), &len.to_le_bytes(), magic] {
        output
            .write_all(part)
            .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    }
    Ok(())
}

/// A new file nothing else can get at while it is being written. Anything left at that name by a
/// run that died goes first, and a link there goes as a link.
fn new_partial_file(path: &Path) -> Result<File> {
    let _ = fs::remove_file(path);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .share_mode(0)
        .open(path)
        .map_err(|error| WinAppMigratorError::io(path, error))
}

fn write_loader(
    stub: &InstallerStub,
    hash: &[u8; SFX_HASH_BYTES],
    len: u64,
    output_path: &Path,
) -> Result<()> {
    let mut output = new_partial_file(output_path)?;
    write_stub_to(stub, &mut output, output_path)?;
    write_footer(&mut output, output_path, hash, len, LOADER_MAGIC)
}

fn write_installer(stub: &InstallerStub, archive_path: &Path, output_path: &Path) -> Result<()> {
    let mut output = new_partial_file(output_path)?;
    write_stub_to(stub, &mut output, output_path)?;
    let mut archive =
        File::open(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    let (hash, archive_len) = copy_hashing(
        &mut archive,
        &mut output,
        archive_path,
        output_path,
        &|| false,
    )?;
    write_footer(&mut output, output_path, &hash, archive_len, SFX_MAGIC)
}

/// Copy all that is left of one file to another, saying what it hashed to and how long it was.
fn copy_hashing(
    source: &mut File,
    output: &mut File,
    source_path: &Path,
    output_path: &Path,
    is_cancelled: &dyn Fn() -> bool,
) -> Result<([u8; SFX_HASH_BYTES], u64)> {
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut len = 0_u64;
    loop {
        if is_cancelled() {
            return Err(WinAppMigratorError::Cancelled);
        }
        let read = source
            .read(&mut buffer)
            .map_err(|error| WinAppMigratorError::io(source_path, error))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        output
            .write_all(&buffer[..read])
            .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        len += read as u64;
    }
    let mut hash = [0_u8; SFX_HASH_BYTES];
    hash.copy_from_slice(hasher.finalize().as_slice());
    Ok((hash, len))
}

/// An installer on its way out, whichever form it ends up in. What is being written is beside where
/// it goes, under a name of its own, and goes if this does. So a failed or cancelled build leaves
/// nothing behind, and an installer from an earlier run stays until there is one to replace it.
pub(crate) struct PendingInstaller {
    output_path: PathBuf,
    partial_path: PathBuf,
    /// Whether what is being written starts with the stub.
    stub_first: bool,
    stub_bytes: u64,
    /// Nothing is left to clear up once this is false.
    armed: bool,
    also_remove: Option<PathBuf>,
}

/// What a finished build came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WrittenInstaller {
    pub installer: PathBuf,
    pub data_file: Option<PathBuf>,
}

impl PendingInstaller {
    /// Start writing. A package sure to fit in one executable goes straight after the stub. One
    /// that may not is written by itself, since a file cannot lose what it starts with. Returns
    /// the file to write the package to and how much is in it already.
    pub(crate) fn start(
        stub: &InstallerStub,
        output_path: &Path,
        surely_one_file: bool,
    ) -> Result<(Self, File, u64)> {
        refuse_signed_stub(stub)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
        }
        let partial_path = output_path.with_extension(if surely_one_file {
            PARTIAL_EXTENSION
        } else {
            PARTIAL_DATA_EXTENSION
        });
        let mut pending = Self {
            output_path: output_path.to_path_buf(),
            partial_path: partial_path.clone(),
            stub_first: surely_one_file,
            stub_bytes: stub_len(stub)?,
            armed: true,
            also_remove: None,
        };
        let mut file = new_partial_file(&partial_path)?;
        let mut base = 0;
        if surely_one_file {
            write_stub_to(stub, &mut file, &partial_path)?;
            base = pending.stub_bytes;
        }
        pending.armed = true;
        Ok((pending, file, base))
    }

    /// Finish it, in whichever form it fits.
    pub(crate) fn complete(
        mut self,
        stub: &InstallerStub,
        finished: FinishedZip,
        monitor: &BuildMonitor,
    ) -> Result<WrittenInstaller> {
        let FinishedZip {
            mut file,
            len,
            sha256,
        } = finished;
        let total = self
            .stub_bytes
            .saturating_add(len)
            .saturating_add(SFX_FOOTER_LEN);
        let data_path = data_file_beside(&self.output_path);
        if self.stub_first {
            refuse_oversized_installer(total)?;
            write_footer(&mut file, &self.partial_path, &sha256, len, SFX_MAGIC)?;
            drop(file);
            self.put_in_place(&self.partial_path.clone())?;
            remove_earlier_data_files(&data_path);
            return Ok(self.written(None));
        }
        let exe_partial = self.output_path.with_extension(PARTIAL_EXTENSION);
        self.also_remove = Some(exe_partial.clone());
        let mut exe = new_partial_file(&exe_partial)?;
        write_stub_to(stub, &mut exe, &exe_partial)?;
        if fits_in_one_file(total) {
            // It came down far enough after all, so it goes after the stub, as usual.
            file.seek(SeekFrom::Start(0))
                .map_err(|error| WinAppMigratorError::io(&self.partial_path, error))?;
            let (copied, copied_len) = copy_hashing(
                &mut file,
                &mut exe,
                &self.partial_path,
                &exe_partial,
                &|| monitor.is_cancelled(),
            )?;
            if copied != sha256 || copied_len != len {
                return Err(WinAppMigratorError::Archive(
                    "the package changed while it was being put after the stub, so no installer \
                     was written"
                        .to_owned(),
                ));
            }
            write_footer(&mut exe, &exe_partial, &sha256, len, SFX_MAGIC)?;
            drop(exe);
            drop(file);
            self.put_in_place(&exe_partial)?;
            let _ = fs::remove_file(&self.partial_path);
            remove_earlier_data_files(&data_path);
            return Ok(self.written(None));
        }
        write_footer(&mut exe, &exe_partial, &sha256, len, LOADER_MAGIC)?;
        drop(exe);
        drop(file);
        self.put_both_in_place(&exe_partial, &data_path)?;
        Ok(self.written(Some(data_path)))
    }

    /// Put a loader and its data file where they go. An earlier data file stays, under another name,
    /// until the new loader is in place, and comes back if it cannot be, so a build that fails here
    /// does not leave an earlier loader with nothing to load. A run that dies in between can leave it
    /// under that other name, and the next build of this name clears it.
    fn put_both_in_place(&self, exe_partial: &Path, data_path: &Path) -> Result<()> {
        let earlier = data_path.with_extension(EARLIER_DATA_EXTENSION);
        let _ = fs::remove_file(&earlier);
        let has_earlier = fs::symlink_metadata(data_path).is_ok();
        if has_earlier {
            fs::rename(data_path, &earlier)
                .map_err(|error| WinAppMigratorError::io(data_path, error))?;
        }
        let put_back = |error: WinAppMigratorError| {
            if has_earlier {
                let _ = fs::rename(&earlier, data_path);
            }
            error
        };
        fs::rename(&self.partial_path, data_path)
            .map_err(|error| put_back(WinAppMigratorError::io(data_path, error)))?;
        if let Err(error) = self.put_in_place(exe_partial) {
            let _ = fs::remove_file(data_path);
            return Err(put_back(error));
        }
        if has_earlier {
            let _ = fs::remove_file(&earlier);
        }
        Ok(())
    }

    fn put_in_place(&self, partial: &Path) -> Result<()> {
        fs::rename(partial, &self.output_path)
            .map_err(|error| WinAppMigratorError::io(&self.output_path, error))
    }

    fn written(&mut self, data_file: Option<PathBuf>) -> WrittenInstaller {
        self.armed = false;
        WrittenInstaller {
            installer: self.output_path.clone(),
            data_file,
        }
    }
}

/// What is left of an earlier build of this name, which would only say this installer needs it.
fn remove_earlier_data_files(data_path: &Path) {
    let _ = fs::remove_file(data_path);
    let _ = fs::remove_file(data_path.with_extension(EARLIER_DATA_EXTENSION));
}

impl Drop for PendingInstaller {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_file(&self.partial_path);
            if let Some(path) = self.also_remove.take() {
                let _ = fs::remove_file(path);
            }
        }
    }
}

/// The package inside an installer, or in the data file beside it: found, checked against the hash
/// in the installer's footer before any of it is parsed, and held open. While this exists no other
/// process can change, rename, or replace either file, so what is read later is what was checked.
/// An installer usually sits in a folder anyone can write to, and it runs elevated.
///
/// The hash catches a truncated copy or an edited manifest. It does not prove who made
/// the package, since whoever edits one can also update its footer. Only signing the
/// finished installer does that. It covers the footer, so it covers a data file through its hash.
#[derive(Debug)]
pub struct EmbeddedPackage {
    file: File,
    /// The data file beside a loader. Without one, the package is in `file`.
    data_file: Option<File>,
    location: ArchiveLocation,
    /// What was seen of each file, as it was last unpacked to the folder named.
    unpacked: Mutex<Option<(PathBuf, Arc<PackageDigests>)>>,
}

impl EmbeddedPackage {
    /// Open an installer's package. `Ok(None)` for a bare stub, which carries none. This
    /// reads the whole package, so a large installer takes a moment.
    pub fn open(sfx_executable: &Path) -> Result<Option<Self>> {
        Self::open_unless(sfx_executable, &|| false)
    }

    /// [`Self::open`], giving up with [`WinAppMigratorError::Cancelled`] if asked to.
    pub fn open_unless(
        sfx_executable: &Path,
        is_cancelled: &dyn Fn() -> bool,
    ) -> Result<Option<Self>> {
        let mut file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(sfx_executable)
            .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
        let Some(location) = locate_embedded_archive(&mut file)
            .map_err(|error| WinAppMigratorError::io(sfx_executable, error))??
        else {
            return Ok(None);
        };
        let data_file = match location.form {
            PackageForm::InTheInstaller => {
                check_package_hash(&mut file, location, is_cancelled)
                    .map_err(|error| WinAppMigratorError::io(sfx_executable, error))??;
                None
            }
            PackageForm::BesideTheInstaller => Some(open_data_file(
                &data_file_beside(sfx_executable),
                location,
                is_cancelled,
            )?),
        };
        Ok(Some(Self {
            file,
            data_file,
            location,
            unpacked: Mutex::new(None),
        }))
    }

    fn section(&self) -> Result<ArchiveSection> {
        let file = self
            .data_file
            .as_ref()
            .unwrap_or(&self.file)
            .try_clone()
            .map_err(|error| WinAppMigratorError::io("installer package", error))?;
        Ok(ArchiveSection::new(
            Arc::new(file),
            self.location.start,
            self.location.len,
        ))
    }

    /// How much room everything in the package takes once unpacked, by what its entries say. An
    /// entry can only hold as much as it says: see [`read_one_entry`].
    pub fn unpacked_bytes(&self) -> Result<u64> {
        let archive = ZipArchive::new(BufReader::new(self.section()?))
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let mut total = 0_u64;
        for index in 0..archive.len() {
            let entry = archive
                .by_index_data(index)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
            total = total.saturating_add(entry.size());
        }
        Ok(total)
    }

    /// How big the stub in this installer is, for the copy of it an install keeps.
    pub fn stub_bytes(&self) -> u64 {
        self.location.stub_len
    }

    /// Unpack the package into `destination_root`.
    pub fn extract_to(&self, destination_root: &Path) -> Result<()> {
        self.extract_monitored(destination_root, &InstallMonitor::default())
    }

    /// Unpack the package, reporting each file and stopping with
    /// [`WinAppMigratorError::Cancelled`] if it is cancelled. What each file hashed to is seen as
    /// it comes out, and kept, so that an install does not have to read them again to check them.
    pub fn extract_monitored(
        &self,
        destination_root: &Path,
        monitor: &InstallMonitor,
    ) -> Result<()> {
        fs::create_dir_all(destination_root)
            .map_err(|error| WinAppMigratorError::io(destination_root, error))?;
        let digests = extract_zip_reader(self.section()?, destination_root, monitor)?;
        if let Ok(mut unpacked) = self.unpacked.lock() {
            *unpacked = Some((destination_root.to_path_buf(), Arc::new(digests)));
        }
        Ok(())
    }

    /// What was seen of each file when the package was unpacked to this folder, if it was.
    pub fn digests_of_unpacked(&self, package_root: &Path) -> Option<Arc<PackageDigests>> {
        let unpacked = self.unpacked.lock().ok()?;
        let (folder, digests) = unpacked.as_ref()?;
        (folder == package_root).then(|| Arc::clone(digests))
    }

    /// Read everything in the package and write it nowhere, saying what each file hashed to.
    pub fn digests(&self, is_cancelled: &(dyn Fn() -> bool + Sync)) -> Result<PackageDigests> {
        read_every_entry(self.section()?, None, &|_, _| {}, is_cancelled)
    }

    /// Write the stub this installer was built from, with no package and no signature, to a
    /// new file. See [`bare_stub_image`].
    pub fn write_bare_stub(&self, destination: &Path) -> Result<()> {
        let mut file = self
            .file
            .try_clone()
            .map_err(|error| WinAppMigratorError::io("installer package", error))?;
        let image = read_stub_image(&mut file, self.location.stub_len)
            .map_err(|error| WinAppMigratorError::io("installer package", error))?;
        let image = bare_stub_image(image)
            .map_err(|error| WinAppMigratorError::io("installer package", error))??;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)
            .map_err(|error| WinAppMigratorError::io(destination, error))?;
        output
            .write_all(&image)
            .map_err(|error| WinAppMigratorError::io(destination, error))?;
        Ok(())
    }

    /// Read one file without extracting the rest.
    pub fn read_file(&self, name: &str) -> Result<Vec<u8>> {
        let mut archive = ZipArchive::new(BufReader::new(self.section()?))
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let entry = archive
            .by_name(name)
            .map_err(|error| WinAppMigratorError::Archive(format!("{name}: {error}")))?;
        let mut bytes = Vec::new();
        entry
            .take(MAX_EMBEDDED_FILE_BYTES)
            .read_to_end(&mut bytes)
            .map_err(|error| WinAppMigratorError::Archive(format!("{name}: {error}")))?;
        Ok(bytes)
    }
}

/// Whether an installer carries a package, without reading all of it.
pub fn has_embedded_self_archive(sfx_executable: &Path) -> Result<bool> {
    let mut file = File::open(sfx_executable)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    let location = locate_embedded_archive(&mut file)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))??;
    Ok(location.is_some())
}

fn read_stub_image<F: Read + Seek>(file: &mut F, len: u64) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(0))?;
    let mut image = Vec::new();
    file.take(len).read_to_end(&mut image)?;
    Ok(image)
}

/// Turn the part of an installer before its package back into the bare stub it was built
/// from. **A bare stub must never carry a valid signature.** One would let anyone hide a
/// package in its certificate table and have it run elevated under that name, as
/// [`refuse_signed_stub`] explains. An installer may have been signed by whoever built it, and
/// signing leaves two marks in the headers before the package: the table's location, and
/// the checksum. With those cleared and everything after the program's own sections cut off,
/// the certificate table is gone, and with it any signature. An image that does not come
/// out that way is refused, not written.
fn bare_stub_image(mut image: Vec<u8>) -> io::Result<Result<Vec<u8>>> {
    let not_a_stub = || {
        WinAppMigratorError::InvalidInput(
            "the part of this installer before its package is not a program".to_owned(),
        )
    };
    let len = image.len() as u64;
    let mut reader = io::Cursor::new(&image);
    let Some(pe_offset) = pe_header(&mut reader, len)? else {
        return Ok(Err(not_a_stub()));
    };
    let Some(image_end) = pe_image_end(&mut reader, len)? else {
        return Ok(Err(not_a_stub()));
    };
    let optional_header = pe_offset + PE_SIGNATURE_LEN + COFF_HEADER_LEN;
    let directory = match read_u16_at(&mut reader, len, optional_header)? {
        Some(PE32_MAGIC) => optional_header + PE32_SECURITY_DIRECTORY,
        Some(PE32_PLUS_MAGIC) => optional_header + PE32_PLUS_SECURITY_DIRECTORY,
        _ => return Ok(Err(not_a_stub())),
    };
    let checksum = optional_header + PE_CHECKSUM_FIELD;
    if image_end > len
        || directory + PE_DATA_DIRECTORY_ENTRY_LEN as u64 > image_end
        || checksum + PE_CHECKSUM_LEN as u64 > image_end
    {
        return Ok(Err(not_a_stub()));
    }
    // Fits, by the check above.
    let (Ok(image_end), Ok(directory), Ok(checksum)) = (
        usize::try_from(image_end),
        usize::try_from(directory),
        usize::try_from(checksum),
    ) else {
        return Ok(Err(not_a_stub()));
    };
    image.truncate(image_end);
    image[checksum..checksum + PE_CHECKSUM_LEN].fill(0);
    image[directory..directory + PE_DATA_DIRECTORY_ENTRY_LEN].fill(0);
    let mut reader = io::Cursor::new(&image);
    if certificate_table(&mut reader, image_end as u64)?.is_some() {
        return Ok(Err(not_a_stub()));
    }
    Ok(Ok(image))
}

/// Whether a file carries an Authenticode signature at all, valid or not, which can be
/// answered at once. Checking the signature itself takes as long as hashing the file.
pub fn has_certificate_table(path: &Path) -> Result<bool> {
    let mut file = File::open(path).map_err(|error| WinAppMigratorError::io(path, error))?;
    let len = file
        .metadata()
        .map_err(|error| WinAppMigratorError::io(path, error))?
        .len();
    let table =
        certificate_table(&mut file, len).map_err(|error| WinAppMigratorError::io(path, error))?;
    Ok(table.is_some())
}

/// Compare the package against the SHA-256 in its footer.
fn check_package_hash<F: Read + Seek>(
    file: &mut F,
    location: ArchiveLocation,
    is_cancelled: &dyn Fn() -> bool,
) -> io::Result<Result<()>> {
    file.seek(SeekFrom::Start(location.start))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut remaining = location.len;
    while remaining > 0 {
        if is_cancelled() {
            return Ok(Err(WinAppMigratorError::Cancelled));
        }
        let want = usize::try_from(remaining).map_or(buffer.len(), |left| left.min(buffer.len()));
        let read = file.read(&mut buffer[..want])?;
        if read == 0 {
            return Ok(Err(damaged_installer()));
        }
        hasher.update(&buffer[..read]);
        remaining -= read as u64;
    }
    if hasher.finalize().as_slice() == location.sha256 {
        Ok(Ok(()))
    } else {
        Ok(Err(damaged_installer()))
    }
}

fn damaged_installer() -> WinAppMigratorError {
    WinAppMigratorError::Archive(
        "this installer is damaged or was changed after it was built, so nothing was read from \
         its package. Copy it again from where it was built."
            .to_owned(),
    )
}

/// Open a loader's data file and check it against the loader's footer, before anything in it is
/// parsed. It is looked for in one place, by one name, and whatever has that name is what gets
/// opened, not what a link there leads to. From here on it cannot be changed, renamed, or deleted,
/// and everything read from it goes through this one handle.
fn open_data_file(
    path: &Path,
    location: ArchiveLocation,
    is_cancelled: &dyn Fn() -> bool,
) -> Result<File> {
    let name = path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let mut file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => WinAppMigratorError::Archive(format!(
                "this installer keeps its package in a file named {name}, in the same folder, and \
                 that file is not there. The two files have to stay together: copy {name} here \
                 from where the installer was built, then run this again."
            )),
            _ => WinAppMigratorError::Archive(format!(
                "this installer's data file, {name}, could not be opened: {error}. The two files \
                 have to stay together, in a folder this PC can read."
            )),
        })?;
    let metadata = file
        .metadata()
        .map_err(|error| WinAppMigratorError::io(path, error))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(WinAppMigratorError::Archive(format!(
            "{name} is a link or a folder, and this installer's data file has to be a file of its \
             own, right beside it, so nothing was read from it. Copy the real file here."
        )));
    }
    let wrong_file = || {
        WinAppMigratorError::Archive(format!(
            "{name} is not this installer's data file, or it is damaged or was changed after it \
             was built, so nothing was read from it. Copy both files again from where they were \
             built."
        ))
    };
    if metadata.len() != location.len {
        return Err(wrong_file());
    }
    match check_package_hash(&mut file, location, is_cancelled)
        .map_err(|error| WinAppMigratorError::io(path, error))?
    {
        Ok(()) => Ok(file),
        Err(WinAppMigratorError::Cancelled) => Err(WinAppMigratorError::Cancelled),
        Err(_) => Err(wrong_file()),
    }
}

/// A signed stub must never become an installer. Windows does not hash its own
/// certificate table, so anyone holding a signed stub could hide a package there
/// and keep a valid signature, turning it into a trusted, elevated loader for
/// arbitrary content. Sign a finished installer instead, so the signature covers
/// that one payload.
fn refuse_signed_stub(stub: &InstallerStub) -> Result<()> {
    let table = match stub {
        InstallerStub::File(path) => {
            let mut file =
                File::open(path).map_err(|error| WinAppMigratorError::io(path, error))?;
            let len = file
                .metadata()
                .map_err(|error| WinAppMigratorError::io(path, error))?
                .len();
            certificate_table(&mut file, len)
                .map_err(|error| WinAppMigratorError::io(path, error))?
        }
        InstallerStub::Embedded(bytes) => {
            certificate_table(&mut io::Cursor::new(bytes), bytes.len() as u64)
                .map_err(|error| WinAppMigratorError::io("embedded installer stub", error))?
        }
    };
    if table.is_some() {
        return Err(WinAppMigratorError::InvalidInput(
            "the installer stub is signed; it must stay unsigned, so sign the finished \
             installer instead"
                .to_owned(),
        ));
    }
    Ok(())
}

fn stub_len(stub: &InstallerStub) -> Result<u64> {
    match stub {
        InstallerStub::File(path) => Ok(fs::metadata(path)
            .map_err(|error| WinAppMigratorError::io(path, error))?
            .len()),
        InstallerStub::Embedded(bytes) => Ok(bytes.len() as u64),
    }
}

/// Whether an installer of this size can be one executable, with room left to sign it.
fn fits_in_one_file(size: u64) -> bool {
    size <= MOST_INSTALLER_BYTES
}

/// Windows will not start an executable of 4 GiB or more ("not a valid Win32
/// application"). Reporting success for a file that can never run would only be
/// discovered on the target PC, possibly after the source is gone. A build goes past this with
/// a loader and a data file, so this is only for a package given to this already zipped, or
/// one that was sure to fit and has grown since it was scanned.
fn refuse_oversized_installer(size: u64) -> Result<()> {
    if fits_in_one_file(size) {
        return Ok(());
    }
    Err(WinAppMigratorError::InvalidInput(format!(
        "the installer would be {}, and Windows cannot run an executable of 4 GB or more; \
         nothing was written",
        crate::space::size_text(size)
    )))
}

/// Which file a package is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PackageForm {
    /// Right after the stub, in one executable.
    InTheInstaller,
    /// The data file beside a loader, all of it.
    BesideTheInstaller,
}

/// Where a package is, and what is said of it in its installer's footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArchiveLocation {
    /// Where it starts in the file it is in.
    start: u64,
    len: u64,
    sha256: [u8; SFX_HASH_BYTES],
    /// How much of the installer comes before anything added to the stub.
    stub_len: u64,
    form: PackageForm,
}

/// Find the embedded package. A signed installer keeps its package directly
/// before the certificate table, which is the only place a signature covers.
/// Bytes inside the table are never read, and bytes after it are refused. A loader keeps its
/// footer in the same place, and so the same goes for that.
fn locate_embedded_archive<F: Read + Seek>(
    file: &mut F,
) -> io::Result<Result<Option<ArchiveLocation>>> {
    let file_len = file.seek(SeekFrom::End(0))?;
    let end = match certificate_table(file, file_len)? {
        None => file_len,
        Some(table) => {
            if table.offset.saturating_add(table.size) != file_len {
                return Ok(Err(WinAppMigratorError::Archive(
                    "installer has data outside its signature and may have been tampered with"
                        .to_owned(),
                )));
            }
            table.offset - trailing_zero_padding(file, table.offset)?
        }
    };
    if end < SFX_FOOTER_LEN {
        return Ok(Ok(None));
    }

    let footer_start = end - SFX_FOOTER_LEN;
    file.seek(SeekFrom::Start(footer_start))?;
    let mut footer = [0_u8; SFX_FOOTER_LEN as usize];
    file.read_exact(&mut footer)?;
    let (sha256, rest) = footer.split_at(SFX_HASH_BYTES);
    let (length_bytes, magic) = rest.split_at(SFX_LENGTH_BYTES);
    let form = if magic == SFX_MAGIC {
        PackageForm::InTheInstaller
    } else if magic == LOADER_MAGIC {
        PackageForm::BesideTheInstaller
    } else {
        // Data past the program with no footer means a package whose end was cut off, a
        // common result of an interrupted copy. Calling that a bare stub would be wrong.
        if pe_image_end(file, file_len)?.is_some_and(|image_end| end > image_end) {
            return Ok(Err(damaged_installer()));
        }
        return Ok(Ok(None));
    };

    let mut len = [0_u8; SFX_LENGTH_BYTES];
    len.copy_from_slice(length_bytes);
    let len = u64::from_le_bytes(len);
    let mut hash = [0_u8; SFX_HASH_BYTES];
    hash.copy_from_slice(sha256);
    Ok(Ok(Some(match form {
        PackageForm::InTheInstaller => {
            if len > footer_start {
                return Ok(Err(WinAppMigratorError::Archive(
                    "self-extracting archive footer length is invalid".to_owned(),
                )));
            }
            ArchiveLocation {
                start: footer_start - len,
                len,
                sha256: hash,
                stub_len: footer_start - len,
                form,
            }
        }
        PackageForm::BesideTheInstaller => ArchiveLocation {
            start: 0,
            len,
            sha256: hash,
            stub_len: footer_start,
            form,
        },
    })))
}

/// The offset of a well-formed PE's signature.
fn pe_header<F: Read + Seek>(file: &mut F, file_len: u64) -> io::Result<Option<u64>> {
    let Some(pe_offset) = read_u32_at(file, file_len, PE_HEADER_OFFSET_FIELD)? else {
        return Ok(None);
    };
    let pe_offset = u64::from(pe_offset);
    if pe_offset.saturating_add(PE_SIGNATURE_LEN) > file_len {
        return Ok(None);
    }
    file.seek(SeekFrom::Start(pe_offset))?;
    let mut signature = [0_u8; PE_SIGNATURE_LEN as usize];
    file.read_exact(&mut signature)?;
    Ok((signature == PE_SIGNATURE).then_some(pe_offset))
}

/// Where a PE's own sections stop, which is where anything appended to it starts.
fn pe_image_end<F: Read + Seek>(file: &mut F, file_len: u64) -> io::Result<Option<u64>> {
    let Some(pe_offset) = pe_header(file, file_len)? else {
        return Ok(None);
    };
    let (Some(section_count), Some(optional_header_size)) = (
        read_u16_at(file, file_len, pe_offset + PE_SECTION_COUNT_FIELD)?,
        read_u16_at(file, file_len, pe_offset + PE_OPTIONAL_HEADER_SIZE_FIELD)?,
    ) else {
        return Ok(None);
    };
    let sections = pe_offset + PE_SIGNATURE_LEN + COFF_HEADER_LEN + u64::from(optional_header_size);
    let mut image_end = None;
    for index in 0..u64::from(section_count) {
        let section = sections + index * SECTION_HEADER_LEN;
        let (Some(size), Some(pointer)) = (
            read_u32_at(file, file_len, section + SECTION_RAW_SIZE_FIELD)?,
            read_u32_at(file, file_len, section + SECTION_RAW_POINTER_FIELD)?,
        ) else {
            return Ok(None);
        };
        let section_end = u64::from(pointer) + u64::from(size);
        image_end = image_end.max(Some(section_end));
    }
    Ok(image_end)
}

/// An Authenticode certificate table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CertificateTable {
    offset: u64,
    size: u64,
}

/// Read the PE security data directory. Anything that is not a well-formed PE
/// has no certificate table.
fn certificate_table<F: Read + Seek>(
    file: &mut F,
    file_len: u64,
) -> io::Result<Option<CertificateTable>> {
    let Some(pe_offset) = pe_header(file, file_len)? else {
        return Ok(None);
    };

    let optional_header = pe_offset + PE_SIGNATURE_LEN + COFF_HEADER_LEN;
    let Some(magic) = read_u16_at(file, file_len, optional_header)? else {
        return Ok(None);
    };
    let directory = match magic {
        PE32_MAGIC => optional_header + PE32_SECURITY_DIRECTORY,
        PE32_PLUS_MAGIC => optional_header + PE32_PLUS_SECURITY_DIRECTORY,
        _ => return Ok(None),
    };
    let (Some(offset), Some(size)) = (
        read_u32_at(file, file_len, directory)?,
        read_u32_at(file, file_len, directory + 4)?,
    ) else {
        return Ok(None);
    };
    if offset == 0 || size == 0 {
        return Ok(None);
    }
    Ok(Some(CertificateTable {
        offset: u64::from(offset),
        size: u64::from(size),
    }))
}

fn read_u16_at<F: Read + Seek>(
    file: &mut F,
    file_len: u64,
    offset: u64,
) -> io::Result<Option<u16>> {
    if offset.saturating_add(2) > file_len {
        return Ok(None);
    }
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = [0_u8; 2];
    file.read_exact(&mut bytes)?;
    Ok(Some(u16::from_le_bytes(bytes)))
}

fn read_u32_at<F: Read + Seek>(
    file: &mut F,
    file_len: u64,
    offset: u64,
) -> io::Result<Option<u32>> {
    if offset.saturating_add(4) > file_len {
        return Ok(None);
    }
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = [0_u8; 4];
    file.read_exact(&mut bytes)?;
    Ok(Some(u32::from_le_bytes(bytes)))
}

/// Count the alignment padding directly before `end`, which signing tools add
/// ahead of the certificate table. The footer magic never ends in a zero byte.
fn trailing_zero_padding<F: Read + Seek>(file: &mut F, end: u64) -> io::Result<u64> {
    let window = end.min(MAX_SIGNATURE_PADDING);
    file.seek(SeekFrom::Start(end - window))?;
    let mut bytes = vec![0_u8; window as usize];
    file.read_exact(&mut bytes)?;
    Ok(bytes.iter().rev().take_while(|byte| **byte == 0).count() as u64)
}

/// A bounded view of the package inside a file, so it is read in place rather than buffered in
/// memory or copied out to a second file first. Each clone keeps its own place, and reads by where
/// that is, so any number can read the one open file at once.
#[derive(Clone)]
struct ArchiveSection {
    file: Arc<File>,
    start: u64,
    len: u64,
    position: u64,
}

impl ArchiveSection {
    fn new(file: Arc<File>, start: u64, len: u64) -> Self {
        Self {
            file,
            start,
            len,
            position: 0,
        }
    }
}

impl Read for ArchiveSection {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let remaining = self.len.saturating_sub(self.position);
        let limit = usize::try_from(remaining).map_or(buffer.len(), |left| left.min(buffer.len()));
        if limit == 0 {
            return Ok(0);
        }
        let read = self.file.seek_read(
            &mut buffer[..limit],
            self.start.saturating_add(self.position),
        )?;
        self.position += read as u64;
        Ok(read)
    }
}

impl Seek for ArchiveSection {
    fn seek(&mut self, target: SeekFrom) -> io::Result<u64> {
        let position = match target {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::Current(delta) => self.position.checked_add_signed(delta),
            SeekFrom::End(delta) => self.len.checked_add_signed(delta),
        }
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek outside the package"))?;
        self.position = position;
        Ok(position)
    }
}

fn zip_entry_name(root: &Path, path: &Path) -> Result<String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(WinAppMigratorError::Archive(format!(
                    "unsafe package path: {}",
                    path.display()
                )));
            }
        }
    }
    if parts.is_empty() {
        return Err(WinAppMigratorError::Archive(format!(
            "empty package path: {}",
            path.display()
        )));
    }
    Ok(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zip_stream::{WholeEntry, ZipMethod, ZipStream};
    use time::OffsetDateTime;
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    #[test]
    fn zip_round_trip_rejects_traversal_by_construction() -> Result<()> {
        let root = temp_test_dir("archive")?;
        let package_root = root.join("package");
        let extract_root = root.join("extract");
        fs::create_dir_all(package_root.join("files"))
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("manifest.json"), "{}")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("files").join("app.exe"), "test")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;

        let archive_path = root.join("package.zip");
        create_zip_archive(&package_root, &archive_path, &BuildMonitor::default())?;
        extract_zip_archive(&archive_path, &extract_root)?;

        assert!(extract_root.join("manifest.json").is_file());
        assert!(extract_root.join("files").join("app.exe").is_file());
        fs::remove_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        Ok(())
    }

    fn zip_holding(names: &[&str]) -> Result<Vec<u8>> {
        let mut bytes = io::Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(&mut bytes);
        for name in names {
            writer
                .start_file(*name, SimpleFileOptions::default())
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
            writer
                .write_all(b"x")
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        }
        writer
            .finish()
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        Ok(bytes.into_inner())
    }

    #[test]
    fn extraction_refuses_names_that_are_not_plain_files() -> Result<()> {
        for name in [
            "files/CON",
            "files/nul",
            "files/a.exe:stream",
            "files/.. ",
            "../outside.txt",
            "files/../../outside.txt",
            "/outside.txt",
            "C:/outside.txt",
            "C:outside.txt",
            r"\\server\share\outside.txt",
            "files/a?b.txt",
            "files/a\0b.txt",
        ] {
            let root =
                tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
            let zip = zip_holding(&[name])?;
            assert!(
                extract_zip_reader(
                    io::Cursor::new(zip),
                    root.path(),
                    &InstallMonitor::default()
                )
                .is_err(),
                "{name} should be refused"
            );
            assert_eq!(
                fs::read_dir(root.path())
                    .map_err(|error| WinAppMigratorError::io(root.path(), error))?
                    .filter_map(std::result::Result::ok)
                    .filter(|entry| entry.path().is_file())
                    .count(),
                0
            );
        }
        Ok(())
    }

    /// The zip crate panics on the first of these if its own name check gets to see it.
    #[test]
    fn extraction_keeps_names_that_are_not_ascii() -> Result<()> {
        let names = [
            "files/Größe.txt",
            "files/Grüße/Größe.txt",
            "files/a日本語.txt",
            "files/日本語.txt",
            "files/ab\u{1F600}.txt",
        ];
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let zip = zip_holding(&names)?;

        extract_zip_reader(
            io::Cursor::new(zip),
            root.path(),
            &InstallMonitor::default(),
        )?;

        for name in names {
            assert!(root.path().join(name).is_file(), "{name}");
        }
        Ok(())
    }

    #[test]
    fn extraction_never_opens_something_already_at_the_destination() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let planted = root.path().join("a.txt");
        fs::write(&planted, b"keep").map_err(|error| WinAppMigratorError::io(&planted, error))?;
        let zip = zip_holding(&["a.txt"])?;

        assert!(
            extract_zip_reader(
                io::Cursor::new(zip),
                root.path(),
                &InstallMonitor::default()
            )
            .is_err()
        );
        assert_eq!(
            fs::read(&planted).map_err(|error| WinAppMigratorError::io(&planted, error))?,
            b"keep"
        );
        Ok(())
    }

    /// The bytes of a minimal PE32+ image, with its security directory pointing
    /// at `certificate` when one is given.
    fn fake_pe(certificate: Option<CertificateTable>) -> Vec<u8> {
        const PE_OFFSET: usize = 0x80;
        const IMAGE_LEN: usize = 0x400;
        let mut bytes = vec![0_u8; IMAGE_LEN];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[PE_HEADER_OFFSET_FIELD as usize..PE_HEADER_OFFSET_FIELD as usize + 4]
            .copy_from_slice(&(PE_OFFSET as u32).to_le_bytes());
        bytes[PE_OFFSET..PE_OFFSET + 4].copy_from_slice(PE_SIGNATURE);
        let optional = PE_OFFSET + (PE_SIGNATURE_LEN + COFF_HEADER_LEN) as usize;
        bytes[optional..optional + 2].copy_from_slice(&PE32_PLUS_MAGIC.to_le_bytes());
        if let Some(table) = certificate {
            let directory = optional + PE32_PLUS_SECURITY_DIRECTORY as usize;
            bytes[directory..directory + 4].copy_from_slice(&(table.offset as u32).to_le_bytes());
            bytes[directory + 4..directory + 8].copy_from_slice(&(table.size as u32).to_le_bytes());
        }
        bytes
    }

    fn sha256_of(archive: &[u8]) -> [u8; SFX_HASH_BYTES] {
        let mut hash = [0_u8; SFX_HASH_BYTES];
        hash.copy_from_slice(Sha256::digest(archive).as_slice());
        hash
    }

    fn footer_for(archive: &[u8]) -> Vec<u8> {
        let mut footer = sha256_of(archive).to_vec();
        footer.extend_from_slice(&(archive.len() as u64).to_le_bytes());
        footer.extend_from_slice(SFX_MAGIC);
        footer
    }

    fn locate(bytes: Vec<u8>) -> Result<Option<ArchiveLocation>> {
        locate_embedded_archive(&mut io::Cursor::new(bytes))
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
    }

    #[test]
    fn unsigned_installer_finds_package_at_end_of_file() -> Result<()> {
        let payload = b"package";
        let mut bytes = fake_pe(None);
        let start = bytes.len() as u64;
        bytes.extend_from_slice(payload);
        bytes.extend_from_slice(&footer_for(payload));

        assert_eq!(
            locate(bytes)?,
            Some(ArchiveLocation {
                start,
                len: payload.len() as u64,
                sha256: sha256_of(payload),
                stub_len: start,
                form: PackageForm::InTheInstaller,
            })
        );
        Ok(())
    }

    #[test]
    fn installer_signed_after_build_still_finds_its_package() -> Result<()> {
        let payload = b"package";
        let image_len = fake_pe(None).len() as u64;
        let mut tail = payload.to_vec();
        tail.extend_from_slice(&footer_for(payload));
        // Signing tools zero-pad up to an 8-byte boundary before the table.
        let padding = (8 - (image_len + tail.len() as u64) % 8) % 8;
        tail.resize(tail.len() + padding as usize, 0);
        let table = CertificateTable {
            offset: image_len + tail.len() as u64,
            size: 64,
        };

        let mut bytes = fake_pe(Some(table));
        bytes.extend_from_slice(&tail);
        bytes.resize(bytes.len() + table.size as usize, 0xAB);

        assert_eq!(
            locate(bytes)?,
            Some(ArchiveLocation {
                start: image_len,
                len: payload.len() as u64,
                sha256: sha256_of(payload),
                stub_len: image_len,
                form: PackageForm::InTheInstaller,
            })
        );
        Ok(())
    }

    #[test]
    fn a_signed_installers_package_still_passes_its_hash_check() -> Result<()> {
        let payload = b"package";
        let image_len = fake_pe(None).len() as u64;
        let mut tail = payload.to_vec();
        tail.extend_from_slice(&footer_for(payload));
        let padding = (8 - (image_len + tail.len() as u64) % 8) % 8;
        tail.resize(tail.len() + padding as usize, 0);
        let table = CertificateTable {
            offset: image_len + tail.len() as u64,
            size: 64,
        };
        let mut bytes = fake_pe(Some(table));
        bytes.extend_from_slice(&tail);
        bytes.resize(bytes.len() + table.size as usize, 0xAB);

        let mut file = io::Cursor::new(bytes);
        let location = locate_embedded_archive(&mut file)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))??
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        check_package_hash(&mut file, location, &|| false)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))??;
        Ok(())
    }

    #[test]
    fn a_package_that_does_not_match_its_footer_is_refused() -> Result<()> {
        let mut bytes = fake_pe(None);
        bytes.extend_from_slice(b"edited!");
        bytes.extend_from_slice(&footer_for(b"package"));

        let mut file = io::Cursor::new(bytes);
        let location = locate_embedded_archive(&mut file)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))??
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let checked = check_package_hash(&mut file, location, &|| false)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        assert!(checked.is_err());
        Ok(())
    }

    /// A minimal PE with one section, as long as `image_len`.
    fn fake_pe_with_section(image_len: usize) -> Vec<u8> {
        const PE_OFFSET: usize = 0x80;
        const OPTIONAL_HEADER_SIZE: usize = 0xF0;
        let mut bytes = fake_pe(None);
        bytes.resize(image_len, 0);
        bytes[PE_OFFSET + PE_SECTION_COUNT_FIELD as usize] = 1;
        bytes[PE_OFFSET + PE_OPTIONAL_HEADER_SIZE_FIELD as usize] = OPTIONAL_HEADER_SIZE as u8;
        let section =
            PE_OFFSET + (PE_SIGNATURE_LEN + COFF_HEADER_LEN) as usize + OPTIONAL_HEADER_SIZE;
        let raw_size = section + SECTION_RAW_SIZE_FIELD as usize;
        bytes[raw_size..raw_size + 4].copy_from_slice(&((image_len - 0x400) as u32).to_le_bytes());
        let raw_pointer = section + SECTION_RAW_POINTER_FIELD as usize;
        bytes[raw_pointer..raw_pointer + 4].copy_from_slice(&0x400_u32.to_le_bytes());
        bytes
    }

    /// A minimal stub, with an installer built from it: package, footer, padding, and a
    /// certificate table, with the marks signing leaves in the headers.
    fn signed_installer_from(stub: &[u8]) -> (Vec<u8>, u64) {
        const PE_OFFSET: usize = 0x80;
        let payload = b"package";
        let mut tail = payload.to_vec();
        tail.extend_from_slice(&footer_for(payload));
        let padding = (8 - (stub.len() as u64 + tail.len() as u64) % 8) % 8;
        tail.resize(tail.len() + padding as usize, 0);
        let offset = stub.len() as u64 + tail.len() as u64;
        let mut bytes = stub.to_vec();
        let optional = PE_OFFSET + (PE_SIGNATURE_LEN + COFF_HEADER_LEN) as usize;
        let directory = optional + PE32_PLUS_SECURITY_DIRECTORY as usize;
        bytes[directory..directory + 4].copy_from_slice(&(offset as u32).to_le_bytes());
        bytes[directory + 4..directory + 8].copy_from_slice(&64_u32.to_le_bytes());
        let checksum = optional + PE_CHECKSUM_FIELD as usize;
        bytes[checksum..checksum + 4].copy_from_slice(&0x1234_5678_u32.to_le_bytes());
        bytes.extend_from_slice(&tail);
        bytes.resize(bytes.len() + 64, 0xAB);
        (bytes, stub.len() as u64)
    }

    fn carve(stub_part: Vec<u8>) -> Result<Vec<u8>> {
        bare_stub_image(stub_part)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
    }

    #[test]
    fn a_bare_stub_carved_from_a_signed_installer_has_no_signature_left() -> Result<()> {
        let stub = fake_pe_with_section(0x800);
        let (installer, start) = signed_installer_from(&stub);
        assert!(
            certificate_table(&mut io::Cursor::new(&installer), installer.len() as u64)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
                .is_some()
        );

        let carved = carve(
            read_stub_image(&mut io::Cursor::new(&installer), start)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?,
        )?;

        // Byte for byte what it was built from, signing's marks included.
        assert_eq!(carved, stub);
        assert_eq!(
            certificate_table(&mut io::Cursor::new(&carved), carved.len() as u64)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?,
            None
        );
        assert_eq!(locate(carved)?, None, "it reads as a bare stub");
        Ok(())
    }

    #[test]
    fn a_bare_stub_carved_from_an_unsigned_installer_is_what_it_was_built_from() -> Result<()> {
        let stub = fake_pe_with_section(0x800);
        assert_eq!(carve(stub.clone())?, stub);
        Ok(())
    }

    #[test]
    fn anything_past_the_program_does_not_survive_carving() -> Result<()> {
        let stub = fake_pe_with_section(0x800);
        let mut with_extra = stub.clone();
        with_extra.extend_from_slice(&[0xEE; 32]);
        assert_eq!(carve(with_extra)?, stub);
        Ok(())
    }

    #[test]
    fn what_is_not_a_program_is_not_carved() {
        assert!(carve(b"not a program at all".to_vec()).is_err());
        // Its sections claim to run past what is there.
        let mut cut_off = fake_pe_with_section(0x800);
        cut_off.truncate(0x500);
        assert!(carve(cut_off).is_err());
    }

    #[test]
    fn unpacking_reports_each_file_and_can_be_cancelled() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let zip = zip_holding(&["one.txt", "two.txt"])?;
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&seen);
        let monitor = InstallMonitor::new(move |step: InstallStep| {
            if let Ok(mut steps) = sink.lock() {
                steps.push((step.done, step.total));
            }
        });
        extract_zip_reader(io::Cursor::new(&zip), root.path(), &monitor)?;
        let steps = seen
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?
            .clone();
        assert_eq!(steps, [(0, 2), (1, 2), (2, 2)]);

        let other = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        monitor.cancel();
        assert!(matches!(
            extract_zip_reader(io::Cursor::new(&zip), other.path(), &monitor),
            Err(WinAppMigratorError::Cancelled)
        ));
        assert!(!other.path().join("one.txt").exists());
        Ok(())
    }

    #[test]
    fn a_bare_stub_is_not_mistaken_for_a_damaged_installer() -> Result<()> {
        assert_eq!(locate(fake_pe_with_section(0x800))?, None);
        Ok(())
    }

    #[test]
    fn a_copy_with_its_end_cut_off_is_called_damaged_not_a_bare_stub() {
        let payload = vec![7_u8; 4096];
        let mut bytes = fake_pe_with_section(0x800);
        bytes.extend_from_slice(&payload);
        bytes.extend_from_slice(&footer_for(&payload));
        bytes.truncate(bytes.len() - 100);

        assert!(locate(bytes).is_err());
    }

    #[test]
    fn package_hidden_inside_certificate_table_is_ignored() -> Result<()> {
        // Padding the certificate table keeps a signature valid, so a signed stub
        // must never load a package placed there.
        let payload = b"attacker package";
        let mut hidden = vec![0xAB_u8; 64];
        hidden.extend_from_slice(payload);
        hidden.extend_from_slice(&footer_for(payload));
        let image_len = fake_pe(None).len() as u64;

        let mut bytes = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: hidden.len() as u64,
        }));
        bytes.extend_from_slice(&hidden);

        assert_eq!(locate(bytes)?, None);
        Ok(())
    }

    #[test]
    fn package_appended_after_a_signature_is_refused() {
        let payload = b"attacker package";
        let image_len = fake_pe(None).len() as u64;
        let mut bytes = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: 64,
        }));
        bytes.resize(bytes.len() + 64, 0xAB);
        bytes.extend_from_slice(payload);
        bytes.extend_from_slice(&footer_for(payload));

        assert!(locate(bytes).is_err());
    }

    #[test]
    fn footer_longer_than_the_file_is_refused() {
        let mut bytes = fake_pe(None);
        bytes.extend_from_slice(&[0_u8; SFX_HASH_BYTES]);
        bytes.extend_from_slice(&u64::MAX.to_le_bytes());
        bytes.extend_from_slice(SFX_MAGIC);

        assert!(locate(bytes).is_err());
    }

    #[test]
    fn signed_stub_is_refused_as_an_installer_template() -> Result<()> {
        let root = temp_test_dir("signed-stub")?;
        fs::create_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        let image_len = fake_pe(None).len() as u64;
        let mut stub = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: 64,
        }));
        stub.resize(stub.len() + 64, 0xAB);
        let stub_path = root.join("stub.exe");
        fs::write(&stub_path, &stub).map_err(|error| WinAppMigratorError::io(&stub_path, error))?;
        let archive_path = root.join("package.zip");
        fs::write(&archive_path, b"zip")
            .map_err(|error| WinAppMigratorError::io(&archive_path, error))?;
        let output_path = root.join("installer.exe");

        let from_file = InstallerStub::File(stub_path);
        let result = create_self_extracting_archive(&from_file, &archive_path, &output_path);
        assert!(matches!(result, Err(WinAppMigratorError::InvalidInput(_))));
        assert!(!output_path.exists());

        let from_bytes = InstallerStub::Embedded(stub.leak());
        let result = create_self_extracting_archive(&from_bytes, &archive_path, &output_path);
        assert!(matches!(result, Err(WinAppMigratorError::InvalidInput(_))));
        assert!(!output_path.exists());
        fs::remove_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        Ok(())
    }

    #[test]
    fn self_extracting_round_trip_reads_the_package_in_place() -> Result<()> {
        let root = temp_test_dir("sfx-round-trip")?;
        let package_root = root.join("package");
        fs::create_dir_all(package_root.join("files"))
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("manifest.json"), "{}")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("files").join("app.exe"), "test")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        let archive_path = root.join("package.zip");
        create_zip_archive(&package_root, &archive_path, &BuildMonitor::default())?;
        let stub_path = root.join("stub.exe");
        fs::write(&stub_path, fake_pe(None))
            .map_err(|error| WinAppMigratorError::io(&stub_path, error))?;
        let installer_path = root.join("installer.exe");
        let stub = InstallerStub::File(stub_path.clone());
        create_self_extracting_archive(&stub, &archive_path, &installer_path)?;

        assert!(has_embedded_self_archive(&installer_path)?);
        assert!(!has_embedded_self_archive(&stub_path)?);
        assert!(!installer_path.with_extension("partial").exists());
        let package = EmbeddedPackage::open(&installer_path)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert_eq!(package.read_file("manifest.json")?, b"{}");
        drop(package);
        assert!(EmbeddedPackage::open(&stub_path)?.is_none());

        let extract_root = root.join("extract");
        EmbeddedPackage::open(&installer_path)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?
            .extract_to(&extract_root)?;
        assert!(extract_root.join("manifest.json").is_file());
        assert!(extract_root.join("files").join("app.exe").is_file());
        assert!(!extract_root.join("embedded-package.zip").exists());
        fs::remove_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        Ok(())
    }

    /// Build an installer in `root`, returning its path.
    fn installer_in(root: &Path) -> Result<PathBuf> {
        let package_root = root.join("package");
        fs::create_dir_all(&package_root)
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("manifest.json"), "{\"name\":\"app\"}")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        let archive_path = root.join("package.zip");
        create_zip_archive(&package_root, &archive_path, &BuildMonitor::default())?;
        let stub_path = root.join("stub.exe");
        fs::write(&stub_path, fake_pe(None))
            .map_err(|error| WinAppMigratorError::io(&stub_path, error))?;
        let installer_path = root.join("installer.exe");
        create_self_extracting_archive(
            &InstallerStub::File(stub_path),
            &archive_path,
            &installer_path,
        )?;
        Ok(installer_path)
    }

    #[test]
    fn an_edited_manifest_is_caught_before_it_is_read() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = installer_in(root.path())?;
        let mut bytes =
            fs::read(&installer).map_err(|error| WinAppMigratorError::io(&installer, error))?;
        // Stored compressed or not, this changes one byte of the package.
        let middle = fake_pe(None).len() + 10;
        bytes[middle] ^= 0xFF;
        fs::write(&installer, bytes).map_err(|error| WinAppMigratorError::io(&installer, error))?;

        let refused = EmbeddedPackage::open(&installer);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::Archive(message)) if message.contains("damaged")),
            "{refused:?}"
        );
        Ok(())
    }

    #[test]
    fn a_truncated_copy_is_caught_before_it_is_read() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = installer_in(root.path())?;
        let mut bytes =
            fs::read(&installer).map_err(|error| WinAppMigratorError::io(&installer, error))?;
        // Keep the footer but lose bytes before it, the case that still looks complete.
        let footer = bytes.split_off(bytes.len() - SFX_FOOTER_LEN as usize);
        bytes.truncate(bytes.len() - 20);
        bytes.extend_from_slice(&footer);
        fs::write(&installer, bytes).map_err(|error| WinAppMigratorError::io(&installer, error))?;

        assert!(EmbeddedPackage::open(&installer).is_err());
        Ok(())
    }

    #[test]
    fn nothing_can_change_or_replace_an_installer_while_it_is_held_open() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let installer = installer_in(root.path())?;
        let package = EmbeddedPackage::open(&installer)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;

        assert!(OpenOptions::new().write(true).open(&installer).is_err());
        assert!(fs::rename(&installer, root.path().join("moved.exe")).is_err());
        assert!(fs::remove_file(&installer).is_err());
        assert!(File::open(&installer).is_ok(), "others may still read it");
        assert_eq!(package.read_file("manifest.json")?, b"{\"name\":\"app\"}");

        drop(package);
        assert!(OpenOptions::new().write(true).open(&installer).is_ok());
        Ok(())
    }

    /// A real executable that is running, which is what an installer opens: its own file.
    /// It has nothing appended, so it reads as carrying no package.
    #[test]
    fn a_running_executable_can_be_held_open() -> Result<()> {
        let exe = std::env::current_exe()
            .map_err(|error| WinAppMigratorError::io("current executable", error))?;
        assert!(EmbeddedPackage::open(&exe)?.is_none());
        Ok(())
    }

    #[test]
    fn an_unsigned_file_has_no_certificate_table_and_a_signed_one_does() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let unsigned = root.path().join("unsigned.exe");
        fs::write(&unsigned, fake_pe(None))
            .map_err(|error| WinAppMigratorError::io(&unsigned, error))?;
        assert!(!has_certificate_table(&unsigned)?);

        let image_len = fake_pe(None).len() as u64;
        let mut signed_bytes = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: 64,
        }));
        signed_bytes.resize(signed_bytes.len() + 64, 0xAB);
        let signed = root.path().join("signed.exe");
        fs::write(&signed, signed_bytes)
            .map_err(|error| WinAppMigratorError::io(&signed, error))?;
        assert!(has_certificate_table(&signed)?);
        Ok(())
    }

    #[test]
    fn an_installer_windows_cannot_start_is_refused() {
        assert!(refuse_oversized_installer(MOST_INSTALLER_BYTES).is_ok());
        assert!(refuse_oversized_installer(MOST_INSTALLER_BYTES + 1).is_err());
        assert!(
            refuse_oversized_installer(4 * 1024 * 1024 * 1024 - 1).is_err(),
            "one that Windows could start, and signing it would then take past that"
        );
    }

    /// A folder with a stub and a zip in it that says what it has.
    struct Parts {
        stub: PathBuf,
        zip: PathBuf,
        output: PathBuf,
        root: tempfile::TempDir,
    }

    fn parts() -> Result<Parts> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let package = root.path().join("package");
        fs::create_dir_all(package.join("files"))
            .map_err(|error| WinAppMigratorError::io(&package, error))?;
        fs::write(package.join("manifest.json"), "{\"name\":\"app\"}")
            .map_err(|error| WinAppMigratorError::io(&package, error))?;
        fs::write(package.join("files").join("a.txt"), "the app")
            .map_err(|error| WinAppMigratorError::io(&package, error))?;
        let zip = root.path().join("package.zip");
        create_zip_archive(&package, &zip, &BuildMonitor::default())?;
        let stub = root.path().join("stub.exe");
        fs::write(&stub, fake_pe_with_section(0x800))
            .map_err(|error| WinAppMigratorError::io(&stub, error))?;
        Ok(Parts {
            stub,
            zip,
            output: root.path().join("App_installer.exe"),
            root,
        })
    }

    #[test]
    fn a_loader_is_its_stub_and_a_footer_and_finds_what_is_beside_it() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        assert_eq!(data, parts.root.path().join("App_installer.wamdata"));

        // It stays small: the stub, and the footer.
        let loader = fs::read(&parts.output).map_err(|error| WinAppMigratorError::io("", error))?;
        let stub = fs::read(&parts.stub).map_err(|error| WinAppMigratorError::io("", error))?;
        assert_eq!(loader.len() as u64, stub.len() as u64 + SFX_FOOTER_LEN);
        assert_eq!(loader[..stub.len()], stub);

        // The footer is what the data file really is.
        let bytes = fs::read(&data).map_err(|error| WinAppMigratorError::io("", error))?;
        let footer = &loader[stub.len()..];
        assert_eq!(footer[..SFX_HASH_BYTES], *Sha256::digest(&bytes));
        assert_eq!(
            footer[SFX_HASH_BYTES..SFX_HASH_BYTES + SFX_LENGTH_BYTES],
            (bytes.len() as u64).to_le_bytes()
        );
        assert_eq!(footer[SFX_HASH_BYTES + SFX_LENGTH_BYTES..], *LOADER_MAGIC);

        assert!(has_embedded_self_archive(&parts.output)?);
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert!(package.data_file.is_some());
        assert_eq!(package.stub_bytes(), stub.len() as u64);
        assert_eq!(package.read_file("manifest.json")?, b"{\"name\":\"app\"}");

        let unpacked = parts.root.path().join("unpacked");
        package.extract_to(&unpacked)?;
        assert_eq!(
            fs::read(unpacked.join(r"files\a.txt"))
                .map_err(|error| WinAppMigratorError::io("", error))?,
            b"the app"
        );
        Ok(())
    }

    #[test]
    fn a_missing_data_file_says_what_to_do() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        fs::remove_file(&data).map_err(|error| WinAppMigratorError::io(&data, error))?;
        let refused = EmbeddedPackage::open(&parts.output);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::Archive(message))
                if message.contains("App_installer.wamdata")
                    && message.contains("stay together")
                    && message.contains("same folder")),
            "{refused:?}"
        );
        // Knowing it has a package does not take reading any of it.
        assert!(has_embedded_self_archive(&parts.output)?);
        Ok(())
    }

    /// Everything in a file except one byte, or with a little more or less of it, is not it.
    #[test]
    fn a_data_file_that_is_not_the_one_it_was_built_with_is_refused() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        let good = fs::read(&data).map_err(|error| WinAppMigratorError::io(&data, error))?;
        let mut one_byte = good.clone();
        one_byte[good.len() / 2] ^= 0x01;
        let mut longer = good.clone();
        longer.push(0);
        let shorter = good[..good.len() - 1].to_vec();
        // Another package altogether, with its own hash right, for a different installer.
        let other_root = parts.root.path().join("other");
        fs::create_dir_all(&other_root)
            .map_err(|error| WinAppMigratorError::io(&other_root, error))?;
        fs::write(other_root.join("manifest.json"), "{\"name\":\"another\"}")
            .map_err(|error| WinAppMigratorError::io(&other_root, error))?;
        let other_zip = parts.root.path().join("other.zip");
        create_zip_archive(&other_root, &other_zip, &BuildMonitor::default())?;
        let other = fs::read(&other_zip).map_err(|error| WinAppMigratorError::io("", error))?;
        for (what, bytes) in [
            ("one byte changed", one_byte),
            ("longer", longer),
            ("shorter", shorter),
            ("another package", other),
            ("empty", Vec::new()),
        ] {
            fs::write(&data, &bytes).map_err(|error| WinAppMigratorError::io(&data, error))?;
            let refused = EmbeddedPackage::open(&parts.output);
            assert!(
                matches!(&refused, Err(WinAppMigratorError::Archive(message))
                    if message.contains("App_installer.wamdata")
                        && message.contains("nothing was read from it")),
                "{what}: {refused:?}"
            );
        }
        fs::write(&data, &good).map_err(|error| WinAppMigratorError::io(&data, error))?;
        assert!(EmbeddedPackage::open(&parts.output)?.is_some());
        Ok(())
    }

    #[test]
    fn a_data_file_is_not_looked_for_through_a_link() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        // The real, right file is what the link would lead to.
        let elsewhere = parts.root.path().join("elsewhere");
        fs::create_dir_all(&elsewhere).map_err(|error| WinAppMigratorError::io("", error))?;
        let real = elsewhere.join("App_installer.wamdata");
        fs::rename(&data, &real).map_err(|error| WinAppMigratorError::io(&data, error))?;
        let linked = crate::path_utils::hidden_command("cmd")
            .args(["/D", "/C", "mklink"])
            .arg(&data)
            .arg(&real)
            .output()
            .map_err(|error| WinAppMigratorError::io("mklink", error))?;
        if !linked.status.success() {
            // A symbolic link to a file takes a right most accounts do not have, and a folder is
            // the only kind of link they can make, which the test below covers.
            println!("skipping: {}", String::from_utf8_lossy(&linked.stderr));
            return Ok(());
        }
        let refused = EmbeddedPackage::open(&parts.output);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::Archive(message))
                if message.contains("is a link")),
            "{refused:?}"
        );
        Ok(())
    }

    #[test]
    fn a_folder_by_the_data_files_name_is_not_it() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        let good = fs::read(&data).map_err(|error| WinAppMigratorError::io(&data, error))?;
        fs::remove_file(&data).map_err(|error| WinAppMigratorError::io(&data, error))?;
        fs::create_dir(&data).map_err(|error| WinAppMigratorError::io(&data, error))?;
        fs::write(data.join("manifest.json"), good)
            .map_err(|error| WinAppMigratorError::io(&data, error))?;
        assert!(EmbeddedPackage::open(&parts.output).is_err());
        Ok(())
    }

    #[test]
    fn nothing_can_change_a_data_file_while_it_is_held_open() -> Result<()> {
        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;

        assert!(OpenOptions::new().write(true).open(&data).is_err());
        assert!(fs::rename(&data, parts.root.path().join("moved")).is_err());
        assert!(fs::remove_file(&data).is_err());
        assert!(File::open(&data).is_ok(), "others may still read it");
        assert_eq!(package.read_file("manifest.json")?, b"{\"name\":\"app\"}");

        drop(package);
        assert!(OpenOptions::new().write(true).open(&data).is_ok());
        Ok(())
    }

    #[test]
    fn a_bare_stub_carved_from_each_form_is_the_stub_it_was_built_from() -> Result<()> {
        // One executable, a signed one, a loader, and a signed loader.
        let stub = fake_pe_with_section(0x800);
        let (signed, signed_start) = signed_installer_from(&stub);
        let signed_carved = carve(
            read_stub_image(&mut io::Cursor::new(&signed), signed_start)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?,
        )?;
        assert_eq!(signed_carved, stub);

        let parts = parts()?;
        let data = create_installer_with_data_file(
            &InstallerStub::File(parts.stub.clone()),
            &parts.zip,
            &parts.output,
        )?;
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        let carved = parts.root.path().join("uninstall.exe");
        package.write_bare_stub(&carved)?;
        assert_eq!(
            fs::read(&carved).map_err(|error| WinAppMigratorError::io("", error))?,
            stub
        );
        assert!(!has_certificate_table(&carved)?);
        assert!(
            EmbeddedPackage::open(&carved)?.is_none(),
            "it reads as a bare stub"
        );
        drop(package);

        // Signing a loader puts padding and a table after its footer, and it still works, and
        // what is carved from it is still the stub.
        let mut loader =
            fs::read(&parts.output).map_err(|error| WinAppMigratorError::io("", error))?;
        const PE_OFFSET: usize = 0x80;
        let padding = (8 - loader.len() as u64 % 8) % 8;
        loader.resize(loader.len() + padding as usize, 0);
        let offset = loader.len() as u32;
        let optional = PE_OFFSET + (PE_SIGNATURE_LEN + COFF_HEADER_LEN) as usize;
        let directory = optional + PE32_PLUS_SECURITY_DIRECTORY as usize;
        loader[directory..directory + 4].copy_from_slice(&offset.to_le_bytes());
        loader[directory + 4..directory + 8].copy_from_slice(&64_u32.to_le_bytes());
        let checksum = optional + PE_CHECKSUM_FIELD as usize;
        loader[checksum..checksum + 4].copy_from_slice(&0x1234_5678_u32.to_le_bytes());
        loader.resize(loader.len() + 64, 0xAB);
        fs::write(&parts.output, &loader).map_err(|error| WinAppMigratorError::io("", error))?;
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert_eq!(package.read_file("manifest.json")?, b"{\"name\":\"app\"}");
        let carved_again = parts.root.path().join("uninstall2.exe");
        package.write_bare_stub(&carved_again)?;
        assert_eq!(
            fs::read(&carved_again).map_err(|error| WinAppMigratorError::io("", error))?,
            stub
        );
        assert!(!has_certificate_table(&carved_again)?);
        drop(package);
        // And the data file is what it is, signature or not.
        let good = fs::read(&data).map_err(|error| WinAppMigratorError::io("", error))?;
        let mut bad = good.clone();
        bad[0] ^= 1;
        fs::write(&data, &bad).map_err(|error| WinAppMigratorError::io("", error))?;
        assert!(EmbeddedPackage::open(&parts.output).is_err());
        Ok(())
    }

    #[test]
    fn a_loaders_footer_in_a_certificate_table_is_ignored_and_after_one_is_refused() -> Result<()> {
        let image_len = fake_pe(None).len() as u64;
        let mut hidden = vec![0xAB_u8; 64];
        hidden.extend_from_slice(&sha256_of(b"x"));
        hidden.extend_from_slice(&1_u64.to_le_bytes());
        hidden.extend_from_slice(LOADER_MAGIC);
        let mut inside = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: hidden.len() as u64,
        }));
        inside.extend_from_slice(&hidden);
        assert_eq!(locate(inside)?, None);

        let mut after = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: 64,
        }));
        after.resize(after.len() + 64, 0xAB);
        after.extend_from_slice(&sha256_of(b"x"));
        after.extend_from_slice(&1_u64.to_le_bytes());
        after.extend_from_slice(LOADER_MAGIC);
        assert!(locate(after).is_err());
        Ok(())
    }

    fn zip_of(parts: &Parts, name: &str, content: &[u8]) -> Result<FinishedZip> {
        let file = new_partial_file(&parts.root.path().join(name))?;
        let mut stream = ZipStream::new(file, 0);
        stream
            .add_whole(&WholeEntry {
                name: "manifest.json",
                method: ZipMethod::Stored,
                crc32: crc32fast::hash(content),
                uncompressed_bytes: content.len() as u64,
                data: content,
            })
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        stream
            .finish()
            .map_err(|error| WinAppMigratorError::io("zip", error))
    }

    #[test]
    fn a_package_sure_to_fit_goes_after_the_stub_and_leaves_nothing_else() -> Result<()> {
        let parts = parts()?;
        let stub = InstallerStub::File(parts.stub.clone());
        // One from an earlier build of this name, which has to go.
        let stale = data_file_beside(&parts.output);
        fs::write(&stale, b"old").map_err(|error| WinAppMigratorError::io(&stale, error))?;
        let (pending, file, base) = PendingInstaller::start(&stub, &parts.output, true)?;
        assert_eq!(base, 0x800);
        let mut stream = ZipStream::new(file, base);
        stream
            .add_whole(&WholeEntry {
                name: "manifest.json",
                method: ZipMethod::Stored,
                crc32: crc32fast::hash(b"{}"),
                uncompressed_bytes: 2,
                data: b"{}",
            })
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let finished = stream
            .finish()
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let written = pending.complete(&stub, finished, &BuildMonitor::default())?;
        assert_eq!(written.installer, parts.output);
        assert_eq!(written.data_file, None);
        assert!(!stale.exists());
        assert!(!parts.output.with_extension("partial").exists());
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert!(package.data_file.is_none());
        assert_eq!(package.read_file("manifest.json")?, b"{}");
        Ok(())
    }

    #[test]
    fn a_package_that_might_not_fit_and_does_ends_up_after_the_stub_all_the_same() -> Result<()> {
        let parts = parts()?;
        let stub = InstallerStub::File(parts.stub.clone());
        let (pending, file, base) = PendingInstaller::start(&stub, &parts.output, false)?;
        assert_eq!(base, 0, "it starts by itself, not knowing");
        let partial = parts.output.with_extension(PARTIAL_DATA_EXTENSION);
        assert!(partial.is_file());
        let mut stream = ZipStream::new(file, base);
        stream
            .add_whole(&WholeEntry {
                name: "manifest.json",
                method: ZipMethod::Stored,
                crc32: crc32fast::hash(b"{}"),
                uncompressed_bytes: 2,
                data: b"{}",
            })
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let finished = stream
            .finish()
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let written = pending.complete(&stub, finished, &BuildMonitor::default())?;
        assert_eq!(written.data_file, None);
        assert!(!partial.exists());
        assert!(!parts.output.with_extension(PARTIAL_EXTENSION).exists());
        assert!(!data_file_beside(&parts.output).exists());
        let package = EmbeddedPackage::open(&parts.output)?
            .ok_or_else(|| WinAppMigratorError::Archive("no package".to_owned()))?;
        assert!(package.data_file.is_none());
        assert_eq!(package.read_file("manifest.json")?, b"{}");
        Ok(())
    }

    #[test]
    fn a_cancel_while_a_package_goes_after_the_stub_leaves_nothing_behind() -> Result<()> {
        let parts = parts()?;
        let stub = InstallerStub::File(parts.stub.clone());
        let (pending, file, base) = PendingInstaller::start(&stub, &parts.output, false)?;
        let mut stream = ZipStream::new(file, base);
        stream
            .add_whole(&WholeEntry {
                name: "manifest.json",
                method: ZipMethod::Stored,
                crc32: crc32fast::hash(b"{}"),
                uncompressed_bytes: 2,
                data: b"{}",
            })
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let finished = stream
            .finish()
            .map_err(|error| WinAppMigratorError::io("zip", error))?;
        let monitor = BuildMonitor::default();
        monitor.cancel();
        assert!(matches!(
            pending.complete(&stub, finished, &monitor),
            Err(WinAppMigratorError::Cancelled)
        ));
        for extension in [
            "exe",
            PARTIAL_EXTENSION,
            PARTIAL_DATA_EXTENSION,
            DATA_FILE_EXTENSION,
        ] {
            assert!(
                !parts.output.with_extension(extension).exists(),
                "{extension}"
            );
        }
        Ok(())
    }

    /// Two builds of the same name: it goes from one to the other, or it stays as it was.
    #[test]
    fn a_loader_and_its_data_file_go_in_together_or_not_at_all() -> Result<()> {
        let parts = parts()?;
        let stub = InstallerStub::File(parts.stub.clone());
        let data = data_file_beside(&parts.output);
        fs::write(&parts.output, b"an earlier loader")
            .map_err(|error| WinAppMigratorError::io("", error))?;
        fs::write(&data, b"an earlier data file")
            .map_err(|error| WinAppMigratorError::io("", error))?;

        let (pending, file, _) = PendingInstaller::start(&stub, &parts.output, false)?;
        drop(file);
        let exe_partial = parts.output.with_extension(PARTIAL_EXTENSION);
        // The loader cannot go in: something holds what is at its name.
        fs::write(&exe_partial, b"the new loader")
            .map_err(|error| WinAppMigratorError::io("", error))?;
        let held = OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&parts.output)
            .map_err(|error| WinAppMigratorError::io("", error))?;
        assert!(pending.put_both_in_place(&exe_partial, &data).is_err());
        drop(held);
        assert_eq!(
            fs::read(&data).map_err(|error| WinAppMigratorError::io("", error))?,
            b"an earlier data file"
        );
        assert_eq!(
            fs::read(&parts.output).map_err(|error| WinAppMigratorError::io("", error))?,
            b"an earlier loader"
        );

        // With nothing in the way, both change.
        let partial = parts.output.with_extension(PARTIAL_DATA_EXTENSION);
        fs::write(&partial, b"the new data file")
            .map_err(|error| WinAppMigratorError::io("", error))?;
        pending.put_both_in_place(&exe_partial, &data)?;
        assert_eq!(
            fs::read(&data).map_err(|error| WinAppMigratorError::io("", error))?,
            b"the new data file"
        );
        assert_eq!(
            fs::read(&parts.output).map_err(|error| WinAppMigratorError::io("", error))?,
            b"the new loader"
        );
        assert!(!data.with_extension(EARLIER_DATA_EXTENSION).exists());
        Ok(())
    }

    #[test]
    fn a_build_that_stops_leaves_nothing_behind_and_keeps_an_earlier_installer() -> Result<()> {
        let parts = parts()?;
        let stub = InstallerStub::File(parts.stub.clone());
        fs::write(&parts.output, b"from an earlier run")
            .map_err(|error| WinAppMigratorError::io("", error))?;
        for surely_one_file in [true, false] {
            let (pending, file, base) =
                PendingInstaller::start(&stub, &parts.output, surely_one_file)?;
            let mut stream = ZipStream::new(file, base);
            stream
                .add_whole(&WholeEntry {
                    name: "a.txt",
                    method: ZipMethod::Stored,
                    crc32: crc32fast::hash(b"a"),
                    uncompressed_bytes: 1,
                    data: b"a",
                })
                .map_err(|error| WinAppMigratorError::io("zip", error))?;
            drop(stream);
            drop(pending);
        }
        let mut left: Vec<String> = fs::read_dir(parts.root.path())
            .map_err(|error| WinAppMigratorError::io("", error))?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("App_installer"))
            .collect();
        left.sort();
        assert_eq!(left, ["App_installer.exe"]);
        assert_eq!(
            fs::read(&parts.output).map_err(|error| WinAppMigratorError::io("", error))?,
            b"from an earlier run"
        );
        Ok(())
    }

    #[test]
    fn a_signed_stub_cannot_start_any_form() -> Result<()> {
        let parts = parts()?;
        let image_len = fake_pe(None).len() as u64;
        let mut signed = fake_pe(Some(CertificateTable {
            offset: image_len,
            size: 64,
        }));
        signed.resize(signed.len() + 64, 0xAB);
        fs::write(&parts.stub, signed).map_err(|error| WinAppMigratorError::io("", error))?;
        let stub = InstallerStub::File(parts.stub.clone());
        for surely_one_file in [true, false] {
            assert!(matches!(
                PendingInstaller::start(&stub, &parts.output, surely_one_file),
                Err(WinAppMigratorError::InvalidInput(_))
            ));
        }
        assert!(matches!(
            create_installer_with_data_file(&stub, &parts.zip, &parts.output),
            Err(WinAppMigratorError::InvalidInput(_))
        ));
        assert!(!parts.output.exists());
        assert!(!data_file_beside(&parts.output).exists());
        Ok(())
    }

    #[test]
    fn a_stale_partial_in_the_way_is_replaced_and_not_written_through() -> Result<()> {
        let parts = parts()?;
        let partial = parts.output.with_extension(PARTIAL_EXTENSION);
        fs::write(&partial, b"left by a run that died")
            .map_err(|error| WinAppMigratorError::io("", error))?;
        let finished = zip_of(&parts, "a.zip", b"{}")?;
        drop(finished.file);
        let stub = InstallerStub::File(parts.stub.clone());
        let (pending, file, base) = PendingInstaller::start(&stub, &parts.output, true)?;
        assert_eq!(
            fs::metadata(&partial)
                .map_err(|error| WinAppMigratorError::io("", error))?
                .len(),
            base
        );
        drop(file);
        drop(pending);
        assert!(!partial.exists());
        Ok(())
    }

    #[test]
    fn unpacking_hashes_what_comes_out_and_reports_it_by_name() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let zip = zip_holding(&["files/one.txt", "files/Sub/Two.txt"])?;
        let digests = extract_zip_reader(
            io::Cursor::new(&zip),
            root.path(),
            &InstallMonitor::default(),
        )?;
        let one = digests
            .get(Path::new(r"files\one.txt"))
            .ok_or_else(|| WinAppMigratorError::Archive("no digest".to_owned()))?;
        assert_eq!(one.size, 1);
        assert_eq!(one.sha256, hex_lower(&Sha256::digest(b"x")));
        assert!(
            digests.get(Path::new(r"FILES\SUB\two.txt")).is_some(),
            "by Windows' rules"
        );
        assert!(digests.get(Path::new(r"files\three.txt")).is_none());
        Ok(())
    }

    #[test]
    fn an_entry_that_holds_more_than_it_says_is_stopped_at_what_it_says() -> Result<()> {
        // A real zip, with what its central directory says of the size cut down.
        let mut zip = zip_holding(&["files/a.txt"])?;
        let central = zip
            .windows(4)
            .position(|window| window == b"PK\x01\x02")
            .ok_or_else(|| WinAppMigratorError::Archive("no central directory".to_owned()))?;
        // The uncompressed size, 24 bytes into a central header, from one byte to none.
        zip[central + 24] = 0;
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        assert!(
            extract_zip_reader(
                io::Cursor::new(&zip),
                root.path(),
                &InstallMonitor::default()
            )
            .is_err()
        );
        Ok(())
    }

    #[test]
    fn a_folder_goes_in_a_zip_in_order_by_name() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let package = root.path().join("package");
        for name in ["z.txt", "a.txt", r"m\b.txt", r"m\a.txt", "files.txt"] {
            let path = package.join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io("", error))?;
            }
            fs::write(&path, name).map_err(|error| WinAppMigratorError::io("", error))?;
        }
        let zip = root.path().join("out.zip");
        create_zip_archive(&package, &zip, &BuildMonitor::default())?;
        let archive =
            ZipArchive::new(File::open(&zip).map_err(|error| WinAppMigratorError::io("", error))?)
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let names: Vec<String> = archive
            .file_names()
            .filter_map(std::result::Result::ok)
            .map(|name| name.into_owned())
            .collect();
        assert_eq!(names, ["a.txt", "files.txt", "m/a.txt", "m/b.txt", "z.txt"]);
        Ok(())
    }

    #[test]
    fn cancelled_build_stops_compressing() -> Result<()> {
        let root = temp_test_dir("cancel")?;
        let package_root = root.join("package");
        fs::create_dir_all(&package_root)
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        fs::write(package_root.join("manifest.json"), "{}")
            .map_err(|error| WinAppMigratorError::io(&package_root, error))?;
        let monitor = BuildMonitor::default();
        monitor.cancel();

        let result = create_zip_archive(&package_root, &root.join("package.zip"), &monitor);

        assert!(matches!(result, Err(WinAppMigratorError::Cancelled)));
        fs::remove_dir_all(&root).map_err(|error| WinAppMigratorError::io(&root, error))?;
        Ok(())
    }

    fn temp_test_dir(label: &str) -> Result<PathBuf> {
        Ok(std::env::temp_dir().join(format!(
            "winappmigrator-{label}-{}-{}",
            std::process::id(),
            OffsetDateTime::now_utc().unix_timestamp_nanos()
        )))
    }
}
