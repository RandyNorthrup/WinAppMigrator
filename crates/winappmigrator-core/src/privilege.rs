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
    path_under_any_env(
        target_root,
        &["ProgramFiles", "ProgramFiles(x86)", "SystemRoot"],
    )
}

/// True when writing to `destination` modifies a machine-wide, protected
/// location (ProgramData or the same protected roots as `target_requires_admin`)
/// and therefore needs elevation. Used to gate real-location user-data restore.
pub fn destination_requires_admin(destination: &Path) -> bool {
    path_under_any_env(
        destination,
        &[
            "ProgramFiles",
            "ProgramFiles(x86)",
            "ProgramData",
            "SystemRoot",
        ],
    )
}

fn path_under_any_env(path: &Path, env_vars: &[&str]) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    env_vars.iter().any(|env_var| {
        std::env::var_os(env_var)
            .map(|value| lower.starts_with(&value.to_string_lossy().to_ascii_lowercase()))
            .unwrap_or(false)
    })
}
