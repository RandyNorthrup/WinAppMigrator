//! The first half of a real move between two PCs, on GitHub's throwaway Windows runner and nowhere
//! else. A harmless fake app is set up here and built into installers with the real code. They go to
//! another runner, which has never seen any of this, and `check-moved-app.ps1` installs them there
//! and looks at the result with Windows' own tools. That runner builds nothing, and has nothing of
//! this project's to run but the installers themselves and that script.
//!
//! **This changes the machine it runs on, and nothing here undoes it.** It does nothing unless
//! `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, `WAM_ELEVATED_E2E=1`, and
//! `WAM_MOVE_OUTPUT` are all set. Never set those on a real PC. With all of them set and no elevation
//! it fails, so that the job cannot pass without having run.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::json;
use winappmigrator_core::archive::create_zip_archive;
use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::{
    AppSelection, BuildMonitor, InstallerStub, PackageOptions, ProfileChoice, ScanSource,
    SystemContext, create_installer_with_data_file, create_package, list_installed_apps_from,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const OUTPUT_VARIABLE: &str = "WAM_MOVE_OUTPUT";
const PROGRAM_FILES: &str = r"C:\Program Files";
const APP: &str = "WAM Move App";
const VENDOR: &str = "WAM Move Vendor";
const SERVICE: &str = "WamMoveSvc";
const TASK_FOLDER: &str = r"\WAM Move\";
const TASK: &str = "Daily";
const REGISTRY_KEY: &str = r"HKLM\Software\WAM Move Vendor\WAM Move App";
const UNINSTALL_KEY: &str =
    r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\WAM Move App";
/// With a file named for a device, which only some Windows can have.
const DEVICE_NAME_APP: &str = "WAM Move Device Name";
/// In the form a package too big for one executable is.
const TWO_FILE_APP: &str = "WAM Move Two Files";

fn is_throwaway_runner(
    github_actions: Option<&str>,
    runner_environment: Option<&str>,
    flag: Option<&str>,
) -> bool {
    github_actions == Some("true")
        && runner_environment == Some("github-hosted")
        && flag == Some("1")
}

/// Whether this run may change the machine, and where to put what it builds.
fn output_folder() -> TestResult<Option<PathBuf>> {
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
        return Ok(None);
    }
    let Some(output) = read(OUTPUT_VARIABLE) else {
        println!("skipping: it is the job that sets {OUTPUT_VARIABLE} that builds these");
        return Ok(None);
    };
    if !is_elevated() {
        return Err("the runner's signals are all set, but this process is not elevated".into());
    }
    Ok(Some(PathBuf::from(output)))
}

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

fn must(program: &str, args: &[&str]) -> TestResult<String> {
    let output = Command::new(program).args(args).output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(format!(
        "{program} {args:?} failed with {}: {}{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .into())
}

fn powershell(script: &str) -> TestResult<String> {
    let script = format!("{POWERSHELL_UTF8_OUTPUT}$ErrorActionPreference = 'Stop'; {script}");
    let output = Command::new("powershell.exe")
        .env_remove("PSModulePath")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(format!(
        "PowerShell failed with {}: {}{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .into())
}

/// Each file below a folder with its hash, by .NET, and nothing of this project's. By the full form of
/// each path, as Windows PowerShell's `Get-FileHash` will not look at a file named `aux.h`, even on a
/// Windows where it is a file like any other.
fn tree_hashes(root: &Path) -> TestResult<Vec<serde_json::Value>> {
    let shown = powershell(&format!(
        r"$root = (Get-Item -LiteralPath '{}').FullName
        $sha = [Security.Cryptography.SHA256]::Create()
        Get-ChildItem -LiteralPath $root -Recurse -File -Force | ForEach-Object {{
            $stream = [IO.File]::OpenRead('\\?\' + $_.FullName)
            $hash = [BitConverter]::ToString($sha.ComputeHash($stream)).Replace('-', '')
            $stream.Dispose()
            '{{0}}|{{1}}' -f $_.FullName.Substring($root.Length + 1), $hash.ToLowerInvariant()
        }}",
        root.display()
    ))?;
    let mut files = Vec::new();
    for line in shown.lines().filter(|line| !line.trim().is_empty()) {
        let (relative, hash) = line
            .trim()
            .rsplit_once('|')
            .ok_or_else(|| format!("could not read: {line}"))?;
        files.push(json!({ "relative": relative, "sha256": hash }));
    }
    Ok(files)
}

fn noise(len: usize, mut state: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(len + 8);
    while bytes.len() < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        bytes.extend_from_slice(&state.to_le_bytes());
    }
    bytes.truncate(len);
    bytes
}

/// A folder with a real program, which on another PC only has to start, and files of every size.
fn app_folder(name: &str) -> TestResult<(PathBuf, PathBuf)> {
    let folder = Path::new(PROGRAM_FILES).join(name);
    fs::create_dir_all(folder.join("data"))?;
    let program = folder.join(format!("{name}.exe"));
    fs::copy(stub_binary(), &program)?;
    fs::write(
        folder.join("readme.txt"),
        "A harmless app for testing a move.\r\n",
    )?;
    fs::write(folder.join(r"data\Größe.txt"), "a name that is not ASCII")?;
    fs::write(folder.join(r"data\noise.bin"), noise(4 * 1024 * 1024, 7))?;
    fs::write(
        folder.join(r"data\large text.log"),
        "the same line of a long log\r\n".repeat(600_000),
    )?;
    Ok((folder, program))
}

fn set_up_the_app() -> TestResult<(PathBuf, PathBuf)> {
    let (folder, program) = app_folder(APP)?;
    let folder_text = folder.display().to_string();
    let program_text = program.display().to_string();
    for (name, value) in [
        ("DisplayName", APP),
        ("Publisher", VENDOR),
        ("DisplayVersion", "1.2.3"),
        ("InstallLocation", folder_text.as_str()),
        ("UninstallString", program_text.as_str()),
    ] {
        must(
            "reg.exe",
            &[
                "add",
                UNINSTALL_KEY,
                "/v",
                name,
                "/t",
                "REG_SZ",
                "/d",
                value,
                "/f",
            ],
        )?;
    }
    must(
        "reg.exe",
        &[
            "add",
            REGISTRY_KEY,
            "/v",
            "Setting",
            "/t",
            "REG_SZ",
            "/d",
            "hello",
            "/f",
        ],
    )?;
    must(
        "sc.exe",
        &[
            "create",
            SERVICE,
            "binPath=",
            &format!("\"{program_text}\" --service"),
            "start=",
            "demand",
        ],
    )?;
    must(
        "schtasks.exe",
        &[
            "/Create",
            "/TN",
            &format!("{TASK_FOLDER}{TASK}"),
            "/TR",
            &format!("\"{program_text}\" --task"),
            "/SC",
            "DAILY",
            "/ST",
            "03:00",
            "/RU",
            "SYSTEM",
            "/F",
        ],
    )?;
    Ok((folder, program))
}

fn real_app_options(output: &Path, installers: &Path) -> TestResult<PackageOptions> {
    let app = list_installed_apps_from(&SystemContext::live())?
        .into_iter()
        .find(|app| app.display_name == APP)
        .ok_or("the installed-app listing does not have the fake app")?;
    Ok(PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app)),
        output_dir: output.to_path_buf(),
        current_directory: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(installers.to_path_buf()),
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: ScanSource::Live,
        user_profile: ProfileChoice::Default,
    })
}

/// This only reads, in a folder of its own, so it runs anywhere, and a mistake in what those hashes
/// are found by shows up here and not twelve minutes into the job on a runner.
#[test]
fn tree_hashes_reads_a_file_named_for_a_device_and_a_name_that_is_not_ascii() -> TestResult {
    const SHA256_OF_ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let folder = tempfile::tempdir()?;
    // The full form, which makes it a file on any Windows.
    let root = fs::canonicalize(folder.path())?;
    fs::create_dir_all(root.join("data"))?;
    fs::write(root.join("aux.h"), "abc")?;
    fs::write(root.join(r"data\Größe.txt"), "abc")?;

    let found = tree_hashes(folder.path())?;

    assert_eq!(found.len(), 2, "{found:?}");
    for relative in ["aux.h", r"data\Größe.txt"] {
        assert!(
            found
                .iter()
                .any(|file| file["relative"] == relative && file["sha256"] == SHA256_OF_ABC),
            "{relative} is not in {found:?}"
        );
    }
    Ok(())
}

#[test]
fn build_what_goes_to_the_other_pc() -> TestResult {
    let Some(output) = output_folder()? else {
        return Ok(());
    };
    fs::create_dir_all(&output)?;
    let work = tempfile::tempdir()?;

    // The whole app, as a person would pick it from the list.
    let (folder, _) = set_up_the_app()?;
    let main = create_package(&real_app_options(&work.path().join("main"), &output)?)?;
    let main_installer = main
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;

    // With a file named for a device, which the Windows it is built on can have.
    let (device_folder, device_program) = app_folder(DEVICE_NAME_APP)?;
    fs::write(device_folder.join("aux.h"), "#define NOTHING\r\n")?;
    let device = create_package(&PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(output.clone()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&device_program, work.path().join(DEVICE_NAME_APP))
    })?;
    let device_installer = device
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;

    // As a small loader and a data file, which also have to get there and stay together.
    let (two_folder, two_program) = app_folder(TWO_FILE_APP)?;
    let two_package = work.path().join(TWO_FILE_APP);
    create_package(&PackageOptions {
        output_dir: two_package.clone(),
        create_self_extracting_archive: false,
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&two_program, &two_package)
    })?;
    let two_zip = work.path().join("two.zip");
    create_zip_archive(&two_package, &two_zip, &BuildMonitor::default())?;
    let two_installer = output.join("WAM_Move_Two_Files_installer.exe");
    let two_data = create_installer_with_data_file(
        &InstallerStub::File(stub_binary()),
        &two_zip,
        &two_installer,
    )?;

    let name_of = |path: &Path| {
        path.file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
    };
    let build = powershell(
        "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion').CurrentBuildNumber",
    )?;
    let expected = json!({
        "source_build": build.trim().parse::<u32>()?,
        "app": {
            "name": APP,
            "installer": name_of(&main_installer),
            "files": tree_hashes(&folder)?,
            "program": format!("{APP}.exe"),
            "service": SERVICE,
            "task_folder": TASK_FOLDER,
            "task": TASK,
            "registry_key": r"HKLM:\Software\WAM Move Vendor\WAM Move App",
        },
        "device_name_app": {
            "installer": name_of(&device_installer),
            "files": tree_hashes(&device_folder)?,
        },
        "two_files": {
            "installer": name_of(&two_installer),
            "data_file": name_of(&two_data),
            "files": tree_hashes(&two_folder)?,
        },
    });
    fs::write(
        output.join("expected.json"),
        serde_json::to_vec_pretty(&expected)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&expected)?);
    Ok(())
}
