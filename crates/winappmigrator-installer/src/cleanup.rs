//! Removes package-extraction directories left by previous or crashed installer
//! runs, so nothing accumulates over time regardless of how a prior run ended.

use std::path::Path;
use std::time::{Duration, SystemTime};

use winappmigrator_core::privilege::protect_staging_folder;

/// Prefix of the directories the installer extracts its embedded package into.
pub const EXTRACTION_PREFIX: &str = "winappmigrator-install-";

/// A younger extraction folder may belong to an installer that is still running. An
/// install that has been going for this long and has stopped touching its folder is not
/// expected, and a folder left behind is only wasted space until a later run.
const STALE_AFTER: Duration = Duration::from_secs(60 * 60);

/// Remove stale extraction directories from the two places the installer stages
/// its package — beside the installer executable and in the system temp
/// directory. Call this at startup **before** creating the current run's
/// directory, so only prior leftovers are removed. Best-effort, and anything newer
/// than [`STALE_AFTER`] is left alone, because it can be another installer's.
pub fn sweep_stale_extractions(executable: &Path) {
    sweep_extractions_older_than(executable, SystemTime::now(), STALE_AFTER);
}

fn sweep_extractions_older_than(executable: &Path, now: SystemTime, age: Duration) {
    let mut roots = vec![std::env::temp_dir()];
    if let Some(parent) = executable.parent() {
        let parent = parent.to_path_buf();
        if !roots.contains(&parent) {
            roots.push(parent);
        }
    }
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

/// A real folder, not touched for `age`. Both folders swept are ones another process can
/// write to, so a link given the right name must not be followed or removed.
fn is_stale_folder(path: &Path, now: SystemTime, age: Duration) -> bool {
    let Ok(metadata) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return false;
    }
    metadata
        .modified()
        .ok()
        .and_then(|modified| now.duration_since(modified).ok())
        .is_some_and(|elapsed| elapsed >= age)
}

/// Create a fresh extraction staging directory, preferring a drive with room for
/// a large package: the install destination's drive first (when `target_root` is
/// given), then the installer executable's own drive, then the system temp drive
/// — so a large package never fails to extract for lack of space on `C:`. Using
/// [`EXTRACTION_PREFIX`] keeps every staging directory matchable by
/// [`sweep_stale_extractions`]. Auto-removed when the returned `TempDir` drops.
///
/// Elevated code reads from this folder, so it is made admin-only before anything is
/// written there. A location where that fails is not used, and if none works the
/// install does not go ahead.
pub fn create_staging_dir(
    target_root: Option<&Path>,
    executable: &Path,
) -> std::io::Result<tempfile::TempDir> {
    // The nearest existing ancestor of the install location sits on the
    // destination drive.
    let mut ancestor = target_root;
    while let Some(dir) = ancestor {
        if dir.is_dir()
            && let Ok(temp) = staging_in(dir)
        {
            return Ok(temp);
        }
        ancestor = dir.parent();
    }
    if let Some(parent) = executable.parent()
        && let Ok(temp) = staging_in(parent)
    {
        return Ok(temp);
    }
    staging_in(&std::env::temp_dir())
}

fn staging_in(dir: &Path) -> std::io::Result<tempfile::TempDir> {
    let temp = tempfile::Builder::new()
        .prefix(EXTRACTION_PREFIX)
        .tempdir_in(dir)?;
    protect_staging_folder(temp.path())?;
    Ok(temp)
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
        sweep_extractions_older_than(&exe, later, STALE_AFTER);

        assert!(!stale.exists(), "stale extraction dir should be removed");
        assert!(unrelated.exists(), "unrelated dir must be kept");
        Ok(())
    }

    #[test]
    fn leaves_a_folder_a_running_installer_may_be_using() -> std::io::Result<()> {
        let base = tempfile::tempdir()?;
        let exe = base.path().join("SomeApp_installer.exe");
        let recent = stale_dir_in(base.path());

        sweep_stale_extractions(&exe);

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
        sweep_extractions_older_than(&base.path().join("x.exe"), later, STALE_AFTER);

        assert!(victim.join("important.txt").is_file());
        Ok(())
    }

    #[test]
    fn staging_is_created_on_the_install_destinations_drive_when_it_can_be() -> std::io::Result<()>
    {
        let base = tempfile::tempdir()?;
        let target = base.path().join(r"not\there\yet");
        let staging = create_staging_dir(Some(&target), &base.path().join("x.exe"))?;
        assert_eq!(staging.path().parent(), Some(base.path()));
        Ok(())
    }
}
