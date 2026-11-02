//! The rest of what changes a machine or reads another PC's, on GitHub's throwaway Windows runner
//! and nowhere else: a real drive from another PC, real second accounts, and every real scheduled
//! task. See `elevated_round_trip.rs` for the install itself and all the ways it comes back out.
//!
//! The drive is a real NTFS VHD laid out as a Windows install, with hives `reg save` wrote and a
//! scan that goes through the real code at another letter. It is hostile on purpose. Its values
//! point at a marker on this machine in every way they can be written, and links on it lead back
//! here. What happened is checked with Windows' own tools, not the app's verifier.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do nothing
//! unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and `WAM_ELEVATED_E2E=1` are
//! all set. Never set those on a real PC. With all three set and no elevation they fail, so that
//! the job cannot pass without having run.
//!
//! Every check is made even after one fails, so one run shows everything that is wrong.

use std::collections::hash_map::RandomState;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::hash::{BuildHasher, Hasher};
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use winappmigrator_core::path_utils::{POWERSHELL_UTF8_OUTPUT, signed_in_user};
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::task_scan::read_task_files;
use winappmigrator_core::{
    AppSelection, EmbeddedPackage, InstalledApp, InstallerStub, PackageManifest, PackageOptions,
    ProfileChoice, RegistryHive, ScanSource, SystemContext, create_package,
    default_install_location, list_installed_apps_from, read_embedded_manifest, survey_profiles,
    sweep_leaked_hive_mounts,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const DRIVE: &str = r"V:\";
const DRIVE_FOLDER: &str = "V:";
const APP: &str = "WAM Offline App";
const VENDOR: &str = "WAM Offline Vendor";
const LIVE_ROOT: &str = r"C:\Program Files\WAM Offline App";
const TASK_FOLDER: &str = r"\WAM Offline\";
const TASK_NAME: &str = "Wartung Größe";
const SERVICE: &str = "WamOfflineSvc";
const SOURCE_COMPUTER: &str = "WAM-SOURCE-PC";
const SOURCE_BUILD: &str = "12345";
/// A second account on the drive, with data of its own that is nobody else's.
const OTHER_ON_DRIVE: &str = "wam.other";
const OTHER_SID_ON_DRIVE: &str = "S-1-5-21-1111-2222-3333-1001";
/// On this machine, named as a folder an app would have, and to be found in no package.
const HOST_FOLDER: &str = r"C:\WAM Host Secret";
const HOST_SECRET: &str = "wam-host-secret-must-not-be-captured";
const OTHER_SECRET: &str = "wam-other-account-private";
/// In this machine's registry, where a link in the drive's registry leads.
const HOST_REGISTRY_KEY: &str = r"HKLM\SOFTWARE\WAM Host Registry Secret";
const HOST_REGISTRY_TARGET: &str = r"\REGISTRY\MACHINE\SOFTWARE\WAM Host Registry Secret";
const HOST_USER_KEY: &str = r"Software\WAM Host User Secret";
/// Where this machine's own marker is, by a share of its own, which is enough to see a connection.
const HOST_SHARE: &str = r"\\127.0.0.1\C$\WAM Host Secret";

const UNINSTALL_KEY: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM Offline App";
const APP_PATHS_KEY: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\App Paths\WAM Offline App.exe";
const CLSID: &str = "{5A7E6E0C-E2E0-4000-8000-00000000AB01}";
const USER_CLSID: &str = "{5A7E6E0C-E2E0-4000-8000-00000000AB02}";
const MACHINE_KEY: &str = r"HKLM\Software\WAM Offline Vendor\WAM Offline App";
const USER_KEY: &str = r"HKCU\Software\WAM Offline Vendor\WAM Offline App";
const MACHINE_RUN: &str = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Run";
const USER_RUN: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
const MACHINE_ENVIRONMENT: &str =
    r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment";
const USER_ENVIRONMENT: &str = r"HKCU\Environment";
const USER_SHELL_FOLDERS: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders";
const SAVED_GAMES_VALUE: &str = "{4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}";
const HIVES_FOLDER: &str = r"C:\ProgramData\WinAppMigrator\hives";
const INSTALLS_DIR: &str = r"C:\ProgramData\WinAppMigrator\installs";
const SELF_REMOVAL_PATIENCE: Duration = Duration::from_secs(90);
const HOLD_DRIVE_VARIABLE: &str = "WAM_E2E_HOLD_DRIVE";

// ---------------------------------------------------------------------------------------
// The guard
// ---------------------------------------------------------------------------------------

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
    for (actions, environment, flag, expected) in [
        (Some("true"), Some("github-hosted"), Some("1"), true),
        (None, None, None, false),
        (Some("true"), Some("self-hosted"), Some("1"), false),
        (Some("true"), Some("github-hosted"), Some("0"), false),
        (Some("true"), None, Some("1"), false),
        (None, Some("github-hosted"), Some("1"), false),
    ] {
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

fn reg_data(key: &str, name: Option<&str>) -> TestResult<Option<String>> {
    Ok(reg_value(key, name)?.map(|(_, data)| data))
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

/// The hives loaded under `HKEY_LOCAL_MACHINE` that this app could have named.
fn wam_mounts() -> TestResult<Vec<String>> {
    Ok(must("reg.exe", &["query", "HKLM"])?
        .lines()
        .filter_map(|line| line.trim().strip_prefix(r"HKEY_LOCAL_MACHINE\"))
        .filter(|name| name.to_ascii_uppercase().starts_with("WAM_"))
        .map(ToOwned::to_owned)
        .collect())
}

/// Every file and link under a folder, without going through a link, with what is in each file,
/// its size, and when it was written. A link's target is shown instead.
fn tree_state(root: &str) -> TestResult<BTreeMap<String, String>> {
    let shown = powershell(&format!(
        r#"$root = {};
           $sha = [Security.Cryptography.SHA256]::Create();
           function Walk($folder) {{
             foreach ($path in [IO.Directory]::EnumerateFileSystemEntries($folder)) {{
               $attributes = [IO.File]::GetAttributes($path);
               $name = $path.Substring($root.Length);
               if ($name -like 'System Volume Information*' -or $name -like '$RECYCLE.BIN*') {{ continue }}
               if ($attributes -band [IO.FileAttributes]::ReparsePoint) {{
                 $target = try {{ (Get-Item -LiteralPath $path -Force).Target -join ',' }} catch {{ 'unreadable' }};
                 '{{0}}|link|{{1}}' -f $name, $target;
               }} elseif ($attributes -band [IO.FileAttributes]::Directory) {{
                 '{{0}}|folder' -f $name;
                 Walk $path;
               }} else {{
                 $file = [IO.File]::Open($path, 'Open', 'Read', 'ReadWrite');
                 $hash = [BitConverter]::ToString($sha.ComputeHash($file)) -replace '-', '';
                 $length = $file.Length;
                 $file.Close();
                 '{{0}}|{{1}}|{{2}}|{{3}}' -f $name, $hash, $length, [IO.File]::GetLastWriteTimeUtc($path).Ticks;
               }}
             }}
           }}
           Walk $root"#,
        quoted(root)
    ))?;
    Ok(shown
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(path, state)| (path.to_owned(), state.trim().to_owned()))
        .collect())
}

/// How many connections anything has made to this machine's SMB port since this began to watch it.
/// A link to `\\\\127.0.0.1\\...` is followed to here, by Windows' own client, whatever made it do
/// that, and it is this listener that counts them. The runner's Server service is not running, so the
/// port is free, and no credentials go anywhere. One watcher for the process, since the port is one.
fn smb_connections() -> TestResult<usize> {
    static WATCH: OnceLock<Result<Arc<AtomicUsize>, String>> = OnceLock::new();
    let watch = WATCH.get_or_init(|| {
        let listener = std::net::TcpListener::bind("127.0.0.1:445").map_err(|error| {
            format!("something has this machine's SMB port, so it cannot be watched: {error}")
        })?;
        let count = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&count);
        std::thread::spawn(move || {
            for connection in listener.incoming() {
                counter.fetch_add(1, Ordering::SeqCst);
                drop(connection);
            }
        });
        Ok(count)
    });
    match watch {
        Ok(count) => Ok(count.load(Ordering::SeqCst)),
        Err(reason) => Err(reason.clone().into()),
    }
}

/// What is different between two looks at a folder.
fn tree_differences(
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut differences = Vec::new();
    for (path, state) in before {
        match after.get(path) {
            None => differences.push(format!("gone: {path}")),
            Some(now) if now != state => differences.push(format!("changed: {path}")),
            Some(_) => {}
        }
    }
    for path in after.keys() {
        if !before.contains_key(path) {
            differences.push(format!("new: {path}"));
        }
    }
    differences
}

/// Any file under a folder that holds this text, by Windows' own search.
fn files_holding(folder: &Path, needle: &str) -> TestResult<String> {
    powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Recurse -File -Force -ErrorAction SilentlyContinue | \
         Select-String -SimpleMatch -Encoding Unicode -Pattern {needle} -List | \
         ForEach-Object {{ $_.Path }}; \
         Get-ChildItem -LiteralPath {} -Recurse -File -Force -ErrorAction SilentlyContinue | \
         Select-String -SimpleMatch -Pattern {needle} -List | ForEach-Object {{ $_.Path }}",
        quoted(&folder.display().to_string()),
        quoted(&folder.display().to_string()),
        needle = quoted(needle),
    ))
}

struct InstallerRun {
    success: bool,
    output: String,
}

fn run_installer(installer: &Path, args: &[&str]) -> TestResult<InstallerRun> {
    println!("\n--- {} {}", installer.display(), args.join(" "));
    let started = Instant::now();
    let output = Command::new(installer).args(args).output()?;
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    println!(
        "{shown}--- exit: {}, after {:?}",
        output.status,
        started.elapsed()
    );
    Ok(InstallerRun {
        success: output.status.success(),
        output: shown,
    })
}

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

fn install_id_in(output: &str) -> TestResult<String> {
    Ok(output
        .lines()
        .find_map(|line| line.strip_prefix("Install id: "))
        .ok_or("the install did not say its id")?
        .trim()
        .to_owned())
}

/// Run an install's uninstaller as Windows would be told to, and wait for its folder to go.
fn uninstall_quietly(checks: &mut Checks, id: &str, extra: &str) -> TestResult<InstallerRun> {
    let program = format!(r"{INSTALLS_DIR}\{id}\uninstall.exe");
    let mut command = Command::new(&program);
    command.raw_arg(format!("--uninstall {id} --quiet{extra}"));
    println!("\n--- {program} --uninstall {id} --quiet{extra}");
    let output = command.output()?;
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    println!("{shown}--- exit: {}", output.status);
    let folder = Path::new(INSTALLS_DIR).join(id);
    let started = Instant::now();
    while folder.exists() && output.status.success() && started.elapsed() < SELF_REMOVAL_PATIENCE {
        std::thread::sleep(Duration::from_millis(250));
    }
    if output.status.success() {
        checks.that(
            "the uninstaller's folder went once it had exited",
            !folder.exists(),
            folder.display(),
        );
    }
    Ok(InstallerRun {
        success: output.status.success(),
        output: shown,
    })
}

/// A program with nothing in it, for what has to be a real executable.
fn fake_program() -> TestResult<PathBuf> {
    let folder = std::env::temp_dir().join("wam-e2e-offline-program");
    let program = folder.join("fake-app.exe");
    if program.is_file() {
        return Ok(program);
    }
    fs::create_dir_all(&folder)?;
    let source = folder.join("fake_app.rs");
    fs::write(&source, "fn main() {}\n")?;
    must(
        "rustc",
        &[
            "--edition",
            "2021",
            "-o",
            &program.display().to_string(),
            &source.display().to_string(),
        ],
    )?;
    Ok(program)
}

fn current_sid() -> TestResult<String> {
    let line = must("whoami", &["/user", "/fo", "csv", "/nh"])?;
    Ok(line
        .trim()
        .rsplit(',')
        .next()
        .unwrap_or_default()
        .trim_matches('"')
        .to_owned())
}

fn utf16_file(path: &Path, content: &str) -> TestResult {
    let mut bytes = vec![0xFF, 0xFE];
    bytes.extend(content.encode_utf16().flat_map(u16::to_le_bytes));
    Ok(fs::write(path, bytes)?)
}

fn robocopy(from: &str, to: &str) -> TestResult {
    let output = tool(
        "robocopy.exe",
        &[
            from, to, "/E", "/B", "/R:1", "/W:1", "/NFL", "/NDL", "/NJH", "/NJS", "/NP",
        ],
    )?;
    // Anything under 8 is a success, by what it copied.
    if output.status.code().is_some_and(|code| code < 8) {
        return Ok(());
    }
    Err(format!(
        "robocopy {from} {to} failed with {}: {}",
        output.status,
        text(&output.stdout)
    )
    .into())
}

fn junction(link: &str, target: &str) -> TestResult {
    let link_parent = Path::new(link).parent().ok_or("a link has a folder")?;
    fs::create_dir_all(link_parent)?;
    must("cmd", &["/C", "mklink", "/J", link, target]).map(|_| ())
}

fn create_shortcut(path: &str, target: &str, arguments: &str) -> TestResult {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    powershell(&format!(
        "$link = (New-Object -ComObject WScript.Shell).CreateShortcut({}); \
         $link.TargetPath = {}; $link.Arguments = {}; $link.Save()",
        quoted(path),
        quoted(target),
        quoted(arguments),
    ))
    .map(|_| ())
}

/// Target and arguments, as read back through the shell.
fn read_shortcut(path: &str) -> TestResult<Option<[String; 2]>> {
    if !Path::new(path).is_file() {
        return Ok(None);
    }
    let shown = powershell(&format!(
        "$link = (New-Object -ComObject WScript.Shell).CreateShortcut({}); \
         $link.TargetPath; '|'; $link.Arguments",
        quoted(path)
    ))?;
    let mut parts = shown.split('|').map(|part| part.trim().to_owned());
    Ok(Some([
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default(),
    ]))
}

fn attach_ntfs_volume(folder: &Path) -> TestResult {
    let vhd = folder.join("wam-e2e-source-drive.vhd");
    let script = folder.join("diskpart.txt");
    fs::write(
        &script,
        format!(
            "create vdisk file=\"{vhd}\" maximum=8192 type=expandable\r\n\
             select vdisk file=\"{vhd}\"\r\n\
             attach vdisk\r\n\
             create partition primary\r\n\
             format fs=ntfs quick label=WAMSOURCE\r\n\
             assign letter=V\r\n",
            vhd = vhd.display()
        ),
    )?;
    println!(
        "{}",
        must("diskpart.exe", &["/s", &script.display().to_string()])?
    );
    let started = Instant::now();
    while !Path::new(DRIVE).exists() {
        if started.elapsed() > Duration::from_secs(60) {
            return Err("the NTFS volume did not appear".into());
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(())
}

/// Registry work on a hive file, with a name this app would never give a mount.
fn edit_hive(file: &str, edit: impl FnOnce(&str) -> TestResult) -> TestResult {
    let key = r"HKLM\WamTestEdit";
    must("reg.exe", &["load", key, file])?;
    let result = edit(key);
    // Windows takes a moment to let go of what was just written.
    let started = Instant::now();
    loop {
        let unloaded = tool("reg.exe", &["unload", key])?;
        if unloaded.status.success() {
            break;
        }
        if started.elapsed() > Duration::from_secs(30) {
            return Err(format!(
                "the hive {file} would not unload: {}",
                text(&unloaded.stderr)
            )
            .into());
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    // What Windows put beside the hive is its own business, and not part of the drive being read.
    for suffix in [".LOG1", ".LOG2"] {
        let _ = fs::remove_file(format!("{file}{suffix}"));
    }
    if let Some(parent) = Path::new(file).parent() {
        for entry in fs::read_dir(parent)?.flatten() {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if name.contains(".regtrans-ms") || name.ends_with(".blf") {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    result
}

// ---------------------------------------------------------------------------------------
// A real drive from another PC
// ---------------------------------------------------------------------------------------

struct Paths {
    profile: String,
    profile_name: String,
    sid: String,
    desktop: String,
    start_menu: String,
    common_desktop: String,
    common_start_menu: String,
    roaming: String,
    local: String,
    documents: String,
    saved_games: String,
    program_data: String,
}

fn paths() -> TestResult<Paths> {
    let profile = std::env::var("USERPROFILE")?;
    let profile_name = Path::new(&profile)
        .file_name()
        .ok_or("the profile has a name")?
        .to_string_lossy()
        .into_owned();
    let shown = powershell(
        "'Desktop', 'StartMenu', 'CommonDesktopDirectory', 'CommonStartMenu' | \
         ForEach-Object { [Environment]::GetFolderPath($_) }",
    )?;
    let mut lines = shown.lines().map(str::trim).map(ToOwned::to_owned);
    let mut next = || -> TestResult<String> {
        lines
            .next()
            .filter(|line| !line.is_empty())
            .ok_or_else(|| "a shell folder is missing".into())
    };
    Ok(Paths {
        desktop: next()?,
        start_menu: next()?,
        common_desktop: next()?,
        common_start_menu: next()?,
        roaming: std::env::var("APPDATA")?,
        local: std::env::var("LOCALAPPDATA")?,
        documents: format!(r"{profile}\Documents"),
        saved_games: format!(r"{profile}\Saved Games"),
        program_data: std::env::var("ProgramData")?,
        sid: current_sid()?,
        profile,
        profile_name,
    })
}

fn exe_of(root: &str) -> String {
    format!(r"{root}\bin\{APP}.exe")
}

/// `(relative to the app's folder, content)`.
fn app_files() -> Vec<(&'static str, &'static [u8])> {
    vec![
        (r"conf\app.conf", b"setting=1\r\n"),
        (r"plugins\Größe.txt", "größe\r\n".as_bytes()),
    ]
}

/// `(where, content)`: what the app keeps outside its folder.
fn app_data(paths: &Paths) -> Vec<(String, &'static [u8])> {
    vec![
        (format!(r"{}\{APP}\settings.json", paths.roaming), b"{}"),
        (format!(r"{}\{APP}\cache.db", paths.local), b"cache"),
        (format!(r"{}\{APP}\doc.txt", paths.documents), b"document"),
        (format!(r"{}\{APP}\slot.sav", paths.saved_games), b"saved"),
        (
            format!(r"{}\{APP}\shared.dat", paths.program_data),
            b"shared",
        ),
    ]
}

/// The app, on this machine, by Windows' own tools.
fn create_live_app(paths: &Paths, program: &Path) -> TestResult {
    let root = LIVE_ROOT;
    let exe = exe_of(root);
    let bin = format!(r"{root}\bin");
    fs::create_dir_all(&bin)?;
    fs::copy(program, &exe)?;
    fs::copy(program, format!(r"{bin}\helper.exe"))?;
    fs::copy(program, format!(r"{bin}\Shell.dll"))?;
    for (relative, content) in app_files() {
        let path = Path::new(root).join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    for (path, content) in app_data(paths) {
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }

    for (name, data) in [
        ("DisplayName", APP),
        ("Publisher", VENDOR),
        ("DisplayVersion", "2.0.0"),
        ("InstallLocation", root),
    ] {
        reg_add(UNINSTALL_KEY, Some(name), "REG_SZ", data)?;
    }
    reg_add(
        UNINSTALL_KEY,
        Some("DisplayIcon"),
        "REG_SZ",
        &format!("{exe},0"),
    )?;
    reg_add(APP_PATHS_KEY, None, "REG_SZ", &exe)?;
    reg_add(APP_PATHS_KEY, Some("Path"), "REG_SZ", &bin)?;
    reg_add(MACHINE_KEY, Some("InstallDir"), "REG_SZ", root)?;
    reg_add(
        USER_KEY,
        Some("LastFolder"),
        "REG_SZ",
        &format!(r"{root}\plugins"),
    )?;
    reg_add(
        MACHINE_RUN,
        Some(APP),
        "REG_SZ",
        &format!("\"{exe}\" --tray"),
    )?;
    reg_add(USER_RUN, Some(APP), "REG_SZ", &format!("\"{exe}\" --user"))?;
    reg_add(
        MACHINE_ENVIRONMENT,
        Some("WAM_OFFLINE_HOME"),
        "REG_SZ",
        root,
    )?;
    reg_add(
        USER_ENVIRONMENT,
        Some("WAM_OFFLINE_USER_HOME"),
        "REG_SZ",
        &format!(r"{root}\conf"),
    )?;
    let server = format!(r"{bin}\Shell.dll");
    reg_add(
        &format!(r"HKLM\Software\Classes\CLSID\{CLSID}\InprocServer32"),
        None,
        "REG_SZ",
        &server,
    )?;
    reg_add(
        r"HKLM\Software\Classes\.wamoff",
        None,
        "REG_SZ",
        "WamOffline.Document",
    )?;
    reg_add(
        r"HKLM\Software\Classes\WamOffline.Document\shell\open\command",
        None,
        "REG_SZ",
        &format!("\"{exe}\" \"%1\""),
    )?;
    // These live in `UsrClass.dat`, which is its own hive.
    reg_add(
        &format!(r"HKCU\Software\Classes\CLSID\{USER_CLSID}\InprocServer32"),
        None,
        "REG_SZ",
        &server,
    )?;
    reg_add(
        r"HKCU\Software\Classes\.wamoffu",
        None,
        "REG_SZ",
        "WamOffline.UserDocument",
    )?;
    reg_add(
        r"HKCU\Software\Classes\WamOffline.UserDocument\shell\open\command",
        None,
        "REG_SZ",
        &format!("\"{exe}\" \"%1\""),
    )?;
    must(
        "sc.exe",
        &[
            "create",
            SERVICE,
            "binPath=",
            &format!("\"{exe}\" --service"),
            "start=",
            "demand",
        ],
    )?;
    let definition = std::env::temp_dir().join("wam-offline-task.xml");
    utf16_file(
        &definition,
        &format!(
            r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <Triggers><TimeTrigger><StartBoundary>2099-01-01T00:00:00</StartBoundary></TimeTrigger></Triggers>
  <Principals><Principal id="Author"><UserId>S-1-5-18</UserId><RunLevel>HighestAvailable</RunLevel></Principal></Principals>
  <Settings><Enabled>false</Enabled></Settings>
  <Actions Context="Author">
    <Exec><Command>"{exe}"</Command><Arguments>--first</Arguments></Exec>
    <Exec><Command>{bin}\helper.exe</Command><Arguments>--second</Arguments></Exec>
  </Actions>
</Task>"#
        ),
    )?;
    must(
        "schtasks.exe",
        &[
            "/create",
            "/tn",
            &format!("{TASK_FOLDER}{TASK_NAME}"),
            "/xml",
            &definition.display().to_string(),
            "/f",
        ],
    )?;
    for (folder, name) in shortcut_places(paths) {
        create_shortcut(&format!(r"{folder}\{name}.lnk"), &exe, "--from-shortcut")?;
    }
    Ok(())
}

fn shortcut_places(paths: &Paths) -> Vec<(String, String)> {
    vec![
        (paths.desktop.clone(), APP.to_owned()),
        (paths.common_desktop.clone(), format!("{APP} Everyone")),
        (
            format!(r"{}\Programs\{APP}", paths.start_menu),
            APP.to_owned(),
        ),
        (
            format!(r"{}\Programs\{APP}", paths.common_start_menu),
            APP.to_owned(),
        ),
        // Failing to replace this fails an install, and shortcuts are nearly the last thing done.
        (startup_folder(paths), format!("{APP} Startup")),
    ]
}

fn startup_folder(paths: &Paths) -> String {
    format!(r"{}\Programs\Startup", paths.start_menu)
}

/// Lay a real Windows install out on the volume, from what is really on this machine.
fn lay_out_the_drive(paths: &Paths) -> TestResult {
    let config = format!(r"{DRIVE_FOLDER}\Windows\System32\config");
    fs::create_dir_all(&config)?;
    let profile = format!(r"{DRIVE_FOLDER}\Users\{}", paths.profile_name);
    let hive_folder = format!(r"{profile}\AppData\Local\Microsoft\Windows");
    fs::create_dir_all(&hive_folder)?;
    let started = Instant::now();
    must(
        "reg.exe",
        &[
            "save",
            r"HKLM\SOFTWARE",
            &format!(r"{config}\SOFTWARE"),
            "/y",
        ],
    )?;
    must(
        "reg.exe",
        &["save", r"HKLM\SYSTEM", &format!(r"{config}\SYSTEM"), "/y"],
    )?;
    must(
        "reg.exe",
        &["save", "HKCU", &format!(r"{profile}\NTUSER.DAT"), "/y"],
    )?;
    must(
        "reg.exe",
        &[
            "save",
            &format!(r"HKU\{}_Classes", paths.sid),
            &format!(r"{hive_folder}\UsrClass.dat"),
            "/y",
        ],
    )?;
    println!("saved the hives in {:?}", started.elapsed());

    robocopy(LIVE_ROOT, &format!(r"{DRIVE_FOLDER}\Program Files\{APP}"))?;
    robocopy(
        &format!(r"{}\{APP}", paths.roaming),
        &format!(r"{profile}\AppData\Roaming\{APP}"),
    )?;
    robocopy(
        &format!(r"{}\{APP}", paths.local),
        &format!(r"{profile}\AppData\Local\{APP}"),
    )?;
    robocopy(
        &format!(r"{}\{APP}", paths.documents),
        &format!(r"{profile}\Documents\{APP}"),
    )?;
    robocopy(
        &format!(r"{}\{APP}", paths.saved_games),
        &format!(r"{profile}\Saved Games\{APP}"),
    )?;
    robocopy(
        &format!(r"{}\{APP}", paths.program_data),
        &format!(r"{DRIVE_FOLDER}\ProgramData\{APP}"),
    )?;
    robocopy(
        r"C:\Windows\System32\Tasks\WAM Offline",
        &format!(r"{DRIVE_FOLDER}\Windows\System32\Tasks\WAM Offline"),
    )?;
    let relative_to_profile = |folder: &str| -> String {
        folder
            .strip_prefix(paths.profile.as_str())
            .unwrap_or(folder)
            .to_owned()
    };
    let relative_to_drive = |folder: &str| folder.get(2..).unwrap_or(folder).to_owned();
    for (from, to) in [
        (
            paths.desktop.clone(),
            format!(r"{profile}{}", relative_to_profile(&paths.desktop)),
        ),
        (
            paths.common_desktop.clone(),
            format!(
                r"{DRIVE_FOLDER}{}",
                relative_to_drive(&paths.common_desktop)
            ),
        ),
    ] {
        fs::create_dir_all(&to)?;
        for entry in fs::read_dir(&from)?.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(APP) && name.ends_with(".lnk") {
                fs::copy(entry.path(), format!(r"{to}\{name}"))?;
            }
        }
    }
    robocopy(
        &format!(r"{}\Programs\{APP}", paths.start_menu),
        &format!(
            r"{profile}{}\Programs\{APP}",
            relative_to_profile(&paths.start_menu)
        ),
    )?;
    robocopy(
        &format!(r"{}\Programs\{APP}", paths.common_start_menu),
        &format!(
            r"{DRIVE_FOLDER}{}\Programs\{APP}",
            relative_to_drive(&paths.common_start_menu)
        ),
    )?;
    let startup_on_drive = format!(r"{profile}{}", relative_to_profile(&startup_folder(paths)));
    fs::create_dir_all(&startup_on_drive)?;
    fs::copy(
        format!(r"{}\{APP} Startup.lnk", startup_folder(paths)),
        format!(r"{startup_on_drive}\{APP} Startup.lnk"),
    )?;

    // A second account, with a hive of its own that tries to take the first's Documents for its
    // own, and a private file named for the app.
    let other = format!(r"{DRIVE_FOLDER}\Users\{OTHER_ON_DRIVE}");
    fs::create_dir_all(format!(r"{other}\AppData\Roaming\{APP}"))?;
    fs::write(
        format!(r"{other}\AppData\Roaming\{APP}\private.txt"),
        OTHER_SECRET,
    )?;
    fs::copy(
        format!(r"{profile}\NTUSER.DAT"),
        format!(r"{other}\NTUSER.DAT"),
    )?;
    let vendor_key = format!(r"Software\{VENDOR}\{APP}");
    let main_profile = paths.profile.clone();
    edit_hive(&format!(r"{other}\NTUSER.DAT"), |key| {
        reg_add(
            &format!(r"{key}\{vendor_key}"),
            Some("Owner"),
            "REG_SZ",
            OTHER_SECRET,
        )?;
        reg_add(
            &format!(r"{key}\{USER_SHELL_FOLDERS}"),
            Some("Personal"),
            "REG_EXPAND_SZ",
            &format!(r"{main_profile}\Documents"),
        )
    })?;
    Ok(())
}

/// Make a key in a hive that is a link to somewhere else in the registry. Only the Windows API can,
/// and this is a test, so it can use it. A registry link in a hive would be followed by opening
/// it, as a link on a disk is.
fn registry_link(parent_key: &str, name: &str, target: &str) -> TestResult {
    let parent = parent_key
        .strip_prefix(r"HKLM\")
        .ok_or("the key has to be under HKLM")?;
    powershell(&format!(
        r#"Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class WamRegistry {{
    [DllImport("advapi32.dll", CharSet = CharSet.Unicode)]
    public static extern int RegCreateKeyExW(IntPtr key, string subKey, int reserved, string className, int options, int access, IntPtr security, out IntPtr result, out int disposition);
    [DllImport("advapi32.dll", CharSet = CharSet.Unicode)]
    public static extern int RegSetValueExW(IntPtr key, string name, int reserved, int type, byte[] data, int size);
    [DllImport("advapi32.dll")]
    public static extern int RegCloseKey(IntPtr key);
}}
'@;
           $machine = [IntPtr]::new([long]0x80000002 - 0x100000000);
           $created = [IntPtr]::Zero; $disposition = 0;
           $code = [WamRegistry]::RegCreateKeyExW($machine, {key}, 0, $null, 2, 0xF003F, [IntPtr]::Zero, [ref]$created, [ref]$disposition);
           if ($code -ne 0) {{ throw "RegCreateKeyEx: $code" }};
           $bytes = [Text.Encoding]::Unicode.GetBytes({target});
           $code = [WamRegistry]::RegSetValueExW($created, 'SymbolicLinkValue', 0, 6, $bytes, $bytes.Length);
           if ($code -ne 0) {{ throw "RegSetValueEx: $code" }};
           [void][WamRegistry]::RegCloseKey($created)"#,
        key = quoted(&format!(r"{parent}\{name}")),
        target = quoted(target),
    ))
    .map(|_| ())
}

/// What makes the drive's registry the other PC's and not a copy of this one's, and what makes it
/// hostile.
fn edit_the_drives_registry(paths: &Paths) -> TestResult {
    let config = format!(r"{DRIVE_FOLDER}\Windows\System32\config");
    edit_hive(&format!(r"{config}\SYSTEM"), |key| {
        let current = reg_data(&format!(r"{key}\Select"), Some("Current"))?
            .ok_or("the hive has no current control set")?;
        let number = u32::from_str_radix(current.trim_start_matches("0x"), 16)?;
        let control_set = format!(r"{key}\ControlSet{number:03}");
        reg_add(
            &format!(r"{control_set}\Control\ComputerName\ComputerName"),
            Some("ComputerName"),
            "REG_SZ",
            SOURCE_COMPUTER,
        )?;
        // A service that names this machine's files. It is no part of the app, and must not be
        // looked at for what it says.
        for (name, image) in [
            (
                "WamOfflineRooted",
                r"\WAM Host Secret\WAM Offline App\host.exe".to_owned(),
            ),
            (
                "WamOfflineClimb",
                format!(r"{LIVE_ROOT}\..\..\..\WAM Host Secret\WAM Offline App\host.exe"),
            ),
        ] {
            let service = format!(r"{control_set}\Services\{name}");
            reg_add(&service, Some("ImagePath"), "REG_EXPAND_SZ", &image)?;
            reg_add(&service, Some("Type"), "REG_DWORD", "16")?;
            reg_add(&service, Some("Start"), "REG_DWORD", "3")?;
        }
        // What is captured with the app's own service, that is this machine's registry.
        let parameters = format!(r"{control_set}\Services\{SERVICE}\Parameters");
        reg_add(&parameters, Some("Marker"), "REG_SZ", "the app's own")?;
        registry_link(&parameters, "HostLink", HOST_REGISTRY_TARGET)?;
        Ok(())
    })?;
    edit_hive(&format!(r"{config}\SOFTWARE"), |key| {
        let version = format!(r"{key}\Microsoft\Windows NT\CurrentVersion");
        reg_add(&version, Some("CurrentBuildNumber"), "REG_SZ", SOURCE_BUILD)?;
        reg_add(&version, Some("UBR"), "REG_DWORD", "77")?;
        reg_add(
            &format!(r"{key}\Microsoft\Windows\CurrentVersion\Authentication\LogonUI"),
            Some("LastLoggedOnUserSID"),
            "REG_SZ",
            &paths.sid,
        )?;
        // The second account, and records that try to make this machine's folders an account.
        let profiles = format!(r"{version}\ProfileList");
        for (sid, folder) in [
            (
                OTHER_SID_ON_DRIVE,
                format!(r"%SystemDrive%\Users\{OTHER_ON_DRIVE}"),
            ),
            ("S-1-5-21-1111-2222-3333-1002", HOST_FOLDER[2..].to_owned()),
            (
                "S-1-5-21-1111-2222-3333-1003",
                format!(r"C:\Users\..\..{}", &HOST_FOLDER[2..]),
            ),
            (
                "S-1-5-21-1111-2222-3333-1004",
                r"\\127.0.0.1\C$\WAM Host Secret".to_owned(),
            ),
        ] {
            reg_add(
                &format!(r"{profiles}\{sid}"),
                Some("ProfileImagePath"),
                "REG_EXPAND_SZ",
                &folder,
            )?;
        }
        // Values that say the app is on this machine too, every way that can be written.
        let host_data = format!(r"{}\{APP}", &HOST_FOLDER[2..]);
        let on_the_host = [
            host_data.clone(),
            format!(r"{LIVE_ROOT}\..\..\..{host_data}"),
            format!(r"..\..\..\..\..\..{host_data}"),
            format!(r"\\127.0.0.1\C${host_data}"),
            format!(r"C:\Program Files\{APP}\bin"),
        ]
        .join(";");
        reg_add(
            &format!(r"{key}\Microsoft\Windows\CurrentVersion\App Paths\{APP}.exe"),
            Some("Path"),
            "REG_SZ",
            &on_the_host,
        )?;
        // A key under the app's own, which is all captured, that is this machine's registry.
        registry_link(
            &format!(r"{key}\{VENDOR}\{APP}"),
            "HostLink",
            HOST_REGISTRY_TARGET,
        )?;
        // A whole app that is on this machine, by its uninstall entry.
        let hostile = format!(r"{key}\Microsoft\Windows\CurrentVersion\Uninstall\WAM Hostile App");
        reg_add(&hostile, Some("DisplayName"), "REG_SZ", "WAM Hostile App")?;
        reg_add(&hostile, Some("InstallLocation"), "REG_SZ", &host_data)?;
        reg_add(
            &hostile,
            Some("UninstallString"),
            "REG_SZ",
            &format!(r"\\127.0.0.1\C${host_data}\uninst.exe"),
        )?;
        reg_add(
            &hostile,
            Some("DisplayIcon"),
            "REG_SZ",
            &format!(r"C:\Windows\..{host_data}\host.exe,0"),
        )?;
        // Apps whose files are links to a share, where only looking at one would connect to it.
        for (name, location, icon) in [
            (
                "WAM Share App",
                r"C:\Program Files\WAM Share App".to_owned(),
                r"C:\Program Files\WAM Share App\share.exe,0".to_owned(),
            ),
            (
                "WAM Share Squirrel App",
                format!(r"{}\AppData\Local\WAM Share Squirrel App", paths.profile),
                format!(
                    r"{}\AppData\Local\WAM Share Squirrel App\Update.exe,0",
                    paths.profile
                ),
            ),
        ] {
            let entry = format!(r"{key}\Microsoft\Windows\CurrentVersion\Uninstall\{name}");
            reg_add(&entry, Some("DisplayName"), "REG_SZ", name)?;
            reg_add(&entry, Some("InstallLocation"), "REG_SZ", &location)?;
            reg_add(&entry, Some("DisplayIcon"), "REG_SZ", &icon)?;
        }
        Ok(())
    })?;
    // The same in an account's own hive, that is this machine's account.
    edit_hive(
        &format!(r"{DRIVE_FOLDER}\Users\{}\NTUSER.DAT", paths.profile_name),
        |key| {
            registry_link(
                &format!(r"{key}\Software\{VENDOR}\{APP}"),
                "HostUserLink",
                &format!(r"\REGISTRY\USER\{}\{HOST_USER_KEY}", paths.sid),
            )
        },
    )?;
    // A known folder that is a link to a share.
    edit_hive(
        &format!(r"{DRIVE_FOLDER}\Users\{OTHER_ON_DRIVE}\NTUSER.DAT"),
        |key| {
            reg_add(
                &format!(r"{key}\{USER_SHELL_FOLDERS}"),
                Some(SAVED_GAMES_VALUE),
                "REG_EXPAND_SZ",
                &format!(r"C:\Users\{OTHER_ON_DRIVE}\ShareGames"),
            )
        },
    )?;
    Ok(())
}

/// A marker on this machine, and links on the drive that lead to it. Each would be followed by
/// anything that took a link on another PC's disk at its word.
fn plant_the_links(paths: &Paths, program: &Path) -> TestResult {
    let host_data = format!(r"{HOST_FOLDER}\{APP}");
    fs::create_dir_all(&host_data)?;
    fs::write(format!(r"{host_data}\host-secret.txt"), HOST_SECRET)?;
    fs::copy(program, format!(r"{host_data}\host.exe"))?;

    let profile = format!(r"{DRIVE_FOLDER}\Users\{}", paths.profile_name);
    // Inside what is walked, in a folder named for the app, and on the way to a vendor's folder.
    junction(
        &format!(r"{DRIVE_FOLDER}\Program Files\{APP}\linked"),
        &host_data,
    )?;
    junction(&format!(r"{profile}\AppData\LocalLow\{APP}"), &host_data)?;
    junction(
        &format!(r"{DRIVE_FOLDER}\ProgramData\{VENDOR}"),
        HOST_FOLDER,
    )?;
    // An account that is this one's real profile, by a junction at the name of a profile.
    junction(&format!(r"{DRIVE_FOLDER}\Users\linked"), &paths.profile)?;
    // A folder of tasks that is this machine's.
    junction(
        &format!(r"{DRIVE_FOLDER}\Windows\System32\Tasks\Linked"),
        r"C:\Windows\System32\Tasks",
    )?;
    must(
        "cmd",
        &[
            "/C",
            "mklink",
            &format!(r"{DRIVE_FOLDER}\Program Files\{APP}\linked.txt"),
            &format!(r"{host_data}\host-secret.txt"),
        ],
    )?;
    // The same to a share, each named for what looks for that name. Looking through one at all
    // connects to it, which the end of the test can see.
    for link in [
        format!(r"{DRIVE_FOLDER}\Users\shareprofile"),
        format!(r"{DRIVE_FOLDER}\Users\{OTHER_ON_DRIVE}\ShareGames"),
        format!(r"{DRIVE_FOLDER}\Program Files\{APP}\share-linked"),
        format!(r"{DRIVE_FOLDER}\Program Files\{APP}\Application"),
        format!(r"{DRIVE_FOLDER}\Program Files\WAM Share App\bin"),
        format!(r"{DRIVE_FOLDER}\ProgramData\WAM Share App"),
        format!(r"{profile}\AppData\Local\Programs"),
        format!(r"{profile}\AppData\Local\WAM Share Squirrel App\app-1.0.0"),
    ] {
        if let Some(parent) = Path::new(&link).parent() {
            fs::create_dir_all(parent)?;
        }
        must("cmd", &["/C", "mklink", "/D", &link, HOST_SHARE])?;
    }
    // A real folder whose name opens the link beside it, since Windows drops a dot from the end of
    // a name. What a folder's listing says about it does not show that. In each place something
    // walks, and at the top of a license store, which any account can make on a PC.
    for link in [
        format!(r"{DRIVE_FOLDER}\Program Files\{APP}\twin"),
        format!(r"{DRIVE_FOLDER}\ProgramData\FlexLM\twin"),
        format!(r"{DRIVE_FOLDER}\ProgramData\Microsoft\Windows\Start Menu\Programs\twin"),
        format!(r"{DRIVE_FOLDER}\Windows\System32\Tasks\twin"),
        format!(r"{profile}\AppData\Roaming\{APP}\twin"),
    ] {
        if let Some(parent) = Path::new(&link).parent() {
            fs::create_dir_all(parent)?;
        }
        must("cmd", &["/C", "mklink", "/D", &link, HOST_SHARE])?;
        fs::create_dir(format!(r"\\?\{link}."))?;
    }
    // The same under `Users` with a space, where the link reads as the real folder until something
    // is asked for inside it.
    fs::create_dir(format!(r"{DRIVE_FOLDER}\Users\spaced"))?;
    std::os::windows::fs::symlink_dir(HOST_SHARE, format!(r"\\?\{DRIVE_FOLDER}\Users\spaced "))?;
    // A store with a real license, and a junction to this machine's marker beside it with its twin.
    fs::write(
        format!(r"{DRIVE_FOLDER}\ProgramData\FlexLM\real.lic"),
        b"a real license",
    )?;
    junction(
        &format!(r"{DRIVE_FOLDER}\ProgramData\FlexLM\hosted"),
        &host_data,
    )?;
    fs::create_dir(format!(r"\\?\{DRIVE_FOLDER}\ProgramData\FlexLM\hosted."))?;
    let on_the_share = format!(r"{HOST_SHARE}\{APP}\host.exe");
    for link in [
        format!(r"{DRIVE_FOLDER}\Program Files\{APP}\share.dll"),
        format!(r"{DRIVE_FOLDER}\Program Files\{APP}\share.exe.manifest"),
        format!(r"{DRIVE_FOLDER}\Program Files\WAM Share App\share.exe"),
        format!(r"{profile}\AppData\Local\WAM Share Squirrel App\Update.exe"),
    ] {
        must("cmd", &["/C", "mklink", &link, &on_the_share])?;
    }
    // A task of this machine's that runs the app, which is not on the drive.
    must(
        "schtasks.exe",
        &[
            "/create",
            "/tn",
            r"\WAM Host Only\On This Machine",
            "/tr",
            &exe_of(LIVE_ROOT),
            "/sc",
            "once",
            "/st",
            "00:00",
            "/sd",
            "01/01/2099",
            "/ru",
            "SYSTEM",
            "/f",
        ],
    )?;
    Ok(())
}

/// Take the app off this machine, and look at what is left with Windows' own tools.
fn remove_live_app(paths: &Paths, checks: &mut Checks) -> TestResult {
    let _ = tool("sc.exe", &["delete", SERVICE])?;
    let _ = tool(
        "schtasks.exe",
        &["/delete", "/tn", &format!("{TASK_FOLDER}{TASK_NAME}"), "/f"],
    )?;
    let _ = fs::remove_dir_all(r"C:\Windows\System32\Tasks\WAM Offline");
    for key in [
        UNINSTALL_KEY,
        APP_PATHS_KEY,
        r"HKLM\Software\WAM Offline Vendor",
        r"HKCU\Software\WAM Offline Vendor",
        r"HKLM\Software\Classes\.wamoff",
        r"HKLM\Software\Classes\WamOffline.Document",
        r"HKCU\Software\Classes\.wamoffu",
        r"HKCU\Software\Classes\WamOffline.UserDocument",
    ] {
        reg_delete_key(key);
    }
    reg_delete_key(&format!(r"HKLM\Software\Classes\CLSID\{CLSID}"));
    reg_delete_key(&format!(r"HKCU\Software\Classes\CLSID\{USER_CLSID}"));
    reg_delete_value(MACHINE_RUN, APP);
    reg_delete_value(USER_RUN, APP);
    reg_delete_value(MACHINE_ENVIRONMENT, "WAM_OFFLINE_HOME");
    reg_delete_value(USER_ENVIRONMENT, "WAM_OFFLINE_USER_HOME");
    for (folder, name) in shortcut_places(paths) {
        let _ = fs::remove_file(format!(r"{folder}\{name}.lnk"));
    }
    fs::remove_dir_all(LIVE_ROOT)?;
    for (path, _) in app_data(paths) {
        if let Some(parent) = Path::new(&path).parent() {
            fs::remove_dir_all(parent)?;
        }
    }
    checks.that(
        "the app's folder is gone from this machine",
        !Path::new(LIVE_ROOT).exists(),
        LIVE_ROOT,
    );
    checks.that(
        "reg.exe: its uninstall entry is gone",
        !reg_key_exists(UNINSTALL_KEY)?,
        UNINSTALL_KEY,
    );
    Ok(())
}

fn find_app(apps: &[InstalledApp], name: &str) -> TestResult<InstalledApp> {
    apps.iter()
        .find(|app| app.display_name == name)
        .cloned()
        .ok_or_else(|| format!("the listing does not have {name}").into())
}

fn package_options(
    app: &InstalledApp,
    output: &Path,
    source: ScanSource,
    user_profile: ProfileChoice,
) -> PackageOptions {
    PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app.clone())),
        output_dir: output.to_path_buf(),
        current_directory: None,
        archive_format: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: None,
        include_user_data: true,
        include_program_data: true,
        include_license_data: false,
        bundle_prerequisites: false,
        source,
        user_profile,
        ui_theme: None,
    }
}

fn manifest_of(installer: &Path) -> TestResult<PackageManifest> {
    Ok(read_embedded_manifest(
        &EmbeddedPackage::open(installer)?.ok_or("the installer has no package")?,
    )?)
}

fn check_the_scan(checks: &mut Checks, manifest: &PackageManifest, paths: &Paths) {
    section("What the package says");
    checks.same(
        "the computer is the drive's, not this one",
        manifest.source_computer_name.as_deref(),
        Some(SOURCE_COMPUTER),
    );
    checks.that(
        "the Windows version is the drive's, not this one's",
        manifest
            .source_windows_version
            .as_deref()
            .is_some_and(|version| version.contains(&format!(".{SOURCE_BUILD}.77"))),
        format!("{:?}", manifest.source_windows_version),
    );
    checks.same(
        "the install location is where it was on its PC",
        manifest
            .application_metadata
            .as_ref()
            .and_then(|app| app.install_location.clone()),
        Some(PathBuf::from(LIVE_ROOT)),
    );
    checks.same(
        "so that is what an installer offers",
        default_install_location(manifest),
        Some(PathBuf::from(LIVE_ROOT)),
    );
    checks.same(
        "the program is where it was on its PC",
        manifest.main_executable_path.clone(),
        PathBuf::from(exe_of(LIVE_ROOT)),
    );

    let has_key = |hive: RegistryHive, part: &str| {
        manifest.registry_keys_exported.iter().any(|entry| {
            entry.hive == hive
                && entry
                    .key_path
                    .to_ascii_lowercase()
                    .contains(&part.to_ascii_lowercase())
        })
    };
    for (label, hive, part) in [
        ("COM class", RegistryHive::LocalMachine, CLSID),
        (
            "file association",
            RegistryHive::LocalMachine,
            r"Classes\.wamoff",
        ),
        (
            "its file type",
            RegistryHive::LocalMachine,
            "WamOffline.Document",
        ),
        (
            "App Paths",
            RegistryHive::LocalMachine,
            r"App Paths\WAM Offline App.exe",
        ),
        (
            "uninstall entry",
            RegistryHive::LocalMachine,
            r"Uninstall\WAM Offline App",
        ),
        (
            "the vendor's key",
            RegistryHive::LocalMachine,
            r"WAM Offline Vendor\WAM Offline App",
        ),
        (
            "the user's key",
            RegistryHive::CurrentUser,
            r"WAM Offline Vendor\WAM Offline App",
        ),
        (
            "a user's COM class, from UsrClass.dat",
            RegistryHive::CurrentUser,
            USER_CLSID,
        ),
        (
            "a user's file association, from UsrClass.dat",
            RegistryHive::CurrentUser,
            r"Classes\.wamoffu",
        ),
        (
            "its file type, from UsrClass.dat",
            RegistryHive::CurrentUser,
            "WamOffline.UserDocument",
        ),
    ] {
        checks.that(&format!("scan: {label}"), has_key(hive, part), part);
    }
    for (label, hive) in [
        ("the machine's Run value", RegistryHive::LocalMachine),
        ("the user's Run value", RegistryHive::CurrentUser),
    ] {
        checks.that(
            &format!("scan: {label}"),
            manifest.registry_keys_exported.iter().any(|entry| {
                entry.hive == hive
                    && entry.key_path.ends_with(r"CurrentVersion\Run")
                    && entry.value_name.as_deref() == Some(APP)
            }),
            "it was not found",
        );
    }
    checks.that(
        "scan: the COM classes are told apart as well",
        manifest
            .com_registrations_detected
            .iter()
            .any(|class| class.clsid.eq_ignore_ascii_case(CLSID))
            && manifest
                .com_registrations_detected
                .iter()
                .any(|class| class.clsid.eq_ignore_ascii_case(USER_CLSID)),
        format!("{:?}", manifest.com_registrations_detected),
    );
    for (scope, name) in [
        ("Machine", "WAM_OFFLINE_HOME"),
        ("User", "WAM_OFFLINE_USER_HOME"),
    ] {
        checks.that(
            &format!("scan: the {scope} variable {name}"),
            manifest
                .environment_variables_detected
                .iter()
                .any(|variable| variable.name == name && format!("{:?}", variable.scope) == scope),
            "it was not found",
        );
    }

    checks.same(
        "scan: the service, by its path on its PC",
        manifest
            .services_detected
            .iter()
            .map(|service| (service.service_name.clone(), service.binary_path.clone()))
            .collect::<Vec<_>>(),
        vec![(SERVICE.to_owned(), PathBuf::from(exe_of(LIVE_ROOT)))],
    );
    let tasks: Vec<String> = manifest
        .scheduled_tasks_detected
        .iter()
        .map(|task| format!("{}{}", task.task_path, task.task_name))
        .collect();
    checks.same(
        "scan: the task with two programs, once, and none from this machine",
        tasks,
        vec![format!("{TASK_FOLDER}{TASK_NAME}")],
    );
    checks.same(
        "scan: the task's program, by its path on its PC",
        manifest
            .scheduled_tasks_detected
            .first()
            .map(|task| task.action_command.clone()),
        Some(PathBuf::from(exe_of(LIVE_ROOT))),
    );
    let mut shortcuts: Vec<PathBuf> = manifest
        .shortcuts_detected
        .iter()
        .map(|shortcut| shortcut.original_lnk_path.clone())
        .collect();
    shortcuts.sort();
    let mut expected: Vec<PathBuf> = shortcut_places(paths)
        .iter()
        .map(|(folder, name)| PathBuf::from(format!(r"{folder}\{name}.lnk")))
        .collect();
    expected.sort();
    checks.same(
        "scan: the shortcuts, read from the drive",
        shortcuts,
        expected,
    );
    checks.that(
        "scan: each shortcut says what it runs and how",
        manifest.shortcuts_detected.iter().all(|shortcut| {
            shortcut.target_path == Path::new(&exe_of(LIVE_ROOT))
                && shortcut.arguments.as_deref() == Some("--from-shortcut")
        }),
        format!("{:?}", manifest.shortcuts_detected),
    );

    let copied: Vec<&Path> = manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .map(|file| file.original_path.as_path())
        .collect();
    for wanted in [
        exe_of(LIVE_ROOT),
        format!(r"{LIVE_ROOT}\conf\app.conf"),
        format!(r"{LIVE_ROOT}\plugins\Größe.txt"),
    ]
    .into_iter()
    .chain(app_data(paths).into_iter().map(|(path, _)| path))
    {
        checks.that(
            &format!("scan: the file {wanted}, by its path on its PC"),
            copied.contains(&Path::new(&wanted)),
            format!("{copied:?}"),
        );
    }
    for path in &copied {
        let text = path.display().to_string();
        checks.that(
            "no file in the package is from this machine or another account",
            !text.contains("WAM Host Secret")
                && !text.contains("linked")
                && !text.contains(OTHER_ON_DRIVE),
            &text,
        );
    }
    checks.that(
        "the package holds no path as the drive is now",
        !manifest
            .file_inventory
            .iter()
            .any(|file| file.path.starts_with(DRIVE) || file.original_path.starts_with(DRIVE)),
        "it does",
    );
    let warned = manifest
        .warnings
        .iter()
        .any(|warning| warning.code == "drive_paths_not_followed");
    checks.that(
        "what a value named and was not followed is said",
        warned,
        format!("{:?}", manifest.warnings),
    );
    checks.same(
        "it holds the data of the account that last signed in there",
        manifest.user_data_profile.as_deref(),
        Some(paths.profile_name.as_str()),
    );
}

fn check_installed(checks: &mut Checks, paths: &Paths, root: &str) -> TestResult {
    section("What the install did, by Windows' own tools");
    let exe = exe_of(root);
    for (relative, content) in app_files() {
        let path = Path::new(root).join(relative);
        checks.same(
            &format!("the file {relative}"),
            fs::read(&path).ok(),
            Some(content.to_vec()),
        );
    }
    for (path, content) in app_data(paths) {
        checks.same(
            &format!("{path}, in the real folder"),
            fs::read(&path).ok(),
            Some(content.to_vec()),
        );
    }
    checks.that(
        "Documents did not go under the install folder",
        !Path::new(root).join("_external").exists(),
        root,
    );
    checks.same(
        "reg.exe: the install location in the uninstall entry",
        reg_data(UNINSTALL_KEY, Some("InstallLocation"))?,
        Some(root.to_owned()),
    );
    checks.same(
        "reg.exe: App Paths",
        reg_data(APP_PATHS_KEY, None)?,
        Some(exe.clone()),
    );
    checks.same(
        "reg.exe: the COM class",
        reg_data(
            &format!(r"HKLM\Software\Classes\CLSID\{CLSID}\InprocServer32"),
            None,
        )?,
        Some(format!(r"{root}\bin\Shell.dll")),
    );
    checks.same(
        "reg.exe: the file association",
        reg_data(
            r"HKLM\Software\Classes\WamOffline.Document\shell\open\command",
            None,
        )?,
        Some(format!("\"{exe}\" \"%1\"")),
    );
    checks.same(
        "reg.exe: a user's COM class",
        reg_data(
            &format!(r"HKCU\Software\Classes\CLSID\{USER_CLSID}\InprocServer32"),
            None,
        )?,
        Some(format!(r"{root}\bin\Shell.dll")),
    );
    checks.same(
        "reg.exe: a user's key",
        reg_data(USER_KEY, Some("LastFolder"))?,
        Some(format!(r"{root}\plugins")),
    );
    checks.same(
        "reg.exe: the machine's Run value",
        reg_data(MACHINE_RUN, Some(APP))?,
        Some(format!("\"{exe}\" --tray")),
    );
    checks.same(
        "reg.exe: the user's Run value",
        reg_data(USER_RUN, Some(APP))?,
        Some(format!("\"{exe}\" --user")),
    );
    checks.same(
        "reg.exe: the machine variable",
        reg_data(MACHINE_ENVIRONMENT, Some("WAM_OFFLINE_HOME"))?,
        Some(root.to_owned()),
    );
    checks.same(
        "reg.exe: the user's variable",
        reg_data(USER_ENVIRONMENT, Some("WAM_OFFLINE_USER_HOME"))?,
        Some(format!(r"{root}\conf")),
    );
    let service = must("sc.exe", &["qc", SERVICE, "8192"])?;
    checks.that(
        "sc.exe: the service runs the file on this machine",
        service.contains(&exe),
        &service,
    );
    let task = must(
        "schtasks.exe",
        &[
            "/query",
            "/tn",
            &format!("{TASK_FOLDER}{TASK_NAME}"),
            "/xml",
        ],
    )?;
    checks.that(
        "schtasks.exe: the task runs both of its programs here",
        task.contains(&exe) && task.contains(&format!(r"{root}\bin\helper.exe")),
        &task,
    );
    for (folder, name) in shortcut_places(paths) {
        let path = format!(r"{folder}\{name}.lnk");
        checks.same(
            &format!("WScript.Shell: the shortcut {path}"),
            read_shortcut(&path)?,
            Some([exe.clone(), "--from-shortcut".to_owned()]),
        );
    }
    Ok(())
}

/// Each new location comes back out by an uninstall and is put back by a rollback.
fn new_locations_come_back_out(
    checks: &mut Checks,
    installer: &Path,
    paths: &Paths,
    root: &str,
) -> TestResult {
    section("A late failure puts back what was in Documents and Saved Games");
    // What was there, which the install replaces.
    let document = format!(r"{}\{APP}\doc.txt", paths.documents);
    let save = format!(r"{}\{APP}\slot.sav", paths.saved_games);
    for path in [&document, &save] {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, b"what was there before")?;
    }
    let started_with = |path: &String| fs::read(path).ok();
    let startup = format!(r"{}\{APP} Startup.lnk", startup_folder(paths));
    create_shortcut(&startup, r"C:\Windows\notepad.exe", "")?;
    let held = fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&startup)?;
    let run = run_installer(installer, &["--apply", "--force", "--restore-user-data"])?;
    drop(held);
    checks.that("the install reports failure", !run.success, &run.output);
    checks.that(
        "it says what it did about it",
        run.output.contains("what it changed was put back"),
        &run.output,
    );
    checks.same(
        "rollback: Documents has what it had",
        started_with(&document),
        Some(b"what was there before".to_vec()),
    );
    checks.same(
        "rollback: Saved Games has what it had",
        started_with(&save),
        Some(b"what was there before".to_vec()),
    );
    checks.that(
        "rollback: reg.exe finds no key of the user's left",
        !reg_key_exists(USER_KEY)?,
        USER_KEY,
    );
    checks.that(
        "rollback: nothing is left in the install folder",
        !Path::new(root).exists()
            || fs::read_dir(root).is_ok_and(|mut entries| entries.next().is_none()),
        root,
    );
    checks.that(
        "rollback: the data it would have added is gone",
        !Path::new(&format!(r"{}\{APP}\settings.json", paths.roaming)).exists(),
        &paths.roaming,
    );
    fs::remove_file(&startup)?;

    section("An install into all of it");
    let run = run_installer(installer, &["--apply", "--force", "--restore-user-data"])?;
    checks.that("the install succeeded", run.success, &run.output);
    check_installed(checks, paths, root)?;
    let id = install_id_in(&run.output)?;

    section("An uninstall that takes the user data with it");
    let run = uninstall_quietly(checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    checks.same(
        "uninstall: what Documents had comes back",
        started_with(&document),
        Some(b"what was there before".to_vec()),
    );
    checks.same(
        "uninstall: what Saved Games had comes back",
        started_with(&save),
        Some(b"what was there before".to_vec()),
    );
    for (path, _) in app_data(paths) {
        if path != document && path != save {
            checks.that(
                &format!("uninstall: {path} is gone"),
                !Path::new(&path).exists(),
                &path,
            );
        }
    }
    checks.that(
        "uninstall: reg.exe finds the user's key gone",
        !reg_key_exists(USER_KEY)?,
        USER_KEY,
    );
    checks.that(
        "uninstall: reg.exe finds the user's classes gone",
        !reg_key_exists(r"HKCU\Software\Classes\.wamoffu")?,
        ".wamoffu",
    );
    checks.that(
        "uninstall: the install folder is gone",
        !Path::new(root).exists(),
        root,
    );

    section("An uninstall that leaves it");
    for path in [&document, &save] {
        fs::remove_file(path)?;
    }
    let run = run_installer(installer, &["--apply", "--force", "--restore-user-data"])?;
    checks.that("the install succeeded", run.success, &run.output);
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(checks, &id, "")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    checks.same(
        "uninstall: a file in Documents stays unless it is asked to go",
        started_with(&document),
        Some(b"document".to_vec()),
    );
    checks.same(
        "uninstall: a file in Saved Games stays unless it is asked to go",
        started_with(&save),
        Some(b"saved".to_vec()),
    );
    for (path, _) in app_data(paths) {
        if let Some(parent) = Path::new(&path).parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
    Ok(())
}

/// A folder of Documents that is somewhere this will not write: what happens, and that it says.
fn a_documents_folder_outside_the_profile_is_said(
    checks: &mut Checks,
    installer: &Path,
    paths: &Paths,
    root: &str,
) -> TestResult {
    section("With Documents outside the profile, the files are kept and it says where they went");
    let key = format!(r"HKU\{}\{USER_SHELL_FOLDERS}", paths.sid);
    let was = reg_value(&key, Some("Personal"))?;
    let elsewhere = r"C:\WAM Redirected Documents";
    fs::create_dir_all(elsewhere)?;
    reg_add(&key, Some("Personal"), "REG_EXPAND_SZ", elsewhere)?;
    let run = run_installer(installer, &["--apply", "--force", "--restore-user-data"])?;
    match was {
        Some((kind, data)) => reg_add(&key, Some("Personal"), &kind, &data)?,
        None => reg_delete_value(&key, "Personal"),
    }
    checks.that("the install succeeded", run.success, &run.output);
    checks.that(
        "it says where the files went, and why",
        run.output.contains("user_folder_elsewhere") && run.output.contains(elsewhere),
        &run.output,
    );
    checks.that(
        "nothing was written outside the profile",
        fs::read_dir(elsewhere).is_ok_and(|mut entries| entries.next().is_none()),
        elsewhere,
    );
    let held_under_install = powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Recurse -File -Filter doc.txt | ForEach-Object {{ $_.FullName }}",
        quoted(&format!(r"{root}\_external"))
    ))?;
    checks.that(
        "the file is under the install location, not lost",
        held_under_install.contains("doc.txt"),
        &held_under_install,
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    Ok(())
}

#[test]
fn a_real_drive_from_another_pc_round_trips() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let paths = paths()?;
    let program = fake_program()?;
    let work = tempfile::tempdir()?;

    section("The app on this machine, then a real NTFS drive made from what is really here");
    create_live_app(&paths, &program)?;
    attach_ntfs_volume(work.path())?;
    lay_out_the_drive(&paths)?;
    edit_the_drives_registry(&paths)?;
    plant_the_links(&paths, &program)?;

    section("Take the app off this machine, so that nothing can come from here by accident");
    remove_live_app(&paths, &mut checks)?;

    section("What Windows itself does with a registry link in a hive it has just loaded");
    reg_add(HOST_REGISTRY_KEY, Some("Token"), "REG_SZ", HOST_SECRET)?;
    reg_add(
        &format!(r"HKCU\{HOST_USER_KEY}"),
        Some("Token"),
        "REG_SZ",
        HOST_SECRET,
    )?;
    for (hive, link) in [
        (
            format!(r"{DRIVE_FOLDER}\Windows\System32\config\SOFTWARE"),
            format!(r"{VENDOR}\{APP}\HostLink"),
        ),
        (
            format!(r"{DRIVE_FOLDER}\Users\{}\NTUSER.DAT", paths.profile_name),
            format!(r"Software\{VENDOR}\{APP}\HostUserLink"),
        ),
    ] {
        edit_hive(&hive, |key| {
            let output = tool("reg.exe", &["query", &format!(r"{key}\{link}"), "/s"])?;
            println!(
                "reg query of {link}: {}\n{}{}",
                output.status,
                text(&output.stdout),
                text(&output.stderr)
            );
            Ok(())
        })?;
    }

    let source = ScanSource::offline(DRIVE);
    let before = tree_state(DRIVE_FOLDER)?;
    println!("{} entries on the drive", before.len());
    let connections_before = smb_connections()?;
    println!("connections to this machine's SMB port so far: {connections_before}");

    section("List what is on the drive, through the real code");
    let system = SystemContext::open(&source)?;
    println!("hives while listing: {:?}", wam_mounts()?);
    let apps = list_installed_apps_from(&system)?;
    drop(system);
    checks.same(
        "no hive stays loaded after a listing",
        wam_mounts()?,
        Vec::new(),
    );
    let app = find_app(&apps, APP)?;
    checks.same(
        "the listing reads it where the drive is now",
        app.main_executable.clone(),
        Some(PathBuf::from(exe_of(&format!(
            r"{DRIVE_FOLDER}\Program Files\{APP}"
        )))),
    );
    let hostile = find_app(&apps, "WAM Hostile App")?;
    println!("the hostile app reads as {hostile:?}");
    checks.that(
        "an app whose record names this machine's files has none of them",
        hostile.install_location.is_none() && hostile.main_executable.is_none(),
        format!("{hostile:?}"),
    );

    section("Who can be chosen on the drive");
    let survey = survey_profiles(&source)?;
    checks.same("no hive stays loaded after that", wam_mounts()?, Vec::new());
    let picker = survey.picker_for(&app);
    println!("{picker:?}");
    let offered: BTreeSet<String> = picker.rows.iter().map(|row| row.name.clone()).collect();
    checks.same(
        "the two accounts with data, and no folder of this machine's",
        offered,
        BTreeSet::from([paths.profile_name.clone(), OTHER_ON_DRIVE.to_owned()]),
    );
    checks.same(
        "by default, whoever last signed in there",
        picker.selected.clone(),
        Some(paths.sid.clone()),
    );

    section("Scan and package it, through the real code, at a letter that is not C:");
    let scanned_at = Instant::now();
    let package = create_package(&package_options(
        &app,
        &work.path().join(APP),
        source.clone(),
        ProfileChoice::Default,
    ))?;
    println!("scanned and packaged in {:?}", scanned_at.elapsed());
    for warning in &package.warnings {
        println!("packaging warning {}: {}", warning.code, warning.message);
    }
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let manifest = manifest_of(&installer)?;
    check_the_scan(&mut checks, &manifest, &paths);

    section("Nothing of this machine's is in it, by Windows' own search of what was packaged");
    let holding = files_holding(&package.package_root, HOST_SECRET)?;
    checks.same(
        "the marker on this machine is in no file",
        holding.trim(),
        "",
    );
    let holding = files_holding(&package.package_root, OTHER_SECRET)?;
    checks.same("another account's data is in no file", holding.trim(), "");
    checks.that(
        "a registry link to this machine was not followed into the package",
        !manifest.registry_keys_exported.iter().any(|entry| {
            entry.value_name.as_deref() == Some("Token")
                || entry.value_data.as_deref() == Some(HOST_SECRET)
        }),
        "the value from this machine's registry is there",
    );
    let raw = fs::read_to_string(&package.manifest_path)?;
    let without_warnings: PackageManifest = serde_json::from_str(&raw)?;
    let files_text = serde_json::to_string(&without_warnings.file_inventory)?;
    checks.that(
        "no file in the manifest says where the drive is now",
        !files_text.contains(r"V:\\"),
        "it does",
    );

    section("The hostile app gets nothing, however it is asked for");
    let attempt = create_package(&package_options(
        &hostile,
        &work.path().join("hostile"),
        source.clone(),
        ProfileChoice::Default,
    ));
    println!(
        "building it: {:?}",
        attempt.as_ref().map(|result| result.files_copied)
    );
    if let Ok(result) = attempt.as_ref() {
        let holding = files_holding(&result.package_root, HOST_SECRET)?;
        checks.same("the marker is not in that either", holding.trim(), "");
    }
    let by_a_forged_selection = package_options(
        &InstalledApp {
            main_executable: Some(PathBuf::from(format!(r"{HOST_FOLDER}\{APP}\host.exe"))),
            install_location: Some(PathBuf::from(format!(r"{HOST_FOLDER}\{APP}"))),
            ..hostile.clone()
        },
        &work.path().join("forged"),
        source.clone(),
        ProfileChoice::Default,
    );
    checks.that(
        "a selection that names this machine's program is refused",
        create_package(&by_a_forged_selection).is_err(),
        "it was built",
    );

    section("Apps whose files are links to a share get none of it, and connect to nothing");
    for name in ["WAM Share App", "WAM Share Squirrel App"] {
        let linked = find_app(&apps, name)?;
        println!("{name} reads as {linked:?}");
        checks.that(
            &format!("{name} has no program, since its program is a link"),
            linked.main_executable.is_none(),
            format!("{linked:?}"),
        );
        let attempt = create_package(&package_options(
            &linked,
            &work.path().join(name),
            source.clone(),
            ProfileChoice::Default,
        ));
        println!(
            "building it: {:?}",
            attempt.as_ref().map(|result| result.files_copied)
        );
        if let Ok(result) = attempt.as_ref() {
            let holding = files_holding(&result.package_root, HOST_SECRET)?;
            checks.same("the marker is not in that either", holding.trim(), "");
        }
    }

    section("A license store on the drive with links in it gets its own files and no others");
    let licensed = create_package(&PackageOptions {
        include_license_data: true,
        ..package_options(
            &app,
            &work.path().join("licensed"),
            source.clone(),
            ProfileChoice::Default,
        )
    })?;
    let holding = files_holding(&licensed.package_root, HOST_SECRET)?;
    checks.same("the marker is in no file", holding.trim(), "");
    let holding = files_holding(&licensed.package_root, "a real license")?;
    checks.that("its own license is there", !holding.trim().is_empty(), "no");

    section("Another account on the drive, on request");
    let other = create_package(&package_options(
        &app,
        &work.path().join("other account"),
        source.clone(),
        ProfileChoice::Profile(OTHER_SID_ON_DRIVE.to_owned()),
    ))?;
    let other_installer = other
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let other_manifest = manifest_of(&other_installer)?;
    let other_files: Vec<String> = other_manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .map(|file| file.original_path.display().to_string())
        .collect();
    checks.that(
        "that account's own data is there",
        other_files.iter().any(|file| file.ends_with("private.txt")),
        format!("{other_files:?}"),
    );
    for theirs in ["settings.json", "cache.db", "doc.txt", "slot.sav"] {
        checks.that(
            &format!("the first account's {theirs} is not, even with a value that names it"),
            !other_files.iter().any(|file| file.ends_with(theirs)),
            format!("{other_files:?}"),
        );
    }
    checks.that(
        "the settings are that account's",
        other_manifest.registry_keys_exported.iter().any(|entry| {
            entry.hive == RegistryHive::CurrentUser
                && entry.value_name.as_deref() == Some("Owner")
                && entry.value_data.as_deref() == Some(OTHER_SECRET)
        }),
        "the value is not there",
    );

    section("The drive is as it was, and nothing stays loaded");
    let after = tree_state(DRIVE_FOLDER)?;
    let differences = tree_differences(&before, &after);
    checks.same(
        "every file on the drive hashes the same",
        differences,
        Vec::new(),
    );
    checks.same("no hive stays loaded", wam_mounts()?, Vec::new());
    checks.same(
        "no copy of a hive stays",
        fs::read_dir(HIVES_FOLDER)
            .map(|entries| entries.flatten().count())
            .unwrap_or_default(),
        0,
    );

    section("What loading the drive's own hive would have done, for comparison");
    let control = tree_state(DRIVE_FOLDER)?;
    let direct = format!(r"{DRIVE_FOLDER}\Users\{OTHER_ON_DRIVE}\NTUSER.DAT");
    must("reg.exe", &["load", r"HKLM\WamControl", &direct])?;
    let started = Instant::now();
    while !tool("reg.exe", &["unload", r"HKLM\WamControl"])?
        .status
        .success()
        && started.elapsed() < Duration::from_secs(30)
    {
        std::thread::sleep(Duration::from_millis(250));
    }
    let changed = tree_differences(&control, &tree_state(DRIVE_FOLDER)?);
    println!("loading it where it is changed the drive: {changed:?}");
    checks.that(
        "that would have written to the drive, so the comparison above can see it",
        !changed.is_empty(),
        "nothing changed",
    );

    section("No SMB connection was made to where a value or a link pointed");
    let connections_after = smb_connections()?;
    println!("connections to this machine's SMB port by now: {connections_after}");
    checks.same(
        "none, in all that was read from the drive",
        connections_after,
        connections_before,
    );
    // The same look finds one that really is made, through a link that was left there.
    let looked = tool(
        "cmd",
        &[
            "/C",
            "dir",
            &format!(r"{DRIVE_FOLDER}\Users\shareprofile\{APP}\host.exe"),
        ],
    )?;
    println!(
        "looking through a link on purpose: {}\n{}{}",
        looked.status,
        text(&looked.stdout),
        text(&looked.stderr)
    );
    let connections_control = smb_connections()?;
    println!("then, {connections_control}");
    checks.that(
        "going through a link on purpose does show, so the count above means something",
        connections_control > connections_after,
        format!("{connections_control} after {connections_after}"),
    );

    let target = default_install_location(&manifest)
        .ok_or("no install location")?
        .display()
        .to_string();
    new_locations_come_back_out(&mut checks, &installer, &paths, &target)?;
    a_documents_folder_outside_the_profile_is_said(&mut checks, &installer, &paths, &target)?;

    a_killed_scan_is_swept(&mut checks, work.path())?;
    checks.finish()
}

/// Not a test of anything by itself: this is what `a_killed_scan_is_swept` starts and kills. It
/// does nothing without its variable.
#[test]
fn holds_a_drive_mounted_until_killed() -> TestResult {
    let Some(drive) = std::env::var_os(HOLD_DRIVE_VARIABLE) else {
        return Ok(());
    };
    if !may_change_this_machine()? {
        return Ok(());
    }
    let system = SystemContext::open(&ScanSource::offline(PathBuf::from(drive)))?;
    println!("mounted: {}", system.is_offline());
    std::thread::sleep(Duration::from_secs(600));
    drop(system);
    Ok(())
}

fn a_killed_scan_is_swept(checks: &mut Checks, work: &Path) -> TestResult {
    section("A scan killed with the hives loaded is swept, and only what it left");
    // Hives with names this app would and would not give one, made the way another program would.
    let decoy_file = work.join("decoy.dat");
    must(
        "reg.exe",
        &[
            "save",
            r"HKCU\Environment",
            &decoy_file.display().to_string(),
            "/y",
        ],
    )?;
    let decoys = ["WAM_NotOurs", "WAM_a_b_OTHER"];
    for decoy in decoys {
        let copy = work.join(format!("{decoy}.dat"));
        fs::copy(&decoy_file, &copy)?;
        must(
            "reg.exe",
            &[
                "load",
                &format!(r"HKLM\{decoy}"),
                &copy.display().to_string(),
            ],
        )?;
    }

    let mut holder = Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "holds_a_drive_mounted_until_killed",
            "--nocapture",
        ])
        .env(HOLD_DRIVE_VARIABLE, DRIVE)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let is_ours = |name: &String| !decoys.contains(&name.as_str());
    let started = Instant::now();
    let mut held: Vec<String> = Vec::new();
    while started.elapsed() < Duration::from_secs(180) {
        held = wam_mounts()?.into_iter().filter(is_ours).collect();
        if held.iter().any(|name| name.contains("_USER_")) {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    println!("loaded by the other run: {held:?}");
    checks.that(
        "the other run has hives loaded",
        !held.is_empty(),
        "it has none",
    );

    let while_alive = sweep_leaked_hive_mounts();
    println!("swept while it runs: {while_alive:?}");
    checks.same(
        "a run that is still going keeps its hives",
        while_alive.unloaded,
        Vec::new(),
    );

    holder.kill()?;
    let _ = holder.wait()?;
    let left: Vec<String> = wam_mounts()?.into_iter().filter(is_ours).collect();
    checks.same(
        "reg.exe: killing it left every hive loaded",
        left.len(),
        held.len(),
    );

    let swept = sweep_leaked_hive_mounts();
    println!("swept after: {swept:?}");
    checks.same(
        "the next start unloads all of it",
        wam_mounts()?
            .into_iter()
            .filter(is_ours)
            .collect::<Vec<_>>(),
        Vec::new(),
    );
    checks.same(
        "and removes the copies of its hives",
        swept.failed,
        Vec::new(),
    );
    checks.same(
        "reg.exe: nothing this app did not name was touched",
        wam_mounts()?.into_iter().collect::<BTreeSet<_>>(),
        decoys
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<BTreeSet<_>>(),
    );
    for decoy in decoys {
        must("reg.exe", &["unload", &format!(r"HKLM\{decoy}")])?;
    }
    checks.same(
        "no copy of a hive stays",
        fs::read_dir(HIVES_FOLDER)
            .map(|entries| entries.flatten().count())
            .unwrap_or_default(),
        0,
    );
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Real accounts
// ---------------------------------------------------------------------------------------

const PROFILES_APP: &str = "WAM Profiles App";
const PROFILES_VENDOR: &str = "WAM Profiles Vendor";
const PROFILES_ROOT: &str = r"C:\Program Files\WAM Profiles App";
const PROFILES_UNINSTALL_KEY: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM Profiles App";
const OTHER_USER: &str = "wamother";
const OTHER_ADMIN: &str = "wamadmin";
const SHARED_FOLDER: &str = r"C:\WAM Shared";

fn random_password() -> String {
    let random = RandomState::new().build_hasher().finish();
    format!("Wam-{random:016x}-Aa1!")
}

/// A real local account with a real profile that nobody has signed in to.
fn create_account(
    name: &str,
    password: &str,
    administrator: bool,
) -> TestResult<(String, PathBuf)> {
    must("net.exe", &["user", name, password, "/add", "/y"])?;
    if administrator {
        must("net.exe", &["localgroup", "Administrators", name, "/add"])?;
    }
    let shown = powershell(&format!(
        r#"$sid = (New-Object System.Security.Principal.NTAccount({name})).Translate([System.Security.Principal.SecurityIdentifier]).Value;
           Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Text;
public static class WamProfile {{
    [DllImport("userenv.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
    public static extern int CreateProfile(string sid, string name, StringBuilder path, uint size);
}}
'@;
           $path = New-Object System.Text.StringBuilder 260;
           $result = [WamProfile]::CreateProfile($sid, {name}, $path, 260);
           if ($result -ne 0) {{ throw "CreateProfile failed: $result" }};
           '{{0}}|{{1}}' -f $sid, $path.ToString()"#,
        name = quoted(name),
    ))?;
    let (sid, profile) = shown
        .trim()
        .split_once('|')
        .ok_or("CreateProfile said nothing")?;
    Ok((sid.to_owned(), PathBuf::from(profile)))
}

fn write_data(base: &Path, relative: &str, content: &str) -> TestResult {
    let path = base.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, content)?)
}

/// `(relative to a profile, what it holds for that account)`.
fn account_data(who: &str) -> Vec<(String, String)> {
    [
        format!(r"AppData\Roaming\{PROFILES_APP}\{who}-settings.txt"),
        format!(r"AppData\Local\{PROFILES_APP}\{who}-cache.txt"),
        format!(r"Documents\{PROFILES_APP}\{who}-doc.txt"),
        format!(r"Saved Games\{PROFILES_APP}\{who}-save.txt"),
    ]
    .into_iter()
    .map(|relative| (relative, format!("private to {who}")))
    .collect()
}

fn give_account_data(profile: &Path, who: &str) -> TestResult {
    for (relative, content) in account_data(who) {
        write_data(profile, &relative, &content)?;
    }
    Ok(())
}

fn copied_names(manifest: &PackageManifest) -> Vec<String> {
    manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .filter_map(|file| {
            file.original_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .collect()
}

fn check_holds_only(checks: &mut Checks, manifest: &PackageManifest, who: &str, not: &[&str]) {
    let names = copied_names(manifest);
    for (relative, _) in account_data(who) {
        let name = Path::new(&relative)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        checks.that(
            &format!("the package has {name}"),
            names.contains(&name),
            format!("{names:?}"),
        );
    }
    for other in not {
        checks.that(
            &format!("the package has nothing of {other}'s"),
            !names.iter().any(|name| name.starts_with(other)),
            format!("{names:?}"),
        );
    }
    checks.that(
        &format!("its per-user settings are {who}'s"),
        manifest.registry_keys_exported.iter().any(|entry| {
            entry.hive == RegistryHive::CurrentUser
                && entry.value_name.as_deref() == Some("Who")
                && entry.value_data.as_deref() == Some(who)
        }) && !manifest.registry_keys_exported.iter().any(|entry| {
            entry.hive == RegistryHive::CurrentUser
                && entry.value_name.as_deref() == Some("Who")
                && entry.value_data.as_deref() != Some(who)
        }),
        "another account's value is there, or its own is not",
    );
    checks.same(
        "the package says whose",
        manifest.user_data_profile.is_some(),
        true,
    );
}

/// Whose desktop this session is, as Windows says. `None` if there is none here.
fn desktop_owner_here() -> TestResult<Option<String>> {
    let shown = powershell(
        "$session = [Diagnostics.Process]::GetCurrentProcess().SessionId; \
         Get-CimInstance Win32_Process -Filter \"Name='explorer.exe' AND SessionId=$session\" | \
         ForEach-Object { (Invoke-CimMethod -InputObject $_ -MethodName GetOwnerSid).Sid } | \
         Select-Object -First 1",
    )?;
    let owner = shown.trim();
    Ok((!owner.is_empty()).then(|| owner.to_owned()))
}

/// Run a command line as another account, in this session, from a script that sets what it needs.
fn run_as_account(
    account: &str,
    password: &str,
    script_name: &str,
    script: &str,
) -> TestResult<(bool, String)> {
    let script_path = format!(r"{SHARED_FOLDER}\{script_name}.cmd");
    let out = format!(r"{SHARED_FOLDER}\{script_name}.out");
    let err = format!(r"{SHARED_FOLDER}\{script_name}.err");
    fs::write(&script_path, script)?;
    // What a UAC prompt gives: that administrator's whole token, in this session. Start-Process
    // -Credential gives the filtered one. A batch logon gets the whole one, and with the secondary
    // logon service, the process is in this session.
    let command = format!("cmd.exe /c \"\"{script_path}\" > \"{out}\" 2> \"{err}\"\"");
    let shown = powershell(&format!(
        r#"Add-Type -TypeDefinition @'
using System;
using System.Text;
using System.Runtime.InteropServices;
public static class WamElevated {{
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    public struct STARTUPINFO {{ public int cb; public string lpReserved; public string lpDesktop; public string lpTitle; public int dwX; public int dwY; public int dwXSize; public int dwYSize; public int dwXCountChars; public int dwYCountChars; public int dwFillAttribute; public int dwFlags; public short wShowWindow; public short cbReserved2; public IntPtr lpReserved2; public IntPtr hStdInput; public IntPtr hStdOutput; public IntPtr hStdError; }}
    [StructLayout(LayoutKind.Sequential)]
    public struct PROCESS_INFORMATION {{ public IntPtr hProcess; public IntPtr hThread; public int dwProcessId; public int dwThreadId; }}
    [DllImport("advapi32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    public static extern bool LogonUserW(string user, string domain, string password, int type, int provider, out IntPtr token);
    [DllImport("advapi32.dll", SetLastError = true)]
    public static extern bool DuplicateTokenEx(IntPtr token, int access, IntPtr attributes, int level, int type, out IntPtr result);
    [DllImport("advapi32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    public static extern bool CreateProcessWithTokenW(IntPtr token, int logonFlags, string application, StringBuilder command, int flags, IntPtr environment, string folder, ref STARTUPINFO startup, out PROCESS_INFORMATION process);
    [DllImport("kernel32.dll")]
    public static extern int WaitForSingleObject(IntPtr handle, int milliseconds);
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool GetExitCodeProcess(IntPtr handle, out int code);
}}
'@;
           $last = {{ [Runtime.InteropServices.Marshal]::GetLastWin32Error() }};
           $token = [IntPtr]::Zero;
           if (-not [WamElevated]::LogonUserW({account}, '.', {password}, 4, 0, [ref]$token)) {{ throw "LogonUser: $(& $last)" }};
           $primary = [IntPtr]::Zero;
           if (-not [WamElevated]::DuplicateTokenEx($token, 0x02000000, [IntPtr]::Zero, 2, 1, [ref]$primary)) {{ throw "DuplicateTokenEx: $(& $last)" }};
           $startup = New-Object 'WamElevated+STARTUPINFO';
           $startup.cb = [Runtime.InteropServices.Marshal]::SizeOf([type]'WamElevated+STARTUPINFO');
           $process = New-Object 'WamElevated+PROCESS_INFORMATION';
           $line = New-Object Text.StringBuilder({command}, 4096);
           if (-not [WamElevated]::CreateProcessWithTokenW($primary, 1, 'C:\Windows\System32\cmd.exe', $line, 0x08000000, [IntPtr]::Zero, {folder}, [ref]$startup, [ref]$process)) {{ throw "CreateProcessWithToken: $(& $last)" }};
           [void][WamElevated]::WaitForSingleObject($process.hProcess, -1);
           $code = 0;
           [void][WamElevated]::GetExitCodeProcess($process.hProcess, [ref]$code);
           $code"#,
        account = quoted(account),
        password = quoted(password),
        command = quoted(&command),
        folder = quoted(SHARED_FOLDER),
    ))?;
    let exit = shown.trim().to_owned();
    let output = format!(
        "{}{}",
        fs::read_to_string(&out).unwrap_or_default(),
        fs::read_to_string(&err).unwrap_or_default()
    );
    println!("--- as {account}: exit {exit}\n{output}---");
    Ok((exit == "0", output))
}

/// Not a test of anything by itself: this is what the account test starts as another
/// administrator. It does nothing without its marker file.
#[test]
fn scans_for_the_other_administrator() -> TestResult {
    let output = Path::new(SHARED_FOLDER).join("scan-output");
    if !Path::new(SHARED_FOLDER).join("scan-request").is_file() || !may_change_this_machine()? {
        return Ok(());
    }
    let running_as = must("whoami", &["/user", "/fo", "csv", "/nh"])?;
    println!("running as {running_as}");
    println!("signed in: {:?}", signed_in_user());
    let app = find_app(
        &list_installed_apps_from(&SystemContext::live())?,
        PROFILES_APP,
    )?;
    let package = create_package(&package_options(
        &app,
        &output,
        ScanSource::Live,
        ProfileChoice::Default,
    ))?;
    println!("built {:?}", package.self_extracting_archive_path);
    Ok(())
}

#[test]
fn accounts_are_kept_apart() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let program = fake_program()?;
    let work = tempfile::tempdir()?;
    let password = random_password();
    let my_sid = current_sid()?;
    let my_profile = PathBuf::from(std::env::var("USERPROFILE")?);
    println!(
        "this session's desktop belongs to {:?}, this account is {my_sid}, signed in: {:?}",
        desktop_owner_here()?,
        signed_in_user()
    );

    section("A second account with a real profile, which is not signed in");
    let (other_sid, other_profile) = create_account(OTHER_USER, &password, false)?;
    println!("{OTHER_USER}: {other_sid} at {}", other_profile.display());
    checks.that(
        "its NTUSER.DAT is there, and its hive is not loaded",
        other_profile.join("NTUSER.DAT").is_file()
            && !reg_key_exists(&format!(r"HKU\{other_sid}"))?,
        other_profile.display(),
    );

    section("One app, with each account's own data and settings");
    fs::create_dir_all(format!(r"{PROFILES_ROOT}\bin"))?;
    let exe = format!(r"{PROFILES_ROOT}\bin\{PROFILES_APP}.exe");
    fs::copy(&program, &exe)?;
    for (name, data) in [
        ("DisplayName", PROFILES_APP),
        ("Publisher", PROFILES_VENDOR),
        ("InstallLocation", PROFILES_ROOT),
    ] {
        reg_add(PROFILES_UNINSTALL_KEY, Some(name), "REG_SZ", data)?;
    }
    reg_add(
        PROFILES_UNINSTALL_KEY,
        Some("DisplayIcon"),
        "REG_SZ",
        &format!("{exe},0"),
    )?;
    give_account_data(&my_profile, "mine")?;
    give_account_data(&other_profile, "theirs")?;
    let vendor_key = format!(r"Software\{PROFILES_VENDOR}\{PROFILES_APP}");
    reg_add(
        &format!(r"HKCU\{vendor_key}"),
        Some("Who"),
        "REG_SZ",
        "mine",
    )?;
    let other_hive = other_profile.join("NTUSER.DAT").display().to_string();
    must("reg.exe", &["load", r"HKU\WamTestOther", &other_hive])?;
    let edited = reg_add(
        &format!(r"HKU\WamTestOther\{vendor_key}"),
        Some("Who"),
        "REG_SZ",
        "theirs",
    );
    let started = Instant::now();
    while !tool("reg.exe", &["unload", r"HKU\WamTestOther"])?
        .status
        .success()
        && started.elapsed() < Duration::from_secs(30)
    {
        std::thread::sleep(Duration::from_millis(250));
    }
    edited?;

    section("Who can be chosen");
    let source = ScanSource::Live;
    let survey = survey_profiles(&source)?;
    let app = find_app(
        &list_installed_apps_from(&SystemContext::live())?,
        PROFILES_APP,
    )?;
    let picker = survey.picker_for(&app);
    println!("{picker:?}");
    let by_id = |id: &str| picker.rows.iter().find(|row| row.id == id);
    checks.that(
        "both accounts with data are offered",
        by_id(&other_sid).is_some_and(|row| row.has_data)
            && picker.rows.iter().any(|row| row.is_default && row.has_data),
        format!("{picker:?}"),
    );
    checks.that(
        "it comes to whoever is signed in",
        picker.selected.as_deref().is_some_and(|id| {
            signed_in_user()
                .account()
                .is_some_and(|account| account.sid == id)
        }),
        format!("{picker:?}"),
    );
    checks.same("no hive stays loaded", wam_mounts()?, Vec::new());

    section("By default, this account's, and nothing of the other's");
    let mine = create_package(&package_options(
        &app,
        &work.path().join("mine"),
        source.clone(),
        ProfileChoice::Default,
    ))?;
    let mine_manifest = manifest_of(
        mine.self_extracting_archive_path
            .as_deref()
            .ok_or("no installer")?,
    )?;
    check_holds_only(&mut checks, &mine_manifest, "mine", &["theirs"]);

    section(
        "On request, the other account's, from its hive that is not loaded, and nothing of this one's",
    );
    let theirs = create_package(&package_options(
        &app,
        &work.path().join("theirs"),
        source.clone(),
        ProfileChoice::Profile(other_sid.clone()),
    ))?;
    let theirs_installer = theirs
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer")?;
    let theirs_manifest = manifest_of(&theirs_installer)?;
    check_holds_only(&mut checks, &theirs_manifest, "theirs", &["mine"]);
    checks.same("no hive stays loaded", wam_mounts()?, Vec::new());
    checks.that(
        "reg.exe: the other account's hive is as it was, not loaded",
        !reg_key_exists(&format!(r"HKU\{other_sid}"))?,
        &other_sid,
    );

    section("Installed, the other account's data lands in this one's real folders and hive");
    let own_keys = reg_data(&format!(r"HKCU\{vendor_key}"), Some("Who"))?;
    reg_delete_key(&format!(r"HKCU\Software\{PROFILES_VENDOR}"));
    for folder in [
        format!(r"AppData\Roaming\{PROFILES_APP}"),
        format!(r"AppData\Local\{PROFILES_APP}"),
        format!(r"Documents\{PROFILES_APP}"),
        format!(r"Saved Games\{PROFILES_APP}"),
    ] {
        fs::remove_dir_all(my_profile.join(folder))?;
    }
    let target = r"C:\Program Files\WAM Profiles Restored";
    let run = run_installer(
        &theirs_installer,
        &[
            "--target-root",
            target,
            "--apply",
            "--force",
            "--restore-user-data",
        ],
    )?;
    checks.that("the install succeeded", run.success, &run.output);
    for (relative, content) in account_data("theirs") {
        checks.same(
            &format!("{relative} is in this account's real folder"),
            fs::read_to_string(my_profile.join(&relative)).ok(),
            Some(content),
        );
    }
    checks.same(
        "reg.exe: the setting is in this account's real hive",
        reg_data(&format!(r"HKCU\{vendor_key}"), Some("Who"))?,
        Some("theirs".to_owned()),
    );
    println!("(this account's own was {own_keys:?})");
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&mut checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    for (relative, _) in account_data("theirs") {
        checks.that(
            &format!("uninstall: {relative} is gone"),
            !my_profile.join(&relative).exists(),
            &relative,
        );
    }
    checks.that(
        "uninstall: reg.exe finds the setting gone",
        !reg_key_exists(&format!(r"HKCU\{vendor_key}"))?,
        &vendor_key,
    );

    section("A Documents folder moved inside the profile is where it goes");
    let shell_folders = format!(r"HKCU\{USER_SHELL_FOLDERS}");
    let was_documents = reg_value(&shell_folders, Some("Personal"))?;
    let was_games = reg_value(&shell_folders, Some(SAVED_GAMES_VALUE))?;
    reg_add(
        &shell_folders,
        Some("Personal"),
        "REG_EXPAND_SZ",
        r"%USERPROFILE%\OneDrive\Documents",
    )?;
    reg_add(
        &shell_folders,
        Some(SAVED_GAMES_VALUE),
        "REG_EXPAND_SZ",
        r"%USERPROFILE%\OneDrive\Games",
    )?;
    let run = run_installer(
        &theirs_installer,
        &[
            "--target-root",
            target,
            "--apply",
            "--force",
            "--restore-user-data",
        ],
    )?;
    checks.that("the install succeeded", run.success, &run.output);
    let moved_document =
        my_profile.join(format!(r"OneDrive\Documents\{PROFILES_APP}\theirs-doc.txt"));
    let moved_save = my_profile.join(format!(r"OneDrive\Games\{PROFILES_APP}\theirs-save.txt"));
    checks.that(
        "the file is in the moved Documents",
        moved_document.is_file(),
        moved_document.display(),
    );
    checks.that(
        "the file is in the moved Saved Games",
        moved_save.is_file(),
        moved_save.display(),
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&mut checks, &id, " --remove-user-data")?;
    checks.that("the uninstall succeeded", run.success, &run.output);
    checks.that(
        "uninstall: it comes back out of there too",
        !moved_document.exists() && !moved_save.exists(),
        moved_document.display(),
    );
    match was_documents {
        Some((kind, data)) => reg_add(&shell_folders, Some("Personal"), &kind, &data)?,
        None => reg_delete_value(&shell_folders, "Personal"),
    }
    match was_games {
        Some((kind, data)) => reg_add(&shell_folders, Some(SAVED_GAMES_VALUE), &kind, &data)?,
        None => reg_delete_value(&shell_folders, SAVED_GAMES_VALUE),
    }

    what_can_name_another_accounts_folder(&mut checks, &program, &other_profile, work.path())?;
    another_administrator_at_the_uac_prompt(&mut checks, &password)?;

    checks.finish()
}

/// What an account can write with no rights at all that names another's folder, or a share. An
/// elevated build reads what it is told to, so none of this may be what tells it.
fn what_can_name_another_accounts_folder(
    checks: &mut Checks,
    program: &Path,
    other_profile: &Path,
    work: &Path,
) -> TestResult {
    let other_data = format!(r"AppData\Roaming\{PROFILES_APP}");
    let other_name = other_profile
        .file_name()
        .ok_or("the profile has no name")?
        .to_string_lossy()
        .into_owned();
    let my_profile = PathBuf::from(std::env::var("USERPROFILE")?);
    let connections_before = smb_connections()?;

    section("A value in another account's hive that names a share, when the app starts");
    let hive = other_profile.join("NTUSER.DAT").display().to_string();
    edit_hive(&hive, |key| {
        reg_add(
            &format!(r"{key}\{USER_SHELL_FOLDERS}"),
            Some("Personal"),
            "REG_EXPAND_SZ",
            r"\\127.0.0.1\C$\Windows",
        )
    })?;
    let survey = survey_profiles(&ScanSource::Live)?;
    let apps = list_installed_apps_from(&SystemContext::live())?;
    println!("{:?}", survey.picker_for(&find_app(&apps, PROFILES_APP)?));
    checks.same(
        "reading that connected to nothing, as this machine's SMB port saw it",
        smb_connections()?,
        connections_before,
    );
    edit_hive(&hive, |key| {
        reg_add(
            &format!(r"{key}\{USER_SHELL_FOLDERS}"),
            Some("Personal"),
            "REG_EXPAND_SZ",
            r"%USERPROFILE%\Documents",
        )
    })?;

    section("A per-user entry whose location is another account's folder, by a share");
    let mine = my_profile.join(r"AppData\Local\WAM By A Share");
    fs::create_dir_all(&mine)?;
    let exe = mine.join("wam-by-a-share.exe");
    fs::copy(program, &exe)?;
    let entry = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM By A Share";
    reg_add(entry, Some("DisplayName"), "REG_SZ", "WAM By A Share")?;
    reg_add(
        entry,
        Some("DisplayIcon"),
        "REG_SZ",
        &exe.display().to_string(),
    )?;
    reg_add(
        entry,
        Some("InstallLocation"),
        "REG_SZ",
        &format!(r"\\127.0.0.1\C$\Users\{other_name}\{other_data}"),
    )?;
    let apps = list_installed_apps_from(&SystemContext::live())?;
    let by_a_share = find_app(&apps, "WAM By A Share")?;
    println!("it reads as {by_a_share:?}");
    checks.same(
        "the location is no use",
        by_a_share.install_location.clone(),
        None,
    );
    let package = create_package(&package_options(
        &by_a_share,
        &work.join("by a share"),
        ScanSource::Live,
        ProfileChoice::Default,
    ))?;
    let holding = files_holding(&package.package_root, "private to theirs")?;
    checks.same("none of that account's data is in it", holding.trim(), "");
    checks.same(
        "and nothing connected, as this machine's SMB port saw it",
        smb_connections()?,
        connections_before,
    );
    reg_delete_key(entry);

    section("A per-user entry whose location is a folder that is not this account's");
    let not_mine = r"C:\WAM Not Mine";
    fs::create_dir_all(not_mine)?;
    fs::write(format!(r"{not_mine}\secret.txt"), "private to theirs")?;
    must(
        "icacls",
        &[not_mine, "/setowner", "*S-1-5-18", "/T", "/C", "/Q"],
    )?;
    println!(
        "owner: {}",
        powershell(&format!(
            "Import-Module Microsoft.PowerShell.Security; (Get-Acl -LiteralPath {}).Owner",
            quoted(not_mine)
        ))?
    );
    reg_add(entry, Some("DisplayName"), "REG_SZ", "WAM By A Share")?;
    reg_add(
        entry,
        Some("DisplayIcon"),
        "REG_SZ",
        &exe.display().to_string(),
    )?;
    reg_add(entry, Some("InstallLocation"), "REG_SZ", not_mine)?;
    let apps = list_installed_apps_from(&SystemContext::live())?;
    let not_its_own = find_app(&apps, "WAM By A Share")?;
    println!("it reads as {not_its_own:?}");
    let package = create_package(&package_options(
        &not_its_own,
        &work.join("not mine"),
        ScanSource::Live,
        ProfileChoice::Default,
    ))?;
    let holding = files_holding(&package.package_root, "private to theirs")?;
    checks.same(
        "what is there is not in it, whatever the entry says",
        holding.trim(),
        "",
    );
    reg_delete_key(entry);

    section("A shortcut of this account's that names the program of a machine-wide app");
    let theirs = other_profile.join(&other_data);
    fs::create_dir_all(&theirs)?;
    let their_program = theirs.join("wam-hinted.exe");
    fs::copy(program, &their_program)?;
    let machine_entry = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM Hinted App";
    reg_add(
        machine_entry,
        Some("DisplayName"),
        "REG_SZ",
        "WAM Hinted App",
    )?;
    let shortcut = my_profile
        .join(r"AppData\Roaming\Microsoft\Windows\Start Menu\Programs\WAM Hinted App.lnk");
    if let Some(folder) = shortcut.parent() {
        fs::create_dir_all(folder)?;
    }
    create_shortcut(
        &shortcut.display().to_string(),
        &their_program.display().to_string(),
        "",
    )?;
    let apps = list_installed_apps_from(&SystemContext::live())?;
    let hinted = find_app(&apps, "WAM Hinted App")?;
    println!("it reads as {hinted:?}");
    checks.same(
        "the shortcut is what it goes by, so this is what is being tried",
        hinted.main_executable.clone(),
        Some(their_program.clone()),
    );
    let attempt = create_package(&package_options(
        &hinted,
        &work.join("hinted"),
        ScanSource::Live,
        ProfileChoice::Default,
    ));
    println!(
        "building it: {:?}",
        attempt.as_ref().map(|result| result.files_copied)
    );
    checks.that(
        "that does not make their folder the app's: it will not build",
        attempt.is_err(),
        "it was built",
    );
    fs::remove_file(&shortcut)?;
    fs::remove_file(&their_program)?;
    reg_delete_key(machine_entry);
    Ok(())
}

/// What an elevated process run by another administrator does. Needs a desktop in this session
/// for there to be someone signed in other than who is running it.
fn another_administrator_at_the_uac_prompt(checks: &mut Checks, password: &str) -> TestResult {
    section("Run as another administrator, as someone would at a UAC prompt");
    let my_sid = current_sid()?;
    let my_profile = PathBuf::from(std::env::var("USERPROFILE")?);
    let mut desktop = desktop_owner_here()?;
    if desktop.is_none() {
        println!("no desktop in this session, so trying to start one");
        let _ = powershell("Start-Process -FilePath 'C:\\Windows\\explorer.exe'");
        std::thread::sleep(Duration::from_secs(15));
        desktop = desktop_owner_here()?;
    }
    println!("this session's desktop belongs to {desktop:?}");
    if desktop.as_deref() != Some(my_sid.as_str()) {
        println!(
            "NOT PROVEN: this runner has no desktop of this account's in this session, so a \
             process run as another administrator here has no one else signed in to go to. The \
             rule itself is covered by a unit test of what each case comes to."
        );
        return Ok(());
    }
    fs::create_dir_all(SHARED_FOLDER)?;
    must(
        "icacls",
        &[SHARED_FOLDER, "/grant", "*S-1-1-0:(OI)(CI)F", "/T", "/Q"],
    )?;
    let (admin_sid, admin_profile) = create_account(OTHER_ADMIN, password, true)?;
    println!("{OTHER_ADMIN}: {admin_sid} at {}", admin_profile.display());
    give_account_data(&admin_profile, "admins")?;

    // This account's own data for the app, which is what a scan run by the other should find.
    give_account_data(&my_profile, "mine")?;
    let vendor_key = format!(r"Software\{PROFILES_VENDOR}\{PROFILES_APP}");
    reg_add(
        &format!(r"HKCU\{vendor_key}"),
        Some("Who"),
        "REG_SZ",
        "mine",
    )?;

    let here = std::process::id();
    let this_session = powershell(&format!("(Get-Process -Id {here}).SessionId"))?;
    let probe = run_as_account(
        OTHER_ADMIN,
        password,
        "probe",
        "@echo off\r\nwhoami /groups\r\npowershell -NoProfile -Command \"'session=' +          [Diagnostics.Process]::GetCurrentProcess().SessionId\"\r\n",
    );
    let (is_whole_token, in_this_session) = match &probe {
        Ok((_, shown)) => (
            shown.contains("S-1-16-12288"),
            shown.contains(&format!("session={}", this_session.trim())),
        ),
        Err(error) => {
            println!("could not start anything as that administrator: {error}");
            (false, false)
        }
    };
    println!("whole token: {is_whole_token}, in this session: {in_this_session}");
    if !(is_whole_token && in_this_session) {
        println!(
            "NOT PROVEN: this runner would not start a process as another administrator with \
             that administrator's whole token in this session, which is what a UAC prompt does. \
             The rule itself is covered by a unit test of what each case comes to."
        );
        return Ok(());
    }

    let test_binary = std::env::current_exe()?;
    fs::write(Path::new(SHARED_FOLDER).join("scan-request"), b"")?;
    let (scanned, shown) = run_as_account(
        OTHER_ADMIN,
        password,
        "scan",
        &format!(
            "@echo off\r\nset GITHUB_ACTIONS=true\r\nset RUNNER_ENVIRONMENT=github-hosted\r\n\
             set WAM_ELEVATED_E2E=1\r\nwhoami /user\r\n\"{}\" --exact scans_for_the_other_administrator --nocapture\r\n",
            test_binary.display()
        ),
    )?;
    fs::remove_file(Path::new(SHARED_FOLDER).join("scan-request"))?;
    checks.that("the scan ran as the other administrator", scanned, &shown);
    checks.that(
        "it says it is that account",
        shown.contains(&admin_sid.to_ascii_lowercase()) || shown.contains(&admin_sid),
        &shown,
    );
    let installer = find_file_ending(Path::new(SHARED_FOLDER), "_installer.exe")?;
    let manifest = manifest_of(&installer)?;
    check_holds_only(checks, &manifest, "mine", &["admins"]);

    section("And an install run by that administrator goes to whoever is signed in");
    reg_delete_key(&format!(r"HKCU\Software\{PROFILES_VENDOR}"));
    for folder in [
        format!(r"AppData\Roaming\{PROFILES_APP}"),
        format!(r"AppData\Local\{PROFILES_APP}"),
        format!(r"Documents\{PROFILES_APP}"),
        format!(r"Saved Games\{PROFILES_APP}"),
    ] {
        fs::remove_dir_all(my_profile.join(folder))?;
    }
    let target = r"C:\Program Files\WAM Profiles OTS";
    let (installed, shown) = run_as_account(
        OTHER_ADMIN,
        password,
        "install",
        &format!(
            "@echo off\r\n\"{}\" --target-root \"{target}\" --apply --force --restore-user-data\r\n",
            installer.display()
        ),
    )?;
    checks.that("the install succeeded", installed, &shown);
    checks.that(
        "it says whose this went to",
        shown.contains("who is signed in here"),
        &shown,
    );
    for (relative, content) in account_data("mine") {
        checks.same(
            &format!("{relative} is in the signed-in account's real folder"),
            fs::read_to_string(my_profile.join(&relative)).ok(),
            Some(content),
        );
        checks.that(
            &format!("{relative} is not in the administrator's"),
            !admin_profile.join(&relative).exists(),
            &relative,
        );
    }
    checks.same(
        "reg.exe: the setting is in the signed-in account's hive",
        reg_data(&format!(r"HKU\{my_sid}\{vendor_key}"), Some("Who"))?,
        Some("mine".to_owned()),
    );
    let id = install_id_in(&shown)?;

    section("Undone by the same administrator, with the account's hive and folders");
    let (uninstalled, shown) = run_as_account(
        OTHER_ADMIN,
        password,
        "uninstall",
        &format!(
            "@echo off\r\n\"{INSTALLS_DIR}\\{id}\\uninstall.exe\" --uninstall {id} --quiet --remove-user-data\r\n"
        ),
    )?;
    checks.that("the uninstall succeeded", uninstalled, &shown);
    let started = Instant::now();
    while Path::new(INSTALLS_DIR).join(&id).exists() && started.elapsed() < SELF_REMOVAL_PATIENCE {
        std::thread::sleep(Duration::from_millis(250));
    }
    for (relative, _) in account_data("mine") {
        checks.that(
            &format!("{relative} is gone from the signed-in account's folder"),
            !my_profile.join(&relative).exists(),
            &relative,
        );
    }
    checks.that(
        "reg.exe: the setting is gone from the signed-in account's hive",
        !reg_key_exists(&format!(r"HKU\{my_sid}\{vendor_key}"))?,
        &vendor_key,
    );
    println!(
        "PROVEN: a scan and an install run as another administrator, in a session with a real \
         desktop of someone else's"
    );
    Ok(())
}

fn find_file_ending(folder: &Path, ending: &str) -> TestResult<PathBuf> {
    for entry in walkdir_files(folder)? {
        if entry.to_string_lossy().ends_with(ending) {
            return Ok(entry);
        }
    }
    Err(format!("nothing in {} ends in {ending}", folder.display()).into())
}

fn walkdir_files(folder: &Path) -> TestResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut pending = vec![folder.to_path_buf()];
    while let Some(current) = pending.pop() {
        for entry in fs::read_dir(&current)?.flatten() {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

// ---------------------------------------------------------------------------------------
// Real scheduled tasks
// ---------------------------------------------------------------------------------------

/// What the query through PowerShell used to run, one Export for each task, to compare with.
const POWERSHELL_READ_OF_TASKS: &str = r#"
Import-Module ScheduledTasks; Import-Module Microsoft.PowerShell.Utility;
$items = foreach ($task in Get-ScheduledTask) {
  $definitionXml = $null
  try {
    $definitionXml = Export-ScheduledTask -TaskName $task.TaskName -TaskPath $task.TaskPath
  } catch {
    $definitionXml = $null
  }
  foreach ($action in $task.Actions) {
    if ($null -ne $action.Execute) {
      [pscustomobject]@{
        Task = ([string]$task.TaskPath + [string]$task.TaskName)
        Execute = [string]$action.Execute
        HasDefinition = ($null -ne $definitionXml)
      }
    }
  }
}
$items | ConvertTo-Json -Depth 4 -Compress
"#;

/// The commands in a task as `schtasks` writes it. By looking for them, since it writes `]]>` as it
/// is in text, and a strict parser will not take that. This has no part in what is being tested.
fn tasks_commands(task: &str, commands: &mut Vec<String>) -> TestResult {
    let mut rest = task;
    loop {
        let start = [rest.find("<Exec>"), rest.find("<Exec ")]
            .into_iter()
            .flatten()
            .min();
        let Some(start) = start else {
            return Ok(());
        };
        let (action, after) = rest[start..]
            .split_once("</Exec>")
            .ok_or("an action that does not end")?;
        if let Some((_, command)) = action.split_once("<Command>") {
            let (command, _) = command
                .split_once("</Command>")
                .ok_or("a command that does not end")?;
            commands.push(
                command
                    .trim()
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&quot;", "\"")
                    .replace("&apos;", "'")
                    .replace("&amp;", "&"),
            );
        }
        rest = after;
    }
}

/// The tasks `schtasks /query /xml` lists, with the commands in each, as it writes them.
fn tasks_by_schtasks() -> TestResult<BTreeMap<String, Vec<String>>> {
    let shown = powershell(
        "$bytes = & cmd.exe /c 'chcp 65001 >nul & schtasks.exe /query /xml'; \
         [string]::Join(\"`n\", $bytes)",
    )?;
    // Each task is a comment with its name and then a document of its own, however they are
    // wrapped, which is why this does not parse the whole of it as one.
    let mut tasks = BTreeMap::new();
    let mut rest = shown.as_str();
    while let Some(start) = rest.find("<!--") {
        let after = &rest[start + "<!--".len()..];
        let (name, after) = after
            .split_once("-->")
            .ok_or("a comment that does not end")?;
        let (Some(open), Some(close)) = (after.find("<Task "), after.find("</Task>")) else {
            break;
        };
        let end = close + "</Task>".len();
        let mut commands = Vec::new();
        tasks_commands(&after[open..end], &mut commands)?;
        tasks.insert(name.trim().to_owned(), commands);
        rest = &after[end..];
    }
    if tasks.is_empty() {
        return Err(format!("schtasks wrote no tasks: {shown}").into());
    }
    Ok(tasks)
}

#[test]
fn every_real_task_reads_as_windows_says_it_does() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let non_ascii = r"\WAM Tasks\Wartung Größe 日本";
    let two_programs = r"\WAM Tasks\Two Programs";
    let definition = std::env::temp_dir().join("wam-e2e-task.xml");
    utf16_file(
        &definition,
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <Triggers><TimeTrigger><StartBoundary>2099-01-01T00:00:00</StartBoundary></TimeTrigger></Triggers>
  <Principals><Principal id="Author"><UserId>S-1-5-18</UserId></Principal></Principals>
  <Settings><Enabled>false</Enabled></Settings>
  <Actions Context="Author">
    <Exec><Command>C:\Windows\System32\notepad.exe</Command></Exec>
    <Exec><Command>"C:\Program Files\WAM Tasks\größe.exe"</Command><Arguments>--a "b"</Arguments></Exec>
  </Actions>
</Task>"#,
    )?;
    for name in [non_ascii, two_programs] {
        must(
            "schtasks.exe",
            &[
                "/create",
                "/tn",
                name,
                "/xml",
                &definition.display().to_string(),
                "/f",
            ],
        )?;
    }
    // A file that cannot be read among them, which is what used to lose all of them.
    let broken = r"C:\Windows\System32\Tasks\WAM Tasks\Broken";
    fs::write(broken, b"this is not a task <")?;

    section("The reader, timed, on every real task on this machine");
    let tasks_root = Path::new(r"C:\Windows\System32\Tasks");
    let started = Instant::now();
    let (files, warnings) = read_task_files(tasks_root);
    let reader_time = started.elapsed();
    println!("the reader took {reader_time:?} for {} tasks", files.len());
    for warning in &warnings {
        println!("  {}: {}", warning.code, warning.message);
    }
    checks.same(
        "one task that cannot be read is one warning, for it",
        warnings
            .iter()
            .map(|warning| warning.source.clone().unwrap_or_default())
            .collect::<Vec<_>>(),
        vec![broken.to_owned()],
    );
    fs::remove_file(broken)?;

    section("What schtasks.exe says");
    let started = Instant::now();
    let windows = tasks_by_schtasks()?;
    println!(
        "schtasks took {:?} for {} tasks",
        started.elapsed(),
        windows.len()
    );
    let read: BTreeMap<String, Vec<String>> = files
        .iter()
        .map(|file| {
            (
                format!("{}{}", file.task_path, file.task_name),
                file.actions
                    .iter()
                    .map(|action| action.command.clone())
                    .collect(),
            )
        })
        .collect();
    let lower = |map: &BTreeMap<String, Vec<String>>| -> BTreeSet<String> {
        map.keys().map(|name| name.to_lowercase()).collect()
    };
    let only_read: Vec<String> = lower(&read).difference(&lower(&windows)).cloned().collect();
    let only_windows: Vec<String> = lower(&windows).difference(&lower(&read)).cloned().collect();
    println!("only in the files: {only_read:?}");
    println!("only in schtasks: {only_windows:?}");
    checks.same(
        "every task schtasks lists was read",
        only_windows,
        Vec::new(),
    );
    checks.same("and no others", only_read, Vec::new());
    let mut differing = Vec::new();
    for (name, commands) in &read {
        let Some(said) = windows
            .iter()
            .find(|(other, _)| other.eq_ignore_ascii_case(name))
            .map(|(_, commands)| commands)
        else {
            continue;
        };
        if said != commands {
            differing.push(format!("{name}: read {commands:?}, schtasks says {said:?}"));
        }
    }
    checks.same(
        "each task's programs are the same, in order",
        differing,
        Vec::new(),
    );
    checks.same(
        "the task with a name that is not ASCII",
        read.get(non_ascii).map(Vec::len),
        Some(2),
    );
    checks.same(
        "the task with two programs has both, as written",
        read.get(two_programs).cloned(),
        Some(vec![
            r"C:\Windows\System32\notepad.exe".to_owned(),
            r#""C:\Program Files\WAM Tasks\größe.exe""#.to_owned(),
        ]),
    );

    section("And the query it replaces, for the same tasks and the time it took");
    let started = Instant::now();
    let old = powershell(POWERSHELL_READ_OF_TASKS)?;
    let old_time = started.elapsed();
    println!(
        "BEFORE AND AFTER: PowerShell took {old_time:?}, the reader took {reader_time:?}, for {} tasks",
        files.len()
    );
    let old: serde_json::Value = serde_json::from_str(old.trim())?;
    let mut old_commands: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in old.as_array().ok_or("the query gave no list")? {
        let task = row["Task"].as_str().unwrap_or_default().to_lowercase();
        let execute = row["Execute"].as_str().unwrap_or_default().to_owned();
        old_commands.entry(task).or_default().push(execute);
    }
    let mut differing = Vec::new();
    for (name, commands) in &old_commands {
        let found = read
            .iter()
            .find(|(other, _)| other.to_lowercase() == *name)
            .map(|(_, commands)| commands);
        if found != Some(commands) {
            differing.push(format!("{name}: PowerShell {commands:?}, reader {found:?}"));
        }
    }
    checks.same(
        "every task the old query listed reads the same now",
        differing,
        Vec::new(),
    );
    checks.that(
        "the reader is far quicker",
        reader_time < old_time,
        format!("{reader_time:?} against {old_time:?}"),
    );

    for name in [non_ascii, two_programs] {
        let _ = tool("schtasks.exe", &["/delete", "/tn", name, "/f"]);
    }
    checks.finish()
}
