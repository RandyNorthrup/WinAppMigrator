//! A real round trip through the code that changes the machine, on GitHub's throwaway
//! Windows runner and nowhere else.
//!
//! It sets up a harmless fake app on the runner: files under Program Files, an uninstall
//! entry, services, a driver entry that is never loaded, scheduled tasks, registry keys,
//! Run values, environment variables, `Path` entries, and shortcuts. It scans and packages
//! that with the real core code, removes all of it, runs the real installer, and then looks
//! at the machine with Windows' own tools rather than with the app's verifier. Packages
//! written by hand cover what a scan cannot produce, such as asking for refused items.
//!
//! It then takes an install back out in every way that can happen: an uninstall run as Windows
//! would run it, a failure late in an install, and an installer killed part-way, comparing the
//! machine before and after with those same tools.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do
//! nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
//! `WAM_ELEVATED_E2E=1` are all set. Never set those on a real PC. With all three set
//! and no elevation they fail, so that the job cannot pass without having run.
//!
//! Every check is made even after one fails, so one run shows everything that is wrong.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use winappmigrator_core::archive::{create_zip_archive, has_certificate_table};
use winappmigrator_core::hashing::sha256_file;
use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::prerequisites::download;
use winappmigrator_core::privilege::{ProtectedFolder, is_elevated};
use winappmigrator_core::{
    AppArchitecture, AppSelection, BuildMonitor, DriverEntry, DriverKind, EmbeddedPackage,
    EnvironmentScope, EnvironmentVariableEntry, FileClassification, FileEntry, InstallerStub,
    MatchConfidence, PackageManifest, PackageOptions, PrerequisiteBundle, RegistryEntry,
    RegistryHive, RegistryValueKind, RuntimePrerequisite, RuntimePrerequisiteKind, ScanSource,
    ScheduledTaskEntry, ServiceEntry, ShortcutEntry, SystemContext, create_package,
    create_self_extracting_archive, list_installed_apps_from, read_embedded_manifest,
};
use winappmigrator_installer::cleanup::create_staging_dir;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const APP_NAME: &str = "WAM E2E App";
const VENDOR: &str = "WAM E2E Vendor";
const SOURCE_ROOT: &str = r"C:\Program Files\WAM E2E App";
const TARGET_ROOT: &str = r"C:\Program Files\WAM E2E Restored";
const PROGRAM_FILES: &str = r"C:\Program Files";
const PROGRAM_DATA_FOLDER: &str = r"C:\ProgramData\WAM E2E App";
/// Where the fake app says what it was started with. Not named for the product, so a scan
/// does not take it for the app's own data.
const MARKER_DIR: &str = r"C:\ProgramData\WAM E2E Markers";
/// On the source's `Path`, and belonging to something else, so it must not be restored.
const OTHER_PATH_ENTRY: &str = r"C:\WAM E2E Other Tool\bin";
/// Where everything an install keeps is. Fixed, so a test can only look.
const STATE_ROOT: &str = r"C:\ProgramData\WinAppMigrator";
const INSTALLS_DIR: &str = r"C:\ProgramData\WinAppMigrator\installs";
const LOGS_DIR: &str = r"C:\ProgramData\WinAppMigrator\logs";
/// In values the fake app holds, and never to be found in a log.
const SECRET_IN_REGISTRY: &str = "wam-e2e-secret-in-the-registry";
const SECRET_IN_ENVIRONMENT: &str = "wam-e2e-secret-in-the-environment";
/// How long the folder of an uninstaller is given to go after it exits.
const SELF_REMOVAL_PATIENCE: Duration = Duration::from_secs(90);
/// What a process gets from Windows for starting a program that demands elevation, which is
/// the answer that means Windows could read it, so a person would get asked.
const ERROR_ELEVATION_REQUIRED: i32 = 740;

const HKLM_APP_KEY: &str = r"HKLM\Software\WAM E2E Vendor\WAM E2E App";
const HKLM_WOW_KEY: &str = r"HKLM\Software\WOW6432Node\WAM E2E Vendor\WAM E2E App";
const HKCU_APP_KEY: &str = r"HKCU\Software\WAM E2E Vendor\WAM E2E App";
const UNINSTALL_KEY: &str = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM E2E App";
const HKLM_RUN_KEY: &str = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Run";
const HKCU_RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
const APP_PATHS_KEY: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\App Paths\WAM E2E App.exe";
const CLSID_KEY: &str = r"HKLM\Software\Classes\CLSID\{5A7E6E0C-E2E0-4000-8000-00000000E2E0}";
const HKCU_EXTENSION_KEY: &str = r"HKCU\Software\Classes\.wame2e";
const HKCU_PROG_ID_KEY: &str = r"HKCU\Software\Classes\WamE2E.Document";
const MACHINE_ENVIRONMENT_KEY: &str =
    r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment";
const USER_ENVIRONMENT_KEY: &str = r"HKCU\Environment";
const SERVICES_KEY: &str = r"HKLM\SYSTEM\CurrentControlSet\Services";
const VC_RUNTIME_KEY: &str = r"HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64";
const VC_RUNTIME_KEY_32: &str =
    r"HKLM\SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64";
/// A short link that redirects to Microsoft's download servers, which is why it is the
/// real test of following a redirect.
const VC_REDIST_URL: &str = "https://aka.ms/vs/17/release/vc_redist.x64.exe";

const TASK_FOLDER: &str = r"\WAM E2E\";
const SYSTEM_TASK: &str = "Wartung Größe";
const USER_TASK: &str = "Per User";

const DRIVER_NAME: &str = "WamE2EDriver";
const FAT32_DRIVE: &str = r"W:\";

/// A program with no dependencies but the runtime the scan should find. It writes down how
/// it was started and as whom, and does nothing else. It is not a real service, so Windows
/// reports that it did not start, after it has already run as the service's account. Built
/// without `crt-static`, unlike this project, so that it imports the Visual C++ runtime.
const FAKE_APP_SOURCE: &str = r#"
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let after = |flag: &str| {
        args.windows(2)
            .find(|pair| pair[0] == flag)
            .map(|pair| pair[1].clone())
    };
    let marker = after("--marker").unwrap_or_else(|| "unmarked".to_owned());
    let who = Command::new(r"C:\Windows\System32\whoami.exe")
        .args(["/user", "/fo", "csv", "/nh"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    let exe = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_default();
    let started = after("--try-run")
        .map(|path| match Command::new(&path).output() {
            Ok(_) => "ran".to_owned(),
            Err(error) => format!("refused: {}", error.raw_os_error().unwrap_or_default()),
        })
        .unwrap_or_default();
    let read = after("--try-read")
        .map(|path| match std::fs::read(&path) {
            Ok(_) => "read".to_owned(),
            Err(error) => format!("refused: {error}"),
        })
        .unwrap_or_default();
    let probe = after("--try-write")
        .map(|path| match std::fs::write(&path, b"planted") {
            Ok(()) => "written".to_owned(),
            Err(error) => format!("refused: {error}"),
        })
        .unwrap_or_default();
    let text = format!(
        "exe={exe}\nwho={who}\nargs={}\nstarted={started}\nread={read}\nprobe={probe}\n",
        args.join("|")
    );
    let _ = std::fs::write(format!(r"C:\ProgramData\WAM E2E Markers\{marker}.txt"), text);
}
"#;

// ---------------------------------------------------------------------------------------
// The guard
// ---------------------------------------------------------------------------------------

/// True only for all three signals exactly as GitHub's hosted runner and the dedicated job
/// give them. Kept pure so every near miss can be tested on any machine.
fn is_throwaway_runner(
    github_actions: Option<&str>,
    runner_environment: Option<&str>,
    flag: Option<&str>,
) -> bool {
    github_actions == Some("true")
        && runner_environment == Some("github-hosted")
        && flag == Some("1")
}

/// Whether this run may change the machine. The first thing every test below does.
fn may_change_this_machine() -> TestResult<bool> {
    let read = |name: &str| std::env::var(name).ok();
    if !is_throwaway_runner(
        read("GITHUB_ACTIONS").as_deref(),
        read("RUNNER_ENVIRONMENT").as_deref(),
        read("WAM_ELEVATED_E2E").as_deref(),
    ) {
        println!(
            "skipping: this changes the machine, so it only runs on GitHub's hosted runner, in \
             the job that sets WAM_ELEVATED_E2E=1"
        );
        return Ok(false);
    }
    if !is_elevated() {
        return Err("the runner's signals are all set, but this process is not elevated".into());
    }
    Ok(true)
}

#[test]
fn only_a_github_hosted_runner_in_the_dedicated_job_may_run_this() {
    let cases = [
        (Some("true"), Some("github-hosted"), Some("1"), true),
        (None, None, None, false),
        (None, Some("github-hosted"), Some("1"), false),
        (Some("true"), None, Some("1"), false),
        (Some("true"), Some("github-hosted"), None, false),
        (Some("TRUE"), Some("github-hosted"), Some("1"), false),
        (Some("true"), Some("self-hosted"), Some("1"), false),
        (Some("true"), Some("github-hosted"), Some("0"), false),
        (Some("true"), Some("github-hosted"), Some("true"), false),
    ];
    for (actions, environment, flag, expected) in cases {
        assert_eq!(
            is_throwaway_runner(actions, environment, flag),
            expected,
            "{actions:?} {environment:?} {flag:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------
// Keeping score
// ---------------------------------------------------------------------------------------

/// Checks that are all made, then reported together.
#[derive(Default)]
struct Checks {
    failures: Vec<String>,
}

impl Checks {
    fn that(&mut self, label: &str, passed: bool, detail: impl Display) {
        if passed {
            println!("  ok: {label}");
        } else {
            println!("  FAILED: {label}: {detail}");
            self.failures.push(format!("{label}: {detail}"));
        }
    }

    fn same<T: PartialEq + Debug>(&mut self, label: &str, actual: T, expected: T) {
        let detail = format!("got {actual:?}, expected {expected:?}");
        self.that(label, actual == expected, detail);
    }

    fn finish(self) -> TestResult {
        if self.failures.is_empty() {
            return Ok(());
        }
        Err(format!(
            "{} checks failed:\n{}",
            self.failures.len(),
            self.failures.join("\n")
        )
        .into())
    }
}

fn section(title: &str) {
    println!("\n=== {title}");
}

// ---------------------------------------------------------------------------------------
// Windows' own tools
// ---------------------------------------------------------------------------------------

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn tool(program: &str, args: &[&str]) -> TestResult<Output> {
    Ok(Command::new(program).args(args).output()?)
}

/// What a tool said, whether it succeeded or not.
fn shown(program: &str, args: &[&str]) -> TestResult<String> {
    let output = tool(program, args)?;
    Ok(format!("{}{}", text(&output.stdout), text(&output.stderr)))
}

/// Run a tool that has to succeed.
fn must(program: &str, args: &[&str]) -> TestResult<String> {
    let output = tool(program, args)?;
    if output.status.success() {
        return Ok(text(&output.stdout));
    }
    Err(format!(
        "{program} {args:?} failed with {}: {}{}",
        output.status,
        text(&output.stdout),
        text(&output.stderr)
    )
    .into())
}

/// Windows PowerShell, with its own module path rather than the one PowerShell 7 hands down
/// in the CI step, which stops its own modules from loading.
fn powershell(script: &str) -> TestResult<String> {
    let script = format!("{POWERSHELL_UTF8_OUTPUT}$ErrorActionPreference = 'Stop'; {script}");
    let output = Command::new("powershell.exe")
        .env_remove("PSModulePath")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()?;
    if output.status.success() {
        return Ok(text(&output.stdout));
    }
    Err(format!(
        "PowerShell failed with {}: {}{}\nScript: {script}",
        output.status,
        text(&output.stdout),
        text(&output.stderr)
    )
    .into())
}

fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// A registry value's type and data, as `reg query` shows them. `None` means no such value.
fn reg_value(key: &str, name: Option<&str>) -> TestResult<Option<(String, String)>> {
    let mut args = vec!["query", key];
    match name {
        Some(name) => args.extend(["/v", name]),
        None => args.push("/ve"),
    }
    let output = tool("reg.exe", &args)?;
    if !output.status.success() {
        return Ok(None);
    }
    let shown = text(&output.stdout);
    let wanted = name.unwrap_or("(Default)");
    for line in shown.lines() {
        let Some(rest) = line.trim_start().strip_prefix(wanted) else {
            continue;
        };
        let rest = rest.trim_start();
        if !rest.starts_with("REG_") {
            continue;
        }
        let (kind, data) = rest.split_once("    ").unwrap_or((rest, ""));
        if data == "(value not set)" {
            return Ok(None);
        }
        return Ok(Some((kind.to_owned(), data.to_owned())));
    }
    Ok(None)
}

fn reg_key_exists(key: &str) -> TestResult<bool> {
    Ok(tool("reg.exe", &["query", key])?.status.success())
}

fn reg_add(key: &str, name: Option<&str>, kind: &str, data: &str) -> TestResult {
    let mut args = vec!["add", key];
    match name {
        Some(name) => args.extend(["/v", name]),
        None => args.push("/ve"),
    }
    args.extend(["/t", kind, "/d", data, "/f"]);
    must("reg.exe", &args).map(|_| ())
}

fn reg_delete_key(key: &str) {
    let _ = tool("reg.exe", &["delete", key, "/f"]);
}

fn reg_delete_value(key: &str, name: &str) {
    let _ = tool("reg.exe", &["delete", key, "/v", name, "/f"]);
}

/// The raw text of a value, for a before-and-after comparison.
fn reg_state(key: &str, name: Option<&str>) -> TestResult<String> {
    Ok(reg_value(key, name)?.map_or_else(
        || "absent".to_owned(),
        |(kind, data)| format!("{kind} {data}"),
    ))
}

/// What `sc.exe qc` says about a service. `None` means there is no such service.
fn sc_qc(name: &str) -> TestResult<Option<BTreeMap<String, String>>> {
    let output = tool("sc.exe", &["qc", name, "8192"])?;
    if !output.status.success() {
        return Ok(None);
    }
    let fields = text(&output.stdout)
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            let key = key.trim();
            let is_field = !key.is_empty()
                && key
                    .chars()
                    .all(|character| character.is_ascii_uppercase() || character == '_');
            is_field.then(|| (key.to_owned(), value.trim().to_owned()))
        })
        .collect();
    Ok(Some(fields))
}

fn sc_field(fields: &Option<BTreeMap<String, String>>, name: &str) -> String {
    fields
        .as_ref()
        .and_then(|fields| fields.get(name).cloned())
        .unwrap_or_default()
}

fn service_exists(name: &str) -> TestResult<bool> {
    Ok(sc_qc(name)?.is_some())
}

fn task_full_name(task_name: &str) -> String {
    format!("{TASK_FOLDER}{task_name}")
}

/// A task's definition, from `schtasks`. `None` means no such task.
fn task_xml(full_name: &str) -> TestResult<Option<String>> {
    let output = tool("schtasks.exe", &["/query", "/tn", full_name, "/xml"])?;
    Ok(output.status.success().then(|| text(&output.stdout)))
}

/// The SHA-256 of every file under `root`, by path below it. Read through a `\\?\` path,
/// because Windows PowerShell's own path handling takes `aux.h` for a device.
fn tree_hashes(root: &str) -> TestResult<BTreeMap<String, String>> {
    if !Path::new(root).exists() {
        return Ok(BTreeMap::new());
    }
    let shown = powershell(&format!(
        "$root = {}; $sha = [Security.Cryptography.SHA256]::Create(); \
         Get-ChildItem -LiteralPath $root -Recurse -File -Force | ForEach-Object {{ \
           $file = [IO.File]::OpenRead('\\\\?\\' + $_.FullName); \
           $hash = [BitConverter]::ToString($sha.ComputeHash($file)) -replace '-', ''; \
           $file.Close(); \
           '{{0}}|{{1}}' -f $_.FullName.Substring($root.Length + 1), $hash }}",
        quoted(root)
    ))?;
    Ok(shown
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(path, hash)| (path.to_owned(), hash.to_owned()))
        .collect())
}

// ---------------------------------------------------------------------------------------
// The fake app
// ---------------------------------------------------------------------------------------

fn app_exe(root: &str) -> String {
    format!(r"{root}\bin\{APP_NAME}.exe")
}

fn app_bin(root: &str) -> String {
    format!(r"{root}\bin")
}

fn app_conf(root: &str) -> String {
    format!(r"{root}\conf\app.conf")
}

/// The one program, built once for all the tests that follow.
fn fake_app_program() -> TestResult<PathBuf> {
    let folder = std::env::temp_dir().join("wam-e2e-fake-app");
    let program = folder.join("fake-app.exe");
    if program.is_file() {
        return Ok(program);
    }
    fs::create_dir_all(&folder)?;
    let source = folder.join("fake_app.rs");
    fs::write(&source, FAKE_APP_SOURCE)?;
    let (source, program_text) = (source.display().to_string(), program.display().to_string());
    must(
        "rustc",
        &[
            "--edition",
            "2021",
            "-C",
            "target-feature=-crt-static",
            "-o",
            &program_text,
            &source,
        ],
    )?;
    Ok(program)
}

fn set_up_marker_folder() -> TestResult {
    fs::create_dir_all(MARKER_DIR)?;
    // LocalService, NetworkService, and virtual accounts all have to be able to write here.
    must(
        "icacls",
        &[MARKER_DIR, "/grant", "*S-1-1-0:(OI)(CI)M", "/T", "/Q"],
    )
    .map(|_| ())
}

/// The fields of a marker the fake app wrote, once it appears.
fn wait_for_marker(name: &str, patience: Duration) -> Option<BTreeMap<String, String>> {
    let path = Path::new(MARKER_DIR).join(format!("{name}.txt"));
    let started = Instant::now();
    while started.elapsed() < patience {
        if let Ok(content) = fs::read_to_string(&path)
            && content.contains("probe=")
        {
            return Some(
                content
                    .lines()
                    .filter_map(|line| line.split_once('='))
                    .map(|(key, value)| (key.to_owned(), value.to_owned()))
                    .collect(),
            );
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    None
}

fn remove_marker(name: &str) {
    let _ = fs::remove_file(Path::new(MARKER_DIR).join(format!("{name}.txt")));
}

fn marker_field(marker: &Option<BTreeMap<String, String>>, name: &str) -> String {
    marker
        .as_ref()
        .and_then(|fields| fields.get(name).cloned())
        .unwrap_or_default()
}

/// A registry value the fake app has, and what it should say for an install at a location.
struct ExpectedValue {
    key: String,
    name: Option<&'static str>,
    kind: &'static str,
    data: String,
}

fn value(key: &str, name: Option<&'static str>, kind: &'static str, data: String) -> ExpectedValue {
    ExpectedValue {
        key: key.to_owned(),
        name,
        kind,
        data,
    }
}

fn expected_registry(root: &str) -> Vec<ExpectedValue> {
    let exe = app_exe(root);
    let conf = app_conf(root);
    let plugins = format!(r"{root}\plugins\a\0{root}\plugins\b");
    let parameters = format!(r"{SERVICES_KEY}\WamE2ELocal\Parameters");
    let inproc_server = format!(r"{CLSID_KEY}\InprocServer32");
    vec![
        value(HKLM_APP_KEY, Some("InstallDir"), "REG_SZ", root.to_owned()),
        value(
            HKLM_APP_KEY,
            Some("LogDir"),
            "REG_EXPAND_SZ",
            r"%ProgramData%\WAM E2E App\logs".to_owned(),
        ),
        value(
            HKLM_APP_KEY,
            Some("ApiToken"),
            "REG_SZ",
            SECRET_IN_REGISTRY.to_owned(),
        ),
        value(HKLM_APP_KEY, Some("Count"), "REG_DWORD", "0x2a".to_owned()),
        value(
            HKLM_APP_KEY,
            Some("Big"),
            "REG_QWORD",
            "0x1122334455".to_owned(),
        ),
        value(
            HKLM_APP_KEY,
            Some("Blob"),
            "REG_BINARY",
            "DEADBEEF".to_owned(),
        ),
        value(
            HKLM_APP_KEY,
            Some("Plugins"),
            "REG_MULTI_SZ",
            plugins.clone(),
        ),
        value(
            &format!(r"{HKLM_APP_KEY}\Sub\Deep"),
            None,
            "REG_SZ",
            conf.clone(),
        ),
        value(HKLM_WOW_KEY, Some("InstallDir"), "REG_SZ", root.to_owned()),
        value(
            HKCU_APP_KEY,
            Some("LastFolder"),
            "REG_SZ",
            format!(r"{root}\plugins"),
        ),
        value(HKCU_APP_KEY, Some("Count"), "REG_DWORD", "0x7".to_owned()),
        value(
            UNINSTALL_KEY,
            Some("DisplayName"),
            "REG_SZ",
            APP_NAME.to_owned(),
        ),
        value(
            UNINSTALL_KEY,
            Some("Publisher"),
            "REG_SZ",
            VENDOR.to_owned(),
        ),
        value(
            UNINSTALL_KEY,
            Some("DisplayVersion"),
            "REG_SZ",
            "1.2.3".to_owned(),
        ),
        value(
            UNINSTALL_KEY,
            Some("InstallLocation"),
            "REG_SZ",
            root.to_owned(),
        ),
        value(
            UNINSTALL_KEY,
            Some("DisplayIcon"),
            "REG_SZ",
            format!("{exe},0"),
        ),
        value(
            UNINSTALL_KEY,
            Some("EstimatedSize"),
            "REG_DWORD",
            "0x10".to_owned(),
        ),
        value(
            HKLM_RUN_KEY,
            Some(APP_NAME),
            "REG_SZ",
            format!("\"{exe}\" --tray"),
        ),
        // Written without quotes on the source. Restoring has to add them, because a Run
        // value is a command line and the path has spaces.
        value(
            HKCU_RUN_KEY,
            Some(APP_NAME),
            "REG_SZ",
            format!("\"{exe}\" --tray"),
        ),
        value(APP_PATHS_KEY, None, "REG_SZ", exe.clone()),
        value(APP_PATHS_KEY, Some("Path"), "REG_SZ", app_bin(root)),
        value(
            &inproc_server,
            None,
            "REG_SZ",
            format!(r"{root}\bin\WAM E2E Shell.dll"),
        ),
        value(
            &inproc_server,
            Some("ThreadingModel"),
            "REG_SZ",
            "Apartment".to_owned(),
        ),
        value(
            HKCU_EXTENSION_KEY,
            None,
            "REG_SZ",
            "WamE2E.Document".to_owned(),
        ),
        value(
            &format!(r"{HKCU_PROG_ID_KEY}\shell\open\command"),
            None,
            "REG_SZ",
            format!("\"{exe}\" \"%1\""),
        ),
        value(
            MACHINE_ENVIRONMENT_KEY,
            Some("WAM_E2E_HOME"),
            "REG_SZ",
            root.to_owned(),
        ),
        value(
            MACHINE_ENVIRONMENT_KEY,
            Some("WAM_E2E_TOKEN"),
            "REG_SZ",
            format!(r"{root}\{SECRET_IN_ENVIRONMENT}"),
        ),
        value(
            USER_ENVIRONMENT_KEY,
            Some("WAM_E2E_USER_HOME"),
            "REG_SZ",
            format!(r"{root}\conf"),
        ),
        value(&parameters, Some("ConfigFile"), "REG_SZ", conf),
        value(&parameters, Some("Retries"), "REG_DWORD", "0x3".to_owned()),
        value(&parameters, Some("Folders"), "REG_MULTI_SZ", plugins),
        value(
            &format!(r"{parameters}\Deep"),
            Some("Level"),
            "REG_SZ",
            "high".to_owned(),
        ),
    ]
}

/// One of the fake app's services.
struct ServiceSpec {
    name: &'static str,
    marker: &'static str,
    display_name: &'static str,
    /// What `sc.exe` is told on the source.
    source_start: &'static str,
    source_object: Option<&'static str>,
    /// What `sc.exe qc` should say once restored.
    expected_start: &'static str,
    expected_object: &'static str,
    /// The start of the SID it really runs as, if it can be started at all.
    expected_sid: Option<&'static str>,
}

const SERVICES: [ServiceSpec; 5] = [
    ServiceSpec {
        name: "WamE2ELocal",
        marker: "service-local",
        display_name: "WAM E2E Local Service",
        source_start: "demand",
        source_object: Some(r"NT AUTHORITY\LocalService"),
        expected_start: "DEMAND_START",
        expected_object: r"NT AUTHORITY\LocalService",
        expected_sid: Some("S-1-5-19"),
    },
    ServiceSpec {
        name: "WamE2ENetwork",
        marker: "service-network",
        display_name: "WAM E2E Network Service",
        source_start: "delayed-auto",
        source_object: Some(r"NT AUTHORITY\NetworkService"),
        expected_start: "AUTO_START  (DELAYED)",
        expected_object: r"NT AUTHORITY\NetworkService",
        expected_sid: Some("S-1-5-20"),
    },
    ServiceSpec {
        name: "WamE2ESystem",
        marker: "service-system",
        display_name: "WAM E2E System Service",
        source_start: "demand",
        source_object: Some("LocalSystem"),
        expected_start: "DEMAND_START",
        expected_object: "LocalSystem",
        expected_sid: Some("S-1-5-18"),
    },
    // Windows gives each service its own `NT SERVICE\<name>` account.
    ServiceSpec {
        name: "WamE2EVirtual",
        marker: "service-virtual",
        display_name: "WAM E2E Virtual Account Service",
        source_start: "demand",
        source_object: Some(r"NT SERVICE\WamE2EVirtual"),
        expected_start: "DEMAND_START",
        expected_object: r"NT SERVICE\WamE2EVirtual",
        expected_sid: Some("S-1-5-80-"),
    },
    // Its account's password cannot be moved, so it comes back as LocalService.
    ServiceSpec {
        name: "WamE2ECustom",
        marker: "service-custom",
        display_name: "WAM E2E Custom Account Service",
        source_start: "disabled",
        source_object: None,
        expected_start: "DISABLED",
        expected_object: r"NT AUTHORITY\LocalService",
        expected_sid: None,
    },
];

const CUSTOM_SOURCE_ACCOUNT: &str = r"WAM-E2E\Somebody";

fn service_image_path(root: &str, marker: &str) -> String {
    format!(
        "\"{}\" --marker {marker} --config \"{}\"",
        app_exe(root),
        app_conf(root)
    )
}

/// Files of the app by path below its install location.
fn app_files() -> Vec<(&'static str, &'static [u8])> {
    vec![
        (r"conf\app.conf", b"setting = 1\r\n".as_slice()),
        (r"conf\readonly.dat", b"an earlier install left this\r\n"),
        (r"plugins\a\plugin.txt", b"a\r\n"),
        (r"plugins\Größe.txt", b"not ASCII\r\n"),
        // A device's name with an extension is an ordinary file on this version of Windows.
        (r"include\aux.h", b"// not a device\r\n"),
    ]
}

/// Where the fake app's shortcuts are, and what each holds for an install at a location.
struct ShortcutSpec {
    path: String,
    arguments: String,
    working_directory: String,
    icon: String,
}

struct ShellFolders {
    desktop: String,
    common_desktop: String,
    start_menu: String,
    common_start_menu: String,
    startup: String,
}

fn shell_folders() -> TestResult<ShellFolders> {
    let shown = powershell(
        "'Desktop', 'CommonDesktopDirectory', 'StartMenu', 'CommonStartMenu', 'Startup' | \
         ForEach-Object { [Environment]::GetFolderPath($_) }",
    )?;
    let mut lines = shown.lines().map(str::trim);
    let mut next = || -> TestResult<String> {
        Ok(lines
            .next()
            .filter(|line| !line.is_empty())
            .ok_or("a shell folder is missing")?
            .to_owned())
    };
    Ok(ShellFolders {
        desktop: next()?,
        common_desktop: next()?,
        start_menu: next()?,
        common_start_menu: next()?,
        startup: next()?,
    })
}

fn shortcut_specs(folders: &ShellFolders, root: &str) -> Vec<ShortcutSpec> {
    let exe = app_exe(root);
    vec![
        ShortcutSpec {
            path: format!(r"{}\{APP_NAME}.lnk", folders.desktop),
            arguments: format!("--open \"{}\"", app_conf(root)),
            working_directory: app_bin(root),
            icon: format!("{exe},0"),
        },
        // With nothing but a target, the shape many installers write.
        ShortcutSpec {
            path: format!(r"{}\{APP_NAME} Everyone.lnk", folders.common_desktop),
            arguments: String::new(),
            working_directory: String::new(),
            icon: ",0".to_owned(),
        },
        ShortcutSpec {
            path: format!(
                r"{}\Programs\{APP_NAME}\{APP_NAME} Größe.lnk",
                folders.start_menu
            ),
            arguments: "--from-start-menu".to_owned(),
            working_directory: root.to_owned(),
            icon: format!(r"{root}\bin\WAM E2E Shell.dll,1"),
        },
        ShortcutSpec {
            path: format!(
                r"{}\Programs\{APP_NAME}\{APP_NAME}.lnk",
                folders.common_start_menu
            ),
            arguments: String::new(),
            working_directory: app_bin(root),
            icon: ",0".to_owned(),
        },
        ShortcutSpec {
            path: format!(r"{}\{APP_NAME} Startup.lnk", folders.startup),
            arguments: "--tray".to_owned(),
            working_directory: String::new(),
            icon: ",0".to_owned(),
        },
    ]
}

fn create_shortcut(spec: &ShortcutSpec, target: &str) -> TestResult {
    if let Some(parent) = Path::new(&spec.path).parent() {
        fs::create_dir_all(parent)?;
    }
    powershell(&format!(
        "$link = (New-Object -ComObject WScript.Shell).CreateShortcut({}); \
         $link.TargetPath = {}; $link.Arguments = {}; $link.WorkingDirectory = {}; \
         $link.IconLocation = {}; $link.Save()",
        quoted(&spec.path),
        quoted(target),
        quoted(&spec.arguments),
        quoted(&spec.working_directory),
        quoted(&spec.icon),
    ))
    .map(|_| ())
}

/// Target, arguments, working folder, and icon, as read back through the shell.
fn read_shortcut(path: &str) -> TestResult<Option<[String; 4]>> {
    if !Path::new(path).is_file() {
        return Ok(None);
    }
    let shown = powershell(&format!(
        "$link = (New-Object -ComObject WScript.Shell).CreateShortcut({}); \
         $link.TargetPath; '|'; $link.Arguments; '|'; $link.WorkingDirectory; '|'; \
         $link.IconLocation",
        quoted(path)
    ))?;
    let mut parts = shown
        .split('|')
        .map(|part| part.trim().to_owned())
        .collect::<Vec<_>>()
        .into_iter();
    let mut next = || parts.next().unwrap_or_default();
    Ok(Some([next(), next(), next(), next()]))
}

fn path_with(base: &str, additions: &[&str]) -> String {
    let mut path = base.trim_end_matches(';').to_owned();
    for addition in additions {
        if !path.is_empty() {
            path.push(';');
        }
        path.push_str(addition);
    }
    path
}

/// What the `Path` values were before the fake app touched them.
struct OriginalPaths {
    machine_kind: String,
    machine: String,
    user_kind: String,
    user: String,
}

fn read_original_paths() -> TestResult<OriginalPaths> {
    let (machine_kind, machine) = reg_value(MACHINE_ENVIRONMENT_KEY, Some("Path"))?
        .ok_or("this PC's environment has no machine Path")?;
    let (user_kind, user) = reg_value(USER_ENVIRONMENT_KEY, Some("Path"))?
        .unwrap_or_else(|| ("REG_EXPAND_SZ".to_owned(), String::new()));
    Ok(OriginalPaths {
        machine_kind,
        machine,
        user_kind,
        user,
    })
}

fn write_paths(paths: &OriginalPaths, machine: &str, user: &str) -> TestResult {
    reg_add(
        MACHINE_ENVIRONMENT_KEY,
        Some("Path"),
        &paths.machine_kind,
        machine,
    )?;
    if user.is_empty() {
        reg_delete_value(USER_ENVIRONMENT_KEY, "Path");
        Ok(())
    } else {
        reg_add(USER_ENVIRONMENT_KEY, Some("Path"), &paths.user_kind, user)
    }
}

/// Register the app's tasks. One names a quoted command, as many installers do, and runs
/// as SYSTEM, so it can be started here. The other runs as the current user with two
/// programs, which Windows allows.
fn create_tasks(root: &str) -> TestResult {
    let exe = app_exe(root);
    let conf = app_conf(root);
    let arguments = format!("--marker task-system --config \"{conf}\"");
    powershell(&format!(
        "$action = New-ScheduledTaskAction -Execute {} -Argument {} -WorkingDirectory {}; \
         $trigger = New-ScheduledTaskTrigger -Once -At '2099-01-01T00:00:00'; \
         $principal = New-ScheduledTaskPrincipal -UserId 'S-1-5-18' -LogonType ServiceAccount -RunLevel Highest; \
         Register-ScheduledTask -TaskName {} -TaskPath {} -Action $action -Trigger $trigger \
           -Principal $principal | Out-Null",
        quoted(&format!("\"{exe}\"")),
        quoted(&arguments),
        quoted(&app_bin(root)),
        quoted(SYSTEM_TASK),
        quoted(TASK_FOLDER),
    ))?;
    powershell(&format!(
        "$first = New-ScheduledTaskAction -Execute {exe} -Argument '--marker task-first'; \
         $second = New-ScheduledTaskAction -Execute {exe} -Argument '--marker task-second'; \
         $trigger = New-ScheduledTaskTrigger -Once -At '2099-01-01T00:00:00'; \
         $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive; \
         Register-ScheduledTask -TaskName {} -TaskPath {} -Action $first,$second \
           -Trigger $trigger -Principal $principal | Out-Null",
        quoted(USER_TASK),
        quoted(TASK_FOLDER),
        exe = quoted(&exe),
    ))
    .map(|_| ())
}

fn create_services(root: &str) -> TestResult {
    for service in &SERVICES {
        let image_path = service_image_path(root, service.marker);
        let mut args = vec![
            "create",
            service.name,
            "binPath=",
            image_path.as_str(),
            "start=",
            service.source_start,
            "DisplayName=",
            service.display_name,
        ];
        // A virtual account is set afterward below. Restoring asks for it in `create`, so the
        // source is not set up in the way that is being tested.
        match service.source_object {
            Some(object) if object.starts_with("NT AUTHORITY") => {
                args.extend(["obj=", object, "password=", ""]);
            }
            Some("LocalSystem") => args.extend(["obj=", "LocalSystem"]),
            _ => {}
        }
        must("sc.exe", &args)?;
        if let Some(object) = service
            .source_object
            .filter(|object| object.starts_with("NT SERVICE"))
        {
            must("sc.exe", &["config", service.name, "obj=", object])?;
        }
    }
    must(
        "sc.exe",
        &[
            "description",
            "WamE2ELocal",
            "A harmless service for the elevated round trip",
        ],
    )?;
    must("sc.exe", &["config", "WamE2ELocal", "depend=", "RpcSs"])?;
    // Only what a scan reads. Windows is never asked to log on as this.
    reg_add(
        &format!(r"{SERVICES_KEY}\WamE2ECustom"),
        Some("ObjectName"),
        "REG_SZ",
        CUSTOM_SOURCE_ACCOUNT,
    )?;
    let driver_path = format!(r"{root}\drivers\wame2e.sys");
    must(
        "sc.exe",
        &[
            "create",
            DRIVER_NAME,
            "type=",
            "kernel",
            "start=",
            "demand",
            "binPath=",
            &driver_path,
        ],
    )
    .map(|_| ())
}

/// Lay out the whole app on this machine, using only Windows' own tools.
fn create_fake_app(
    program: &Path,
    folders: &ShellFolders,
    paths: &OriginalPaths,
    data: &[PathBuf],
) -> TestResult {
    let root = SOURCE_ROOT;
    fs::create_dir_all(app_bin(root))?;
    fs::create_dir_all(format!(r"{root}\drivers"))?;
    fs::copy(program, app_exe(root))?;
    fs::copy(program, format!(r"{root}\bin\WAM E2E Shell.dll"))?;
    fs::write(
        format!(r"{root}\drivers\wame2e.sys"),
        b"WAM E2E: not a driver, and never loaded\r\n",
    )?;
    for (relative, content) in app_files() {
        let path = Path::new(root).join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    // A rename does not replace a read-only file, which restoring with --force has to.
    let readonly = format!(r"{root}\conf\readonly.dat");
    let mut permissions = fs::metadata(&readonly)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&readonly, permissions)?;
    for path in data {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, b"the app's data\r\n")?;
    }

    create_services(root)?;
    for expected in expected_registry(root) {
        reg_add(&expected.key, expected.name, expected.kind, &expected.data)?;
    }
    // What the source says about removing it from the source, which must not survive.
    for (name, data) in [
        (
            "UninstallString",
            format!("\"{}\" --uninstall", app_exe(root)),
        ),
        (
            "QuietUninstallString",
            format!("\"{}\" --uninstall /S", app_exe(root)),
        ),
        ("ModifyPath", format!("\"{}\" --modify", app_exe(root))),
    ] {
        reg_add(UNINSTALL_KEY, Some(name), "REG_SZ", &data)?;
    }
    // On the source this one is not quoted. See `expected_registry`.
    reg_add(
        HKCU_RUN_KEY,
        Some(APP_NAME),
        "REG_SZ",
        &format!("{} --tray", app_exe(root)),
    )?;
    must("reg.exe", &["add", &format!(r"{HKLM_APP_KEY}\Empty"), "/f"])?;
    write_paths(
        paths,
        &path_with(&paths.machine, &[OTHER_PATH_ENTRY, &app_bin(root)]),
        &path_with(&paths.user, &[&app_bin(root)]),
    )?;
    create_tasks(root)?;
    for spec in shortcut_specs(folders, root) {
        create_shortcut(&spec, &app_exe(root))?;
    }
    Ok(())
}

/// Remove everything [`create_fake_app`] set up, or an install of it at `root`.
fn remove_fake_app(
    root: &str,
    folders: &ShellFolders,
    paths: &OriginalPaths,
    data: &[PathBuf],
) -> TestResult {
    for service in &SERVICES {
        must("sc.exe", &["delete", service.name])?;
    }
    must("sc.exe", &["delete", DRIVER_NAME])?;
    for key in [
        HKLM_APP_KEY,
        HKLM_WOW_KEY,
        HKCU_APP_KEY,
        UNINSTALL_KEY,
        APP_PATHS_KEY,
        CLSID_KEY,
        HKCU_EXTENSION_KEY,
        HKCU_PROG_ID_KEY,
    ] {
        reg_delete_key(key);
    }
    for key in [
        r"HKLM\Software\WAM E2E Vendor",
        r"HKLM\Software\WOW6432Node\WAM E2E Vendor",
        r"HKCU\Software\WAM E2E Vendor",
    ] {
        reg_delete_key(key);
    }
    reg_delete_value(HKLM_RUN_KEY, APP_NAME);
    reg_delete_value(HKCU_RUN_KEY, APP_NAME);
    reg_delete_value(MACHINE_ENVIRONMENT_KEY, "WAM_E2E_HOME");
    reg_delete_value(MACHINE_ENVIRONMENT_KEY, "WAM_E2E_TOKEN");
    reg_delete_value(USER_ENVIRONMENT_KEY, "WAM_E2E_USER_HOME");
    write_paths(paths, &paths.machine, &paths.user)?;
    for task in [SYSTEM_TASK, USER_TASK] {
        must(
            "schtasks.exe",
            &["/delete", "/tn", &task_full_name(task), "/f"],
        )?;
    }
    delete_task_folder()?;
    for spec in shortcut_specs(folders, root) {
        let _ = fs::remove_file(&spec.path);
    }
    let _ = fs::remove_dir(format!(r"{}\Programs\{APP_NAME}", folders.start_menu));
    let _ = fs::remove_dir(format!(
        r"{}\Programs\{APP_NAME}",
        folders.common_start_menu
    ));
    fs::remove_dir_all(root)?;
    for path in data {
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
    Ok(())
}

/// Windows makes the folder a task goes in and never takes it away.
fn delete_task_folder() -> TestResult {
    powershell(&format!(
        "$s = New-Object -ComObject Schedule.Service; $s.Connect(); \
         $s.GetFolder('\\').DeleteFolder({}, 0)",
        quoted(TASK_FOLDER.trim_end_matches('\\'))
    ))
    .map(|_| ())
}

fn check_is_gone(
    checks: &mut Checks,
    root: &str,
    folders: &ShellFolders,
    paths: &OriginalPaths,
    data: &[PathBuf],
) -> TestResult {
    checks.that(
        "the install folder is gone",
        !Path::new(root).exists(),
        root,
    );
    for service in &SERVICES {
        checks.that(
            &format!("sc.exe does not know {}", service.name),
            !service_exists(service.name)?,
            "it still exists",
        );
    }
    checks.that(
        "sc.exe does not know the driver",
        !service_exists(DRIVER_NAME)?,
        "it still exists",
    );
    for expected in expected_registry(root) {
        checks.same(
            &format!(
                "reg.exe has no {}\\{}",
                expected.key,
                expected.name.unwrap_or("(Default)")
            ),
            reg_value(&expected.key, expected.name)?,
            None,
        );
    }
    checks.same(
        "the machine Path is back to what it was",
        reg_value(MACHINE_ENVIRONMENT_KEY, Some("Path"))?.map(|(_, data)| data),
        Some(paths.machine.clone()),
    );
    for task in [SYSTEM_TASK, USER_TASK] {
        checks.that(
            &format!("schtasks.exe does not know {task}"),
            task_xml(&task_full_name(task))?.is_none(),
            "it still exists",
        );
    }
    for spec in shortcut_specs(folders, root) {
        checks.that(
            &format!("no shortcut at {}", spec.path),
            !Path::new(&spec.path).exists(),
            "it still exists",
        );
    }
    for path in data {
        checks.that(
            &format!("no data at {}", path.display()),
            !path.exists(),
            "it still exists",
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Running the real installer
// ---------------------------------------------------------------------------------------

struct InstallerRun {
    success: bool,
    output: String,
}

fn run_installer(installer: &Path, args: &[&str]) -> TestResult<InstallerRun> {
    println!("\n--- {} {}", installer.display(), args.join(" "));
    let output = Command::new(installer).args(args).output()?;
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    println!("{shown}--- exit: {}", output.status);
    Ok(InstallerRun {
        success: output.status.success(),
        output: shown,
    })
}

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

// ---------------------------------------------------------------------------------------
// Comparing the machine before and after
// ---------------------------------------------------------------------------------------

/// How the machine stands, as Windows' own tools say, for everything the fake app touches.
type Snapshot = BTreeMap<String, String>;

/// Every file and folder under `root`, with its hash, attributes, and permissions.
fn tree_details(root: &str) -> TestResult<BTreeMap<String, String>> {
    if !Path::new(root).exists() {
        return Ok(BTreeMap::new());
    }
    let shown = powershell(&format!(
        "$root = {}; $sha = [Security.Cryptography.SHA256]::Create(); \
         Get-ChildItem -LiteralPath $root -Recurse -Force | ForEach-Object {{ \
           $name = $_.FullName.Substring($root.Length + 1); \
           $access = try {{ $_.GetAccessControl('Access').Sddl }} catch {{ 'unreadable' }}; \
           if ($_.PSIsContainer) {{ '{{0}}|folder|{{1}}' -f $name, $access }} else {{ \
             $file = [IO.File]::OpenRead('\\\\?\\' + $_.FullName); \
             $hash = [BitConverter]::ToString($sha.ComputeHash($file)) -replace '-', ''; \
             $file.Close(); \
             '{{0}}|{{1}}|{{2}}|{{3}}' -f $name, $hash, ($_.Attributes -band 7), $access }} }}",
        quoted(root)
    ))?;
    Ok(shown
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(path, details)| (path.to_owned(), details.to_owned()))
        .collect())
}

fn whole_key(key: &str) -> TestResult<String> {
    let output = tool("reg.exe", &["query", key, "/s"])?;
    Ok(if output.status.success() {
        text(&output.stdout)
    } else {
        "absent".to_owned()
    })
}

fn task_folder_exists(path: &str) -> TestResult<bool> {
    let shown = powershell(&format!(
        "$s = New-Object -ComObject Schedule.Service; $s.Connect(); \
         try {{ $null = $s.GetFolder({}); 'yes' }} catch {{ 'no' }}",
        quoted(path)
    ))?;
    Ok(shown.trim() == "yes")
}

/// Everything the fake app, installed at `root`, can have touched.
fn machine_snapshot(root: &str, folders: &ShellFolders, data: &[PathBuf]) -> TestResult<Snapshot> {
    let mut snapshot = Snapshot::new();
    snapshot.insert(
        "install folder exists".to_owned(),
        Path::new(root).exists().to_string(),
    );
    for (path, details) in tree_details(root)? {
        snapshot.insert(format!(r"file {path}"), details);
    }
    let parameters = format!(r"{SERVICES_KEY}\WamE2ELocal\Parameters");
    for key in [
        HKLM_APP_KEY,
        r"HKLM\Software\WAM E2E Vendor",
        HKLM_WOW_KEY,
        r"HKLM\Software\WOW6432Node\WAM E2E Vendor",
        HKCU_APP_KEY,
        r"HKCU\Software\WAM E2E Vendor",
        UNINSTALL_KEY,
        APP_PATHS_KEY,
        CLSID_KEY,
        HKCU_EXTENSION_KEY,
        HKCU_PROG_ID_KEY,
        &parameters,
    ] {
        snapshot.insert(format!("registry key {key}"), whole_key(key)?);
    }
    for (key, name) in [
        (HKLM_RUN_KEY, APP_NAME),
        (HKCU_RUN_KEY, APP_NAME),
        (MACHINE_ENVIRONMENT_KEY, "WAM_E2E_HOME"),
        (MACHINE_ENVIRONMENT_KEY, "WAM_E2E_TOKEN"),
        (MACHINE_ENVIRONMENT_KEY, "Path"),
        (USER_ENVIRONMENT_KEY, "WAM_E2E_USER_HOME"),
        (USER_ENVIRONMENT_KEY, "Path"),
    ] {
        snapshot.insert(format!(r"value {key}\{name}"), reg_state(key, Some(name))?);
    }
    for name in SERVICES
        .iter()
        .map(|service| service.name)
        .chain([DRIVER_NAME])
    {
        snapshot.insert(format!("sc.exe qc {name}"), format!("{:?}", sc_qc(name)?));
        snapshot.insert(
            format!("sc.exe qdescription {name}"),
            shown("sc.exe", &["qdescription", name, "8192"])?,
        );
        snapshot.insert(
            format!("delayed start of {name}"),
            reg_state(&format!(r"{SERVICES_KEY}\{name}"), Some("DelayedAutostart"))?,
        );
    }
    for task in [SYSTEM_TASK, USER_TASK] {
        snapshot.insert(
            format!("schtasks.exe {task}"),
            format!("{:?}", task_xml(&task_full_name(task))?),
        );
    }
    snapshot.insert(
        "the task folder".to_owned(),
        task_folder_exists(TASK_FOLDER.trim_end_matches('\\'))?.to_string(),
    );
    for spec in shortcut_specs(folders, root) {
        snapshot.insert(
            format!("shortcut {}", spec.path),
            format!("{:?}", read_shortcut(&spec.path)?),
        );
    }
    for folder in [&folders.start_menu, &folders.common_start_menu] {
        let path = format!(r"{folder}\Programs\{APP_NAME}");
        snapshot.insert(
            format!("folder {path}"),
            Path::new(&path).exists().to_string(),
        );
    }
    for path in data {
        snapshot.insert(
            format!("data {}", path.display()),
            sha256_of_file_if_there(path)?,
        );
    }
    Ok(snapshot)
}

fn sha256_of_file_if_there(path: &Path) -> TestResult<String> {
    Ok(if path.is_file() {
        sha256_file(path)?
    } else {
        "absent".to_owned()
    })
}

/// That two snapshots agree, other than for what is named in `except`.
fn check_same_state(
    checks: &mut Checks,
    label: &str,
    actual: &Snapshot,
    expected: &Snapshot,
    except: &[String],
) {
    let keys: std::collections::BTreeSet<&String> = actual.keys().chain(expected.keys()).collect();
    let different: Vec<String> = keys
        .into_iter()
        .filter(|key| {
            !except
                .iter()
                .any(|excepted| key.starts_with(excepted.as_str()))
        })
        .filter(|key| actual.get(*key) != expected.get(*key))
        .map(|key| {
            format!(
                "{key}\n    was: {:?}\n    now: {:?}",
                expected.get(key),
                actual.get(key)
            )
        })
        .collect();
    checks.that(label, different.is_empty(), different.join("\n"));
}

fn installs_on_record() -> TestResult<Vec<String>> {
    if !Path::new(INSTALLS_DIR).is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = fs::read_dir(INSTALLS_DIR)?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    Ok(names)
}

/// What Windows does with a value like `UninstallString`: hands the whole line to
/// `CreateProcess`, which is told what to run by the quoted path at the start of it.
fn command_as_windows_would_run_it(command_line: &str) -> TestResult<Command> {
    let rest = command_line
        .strip_prefix('"')
        .ok_or("the command does not start with a quoted path")?;
    let (program, arguments) = rest.split_once('"').ok_or("the quote is never closed")?;
    let mut command = Command::new(program);
    command.raw_arg(arguments.trim_start());
    Ok(command)
}

fn run_as_windows_would(command_line: &str) -> TestResult<InstallerRun> {
    println!("\n--- as Windows would: {command_line}");
    let output = command_as_windows_would_run_it(command_line)?.output()?;
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    println!("{shown}--- exit: {}", output.status);
    Ok(InstallerRun {
        success: output.status.success(),
        output: shown,
    })
}

/// Run an install's uninstaller with no window, as Windows would be told to, with anything
/// extra added, and wait for its folder to go.
fn uninstall_quietly(checks: &mut Checks, id: &str, extra: &str) -> TestResult<InstallerRun> {
    let command =
        format!("\"{INSTALLS_DIR}\\{id}\\uninstall.exe\" --uninstall {id} --quiet{extra}");
    let run = run_as_windows_would(&command)?;
    let folder = Path::new(INSTALLS_DIR).join(id);
    let started = Instant::now();
    while folder.exists() && run.success && started.elapsed() < SELF_REMOVAL_PATIENCE {
        std::thread::sleep(Duration::from_millis(250));
    }
    if run.success {
        checks.that(
            "the uninstaller's folder went once it had exited",
            !folder.exists(),
            folder.display(),
        );
    }
    Ok(run)
}

fn install_id_in(output: &str) -> TestResult<String> {
    Ok(output
        .lines()
        .find_map(|line| line.strip_prefix("Install id: "))
        .ok_or("the install did not say its id")?
        .trim()
        .to_owned())
}

/// The newest folder on record that was not there before.
fn new_install_folder(before: &[String]) -> TestResult<Option<PathBuf>> {
    Ok(installs_on_record()?
        .into_iter()
        .find(|name| !before.contains(name))
        .map(|name| Path::new(INSTALLS_DIR).join(name)))
}

/// Start an installer, wait for its journal to say it has got as far as `marker`, and kill it,
/// as a power cut or a crash would stop it.
fn kill_installer_at(
    installer: &Path,
    args: &[&str],
    marker: &str,
    checks: &mut Checks,
) -> TestResult<Option<PathBuf>> {
    let before = installs_on_record()?;
    println!(
        "\n--- {} {}, killed at {marker}",
        installer.display(),
        args.join(" ")
    );
    let mut child = Command::new(installer)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let started = Instant::now();
    let mut reached = None;
    while started.elapsed() < Duration::from_secs(300) {
        if let Some(folder) = new_install_folder(&before)?
            && fs::read_to_string(folder.join("journal.jsonl"))
                .is_ok_and(|journal| journal.contains(marker))
        {
            reached = Some(folder);
            break;
        }
        if child.try_wait()?.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    child.kill()?;
    child.wait()?;
    checks.that(
        &format!("the installer got as far as {marker} before it was killed"),
        reached.is_some(),
        "it finished or stopped first",
    );
    if let Some(folder) = reached.as_ref() {
        let journal = fs::read_to_string(folder.join("journal.jsonl"))?;
        checks.that(
            "its journal does not say it finished",
            !journal.contains("committed"),
            "it says it finished",
        );
    }
    Ok(reached)
}

// ---------------------------------------------------------------------------------------
// The round trip
// ---------------------------------------------------------------------------------------

/// Everything that should be true of a restored app, asked of Windows' own tools.
fn check_restored(
    checks: &mut Checks,
    root: &str,
    file_hashes: &BTreeMap<String, String>,
    folders: &ShellFolders,
    paths: &OriginalPaths,
    data: &[PathBuf],
) -> TestResult {
    let actual_hashes = tree_hashes(root)?;
    checks.same(
        "SHA-256 from .NET: exactly the app's files, with the same contents",
        &actual_hashes,
        file_hashes,
    );
    let access = shown("icacls", &[&app_exe(root)])?;
    checks.that(
        "icacls: the installed program inherited its folder's permissions, not staging's",
        access.contains(r"BUILTIN\Users:(I)(RX)"),
        &access,
    );
    let staging_left = powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Filter 'winappmigrator-install-*' -Force | \
         ForEach-Object {{ $_.FullName }}",
        quoted(PROGRAM_FILES)
    ))?;
    checks.same(
        "no staging folder is left in Program Files",
        staging_left.trim(),
        "",
    );
    for path in data {
        checks.that(
            &format!("the data file is at its real location: {}", path.display()),
            path.is_file(),
            "it is missing",
        );
    }

    for expected in expected_registry(root) {
        checks.same(
            &format!(
                "reg.exe: {}\\{}",
                expected.key,
                expected.name.unwrap_or("(Default)")
            ),
            reg_value(&expected.key, expected.name)?,
            Some((expected.kind.to_owned(), expected.data.clone())),
        );
    }
    checks.that(
        "reg.exe: a key with no values came back",
        reg_key_exists(&format!(r"{HKLM_APP_KEY}\Empty"))?,
        "it is missing",
    );

    checks.same(
        "reg.exe: the machine Path gained only this app's folder, and kept its type",
        reg_value(MACHINE_ENVIRONMENT_KEY, Some("Path"))?,
        Some((
            paths.machine_kind.clone(),
            path_with(&paths.machine, &[&app_bin(root)]),
        )),
    );
    checks.same(
        "reg.exe: the user Path gained only this app's folder, and kept its type",
        reg_value(USER_ENVIRONMENT_KEY, Some("Path"))?,
        Some((
            paths.user_kind.clone(),
            path_with(&paths.user, &[&app_bin(root)]),
        )),
    );

    for service in &SERVICES {
        let fields = sc_qc(service.name)?;
        let label = |what: &str| format!("sc.exe qc {}: {what}", service.name);
        checks.same(
            &label("runs the restored program with its arguments, quoted"),
            sc_field(&fields, "BINARY_PATH_NAME"),
            service_image_path(root, service.marker),
        );
        checks.same(
            &label("the account"),
            sc_field(&fields, "SERVICE_START_NAME"),
            service.expected_object.to_owned(),
        );
        let start = sc_field(&fields, "START_TYPE");
        checks.that(
            &label("the start type"),
            start.ends_with(service.expected_start),
            &start,
        );
        let kind = sc_field(&fields, "TYPE");
        checks.that(
            &label("its own process"),
            kind.ends_with("WIN32_OWN_PROCESS"),
            &kind,
        );
        checks.same(
            &label("the display name"),
            sc_field(&fields, "DISPLAY_NAME"),
            service.display_name.to_owned(),
        );
    }
    let local = sc_qc("WamE2ELocal")?;
    checks.same(
        "sc.exe qc WamE2ELocal: the dependency",
        sc_field(&local, "DEPENDENCIES"),
        "RpcSs".to_owned(),
    );
    let description = shown("sc.exe", &["qdescription", "WamE2ELocal", "8192"])?;
    checks.that(
        "sc.exe qdescription WamE2ELocal: the description",
        description.contains("A harmless service for the elevated round trip"),
        &description,
    );

    check_driver(checks, root)?;
    check_tasks(checks, root)?;
    check_shortcuts(checks, folders, root)?;
    check_uninstall_entry(checks, root).map(|_| ())
}

/// What Settings would run, read from the registry and looked at from outside. Returns the
/// install's id.
fn check_uninstall_entry(checks: &mut Checks, root: &str) -> TestResult<String> {
    let uninstall = entry_value("UninstallString")?;
    let quiet = entry_value("QuietUninstallString")?;
    let exe_end = uninstall
        .find(r#".exe" "#)
        .ok_or("no program in UninstallString")?
        + 4;
    let exe = uninstall[1..exe_end].to_owned();
    let id = uninstall
        .rsplit(' ')
        .next()
        .ok_or("no id in UninstallString")?
        .to_owned();
    checks.same(
        "reg.exe: UninstallString runs the copy in the admin-only folder, with its path quoted",
        uninstall.clone(),
        format!("\"{INSTALLS_DIR}\\{id}\\uninstall.exe\" --uninstall {id}"),
    );
    checks.same(
        "reg.exe: QuietUninstallString is the same with --quiet",
        quiet,
        format!("{uninstall} --quiet"),
    );
    checks.that(
        "the uninstaller is not in the install folder",
        !exe.to_ascii_lowercase()
            .starts_with(&root.to_ascii_lowercase()),
        &exe,
    );
    checks.same(
        "reg.exe: the source's ModifyPath did not survive",
        reg_value(UNINSTALL_KEY, Some("ModifyPath"))?,
        None,
    );
    for flag in ["NoModify", "NoRepair"] {
        checks.same(
            &format!("reg.exe: {flag}"),
            reg_value(UNINSTALL_KEY, Some(flag))?,
            Some(("REG_DWORD".to_owned(), "0x1".to_owned())),
        );
    }
    let signature = powershell(&format!(
        "(Get-AuthenticodeSignature -LiteralPath {}).Status",
        quoted(&exe)
    ))?;
    checks.same(
        "Get-AuthenticodeSignature: the uninstaller is not signed",
        signature.trim(),
        "NotSigned",
    );
    Ok(id)
}

fn entry_value(name: &str) -> TestResult<String> {
    Ok(reg_value(UNINSTALL_KEY, Some(name))?
        .ok_or_else(|| format!("the uninstall entry has no {name}"))?
        .1)
}

/// As far as a driver can be checked without loading one. It is registered and points at
/// its image with no quotes, and it never ran. Nothing here starts it.
fn check_driver(checks: &mut Checks, root: &str) -> TestResult {
    let fields = sc_qc(DRIVER_NAME)?;
    let kind = sc_field(&fields, "TYPE");
    checks.that(
        "sc.exe qc driver: a kernel driver",
        kind.ends_with("KERNEL_DRIVER"),
        &kind,
    );
    let start = sc_field(&fields, "START_TYPE");
    checks.that(
        "sc.exe qc driver: starts on demand",
        start.ends_with("DEMAND_START"),
        &start,
    );
    let image = sc_field(&fields, "BINARY_PATH_NAME");
    checks.that(
        "sc.exe qc driver: points at the restored image, with no quotes",
        image.ends_with(&format!(r"{root}\drivers\wame2e.sys")) && !image.contains('"'),
        &image,
    );
    let state = shown("sc.exe", &["query", DRIVER_NAME])?;
    checks.that(
        "sc.exe query driver: never started",
        state.contains("STOPPED"),
        &state,
    );
    Ok(())
}

fn check_tasks(checks: &mut Checks, root: &str) -> TestResult {
    let names = powershell(&format!(
        "$ErrorActionPreference = 'Continue'; \
         Get-ScheduledTask -TaskPath {} -ErrorAction SilentlyContinue | \
         ForEach-Object {{ $_.TaskName }} | Sort-Object; exit 0",
        quoted(TASK_FOLDER)
    ))?;
    checks.same(
        "Get-ScheduledTask: both tasks, with a name that is not ASCII intact",
        names.lines().map(str::trim).collect::<Vec<_>>(),
        vec![USER_TASK, SYSTEM_TASK],
    );
    let exe = app_exe(root);
    let conf = app_conf(root);
    let system = task_xml(&task_full_name(SYSTEM_TASK))?.unwrap_or_default();
    for (what, wanted) in [
        (
            "the quoted command",
            format!("<Command>\"{exe}\"</Command>"),
        ),
        (
            "the arguments",
            format!("--marker task-system --config \"{conf}\""),
        ),
        (
            "the working folder",
            format!("<WorkingDirectory>{}</WorkingDirectory>", app_bin(root)),
        ),
        ("the account", "<UserId>S-1-5-18</UserId>".to_owned()),
    ] {
        checks.that(
            &format!("schtasks /query /xml, the SYSTEM task: {what}"),
            system.contains(&wanted),
            &system,
        );
    }
    let user = task_xml(&task_full_name(USER_TASK))?.unwrap_or_default();
    checks.same(
        "schtasks /query /xml, the user's task: both programs",
        user.matches(&format!("<Command>{exe}</Command>")).count(),
        2,
    );
    Ok(())
}

fn check_shortcuts(checks: &mut Checks, folders: &ShellFolders, root: &str) -> TestResult {
    for spec in shortcut_specs(folders, root) {
        checks.same(
            &format!("the shortcut at {}", spec.path),
            read_shortcut(&spec.path)?,
            Some([
                app_exe(root),
                spec.arguments.clone(),
                spec.working_directory.clone(),
                spec.icon.clone(),
            ]),
        );
    }
    Ok(())
}

/// Start each service for real. The fake app writes down how Windows started it, which
/// proves what `sc.exe qc` only says: that the quoting and arguments work, and that Windows
/// can log on as the account.
fn check_services_really_run(checks: &mut Checks, root: &str) -> TestResult {
    for service in &SERVICES {
        let Some(sid) = service.expected_sid else {
            continue;
        };
        remove_marker(service.marker);
        let started = tool("sc.exe", &["start", service.name])?;
        println!(
            "sc.exe start {}: {} {}",
            service.name,
            started.status,
            text(&started.stdout).trim()
        );
        let marker = wait_for_marker(service.marker, Duration::from_secs(60));
        let label = |what: &str| format!("started {}: {what}", service.name);
        checks.same(
            &label("Windows ran the restored program"),
            marker_field(&marker, "exe"),
            app_exe(root),
        );
        checks.same(
            &label("it got its arguments, split where they should be"),
            marker_field(&marker, "args"),
            format!("--marker|{}|--config|{}", service.marker, app_conf(root)),
        );
        let who = marker_field(&marker, "who");
        checks.that(
            &label("it ran as the right account"),
            who.contains(sid),
            &who,
        );
    }
    let shown_sid = shown("sc.exe", &["showsid", "WamE2EVirtual"])?;
    let virtual_marker = wait_for_marker("service-virtual", Duration::from_secs(1));
    let who = marker_field(&virtual_marker, "who");
    let sid = who.rsplit(',').next().unwrap_or_default().trim_matches('"');
    checks.that(
        "the virtual account's SID is the one sc.exe showsid gives for the service",
        !sid.is_empty() && shown_sid.contains(sid),
        format!("{sid} in {shown_sid}"),
    );
    Ok(())
}

fn check_task_really_runs(checks: &mut Checks, root: &str) -> TestResult {
    remove_marker("task-system");
    println!(
        "{}",
        shown(
            "schtasks.exe",
            &["/run", "/tn", &task_full_name(SYSTEM_TASK)]
        )?
    );
    let marker = wait_for_marker("task-system", Duration::from_secs(60));
    checks.same(
        "the task ran the restored program",
        marker_field(&marker, "exe"),
        app_exe(root),
    );
    checks.same(
        "the task's arguments arrived",
        marker_field(&marker, "args"),
        format!("--marker|task-system|--config|{}", app_conf(root)),
    );
    let who = marker_field(&marker, "who");
    checks.that("the task ran as SYSTEM", who.contains("S-1-5-18"), &who);
    Ok(())
}

/// What a scan is expected to have found, checked before the app is removed, so that
/// something it missed does not look like something restoring lost.
fn check_scan(checks: &mut Checks, manifest: &PackageManifest) {
    let mut services: Vec<&str> = manifest
        .services_detected
        .iter()
        .map(|service| service.service_name.as_str())
        .collect();
    services.sort_unstable();
    checks.same(
        "scan: the services",
        services,
        vec![
            "WamE2ECustom",
            "WamE2ELocal",
            "WamE2ENetwork",
            "WamE2ESystem",
            "WamE2EVirtual",
        ],
    );
    let custom = manifest
        .services_detected
        .iter()
        .find(|service| service.service_name == "WamE2ECustom");
    let delayed = manifest
        .services_detected
        .iter()
        .find(|service| service.service_name == "WamE2ENetwork");
    checks.same(
        "scan: a delayed automatic start is told from a plain one",
        delayed.and_then(|service| service.start_type.as_deref()),
        Some("delayed-automatic"),
    );
    checks.same(
        "scan: the custom account",
        custom.and_then(|service| service.account_name.as_deref()),
        Some(CUSTOM_SOURCE_ACCOUNT),
    );
    let local = manifest
        .services_detected
        .iter()
        .find(|service| service.service_name == "WamE2ELocal");
    checks.same(
        "scan: the service's arguments",
        local.and_then(|service| service.arguments.clone()),
        Some(format!(
            "--marker service-local --config \"{}\"",
            app_conf(SOURCE_ROOT)
        )),
    );
    checks.that(
        "scan: the service's Parameters key",
        local.is_some_and(|service| service.parameters.len() >= 4),
        format!("{:?}", local.map(|service| service.parameters.len())),
    );

    let driver = manifest.drivers_detected.first();
    checks.same(
        "scan: the driver",
        driver.map(|driver| driver.service_name.as_str()),
        Some(DRIVER_NAME),
    );
    checks.that(
        "packaging: the driver image is bundled, with a hash",
        driver.is_some_and(|driver| {
            driver.package_relative_path.is_some() && driver.hash_sha256.is_some()
        }),
        "it was not",
    );

    let mut tasks: Vec<String> = manifest
        .scheduled_tasks_detected
        .iter()
        .map(|task| format!("{}{}", task.task_path, task.task_name))
        .collect();
    tasks.sort_unstable();
    checks.same(
        "scan: each task once, even the one with two programs",
        tasks,
        vec![task_full_name(USER_TASK), task_full_name(SYSTEM_TASK)],
    );
    checks.same("scan: the shortcuts", manifest.shortcuts_detected.len(), 5);
    for (scope, name) in [
        (EnvironmentScope::Machine, "WAM_E2E_HOME"),
        (EnvironmentScope::Machine, "Path"),
        (EnvironmentScope::User, "WAM_E2E_USER_HOME"),
        (EnvironmentScope::User, "Path"),
    ] {
        checks.that(
            &format!("scan: the {scope:?} variable {name}"),
            manifest
                .environment_variables_detected
                .iter()
                .any(|variable| {
                    variable.scope == scope && variable.name.eq_ignore_ascii_case(name)
                }),
            "it was not found",
        );
    }
    for wanted in [
        "InstallDir",
        "LogDir",
        "Plugins",
        "DisplayName",
        "ThreadingModel",
    ] {
        checks.that(
            &format!("scan: the registry value {wanted}"),
            manifest
                .registry_keys_exported
                .iter()
                .any(|entry| entry.value_name.as_deref() == Some(wanted)),
            "it was not found",
        );
    }
    for wanted in [
        r"bin\WAM E2E App.exe",
        r"bin\WAM E2E Shell.dll",
        r"conf\app.conf",
        r"plugins\Größe.txt",
        r"include\aux.h",
    ] {
        checks.that(
            &format!("scan: the file {wanted}"),
            manifest.file_inventory.iter().any(|file| {
                file.should_copy && file.original_path == Path::new(SOURCE_ROOT).join(wanted)
            }),
            "it was not found",
        );
    }
    let prerequisite = manifest.runtime_prerequisites.iter().find(|prerequisite| {
        prerequisite.kind == RuntimePrerequisiteKind::VisualCppRedistributable
    });
    let bundle = prerequisite.and_then(|prerequisite| prerequisite.bundle.as_ref());
    checks.same(
        "packaging: the Visual C++ runtime is bundled",
        bundle.and_then(|bundle| bundle.package_relative_path.clone()),
        Some(PathBuf::from(r"prerequisites\vc_redist.x64.exe")),
    );
    checks.that(
        "packaging: the download passed its signature check",
        !manifest
            .warnings
            .iter()
            .any(|warning| warning.code.starts_with("prerequisite_")),
        format!("{:?}", manifest.warnings),
    );
}

/// What actually downloaded, seen by something other than the code that fetched it.
fn check_downloaded_payload(checks: &mut Checks, package_root: &Path) -> TestResult {
    let hops = must(
        "curl.exe",
        &[
            "-sIL",
            "--max-redirs",
            "5",
            "-o",
            "NUL",
            "-w",
            "%{num_redirects} %{url_effective}",
            VC_REDIST_URL,
        ],
    )?;
    println!("curl says the download link goes: {hops}");
    let redirects: u32 = hops
        .split_whitespace()
        .next()
        .and_then(|count| count.parse().ok())
        .unwrap_or_default();
    checks.that(
        "curl: the link the runtime is downloaded from really redirects, and ends on HTTPS",
        redirects >= 1 && hops.contains("https://"),
        &hops,
    );
    let payload = package_root.join(r"prerequisites\vc_redist.x64.exe");
    let signature = powershell(&format!(
        "$s = Get-AuthenticodeSignature -LiteralPath {}; \
         '{{0}}|{{1}}' -f $s.Status, $s.SignerCertificate.Subject",
        quoted(&payload.display().to_string())
    ))?;
    checks.that(
        "Get-AuthenticodeSignature: the payload is validly signed by Microsoft",
        signature.starts_with("Valid|") && signature.contains("O=Microsoft Corporation"),
        &signature,
    );
    Ok(())
}

/// Take the x64 Visual C++ runtime off this runner, so the next install really has to run
/// the payload it carries. Everything needing it has been built by now.
fn remove_visual_cpp_runtime() -> TestResult {
    let shown = powershell(
        r#"$roots = 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall', 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall';
        $entries = Get-ChildItem $roots -ErrorAction SilentlyContinue | Get-ItemProperty -ErrorAction SilentlyContinue |
          Where-Object { $_.DisplayName -match 'Visual C\+\+ (2015|2017|2019|2022|v14).*x64' };
        foreach ($entry in ($entries | Where-Object { $_.BundleVersion })) {
          "bundle $($entry.DisplayName): $($entry.QuietUninstallString)";
          $process = Start-Process -FilePath 'cmd.exe' -ArgumentList '/c', $entry.QuietUninstallString, 'IGNOREDEPENDENCIES=ALL', '/norestart' -Wait -PassThru -WindowStyle Hidden;
          "  exit $($process.ExitCode)"
        }
        foreach ($entry in ($entries | Where-Object { $_.WindowsInstaller -eq 1 })) {
          "msi $($entry.DisplayName): $($entry.PSChildName)";
          $process = Start-Process -FilePath 'msiexec.exe' -ArgumentList '/x', $entry.PSChildName, '/qn', '/norestart', 'IGNOREDEPENDENCIES=ALL' -Wait -PassThru;
          "  exit $($process.ExitCode)"
        }"#,
    )?;
    println!("{shown}");
    // Removing the packages leaves this behind, and it is what says the runtime is there.
    // The next install has to run the payload to write it again.
    reg_delete_value(VC_RUNTIME_KEY, "Installed");
    reg_delete_value(VC_RUNTIME_KEY_32, "Installed");
    Ok(())
}

/// Change what an install set up, so that a forced install has something to put back. Any
/// of this can fail if the install it follows did, which the checks have already reported.
fn drift_the_machine(root: &str, folders: &ShellFolders, paths: &OriginalPaths) -> TestResult {
    let drifted_image = format!("\"{}\" --drifted", app_exe(root));
    for args in [
        vec![
            "config",
            "WamE2ELocal",
            "binPath=",
            drifted_image.as_str(),
            "start=",
            "disabled",
            "obj=",
            "LocalSystem",
        ],
        vec!["config", "WamE2EVirtual", "obj=", "LocalSystem"],
        vec![
            "config",
            "WamE2ESystem",
            "obj=",
            r"NT AUTHORITY\NetworkService",
            "password=",
            "",
        ],
    ] {
        println!("sc.exe {args:?}: {}", shown("sc.exe", &args)?.trim());
    }
    reg_delete_value(
        &format!(r"{SERVICES_KEY}\WamE2ELocal\Parameters"),
        "Retries",
    );
    reg_add(HKLM_APP_KEY, Some("InstallDir"), "REG_SZ", r"C:\drifted")?;
    reg_delete_value(HKCU_RUN_KEY, APP_NAME);
    let _ = fs::remove_file(app_conf(root));
    write_paths(paths, &paths.machine, &paths.user)?;
    let _ = tool(
        "schtasks.exe",
        &["/delete", "/tn", &task_full_name(USER_TASK), "/f"],
    )?;
    if let Some(desktop) = shortcut_specs(folders, root).first() {
        let _ = fs::remove_file(&desktop.path);
    }
    Ok(())
}

#[test]
fn round_trip_of_a_real_app() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let program = fake_app_program()?;
    set_up_marker_folder()?;
    let folders = shell_folders()?;
    let paths = read_original_paths()?;
    let data: Vec<PathBuf> = vec![
        Path::new(&std::env::var("APPDATA")?).join(format!(r"{APP_NAME}\settings.json")),
        Path::new(&std::env::var("LOCALAPPDATA")?).join(format!(r"{APP_NAME}\cache.db")),
        Path::new(PROGRAM_DATA_FOLDER).join("shared.dat"),
    ];

    section("Set up the fake app");
    for name in [SERVICES[0].name, DRIVER_NAME] {
        if service_exists(name)? {
            return Err(format!("{name} is already on this machine").into());
        }
    }
    if Path::new(SOURCE_ROOT).exists() || Path::new(TARGET_ROOT).exists() {
        return Err("the fake app's folders are already on this machine".into());
    }
    create_fake_app(&program, &folders, &paths, &data)?;
    println!("{}", must("sc.exe", &["qc", "WamE2EVirtual", "8192"])?);
    println!("{}", must("sc.exe", &["qc", DRIVER_NAME, "8192"])?);
    println!(
        "{:?}",
        reg_value(&format!(r"{SERVICES_KEY}\{DRIVER_NAME}"), Some("ImagePath"))?
    );
    println!(
        "{}",
        task_xml(&task_full_name(SYSTEM_TASK))?.unwrap_or_default()
    );

    section("Scan and package it with the real code");
    let app = list_installed_apps_from(&SystemContext::live())?
        .into_iter()
        .find(|app| app.display_name == APP_NAME)
        .ok_or("the installed-app listing does not have the fake app")?;
    checks.same(
        "listing: the install location",
        app.install_location.clone(),
        Some(PathBuf::from(SOURCE_ROOT)),
    );
    checks.same(
        "listing: the main program",
        app.main_executable.clone(),
        Some(PathBuf::from(app_exe(SOURCE_ROOT))),
    );
    let work = tempfile::tempdir()?;
    let package = create_package(&PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app)),
        output_dir: work.path().join(APP_NAME),
        current_directory: None,
        archive_format: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: None,
        include_user_data: true,
        include_program_data: true,
        include_license_data: false,
        bundle_prerequisites: true,
        source: ScanSource::Live,
        ui_theme: None,
    })?;
    for warning in &package.warnings {
        println!("packaging warning {}: {}", warning.code, warning.message);
    }
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let manifest = read_embedded_manifest(
        &EmbeddedPackage::open(&installer)?.ok_or("the installer has no package")?,
    )?;
    check_scan(&mut checks, &manifest);
    check_downloaded_payload(&mut checks, &package.package_root)?;

    section("Remove all of it");
    let file_hashes = tree_hashes(SOURCE_ROOT)?;
    remove_fake_app(SOURCE_ROOT, &folders, &paths, &data)?;
    check_is_gone(&mut checks, SOURCE_ROOT, &folders, &paths, &data)?;

    section("A preview changes nothing");
    let preview = run_installer(&installer, &["--target-root", TARGET_ROOT])?;
    checks.that("the preview succeeded", preview.success, &preview.output);
    check_is_gone(&mut checks, TARGET_ROOT, &folders, &paths, &data)?;

    uninstall_puts_the_machine_back(&mut checks, &installer, &folders, &paths, &data)?;

    section("Install on the clean machine, without --force");
    let install = run_installer(
        &installer,
        &[
            "--target-root",
            TARGET_ROOT,
            "--apply",
            "--restore-user-data",
        ],
    )?;
    checks.that("the install succeeded", install.success, &install.output);
    checks.that(
        "the install says the custom account was not restored",
        install.output.contains("service_account_not_restored"),
        &install.output,
    );
    check_restored(
        &mut checks,
        TARGET_ROOT,
        &file_hashes,
        &folders,
        &paths,
        &data,
    )?;
    section("What was restored really runs");
    check_services_really_run(&mut checks, TARGET_ROOT)?;
    check_task_really_runs(&mut checks, TARGET_ROOT)?;

    section("Without --force, an install over this one stops before changing anything");
    let refused = run_installer(
        &installer,
        &[
            "--target-root",
            TARGET_ROOT,
            "--apply",
            "--restore-user-data",
        ],
    )?;
    checks.that(
        "the second install was refused",
        !refused.success && refused.output.contains("already exists"),
        &refused.output,
    );
    check_restored(
        &mut checks,
        TARGET_ROOT,
        &file_hashes,
        &folders,
        &paths,
        &data,
    )?;

    section("With --force, an install puts back what has changed");
    drift_the_machine(TARGET_ROOT, &folders, &paths)?;
    let forced = run_installer(
        &installer,
        &[
            "--target-root",
            TARGET_ROOT,
            "--apply",
            "--force",
            "--restore-user-data",
        ],
    )?;
    checks.that(
        "the forced install succeeded",
        forced.success,
        &forced.output,
    );
    check_restored(
        &mut checks,
        TARGET_ROOT,
        &file_hashes,
        &folders,
        &paths,
        &data,
    )?;
    check_services_really_run(&mut checks, TARGET_ROOT)?;

    an_install_over_another_comes_back_out(&mut checks, &installer, &folders, &data)?;
    a_late_failure_puts_everything_back(&mut checks, &installer, &folders, &data)?;
    a_killed_installer_is_undone_by_the_next_run(&mut checks, &installer, &folders, &data)?;

    section("The runtime it carries really installs, when it is missing");
    println!(
        "before: {:?} {:?}",
        reg_value(VC_RUNTIME_KEY, Some("Installed"))?,
        reg_value(VC_RUNTIME_KEY, Some("Version"))?
    );
    remove_visual_cpp_runtime()?;
    let runtime_before = reg_value(VC_RUNTIME_KEY, Some("Installed"))?;
    println!("after removing it: {runtime_before:?}");
    checks.that(
        "the runtime could be taken off the runner",
        runtime_before
            .as_ref()
            .is_none_or(|(_, data)| data == "0x0"),
        format!("{runtime_before:?}"),
    );
    let with_runtime = run_installer(
        &installer,
        &[
            "--target-root",
            TARGET_ROOT,
            "--apply",
            "--force",
            "--restore-user-data",
        ],
    )?;
    checks.that(
        "the install with the runtime missing succeeded",
        with_runtime.success,
        &with_runtime.output,
    );
    checks.same(
        "reg.exe: the runtime is registered again",
        reg_value(VC_RUNTIME_KEY, Some("Installed"))?,
        Some(("REG_DWORD".to_owned(), "0x1".to_owned())),
    );
    println!(
        "the runtime is now {:?}",
        reg_value(VC_RUNTIME_KEY, Some("Version"))?
    );
    let dll = powershell(
        r"$s = Get-AuthenticodeSignature -LiteralPath 'C:\Windows\System32\vcruntime140.dll'; '{0}|{1}' -f $s.Status, $s.SignerCertificate.Subject",
    )?;
    checks.that(
        "Get-AuthenticodeSignature: the runtime's DLL is in place, signed by Microsoft",
        dll.starts_with("Valid|") && dll.contains("O=Microsoft Corporation"),
        &dll,
    );
    check_services_really_run(&mut checks, TARGET_ROOT)?;

    logs_are_kept_and_hold_no_secrets(&mut checks, work.path())?;

    checks.finish()
}

const FORCED_INSTALL: [&str; 5] = [
    "--target-root",
    TARGET_ROOT,
    "--apply",
    "--force",
    "--restore-user-data",
];

/// On a machine with none of the fake app on it.
fn uninstall_puts_the_machine_back(
    checks: &mut Checks,
    installer: &Path,
    folders: &ShellFolders,
    paths: &OriginalPaths,
    data: &[PathBuf],
) -> TestResult {
    section("Uninstalling as Windows would puts the machine back");
    let clean = machine_snapshot(TARGET_ROOT, folders, data)?;
    let records = installs_on_record()?;
    let install = run_installer(installer, &FORCED_INSTALL)?;
    checks.that("the install succeeded", install.success, &install.output);
    let id = check_uninstall_entry(checks, TARGET_ROOT)?;
    checks.same(
        "reg.exe: the delayed start came back as delayed",
        reg_value(
            &format!(r"{SERVICES_KEY}\WamE2ENetwork"),
            Some("DelayedAutostart"),
        )?,
        Some(("REG_DWORD".to_owned(), "0x1".to_owned())),
    );
    let installed = machine_snapshot(TARGET_ROOT, folders, data)?;

    // The window opens, and nothing happens until someone clicks in it. It cannot be driven
    // here, and this runner may not be able to show one at all.
    let uninstall = entry_value("UninstallString")?;
    println!("\n--- starting, as Windows would, with a window: {uninstall}");
    let mut window = command_as_windows_would_run_it(&uninstall)?
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    std::thread::sleep(Duration::from_secs(10));
    match window.try_wait()? {
        Some(status) => println!("it exited by itself, with {status}: no window on this runner"),
        None => println!("still running after 10 seconds, waiting for someone to click"),
    }
    let _ = window.kill();
    let _ = window.wait();
    let after_window = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(
        checks,
        "the uninstaller with a window changed nothing without being asked",
        &after_window,
        &installed,
        &[],
    );

    let quiet = entry_value("QuietUninstallString")?;
    let run = run_as_windows_would(&quiet)?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    checks.that(
        "it said user data stays",
        run.output.contains("settings.json stays"),
        &run.output,
    );
    let folder = Path::new(INSTALLS_DIR).join(&id);
    let started = Instant::now();
    while folder.exists() && started.elapsed() < SELF_REMOVAL_PATIENCE {
        std::thread::sleep(Duration::from_millis(250));
    }
    checks.that(
        "the uninstaller removed its own folder once it had exited",
        !folder.exists(),
        folder.display(),
    );
    checks.same("no record is left", installs_on_record()?, records.clone());
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    let data_keys: Vec<String> = data
        .iter()
        .map(|path| format!("data {}", path.display()))
        .collect();
    check_same_state(
        checks,
        "the machine is as it was, apart from the user data",
        &after,
        &clean,
        &data_keys,
    );
    for path in data {
        checks.that(
            &format!("user data stays unless asked for: {}", path.display()),
            path.is_file(),
            "it is gone",
        );
        if let Some(parent) = path.parent() {
            fs::remove_dir_all(parent)?;
        }
    }

    section("What another program changed in between is left alone");
    let install = run_installer(installer, &FORCED_INSTALL)?;
    checks.that("the install succeeded", install.success, &install.output);
    let id = install_id_in(&install.output)?;
    fs::write(app_conf(TARGET_ROOT), b"changed by the user")?;
    fs::write(format!(r"{TARGET_ROOT}\notes.txt"), b"added by the user")?;
    reg_add(HKLM_APP_KEY, Some("InstallDir"), "REG_SZ", "changed since")?;
    reg_add(
        HKLM_APP_KEY,
        Some("AddedLater"),
        "REG_SZ",
        "another program's",
    )?;
    reg_add(
        MACHINE_ENVIRONMENT_KEY,
        Some("WAM_E2E_HOME"),
        "REG_SZ",
        "changed since",
    )?;
    let later_entry = r"C:\WAM E2E Added Later\bin";
    let (_, machine_path) =
        reg_value(MACHINE_ENVIRONMENT_KEY, Some("Path"))?.ok_or("no machine Path")?;
    reg_add(
        MACHINE_ENVIRONMENT_KEY,
        Some("Path"),
        &paths.machine_kind,
        &path_with(&machine_path, &[later_entry]),
    )?;
    let changed_image = format!("\"{}\" --changed", app_exe(TARGET_ROOT));
    must(
        "sc.exe",
        &["config", "WamE2ELocal", "binPath=", &changed_image],
    )?;
    powershell(&format!(
        "$action = New-ScheduledTaskAction -Execute {} -Argument '--changed'; \
         Set-ScheduledTask -TaskName {} -TaskPath {} -Action $action | Out-Null",
        quoted(&app_exe(TARGET_ROOT)),
        quoted(USER_TASK),
        quoted(TASK_FOLDER),
    ))?;

    let run = uninstall_quietly(checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    // The service and task that were kept only had their arguments changed, so they still run
    // the program. It stays with them: without it, they would be set to run a file that has gone,
    // from a folder others may be able to put another one in.
    let program = Path::new(&app_exe(TARGET_ROOT))
        .strip_prefix(TARGET_ROOT)?
        .display()
        .to_string();
    let mut wanted = vec![r"conf\app.conf".to_owned(), "notes.txt".to_owned(), program];
    wanted.sort();
    checks.same(
        "only the changed file, the added one, and what is still used stay",
        tree_hashes(TARGET_ROOT)?
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        wanted,
    );
    checks.same(
        "what the user wrote in that file is still there",
        fs::read(app_conf(TARGET_ROOT))?,
        b"changed by the user".to_vec(),
    );
    checks.same(
        "reg.exe: a value someone else changed is left",
        reg_value(HKLM_APP_KEY, Some("InstallDir"))?.map(|(_, data)| data),
        Some("changed since".to_owned()),
    );
    checks.same(
        "reg.exe: a value someone else added is left",
        reg_value(HKLM_APP_KEY, Some("AddedLater"))?.map(|(_, data)| data),
        Some("another program's".to_owned()),
    );
    checks.same(
        "reg.exe: the rest of what the install wrote there is gone",
        reg_value(HKLM_APP_KEY, Some("Count"))?,
        None,
    );
    checks.same(
        "reg.exe: an environment variable someone else changed is left",
        reg_value(MACHINE_ENVIRONMENT_KEY, Some("WAM_E2E_HOME"))?.map(|(_, data)| data),
        Some("changed since".to_owned()),
    );
    checks.same(
        "reg.exe: only the app's entry came out of the machine Path",
        reg_value(MACHINE_ENVIRONMENT_KEY, Some("Path"))?.map(|(_, data)| data),
        Some(path_with(&paths.machine, &[later_entry])),
    );
    checks.same(
        "sc.exe: a service someone else changed is left",
        sc_field(&sc_qc("WamE2ELocal")?, "BINARY_PATH_NAME"),
        changed_image,
    );
    checks.that(
        "sc.exe: a service nobody changed is gone",
        !service_exists("WamE2ESystem")?,
        "it is still there",
    );
    checks.that(
        "schtasks.exe: a task someone else changed is left",
        task_xml(&task_full_name(USER_TASK))?.is_some_and(|xml| xml.contains("--changed")),
        "it is gone or changed",
    );
    checks.that(
        "schtasks.exe: a task nobody changed is gone",
        task_xml(&task_full_name(SYSTEM_TASK))?.is_none(),
        "it is still there",
    );
    for (what, wanted) in [
        ("the changed file", "app.conf stays"),
        (
            "the program",
            "stays, because something that stays still uses it",
        ),
        ("the folder", "stays, because it holds files"),
        ("the value", r"InstallDir stays"),
        ("the environment variable", "WAM_E2E_HOME stays"),
        ("the service", "WamE2ELocal stays"),
        ("the task", "Per User stays"),
    ] {
        checks.that(
            &format!("it reported {what}"),
            run.output.contains(wanted),
            &run.output,
        );
    }
    for path in data {
        checks.that(
            &format!("user data went, having been asked for: {}", path.display()),
            !path.exists(),
            "it is still there",
        );
    }

    // What was left, as someone would clean it up.
    must("sc.exe", &["delete", "WamE2ELocal"])?;
    must(
        "schtasks.exe",
        &["/delete", "/tn", &task_full_name(USER_TASK), "/f"],
    )?;
    delete_task_folder()?;
    reg_delete_key(HKLM_APP_KEY);
    reg_delete_key(r"HKLM\Software\WAM E2E Vendor");
    reg_delete_value(MACHINE_ENVIRONMENT_KEY, "WAM_E2E_HOME");
    write_paths(paths, &paths.machine, &paths.user)?;
    fs::remove_dir_all(TARGET_ROOT)?;
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(
        checks,
        "and with that cleaned up, nothing else is left anywhere",
        &after,
        &clean,
        &[],
    );
    Ok(())
}

/// With an install in place, another over it, and then that one uninstalled.
fn an_install_over_another_comes_back_out(
    checks: &mut Checks,
    installer: &Path,
    folders: &ShellFolders,
    data: &[PathBuf],
) -> TestResult {
    section("An install over another comes back out, and what it replaced comes back");
    // So that there is something for it to replace that it will not write itself.
    reg_add(HKLM_APP_KEY, Some("Count"), "REG_DWORD", "0x63")?;
    fs::write(app_conf(TARGET_ROOT), b"as it was before\r\n")?;
    let before = machine_snapshot(TARGET_ROOT, folders, data)?;
    let forced = run_installer(installer, &FORCED_INSTALL)?;
    checks.that("the install succeeded", forced.success, &forced.output);
    let id = install_id_in(&forced.output)?;
    checks.same(
        "reg.exe: it replaced the value",
        reg_value(HKLM_APP_KEY, Some("Count"))?.map(|(_, data)| data),
        Some("0x2a".to_owned()),
    );
    let run = uninstall_quietly(checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(checks, "the machine is as it was", &after, &before, &[]);
    Ok(())
}

/// Files, values, a service, and a task are all already there, and what is about to fail comes
/// after nearly everything else has been done.
fn a_late_failure_puts_everything_back(
    checks: &mut Checks,
    installer: &Path,
    folders: &ShellFolders,
    data: &[PathBuf],
) -> TestResult {
    section("A failure late in an install puts everything back");
    let before = machine_snapshot(TARGET_ROOT, folders, data)?;
    let records = installs_on_record()?;
    // A shortcut in the Startup folder makes something run, so failing to replace it fails the
    // install, and shortcuts are nearly the last thing to be done.
    let startup = shortcut_specs(folders, TARGET_ROOT)
        .pop()
        .ok_or("no startup shortcut")?
        .path;
    let held = fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&startup)?;
    let run = run_installer(installer, &FORCED_INSTALL)?;
    drop(held);
    checks.that("the install reports failure", !run.success, &run.output);
    checks.that(
        "it says what it did about it",
        run.output.contains("what it changed was put back"),
        &run.output,
    );
    checks.that(
        "everything could be undone",
        run.output.contains("Could not be undone: 0"),
        &run.output,
    );
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(checks, "the machine is as it was", &after, &before, &[]);
    checks.same(
        "the backups and the record went with it",
        installs_on_record()?,
        records.clone(),
    );

    section("So does a file an administrator is denied");
    // Nobody may add a file to this folder, which is how a file gets put in place.
    let conf_folder = format!(r"{TARGET_ROOT}\conf");
    must("icacls", &[&conf_folder, "/deny", "*S-1-1-0:(WD)"])?;
    let before = machine_snapshot(TARGET_ROOT, folders, data)?;
    let run = run_installer(installer, &FORCED_INSTALL)?;
    checks.that("the install reports failure", !run.success, &run.output);
    checks.that(
        "everything could be undone",
        run.output.contains("Could not be undone: 0"),
        &run.output,
    );
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(checks, "the machine is as it was", &after, &before, &[]);
    checks.same("no record is left", installs_on_record()?, records);
    must("icacls", &[&conf_folder, "/remove:d", "*S-1-1-0"])?;
    Ok(())
}

fn a_killed_installer_is_undone_by_the_next_run(
    checks: &mut Checks,
    installer: &Path,
    folders: &ShellFolders,
    data: &[PathBuf],
) -> TestResult {
    let package = hand_built_package("WAM E2E Recover Source")?;
    let other = package.build("recover")?;
    let other_target = format!(r"{PROGRAM_FILES}\WAM E2E Recover");
    let other_install = ["--target-root", other_target.as_str(), "--apply", "--force"];
    let before = machine_snapshot(TARGET_ROOT, folders, data)?;
    for (marker, stage) in [
        (r#""change":"service""#, "setting up services"),
        (r#""change":"task""#, "setting up tasks"),
        (r#""change":"environment_value""#, "nearly done"),
    ] {
        section(&format!("Killed {stage}, and undone by the next run"));
        let killed = kill_installer_at(installer, &FORCED_INSTALL, marker, checks)?;
        let preview = run_installer(&other, &["--target-root", &other_target])?;
        checks.that("a preview succeeds", preview.success, &preview.output);
        checks.that(
            "a preview says an earlier install never finished",
            preview.output.contains("earlier_install_never_finished"),
            &preview.output,
        );
        checks.that(
            "and leaves it where it is",
            killed.as_ref().is_some_and(|folder| folder.exists()),
            "its record is gone",
        );
        let next = run_installer(&other, &other_install)?;
        checks.that("the next install succeeds", next.success, &next.output);
        checks.that(
            "it says it undid the earlier one",
            next.output.contains("earlier_install_undone"),
            &next.output,
        );
        checks.that(
            "the record of the killed one is gone",
            killed.as_ref().is_some_and(|folder| !folder.exists()),
            "it is still there",
        );
        let after = machine_snapshot(TARGET_ROOT, folders, data)?;
        check_same_state(checks, "the machine is as it was", &after, &before, &[]);
    }

    section("Killed while undoing that, and finished by the run after");
    let killed = kill_installer_at(
        installer,
        &FORCED_INSTALL,
        r#""change":"environment_value""#,
        checks,
    )?
    .ok_or("the installer did not get far enough to be killed")?;
    let backups = killed.join("backups");
    let waiting_with = fs::read_dir(&backups)?.count();
    let mut undoing = Command::new(&other)
        .args(other_install)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let started = Instant::now();
    let mut caught_it = false;
    while started.elapsed() < Duration::from_secs(120) {
        if fs::read_dir(&backups).map_or(0, Iterator::count) < waiting_with {
            caught_it = true;
            break;
        }
        if undoing.try_wait()?.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    undoing.kill()?;
    undoing.wait()?;
    println!(
        "killed part-way through undoing: {caught_it}, with {} of {waiting_with} backups left",
        fs::read_dir(&backups).map_or(0, Iterator::count)
    );
    checks.that(
        "the record is still there, since that did not finish",
        !caught_it || killed.exists(),
        "its record is gone",
    );
    let next = run_installer(&other, &other_install)?;
    checks.that("the run after that succeeds", next.success, &next.output);
    let after = machine_snapshot(TARGET_ROOT, folders, data)?;
    check_same_state(checks, "the machine is as it was", &after, &before, &[]);
    checks.that("its record is gone too", !killed.exists(), killed.display());
    Ok(())
}

fn logs_are_kept_and_hold_no_secrets(checks: &mut Checks, work: &Path) -> TestResult {
    section("What is logged is kept, and holds no secrets");
    check_is_protected(checks, LOGS_DIR)?;
    check_is_protected(checks, INSTALLS_DIR)?;
    check_is_protected(checks, STATE_ROOT)?;
    // Asked for by name as well, and held to the same.
    let asked_for = work.join("asked-for.log");
    let package = hand_built_package("WAM E2E Log Source")?;
    let installer = package.build("logged")?;
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Logged");
    let asked_text = asked_for.display().to_string();
    let run = run_installer(
        &installer,
        &["--target-root", &target, "--apply", "--log", &asked_text],
    )?;
    checks.that("the install succeeded", run.success, &run.output);
    checks.that(
        "a log asked for by name was written",
        fs::read_to_string(&asked_for).is_ok_and(|log| log.contains("install finished")),
        asked_for.display(),
    );
    let refused = run_installer(
        &installer,
        &["--target-root", &target, "--apply", "--log", &asked_text],
    )?;
    checks.that(
        "one that is already there is refused, and nothing is done",
        !refused.success,
        &refused.output,
    );
    // Only now, since each new log can take the place of the oldest.
    let logs: Vec<PathBuf> = fs::read_dir(LOGS_DIR)?
        .flatten()
        .map(|entry| entry.path())
        .collect();
    checks.that(
        "there are logs, and no more than the limit, after far more runs than that",
        !logs.is_empty() && logs.len() <= 20,
        format!("{logs:?}"),
    );
    let mut text_of_logs = String::new();
    for log in logs.iter().chain([&asked_for]) {
        text_of_logs.push_str(&fs::read_to_string(log)?);
    }
    checks.that(
        "the logs say what happened",
        text_of_logs.contains("install started") && text_of_logs.contains(APP_NAME),
        "they do not",
    );
    for secret in [SECRET_IN_REGISTRY, SECRET_IN_ENVIRONMENT, "the app's data"] {
        checks.that(
            &format!("no log holds {secret}"),
            !text_of_logs.contains(secret),
            "it was there",
        );
    }
    let id = install_id_in(&run.output)?;
    let mut uninstall_checks = Checks::default();
    let run = uninstall_quietly(&mut uninstall_checks, &id, "")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Packages written by hand
// ---------------------------------------------------------------------------------------

/// A package of one program, from the real packaging code, unpacked and ready to be
/// changed. Its program is at the install location it reports.
struct HandBuilt {
    work: tempfile::TempDir,
    package_root: PathBuf,
    manifest: PackageManifest,
    program: String,
}

fn hand_built_package(name: &str) -> TestResult<HandBuilt> {
    let source = format!(r"{PROGRAM_FILES}\{name}");
    fs::create_dir_all(&source)?;
    let program = format!(r"{source}\{name}.exe");
    fs::copy(fake_app_program()?, &program)?;
    let work = tempfile::tempdir()?;
    let package_root = work.path().join(name);
    let package = create_package(&PackageOptions {
        output_dir: package_root.clone(),
        create_self_extracting_archive: false,
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&program, &package_root)
    })?;
    let manifest = serde_json::from_slice(&fs::read(&package.manifest_path)?)?;
    Ok(HandBuilt {
        work,
        package_root,
        manifest,
        program,
    })
}

impl HandBuilt {
    /// Put a file in the package, and return its hash and size.
    fn add_file(&self, relative: &str, content: &Path) -> TestResult<(String, u64)> {
        let path = self.package_root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(content, &path)?;
        Ok((sha256_file(&path)?, fs::metadata(&path)?.len()))
    }

    /// Write the installer with the manifest as it now stands.
    fn build(&self, file_name: &str) -> TestResult<PathBuf> {
        fs::write(
            self.package_root.join("manifest.json"),
            serde_json::to_vec_pretty(&self.manifest)?,
        )?;
        let zip = self.work.path().join(format!("{file_name}.zip"));
        create_zip_archive(&self.package_root, &zip, &BuildMonitor::default())?;
        let installer = self.work.path().join(format!("{file_name}.exe"));
        create_self_extracting_archive(&InstallerStub::File(stub_binary()), &zip, &installer)?;
        Ok(installer)
    }
}

fn registry_entry(
    hive: RegistryHive,
    key_path: &str,
    value_name: Option<&str>,
    value_data: &str,
    kind: RegistryValueKind,
) -> RegistryEntry {
    RegistryEntry {
        hive,
        key_path: key_path.to_owned(),
        value_name: value_name.map(str::to_owned),
        value_data: Some(value_data.to_owned()),
        value_kind: Some(kind),
        value_bytes_hex: None,
        confidence: MatchConfidence::High,
        requires_admin: false,
    }
}

fn service_entry(name: &str, binary: &str, parameters: Vec<RegistryEntry>) -> ServiceEntry {
    ServiceEntry {
        service_name: name.to_owned(),
        display_name: None,
        binary_path: PathBuf::from(binary),
        arguments: None,
        parameters,
        start_type: Some("manual".to_owned()),
        account_name: None,
        dependencies: Vec::new(),
        description: None,
        requires_admin: true,
        restore_command_suggestion: String::new(),
    }
}

fn task_entry(
    task_path: &str,
    task_name: &str,
    command: &str,
    actions: &str,
) -> ScheduledTaskEntry {
    ScheduledTaskEntry {
        task_name: task_name.to_owned(),
        task_path: task_path.to_owned(),
        action_command: PathBuf::from(command),
        arguments: None,
        working_directory: None,
        trigger_summary: None,
        run_level: None,
        account: None,
        requires_admin: false,
        restore_instructions: String::new(),
        definition_xml: Some(format!(
            "<Task xmlns=\"http://schemas.microsoft.com/windows/2004/02/mit/task\"><Actions>{actions}</Actions></Task>"
        )),
    }
}

fn exec_action(command: &str) -> String {
    format!("<Exec><Command>{command}</Command></Exec>")
}

fn shortcut_entry(lnk: &str, target: &str) -> ShortcutEntry {
    ShortcutEntry {
        original_lnk_path: PathBuf::from(lnk),
        target_path: PathBuf::from(target),
        arguments: None,
        working_directory: None,
        icon_path: None,
        target_is_selected_app: true,
    }
}

fn environment_entry(scope: EnvironmentScope, name: &str, value: &str) -> EnvironmentVariableEntry {
    EnvironmentVariableEntry {
        scope,
        name: name.to_owned(),
        value: value.to_owned(),
        value_kind: RegistryValueKind::String,
        requires_admin: false,
        confidence: MatchConfidence::High,
    }
}

const CMD: &str = r"C:\Windows\System32\cmd.exe";
const HOSTILE_JOURNAL: &str = r"C:\ProgramData\WinAppMigrator\installs\wam-e2eforged\journal.jsonl";
const HOSTILE_DRIVER_NAME: &str = "Spooler";
const IFEO_KEY: &str =
    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\wame2e-victim.exe";
const WINLOGON_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
const LSA_KEY: &str = r"SYSTEM\CurrentControlSet\Control\Lsa";
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const EXEFILE_KEY: &str = r"Software\Classes\exefile\shell\open\command";
const POLICIES_KEY: &str = r"Software\Policies\WAM E2E Hostile";
const SPOOLER_PARAMETERS_KEY: &str = r"SYSTEM\CurrentControlSet\Services\Spooler\Parameters";
const SHELL_LINK_CLASS: &str =
    r"SOFTWARE\Classes\CLSID\{00021401-0000-0000-C000-000000000046}\InprocServer32";

/// Everything a hostile package below asks for that is refused, and how to see the machine
/// did not change: what is in `reg.exe`, or `sc.exe`, or on disk, before and after.
fn hostile_state() -> TestResult<BTreeMap<String, String>> {
    let mut state = BTreeMap::new();
    for (label, key, name) in [
        ("IFEO", format!(r"HKLM\{IFEO_KEY}"), Some("Debugger")),
        ("Winlogon", format!(r"HKLM\{WINLOGON_KEY}"), Some("Shell")),
        ("LSA", format!(r"HKLM\{LSA_KEY}"), Some("WamE2EHostile")),
        ("Run", format!(r"HKCU\{RUN_KEY}"), Some("WamE2EHostile")),
        ("exefile", format!(r"HKLM\{EXEFILE_KEY}"), None),
        ("policies", format!(r"HKLM\{POLICIES_KEY}"), Some("Value")),
        (
            "spooler parameters",
            format!(r"HKLM\{SPOOLER_PARAMETERS_KEY}"),
            Some("ServiceDll"),
        ),
        (
            "user environment",
            USER_ENVIRONMENT_KEY.to_owned(),
            Some("WamE2EHostile"),
        ),
        (
            "COR_PROFILER",
            MACHINE_ENVIRONMENT_KEY.to_owned(),
            Some("COR_PROFILER"),
        ),
        (
            "PATHEXT",
            MACHINE_ENVIRONMENT_KEY.to_owned(),
            Some("PATHEXT"),
        ),
        (
            "ComSpec",
            MACHINE_ENVIRONMENT_KEY.to_owned(),
            Some("ComSpec"),
        ),
        (
            "shell link class",
            format!(r"HKLM\{SHELL_LINK_CLASS}"),
            None,
        ),
    ] {
        state.insert(format!("reg.exe: {label}"), reg_state(&key, name)?);
    }
    for service in [
        "Spooler",
        "WamE2EHostileCmd",
        "WamE2EHostileKey",
        "WamE2EHostileName",
    ] {
        state.insert(
            format!("sc.exe qc {service}"),
            format!("{:?}", sc_qc(service)?),
        );
    }
    for task in [
        r"\Microsoft\Windows\WamE2EHostile",
        r"\WAM E2E Hostile\Runs Cmd",
        r"\WAM E2E Hostile\Com Handler",
    ] {
        state.insert(
            format!("schtasks.exe: {task}"),
            format!("{:?}", task_xml(task)?.is_some()),
        );
    }
    for file in hostile_files()? {
        state.insert(
            format!("on disk: {}", file.display()),
            file.exists().to_string(),
        );
    }
    Ok(state)
}

fn hostile_files() -> TestResult<Vec<PathBuf>> {
    Ok(vec![
        PathBuf::from(r"C:\Windows\System32\wame2e-hostile.lnk"),
        Path::new(&std::env::var("APPDATA")?)
            .join(r"Microsoft\Windows\Start Menu\Programs\Startup\wame2e-hostile.exe"),
        Path::new(&shell_folders()?.desktop).join("WAM E2E Hostile Cmd.lnk"),
        PathBuf::from(HOSTILE_JOURNAL),
    ])
}

/// A package asking for all of it. What is refused is left out, and the rest goes ahead.
fn ask_for_refused_items(package: &mut HandBuilt) -> TestResult<Vec<&'static str>> {
    let own = package.program.clone();
    let fake = fake_app_program()?;
    let manifest = &mut package.manifest;
    let (hive, kind) = (RegistryHive::LocalMachine, RegistryValueKind::String);
    manifest.registry_keys_exported.extend([
        registry_entry(hive.clone(), IFEO_KEY, Some("Debugger"), &own, kind.clone()),
        registry_entry(
            hive.clone(),
            WINLOGON_KEY,
            Some("Shell"),
            &own,
            kind.clone(),
        ),
        registry_entry(
            hive.clone(),
            LSA_KEY,
            Some("WamE2EHostile"),
            "1",
            kind.clone(),
        ),
        registry_entry(
            RegistryHive::CurrentUser,
            RUN_KEY,
            Some("WamE2EHostile"),
            r"rundll32.exe C:\wam-e2e\hostile.dll,Entry",
            kind.clone(),
        ),
        registry_entry(hive.clone(), EXEFILE_KEY, None, &own, kind.clone()),
        registry_entry(hive.clone(), POLICIES_KEY, Some("Value"), "1", kind.clone()),
        registry_entry(
            hive.clone(),
            SPOOLER_PARAMETERS_KEY,
            Some("ServiceDll"),
            &own,
            kind.clone(),
        ),
        registry_entry(
            RegistryHive::CurrentUser,
            "Environment",
            Some("WamE2EHostile"),
            &own,
            kind.clone(),
        ),
        registry_entry(
            RegistryHive::CurrentUser,
            "WAM E2E Hostile Link",
            Some("SymbolicLinkValue"),
            &own,
            RegistryValueKind::Link,
        ),
    ]);
    manifest.services_detected.extend([
        service_entry("WamE2EHostileCmd", CMD, Vec::new()),
        // Already on every PC, and running something else.
        service_entry("Spooler", &own, Vec::new()),
        service_entry("WamE2EHostile=Name", &own, Vec::new()),
        service_entry(
            "WamE2EHostileKey",
            &own,
            vec![registry_entry(
                hive.clone(),
                SPOOLER_PARAMETERS_KEY,
                Some("ServiceDll"),
                &own,
                kind.clone(),
            )],
        ),
    ]);
    let (driver_hash, _) = package.add_file(r"drivers\Spooler_hostile.sys", &fake)?;
    let manifest = &mut package.manifest;
    manifest.drivers_detected.push(DriverEntry {
        service_name: HOSTILE_DRIVER_NAME.to_owned(),
        display_name: None,
        image_path: PathBuf::from(r"C:\Windows\System32\drivers\hostile.sys"),
        driver_kind: DriverKind::Kernel,
        start_type: Some("system".to_owned()),
        account_name: None,
        description: None,
        requires_admin: true,
        restore_instructions: String::new(),
        app_owned: true,
        signer: None,
        hash_sha256: Some(driver_hash),
        package_relative_path: Some(PathBuf::from(r"drivers\Spooler_hostile.sys")),
    });
    manifest.scheduled_tasks_detected.extend([
        task_entry(
            r"\Microsoft\Windows\",
            "WamE2EHostile",
            &own,
            &exec_action(&own),
        ),
        task_entry(r"\WAM E2E Hostile\", "Runs Cmd", CMD, &exec_action(CMD)),
        task_entry(
            r"\WAM E2E Hostile\",
            "Com Handler",
            &own,
            "<ComHandler><ClassId>{00000000-0000-0000-0000-000000000000}</ClassId></ComHandler>",
        ),
    ]);
    let desktop_folder = shell_folders()?.desktop;
    manifest.shortcuts_detected.extend([
        shortcut_entry(r"C:\Windows\System32\wame2e-hostile.lnk", &own),
        shortcut_entry(&format!(r"{desktop_folder}\WAM E2E Hostile Cmd.lnk"), CMD),
    ]);
    manifest.environment_variables_detected.extend([
        environment_entry(EnvironmentScope::Machine, "COR_PROFILER", &own),
        environment_entry(EnvironmentScope::Machine, "PATHEXT", ".EXE"),
        environment_entry(EnvironmentScope::Machine, "ComSpec", &own),
    ]);
    for (file, relative) in [
        (
            hostile_files()?
                .into_iter()
                .nth(1)
                .ok_or("no startup file")?,
            r"files\_external\startup\wame2e-hostile.exe",
        ),
        (
            // A forged record of an install, for the next install to act on.
            PathBuf::from(HOSTILE_JOURNAL),
            r"files\_external\state\journal.jsonl",
        ),
    ] {
        let (hash, size) = package.add_file(relative, &fake)?;
        package.manifest.file_inventory.push(FileEntry {
            path: file.clone(),
            original_path: file,
            package_relative_path: Some(PathBuf::from(relative)),
            suggested_destination_path: None,
            size_bytes: Some(size),
            hash_sha256: Some(hash),
            classification: FileClassification::UserData,
            should_copy: true,
            reason: String::new(),
        });
    }
    Ok(vec![
        "wame2e-victim.exe",
        r"Winlogon\Shell",
        r"Lsa\WamE2EHostile",
        r"Run\WamE2EHostile",
        r"exefile\shell\open\command",
        r"Policies\WAM E2E Hostile",
        r"Spooler\Parameters\ServiceDll",
        r"Environment\WamE2EHostile",
        "WAM E2E Hostile Link",
        "The service WamE2EHostileCmd",
        "The service Spooler",
        "The service WamE2EHostile=Name",
        "The service WamE2EHostileKey",
        "The driver Spooler",
        r"\Microsoft\Windows\WamE2EHostile",
        r"\WAM E2E Hostile\Runs Cmd",
        r"\WAM E2E Hostile\Com Handler",
        "wame2e-hostile.lnk",
        "WAM E2E Hostile Cmd.lnk",
        "The environment variable COR_PROFILER",
        "The environment variable PATHEXT",
        "The environment variable ComSpec",
        r"Startup\wame2e-hostile.exe",
        "wam-e2eforged",
    ])
}

/// Ask for what the rules cannot refuse up front, because it depends on what is on this PC.
fn ask_for_what_is_refused_when_installing(package: &mut HandBuilt) {
    let own = package.program.clone();
    package.manifest.registry_keys_exported.push(registry_entry(
        RegistryHive::LocalMachine,
        SHELL_LINK_CLASS,
        None,
        &own,
        RegistryValueKind::String,
    ));
    // Already there, running something else, and --force does not change that.
    package.manifest.scheduled_tasks_detected.push(task_entry(
        r"\WAM E2E Hostile\",
        "Existing",
        &own,
        &exec_action(&own),
    ));
}

/// Ask for a runtime whose file is not what it should be, or is not where it should be. On the
/// Arm64 runtime, which this runner does not have, so it gets as far as running.
fn ask_for_a_runtime_that_fails(
    package: &mut HandBuilt,
    canary: &Path,
    relative: &str,
) -> TestResult {
    let (hash, size) = package.add_file(relative, canary)?;
    package
        .manifest
        .runtime_prerequisites
        .push(RuntimePrerequisite {
            name: format!("Hostile runtime at {relative}"),
            kind: RuntimePrerequisiteKind::VisualCppRedistributable,
            evidence: String::new(),
            install_manually: true,
            present_on_source: None,
            bundle: Some(PrerequisiteBundle {
                architecture: AppArchitecture::Arm64,
                package_relative_path: Some(PathBuf::from(relative)),
                hash_sha256: Some(hash),
                size_bytes: Some(size),
            }),
            presence_probe: None,
        });
    Ok(())
}

#[test]
fn hostile_package_gets_nothing_it_was_refused() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    set_up_marker_folder()?;
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Hostile");

    a_state_folder_someone_else_made_is_not_used(&mut checks)?;

    section("A package asking for what no app should do");
    let before = hostile_state()?;
    let mut package = hand_built_package("WAM E2E Hostile Source")?;
    let refused_items = ask_for_refused_items(&mut package)?;
    let installer = package.build("hostile")?;
    let run = run_installer(
        &installer,
        &[
            "--target-root",
            &target,
            "--apply",
            "--force",
            "--restore-user-data",
        ],
    )?;
    checks.that(
        "refusing items does not fail the install, and the rest of it goes ahead",
        run.success,
        &run.output,
    );
    checks.that(
        "the app's own program was installed",
        Path::new(&target)
            .join("WAM E2E Hostile Source.exe")
            .is_file(),
        "it is missing",
    );
    for item in &refused_items {
        checks.that(
            &format!("reported as refused: {item}"),
            run.output
                .lines()
                .any(|line| line.contains(item) && line.contains("was left out")),
            &run.output,
        );
    }
    checks.that(
        "the count of refused items matches",
        run.output
            .contains(&format!("Refused: {}", refused_items.len())),
        &run.output,
    );
    let after = hostile_state()?;
    for (label, was) in &before {
        checks.same(label, after.get(label), Some(was));
    }

    section("What can only be refused at the moment it would be done");
    let notepad = r"C:\Windows\System32\notepad.exe";
    powershell(&format!(
        "$action = New-ScheduledTaskAction -Execute {}; \
         $trigger = New-ScheduledTaskTrigger -Once -At '2099-01-01T00:00:00'; \
         Register-ScheduledTask -TaskName 'Existing' -TaskPath '\\WAM E2E Hostile\\' \
           -Action $action -Trigger $trigger | Out-Null",
        quoted(notepad)
    ))?;
    let class_before = reg_state(&format!(r"HKLM\{SHELL_LINK_CLASS}"), None)?;
    let mut package = hand_built_package("WAM E2E Refused Source")?;
    ask_for_what_is_refused_when_installing(&mut package);
    let installer = package.build("refused")?;
    let refused_target = format!(r"{PROGRAM_FILES}\WAM E2E Refused");
    let run = run_installer(
        &installer,
        &["--target-root", &refused_target, "--apply", "--force"],
    )?;
    checks.that(
        "refusing those does not fail the install",
        run.success,
        &run.output,
    );
    for (what, wanted) in [
        ("the COM class", r"InprocServer32"),
        ("the existing task", r"\WAM E2E Hostile\Existing"),
    ] {
        checks.that(
            &format!("reported as left out: {what}"),
            run.output
                .lines()
                .any(|line| line.contains(wanted) && line.contains("was left out")),
            &run.output,
        );
    }
    checks.same(
        "reg.exe: Windows' shortcut class is still Windows'",
        reg_state(&format!(r"HKLM\{SHELL_LINK_CLASS}"), None)?,
        class_before,
    );
    let existing = task_xml(r"\WAM E2E Hostile\Existing")?.unwrap_or_default();
    checks.that(
        "schtasks.exe: the existing task still runs what it did",
        existing.contains("notepad.exe") && !existing.contains("WAM E2E Refused"),
        &existing,
    );

    section("A step that fails at the moment it would do something");
    let canary_marker = "unmarked";
    remove_marker(canary_marker);
    for (name, relative, expected) in [
        (
            "WAM E2E Failing Source",
            r"prerequisites\vc_redist.arm64.exe",
            "failed its signature check",
        ),
        (
            "WAM E2E Misplaced Source",
            r"prerequisites\elsewhere.exe",
            "not where it belongs",
        ),
    ] {
        let mut package = hand_built_package(name)?;
        ask_for_a_runtime_that_fails(&mut package, &fake_app_program()?, relative)?;
        let installer = package.build(name)?;
        let target = format!(r"{PROGRAM_FILES}\{name} Target");
        let run = run_installer(
            &installer,
            &["--target-root", &target, "--apply", "--force"],
        )?;
        checks.that("the install reports failure", !run.success, &run.output);
        checks.that(
            "it says the runtime could not be installed",
            run.output.contains("prerequisite_install_failed"),
            &run.output,
        );
        checks.that(
            &format!("and why: {expected}"),
            run.output.contains(expected),
            &run.output,
        );
        checks.that(
            "it put back what it had done",
            run.output.contains("what it changed was put back") && !Path::new(&target).exists(),
            &run.output,
        );
    }
    checks.that(
        "the fake runtime never ran",
        wait_for_marker(canary_marker, Duration::from_secs(5)).is_none(),
        "its marker appeared",
    );

    section("A link planted at the install location");
    let package = hand_built_package("WAM E2E Link Source")?;
    let installer = package.build("links")?;
    let victim_folder = format!(r"{PROGRAM_FILES}\WAM E2E Link Victim");
    fs::create_dir_all(&victim_folder)?;
    let victim = Path::new(&victim_folder).join("victim.bin");
    fs::write(&victim, b"keep")?;
    let link_target = format!(r"{PROGRAM_FILES}\WAM E2E Link Target");
    fs::create_dir_all(&link_target)?;
    std::os::windows::fs::symlink_file(
        &victim,
        Path::new(&link_target).join("WAM E2E Link Source.exe"),
    )?;
    let run = run_installer(
        &installer,
        &["--target-root", &link_target, "--apply", "--force"],
    )?;
    checks.that(
        "a symbolic link at a file's destination stops the install",
        !run.success && run.output.contains("through a link"),
        &run.output,
    );
    checks.same(
        "what the link points at is untouched",
        fs::read(&victim)?,
        b"keep".to_vec(),
    );
    let linked_root = format!(r"{PROGRAM_FILES}\WAM E2E Link Root");
    std::os::windows::fs::symlink_dir(&victim_folder, &linked_root)?;
    let run = run_installer(
        &installer,
        &["--target-root", &linked_root, "--apply", "--force"],
    )?;
    checks.that(
        "a symbolic link as the install location stops the install",
        !run.success && run.output.contains("is a link to somewhere else"),
        &run.output,
    );
    checks.same(
        "nothing landed where that link points",
        fs::read_dir(&victim_folder)?.count(),
        1,
    );

    a_location_is_where_it_is_however_it_is_written(&mut checks)?;
    an_uninstall_cannot_be_led_outside_the_install(&mut checks)?;
    a_drive_that_has_gone_is_waited_for(&mut checks)?;
    a_signed_installer_leaves_an_unsigned_uninstaller(&mut checks)?;

    checks.finish()
}

/// Anyone can make a folder in `ProgramData`, and may do so first, to have their own journal
/// acted on with full rights. This is the first test to run, so none is there yet.
fn a_state_folder_someone_else_made_is_not_used(checks: &mut Checks) -> TestResult {
    section("A state folder someone else made first is not used");
    checks.that(
        "no state folder is here yet",
        !Path::new(STATE_ROOT).exists(),
        STATE_ROOT,
    );
    let package = hand_built_package("WAM E2E Squatted Source")?;
    let installer = package.build("squatted")?;
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Squatted");
    let forged = format!(r"{INSTALLS_DIR}\wam-e2eforged");
    let canary = format!(r"{PROGRAM_FILES}\WAM E2E Squatted Canary.txt");
    fs::write(&canary, b"keep")?;
    fs::create_dir_all(&forged)?;
    must(
        "icacls",
        &[STATE_ROOT, "/grant", "*S-1-5-32-545:(OI)(CI)F", "/T", "/Q"],
    )?;
    let journal = format!(
        "{{\"header\":{{\"schema\":1,\"id\":\"wam-e2eforged\",\"app_name\":\"Forged\",\
         \"target_root\":\"C:\\\\Forged\",\"user_sid\":null,\"started_at_utc\":\"\",\
         \"staging\":null}}}}\n\
         {{\"change\":{{\"change\":\"root\",\"path\":\"C:\\\\Program Files\",\"real\":\"C:\\\\Program Files\"}}}}\n\
         {{\"change\":{{\"change\":\"file\",\"path\":{path:?},\"root\":\"C:\\\\Program Files\",\
         \"item\":\"program\",\"backup\":null,\"sha256\":null,\"partial\":null}}}}\n",
        path = canary,
    );
    fs::write(format!(r"{forged}\journal.jsonl"), journal)?;

    let run = run_installer(&installer, &["--target-root", &target, "--apply"])?;
    checks.that(
        "an install will not go ahead with a state folder others can change",
        !run.success && !Path::new(&target).exists(),
        &run.output,
    );
    checks.that(
        "and does not act on what is in it",
        Path::new(&canary).is_file(),
        "the file the forged record named is gone",
    );

    fs::remove_dir_all(&forged)?;
    fs::remove_dir_all(INSTALLS_DIR)?;
    let run = run_installer(&installer, &["--target-root", &target, "--apply"])?;
    checks.that(
        "one left empty is replaced with a proper one",
        run.success,
        &run.output,
    );
    check_is_protected(checks, STATE_ROOT)?;
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    Ok(())
}

/// A package of a program and a file in a folder, installed at `target`.
fn installed_with_a_subfolder(name: &str, target: &str) -> TestResult<String> {
    let mut package = hand_built_package(name)?;
    let content = package.work.path().join("plugin.dat");
    fs::write(&content, b"the same in both places")?;
    let relative = r"files\sub\plugin.dat";
    let (hash, size) = package.add_file(relative, &content)?;
    let source = Path::new(&package.program)
        .parent()
        .ok_or("no install folder")?
        .join(r"sub\plugin.dat");
    package.manifest.file_inventory.push(FileEntry {
        path: source.clone(),
        original_path: source,
        package_relative_path: Some(PathBuf::from(relative)),
        suggested_destination_path: None,
        size_bytes: Some(size),
        hash_sha256: Some(hash),
        classification: FileClassification::AppOwned,
        should_copy: true,
        reason: String::new(),
    });
    let installer = package.build(name)?;
    let run = run_installer(&installer, &["--target-root", target, "--apply", "--force"])?;
    if !run.success {
        return Err(format!("the install failed: {}", run.output).into());
    }
    install_id_in(&run.output)
}

fn junction(link: &str, target: &str) -> TestResult {
    must("cmd.exe", &["/D", "/C", "mklink", "/J", link, target]).map(|_| ())
}

/// The install folder can be one other programs may write to, and an uninstaller runs elevated.
/// What it finds has been swapped for a link is left, even when what is at the other end
/// is byte for byte what it would have removed.
fn an_uninstall_cannot_be_led_outside_the_install(checks: &mut Checks) -> TestResult {
    let victim = format!(r"{PROGRAM_FILES}\WAM E2E Swap Victim");
    fs::create_dir_all(&victim)?;
    let victim_file = format!(r"{victim}\plugin.dat");
    fs::write(&victim_file, b"the same in both places")?;
    fs::create_dir_all(format!(r"{victim}\sub"))?;
    let victim_nested = format!(r"{victim}\sub\plugin.dat");
    fs::write(&victim_nested, b"the same in both places")?;

    section("A folder in the install swapped for a link");
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Swap");
    let id = installed_with_a_subfolder("WAM E2E Swap Source", &target)?;
    let sub = format!(r"{target}\sub");
    fs::remove_dir_all(&sub)?;
    junction(&sub, &victim)?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "it reports what it could not undo, rather than succeeding",
        !run.success && run.output.contains("through a link"),
        &run.output,
    );
    checks.same(
        "what the link points at is untouched",
        fs::read(&victim_file)?,
        b"the same in both places".to_vec(),
    );
    fs::remove_dir(&sub)?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "with the link gone, running it again finishes the job",
        run.success && !Path::new(&target).exists(),
        &run.output,
    );

    section("The install location swapped for a link");
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Swap Root");
    let id = installed_with_a_subfolder("WAM E2E Swap Root Source", &target)?;
    let moved = format!(r"{PROGRAM_FILES}\WAM E2E Swap Root Moved");
    fs::rename(&target, &moved)?;
    junction(&target, &victim)?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "it reports that it now leads elsewhere, rather than succeeding",
        !run.success && run.output.contains("now leads to"),
        &run.output,
    );
    checks.same(
        "what the link points at is untouched",
        fs::read(&victim_nested)?,
        b"the same in both places".to_vec(),
    );
    Ok(())
}

/// Windows reads all of these as the state folder or the Windows folder, and an install cannot
/// be let in by writing one of them differently.
fn a_location_is_where_it_is_however_it_is_written(checks: &mut Checks) -> TestResult {
    section("The state folder and the Windows folder, written some other way");
    const LEAF: &str = "WAM E2E Spelled";
    const WINDOWS: &str = r"C:\Windows";
    let package = hand_built_package("WAM E2E Spelled Source")?;
    let installer = package.build("spelled")?;
    let link = format!(r"{PROGRAM_FILES}\WAM E2E Link To Windows");
    junction(&link, WINDOWS)?;
    let short_program_data = powershell(
        "(New-Object -ComObject Scripting.FileSystemObject).GetFolder('C:\\ProgramData').ShortPath",
    )?;
    let short_program_data = short_program_data.trim();
    println!("ProgramData's short name here: {short_program_data}");
    let mut locations = vec![
        format!(r"C:\.\ProgramData\WinAppMigrator\{LEAF}"),
        format!(r"C:\\ProgramData\WinAppMigrator\{LEAF}"),
        format!(r"C:/ProgramData/WinAppMigrator/{LEAF}"),
        format!(r"\\?\{STATE_ROOT}\{LEAF}"),
        format!(r"C:\ProgramData\WinAppMigrator.\{LEAF}"),
        format!(r"C:\.\Windows\{LEAF}"),
        format!(r"C:\\Windows\{LEAF}"),
        format!(r"\\?\{WINDOWS}\{LEAF}"),
        format!(r"\\localhost\C$\Windows\{LEAF}"),
        format!(r"C:\Windows.\{LEAF}"),
        format!(r"{link}\{LEAF}"),
    ];
    if !short_program_data.eq_ignore_ascii_case(r"C:\ProgramData") {
        locations.push(format!(r"{short_program_data}\WinAppMigrator\{LEAF}"));
    }
    let on_record = installs_on_record()?;
    // Under a link to a plain folder, which nothing else is wrong with.
    let plain = format!(r"{PROGRAM_FILES}\WAM E2E Plain Folder");
    fs::create_dir_all(&plain)?;
    let link_to_plain = format!(r"{PROGRAM_FILES}\WAM E2E Link To Plain Folder");
    junction(&link_to_plain, &plain)?;
    let run = run_installer(
        &installer,
        &[
            "--target-root",
            &format!(r"{link_to_plain}\{LEAF}"),
            "--apply",
        ],
    )?;
    checks.that(
        "a location under a link is refused, and it says which",
        !run.success && run.output.contains("is a link to somewhere else"),
        &run.output,
    );
    checks.same(
        "with nothing put where it leads",
        fs::read_dir(&plain)?.count(),
        0,
    );
    fs::remove_dir(&link_to_plain)?;
    fs::remove_dir(&plain)?;
    let startup = r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup";
    let run = run_installer(
        &installer,
        &["--target-root", &format!(r"{startup}\{LEAF}"), "--apply"],
    )?;
    checks.that(
        "a Startup folder is refused as a location",
        !run.success && !Path::new(startup).join(LEAF).exists(),
        &run.output,
    );
    for location in &locations {
        let run = run_installer(&installer, &["--target-root", location, "--apply"])?;
        checks.that(&format!("{location} is refused"), !run.success, &run.output);
        checks.that(
            "with nothing put in the Windows folder",
            !Path::new(WINDOWS).join(LEAF).exists(),
            location,
        );
        checks.that(
            "or where installs are recorded",
            !Path::new(STATE_ROOT).join(LEAF).exists(),
            location,
        );
    }
    checks.same(
        "and no install was recorded",
        installs_on_record()?,
        on_record,
    );
    fs::remove_dir(&link)?;
    Ok(())
}

/// A drive can be unplugged between installing and uninstalling, and that is not what has gone
/// wrong with anything. It comes back, so everything has to still be there to finish with.
fn a_drive_that_has_gone_is_waited_for(checks: &mut Checks) -> TestResult {
    section("An install on a drive that is not connected");
    const DRIVE: &str = "W:";
    const ENTRIES: &str = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall";
    let backing = format!(r"{PROGRAM_FILES}\WAM E2E Drive");
    fs::create_dir_all(&backing)?;
    must("subst", &[DRIVE, &backing])?;
    let id = installed_with_a_subfolder("WAM E2E Drive Source", &format!(r"{DRIVE}\Acme"))?;
    let installed = format!(r"{backing}\Acme\sub\plugin.dat");
    checks.that(
        "it was installed",
        Path::new(&installed).is_file(),
        &installed,
    );
    checks.that(
        "and has an entry in Settings",
        whole_key(ENTRIES)?.contains(&id),
        &id,
    );

    must("subst", &[DRIVE, "/D"])?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "the uninstall does not say it finished",
        !run.success,
        &run.output,
    );
    checks.that(
        "and says why, without calling it a failure",
        run.output.contains("not connected") && run.output.contains("Could not be undone: 0"),
        &run.output,
    );
    checks.that(
        "its entry in Settings stays, to finish from",
        whole_key(ENTRIES)?.contains(&id),
        &id,
    );
    checks.that(
        "so does its record",
        installs_on_record()?.contains(&id),
        &id,
    );
    checks.that(
        "and so does the file",
        Path::new(&installed).is_file(),
        &installed,
    );

    must("subst", &[DRIVE, &backing])?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "with the drive back, running it again finishes the job",
        run.success && !Path::new(&format!(r"{backing}\Acme")).exists(),
        &run.output,
    );
    checks.that(
        "and its entry in Settings goes",
        !whole_key(ENTRIES)?.contains(&id),
        &id,
    );
    must("subst", &[DRIVE, "/D"])?;
    fs::remove_dir_all(&backing)?;
    Ok(())
}

/// Whoever builds an installer may sign it. What it leaves behind for Windows to run elevated
/// must not carry that signature, or anyone could turn that copy into anything they like.
/// The certificate is a throwaway made here, trusted only on this runner, and has nothing to
/// do with this project's.
fn a_signed_installer_leaves_an_unsigned_uninstaller(checks: &mut Checks) -> TestResult {
    section("A validly signed installer leaves an uninstaller with no signature");
    let package = hand_built_package("WAM E2E Signed Source")?;
    let installer = package.build("signed")?;
    let installer_text = installer.display().to_string();
    let signed = powershell(&format!(
        "$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=WAM E2E Throwaway' \
           -CertStoreLocation Cert:\\LocalMachine\\My; \
         foreach ($name in 'Root', 'TrustedPublisher') {{ \
           $store = New-Object Security.Cryptography.X509Certificates.X509Store $name, 'LocalMachine'; \
           $store.Open('ReadWrite'); $store.Add($cert); $store.Close() }}; \
         $null = Set-AuthenticodeSignature -LiteralPath {path} -Certificate $cert -HashAlgorithm SHA256; \
         (Get-AuthenticodeSignature -LiteralPath {path}).Status",
        path = quoted(&installer_text)
    ))?;
    checks.same(
        "Get-AuthenticodeSignature: the installer is validly signed",
        signed.trim(),
        "Valid",
    );
    checks.that(
        "it has a certificate table",
        has_certificate_table(&installer)?,
        "it does not",
    );
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Signed");
    let run = run_installer(&installer, &["--target-root", &target, "--apply"])?;
    checks.that("the signed installer installs", run.success, &run.output);
    let id = install_id_in(&run.output)?;
    let uninstaller = Path::new(INSTALLS_DIR).join(&id).join("uninstall.exe");
    let status = powershell(&format!(
        "$s = Get-AuthenticodeSignature -LiteralPath {}; '{{0}}|{{1}}' -f $s.Status, $s.SignerCertificate",
        quoted(&uninstaller.display().to_string())
    ))?;
    checks.same(
        "Get-AuthenticodeSignature: the uninstaller it left is not signed",
        status.trim(),
        "NotSigned|",
    );
    checks.that(
        "the uninstaller has no certificate table",
        !has_certificate_table(&uninstaller)?,
        "it has one",
    );
    checks.same(
        "the uninstaller is byte for byte the stub it was built from",
        sha256_file(&uninstaller)?,
        sha256_file(&stub_binary())?,
    );
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that(
        "it still works",
        run.success && !Path::new(&target).exists(),
        &run.output,
    );
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Staging
// ---------------------------------------------------------------------------------------

/// Attach a small FAT32 volume, which has no permissions to set. Windows' own tools only.
fn attach_fat32_volume(folder: &Path) -> TestResult {
    let vhd = folder.join("wam-e2e-fat32.vhd");
    let script = folder.join("diskpart.txt");
    fs::write(
        &script,
        format!(
            "create vdisk file=\"{vhd}\" maximum=64 type=fixed\r\n\
             select vdisk file=\"{vhd}\"\r\n\
             attach vdisk\r\n\
             create partition primary\r\n\
             format fs=fat32 quick label=WAME2E\r\n\
             assign letter=W\r\n",
            vhd = vhd.display()
        ),
    )?;
    let shown = must("diskpart.exe", &["/s", &script.display().to_string()])?;
    println!("{shown}");
    let started = Instant::now();
    while !Path::new(FAT32_DRIVE).exists() {
        if started.elapsed() > Duration::from_secs(30) {
            return Err("the FAT32 volume did not appear".into());
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(())
}

/// What `Get-Acl` says of a protected folder, wherever it is.
fn check_is_protected(checks: &mut Checks, folder: &str) -> TestResult {
    let shown = powershell(&format!(
        "$acl = Get-Acl -LiteralPath {}; $acl.Owner; $acl.AreAccessRulesProtected; \
         $acl.Access | ForEach-Object {{ '{{0}}|{{1}}|{{2}}|{{3}}|{{4}}' -f \
           $_.IdentityReference, $_.AccessControlType, $_.FileSystemRights, $_.IsInherited, \
           $_.InheritanceFlags }} | Sort-Object",
        quoted(folder)
    ))?;
    checks.same(
        "Get-Acl: administrators own it, nothing is inherited, and only they and SYSTEM are on it",
        shown.lines().map(str::trim).collect::<Vec<_>>(),
        vec![
            r"BUILTIN\Administrators",
            "True",
            r"BUILTIN\Administrators|Allow|FullControl|False|ContainerInherit, ObjectInherit",
            r"NT AUTHORITY\SYSTEM|Allow|FullControl|False|ContainerInherit, ObjectInherit",
        ],
    );
    Ok(())
}

#[test]
fn staging_folder_is_locked_down_and_fails_closed() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    set_up_marker_folder()?;
    let program = fake_app_program()?;
    let parent = format!(r"{PROGRAM_FILES}\WAM E2E Probe");
    fs::create_dir_all(&parent)?;
    let probe = format!(r"{parent}\probe.exe");
    fs::copy(&program, &probe)?;

    section("A protected folder, with an administrator asking for it");
    let staging = ProtectedFolder::new_in(Path::new(&parent), "wam-e2e-stage-")?;
    let folder = staging.path().display().to_string();
    println!("{}", must("icacls", &[&folder])?);
    check_is_protected(&mut checks, &folder)?;

    section("A real account that is not an administrator tries to write there");
    let planted = format!(r"{folder}\planted.txt");
    let image = format!("\"{probe}\" --marker staging-probe --try-write \"{planted}\"");
    must(
        "sc.exe",
        &[
            "create",
            "WamE2EProbe",
            "binPath=",
            &image,
            "start=",
            "demand",
            "obj=",
            r"NT AUTHORITY\LocalService",
            "password=",
            "",
        ],
    )?;
    let _ = tool("sc.exe", &["start", "WamE2EProbe"])?;
    let marker = wait_for_marker("staging-probe", Duration::from_secs(60));
    let result = marker_field(&marker, "probe");
    let who = marker_field(&marker, "who");
    checks.that(
        "the probe ran as LocalService",
        who.contains("S-1-5-19"),
        &who,
    );
    checks.that(
        "LocalService could not write to the staging folder",
        result.starts_with("refused"),
        &result,
    );
    checks.that(
        "nothing was planted",
        !Path::new(&planted).exists(),
        "the file is there",
    );
    // The same probe writing where it is allowed, so a refusal above means something.
    let allowed = format!(r"{MARKER_DIR}\control.txt");
    let image = format!("\"{probe}\" --marker staging-control --try-write \"{allowed}\"");
    must("sc.exe", &["config", "WamE2EProbe", "binPath=", &image])?;
    let _ = tool("sc.exe", &["start", "WamE2EProbe"])?;
    let control = wait_for_marker("staging-control", Duration::from_secs(60));
    checks.same(
        "the same probe can write where it is allowed",
        marker_field(&control, "probe"),
        "written".to_owned(),
    );
    drop(staging);
    checks.that(
        "the folder is gone once it is dropped",
        !Path::new(&folder).exists(),
        "it is still there",
    );

    section("Where permissions cannot be set, the folder is not used");
    let work = tempfile::tempdir()?;
    attach_fat32_volume(work.path())?;
    let on_fat32 = ProtectedFolder::new_in(Path::new(FAT32_DRIVE), "wam-e2e-stage-");
    println!(
        "new_in on FAT32: {:?}",
        on_fat32
            .as_ref()
            .map(|folder| folder.path().display().to_string())
    );
    checks.that(
        "no protected folder on a volume with no permissions",
        on_fat32.is_err(),
        "it reported success",
    );
    drop(on_fat32);
    let fat32_target = format!(r"{FAT32_DRIVE}WAM E2E App");
    let beside = work.path().join("installer.exe");
    let fallback = create_staging_dir(Some(Path::new(&fat32_target)), &beside)?;
    checks.that(
        "the installer stages somewhere it can protect instead",
        !fallback
            .path()
            .display()
            .to_string()
            .starts_with(FAT32_DRIVE),
        fallback.path().display(),
    );
    println!(
        "the temp folder's own permissions: {}",
        powershell("(Get-Acl -LiteralPath $env:TEMP).Sddl")?.trim()
    );
    check_is_protected(&mut checks, &fallback.path().display().to_string())?;
    drop(fallback);
    let package = hand_built_package("WAM E2E Plain Source")?;
    let installer = package.build("plain")?;
    let run = run_installer(&installer, &["--target-root", &fat32_target, "--apply"])?;
    checks.that(
        "the real installer still installs there",
        run.success,
        &run.output,
    );
    checks.that(
        "the program is on that volume",
        Path::new(&fat32_target)
            .join("WAM E2E Plain Source.exe")
            .is_file(),
        "it is missing",
    );
    let left = powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Filter 'winappmigrator-install-*' -Force | \
         ForEach-Object {{ $_.FullName }}",
        quoted(FAT32_DRIVE)
    ))?;
    checks.same("no staging folder is left on that volume", left.trim(), "");

    section("From another drive, what an install replaces is copied, and still comes back");
    let plain_file = Path::new(&fat32_target).join("WAM E2E Plain Source.exe");
    let hash_before = sha256_file(&plain_file)?;
    fs::write(&plain_file, b"changed since")?;
    let changed_hash = sha256_file(&plain_file)?;
    let run = run_installer(
        &installer,
        &["--target-root", &fat32_target, "--apply", "--force"],
    )?;
    checks.that("the install over it succeeded", run.success, &run.output);
    checks.same(
        "the program is back to what the package holds",
        sha256_file(&plain_file)?,
        hash_before.clone(),
    );
    let id = install_id_in(&run.output)?;
    let backups = Path::new(INSTALLS_DIR).join(&id).join("backups");
    let names: Vec<String> = fs::read_dir(&backups)?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    checks.that(
        "the backup is a copy, since a file cannot be moved between drives",
        names.iter().any(|name| name.ends_with(".copied")),
        format!("{names:?}"),
    );
    let run = uninstall_quietly(&mut checks, &id, "")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    checks.same(
        "what it replaced came back",
        sha256_file(&plain_file)?,
        changed_hash,
    );

    what_only_administrators_may_change_they_alone_can(&mut checks, &probe)?;

    checks.finish()
}

/// With a real account that is not an administrator. Everything an uninstaller trusts is here,
/// and it runs elevated.
fn what_only_administrators_may_change_they_alone_can(
    checks: &mut Checks,
    probe: &str,
) -> TestResult {
    section("An account that is not an administrator cannot change what an uninstaller trusts");
    let package = hand_built_package("WAM E2E Protected Source")?;
    let installer = package.build("protected")?;
    let target = format!(r"{PROGRAM_FILES}\WAM E2E Protected");
    let force = fs::create_dir_all(&target).and_then(|()| {
        fs::write(
            Path::new(&target).join("WAM E2E Protected Source.exe"),
            b"something to replace",
        )
    });
    force?;
    let run = run_installer(
        &installer,
        &["--target-root", &target, "--apply", "--force"],
    )?;
    checks.that("the install succeeded", run.success, &run.output);
    let id = install_id_in(&run.output)?;
    let folder = format!(r"{INSTALLS_DIR}\{id}");
    let uninstaller = format!(r"{folder}\uninstall.exe");
    check_is_protected(checks, &folder)?;

    let probe_with = |marker: &str,
                      argument: &str,
                      path: &str|
     -> TestResult<Option<BTreeMap<String, String>>> {
        remove_marker(marker);
        let image = format!("\"{probe}\" --marker {marker} {argument} \"{path}\"");
        must("sc.exe", &["config", "WamE2EProbe", "binPath=", &image])?;
        let _ = tool("sc.exe", &["start", "WamE2EProbe"])?;
        Ok(wait_for_marker(marker, Duration::from_secs(60)))
    };
    for (name, path) in [
        ("the per-install folder", format!(r"{folder}\planted.txt")),
        ("its journal", format!(r"{folder}\journal.jsonl")),
        ("its uninstaller", uninstaller.clone()),
        ("its backups", format!(r"{folder}\backups\planted.txt")),
        (
            "a backup itself",
            format!(r"{folder}\backups\00000001.moved"),
        ),
        (
            "the installs folder",
            format!(r"{INSTALLS_DIR}\planted.txt"),
        ),
        ("the logs folder", format!(r"{LOGS_DIR}\planted.log")),
        ("the state folder", format!(r"{STATE_ROOT}\planted.txt")),
    ] {
        let exists_before = Path::new(&path).exists();
        let marker = probe_with("protected-write", "--try-write", &path)?;
        let result = marker_field(&marker, "probe");
        let who = marker_field(&marker, "who");
        checks.that(
            &format!("LocalService could not write to {name}"),
            result.starts_with("refused") && who.contains("S-1-5-19"),
            format!("{who}: {result}"),
        );
        checks.same(
            &format!("nothing was planted in {name}"),
            Path::new(&path).exists(),
            exists_before,
        );
    }
    let journal_read = probe_with(
        "protected-read",
        "--try-read",
        &format!(r"{folder}\journal.jsonl"),
    )?;
    let result = marker_field(&journal_read, "read");
    checks.that(
        "LocalService cannot read the journal, which can hold what values held",
        result.starts_with("refused"),
        &result,
    );
    // With UAC on, that is as far as it gets. This runner has it off, so it just runs, with no
    // arguments and no rights, and does nothing. What matters is that Windows could read it.
    let started = probe_with("protected-run", "--try-run", &uninstaller)?;
    let started = marker_field(&started, "started");
    checks.that(
        "LocalService can start the uninstaller, so it can be started from Settings by someone \
         who is not elevated yet",
        started == "ran" || started == format!("refused: {ERROR_ELEVATION_REQUIRED}"),
        &started,
    );
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Downloads
// ---------------------------------------------------------------------------------------

/// A redirect is where a download would be led off HTTPS. This needs a public server that
/// redirects to plain HTTP, and if none answers it says so, rather than passing silently or
/// failing a build over someone else's service.
#[test]
fn downloads_stay_on_https() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let work = tempfile::tempdir()?;
    let destination = work.path().join("download.bin");
    let mut proven = false;
    for url in [
        "https://httpbin.org/redirect-to?url=http%3A%2F%2Fexample.com%2F",
        "https://httpbingo.org/redirect-to?url=http%3A%2F%2Fexample.com%2F",
    ] {
        let curl = tool("curl.exe", &["-sI", "--max-time", "30", url])?;
        println!("curl -I {url}:\n{}", text(&curl.stdout));
        let result = download(url, &destination);
        println!("download {url}: {result:?}");
        checks.that(
            "nothing was downloaded off HTTPS",
            result.is_err() && !destination.exists(),
            format!("{result:?}"),
        );
        if result.is_err_and(|error| error.to_string().contains("https_only")) {
            proven = true;
            break;
        }
    }
    println!(
        "{}",
        if proven {
            "PROVEN: a real redirect from HTTPS to HTTP was refused by https_only"
        } else {
            "NOT PROVEN: no public server gave a redirect to plain HTTP in this run"
        }
    );
    checks.finish()
}
