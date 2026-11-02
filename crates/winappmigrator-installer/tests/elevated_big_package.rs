//! A real package past what Windows can start as one executable, on GitHub's throwaway Windows
//! runner and nowhere else, in a job of its own for the room and time it takes.
//!
//! It holds a real file of over 4 GiB of data that does not compress, goes through the real scan and
//! the real build, and installs. `Get-FileHash` says whether what came out is what went in, .NET
//! reads the data file as a zip, and Windows itself is what starts the loader.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do
//! nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
//! `WAM_ELEVATED_E2E=1` are all set. Never set those on a real PC. With all three set
//! and no elevation they fail, so that the job cannot pass without having run.

use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::{
    BuildMonitor, InstallerStub, PackageOptions, create_package, verify_installer,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const GIB: u64 = 1024 * 1024 * 1024;
/// Over 4 GiB by itself, and well past the limit with nothing to compress it.
const BIG_FILE_BYTES: u64 = 5 * GIB;
/// The source, the data file, what an install unpacks, and what it installs: four of it, and
/// some to spare.
const ROOM_FOR_ALL_OF_IT: u64 = 4 * BIG_FILE_BYTES + 3 * GIB;
const WRITE_CHUNK_BYTES: usize = 1024 * 1024;
const FOOTER_BYTES: u64 = 32 + 8 + 21;
const FOUR_GIB: u64 = 4 * GIB;
const INSTALLS_DIR: &str = r"C:\ProgramData\WinAppMigrator\installs";

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

fn quoted(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "''"))
}

fn file_hash(path: &Path) -> TestResult<String> {
    Ok(powershell(&format!(
        "(Get-FileHash -LiteralPath {} -Algorithm SHA256).Hash.ToLowerInvariant()",
        quoted(path)
    ))?
    .trim()
    .to_owned())
}

/// A file of this size that will not compress, written a chunk at a time.
fn write_noise(path: &Path, len: u64) -> TestResult {
    let mut output = BufWriter::with_capacity(WRITE_CHUNK_BYTES, File::create(path)?);
    let mut state = 0x9E37_79B9_7F4A_7C15_u64;
    let mut written = 0_u64;
    let mut chunk = vec![0_u8; WRITE_CHUNK_BYTES];
    while written < len {
        for word in chunk.as_chunks_mut::<8>().0 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *word = state.to_le_bytes();
        }
        let take = usize::try_from(len - written).map_or(chunk.len(), |left| left.min(chunk.len()));
        output.write_all(&chunk[..take])?;
        written += take as u64;
    }
    output.flush()?;
    Ok(())
}

/// A file of this size that compresses to almost nothing.
fn write_text(path: &Path, len: u64) -> TestResult {
    let mut output = BufWriter::with_capacity(WRITE_CHUNK_BYTES, File::create(path)?);
    let line = b"a line of a very long log, the same each time, which compresses very well\n";
    let mut written = 0_u64;
    while written < len {
        let take = usize::try_from(len - written).map_or(line.len(), |left| left.min(line.len()));
        output.write_all(&line[..take])?;
        written += take as u64;
    }
    output.flush()?;
    Ok(())
}

/// What PowerShell says of each drive, and the one with the most room.
fn roomiest_drive() -> TestResult<(String, u64)> {
    let shown = powershell(
        "Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3' | \
         ForEach-Object { '{0} {1} {2}' -f $_.DeviceID, $_.FreeSpace, $_.Size }",
    )?;
    println!("drives (letter, free bytes, size):\n{shown}");
    let mut best = None;
    for line in shown.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let [letter, free, _] = parts[..] {
            let free: u64 = free.parse()?;
            if best.as_ref().is_none_or(|(_, most)| free > *most) {
                best = Some((letter.to_owned(), free));
            }
        }
    }
    best.ok_or_else(|| "no fixed drive".into())
}

struct InstallerRun {
    success: bool,
    output: String,
}

fn run_installer(installer: &Path, args: &[&str]) -> TestResult<InstallerRun> {
    println!("\n--- {} {}", installer.display(), args.join(" "));
    let started = Instant::now();
    let output = Command::new(installer).args(args).output()?;
    let run = InstallerRun {
        success: output.status.success(),
        output: format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    };
    println!(
        "exit {:?} in {:?}\n{}",
        output.status.code(),
        started.elapsed(),
        run.output
    );
    Ok(run)
}

#[test]
fn a_package_past_four_gib_is_a_loader_and_a_data_file_and_installs() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let (drive, free) = roomiest_drive()?;
    if free < ROOM_FOR_ALL_OF_IT {
        return Err(format!(
            "the roomiest drive here, {drive}, has {free} bytes free, and this takes \
             {ROOM_FOR_ALL_OF_IT}. Find a test that fits rather than let this pass."
        )
        .into());
    }
    let root = PathBuf::from(format!(r"{drive}\WAM Big"));
    let source = root.join("App");
    fs::create_dir_all(&source)?;
    let program = source.join("WAM Big App.exe");
    fs::copy(stub_binary(), &program)?;
    let big = source.join("big data.bin");

    section("A real file of over 4 GiB, of data that does not compress");
    let started = Instant::now();
    write_noise(&big, BIG_FILE_BYTES)?;
    println!("written in {:?}", started.elapsed());
    let big_hash = file_hash(&big)?;
    println!("Get-FileHash: {big_hash}");

    section("Through the real scan and the real build");
    let staging = root.join("staging");
    let out = root.join("out");
    fs::create_dir_all(&out)?;
    let started = Instant::now();
    let package = create_package(&PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(out.clone()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&program, staging.join("WAM Big App"))
    })?;
    println!("built in {:?}", started.elapsed());
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let data = package
        .data_file_path
        .clone()
        .ok_or("no data file was made, though this does not fit in one executable")?;
    let installer_bytes = fs::metadata(&installer)?.len();
    let data_bytes = fs::metadata(&data)?.len();
    println!("loader {installer_bytes} bytes, data file {data_bytes} bytes");
    checks.same(
        "the loader is the stub and a footer",
        installer_bytes,
        fs::metadata(stub_binary())?.len() + FOOTER_BYTES,
    );
    checks.that(
        "the data file is past 4 GiB",
        data_bytes > FOUR_GIB,
        data_bytes,
    );
    checks.that(
        "and past what it holds by no more than a zip takes",
        data_bytes < BIG_FILE_BYTES + 64 * 1024 * 1024,
        data_bytes,
    );
    checks.same(
        "its name goes with the loader's",
        data.with_extension("exe"),
        installer.clone(),
    );

    section("The footer, the data file, and what is in it, by Windows' own tools");
    let loader_bytes = fs::read(&installer)?;
    let footer = &loader_bytes[loader_bytes.len() - usize::try_from(FOOTER_BYTES)?..];
    let footer_hash: String = footer[..32]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    checks.same(
        "Get-FileHash: the footer holds the data file's hash",
        footer_hash,
        file_hash(&data)?,
    );
    let shown = powershell(&format!(
        "Add-Type -AssemblyName System.IO.Compression.FileSystem; \
         $zip = [IO.Compression.ZipFile]::OpenRead({}); \
         $entry = $zip.Entries | Where-Object FullName -like '*big data.bin'; \
         $sha = [Security.Cryptography.SHA256]::Create(); \
         $stream = $entry.Open(); $hash = $sha.ComputeHash($stream); $stream.Dispose(); \
         '{{0}} {{1}}' -f $entry.Length, ([BitConverter]::ToString($hash).Replace('-', '').ToLowerInvariant()); \
         $zip.Dispose()",
        quoted(&data)
    ))?;
    let (length, hash) = shown.trim().split_once(' ').ok_or("no answer from .NET")?;
    checks.same(
        ".NET: the file in the zip is its full size",
        length.parse::<u64>()?,
        BIG_FILE_BYTES,
    );
    checks.same(
        ".NET: and hashes as the file did",
        hash.to_owned(),
        big_hash.clone(),
    );

    section("The app's own check agrees");
    let started = Instant::now();
    let report = verify_installer(&installer, &BuildMonitor::default())?;
    println!("checked in {:?}", started.elapsed());
    checks.that(
        "it passes",
        report.package_valid,
        format!("{:?}", report.errors),
    );

    section("Windows starts the loader, and it installs");
    let target = root.join("Restored");
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string(), "--apply"],
    )?;
    checks.that("it installs", run.success, &run.output);
    checks.that(
        "and says what it read back hashes right",
        run.output.contains("Reinstall hashes valid: true"),
        &run.output,
    );
    checks.same(
        "Get-FileHash: the installed file is the file that went in",
        file_hash(&target.join("big data.bin"))?,
        big_hash,
    );
    let id = run
        .output
        .lines()
        .find_map(|line| line.strip_prefix("Install id: "))
        .ok_or("no install id")?
        .trim()
        .to_owned();
    let uninstaller = Path::new(INSTALLS_DIR).join(&id).join("uninstall.exe");
    checks.same(
        "Get-FileHash: what it left to uninstall with is the bare stub",
        file_hash(&uninstaller)?,
        file_hash(&stub_binary())?,
    );
    let run = run_installer(&uninstaller, &["--uninstall", &id, "--quiet"])?;
    checks.that(
        "it uninstalls",
        run.success && !target.exists(),
        &run.output,
    );

    section("With the data file changed by one byte, it is refused, and it says what to do");
    let mut bytes = fs::OpenOptions::new().write(true).read(true).open(&data)?;
    std::io::Seek::seek(&mut bytes, std::io::SeekFrom::Start(FOUR_GIB + 1))?;
    bytes.write_all(b"\x5A")?;
    drop(bytes);
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string(), "--apply"],
    )?;
    checks.that(
        "refused",
        !run.success && run.output.contains("nothing was read from it"),
        &run.output,
    );
    checks.same("no folder was made", target.exists(), false);
    fs::remove_dir_all(&out)?;
    fs::remove_dir_all(&staging)?;
    fs::remove_file(&big)?;

    section("A package over the limit before compressing, and under it after, is one executable");
    let text = source.join("big text.log");
    write_text(&text, BIG_FILE_BYTES)?;
    let out = root.join("out text");
    fs::create_dir_all(&out)?;
    let started = Instant::now();
    let package = create_package(&PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(out.clone()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&program, root.join("staging text").join("WAM Big App"))
    })?;
    println!("built in {:?}", started.elapsed());
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    checks.same(
        "there is no data file",
        package.data_file_path.clone(),
        None,
    );
    let names: Vec<String> = fs::read_dir(&out)?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    checks.same("and nothing else in the output folder", names.len(), 1);
    checks.that(
        "it is under the limit",
        fs::metadata(&installer)?.len() < FOUR_GIB,
        fs::metadata(&installer)?.len(),
    );
    let text_hash = file_hash(&text)?;
    let target = root.join("Restored text");
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string(), "--apply"],
    )?;
    checks.that("it installs", run.success, &run.output);
    checks.same(
        "Get-FileHash: the installed file is the file that went in",
        file_hash(&target.join("big text.log"))?,
        text_hash,
    );
    checks.finish()
}
