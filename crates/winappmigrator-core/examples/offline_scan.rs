//! Smoke-test the offline-volume scan against a mounted disk.
//!
//! Usage: `offline_scan <volume_root> [output_file]`
//! e.g.   `offline_scan F:\ report.txt`
//!
//! Must be run elevated (offline scanning mounts the disk's registry hives via
//! `reg load`, which needs administrator rights). Writes a short report of the
//! installed apps found on the volume to `output_file`, or stdout if omitted.

use std::path::PathBuf;

use winappmigrator_core::{ScanSource, SystemContext, list_installed_apps_from};

fn main() {
    let mut args = std::env::args().skip(1);
    let volume = args.next().unwrap_or_else(|| r"F:\".to_owned());
    let output = args.next();

    let mut report = String::new();
    report.push_str(&format!("Offline scan of {volume}\n"));
    report.push_str(&format!(
        "Elevated: {}\n\n",
        winappmigrator_core::privilege::is_elevated()
    ));

    let source = ScanSource::offline(PathBuf::from(&volume));
    match SystemContext::open(&source) {
        Ok(system) => match list_installed_apps_from(&system) {
            Ok(apps) => {
                report.push_str(&format!("OK: found {} installed apps.\n", apps.len()));
                for app in apps.iter().take(40) {
                    report.push_str(&format!(
                        "  - {} | {} | {} | {}\n",
                        app.display_name,
                        app.publisher.as_deref().unwrap_or("?"),
                        app.version.as_deref().unwrap_or("?"),
                        app.install_location
                            .as_ref()
                            .map(|path| path.display().to_string())
                            .unwrap_or_default()
                    ));
                }
                if apps.len() > 40 {
                    report.push_str(&format!("  … and {} more.\n", apps.len() - 40));
                }
            }
            Err(error) => report.push_str(&format!("LIST ERROR: {error}\n")),
        },
        Err(error) => report.push_str(&format!("OPEN ERROR: {error}\n")),
    }

    match output {
        Some(path) => {
            if let Err(error) = std::fs::write(&path, report.as_bytes()) {
                eprintln!("could not write {path}: {error}");
            }
        }
        None => print!("{report}"),
    }
}
