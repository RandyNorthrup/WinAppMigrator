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
    let metadata = file_metadata(path, &pe)?;
    let architecture = architecture_from_machine(pe.file_header().Machine);
    let mut warnings = Vec::new();
    let direct_imports = read_imports(path, &pe, &mut warnings);

    Ok(PeScanResult {
        metadata,
        architecture,
        direct_imports,
        warnings,
    })
}

pub fn read_pe_architecture(path: &Path) -> Result<AppArchitecture> {
    let bytes = fs::read(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let pe = PeFile::from_bytes(&bytes)
        .map_err(|source| WinAppMigratorError::pe(path, source.to_string()))?;
    Ok(architecture_from_machine(pe.file_header().Machine))
}

fn file_metadata(path: &Path, pe: &PeFile<'_>) -> Result<ExecutableMetadata> {
    let metadata = fs::metadata(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let modified_utc = metadata.modified().ok().and_then(format_system_time_utc);
    let external_manifest =
        external_manifest_path(path).filter(|manifest_path| manifest_path.exists());

    Ok(ExecutableMetadata {
        path: path.to_path_buf(),
        file_name: file_name_string(path),
        file_size_bytes: metadata.len(),
        modified_utc,
        version_info: read_version_info(pe),
        digital_signature: signature_status(pe),
        external_manifest,
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

    #[test]
    fn parses_imports_from_current_test_binary() -> Result<()> {
        let current = std::env::current_exe()
            .map_err(|source| WinAppMigratorError::io("current_exe", source))?;
        let scan = scan_pe(&current)?;

        assert_ne!(scan.architecture, AppArchitecture::Unknown);
        assert!(!scan.direct_imports.is_empty());
        Ok(())
    }
}
