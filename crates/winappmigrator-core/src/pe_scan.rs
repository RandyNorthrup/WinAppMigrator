use std::collections::BTreeSet;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use pelite::{PeFile, Wrap};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::errors::{Result, WinAppMigratorError, panic_text};
use crate::models::{
    AppArchitecture, ExecutableMetadata, FileVersionInfo, ProgramNeed, ProgramRole,
    SignatureStatus, Warning, WarningSeverity,
};
use crate::path_utils::file_name_string;

const IMAGE_FILE_MACHINE_ARM64: u16 = 0xaa64;
/// Data-directory index of the delay-load import table.
const DELAY_IMPORT_DIRECTORY_INDEX: usize = 13;
/// Data-directory index of the CLR/COR20 (.NET) descriptor.
const COM_DESCRIPTOR_DIRECTORY_INDEX: usize = 14;
/// A delay-load descriptor (`IMAGE_DELAYLOAD_DESCRIPTOR`) is eight `u32` fields.
const DELAY_DESCRIPTOR_BYTES: usize = 32;
/// Bit 0 of the descriptor attributes: names/tables are RVAs (v2 format). When
/// clear the descriptor is the obsolete v1 format that stored virtual
/// addresses; those are skipped rather than guessing the image base.
const DELAY_ATTR_RVA_BASED: u32 = 0x1;
/// Defensive cap on descriptors walked in a (possibly malformed) table.
const MAX_DELAY_DESCRIPTORS: usize = 4096;
/// Defensive cap on an imported DLL name length.
const MAX_DLL_NAME_LEN: usize = 260;

#[derive(Debug, Clone)]
pub struct PeScanResult {
    pub metadata: ExecutableMetadata,
    pub architecture: AppArchitecture,
    pub direct_imports: Vec<ImportedDll>,
    pub warnings: Vec<Warning>,
}

const MAX_PROGRAM_BYTES: u64 = 4 * 1024 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ImportedDll {
    pub name: String,
    pub delay_loaded: bool,
}

pub fn scan_pe(path: &Path) -> Result<PeScanResult> {
    let bytes = read_program(path)?;
    not_a_panic(path, || scan_pe_bytes(path, &bytes))
}

/// What a program's bytes can do to what reads them stays this file's problem. A version resource
/// that says it is longer than it is makes the parser panic, and that would take the whole scan with
/// it, when any one file on a drive can be whatever someone made it.
fn not_a_panic<T>(path: &Path, read: impl FnOnce() -> Result<T>) -> Result<T> {
    panic::catch_unwind(AssertUnwindSafe(read)).unwrap_or_else(|panicked| {
        Err(WinAppMigratorError::pe(
            path,
            format!("it could not be read: {}", panic_text(&*panicked)),
        ))
    })
}

/// A program's headers, parsed if they can be without trusting what the parser would take on trust:
/// it makes a slice of section headers wherever the optional header says it ends, and one that is not
/// on a four-byte boundary is undefined behaviour, whatever it reads as.
fn parse_program<'a>(path: &Path, bytes: &'a [u8]) -> Result<PeFile<'a>> {
    if !headers_are_where_they_can_be(bytes) {
        return Err(WinAppMigratorError::pe(
            path,
            "its headers are not on the boundaries a program's have to be".to_owned(),
        ));
    }
    PeFile::from_bytes(bytes).map_err(|source| WinAppMigratorError::pe(path, source.to_string()))
}

fn headers_are_where_they_can_be(bytes: &[u8]) -> bool {
    const DOS_HEADER_OFFSET_FIELD: usize = 0x3C;
    const OPTIONAL_HEADER_SIZE_FIELD: usize = 20;
    const OPTIONAL_HEADER_START: usize = 24;
    const BOUNDARY: usize = 4;
    let Some(field) = bytes.get(DOS_HEADER_OFFSET_FIELD..DOS_HEADER_OFFSET_FIELD + 4) else {
        return false;
    };
    let headers = u32::from_le_bytes([field[0], field[1], field[2], field[3]]) as usize;
    let Some(size) = headers
        .checked_add(OPTIONAL_HEADER_SIZE_FIELD)
        .and_then(|at| bytes.get(at..at.checked_add(2)?))
    else {
        return false;
    };
    let optional_size = u16::from_le_bytes([size[0], size[1]]) as usize;
    headers.is_multiple_of(BOUNDARY)
        && (headers + OPTIONAL_HEADER_START + optional_size).is_multiple_of(BOUNDARY)
}

fn scan_pe_bytes(path: &Path, bytes: &[u8]) -> Result<PeScanResult> {
    let pe = parse_program(path, bytes)?;
    let metadata = file_metadata(path, &pe, bytes)?;
    let architecture = architecture_from_machine(pe.file_header().Machine);
    let mut warnings = Vec::new();
    let mut direct_imports = read_imports(path, &pe, &mut warnings);
    merge_delay_imports(
        &mut direct_imports,
        read_delay_imports(&pe, bytes, &mut warnings),
    );

    Ok(PeScanResult {
        metadata,
        architecture,
        direct_imports,
        warnings,
    })
}

/// Append delay-load imports that are not already present as normal imports.
/// When a DLL is imported both normally and via delay-load, the normal entry is
/// kept (it is loaded eagerly), so only genuinely delay-only DLLs are added.
fn merge_delay_imports(imports: &mut Vec<ImportedDll>, delay: Vec<ImportedDll>) {
    let existing: BTreeSet<String> = imports
        .iter()
        .map(|import| import.name.to_ascii_lowercase())
        .collect();
    for import in delay {
        if !existing.contains(&import.name.to_ascii_lowercase()) {
            imports.push(import);
        }
    }
    imports.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });
}

/// A program's file, as far as anything here looks at it. Windows will not start one of the size
/// this refuses, and on a drive from elsewhere, one could be made big enough to run this out of memory.
/// Most of a large program is code, which nothing here reads, and reading it would be the second or
/// third time that file was read for a build.
fn read_program(path: &Path) -> Result<Vec<u8>> {
    let size = fs::metadata(path)
        .map_err(|source| WinAppMigratorError::io(path, source))?
        .len();
    if size > MAX_PROGRAM_BYTES {
        return Err(WinAppMigratorError::pe(
            path,
            format!("it is {size} bytes, too large to be a program"),
        ));
    }
    if size > READ_WHOLE_UP_TO_BYTES
        && let Some(bytes) = read_what_is_looked_at(path, size)
    {
        return Ok(bytes);
    }
    fs::read(path).map_err(|source| WinAppMigratorError::io(path, source))
}

/// A program of this size or less is read whole, as it costs next to nothing.
const READ_WHOLE_UP_TO_BYTES: u64 = 8 * 1024 * 1024;
const IMPORT_DIRECTORY_INDEX: usize = 1;
const RESOURCE_DIRECTORY_INDEX: usize = 2;
const SECURITY_DIRECTORY_INDEX: usize = 4;
const IMPORT_DESCRIPTOR_BYTES: usize = 20;
const IMPORT_DESCRIPTOR_NAME_FIELD: usize = 12;
const DELAY_DESCRIPTOR_NAME_FIELD: usize = 4;
const MOST_DESCRIPTORS_FOLLOWED: usize = 4096;

/// One section of a program, as its headers say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SectionPlace {
    rva: u32,
    virtual_size: u32,
    file_offset: u64,
    file_size: u64,
}

/// Where a program keeps things, from its headers alone.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ProgramLayout {
    sections: Vec<SectionPlace>,
    /// The address and size of each data directory.
    directories: Vec<(u32, u32)>,
}

impl ProgramLayout {
    fn section_of(&self, rva: u32) -> Option<&SectionPlace> {
        self.sections.iter().find(|section| {
            let size = section
                .virtual_size
                .max(u32::try_from(section.file_size).unwrap_or(u32::MAX));
            rva >= section.rva && u64::from(rva) < u64::from(section.rva) + u64::from(size)
        })
    }

    fn file_offset_of(&self, rva: u32) -> Option<usize> {
        let section = self.section_of(rva)?;
        let into = u64::from(rva - section.rva);
        (into < section.file_size)
            .then(|| usize::try_from(section.file_offset + into).ok())
            .flatten()
    }
}

/// Read a program's section table. `None` if it is not all here, or it is not a program.
fn program_layout(start: &[u8]) -> Option<ProgramLayout> {
    const SECTION_HEADER_BYTES: usize = 40;
    const SECTION_COUNT_FIELD: usize = 6;
    const OPTIONAL_HEADER_SIZE_FIELD: usize = 20;
    const SECTION_VIRTUAL_SIZE_FIELD: usize = 8;
    const SECTION_RVA_FIELD: usize = 12;
    const SECTION_RAW_SIZE_FIELD: usize = 16;
    const SECTION_RAW_POINTER_FIELD: usize = 20;
    const PE32_DIRECTORY_COUNT_FIELD: usize = 92;
    const PE32_PLUS_DIRECTORY_COUNT_FIELD: usize = 108;
    header_facts(start)?;
    let pe = usize::try_from(u32_at(start, PE_OFFSET_FIELD)?).ok()?;
    let sections_in_all = usize::from(u16_at(start, pe + SECTION_COUNT_FIELD)?);
    let optional_size = usize::from(u16_at(start, pe + OPTIONAL_HEADER_SIZE_FIELD)?);
    let optional = pe + PE_SIGNATURE.len() + COFF_HEADER_BYTES;
    let (count_field, first_directory) = match u16_at(start, optional)? {
        OPTIONAL_MAGIC_PE32 => (PE32_DIRECTORY_COUNT_FIELD, PE32_DIRECTORIES_FIELD),
        OPTIONAL_MAGIC_PE32_PLUS => (PE32_PLUS_DIRECTORY_COUNT_FIELD, PE32_PLUS_DIRECTORIES_FIELD),
        _ => return None,
    };
    let directory_count = usize::try_from(u32_at(start, optional + count_field)?).ok()?;
    let mut directories = Vec::new();
    for index in 0..directory_count.min(16) {
        let entry = optional + first_directory + index * DIRECTORY_ENTRY_BYTES;
        directories.push((u32_at(start, entry)?, u32_at(start, entry + 4)?));
    }
    let table = optional + optional_size;
    let mut sections = Vec::with_capacity(sections_in_all);
    for index in 0..sections_in_all {
        let header = table + index * SECTION_HEADER_BYTES;
        sections.push(SectionPlace {
            rva: u32_at(start, header + SECTION_RVA_FIELD)?,
            virtual_size: u32_at(start, header + SECTION_VIRTUAL_SIZE_FIELD)?,
            file_offset: u64::from(u32_at(start, header + SECTION_RAW_POINTER_FIELD)?),
            file_size: u64::from(u32_at(start, header + SECTION_RAW_SIZE_FIELD)?),
        });
    }
    Some(ProgramLayout {
        sections,
        directories,
    })
}

/// A program's file with only what is looked at in it read, and zeros in the rest, which is memory
/// nothing touches. That is its headers, the sections with its import tables, the names those give,
/// its resources, and its signature. A managed program's metadata can be anywhere, so that is read
/// whole, as is anything that does not read as expected. `None` means to read it whole.
fn read_what_is_looked_at(path: &Path, size: u64) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;
    let start = read_start_of(path).ok()?;
    let layout = program_layout(&start)?;
    if header_facts(&start)?.is_managed {
        return None;
    }
    let mut bytes = vec![0_u8; usize::try_from(size).ok()?];
    bytes.get_mut(..start.len())?.copy_from_slice(&start);
    // Sections can all say they are the whole file, and a read of a part should not be many of all
    // of it. Past that, it is read whole, which is one.
    let mut read_so_far = 0_u64;
    let mut read_section = |bytes: &mut Vec<u8>, section: &SectionPlace| -> Option<()> {
        let from = section.file_offset;
        let to = from.checked_add(section.file_size)?.min(size);
        if from >= to {
            return Some(());
        }
        read_so_far = read_so_far.saturating_add(to - from);
        if read_so_far > size {
            return None;
        }
        file.seek(SeekFrom::Start(from)).ok()?;
        let place = bytes.get_mut(usize::try_from(from).ok()?..usize::try_from(to).ok()?)?;
        file.read_exact(place).ok()
    };
    let mut loaded: Vec<u32> = Vec::new();
    let mut load = |bytes: &mut Vec<u8>, rva: u32| -> Option<()> {
        let Some(section) = layout.section_of(rva) else {
            return Some(());
        };
        if !loaded.contains(&section.rva) {
            loaded.push(section.rva);
            read_section(bytes, section)?;
        }
        Some(())
    };
    let directory = |index: usize| {
        layout
            .directories
            .get(index)
            .copied()
            .filter(|(address, size)| *address != 0 && *size != 0)
    };
    if let Some((address, _)) = directory(RESOURCE_DIRECTORY_INDEX) {
        load(&mut bytes, address)?;
    }
    // Each table, then the names it gives, which a linker can put in another section.
    for (index, entry_bytes, name_field) in [
        (
            IMPORT_DIRECTORY_INDEX,
            IMPORT_DESCRIPTOR_BYTES,
            IMPORT_DESCRIPTOR_NAME_FIELD,
        ),
        (
            DELAY_IMPORT_DIRECTORY_INDEX,
            DELAY_DESCRIPTOR_BYTES,
            DELAY_DESCRIPTOR_NAME_FIELD,
        ),
    ] {
        let Some((address, _)) = directory(index) else {
            continue;
        };
        load(&mut bytes, address)?;
        let Some(mut entry) = layout.file_offset_of(address) else {
            continue;
        };
        for _ in 0..MOST_DESCRIPTORS_FOLLOWED {
            let Some(descriptor) = bytes.get(entry..entry + entry_bytes) else {
                break;
            };
            if descriptor.iter().all(|byte| *byte == 0) {
                break;
            }
            if let Some(name) = u32_at(&bytes, entry + name_field) {
                load(&mut bytes, name)?;
            }
            entry += entry_bytes;
        }
    }
    // A signature is by where it is in the file, past the sections.
    if let Some((offset, table_size)) = directory(SECURITY_DIRECTORY_INDEX) {
        let from = u64::from(offset);
        let to = from.checked_add(u64::from(table_size))?.min(size);
        if from < to {
            file.seek(SeekFrom::Start(from)).ok()?;
            let place = bytes.get_mut(usize::try_from(from).ok()?..usize::try_from(to).ok()?)?;
            file.read_exact(place).ok()?;
        }
    }
    Some(bytes)
}

/// What a program needs in order to run, read from nothing but its own file.
#[derive(Debug, Clone)]
pub(crate) struct ProgramNeeds {
    pub architecture: AppArchitecture,
    pub imports: Vec<ImportedDll>,
    /// Only for a managed image.
    pub clr_version: Option<String>,
    pub is_managed: bool,
    pub managed: ManagedImageFacts,
    /// A .NET program packed into one file, with what it runs on inside it.
    pub is_a_single_file_bundle: bool,
}

/// Read what a program needs, leaving out everything a scan of the main program also looks at.
pub(crate) fn read_program_needs(path: &Path) -> Result<ProgramNeeds> {
    let bytes = read_program(path)?;
    not_a_panic(path, || program_needs_of_bytes(path, &bytes))
}

fn program_needs_of_bytes(path: &Path, bytes: &[u8]) -> Result<ProgramNeeds> {
    let pe = parse_program(path, bytes)?;
    let mut warnings = Vec::new();
    let mut imports = read_imports(path, &pe, &mut warnings);
    merge_delay_imports(&mut imports, read_delay_imports(&pe, bytes, &mut warnings));
    let is_managed = is_dotnet_image(&pe);
    Ok(ProgramNeeds {
        architecture: architecture_from_machine(pe.file_header().Machine),
        imports,
        clr_version: is_managed.then(|| dotnet_metadata_version(bytes)).flatten(),
        is_managed,
        managed: if is_managed {
            managed_image_facts(bytes)
        } else {
            ManagedImageFacts::default()
        },
        is_a_single_file_bundle: !is_managed && find_bytes(bytes, &SINGLE_FILE_SIGNATURE).is_some(),
    })
}

/// What `.NET` writes into a program it has packed into one file, to find what is in it by. It has
/// no `runtimeconfig.json` on disk and looks like a native program.
const SINGLE_FILE_SIGNATURE: [u8; 32] = [
    0x8b, 0x12, 0x02, 0xb9, 0x6a, 0x61, 0x20, 0x38, 0x72, 0x7b, 0x93, 0x02, 0x14, 0xd7, 0xa0, 0x32,
    0x13, 0xf5, 0xb9, 0xe6, 0xef, 0xae, 0x33, 0x18, 0xee, 0x3b, 0x2d, 0xce, 0x24, 0xb3, 0x6a, 0xae,
];

/// A file's version as Windows would show it, for telling which of two copies of a runtime is newer.
pub(crate) fn file_version_of(path: &Path) -> Option<String> {
    let bytes = read_program(path).ok()?;
    not_a_panic(path, || {
        let pe = parse_program(path, &bytes)?;
        Ok(read_version_info(&pe).and_then(|info| info.file_version))
    })
    .ok()
    .flatten()
}

/// Which architecture a program is built for, from its headers alone.
pub fn read_pe_architecture(path: &Path) -> Result<AppArchitecture> {
    let start = read_start_of(path)?;
    let facts = header_facts(&start)
        .ok_or_else(|| WinAppMigratorError::pe(path, "it has no PE headers".to_owned()))?;
    Ok(architecture_from_machine(facts.machine))
}

/// How much of a program's file holds its headers, nearly always. One that puts them further along
/// is read as far as that takes.
const HEADER_BYTES: usize = 4096;
/// Where a PE says its headers start, in the DOS header.
const PE_OFFSET_FIELD: usize = 0x3C;
/// The furthest along a real linker puts them. Any further is not a program worth believing.
const MOST_PE_OFFSET: usize = 1024 * 1024;
const PE_SIGNATURE: &[u8] = b"PE\0\0";
const COFF_HEADER_BYTES: usize = 20;
/// Offsets from the start of the optional header, the same in both formats up to the directories.
const OPTIONAL_SUBSYSTEM_MAJOR_FIELD: usize = 48;
const OPTIONAL_SUBSYSTEM_MINOR_FIELD: usize = 50;
const OPTIONAL_MAGIC_PE32: u16 = 0x10B;
const OPTIONAL_MAGIC_PE32_PLUS: u16 = 0x20B;
const PE32_DIRECTORIES_FIELD: usize = 96;
const PE32_PLUS_DIRECTORIES_FIELD: usize = 112;
const DIRECTORY_ENTRY_BYTES: usize = 8;

/// What a program's headers say of what it needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HeaderFacts {
    pub machine: u16,
    pub subsystem_major: u16,
    pub subsystem_minor: u16,
    pub is_managed: bool,
}

fn u16_at(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let found = bytes.get(offset..end)?;
    Some(u16::from_le_bytes([found[0], found[1]]))
}

fn u32_at(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let found = bytes.get(offset..end)?;
    Some(u32::from_le_bytes([found[0], found[1], found[2], found[3]]))
}

/// Read a program's headers out of however much of its start is here. `None` if they are not all
/// here, or it is not a program.
pub(crate) fn header_facts(bytes: &[u8]) -> Option<HeaderFacts> {
    if bytes.get(..2) != Some(b"MZ".as_slice()) {
        return None;
    }
    let pe = usize::try_from(u32_at(bytes, PE_OFFSET_FIELD)?).ok()?;
    if bytes.get(pe..pe.checked_add(PE_SIGNATURE.len())?) != Some(PE_SIGNATURE) {
        return None;
    }
    let machine = u16_at(bytes, pe + PE_SIGNATURE.len())?;
    let optional = pe + PE_SIGNATURE.len() + COFF_HEADER_BYTES;
    let directories = match u16_at(bytes, optional)? {
        OPTIONAL_MAGIC_PE32 => optional + PE32_DIRECTORIES_FIELD,
        OPTIONAL_MAGIC_PE32_PLUS => optional + PE32_PLUS_DIRECTORIES_FIELD,
        _ => return None,
    };
    let descriptor = directories + COM_DESCRIPTOR_DIRECTORY_INDEX * DIRECTORY_ENTRY_BYTES;
    Some(HeaderFacts {
        machine,
        subsystem_major: u16_at(bytes, optional + OPTIONAL_SUBSYSTEM_MAJOR_FIELD)?,
        subsystem_minor: u16_at(bytes, optional + OPTIONAL_SUBSYSTEM_MINOR_FIELD)?,
        // A short directory table means no such entry, not a damaged program.
        is_managed: u32_at(bytes, descriptor).is_some_and(|address| address != 0),
    })
}

/// The first part of a file, enough for its headers.
fn read_start_of(path: &Path) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let mut bytes = Vec::with_capacity(HEADER_BYTES);
    (&mut file)
        .take(HEADER_BYTES as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| WinAppMigratorError::io(path, source))?;
    if bytes.len() == HEADER_BYTES
        && header_facts(&bytes).is_none()
        && let Some(offset) = u32_at(&bytes, PE_OFFSET_FIELD)
            .and_then(|offset| usize::try_from(offset).ok())
            .filter(|offset| (HEADER_BYTES..=MOST_PE_OFFSET).contains(offset))
    {
        bytes.clear();
        file.seek(SeekFrom::Start(0))
            .map_err(|source| WinAppMigratorError::io(path, source))?;
        file.take((offset + HEADER_BYTES) as u64)
            .read_to_end(&mut bytes)
            .map_err(|source| WinAppMigratorError::io(path, source))?;
    }
    Ok(bytes)
}

/// What a program needs from Windows, from its headers. `None` if it is not a program.
pub(crate) fn read_program_need(
    readable_at: &Path,
    logical: &Path,
    role: ProgramRole,
) -> Option<ProgramNeed> {
    let facts = header_facts(&read_start_of(readable_at).ok()?)?;
    Some(ProgramNeed {
        path: logical.to_path_buf(),
        role,
        machine: facts.machine,
        min_windows_major: facts.subsystem_major,
        min_windows_minor: facts.subsystem_minor,
        is_managed: facts.is_managed,
    })
}

fn file_metadata(path: &Path, pe: &PeFile<'_>, bytes: &[u8]) -> Result<ExecutableMetadata> {
    let metadata = fs::metadata(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let modified_utc = metadata.modified().ok().and_then(format_system_time_utc);
    // Only that it is there. Following a link to find out could connect to a share.
    let external_manifest = external_manifest_path(path)
        .filter(|manifest_path| fs::symlink_metadata(manifest_path).is_ok());

    let embedded_manifest_xml = read_embedded_manifest(pe);
    let requested_execution_level = embedded_manifest_xml
        .as_deref()
        .and_then(requested_execution_level);
    let sxs_dependencies = embedded_manifest_xml
        .as_deref()
        .map(sxs_dependencies)
        .unwrap_or_default();

    Ok(ExecutableMetadata {
        path: path.to_path_buf(),
        file_name: file_name_string(path),
        file_size_bytes: metadata.len(),
        modified_utc,
        version_info: read_version_info(pe),
        digital_signature: signature_status(pe),
        external_manifest,
        is_dotnet: is_dotnet_image(pe),
        dotnet_runtime_version: is_dotnet_image(pe)
            .then(|| dotnet_metadata_version(bytes))
            .flatten(),
        dotnet_target_framework: is_dotnet_image(pe)
            .then(|| managed_image_facts(bytes).target_framework)
            .flatten(),
        embedded_manifest: embedded_manifest_xml.is_some(),
        requested_execution_level,
        sxs_dependencies,
    })
}

fn read_imports(path: &Path, pe: &PeFile<'_>, warnings: &mut Vec<Warning>) -> Vec<ImportedDll> {
    let imports = match pe.imports() {
        Ok(imports) => imports,
        Err(error) if error.is_null() => return Vec::new(),
        Err(error) => {
            warnings.push(Warning::new(
                "pe_imports_unreadable",
                WarningSeverity::Warning,
                format!("Import table could not be read: {error}"),
                Some(path.display().to_string()),
            ));
            return Vec::new();
        }
    };

    let mut names = Vec::new();
    for descriptor in imports {
        let dll_name = match descriptor {
            Wrap::T32(descriptor) => descriptor.dll_name(),
            Wrap::T64(descriptor) => descriptor.dll_name(),
        };

        match dll_name {
            Ok(name) => names.push(ImportedDll {
                name: name.to_string(),
                delay_loaded: false,
            }),
            Err(error) => warnings.push(Warning::new(
                "pe_import_name_unreadable",
                WarningSeverity::Warning,
                format!("An import descriptor name could not be read: {error}"),
                Some(path.display().to_string()),
            )),
        }
    }

    names.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });
    names.dedup_by(|left, right| left.name.eq_ignore_ascii_case(&right.name));
    names
}

fn read_version_info(pe: &PeFile<'_>) -> Option<FileVersionInfo> {
    // Its own, so that a program with a resource like this is only a program with no version.
    panic::catch_unwind(AssertUnwindSafe(|| {
        let resources = pe.resources().ok()?;
        let version_info = resources.version_info().ok()?;
        let mut collector = VersionCollector::default();
        version_info.visit(&mut collector);
        Some(collector.into_version_info())
    }))
    .ok()
    .flatten()
}

#[derive(Default)]
struct VersionCollector {
    file_version: Option<String>,
    product_version: Option<String>,
    company_name: Option<String>,
    product_name: Option<String>,
    file_description: Option<String>,
}

impl VersionCollector {
    fn into_version_info(self) -> FileVersionInfo {
        FileVersionInfo {
            file_version: self.file_version,
            product_version: self.product_version,
            company_name: self.company_name,
            product_name: self.product_name,
            file_description: self.file_description,
        }
    }
}

impl<'a> pelite::resources::version_info::Visit<'a> for VersionCollector {
    fn version_info(
        &mut self,
        _key: &[u16],
        fixed: Option<&pelite::image::VS_FIXEDFILEINFO>,
    ) -> bool {
        if let Some(fixed) = fixed {
            self.file_version.get_or_insert_with(|| {
                format!(
                    "{}.{}.{}.{}",
                    fixed.dwFileVersion.Major,
                    fixed.dwFileVersion.Minor,
                    fixed.dwFileVersion.Patch,
                    fixed.dwFileVersion.Build
                )
            });
            self.product_version.get_or_insert_with(|| {
                format!(
                    "{}.{}.{}.{}",
                    fixed.dwProductVersion.Major,
                    fixed.dwProductVersion.Minor,
                    fixed.dwProductVersion.Patch,
                    fixed.dwProductVersion.Build
                )
            });
        }
        true
    }

    fn string(&mut self, key: &[u16], value: &[u16]) {
        let key = String::from_utf16_lossy(key);
        let value = String::from_utf16_lossy(value);
        match key.as_str() {
            "CompanyName" => self.company_name = Some(value),
            "FileDescription" => self.file_description = Some(value),
            "FileVersion" => self.file_version = Some(value),
            "ProductName" => self.product_name = Some(value),
            "ProductVersion" => self.product_version = Some(value),
            _ => {}
        }
    }
}

fn signature_status(pe: &PeFile<'_>) -> SignatureStatus {
    match pe.security() {
        Ok(security) if !security.certificate_data().is_empty() => {
            SignatureStatus::EmbeddedSignaturePresent
        }
        Ok(_) => SignatureStatus::Unknown,
        Err(error) if error.is_null() => SignatureStatus::NotPresent,
        Err(_) => SignatureStatus::Unknown,
    }
}

fn external_manifest_path(path: &Path) -> Option<PathBuf> {
    let file_name = path.file_name()?.to_str()?;
    Some(path.with_file_name(format!("{file_name}.manifest")))
}

/// A PE section's address mapping, used to translate an RVA into a file offset
/// without relying on a parser that does not expose the delay-import table.
#[derive(Debug, Clone, Copy)]
struct SectionSpan {
    virtual_address: u32,
    virtual_size: u32,
    raw_pointer: u32,
    raw_size: u32,
}

fn section_spans(pe: &PeFile<'_>) -> Vec<SectionSpan> {
    pe.section_headers()
        .iter()
        .map(|section| SectionSpan {
            virtual_address: section.VirtualAddress,
            virtual_size: section.VirtualSize,
            raw_pointer: section.PointerToRawData,
            raw_size: section.SizeOfRawData,
        })
        .collect()
}

/// Read the delay-load import directory. `pelite` does not parse delay imports,
/// so the descriptor table is walked directly over the file bytes. Only the
/// modern RVA-based (v2) format is parsed; obsolete VA-based (v1) descriptors
/// are reported and skipped rather than guessing the image base.
fn read_delay_imports(
    pe: &PeFile<'_>,
    bytes: &[u8],
    warnings: &mut Vec<Warning>,
) -> Vec<ImportedDll> {
    let Some(directory) = pe.data_directory().get(DELAY_IMPORT_DIRECTORY_INDEX) else {
        return Vec::new();
    };
    if directory.VirtualAddress == 0 || directory.Size == 0 {
        return Vec::new();
    }
    let spans = section_spans(pe);
    let (names, legacy_skipped) = read_delay_import_names(bytes, &spans, directory.VirtualAddress);
    if legacy_skipped > 0 {
        warnings.push(Warning::new(
            "delay_import_legacy_format",
            WarningSeverity::Info,
            format!(
                "{legacy_skipped} legacy (v1) delay-load descriptor(s) were skipped; only modern RVA-based delay imports are parsed."
            ),
            None,
        ));
    }
    names
        .into_iter()
        .map(|name| ImportedDll {
            name,
            delay_loaded: true,
        })
        .collect()
}

/// Walk the delay-load descriptor table and collect imported DLL names.
/// Returns the (sorted, de-duplicated) names and the count of skipped legacy
/// descriptors. Pure over the inputs so it can be unit-tested with crafted
/// bytes.
fn read_delay_import_names(
    bytes: &[u8],
    spans: &[SectionSpan],
    directory_rva: u32,
) -> (Vec<String>, usize) {
    let mut names = Vec::new();
    let mut seen = BTreeSet::new();
    let mut legacy_skipped = 0usize;
    for index in 0..MAX_DELAY_DESCRIPTORS {
        let descriptor_rva = directory_rva.wrapping_add((index * DELAY_DESCRIPTOR_BYTES) as u32);
        let Some(offset) = rva_to_offset(spans, descriptor_rva) else {
            break;
        };
        let Some(fields) = read_u32_le_array::<8>(bytes, offset) else {
            break;
        };
        if fields.iter().all(|field| *field == 0) {
            break;
        }
        let attributes = fields[0];
        let name_field = fields[1];
        if name_field == 0 {
            break;
        }
        if attributes & DELAY_ATTR_RVA_BASED == 0 {
            legacy_skipped += 1;
            continue;
        }
        if let Some(name_offset) = rva_to_offset(spans, name_field)
            && let Some(name) = read_ascii_c_string(bytes, name_offset)
            && seen.insert(name.to_ascii_lowercase())
        {
            names.push(name);
        }
    }
    names.sort_by_key(|name| name.to_ascii_lowercase());
    (names, legacy_skipped)
}

fn rva_to_offset(spans: &[SectionSpan], rva: u32) -> Option<usize> {
    for span in spans {
        let span_size = span.virtual_size.max(span.raw_size);
        let end = span.virtual_address.checked_add(span_size)?;
        if rva >= span.virtual_address && rva < end {
            let delta = rva - span.virtual_address;
            if delta >= span.raw_size {
                // Address falls in a virtual-only (uninitialized) region with no
                // backing file bytes.
                return None;
            }
            return Some(span.raw_pointer.checked_add(delta)? as usize);
        }
    }
    None
}

fn read_u32_le_array<const N: usize>(bytes: &[u8], offset: usize) -> Option<[u32; N]> {
    let end = offset.checked_add(N * 4)?;
    let slice = bytes.get(offset..end)?;
    let mut output = [0u32; N];
    for (index, value) in output.iter_mut().enumerate() {
        let start = index * 4;
        *value = u32::from_le_bytes([
            slice[start],
            slice[start + 1],
            slice[start + 2],
            slice[start + 3],
        ]);
    }
    Some(output)
}

fn read_ascii_c_string(bytes: &[u8], offset: usize) -> Option<String> {
    let slice = bytes.get(offset..)?;
    let mut output = String::new();
    for &byte in slice.iter().take(MAX_DLL_NAME_LEN) {
        if byte == 0 {
            return (!output.is_empty()).then_some(output);
        }
        if !byte.is_ascii_graphic() && byte != b' ' {
            return None;
        }
        output.push(byte as char);
    }
    None
}

/// True when the PE declares a CLR/COR20 (.NET) data directory.
fn is_dotnet_image(pe: &PeFile<'_>) -> bool {
    pe.data_directory()
        .get(COM_DESCRIPTOR_DIRECTORY_INDEX)
        .is_some_and(|directory| directory.VirtualAddress != 0 && directory.Size != 0)
}

/// What a managed image's metadata says about which .NET it was built for. `v4.0.30319` is the
/// metadata version of .NET Framework 4.x and of modern .NET alike, so it takes this to tell them
/// apart.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ManagedImageFacts {
    /// The `TargetFramework` attribute's text, such as `.NETCoreApp,Version=v8.0`.
    pub target_framework: Option<String>,
    /// It refers to `mscorlib`, which only .NET Framework has as its core library.
    pub refers_to_mscorlib: bool,
    /// It refers to WPF or Windows Forms, which take the Windows Desktop runtime.
    pub refers_to_desktop: bool,
    /// It refers to ASP.NET Core.
    pub refers_to_asp_net: bool,
}

const TARGET_FRAMEWORK_PREFIXES: &[&[u8]] = &[b".NETCoreApp,Version=v", b".NETFramework,Version=v"];
const DESKTOP_ASSEMBLIES: &[&[u8]] = &[
    b"\0PresentationFramework\0",
    b"\0PresentationCore\0",
    b"\0System.Windows.Forms\0",
];
const ASP_NET_ASSEMBLY_PREFIX: &[u8] = b"\0Microsoft.AspNetCore";
const MSCORLIB_ASSEMBLY: &[u8] = b"\0mscorlib\0";
/// More than the longest text there is any reason for.
const MAX_TARGET_FRAMEWORK_CHARS: usize = 64;

/// Read a managed image's metadata streams. Offsets in a file can say anything, so all of it is
/// checked, and what does not hold together says nothing.
pub(crate) fn managed_image_facts(bytes: &[u8]) -> ManagedImageFacts {
    let Some((strings, blob)) = metadata_streams(bytes) else {
        return ManagedImageFacts::default();
    };
    let target_framework = TARGET_FRAMEWORK_PREFIXES.iter().find_map(|prefix| {
        let start = find_bytes(blob, prefix)?;
        let text: Vec<u8> = blob[start..]
            .iter()
            .copied()
            .take(MAX_TARGET_FRAMEWORK_CHARS)
            .take_while(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b',' | b'=' | b'-')
            })
            .collect();
        String::from_utf8(text).ok()
    });
    ManagedImageFacts {
        target_framework,
        refers_to_mscorlib: find_bytes(strings, MSCORLIB_ASSEMBLY).is_some(),
        refers_to_desktop: DESKTOP_ASSEMBLIES
            .iter()
            .any(|name| find_bytes(strings, name).is_some()),
        refers_to_asp_net: find_bytes(strings, ASP_NET_ASSEMBLY_PREFIX).is_some(),
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// The `#Strings` and `#Blob` streams of the metadata root.
/// Where a managed image's metadata is, by what its headers say. Its signature can be anywhere else in
/// a file as well, in a string or in what is appended, and a program's own data can say what it likes.
/// Only what cannot be parsed as a program at all is searched for it.
fn metadata_root_offset(bytes: &[u8]) -> Option<usize> {
    const SIGNATURE: &[u8; 4] = b"BSJB";
    const CLI_HEADER_DIRECTORY: usize = 14;
    const METADATA_RVA_FIELD: usize = 8;
    let Some(layout) = program_layout(bytes) else {
        return find_bytes(bytes, SIGNATURE);
    };
    let (cli_header, _) = layout.directories.get(CLI_HEADER_DIRECTORY).copied()?;
    let at = usize::try_from(layout.file_offset_of(cli_header)?).ok()?;
    let field =
        bytes.get(at.checked_add(METADATA_RVA_FIELD)?..at.checked_add(METADATA_RVA_FIELD + 4)?)?;
    let metadata = u32::from_le_bytes([field[0], field[1], field[2], field[3]]);
    let root = usize::try_from(layout.file_offset_of(metadata)?).ok()?;
    (bytes.get(root..root.checked_add(SIGNATURE.len())?)? == SIGNATURE).then_some(root)
}

fn metadata_streams(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    const HEADER_PREFIX: usize = 12;
    const MAX_STREAMS: u16 = 16;
    const MAX_STREAM_NAME: usize = 32;
    let u32_at = |offset: usize| -> Option<usize> {
        let field = bytes.get(offset..offset.checked_add(4)?)?;
        Some(u32::from_le_bytes([field[0], field[1], field[2], field[3]]) as usize)
    };
    let root = metadata_root_offset(bytes)?;
    let version_length = u32_at(root + HEADER_PREFIX)?;
    // The version string, then a `u16` of flags, and a `u16` count of streams.
    let count_at = root
        .checked_add(HEADER_PREFIX + 4)?
        .checked_add(version_length)?
        .checked_add(2)?;
    let count_field = bytes.get(count_at..count_at.checked_add(2)?)?;
    let count = u16::from_le_bytes([count_field[0], count_field[1]]).min(MAX_STREAMS);
    let mut at = count_at + 2;
    let mut strings: &[u8] = &[];
    let mut blob: &[u8] = &[];
    for _ in 0..count {
        let start = root.checked_add(u32_at(at)?)?;
        let size = u32_at(at + 4)?;
        let name_at = at + 8;
        let name = bytes.get(name_at..name_at.checked_add(MAX_STREAM_NAME)?.min(bytes.len()))?;
        let name_length = name.iter().position(|byte| *byte == 0)?;
        let stream = bytes
            .get(start..start.checked_add(size)?)
            .unwrap_or_default();
        match &name[..name_length] {
            b"#Strings" => strings = stream,
            b"#Blob" => blob = stream,
            _ => {}
        }
        // A stream's name is padded to four bytes, ending in at least one NUL.
        at = name_at + (name_length + 4) / 4 * 4;
    }
    Some((strings, blob))
}

/// The version string in a managed image's CLI metadata root, e.g. `v2.0.50727`
/// or `v4.0.30319`. The metadata root begins with the `BSJB` signature followed
/// by two `u16` version fields, a reserved `u32`, a `u32` length, then the
/// UTF-8 version string of that length. Scanning the file bytes for `BSJB` finds
/// it without mapping RVAs, and a managed image has exactly one such root.
fn dotnet_metadata_version(bytes: &[u8]) -> Option<String> {
    const HEADER_PREFIX: usize = 12; // signature(4) + major(2) + minor(2) + reserved(4)
    const MAX_VERSION_LEN: usize = 256;

    let start = metadata_root_offset(bytes)?;
    let length_offset = start + HEADER_PREFIX;
    let string_offset = length_offset + 4;
    if string_offset > bytes.len() {
        return None;
    }
    let length = u32::from_le_bytes([
        bytes[length_offset],
        bytes[length_offset + 1],
        bytes[length_offset + 2],
        bytes[length_offset + 3],
    ]) as usize;
    if length == 0 || length > MAX_VERSION_LEN {
        return None;
    }
    let end = string_offset.checked_add(length)?;
    let raw = bytes.get(string_offset..end.min(bytes.len()))?;
    let text: String = raw
        .iter()
        .take_while(|&&byte| byte != 0)
        .map(|&byte| byte as char)
        .collect();
    let trimmed = text.trim();
    // A real runtime string starts with 'v' and a digit (e.g. `v4.0.30319`).
    let mut chars = trimmed.chars();
    match (chars.next(), chars.next()) {
        (Some('v'), Some(second)) if second.is_ascii_digit() => Some(trimmed.to_owned()),
        _ => None,
    }
}

fn read_embedded_manifest(pe: &PeFile<'_>) -> Option<String> {
    pe.resources()
        .ok()
        .and_then(|resources| resources.manifest().ok())
        .map(ToOwned::to_owned)
}

/// Extract the `requestedExecutionLevel` (`asInvoker` / `highestAvailable` /
/// `requireAdministrator`) from a manifest.
fn requested_execution_level(manifest: &str) -> Option<String> {
    attribute_value(manifest, "requestedExecutionLevel", "level")
}

/// Extract the side-by-side assembly identities declared as dependencies in a
/// manifest (`<dependency><dependentAssembly><assemblyIdentity name=.../>`).
fn sxs_dependencies(manifest: &str) -> Vec<String> {
    const OPEN: &str = "<dependentAssembly";
    const CLOSE: &str = "</dependentAssembly>";
    let mut dependencies = Vec::new();
    let mut seen = BTreeSet::new();
    let mut cursor = 0;
    while let Some(relative) = manifest[cursor..].find(OPEN) {
        let start = cursor + relative;
        let end = manifest[start..]
            .find(CLOSE)
            .map(|offset| start + offset + CLOSE.len())
            .unwrap_or(manifest.len());
        if let Some(name) = attribute_value(&manifest[start..end], "assemblyIdentity", "name")
            && seen.insert(name.to_ascii_lowercase())
        {
            dependencies.push(name);
        }
        if end >= manifest.len() {
            break;
        }
        cursor = end;
    }
    dependencies
}

/// Read a single attribute's value from the first matching element tag. The
/// search is bounded to the element's opening tag so it cannot bleed into a
/// later element.
fn attribute_value(haystack: &str, element: &str, attribute: &str) -> Option<String> {
    let element_pos = haystack.find(&format!("<{element}"))?;
    let after_element = &haystack[element_pos..];
    let tag_end = after_element
        .find('>')
        .map(|offset| offset + 1)
        .unwrap_or(after_element.len());
    let tag = &after_element[..tag_end];
    let attribute_key = format!("{attribute}=\"");
    let attribute_pos = tag.find(&attribute_key)? + attribute_key.len();
    let rest = &tag[attribute_pos..];
    let end = rest.find('"')?;
    let value = rest[..end].trim();
    (!value.is_empty()).then(|| value.to_owned())
}

fn architecture_from_machine(machine: u16) -> AppArchitecture {
    match machine {
        pelite::image::IMAGE_FILE_MACHINE_I386 => AppArchitecture::X86,
        pelite::image::IMAGE_FILE_MACHINE_AMD64 => AppArchitecture::X64,
        IMAGE_FILE_MACHINE_ARM64 => AppArchitecture::Arm64,
        _ => AppArchitecture::Unknown,
    }
}

fn format_system_time_utc(value: SystemTime) -> Option<String> {
    let datetime: OffsetDateTime = value.into();
    datetime.format(&Rfc3339).ok()
}

#[cfg(test)]
mod header_tests {
    use super::*;

    /// What a scan takes from a program. Compared whole, since anything left out of a partial read
    /// would show up here as a missing import or version.
    fn what_a_scan_finds(path: &Path, bytes: &[u8]) -> Option<String> {
        let pe = PeFile::from_bytes(bytes).ok()?;
        let mut warnings = Vec::new();
        let imports = read_imports(path, &pe, &mut warnings);
        let delay = read_delay_imports(&pe, bytes, &mut warnings);
        let manifest = read_embedded_manifest(&pe);
        let version = read_version_info(&pe);
        let signature = signature_status(&pe);
        Some(format!(
            "{:?} {:?} {manifest:?} {version:?} {signature:?} {warnings:?}",
            imports
                .iter()
                .map(|import| import.name.clone())
                .collect::<Vec<_>>(),
            delay
                .iter()
                .map(|import| import.name.clone())
                .collect::<Vec<_>>(),
        ))
    }

    /// Real programs of Windows' own, which every PC this builds on has.
    #[test]
    fn reading_part_of_a_program_finds_all_that_reading_it_whole_does() -> Result<()> {
        let folder = crate::path_utils::system_root().join("System32");
        let mut programs: Vec<PathBuf> = fs::read_dir(&folder)
            .map_err(|error| WinAppMigratorError::io(&folder, error))?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "dll" || extension == "exe")
            })
            .collect();
        programs.sort();
        let mut compared = 0;
        let mut partial_was_less = 0;
        for program in programs.iter().step_by(100).take(40) {
            let Ok(whole) = fs::read(program) else {
                continue;
            };
            let Some(part) = read_what_is_looked_at(program, whole.len() as u64) else {
                continue;
            };
            let nonzero = part.iter().filter(|byte| **byte != 0).count();
            let nonzero_whole = whole.iter().filter(|byte| **byte != 0).count();
            partial_was_less += usize::from(nonzero < nonzero_whole);
            assert_eq!(
                what_a_scan_finds(program, &part),
                what_a_scan_finds(program, &whole),
                "{}",
                program.display()
            );
            compared += 1;
        }
        assert!(compared > 10, "only {compared} programs were compared");
        assert!(
            partial_was_less > 10,
            "only {partial_was_less} of them left anything out"
        );
        Ok(())
    }

    #[test]
    fn a_managed_program_is_read_whole() {
        // Its metadata can be anywhere, so nothing short of all of it will do. A file that is not
        // read as a program comes to the same thing.
        let mut bytes = vec![0_u8; 0x400];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[PE_OFFSET_FIELD] = 0x80;
        bytes[0x80..0x84].copy_from_slice(PE_SIGNATURE);
        bytes[0x84..0x86].copy_from_slice(&0x8664_u16.to_le_bytes());
        let optional = 0x80 + 4 + COFF_HEADER_BYTES;
        bytes[optional..optional + 2].copy_from_slice(&OPTIONAL_MAGIC_PE32_PLUS.to_le_bytes());
        bytes[optional + PE32_PLUS_DIRECTORIES_FIELD + 14 * DIRECTORY_ENTRY_BYTES] = 1;
        assert!(header_facts(&bytes).is_some_and(|facts| facts.is_managed));
    }

    #[test]
    fn a_real_program_has_what_it_says_read_from_its_start_alone() -> Result<()> {
        let exe = std::env::current_exe()
            .map_err(|error| WinAppMigratorError::io("current executable", error))?;
        let start = read_start_of(&exe)?;
        assert!(start.len() <= HEADER_BYTES);
        let facts = header_facts(&start).ok_or_else(|| {
            WinAppMigratorError::pe(&exe, "these headers should have been readable".to_owned())
        })?;
        let whole = fs::read(&exe).map_err(|error| WinAppMigratorError::io(&exe, error))?;
        let pe = PeFile::from_bytes(&whole)
            .map_err(|error| WinAppMigratorError::pe(&exe, error.to_string()))?;
        assert_eq!(facts.machine, pe.file_header().Machine);
        assert_eq!(facts.is_managed, is_dotnet_image(&pe));
        assert!(facts.subsystem_major >= 5, "{facts:?}");
        assert_eq!(read_pe_architecture(&exe)?, AppArchitecture::X64);
        Ok(())
    }

    #[test]
    fn what_is_not_a_program_has_no_headers() {
        assert_eq!(header_facts(b""), None);
        assert_eq!(header_facts(b"MZ"), None);
        assert_eq!(header_facts(&[b'M'; 4096]), None);
        let mut cut_off = vec![0_u8; 0x40];
        cut_off[0] = b'M';
        cut_off[1] = b'Z';
        cut_off[PE_OFFSET_FIELD] = 0x40;
        assert_eq!(header_facts(&cut_off), None, "they start past the end");
        cut_off[PE_OFFSET_FIELD..PE_OFFSET_FIELD + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(header_facts(&cut_off), None, "nowhere at all");
    }

    #[test]
    fn a_header_says_what_a_small_table_of_directories_does_not_hold() {
        let mut bytes = vec![0_u8; 0x200];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[PE_OFFSET_FIELD] = 0x80;
        bytes[0x80..0x84].copy_from_slice(PE_SIGNATURE);
        bytes[0x84..0x86].copy_from_slice(&0xAA64_u16.to_le_bytes());
        let optional = 0x80 + 4 + COFF_HEADER_BYTES;
        bytes[optional..optional + 2].copy_from_slice(&OPTIONAL_MAGIC_PE32_PLUS.to_le_bytes());
        bytes[optional + OPTIONAL_SUBSYSTEM_MAJOR_FIELD] = 10;
        let facts = header_facts(&bytes);
        assert_eq!(
            facts,
            Some(HeaderFacts {
                machine: 0xAA64,
                subsystem_major: 10,
                subsystem_minor: 0,
                is_managed: false,
            })
        );
        let descriptor = optional + PE32_PLUS_DIRECTORIES_FIELD + 14 * DIRECTORY_ENTRY_BYTES;
        bytes[descriptor] = 1;
        assert!(header_facts(&bytes).is_some_and(|facts| facts.is_managed));
        assert!(
            header_facts(&bytes[..descriptor]).is_some_and(|facts| !facts.is_managed),
            "cut off before that entry"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small generator, so that what this tries is the same each time.
    struct Mutations(u64);

    impl Mutations {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        fn below(&mut self, limit: usize) -> usize {
            usize::try_from(self.next() % limit.max(1) as u64).unwrap_or(0)
        }
    }

    /// Windows' own programs, changed a little at a time where a parser looks: whatever comes of
    /// reading them, it comes back as an answer. It found a panic and a misaligned read in 84,000
    /// tries at first. This is few enough to run each time and says those stay found.
    #[test]
    fn no_program_however_damaged_takes_what_reads_it_down() {
        const TRIES_OF_EACH: usize = 400;
        const FIRST_PART: usize = 4096;
        let path = Path::new("fuzzed.exe");
        let seeds = [
            crate::path_utils::system_root().join(r"System32\whoami.exe"),
            crate::path_utils::system_root().join(r"System32\sc.exe"),
        ];
        let mut mutations = Mutations(0x5EED_5EED_5EED_5EED);
        let mut read = 0_usize;
        for seed in seeds {
            let Ok(original) = fs::read(&seed) else {
                continue;
            };
            read += 1;
            for _ in 0..TRIES_OF_EACH {
                let mut bytes = original.clone();
                for _ in 0..=mutations.below(8) {
                    let within = if mutations.below(2) == 0 {
                        FIRST_PART.min(bytes.len())
                    } else {
                        bytes.len()
                    };
                    let at = mutations.below(within);
                    let value = mutations.next().to_le_bytes()[0];
                    bytes[at] = value;
                }
                let _ = not_a_panic(path, || scan_pe_bytes(path, &bytes));
                let _ = not_a_panic(path, || program_needs_of_bytes(path, &bytes));
            }
        }
        assert!(read > 0, "there was nothing to try this on");
    }

    #[test]
    fn a_program_whose_headers_are_not_on_a_boundary_is_not_given_to_the_parser() {
        let mut bytes = vec![0_u8; 512];
        bytes[0x3C] = 0x80;
        bytes[0x80 + 20] = 0xF0;
        assert!(headers_are_where_they_can_be(&bytes));
        bytes[0x80 + 20] = 0xF1;
        assert!(
            !headers_are_where_they_can_be(&bytes),
            "an odd optional header"
        );
        bytes[0x80 + 20] = 0xF0;
        bytes[0x3C] = 0x81;
        assert!(
            !headers_are_where_they_can_be(&bytes),
            "headers at an odd place"
        );
        assert!(
            !headers_are_where_they_can_be(&[0_u8; 16]),
            "too short to say"
        );
    }

    fn metadata_root(version: &str) -> Vec<u8> {
        // BSJB signature, major(2), minor(2), reserved(4), length(4), version.
        let mut bytes = b"BSJB".to_vec();
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        let mut version_bytes = version.as_bytes().to_vec();
        version_bytes.push(0);
        while !version_bytes.len().is_multiple_of(4) {
            version_bytes.push(0);
        }
        bytes.extend_from_slice(&(version_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&version_bytes);
        bytes
    }

    /// A metadata root with these streams, laid out as a real one is.
    fn metadata_with_streams(strings: &[u8], blob: &[u8]) -> Vec<u8> {
        let mut root = metadata_root("v4.0.30319");
        root.extend_from_slice(&0u16.to_le_bytes());
        root.extend_from_slice(&2u16.to_le_bytes());
        let header_at = root.len();
        let headers = 8 + b"#Strings\0\0\0\0".len() + 8 + b"#Blob\0\0\0".len();
        let strings_at = header_at + headers;
        let blob_at = strings_at + strings.len();
        for (offset, size, name) in [
            (strings_at, strings.len(), &b"#Strings\0\0\0\0"[..]),
            (blob_at, blob.len(), &b"#Blob\0\0\0"[..]),
        ] {
            root.extend_from_slice(&(offset as u32).to_le_bytes());
            root.extend_from_slice(&(size as u32).to_le_bytes());
            root.extend_from_slice(name);
        }
        root.extend_from_slice(strings);
        root.extend_from_slice(blob);
        root
    }

    #[test]
    fn a_managed_image_says_which_dotnet_it_was_built_for() {
        let modern = metadata_with_streams(
            b"\0System.Runtime\0PresentationFramework\0",
            b"\x1a.NETCoreApp,Version=v8.0\x01",
        );
        let facts = managed_image_facts(&modern);
        assert_eq!(
            facts.target_framework.as_deref(),
            Some(".NETCoreApp,Version=v8.0")
        );
        assert!(!facts.refers_to_mscorlib);
        assert!(facts.refers_to_desktop);
        assert!(!facts.refers_to_asp_net);

        let framework =
            metadata_with_streams(b"\0mscorlib\0", b"\x1c.NETFramework,Version=v4.7.2\x01");
        let facts = managed_image_facts(&framework);
        assert_eq!(
            facts.target_framework.as_deref(),
            Some(".NETFramework,Version=v4.7.2")
        );
        assert!(facts.refers_to_mscorlib);
        assert!(!facts.refers_to_desktop);

        let web = metadata_with_streams(b"\0Microsoft.AspNetCore.Hosting\0", b"");
        assert!(managed_image_facts(&web).refers_to_asp_net);
    }

    #[test]
    fn metadata_that_does_not_hold_together_says_nothing() {
        assert_eq!(managed_image_facts(b""), ManagedImageFacts::default());
        assert_eq!(managed_image_facts(b"BSJB"), ManagedImageFacts::default());
        // Claims more streams than are there, and streams that go past the end of the file.
        let mut lies = metadata_root("v4.0.30319");
        lies.extend_from_slice(&0u16.to_le_bytes());
        lies.extend_from_slice(&u16::MAX.to_le_bytes());
        lies.extend_from_slice(&u32::MAX.to_le_bytes());
        lies.extend_from_slice(&u32::MAX.to_le_bytes());
        lies.extend_from_slice(b"#Blob\0\0\0");
        assert_eq!(managed_image_facts(&lies), ManagedImageFacts::default());
    }

    #[test]
    fn reads_clr_metadata_version_from_root() {
        let blob = metadata_root("v2.0.50727");
        assert_eq!(
            dotnet_metadata_version(&blob).as_deref(),
            Some("v2.0.50727")
        );
        let blob4 = metadata_root("v4.0.30319");
        assert_eq!(
            dotnet_metadata_version(&blob4).as_deref(),
            Some("v4.0.30319")
        );
    }

    #[test]
    fn ignores_bytes_without_a_metadata_root() {
        assert_eq!(dotnet_metadata_version(b"not a managed image"), None);
    }

    #[test]
    fn parses_imports_from_current_test_binary() -> Result<()> {
        let current = std::env::current_exe()
            .map_err(|source| WinAppMigratorError::io("current_exe", source))?;
        let scan = scan_pe(&current)?;

        assert_ne!(scan.architecture, AppArchitecture::Unknown);
        assert!(!scan.direct_imports.is_empty());
        Ok(())
    }

    fn put_u32(buffer: &mut [u8], offset: usize, value: u32) {
        buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    #[test]
    fn reads_v2_delay_imports_and_skips_legacy() {
        // One section mapping RVA 0x1000 -> file offset 0x200.
        let spans = [SectionSpan {
            virtual_address: 0x1000,
            virtual_size: 0x1000,
            raw_pointer: 0x200,
            raw_size: 0x1000,
        }];
        let mut bytes = vec![0u8; 0x400];
        // Descriptor 0 at RVA 0x1000 (offset 0x200): legacy v1 (attributes bit clear).
        put_u32(&mut bytes, 0x200, 0); // attributes
        put_u32(&mut bytes, 0x204, 0x1060); // name (ignored for v1)
        // Descriptor 1 at RVA 0x1020 (offset 0x220): v2, name at RVA 0x1070.
        put_u32(&mut bytes, 0x220, DELAY_ATTR_RVA_BASED);
        put_u32(&mut bytes, 0x224, 0x1070);
        // Descriptor 2 at RVA 0x1040 (offset 0x240): all-zero terminator (already 0).
        // Name "REAL.dll\0" at RVA 0x1070 (offset 0x270).
        bytes[0x270..0x270 + 9].copy_from_slice(b"REAL.dll\0");

        let (names, legacy_skipped) = read_delay_import_names(&bytes, &spans, 0x1000);

        assert_eq!(names, vec!["REAL.dll".to_owned()]);
        assert_eq!(legacy_skipped, 1);
    }

    #[test]
    fn rva_to_offset_rejects_virtual_only_region() {
        let spans = [SectionSpan {
            virtual_address: 0x1000,
            virtual_size: 0x2000,
            raw_pointer: 0x200,
            raw_size: 0x1000,
        }];
        assert_eq!(rva_to_offset(&spans, 0x1000), Some(0x200));
        // Within raw data: 0x1000 + 0x800 -> 0x200 + 0x800.
        assert_eq!(rva_to_offset(&spans, 0x1800), Some(0xA00));
        // Past raw_size but inside virtual_size: virtual-only, no file bytes.
        assert_eq!(rva_to_offset(&spans, 0x2800), None);
        // Past the whole section.
        assert_eq!(rva_to_offset(&spans, 0x9999), None);
    }

    #[test]
    fn parses_manifest_execution_level_and_sxs() {
        let manifest = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity type="win32" name="MyApp" version="1.0.0.0"/>
  <trustInfo><security><requestedPrivileges>
    <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
  </requestedPrivileges></security></trustInfo>
  <dependency><dependentAssembly>
    <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0"
      processorArchitecture="*" publicKeyToken="6595b64144ccf1df"/>
  </dependentAssembly></dependency>
</assembly>"#;

        assert_eq!(
            requested_execution_level(manifest).as_deref(),
            Some("requireAdministrator")
        );
        assert_eq!(
            sxs_dependencies(manifest),
            vec!["Microsoft.Windows.Common-Controls".to_owned()]
        );
    }
}
