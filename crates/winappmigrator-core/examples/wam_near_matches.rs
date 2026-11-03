//! Say what would be offered as a near match, for every app listed on this PC. It reads names and
//! counts a little of what is offered. It writes only the file it is given, and changes nothing.
//!
//! Usage: `wam_near_matches <out.txt> [--scan <app name>]...`. With `--scan`, that app also has its
//! whole scan run, as the app would, to see that what it offers is the same.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;

use winappmigrator_core::{
    AppSelection, Footprint, FootprintChoices, NearMatchKind, ScanFlags, ScanMonitor, ScanOptions,
    ScanSource, SystemContext, list_installed_apps_from, near_matches_of, scan_app, what_goes_with,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(out) = args.next() else {
        eprintln!("usage: wam_near_matches <out.txt> [--scan <app name>]...");
        return;
    };
    let mut to_scan = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--scan"
            && let Some(name) = args.next()
        {
            to_scan.push(name.to_lowercase());
        }
    }
    let system = match SystemContext::open(&ScanSource::Live) {
        Ok(system) => system,
        Err(error) => {
            eprintln!("could not look at this PC: {error}");
            return;
        }
    };
    let apps = match list_installed_apps_from(&system) {
        Ok(apps) => apps,
        Err(error) => {
            eprintln!("could not list apps: {error}");
            return;
        }
    };
    let mut text = String::new();
    let (mut with_offers, mut offers, mut hidden) = (0_usize, 0_usize, 0_usize);
    let mut refusals: BTreeMap<&'static str, usize> = BTreeMap::new();
    for app in &apps {
        let look = near_matches_of(app, &system);
        for refused in &look.refused {
            *refusals.entry(refused.why).or_default() += 1;
        }
        let offered: Vec<_> = look.offered().collect();
        if !offered.is_empty() {
            with_offers += 1;
        }
        offers += offered.len();
        let _ = writeln!(
            text,
            "\n{} [{}], {} offered, {} refused",
            app.display_name,
            app.publisher.as_deref().unwrap_or("no publisher"),
            offered.len(),
            look.refused.len()
        );
        for near in offered {
            if near.kind == NearMatchKind::HiddenEntry {
                hidden += 1;
            }
            let _ = writeln!(
                text,
                "  OFFER {:?} {} <{}> {} files, {} bytes{}{}: {}",
                near.kind,
                near.name,
                near.path
                    .as_deref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default(),
                near.file_count,
                near.total_bytes,
                if near.measured_all { "" } else { " or more" },
                if near.follows_user_data {
                    ", with user data"
                } else {
                    ""
                },
                near.why
            );
        }
        for refused in look.refused.iter() {
            let _ = writeln!(
                text,
                "  refused {} <{}>: {}",
                refused.name,
                refused.path.display(),
                refused.why
            );
        }
        if to_scan.contains(&app.display_name.to_lowercase()) {
            let started = std::time::Instant::now();
            let options = ScanOptions {
                selection: AppSelection::InstalledApp(Box::new(app.clone())),
                output_dir: PathBuf::from("reports"),
                current_directory: None,
                recursive_dependencies: true,
                include_hashes: false,
                include_user_data: false,
                include_program_data: false,
                include_license_data: false,
                source: ScanSource::Live,
                user_profile: Default::default(),
                near_matches: Vec::new(),
            };
            let steps = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
            let last = std::sync::Mutex::new(std::time::Instant::now());
            let monitor = {
                let steps = std::sync::Arc::clone(&steps);
                ScanMonitor::new(move |step| {
                    if let (Ok(mut steps), Ok(mut last)) = (steps.lock(), last.lock()) {
                        let _ = write!(steps, " [{:?} then {step:?}]", last.elapsed());
                        *last = std::time::Instant::now();
                    }
                })
            };
            match scan_app(&options, &monitor) {
                Ok(report) => {
                    let _ = writeln!(
                        text,
                        "  SCANNED in {:?}: {} files listed, {} near matches{}",
                        started.elapsed(),
                        report.file_inventory.len(),
                        report.near_matches.len(),
                        steps.lock().map(|steps| steps.clone()).unwrap_or_default()
                    );
                    for near in &report.near_matches {
                        let _ = writeln!(text, "    near: {} ({})", near.name, near.why);
                    }
                    let timing = std::time::Instant::now();
                    if let Ok(footprint) = Footprint::of(&report, ScanFlags::default()) {
                        let _ = writeln!(
                            text,
                            "  Footprint of it in {:?}: {} folder rows",
                            timing.elapsed(),
                            footprint.folders.len()
                        );
                        let mut choices = FootprintChoices::default();
                        let free = footprint.folders.iter().find(|row| row.locked.is_none());
                        if let Some(row) = free {
                            choices.set_included(&row.id, false);
                        }
                        let timing = std::time::Instant::now();
                        let also = what_goes_with(&report, &footprint, &choices);
                        let _ = writeln!(
                            text,
                            "  Leaving out {:?} took {:?}: {:?} also",
                            free.map(|row| row.name.as_str()),
                            timing.elapsed(),
                            also.map(|also| also.len())
                        );
                    }
                }
                Err(error) => {
                    let _ = writeln!(text, "  scan failed: {error}");
                }
            }
        }
    }
    let summary = format!(
        "{} apps listed. {with_offers} have something to offer, {offers} offers in all, {hidden} of \
         them hidden entries. Refused: {refusals:?}\n",
        apps.len()
    );
    println!("{summary}");
    if let Err(error) = std::fs::write(&out, summary + &text) {
        eprintln!("could not write {out}: {error}");
    }
}
