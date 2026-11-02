use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Component, Path};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::reject_unsafe_components;
use crate::models::InstallerStub;
use crate::progress::{BuildMonitor, BuildStep};

const ZIP_BUFFER_SIZE: usize = 1024 * 1024;
const SFX_MAGIC: &[u8] = b"WINAPPMIGRATOR_SFX_V2";
const SFX_HASH_BYTES: usize = 32;
const SFX_LENGTH_BYTES: usize = 8;
/// After the package: its SHA-256, its length, then the magic. The stub and its package
/// always ship together, so the layout can change freely between versions.
const SFX_FOOTER_LEN: u64 = (SFX_HASH_BYTES + SFX_LENGTH_BYTES + SFX_MAGIC.len()) as u64;
/// Lets other programs read an installer while it is in use, but not change, rename, or
/// delete it.
const FILE_SHARE_READ: u32 = 0x0000_0001;
const HASH_BUFFER_BYTES: usize = 1024 * 1024;

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
/// Offset of the security data directory from the start of the optional header.
const PE32_SECURITY_DIRECTORY: u64 = 128;
const PE32_PLUS_SECURITY_DIRECTORY: u64 = 144;
/// Signing tools align the certificate table to 8 bytes with zero padding.
const MAX_SIGNATURE_PADDING: u64 = 7;
/// Upper bound for a single file read out of a package into memory (the manifest).
const MAX_EMBEDDED_FILE_BYTES: u64 = 1024 * 1024 * 1024;
const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
const MAX_INSTALLER_BYTES: u64 = 4 * 1024 * 1024 * 1024;

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
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let mut buffer = vec![0_u8; ZIP_BUFFER_SIZE];

    let mut entries = Vec::new();
    for entry in WalkDir::new(package_root).follow_links(false) {
        let entry = entry.map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        if entry.path() != package_root && !entry.file_type().is_dir() {
            entries.push(entry);
        }
    }
    let total = entries.len();
    monitor.report(BuildStep::Compressing { done: 0, total });

    for (done, entry) in entries.iter().enumerate() {
        monitor.checkpoint()?;
        let path = entry.path();
        let name = zip_entry_name(package_root, path)?;
        let size = entry
            .metadata()
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?
            .len();
        // Without ZIP64 a file over 4 GiB fails the build after all the copying
        // has already been done.
        let options = options.large_file(size >= u64::from(u32::MAX));
        writer
            .start_file(name, options)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let mut source = File::open(path).map_err(|error| WinAppMigratorError::io(path, error))?;
        loop {
            let read = source
                .read(&mut buffer)
                .map_err(|error| WinAppMigratorError::io(path, error))?;
            if read == 0 {
                break;
            }
            writer
                .write_all(&buffer[..read])
                .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        }
        monitor.report(BuildStep::Compressing {
            done: done + 1,
            total,
        });
    }

    writer
        .finish()
        .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
    Ok(())
}

pub fn extract_zip_archive(archive_path: &Path, destination_root: &Path) -> Result<()> {
    fs::create_dir_all(destination_root)
        .map_err(|error| WinAppMigratorError::io(destination_root, error))?;
    let file =
        File::open(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    extract_zip_reader(file, destination_root)
}

fn extract_zip_reader<R: Read + Seek>(reader: R, destination_root: &Path) -> Result<()> {
    let mut archive =
        ZipArchive::new(reader).map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let Some(enclosed_name) = file.enclosed_name() else {
            let name = file
                .name()
                .map_or_else(|_| "<invalid name>".to_owned(), |name| name.into_owned());
            return Err(WinAppMigratorError::Archive(format!(
                "zip entry has unsafe path: {name}"
            )));
        };
        reject_unsafe_components(&enclosed_name)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let destination = destination_root.join(enclosed_name);

        if file.is_dir() {
            fs::create_dir_all(&destination)
                .map_err(|error| WinAppMigratorError::io(&destination, error))?;
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
        }
        // Extraction goes into a new, empty folder, so anything already at this path was
        // not put there by this package, and opening it could follow a link.
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&destination)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;
        io::copy(&mut file, &mut output)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;
    }
    Ok(())
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
    let partial_path = output_path.with_extension("partial");
    let result = write_installer(stub, archive_path, &partial_path).and_then(|()| {
        fs::rename(&partial_path, output_path)
            .map_err(|error| WinAppMigratorError::io(output_path, error))
    });
    if result.is_err() {
        let _ = fs::remove_file(&partial_path);
    }
    result
}

fn write_installer(stub: &InstallerStub, archive_path: &Path, output_path: &Path) -> Result<()> {
    match stub {
        InstallerStub::File(path) => {
            fs::copy(path, output_path)
                .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        }
        InstallerStub::Embedded(bytes) => {
            fs::write(output_path, bytes)
                .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        }
    }
    let mut output = OpenOptions::new()
        .append(true)
        .open(output_path)
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    let mut archive =
        File::open(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut archive_len = 0_u64;
    loop {
        let read = archive
            .read(&mut buffer)
            .map_err(|error| WinAppMigratorError::io(archive_path, error))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        output
            .write_all(&buffer[..read])
            .map_err(|error| WinAppMigratorError::io(output_path, error))?;
        archive_len += read as u64;
    }
    for part in [
        hasher.finalize().as_slice(),
        &archive_len.to_le_bytes(),
        SFX_MAGIC,
    ] {
        output
            .write_all(part)
            .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    }
    Ok(())
}

/// The package inside an installer: found, checked against the hash in its footer before
/// any of it is parsed, and held open. While this exists no other process can change,
/// rename, or replace the installer file, so what is read later is what was checked. An
/// installer usually sits in a folder anyone can write to, and it runs elevated.
///
/// The hash catches a truncated copy or an edited manifest. It does not prove who made
/// the package, since whoever edits one can also update its footer. Only signing the
/// finished installer does that.
#[derive(Debug)]
pub struct EmbeddedPackage {
    file: File,
    location: ArchiveLocation,
}

impl EmbeddedPackage {
    /// Open an installer's package. `Ok(None)` for a bare stub, which carries none. This
    /// reads the whole package, so a large installer takes a moment.
    pub fn open(sfx_executable: &Path) -> Result<Option<Self>> {
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
        check_package_hash(&mut file, location)
            .map_err(|error| WinAppMigratorError::io(sfx_executable, error))??;
        Ok(Some(Self { file, location }))
    }

    fn section(&self) -> Result<ArchiveSection> {
        let file = self
            .file
            .try_clone()
            .map_err(|error| WinAppMigratorError::io("installer package", error))?;
        ArchiveSection::new(file, self.location)
            .map_err(|error| WinAppMigratorError::io("installer package", error))
    }

    /// Unpack the package into `destination_root`.
    pub fn extract_to(&self, destination_root: &Path) -> Result<()> {
        fs::create_dir_all(destination_root)
            .map_err(|error| WinAppMigratorError::io(destination_root, error))?;
        extract_zip_reader(BufReader::new(self.section()?), destination_root)
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

pub fn extract_embedded_self_archive(
    sfx_executable: &Path,
    destination_root: &Path,
) -> Result<bool> {
    let Some(package) = EmbeddedPackage::open(sfx_executable)? else {
        return Ok(false);
    };
    package.extract_to(destination_root)?;
    Ok(true)
}

/// Whether an installer carries a package, without reading all of it.
pub fn has_embedded_self_archive(sfx_executable: &Path) -> Result<bool> {
    let mut file = File::open(sfx_executable)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    let location = locate_embedded_archive(&mut file)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))??;
    Ok(location.is_some())
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
) -> io::Result<Result<()>> {
    file.seek(SeekFrom::Start(location.start))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut remaining = location.len;
    while remaining > 0 {
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

/// Windows will not start an executable of 4 GiB or more ("not a valid Win32
/// application"). Reporting success for a file that can never run would only be
/// discovered on the target PC, possibly after the source is gone.
fn refuse_oversized_installer(size: u64) -> Result<()> {
    if size < MAX_INSTALLER_BYTES {
        return Ok(());
    }
    Err(WinAppMigratorError::InvalidInput(format!(
        "the installer would be {:.1} GiB, and Windows cannot run an executable of 4 GiB or \
         more; nothing was written",
        size as f64 / GIB
    )))
}

/// Where a package sits inside an installer executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ArchiveLocation {
    start: u64,
    len: u64,
    sha256: [u8; SFX_HASH_BYTES],
}

/// Find the embedded package. A signed installer keeps its package directly
/// before the certificate table, which is the only place a signature covers.
/// Bytes inside the table are never read, and bytes after it are refused.
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
    if magic != SFX_MAGIC {
        // Data past the program with no footer means a package whose end was cut off, a
        // common result of an interrupted copy. Calling that a bare stub would be wrong.
        if pe_image_end(file, file_len)?.is_some_and(|image_end| end > image_end) {
            return Ok(Err(damaged_installer()));
        }
        return Ok(Ok(None));
    }

    let mut len = [0_u8; SFX_LENGTH_BYTES];
    len.copy_from_slice(length_bytes);
    let len = u64::from_le_bytes(len);
    if len > footer_start {
        return Ok(Err(WinAppMigratorError::Archive(
            "self-extracting archive footer length is invalid".to_owned(),
        )));
    }
    let mut hash = [0_u8; SFX_HASH_BYTES];
    hash.copy_from_slice(sha256);
    Ok(Ok(Some(ArchiveLocation {
        start: footer_start - len,
        len,
        sha256: hash,
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

/// A bounded view of the package inside an installer, so it is read in place
/// rather than buffered in memory or copied out to a second file first.
struct ArchiveSection {
    file: File,
    location: ArchiveLocation,
    position: u64,
}

impl ArchiveSection {
    fn new(mut file: File, location: ArchiveLocation) -> io::Result<Self> {
        file.seek(SeekFrom::Start(location.start))?;
        Ok(Self {
            file,
            location,
            position: 0,
        })
    }
}

impl Read for ArchiveSection {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let remaining = self.location.len.saturating_sub(self.position);
        let limit = usize::try_from(remaining).map_or(buffer.len(), |left| left.min(buffer.len()));
        if limit == 0 {
            return Ok(0);
        }
        let read = self.file.read(&mut buffer[..limit])?;
        self.position += read as u64;
        Ok(read)
    }
}

impl Seek for ArchiveSection {
    fn seek(&mut self, target: SeekFrom) -> io::Result<u64> {
        let position = match target {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::Current(delta) => self.position.checked_add_signed(delta),
            SeekFrom::End(delta) => self.location.len.checked_add_signed(delta),
        }
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek outside the package"))?;
        self.file.seek(SeekFrom::Start(
            self.location.start.saturating_add(position),
        ))?;
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
    use std::path::PathBuf;
    use time::OffsetDateTime;

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
            "files/nul.txt",
            "files/a.exe:stream",
            "files/.. ",
            "../outside.txt",
        ] {
            let root =
                tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
            let zip = zip_holding(&[name])?;
            assert!(
                extract_zip_reader(io::Cursor::new(zip), root.path()).is_err(),
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

    #[test]
    fn extraction_never_opens_something_already_at_the_destination() -> Result<()> {
        let root = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let planted = root.path().join("a.txt");
        fs::write(&planted, b"keep").map_err(|error| WinAppMigratorError::io(&planted, error))?;
        let zip = zip_holding(&["a.txt"])?;

        assert!(extract_zip_reader(io::Cursor::new(zip), root.path()).is_err());
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
        check_package_hash(&mut file, location)
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
        let checked = check_package_hash(&mut file, location)
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
        assert!(extract_embedded_self_archive(
            &installer_path,
            &extract_root
        )?);
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
        assert!(extract_embedded_self_archive(&installer, &root.path().join("out")).is_err());
        assert!(!root.path().join("out").join("manifest.json").exists());
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
        assert!(refuse_oversized_installer(MAX_INSTALLER_BYTES - 1).is_ok());
        assert!(refuse_oversized_installer(MAX_INSTALLER_BYTES).is_err());
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
