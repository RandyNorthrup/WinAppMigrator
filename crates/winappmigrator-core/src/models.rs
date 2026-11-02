use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::system_context::ScanSource;

pub const SCAN_SCHEMA_VERSION: &str = "winappmigrator.scan.v1";
pub const PACKAGE_SCHEMA_VERSION: &str = "winappmigrator.package.v1";
pub const VERIFICATION_SCHEMA_VERSION: &str = "winappmigrator.verification.v1";

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
    /// Include genuine per-user data (Roaming/Local/LocalLow, Documents, Saved
    /// Games). Machine-wide `ProgramData` is controlled separately by
    /// `include_program_data`.
    #[serde(default = "default_true")]
    pub include_user_data: bool,
    /// Include machine-wide `ProgramData` application data.
    #[serde(default = "default_true")]
    pub include_program_data: bool,
    /// Detect and (when packaging) copy the application's own licensing data.
    /// Defaults to false: license data is sensitive and may be machine-bound,
    /// so it is opt-in.
    #[serde(default)]
    pub include_license_data: bool,
    /// Where to read the application from: the live OS (default) or a connected
    /// offline volume (a disk from another PC).
    #[serde(default)]
    pub source: ScanSource,
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
            include_program_data: true,
            include_license_data: false,
            source: ScanSource::Live,
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
    #[serde(default)]
    pub license_artifacts: Vec<LicenseArtifact>,
    /// Transferable license values captured from the registry (non-machine-bound
    /// only) so the installer can restore them automatically.
    #[serde(default)]
    pub license_registry_entries: Vec<RegistryEntry>,
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
    /// The CLR metadata-root version string of a managed image (for example
    /// `v2.0.50727` for .NET Framework 2.0/3.5, `v4.0.30319` for 4.x and modern
    /// .NET). Used to pick the correct .NET prerequisite. `None` when not managed
    /// or the metadata root could not be read.
    #[serde(default)]
    pub dotnet_runtime_version: Option<String>,
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
    #[serde(default)]
    pub license_artifacts_detected: Vec<LicenseArtifact>,
    /// Transferable license values to restore into the registry on the target.
    #[serde(default)]
    pub license_registry_entries: Vec<RegistryEntry>,
    pub warnings: Vec<Warning>,
    pub items_intentionally_excluded: Vec<String>,
    /// Whether per-user application data was captured at build time. Drives the
    /// installer's "Restore user data" checkbox: it is only shown when true.
    #[serde(default)]
    pub included_user_data: bool,
    /// Whether machine-wide ProgramData was captured at build time. Also gates
    /// the installer's "Restore user data" checkbox.
    #[serde(default)]
    pub included_program_data: bool,
    /// Whether license artifacts were captured at build time. Drives the
    /// installer's "Restore license" checkbox: it is only shown when true.
    #[serde(default)]
    pub included_license_data: bool,
    /// The UI theme tag ("light"/"dark") active in the app when the installer was
    /// built, so the produced installer can present in the same theme. `None` on
    /// older packages, which the installer treats as the default (dark).
    #[serde(default)]
    pub ui_theme: Option<String>,
}

/// The installer stub an installer is built from.
#[derive(Debug, Clone)]
pub enum InstallerStub {
    /// A stub on disk (a development build, or the end-to-end test).
    File(PathBuf),
    /// The stub compiled into the app. Written straight to the installer, so it
    /// never passes through a user-writable folder where it could be swapped.
    Embedded(&'static [u8]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOptions {
    pub selection: AppSelection,
    pub output_dir: PathBuf,
    pub current_directory: Option<PathBuf>,
    pub archive_format: Option<PackageArchiveFormat>,
    pub create_self_extracting_archive: bool,
    #[serde(skip)]
    pub self_extracting_stub: Option<InstallerStub>,
    /// Directory the single self-extracting installer executable is written to.
    /// Defaults to the parent of `output_dir` when `None`. The installer's file
    /// name encodes the chosen contents (e.g. `App_userdata_license_installer.exe`).
    #[serde(default)]
    pub installer_dir: Option<PathBuf>,
    #[serde(default = "default_true")]
    pub include_user_data: bool,
    #[serde(default = "default_true")]
    pub include_program_data: bool,
    #[serde(default)]
    pub include_license_data: bool,
    /// Download detected runtime prerequisites (VC++ redistributable, etc.) and
    /// embed them in the package so the installer can install them offline on the
    /// target. Requires network access at package time; failures degrade to a
    /// documented winget/URL fallback recorded in the manifest.
    #[serde(default)]
    pub bundle_prerequisites: bool,
    /// Where to read the application from: the live OS (default) or a connected
    /// offline volume (a disk from another PC).
    #[serde(default)]
    pub source: ScanSource,
    /// UI theme tag ("light"/"dark") to record in the package so the produced
    /// installer presents in the theme the app used when building it.
    #[serde(default)]
    pub ui_theme: Option<String>,
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
            installer_dir: None,
            include_user_data: true,
            include_program_data: true,
            include_license_data: false,
            bundle_prerequisites: false,
            source: ScanSource::Live,
            ui_theme: None,
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
    /// Per-user application data (Roaming/Local/LocalLow settings, Documents,
    /// Saved Games). Copied when user-data backup is enabled.
    UserData,
    /// Machine-wide application data under `ProgramData`. Shared by all users
    /// and distinct from per-user data; copied when ProgramData backup is
    /// enabled.
    MachineAppData,
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
    Documents,
    SavedGames,
    /// Shared `C:\Users\Public\Documents` — machine-wide data some apps keep in
    /// the Public profile rather than ProgramData (e.g. YDIS's database).
    PublicDocuments,
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
            FootprintKind::Documents => "Documents",
            FootprintKind::SavedGames => "Saved Games",
            FootprintKind::PublicDocuments => "Public Documents",
            FootprintKind::RegistryReference => "Referenced location",
            FootprintKind::Other => "Other location",
        }
    }

    /// Whether this location holds genuine per-user data (as opposed to the
    /// machine-wide `ProgramData`, the install directory, or a bare reference).
    pub fn is_user_data(&self) -> bool {
        matches!(
            self,
            FootprintKind::RoamingAppData
                | FootprintKind::LocalAppData
                | FootprintKind::LocalLowAppData
                | FootprintKind::Documents
                | FootprintKind::SavedGames
        )
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
    /// What followed the executable in the service's `ImagePath`.
    #[serde(default)]
    pub arguments: Option<String>,
    /// The values under the service's `Parameters` key, which many services read their
    /// settings from. Written back only when the service itself is restored.
    #[serde(default)]
    pub parameters: Vec<RegistryEntry>,
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
    /// True when the driver image is an application-owned `.sys` (under the app's
    /// install footprint, not a Windows inbox/System32 driver). Only app-owned
    /// drivers are bundled and re-registered automatically.
    #[serde(default)]
    pub app_owned: bool,
    /// Authenticode signer subject of the driver image, when determinable.
    #[serde(default)]
    pub signer: Option<String>,
    #[serde(default)]
    pub hash_sha256: Option<String>,
    /// Path inside the package (e.g. `drivers/acmeflt.sys`) when the driver image
    /// was bundled for automatic re-registration.
    #[serde(default)]
    pub package_relative_path: Option<PathBuf>,
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

/// A licensing/activation artifact discovered for an application. Captured to
/// help an owner migrate their own license, with explicit transferability
/// warnings. WinAppMigrator never decrypts, bypasses activation, or emulates
/// dongles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseArtifact {
    pub kind: LicenseArtifactKind,
    pub framework: LicenseFramework,
    /// Filesystem path for file/framework-store artifacts.
    pub path: Option<PathBuf>,
    /// Registry location (`HIVE\Key\Value`) for registry-based artifacts.
    pub registry_location: Option<String>,
    /// Human-readable evidence for why this was flagged as licensing data.
    pub detail: String,
    /// True when the license is bound to this machine/hardware/user (hardware
    /// dongle, machine-locked activation, DPAPI-encrypted) and therefore is
    /// unlikely to work after migration without reactivation.
    pub machine_bound: bool,
    pub requires_admin: bool,
    pub transferable: TransferableHint,
    /// Whether this artifact is copied into the package (file/framework-store
    /// artifacts only, when license backup is enabled).
    pub should_copy: bool,
    pub package_relative_path: Option<PathBuf>,
    pub size_bytes: Option<u64>,
    pub hash_sha256: Option<String>,
    /// For a framework-store folder, each file copied from it. A folder has no hash of
    /// its own, so these are what gets checked.
    #[serde(default)]
    pub store_files: Vec<LicenseStoreFile>,
}

/// One file of a license store folder.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LicenseStoreFile {
    /// Relative to the store's folder.
    pub relative_path: PathBuf,
    pub size_bytes: u64,
    pub hash_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseArtifactKind {
    File,
    FrameworkStore,
    RegistryValue,
    EnvironmentVariable,
    HardwareDongle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseFramework {
    FlexLm,
    SentinelHaspLdk,
    CodeMeter,
    RepriseRlm,
    FileBased,
    RegistryBased,
    EnvironmentVariable,
    HardwareDongle,
    Unknown,
}

impl LicenseFramework {
    pub fn label(&self) -> &'static str {
        match self {
            LicenseFramework::FlexLm => "FlexLM / FlexNet",
            LicenseFramework::SentinelHaspLdk => "Sentinel HASP / LDK",
            LicenseFramework::CodeMeter => "Wibu CodeMeter",
            LicenseFramework::RepriseRlm => "Reprise RLM",
            LicenseFramework::FileBased => "File-based license",
            LicenseFramework::RegistryBased => "Registry-based license",
            LicenseFramework::EnvironmentVariable => "License environment variable",
            LicenseFramework::HardwareDongle => "Hardware dongle",
            LicenseFramework::Unknown => "Unknown licensing",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferableHint {
    /// Plain license file with no obvious machine binding; may transfer.
    Likely,
    /// Hardware/machine-locked or encrypted; reactivation likely required.
    Unlikely,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePrerequisite {
    pub name: String,
    pub kind: RuntimePrerequisiteKind,
    pub evidence: String,
    /// True when the runtime must be installed on the target system (it does not
    /// ship app-local with the application). Kept for back-compat; the installer
    /// now acts on `bundle` instead of leaving this to the user.
    pub install_manually: bool,
    /// Whether the runtime was already present on the source machine at scan
    /// time. Informational; `None` when not probed.
    #[serde(default)]
    pub present_on_source: Option<bool>,
    /// Concrete plan for making the runtime available on the target: a bundled
    /// installer payload, or a documented winget/URL fallback. `None` when
    /// WinAppMigrator has no installer for this runtime (e.g. an app-local
    /// runtime that needs no separate install).
    #[serde(default)]
    pub bundle: Option<PrerequisiteBundle>,
    /// Registry probe the installer uses to confirm the runtime is present,
    /// before and after installing it. `None` when no reliable probe is known.
    #[serde(default)]
    pub presence_probe: Option<PrerequisiteRegistryProbe>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimePrerequisiteKind {
    VisualCppRedistributable,
    /// Modern .NET (Core) desktop runtime, acquired via winget/download.
    DotNetDesktopRuntime,
    /// Classic .NET Framework 2.0/3.0/3.5 (CLR v2.0.50727). Installed on the
    /// target by enabling the `NetFx3` Windows optional feature.
    DotNetFramework35,
    /// Classic .NET Framework 4.x (CLR v4.0.30319). Ships with Windows 10/11, so
    /// it is listed for transparency but needs no install on modern targets.
    DotNetFramework4x,
    JavaRuntime,
    PythonRuntime,
    ElectronNodeRuntime,
    Unknown,
}

/// How the installer makes a runtime prerequisite available on the target.
///
/// A package is not authenticated, and this runs elevated, so it does not say what to run
/// or how. The URL, arguments, winget id, Windows feature, and registry probe all come from
/// the compiled spec for the prerequisite's kind (see `prerequisites::spec`). All this adds
/// is which build of it, and the payload that was bundled, if one was.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrerequisiteBundle {
    /// Which build of the runtime the app needs, which picks among the compiled specs.
    #[serde(default = "default_prerequisite_architecture")]
    pub architecture: AppArchitecture,
    /// Path inside the package (e.g. `prerequisites/vc_redist.x64.exe`) when the
    /// installer payload was downloaded and embedded. `None` means the payload
    /// could not be bundled and the target must acquire it another way.
    pub package_relative_path: Option<PathBuf>,
    pub hash_sha256: Option<String>,
    pub size_bytes: Option<u64>,
}

fn default_prerequisite_architecture() -> AppArchitecture {
    AppArchitecture::Unknown
}

/// A registry presence check for a runtime prerequisite. The runtime is
/// considered present when any candidate key holds `value_name` as a DWORD that
/// is `>= min_value`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrerequisiteRegistryProbe {
    pub key_paths: Vec<String>,
    pub value_name: String,
    pub min_value: u32,
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
pub struct RestoreResult {
    pub applied: bool,
    /// What was done, or for a preview what would be, with what reaches beyond files
    /// first, so the list leads with what matters however many files the app has.
    pub changes_logged: Vec<String>,
    pub warnings: Vec<Warning>,
    pub errors: Vec<Warning>,
    /// What the package asked for and the restore rules did not allow. Nothing was done
    /// for these, and it does not make the install a failure: a hostile package loses
    /// only what it should not have asked for.
    #[serde(default)]
    pub refused: Vec<Warning>,
    #[serde(default)]
    pub verification: Option<VerificationReport>,
    /// The install's id, once it has finished and can be uninstalled.
    #[serde(default)]
    pub install_id: Option<String>,
    /// What was put back after an install that failed or was cancelled.
    #[serde(default)]
    pub rollback: Option<UndoReport>,
}

/// What came of undoing an install. Paths and names only, like the log it goes in.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoReport {
    /// What was removed or put back.
    pub undone: Vec<String>,
    /// What was left on purpose: it has changed since, or belongs to the user.
    pub kept: Vec<String>,
    /// What could not be undone, and why.
    pub failed: Vec<String>,
    /// What cannot be undone at all: runtimes that were installed.
    pub left_behind: Vec<String>,
    /// The folders that hold what is on a drive that is not connected, and can be undone when
    /// it is.
    #[serde(default)]
    pub drives_waited_for: Vec<PathBuf>,
}

impl UndoReport {
    /// Part of it could not be looked at, because its drive is not connected.
    pub fn waits_for_a_drive(&self) -> bool {
        !self.drives_waited_for.is_empty()
    }
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
    /// When true, captured license artifacts and transferable license registry
    /// values are restored to the target. Gated because a license is only
    /// meaningful when it was captured at build time and the user opted in.
    /// Defaults to true so existing callers keep restoring license data.
    #[serde(default = "default_true")]
    pub restore_license: bool,
    /// Where installs keep their records. On a real PC this is only ever
    /// `journal::machine_state_root`, and it is not something a caller of the installer can
    /// choose: it is here so that a test can use a temp folder.
    pub state_root: PathBuf,
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
    /// Must match the `restore_license` value used during restore so the
    /// verifier does not expect license files that were intentionally skipped.
    #[serde(default = "default_true")]
    pub restore_license: bool,
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
    Prerequisite,
    License,
    /// Something the package asked for that the restore rules did not allow.
    Refused,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Passed,
    Warning,
    Failed,
    Skipped,
}
