pub mod app_inventory;
pub mod archive;
pub mod authenticode;
pub mod dll_resolver;
pub mod env_scan;
pub mod errors;
pub mod file_discovery;
pub mod hashing;
pub mod install_source;
mod integration_scan;
pub mod journal;
pub mod license_scan;
pub mod logging;
pub mod models;
pub mod package;
pub mod path_rewrite;
pub mod path_utils;
pub mod pe_scan;
pub mod prerequisites;
pub mod privilege;
mod product_match;
pub mod profile_survey;
pub mod progress;
pub mod registry_scan;
mod registry_target;
pub mod reporting;
pub mod restore;
pub mod restore_policy;
pub mod restore_review;
pub mod rollback;
mod runtime_detection;
pub mod scan;
pub mod secret_rules;
pub mod service_scan;
pub mod shortcut_scan;
pub mod system_context;
pub mod task_scan;
pub mod verify;

pub use app_inventory::list_installed_apps_from;
pub use archive::{EmbeddedPackage, create_self_extracting_archive, has_embedded_self_archive};
pub use errors::{Result, WinAppMigratorError};
pub use models::*;
pub use package::{create_package, create_package_monitored, read_embedded_manifest};
pub use path_utils::hidden_command;
pub use profile_survey::{ProfilePicker, ProfileRow, ProfileSurvey, survey_of, survey_profiles};
pub use progress::{BuildMonitor, BuildStep, InstallMonitor, InstallPhase, InstallStep, Monitor};
pub use restore::{default_install_location, restore_package, restore_package_monitored};
pub use restore_review::{
    SystemChange, SystemChangeKind, SystemChanges, install_location_problem, system_changes,
    system_changes_restoring,
};
pub use system_context::{
    ProfileChoice, ScanSource, SystemContext, looks_like_a_windows_drive, sweep_leaked_hive_mounts,
};
pub use verify::verify_package;
