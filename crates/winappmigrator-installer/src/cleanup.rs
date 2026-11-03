//! Removes package-extraction directories left by previous or crashed installer
//! runs, so nothing accumulates over time regardless of how a prior run ended.

use std::cell::RefCell;
use std::fs::OpenOptions;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use std::time::{Duration, SystemTime};

use winappmigrator_core::file_discovery::first_link_to;
use winappmigrator_core::journal::{STAGING_PREFIX, admin_only_staging_root, staging_root_in};
use winappmigrator_core::path_utils::is_network_path;
use winappmigrator_core::privilege::{ProtectedFolder, others_could_keep_a_folder_made_in};
use winappmigrator_core::space::{on_same_volume, volume_stats_at};

const DELETE: u32 = 0x0001_0000;
const FILE_SHARE_READ: u32 = 0x0000_0001;
const FILE_SHARE_WRITE: u32 = 0x0000_0002;
const FILE_SHARE_DELETE: u32 = 0x0000_0004;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

/// Prefix of the directories the installer extracts its embedded package into.
pub const EXTRACTION_PREFIX: &str = STAGING_PREFIX;

/// A younger extraction folder may belong to an installer that is still running. An
/// install that has been going for this long and has stopped touching its folder is not
/// expected, and a folder left behind is only wasted space until a later run.
const STALE_AFTER: Duration = Duration::from_secs(60 * 60);

/// Remove stale extraction directories from the places the installer stages its
/// package — where it keeps its records, beside the installer executable, and in the
/// system temp directory. Call this at startup **before** creating the current run's
/// directory, so only prior leftovers are removed. Best-effort, and anything newer
/// than [`STALE_AFTER`] is left alone, because it can be another installer's.
pub fn sweep_stale_extractions(executable: &Path, state_root: Option<&Path>) {
    sweep_extractions_older_than(executable, state_root, SystemTime::now(), STALE_AFTER);
}

fn sweep_extractions_older_than(
    executable: &Path,
    state_root: Option<&Path>,
    now: SystemTime,
    age: Duration,
) {
    let mut roots = vec![std::env::temp_dir()];
    if let Some(parent) = executable.parent() {
        let parent = parent.to_path_buf();
        if !roots.contains(&parent) {
            roots.push(parent);
        }
    }
    roots.extend(state_root.map(staging_root_in));
    for root in roots {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_extraction = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(EXTRACTION_PREFIX));
            if is_extraction && is_stale_folder(&path, now, age) {
                let _ = std::fs::remove_dir_all(&path);
            }
        }
    }
}

/// A real folder, not touched for `age`, that no installer has a hold on. Both folders swept are ones
/// another process can write to, so a link given the right name must not be followed or removed. A
/// folder's time does not change as what is in it is read, so a long install's can look old, and a
/// hold keeps the folder from going but not what is in it.
fn is_stale_folder(path: &Path, now: SystemTime, age: Duration) -> bool {
    let Ok(metadata) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return false;
    }
    let is_old = metadata
        .modified()
        .ok()
        .and_then(|modified| now.duration_since(modified).ok())
        .is_some_and(|elapsed| elapsed >= age);
    is_old && !is_being_held(path)
}

/// Whether something has this folder open without letting it be deleted, as a running installer does.
/// Whatever else stops it being opened to be deleted is taken for that too, as all that is lost by
/// being wrong is a folder staying a while longer.
fn is_being_held(path: &Path) -> bool {
    OpenOptions::new()
        .access_mode(DELETE)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .is_err()
}

/// Create a fresh extraction staging directory, so that a large package never fails to extract for
/// lack of space on `C:`. Using [`EXTRACTION_PREFIX`] keeps every staging directory matchable by
/// [`sweep_stale_extractions`]. Removed when the result drops.
///
/// Elevated code reads from this folder, so what it reads must not be up to anyone else. Inside
/// `state_root` nobody but an administrator can open what is made, from the moment it is made. Anywhere
/// else, it is made admin-only just afterward and held so it cannot be swapped, and whoever could
/// open it in that moment and keep the right to change its permissions could still get in, so a place
/// like an account's own folders, where that is who has the rights, is not used. In order:
///
/// - `state_root`, if it is on the install destination's drive, as files then only move once.
/// - The destination's drive, by the nearest folder of the install location that exists.
/// - `state_root`, on another drive.
/// - Beside the installer, and then the system's temp folder, which is what is left if `state_root`
///   cannot be used. Usually neither can be, by that same rule, and then the install does not go ahead.
///
/// `unpacked_bytes` is how much will go in it. On the destination's drive there has to be room for
/// that twice over to be worth it, since the same files go on to the install location. A drive that
/// will not say how much room it has is taken at its word, and if nowhere has room, the first place
/// that can be used is, for what checks the room to say so.
///
/// A location where any of this fails is not used. A link on the way to the install location is
/// passed over, because the package would be unpacked wherever it points before the install got as
/// far as refusing it.
pub fn create_staging_dir(
    target_root: Option<&Path>,
    executable: &Path,
    unpacked_bytes: u64,
    state_root: Option<&Path>,
) -> std::io::Result<ProtectedFolder> {
    create_staging_dir_where(
        target_root,
        executable,
        unpacked_bytes,
        state_root,
        &|place| !others_could_keep_a_folder_made_in(place),
    )
}

/// [`create_staging_dir`], with whether a place can be used passed in, so that the order can be
/// tested on its own, as a test's folders are in its account's own folders.
fn create_staging_dir_where(
    target_root: Option<&Path>,
    executable: &Path,
    unpacked_bytes: u64,
    state_root: Option<&Path>,
    may_use: &dyn Fn(&Path) -> bool,
) -> std::io::Result<ProtectedFolder> {
    let destination = target_root
        .filter(|target| !is_network_path(&target.to_string_lossy()))
        .and_then(|target| target.ancestors().find(|ancestor| is_real_folder(ancestor)))
        .filter(|place| first_link_to(place).is_none());
    let admin_only = state_root.and_then(|root| admin_only_staging_root(root).ok());
    let beside_installer = executable.parent();
    let temp = std::env::temp_dir();
    let has_room = |place: &Path, needed: u64| {
        volume_stats_at(place).is_none_or(|stats| stats.free_bytes >= needed)
    };
    let last_failure = RefCell::new(None);
    let make_in_protected = |place: &Path| match ProtectedFolder::new_in(place, EXTRACTION_PREFIX) {
        Ok(staging) => Some(staging),
        Err(error) => {
            *last_failure.borrow_mut() = Some(format!("{}: {error}", place.display()));
            None
        }
    };
    let make_in = |place: &Path| may_use(place).then(|| make_in_protected(place)).flatten();
    let twice = unpacked_bytes.saturating_mul(2);

    if let (Some(admin_only), Some(destination)) = (admin_only.as_deref(), destination)
        && on_same_volume(admin_only, destination)
        && has_room(admin_only, twice)
        && let Some(staging) = make_in_protected(admin_only)
    {
        return Ok(staging);
    }
    if let Some(destination) = destination.filter(|place| has_room(place, twice))
        && let Some(staging) = make_in(destination)
    {
        return Ok(staging);
    }
    if let Some(admin_only) = admin_only
        .as_deref()
        .filter(|place| has_room(place, unpacked_bytes))
        && let Some(staging) = make_in_protected(admin_only)
    {
        return Ok(staging);
    }
    if let Some(parent) = beside_installer.filter(|place| has_room(place, unpacked_bytes))
        && let Some(staging) = make_in(parent)
    {
        return Ok(staging);
    }
    if has_room(&temp, unpacked_bytes)
        && let Some(staging) = make_in(&temp)
    {
        return Ok(staging);
    }
    // Nowhere has room. What checks that says so, of the first place that would have been used.
    admin_only
        .as_deref()
        .and_then(make_in_protected)
        .or_else(|| destination.and_then(make_in))
        .or_else(|| beside_installer.and_then(make_in))
        .or_else(|| make_in(&temp))
        .ok_or_else(|| {
            let why = last_failure.take().map_or_else(String::new, |failure| {
                format!(". The last that failed was {failure}")
            });
            std::io::Error::other(format!(
                "there is nowhere to unpack the package that only administrators can get into{why}"
            ))
        })
}

fn is_real_folder(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stale_dir_in(base: &Path) -> std::path::PathBuf {
        let stale = base.join(format!("{EXTRACTION_PREFIX}ABC123"));
        let _ = std::fs::create_dir_all(stale.join("files"));
        let _ = std::fs::write(stale.join("files").join("x.bin"), b"data");
        stale
    }

    #[test]
    fn removes_stale_extraction_dirs_beside_the_executable() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let exe = base.path().join("SomeApp_installer.exe");
        std::fs::write(&exe, b"stub")?;
        let stale = stale_dir_in(base.path());
        let unrelated = base.path().join("keep-me");
        std::fs::create_dir_all(&unrelated)?;

        let later = SystemTime::now() + STALE_AFTER + Duration::from_secs(60);
        sweep_extractions_older_than(&exe, None, later, STALE_AFTER);

        assert!(!stale.exists(), "stale extraction dir should be removed");
        assert!(unrelated.exists(), "unrelated dir must be kept");
        Ok(())
    }

    /// As the review found it: a long install's folder looks old, and a second installer deleted
    /// what was in it, with the hold only keeping the folder itself.
    #[test]
    fn a_folder_held_by_a_running_installer_is_not_stale_however_old_it_looks()
    -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let exe = base.path().join("SomeApp_installer.exe");
        let held = stale_dir_in(base.path());
        let hold = winappmigrator_core::privilege::hold_folder(&held)?;
        let later = SystemTime::now() + STALE_AFTER + Duration::from_secs(60);

        assert!(!is_stale_folder(&held, later, STALE_AFTER));
        sweep_extractions_older_than(&exe, None, later, STALE_AFTER);
        assert!(held.join("files").join("x.bin").is_file());

        drop(hold);
        assert!(is_stale_folder(&held, later, STALE_AFTER));
        Ok(())
    }

    #[test]
    fn leaves_a_folder_a_running_installer_may_be_using() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let exe = base.path().join("SomeApp_installer.exe");
        let recent = stale_dir_in(base.path());

        sweep_stale_extractions(&exe, None);

        assert!(recent.join("files").join("x.bin").is_file());
        Ok(())
    }

    #[test]
    fn a_link_named_like_an_extraction_is_never_followed() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let victim = base.path().join("victim");
        std::fs::create_dir_all(&victim)?;
        std::fs::write(victim.join("important.txt"), b"keep")?;
        let link = base.path().join(format!("{EXTRACTION_PREFIX}planted"));
        let made = winappmigrator_core::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&victim)
            .output()?;
        assert!(made.status.success());

        let later = SystemTime::now() + STALE_AFTER + Duration::from_secs(60);
        assert!(!is_stale_folder(&link, later, STALE_AFTER));
        sweep_extractions_older_than(&base.path().join("x.exe"), None, later, STALE_AFTER);

        assert!(victim.join("important.txt").is_file());
        Ok(())
    }

    #[test]
    fn staging_is_created_on_the_install_destinations_drive_when_it_can_be() -> std::io::Result<()>
    {
        let base = tempfile::tempdir()?;
        let target = base.path().join(r"not\there\yet");
        let staging =
            create_staging_dir_where(Some(&target), &base.path().join("x.exe"), 0, None, &|_| {
                true
            })?;
        assert_eq!(staging.path().parent(), Some(base.path()));
        Ok(())
    }

    /// As the review found it: the folder found was real, and a link was further up the way to it.
    #[test]
    fn staging_is_not_made_through_a_link_anywhere_on_the_way() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let elsewhere = tempfile::tempdir()?;
        std::fs::create_dir_all(elsewhere.path().join("real"))?;
        let link = base.path().join("link");
        let made = winappmigrator_core::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(elsewhere.path())
            .output()?;
        assert!(made.status.success());

        let target = link.join(r"real\app");
        let staging =
            create_staging_dir_where(Some(&target), &base.path().join("x.exe"), 0, None, &|_| {
                true
            })?;

        assert_eq!(staging.path().parent(), Some(base.path()));
        assert_eq!(std::fs::read_dir(elsewhere.path().join("real"))?.count(), 0);
        Ok(())
    }

    #[test]
    fn staging_is_never_asked_to_look_at_a_share() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let staging = create_staging_dir_where(
            Some(Path::new(r"\\localhost\c$\Windows\Temp")),
            &base.path().join("x.exe"),
            0,
            None,
            &|_| true,
        )?;
        assert_eq!(staging.path().parent(), Some(base.path()));
        Ok(())
    }

    #[test]
    fn staging_passes_over_a_drive_without_room_when_another_has_it() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        // Not on any drive there is, so there is only the temp folder's to fall back on, and more
        // than any drive holds, so nothing has room and the first place that can be is used.
        let staging = create_staging_dir_where(
            Some(&base.path().join("target")),
            &base.path().join("x.exe"),
            u64::MAX,
            None,
            &|_| true,
        )?;
        assert_eq!(staging.path().parent(), Some(base.path()));
        Ok(())
    }

    #[test]
    fn staging_passes_over_a_junction_at_the_install_location() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let elsewhere = base.path().join("elsewhere");
        std::fs::create_dir_all(&elsewhere)?;
        let target = base.path().join("target");
        let made = winappmigrator_core::hidden_command("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&target)
            .arg(&elsewhere)
            .output()?;
        assert!(made.status.success());

        let staging =
            create_staging_dir_where(Some(&target), &base.path().join("x.exe"), 0, None, &|_| {
                true
            })?;

        assert_eq!(staging.path().parent(), Some(base.path()));
        assert_eq!(std::fs::read_dir(&elsewhere)?.count(), 0);
        Ok(())
    }

    /// Where an account's own folders are all there is, there is nowhere to unpack.
    #[test]
    fn nowhere_that_can_be_kept_hold_of_is_used() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let staging = create_staging_dir_where(
            Some(&base.path().join("target")),
            &base.path().join("x.exe"),
            0,
            None,
            &|_| false,
        );
        assert!(staging.is_err());
        assert_eq!(std::fs::read_dir(base.path())?.count(), 0);
        Ok(())
    }
}
