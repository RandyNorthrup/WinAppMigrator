mod account_settings;
pub mod account_traces;
pub mod app_inventory;
pub mod archive;
pub mod authenticode;
pub mod disk_image;
pub mod dll_resolver;
pub mod env_scan;
pub mod errors;
pub mod file_discovery;
pub mod footprint;
pub mod hashing;
pub mod install_source;
mod integration_scan;
pub mod journal;
pub mod license_scan;
pub mod logging;
pub mod models;
pub mod near_match;
pub mod package;
mod packer;
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
pub mod space;
pub mod system_context;
pub mod task_scan;
pub mod verify;
mod zip_stream;

pub use app_inventory::list_installed_apps_from;
pub use archive::{
    DATA_FILE_EXTENSION, EmbeddedPackage, create_installer_with_data_file,
    create_self_extracting_archive, data_file_beside, has_embedded_self_archive,
};
pub use errors::{Result, WinAppMigratorError, panic_text};
pub use footprint::{
    AlsoLeftOut, ChoiceId, ChosenScan, FolderGroup, FolderRow, Footprint, FootprintChoices,
    IntegrationRow, ItemRow, LICENSES_LEFT_OUT_TEXT, LeftOutGroup, PrerequisiteRow, ScanFlags,
    WhatGoes, apply_choices, what_goes_with,
};
pub use models::*;
pub use near_match::{NearMatchLook, RefusedNearMatch, near_matches_of};
pub use package::{
    create_package, create_package_from_scan, create_package_monitored, read_embedded_manifest,
    scan_options_for,
};
pub use path_utils::hidden_command;
pub use profile_survey::{ProfilePicker, ProfileRow, ProfileSurvey, survey_of, survey_profiles};
pub use progress::{
    BuildMonitor, BuildStep, InstallMonitor, InstallPhase, InstallStep, Monitor, ScanMonitor,
    ScanStep,
};
pub use restore::{default_install_location, restore_package, restore_package_monitored};
pub use restore_review::{
    CERTIFICATES_LEFT_OUT_TEXT, CREDENTIALS_LEFT_OUT_TEXT, FONTS_LEFT_OUT_TEXT,
    SETTINGS_LEFT_OUT_TEXT, SystemChange, SystemChangeKind, SystemChanges,
    files_no_install_put_there, folder_has_files, install_location_problem, install_places,
    location_a_list_is_for, refuse_what_cannot_run_here, refuse_without_room_to_install,
    system_changes, system_changes_restoring,
};
pub use scan::scan_app;
pub use space::{
    BuildInputs, RoomCheck, VolumeStats, preview_needs, room_before_a_build, size_text,
    volume_stats_at,
};
pub use system_context::{
    ProfileChoice, ScanSource, SourceIdentity, SystemContext, looks_like_a_windows_drive,
    sweep_leaked_hive_mounts,
};
pub use verify::{verify_installer, verify_package, verify_unpacked_package};
