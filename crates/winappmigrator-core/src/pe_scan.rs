use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use pelite::{PeFile, Wrap};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::errors::{Result, WinAppMigratorError};
use crate::models::{
    AppArchitecture, ExecutableMetadata, FileVersionInfo, SignatureStatus, Warning, WarningSeverity,
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

#[derive(Debug, Clone)]
pub struct ImportedDll {
    pub name: String,
    pub delay_loaded: bool,
}

pub fn scan_pe(path: &Path) -> Result<PeScanResult> {
    let bytes = fs::read(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let pe = PeFile::from_bytes(&bytes)
        .map_err(|source| WinAppMigratorError::pe(path, source.to_string()))?;
    let metadata = file_metadata(path, &pe, &bytes)?;
    let architecture = architecture_from_machine(pe.file_header().Machine);
    let mut warnings = Vec::new();
    let mut direct_imports = read_imports(path, &pe, &mut warnings);
    merge_delay_imports(
        &mut direct_imports,
        read_delay_imports(&pe, &bytes, &mut warnings),
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

pub fn read_pe_architecture(path: &Path) -> Result<AppArchitecture> {
    let bytes = fs::read(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let pe = PeFile::from_bytes(&bytes)
        .map_err(|source| WinAppMigratorError::pe(path, source.to_string()))?;
    Ok(architecture_from_machine(pe.file_header().Machine))
}

fn file_metadata(path: &Path, pe: &PeFile<'_>, bytes: &[u8]) -> Result<ExecutableMetadata> {
    let metadata = fs::metadata(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let modified_utc = metadata.modified().ok().and_then(format_system_time_utc);
    let external_manifest =
        external_manifest_path(path).filter(|manifest_path| manifest_path.exists());

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
    let resources = pe.resources().ok()?;
    let version_info = resources.version_info().ok()?;
    let mut collector = VersionCollector::default();
    version_info.visit(&mut collector);
    Some(collector.into_version_info())
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

/// The version string in a managed image's CLI metadata root, e.g. `v2.0.50727`
/// or `v4.0.30319`. The metadata root begins with the `BSJB` signature followed
/// by two `u16` version fields, a reserved `u32`, a `u32` length, then the
/// UTF-8 version string of that length. Scanning the file bytes for `BSJB` finds
/// it without mapping RVAs, and a managed image has exactly one such root.
fn dotnet_metadata_version(bytes: &[u8]) -> Option<String> {
    const SIGNATURE: &[u8; 4] = b"BSJB";
    const HEADER_PREFIX: usize = 12; // signature(4) + major(2) + minor(2) + reserved(4)
    const MAX_VERSION_LEN: usize = 256;

    let start = bytes
        .windows(SIGNATURE.len())
        .position(|window| window == SIGNATURE)?;
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
mod tests {
    use super::*;

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
