//! Runtime prerequisite acquisition, installation, and presence probing.
//!
//! WinAppMigrator detects the redistributable runtimes an application needs
//! (Visual C++ runtime, .NET, …) and makes a concrete plan to put them on the
//! target machine. For redistributables it can bundle, it downloads the official
//! installer at package time and embeds it; the installer then installs it
//! silently and offline on the target and confirms the result with a registry
//! probe. When a redistributable cannot be bundled it falls back to a documented
//! winget/URL install on the target. Every step is best-effort and reported.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use crate::errors::{Result, WinAppMigratorError};
use crate::file_discovery::package_destination_path;
use crate::models::{
    AppArchitecture, PrerequisiteBundle, PrerequisiteRegistryProbe, RuntimePrerequisiteKind,
};
use crate::path_utils::hidden_command;

const WINGET_EXE: &str = "winget";
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(180);
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
    pub file_name: &'static str,
    pub download_url: String,
    pub silent_install_args: Vec<String>,
    pub success_exit_codes: Vec<i32>,
    pub reboot_exit_codes: Vec<i32>,
    pub winget_id: Option<String>,
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

/// Resolve the known redistributable for a prerequisite kind and architecture.
/// Returns `None` for runtimes WinAppMigrator has no installer plan for (these
/// are typically shipped app-local and need no separate install).
pub fn spec(
    kind: &RuntimePrerequisiteKind,
    architecture: &AppArchitecture,
) -> Option<PrerequisiteSpec> {
    match kind {
        RuntimePrerequisiteKind::VisualCppRedistributable => Some(visual_cpp_spec(architecture)),
        RuntimePrerequisiteKind::DotNetDesktopRuntime => Some(dotnet_desktop_spec()),
        _ => None,
    }
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

/// Build the bundle record for a spec, given the embedded payload (when one was
/// downloaded). A `package_relative_path` of `None` means winget/URL fallback.
pub fn bundle_from_spec(
    spec: &PrerequisiteSpec,
    package_relative_path: Option<std::path::PathBuf>,
    hash_sha256: Option<String>,
    size_bytes: Option<u64>,
) -> PrerequisiteBundle {
    PrerequisiteBundle {
        package_relative_path,
        download_url: spec.download_url.clone(),
        hash_sha256,
        size_bytes,
        silent_install_args: spec.silent_install_args.clone(),
        success_exit_codes: spec.success_exit_codes.clone(),
        reboot_exit_codes: spec.reboot_exit_codes.clone(),
        winget_id: spec.winget_id.clone(),
        requires_admin: spec.requires_admin,
    }
}

/// Download `url` to `dest` over HTTPS (SChannel via native-tls), following
/// redirects. Used at package time only.
pub fn download(url: &str, dest: &Path) -> Result<()> {
    let connector = native_tls::TlsConnector::new()
        .map_err(|error| WinAppMigratorError::Network(format!("TLS init failed: {error}")))?;
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(connector))
        .redirects(10)
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
    let mut reader = response.into_reader();
    let mut bytes = Vec::new();
    reader
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

/// Install a bundled (or winget-fallback) prerequisite on the target. Best-effort
/// and non-panicking; the caller decides how to surface the outcome.
pub fn install_bundle(
    bundle: &PrerequisiteBundle,
    package_root: &Path,
) -> PrerequisiteInstallOutcome {
    if let Some(relative) = bundle.package_relative_path.as_ref() {
        match package_destination_path(package_root, relative) {
            Ok(payload) if payload.is_file() => return run_payload(&payload, bundle),
            Ok(payload) => {
                return PrerequisiteInstallOutcome::Failed(format!(
                    "bundled installer is missing: {}",
                    payload.display()
                ));
            }
            Err(error) => {
                return PrerequisiteInstallOutcome::Failed(format!(
                    "bundled installer path is invalid: {error}"
                ));
            }
        }
    }
    if let Some(winget_id) = bundle.winget_id.as_deref() {
        return run_winget(winget_id);
    }
    PrerequisiteInstallOutcome::Skipped(format!(
        "no bundled installer or winget id; acquire it from {}",
        bundle.download_url
    ))
}

fn run_payload(payload: &Path, bundle: &PrerequisiteBundle) -> PrerequisiteInstallOutcome {
    let mut command = hidden_command(payload);
    command.args(&bundle.silent_install_args);
    match wait_with_timeout(command, INSTALL_TIMEOUT) {
        WaitOutcome::Exited(code) => classify_exit_code(code, bundle),
        WaitOutcome::TimedOut => PrerequisiteInstallOutcome::Failed(
            "bundled installer did not finish within the timeout and was terminated".to_owned(),
        ),
        WaitOutcome::StartFailed(error) => PrerequisiteInstallOutcome::Failed(format!(
            "could not start bundled installer: {error}"
        )),
    }
}

fn run_winget(winget_id: &str) -> PrerequisiteInstallOutcome {
    let mut command = hidden_command(WINGET_EXE);
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

fn classify_exit_code(
    code: Option<i32>,
    bundle: &PrerequisiteBundle,
) -> PrerequisiteInstallOutcome {
    let Some(code) = code else {
        return PrerequisiteInstallOutcome::Failed(
            "installer was terminated without an exit code".to_owned(),
        );
    };
    if bundle.reboot_exit_codes.contains(&code) {
        PrerequisiteInstallOutcome::InstalledNeedsReboot
    } else if bundle.success_exit_codes.contains(&code) {
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
        file_name: vc_redist_file_name(suffix),
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

fn vc_redist_file_name(suffix: &str) -> &'static str {
    match suffix {
        "x86" => "vc_redist.x86.exe",
        "arm64" => "vc_redist.arm64.exe",
        _ => "vc_redist.x64.exe",
    }
}

fn dotnet_desktop_spec() -> PrerequisiteSpec {
    // .NET runtime download URLs are version-pinned and rotate, so it is not
    // bundled; the target acquires it via winget. Its presence is not probed via
    // a single DWORD key, so restore/verify treat it as best-effort.
    PrerequisiteSpec {
        bundleable: false,
        file_name: "windowsdesktop-runtime.exe",
        download_url: "https://dotnet.microsoft.com/download/dotnet".to_owned(),
        silent_install_args: ["/install", "/quiet", "/norestart"]
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect(),
        success_exit_codes: vec![0, 1638],
        reboot_exit_codes: vec![3010],
        winget_id: Some("Microsoft.DotNet.DesktopRuntime.8".to_owned()),
        requires_admin: true,
        probe: None,
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
        let bundle = bundle_from_spec(
            &visual_cpp_spec(&AppArchitecture::X64),
            Some(std::path::PathBuf::from("prerequisites/vc_redist.x64.exe")),
            None,
            None,
        );
        assert_eq!(
            classify_exit_code(Some(3010), &bundle),
            PrerequisiteInstallOutcome::InstalledNeedsReboot
        );
        assert_eq!(
            classify_exit_code(Some(0), &bundle),
            PrerequisiteInstallOutcome::Installed
        );
        assert_eq!(
            classify_exit_code(Some(1603), &bundle),
            PrerequisiteInstallOutcome::Failed("installer exited with code 1603".to_owned())
        );
    }

    #[test]
    fn dotnet_is_winget_only() {
        let spec = dotnet_desktop_spec();
        assert!(!spec.bundleable);
        assert!(spec.winget_id.is_some());
        assert!(spec.probe.is_none());
    }
}
