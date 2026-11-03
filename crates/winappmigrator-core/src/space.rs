//! Whether there is room for a build or an install, before either starts.
//!
//! What is needed is worked out from what is known beforehand, and always as the most it can come
//! to, since a build that fills a drive part-way has wasted the time, and on a failing drive there
//! may not be another try. A drive that will not say how much room it has is said not to have said,
//! rather than stopping anything or being taken to be fine.

use std::collections::HashMap;
use std::path::{Component, Path, PathBuf, Prefix};

use crate::zip_stream::ENTRY_OVERHEAD_BYTES;

const KIB: u64 = 1024;
const MIB: u64 = 1024 * KIB;
const GIB: u64 = 1024 * MIB;
/// Windows cannot start an executable of 4 GiB or more. A little under, so that signing a finished
/// installer, which adds its certificates to the file, cannot take it over.
pub const MOST_INSTALLER_BYTES: u64 = 4 * GIB - MIB;
/// A manifest, the reports, and what goes with them, of a build with few files.
const METADATA_BASE_BYTES: u64 = 32 * MIB;
/// What a manifest and a report say of each file.
const METADATA_PER_FILE_BYTES: u64 = 4 * KIB;
/// A runtime's installer, which is not known until it has been downloaded. The largest there is now
/// is the .NET hosting bundle, at about 110 MiB.
const PREREQUISITE_ALLOWANCE_BYTES: u64 = 150 * MIB;
/// An install's record, and what it logs.
const INSTALL_RECORD_BYTES: u64 = 32 * MIB;
/// How far apart two answers about how much room is free can be and still be the same volume's.
const SAME_VOLUME_FREE_SLACK_BYTES: u64 = 64 * MIB;
/// A driver image, whose size a package does not record. The largest are a few MiB.
pub const DRIVER_ALLOWANCE_BYTES: u64 = 32 * MIB;

/// What is known of a build once its scan is done.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuildInputs {
    /// What is to be read for the package: `copyable_bytes`, in the terms of a scan.
    pub payload_bytes: u64,
    pub payload_files: usize,
    /// The biggest of those. On a drive it can take a copy in the protected folder to read one.
    pub largest_file_bytes: u64,
    /// License files and driver images, which are copied to the protected folder first.
    pub held_bytes: u64,
    pub prerequisites: usize,
    pub stub_bytes: u64,
    pub connected_drive: bool,
}

/// How much a build needs, at the most it can come to, and where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildNeeds {
    /// In the output folder, at the moment there is the most there.
    pub output_bytes: u64,
    /// In the protected folder.
    pub staging_bytes: u64,
    /// The most a finished package and its stub can come to, with nothing in it compressing.
    pub installer_bytes: u64,
    /// Whether it is sure to fit in one executable. If not, how much it compresses decides.
    pub surely_one_file: bool,
}

/// The most a build can need. Nothing is copied to the protected folder but what has to be, and the
/// package is written directly where it goes, so the most there is at once is the package itself.
///
/// The one exception is a package that might fit in one executable and is not sure to. It is written
/// by itself, since it cannot be told yet whether it goes after the stub or beside it. If it then fits,
/// it is copied after the stub, and there are two of it for that long.
pub fn build_needs(inputs: &BuildInputs) -> BuildNeeds {
    let metadata = metadata_bytes(inputs.payload_files);
    let held = inputs.held_bytes;
    let prerequisites = inputs.prerequisites as u64 * PREREQUISITE_ALLOWANCE_BYTES;
    let package = inputs
        .payload_bytes
        .saturating_add(held)
        .saturating_add(prerequisites)
        .saturating_add(metadata)
        .saturating_add(inputs.payload_files as u64 * ENTRY_OVERHEAD_BYTES);
    let installer_bytes = package.saturating_add(inputs.stub_bytes);
    let surely_one_file = installer_bytes <= MOST_INSTALLER_BYTES;
    let output_bytes = if surely_one_file {
        installer_bytes
    } else {
        installer_bytes.max(MOST_INSTALLER_BYTES.saturating_mul(2))
    };
    let spool = if inputs.connected_drive {
        inputs.largest_file_bytes
    } else {
        0
    };
    BuildNeeds {
        output_bytes,
        staging_bytes: metadata
            .saturating_add(held)
            .saturating_add(prerequisites)
            .saturating_add(spool),
        installer_bytes,
        surely_one_file,
    }
}

fn metadata_bytes(files: usize) -> u64 {
    METADATA_BASE_BYTES.saturating_add((files as u64).saturating_mul(METADATA_PER_FILE_BYTES))
}

/// One file an install puts somewhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedFile {
    pub destination: PathBuf,
    pub new_bytes: u64,
    /// How big what it replaces is, if something is there.
    pub replaced_bytes: u64,
}

/// A place something is written, how much goes there, and what it is for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceNeed {
    pub place: PathBuf,
    pub bytes: u64,
    pub what: &'static str,
}

/// What an install needs, by where it goes. Unpacking takes the whole package in the staging folder,
/// each file takes its size wherever it goes, and the record of the install takes a copy of the
/// stub. A file that is replaced stays until the install is undone or finished with. On the drive
/// that holds the records it is moved aside and takes no more room than it did, and from anywhere
/// else it is copied there.
pub fn install_needs(
    staging: &Path,
    unpacked_bytes: u64,
    files: &[PlannedFile],
    state_root: &Path,
    stub_bytes: u64,
    on_records_volume: &dyn Fn(&Path) -> bool,
) -> Vec<PlaceNeed> {
    let mut needs = vec![
        PlaceNeed {
            place: staging.to_path_buf(),
            bytes: unpacked_bytes,
            what: "unpacking the package",
        },
        PlaceNeed {
            place: state_root.to_path_buf(),
            bytes: INSTALL_RECORD_BYTES.saturating_add(stub_bytes),
            what: "the record of the install",
        },
    ];
    for file in files {
        needs.push(PlaceNeed {
            place: file.destination.clone(),
            bytes: file.new_bytes,
            what: "the app's files",
        });
        if file.replaced_bytes > 0 && !on_records_volume(&file.destination) {
            needs.push(PlaceNeed {
                place: state_root.to_path_buf(),
                bytes: file.replaced_bytes,
                what: "keeping what it replaces",
            });
        }
    }
    needs
}

/// What a preview needs: it unpacks the package, and goes no further.
pub fn preview_needs(staging: &Path, unpacked_bytes: u64) -> Vec<PlaceNeed> {
    vec![PlaceNeed {
        place: staging.to_path_buf(),
        bytes: unpacked_bytes,
        what: "unpacking the package",
    }]
}

/// A drive letter, if a path says one.
fn drive_letter(path: &Path) -> Option<char> {
    match path.components().next() {
        Some(Component::Prefix(prefix)) => match prefix.kind() {
            Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                Some(char::from(letter).to_ascii_uppercase())
            }
            _ => None,
        },
        _ => None,
    }
}

/// Whether a path is on the same volume as another, as far as Windows will say. That takes the same
/// size, and about the same room free, since something else may be writing to it between asking
/// about one and the other. Anything else counts as no, which asks for more room rather than less.
pub fn on_same_volume(left: &Path, right: &Path) -> bool {
    let (Some(left_letter), Some(right_letter)) = (drive_letter(left), drive_letter(right)) else {
        return false;
    };
    if left_letter != right_letter {
        return false;
    }
    match (volume_stats_at(left), volume_stats_at(right)) {
        (Some(left), Some(right)) => {
            left.total_bytes == right.total_bytes
                && left.free_bytes.abs_diff(right.free_bytes) <= SAME_VOLUME_FREE_SLACK_BYTES
        }
        _ => false,
    }
}

/// Where a build writes, and how much it writes there at the most.
pub fn build_places(
    staging: &Path,
    installer: Option<&Path>,
    inputs: &BuildInputs,
) -> Vec<PlaceNeed> {
    let needs = build_needs(inputs);
    match installer {
        Some(installer) => vec![
            PlaceNeed {
                place: staging.to_path_buf(),
                bytes: needs.staging_bytes,
                what: "what a build works with",
            },
            PlaceNeed {
                place: installer.to_path_buf(),
                bytes: needs.output_bytes,
                what: "the installer",
            },
        ],
        // A package that stays a folder is all in that folder, and is the size of what is read.
        None => vec![PlaceNeed {
            place: staging.to_path_buf(),
            bytes: needs.staging_bytes.saturating_add(inputs.payload_bytes),
            what: "the package",
        }],
    }
}

/// How much room a build of this would need before it starts, given what a drive said it has, which
/// is the check a build makes again when it starts. It works in and writes to the output folder.
pub fn room_before_a_build(
    inputs: &BuildInputs,
    output_folder: &Path,
    stats: Option<VolumeStats>,
) -> RoomCheck {
    RoomCheck::of(
        &build_places(
            &output_folder.join("staging"),
            Some(&output_folder.join("installer")),
            inputs,
        ),
        &|_| stats,
    )
}

/// How a volume answers for how big it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolumeStats {
    pub free_bytes: u64,
    pub total_bytes: u64,
}

/// What Windows says of the volume a path is on, or would be on, as this account sees it, so that a
/// quota counts. A volume mounted on a folder answers for itself, by the nearest folder that exists.
pub fn volume_stats_at(path: &Path) -> Option<VolumeStats> {
    let existing = path
        .ancestors()
        .find(|ancestor| std::fs::metadata(ancestor).is_ok())?;
    let stats = fs4::statvfs(existing).ok()?;
    // A drive that reports nothing has not said it is full. It has not said anything.
    (stats.total_space() > 0).then(|| VolumeStats {
        free_bytes: stats.available_space(),
        total_bytes: stats.total_space(),
    })
}

/// One drive's part of a check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriveRoom {
    /// What to call it: a drive letter's root, or the place for one that has none.
    pub drive: String,
    pub needed_bytes: u64,
    /// `None` if the drive would not say.
    pub free_bytes: Option<u64>,
    /// What the room is for, in the order it came up.
    pub what_for: Vec<&'static str>,
}

/// Whether there is room on every drive something would be written to.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoomCheck {
    pub drives: Vec<DriveRoom>,
}

impl RoomCheck {
    /// Add up what is needed by the volume it goes on. Two places are the same volume if they have
    /// the same letter and the same size, which unlike how much is free cannot change between asking
    /// about one and the other. A folder with a volume mounted on it has the size of that volume. Two
    /// volumes of exactly the same size would be counted together, as if all of it went on the fuller
    /// one, which can only ask for too much.
    pub fn of(needs: &[PlaceNeed], stats: &dyn Fn(&Path) -> Option<VolumeStats>) -> Self {
        let mut keyed: Vec<(Option<char>, Option<u64>, DriveRoom)> = Vec::new();
        let mut asked: HashMap<&Path, Option<VolumeStats>> = HashMap::new();
        for need in needs {
            let place = need.place.as_path();
            let folder = place.parent().unwrap_or(place);
            let answer = *asked.entry(folder).or_insert_with(|| stats(place));
            let letter = drive_letter(place);
            let size = answer.map(|answer| answer.total_bytes);
            let found = keyed
                .iter_mut()
                .find(|(key_letter, key_size, _)| *key_letter == letter && *key_size == size);
            match found {
                Some((_, _, room)) => {
                    room.needed_bytes = room.needed_bytes.saturating_add(need.bytes);
                    room.free_bytes = room.free_bytes.min(answer.map(|answer| answer.free_bytes));
                    if !room.what_for.contains(&need.what) {
                        room.what_for.push(need.what);
                    }
                }
                None => keyed.push((
                    letter,
                    size,
                    DriveRoom {
                        drive: letter.map_or_else(
                            || place.display().to_string(),
                            |letter| format!("{letter}:"),
                        ),
                        needed_bytes: need.bytes,
                        free_bytes: answer.map(|answer| answer.free_bytes),
                        what_for: vec![need.what],
                    },
                )),
            }
        }
        Self {
            drives: keyed.into_iter().map(|(_, _, room)| room).collect(),
        }
    }

    /// [`Self::of`], asking Windows.
    pub fn on_this_pc(needs: &[PlaceNeed]) -> Self {
        Self::of(needs, &volume_stats_at)
    }

    /// The drives without enough room, as a sentence each.
    pub fn shortfalls(&self) -> Vec<String> {
        self.drives
            .iter()
            .filter_map(|drive| {
                let free = drive.free_bytes?;
                (free < drive.needed_bytes).then(|| {
                    format!(
                        "{} needs up to {} free for {}, and has {}",
                        drive.drive,
                        size_text(drive.needed_bytes),
                        drive.what_for.join(" and "),
                        size_text(free),
                    )
                })
            })
            .collect()
    }

    /// What could not be checked, as a sentence each.
    pub fn could_not_tell(&self) -> Vec<String> {
        self.drives
            .iter()
            .filter(|drive| drive.free_bytes.is_none())
            .map(|drive| {
                format!(
                    "{} would not say how much room it has, so whether the {} it needs fits was \
                     not checked",
                    drive.drive,
                    size_text(drive.needed_bytes),
                )
            })
            .collect()
    }

    /// What it needs where, for showing: `up to 1.2 GB on C: (356 GB free)`.
    pub fn summary(&self) -> String {
        self.drives
            .iter()
            .map(|drive| match drive.free_bytes {
                Some(free) => format!(
                    "up to {} on {} ({} free)",
                    size_text(drive.needed_bytes),
                    drive.drive,
                    size_text(free)
                ),
                None => format!(
                    "up to {} on {} (how much is free could not be told)",
                    size_text(drive.needed_bytes),
                    drive.drive
                ),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// A size in the units a person would say it in.
pub fn size_text(bytes: u64) -> String {
    const UNITS: [(u64, &str); 3] = [(GIB, "GiB"), (MIB, "MiB"), (KIB, "KiB")];
    for (size, unit) in UNITS {
        if bytes >= size {
            return format!("{:.1} {unit}", bytes as f64 / size as f64);
        }
    }
    format!("{bytes} bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE_GIB: u64 = GIB;

    fn build(payload_bytes: u64) -> BuildInputs {
        BuildInputs {
            payload_bytes,
            payload_files: 10,
            largest_file_bytes: payload_bytes / 2,
            held_bytes: 0,
            prerequisites: 0,
            stub_bytes: 14 * MIB,
            connected_drive: false,
        }
    }

    #[test]
    fn a_small_build_needs_about_its_package_with_nothing_copied_first() {
        let needs = build_needs(&build(ONE_GIB));
        assert!(needs.surely_one_file);
        assert_eq!(needs.output_bytes, needs.installer_bytes);
        assert!(needs.installer_bytes >= ONE_GIB + 14 * MIB);
        assert!(needs.installer_bytes < ONE_GIB + 100 * MIB, "{needs:?}");
        assert_eq!(needs.staging_bytes, metadata_bytes(10));
    }

    #[test]
    fn a_package_that_may_fit_in_one_file_needs_room_for_two_of_it_at_the_end() {
        let needs = build_needs(&build(5 * ONE_GIB));
        assert!(!needs.surely_one_file);
        assert_eq!(needs.output_bytes, MOST_INSTALLER_BYTES * 2);
    }

    #[test]
    fn a_package_much_past_the_limit_only_needs_itself() {
        let needs = build_needs(&build(20 * ONE_GIB));
        assert!(!needs.surely_one_file);
        assert_eq!(needs.output_bytes, needs.installer_bytes);
    }

    #[test]
    fn what_goes_in_the_protected_folder_is_counted_there() {
        let mut inputs = build(ONE_GIB);
        inputs.held_bytes = 5 * MIB;
        inputs.prerequisites = 2;
        let needs = build_needs(&inputs);
        assert_eq!(
            needs.staging_bytes,
            metadata_bytes(10) + 5 * MIB + 2 * PREREQUISITE_ALLOWANCE_BYTES
        );
        assert!(needs.installer_bytes > ONE_GIB + 5 * MIB + 2 * PREREQUISITE_ALLOWANCE_BYTES);

        inputs.connected_drive = true;
        assert_eq!(
            build_needs(&inputs).staging_bytes,
            metadata_bytes(10)
                + 5 * MIB
                + 2 * PREREQUISITE_ALLOWANCE_BYTES
                + inputs.largest_file_bytes,
            "a file only a backup-mode copy can read takes a copy of it for as long as it is read"
        );
    }

    #[test]
    fn arithmetic_cannot_overflow_into_seeming_to_fit() {
        let mut inputs = build(u64::MAX);
        inputs.held_bytes = u64::MAX;
        let needs = build_needs(&inputs);
        assert_eq!(needs.installer_bytes, u64::MAX);
        assert!(!needs.surely_one_file);
    }

    fn stats_of(free: u64) -> impl Fn(&Path) -> Option<VolumeStats> {
        move |path| match drive_letter(path) {
            Some('C') => Some(VolumeStats {
                free_bytes: free,
                total_bytes: 100 * ONE_GIB,
            }),
            Some('D') => Some(VolumeStats {
                free_bytes: 2 * ONE_GIB,
                total_bytes: 50 * ONE_GIB,
            }),
            _ => None,
        }
    }

    #[test]
    fn what_goes_on_the_same_drive_is_added_up() {
        let needs = [
            PlaceNeed {
                place: PathBuf::from(r"C:\staging"),
                bytes: 3 * ONE_GIB,
                what: "unpacking the package",
            },
            PlaceNeed {
                place: PathBuf::from(r"c:\Program Files\App\a.exe"),
                bytes: 3 * ONE_GIB,
                what: "the app's files",
            },
        ];
        let check = RoomCheck::of(&needs, &stats_of(5 * ONE_GIB));
        assert_eq!(check.drives.len(), 1);
        assert_eq!(check.drives[0].needed_bytes, 6 * ONE_GIB);
        assert_eq!(check.shortfalls().len(), 1);
        assert!(
            check.shortfalls()[0].contains("C:"),
            "{:?}",
            check.shortfalls()
        );
        assert!(
            check.shortfalls()[0].contains("6.0 GiB"),
            "{:?}",
            check.shortfalls()
        );
        assert!(
            check.shortfalls()[0].contains("5.0 GiB"),
            "{:?}",
            check.shortfalls()
        );
        assert!(check.could_not_tell().is_empty());

        assert!(
            RoomCheck::of(&needs, &stats_of(6 * ONE_GIB))
                .shortfalls()
                .is_empty()
        );
    }

    #[test]
    fn each_drive_is_checked_by_itself() {
        let needs = [
            PlaceNeed {
                place: PathBuf::from(r"C:\staging"),
                bytes: ONE_GIB,
                what: "unpacking the package",
            },
            PlaceNeed {
                place: PathBuf::from(r"D:\App\a.exe"),
                bytes: 3 * ONE_GIB,
                what: "the app's files",
            },
        ];
        let check = RoomCheck::of(&needs, &stats_of(50 * ONE_GIB));
        assert_eq!(check.drives.len(), 2);
        let shortfalls = check.shortfalls();
        assert_eq!(shortfalls.len(), 1, "{shortfalls:?}");
        assert!(shortfalls[0].starts_with("D:"), "{shortfalls:?}");
        assert!(check.summary().contains("on C:"), "{}", check.summary());
        assert!(check.summary().contains("on D:"), "{}", check.summary());
    }

    #[test]
    fn a_folder_with_a_volume_mounted_on_it_is_not_the_drive_it_is_on() {
        let stats = |path: &Path| {
            Some(if path.starts_with(r"C:\Mounted") {
                VolumeStats {
                    free_bytes: ONE_GIB,
                    total_bytes: 20 * ONE_GIB,
                }
            } else {
                VolumeStats {
                    free_bytes: 100 * ONE_GIB,
                    total_bytes: 200 * ONE_GIB,
                }
            })
        };
        let needs = [
            PlaceNeed {
                place: PathBuf::from(r"C:\staging"),
                bytes: 2 * ONE_GIB,
                what: "unpacking the package",
            },
            PlaceNeed {
                place: PathBuf::from(r"C:\Mounted\App\a.exe"),
                bytes: 2 * ONE_GIB,
                what: "the app's files",
            },
        ];
        let check = RoomCheck::of(&needs, &stats);
        assert_eq!(check.drives.len(), 2, "{check:?}");
        assert_eq!(check.shortfalls().len(), 1);
    }

    #[test]
    fn a_drive_that_will_not_say_neither_stops_it_nor_passes_silently() {
        let needs = [PlaceNeed {
            place: PathBuf::from(r"E:\App\a.exe"),
            bytes: ONE_GIB,
            what: "the app's files",
        }];
        let check = RoomCheck::of(&needs, &stats_of(ONE_GIB));
        assert!(check.shortfalls().is_empty());
        let unknown = check.could_not_tell();
        assert_eq!(unknown.len(), 1);
        assert!(unknown[0].starts_with("E:"), "{unknown:?}");
        assert!(
            check.summary().contains("could not be told"),
            "{}",
            check.summary()
        );
    }

    #[test]
    fn what_an_install_replaces_only_takes_room_where_it_has_to_be_copied() {
        let staging = Path::new(r"D:\staging");
        let state = Path::new(r"C:\ProgramData\WinAppMigrator");
        let files = [
            PlannedFile {
                destination: PathBuf::from(r"D:\App\replaced.exe"),
                new_bytes: 10,
                replaced_bytes: 7,
            },
            PlannedFile {
                destination: PathBuf::from(r"C:\Users\me\AppData\a.ini"),
                new_bytes: 1,
                replaced_bytes: 100,
            },
            PlannedFile {
                destination: PathBuf::from(r"D:\App\new.exe"),
                new_bytes: 5,
                replaced_bytes: 0,
            },
        ];
        let on_records_volume = |path: &Path| drive_letter(path) == Some('C');
        let needs = install_needs(staging, 16, &files, state, 1000, &on_records_volume);
        let on_c: u64 = needs
            .iter()
            .filter(|need| drive_letter(&need.place) == Some('C'))
            .map(|need| need.bytes)
            .sum();
        let on_d: u64 = needs
            .iter()
            .filter(|need| drive_letter(&need.place) == Some('D'))
            .map(|need| need.bytes)
            .sum();
        // The record, the stub's copy, the data file, and the 7 bytes copied from D:. The file on C:
        // is moved aside, which takes nothing more.
        assert_eq!(on_c, INSTALL_RECORD_BYTES + 1000 + 1 + 7);
        assert_eq!(on_d, 16 + 10 + 5);
    }

    #[test]
    fn a_full_mounted_volume_of_the_same_size_as_its_drive_is_not_passed_over() {
        let stats = |path: &Path| {
            Some(VolumeStats {
                free_bytes: if path.starts_with(r"C:\Mounted") {
                    ONE_GIB
                } else {
                    100 * ONE_GIB
                },
                total_bytes: 200 * ONE_GIB,
            })
        };
        let needs = [
            PlaceNeed {
                place: PathBuf::from(r"C:\staging"),
                bytes: 2 * ONE_GIB,
                what: "unpacking the package",
            },
            PlaceNeed {
                place: PathBuf::from(r"C:\Mounted\App\a.exe"),
                bytes: 2 * ONE_GIB,
                what: "the app's files",
            },
        ];
        let check = RoomCheck::of(&needs, &stats);
        assert_eq!(check.drives.len(), 1);
        assert_eq!(check.drives[0].free_bytes, Some(ONE_GIB));
        assert_eq!(check.shortfalls().len(), 1);
    }

    #[test]
    fn a_path_is_on_its_own_volume_and_not_on_another_letter() {
        let here = std::env::temp_dir();
        assert!(on_same_volume(&here, &here.join("not there")));
        assert!(!on_same_volume(Path::new(r"Q:\a"), &here));
        assert!(!on_same_volume(Path::new("relative"), &here));
    }

    #[test]
    fn a_preview_only_needs_room_to_unpack() {
        let needs = preview_needs(Path::new(r"C:\staging"), 42);
        assert_eq!(needs.len(), 1);
        assert_eq!(needs[0].bytes, 42);
    }

    #[test]
    fn this_drive_answers() {
        let stats = volume_stats_at(&std::env::temp_dir()).unwrap_or(VolumeStats {
            free_bytes: 0,
            total_bytes: 0,
        });
        assert!(stats.total_bytes > 0);
        let missing = std::env::temp_dir().join("not there").join("nor this");
        assert_eq!(
            volume_stats_at(&missing).map(|stats| stats.total_bytes),
            Some(stats.total_bytes),
            "by the nearest folder that exists"
        );
    }

    #[test]
    fn sizes_read_as_a_person_would_say_them() {
        assert_eq!(size_text(512), "512 bytes");
        assert_eq!(size_text(2 * KIB), "2.0 KiB");
        assert_eq!(size_text(MIB + MIB / 2), "1.5 MiB");
        assert_eq!(size_text(3 * GIB), "3.0 GiB");
    }
}
