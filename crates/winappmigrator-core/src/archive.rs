use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Path};

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::ensure_within_root;

const ZIP_BUFFER_SIZE: usize = 1024 * 1024;
const SFX_MAGIC: &[u8] = b"WINAPPMIGRATOR_SFX_V1";
const SFX_LENGTH_BYTES: usize = 8;

pub fn create_zip_archive(package_root: &Path, archive_path: &Path) -> Result<()> {
    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }

    let file =
        File::create(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let mut buffer = vec![0_u8; ZIP_BUFFER_SIZE];

    for entry in WalkDir::new(package_root).follow_links(false) {
        let entry = entry.map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let path = entry.path();
        if path == package_root || entry.file_type().is_dir() {
            continue;
        }
        let name = zip_entry_name(package_root, path)?;
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
    let mut archive =
        ZipArchive::new(file).map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| WinAppMigratorError::Archive(error.to_string()))?;
        let Some(enclosed_name) = file.enclosed_name() else {
            return Err(WinAppMigratorError::Archive(format!(
                "zip entry has unsafe path: {}",
                file.name()
            )));
        };
        let destination = destination_root.join(enclosed_name);
        ensure_within_root(destination_root, &destination)?;

        if file.is_dir() {
            fs::create_dir_all(&destination)
                .map_err(|error| WinAppMigratorError::io(&destination, error))?;
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
        }
        let mut output = File::create(&destination)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;
        io::copy(&mut file, &mut output)
            .map_err(|error| WinAppMigratorError::io(&destination, error))?;
    }
    Ok(())
}

pub fn create_self_extracting_archive(
    stub_executable: &Path,
    archive_path: &Path,
    output_path: &Path,
) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    fs::copy(stub_executable, output_path)
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    let archive_len = fs::metadata(archive_path)
        .map_err(|error| WinAppMigratorError::io(archive_path, error))?
        .len();
    let mut output = OpenOptions::new()
        .append(true)
        .open(output_path)
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    let mut archive =
        File::open(archive_path).map_err(|error| WinAppMigratorError::io(archive_path, error))?;
    io::copy(&mut archive, &mut output)
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    output
        .write_all(&archive_len.to_le_bytes())
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    output
        .write_all(SFX_MAGIC)
        .map_err(|error| WinAppMigratorError::io(output_path, error))?;
    Ok(())
}

pub fn extract_embedded_self_archive(
    sfx_executable: &Path,
    destination_root: &Path,
) -> Result<bool> {
    let Some(archive_bytes) = read_embedded_archive_bytes(sfx_executable)? else {
        return Ok(false);
    };
    fs::create_dir_all(destination_root)
        .map_err(|error| WinAppMigratorError::io(destination_root, error))?;
    let archive_path = destination_root.join("embedded-package.zip");
    fs::write(&archive_path, archive_bytes)
        .map_err(|error| WinAppMigratorError::io(&archive_path, error))?;
    extract_zip_archive(&archive_path, destination_root)?;
    fs::remove_file(&archive_path)
        .map_err(|error| WinAppMigratorError::io(&archive_path, error))?;
    Ok(true)
}

pub fn has_embedded_self_archive(sfx_executable: &Path) -> Result<bool> {
    Ok(read_embedded_archive_bytes(sfx_executable)?.is_some())
}

fn read_embedded_archive_bytes(sfx_executable: &Path) -> Result<Option<Vec<u8>>> {
    let mut file = File::open(sfx_executable)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    let file_len = file
        .metadata()
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?
        .len();
    let footer_len = (SFX_LENGTH_BYTES + SFX_MAGIC.len()) as u64;
    if file_len < footer_len {
        return Ok(None);
    }

    file.seek(SeekFrom::End(-(footer_len as i64)))
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    let mut footer = vec![0_u8; footer_len as usize];
    file.read_exact(&mut footer)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    if &footer[SFX_LENGTH_BYTES..] != SFX_MAGIC {
        return Ok(None);
    }

    let mut length_bytes = [0_u8; SFX_LENGTH_BYTES];
    length_bytes.copy_from_slice(&footer[..SFX_LENGTH_BYTES]);
    let archive_len = u64::from_le_bytes(length_bytes);
    if archive_len > file_len.saturating_sub(footer_len) {
        return Err(WinAppMigratorError::Archive(
            "self-extracting archive footer length is invalid".to_owned(),
        ));
    }
    let archive_start = file_len - footer_len - archive_len;
    file.seek(SeekFrom::Start(archive_start))
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    let mut bytes = vec![0_u8; archive_len as usize];
    file.read_exact(&mut bytes)
        .map_err(|error| WinAppMigratorError::io(sfx_executable, error))?;
    Ok(Some(bytes))
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
        create_zip_archive(&package_root, &archive_path)?;
        extract_zip_archive(&archive_path, &extract_root)?;

        assert!(extract_root.join("manifest.json").is_file());
        assert!(extract_root.join("files").join("app.exe").is_file());
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
