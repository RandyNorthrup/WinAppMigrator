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

/// Take away the folders this run mounted the image's volumes on, then detach it, if it is still
/// attached. A run detaches what it attached. A sweep only detaches an image one of whose volumes is
/// mounted in that run's folder: anything else may have attached it since, and be using it.
const DETACH_SCRIPT: &str = r#"$ErrorActionPreference = 'Stop'
Import-Module Storage
$path = $env:WAM_IMAGE_PATH
$folder = $env:WAM_IMAGE_FOLDER.TrimEnd('\') + '\'
$image = Get-DiskImage -ImagePath $path
if (-not $image.Attached) { exit 0 }
$disk = $image | Get-Disk
$ours = $false
foreach ($partition in Get-Partition -DiskNumber $disk.Number) {
    foreach ($access in $partition.AccessPaths) {
        if ($access.StartsWith($folder, [StringComparison]::OrdinalIgnoreCase)) {
            $ours = $true
            Remove-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath $access
        }
    }
}
if ($ours -or $env:WAM_IMAGE_OURS -eq '1') { Dismount-DiskImage -ImagePath $path | Out-Null }
"#;

/// A disk image, attached read-only until this is dropped.
pub struct AttachedImage {
    image: PathBuf,
    volumes: Vec<PathBuf>,
    /// Whether this run attached it, and so detaches it.
    attached_here: bool,
    // Dropped in this order: the lease, then the folder the volumes were mounted in, which is
    // emptied first by `Drop`.
    _lease: File,
    folder: ProtectedFolder,
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
            _lease: lease,
            folder,
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
            .filter(|volume| volume.starts_with(attached.folder.path()))
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
        if let Err(error) = detach(&self.image, self.folder.path(), true) {
            warn!(%error, image = %self.image.display(), "failed to detach a disk image");
        }
    }
}

fn detach(image: &Path, folder: &Path, ours: bool) -> std::result::Result<(), String> {
    let output = powershell_command(format!(
        "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_MANAGEMENT}{DETACH_SCRIPT}"
    ))
    .env(IMAGE_PATH_VARIABLE, image)
    .env(IMAGE_FOLDER_VARIABLE, folder)
    .env(IMAGE_OURS_VARIABLE, if ours { "1" } else { "0" })
    .output()
    .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(first_line(&String::from_utf8_lossy(&output.stderr)))
    }
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

    #[test]
    fn a_tool_says_what_went_wrong_in_a_line() {
        assert_eq!(
            first_line("\r\n  The file is in use.\r\nAt line:1"),
            "The file is in use."
        );
        assert_eq!(first_line(""), "Windows did not say why");
    }
}
