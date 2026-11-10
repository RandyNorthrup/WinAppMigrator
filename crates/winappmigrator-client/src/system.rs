//! What the client does to the PC it runs on, and undoes: a shadow copy of the Windows drive, and two
//! Windows Firewall rules for its own ports from the local network. Each is recorded in a state file
//! the client holds open while it runs, so that a run that ended without undoing them can be cleaned
//! up after by the next, and a run still going is left alone.
//!
//! Everything here goes through tools every Windows from 7 on has: WMI through PowerShell 2.0, and
//! `netsh`. Nothing else on the PC is changed.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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

/// Windows' own folder, from where it records it, which only an administrator can change. An
/// elevated program does not go by the environment it was started with.
pub fn windows_folder() -> PathBuf {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(CURRENT_VERSION_KEY, KEY_READ)
        .and_then(|key| key.get_value::<String, _>("SystemRoot"))
        .map_or_else(|_| PathBuf::from(r"C:\Windows"), PathBuf::from)
}

fn tool(name: &str) -> PathBuf {
    windows_folder().join("System32").join(name)
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
    Command::new(powershell_program())
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
    let machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let value = |key: &str, name: &str| {
        machine
            .open_subkey_with_flags(key, KEY_READ)
            .and_then(|key| key.get_value::<String, _>(name))
            .unwrap_or_default()
    };
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
        getrandom::getrandom(&mut bytes).map_err(|error| io::Error::other(error.to_string()))?;
        let number = u32::from_le_bytes(bytes);
        if number < LIMIT {
            let digits = format!("{:0width$}", number % 1_000_000, width = CODE_DIGITS);
            return Ok(format!("{} {}", &digits[..3], &digits[3..]));
        }
    }
}

/// The folder the state file is in, under `ProgramData`.
fn state_folder() -> PathBuf {
    let program_data = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(PROFILE_LIST_KEY, KEY_READ)
        .and_then(|key| key.get_value::<String, _>("ProgramData"))
        .map(|value| {
            let drive = windows_folder()
                .to_string_lossy()
                .chars()
                .take(2)
                .collect::<String>();
            value.replace("%SystemDrive%", &drive)
        })
        .unwrap_or_else(|_| r"C:\ProgramData".to_owned());
    PathBuf::from(program_data).join(STATE_FOLDER)
}

/// What a run has done to the PC, held open for as long as it runs.
pub struct RunState {
    file: File,
    path: PathBuf,
}

impl RunState {
    /// Undo what an earlier run left, unless it is still running, and start this run's record.
    pub fn begin() -> io::Result<Self> {
        let folder = state_folder();
        fs::create_dir_all(&folder)?;
        let path = folder.join(STATE_FILE);
        if let Ok(left) = fs::read_to_string(&path) {
            match OpenOptions::new().write(true).share_mode(0).open(&path) {
                Ok(_) => undo(&left),
                Err(error) if error.raw_os_error() == Some(ERROR_SHARING_VIOLATION) => {
                    return Err(io::Error::other(
                        "the client is already running on this PC. Use that one, or stop it first.",
                    ));
                }
                Err(_) => {}
            }
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .share_mode(0)
            .open(&path)?;
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
        let drive: String = windows_folder().to_string_lossy().chars().take(2).collect();
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
        let output = Command::new(tool("netsh.exe"))
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
        let _ = Command::new(tool("netsh.exe"))
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
