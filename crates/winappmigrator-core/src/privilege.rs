use std::path::Path;
use std::process::Command;

pub fn is_elevated() -> bool {
    let script = "[Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent() | ForEach-Object { $_.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator) }";
    Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .ok()
        .and_then(|output| {
            output.status.success().then(|| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .eq_ignore_ascii_case("true")
            })
        })
        .unwrap_or(false)
}

pub fn target_requires_admin(target_root: &Path) -> bool {
    let lower = target_root.to_string_lossy().to_ascii_lowercase();
    std::env::var_os("ProgramFiles")
        .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
        .unwrap_or(false)
        || std::env::var_os("ProgramFiles(x86)")
            .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
            .unwrap_or(false)
        || std::env::var_os("SystemRoot")
            .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
            .unwrap_or(false)
}
