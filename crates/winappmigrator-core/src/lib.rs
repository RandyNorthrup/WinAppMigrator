pub mod app_inventory;
pub mod archive;
pub mod dll_resolver;
pub mod env_scan;
pub mod errors;
pub mod file_discovery;
pub mod hashing;
pub mod models;
pub mod package;
pub mod path_utils;
pub mod pe_scan;
pub mod privilege;
pub mod registry_scan;
pub mod reporting;
pub mod restore;
pub mod scan;
pub mod service_scan;
pub mod shortcut_scan;
pub mod task_scan;

pub use app_inventory::list_installed_apps;
pub use archive::{
    create_self_extracting_archive, create_zip_archive, extract_embedded_self_archive,
    extract_zip_archive, has_embedded_self_archive,
};
pub use errors::{Result, WinAppMigratorError};
pub use models::*;
pub use package::create_package;
pub use restore::{plan_restore, restore_package};
pub use scan::scan_and_write_reports;
