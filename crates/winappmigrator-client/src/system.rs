//! What the client does to the PC it runs on, and undoes: a shadow copy of the Windows drive, and two
//! Windows Firewall rules for its own ports from the local network. Each is recorded in a state file
//! the client holds open while it runs, so that a run that ended without undoing them can be cleaned
//! up after by its caretaker, a copy of the client with no window that waits for the run to end
//! however it does, or by the next run, and a run still going is left alone.
//!
//! Everything here goes through tools every Windows from 7 on has: WMI through PowerShell 2.0, and
//! `netsh`. Nothing else on the PC is changed.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::thread;
use std::time::Duration;

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WOW64_64KEY};

/// Read the registry as 64-bit Windows keeps it, from the 32-bit client too, which would otherwise be
/// shown `WOW6432Node`. On 32-bit Windows the flag is ignored.
const KEY_READ: u32 = winreg::enums::KEY_READ | KEY_WOW64_64KEY;

use winappmigrator_link::protocol::About;
use winappmigrator_link::{CODE_DIGITS, DISCOVERY_PORT, LINK_PORT};

const CURRENT_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
const ENVIRONMENT_KEY: &str = r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";
const COMPUTER_NAME_KEY: &str = r"SYSTEM\CurrentControlSet\Control\ComputerName\ActiveComputerName";
const PROFILE_LIST_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList";
const STATE_FOLDER: &str = "WinAppMigrator Client";
const FIRST_WINDOWS_11_BUILD: u32 = 22000;
const STATE_FILE: &str = "running.txt";
const RULE_TCP: &str = "WinAppMigrator Client (link)";
const RULE_UDP: &str = "WinAppMigrator Client (discovery)";
const ERROR_SHARING_VIOLATION: i32 = 32;

/// The argument that starts the client as a run's caretaker, rather than as a run.
pub const CARETAKER: &str = "--caretaker";
/// No console, so that closing a run's window does not end its caretaker too.
const DETACHED_PROCESS: u32 = 0x0000_0008;
/// Out of any job the run is in, so that the job ending does not end the caretaker too. A job that
/// does not allow it refuses, and the caretaker is started without it.
const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
/// No window for a tool started with no console.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
/// Where Windows records where `Program Files` is.
const WINDOWS_FOLDERS_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion";
/// How long a caretaker waits for Windows to let go of a record once its run has ended.
const CARETAKER_TRIES: u32 = 40;
const CARETAKER_PAUSE: Duration = Duration::from_millis(250);

/// Windows' own folder, from where it records it, which only an administrator can change. An
/// elevated program does not go by the environment it was started with.
pub fn windows_folder() -> PathBuf {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(CURRENT_VERSION_KEY, KEY_READ)
        .and_then(|key| key.get_value::<String, _>("SystemRoot"))
        .map_or_else(|_| PathBuf::from(r"C:\Windows"), PathBuf::from)
}

/// A tool in `System32`, started as [`give_windows_environment`] says.
pub fn windows_tool(name: &str) -> Command {
    let mut command = Command::new(windows_folder().join("System32").join(name));
    give_windows_environment(&mut command);
    command
}

/// Start a Windows tool with an environment of Windows' own rather than the person's. An elevated
/// program is given the environment of whoever started it, which they can change without being an
/// administrator. PowerShell is a .NET program, so `COR_PROFILER` would load a DLL into it,
/// `COMPLUS_InstallRoot` a runtime from any folder, and `PSModulePath` other commands in place of
/// Windows' own. As the app does (`set_tool_environment` in the core), with what these tools use.
fn give_windows_environment(command: &mut Command) {
    let windows = windows_folder();
    let system32 = windows.join("System32");
    let powershell_home = system32.join(r"WindowsPowerShell\v1.0");
    let temp = windows.join("Temp");
    let program_data = program_data();
    // A tool told a bare name looks for it in the folder it starts in before `Path`, and the client
    // starts in whatever folder it was run from, which can be a USB drive anyone can write to.
    command
        .current_dir(&system32)
        .env_clear()
        .env("NoDefaultCurrentDirectoryInExePath", "1")
        .env("SystemRoot", &windows)
        .env("windir", &windows)
        .env("SystemDrive", system_drive())
        .env("ComSpec", system32.join("cmd.exe"))
        .env("ProgramData", &program_data)
        .env("ALLUSERSPROFILE", &program_data)
        .env("TEMP", &temp)
        .env("TMP", &temp)
        .env("OS", "Windows_NT")
        .env("PATHEXT", ".COM;.EXE;.BAT;.CMD")
        .env("PSModulePath", powershell_home.join("Modules"));
    // As a 64-bit tool on 64-bit Windows would be told, which is what PowerShell is here.
    if let Some(architecture) = machine_value(ENVIRONMENT_KEY, "PROCESSOR_ARCHITECTURE") {
        command.env("PROCESSOR_ARCHITECTURE", architecture);
    }
    if let Ok(path) =
        std::env::join_paths([system32.clone(), system32.join("Wbem"), powershell_home])
    {
        command.env("Path", path);
    }
}

/// A value of the machine's registry, which only an administrator can change.
fn machine_value(key: &str, name: &str) -> Option<String> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(key, KEY_READ)
        .and_then(|key| key.get_value::<String, _>(name))
        .ok()
}

/// The drive Windows is on, as `C:`.
fn system_drive() -> String {
    windows_folder().to_string_lossy().chars().take(2).collect()
}

/// `ProgramData`, from where Windows records it.
fn program_data() -> PathBuf {
    machine_value(PROFILE_LIST_KEY, "ProgramData").map_or_else(
        || PathBuf::from(r"C:\ProgramData"),
        |value| PathBuf::from(value.replace("%SystemDrive%", &system_drive())),
    )
}

/// Windows' own PowerShell, as the 64-bit program on 64-bit Windows. The client is a 32-bit program,
/// so that one file runs on every Windows, and 64-bit Windows makes no snapshot for a 32-bit one
/// ("Initialization failure", tried on Windows 7). `Sysnative` is how a 32-bit program reaches the
/// 64-bit `System32`, and is only there for one.
fn powershell_program() -> PathBuf {
    let windows = windows_folder();
    let native = windows.join(r"Sysnative\WindowsPowerShell\v1.0\powershell.exe");
    if native.is_file() {
        native
    } else {
        windows.join(r"System32\WindowsPowerShell\v1.0\powershell.exe")
    }
}

fn powershell(script: &str) -> io::Result<Output> {
    let mut command = Command::new(powershell_program());
    give_windows_environment(&mut command);
    command
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .output()
}

fn text_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// What this PC says of itself, read from its registry.
pub fn about_this_pc(snapshot_taken: &str) -> About {
    let value = |key: &str, name: &str| machine_value(key, name).unwrap_or_default();
    let mut product = value(CURRENT_VERSION_KEY, "ProductName");
    // Windows 11 still calls itself Windows 10 there; its builds start at 22000.
    let build: u32 = value(CURRENT_VERSION_KEY, "CurrentBuildNumber")
        .trim()
        .parse()
        .unwrap_or(0);
    if build >= FIRST_WINDOWS_11_BUILD {
        product = product.replacen("Windows 10", "Windows 11", 1);
    }
    let service_pack = value(CURRENT_VERSION_KEY, "CSDVersion");
    let windows_name = if service_pack.is_empty() {
        product
    } else {
        format!("{product} {service_pack}")
    };
    About {
        client_version: crate::VERSION.to_owned(),
        computer_name: value(COMPUTER_NAME_KEY, "ComputerName"),
        windows_name,
        architecture: value(ENVIRONMENT_KEY, "PROCESSOR_ARCHITECTURE"),
        snapshot_taken: snapshot_taken.to_owned(),
    }
}

/// A pairing code: [`CODE_DIGITS`] random digits, shown as `482 913`.
pub fn new_code() -> io::Result<String> {
    // The largest multiple of a million a `u32` holds, so that every code is as likely.
    const LIMIT: u32 = 4_294_000_000;
    loop {
        let mut bytes = [0_u8; 4];
        getrandom::fill(&mut bytes).map_err(|error| io::Error::other(error.to_string()))?;
        let number = u32::from_le_bytes(bytes);
        if number < LIMIT {
            let digits = format!("{:0width$}", number % 1_000_000, width = CODE_DIGITS);
            return Ok(format!("{} {}", &digits[..3], &digits[3..]));
        }
    }
}

/// The record of what a run did, in [`safe_folder`], which only an administrator can change. A folder
/// in `ProgramData` would not do: any account can make one there first, and own it, and could then
/// have this open, empty, and remove a file of its choosing through a link in it.
fn state_path() -> PathBuf {
    safe_folder().join(STATE_FILE)
}

/// What a run has done to the PC, held open for as long as it runs.
pub struct RunState {
    file: File,
    path: PathBuf,
}

impl RunState {
    /// Undo what an earlier run left, unless it is still running, and start this run's record.
    pub fn begin() -> io::Result<Self> {
        fs::create_dir_all(safe_folder())?;
        let path = state_path();
        let left_by_another = path.exists();
        // Held from here, so that the record is read, undone and started again by this run alone.
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(&path)
            .map_err(|error| {
                if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION) {
                    io::Error::other(
                        "the client is already running on this PC, or is cleaning up after one \
                         that was closed. Use that one, or wait a moment and start this again.",
                    )
                } else {
                    error
                }
            })?;
        if left_by_another {
            let mut left = String::new();
            file.read_to_string(&mut left)?;
            undo(&left);
            file.set_len(0)?;
            file.rewind()?;
        }
        Ok(Self { file, path })
    }

    fn note(&mut self, line: &str) -> io::Result<()> {
        writeln!(self.file, "{line}")?;
        self.file.sync_all()
    }

    /// Undo what this run did, and its record.
    pub fn end(self, snapshot: Option<&Snapshot>) {
        if let Some(snapshot) = snapshot {
            snapshot.remove();
        }
        close_firewall();
        let Self { file, path } = self;
        drop(file);
        let _ = fs::remove_file(path);
    }
}

/// Start a copy of this program, with no window, that waits for this run to end and then undoes
/// whatever its record still lists. Closing the run's window ends it before it can clean up after
/// itself, and so would anything else that ends it, but not its caretaker. The two share nothing but
/// a pipe, which closes when the run ends or drops the child, so keep the child until after
/// [`RunState::end`].
pub fn start_caretaker() -> io::Result<Child> {
    let program = std::env::current_exe()?;
    let start = |flags| {
        Command::new(&program)
            .arg(CARETAKER)
            .current_dir(windows_folder().join("System32"))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(flags)
            .spawn()
    };
    start(DETACHED_PROCESS | CREATE_BREAKAWAY_FROM_JOB).or_else(|_| start(DETACHED_PROCESS))
}

/// Be a run's caretaker: wait for the run to end, then undo what its record still lists. A run that
/// cleaned up after itself has removed its record, and a run started since holds its own, which is
/// left alone. Then remove the copy of the client the run and this ran from.
pub fn take_care() {
    let _ = io::copy(&mut io::stdin().lock(), &mut io::sink());
    let path = state_path();
    for _ in 0..CARETAKER_TRIES {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(0)
            .open(&path)
        {
            Ok(mut file) => {
                let mut left = String::new();
                if file.read_to_string(&mut left).is_ok() {
                    undo(&left);
                }
                drop(file);
                // A run that has started since holds its record, which this then cannot remove.
                let _ = fs::remove_file(&path);
                break;
            }
            Err(error) if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION) => {
                thread::sleep(CARETAKER_PAUSE);
            }
            Err(_) => break,
        }
    }
    remove_this_copy_once_ended();
}

/// The folder a run copies the client to and runs it from: `WinAppMigrator Client` in Program Files,
/// which only an administrator can change, and which a new folder there takes after.
///
/// The client is run from wherever a person put it, often a USB drive or a shared folder. Windows
/// loads some DLLs only when they are first needed, by name, and looks for them in the program's own
/// folder first, whatever the program asks: on 32-bit Windows 7, `RtlGenRandom` loads
/// `cryptbase.dll` that way, and a copy of it planted beside the client was loaded into it, elevated.
/// Run from here, the only folder looked in first is one nobody but an administrator can put a DLL in.
fn safe_folder() -> PathBuf {
    machine_value(WINDOWS_FOLDERS_KEY, "ProgramFilesDir")
        .map_or_else(|| PathBuf::from(r"C:\Program Files"), PathBuf::from)
        .join(STATE_FOLDER)
}

fn same_path(left: &Path, right: &Path) -> bool {
    let plain = |path: &Path| path.to_string_lossy().trim_end_matches('\\').to_lowercase();
    plain(left) == plain(right)
}

/// Whether this is a copy in [`safe_folder`].
pub fn runs_from_safe_folder() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|program| program.parent().map(Path::to_path_buf))
        .is_some_and(|folder| same_path(&folder, &safe_folder()))
}

/// Copy this program into [`safe_folder`], under a name of its own, run the copy in this window with
/// the same arguments, and wait for it. Then remove the copy, once its caretaker has let go of it.
/// Nothing before the copy starts loads a DLL by name: tried with Windows' own DLLs planted beside it,
/// on Windows 7 32-bit and 64-bit.
pub fn run_from_safe_folder() -> io::Result<ExitStatus> {
    let folder = safe_folder();
    fs::create_dir_all(&folder)?;
    // What an earlier run could not remove. One still in use is left, and so is the record of what an
    // earlier run did, which the run started here undoes.
    if let Ok(entries) = fs::read_dir(&folder) {
        for entry in entries.flatten() {
            if !entry.file_name().eq_ignore_ascii_case(STATE_FILE) {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    let copy = folder.join(format!("WinAppMigrator Client {}.exe", std::process::id()));
    fs::copy(std::env::current_exe()?, &copy)?;
    let status = Command::new(&copy)
        .args(std::env::args_os().skip(1))
        .current_dir(windows_folder().join("System32"))
        .status();
    for _ in 0..CARETAKER_TRIES {
        match fs::remove_file(&copy) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => {
                thread::sleep(CARETAKER_PAUSE);
            }
            _ => break,
        }
    }
    let _ = fs::remove_dir(&folder);
    status
}

/// Remove the copy this runs from, and its folder if nothing else is in it, a moment after this has
/// ended, which is the soonest Windows lets it go. The run that copied it does the same if it is still
/// there to, and the next run removes any that is left.
fn remove_this_copy_once_ended() {
    let Ok(program) = std::env::current_exe() else {
        return;
    };
    if !runs_from_safe_folder() {
        return;
    }
    // `ping` is what waits: `cmd` has no pause that needs no console. Named by where it is, so that no
    // other `ping` can be found first.
    let ping = windows_folder().join(r"System32\PING.EXE");
    let _ = windows_tool("cmd.exe")
        .raw_arg(format!(
            "/d /s /c \"\"{}\" -n 3 127.0.0.1 >nul & del /f /q \"{}\" & rd \"{}\"\"",
            ping.display(),
            program.display(),
            safe_folder().display()
        ))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
        .spawn();
}

/// Undo each thing a record says a run did.
fn undo(record: &str) {
    for line in record.lines() {
        if let Some(id) = line.strip_prefix("snapshot ") {
            remove_snapshot(id.trim());
        }
    }
    close_firewall();
}

/// A shadow copy of the Windows drive, read through its device path.
pub struct Snapshot {
    pub id: String,
    /// `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN\`.
    pub root: PathBuf,
    /// When it was taken, as WMI says.
    pub taken: String,
}

impl Snapshot {
    /// Take one of the drive Windows is on, and record it before anything else can fail.
    pub fn take(state: &mut RunState) -> io::Result<Self> {
        let drive = system_drive();
        // PowerShell 2.0, as Windows 7 has it: no `::new`, no `-LiteralPath` on `Get-Acl`.
        let script = format!(
            "$r = ([wmiclass]'root\\cimv2:Win32_ShadowCopy').Create('{drive}\\', 'ClientAccessible'); \
             if ($r.ReturnValue -ne 0) {{ [Console]::Out.Write('failed ' + $r.ReturnValue); exit 1 }}; \
             $s = Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $r.ShadowID }}; \
             [Console]::Out.Write($s.ID + '|' + $s.DeviceObject + '|' + $s.InstallDate)"
        );
        let output = powershell(&script)?;
        let text = text_of(&output);
        let mut parts = text.split('|');
        let (Some(id), Some(device), taken) = (parts.next(), parts.next(), parts.next()) else {
            return Err(io::Error::other(format!(
                "Windows did not make a snapshot of {drive}: {text} {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        };
        if !output.status.success()
            || !id.starts_with('{')
            || !device.starts_with(r"\\?\GLOBALROOT\Device\")
        {
            return Err(io::Error::other(format!(
                "Windows did not make a snapshot of {drive}: {text}"
            )));
        }
        state.note(&format!("snapshot {id}"))?;
        Ok(Self {
            id: id.to_owned(),
            root: PathBuf::from(format!("{device}\\")),
            taken: taken.unwrap_or_default().to_owned(),
        })
    }

    pub fn remove(&self) {
        remove_snapshot(&self.id);
    }
}

fn remove_snapshot(id: &str) {
    // Only ever a shadow copy's ID, which is all a record can make this delete.
    let is_an_id = id.len() == 38
        && id.starts_with('{')
        && id.ends_with('}')
        && id[1..37]
            .chars()
            .all(|character| character.is_ascii_hexdigit() || character == '-');
    if !is_an_id {
        return;
    }
    let _ = powershell(&format!(
        "Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq '{id}' }} | ForEach-Object {{ $_.Delete() }}"
    ));
}

/// Let this program's two ports through Windows Firewall, from the local network only.
pub fn open_firewall(program: &Path) -> io::Result<()> {
    close_firewall();
    for (name, protocol, port) in [
        (RULE_TCP, "TCP", LINK_PORT),
        (RULE_UDP, "UDP", DISCOVERY_PORT),
    ] {
        let output = windows_tool("netsh.exe")
            .args(["advfirewall", "firewall", "add", "rule"])
            .arg(format!("name={name}"))
            .args(["dir=in", "action=allow"])
            .arg(format!("program={}", program.display()))
            .arg(format!("protocol={protocol}"))
            .arg(format!("localport={port}"))
            .args(["remoteip=localsubnet", "profile=any"])
            .output()?;
        if !output.status.success() {
            return Err(io::Error::other(format!(
                "Windows Firewall would not let the client listen: {}",
                text_of(&output)
            )));
        }
    }
    Ok(())
}

pub fn close_firewall() {
    for name in [RULE_TCP, RULE_UDP] {
        let _ = windows_tool("netsh.exe")
            .args(["advfirewall", "firewall", "delete", "rule"])
            .arg(format!("name={name}"))
            .output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_is_six_digits_in_two_groups() {
        for _ in 0..50 {
            let code = new_code().unwrap_or_default();
            assert_eq!(code.len(), 7, "{code}");
            assert_eq!(code.chars().nth(3), Some(' '));
            assert!(code.chars().filter(char::is_ascii_digit).count() == CODE_DIGITS);
        }
    }

    #[test]
    fn this_pc_says_what_it_is() {
        let about = about_this_pc("now");
        assert!(about.windows_name.starts_with("Windows"), "{about:?}");
        assert!(!about.computer_name.is_empty());
        assert!(!about.architecture.is_empty());
    }
}
