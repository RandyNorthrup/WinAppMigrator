//! Which runtimes an app needs, read from the programs it has.
//!
//! The main program alone is not enough to go by: one app can have a managed launcher and a native
//! engine, or a 32-bit program beside a 64-bit one, and each needs its own runtime. So all of its
//! programs are looked at, other than what exists to install or update it, whose dependencies say
//! nothing about the app. What a program needs is read from its import table, what its own folder
//! holds decides whether that ships with it, and for .NET its `runtimeconfig.json` and its own
//! metadata say which .NET.
//!
//! All of it is a name to look up in the specs compiled into [`crate::prerequisites`], which decide
//! what runs. Nothing here is read as a command.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::dll_resolver::{VisualCppRuntime, is_legacy_directx_dll, visual_cpp_dll};
use crate::models::{AppArchitecture, DotNetFlavor, FileClassification, FileEntry};
use crate::path_utils::{is_auxiliary_executable, path_has_extension, strip_prefix_ignore_case};
use crate::pe_scan::{ProgramNeeds, read_pe_architecture, read_program_needs};
use crate::system_context::SystemContext;

const EXE_EXTENSION: &str = "exe";
const DLL_EXTENSION: &str = "dll";
const RUNTIME_CONFIG_SUFFIX: &str = ".runtimeconfig.json";
/// How many programs are looked at, shallowest first. A developer tool can have thousands, and
/// the ones that matter are near the top.
const MAX_PROGRAMS: usize = 64;
/// How many files are read in all, with the DLLs those programs import from their own folders.
const MAX_FILES_READ: usize = 512;
/// A program larger than this is left out, unless it is the main one. It is read whole.
const MAX_PROGRAM_BYTES: u64 = 512 * 1024 * 1024;
const MAX_BYTES_READ: u64 = 2 * 1024 * 1024 * 1024;
/// No `runtimeconfig.json` is near this.
const MAX_RUNTIME_CONFIG_BYTES: u64 = 1024 * 1024;
/// Folders that hold what installs something, not what the app runs.
const INSTALLER_FOLDERS: &[&str] = &[
    "_commonredist",
    "dotnetfx",
    "installer",
    "installers",
    "prerequisites",
    "prereqs",
    "redist",
    "redistributable",
    "setup",
    "vcredist",
];
/// What a program's name says about it being a way of installing something, beyond what
/// `is_auxiliary_executable` knows: what Python's and Squirrel's own installers leave in a folder.
const INSTALLER_PROGRAM_NAMES: &[&str] = &["squirrel", "wininst"];
/// The files a self-contained .NET app has beside its program.
const SELF_CONTAINED_DOTNET_FILES: &[&str] = &["coreclr.dll", "hostfxr.dll", "hostpolicy.dll"];

/// A .NET a program runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DotNetNeed {
    /// CLR 1.x or 2.0, which on Windows 10 and 11 is the `NetFx3` feature.
    Framework35,
    Framework4x,
    /// Modern .NET of this version, shared on the PC.
    Modern {
        flavor: DotNetFlavor,
        major: u32,
    },
    /// Modern .NET whose version it does not say.
    ModernOfNoKnownVersion,
    /// Modern .NET it ships itself.
    SelfContained,
}

/// What an app's programs were found to need, each with a program that needs it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct AppRuntimes {
    pub visual_cpp: BTreeMap<(VisualCppRuntime, ArchitectureKey), String>,
    pub dotnet: BTreeMap<(DotNetNeed, ArchitectureKey), String>,
    /// Debug runtimes, by DLL, which no one can install for it.
    pub debug_dlls: BTreeSet<String>,
    /// Programs that could not all be looked at, which is said rather than taken to mean anything.
    pub programs_left_out: usize,
    /// .NET programs packed into one file, which say nothing of which .NET they need.
    pub single_file_programs: BTreeSet<String>,
    /// What says a program needs the DirectX End-User Runtime, if one does: a DLL of it that the
    /// program imports and that does not ship beside it.
    pub legacy_directx: Option<String>,
}

/// [`AppArchitecture`], in an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ArchitectureKey {
    X86,
    X64,
    Arm64,
    Unknown,
}

impl ArchitectureKey {
    pub(crate) fn of(architecture: &AppArchitecture) -> Self {
        match architecture {
            AppArchitecture::X86 => Self::X86,
            AppArchitecture::X64 => Self::X64,
            AppArchitecture::Arm64 => Self::Arm64,
            AppArchitecture::Unknown => Self::Unknown,
        }
    }

    pub(crate) fn architecture(self) -> AppArchitecture {
        match self {
            Self::X86 => AppArchitecture::X86,
            Self::X64 => AppArchitecture::X64,
            Self::Arm64 => AppArchitecture::Arm64,
            Self::Unknown => AppArchitecture::Unknown,
        }
    }
}

/// A file's name in lowercase, for comparing.
fn lower_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default()
}

/// A path in lowercase, for looking up.
fn lower_path(path: &Path) -> String {
    path.to_string_lossy().to_ascii_lowercase()
}

/// True for a program that installs or supports the app, whose dependencies are its own business.
/// `is_auxiliary_executable` goes by its name, and this adds where it is.
pub(crate) fn is_helper_program(path: &Path, install_location: Option<&Path>) -> bool {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if is_auxiliary_executable(path)
        || INSTALLER_PROGRAM_NAMES
            .iter()
            .any(|name| stem.starts_with(name))
    {
        return true;
    }
    let below = install_location
        .and_then(|location| strip_prefix_ignore_case(path, location))
        .unwrap_or_else(|| path.to_path_buf());
    below
        .parent()
        .into_iter()
        .flat_map(Path::components)
        .any(|component| {
            let name = component.as_os_str().to_string_lossy().to_ascii_lowercase();
            INSTALLER_FOLDERS.contains(&name.as_str())
        })
}

/// The app's programs to look at, the main one first and then shallowest first, with how many
/// more there were.
fn programs_to_look_at(
    files: &[FileEntry],
    main_program: &Path,
    install_location: Option<&Path>,
) -> (Vec<PathBuf>, usize) {
    let mut found: Vec<&Path> = files
        .iter()
        .filter(|file| file.should_copy && file.classification == FileClassification::AppOwned)
        .map(|file| file.path.as_path())
        .filter(|path| {
            path_has_extension(path, EXE_EXTENSION)
                && *path != main_program
                && !is_helper_program(path, install_location)
        })
        .collect();
    found.sort_by_key(|path| (path.components().count(), lower_path(path)));
    let left_out = found.len().saturating_sub(MAX_PROGRAMS);
    let mut programs = vec![main_program.to_path_buf()];
    programs.extend(found.into_iter().take(MAX_PROGRAMS).map(Path::to_path_buf));
    (programs, left_out)
}

/// A file an app's own folder has, by lowercase path.
fn app_files(files: &[FileEntry]) -> HashSet<String> {
    files
        .iter()
        .filter(|file| file.should_copy)
        .map(|file| lower_path(&file.path))
        .collect()
}

/// What a managed image's metadata says it needs.
pub(crate) fn dotnet_need_of(needs: &ProgramNeeds) -> Option<DotNetNeed> {
    if !needs.is_managed {
        return None;
    }
    let version = needs.clr_version.as_deref().unwrap_or_default();
    if version.starts_with("v1.") || version.starts_with("v2.") {
        return Some(DotNetNeed::Framework35);
    }
    let managed = &needs.managed;
    let target = managed.target_framework.as_deref().unwrap_or_default();
    if target.starts_with(".NETFramework") {
        return Some(DotNetNeed::Framework4x);
    }
    if let Some(version) = target.strip_prefix(".NETCoreApp,Version=v") {
        let flavor = if managed.refers_to_desktop {
            DotNetFlavor::Desktop
        } else if managed.refers_to_asp_net {
            DotNetFlavor::AspNetCore
        } else {
            DotNetFlavor::Core
        };
        return Some(
            major_of(version).map_or(DotNetNeed::ModernOfNoKnownVersion, |major| {
                DotNetNeed::Modern { flavor, major }
            }),
        );
    }
    // Before attributes said so, for one built with an old compiler or one with them stripped.
    if managed.refers_to_mscorlib {
        return Some(DotNetNeed::Framework4x);
    }
    Some(DotNetNeed::ModernOfNoKnownVersion)
}

fn major_of(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

/// A `.runtimeconfig.json`, as far as it matters here.
#[derive(Debug, Deserialize)]
struct RuntimeConfigFile {
    #[serde(rename = "runtimeOptions")]
    runtime_options: RuntimeOptions,
}

#[derive(Debug, Default, Deserialize)]
struct RuntimeOptions {
    framework: Option<RuntimeFramework>,
    #[serde(default)]
    frameworks: Vec<RuntimeFramework>,
    /// Only a self-contained app says this.
    #[serde(rename = "includedFrameworks", default)]
    included_frameworks: Vec<RuntimeFramework>,
}

#[derive(Debug, Deserialize)]
struct RuntimeFramework {
    name: String,
    #[serde(default)]
    version: String,
}

/// What a `.runtimeconfig.json` says an app runs on. Nothing it says goes anywhere but this.
pub(crate) fn dotnet_needs_of_runtime_config(text: &str) -> Vec<DotNetNeed> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let Ok(config) = serde_json::from_str::<RuntimeConfigFile>(text) else {
        return Vec::new();
    };
    let options = config.runtime_options;
    if !options.included_frameworks.is_empty() {
        return vec![DotNetNeed::SelfContained];
    }
    options
        .framework
        .iter()
        .chain(&options.frameworks)
        .map(|framework| {
            let flavor = match framework.name.to_ascii_lowercase().as_str() {
                "microsoft.netcore.app" => DotNetFlavor::Core,
                "microsoft.windowsdesktop.app" => DotNetFlavor::Desktop,
                "microsoft.aspnetcore.app" => DotNetFlavor::AspNetCore,
                _ => return DotNetNeed::ModernOfNoKnownVersion,
            };
            major_of(&framework.version).map_or(DotNetNeed::ModernOfNoKnownVersion, |major| {
                DotNetNeed::Modern { flavor, major }
            })
        })
        .collect()
}

/// Look at all of an app's programs. `app_architecture` is its main program's, for what has none
/// of its own, such as a managed library.
pub(crate) fn inspect_app_runtimes(
    files: &[FileEntry],
    main_program: &Path,
    install_location: Option<&Path>,
    app_architecture: &AppArchitecture,
    system: &SystemContext,
) -> AppRuntimes {
    let mut runtimes = AppRuntimes::default();
    let (programs, left_out) = programs_to_look_at(files, main_program, install_location);
    runtimes.programs_left_out = left_out;
    let own_files = app_files(files);
    let mut files_read = 0_usize;
    let mut bytes_read = 0_u64;
    let mut seen: HashSet<String> = HashSet::new();

    for program in &programs {
        let program_name = program
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        let mut pending = VecDeque::from([program.clone()]);
        let mut architecture = None;
        while let Some(path) = pending.pop_front() {
            if !seen.insert(lower_path(&path)) {
                continue;
            }
            let is_main = path == *main_program;
            let Some(size) = may_read_it(&path, is_main, bytes_read, files_read, system) else {
                runtimes.programs_left_out += usize::from(path == *program);
                continue;
            };
            files_read += 1;
            bytes_read += size;
            let Ok(needs) = read_program_needs(&path) else {
                runtimes.programs_left_out += usize::from(path == *program);
                continue;
            };
            let build = ArchitectureKey::of(&needs.architecture);
            // Such a program in an app of another build can only be there for some other PC, as
            // packaging tools keep a launcher of each kind. A 32-bit one runs on 64-bit Windows.
            if build == ArchitectureKey::Arm64 && ArchitectureKey::of(app_architecture) != build {
                continue;
            }
            let is_the_program = path == *program;
            if is_the_program {
                if needs.is_a_single_file_bundle {
                    runtimes.single_file_programs.insert(program_name.clone());
                }
                architecture = Some(build);
                if let Some(dotnet) = dotnet_need_of(&needs) {
                    runtimes
                        .dotnet
                        .entry((dotnet, build))
                        .or_insert_with(|| program_name.clone());
                }
            }
            let folder = path.parent().unwrap_or(Path::new(""));
            for import in &needs.imports {
                // A name in an import table is a file's name and no more.
                if import.name.contains(['\\', '/', ':']) {
                    continue;
                }
                let beside = folder.join(&import.name);
                let ships_with_it = own_files.contains(&lower_path(&beside));
                match visual_cpp_dll(&import.name) {
                    Some(dll) if !ships_with_it && dll.debug => {
                        runtimes.debug_dlls.insert(import.name.to_ascii_lowercase());
                    }
                    Some(dll) if !ships_with_it => {
                        runtimes
                            .visual_cpp
                            .entry((dll.runtime, architecture.unwrap_or(build)))
                            .or_insert_with(|| format!("{program_name} imports {}", import.name));
                    }
                    Some(_) => {}
                    None if ships_with_it => pending.push_back(beside),
                    None if is_legacy_directx_dll(&import.name) => {
                        runtimes.legacy_directx.get_or_insert_with(|| {
                            format!("{program_name} imports {}", import.name)
                        });
                    }
                    None => {}
                }
            }
        }
    }
    let architecture = ArchitectureKey::of(app_architecture);
    for file in files.iter().filter(|file| file.should_copy) {
        note_runtime_config(
            file,
            install_location,
            architecture,
            &own_files,
            system,
            &mut runtimes,
        );
    }
    runtimes
}

/// How big this is, if it may be read, staying within what a scan reads in all.
fn may_read_it(
    path: &Path,
    is_main: bool,
    bytes_read: u64,
    files_read: usize,
    system: &SystemContext,
) -> Option<u64> {
    if !system.may_read(path) {
        return None;
    }
    let size = crate::source_fs::metadata(path).ok()?.len();
    let within = files_read < MAX_FILES_READ
        && size <= MAX_PROGRAM_BYTES
        && bytes_read.saturating_add(size) <= MAX_BYTES_READ;
    (is_main || within).then_some(size)
}

fn note_runtime_config(
    file: &FileEntry,
    install_location: Option<&Path>,
    architecture: ArchitectureKey,
    own_files: &HashSet<String>,
    system: &SystemContext,
    runtimes: &mut AppRuntimes,
) {
    let name = lower_name(&file.path);
    let Some(stem) = name.strip_suffix(RUNTIME_CONFIG_SUFFIX) else {
        return;
    };
    let program = file.path.with_file_name(format!("{stem}.{EXE_EXTENSION}"));
    if is_helper_program(&program, install_location)
        || !file
            .size_bytes
            .is_some_and(|size| size <= MAX_RUNTIME_CONFIG_BYTES)
    {
        return;
    }
    if !system.may_read(&file.path) {
        return;
    }
    let Ok(text) = crate::source_fs::read_to_string(&file.path) else {
        return;
    };
    let folder = file.path.parent().unwrap_or(Path::new(""));
    let ships_its_own = SELF_CONTAINED_DOTNET_FILES
        .iter()
        .any(|name| own_files.contains(&lower_path(&folder.join(name))));
    // A library loaded by something has one of these too, and no program of its name.
    let has_something_to_run = own_files.contains(&lower_path(&program))
        || own_files.contains(&lower_path(
            &file.path.with_file_name(format!("{stem}.{DLL_EXTENSION}")),
        ));
    if !has_something_to_run {
        return;
    }
    // What it says is for the program of its name, whichever build the app's main program is.
    let architecture = system
        .may_read(&program)
        .then(|| read_pe_architecture(&program).ok())
        .flatten()
        .map_or(architecture, |build| ArchitectureKey::of(&build));
    for need in dotnet_needs_of_runtime_config(&text) {
        let need = if ships_its_own {
            DotNetNeed::SelfContained
        } else {
            need
        };
        runtimes
            .dotnet
            .entry((need, architecture))
            .or_insert_with(|| name.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pe_scan::ManagedImageFacts;

    #[test]
    fn a_runtime_config_with_a_byte_order_mark_says_what_it_says() {
        let config = r#"{"runtimeOptions":{"framework":{"name":"Microsoft.NETCore.App","version":"8.0.0"}}}"#;
        let plain = dotnet_needs_of_runtime_config(config);
        assert_eq!(plain.len(), 1);
        assert_eq!(
            dotnet_needs_of_runtime_config(&format!("\u{feff}{config}")),
            plain
        );
    }

    fn managed(clr: &str, facts: ManagedImageFacts) -> ProgramNeeds {
        ProgramNeeds {
            architecture: AppArchitecture::X64,
            imports: Vec::new(),
            clr_version: Some(clr.to_owned()),
            is_managed: true,
            managed: facts,
            is_a_single_file_bundle: false,
        }
    }

    #[test]
    fn a_managed_image_is_told_apart_by_what_it_says_it_was_built_for() {
        let facts = |target: &str| ManagedImageFacts {
            target_framework: Some(target.to_owned()),
            ..ManagedImageFacts::default()
        };
        assert_eq!(
            dotnet_need_of(&managed("v2.0.50727", ManagedImageFacts::default())),
            Some(DotNetNeed::Framework35)
        );
        assert_eq!(
            dotnet_need_of(&managed("v4.0.30319", facts(".NETFramework,Version=v4.8"))),
            Some(DotNetNeed::Framework4x)
        );
        assert_eq!(
            dotnet_need_of(&managed("v4.0.30319", facts(".NETCoreApp,Version=v8.0"))),
            Some(DotNetNeed::Modern {
                flavor: DotNetFlavor::Core,
                major: 8
            }),
            "this says v4.0.30319 too, and it is not .NET Framework"
        );
        assert_eq!(
            dotnet_need_of(&managed(
                "v4.0.30319",
                ManagedImageFacts {
                    refers_to_desktop: true,
                    ..facts(".NETCoreApp,Version=v6.0")
                }
            )),
            Some(DotNetNeed::Modern {
                flavor: DotNetFlavor::Desktop,
                major: 6
            })
        );
        assert_eq!(
            dotnet_need_of(&managed(
                "v4.0.30319",
                ManagedImageFacts {
                    refers_to_mscorlib: true,
                    ..ManagedImageFacts::default()
                }
            )),
            Some(DotNetNeed::Framework4x),
            "with no attribute, only .NET Framework has mscorlib"
        );
        assert_eq!(
            dotnet_need_of(&managed("v4.0.30319", ManagedImageFacts::default())),
            Some(DotNetNeed::ModernOfNoKnownVersion)
        );
        let native = ProgramNeeds {
            is_managed: false,
            ..managed("", ManagedImageFacts::default())
        };
        assert_eq!(dotnet_need_of(&native), None);
    }

    #[test]
    fn a_runtime_config_says_which_dotnet_and_whether_it_ships_with_the_app() {
        assert_eq!(
            dotnet_needs_of_runtime_config(
                r#"{"runtimeOptions":{"framework":{"name":"Microsoft.WindowsDesktop.App","version":"8.0.11"}}}"#
            ),
            [DotNetNeed::Modern {
                flavor: DotNetFlavor::Desktop,
                major: 8
            }]
        );
        assert_eq!(
            dotnet_needs_of_runtime_config(
                r#"{"runtimeOptions":{"frameworks":[
                    {"name":"Microsoft.NETCore.App","version":"9.0.0"},
                    {"name":"Microsoft.AspNetCore.App","version":"9.0.0"}]}}"#
            ),
            [
                DotNetNeed::Modern {
                    flavor: DotNetFlavor::Core,
                    major: 9
                },
                DotNetNeed::Modern {
                    flavor: DotNetFlavor::AspNetCore,
                    major: 9
                }
            ]
        );
        assert_eq!(
            dotnet_needs_of_runtime_config(
                r#"{"runtimeOptions":{"includedFrameworks":[{"name":"Microsoft.NETCore.App","version":"8.0.1"}]}}"#
            ),
            [DotNetNeed::SelfContained]
        );
        assert_eq!(
            dotnet_needs_of_runtime_config(
                r#"{"runtimeOptions":{"framework":{"name":"Vendor.Other.App","version":"1.0"}}}"#
            ),
            [DotNetNeed::ModernOfNoKnownVersion]
        );
        assert!(dotnet_needs_of_runtime_config("not json").is_empty());
    }

    #[test]
    fn what_installs_or_supports_an_app_says_nothing_about_what_it_needs() {
        let install = Some(Path::new(r"C:\Program Files\Acme"));
        for helper in [
            r"C:\Program Files\Acme\unins000.exe",
            r"C:\Program Files\Acme\AcmeUpdate.exe",
            r"C:\Program Files\Acme\crashpad_handler.exe",
            r"C:\Program Files\Acme\Lib\distutils\command\wininst-7.1.exe",
            r"C:\Program Files\Acme\squirrel.exe",
            r"C:\Program Files\Acme\redist\anything.exe",
            r"C:\Program Files\Acme\_CommonRedist\vcredist\2013\thing.exe",
            r"C:\Program Files\Acme\Installers\thing.exe",
        ] {
            assert!(is_helper_program(Path::new(helper), install), "{helper}");
        }
        for program in [
            r"C:\Program Files\Acme\acme.exe",
            r"C:\Program Files\Acme\bin\engine.exe",
            // Only below where the app is. Its own folder can be called anything.
            r"C:\Installer\Acme\acme.exe",
        ] {
            let install = if program.starts_with(r"C:\Installer") {
                Some(Path::new(r"C:\Installer\Acme"))
            } else {
                install
            };
            assert!(!is_helper_program(Path::new(program), install), "{program}");
        }
    }

    fn file(path: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            original_path: PathBuf::from(path),
            package_relative_path: None,
            size_bytes: Some(1),
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: String::new(),
            user_folder: None,
        }
    }

    #[test]
    fn the_main_program_comes_first_and_then_the_shallowest() {
        let main = PathBuf::from(r"C:\A\deep\down\main.exe");
        let files = [
            file(r"C:\A\deep\down\main.exe"),
            file(r"C:\A\deep\other.exe"),
            file(r"C:\A\top.exe"),
            file(r"C:\A\unins000.exe"),
            file(r"C:\A\notes.txt"),
        ];
        let (programs, left_out) = programs_to_look_at(&files, &main, Some(Path::new(r"C:\A")));
        assert_eq!(
            programs,
            [
                main,
                PathBuf::from(r"C:\A\top.exe"),
                PathBuf::from(r"C:\A\deep\other.exe")
            ]
        );
        assert_eq!(left_out, 0);
    }

    #[test]
    fn programs_past_the_limit_are_counted_and_not_read() {
        let main = PathBuf::from(r"C:\A\main.exe");
        let files: Vec<FileEntry> = (0..MAX_PROGRAMS + 5)
            .map(|number| file(&format!(r"C:\A\tool{number}.exe")))
            .collect();
        let (programs, left_out) = programs_to_look_at(&files, &main, None);
        assert_eq!(programs.len(), MAX_PROGRAMS + 1);
        assert_eq!(left_out, 5);
    }
}
