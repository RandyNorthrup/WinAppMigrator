//! A disk image of another PC (`.vhd` or `.vhdx`), attached read-only for as long as it is read.
//!
//! Disk2vhd makes one of a PC that is still running, Windows 7's own "Create a system image" makes
//! a `.vhd`, and Hyper-V keeps its disks as either. Windows attaches them itself, and its own NTFS
//! and registry code then reads what is on them, so a scan of an image is a scan of a connected
//! drive (`ScanSource::OfflineVolume`), with nothing on it read any other way.
//!
//! An image is attached read-only, so nothing writes to it, and with no drive letter, so nothing
//! else on the PC is offered it. Each of its volumes is mounted on a folder of its own inside one
//! only administrators can change, under the app's folder in `ProgramData`, for as long as this
//! lives. A run holds a lease there, as one that loads hives does, and [`sweep_leaked_images`]
//! detaches what a run that ended without doing so left attached.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use tracing::warn;

use crate::errors::{Result, WinAppMigratorError};
use crate::journal::machine_state_root;
use crate::path_utils::{
    POWERSHELL_IMPORT_MANAGEMENT, POWERSHELL_UTF8_OUTPUT, file_name_string, path_has_extension,
    powershell_command,
};
use crate::privilege::{
    ProtectedFolder, confirm_protected_folder, ensure_protected_folder, is_elevated,
};
use crate::system_context::looks_like_a_windows_drive;

/// What Windows can attach, which is what can be picked.
pub const DISK_IMAGE_EXTENSIONS: &[&str] = &["vhd", "vhdx"];
const IMAGES_FOLDER: &str = "images";
const IMAGE_FOLDER_PREFIX: &str = "i-";
const LEASE_FILE: &str = "in-use.lock";
/// The image a run attached, so that a sweep can detach it.
const IMAGE_RECORD: &str = "image.txt";
const ERROR_SHARING_VIOLATION: i32 = 32;
const IMAGE_PATH_VARIABLE: &str = "WAM_IMAGE_PATH";
const IMAGE_FOLDER_VARIABLE: &str = "WAM_IMAGE_FOLDER";
/// Said when Windows already has the image attached, as whoever else attached it can write to it.
const ALREADY_ATTACHED: &str = "WAM_ALREADY_ATTACHED";
/// Printed once Windows has attached the image, which from then on is this run's to detach.
const ATTACHED_MARKER: &str = "WAM_ATTACHED";
const IMAGE_OURS_VARIABLE: &str = "WAM_IMAGE_OURS";

/// Attach read-only and mount each volume with a file system on a folder of its own, printing those
/// folders. Values come in through the environment, never through the script's text.
const ATTACH_SCRIPT: &str = r#"$ErrorActionPreference = 'Stop'
Import-Module Storage
$path = $env:WAM_IMAGE_PATH
$folder = $env:WAM_IMAGE_FOLDER
if ((Get-DiskImage -ImagePath $path).Attached) { [Console]::Error.WriteLine('WAM_ALREADY_ATTACHED'); exit 3 }
$image = Mount-DiskImage -ImagePath $path -Access ReadOnly -NoDriveLetter -PassThru
'WAM_ATTACHED'
$disk = $image | Get-Disk
foreach ($partition in Get-Partition -DiskNumber $disk.Number) {
    $volume = $partition | Get-Volume -ErrorAction SilentlyContinue
    if (-not $volume -or -not $volume.FileSystem) { continue }
    $where = Join-Path $folder ('v' + $partition.PartitionNumber)
    New-Item -ItemType Directory -Path $where | Out-Null
    Add-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath ($where + '\')
    $where
}
"#;

/// Printed when the image is no longer attached, by this or anything else, so its record can go.
const DETACHED_MARKER: &str = "WAM_DETACHED";

/// Take away the folders this run mounted the image's volumes on, then detach it, if it is still
/// attached, and print `WAM_DETACHED` once it is not. A run detaches what it attached. A sweep only
/// detaches an image one of whose volumes is mounted in that run's folder: anything else may have
/// attached it since, and be using it.
///
/// A disk with no partitions, or none Windows can read, has none to list, and `Get-Partition` then
/// fails. Neither that nor failing to take away a folder stops a run detaching what it attached.
const DETACH_SCRIPT: &str = r#"$ErrorActionPreference = 'Stop'
Import-Module Storage
$path = $env:WAM_IMAGE_PATH
$folder = $env:WAM_IMAGE_FOLDER.TrimEnd('\') + '\'
$image = Get-DiskImage -ImagePath $path
if (-not $image.Attached) { 'WAM_DETACHED'; exit 0 }
$ours = $false
$disk = $null
$partitions = @()
try {
    $disk = $image | Get-Disk
    $partitions = @(Get-Partition -DiskNumber $disk.Number)
} catch { }
foreach ($partition in $partitions) {
    foreach ($access in $partition.AccessPaths) {
        if ($access.StartsWith($folder, [StringComparison]::OrdinalIgnoreCase)) {
            $ours = $true
            try {
                Remove-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath $access
            } catch { [Console]::Error.WriteLine($_.Exception.Message) }
        }
    }
}
if ($ours -or $env:WAM_IMAGE_OURS -eq '1') { Dismount-DiskImage -ImagePath $path | Out-Null }
if (-not (Get-DiskImage -ImagePath $path).Attached) { 'WAM_DETACHED' }
"#;

/// A VHD's footer is its last 512 bytes, or 511 from Virtual PC before 2004, and a dynamic or
/// differencing one has a copy of it at its start.
const VHD_FOOTER_BYTES: u64 = 512;
const VHD_OLD_FOOTER_BYTES: u64 = 511;
const VHD_COOKIE: &[u8] = b"conectix";
/// Where a VHD's footer says what kind of disk it is, big-endian.
const VHD_DISK_TYPE_AT: usize = 60;
const VHD_DIFFERENCING: u32 = 4;
const VHDX_SIGNATURE: &[u8] = b"vhdxfile";
/// The two copies of a VHDX's region table, either of which Windows may go by.
const VHDX_REGION_TABLES: [u64; 2] = [192 * 1024, 256 * 1024];
const VHDX_REGION_TABLE_SIGNATURE: &[u8] = b"regi";
const VHDX_METADATA_SIGNATURE: &[u8] = b"metadata";
/// A region table and a metadata table are each 64 KiB, with no more than 2047 entries.
const VHDX_TABLE_BYTES: u64 = 64 * 1024;
const VHDX_MAX_ENTRIES: usize = 2047;
const VHDX_REGION_ENTRIES_AT: usize = 16;
const VHDX_METADATA_ENTRIES_AT: usize = 32;
const VHDX_ENTRY_BYTES: usize = 32;
const VHDX_METADATA_REGION: [u8; 16] = guid(
    0x8B7C_A206,
    0x4790,
    0x4B9A,
    [0xB8, 0xFE, 0x57, 0x5F, 0x05, 0x0F, 0x88, 0x6E],
);
const VHDX_FILE_PARAMETERS: [u8; 16] = guid(
    0xCAA1_6737,
    0xFA36,
    0x4D43,
    [0xB3, 0xB6, 0x33, 0xF0, 0xAA, 0x44, 0xE7, 0x6B],
);
const VHDX_PARENT_LOCATOR: [u8; 16] = guid(
    0xA8D3_5F2D,
    0xB30B,
    0x454D,
    [0xAB, 0xF7, 0xD3, 0xD8, 0x48, 0x34, 0xAB, 0x0C],
);
/// In a VHDX's file parameters, after the block size.
const VHDX_HAS_PARENT: u32 = 0x2;

/// A GUID as Windows stores it: the first three parts little-endian, the rest as written.
const fn guid(first: u32, second: u16, third: u16, rest: [u8; 8]) -> [u8; 16] {
    let a = first.to_le_bytes();
    let b = second.to_le_bytes();
    let c = third.to_le_bytes();
    [
        a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], rest[0], rest[1], rest[2], rest[3],
        rest[4], rest[5], rest[6], rest[7],
    ]
}

/// A disk image, attached read-only until this is dropped.
pub struct AttachedImage {
    image: PathBuf,
    volumes: Vec<PathBuf>,
    /// Whether this run attached it, and so detaches it.
    attached_here: bool,
    folder_path: PathBuf,
    // Dropped in this order: the lease, then the folder the volumes were mounted in, which is
    // emptied first by `Drop`, or kept if the image could not be detached.
    _lease: File,
    folder: Option<ProtectedFolder>,
}

impl AttachedImage {
    /// Attach an image read-only. It has to be a `.vhd` or `.vhdx` that Windows does not already
    /// have attached, since whoever attached it can be writing to it.
    pub fn attach(image: &Path) -> Result<Self> {
        if !DISK_IMAGE_EXTENSIONS
            .iter()
            .any(|extension| path_has_extension(image, extension))
        {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "{} is not a disk image Windows can attach (.vhd or .vhdx)",
                image.display()
            )));
        }
        if !is_elevated() {
            return Err(WinAppMigratorError::InvalidInput(
                "attaching a disk image needs administrator rights".to_owned(),
            ));
        }
        let image = fs::canonicalize(image)
            .map(|path| strip_verbatim(&path))
            .map_err(|error| WinAppMigratorError::io(image, error))?;
        if !fs::metadata(&image).is_ok_and(|metadata| metadata.is_file()) {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "{} is not a file",
                image.display()
            )));
        }
        let kind = File::open(&image)
            .and_then(|mut file| kind_of_image(&mut file))
            .map_err(|error| WinAppMigratorError::io(&image, error))?;
        match kind {
            ImageKind::Whole => {}
            ImageKind::Differencing => {
                return Err(WinAppMigratorError::InvalidInput(format!(
                    "{} is a differencing disk, which only holds what changed since the disk it \
                     was made from, and Windows would open that disk too, wherever it says it is. \
                     Merge it into that disk first, with Hyper-V's Edit Disk, and pick that",
                    image.display()
                )));
            }
            ImageKind::Unreadable => {
                return Err(WinAppMigratorError::InvalidInput(format!(
                    "{} is not a .vhd or .vhdx that could be read",
                    image.display()
                )));
            }
        }
        sweep_leaked_images();
        let parent = images_folder().ok_or_else(|| {
            WinAppMigratorError::InvalidInput(
                "this PC's ProgramData folder could not be found".to_owned(),
            )
        })?;
        for folder in parent.ancestors().take(2).collect::<Vec<_>>().iter().rev() {
            ensure_protected_folder(folder)
                .map_err(|error| WinAppMigratorError::io(folder, error))?;
        }
        let folder = ProtectedFolder::new_in(&parent, IMAGE_FOLDER_PREFIX)
            .map_err(|error| WinAppMigratorError::io(&parent, error))?;
        let lease_path = folder.path().join(LEASE_FILE);
        let lease = OpenOptions::new()
            .write(true)
            .create_new(true)
            .share_mode(0)
            .open(&lease_path)
            .map_err(|error| WinAppMigratorError::io(&lease_path, error))?;
        let record = folder.path().join(IMAGE_RECORD);
        fs::write(&record, image.to_string_lossy().as_bytes())
            .map_err(|error| WinAppMigratorError::io(&record, error))?;

        let output = powershell_command(format!(
            "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_MANAGEMENT}{ATTACH_SCRIPT}"
        ))
        .env(IMAGE_PATH_VARIABLE, &image)
        .env(IMAGE_FOLDER_VARIABLE, folder.path())
        .output()
        .map_err(|error| WinAppMigratorError::io("powershell", error))?;
        let listed = String::from_utf8_lossy(&output.stdout);
        let mut attached = Self {
            image,
            volumes: Vec::new(),
            attached_here: listed.lines().any(|line| line.trim() == ATTACHED_MARKER),
            folder_path: folder.path().to_path_buf(),
            _lease: lease,
            folder: Some(folder),
        };
        if !output.status.success() {
            let said = String::from_utf8_lossy(&output.stderr);
            let reason = if said.contains(ALREADY_ATTACHED) {
                "Windows already has it attached, and whatever attached it may be writing to it. \
                 Detach it first, or pick its drive as a connected drive"
                    .to_owned()
            } else {
                first_line(&said)
            };
            return Err(WinAppMigratorError::InvalidInput(format!(
                "{} could not be attached: {reason}",
                attached.image.display()
            )));
        }
        attached.volumes = listed
            .lines()
            .map(|line| line.trim_start_matches('\u{feff}').trim())
            .filter(|line| !line.is_empty() && *line != ATTACHED_MARKER)
            .map(PathBuf::from)
            .filter(|volume| volume.starts_with(&attached.folder_path))
            .collect();
        if attached.windows_volumes().is_empty() {
            return Err(WinAppMigratorError::InvalidInput(format!(
                "{} has no volume with Windows on it that could be read. A volume encrypted \
                 with BitLocker has to be unlocked first.",
                attached.image.display()
            )));
        }
        Ok(attached)
    }

    /// The image this is.
    pub fn image(&self) -> &Path {
        &self.image
    }

    /// Each of its volumes that has Windows on it, where it can be read, first by where it is on
    /// the disk.
    pub fn windows_volumes(&self) -> Vec<PathBuf> {
        self.volumes
            .iter()
            .filter(|volume| looks_like_a_windows_drive(volume))
            .cloned()
            .collect()
    }
}

impl Drop for AttachedImage {
    fn drop(&mut self) {
        if !self.attached_here {
            return;
        }
        if let Err(error) = detach(&self.image, &self.folder_path, true) {
            warn!(%error, image = %self.image.display(), "failed to detach a disk image");
            // Its record stays, so that a later sweep can try again.
            if let Some(folder) = self.folder.take() {
                drop(folder.keep());
            }
        }
    }
}

/// Detach an image. Unless it is no longer attached afterward, this says why it still is.
fn detach(image: &Path, folder: &Path, ours: bool) -> std::result::Result<(), String> {
    let output = powershell_command(format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_MANAGEMENT}{DETACH_SCRIPT}"
    ))
    .env(IMAGE_PATH_VARIABLE, image)
    .env(IMAGE_FOLDER_VARIABLE, folder)
    .env(IMAGE_OURS_VARIABLE, if ours { "1" } else { "0" })
    .output()
    .map_err(|error| error.to_string())?;
    if says_detached(&String::from_utf8_lossy(&output.stdout)) {
        Ok(())
    } else if output.status.success() {
        Err("it is still attached, and nothing shows this app attached it".to_owned())
    } else {
        Err(first_line(&String::from_utf8_lossy(&output.stderr)))
    }
}

/// Whether the detach script said the image is no longer attached.
fn says_detached(said: &str) -> bool {
    said.lines()
        .any(|line| line.trim_start_matches('\u{feff}').trim() == DETACHED_MARKER)
}

/// Detach what a run that is over left attached, and take away its folder. A run that is still
/// going holds its lease, and what has no record of this app's is not touched.
pub fn sweep_leaked_images() {
    if !is_elevated() {
        return;
    }
    let Some(images) = images_folder() else {
        return;
    };
    // Anyone could have made this before this app first ran, and what is below deletes what is in it.
    if confirm_protected_folder(&images).is_err() {
        return;
    }
    let Ok(entries) = fs::read_dir(&images) else {
        return;
    };
    for entry in entries.flatten() {
        let folder = entry.path();
        if !file_name_string(&folder).starts_with(IMAGE_FOLDER_PREFIX)
            || !entry.file_type().is_ok_and(|kind| kind.is_dir())
            || lease_is_held(&folder.join(LEASE_FILE))
        {
            continue;
        }
        // The record is what finds the image, so it goes only once the image is detached.
        if let Ok(image) = fs::read_to_string(folder.join(IMAGE_RECORD))
            && let Err(error) = detach(Path::new(image.trim()), &folder, false)
        {
            warn!(%error, image, "failed to detach a disk image a run left attached");
            continue;
        }
        if let Err(error) = fs::remove_dir_all(&folder) {
            warn!(%error, folder = %folder.display(), "failed to remove what a run left behind");
        }
    }
}

fn images_folder() -> Option<PathBuf> {
    machine_state_root().map(|root| root.join(IMAGES_FOLDER))
}

/// See `system_context`'s lease: not being able to tell counts as held.
fn lease_is_held(lease: &Path) -> bool {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(0)
        .open(lease)
    {
        Ok(_) => false,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION) => true,
        Err(_) => true,
    }
}

/// `C:\x` of `\\?\C:\x`, which is what `Mount-DiskImage` takes. A share stays as it is.
fn strip_verbatim(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(share) = text.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{share}"))
    } else if let Some(local) = text.strip_prefix(r"\\?\") {
        PathBuf::from(local)
    } else {
        path.to_path_buf()
    }
}

/// What a tool said went wrong, as one line.
fn first_line(said: &str) -> String {
    said.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("Windows did not say why")
        .chars()
        .take(300)
        .collect()
}

/// What a disk image is, as far as attaching it goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImageKind {
    /// A disk of its own.
    Whole,
    /// A differencing disk, which names the disk it was made from, and which Windows opens too when
    /// it attaches this one, wherever it is, on this PC or on a share.
    Differencing,
    /// Not a `.vhd` or `.vhdx`, or too damaged to tell which it is.
    Unreadable,
}

/// Look at an image the way Windows will before it is attached: by what is in it, whatever its name
/// says, and by every copy of what says what it is that Windows could go by. No more than a few
/// tables of it are read.
fn kind_of_image(image: &mut (impl Read + Seek)) -> io::Result<ImageKind> {
    let len = image.seek(SeekFrom::End(0))?;
    let mut known = false;
    if read_at(image, 0, VHDX_SIGNATURE.len() as u64)? == VHDX_SIGNATURE {
        match vhdx_has_parent(image)? {
            Some(true) => return Ok(ImageKind::Differencing),
            Some(false) => known = true,
            None => return Ok(ImageKind::Unreadable),
        }
    }
    let mut footers = vec![0];
    footers.extend(
        [VHD_FOOTER_BYTES, VHD_OLD_FOOTER_BYTES]
            .iter()
            .filter_map(|bytes| len.checked_sub(*bytes)),
    );
    for at in footers {
        let footer = read_at(image, at, VHD_FOOTER_BYTES)?;
        if !footer.starts_with(VHD_COOKIE) {
            continue;
        }
        let Some(disk_type) = be_u32(&footer, VHD_DISK_TYPE_AT) else {
            continue;
        };
        if disk_type == VHD_DIFFERENCING {
            return Ok(ImageKind::Differencing);
        }
        known = true;
    }
    Ok(if known {
        ImageKind::Whole
    } else {
        ImageKind::Unreadable
    })
}

/// Whether a VHDX has a parent, by each copy of its region table that Windows could go by. `None`
/// if there is none, or one that leads to metadata that cannot be read.
fn vhdx_has_parent(image: &mut (impl Read + Seek)) -> io::Result<Option<bool>> {
    let mut read_one = false;
    for table_at in VHDX_REGION_TABLES {
        let table = read_at(image, table_at, VHDX_TABLE_BYTES)?;
        if !table.starts_with(VHDX_REGION_TABLE_SIGNATURE) {
            continue;
        }
        let Some(count) = le_u32(&table, 8).and_then(|count| usize::try_from(count).ok()) else {
            return Ok(None);
        };
        if count > VHDX_MAX_ENTRIES {
            return Ok(None);
        }
        let mut has_metadata = false;
        for index in 0..count {
            let at = VHDX_REGION_ENTRIES_AT + index * VHDX_ENTRY_BYTES;
            let Some(entry) = table.get(at..at + VHDX_ENTRY_BYTES) else {
                return Ok(None);
            };
            if entry[..16] != VHDX_METADATA_REGION {
                continue;
            }
            let Some(region) = le_u64(entry, 16) else {
                return Ok(None);
            };
            match metadata_has_parent(image, region)? {
                Some(true) => return Ok(Some(true)),
                Some(false) => has_metadata = true,
                None => return Ok(None),
            }
        }
        if !has_metadata {
            return Ok(None);
        }
        read_one = true;
    }
    Ok(read_one.then_some(false))
}

/// Whether a VHDX's metadata at `region` says it has a parent, or names one. `None` if it is not
/// metadata, or does not say.
fn metadata_has_parent(image: &mut (impl Read + Seek), region: u64) -> io::Result<Option<bool>> {
    let table = read_at(image, region, VHDX_TABLE_BYTES)?;
    if !table.starts_with(VHDX_METADATA_SIGNATURE) {
        return Ok(None);
    }
    let Some(count) = le_u16(&table, 10).map(usize::from) else {
        return Ok(None);
    };
    if count > VHDX_MAX_ENTRIES {
        return Ok(None);
    }
    let mut said = None;
    for index in 0..count {
        let at = VHDX_METADATA_ENTRIES_AT + index * VHDX_ENTRY_BYTES;
        let Some(entry) = table.get(at..at + VHDX_ENTRY_BYTES) else {
            return Ok(None);
        };
        let item = &entry[..16];
        if item == VHDX_PARENT_LOCATOR {
            return Ok(Some(true));
        }
        if item != VHDX_FILE_PARAMETERS {
            continue;
        }
        let Some(parameters_at) =
            le_u32(entry, 16).and_then(|offset| region.checked_add(u64::from(offset)))
        else {
            return Ok(None);
        };
        let parameters = read_at(image, parameters_at, 8)?;
        let Some(flags) = le_u32(&parameters, 4) else {
            return Ok(None);
        };
        if flags & VHDX_HAS_PARENT != 0 {
            return Ok(Some(true));
        }
        said = Some(false);
    }
    Ok(said)
}

/// Up to `most` bytes from `at`, fewer where the image ends.
fn read_at(image: &mut (impl Read + Seek), at: u64, most: u64) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    // Past where any file can end, which Windows will not seek to.
    if i64::try_from(at).is_err() {
        return Ok(bytes);
    }
    image.seek(SeekFrom::Start(at))?;
    image.by_ref().take(most).read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn le_u16(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(bytes.get(at..at + 2)?.try_into().ok()?))
}

fn le_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

fn le_u64(bytes: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(bytes.get(at..at + 8)?.try_into().ok()?))
}

fn be_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_what_windows_can_attach_is_asked_for() {
        for name in [r"C:\x.vmdk", r"C:\x.vdi", r"C:\x.iso", r"C:\x"] {
            let error = AttachedImage::attach(Path::new(name)).err();
            assert!(
                error.is_some_and(|error| error.to_string().contains(".vhd or .vhdx")),
                "{name}"
            );
        }
    }

    #[test]
    fn a_verbatim_path_is_written_as_windows_tools_take_it() {
        assert_eq!(
            strip_verbatim(Path::new(r"\\?\C:\Images\old.vhdx")),
            PathBuf::from(r"C:\Images\old.vhdx")
        );
        assert_eq!(
            strip_verbatim(Path::new(r"\\?\UNC\server\share\old.vhd")),
            PathBuf::from(r"\\server\share\old.vhd")
        );
        assert_eq!(
            strip_verbatim(Path::new(r"D:\old.vhd")),
            PathBuf::from(r"D:\old.vhd")
        );
    }

    fn kind(bytes: Vec<u8>) -> Option<ImageKind> {
        kind_of_image(&mut io::Cursor::new(bytes)).ok()
    }

    fn vhd_footer(disk_type: u32) -> Vec<u8> {
        let mut footer = vec![0_u8; 512];
        footer[..8].copy_from_slice(VHD_COOKIE);
        footer[VHD_DISK_TYPE_AT..VHD_DISK_TYPE_AT + 4].copy_from_slice(&disk_type.to_be_bytes());
        footer
    }

    /// A fixed VHD: its disk, then its footer.
    fn fixed_vhd(disk_type: u32) -> Vec<u8> {
        let mut image = vec![0_u8; 4096];
        image.extend(vhd_footer(disk_type));
        image
    }

    #[test]
    fn a_differencing_vhd_is_told_from_one_that_stands_alone() {
        assert_eq!(kind(fixed_vhd(2)), Some(ImageKind::Whole));
        assert_eq!(
            kind(fixed_vhd(VHD_DIFFERENCING)),
            Some(ImageKind::Differencing)
        );
        // A dynamic one starts with a copy of its footer, and either copy may be the one read.
        let mut dynamic = vhd_footer(3);
        dynamic.extend(vec![0_u8; 4096]);
        dynamic.extend(vhd_footer(3));
        assert_eq!(kind(dynamic), Some(ImageKind::Whole));
        let mut mixed = vhd_footer(VHD_DIFFERENCING);
        mixed.extend(vec![0_u8; 4096]);
        mixed.extend(vhd_footer(3));
        assert_eq!(kind(mixed), Some(ImageKind::Differencing));
        // Virtual PC before 2004 wrote a footer of 511 bytes.
        let mut old = vec![0_u8; 4096];
        old.extend(&vhd_footer(VHD_DIFFERENCING)[..511]);
        assert_eq!(kind(old), Some(ImageKind::Differencing));
    }

    const METADATA_AT: usize = 1024 * 1024;
    const PARAMETERS_OFFSET: u32 = 64 * 1024;

    /// A VHDX with a region table, both copies, leading to metadata with file parameters and
    /// `extra` items besides.
    fn vhdx(flags: u32, extra: &[[u8; 16]]) -> Vec<u8> {
        let mut image = vec![0_u8; METADATA_AT + PARAMETERS_OFFSET as usize + 8];
        image[..8].copy_from_slice(VHDX_SIGNATURE);
        for table_at in VHDX_REGION_TABLES {
            let table = table_at as usize;
            image[table..table + 4].copy_from_slice(VHDX_REGION_TABLE_SIGNATURE);
            image[table + 8..table + 12].copy_from_slice(&1_u32.to_le_bytes());
            let entry = table + VHDX_REGION_ENTRIES_AT;
            image[entry..entry + 16].copy_from_slice(&VHDX_METADATA_REGION);
            image[entry + 16..entry + 24].copy_from_slice(&(METADATA_AT as u64).to_le_bytes());
            image[entry + 24..entry + 28].copy_from_slice(&(1024 * 1024_u32).to_le_bytes());
        }
        image[METADATA_AT..METADATA_AT + 8].copy_from_slice(VHDX_METADATA_SIGNATURE);
        let items: Vec<[u8; 16]> = std::iter::once(VHDX_FILE_PARAMETERS)
            .chain(extra.iter().copied())
            .collect();
        image[METADATA_AT + 10..METADATA_AT + 12]
            .copy_from_slice(&(items.len() as u16).to_le_bytes());
        for (index, item) in items.iter().enumerate() {
            let entry = METADATA_AT + VHDX_METADATA_ENTRIES_AT + index * VHDX_ENTRY_BYTES;
            image[entry..entry + 16].copy_from_slice(item);
            image[entry + 16..entry + 20].copy_from_slice(&PARAMETERS_OFFSET.to_le_bytes());
            image[entry + 20..entry + 24].copy_from_slice(&8_u32.to_le_bytes());
        }
        let parameters = METADATA_AT + PARAMETERS_OFFSET as usize;
        image[parameters..parameters + 4].copy_from_slice(&(32 * 1024 * 1024_u32).to_le_bytes());
        image[parameters + 4..parameters + 8].copy_from_slice(&flags.to_le_bytes());
        image
    }

    #[test]
    fn a_vhdx_with_a_parent_is_told_from_one_without() {
        assert_eq!(kind(vhdx(0, &[])), Some(ImageKind::Whole));
        // Leave blocks allocated, which says nothing of a parent.
        assert_eq!(kind(vhdx(1, &[])), Some(ImageKind::Whole));
        assert_eq!(
            kind(vhdx(VHDX_HAS_PARENT, &[])),
            Some(ImageKind::Differencing)
        );
        assert_eq!(
            kind(vhdx(0, &[VHDX_PARENT_LOCATOR])),
            Some(ImageKind::Differencing)
        );
        // The second copy of the region table is the one Windows goes by if the first is damaged.
        let mut second = vhdx(0, &[]);
        let elsewhere = second.len();
        second.extend(vhdx(VHDX_HAS_PARENT, &[])[METADATA_AT..].iter().copied());
        let entry = VHDX_REGION_TABLES[1] as usize + VHDX_REGION_ENTRIES_AT;
        second[entry + 16..entry + 24].copy_from_slice(&(elsewhere as u64).to_le_bytes());
        assert_eq!(kind(second), Some(ImageKind::Differencing));
    }

    #[test]
    fn an_image_that_is_cut_short_or_neither_is_not_attached() {
        assert_eq!(kind(Vec::new()), Some(ImageKind::Unreadable));
        assert_eq!(kind(vec![0xA5; 70_000]), Some(ImageKind::Unreadable));
        assert_eq!(kind(b"conectix".to_vec()), Some(ImageKind::Unreadable));
        let whole = vhdx(0, &[]);
        for cut in [
            8,
            200 * 1024,
            METADATA_AT + 40,
            METADATA_AT + PARAMETERS_OFFSET as usize + 6,
        ] {
            assert_eq!(
                kind(whole[..cut].to_vec()),
                Some(ImageKind::Unreadable),
                "{cut}"
            );
        }
        // A table that says it has more entries than one can hold.
        let mut too_many = vhdx(0, &[]);
        too_many[METADATA_AT + 10..METADATA_AT + 12].copy_from_slice(&u16::MAX.to_le_bytes());
        assert_eq!(kind(too_many), Some(ImageKind::Unreadable));
        // Metadata far past the end of the image.
        let mut nowhere = vhdx(0, &[]);
        for table_at in VHDX_REGION_TABLES {
            let entry = table_at as usize + VHDX_REGION_ENTRIES_AT;
            nowhere[entry + 16..entry + 24].copy_from_slice(&u64::MAX.to_le_bytes());
        }
        assert_eq!(kind(nowhere), Some(ImageKind::Unreadable));
    }

    #[test]
    fn a_record_goes_only_once_its_image_is_detached() {
        assert!(says_detached("WAM_DETACHED\r\n"));
        assert!(says_detached("\u{feff}WAM_DETACHED"));
        assert!(!says_detached(""));
        assert!(!says_detached("WAM_ATTACHED\r\n"));
        // However the partitions are looked for, failing to find any does not stop the detach.
        let listing = DETACH_SCRIPT
            .lines()
            .position(|line| line.contains("Get-Partition"));
        let caught = DETACH_SCRIPT
            .lines()
            .position(|line| line.contains("} catch { }"));
        assert!(listing.is_some() && caught > listing);
        assert!(DETACH_SCRIPT.contains("Dismount-DiskImage"));
    }

    #[test]
    fn a_tool_says_what_went_wrong_in_a_line() {
        assert_eq!(
            first_line("\r\n  The file is in use.\r\nAt line:1"),
            "The file is in use."
        );
        assert_eq!(first_line(""), "Windows did not say why");
    }
}
