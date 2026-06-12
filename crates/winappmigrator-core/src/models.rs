use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const SCAN_SCHEMA_VERSION: &str = "winappmigrator.scan.v1";
pub const PACKAGE_SCHEMA_VERSION: &str = "winappmigrator.package.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AppArchitecture {
    X86,
    X64,
    Arm64,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryHive {
    LocalMachine,
    CurrentUser,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentScope {
    User,
    Machine,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallScope {
    PerUser,
    PerMachine,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub display_name: String,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub install_location: Option<PathBuf>,
    pub uninstall_string: Option<String>,
    pub main_executable: Option<PathBuf>,
    pub registry_source_path: String,
    pub architecture: AppArchitecture,
    pub install_scope: InstallScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum AppSelection {
    ExecutablePath(PathBuf),
    InstalledApp(Box<InstalledApp>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub selection: AppSelection,
    pub output_dir: PathBuf,
    pub current_directory: Option<PathBuf>,
    pub recursive_dependencies: bool,
    pub include_hashes: bool,
}

impl ScanOptions {
    pub fn for_executable(
        executable_path: impl Into<PathBuf>,
        output_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            selection: AppSelection::ExecutablePath(executable_path.into()),
            output_dir: output_dir.into(),
            current_directory: None,
            recursive_dependencies: true,
            include_hashes: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub schema_version: String,
    pub generated_at_utc: String,
    pub source_app: Option<InstalledApp>,
    pub main_executable: ExecutableMetadata,
    pub architecture: AppArchitecture,
    pub install_location: Option<PathBuf>,
    pub file_inventory: Vec<FileEntry>,
    pub dependencies: Vec<DependencyEntry>,
    pub app_local_dlls: Vec<FileEntry>,
    pub registry_entries: Vec<RegistryEntry>,
    pub services: Vec<ServiceEntry>,
    pub scheduled_tasks: Vec<ScheduledTaskEntry>,
    pub shortcuts: Vec<ShortcutEntry>,
    pub environment_variables: Vec<EnvironmentVariableEntry>,
    pub runtime_prerequisites: Vec<RuntimePrerequisite>,
    pub warnings: Vec<Warning>,
    pub items_excluded: Vec<String>,
    pub confidence_score: u8,
    pub generated_artifacts: Option<GeneratedArtifacts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedArtifacts {
    pub scan_report_json: PathBuf,
    pub scan_report_markdown: PathBuf,
    pub dependency_graph_json: PathBuf,
    pub warnings_markdown: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutableMetadata {
    pub path: PathBuf,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub modified_utc: Option<String>,
    pub version_info: Option<FileVersionInfo>,
    pub digital_signature: SignatureStatus,
    pub external_manifest: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileVersionInfo {
    pub file_version: Option<String>,
    pub product_version: Option<String>,
    pub company_name: Option<String>,
    pub product_name: Option<String>,
    pub file_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignatureStatus {
    NotPresent,
    EmbeddedSignaturePresent,
    NotChecked,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub schema_version: String,
    pub source_computer_name: Option<String>,
    pub source_windows_version: Option<String>,
    pub scan_timestamp_utc: String,
    pub main_executable_path: PathBuf,
    pub application_metadata: Option<InstalledApp>,
    pub file_inventory: Vec<FileEntry>,
    pub registry_keys_exported: Vec<RegistryEntry>,
    pub services_detected: Vec<ServiceEntry>,
    pub scheduled_tasks_detected: Vec<ScheduledTaskEntry>,
    pub shortcuts_detected: Vec<ShortcutEntry>,
    pub environment_variables_detected: Vec<EnvironmentVariableEntry>,
    pub runtime_prerequisites: Vec<RuntimePrerequisite>,
    pub warnings: Vec<Warning>,
    pub items_intentionally_excluded: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOptions {
    pub selection: AppSelection,
    pub output_dir: PathBuf,
    pub current_directory: Option<PathBuf>,
    pub archive_format: Option<PackageArchiveFormat>,
    pub create_self_extracting_archive: bool,
    pub self_extracting_stub: Option<PathBuf>,
}

impl PackageOptions {
    pub fn for_executable(
        executable_path: impl Into<PathBuf>,
        output_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            selection: AppSelection::ExecutablePath(executable_path.into()),
            output_dir: output_dir.into(),
            current_directory: None,
            archive_format: None,
            create_self_extracting_archive: false,
            self_extracting_stub: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PackageArchiveFormat {
    Zip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResult {
    pub package_root: PathBuf,
    pub manifest_path: PathBuf,
    pub reports_dir: PathBuf,
    pub files_copied: usize,
    pub warnings: Vec<Warning>,
    pub archive_path: Option<PathBuf>,
    pub self_extracting_archive_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub original_path: PathBuf,
    pub package_relative_path: Option<PathBuf>,
    pub suggested_destination_path: Option<PathBuf>,
    pub size_bytes: Option<u64>,
    pub hash_sha256: Option<String>,
    pub classification: FileClassification,
    pub should_copy: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileClassification {
    AppOwned,
    SharedRuntime,
    WindowsSystem,
    Missing,
    DoNotCopy,
    ManualRuntimeInstall,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEntry {
    pub name: String,
    pub resolved_path: Option<PathBuf>,
    pub architecture: AppArchitecture,
    pub hash_sha256: Option<String>,
    pub version_info: Option<FileVersionInfo>,
    pub publisher: Option<String>,
    pub source_category: DependencySourceCategory,
    pub should_copy: bool,
    pub system_provided: bool,
    pub runtime_prerequisite: bool,
    pub missing: bool,
    pub delay_loaded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DependencySourceCategory {
    KnownDll,
    ApiSet,
    AppLocal,
    System32,
    SysWow64,
    Windows,
    CurrentDirectory,
    Path,
    SideBySide,
    RuntimePrerequisite,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub hive: RegistryHive,
    pub key_path: String,
    pub value_name: Option<String>,
    pub value_data: Option<String>,
    pub value_kind: Option<RegistryValueKind>,
    pub value_bytes_hex: Option<String>,
    pub confidence: MatchConfidence,
    pub requires_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryValueKind {
    None,
    String,
    ExpandString,
    Binary,
    Dword,
    DwordBigEndian,
    Link,
    MultiString,
    ResourceList,
    FullResourceDescriptor,
    ResourceRequirementsList,
    Qword,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntry {
    pub service_name: String,
    pub display_name: Option<String>,
    pub binary_path: PathBuf,
    pub start_type: Option<String>,
    pub account_name: Option<String>,
    pub dependencies: Vec<String>,
    pub description: Option<String>,
    pub requires_admin: bool,
    pub restore_command_suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTaskEntry {
    pub task_name: String,
    pub task_path: String,
    pub action_command: PathBuf,
    pub arguments: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub trigger_summary: Option<String>,
    pub run_level: Option<String>,
    pub account: Option<String>,
    pub restore_instructions: String,
    pub definition_xml: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutEntry {
    pub original_lnk_path: PathBuf,
    pub target_path: PathBuf,
    pub arguments: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
    pub target_is_selected_app: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentVariableEntry {
    pub scope: EnvironmentScope,
    pub name: String,
    pub value: String,
    pub value_kind: RegistryValueKind,
    pub requires_admin: bool,
    pub confidence: MatchConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePrerequisite {
    pub name: String,
    pub kind: RuntimePrerequisiteKind,
    pub evidence: String,
    pub install_manually: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimePrerequisiteKind {
    VisualCppRedistributable,
    DotNetDesktopRuntime,
    JavaRuntime,
    PythonRuntime,
    ElectronNodeRuntime,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub code: String,
    pub severity: WarningSeverity,
    pub message: String,
    pub source: Option<String>,
}

impl Warning {
    pub fn new(
        code: impl Into<String>,
        severity: WarningSeverity,
        message: impl Into<String>,
        source: Option<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePlan {
    pub package_root: PathBuf,
    pub target_root: PathBuf,
    pub dry_run: bool,
    pub planned_file_writes: Vec<FileEntry>,
    pub planned_registry_writes: Vec<RegistryEntry>,
    pub planned_service_changes: Vec<ServiceEntry>,
    pub planned_task_changes: Vec<ScheduledTaskEntry>,
    pub planned_shortcut_changes: Vec<ShortcutEntry>,
    pub planned_environment_changes: Vec<EnvironmentVariableEntry>,
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    pub applied: bool,
    pub changes_logged: Vec<String>,
    pub warnings: Vec<Warning>,
    pub errors: Vec<Warning>,
    pub restore_log_json: Option<PathBuf>,
    pub restore_log_markdown: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOptions {
    pub package_root: PathBuf,
    pub target_root: PathBuf,
    pub dry_run: bool,
    pub apply: bool,
    pub force: bool,
}
