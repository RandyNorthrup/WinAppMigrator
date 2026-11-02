//! A standard zip, written in one pass and never gone back over.
//!
//! An installer's footer holds the SHA-256 of its package, and that is worked out here as the
//! bytes go out, so that nothing has to read them back to find it. A writer that seeks back to
//! fill in an entry's sizes, as the `zip` crate does, could not do that. So an entry whose sizes
//! are known says so in its header, and one that is read as it is written says so in a data
//! descriptor after it. Anything that goes by the central directory reads either, which is what
//! this project's own reader, .NET, and Windows' tools do.

use std::fs::File;
use std::io::{self, BufWriter, Seek, SeekFrom, Write};

use sha2::{Digest, Sha256};

const LOCAL_HEADER_SIGNATURE: u32 = 0x0403_4B50;
const CENTRAL_HEADER_SIGNATURE: u32 = 0x0201_4B50;
const DATA_DESCRIPTOR_SIGNATURE: u32 = 0x0807_4B50;
const END_OF_CENTRAL_SIGNATURE: u32 = 0x0605_4B50;
const ZIP64_END_OF_CENTRAL_SIGNATURE: u32 = 0x0606_4B50;
const ZIP64_LOCATOR_SIGNATURE: u32 = 0x0706_4B50;
/// Says that the sizes come after the data.
const FLAG_DATA_DESCRIPTOR: u16 = 1 << 3;
/// Says that names are UTF-8.
const FLAG_UTF8_NAME: u16 = 1 << 11;
const METHOD_STORED: u16 = 0;
const METHOD_DEFLATED: u16 = 8;
const VERSION_PLAIN: u16 = 20;
const VERSION_ZIP64: u16 = 45;
const ZIP64_EXTRA_ID: u16 = 0x0001;
/// What a 32-bit field holds when the real value is in the ZIP64 record.
const IN_ZIP64_RECORD: u32 = u32::MAX;
const MOST_ENTRIES_WITHOUT_ZIP64: usize = 0xFFFF;
/// January 1, 1980, the first date a zip can say. An installer writes what it unpacks with the
/// time it unpacks it, so a real date here would only make two builds differ.
const FIXED_DOS_DATE: u16 = 0x0021;
const FIXED_DOS_TIME: u16 = 0;
const WRITE_BUFFER_BYTES: usize = 1024 * 1024;
const LOCAL_HEADER_BYTES: usize = 30;
const CENTRAL_HEADER_BYTES: usize = 46;
const ZIP64_END_OF_CENTRAL_BYTES: usize = 56;
const ZIP64_LOCATOR_BYTES: usize = 20;
const END_OF_CENTRAL_BYTES: usize = 22;
/// What an entry adds beside its data, for working out how big a package can be: both headers with
/// their ZIP64 records, a data descriptor, and a name of a long path twice over.
pub(crate) const ENTRY_OVERHEAD_BYTES: u64 = 1024;

/// How an entry's bytes are held in the zip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ZipMethod {
    Stored,
    Deflated,
}

impl ZipMethod {
    fn code(self) -> u16 {
        match self {
            Self::Stored => METHOD_STORED,
            Self::Deflated => METHOD_DEFLATED,
        }
    }
}

/// An entry with everything about it known, and its bytes in hand.
pub(crate) struct WholeEntry<'a> {
    pub name: &'a str,
    pub method: ZipMethod,
    pub crc32: u32,
    pub uncompressed_bytes: u64,
    pub data: &'a [u8],
}

/// An entry on its way out.
pub(crate) struct OpenEntry {
    name: Vec<u8>,
    method: ZipMethod,
    offset: u64,
    compressed_bytes: u64,
    /// How the hash stood before this entry, for taking it back out.
    hash_before: Sha256,
}

/// What the central directory says of an entry.
struct CentralRecord {
    name: Vec<u8>,
    method: ZipMethod,
    flags: u16,
    version_needed: u16,
    crc32: u32,
    compressed_bytes: u64,
    uncompressed_bytes: u64,
    offset: u64,
}

/// A finished zip: the file it is in, how long it is, and the SHA-256 of all of it.
pub(crate) struct FinishedZip {
    pub file: File,
    pub len: u64,
    pub sha256: [u8; 32],
}

/// Writes one zip, in order, to a file that may already hold something before it. Offsets in the
/// zip are from where it starts, so that it is a standard zip by itself.
pub(crate) struct ZipStream {
    file: BufWriter<File>,
    /// Where in the file the zip starts.
    base: u64,
    written: u64,
    hasher: Sha256,
    records: Vec<CentralRecord>,
}

impl ZipStream {
    /// Start a zip at the file's current end, which is `base` bytes in.
    pub(crate) fn new(file: File, base: u64) -> Self {
        Self {
            file: BufWriter::with_capacity(WRITE_BUFFER_BYTES, file),
            base,
            written: 0,
            hasher: Sha256::new(),
            records: Vec::new(),
        }
    }

    fn put(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.hasher.update(bytes);
        self.file.write_all(bytes)?;
        self.written += bytes.len() as u64;
        Ok(())
    }

    fn name_bytes(name: &str) -> io::Result<Vec<u8>> {
        if u16::try_from(name.len()).is_err() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("this name is too long for a zip: {name}"),
            ));
        }
        Ok(name.as_bytes().to_vec())
    }

    /// Add an entry whose bytes and sizes are all known.
    pub(crate) fn add_whole(&mut self, entry: &WholeEntry<'_>) -> io::Result<()> {
        let name = Self::name_bytes(entry.name)?;
        let compressed_bytes = entry.data.len() as u64;
        let sizes_fit = entry.uncompressed_bytes < u64::from(IN_ZIP64_RECORD)
            && compressed_bytes < u64::from(IN_ZIP64_RECORD);
        let version_needed = if sizes_fit {
            VERSION_PLAIN
        } else {
            VERSION_ZIP64
        };
        let offset = self.written;
        let header = local_header(&LocalHeader {
            name: &name,
            method: entry.method,
            flags: FLAG_UTF8_NAME,
            version_needed,
            crc32: entry.crc32,
            compressed_bytes,
            uncompressed_bytes: entry.uncompressed_bytes,
            zip64: !sizes_fit,
        });
        self.put(&header)?;
        self.put(entry.data)?;
        self.records.push(CentralRecord {
            name,
            method: entry.method,
            flags: FLAG_UTF8_NAME,
            version_needed,
            crc32: entry.crc32,
            compressed_bytes,
            uncompressed_bytes: entry.uncompressed_bytes,
            offset,
        });
        Ok(())
    }

    /// Start an entry whose sizes will be known when it ends. How the hash stands is kept, so that
    /// a file that turns out not to be readable to its end can be taken back out.
    pub(crate) fn begin(&mut self, name: &str, method: ZipMethod) -> io::Result<OpenEntry> {
        let name = Self::name_bytes(name)?;
        let offset = self.written;
        let hash_before = self.hasher.clone();
        let header = local_header(&LocalHeader {
            name: &name,
            method,
            flags: FLAG_UTF8_NAME | FLAG_DATA_DESCRIPTOR,
            version_needed: VERSION_ZIP64,
            crc32: 0,
            compressed_bytes: 0,
            uncompressed_bytes: 0,
            zip64: true,
        });
        self.put(&header)?;
        Ok(OpenEntry {
            name,
            method,
            offset,
            compressed_bytes: 0,
            hash_before,
        })
    }

    pub(crate) fn write_data(&mut self, entry: &mut OpenEntry, data: &[u8]) -> io::Result<()> {
        self.put(data)?;
        entry.compressed_bytes += data.len() as u64;
        Ok(())
    }

    /// Finish an entry: its sizes go out after its data, and in the central directory.
    pub(crate) fn end(
        &mut self,
        entry: OpenEntry,
        crc32: u32,
        uncompressed_bytes: u64,
    ) -> io::Result<()> {
        // A local header with a ZIP64 record means eight-byte sizes here.
        let mut descriptor = Vec::with_capacity(24);
        descriptor.extend_from_slice(&DATA_DESCRIPTOR_SIGNATURE.to_le_bytes());
        descriptor.extend_from_slice(&crc32.to_le_bytes());
        descriptor.extend_from_slice(&entry.compressed_bytes.to_le_bytes());
        descriptor.extend_from_slice(&uncompressed_bytes.to_le_bytes());
        self.put(&descriptor)?;
        self.records.push(CentralRecord {
            name: entry.name,
            method: entry.method,
            flags: FLAG_UTF8_NAME | FLAG_DATA_DESCRIPTOR,
            version_needed: VERSION_ZIP64,
            crc32,
            compressed_bytes: entry.compressed_bytes,
            uncompressed_bytes,
            offset: entry.offset,
        });
        Ok(())
    }

    /// Take an entry back out, as if it had never been started.
    pub(crate) fn abandon(&mut self, entry: OpenEntry) -> io::Result<()> {
        self.file.flush()?;
        let file = self.file.get_mut();
        file.set_len(self.base + entry.offset)?;
        file.seek(SeekFrom::Start(self.base + entry.offset))?;
        self.written = entry.offset;
        self.hasher = entry.hash_before;
        Ok(())
    }

    /// Write the central directory, and say how long the zip came out and what it hashes to.
    pub(crate) fn finish(mut self) -> io::Result<FinishedZip> {
        let central_start = self.written;
        let records = std::mem::take(&mut self.records);
        for record in &records {
            let header = central_header(record);
            self.put(&header)?;
        }
        let central_bytes = self.written - central_start;
        let needs_zip64 = records.len() >= MOST_ENTRIES_WITHOUT_ZIP64
            || central_start >= u64::from(IN_ZIP64_RECORD)
            || central_bytes >= u64::from(IN_ZIP64_RECORD);
        if needs_zip64 {
            let zip64_end = self.written;
            let end = zip64_end_of_central(records.len() as u64, central_bytes, central_start);
            self.put(&end)?;
            let locator = zip64_locator(zip64_end);
            self.put(&locator)?;
        }
        let end = end_of_central(records.len(), central_bytes, central_start);
        self.put(&end)?;
        self.file.flush()?;
        let len = self.written;
        let sha256 = self.hasher.finalize();
        let mut digest = [0_u8; 32];
        digest.copy_from_slice(sha256.as_slice());
        let file = self.file.into_inner().map_err(|error| error.into_error())?;
        Ok(FinishedZip {
            file,
            len,
            sha256: digest,
        })
    }
}

struct LocalHeader<'a> {
    name: &'a [u8],
    method: ZipMethod,
    flags: u16,
    version_needed: u16,
    crc32: u32,
    compressed_bytes: u64,
    uncompressed_bytes: u64,
    zip64: bool,
}

/// Fits in its field, by what was checked of a name and of sizes before this is called.
fn low16(value: usize) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

/// A value as a 32-bit field, or the mark that says it is in the ZIP64 record.
fn field32(value: u64) -> u32 {
    u32::try_from(value)
        .ok()
        .filter(|fits| *fits != IN_ZIP64_RECORD)
        .unwrap_or(IN_ZIP64_RECORD)
}

fn local_header(header: &LocalHeader<'_>) -> Vec<u8> {
    let extra: Vec<u8> = if header.zip64 {
        let mut extra = Vec::with_capacity(20);
        extra.extend_from_slice(&ZIP64_EXTRA_ID.to_le_bytes());
        extra.extend_from_slice(&16_u16.to_le_bytes());
        extra.extend_from_slice(&header.uncompressed_bytes.to_le_bytes());
        extra.extend_from_slice(&header.compressed_bytes.to_le_bytes());
        extra
    } else {
        Vec::new()
    };
    let (compressed, uncompressed) = if header.zip64 {
        (IN_ZIP64_RECORD, IN_ZIP64_RECORD)
    } else {
        (
            field32(header.compressed_bytes),
            field32(header.uncompressed_bytes),
        )
    };
    let mut bytes = Vec::with_capacity(LOCAL_HEADER_BYTES + header.name.len() + extra.len());
    bytes.extend_from_slice(&LOCAL_HEADER_SIGNATURE.to_le_bytes());
    bytes.extend_from_slice(&header.version_needed.to_le_bytes());
    bytes.extend_from_slice(&header.flags.to_le_bytes());
    bytes.extend_from_slice(&header.method.code().to_le_bytes());
    bytes.extend_from_slice(&FIXED_DOS_TIME.to_le_bytes());
    bytes.extend_from_slice(&FIXED_DOS_DATE.to_le_bytes());
    bytes.extend_from_slice(&header.crc32.to_le_bytes());
    bytes.extend_from_slice(&compressed.to_le_bytes());
    bytes.extend_from_slice(&uncompressed.to_le_bytes());
    bytes.extend_from_slice(&low16(header.name.len()).to_le_bytes());
    bytes.extend_from_slice(&low16(extra.len()).to_le_bytes());
    bytes.extend_from_slice(header.name);
    bytes.extend_from_slice(&extra);
    bytes
}

fn central_header(record: &CentralRecord) -> Vec<u8> {
    let uncompressed = field32(record.uncompressed_bytes);
    let compressed = field32(record.compressed_bytes);
    let offset = field32(record.offset);
    let mut extra_body = Vec::new();
    if uncompressed == IN_ZIP64_RECORD {
        extra_body.extend_from_slice(&record.uncompressed_bytes.to_le_bytes());
    }
    if compressed == IN_ZIP64_RECORD {
        extra_body.extend_from_slice(&record.compressed_bytes.to_le_bytes());
    }
    if offset == IN_ZIP64_RECORD {
        extra_body.extend_from_slice(&record.offset.to_le_bytes());
    }
    let mut extra = Vec::new();
    if !extra_body.is_empty() {
        extra.extend_from_slice(&ZIP64_EXTRA_ID.to_le_bytes());
        extra.extend_from_slice(&low16(extra_body.len()).to_le_bytes());
        extra.extend_from_slice(&extra_body);
    }
    let version_needed = if extra.is_empty() {
        record.version_needed
    } else {
        VERSION_ZIP64
    };
    let mut bytes = Vec::with_capacity(CENTRAL_HEADER_BYTES + record.name.len() + extra.len());
    bytes.extend_from_slice(&CENTRAL_HEADER_SIGNATURE.to_le_bytes());
    bytes.extend_from_slice(&version_needed.to_le_bytes());
    bytes.extend_from_slice(&version_needed.to_le_bytes());
    bytes.extend_from_slice(&record.flags.to_le_bytes());
    bytes.extend_from_slice(&record.method.code().to_le_bytes());
    bytes.extend_from_slice(&FIXED_DOS_TIME.to_le_bytes());
    bytes.extend_from_slice(&FIXED_DOS_DATE.to_le_bytes());
    bytes.extend_from_slice(&record.crc32.to_le_bytes());
    bytes.extend_from_slice(&compressed.to_le_bytes());
    bytes.extend_from_slice(&uncompressed.to_le_bytes());
    bytes.extend_from_slice(&low16(record.name.len()).to_le_bytes());
    bytes.extend_from_slice(&low16(extra.len()).to_le_bytes());
    // Comment length, disk number, internal attributes, and external attributes.
    bytes.extend_from_slice(&[0_u8; 10]);
    bytes.extend_from_slice(&offset.to_le_bytes());
    bytes.extend_from_slice(&record.name);
    bytes.extend_from_slice(&extra);
    bytes
}

fn zip64_end_of_central(entries: u64, central_bytes: u64, central_start: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(ZIP64_END_OF_CENTRAL_BYTES);
    bytes.extend_from_slice(&ZIP64_END_OF_CENTRAL_SIGNATURE.to_le_bytes());
    // Its size, not counting this field or the signature.
    bytes.extend_from_slice(&((ZIP64_END_OF_CENTRAL_BYTES as u64) - 12).to_le_bytes());
    bytes.extend_from_slice(&VERSION_ZIP64.to_le_bytes());
    bytes.extend_from_slice(&VERSION_ZIP64.to_le_bytes());
    // This disk, and the disk the central directory starts on.
    bytes.extend_from_slice(&[0_u8; 8]);
    bytes.extend_from_slice(&entries.to_le_bytes());
    bytes.extend_from_slice(&entries.to_le_bytes());
    bytes.extend_from_slice(&central_bytes.to_le_bytes());
    bytes.extend_from_slice(&central_start.to_le_bytes());
    bytes
}

fn zip64_locator(zip64_end: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(ZIP64_LOCATOR_BYTES);
    bytes.extend_from_slice(&ZIP64_LOCATOR_SIGNATURE.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&zip64_end.to_le_bytes());
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes
}

fn end_of_central(entries: usize, central_bytes: u64, central_start: u64) -> Vec<u8> {
    let count = u16::try_from(entries)
        .ok()
        .filter(|count| usize::from(*count) < MOST_ENTRIES_WITHOUT_ZIP64)
        .unwrap_or(u16::MAX);
    let mut bytes = Vec::with_capacity(END_OF_CENTRAL_BYTES);
    bytes.extend_from_slice(&END_OF_CENTRAL_SIGNATURE.to_le_bytes());
    bytes.extend_from_slice(&[0_u8; 4]);
    bytes.extend_from_slice(&count.to_le_bytes());
    bytes.extend_from_slice(&count.to_le_bytes());
    bytes.extend_from_slice(&field32(central_bytes).to_le_bytes());
    bytes.extend_from_slice(&field32(central_start).to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use zip::ZipArchive;

    use super::*;

    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn crc_of(bytes: &[u8]) -> u32 {
        crc32fast::hash(bytes)
    }

    fn deflated(bytes: &[u8]) -> Vec<u8> {
        let mut encoder =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        let written = encoder.write_all(bytes);
        assert!(written.is_ok());
        encoder.finish().unwrap_or_default()
    }

    /// The zip crate, which is what an installer reads with, and .NET, which is another reader.
    fn read_back(path: &std::path::Path) -> TestResult<Vec<(String, Vec<u8>)>> {
        let mut archive = ZipArchive::new(File::open(path)?)?;
        let mut found = Vec::new();
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index)?;
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;
            found.push((entry.name()?.into_owned(), bytes));
        }
        Ok(found)
    }

    #[test]
    fn a_zip_of_each_kind_of_entry_reads_back() -> TestResult {
        let work = tempfile::tempdir()?;
        let path = work.path().join("a.zip");
        let mut stream = ZipStream::new(File::create(&path)?, 0);
        let text = b"hello hello hello hello hello hello".to_vec();
        stream.add_whole(&WholeEntry {
            name: "files/stored.txt",
            method: ZipMethod::Stored,
            crc32: crc_of(&text),
            uncompressed_bytes: text.len() as u64,
            data: &text,
        })?;
        stream.add_whole(&WholeEntry {
            name: "files/Größe.txt",
            method: ZipMethod::Deflated,
            crc32: crc_of(&text),
            uncompressed_bytes: text.len() as u64,
            data: &deflated(&text),
        })?;
        let mut open = stream.begin("files/streamed stored.bin", ZipMethod::Stored)?;
        stream.write_data(&mut open, &text)?;
        stream.end(open, crc_of(&text), text.len() as u64)?;
        let mut open = stream.begin("files/streamed deflated.bin", ZipMethod::Deflated)?;
        stream.write_data(&mut open, &deflated(&text))?;
        stream.end(open, crc_of(&text), text.len() as u64)?;
        let finished = stream.finish()?;
        drop(finished.file);

        let found = read_back(&path)?;
        assert_eq!(found.len(), 4);
        for (name, bytes) in &found {
            assert_eq!(bytes, &text, "{name}");
        }
        assert_eq!(found[0].0, "files/stored.txt");
        assert_eq!(found[1].0, "files/Größe.txt");
        Ok(())
    }

    #[test]
    fn the_hash_is_of_what_is_in_the_file_and_taking_an_entry_out_leaves_no_trace() -> TestResult {
        let work = tempfile::tempdir()?;
        let with_path = work.path().join("with.zip");
        let without_path = work.path().join("without.zip");
        let text = b"some text".to_vec();
        let add_first = |stream: &mut ZipStream| {
            stream.add_whole(&WholeEntry {
                name: "first.txt",
                method: ZipMethod::Stored,
                crc32: crc_of(&text),
                uncompressed_bytes: text.len() as u64,
                data: &text,
            })
        };

        let mut with = ZipStream::new(File::create(&with_path)?, 0);
        add_first(&mut with)?;
        let mut open = with.begin("gone.bin", ZipMethod::Stored)?;
        with.write_data(&mut open, b"read part of this, and then the drive failed")?;
        with.abandon(open)?;
        let mut open = with.begin("last.bin", ZipMethod::Stored)?;
        with.write_data(&mut open, &text)?;
        with.end(open, crc_of(&text), text.len() as u64)?;
        let with = with.finish()?;
        drop(with.file);

        let mut without = ZipStream::new(File::create(&without_path)?, 0);
        add_first(&mut without)?;
        let mut open = without.begin("last.bin", ZipMethod::Stored)?;
        without.write_data(&mut open, &text)?;
        without.end(open, crc_of(&text), text.len() as u64)?;
        let without = without.finish()?;
        drop(without.file);

        let on_disk = std::fs::read(&with_path)?;
        assert_eq!(on_disk, std::fs::read(&without_path)?);
        assert_eq!(with.sha256.as_slice(), Sha256::digest(&on_disk).as_slice());
        assert_eq!(with.len, on_disk.len() as u64);
        assert_eq!(with.sha256, without.sha256);
        Ok(())
    }

    #[test]
    fn a_zip_can_start_after_what_is_already_in_its_file() -> TestResult {
        let work = tempfile::tempdir()?;
        let path = work.path().join("after.bin");
        let mut file = File::create(&path)?;
        file.write_all(b"a program goes here")?;
        let base = 19;
        let mut stream = ZipStream::new(file, base);
        let mut open = stream.begin("gone.bin", ZipMethod::Stored)?;
        stream.write_data(&mut open, b"not this")?;
        stream.abandon(open)?;
        stream.add_whole(&WholeEntry {
            name: "kept.txt",
            method: ZipMethod::Stored,
            crc32: crc_of(b"kept"),
            uncompressed_bytes: 4,
            data: b"kept",
        })?;
        let finished = stream.finish()?;
        drop(finished.file);

        let on_disk = std::fs::read(&path)?;
        assert_eq!(&on_disk[..19], b"a program goes here");
        let zip = &on_disk[19..];
        assert_eq!(finished.len, zip.len() as u64);
        assert_eq!(finished.sha256.as_slice(), Sha256::digest(zip).as_slice());
        let mut archive = ZipArchive::new(io::Cursor::new(zip.to_vec()))?;
        let mut bytes = Vec::new();
        archive.by_name("kept.txt")?.read_to_end(&mut bytes)?;
        assert_eq!(bytes, b"kept");
        Ok(())
    }

    #[test]
    fn more_entries_than_a_plain_zip_can_count_read_back() -> TestResult {
        let work = tempfile::tempdir()?;
        let path = work.path().join("many.zip");
        let mut stream = ZipStream::new(File::create(&path)?, 0);
        let count = MOST_ENTRIES_WITHOUT_ZIP64 + 5;
        for index in 0..count {
            stream.add_whole(&WholeEntry {
                name: &format!("f/{index}"),
                method: ZipMethod::Stored,
                crc32: crc_of(b""),
                uncompressed_bytes: 0,
                data: b"",
            })?;
        }
        let finished = stream.finish()?;
        drop(finished.file);
        let archive = ZipArchive::new(File::open(&path)?)?;
        assert_eq!(archive.len(), count);
        Ok(())
    }

    #[test]
    fn zip64_fields_go_in_the_record_only_when_they_have_to() {
        let plain = CentralRecord {
            name: b"a".to_vec(),
            method: ZipMethod::Stored,
            flags: 0,
            version_needed: VERSION_PLAIN,
            crc32: 0,
            compressed_bytes: 10,
            uncompressed_bytes: 10,
            offset: 10,
        };
        assert_eq!(central_header(&plain).len(), CENTRAL_HEADER_BYTES + 1);

        let big = CentralRecord {
            compressed_bytes: u64::from(u32::MAX),
            uncompressed_bytes: 5 * 1024 * 1024 * 1024,
            offset: 6 * 1024 * 1024 * 1024,
            ..plain
        };
        // Its ID, its length, and three eight-byte values.
        assert_eq!(
            central_header(&big).len(),
            CENTRAL_HEADER_BYTES + 1 + 4 + 24
        );
    }

    #[test]
    fn a_name_too_long_for_a_zip_is_refused() -> TestResult {
        let work = tempfile::tempdir()?;
        let mut stream = ZipStream::new(File::create(work.path().join("a.zip"))?, 0);
        assert!(
            stream
                .begin(&"x".repeat(70_000), ZipMethod::Stored)
                .is_err()
        );
        Ok(())
    }
}
