use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

use crate::models::{DriverEntry, DriverKind, ServiceEntry, Warning, WarningSeverity};
use crate::path_utils::split_command_executable_path;

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

        drivers.push(DriverEntry {
            service_name: service_name.clone(),
            display_name: service_key.get_value::<String, _>("DisplayName").ok(),
            image_path: binary_path,
            driver_kind: driver_kind(service_type),
            start_type: service_key
                .get_value::<u32, _>("Start")
                .ok()
                .map(start_type_label),
            account_name: service_key.get_value::<String, _>("ObjectName").ok(),
            description: service_key.get_value::<String, _>("Description").ok(),
            requires_admin: true,
            restore_instructions:
                "Kernel and driver components are reported only. Reinstall with the vendor driver package or signed installer; WinAppMigrator does not copy or register drivers automatically."
                    .to_owned(),
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
