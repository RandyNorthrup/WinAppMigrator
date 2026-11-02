//! What a package is allowed to make an elevated process do.
//!
//! A package can come from a hostile or merely odd source drive, through the genuine
//! builder, and nothing authenticates it. So everything it asks for is checked here when
//! restoring, not just when scanning, and the rules are pure functions so each can be
//! tested with a realistic case on both sides. A refusal is a sentence for the person
//! installing, saying what was left out and why.
//!
//! The rules are written to let a real app's own items through: a service or Run key that
//! belongs to the app must keep working.

use std::path::{Path, PathBuf};

use crate::errors::Result;
use crate::file_discovery::{restore_destination, safe_relative_path};
use crate::models::DriverEntry;
use crate::path_rewrite::RestoreMap;
use crate::path_utils::{path_is_strictly_inside, path_is_within, sanitize_filename};
use crate::service_scan::parse_image_path;

/// The longest service name the Service Control Manager accepts.
const MAX_SERVICE_NAME_CHARS: usize = 256;
const LOCAL_SERVICE_ACCOUNT: &str = r"NT AUTHORITY\LocalService";
const NETWORK_SERVICE_ACCOUNT: &str = r"NT AUTHORITY\NetworkService";
const LOCAL_SYSTEM_ACCOUNT: &str = "LocalSystem";
const VIRTUAL_ACCOUNT_PREFIX: &str = r"NT SERVICE\";

/// True when `name` can be passed to `sc.exe` and joined to a registry path as one
/// service's name. Real names hold spaces, `$`, `.`, `(`, and `{`, as in
/// `MSSQL$SQLEXPRESS`, so this refuses only what could change the meaning of a command
/// or a key: separators, control characters, quotes, and `=`, which `sc.exe` reads as the
/// start of an option.
pub fn is_valid_service_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().count() <= MAX_SERVICE_NAME_CHARS
        && name.trim() == name
        && !name.starts_with('-')
        && !name
            .chars()
            .any(|character| character.is_control() || matches!(character, '/' | '\\' | '"' | '='))
}

/// The account a restored service runs as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceAccount {
    /// The package names none, so Windows applies its default, LocalSystem.
    Unspecified,
    LocalSystem,
    LocalService,
    NetworkService,
    /// `NT SERVICE\<service>`, an account Windows manages for that service alone.
    Virtual(String),
    /// A real user or managed account. Its password is not in the package and cannot be.
    Custom(String),
}

impl ServiceAccount {
    /// Work out the account from what the source's registry recorded. Built-in accounts
    /// have several spellings and need no password. `NT SERVICE\` only counts for the
    /// service's own name, the only one Windows would accept.
    pub fn from_recorded(recorded: Option<&str>, service_name: &str) -> Self {
        let Some(recorded) = recorded.map(str::trim).filter(|value| !value.is_empty()) else {
            return Self::Unspecified;
        };
        let lower = recorded.to_ascii_lowercase();
        let lower = lower.strip_prefix(r".\").unwrap_or(&lower);
        match lower {
            "localsystem" | r"nt authority\system" | "system" => Self::LocalSystem,
            "localservice"
            | r"nt authority\localservice"
            | r"nt authority\local service"
            | "local service" => Self::LocalService,
            "networkservice"
            | r"nt authority\networkservice"
            | r"nt authority\network service"
            | "network service" => Self::NetworkService,
            _ => {
                let virtual_name = recorded
                    .get(..VIRTUAL_ACCOUNT_PREFIX.len())
                    .filter(|prefix| prefix.eq_ignore_ascii_case(VIRTUAL_ACCOUNT_PREFIX))
                    .and_then(|prefix| recorded.strip_prefix(prefix));
                if virtual_name.is_some_and(|name| name.eq_ignore_ascii_case(service_name)) {
                    Self::Virtual(recorded.to_owned())
                } else {
                    Self::Custom(recorded.to_owned())
                }
            }
        }
    }

    /// The value for `sc.exe obj=`. A custom account has no password to go with it, so
    /// it becomes LocalService, which has few rights. Running it as LocalSystem instead
    /// would give it more than it had on the source.
    pub fn sc_object_name(&self) -> Option<&str> {
        match self {
            Self::Unspecified => None,
            Self::LocalSystem => Some(LOCAL_SYSTEM_ACCOUNT),
            Self::LocalService | Self::Custom(_) => Some(LOCAL_SERVICE_ACCOUNT),
            Self::NetworkService => Some(NETWORK_SERVICE_ACCOUNT),
            Self::Virtual(name) => Some(name),
        }
    }

    /// LocalService and NetworkService take an empty password when set. A virtual
    /// account must not be given one at all, and LocalSystem ignores it.
    pub fn sc_takes_empty_password(&self) -> bool {
        matches!(
            self,
            Self::LocalService | Self::NetworkService | Self::Custom(_)
        )
    }

    /// What to call this account in the installer's window.
    pub fn label(&self) -> String {
        match self {
            Self::Unspecified | Self::LocalSystem => LOCAL_SYSTEM_ACCOUNT.to_owned(),
            Self::LocalService => LOCAL_SERVICE_ACCOUNT.to_owned(),
            Self::NetworkService => NETWORK_SERVICE_ACCOUNT.to_owned(),
            Self::Virtual(name) => name.clone(),
            Self::Custom(_) => format!("{LOCAL_SERVICE_ACCOUNT} (custom account not restored)"),
        }
    }
}

/// The value to store as a service's `ImagePath`. Quoted, so that for
/// `C:\My Apps\Foo Bar\svc.exe` Windows cannot start `C:\My Apps\Foo.exe` instead.
///
/// `std::process::Command` removes only its own layer of quoting, and `sc.exe` stores what
/// it gets verbatim, so these quotes are what ends up in the registry. A kernel driver's
/// `ImagePath` is a path rather than a command line and must not get them.
pub fn service_image_path_value(binary: &Path, arguments: Option<&str>) -> String {
    let mut value = format!("\"{}\"", binary.display());
    if let Some(arguments) = arguments.map(str::trim).filter(|value| !value.is_empty()) {
        value.push(' ');
        value.push_str(arguments);
    }
    value
}

/// Why a service already on this PC must not be taken over, if so. `--force` does not
/// change this. A package that names `WinDefend` must not be able to repoint it, however
/// the rest of the package looks. A service already running from the install location is
/// an earlier install of this app, which is what `--force` is for.
pub fn existing_service_refusal(
    service_name: &str,
    current_image_path: &str,
    target_root: &Path,
    windows_dir: &Path,
) -> Option<String> {
    let current = parse_image_path(current_image_path, windows_dir);
    (!path_is_strictly_inside(&current.path, target_root)).then(|| {
        format!(
            "The service {service_name} already exists on this PC and runs {}, which is outside \
             the install location, so it was left as it is. If it is an older copy of this app, \
             uninstall that first.",
            current.path.display()
        )
    })
}

/// Where a bundled driver image goes, always inside the install location. The driver image
/// is not a copyable file in the inventory, so its original path is re-anchored, with
/// `<target_root>\drivers\<leaf>` for one that was not under the install location.
pub fn driver_destination(driver: &DriverEntry, map: &RestoreMap) -> Result<PathBuf> {
    let target_root = map.target_root();
    let rewritten = PathBuf::from(map.rewrite(&driver.image_path.to_string_lossy()));
    if path_is_strictly_inside(&rewritten, target_root) {
        return Ok(rewritten);
    }
    let leaf = driver.image_path.file_name().map_or_else(
        || format!("{}.sys", sanitize_filename(&driver.service_name)),
        |name| name.to_string_lossy().into_owned(),
    );
    restore_destination(
        target_root,
        &Path::new("drivers").join(safe_relative_path(Path::new(&leaf))),
    )
}

/// True when the `ImagePath` Windows reports runs exactly this binary with these arguments.
pub fn image_path_matches(
    actual: &str,
    expected_binary: &Path,
    expected_arguments: Option<&str>,
    windows_dir: &Path,
) -> bool {
    let actual = parse_image_path(actual, windows_dir);
    paths_equal(&actual.path, expected_binary)
        && actual
            .arguments
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            == expected_arguments.map(str::trim).unwrap_or_default()
}

/// Paths compared as Windows would, ignoring case and a trailing separator, and no looser.
pub fn paths_equal(left: &Path, right: &Path) -> bool {
    path_is_within(left, right) && path_is_within(right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOWS: &str = r"C:\Windows";

    #[test]
    fn ordinary_service_names_are_accepted() {
        for name in [
            "AcmeSync",
            "MSSQL$SQLEXPRESS",
            "Apache2.4",
            "Steam Client Service",
            "Intel(R) SUR QC SAM",
            "{B5F3A1D2-0000-4000-8000-000000000000}",
            "vmware-converter-agent",
            "NVDisplay.ContainerLocalSystem",
            "Überwachung",
        ] {
            assert!(is_valid_service_name(name), "{name}");
        }
    }

    #[test]
    fn names_that_change_a_command_or_a_key_are_refused() {
        let too_long = "x".repeat(MAX_SERVICE_NAME_CHARS + 1);
        for name in [
            "",
            " ",
            " padded",
            "padded ",
            r"..\Other",
            "a/b",
            "start=",
            "-flag",
            "a\"b",
            "line\nbreak",
            too_long.as_str(),
        ] {
            assert!(!is_valid_service_name(name), "{name:?}");
        }
    }

    #[test]
    fn built_in_accounts_are_recognized_in_every_spelling() {
        for recorded in ["LocalSystem", r".\LocalSystem", r"NT AUTHORITY\SYSTEM"] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::LocalSystem,
                "{recorded}"
            );
        }
        for recorded in [r"NT AUTHORITY\LocalService", r"nt authority\LOCAL SERVICE"] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::LocalService,
                "{recorded}"
            );
        }
        for recorded in [
            r"NT AUTHORITY\NetworkService",
            r"NT AUTHORITY\Network Service",
        ] {
            assert_eq!(
                ServiceAccount::from_recorded(Some(recorded), "App"),
                ServiceAccount::NetworkService,
                "{recorded}"
            );
        }
        assert_eq!(
            ServiceAccount::from_recorded(None, "App"),
            ServiceAccount::Unspecified
        );
    }

    #[test]
    fn low_rights_accounts_are_kept_and_not_turned_into_local_system() {
        let local = ServiceAccount::from_recorded(Some(r"NT AUTHORITY\LocalService"), "App");
        assert_eq!(local.sc_object_name(), Some(r"NT AUTHORITY\LocalService"));
        assert!(local.sc_takes_empty_password());

        let network = ServiceAccount::from_recorded(Some(r"NT AUTHORITY\NETWORK SERVICE"), "App");
        assert_eq!(
            network.sc_object_name(),
            Some(r"NT AUTHORITY\NetworkService")
        );

        let system = ServiceAccount::from_recorded(Some("LocalSystem"), "App");
        assert_eq!(system.sc_object_name(), Some("LocalSystem"));
        assert!(!system.sc_takes_empty_password());
    }

    #[test]
    fn a_virtual_account_is_only_the_services_own() {
        let own = ServiceAccount::from_recorded(Some(r"NT SERVICE\AcmeSync"), "AcmeSync");
        assert_eq!(own.sc_object_name(), Some(r"NT SERVICE\AcmeSync"));
        assert!(!own.sc_takes_empty_password());

        let other = ServiceAccount::from_recorded(Some(r"NT SERVICE\TrustedInstaller"), "AcmeSync");
        assert!(matches!(other, ServiceAccount::Custom(_)));
    }

    #[test]
    fn a_custom_account_loses_rights_rather_than_gaining_them() {
        let custom = ServiceAccount::from_recorded(Some(r"CONTOSO\svc-acme"), "AcmeSync");
        assert_eq!(
            custom,
            ServiceAccount::Custom(r"CONTOSO\svc-acme".to_owned())
        );
        assert_eq!(custom.sc_object_name(), Some(r"NT AUTHORITY\LocalService"));
        assert!(custom.label().contains("not restored"));
    }

    #[test]
    fn the_stored_image_path_is_quoted_and_keeps_its_arguments() {
        assert_eq!(
            service_image_path_value(Path::new(r"C:\My Apps\Foo Bar\svc.exe"), None),
            r#""C:\My Apps\Foo Bar\svc.exe""#
        );
        assert_eq!(
            service_image_path_value(
                Path::new(r"C:\My Apps\Foo Bar\svc.exe"),
                Some(r#"--config "C:\My Apps\Foo Bar\app.conf" -v"#)
            ),
            r#""C:\My Apps\Foo Bar\svc.exe" --config "C:\My Apps\Foo Bar\app.conf" -v"#
        );
        assert_eq!(
            service_image_path_value(Path::new(r"C:\App\svc.exe"), Some("  ")),
            r#""C:\App\svc.exe""#
        );
    }

    #[test]
    fn a_service_outside_the_install_location_is_never_taken_over() {
        let target = Path::new(r"C:\Program Files\Acme");
        let windows = Path::new(WINDOWS);
        for current in [
            r#""C:\ProgramData\Microsoft\Windows Defender\Platform\4.18\MsMpEng.exe""#,
            r"%SystemRoot%\System32\svchost.exe -k netsvcs",
            r"C:\Program Files\Acme Other\svc.exe",
            r"C:\Program Files\Acme\..\Other\svc.exe",
            "",
        ] {
            let refusal = existing_service_refusal("WinDefend", current, target, windows);
            assert!(refusal.is_some(), "{current}");
        }
    }

    #[test]
    fn an_earlier_install_in_the_same_place_can_be_replaced() {
        let target = Path::new(r"C:\Program Files\Acme");
        let windows = Path::new(WINDOWS);
        for current in [
            r#""C:\Program Files\Acme\svc.exe" --run"#,
            r"C:\Program Files\Acme\bin\svc.exe",
            r"c:\program files\acme\SVC.EXE",
        ] {
            assert_eq!(
                existing_service_refusal("AcmeSync", current, target, windows),
                None,
                "{current}"
            );
        }
    }

    #[test]
    fn checking_an_image_path_compares_the_binary_and_arguments_exactly() {
        let windows = Path::new(WINDOWS);
        let binary = Path::new(r"C:\Program Files\Git\svc.exe");
        assert!(image_path_matches(
            r#""C:\Program Files\Git\svc.exe" --run"#,
            binary,
            Some("--run"),
            windows
        ));
        assert!(image_path_matches(
            r#""c:\program files\git\SVC.exe""#,
            binary,
            None,
            windows
        ));
        // A path that merely contains the expected one used to pass.
        assert!(!image_path_matches(
            r#""D:\Evil\C:\Program Files\Git\svc.exe""#,
            binary,
            None,
            windows
        ));
        assert!(!image_path_matches(
            r"C:\Program Files\Git\svc.exe.bak",
            binary,
            None,
            windows
        ));
        assert!(!image_path_matches(
            r#""C:\Program Files\Git\svc.exe" --other"#,
            binary,
            Some("--run"),
            windows
        ));
    }
}
