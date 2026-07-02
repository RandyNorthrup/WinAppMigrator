//! Removes package-extraction directories left by previous or crashed installer
//! runs, so nothing accumulates over time regardless of how a prior run ended.

use std::path::Path;

/// Prefix of the directories the installer extracts its embedded package into.
pub const EXTRACTION_PREFIX: &str = "winappmigrator-install-";

/// Remove stale extraction directories from the two places the installer stages
/// its package — beside the installer executable and in the system temp
/// directory. Call this at startup **before** creating the current run's
/// directory, so only prior leftovers are removed. Best-effort: a directory
/// still in use by a concurrently running installer cannot be removed on Windows
/// and is silently skipped.
pub fn sweep_stale_extractions(executable: &Path) {
    let mut roots = vec![std::env::temp_dir()];
    if let Some(parent) = executable.parent() {
        let parent = parent.to_path_buf();
        if !roots.contains(&parent) {
            roots.push(parent);
        }
    }
    for root in roots {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_extraction = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(EXTRACTION_PREFIX));
            if is_extraction && path.is_dir() {
                let _ = std::fs::remove_dir_all(&path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_stale_extraction_dirs_beside_the_executable() {
        let base = std::env::temp_dir().join(format!("wam-cleanup-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&base);
        let exe = base.join("SomeApp_installer.exe");
        let _ = std::fs::write(&exe, b"stub");
        let stale = base.join(format!("{EXTRACTION_PREFIX}ABC123"));
        let _ = std::fs::create_dir_all(stale.join("files"));
        let _ = std::fs::write(stale.join("files").join("x.bin"), b"data");
        let unrelated = base.join("keep-me");
        let _ = std::fs::create_dir_all(&unrelated);

        sweep_stale_extractions(&exe);

        assert!(!stale.exists(), "stale extraction dir should be removed");
        assert!(unrelated.exists(), "unrelated dir must be kept");
        let _ = std::fs::remove_dir_all(&base);
    }
}
