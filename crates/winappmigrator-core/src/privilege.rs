use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::OnceLock;

use crate::path_utils::hidden_command;

/// SIDs rather than names, because the names are translated on other Windows languages.
const ADMINISTRATORS_ONLY_GRANTS: [&str; 2] = ["*S-1-5-32-544:(OI)(CI)F", "*S-1-5-18:(OI)(CI)F"];

/// Whether this process runs as an administrator. Asking spawns PowerShell, about a
/// second, and the answer cannot change during a run, so it is asked once.
pub fn is_elevated() -> bool {
    static ELEVATED: OnceLock<bool> = OnceLock::new();
    *ELEVATED.get_or_init(query_elevation)
}

fn query_elevation() -> bool {
    let script = "[Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent() | ForEach-Object { $_.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator) }";
    hidden_command("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .ok()
        .and_then(|output| {
            output.status.success().then(|| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .eq_ignore_ascii_case("true")
            })
        })
        .unwrap_or(false)
}

/// Make a freshly created staging folder writable only by administrators, so a process
/// that is not elevated cannot change what elevated code is about to read from it. Call
/// it before writing anything there, and treat a failure as fatal.
///
/// Without elevation there is no boundary to protect, and this process could not read
/// the folder afterward, so development builds and tests leave it alone.
pub fn protect_staging_folder(folder: &Path) -> io::Result<()> {
    if is_elevated() {
        restrict_to_administrators(folder)?;
        refuse_anything_already_inside(folder)?;
    }
    Ok(())
}

/// The arguments that replace a folder's permissions with administrators and SYSTEM.
fn administrators_only_arguments(folder: &Path) -> Vec<OsString> {
    let mut arguments = vec![
        folder.as_os_str().to_owned(),
        OsString::from("/inheritance:r"),
        OsString::from("/grant:r"),
    ];
    arguments.extend(ADMINISTRATORS_ONLY_GRANTS.map(OsString::from));
    arguments
}

fn restrict_to_administrators(folder: &Path) -> io::Result<()> {
    let output = hidden_command("icacls")
        .args(administrators_only_arguments(folder))
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "could not restrict {} to administrators: {}{}",
        folder.display(),
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

/// The folder was empty when it was created. Anything there now arrived in the moment
/// before the permissions changed, and its own permissions may still be open.
fn refuse_anything_already_inside(folder: &Path) -> io::Result<()> {
    if fs::read_dir(folder)?.next().is_some() {
        return Err(io::Error::other(format!(
            "another process wrote to {} before it could be protected",
            folder.display()
        )));
    }
    Ok(())
}

pub fn target_requires_admin(target_root: &Path) -> bool {
    path_under_any_env(
        target_root,
        &["ProgramFiles", "ProgramFiles(x86)", "SystemRoot"],
    )
}

/// True when writing to `destination` modifies a machine-wide, protected
/// location (ProgramData or the same protected roots as `target_requires_admin`)
/// and therefore needs elevation. Used to gate real-location user-data restore.
pub fn destination_requires_admin(destination: &Path) -> bool {
    path_under_any_env(
        destination,
        &[
            "ProgramFiles",
            "ProgramFiles(x86)",
            "ProgramData",
            "SystemRoot",
        ],
    )
}

fn path_under_any_env(path: &Path, env_vars: &[&str]) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    env_vars.iter().any(|env_var| {
        std::env::var_os(env_var)
            .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asking_about_elevation_twice_spawns_nothing_the_second_time() {
        let first = is_elevated();
        let started = std::time::Instant::now();
        assert_eq!(is_elevated(), first);
        assert!(started.elapsed() < std::time::Duration::from_millis(100));
    }

    #[test]
    fn the_acl_arguments_name_only_administrators_and_system() {
        assert_eq!(
            administrators_only_arguments(Path::new(r"D:\Stage Here")),
            [
                r"D:\Stage Here",
                "/inheritance:r",
                "/grant:r",
                "*S-1-5-32-544:(OI)(CI)F",
                "*S-1-5-18:(OI)(CI)F",
            ]
            .map(OsString::from)
        );
    }

    /// Runs the real command on a temp folder. A normal shell is locked out by it, which
    /// shows it worked. An elevated shell is not, and takes the other branch.
    #[test]
    fn the_acl_locks_out_a_process_that_is_not_elevated() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        restrict_to_administrators(folder.path())?;
        let listed = fs::read_dir(folder.path()).map(|_| ());
        // The owner can always change permissions, which lets the folder be cleaned up.
        let reset = hidden_command("icacls")
            .arg(folder.path())
            .arg("/reset")
            .output()?;
        assert!(reset.status.success());
        if is_elevated() {
            assert!(listed.is_ok());
        } else {
            let denied = listed.err().map(|error| error.kind());
            assert_eq!(denied, Some(io::ErrorKind::PermissionDenied));
        }
        Ok(())
    }

    #[test]
    fn a_folder_someone_got_into_first_is_refused() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        assert!(refuse_anything_already_inside(folder.path()).is_ok());
        fs::write(folder.path().join("planted.exe"), b"x")?;
        assert!(refuse_anything_already_inside(folder.path()).is_err());
        Ok(())
    }

    #[test]
    fn protecting_a_folder_leaves_it_usable_by_this_process() -> io::Result<()> {
        let folder = tempfile::tempdir()?;
        protect_staging_folder(folder.path())?;
        fs::write(folder.path().join("manifest.json"), b"{}")?;
        assert!(folder.path().join("manifest.json").is_file());
        Ok(())
    }
}
