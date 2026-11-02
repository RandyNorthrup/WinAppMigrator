use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::OnceLock;

use tempfile::TempDir;

use crate::path_utils::hidden_command;

/// SIDs rather than names, because the names are translated on other Windows languages.
const ADMINISTRATORS_SID: &str = "*S-1-5-32-544";
const ADMINISTRATORS_ONLY_GRANTS: [&str; 2] = ["*S-1-5-32-544:(OI)(CI)F", "*S-1-5-18:(OI)(CI)F"];
/// The mandatory labels of a process running with full rights: High, and System.
const ELEVATED_LABELS: [&str; 2] = ["S-1-16-12288", "S-1-16-16384"];
const FILE_SHARE_READ: u32 = 0x0000_0001;
const FILE_SHARE_WRITE: u32 = 0x0000_0002;
/// What lets `CreateFile` open a folder at all.
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;

/// Whether this process runs with full administrator rights.
pub fn is_elevated() -> bool {
    elevation() == Some(true)
}

/// Asked once, since the answer cannot change during a run. `None` means it could not be
/// found out, which is not the same as no: asking runs a program, and that can be made to
/// fail. Asking `whoami` rather than PowerShell also takes a fraction of the time.
fn elevation() -> Option<bool> {
    static ELEVATED: OnceLock<Option<bool>> = OnceLock::new();
    *ELEVATED.get_or_init(query_elevation)
}

fn query_elevation() -> Option<bool> {
    let output = hidden_command("whoami")
        .args(["/groups", "/fo", "csv"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| groups_show_elevation(&String::from_utf8_lossy(&output.stdout)))
}

/// The group list shows the process's mandatory label by SID, whatever the language.
fn groups_show_elevation(groups: &str) -> bool {
    ELEVATED_LABELS.iter().any(|label| groups.contains(label))
}

/// A folder that elevated code reads from: new, held open so it and the folders above it
/// cannot be renamed and replaced, and when elevated writable only by administrators.
/// Removed when dropped.
///
/// Permissions on a folder do not stop whoever controls its parent from renaming it and
/// putting another in its place, and everything reads by path. Holding it open does.
pub struct ProtectedFolder {
    // In this order, because a field drops in the order it is declared, and the folder
    // cannot be removed while it is held.
    _hold: File,
    temp: TempDir,
}

impl ProtectedFolder {
    /// Create one in `parent`, with a name starting `prefix`. A failure means nothing
    /// should be written there.
    pub fn new_in(parent: &Path, prefix: &str) -> io::Result<Self> {
        let temp = tempfile::Builder::new().prefix(prefix).tempdir_in(parent)?;
        Self::protect(temp)
    }

    /// Create one in the system's temp folder.
    pub fn new_in_temp(prefix: &str) -> io::Result<Self> {
        let temp = tempfile::Builder::new().prefix(prefix).tempdir()?;
        Self::protect(temp)
    }

    fn protect(temp: TempDir) -> io::Result<Self> {
        let hold = hold_folder(temp.path())?;
        protect_staging_folder(temp.path())?;
        Ok(Self { _hold: hold, temp })
    }

    pub fn path(&self) -> &Path {
        self.temp.path()
    }
}

/// Hold a folder open with no delete sharing. While the result is alive, neither it nor
/// any folder above it can be renamed, and what is inside can still be used normally.
pub fn hold_folder(folder: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(folder)
}

/// Make a freshly created staging folder writable only by administrators, so a process
/// that is not elevated cannot change what elevated code is about to read from it.
///
/// Without elevation there is no boundary to protect, and this process could not read
/// the folder afterward, so development builds and tests leave it alone. Not knowing
/// is a failure: a process that was never asked to be fooled about this cannot tell.
fn protect_staging_folder(folder: &Path) -> io::Result<()> {
    match elevation() {
        Some(true) => {
            take_ownership_for_administrators(folder)?;
            restrict_to_administrators(folder)?;
            refuse_anything_already_inside(folder)
        }
        Some(false) => Ok(()),
        None => Err(io::Error::other(
            "could not tell whether this process is elevated",
        )),
    }
}

/// The owner of a folder can always change its permissions, so the owner has to be
/// administrators too. It normally is already, but policy can make it the signed-in user.
fn administrators_owner_arguments(folder: &Path) -> Vec<OsString> {
    vec![
        folder.as_os_str().to_owned(),
        OsString::from("/setowner"),
        OsString::from(ADMINISTRATORS_SID),
    ]
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

fn run_icacls(folder: &Path, arguments: Vec<OsString>) -> io::Result<()> {
    let output = hidden_command("icacls").args(arguments).output()?;
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

fn take_ownership_for_administrators(folder: &Path) -> io::Result<()> {
    run_icacls(folder, administrators_owner_arguments(folder))
}

fn restrict_to_administrators(folder: &Path) -> io::Result<()> {
    run_icacls(folder, administrators_only_arguments(folder))
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
    fn this_shell_can_tell_whether_it_is_elevated() {
        assert!(elevation().is_some());
    }

    #[test]
    fn elevation_is_read_from_the_mandatory_label() {
        let medium = r#""Everyone","Well-known group","S-1-1-0","Mandatory group"
"Mandatory Label\Medium Mandatory Level","Label","S-1-16-8192","""#;
        let high = r#""BUILTIN\Administrators","Alias","S-1-5-32-544","Mandatory group"
"Mandatory Label\High Mandatory Level","Label","S-1-16-12288","""#;
        let system =
            r#""Etiqueta obligatoria\Nivel obligatorio del sistema","Etiqueta","S-1-16-16384","""#;
        assert!(!groups_show_elevation(medium));
        assert!(groups_show_elevation(high));
        assert!(groups_show_elevation(system));
    }

    #[test]
    fn the_owner_arguments_name_administrators() {
        assert_eq!(
            administrators_owner_arguments(Path::new(r"D:\Stage Here")),
            [r"D:\Stage Here", "/setowner", "*S-1-5-32-544"].map(OsString::from)
        );
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
    fn a_protected_folder_is_usable_and_gone_when_dropped() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = ProtectedFolder::new_in(parent.path(), "stage-")?;
        let path = folder.path().to_path_buf();
        assert_eq!(path.parent(), Some(parent.path()));
        fs::write(path.join("manifest.json"), b"{}")?;
        assert!(path.join("manifest.json").is_file());
        drop(folder);
        assert!(!path.exists());
        Ok(())
    }

    /// Run for real in a normal shell, against the rename that an access list does not stop.
    #[test]
    fn a_held_folder_and_the_folders_above_it_cannot_be_renamed() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let above = parent.path().join("above");
        let folder = above.join("stage");
        fs::create_dir_all(&folder)?;

        let hold = hold_folder(&folder)?;
        assert!(fs::rename(&folder, above.join("moved")).is_err());
        assert!(fs::rename(&above, parent.path().join("moved")).is_err());
        fs::write(folder.join("inside.txt"), b"still usable")?;
        fs::remove_file(folder.join("inside.txt"))?;

        drop(hold);
        fs::rename(&above, parent.path().join("moved"))?;
        Ok(())
    }

    #[test]
    fn a_protected_folder_cannot_be_renamed_out_from_under_its_user() -> io::Result<()> {
        let parent = tempfile::tempdir()?;
        let folder = ProtectedFolder::new_in(parent.path(), "stage-")?;
        assert!(fs::rename(folder.path(), parent.path().join("moved")).is_err());
        Ok(())
    }
}
