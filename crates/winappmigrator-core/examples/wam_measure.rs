//! Measure a real build of an installed app: time, peak disk, package size, and how many bytes
//! the process read and wrote in each step, from Windows' own counters. It reads the app and
//! writes only under the work folder it is given, which it removes afterward, so that no package
//! of real data stays around. It never installs anything.
//!
//! Usage: `wam_measure <name of the app> <stub_exe> <work_dir>`

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use winappmigrator_core::{
    AppSelection, BuildMonitor, BuildStep, InstallerStub, PackageManifest, PackageOptions,
    ProfileChoice, ScanSource, SystemContext, create_package_monitored, hidden_command,
    list_installed_apps_from, verify_installer,
};

const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

/// Bytes this process has read and written so far, by Windows' count of its file and device
/// reads. Asking takes a child process, and the time that takes is left out of what is reported.
fn io_counters() -> (u64, u64) {
    let script = format!(
        "$p = Get-CimInstance Win32_Process -Filter 'ProcessId={}'; \
         '{{0}} {{1}}' -f $p.ReadTransferCount, $p.WriteTransferCount",
        std::process::id()
    );
    let output = hidden_command("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();
    let text = output
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    let mut parts = text
        .split_whitespace()
        .map(|part| part.parse().unwrap_or(0));
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

fn tree_bytes(root: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(root) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| match entry.metadata() {
            Ok(metadata) if metadata.is_dir() => tree_bytes(&entry.path()),
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        })
        .sum()
}

#[derive(Clone)]
struct Mark {
    label: &'static str,
    elapsed: Duration,
    read: u64,
    written: u64,
}

struct Marks {
    started: Instant,
    overhead: Duration,
    marks: Vec<Mark>,
}

impl Marks {
    fn mark(&mut self, label: &'static str) {
        let at = Instant::now();
        let elapsed = at
            .duration_since(self.started)
            .saturating_sub(self.overhead);
        let (read, written) = io_counters();
        self.overhead += at.elapsed();
        self.marks.push(Mark {
            label,
            elapsed,
            read,
            written,
        });
    }
}

fn step_label(step: &BuildStep) -> &'static str {
    match step {
        BuildStep::Scanning => "scan",
        BuildStep::Preparing => "prepare",
        BuildStep::RoomChecked(_) => "room",
        BuildStep::Bundling => "bundle",
        BuildStep::Packing { .. } => "pack",
        BuildStep::WritingInstaller => "write installer",
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(name), Some(stub), Some(work)) = (args.next(), args.next(), args.next()) else {
        eprintln!("usage: wam_measure <name of the app> <stub_exe> <work_dir>");
        std::process::exit(2);
    };
    let wanted = name.to_lowercase();
    let apps = match list_installed_apps_from(&SystemContext::live()) {
        Ok(apps) => apps,
        Err(error) => {
            eprintln!("could not list apps: {error}");
            std::process::exit(1);
        }
    };
    let mut matching: Vec<_> = apps
        .into_iter()
        .filter(|app| app.display_name.to_lowercase().contains(&wanted))
        .collect();
    matching.sort_by_key(|app| app.display_name.len());
    let Some(app) = matching.into_iter().next() else {
        eprintln!("no installed app is called {name}");
        std::process::exit(1);
    };
    println!("app: {}", app.display_name);

    let work = PathBuf::from(work);
    let _ = std::fs::create_dir_all(&work);
    let stage = work.join("stage");
    let installer_dir = work.join("out");
    let _ = std::fs::create_dir_all(&stage);
    let _ = std::fs::create_dir_all(&installer_dir);

    let peak = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));
    let sampler = {
        let peak = Arc::clone(&peak);
        let done = Arc::clone(&done);
        let work = work.clone();
        std::thread::spawn(move || {
            while !done.load(Ordering::Relaxed) {
                let started = Instant::now();
                peak.fetch_max(tree_bytes(&work), Ordering::Relaxed);
                // Walking a large tree must not be what the numbers measure.
                std::thread::sleep((started.elapsed() * 9).max(Duration::from_millis(100)));
            }
        })
    };

    let marks = Arc::new(Mutex::new(Marks {
        started: Instant::now(),
        overhead: Duration::ZERO,
        marks: Vec::new(),
    }));
    let seen = Arc::new(Mutex::new(""));
    let monitor = {
        let marks = Arc::clone(&marks);
        let seen = Arc::clone(&seen);
        BuildMonitor::new(move |step| {
            let label = step_label(&step);
            if let (Ok(mut seen), Ok(mut marks)) = (seen.lock(), marks.lock())
                && *seen != label
            {
                *seen = label;
                marks.mark(label);
            }
        })
    };
    let options = PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app)),
        output_dir: stage.join("app"),
        current_directory: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(PathBuf::from(stub))),
        installer_dir: Some(installer_dir.clone()),
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: ScanSource::Live,
        user_profile: ProfileChoice::Default,
    };
    let result = create_package_monitored(&options, &monitor);
    if let Ok(mut marks) = marks.lock() {
        marks.mark("verify");
    }
    let package = match result {
        Ok(package) => package,
        Err(error) => {
            done.store(true, Ordering::Relaxed);
            let _ = sampler.join();
            let _ = std::fs::remove_dir_all(&work);
            eprintln!("build failed: {error}");
            std::process::exit(1);
        }
    };
    // What the app does next with a finished build.
    let verified = package
        .self_extracting_archive_path
        .as_deref()
        .ok_or_else(|| winappmigrator_core::WinAppMigratorError::Archive("no installer".to_owned()))
        .and_then(|installer| verify_installer(installer, &BuildMonitor::default()));
    if let Ok(mut marks) = marks.lock() {
        marks.mark("end");
    }
    done.store(true, Ordering::Relaxed);
    let _ = sampler.join();

    let manifest: Option<PackageManifest> = std::fs::read(&package.manifest_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok());
    let (count, payload) = manifest.as_ref().map_or((0, 0), |manifest| {
        let files: Vec<_> = manifest
            .file_inventory
            .iter()
            .filter(|file| file.should_copy)
            .collect();
        (
            files.len(),
            files.iter().filter_map(|file| file.size_bytes).sum::<u64>(),
        )
    });
    let installer: u64 = [
        package.self_extracting_archive_path.as_deref(),
        package.data_file_path.as_deref(),
    ]
    .into_iter()
    .flatten()
    .filter_map(|path| std::fs::metadata(path).ok())
    .map(|metadata| metadata.len())
    .sum();
    let peak = peak.load(Ordering::Relaxed);
    let marks = marks
        .lock()
        .map(|marks| marks.marks.clone())
        .unwrap_or_default();
    println!(
        "files: {count}  payload: {:.3} GiB  installer: {:.3} GiB  warnings: {}  valid: {:?}",
        payload as f64 / GIB,
        installer as f64 / GIB,
        package.warnings.len(),
        verified
            .map(|report| report.package_valid)
            .map_err(|error| error.to_string()),
    );
    println!(
        "peak disk: {:.3} GiB ({:.2} x installer)",
        peak as f64 / GIB,
        peak as f64 / installer.max(1) as f64
    );
    let mut last = Mark {
        label: "",
        elapsed: Duration::ZERO,
        read: 0,
        written: 0,
    };
    for mark in &marks {
        if !last.label.is_empty() {
            println!(
                "  {:<16} {:>8.1} s  read {:>8.3} GiB ({:>5.2} x payload)  wrote {:>8.3} GiB",
                last.label,
                (mark.elapsed - last.elapsed).as_secs_f64(),
                (mark.read - last.read) as f64 / GIB,
                (mark.read - last.read) as f64 / payload.max(1) as f64,
                (mark.written - last.written) as f64 / GIB,
            );
        }
        last = mark.clone();
    }
    if let Some(end) = marks.last() {
        println!(
            "  {:<16} {:>8.1} s  read {:>8.3} GiB ({:>5.2} x payload)  wrote {:>8.3} GiB",
            "total",
            end.elapsed.as_secs_f64(),
            end.read as f64 / GIB,
            end.read as f64 / payload.max(1) as f64,
            end.written as f64 / GIB,
        );
    }
    drop(package);
    let _ = std::fs::remove_dir_all(&work);
    println!("work folder removed: {}", !work.exists());
}
