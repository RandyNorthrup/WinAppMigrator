use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const SCAN_SCHEMA_VERSION: &str = "winappmigrator.scan.v1";
pub const PACKAGE_SCHEMA_VERSION: &str = "winappmigrator.package.v1";
pub const VERIFICATION_SCHEMA_VERSION: &str = "winappmigrator.verification.v1";
pub const INVENTORY_SNAPSHOT_SCHEMA_VERSION: &str = "winappmigrator.inventory_snapshot.v1";

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

/// How an application was installed. Determines which authoritative source can
/// be consulted to reverse-engineer its footprint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallMethod {
    /// Installed by Windows Installer (MSI); its components are recorded in the
    /// Installer database and can be enumerated authoritatively.
    WindowsInstaller,
    /// MSIX / Appx packaged application.
    Msix,
    /// Squirrel / Electron per-user installer (Discord, Slack, etc.).
    Squirrel,
    /// Installed through the Chocolatey package manager.
    Chocolatey,
    /// Inno Setup installer (keeps an authoritative `unins*.dat` uninstall log).
    InnoSetup,
    /// NSIS (Nullsoft) installer.
    Nsis,
    /// Any other installer (portable, custom, unknown).
    Generic,
}

impl InstallMethod {
    pub fn label(&self) -> &'static str {
        match self {
            InstallMethod::WindowsInstaller => "Windows Installer (MSI)",
            InstallMethod::Msix => "MSIX / Appx package",
            InstallMethod::Squirrel => "Squirrel (Electron)",
            InstallMethod::Chocolatey => "Chocolatey package",
            InstallMethod::InnoSetup => "Inno Setup installer",
            InstallMethod::Nsis => "NSIS installer",
            InstallMethod::Generic => "Generic installer",
        }
    }
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
pub struct InventorySnapshotOptions {
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySnapshotReport {
    pub schema_version: String,
    pub generated_at_utc: String,
    pub applications: Vec<InstalledApp>,
    pub warnings: Vec<Warning>,
    pub artifacts: InventorySnapshotArtifacts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySnapshotArtifacts {
    pub json: PathBuf,
    pub markdown: PathBuf,
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
    #[serde(default = "default_true")]
    pub include_user_data: bool,
}

fn default_true() -> bool {
    true
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
            include_user_data: true,
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
    #[serde(default)]
    pub install_method: Option<InstallMethod>,
    #[serde(default)]
    pub authoritative_file_count: usize,
    #[serde(default)]
    pub install_evidence: Vec<String>,
    #[serde(default)]
    pub footprint_locations: Vec<FootprintLocation>,
    pub file_inventory: Vec<FileEntry>,
    pub dependencies: Vec<DependencyEntry>,
    pub app_local_dlls: Vec<FileEntry>,
    pub registry_entries: Vec<RegistryEntry>,
    #[serde(default)]
    pub com_registrations: Vec<ComRegistrationEntry>,
    #[serde(default)]
    pub file_associations: Vec<FileAssociationEntry>,
    pub services: Vec<ServiceEntry>,
    #[serde(default)]
    pub drivers: Vec<DriverEntry>,
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
    /// True when the PE carries a CLR/COR20 data directory (a managed .NET
    /// image). Managed assemblies resolve their dependencies through the CLR
    /// rather than the PE import table, so this flags additional manual review.
    #[serde(default)]
    pub is_dotnet: bool,
    /// True when an application manifest is embedded as an `RT_MANIFEST`
    /// resource (distinct from an external `<exe>.manifest` file).
    #[serde(default)]
    pub embedded_manifest: bool,
    /// The `requestedExecutionLevel` from the embedded/external manifest
    /// (`asInvoker`, `highestAvailable`, `requireAdministrator`), when present.
    #[serde(default)]
    pub requested_execution_level: Option<String>,
    /// Side-by-side assembly identities the manifest declares as dependencies
    /// (for example `Microsoft.Windows.Common-Controls`). These are serviced by
    /// WinSxS and are reported for manual review, not copied.
    #[serde(default)]
    pub sxs_dependencies: Vec<String>,
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
    #[serde(default)]
    pub com_registrations_detected: Vec<ComRegistrationEntry>,
    #[serde(default)]
    pub file_associations_detected: Vec<FileAssociationEntry>,
    pub services_detected: Vec<ServiceEntry>,
    #[serde(default)]
    pub drivers_detected: Vec<DriverEntry>,
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
    #[serde(default = "default_true")]
    pub include_user_data: bool,
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
            include_user_data: true,
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
    UserData,
    SharedRuntime,
    WindowsSystem,
    Missing,
    DoNotCopy,
    ManualRuntimeInstall,
    Unknown,
}

/// A distinct location on disk that belongs to an application's footprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintLocation {
    pub path: PathBuf,
    pub kind: FootprintKind,
    pub scope: InstallScope,
    pub file_count: usize,
    pub total_bytes: u64,
    pub copyable_bytes: u64,
    pub included: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FootprintKind {
    InstallDirectory,
    ProgramData,
    RoamingAppData,
    LocalAppData,
    LocalLowAppData,
    RegistryReference,
    Other,
}

impl FootprintKind {
    pub fn label(&self) -> &'static str {
        match self {
            FootprintKind::InstallDirectory => "Install directory",
            FootprintKind::ProgramData => "ProgramData",
            FootprintKind::RoamingAppData => "Roaming AppData",
            FootprintKind::LocalAppData => "Local AppData",
            FootprintKind::LocalLowAppData => "LocalLow AppData",
            FootprintKind::RegistryReference => "Referenced location",
            FootprintKind::Other => "Other location",
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComRegistrationEntry {
    pub hive: RegistryHive,
    pub clsid: String,
    pub key_path: String,
    pub server_kind: ComServerKind,
    pub server_path: Option<PathBuf>,
    pub threading_model: Option<String>,
    pub confidence: MatchConfidence,
    pub requires_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComServerKind {
    InProcess,
    LocalServer,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAssociationEntry {
    pub hive: RegistryHive,
    pub extension: Option<String>,
    pub prog_id: Option<String>,
    pub application_name: Option<String>,
    pub verb: Option<String>,
    pub command: Option<String>,
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
pub struct DriverEntry {
    pub service_name: String,
    pub display_name: Option<String>,
    pub image_path: PathBuf,
    pub driver_kind: DriverKind,
    pub start_type: Option<String>,
    pub account_name: Option<String>,
    pub description: Option<String>,
    pub requires_admin: bool,
    pub restore_instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DriverKind {
    Kernel,
    FileSystem,
    Recognizer,
    Unknown,
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
    #[serde(default)]
    pub requires_admin: bool,
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
    pub planned_driver_actions: Vec<DriverEntry>,
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
    #[serde(default)]
    pub verification: Option<VerificationReport>,
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
    /// When true, packaged per-user/ProgramData application data is restored to
    /// the target machine's real `%AppData%`/`%ProgramData%` locations instead
    /// of being staged under `target_root`. Defaults to false to keep the
    /// conservative sandbox behavior; writing into ProgramData requires
    /// elevation. App-owned install-directory files always land under
    /// `target_root` regardless of this flag.
    #[serde(default)]
    pub restore_user_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationOptions {
    pub package_root: PathBuf,
    pub target_root: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    /// Must match the `restore_user_data` value used during restore so the
    /// verifier checks the same destinations the restore wrote to.
    #[serde(default)]
    pub restore_user_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub schema_version: String,
    pub verified_at_utc: String,
    pub package_root: PathBuf,
    pub target_root: Option<PathBuf>,
    pub package_valid: bool,
    pub restore_valid: Option<bool>,
    pub items: Vec<VerificationItem>,
    pub warnings: Vec<Warning>,
    pub errors: Vec<Warning>,
    pub report_json: Option<PathBuf>,
    pub report_markdown: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationItem {
    pub category: VerificationCategory,
    pub name: String,
    pub status: VerificationStatus,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationCategory {
    Manifest,
    PackageFile,
    RestoredFile,
    Registry,
    Environment,
    Service,
    Driver,
    ScheduledTask,
    Shortcut,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Passed,
    Warning,
    Failed,
    Skipped,
}
