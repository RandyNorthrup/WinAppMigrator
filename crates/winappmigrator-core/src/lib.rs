pub mod app_inventory;
pub mod archive;
pub mod dll_resolver;
pub mod env_scan;
pub mod errors;
pub mod file_discovery;
pub mod hashing;
pub mod install_source;
pub mod license_scan;
pub mod models;
pub mod package;
pub mod path_utils;
pub mod pe_scan;
pub mod prerequisites;
pub mod privilege;
pub mod progress;
pub mod registry_scan;
mod registry_target;
pub mod reporting;
pub mod restore;
pub mod scan;
pub mod service_scan;
pub mod shortcut_scan;
pub mod system_context;
pub mod task_scan;
pub mod verify;

pub use app_inventory::list_installed_apps_from;
pub use archive::{
    create_self_extracting_archive, extract_embedded_self_archive, has_embedded_self_archive,
};
pub use errors::{Result, WinAppMigratorError};
pub use models::*;
pub use package::{create_package, create_package_monitored, read_embedded_manifest};
pub use path_utils::hidden_command;
pub use progress::{BuildMonitor, BuildStep};
pub use restore::restore_package;
pub use system_context::{ScanSource, SystemContext};
pub use verify::verify_package;
