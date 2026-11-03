//! What a person chooses on the app's screen, on GitHub's throwaway Windows runner and nowhere else.
//! See `elevated_round_trip.rs`.
//!
//! A real fake app is set up with things to leave out, a hidden part of it to add, and what must
//! never be offered. It is scanned and built by the real code, taken off the runner, and installed.
//! What is in the package is read with .NET's own zip reader and searched with PowerShell, and what
//! is on the machine is looked at with `sc.exe`, `schtasks.exe`, `Get-FileHash`, and `Test-Path`. The
//! app's verifier has no say in any of it.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do
//! nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
//! `WAM_ELEVATED_E2E=1` are all set. Never set those on a real PC. With all three set
//! and no elevation they fail, so that the job cannot pass without having run.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::{
    AppSelection, BuildMonitor, ChoiceId, EmbeddedPackage, Footprint, FootprintChoices,
    InstalledApp, InstallerStub, NearMatchKind, PackageManifest, PackageOptions, ProfileChoice,
    ScanFlags, ScanMonitor, ScanReport, ScanSource, SystemContext, apply_choices,
    create_package_from_scan, list_installed_apps_from, read_embedded_manifest, scan_app,
    scan_options_for, verify_installer, what_goes_with,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const APP_NAME: &str = "WAM Footprint App";
const VENDOR: &str = "WAM Footprint Vendor";
const SOURCE_ROOT: &str = r"C:\Program Files\WAM Footprint App";
const TARGET_ROOT: &str = r"C:\Program Files\WAM Footprint Restored";
const FAILED_TARGET_ROOT: &str = r"C:\Program Files\WAM Footprint Refused";
const HELPER_NAME: &str = "WAM Footprint Helper";
const HELPER_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Helper";
const EXTRAS_ROOT: &str = r"C:\Program Files\WAM Footprint App Extras";
const DATA_NEAR_ROOT: &str = r"C:\ProgramData\WAM Footprint App Settings";
const OTHER_APP_NAME: &str = "WAM Footprint App Other";
const OTHER_APP_ROOT: &str = r"C:\Program Files\WAM Footprint App Other";
const WINDOWS_NAMED_ROOT: &str = r"C:\Program Files\Windows WAM Footprint App";
const LINK_ROOT: &str = r"C:\Program Files\WAM Footprint App Link";
const IMPOSTOR_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Impostor";
const PUBLISHER_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor";
const MSI_APP_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Reports";
/// A product's code, and how Windows Installer's database writes it, with each part read backwards.
const MSI_APP_CODE: &str = "{11111111-2222-3333-4444-555555555555}";
const MSI_APP_SQUISHED: &str = "11111111222233334444555555555555";
const INSTALLER_USER_DATA: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Installer\UserData\S-1-5-18";
const PART_KEY: &str = r"HKLM\Software\WAM Footprint Vendor\WAM Footprint Helper";
const OTHER_VENDOR_KEY: &str = r"HKLM\Software\Another Publisher\WAM Footprint Helper";
const MICROSOFT_KEY: &str = r"HKLM\Software\Microsoft\WAM Footprint Helper";
/// What the hidden part goes by under its publisher, without the first word, and what a whole other
/// app is called. Without being told of that app, a scan takes this for the part's.
const OTHER_APP_KEY: &str = r"HKLM\Software\WAM Footprint Vendor\Footprint Helper";
const NAMED_LIKE_PART: &str = "Footprint Helper";
const NAMED_LIKE_PART_ROOT: &str = r"C:\Program Files\Part Named Tool";
const CODED_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Coded";
const BORROWED_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Borrowed";
const NAMED_LIKE_WINDOWS_CHILD: &str = r"C:\Program Files\Windows WAM Folder\Below";
const BORROWED_FROM_GENERIC_ROOT: &str = r"C:\Program Files\WAM Footprint Vendor Generic";
const GENERIC_APP_ROOT: &str = r"C:\WAM Games\Launcher";
const LOCKED_KEY_FILE: &str = r"C:\Program Files\WAM Footprint Vendor Helper\locked.pem";
const BELOW_MICROSOFT_ROOT: &str = r"C:\ProgramData\Microsoft\WAM Footprint App Below";
const UNINSTALL: &str = r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall";
const USER_UNINSTALL: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall";
const TASK_FOLDER: &str = r"\WAM Footprint\";
const INSTALLS_DIR: &str = r"C:\ProgramData\WinAppMigrator\installs";
/// Each is in one file only, so that finding it anywhere says that file came along.
const LEFT_OUT_CANARY: &str = "wam-footprint-left-out-canary";
const KEPT_CANARY: &str = "wam-footprint-kept-canary";
const HELPER_CANARY: &str = "wam-footprint-helper-canary";
const SECRET_CANARY: &str = "wam-footprint-secret-canary";
const KEY_CANARY: &str = "wam-footprint-key-canary";
const LICENSE_CANARY: &str = "wam-footprint-license-canary";
const EXTRAS_CANARY: &str = "wam-footprint-extras-canary";
const SERVICE_KEPT: &str = "WamFootprintKept";
const SERVICE_CHOSEN_OUT: &str = "WamFootprintChosenOut";
const SERVICE_RUNS_LEFT_OUT: &str = "WamFootprintRunsLeftOut";
const SERVICE_HELPER: &str = "WamFootprintHelper";
const TASK_KEPT: &str = "Kept";
const TASK_CHOSEN_OUT: &str = "Chosen Out";
const TASK_RUNS_LEFT_OUT: &str = "Runs Left Out";
const PART_KEY_CANARY: &str = "wam-footprint-part-key-canary";
const OTHER_VENDOR_KEY_CANARY: &str = "wam-footprint-other-vendor-key-canary";
const MICROSOFT_KEY_CANARY: &str = "wam-footprint-microsoft-key-canary";
const OTHER_APP_KEY_CANARY: &str = "wam-footprint-other-app-key-canary";
const KEY_BY_CONTENT_CANARY: &str = "wam-footprint-key-by-content-canary";
const CERTIFICATE_CANARY: &str = "wam-footprint-certificate-canary";
const DATA_NEAR_CANARY: &str = "wam-footprint-data-near-canary";
const LOCKED_FILE_CANARY: &str = "wam-footprint-locked-file-canary";
const SELF_REMOVAL_PATIENCE: Duration = Duration::from_secs(90);
const FLAGS: ScanFlags = ScanFlags {
    include_user_data: false,
    include_program_data: false,
    include_license_data: false,
};

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
        (None, Some("github-hosted"), Some("1"), false),
        (Some("true"), None, Some("1"), false),
        (Some("true"), Some("github-hosted"), None, false),
    ] {
        assert_eq!(is_throwaway_runner(actions, environment, flag), expected);
    }
}

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

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn must(program: &str, args: &[&str]) -> TestResult<String> {
    let output = Command::new(program).args(args).output()?;
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

// ---------------------------------------------------------------------------------------
// The fake app
// ---------------------------------------------------------------------------------------

fn write(path: &str, content: &str) -> TestResult {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

/// A real program, which nothing here runs.
fn program_at(path: &str) -> TestResult {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(stub_binary(), path)?;
    Ok(())
}

fn uninstall_entry(key: &str, values: &[(&str, &str)]) -> TestResult {
    for (name, value) in values {
        must(
            "reg.exe",
            &["add", key, "/v", name, "/t", "REG_SZ", "/d", value, "/f"],
        )?;
    }
    Ok(())
}

fn hidden_entry(name: &str, location: &str) -> TestResult {
    hidden_entry_at(name, name, location)
}

/// One whose key has one name, and which calls itself something else.
fn hidden_entry_at(key_name: &str, name: &str, location: &str) -> TestResult {
    let key = format!(r"{UNINSTALL}\{key_name}");
    uninstall_entry(
        &key,
        &[
            ("DisplayName", name),
            ("Publisher", VENDOR),
            ("InstallLocation", location),
        ],
    )?;
    must(
        "reg.exe",
        &[
            "add",
            &key,
            "/v",
            "SystemComponent",
            "/t",
            "REG_DWORD",
            "/d",
            "1",
            "/f",
        ],
    )?;
    Ok(())
}

fn create_service(name: &str, program: &str) -> TestResult {
    let image = format!("\"{program}\"");
    must(
        "sc.exe",
        &["create", name, "binPath=", &image, "start=", "demand"],
    )?;
    Ok(())
}

fn create_task(name: &str, program: &str) -> TestResult {
    powershell(&format!(
        "$action = New-ScheduledTaskAction -Execute {}; \
         $trigger = New-ScheduledTaskTrigger -Once -At '2099-01-01T00:00:00'; \
         $principal = New-ScheduledTaskPrincipal -UserId 'S-1-5-18' -LogonType ServiceAccount; \
         Register-ScheduledTask -TaskName {} -TaskPath {} -Action $action -Trigger $trigger \
           -Principal $principal | Out-Null",
        quoted(&format!("\"{program}\"")),
        quoted(name),
        quoted(TASK_FOLDER),
    ))
    .map(|_| ())
}

fn set_up_the_app() -> TestResult {
    let main = format!(r"{SOURCE_ROOT}\bin\{APP_NAME}.exe");
    let kept_program = format!(r"{SOURCE_ROOT}\bin\keeper.exe");
    let left_program = format!(r"{SOURCE_ROOT}\leave\leave.exe");
    program_at(&main)?;
    program_at(&kept_program)?;
    program_at(&left_program)?;
    write(&format!(r"{SOURCE_ROOT}\keep\keep.txt"), KEPT_CANARY)?;
    write(&format!(r"{SOURCE_ROOT}\leave\leave.txt"), LEFT_OUT_CANARY)?;

    uninstall_entry(
        &format!(r"{UNINSTALL}\{APP_NAME}"),
        &[
            ("DisplayName", APP_NAME),
            ("Publisher", VENDOR),
            ("DisplayVersion", "1.0.0"),
            ("InstallLocation", SOURCE_ROOT),
            ("DisplayIcon", &main),
        ],
    )?;
    create_service(SERVICE_KEPT, &kept_program)?;
    create_service(SERVICE_CHOSEN_OUT, &kept_program)?;
    create_service(SERVICE_RUNS_LEFT_OUT, &left_program)?;
    create_task(TASK_KEPT, &main)?;
    create_task(TASK_CHOSEN_OUT, &main)?;
    create_task(TASK_RUNS_LEFT_OUT, &left_program)?;

    // A hidden part, with what must stay out however it comes.
    let helper = format!(r"{HELPER_ROOT}\helper.exe");
    program_at(&helper)?;
    write(&format!(r"{HELPER_ROOT}\helper.txt"), HELPER_CANARY)?;
    write(&format!(r"{HELPER_ROOT}\credentials.json"), SECRET_CANARY)?;
    write(&format!(r"{HELPER_ROOT}\id_rsa"), KEY_CANARY)?;
    write(&format!(r"{HELPER_ROOT}\helper.lic"), LICENSE_CANARY)?;
    write(
        &format!(r"{HELPER_ROOT}\tls.pem"),
        &format!("-----BEGIN PRIVATE KEY-----\n{KEY_BY_CONTENT_CANARY}\n"),
    )?;
    write(
        &format!(r"{HELPER_ROOT}\ca.pem"),
        &format!("-----BEGIN CERTIFICATE-----\n{CERTIFICATE_CANARY}\n"),
    )?;
    // One no administrator can open, and a package is read with more rights than that. It cannot be
    // told what it holds, so it has to stay out. What it holds would go in if it could be read, so
    // this says it was the not being able to.
    write(
        LOCKED_KEY_FILE,
        &format!("-----BEGIN CERTIFICATE-----\n{LOCKED_FILE_CANARY}\n"),
    )?;
    must(
        "icacls",
        &[LOCKED_KEY_FILE, "/inheritance:r", "/grant:r", "*S-1-5-18:F"],
    )?;
    // What goes by the part's name, and what only says it does.
    for (key, canary) in [
        (PART_KEY, PART_KEY_CANARY),
        (OTHER_VENDOR_KEY, OTHER_VENDOR_KEY_CANARY),
        (MICROSOFT_KEY, MICROSOFT_KEY_CANARY),
        (OTHER_APP_KEY, OTHER_APP_KEY_CANARY),
    ] {
        uninstall_entry(key, &[("Marker", canary)])?;
    }
    hidden_entry(HELPER_NAME, HELPER_ROOT)?;
    create_service(SERVICE_HELPER, &helper)?;

    // Near by name and nobody's.
    write(&format!(r"{EXTRAS_ROOT}\extras.txt"), EXTRAS_CANARY)?;
    write(&format!(r"{DATA_NEAR_ROOT}\settings.txt"), DATA_NEAR_CANARY)?;
    Ok(())
}

/// What Windows also calls a folder, which none of the rules would know it by if they went by it. A
/// volume can have these turned off, and a folder not be there.
fn short_name_of(folder: &str) -> TestResult<Option<String>> {
    if !Path::new(folder).is_dir() {
        return Ok(None);
    }
    let short = powershell(&format!(
        "(New-Object -ComObject Scripting.FileSystemObject).GetFolder({}).ShortPath",
        quoted(folder)
    ))?;
    let short = short.trim().to_owned();
    Ok(short.contains('~').then_some(short))
}

/// Everything that has a name close to the app's and must never be offered, each for its own reason.
/// Whether there was a short name to try for what only that rule stops.
fn set_up_what_must_not_be_offered() -> TestResult<bool> {
    write(&format!(r"{OTHER_APP_ROOT}\other.txt"), "another app's")?;
    program_at(&format!(r"{OTHER_APP_ROOT}\other.exe"))?;
    uninstall_entry(
        &format!(r"{UNINSTALL}\{OTHER_APP_NAME}"),
        &[
            ("DisplayName", OTHER_APP_NAME),
            ("Publisher", VENDOR),
            ("InstallLocation", OTHER_APP_ROOT),
        ],
    )?;
    write(
        &format!(r"{WINDOWS_NAMED_ROOT}\a.txt"),
        "named like Windows'",
    )?;
    write(
        &format!(r"{BELOW_MICROSOFT_ROOT}\a.txt"),
        "under Microsoft's",
    )?;
    write(
        &format!(r"{NAMED_LIKE_WINDOWS_CHILD}\a.txt"),
        "under what is named like Windows'",
    )?;
    // A junction to what no app should have, with a name that would otherwise be close.
    must(
        "cmd.exe",
        &[
            "/C",
            "mklink",
            "/J",
            LINK_ROOT,
            r"C:\Windows\System32\drivers\etc",
        ],
    )?;
    // Hidden entries from the same publisher, naming what is shared and what is someone else's.
    let locations = [
        ("WAM Hostile Program Files", r"C:\Program Files"),
        ("WAM Hostile Common Files", r"C:\Program Files\Common Files"),
        ("WAM Hostile System32", r"C:\Windows\System32"),
        ("WAM Hostile Windows", r"C:\Windows"),
        ("WAM Hostile Drive", r"C:\"),
        ("WAM Hostile ProgramData", r"C:\ProgramData"),
        ("WAM Hostile Users", r"C:\Users"),
        ("WAM Hostile Below Microsoft", BELOW_MICROSOFT_ROOT),
        ("WAM Hostile Link", LINK_ROOT),
        ("WAM Hostile Below Windows' Name", NAMED_LIKE_WINDOWS_CHILD),
        ("WAM Hostile Other App", OTHER_APP_ROOT),
        (
            "WAM Hostile Publisher's Folder",
            r"C:\Program Files\WAM Footprint Vendor",
        ),
    ];
    for (name, location) in locations {
        hidden_entry(name, location)?;
    }
    // The same by what else Windows calls them, which no rule goes by.
    for (name, folder) in [
        (
            "WAM Hostile Short Common Files",
            r"C:\Program Files\Common Files",
        ),
        ("WAM Hostile Short Program Files", r"C:\Program Files"),
    ] {
        if let Some(short) = short_name_of(folder)? {
            println!("{short} is a short name for {folder}");
            hidden_entry(name, &short)?;
        }
    }
    // By its long name this is below `Microsoft`, and by a short one nothing else would say so.
    let below_microsoft = short_name_of(BELOW_MICROSOFT_ROOT)?;
    if let Some(short) = &below_microsoft {
        println!("{short} is a short name for {BELOW_MICROSOFT_ROOT}");
        hidden_entry("WAM Hostile Short Below Microsoft", short)?;
    }
    // A hidden entry in a folder of its own, whose uninstaller is another app's, which is where its
    // install record would be read from.
    write(
        &format!(r"{BORROWED_ROOT}\borrowed.txt"),
        "says it is another's",
    )?;
    let others_uninstaller = format!(r"{OTHER_APP_ROOT}\unins000.exe");
    program_at(&others_uninstaller)?;
    hidden_entry("WAM Hostile Uninstaller", BORROWED_ROOT)?;
    must(
        "reg.exe",
        &[
            "add",
            &format!(r"{UNINSTALL}\WAM Hostile Uninstaller"),
            "/v",
            "UninstallString",
            "/t",
            "REG_SZ",
            "/d",
            &format!("\"{others_uninstaller}\""),
            "/f",
        ],
    )?;
    // A hidden entry in a folder of its own, whose install record is another product's.
    write(&format!(r"{CODED_ROOT}\coded.txt"), "says it is another's")?;
    hidden_entry("WAM Hostile Product Code", CODED_ROOT)?;
    must(
        "reg.exe",
        &[
            "add",
            &format!(r"{UNINSTALL}\WAM Hostile Product Code"),
            "/v",
            "UninstallString",
            "/t",
            "REG_SZ",
            "/d",
            &format!("MsiExec.exe /X{MSI_APP_CODE}"),
            "/f",
        ],
    )?;
    // The same, from an app in a folder with a name that says nothing, which is refused for that
    // before it is ever asked whose it is.
    let generic_uninstaller = format!(r"{GENERIC_APP_ROOT}\unins000.exe");
    program_at(&generic_uninstaller)?;
    uninstall_entry(
        &format!(r"{UNINSTALL}\WAM Games Launcher"),
        &[
            ("DisplayName", "WAM Games Launcher"),
            ("Publisher", "Someone Else"),
            ("InstallLocation", GENERIC_APP_ROOT),
        ],
    )?;
    write(
        &format!(r"{BORROWED_FROM_GENERIC_ROOT}\borrowed.txt"),
        "says it is another's",
    )?;
    hidden_entry(
        "WAM Hostile Uninstaller In Generic",
        BORROWED_FROM_GENERIC_ROOT,
    )?;
    must(
        "reg.exe",
        &[
            "add",
            &format!(r"{UNINSTALL}\WAM Hostile Uninstaller In Generic"),
            "/v",
            "UninstallString",
            "/t",
            "REG_SZ",
            "/d",
            &format!("\"{generic_uninstaller}\""),
            "/f",
        ],
    )?;
    // An app whose name is what the hidden part goes by under its publisher.
    program_at(&format!(r"{NAMED_LIKE_PART_ROOT}\tool.exe"))?;
    uninstall_entry(
        &format!(r"{UNINSTALL}\{NAMED_LIKE_PART}"),
        &[
            ("DisplayName", NAMED_LIKE_PART),
            ("Publisher", "Someone Else"),
            ("InstallLocation", NAMED_LIKE_PART_ROOT),
        ],
    )?;
    // A hidden entry of the same publisher, in a folder of its own, calling itself another app.
    write(&format!(r"{IMPOSTOR_ROOT}\impostor.txt"), "an impostor")?;
    hidden_entry_at("WAM Footprint Impostor Key", OTHER_APP_NAME, IMPOSTOR_ROOT)?;
    // Named only for the publisher, with something in it, so that only its name can refuse it.
    write(&format!(r"{PUBLISHER_ROOT}\a.txt"), "the publisher's")?;
    // An app of the same publisher whose entry does not say where it is, as is common of an MSI's.
    // Its folder is close by name, and only Windows Installer's database says whose it is.
    let msi_program = format!(r"{MSI_APP_ROOT}\reports.exe");
    program_at(&msi_program)?;
    let msi_key = format!(r"{UNINSTALL}\{MSI_APP_CODE}");
    uninstall_entry(
        &msi_key,
        &[
            ("DisplayName", "Quarterly Figures"),
            ("Publisher", VENDOR),
            ("UninstallString", &format!("MsiExec.exe /X{MSI_APP_CODE}")),
        ],
    )?;
    must(
        "reg.exe",
        &[
            "add",
            &format!(r"{INSTALLER_USER_DATA}\Products\{MSI_APP_SQUISHED}"),
            "/f",
        ],
    )?;
    must(
        "reg.exe",
        &[
            "add",
            &format!(r"{INSTALLER_USER_DATA}\Components\AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
            "/v",
            MSI_APP_SQUISHED,
            "/t",
            "REG_SZ",
            "/d",
            &msi_program,
            "/f",
        ],
    )?;
    // In an account's own registry, which takes no rights to write, and says nothing of this app.
    let planted = r"C:\Program Files\WAM Footprint Planted";
    write(&format!(r"{planted}\a.txt"), "planted")?;
    let key = format!(r"{USER_UNINSTALL}\WAM Hostile Planted");
    uninstall_entry(
        &key,
        &[
            ("DisplayName", "WAM Hostile Planted"),
            ("Publisher", VENDOR),
            ("InstallLocation", planted),
        ],
    )?;
    must(
        "reg.exe",
        &[
            "add",
            &key,
            "/v",
            "SystemComponent",
            "/t",
            "REG_DWORD",
            "/d",
            "1",
            "/f",
        ],
    )?;
    Ok(below_microsoft.is_some())
}

/// Take off the runner everything the app would have left there, so what is found after an install
/// came from the install.
fn remove_the_app() -> TestResult {
    for service in [
        SERVICE_KEPT,
        SERVICE_CHOSEN_OUT,
        SERVICE_RUNS_LEFT_OUT,
        SERVICE_HELPER,
    ] {
        must("sc.exe", &["delete", service])?;
    }
    for task in [TASK_KEPT, TASK_CHOSEN_OUT, TASK_RUNS_LEFT_OUT] {
        must(
            "schtasks.exe",
            &["/delete", "/tn", &format!("{TASK_FOLDER}{task}"), "/f"],
        )?;
    }
    must(
        "reg.exe",
        &["delete", &format!(r"{UNINSTALL}\{APP_NAME}"), "/f"],
    )?;
    must(
        "reg.exe",
        &["delete", &format!(r"{UNINSTALL}\{HELPER_NAME}"), "/f"],
    )?;
    must("reg.exe", &["delete", PART_KEY, "/f"])?;
    must("icacls", &[LOCKED_KEY_FILE, "/grant", "*S-1-5-32-544:F"])?;
    fs::remove_dir_all(SOURCE_ROOT)?;
    fs::remove_dir_all(HELPER_ROOT)?;
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Looking
// ---------------------------------------------------------------------------------------

fn service_state(name: &str) -> TestResult<String> {
    let output = Command::new("sc.exe").args(["qc", name]).output()?;
    Ok(if output.status.success() {
        text(&output.stdout)
    } else {
        "absent".to_owned()
    })
}

fn task_state(name: &str) -> TestResult<String> {
    let output = Command::new("schtasks.exe")
        .args(["/query", "/tn", &format!("{TASK_FOLDER}{name}"), "/xml"])
        .output()?;
    Ok(if output.status.success() {
        text(&output.stdout)
    } else {
        "absent".to_owned()
    })
}

fn tree_state(root: &str) -> TestResult<String> {
    if !Path::new(root).exists() {
        return Ok("absent".to_owned());
    }
    powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Recurse -Force | Sort-Object FullName | ForEach-Object {{ \
           if ($_.PSIsContainer) {{ $_.FullName }} \
           else {{ '{{0}}|{{1}}' -f $_.FullName, \
             (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash }} }}",
        quoted(root)
    ))
}

/// How the machine stands, as Windows' own tools say, for everything an install of the app could
/// have touched.
fn machine_state(root: &str) -> TestResult<BTreeMap<String, String>> {
    let mut state = BTreeMap::new();
    for service in [
        SERVICE_KEPT,
        SERVICE_CHOSEN_OUT,
        SERVICE_RUNS_LEFT_OUT,
        SERVICE_HELPER,
    ] {
        state.insert(format!("service {service}"), service_state(service)?);
    }
    for task in [TASK_KEPT, TASK_CHOSEN_OUT, TASK_RUNS_LEFT_OUT] {
        state.insert(format!("task {task}"), task_state(task)?);
    }
    state.insert(format!("tree {root}"), tree_state(root)?);
    let query = |key: &str| -> TestResult<String> {
        let output = Command::new("reg.exe").args(["query", key]).output()?;
        Ok(if output.status.success() {
            text(&output.stdout)
        } else {
            "absent".to_owned()
        })
    };
    state.insert(
        "entry".to_owned(),
        query(&format!(r"{UNINSTALL}\{APP_NAME}"))?,
    );
    state.insert(
        "the list of uninstall entries".to_owned(),
        query(UNINSTALL)?,
    );
    state.insert("the part's key".to_owned(), query(PART_KEY)?);
    Ok(state)
}

/// Every name in a zip, by .NET's own reader, and what is in it searched with PowerShell.
struct PackageLook {
    names: Vec<String>,
    unpacked: tempfile::TempDir,
}

fn look_in_package(installer: &Path, work: &Path) -> TestResult<PackageLook> {
    const FOOTER_BYTES: usize = 32 + 8 + 21;
    let bytes = fs::read(installer)?;
    let footer = &bytes[bytes.len() - FOOTER_BYTES..];
    let mut length = [0_u8; 8];
    length.copy_from_slice(&footer[32..40]);
    let length = usize::try_from(u64::from_le_bytes(length))?;
    let end = bytes.len() - FOOTER_BYTES;
    let zip = work.join("package.zip");
    fs::write(&zip, &bytes[end - length..end])?;
    let unpacked = tempfile::tempdir()?;
    let shown = powershell(&format!(
        "Add-Type -AssemblyName System.IO.Compression.FileSystem; \
         $zip = [IO.Compression.ZipFile]::OpenRead({zip}); \
         foreach ($e in $zip.Entries) {{ $e.FullName }}; $zip.Dispose(); \
         [IO.Compression.ZipFile]::ExtractToDirectory({zip}, {to})",
        zip = quoted(&zip.display().to_string()),
        to = quoted(&unpacked.path().display().to_string()),
    ))?;
    Ok(PackageLook {
        names: shown
            .lines()
            .map(|line| line.trim().replace('/', "\\"))
            .filter(|line| !line.is_empty())
            .collect(),
        unpacked,
    })
}

impl PackageLook {
    fn holds_file_named(&self, name: &str) -> bool {
        self.names
            .iter()
            .any(|entry| entry.rsplit('\\').next() == Some(name))
    }

    fn holds_folder(&self, folder: &str) -> bool {
        self.names
            .iter()
            .any(|entry| entry.split('\\').any(|part| part == folder))
    }

    /// Whether this text is anywhere in anything in the package, in UTF-8 or UTF-16, whether or not
    /// a file starts by saying which it is.
    fn holds_text(&self, needle: &str) -> TestResult<bool> {
        let found = powershell(&format!(
            "$files = Get-ChildItem -LiteralPath {} -Recurse -File; \
             $found = @(foreach ($encoding in 'UTF8', 'Unicode') {{ \
               $files | Select-String -SimpleMatch -Pattern {} -List -Encoding $encoding }}); \
             if ($found) {{ 'found' }} else {{ 'not found' }}",
            quoted(&self.unpacked.path().display().to_string()),
            quoted(needle),
        ))?;
        Ok(found.trim() == "found")
    }
}

// ---------------------------------------------------------------------------------------
// Installing
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

fn install_id_in(output: &str) -> TestResult<String> {
    Ok(output
        .lines()
        .find_map(|line| line.strip_prefix("Install id: "))
        .ok_or("the install did not say its id")?
        .trim()
        .to_owned())
}

fn uninstall_quietly(id: &str) -> TestResult<InstallerRun> {
    let uninstaller = Path::new(INSTALLS_DIR).join(id).join("uninstall.exe");
    let run = run_installer(&uninstaller, &["--uninstall", id, "--quiet"])?;
    let folder = Path::new(INSTALLS_DIR).join(id);
    let started = Instant::now();
    while folder.exists() && run.success && started.elapsed() < SELF_REMOVAL_PATIENCE {
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(run)
}

fn package_options(app: &InstalledApp, work: &Path) -> PackageOptions {
    PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app.clone())),
        output_dir: work.join("staging").join(APP_NAME),
        current_directory: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(work.join("out")),
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: ScanSource::Live,
        user_profile: ProfileChoice::Default,
        ui_theme: None,
    }
}

fn scan_with(app: &InstalledApp, work: &Path, near_matches: &[String]) -> TestResult<ScanReport> {
    let mut options = scan_options_for(&package_options(app, work));
    options.near_matches = near_matches.to_vec();
    Ok(scan_app(&options, &ScanMonitor::default())?)
}

/// With every option on, and all that was ticked, what looks like a credential is still not there.
fn check_with_every_option_on(
    checks: &mut Checks,
    app: &InstalledApp,
    work: &Path,
    helper_id: &str,
    data_id: &str,
) -> TestResult {
    section("With the License and User data options on");
    fs::create_dir_all(work)?;
    let options = PackageOptions {
        include_user_data: true,
        include_program_data: true,
        include_license_data: true,
        ..package_options(app, work)
    };
    let mut scan = scan_options_for(&options);
    scan.near_matches = vec![helper_id.to_owned(), data_id.to_owned()];
    let report = scan_app(&scan, &ScanMonitor::default())?;
    checks.that(
        "both are ticked",
        report
            .near_matches
            .iter()
            .filter(|near| near.picked)
            .count()
            == 2,
        format!("{:?}", report.near_matches),
    );
    let result = create_package_from_scan(
        &options,
        &report,
        &FootprintChoices::default(),
        &BuildMonitor::default(),
    )?;
    let installer = result
        .self_extracting_archive_path
        .ok_or("the build made no installer")?;
    let look = look_in_package(&installer, work)?;
    for (name, expected) in [
        ("settings.txt", true),
        ("helper.lic", true),
        ("ca.pem", true),
        ("credentials.json", false),
        ("id_rsa", false),
        ("tls.pem", false),
        ("locked.pem", false),
    ] {
        checks.same(
            &format!("{name} in a package of everything"),
            look.holds_file_named(name),
            expected,
        );
    }
    for (canary, expected) in [
        (DATA_NEAR_CANARY, true),
        (LICENSE_CANARY, true),
        (CERTIFICATE_CANARY, true),
        (SECRET_CANARY, false),
        (KEY_CANARY, false),
        (KEY_BY_CONTENT_CANARY, false),
        (LOCKED_FILE_CANARY, false),
    ] {
        checks.same(
            &format!("{canary} in a package of everything"),
            look.holds_text(canary)?,
            expected,
        );
    }
    Ok(())
}

fn folder_choice(relative: &str) -> ChoiceId {
    ChoiceId::Folder(format!(r"{SOURCE_ROOT}\{relative}").to_ascii_lowercase())
}

fn the_choices() -> FootprintChoices {
    let mut choices = FootprintChoices::default();
    choices.set_included(&folder_choice("leave"), false);
    choices.set_included(
        &ChoiceId::Service(SERVICE_CHOSEN_OUT.to_ascii_lowercase()),
        false,
    );
    choices.set_included(
        &ChoiceId::Task(format!("{TASK_FOLDER}{TASK_CHOSEN_OUT}").to_ascii_lowercase()),
        false,
    );
    choices
}

/// What is offered is what is near and nobody's, and everything else stays out of it whatever a
/// choice asks for.
fn check_what_is_offered(
    checks: &mut Checks,
    app: &InstalledApp,
    work: &Path,
) -> TestResult<String> {
    section("What is offered");
    let report = scan_with(app, work, &[])?;
    let names: Vec<&str> = report
        .near_matches
        .iter()
        .map(|near| near.name.as_str())
        .collect();
    println!("offered: {names:?}");
    let helper = report
        .near_matches
        .iter()
        .find(|near| near.name == HELPER_NAME);
    checks.that(
        "the hidden part is offered",
        helper.is_some(),
        format!("{names:?}"),
    );
    if let Some(helper) = helper {
        checks.same("as a hidden entry", helper.kind, NearMatchKind::HiddenEntry);
        checks.that(
            "it says why",
            helper.why.contains("hidden") && helper.why.contains(VENDOR),
            &helper.why,
        );
        checks.that("not ticked", !helper.picked, "picked");
    }
    let extras = report
        .near_matches
        .iter()
        .find(|near| near.path.as_deref() == Some(Path::new(EXTRAS_ROOT)));
    checks.that(
        "a folder of a close name is offered",
        extras.is_some(),
        format!("{names:?}"),
    );
    let settings = report
        .near_matches
        .iter()
        .find(|near| near.path.as_deref() == Some(Path::new(DATA_NEAR_ROOT)));
    checks.that(
        "one in a data folder is offered, and comes with the User data option",
        settings.is_some_and(|near| near.follows_user_data),
        format!("{names:?}"),
    );
    for hostile in [
        PUBLISHER_ROOT,
        MSI_APP_ROOT,
        CODED_ROOT,
        BORROWED_ROOT,
        BORROWED_FROM_GENERIC_ROOT,
        NAMED_LIKE_WINDOWS_CHILD,
        OTHER_APP_ROOT,
        WINDOWS_NAMED_ROOT,
        LINK_ROOT,
        BELOW_MICROSOFT_ROOT,
        r"C:\Program Files",
        r"C:\Program Files\Common Files",
        r"C:\Windows",
        r"C:\Windows\System32",
        r"C:\ProgramData",
        r"C:\Users",
        r"C:\",
        r"C:\Program Files\WAM Footprint Planted",
        IMPOSTOR_ROOT,
    ] {
        checks.that(
            &format!("{hostile} is not offered"),
            !report
                .near_matches
                .iter()
                .any(|near| near.path.as_deref() == Some(Path::new(hostile))),
            format!("{:?}", report.near_matches),
        );
    }
    for planted in report
        .near_matches
        .iter()
        .filter(|near| near.name.starts_with("WAM Hostile") || near.name == OTHER_APP_NAME)
    {
        checks.that("nothing hostile by name", false, &planted.name);
    }

    section("Nothing can be added by asking for it");
    let asked_for = [
        r"folder:c:\windows\system32".to_owned(),
        r"folder:c:\program files".to_owned(),
        format!("folder:{}", OTHER_APP_ROOT.to_ascii_lowercase()),
        format!("folder:{}", LINK_ROOT.to_ascii_lowercase()),
        format!(
            r"entry:{}\wam hostile system32",
            UNINSTALL.to_ascii_lowercase()
        ),
    ];
    let forced = scan_with(app, work, &asked_for)?;
    checks.same(
        "none of it was taken",
        forced.file_inventory.len(),
        report.file_inventory.len(),
    );
    checks.that(
        "and it says so",
        forced
            .warnings
            .iter()
            .filter(|warning| warning.code == "near_match_no_longer_offered")
            .count()
            == asked_for.len(),
        format!("{:?}", forced.warnings),
    );
    Ok(helper.map(|near| near.id.clone()).unwrap_or_default())
}

/// What goes in is what was chosen, as .NET's zip reader and a search of what it unpacks say.
fn check_the_package(checks: &mut Checks, installer: &Path, work: &Path) -> TestResult {
    section("What is in the package");
    let look = look_in_package(installer, work)?;
    println!("{} entries", look.names.len());
    checks.that(
        "the app's main program",
        look.holds_file_named(&format!("{APP_NAME}.exe")),
        "",
    );
    checks.that("what stays is there", look.holds_file_named("keep.txt"), "");
    checks.that(
        "what was left out is not",
        !look.holds_file_named("leave.txt"),
        "",
    );
    checks.that(
        "nor anything in its folder",
        !look.holds_folder("leave"),
        "",
    );
    checks.that(
        "the hidden part is there",
        look.holds_file_named("helper.exe"),
        "",
    );
    checks.that("and what is in it", look.holds_file_named("helper.txt"), "");
    for name in ["credentials.json", "id_rsa", "helper.lic"] {
        checks.that(
            &format!("{name} is not there"),
            !look.holds_file_named(name),
            "",
        );
    }
    checks.that(
        "what was not ticked is not",
        !look.holds_file_named("extras.txt"),
        "",
    );
    checks.that("nor another app's", !look.holds_file_named("other.txt"), "");
    for (canary, expected) in [
        (KEPT_CANARY, true),
        (HELPER_CANARY, true),
        (LEFT_OUT_CANARY, false),
        (SECRET_CANARY, false),
        (KEY_CANARY, false),
        (LICENSE_CANARY, false),
        (EXTRAS_CANARY, false),
    ] {
        checks.same(
            &format!("{canary} in anything in the package"),
            look.holds_text(canary)?,
            expected,
        );
    }
    checks.same(
        "the name of what was left out, in any report or the manifest",
        look.holds_text("leave.txt")?,
        false,
    );
    for (what, canary, expected) in [
        ("the key by the part's name", PART_KEY_CANARY, true),
        (
            "one by that name of another publisher",
            OTHER_VENDOR_KEY_CANARY,
            false,
        ),
        (
            "one by that name under Microsoft",
            MICROSOFT_KEY_CANARY,
            false,
        ),
        (
            "one the part goes by that is another app's name",
            OTHER_APP_KEY_CANARY,
            false,
        ),
    ] {
        checks.same(what, look.holds_text(canary)?, expected);
    }

    let package = EmbeddedPackage::open(installer)?.ok_or("no package in it")?;
    let manifest: PackageManifest = read_embedded_manifest(&package)?;
    let services: Vec<&str> = manifest
        .services_detected
        .iter()
        .map(|service| service.service_name.as_str())
        .collect();
    checks.that(
        "the manifest has the service kept",
        services.contains(&SERVICE_KEPT),
        format!("{services:?}"),
    );
    checks.that(
        "and the hidden part's",
        services.contains(&SERVICE_HELPER),
        format!("{services:?}"),
    );
    checks.that(
        "and not the one chosen out",
        !services.contains(&SERVICE_CHOSEN_OUT),
        format!("{services:?}"),
    );
    checks.that(
        "nor the one that ran what was left out",
        !services.contains(&SERVICE_RUNS_LEFT_OUT),
        format!("{services:?}"),
    );
    let tasks: Vec<&str> = manifest
        .scheduled_tasks_detected
        .iter()
        .map(|task| task.task_name.as_str())
        .collect();
    checks.same("the manifest's tasks", tasks, vec![TASK_KEPT]);
    checks.that(
        "it says what was added",
        manifest
            .added_by_choice
            .iter()
            .any(|name| name == HELPER_NAME),
        format!("{:?}", manifest.added_by_choice),
    );
    let left_out: Vec<&str> = manifest
        .left_out_by_choice
        .iter()
        .map(|item| item.name.as_str())
        .collect();
    println!("left out by choice: {left_out:?}");
    checks.that(
        "and what was left out, with what went with it",
        left_out.iter().any(|name| name.ends_with("leave"))
            && left_out
                .iter()
                .any(|name| name.contains(SERVICE_CHOSEN_OUT))
            && left_out
                .iter()
                .any(|name| name.contains(SERVICE_RUNS_LEFT_OUT))
            && left_out
                .iter()
                .any(|name| name.contains(TASK_RUNS_LEFT_OUT)),
        format!("{left_out:?}"),
    );
    checks.that(
        "credentials are named as left out, as ever",
        manifest
            .secrets_left_out
            .iter()
            .any(|item| item.name.ends_with("credentials.json")),
        format!("{:?}", manifest.secrets_left_out),
    );
    drop(package);
    let verified = verify_installer(installer, &BuildMonitor::default())?;
    checks.that(
        "the package passes its own check",
        verified.package_valid,
        format!("{:?}", verified.errors),
    );
    Ok(())
}

/// What an install leaves on the machine, as Windows' own tools say.
fn check_the_install(checks: &mut Checks) -> TestResult {
    section("What is on the machine");
    checks.that(
        "the service kept",
        service_state(SERVICE_KEPT)? != "absent",
        "",
    );
    checks.same(
        "the one chosen out",
        service_state(SERVICE_CHOSEN_OUT)?,
        "absent".to_owned(),
    );
    checks.same(
        "the one that ran what was left out",
        service_state(SERVICE_RUNS_LEFT_OUT)?,
        "absent".to_owned(),
    );
    let helper_service = service_state(SERVICE_HELPER)?;
    checks.that(
        "the hidden part's service runs its file where it went",
        helper_service.contains(TARGET_ROOT) && helper_service.contains("helper.exe"),
        &helper_service,
    );
    checks.that("the task kept", task_state(TASK_KEPT)? != "absent", "");
    checks.same(
        "the one chosen out",
        task_state(TASK_CHOSEN_OUT)?,
        "absent".to_owned(),
    );
    checks.same(
        "the one that ran what was left out",
        task_state(TASK_RUNS_LEFT_OUT)?,
        "absent".to_owned(),
    );
    let tree = tree_state(TARGET_ROOT)?;
    for (name, expected) in [
        ("keep.txt", true),
        ("helper.txt", true),
        ("helper.exe", true),
        ("leave.txt", false),
        ("leave.exe", false),
        ("extras.txt", false),
        ("credentials.json", false),
        ("id_rsa", false),
        ("helper.lic", false),
        ("other.txt", false),
    ] {
        checks.same(
            &format!("{name} is on the machine"),
            tree.contains(name),
            expected,
        );
    }
    let kept = format!(r"{TARGET_ROOT}\keep\keep.txt");
    let hash = powershell(&format!(
        "(Get-FileHash -LiteralPath {} -Algorithm SHA256).Hash",
        quoted(&kept)
    ))?;
    let expected = powershell(&format!(
        "$sha = [Security.Cryptography.SHA256]::Create(); \
         [BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes({}))).Replace('-', '')",
        quoted(KEPT_CANARY)
    ))?;
    checks.same("what stayed is what it was", hash.trim(), expected.trim());
    Ok(())
}

#[test]
fn what_is_chosen_is_what_is_in_the_package_and_on_the_machine() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let work = tempfile::tempdir()?;
    set_up_the_app()?;
    let tried_a_short_name = set_up_what_must_not_be_offered()?;
    checks.that(
        "there was a short name to try",
        tried_a_short_name,
        "what is below Microsoft has none on this runner",
    );
    let apps = list_installed_apps_from(&SystemContext::live())?;
    let app = apps
        .iter()
        .find(|app| app.display_name == APP_NAME)
        .ok_or("the app is not on the list")?
        .clone();
    checks.that(
        "a hidden entry is not on the list of apps",
        !apps.iter().any(|app| app.display_name == HELPER_NAME),
        HELPER_NAME,
    );

    let helper_id = check_what_is_offered(&mut checks, &app, work.path())?;
    let data_id = scan_with(&app, work.path(), &[])?
        .near_matches
        .iter()
        .find(|near| near.path.as_deref() == Some(Path::new(DATA_NEAR_ROOT)))
        .map(|near| near.id.clone())
        .unwrap_or_default();

    section("Choosing");
    let report = scan_with(&app, work.path(), std::slice::from_ref(&helper_id))?;
    let footprint = Footprint::of(&report, FLAGS)?;
    let helper = report
        .near_matches
        .iter()
        .find(|near| near.name == HELPER_NAME);
    checks.that(
        "ticking it brings it in",
        helper.is_some_and(|near| near.picked),
        format!("{:?}", report.near_matches),
    );
    checks.that(
        "its files are found by the same scan",
        report
            .file_inventory
            .iter()
            .any(|file| file.original_path == Path::new(HELPER_ROOT).join("helper.exe")),
        "no helper.exe",
    );
    checks.that(
        "its service is found by the same rule as the app's",
        report
            .services
            .iter()
            .any(|service| service.service_name == SERVICE_HELPER),
        format!("{:?}", report.services),
    );
    checks.that(
        "what looks like a credential is left out and named, as for the rest of the app",
        report
            .secrets_left_out
            .iter()
            .any(|item| item.name.ends_with("credentials.json"))
            && report
                .secrets_left_out
                .iter()
                .any(|item| item.name.ends_with("id_rsa")),
        format!("{:?}", report.secrets_left_out),
    );
    checks.that(
        "a license follows the License option",
        report
            .licenses_left_out
            .iter()
            .any(|item| item.name.ends_with("helper.lic")),
        format!("{:?}", report.licenses_left_out),
    );
    let choices = the_choices();
    let also = what_goes_with(&report, &footprint, &choices)?.also;
    let also_names: Vec<&str> = also.iter().map(|item| item.title.as_str()).collect();
    println!("goes with it: {also:?}");
    checks.that(
        "the screen is told what goes with it",
        also_names
            .iter()
            .any(|name| name.contains(SERVICE_RUNS_LEFT_OUT))
            && also_names
                .iter()
                .any(|name| name.contains(TASK_RUNS_LEFT_OUT)),
        format!("{also_names:?}"),
    );
    let chosen = apply_choices(&report, &footprint, &choices)?;
    let estimate = footprint.build_inputs(&choices, &chosen.also, 0, false);
    let copied: Vec<_> = chosen
        .report
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
        .collect();
    checks.same(
        "the estimate counts what goes in",
        estimate.payload_files,
        copied.len(),
    );
    checks.same(
        "and how big it is",
        estimate.payload_bytes,
        copied
            .iter()
            .filter_map(|file| file.size_bytes)
            .sum::<u64>(),
    );

    section("Building");
    let options = package_options(&app, work.path());
    let result = create_package_from_scan(&options, &report, &choices, &BuildMonitor::default())?;
    let installer = result
        .self_extracting_archive_path
        .clone()
        .ok_or("the build made no installer")?;
    checks.same(
        "the build copied as many as were counted",
        result.files_copied,
        estimate.payload_files,
    );
    check_the_package(&mut checks, &installer, work.path())?;
    let in_the_package = look_in_package(&installer, work.path())?
        .names
        .iter()
        .filter(|name| name.starts_with(r"files\") && !name.ends_with('\\'))
        .count();
    checks.same(
        "and as many are in the package",
        in_the_package,
        estimate.payload_files,
    );
    check_with_every_option_on(
        &mut checks,
        &app,
        &work.path().join("everything"),
        &helper_id,
        &data_id,
    )?;

    section("Taking it off the runner");
    remove_the_app()?;
    let before = machine_state(TARGET_ROOT)?;

    section("Installing");
    let install = run_installer(&installer, &["--target-root", TARGET_ROOT, "--apply"])?;
    checks.that("the install succeeded", install.success, &install.output);
    checks.that(
        "it says what whoever built it left out",
        install
            .output
            .contains("Left out by whoever built this installer"),
        &install.output,
    );
    checks.that(
        "and what they added",
        install
            .output
            .contains("Added by whoever built this installer"),
        &install.output,
    );
    check_the_install(&mut checks)?;

    section("Uninstalling");
    let id = install_id_in(&install.output)?;
    let uninstall = uninstall_quietly(&id)?;
    checks.that(
        "the uninstall succeeded",
        uninstall.success,
        &uninstall.output,
    );
    checks.that(
        "the record of the install has gone",
        !Path::new(INSTALLS_DIR).join(&id).exists(),
        &id,
    );
    let after = machine_state(TARGET_ROOT)?;
    checks.same(
        "services, tasks, files, and uninstall entries are as they were",
        after,
        before,
    );

    section("A failure late in an install");
    let denied = format!(r"{FAILED_TARGET_ROOT}\keep");
    fs::create_dir_all(&denied)?;
    must("icacls", &[&denied, "/deny", "*S-1-1-0:(WD)"])?;
    let before = machine_state(FAILED_TARGET_ROOT)?;
    let failed = run_installer(
        &installer,
        &["--target-root", FAILED_TARGET_ROOT, "--apply", "--force"],
    )?;
    checks.that(
        "the install reports failure",
        !failed.success,
        &failed.output,
    );
    checks.that(
        "everything could be undone",
        failed.output.contains("Could not be undone: 0"),
        &failed.output,
    );
    let after = machine_state(FAILED_TARGET_ROOT)?;
    checks.same(
        "services, tasks, files, and uninstall entries are as they were",
        after,
        before,
    );
    must("icacls", &[&denied, "/remove:d", "*S-1-1-0"])?;

    checks.finish()
}
