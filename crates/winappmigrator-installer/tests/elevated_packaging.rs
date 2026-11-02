//! What a build and an install really read, and what they really leave, on GitHub's throwaway
//! Windows runner and nowhere else. See `elevated_round_trip.rs`.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do
//! nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
//! `WAM_ELEVATED_E2E=1` are all set. Never set those on a real PC. With all three set
//! and no elevation they fail, so that the job cannot pass without having run.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::{InstallerStub, PackageOptions, create_package};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const PROGRAM_FILES: &str = r"C:\Program Files";
const SMALL_FILES: usize = 2000;
const LARGE_FILE_BYTES: usize = 200 * 1024 * 1024;

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

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
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
        "PowerShell failed with {}: {}{}\nScript: {script}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .into())
}

fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// Bytes that do not compress, from a fixed start, without a crate for it.
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

fn plain_text(len: usize, seed: u64) -> Vec<u8> {
    let line = format!("line {seed} of a file that compresses about as well as a log does\n");
    line.as_bytes().iter().copied().cycle().take(len).collect()
}

/// A real app's worth of files: many small ones, a large one that compresses, and a large one that
/// does not. Returns its program and how much is in it.
fn app_with_a_payload(name: &str) -> TestResult<(PathBuf, u64)> {
    let folder = Path::new(PROGRAM_FILES).join(name);
    fs::create_dir_all(folder.join("small"))?;
    let program = folder.join(format!("{name}.exe"));
    fs::copy(stub_binary(), &program)?;
    let mut total = fs::metadata(&program)?.len();
    for index in 0..SMALL_FILES {
        let bytes = plain_text(4096 + index * 16, index as u64);
        total += bytes.len() as u64;
        fs::write(folder.join("small").join(format!("{index:05}.txt")), bytes)?;
    }
    let text = plain_text(LARGE_FILE_BYTES, 1);
    fs::write(folder.join("large text.log"), &text)?;
    let random = noise(LARGE_FILE_BYTES, 0x9E37_79B9_7F4A_7C15);
    fs::write(folder.join("large noise.bin"), &random)?;
    total += (text.len() + random.len()) as u64;
    Ok((program, total))
}

/// Bytes this process has read so far, by Windows' own count.
fn bytes_read_by_this_process() -> TestResult<u64> {
    let found = powershell(&format!(
        "(Get-CimInstance Win32_Process -Filter 'ProcessId={}').ReadTransferCount",
        std::process::id()
    ))?;
    Ok(found.trim().parse()?)
}

/// Run an installer and watch how much it reads, by Windows' own count, up to its last moment.
fn run_watching_reads(installer: &Path, arguments: &str) -> TestResult<(i32, u64)> {
    let shown = powershell(&format!(
        "$p = Start-Process -FilePath {installer} -ArgumentList {arguments} -PassThru -WindowStyle Hidden; \
         $read = 0; \
         while (-not $p.HasExited) {{ \
           $c = Get-CimInstance Win32_Process -Filter \"ProcessId=$($p.Id)\" -ErrorAction SilentlyContinue; \
           if ($c) {{ $read = $c.ReadTransferCount }}; \
           Start-Sleep -Milliseconds 20 }}; \
         $p.WaitForExit(); \
         \"$($p.ExitCode) $read\"",
        installer = quoted(&installer.display().to_string()),
        arguments = quoted(arguments),
    ))?;
    let mut parts = shown.split_whitespace();
    let code = parts.next().ok_or("no exit code")?.parse()?;
    let read = parts.next().ok_or("no count")?.parse()?;
    Ok((code, read))
}

const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

#[test]
fn how_much_a_build_and_an_install_read() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let name = "WAM Reads App";
    let (program, payload) = app_with_a_payload(name)?;
    let work = tempfile::tempdir()?;
    let output = work.path().join("app");
    let options = PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(work.path().to_path_buf()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&program, &output)
    };
    let before = bytes_read_by_this_process()?;
    let started = Instant::now();
    let package = create_package(&options)?;
    let build_read = bytes_read_by_this_process()?.saturating_sub(before);
    let build_time = started.elapsed();
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let installer_bytes = fs::metadata(&installer)?.len();
    println!(
        "BUILD: payload {:.3} GiB, installer {:.3} GiB, read {:.3} GiB ({:.2} x payload) in {build_time:?}",
        payload as f64 / GIB,
        installer_bytes as f64 / GIB,
        build_read as f64 / GIB,
        build_read as f64 / payload as f64,
    );

    let target = format!(r"{PROGRAM_FILES}\WAM Reads Restored");
    let started = Instant::now();
    let (code, read) = run_watching_reads(
        &installer,
        &format!(
            "--target-root \"{target}\" --apply --log \"{}\"",
            work.path().join("install.log").display()
        ),
    )?;
    println!(
        "INSTALL: exit {code}, read {:.3} GiB ({:.2} x payload, {:.2} x installer) in {:?}",
        read as f64 / GIB,
        read as f64 / payload as f64,
        read as f64 / installer_bytes as f64,
        started.elapsed(),
    );
    if code != 0 {
        println!(
            "{}",
            fs::read_to_string(work.path().join("install.log")).unwrap_or_default()
        );
        return Err(format!("the install failed with {code}").into());
    }
    Ok(())
}
