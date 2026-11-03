//! What a build and an install really read and leave, what becomes of a package split in two, and
//! what an install refuses before it changes anything, on GitHub's throwaway Windows runner and
//! nowhere else. See `elevated_round_trip.rs`.
//!
//! Everything is looked at with Windows' own tools: `Get-FileHash` for a file, .NET's own zip reader
//! and `tar` for a package, `Get-AuthenticodeSignature` for a signature, and PowerShell's own view of
//! a drive. The app's verifier has no say in any of it.
//!
//! **These tests change the machine they run on, and nothing here undoes it.** They do
//! nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
//! `WAM_ELEVATED_E2E=1` are all set. Never set those on a real PC. With all three set
//! and no elevation they fail, so that the job cannot pass without having run.

use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use winappmigrator_core::archive::create_zip_archive;
use winappmigrator_core::hashing::sha256_file;
use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::{
    ensure_protected_folder, is_elevated, others_could_keep_a_folder_made_in,
    refuse_what_others_put_in,
};
use winappmigrator_core::restore_policy::{MACHINE_ARM64, MACHINE_X64, MACHINE_X86};
use winappmigrator_core::{
    AppArchitecture, BuildMonitor, BuildStep, EmbeddedPackage, InstallMonitor, InstallerStub,
    PackageManifest, PackageOptions, ProgramNeed, ProgramRole, SourcePlatform,
    create_installer_with_data_file, create_package, create_package_monitored,
    create_self_extracting_archive, data_file_beside, read_embedded_manifest, verify_installer,
};
use winappmigrator_core::{ScanMonitor, ScanOptions, SystemContext, hidden_command, scan_app};
use winappmigrator_installer::gui::install_as_the_window_does;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const PROGRAM_FILES: &str = r"C:\Program Files";
const INSTALLS_DIR: &str = r"C:\ProgramData\WinAppMigrator\installs";
const SMALL_FILES: usize = 2000;
const LARGE_FILE_BYTES: usize = 200 * 1024 * 1024;
const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
/// Hash, length, and magic, after a package or at the end of a loader.
const FOOTER_BYTES: usize = 32 + 8 + 21;
const SMALL_DRIVE: &str = r"W:\";

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

/// Windows PowerShell, with its own module path rather than the one PowerShell 7 hands down
/// in the CI step, which stops its own modules from loading.
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

fn path_text(path: &Path) -> String {
    quoted(&path.display().to_string())
}

/// `Get-FileHash`, lowercase as a manifest has it.
fn file_hash(path: &Path) -> TestResult<String> {
    Ok(powershell(&format!(
        "(Get-FileHash -LiteralPath {} -Algorithm SHA256).Hash.ToLowerInvariant()",
        path_text(path)
    ))?
    .trim()
    .to_owned())
}

/// Get-AuthenticodeSignature's status, by name.
fn signature_status(path: &Path) -> TestResult<String> {
    Ok(powershell(&format!(
        "(Get-AuthenticodeSignature -LiteralPath {}).Status.ToString()",
        path_text(path)
    ))?
    .trim()
    .to_owned())
}

/// What a footer at the end of a file says: its hash, and its length.
fn footer_of(path: &Path) -> TestResult<(String, u64)> {
    let bytes = fs::read(path)?;
    let footer = &bytes[bytes.len() - FOOTER_BYTES..];
    let hash: String = footer[..32]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let mut length = [0_u8; 8];
    length.copy_from_slice(&footer[32..40]);
    Ok((hash, u64::from_le_bytes(length)))
}

/// Every entry of a zip with what it hashes to, by .NET's own zip reader, which goes by nothing of
/// this project's.
fn zip_contents_by_dotnet(zip: &Path) -> TestResult<Vec<(String, u64, String)>> {
    let shown = powershell(&format!(
        "Add-Type -AssemblyName System.IO.Compression.FileSystem; \
         $zip = [IO.Compression.ZipFile]::OpenRead({}); \
         $sha = [Security.Cryptography.SHA256]::Create(); \
         foreach ($e in $zip.Entries) {{ \
           $s = $e.Open(); $h = $sha.ComputeHash($s); $s.Dispose(); \
           '{{0}}|{{1}}|{{2}}' -f $e.FullName, $e.Length, \
             ([BitConverter]::ToString($h).Replace('-', '').ToLowerInvariant()) }}; \
         $zip.Dispose()",
        path_text(zip)
    ))?;
    let mut found = Vec::new();
    for line in shown.lines().filter(|line| !line.trim().is_empty()) {
        let parts: Vec<&str> = line.trim().rsplitn(3, '|').collect();
        if parts.len() != 3 {
            return Err(format!("could not read: {line}").into());
        }
        found.push((parts[2].to_owned(), parts[1].parse()?, parts[0].to_owned()));
    }
    Ok(found)
}

/// The package of a one-file installer, cut out by where its footer says it is. It is a standard
/// zip all by itself, which .NET and `tar` can read.
fn cut_out_package(installer: &Path, to: &Path) -> TestResult {
    let bytes = fs::read(installer)?;
    let (_, length) = footer_of(installer)?;
    let end = bytes.len() - FOOTER_BYTES;
    let start = end - usize::try_from(length)?;
    fs::write(to, &bytes[start..end])?;
    Ok(())
}

fn tar_lists(zip: &Path) -> TestResult<usize> {
    let tar = Command::new(r"C:\Windows\System32\tar.exe")
        .arg("-tf")
        .arg(zip)
        .output()?;
    if !tar.status.success() {
        return Err(format!("tar failed: {}", String::from_utf8_lossy(&tar.stderr)).into());
    }
    Ok(String::from_utf8_lossy(&tar.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count())
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

struct InstallerRun {
    success: bool,
    output: String,
}

fn run_installer(installer: &Path, args: &[&str]) -> TestResult<InstallerRun> {
    println!("\n--- {} {}", installer.display(), args.join(" "));
    let output = Command::new(installer).args(args).output()?;
    let run = InstallerRun {
        success: output.status.success(),
        output: format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    };
    println!("exit {:?}\n{}", output.status.code(), run.output);
    Ok(run)
}

fn installs_on_record() -> Vec<String> {
    let Ok(entries) = fs::read_dir(INSTALLS_DIR) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn install_id_in(output: &str) -> TestResult<String> {
    Ok(output
        .lines()
        .find_map(|line| line.strip_prefix("Install id: "))
        .ok_or("the installer did not say its install id")?
        .trim()
        .to_owned())
}

fn uninstall_quietly(id: &str) -> TestResult<InstallerRun> {
    let uninstaller = Path::new(INSTALLS_DIR).join(id).join("uninstall.exe");
    run_installer(&uninstaller, &["--uninstall", id, "--quiet"])
}

/// A package of one program from the real packaging code, unpacked and ready to be changed.
struct HandBuilt {
    work: tempfile::TempDir,
    package_root: PathBuf,
    manifest: PackageManifest,
    program: PathBuf,
}

fn hand_built_package(name: &str, extra: &[(&str, &[u8])]) -> TestResult<HandBuilt> {
    let source = Path::new(PROGRAM_FILES).join(name);
    fs::create_dir_all(&source)?;
    let program = source.join(format!("{name}.exe"));
    fs::copy(stub_binary(), &program)?;
    for (file, bytes) in extra {
        fs::write(source.join(file), bytes)?;
    }
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
    fn zip(&self, file_name: &str) -> TestResult<PathBuf> {
        fs::write(
            self.package_root.join("manifest.json"),
            serde_json::to_vec_pretty(&self.manifest)?,
        )?;
        let zip = self.work.path().join(format!("{file_name}.zip"));
        create_zip_archive(&self.package_root, &zip, &BuildMonitor::default())?;
        Ok(zip)
    }

    fn one_file(&self, file_name: &str) -> TestResult<PathBuf> {
        let zip = self.zip(file_name)?;
        let installer = self.work.path().join(format!("{file_name}.exe"));
        create_self_extracting_archive(&InstallerStub::File(stub_binary()), &zip, &installer)?;
        Ok(installer)
    }

    /// A loader and its data file, which is a package made for this purpose. A real build does the
    /// same when a package will not fit, and the big-package job runs that one for real.
    fn two_files(&self, file_name: &str) -> TestResult<(PathBuf, PathBuf)> {
        let zip = self.zip(file_name)?;
        let installer = self.work.path().join(format!("{file_name}.exe"));
        let data =
            create_installer_with_data_file(&InstallerStub::File(stub_binary()), &zip, &installer)?;
        Ok((installer, data))
    }
}

/// Sign a file with a throwaway certificate made here, trusted only on this runner, and nothing to
/// do with this project's. The certificate is made once per run.
fn sign(path: &Path) -> TestResult<String> {
    let shown = powershell(&format!(
        "$cert = Get-ChildItem Cert:\\LocalMachine\\My | \
           Where-Object Subject -eq 'CN=WAM E2E Packaging Throwaway' | Select-Object -First 1; \
         if (-not $cert) {{ \
           $cert = New-SelfSignedCertificate -Type CodeSigningCert \
             -Subject 'CN=WAM E2E Packaging Throwaway' -CertStoreLocation Cert:\\LocalMachine\\My; \
           foreach ($name in 'Root', 'TrustedPublisher') {{ \
             $store = New-Object Security.Cryptography.X509Certificates.X509Store $name, 'LocalMachine'; \
             $store.Open('ReadWrite'); $store.Add($cert); $store.Close() }} }}; \
         $null = Set-AuthenticodeSignature -LiteralPath {path} -Certificate $cert -HashAlgorithm SHA256; \
         (Get-AuthenticodeSignature -LiteralPath {path}).Status.ToString()",
        path = path_text(path)
    ))?;
    Ok(shown.trim().to_owned())
}

fn flip_a_byte(path: &Path, offset: u64) -> TestResult {
    let mut file = fs::OpenOptions::new().read(true).write(true).open(path)?;
    let mut byte = [0_u8; 1];
    file.seek(SeekFrom::Start(offset))?;
    std::io::Read::read_exact(&mut file, &mut byte)?;
    byte[0] ^= 0x01;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&byte)?;
    Ok(())
}

fn data_file_refused(
    checks: &mut Checks,
    what: &str,
    installer: &Path,
    target: &Path,
) -> TestResult {
    let before = installs_on_record();
    let target_text = target.display().to_string();
    for args in [
        vec!["--target-root", target_text.as_str(), "--apply", "--force"],
        vec!["--target-root", target_text.as_str()],
    ] {
        let run = run_installer(installer, &args)?;
        checks.that(
            &format!("{what}: refused with a clear message"),
            !run.success && run.output.contains(".wamdata"),
            &run.output,
        );
    }
    checks.same(
        &format!("{what}: the install location was not made"),
        target.exists(),
        false,
    );
    checks.same(
        &format!("{what}: no record of an install was started"),
        installs_on_record(),
        before,
    );
    Ok(())
}

#[test]
fn a_signature_on_a_loader_covers_its_data_file() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let payload = noise(5 * 1024 * 1024, 77);
    let package = hand_built_package("WAM Loader App", &[("payload.bin", &payload)])?;
    let (installer, data) = package.two_files("WAM Loader App_installer")?;
    let target = Path::new(PROGRAM_FILES).join("WAM Loader Restored");
    let good_data = fs::read(&data)?;

    section("It is a small loader and a standard zip");
    let stub_bytes = fs::metadata(stub_binary())?.len();
    checks.same(
        "the loader is the stub and a footer",
        fs::metadata(&installer)?.len(),
        stub_bytes + FOOTER_BYTES as u64,
    );
    let (footer_hash, footer_length) = footer_of(&installer)?;
    checks.same(
        "Get-FileHash: the footer holds the data file's hash",
        footer_hash,
        file_hash(&data)?,
    );
    checks.same(
        "the footer holds its length",
        footer_length,
        good_data.len() as u64,
    );
    let listed = zip_contents_by_dotnet(&data)?;
    checks.that(
        ".NET reads the data file as a zip",
        listed.len() > 2,
        format!("{listed:?}"),
    );
    checks.same("tar reads it too", tar_lists(&data)?, listed.len());
    let in_zip = listed
        .iter()
        .find(|(name, _, _)| name.ends_with("payload.bin"))
        .ok_or("the payload is not in the zip")?;
    checks.same(
        ".NET: the payload's hash is what the file's is",
        in_zip.2.clone(),
        file_hash(&package.program.with_file_name("payload.bin"))?,
    );

    section(
        "A signature on the loader is valid, and says nothing about the bytes of the data file",
    );
    checks.same(
        "Get-AuthenticodeSignature: signed",
        sign(&installer)?,
        "Valid".to_owned(),
    );

    section("One byte of the data file changed, with the signature intact");
    flip_a_byte(&data, good_data.len() as u64 / 2)?;
    checks.same(
        "Get-AuthenticodeSignature: the signature cannot see that",
        signature_status(&installer)?,
        "Valid".to_owned(),
    );
    data_file_refused(&mut checks, "a byte changed", &installer, &target)?;

    section("Another package swapped in, whole and correct by itself");
    let other = hand_built_package("WAM Loader Other", &[("other.bin", b"not the same app")])?;
    let other_zip = other.zip("other")?;
    fs::copy(&other_zip, &data)?;
    data_file_refused(&mut checks, "another package", &installer, &target)?;

    section("The data file cut short, longer, and missing");
    fs::write(&data, &good_data[..good_data.len() - 1])?;
    data_file_refused(&mut checks, "cut short", &installer, &target)?;
    let mut longer = good_data.clone();
    longer.push(0);
    fs::write(&data, &longer)?;
    data_file_refused(&mut checks, "longer", &installer, &target)?;
    fs::remove_file(&data)?;
    data_file_refused(&mut checks, "missing", &installer, &target)?;
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string()],
    )?;
    checks.that(
        "missing: it says the two have to stay together",
        run.output.contains("have to stay together"),
        &run.output,
    );

    section("The right file, through a link beside the loader");
    let elsewhere = package.work.path().join("elsewhere");
    fs::create_dir_all(&elsewhere)?;
    let real = elsewhere.join("real.wamdata");
    fs::write(&real, &good_data)?;
    let linked = Command::new("cmd")
        .args(["/D", "/C", "mklink"])
        .arg(&data)
        .arg(&real)
        .output()?;
    checks.that(
        "mklink made the link",
        linked.status.success(),
        String::from_utf8_lossy(&linked.stderr),
    );
    data_file_refused(&mut checks, "through a link", &installer, &target)?;
    fs::remove_file(&data)?;

    section("The loader's footer changed: that is what a signature does see");
    fs::write(&data, &good_data)?;
    let tampered = package.work.path().join("tampered.exe");
    fs::copy(&installer, &tampered)?;
    fs::copy(&data, data_file_beside(&tampered))?;
    // Signing put padding and a table after the footer, so it is found from the stub's end.
    flip_a_byte(&tampered, stub_bytes + 3)?;
    checks.same(
        "Get-AuthenticodeSignature: changing the hash in the footer breaks the signature",
        signature_status(&tampered)?,
        "HashMismatch".to_owned(),
    );
    data_file_refused(&mut checks, "footer changed", &tampered, &target)?;

    section("With the real file back, it installs, and is what it was");
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string(), "--apply"],
    )?;
    checks.that("the signed loader installs", run.success, &run.output);
    checks.that(
        "and says the installed files were read back and hash right",
        run.output.contains("Reinstall hashes valid: true"),
        &run.output,
    );
    checks.same(
        "Get-FileHash: the file installed is the file packaged",
        file_hash(&target.join("payload.bin"))?,
        file_hash(&package.program.with_file_name("payload.bin"))?,
    );

    section("What it leaves behind is the bare stub, and not signed");
    let id = install_id_in(&run.output)?;
    let uninstaller = Path::new(INSTALLS_DIR).join(&id).join("uninstall.exe");
    checks.same(
        "Get-AuthenticodeSignature: the uninstaller is not signed",
        signature_status(&uninstaller)?,
        "NotSigned".to_owned(),
    );
    checks.same(
        "Get-FileHash: the uninstaller is byte for byte the stub",
        file_hash(&uninstaller)?,
        file_hash(&stub_binary())?,
    );
    let run = uninstall_quietly(&id)?;
    checks.that(
        "it still works",
        run.success && !target.exists(),
        &run.output,
    );

    section("And a bare stub by itself still says what it is");
    let bare = package.work.path().join("bare.exe");
    fs::copy(stub_binary(), &bare)?;
    let run = run_installer(&bare, &["--dry-run"])?;
    checks.that(
        "it says it carries no package",
        run.success && run.output.contains("carries no application"),
        &run.output,
    );

    checks.finish()
}

#[test]
fn the_window_installs_what_it_checked_from_where_nobody_else_can_get_in() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let package = hand_built_package("WAM Window Route", &[("payload.bin", b"from the package")])?;
    let installer = package.one_file("window route")?;
    let staging_root = Path::new(r"C:\ProgramData\WinAppMigrator\staging");
    let staged = Arc::new(AtomicBool::new(false));

    section("Where a package is unpacked");
    let seen = Arc::clone(&staged);
    let monitor = InstallMonitor::new(move |_step| {
        if fs::read_dir(staging_root).is_ok_and(|mut entries| entries.next().is_some()) {
            seen.store(true, Ordering::SeqCst);
        }
    });
    let target = Path::new(PROGRAM_FILES).join("WAM Window Route Installed");
    let result = install_as_the_window_does(&installer, &target, true, (false, false), &monitor);
    let result = result.map_err(|error| format!("the install failed: {error}"))?;
    checks.that(
        "the install succeeded",
        result.errors.is_empty(),
        format!("{:?}", result.errors),
    );
    checks.that(
        "and it read back what it installed",
        result
            .verification
            .as_ref()
            .and_then(|report| report.restore_valid)
            == Some(true),
        format!("{:?}", result.verification.is_some()),
    );
    checks.that(
        "it was unpacked inside the folder only administrators can get into",
        staged.load(Ordering::SeqCst),
        staging_root.display(),
    );
    checks.that(
        "and nothing of it is left there",
        fs::read_dir(staging_root).is_ok_and(|mut entries| entries.next().is_none()),
        staging_root.display(),
    );
    checks.same(
        "Get-FileHash: the file installed is the file packaged",
        file_hash(&target.join("payload.bin"))?,
        file_hash(&package.program.with_file_name("payload.bin"))?,
    );
    let id = result.install_id.clone().ok_or("the install gave no id")?;
    let run = uninstall_quietly(&id)?;
    checks.that(
        "it uninstalls",
        run.success && !target.exists(),
        &run.output,
    );

    section("A folder that is an account's own is not one to unpack in");
    let profile = std::env::var("USERPROFILE")?;
    checks.that(
        "an account's own folder gives it what it takes to keep a new folder",
        others_could_keep_a_folder_made_in(Path::new(&profile)),
        &profile,
    );
    checks.that(
        "Program Files does not",
        !others_could_keep_a_folder_made_in(Path::new(PROGRAM_FILES)),
        PROGRAM_FILES,
    );
    let per_user = Path::new(&profile).join(r"AppData\Local\Programs\WAM Window Route");
    staged.store(false, Ordering::SeqCst);
    let result = install_as_the_window_does(&installer, &per_user, false, (false, false), &monitor);
    checks.that(
        "a preview there works",
        result.is_ok(),
        format!("{:?}", result.err()),
    );
    checks.that(
        "and unpacks where nobody else can get in as well",
        staged.load(Ordering::SeqCst),
        staging_root.display(),
    );

    section("A state folder someone got into first is not taken up");
    let state = package.work.path().join("state");
    ensure_protected_folder(&state)?;
    ensure_protected_folder(&state)?;
    checks.that(
        "what this made itself is used again",
        state.is_dir(),
        state.display(),
    );
    let planted = state.join("planted.txt");
    fs::write(&planted, b"x")?;
    let given = Command::new("icacls")
        .arg(&planted)
        .args(["/setowner", "*S-1-5-32-545"])
        .output()?;
    checks.that(
        "the file was given to Users",
        given.status.success(),
        String::from_utf8_lossy(&given.stdout),
    );
    let taken_up = refuse_what_others_put_in(&state);
    checks.that(
        "a later run will not take it up",
        taken_up.is_err(),
        format!("{taken_up:?}"),
    );
    checks.that(
        "and says so",
        format!("{taken_up:?}").contains("An administrator should check"),
        format!("{taken_up:?}"),
    );

    section("What a planted file in the folder of installs does, and the one that is not planted");
    let installs = package.work.path().join("state-installs");
    ensure_protected_folder(&installs)?;
    let give_to_users = |file: &Path| {
        Command::new("icacls")
            .arg(file)
            .args(["/setowner", "*S-1-5-32-545"])
            .output()
    };
    let lock = installs.join("busy.lock");
    fs::write(&lock, b"")?;
    let given = give_to_users(&lock)?;
    checks.that(
        "the lock was given to Users",
        given.status.success(),
        String::from_utf8_lossy(&given.stdout),
    );
    let with_only_the_lock = refuse_what_others_put_in(&installs);
    checks.that(
        "a lock is what a running install keeps there, and is nobody's doing",
        with_only_the_lock.is_ok(),
        format!("{with_only_the_lock:?}"),
    );
    let planted_install = installs.join("planted-record");
    fs::create_dir(&planted_install)?;
    let given = give_to_users(&planted_install)?;
    checks.that(
        "the record was given to Users",
        given.status.success(),
        String::from_utf8_lossy(&given.stdout),
    );
    let with_a_record = refuse_what_others_put_in(&installs);
    checks.that(
        "a record someone else made there is not taken up",
        with_a_record.is_err(),
        format!("{with_a_record:?}"),
    );

    checks.finish()
}

/// What this reports of itself, run with a hostile environment by the test below, one line each. An
/// elevated process gets what its account has set, so any of this could be up to a program of theirs.
const HOSTILE_CHILD_VARIABLE: &str = "WAM_HOSTILE_ENVIRONMENT_CHILD";
const HOSTILE_APP: &str = "WAM Hostile Env App";

#[test]
fn reports_what_it_is_told_under_a_hostile_environment() -> TestResult {
    if std::env::var_os(HOSTILE_CHILD_VARIABLE).is_none() {
        return Ok(());
    }
    let system = SystemContext::live();
    println!("WINDOWS={}", system.windows_dir().display());
    println!("PROGRAMDATA={:?}", system.program_data());
    println!("PROGRAMFILES={:?}", system.program_files_dirs());
    let tools = hidden_command("cmd").args(["/D", "/C", "set"]).output()?;
    println!("TOOL={}", String::from_utf8_lossy(&tools.stdout));
    let program = Path::new(PROGRAM_FILES)
        .join(HOSTILE_APP)
        .join(format!("{HOSTILE_APP}.exe"));
    let report = scan_app(
        &ScanOptions {
            include_program_data: true,
            ..ScanOptions::for_executable(&program, Path::new(""))
        },
        &ScanMonitor::default(),
    )?;
    for file in &report.file_inventory {
        println!("FILE={}", file.original_path.display());
    }
    Ok(())
}

#[test]
fn what_an_account_says_of_windows_own_folders_is_not_gone_by() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let hostile = tempfile::tempdir()?;
    let folder = |name: &str| -> TestResult<PathBuf> {
        let folder = hostile.path().join(name);
        fs::create_dir_all(&folder)?;
        Ok(folder)
    };
    let (temp, data, files, windows) = (
        folder("temp")?,
        folder("data")?,
        folder("files")?,
        folder("windows")?,
    );
    let planted = data.join(HOSTILE_APP);
    fs::create_dir_all(&planted)?;
    fs::write(planted.join("planted.dll"), b"x")?;
    let environment = [
        ("TEMP", temp.as_path()),
        ("TMP", temp.as_path()),
        ("ProgramData", data.as_path()),
        ("ALLUSERSPROFILE", data.as_path()),
        ("ProgramFiles", files.as_path()),
        ("ProgramW6432", files.as_path()),
        ("SystemRoot", windows.as_path()),
        ("windir", windows.as_path()),
        ("USERPROFILE", files.as_path()),
        ("APPDATA", files.as_path()),
        ("LOCALAPPDATA", files.as_path()),
    ];
    let hostile_text = path_text(hostile.path());
    let package = hand_built_package(HOSTILE_APP, &[])?;

    section("Control: what is set reaches a program that goes by it");
    let mut control = Command::new("cmd");
    control.args(["/D", "/C", "echo %ProgramData%;%TEMP%"]);
    for (name, value) in environment {
        control.env(name, value);
    }
    let control = String::from_utf8_lossy(&control.output()?.stdout).to_string();
    println!("{control}");
    checks.that(
        "a plain child sees it",
        control.contains(&path_text(&data)) && control.contains(&path_text(&temp)),
        &control,
    );

    section("What the app makes of it, as itself");
    let mut child = Command::new(std::env::current_exe()?);
    child.args([
        "--exact",
        "reports_what_it_is_told_under_a_hostile_environment",
        "--nocapture",
        "--test-threads=1",
    ]);
    child.env(HOSTILE_CHILD_VARIABLE, "1");
    for (name, value) in environment {
        child.env(name, value);
    }
    let child = child.output()?;
    let told = String::from_utf8_lossy(&child.stdout).to_string();
    println!("{told}\n{}", String::from_utf8_lossy(&child.stderr));
    checks.that("it ran", child.status.success(), &told);
    checks.that(
        "it asked for the folders ran",
        told.contains("WINDOWS=") && told.contains("PROGRAMDATA=") && told.contains("TOOL="),
        &told,
    );
    let says_hostile: Vec<&str> = told
        .lines()
        .filter(|line| line.contains(&hostile_text))
        .collect();
    checks.that(
        "not one thing it says goes by what was set",
        says_hostile.is_empty(),
        format!("{says_hostile:#?}"),
    );
    checks.that(
        "and what was planted there is not the app's",
        !told.contains("planted.dll"),
        &told,
    );

    section("A real install, in the same environment");
    let installer = package.one_file("hostile.exe")?;
    let target = tempfile::tempdir()?;
    let target_text = path_text(&target.path().join("app"));
    let mut install = Command::new(&installer);
    install.args(["--apply", "--target-root", target_text.as_str()]);
    for (name, value) in environment {
        install.env(name, value);
    }
    let install = install.output()?;
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&install.stdout),
        String::from_utf8_lossy(&install.stderr)
    );
    println!("exit {:?}\n{said}", install.status.code());
    checks.that("it installs", install.status.success(), &said);
    let mut made_in_it = Vec::new();
    let mut folders = vec![hostile.path().to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(&folder)?.flatten() {
            if entry.path().is_dir() {
                folders.push(entry.path());
            } else if entry.file_name() != "planted.dll" {
                made_in_it.push(entry.path());
            }
        }
    }
    checks.that(
        "nothing was made where it was told to look",
        made_in_it.is_empty(),
        format!("{made_in_it:#?}"),
    );
    if install.status.success() {
        let id = install_id_in(&said)?;
        uninstall_quietly(&id)?;
    }
    fs::remove_dir_all(Path::new(PROGRAM_FILES).join(HOSTILE_APP))?;
    checks.finish()
}

#[test]
fn a_file_changing_during_a_build_is_what_was_read_all_the_way_through() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let name = "WAM Changing App";
    let source = Path::new(PROGRAM_FILES).join(name);
    fs::create_dir_all(&source)?;
    let program = source.join(format!("{name}.exe"));
    fs::copy(stub_binary(), &program)?;
    const LARGE: usize = 32 * 1024 * 1024;
    const SMALL: usize = 1024 * 1024;
    let large_path = source.join("changing large.bin");
    let small_path = source.join("changing small.txt");
    fs::write(&large_path, noise(LARGE, 1))?;
    fs::write(&small_path, plain_text(SMALL, 0))?;
    fs::write(source.join("steady.txt"), plain_text(4096, 1))?;

    section("Build it with both files being written the whole time");
    let stop = Arc::new(AtomicBool::new(false));
    let writer = {
        let stop = Arc::clone(&stop);
        let (large_path, small_path) = (large_path.clone(), small_path.clone());
        std::thread::spawn(move || -> std::io::Result<u64> {
            let mut large = fs::OpenOptions::new().write(true).open(&large_path)?;
            let mut small = fs::OpenOptions::new().write(true).open(&small_path)?;
            let mut writes = 0_u64;
            while !stop.load(Ordering::Relaxed) {
                large.seek(SeekFrom::Start(
                    writes % (LARGE as u64 / 1024 / 1024) * 1024 * 1024,
                ))?;
                large.write_all(&noise(1024 * 1024, writes + 2))?;
                small.seek(SeekFrom::Start(0))?;
                small.write_all(&plain_text(SMALL, writes))?;
                writes += 1;
            }
            Ok(writes)
        })
    };
    let work = tempfile::tempdir()?;
    let output = work.path().join(name);
    let package = create_package(&PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(work.path().to_path_buf()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&program, &output)
    })?;
    stop.store(true, Ordering::Relaxed);
    let writes = writer.join().map_err(|_| "the writer panicked")??;
    println!("the files were rewritten {writes} times during the build");
    checks.that("they really changed during it", writes > 0, writes);
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;

    section("By .NET's zip reader, each entry is what the manifest says, however it came out");
    let manifest = read_embedded_manifest(
        &EmbeddedPackage::open(&installer)?.ok_or("the installer has no package")?,
    )?;
    let zip = work.path().join("cut out.zip");
    cut_out_package(&installer, &zip)?;
    let listed = zip_contents_by_dotnet(&zip)?;
    checks.that(
        "tar reads the same zip",
        tar_lists(&zip)? == listed.len(),
        "it does not",
    );
    let hash_in_zip = |entry_name: &str| {
        listed
            .iter()
            .find(|(name, _, _)| name.ends_with(entry_name))
            .map(|(_, _, hash)| hash.clone())
    };
    let manifest_hash = |entry_name: &str| {
        manifest
            .file_inventory
            .iter()
            .find(|file| file.original_path.ends_with(entry_name))
            .and_then(|file| file.hash_sha256.clone())
    };
    for entry in ["changing large.bin", "changing small.txt", "steady.txt"] {
        checks.same(
            &format!(".NET: {entry} in the zip is what the manifest recorded"),
            hash_in_zip(entry),
            manifest_hash(entry),
        );
    }
    let report = verify_installer(&installer, &BuildMonitor::default())?;
    checks.that(
        "the app's own check agrees",
        report.package_valid,
        format!("{:?}", report.errors),
    );

    section("Change them again, and install what was packaged");
    fs::write(&large_path, noise(LARGE, 99))?;
    fs::write(&small_path, plain_text(SMALL, 999_999))?;
    let target = Path::new(PROGRAM_FILES).join("WAM Changing Restored");
    let run = run_installer(
        &installer,
        &["--target-root", &target.display().to_string(), "--apply"],
    )?;
    checks.that("it installs", run.success, &run.output);
    for entry in ["changing large.bin", "changing small.txt"] {
        checks.same(
            &format!("Get-FileHash: {entry} installed is what the manifest recorded"),
            Some(file_hash(&target.join(entry))?),
            manifest_hash(entry),
        );
    }
    checks.that(
        "and what is at the source by now is something else",
        Some(file_hash(&large_path)?) != manifest_hash("changing large.bin"),
        "it is the same",
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&id)?;
    checks.that("it uninstalls", run.success, &run.output);
    checks.finish()
}

/// A small NTFS volume, on a VHD, at a letter of its own.
fn attach_small_volume(folder: &Path, megabytes: u32) -> TestResult<PathBuf> {
    let vhd = folder.join("wam-e2e-small.vhd");
    let script = folder.join("diskpart attach.txt");
    fs::write(
        &script,
        format!(
            "create vdisk file=\"{vhd}\" maximum={megabytes} type=fixed\r\n\
             select vdisk file=\"{vhd}\"\r\n\
             attach vdisk\r\n\
             create partition primary\r\n\
             format fs=ntfs quick label=WAMSMALL\r\n\
             assign letter=W\r\n",
            vhd = vhd.display()
        ),
    )?;
    let shown = Command::new("diskpart.exe")
        .arg("/s")
        .arg(&script)
        .output()?;
    println!("{}", String::from_utf8_lossy(&shown.stdout));
    let started = Instant::now();
    while !Path::new(SMALL_DRIVE).exists() {
        if started.elapsed() > Duration::from_secs(30) {
            return Err("the small volume did not appear".into());
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    Ok(vhd)
}

fn drive_contents(drive: &str) -> TestResult<Vec<String>> {
    let shown = powershell(&format!(
        "Get-ChildItem -LiteralPath {} -Force | \
         Where-Object Name -notin 'System Volume Information', '$RECYCLE.BIN' | \
         ForEach-Object {{ $_.Name }}",
        quoted(drive)
    ))?;
    Ok(shown
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

#[test]
fn an_install_or_a_build_without_room_is_refused_before_anything_changes() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let payload = noise(100 * 1024 * 1024, 5);
    let package = hand_built_package("WAM Room App", &[("payload.bin", &payload)])?;
    let installer = package.one_file("WAM Room App_installer")?;
    let keep = tempfile::tempdir()?;
    attach_small_volume(keep.path(), 32)?;
    let before = installs_on_record();

    section("Installing to a drive with too little room");
    let target = format!(r"{SMALL_DRIVE}App");
    let run = run_installer(&installer, &["--target-root", &target, "--apply"])?;
    checks.that("it is refused", !run.success, &run.output);
    checks.that(
        "it says how much is needed and how much is free, on that drive",
        run.output.contains("not enough room")
            && run.output.contains("W:")
            && run.output.contains("needs up to")
            && run.output.contains(" free "),
        &run.output,
    );
    checks.same(
        "PowerShell: nothing was put on the drive, staging included",
        drive_contents(SMALL_DRIVE)?,
        Vec::<String>::new(),
    );
    checks.same(
        "no record of an install was started",
        installs_on_record(),
        before.clone(),
    );

    section("A preview does not need the install location to have room, and says so anyway");
    let run = run_installer(&installer, &["--target-root", &target])?;
    checks.that(
        "the preview goes ahead, unpacking elsewhere",
        run.success,
        &run.output,
    );
    checks.same(
        "PowerShell: nothing was put on the drive by that",
        drive_contents(SMALL_DRIVE)?,
        Vec::<String>::new(),
    );

    section("Building to a drive with too little room");
    let work = tempfile::tempdir()?;
    let output = work.path().join("WAM Room Build");
    let out_on_small = PathBuf::from(format!(r"{SMALL_DRIVE}Out"));
    let refused = create_package(&PackageOptions {
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(out_on_small.clone()),
        include_user_data: false,
        include_program_data: false,
        ..PackageOptions::for_executable(&package.program, &output)
    });
    match &refused {
        Err(error) => {
            let text = error.to_string();
            println!("{text}");
            checks.that(
                "it says how much is needed and how much is free, on that drive",
                text.contains("not enough room")
                    && text.contains("W:")
                    && text.contains("needs up to"),
                &text,
            );
        }
        Ok(_) => checks.that("it is refused", false, "it was built"),
    }
    checks.same(
        "PowerShell: nothing was put on the drive by that either",
        drive_contents(SMALL_DRIVE)?,
        Vec::<String>::new(),
    );

    section("A small install that does fit goes ahead on that drive");
    let small = hand_built_package("WAM Room Small", &[("little.txt", b"a little")])?;
    let mut trimmed = small.manifest.clone();
    trimmed
        .file_inventory
        .retain(|file| !file.original_path.ends_with("WAM Room Small.exe"));
    let small = HandBuilt {
        manifest: trimmed,
        ..small
    };
    fs::remove_file(small.package_root.join("files").join("WAM Room Small.exe"))?;
    let small_installer = small.one_file("WAM Room Small_installer")?;
    let run = run_installer(&small_installer, &["--target-root", &target, "--apply"])?;
    checks.that("it installs", run.success, &run.output);
    checks.that(
        "it says what room it used",
        run.output.contains("Room: up to"),
        &run.output,
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&id)?;
    checks.that("it uninstalls", run.success, &run.output);
    checks.finish()
}

#[test]
fn an_app_that_cannot_run_here_is_refused_before_anything_changes() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let target = Path::new(PROGRAM_FILES).join("WAM Platform Restored");
    let target_text = target.display().to_string();
    let mut package = hand_built_package("WAM Platform App", &[])?;
    let main = package.manifest.main_executable_path.clone();

    section("What a real build recorded, against what Windows says itself");
    let source_build = powershell(
        "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion').CurrentBuildNumber",
    )?;
    let source_build: u32 = source_build.trim().parse()?;
    checks.same(
        "the source's build",
        package
            .manifest
            .source_platform
            .as_ref()
            .and_then(|source| source.windows_build),
        Some(source_build),
    );
    checks.same(
        "the source's architecture",
        package
            .manifest
            .source_platform
            .as_ref()
            .map(|source| source.architecture.clone()),
        Some(AppArchitecture::X64),
    );
    let machine_of_main = powershell(&format!(
        "$b = [IO.File]::ReadAllBytes({}); $pe = [BitConverter]::ToUInt32($b, 0x3C); \
         [BitConverter]::ToUInt16($b, $pe + 4)",
        path_text(&package.program)
    ))?;
    let recorded = package
        .manifest
        .program_needs
        .iter()
        .find(|program| program.role == ProgramRole::Main)
        .ok_or("no main program was recorded")?;
    checks.same(
        "the main program's machine, by reading its header here too",
        recorded.machine,
        machine_of_main.trim().parse::<u16>()?,
    );

    let needing = |machine: u16, role: ProgramRole, major: u16| ProgramNeed {
        path: main.clone(),
        role,
        machine,
        min_windows_major: major,
        min_windows_minor: 0,
        is_managed: false,
    };
    for (what, need, expected) in [
        (
            "an Arm64 program on x64",
            needing(MACHINE_ARM64, ProgramRole::Main, 6),
            "Arm64",
        ),
        (
            "a 32-bit driver on x64",
            needing(MACHINE_X86, ProgramRole::Driver, 6),
            "32-bit x86",
        ),
        (
            "an Arm64 driver on x64",
            needing(MACHINE_ARM64, ProgramRole::Driver, 6),
            "Arm64",
        ),
        (
            "a program that needs a newer Windows",
            needing(MACHINE_X64, ProgramRole::Main, 99),
            "99.0",
        ),
    ] {
        section(what);
        package.manifest.program_needs = vec![need];
        let installer = package.one_file(&what.replace(' ', "-"))?;
        let before = installs_on_record();
        for args in [
            vec!["--target-root", target_text.as_str(), "--apply", "--force"],
            vec!["--target-root", target_text.as_str()],
        ] {
            let run = run_installer(&installer, &args)?;
            checks.that(
                &format!("{what}: refused, and says why"),
                !run.success
                    && run.output.contains("cannot run on this PC")
                    && run.output.contains(expected),
                &run.output,
            );
        }
        checks.same(
            &format!("{what}: no folder was made"),
            target.exists(),
            false,
        );
        checks.same(
            &format!("{what}: no record was started"),
            installs_on_record(),
            before,
        );
    }

    section("A newer Windows than this as its source only warns");
    package.manifest.program_needs = vec![needing(MACHINE_X64, ProgramRole::Main, 6)];
    package.manifest.source_platform = Some(SourcePlatform {
        windows_build: Some(99_999),
        architecture: AppArchitecture::X64,
    });
    let installer = package.one_file("from-a-newer-windows")?;
    let run = run_installer(&installer, &["--target-root", &target_text, "--apply"])?;
    checks.that("it installs", run.success, &run.output);
    checks.that(
        "it says so in the same list a person agrees to",
        run.output
            .contains("Note: This app was on Windows build 99999"),
        &run.output,
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&id)?;
    checks.that("it uninstalls", run.success, &run.output);

    section("A helper for another architecture only warns");
    package.manifest.program_needs = vec![
        needing(MACHINE_X64, ProgramRole::Main, 6),
        ProgramNeed {
            path: main.with_file_name("arm helper.exe"),
            ..needing(MACHINE_ARM64, ProgramRole::Other, 6)
        },
    ];
    package.manifest.source_platform = None;
    let installer = package.one_file("with-a-helper")?;
    let run = run_installer(&installer, &["--target-root", &target_text, "--apply"])?;
    checks.that("it installs", run.success, &run.output);
    checks.that(
        "it says so",
        run.output.contains("arm helper.exe") && run.output.contains("Note:"),
        &run.output,
    );
    let id = install_id_in(&run.output)?;
    let run = uninstall_quietly(&id)?;
    checks.that("it uninstalls", run.success, &run.output);
    checks.finish()
}

#[test]
fn a_cancelled_build_leaves_nothing_beside_where_the_installer_would_have_gone() -> TestResult {
    if !may_change_this_machine()? {
        return Ok(());
    }
    let mut checks = Checks::default();
    let payload = noise(32 * 1024 * 1024, 3);
    let package = hand_built_package("WAM Cancel App", &[("payload.bin", &payload)])?;
    let out = tempfile::tempdir()?;
    let staging = tempfile::tempdir()?;
    let cancel: Arc<std::sync::Mutex<Option<BuildMonitor>>> = Arc::new(std::sync::Mutex::new(None));
    let canceller = Arc::clone(&cancel);
    let monitor = BuildMonitor::new(move |step| {
        if let BuildStep::Packing { bytes_done, .. } = step
            && bytes_done > 0
            && let Ok(canceller) = canceller.lock()
            && let Some(monitor) = canceller.as_ref()
        {
            monitor.cancel();
        }
    });
    *cancel.lock().map_err(|error| error.to_string())? = Some(monitor.clone());
    let built = create_package_monitored(
        &PackageOptions {
            create_self_extracting_archive: true,
            self_extracting_stub: Some(InstallerStub::File(stub_binary())),
            installer_dir: Some(out.path().to_path_buf()),
            include_user_data: false,
            include_program_data: false,
            ..PackageOptions::for_executable(&package.program, staging.path().join("app"))
        },
        &monitor,
    );
    checks.that("it stops", built.is_err(), "it finished");
    let left: Vec<String> = fs::read_dir(out.path())?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    checks.same(
        "nothing is left in the output folder",
        left,
        Vec::<String>::new(),
    );
    checks.finish()
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
        installer = path_text(installer),
        arguments = quoted(arguments),
    ))?;
    let mut parts = shown.split_whitespace();
    let code = parts.next().ok_or("no exit code")?.parse()?;
    let read = parts.next().ok_or("no count")?.parse()?;
    Ok((code, read))
}

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
    let installer = package
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let report = verify_installer(&installer, &BuildMonitor::default())?;
    let build_read = bytes_read_by_this_process()?.saturating_sub(before);
    let build_time = started.elapsed();
    assert!(report.package_valid, "{:?}", report.errors);
    let installer_bytes = fs::metadata(&installer)?.len();
    println!(
        "BUILD: payload {:.3} GiB, installer {:.3} GiB, read {:.3} GiB ({:.2} x payload) in {build_time:?}",
        payload as f64 / GIB,
        installer_bytes as f64 / GIB,
        build_read as f64 / GIB,
        build_read as f64 / payload as f64,
    );

    let target = format!(r"{PROGRAM_FILES}\WAM Reads Restored");
    let log = work.path().join("install.log");
    let started = Instant::now();
    let (code, read) = run_watching_reads(
        &installer,
        &format!(
            "--target-root \"{target}\" --apply --log \"{}\"",
            log.display()
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
        println!("{}", fs::read_to_string(&log).unwrap_or_default());
        return Err(format!("the install failed with {code}").into());
    }
    // A real read-back of what was installed stands behind that exit code, and this looks too.
    assert_eq!(
        file_hash(Path::new(&format!(r"{target}\large noise.bin")))?,
        sha256_file(&program.with_file_name("large noise.bin"))?
    );
    Ok(())
}
