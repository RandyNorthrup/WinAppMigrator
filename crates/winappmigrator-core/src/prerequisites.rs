//! Runtime prerequisite acquisition, installation, and presence probing.
//!
//! WinAppMigrator detects the redistributable runtimes an application needs
//! (Visual C++ runtime, .NET, …) and makes a concrete plan to put them on the
//! target machine. For redistributables it can bundle, it downloads the official
//! installer at package time and embeds it; the installer then installs it
//! silently and offline on the target and confirms the result with a registry
//! probe. When a redistributable cannot be bundled it falls back to a documented
//! winget/URL install on the target. Every step is best-effort and reported.
//!
//! Everything that decides what runs elevated is compiled in here and looked up by the
//! prerequisite's kind. The manifest can say which kind, which build, and where in the
//! package the payload is. A payload has to carry a valid Microsoft signature both when it
//! is downloaded and just before it runs, or it does not run.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::authenticode::check_microsoft_signature;
use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::package_destination_path;
use crate::models::{
    AppArchitecture, DotNetFlavor, PrerequisiteBundle, PrerequisiteRegistryProbe,
    RuntimePrerequisiteKind,
};
use crate::path_utils::{POWERSHELL_IMPORT_APPX, POWERSHELL_IMPORT_UTILITY, hidden_command};

const WINGET_EXE: &str = "winget.exe";
const WINGET_NOT_INSTALLED: &str = "winget is not installed";
const DISM_EXE: &str = "dism";
const POWERSHELL_EXE: &str = "powershell.exe";
/// Where the package keeps prerequisite payloads, and the only place one can be run from.
pub const PREREQUISITES_DIR: &str = "prerequisites";
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(180);
/// A Microsoft download goes through a short link, and sometimes a mirror.
const MAX_DOWNLOAD_REDIRECTS: u32 = 5;
/// Far more than any redistributable, and small enough that a bad server cannot fill a disk.
const MAX_DOWNLOAD_BYTES: u64 = 1024 * 1024 * 1024;
const HTTPS_PREFIX: &str = "https://";
/// Asks Windows where winget is installed. That location is only writable by Windows,
/// unlike the folder on `PATH` holding the shortcut to it, where anyone can plant a file.
const WINGET_LOCATION_SCRIPT: &str = "(Get-AppxPackage -Name Microsoft.DesktopAppInstaller | \
     Sort-Object Version -Descending | Select-Object -First 1).InstallLocation";
/// Hard cap on a single prerequisite install so a hung or interactive installer
/// cannot wedge the whole restore. Large runtimes (.NET, winget) can be slow, so
/// this is generous.
const INSTALL_TIMEOUT: Duration = Duration::from_secs(900);
const WAIT_POLL_INTERVAL: Duration = Duration::from_millis(250);

/// A statically-known redistributable WinAppMigrator can acquire and install.
pub struct PrerequisiteSpec {
    /// True when the official installer can be downloaded and embedded at package
    /// time. When false, the target acquires it via `winget_id`/`download_url`.
    pub bundleable: bool,
    /// File name for the embedded payload (e.g. `vc_redist.x64.exe`).
    pub file_name: String,
    pub download_url: String,
    pub silent_install_args: Vec<String>,
    pub success_exit_codes: Vec<i32>,
    pub reboot_exit_codes: Vec<i32>,
    pub winget_id: Option<String>,
    /// Windows optional feature to enable via DISM (e.g. `NetFx3`) instead of
    /// running a payload/winget. `None` for ordinary redistributables.
    pub windows_feature: Option<String>,
    pub requires_admin: bool,
    pub probe: Option<PrerequisiteRegistryProbe>,
}

/// Result of attempting to install a prerequisite on the target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrerequisiteInstallOutcome {
    Installed,
    InstalledNeedsReboot,
    Skipped(String),
    Failed(String),
}

/// The versions of .NET there is an installer for. Others are named and left to whoever is
/// installing, rather than given one of these, which an app built for another would not run on.
const DOTNET_MAJOR_VERSIONS: [u32; 5] = [6, 7, 8, 9, 10];

/// Resolve the known redistributable for a prerequisite kind and architecture.
/// Returns `None` for runtimes WinAppMigrator has no installer plan for: those shipped app-local
/// need no separate install, and anything else is named for whoever is installing to deal with.
pub fn spec(
    kind: &RuntimePrerequisiteKind,
    architecture: &AppArchitecture,
) -> Option<PrerequisiteSpec> {
    match kind {
        RuntimePrerequisiteKind::VisualCppRedistributable => Some(visual_cpp_spec(architecture)),
        RuntimePrerequisiteKind::VisualCpp2005
        | RuntimePrerequisiteKind::VisualCpp2008
        | RuntimePrerequisiteKind::VisualCpp2010
        | RuntimePrerequisiteKind::VisualCpp2012
        | RuntimePrerequisiteKind::VisualCpp2013 => older_visual_cpp_spec(kind, architecture),
        RuntimePrerequisiteKind::DotNetRuntime { flavor, major } => {
            dotnet_runtime_spec(*flavor, *major, architecture)
        }
        RuntimePrerequisiteKind::DotNetFramework35 => Some(dotnet_framework35_spec()),
        RuntimePrerequisiteKind::DotNetFramework4x => Some(dotnet_framework4x_spec()),
        _ => None,
    }
}

/// Every kind and build there is an installer for, with what to call it, so that a test can try
/// each of their downloads for real.
pub fn compiled_specs() -> Vec<(String, PrerequisiteSpec)> {
    let builds = [
        AppArchitecture::X64,
        AppArchitecture::X86,
        AppArchitecture::Arm64,
    ];
    let mut kinds = vec![
        RuntimePrerequisiteKind::VisualCppRedistributable,
        RuntimePrerequisiteKind::VisualCpp2005,
        RuntimePrerequisiteKind::VisualCpp2008,
        RuntimePrerequisiteKind::VisualCpp2010,
        RuntimePrerequisiteKind::VisualCpp2012,
        RuntimePrerequisiteKind::VisualCpp2013,
        RuntimePrerequisiteKind::DotNetFramework35,
        RuntimePrerequisiteKind::DotNetFramework4x,
    ];
    for major in DOTNET_MAJOR_VERSIONS {
        for flavor in [
            DotNetFlavor::Core,
            DotNetFlavor::Desktop,
            DotNetFlavor::AspNetCore,
        ] {
            kinds.push(RuntimePrerequisiteKind::DotNetRuntime { flavor, major });
        }
    }
    let mut specs = Vec::new();
    for kind in &kinds {
        for build in &builds {
            if let Some(found) = spec(kind, build) {
                specs.push((format!("{kind:?} {build:?}"), found));
            }
        }
    }
    specs
}

/// The presence probe for a prerequisite, if one is known.
pub fn presence_probe(
    kind: &RuntimePrerequisiteKind,
    architecture: &AppArchitecture,
) -> Option<PrerequisiteRegistryProbe> {
    spec(kind, architecture).and_then(|spec| spec.probe)
}

/// True when the runtime the probe describes is present on this machine. A probe
/// passes when any candidate key holds the value as a DWORD `>= min_value`.
pub fn is_present(probe: &PrerequisiteRegistryProbe) -> bool {
    let root = RegKey::predef(HKEY_LOCAL_MACHINE);
    for key_path in &probe.key_paths {
        for view in [KEY_WOW64_64KEY, KEY_WOW64_32KEY] {
            let Ok(key) = root.open_subkey_with_flags(key_path, KEY_READ | view) else {
                continue;
            };
            if let Ok(value) = key.get_value::<u32, _>(&probe.value_name)
                && value >= probe.min_value
            {
                return true;
            }
        }
    }
    false
}

/// The bundle record for a prerequisite: which build it is, and its embedded payload when
/// one was downloaded. A `package_relative_path` of `None` means another route is used.
pub fn bundle_record(
    architecture: &AppArchitecture,
    package_relative_path: Option<PathBuf>,
    hash_sha256: Option<String>,
    size_bytes: Option<u64>,
) -> PrerequisiteBundle {
    PrerequisiteBundle {
        architecture: architecture.clone(),
        package_relative_path,
        hash_sha256,
        size_bytes,
    }
}

/// The compiled spec for a prerequisite that has a bundle record.
pub fn spec_for_bundle(
    kind: &RuntimePrerequisiteKind,
    bundle: &PrerequisiteBundle,
) -> Option<PrerequisiteSpec> {
    spec(kind, &bundle.architecture)
}

/// Download `url` to `dest` over HTTPS (SChannel via native-tls), following a few
/// redirects, all of which must be HTTPS too. Used at package time only.
pub fn download(url: &str, dest: &Path) -> Result<()> {
    if !is_https(url) {
        return Err(WinAppMigratorError::Network(format!(
            "refusing to download {url}: it is not HTTPS"
        )));
    }
    let connector = native_tls::TlsConnector::new()
        .map_err(|error| WinAppMigratorError::Network(format!("TLS init failed: {error}")))?;
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(connector))
        // This one also covers each redirect, which is where a download would be led off
        // HTTPS.
        .https_only(true)
        .redirects(MAX_DOWNLOAD_REDIRECTS)
        .timeout_connect(Duration::from_secs(30))
        .timeout_read(DOWNLOAD_TIMEOUT)
        .build();
    let response = agent
        .get(url)
        .call()
        .map_err(|error| WinAppMigratorError::Network(format!("download {url} failed: {error}")))?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|error| WinAppMigratorError::io(parent, error))?;
    }
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_DOWNLOAD_BYTES)
        .read_to_end(&mut bytes)
        .map_err(|error| WinAppMigratorError::Network(format!("reading {url} failed: {error}")))?;
    if bytes.is_empty() {
        return Err(WinAppMigratorError::Network(format!(
            "download {url} returned no data"
        )));
    }
    std::fs::write(dest, &bytes).map_err(|error| WinAppMigratorError::io(dest, error))?;
    Ok(())
}

fn is_https(url: &str) -> bool {
    url.get(..HTTPS_PREFIX.len())
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case(HTTPS_PREFIX))
}

/// Install a prerequisite on the target, by the route its compiled spec gives.
/// Best-effort and non-panicking; the caller decides how to surface the outcome.
pub fn install_bundle(
    kind: &RuntimePrerequisiteKind,
    bundle: &PrerequisiteBundle,
    package_root: &Path,
) -> PrerequisiteInstallOutcome {
    let Some(spec) = spec_for_bundle(kind, bundle) else {
        return PrerequisiteInstallOutcome::Skipped(
            "there is no known installer for this runtime".to_owned(),
        );
    };
    // A Windows optional feature (e.g. NetFx3) is enabled via DISM, never a
    // payload or winget, so it takes precedence over the other mechanisms.
    if let Some(feature) = spec.windows_feature.as_deref() {
        return run_windows_feature(feature);
    }
    if spec.bundleable
        && let Some(relative) = bundle.package_relative_path.as_ref()
    {
        if !is_expected_payload_path(relative, &spec) {
            return PrerequisiteInstallOutcome::Failed(format!(
                "the package names {} as this runtime's installer, which is not where it \
                 belongs, so it was not run",
                relative.display()
            ));
        }
        return match package_destination_path(package_root, relative) {
            Ok(payload) if payload.is_file() => run_payload(&payload, &spec),
            Ok(payload) => PrerequisiteInstallOutcome::Failed(format!(
                "bundled installer is missing: {}",
                payload.display()
            )),
            Err(error) => PrerequisiteInstallOutcome::Failed(format!(
                "bundled installer path is invalid: {error}"
            )),
        };
    }
    if let Some(winget_id) = spec.winget_id.as_deref() {
        return run_winget(winget_id);
    }
    PrerequisiteInstallOutcome::Skipped(format!(
        "no bundled installer or winget id; acquire it from {}",
        spec.download_url
    ))
}

/// A payload belongs at `prerequisites\<the spec's own file name>`, so a manifest cannot
/// point this at some other file it carries.
fn is_expected_payload_path(relative: &Path, spec: &PrerequisiteSpec) -> bool {
    let expected = Path::new(PREREQUISITES_DIR).join(&spec.file_name);
    let mut actual = relative.components();
    let mut wanted = expected.components();
    loop {
        match (actual.next(), wanted.next()) {
            (None, None) => return true,
            (Some(left), Some(right))
                if left.as_os_str().eq_ignore_ascii_case(right.as_os_str()) => {}
            _ => return false,
        }
    }
}

fn run_payload(payload: &Path, spec: &PrerequisiteSpec) -> PrerequisiteInstallOutcome {
    // Checked again here, just before it runs elevated. The check at build time is on
    // another PC, and nothing authenticates the package in between.
    if let Err(reason) = check_microsoft_signature(payload) {
        return PrerequisiteInstallOutcome::Failed(format!(
            "the bundled installer failed its signature check and was not run: {reason}"
        ));
    }
    let mut command = hidden_command(payload);
    command.args(&spec.silent_install_args);
    match wait_with_timeout(command, INSTALL_TIMEOUT) {
        WaitOutcome::Exited(code) => classify_exit_code(code, spec),
        WaitOutcome::TimedOut => PrerequisiteInstallOutcome::Failed(
            "bundled installer did not finish within the timeout and was terminated".to_owned(),
        ),
        WaitOutcome::StartFailed(error) => PrerequisiteInstallOutcome::Failed(format!(
            "could not start bundled installer: {error}"
        )),
    }
}

/// Enable a Windows optional feature on the target via DISM. Used for runtimes
/// that ship as an on-demand Windows feature rather than a redistributable —
/// notably `.NET Framework 3.5` (`NetFx3`), whose payload comes from Windows
/// Update or install media. Best-effort: on failure the caller surfaces an
/// actionable fallback message.
fn run_windows_feature(feature: &str) -> PrerequisiteInstallOutcome {
    let mut command = hidden_command(DISM_EXE);
    command.args([
        "/online",
        "/enable-feature",
        &format!("/featurename:{feature}"),
        "/all",
        "/norestart",
    ]);
    match wait_with_timeout(command, INSTALL_TIMEOUT) {
        // 0 = enabled, 3010 = enabled but a reboot is required to complete it.
        WaitOutcome::Exited(Some(0)) => PrerequisiteInstallOutcome::Installed,
        WaitOutcome::Exited(Some(3010)) => PrerequisiteInstallOutcome::InstalledNeedsReboot,
        WaitOutcome::Exited(code) => PrerequisiteInstallOutcome::Failed(format!(
            "DISM could not enable the {feature} Windows feature (exit {}); the payload likely \
             could not be downloaded. Enable it via Windows Features, or run \
             `dism /online /enable-feature /featurename:{feature} /all /Source:<install-media>\\sources\\sxs /LimitAccess`.",
            code.map(|code| code.to_string())
                .unwrap_or_else(|| "unknown".to_owned())
        )),
        WaitOutcome::TimedOut => PrerequisiteInstallOutcome::Failed(format!(
            "enabling the {feature} Windows feature did not finish within the timeout and was terminated"
        )),
        WaitOutcome::StartFailed(error) => PrerequisiteInstallOutcome::Failed(format!(
            "DISM is not available to enable the {feature} Windows feature: {error}"
        )),
    }
}

/// Where winget really is, checked the same way as a payload. The `winget` on `PATH` is a
/// shortcut in a folder the user can write to, so it could be anything.
fn trusted_winget_path() -> std::result::Result<PathBuf, String> {
    let output = hidden_command(POWERSHELL_EXE)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("{POWERSHELL_IMPORT_APPX}{POWERSHELL_IMPORT_UTILITY}{WINGET_LOCATION_SCRIPT}"),
        ])
        .output()
        .map_err(|error| format!("could not look for winget: {error}"))?;
    let location = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if !output.status.success() || location.is_empty() {
        return Err(WINGET_NOT_INSTALLED.to_owned());
    }
    let winget = Path::new(&location).join(WINGET_EXE);
    check_microsoft_signature(&winget).map_err(|reason| {
        format!(
            "{} failed its signature check and was not run: {reason}",
            winget.display()
        )
    })?;
    Ok(winget)
}

fn run_winget(winget_id: &str) -> PrerequisiteInstallOutcome {
    let winget = match trusted_winget_path() {
        Ok(winget) => winget,
        Err(reason) => {
            return PrerequisiteInstallOutcome::Failed(format!(
                "could not use winget to install {winget_id}: {reason}"
            ));
        }
    };
    let mut command = hidden_command(winget);
    command.args([
        "install",
        "--id",
        winget_id,
        "--exact",
        "--silent",
        "--accept-package-agreements",
        "--accept-source-agreements",
        "--disable-interactivity",
    ]);
    match wait_with_timeout(command, INSTALL_TIMEOUT) {
        WaitOutcome::Exited(Some(0)) => PrerequisiteInstallOutcome::Installed,
        WaitOutcome::Exited(code) => PrerequisiteInstallOutcome::Failed(format!(
            "winget install {winget_id} exited with {}",
            code.map(|code| code.to_string())
                .unwrap_or_else(|| "an unknown code".to_owned())
        )),
        WaitOutcome::TimedOut => PrerequisiteInstallOutcome::Failed(format!(
            "winget install {winget_id} did not finish within the timeout and was terminated"
        )),
        WaitOutcome::StartFailed(error) => PrerequisiteInstallOutcome::Failed(format!(
            "winget is not available to install {winget_id}: {error}"
        )),
    }
}

enum WaitOutcome {
    Exited(Option<i32>),
    TimedOut,
    StartFailed(std::io::Error),
}

/// Run a command to completion with detached stdio and a hard timeout, killing
/// it if it overruns so a misbehaving installer cannot block restore forever.
fn wait_with_timeout(mut command: Command, timeout: Duration) -> WaitOutcome {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => return WaitOutcome::StartFailed(error),
    };
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return WaitOutcome::Exited(status.code()),
            Ok(None) => {}
            Err(error) => return WaitOutcome::StartFailed(error),
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return WaitOutcome::TimedOut;
        }
        std::thread::sleep(WAIT_POLL_INTERVAL);
    }
}

fn classify_exit_code(code: Option<i32>, spec: &PrerequisiteSpec) -> PrerequisiteInstallOutcome {
    let Some(code) = code else {
        return PrerequisiteInstallOutcome::Failed(
            "installer was terminated without an exit code".to_owned(),
        );
    };
    if spec.reboot_exit_codes.contains(&code) {
        PrerequisiteInstallOutcome::InstalledNeedsReboot
    } else if spec.success_exit_codes.contains(&code) {
        PrerequisiteInstallOutcome::Installed
    } else {
        PrerequisiteInstallOutcome::Failed(format!("installer exited with code {code}"))
    }
}

fn visual_cpp_spec(architecture: &AppArchitecture) -> PrerequisiteSpec {
    let (suffix, winget_id) = match architecture {
        AppArchitecture::X86 => ("x86", "Microsoft.VCRedist.2015+.x86"),
        AppArchitecture::Arm64 => ("arm64", "Microsoft.VCRedist.2015+.arm64"),
        // x64 is the default for unknown architectures: it is the common case for
        // modern desktop applications.
        AppArchitecture::X64 | AppArchitecture::Unknown => ("x64", "Microsoft.VCRedist.2015+.x64"),
    };
    PrerequisiteSpec {
        bundleable: true,
        file_name: format!("vc_redist.{suffix}.exe"),
        download_url: format!("https://aka.ms/vs/17/release/vc_redist.{suffix}.exe"),
        silent_install_args: ["/install", "/quiet", "/norestart"]
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect(),
        // 0 = success, 1638 = a newer version is already installed.
        success_exit_codes: vec![0, 1638],
        // 3010 = success, reboot required to complete.
        reboot_exit_codes: vec![3010],
        winget_id: Some(winget_id.to_owned()),
        windows_feature: None,
        requires_admin: true,
        probe: Some(PrerequisiteRegistryProbe {
            key_paths: vec![
                format!(r"SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\{suffix}"),
                format!(r"SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\{suffix}"),
            ],
            value_name: "Installed".to_owned(),
            min_value: 1,
        }),
    }
}

/// The Visual C++ runtimes from before 2015, which each have a redistributable of their own and
/// are not replaced by a later one. None of them was made for Arm64 on Windows 10 or 11, so that
/// has no spec. Visual Studio 2013 and later say they are there in the same key as 2015 does.
/// 2005 and 2008 say so only by their MSI product, so those have no probe, and run each time.
fn older_visual_cpp_spec(
    kind: &RuntimePrerequisiteKind,
    architecture: &AppArchitecture,
) -> Option<PrerequisiteSpec> {
    let suffix = match architecture {
        AppArchitecture::X86 => "x86",
        AppArchitecture::X64 | AppArchitecture::Unknown => "x64",
        AppArchitecture::Arm64 => return None,
    };
    let quiet: &[&str] = &["/q"];
    let quiet_no_restart: &[&str] = &["/q", "/norestart"];
    let install_quiet: &[&str] = &["/install", "/quiet", "/norestart"];
    let (year, url, args, probe_key) = match kind {
        RuntimePrerequisiteKind::VisualCpp2005 => (
            "2005",
            format!(
                "https://download.microsoft.com/download/8/B/4/8B42259F-5D70-43F4-AC2E-4B208FD8D66A/vcredist_{suffix}.EXE"
            ),
            quiet,
            None,
        ),
        RuntimePrerequisiteKind::VisualCpp2008 => (
            "2008",
            format!(
                "https://download.microsoft.com/download/5/D/8/5D8C65CB-C849-4025-8E95-C3966CAFD8AE/vcredist_{suffix}.exe"
            ),
            quiet,
            None,
        ),
        RuntimePrerequisiteKind::VisualCpp2010 => (
            "2010",
            format!(
                "https://download.microsoft.com/download/1/6/5/165255E7-1014-4D0A-B094-B6A430A6BFFC/vcredist_{suffix}.exe"
            ),
            quiet_no_restart,
            Some(format!(r"Microsoft\VisualStudio\10.0\VC\VCRedist\{suffix}")),
        ),
        RuntimePrerequisiteKind::VisualCpp2012 => (
            "2012",
            format!(
                "https://download.microsoft.com/download/1/6/B/16B06F60-3B20-4FF2-B699-5E9B7962F9AE/VSU_4/vcredist_{suffix}.exe"
            ),
            install_quiet,
            Some(format!(r"Microsoft\VisualStudio\11.0\VC\Runtimes\{suffix}")),
        ),
        RuntimePrerequisiteKind::VisualCpp2013 => (
            "2013",
            format!("https://aka.ms/highdpimfc2013{suffix}enu"),
            install_quiet,
            Some(format!(r"Microsoft\VisualStudio\12.0\VC\Runtimes\{suffix}")),
        ),
        _ => return None,
    };
    Some(PrerequisiteSpec {
        bundleable: true,
        file_name: format!("vcredist{year}.{suffix}.exe"),
        download_url: url,
        silent_install_args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        // 0 = success, 1638 = a newer version is already installed.
        success_exit_codes: vec![0, 1638],
        reboot_exit_codes: vec![3010],
        winget_id: Some(format!("Microsoft.VCRedist.{year}.{suffix}")),
        windows_feature: None,
        requires_admin: true,
        probe: probe_key.map(|key| PrerequisiteRegistryProbe {
            key_paths: vec![
                format!(r"SOFTWARE\{key}"),
                format!(r"SOFTWARE\WOW6432Node\{key}"),
            ],
            value_name: "Installed".to_owned(),
            min_value: 1,
        }),
    })
}

/// A version of .NET's shared runtime, from the link Microsoft keeps for that version's latest
/// build. It is bundled like a Visual C++ runtime is, and signed like one, with winget as what is
/// left if the download fails. Its versions are values named after each build, so no one DWORD says
/// it is there, and an installer run on top of itself just exits.
fn dotnet_runtime_spec(
    flavor: DotNetFlavor,
    major: u32,
    architecture: &AppArchitecture,
) -> Option<PrerequisiteSpec> {
    if !DOTNET_MAJOR_VERSIONS.contains(&major) {
        return None;
    }
    let suffix = match architecture {
        AppArchitecture::X86 => "x86",
        AppArchitecture::Arm64 => "arm64",
        AppArchitecture::X64 | AppArchitecture::Unknown => "x64",
    };
    let (product, winget) = match flavor {
        DotNetFlavor::Core => ("dotnet-runtime", "Runtime"),
        DotNetFlavor::Desktop => ("windowsdesktop-runtime", "DesktopRuntime"),
        DotNetFlavor::AspNetCore => ("aspnetcore-runtime", "AspNetCore"),
    };
    Some(PrerequisiteSpec {
        bundleable: true,
        file_name: format!("{product}-{major}.0-win-{suffix}.exe"),
        download_url: format!("https://aka.ms/dotnet/{major}.0/{product}-win-{suffix}.exe"),
        silent_install_args: ["/install", "/quiet", "/norestart"]
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect(),
        success_exit_codes: vec![0, 1638],
        reboot_exit_codes: vec![3010],
        winget_id: Some(format!("Microsoft.DotNet.{winget}.{major}")),
        windows_feature: None,
        requires_admin: true,
        probe: None,
    })
}

/// .NET Framework 3.5 (CLR v2.0). On Windows 10/11 it is an on-demand Windows
/// feature (`NetFx3`) whose payload comes from Windows Update or install media,
/// so it is enabled via DISM rather than a downloadable installer. Presence is
/// probed via `NDP\v3.5\Install`.
fn dotnet_framework35_spec() -> PrerequisiteSpec {
    PrerequisiteSpec {
        bundleable: false,
        file_name: "netfx3-windows-feature".to_owned(),
        download_url: "https://learn.microsoft.com/dotnet/framework/install/dotnet-35-windows"
            .to_owned(),
        silent_install_args: Vec::new(),
        success_exit_codes: vec![0],
        reboot_exit_codes: vec![3010],
        winget_id: None,
        windows_feature: Some("NetFx3".to_owned()),
        requires_admin: true,
        probe: Some(PrerequisiteRegistryProbe {
            key_paths: vec![r"SOFTWARE\Microsoft\NET Framework Setup\NDP\v3.5".to_owned()],
            value_name: "Install".to_owned(),
            min_value: 1,
        }),
    }
}

/// .NET Framework 4.x (CLR v4.0). Ships with Windows 10/11, so there is nothing
/// to install; the spec exists only to carry a presence probe (`NDP\v4\Full\
/// Release` >= 528040, i.e. 4.8) so the prerequisite can be listed and confirmed.
fn dotnet_framework4x_spec() -> PrerequisiteSpec {
    PrerequisiteSpec {
        bundleable: false,
        file_name: "dotnet-framework-4x-inbox".to_owned(),
        download_url: "https://dotnet.microsoft.com/download/dotnet-framework".to_owned(),
        silent_install_args: Vec::new(),
        success_exit_codes: vec![0],
        reboot_exit_codes: vec![3010],
        winget_id: None,
        windows_feature: None,
        requires_admin: false,
        probe: Some(PrerequisiteRegistryProbe {
            key_paths: vec![r"SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full".to_owned()],
            value_name: "Release".to_owned(),
            min_value: 528_040,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual_cpp_spec_is_arch_specific() {
        let x64 = visual_cpp_spec(&AppArchitecture::X64);
        assert_eq!(x64.file_name, "vc_redist.x64.exe");
        assert!(x64.download_url.ends_with("vc_redist.x64.exe"));
        assert!(x64.bundleable);

        let x86 = visual_cpp_spec(&AppArchitecture::X86);
        assert_eq!(x86.file_name, "vc_redist.x86.exe");

        // Unknown architecture falls back to x64.
        let unknown = visual_cpp_spec(&AppArchitecture::Unknown);
        assert_eq!(unknown.file_name, "vc_redist.x64.exe");
    }

    #[test]
    fn reboot_exit_code_maps_to_needs_reboot() {
        let spec = visual_cpp_spec(&AppArchitecture::X64);
        assert_eq!(
            classify_exit_code(Some(3010), &spec),
            PrerequisiteInstallOutcome::InstalledNeedsReboot
        );
        assert_eq!(
            classify_exit_code(Some(0), &spec),
            PrerequisiteInstallOutcome::Installed
        );
        assert_eq!(
            classify_exit_code(Some(1603), &spec),
            PrerequisiteInstallOutcome::Failed("installer exited with code 1603".to_owned())
        );
    }

    #[test]
    fn only_https_can_be_downloaded() {
        assert!(is_https("https://aka.ms/vs/17/release/vc_redist.x64.exe"));
        assert!(is_https("HTTPS://aka.ms/x"));
        for url in [
            "http://aka.ms/x",
            "file:///C:/x.exe",
            "ftp://x/y",
            "",
            "https",
        ] {
            assert!(!is_https(url), "{url}");
        }
        let folder = std::env::temp_dir().join("wam-never-written.exe");
        let refused = download("http://127.0.0.1:9/x.exe", &folder);
        assert!(
            matches!(&refused, Err(WinAppMigratorError::Network(message)) if message.contains("not HTTPS")),
            "{refused:?}"
        );
        assert!(!folder.exists());
    }

    /// The compiled specs are the only source of what runs, and all of them come from Microsoft
    /// over HTTPS. Whether each is really there and signed is tried for real on CI.
    #[test]
    fn every_compiled_download_is_https_and_microsofts() {
        const MICROSOFT_HOSTS: [&str; 4] = [
            "aka.ms",
            "download.microsoft.com",
            "dotnet.microsoft.com",
            "learn.microsoft.com",
        ];
        let specs = compiled_specs();
        assert!(specs.len() > 40, "{}", specs.len());
        for (name, spec) in specs {
            assert!(
                is_https(&spec.download_url),
                "{name}: {}",
                spec.download_url
            );
            let host = spec.download_url[HTTPS_PREFIX.len()..]
                .split('/')
                .next()
                .unwrap_or_default();
            assert!(MICROSOFT_HOSTS.contains(&host), "{name}: {host}");
        }
    }

    #[test]
    fn each_kind_and_build_has_its_own_installer_and_no_two_share_a_file() {
        let specs = compiled_specs();
        let mut names: Vec<&str> = specs
            .iter()
            .filter(|(_, spec)| spec.bundleable)
            .map(|(_, spec)| spec.file_name.as_str())
            .collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total, "{names:?}");
    }

    #[test]
    fn a_runtime_is_only_ever_what_it_says_and_a_version_there_is_no_spec_for_gets_none() {
        let net = |major| RuntimePrerequisiteKind::DotNetRuntime {
            flavor: DotNetFlavor::Desktop,
            major,
        };
        let eight = spec(&net(8), &AppArchitecture::X64);
        assert_eq!(
            eight.as_ref().and_then(|spec| spec.winget_id.as_deref()),
            Some("Microsoft.DotNet.DesktopRuntime.8")
        );
        assert_eq!(
            eight.map(|spec| spec.download_url),
            Some("https://aka.ms/dotnet/8.0/windowsdesktop-runtime-win-x64.exe".to_owned())
        );
        assert_eq!(
            spec(&net(9), &AppArchitecture::Arm64).map(|spec| spec.file_name),
            Some("windowsdesktop-runtime-9.0-win-arm64.exe".to_owned())
        );
        for major in [0, 3, 5, 11, 999] {
            assert!(
                spec(&net(major), &AppArchitecture::X64).is_none(),
                "{major}"
            );
        }
        assert!(
            spec(
                &RuntimePrerequisiteKind::DotNetDesktopRuntime,
                &AppArchitecture::X64
            )
            .is_none(),
            "a .NET that is unknown or ships with the app has no installer, and is not .NET 8"
        );
        for kind in [
            RuntimePrerequisiteKind::VisualCpp2005,
            RuntimePrerequisiteKind::VisualCpp2008,
            RuntimePrerequisiteKind::VisualCpp2010,
            RuntimePrerequisiteKind::VisualCpp2012,
            RuntimePrerequisiteKind::VisualCpp2013,
        ] {
            let x86 = spec(&kind, &AppArchitecture::X86).map(|spec| spec.file_name);
            assert!(
                x86.is_some_and(|name| name.ends_with(".x86.exe")),
                "{kind:?}"
            );
            assert!(spec(&kind, &AppArchitecture::Arm64).is_none(), "{kind:?}");
        }
    }

    #[test]
    fn a_payload_must_be_where_its_spec_says() {
        let spec = visual_cpp_spec(&AppArchitecture::X64);
        for good in [
            r"prerequisites\vc_redist.x64.exe",
            "prerequisites/vc_redist.x64.exe",
            r"Prerequisites\VC_REDIST.X64.EXE",
        ] {
            assert!(is_expected_payload_path(Path::new(good), &spec), "{good}");
        }
        for bad in [
            r"files\evil.exe",
            "vc_redist.x64.exe",
            r"prerequisites\evil.exe",
            r"prerequisites\vc_redist.x86.exe",
            r"prerequisites\sub\vc_redist.x64.exe",
            r"prerequisites\..\files\vc_redist.x64.exe",
        ] {
            assert!(!is_expected_payload_path(Path::new(bad), &spec), "{bad}");
        }
    }

    fn visual_cpp_bundle(path: &str) -> PrerequisiteBundle {
        bundle_record(&AppArchitecture::X64, Some(PathBuf::from(path)), None, None)
    }

    fn failure_reason(outcome: PrerequisiteInstallOutcome) -> String {
        match outcome {
            PrerequisiteInstallOutcome::Failed(reason) => reason,
            other => panic!("expected a failure, got {other:?}"),
        }
    }

    /// The file is not a program at all, so the test could not run it even by mistake, and
    /// the outcome shows the signature check stopped it first.
    #[test]
    fn an_unsigned_payload_is_not_run_and_says_why() -> std::io::Result<()> {
        let package = tempfile::tempdir()?;
        let payload = package.path().join(r"prerequisites\vc_redist.x64.exe");
        std::fs::create_dir_all(package.path().join(PREREQUISITES_DIR))?;
        std::fs::write(&payload, b"not signed")?;

        let reason = failure_reason(install_bundle(
            &RuntimePrerequisiteKind::VisualCppRedistributable,
            &visual_cpp_bundle(r"prerequisites\vc_redist.x64.exe"),
            package.path(),
        ));

        assert!(reason.contains("signature"), "{reason}");
        assert!(reason.contains("not run"), "{reason}");
        Ok(())
    }

    #[test]
    fn a_manifest_cannot_choose_which_file_runs() -> std::io::Result<()> {
        let package = tempfile::tempdir()?;
        std::fs::write(package.path().join("evil.exe"), b"x")?;

        let reason = failure_reason(install_bundle(
            &RuntimePrerequisiteKind::VisualCppRedistributable,
            &visual_cpp_bundle("evil.exe"),
            package.path(),
        ));

        assert!(reason.contains("evil.exe"), "{reason}");
        assert!(reason.contains("not run"), "{reason}");
        Ok(())
    }

    #[test]
    fn a_runtime_with_no_spec_is_skipped() {
        let outcome = install_bundle(
            &RuntimePrerequisiteKind::JavaRuntime,
            &visual_cpp_bundle(r"prerequisites\vc_redist.x64.exe"),
            Path::new(r"C:\Nowhere"),
        );
        assert!(matches!(outcome, PrerequisiteInstallOutcome::Skipped(_)));
    }

    /// Only asks. Skips on a PC without winget, and nothing else, so a failed signature
    /// check on a real winget would fail this.
    #[test]
    fn winget_is_found_where_windows_installed_it_and_not_on_path() {
        match trusted_winget_path() {
            Ok(winget) => {
                let text = winget.to_string_lossy().to_ascii_lowercase();
                assert!(
                    text.contains(r"\windowsapps\microsoft.desktopappinstaller"),
                    "{text}"
                );
                assert!(!text.contains(r"\appdata\"), "{text}");
            }
            Err(reason) if reason == WINGET_NOT_INSTALLED => eprintln!("skipping: {reason}"),
            Err(reason) => panic!("{reason}"),
        }
    }

    #[test]
    fn dotnet_framework35_is_a_windows_feature() {
        let spec = dotnet_framework35_spec();
        assert!(!spec.bundleable);
        assert_eq!(spec.windows_feature.as_deref(), Some("NetFx3"));
        assert!(spec.winget_id.is_none());
        assert!(spec.requires_admin);
        assert!(
            matches!(spec.probe, Some(ref probe) if probe.value_name == "Install" && probe.min_value == 1),
            "framework 3.5 is confirmed via NDP\\v3.5\\Install >= 1"
        );
    }

    #[test]
    fn dotnet_framework4x_is_inbox_and_probe_only() {
        let spec = dotnet_framework4x_spec();
        assert!(spec.windows_feature.is_none());
        assert!(spec.winget_id.is_none());
        assert!(!spec.requires_admin);
        assert!(
            spec.probe.is_some(),
            "4.x is confirmed via a presence probe"
        );
    }

    #[test]
    fn spec_maps_dotnet_framework_kinds() {
        assert!(matches!(
            spec(&RuntimePrerequisiteKind::DotNetFramework35, &AppArchitecture::X86),
            Some(resolved) if resolved.windows_feature.as_deref() == Some("NetFx3")
        ));
        assert!(matches!(
            spec(&RuntimePrerequisiteKind::DotNetFramework4x, &AppArchitecture::X64),
            Some(resolved) if resolved.windows_feature.is_none() && !resolved.requires_admin
        ));
    }

    #[test]
    fn a_bundle_reaches_its_compiled_spec_by_kind_and_build() {
        let bundle = bundle_record(&AppArchitecture::X86, None, None, None);
        let framework = spec_for_bundle(&RuntimePrerequisiteKind::DotNetFramework35, &bundle);
        assert_eq!(
            framework.and_then(|spec| spec.windows_feature),
            Some("NetFx3".to_owned())
        );
        let visual_cpp =
            spec_for_bundle(&RuntimePrerequisiteKind::VisualCppRedistributable, &bundle);
        assert_eq!(
            visual_cpp.map(|spec| spec.file_name),
            Some("vc_redist.x86.exe".to_owned())
        );
    }

    /// An old or hostile manifest can carry anything. These keys are simply not read.
    #[test]
    fn what_to_run_cannot_come_from_a_manifest() -> serde_json::Result<()> {
        let bundle: PrerequisiteBundle = serde_json::from_str(
            r#"{
                "package_relative_path": null,
                "hash_sha256": null,
                "size_bytes": null,
                "download_url": "http://evil.example/x.exe",
                "silent_install_args": ["/c", "calc"],
                "winget_id": "Evil.Package",
                "windows_feature": "TelnetClient",
                "success_exit_codes": [1],
                "requires_admin": false
            }"#,
        )?;
        assert_eq!(bundle.architecture, AppArchitecture::Unknown);
        let spec = spec_for_bundle(&RuntimePrerequisiteKind::VisualCppRedistributable, &bundle);
        let spec = spec.ok_or_else(|| serde::de::Error::custom("no spec"))?;
        assert_eq!(
            spec.winget_id.as_deref(),
            Some("Microsoft.VCRedist.2015+.x64")
        );
        assert_eq!(spec.windows_feature, None);
        assert_eq!(
            spec.silent_install_args,
            ["/install", "/quiet", "/norestart"]
        );
        Ok(())
    }
}
