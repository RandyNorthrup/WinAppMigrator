use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

use crate::models::{DriverEntry, DriverKind, ServiceEntry, Warning, WarningSeverity};
use crate::path_utils::{hidden_command, split_command_executable_path};

const POWERSHELL_EXE: &str = "powershell.exe";

const SERVICES_KEY: &str = r"SYSTEM\CurrentControlSet\Services";
const SERVICE_KERNEL_DRIVER: u32 = 0x0000_0001;
const SERVICE_FILE_SYSTEM_DRIVER: u32 = 0x0000_0002;
const SERVICE_RECOGNIZER_DRIVER: u32 = 0x0000_0008;

#[derive(Debug, Clone)]
pub struct ServiceScanContext {
    pub executable_path: PathBuf,
    pub install_location: Option<PathBuf>,
}

pub fn scan_services(context: &ServiceScanContext) -> (Vec<ServiceEntry>, Vec<Warning>) {
    let mut services = Vec::new();
    let mut warnings = Vec::new();
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services_key = match root.open_subkey_with_flags(SERVICES_KEY, KEY_READ) {
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
        let image_path = expand_image_path(&image_path);
        let binary_path = split_command_executable_path(&image_path)
            .unwrap_or_else(|| PathBuf::from(image_path.trim()));
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

        services.push(ServiceEntry {
            service_name: service_name.clone(),
            display_name,
            binary_path: binary_path.clone(),
            start_type,
            account_name,
            dependencies,
            description,
            requires_admin: true,
            restore_command_suggestion: format!(
                "sc.exe create \"{}\" binPath= \"{}\"",
                service_name,
                binary_path.display()
            ),
        });
    }

    services.sort_by(|left, right| left.service_name.cmp(&right.service_name));
    (services, warnings)
}

pub fn scan_drivers(context: &ServiceScanContext) -> (Vec<DriverEntry>, Vec<Warning>) {
    let mut drivers = Vec::new();
    let mut warnings = Vec::new();
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services_key = match root.open_subkey_with_flags(SERVICES_KEY, KEY_READ) {
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
        let image_path = expand_image_path(&image_path);
        let binary_path = split_command_executable_path(&image_path)
            .unwrap_or_else(|| PathBuf::from(image_path.trim()));
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
            .is_some_and(|root| path_starts_with_ignore_case(candidate, root))
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
        .is_some_and(|root| path_starts_with_ignore_case(image_path, root));
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
        image_path.display().to_string().replace('\'', "''")
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
            "Application-owned {} driver '{}' ({}{}). The installer bundles its .sys and re-registers the driver service automatically (sc create, requires elevation), then verifies it loads.",
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

fn path_starts_with_ignore_case(path: &Path, root: &Path) -> bool {
    path.to_string_lossy()
        .to_ascii_lowercase()
        .starts_with(&root.to_string_lossy().to_ascii_lowercase())
}

fn expand_image_path(value: &str) -> String {
    let mut expanded = value.to_owned();
    for (key, val) in std::env::vars() {
        let marker = format!("%{key}%");
        expanded = expanded.replace(&marker, &val);
    }
    if let Some(stripped) = expanded.strip_prefix(r"\SystemRoot\")
        && let Some(system_root) = std::env::var_os("SystemRoot")
    {
        return PathBuf::from(system_root)
            .join(stripped)
            .display()
            .to_string();
    }
    expanded
}
