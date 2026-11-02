use std::path::{Path, PathBuf};

use winreg::enums::KEY_READ;

use crate::file_discovery::expand_env_vars;
use crate::models::{DriverEntry, DriverKind, ServiceEntry, Warning, WarningSeverity};
use crate::path_utils::{escape_powershell_single, hidden_command, path_is_within};
use crate::registry_scan::capture_hklm_subtree;
use crate::restore_policy::service_image_path_value;
use crate::system_context::SystemContext;

const POWERSHELL_EXE: &str = "powershell.exe";

const SERVICES_KEY: &str = r"SYSTEM\CurrentControlSet\Services";
const SERVICE_KERNEL_DRIVER: u32 = 0x0000_0001;
const SERVICE_FILE_SYSTEM_DRIVER: u32 = 0x0000_0002;
const SERVICE_RECOGNIZER_DRIVER: u32 = 0x0000_0008;
const PARAMETERS_SUBKEY: &str = "Parameters";
/// What an unquoted `ImagePath` can end in. Without quotes, nothing else says where a
/// path with spaces stops and its arguments start.
const IMAGE_EXTENSIONS: [&str; 5] = [".exe", ".sys", ".dll", ".com", ".bat"];
/// Prefixes the kernel's namespace puts on a path. They mean the same as the path without.
const NT_PATH_PREFIXES: [&str; 4] = [r"\??\", r"\\?\", r"\DosDevices\", r"\GLOBAL??\"];
/// Ways of writing the Windows folder at the start of an `ImagePath`.
const WINDOWS_DIR_PREFIXES: [&str; 4] = [
    r"\SystemRoot\",
    r"%SystemRoot%\",
    r"%windir%\",
    r"SystemRoot\",
];

#[derive(Debug, Clone)]
pub struct ServiceScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

pub fn scan_services(
    context: &ServiceScanContext,
    system: &SystemContext,
) -> (Vec<ServiceEntry>, Vec<Warning>) {
    let mut services = Vec::new();
    let mut warnings = Vec::new();
    let windows_dir = system.windows_dir();
    let services_key = match system.open_hklm(SERVICES_KEY, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            warnings.push(Warning::new(
                "services_registry_unreadable",
                WarningSeverity::Warning,
                format!("Could not read services registry key: {error}"),
                Some(SERVICES_KEY.to_owned()),
            ));
            return (services, warnings);
        }
    };

    for service_name in services_key.enum_keys().flatten() {
        let service_key = match services_key.open_subkey_with_flags(&service_name, KEY_READ) {
            Ok(key) => key,
            Err(_) => continue,
        };
        let service_type = service_key.get_value::<u32, _>("Type").unwrap_or_default();
        if is_driver_service_type(service_type) {
            continue;
        }

        let Ok(image_path) = service_key.get_value::<String, _>("ImagePath") else {
            continue;
        };
        let image_path = parse_image_path(&image_path, &windows_dir);
        let binary_path = system.resolve_source_path(&image_path.path);
        if !path_matches(context, &binary_path) {
            continue;
        }

        let display_name = service_key.get_value::<String, _>("DisplayName").ok();
        let description = service_key.get_value::<String, _>("Description").ok();
        let account_name = service_key.get_value::<String, _>("ObjectName").ok();
        let start_type = service_key
            .get_value::<u32, _>("Start")
            .ok()
            .map(start_type_label);
        let dependencies = service_key
            .get_value::<Vec<String>, _>("DependOnService")
            .unwrap_or_default();

        let parameters = capture_hklm_subtree(
            system,
            &format!(r"{SERVICES_KEY}\{service_name}\{PARAMETERS_SUBKEY}"),
            &mut warnings,
        );

        services.push(ServiceEntry {
            service_name: service_name.clone(),
            display_name,
            binary_path: binary_path.clone(),
            arguments: image_path.arguments.clone(),
            parameters,
            start_type,
            account_name,
            dependencies,
            description,
            requires_admin: true,
            restore_command_suggestion: format!(
                "sc.exe create \"{}\" binPath= {}",
                service_name,
                service_image_path_value(&binary_path, image_path.arguments.as_deref())
            ),
        });
    }

    services.sort_by(|left, right| left.service_name.cmp(&right.service_name));
    (services, warnings)
}

pub fn scan_drivers(
    context: &ServiceScanContext,
    system: &SystemContext,
) -> (Vec<DriverEntry>, Vec<Warning>) {
    let mut drivers = Vec::new();
    let mut warnings = Vec::new();
    let windows_dir = system.windows_dir();
    let services_key = match system.open_hklm(SERVICES_KEY, KEY_READ) {
        Ok(key) => key,
        Err(error) => {
            warnings.push(Warning::new(
                "drivers_registry_unreadable",
                WarningSeverity::Warning,
                format!("Could not read services registry key for driver discovery: {error}"),
                Some(SERVICES_KEY.to_owned()),
            ));
            return (drivers, warnings);
        }
    };

    for service_name in services_key.enum_keys().flatten() {
        let service_key = match services_key.open_subkey_with_flags(&service_name, KEY_READ) {
            Ok(key) => key,
            Err(_) => continue,
        };
        let service_type = service_key.get_value::<u32, _>("Type").unwrap_or_default();
        if !is_driver_service_type(service_type) {
            continue;
        }
        let Ok(image_path) = service_key.get_value::<String, _>("ImagePath") else {
            continue;
        };
        let binary_path =
            system.resolve_source_path(&parse_image_path(&image_path, &windows_dir).path);
        if !path_matches(context, &binary_path) {
            continue;
        }

        let kind = driver_kind(service_type);
        let app_owned = is_app_owned_driver(context, &binary_path);
        let signer = driver_signer(&binary_path);
        let restore_instructions =
            driver_restore_instructions(&service_name, &binary_path, &kind, app_owned, &signer);

        drivers.push(DriverEntry {
            service_name: service_name.clone(),
            display_name: service_key.get_value::<String, _>("DisplayName").ok(),
            image_path: binary_path,
            driver_kind: kind,
            start_type: service_key
                .get_value::<u32, _>("Start")
                .ok()
                .map(start_type_label),
            account_name: service_key.get_value::<String, _>("ObjectName").ok(),
            description: service_key.get_value::<String, _>("Description").ok(),
            requires_admin: true,
            restore_instructions,
            app_owned,
            signer,
            hash_sha256: None,
            package_relative_path: None,
        });
    }

    drivers.sort_by(|left, right| left.service_name.cmp(&right.service_name));
    (drivers, warnings)
}

fn path_matches(context: &ServiceScanContext, candidate: &Path) -> bool {
    path_eq(candidate, &context.executable_path)
        || context
            .install_location
            .as_ref()
            .is_some_and(|root| path_is_within(candidate, root))
}

/// A driver image is application-owned when it lives inside the app's install
/// footprint, exists on disk, and is not a Windows inbox/System32 driver. Only
/// these are bundled and re-registered automatically; inbox/vendor drivers are
/// not redistributed.
fn is_app_owned_driver(context: &ServiceScanContext, image_path: &Path) -> bool {
    let lower = image_path.to_string_lossy().to_ascii_lowercase();
    if lower.contains(r"\windows\") {
        return false;
    }
    let under_install = context
        .install_location
        .as_ref()
        .is_some_and(|root| path_is_within(image_path, root));
    under_install && image_path.is_file()
}

/// Best-effort Authenticode signer subject for a driver image. Spawns PowerShell
/// only for drivers that matched the app (typically zero or one), so cost is
/// bounded.
fn driver_signer(image_path: &Path) -> Option<String> {
    if !image_path.is_file() {
        return None;
    }
    let script = format!(
        "$s = Get-AuthenticodeSignature -LiteralPath '{}'; if ($s -and $s.SignerCertificate) {{ $s.SignerCertificate.Subject }}",
        escape_powershell_single(&image_path.display().to_string())
    );
    let output = hidden_command(POWERSHELL_EXE)
        .args(["-NoProfile", "-NonInteractive", "-Command", script.as_str()])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let subject = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    (!subject.is_empty()).then_some(subject)
}

fn driver_restore_instructions(
    service_name: &str,
    image_path: &Path,
    kind: &DriverKind,
    app_owned: bool,
    signer: &Option<String>,
) -> String {
    let signed = signer
        .as_deref()
        .map(|subject| format!(", signed by {subject}"))
        .unwrap_or_else(|| ", signature unknown".to_owned());
    if app_owned {
        format!(
            "Application-owned {} driver '{}' ({}{}). The installer bundles its .sys and re-registers the driver service automatically (sc create, requires elevation), then confirms the service is registered.",
            driver_kind_label(kind),
            service_name,
            image_path.display(),
            signed
        )
    } else {
        format!(
            "Vendor/inbox {} driver '{}' ({}{}). WinAppMigrator does not redistribute third-party drivers; reinstall it with the vendor's signed driver package on the target before launching the app.",
            driver_kind_label(kind),
            service_name,
            image_path.display(),
            signed
        )
    }
}

fn driver_kind_label(kind: &DriverKind) -> &'static str {
    match kind {
        DriverKind::Kernel => "kernel",
        DriverKind::FileSystem => "file-system",
        DriverKind::Recognizer => "recognizer",
        DriverKind::Unknown => "unknown-type",
    }
}

fn path_eq(left: &Path, right: &Path) -> bool {
    left.to_string_lossy()
        .eq_ignore_ascii_case(&right.to_string_lossy())
}

fn start_type_label(value: u32) -> String {
    match value {
        0 => "boot",
        1 => "system",
        2 => "automatic",
        3 => "manual",
        4 => "disabled",
        _ => "unknown",
    }
    .to_owned()
}

fn is_driver_service_type(value: u32) -> bool {
    value & (SERVICE_KERNEL_DRIVER | SERVICE_FILE_SYSTEM_DRIVER | SERVICE_RECOGNIZER_DRIVER) != 0
}

fn driver_kind(value: u32) -> DriverKind {
    if value & SERVICE_KERNEL_DRIVER != 0 {
        DriverKind::Kernel
    } else if value & SERVICE_FILE_SYSTEM_DRIVER != 0 {
        DriverKind::FileSystem
    } else if value & SERVICE_RECOGNIZER_DRIVER != 0 {
        DriverKind::Recognizer
    } else {
        DriverKind::Unknown
    }
}

/// A service's or driver's `ImagePath`, split into what it runs and what it passes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImagePath {
    pub path: PathBuf,
    pub arguments: Option<String>,
}

/// Read a service's or driver's `ImagePath`. It is either a command line, quoted or not,
/// or for a driver an NT path, and can be relative to the Windows folder. The Windows
/// folder is `windows_dir`, the scanned system's, which is the mounted volume's offline.
pub(crate) fn parse_image_path(raw: &str, windows_dir: &Path) -> ImagePath {
    let trimmed = raw.trim();
    let (path, arguments) = match trimmed.strip_prefix('"') {
        Some(rest) => match rest.split_once('"') {
            Some((path, arguments)) => (path, arguments),
            None => (rest, ""),
        },
        None => split_unquoted_image_path(trimmed),
    };
    let arguments = arguments.trim();
    ImagePath {
        path: expand_image_file(path.trim(), windows_dir),
        arguments: (!arguments.is_empty()).then(|| arguments.to_owned()),
    }
}

/// Where an unquoted path stops: after the first file extension that ends a word. Spaces
/// alone do not say, and `C:\Program Files\Vendor\flt.sys` is a normal value. With no known
/// extension the first word is all that can be said to be the path.
fn split_unquoted_image_path(value: &str) -> (&str, &str) {
    let lower = value.to_ascii_lowercase();
    let end = IMAGE_EXTENSIONS
        .iter()
        .filter_map(|extension| {
            lower.match_indices(extension).find_map(|(start, _)| {
                let end = start + extension.len();
                lower[end..]
                    .chars()
                    .next()
                    .is_none_or(char::is_whitespace)
                    .then_some(end)
            })
        })
        .min();
    let end = end.or_else(|| value.find(char::is_whitespace));
    match end {
        Some(end) => value.split_at(end),
        None => (value, ""),
    }
}

/// Turn the path part of an `ImagePath` into a file path.
fn expand_image_file(path: &str, windows_dir: &Path) -> PathBuf {
    let mut path = path;
    for prefix in NT_PATH_PREFIXES {
        if let Some(rest) = strip_prefix_ci(path, prefix) {
            path = rest;
            break;
        }
    }
    for marker in WINDOWS_DIR_PREFIXES {
        if let Some(rest) = strip_prefix_ci(path, marker) {
            return windows_dir.join(rest);
        }
    }
    // Other `%VAR%` markers expand from the live environment, which does nothing offline,
    // where those variables are absent.
    let expanded = PathBuf::from(expand_env_vars(path));
    // Windows resolves a driver's relative path, usually `system32\drivers\x.sys`, against
    // its own folder.
    if expanded.is_relative() && !expanded.as_os_str().is_empty() {
        return windows_dir.join(expanded);
    }
    expanded
}

/// Strip a leading prefix case-insensitively, returning the remainder.
fn strip_prefix_ci<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    let head = value.get(..prefix.len())?;
    head.eq_ignore_ascii_case(prefix)
        .then(|| &value[prefix.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOWS: &str = r"C:\Windows";

    fn parsed(raw: &str) -> (String, Option<String>) {
        let image = parse_image_path(raw, Path::new(WINDOWS));
        (image.path.display().to_string(), image.arguments)
    }

    fn path_only(path: &str) -> (String, Option<String>) {
        (path.to_owned(), None)
    }

    #[test]
    fn a_quoted_command_line_splits_at_the_quote() {
        assert_eq!(
            parsed(r#""C:\Program Files\Acme\svc.exe" --run -v"#),
            (
                r"C:\Program Files\Acme\svc.exe".to_owned(),
                Some("--run -v".to_owned())
            )
        );
        assert_eq!(
            parsed(r#""C:\Program Files\Acme\svc.exe""#),
            path_only(r"C:\Program Files\Acme\svc.exe")
        );
    }

    #[test]
    fn an_unquoted_command_line_keeps_spaces_in_the_path_and_its_arguments() {
        assert_eq!(
            parsed(r"C:\Program Files\Acme\svc.exe --config C:\x.conf"),
            (
                r"C:\Program Files\Acme\svc.exe".to_owned(),
                Some(r"--config C:\x.conf".to_owned())
            )
        );
        assert_eq!(
            parsed(r"C:\Program Files\Vendor Name\flt.sys"),
            path_only(r"C:\Program Files\Vendor Name\flt.sys")
        );
    }

    #[test]
    fn a_name_that_merely_contains_an_extension_does_not_end_the_path() {
        assert_eq!(
            parsed(r"C:\Tools.exe.d\My App\svc.exe -x"),
            (
                r"C:\Tools.exe.d\My App\svc.exe".to_owned(),
                Some("-x".to_owned())
            )
        );
    }

    #[test]
    fn nt_paths_lose_their_prefix() {
        assert_eq!(
            parsed(r"\??\C:\Program Files\Vendor Name\flt.sys"),
            path_only(r"C:\Program Files\Vendor Name\flt.sys")
        );
        assert_eq!(parsed(r"\\?\C:\App\flt.sys"), path_only(r"C:\App\flt.sys"));
        assert_eq!(
            parsed(r"\DosDevices\D:\App\flt.sys"),
            path_only(r"D:\App\flt.sys")
        );
    }

    #[test]
    fn windows_folder_forms_resolve_against_the_scanned_system() {
        for raw in [
            r"\SystemRoot\System32\drivers\acme.sys",
            r"%SystemRoot%\System32\drivers\acme.sys",
            r"%windir%\System32\drivers\acme.sys",
            r"System32\drivers\acme.sys",
            r"system32\DRIVERS\..\drivers\acme.sys",
        ] {
            let image = parse_image_path(raw, Path::new(r"E:\Windows"));
            assert!(
                image.path.starts_with(r"E:\Windows"),
                "{raw} became {:?}",
                image.path
            );
            assert!(image.path.ends_with("acme.sys"), "{raw}");
        }
    }

    #[test]
    fn a_shared_host_keeps_its_group_as_arguments() {
        assert_eq!(
            parsed(r"%SystemRoot%\system32\svchost.exe -k netsvcs -p"),
            (
                r"C:\Windows\system32\svchost.exe".to_owned(),
                Some("-k netsvcs -p".to_owned())
            )
        );
    }

    #[test]
    fn empty_and_odd_values_do_not_panic() {
        assert_eq!(parsed(""), path_only(""));
        assert_eq!(parsed(r#"""#), path_only(""));
        assert_eq!(parsed(r#""C:\App\svc.exe"#), path_only(r"C:\App\svc.exe"));
        assert_eq!(parsed("é"), path_only(r"C:\Windows\é"));
        assert_eq!(
            parsed(r"C:\Ünï\svc.exe é"),
            (r"C:\Ünï\svc.exe".to_owned(), Some("é".to_owned()))
        );
    }
}
