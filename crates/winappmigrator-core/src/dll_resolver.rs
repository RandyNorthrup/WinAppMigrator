use std::collections::{BTreeSet, VecDeque};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::{Result, WinAppMigratorError};
use crate::hashing::sha256_file;
use crate::models::{
    AppArchitecture, DependencyEntry, DependencySourceCategory, FileClassification, FileEntry,
    Warning, WarningSeverity,
};
use crate::path_utils::path_has_extension;
use crate::pe_scan::{ImportedDll, read_pe_architecture, scan_pe};
use crate::system_context::SystemContext;

const DLL_EXTENSION: &str = "dll";
const WINDOWS_ENV_VAR: &str = "SystemRoot";
const LEGACY_WINDOWS_ENV_VAR: &str = "windir";
const SYSTEM32_DIR: &str = "System32";
const SYSWOW64_DIR: &str = "SysWOW64";

const API_SET_PREFIXES: &[&str] = &["api-ms-win-", "ext-ms-"];
/// The start of each Visual C++ runtime DLL's name, longest first where one starts another. The
/// digits after it say which Visual Studio it came with, so `msvcrt.dll`, which is Windows' own,
/// is not one of them, and neither is `mfc42.dll`.
const VC_RUNTIME_STEMS: &[&str] = &[
    "vcruntime",
    "vccorlib",
    "concrt",
    "msvcp",
    "msvcr",
    "msvcm",
    "vcomp",
    "vcamp",
    "mfcm",
    "mfc",
    "atl",
];
const KNOWN_DLLS: &[&str] = &[
    "advapi32.dll",
    "bcrypt.dll",
    "comctl32.dll",
    "comdlg32.dll",
    "crypt32.dll",
    "dwmapi.dll",
    "gdi32.dll",
    "imm32.dll",
    "kernel32.dll",
    "kernelbase.dll",
    "msvcrt.dll",
    "ntdll.dll",
    "ole32.dll",
    "oleaut32.dll",
    "rpcrt4.dll",
    "sechost.dll",
    "shell32.dll",
    "shlwapi.dll",
    "ucrtbase.dll",
    "user32.dll",
    "userenv.dll",
    "version.dll",
    "winhttp.dll",
    "wininet.dll",
    "winmm.dll",
    "ws2_32.dll",
];

#[derive(Debug, Clone)]
pub struct DllResolutionContext {
    pub executable_path: PathBuf,
    pub app_architecture: AppArchitecture,
    pub current_directory: Option<PathBuf>,
    pub recursive: bool,
    pub include_hashes: bool,
}

#[derive(Debug, Clone)]
struct SearchDirectory {
    path: PathBuf,
    category: DependencySourceCategory,
}

#[derive(Debug, Clone)]
struct ResolvedImport {
    resolved_path: Option<PathBuf>,
    architecture: AppArchitecture,
    hash_sha256: Option<String>,
    source_category: DependencySourceCategory,
    should_copy: bool,
    system_provided: bool,
    runtime_prerequisite: bool,
    missing: bool,
}

pub fn resolve_imports(
    imports: &[ImportedDll],
    context: &DllResolutionContext,
    system: &SystemContext,
) -> (Vec<DependencyEntry>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let search_dirs = build_search_directories(context, system, &mut warnings);
    let mut dependencies = Vec::new();
    let mut pending: VecDeque<ImportedDll> = imports.iter().cloned().collect();
    let mut seen_names = BTreeSet::new();
    let mut scanned_paths = BTreeSet::new();

    while let Some(import) = pending.pop_front() {
        if !seen_names.insert(import.name.to_ascii_lowercase()) {
            continue;
        }

        let dependency = resolve_one_import(
            &import,
            &search_dirs,
            context.include_hashes,
            system,
            &mut warnings,
        );

        if context.recursive
            && dependency.should_copy
            && let Some(path) = dependency.resolved_path.as_ref()
        {
            let key = path.to_string_lossy().to_ascii_lowercase();
            if scanned_paths.insert(key) {
                match scan_pe(path) {
                    Ok(scan) => pending.extend(scan.direct_imports),
                    Err(error) => warnings.push(Warning::new(
                        "recursive_dependency_scan_failed",
                        WarningSeverity::Warning,
                        format!("Could not scan recursive dependency: {error}"),
                        Some(path.display().to_string()),
                    )),
                }
            }
        }

        dependencies.push(dependency);
    }

    dependencies.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });

    (dependencies, warnings)
}

pub fn find_app_local_dlls(
    executable_path: &Path,
    system: &SystemContext,
) -> Result<Vec<FileEntry>> {
    let app_dir = executable_path.parent().ok_or_else(|| {
        WinAppMigratorError::InvalidInput(format!(
            "executable has no parent directory: {}",
            executable_path.display()
        ))
    })?;
    let entries =
        fs::read_dir(app_dir).map_err(|source| WinAppMigratorError::io(app_dir, source))?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| WinAppMigratorError::io(app_dir, source))?;
        let path = entry.path();
        if !path_has_extension(&path, DLL_EXTENSION) || !system.may_read(&path) {
            continue;
        }
        let size_bytes = entry.metadata().ok().map(|metadata| metadata.len());
        files.push(FileEntry {
            path: path.clone(),
            original_path: path,
            package_relative_path: None,
            size_bytes,
            hash_sha256: None,
            classification: FileClassification::AppOwned,
            should_copy: true,
            reason: "DLL located beside selected executable".to_owned(),
            user_folder: None,
        });
    }

    files.sort_by(|left, right| {
        left.path
            .to_string_lossy()
            .to_ascii_lowercase()
            .cmp(&right.path.to_string_lossy().to_ascii_lowercase())
    });
    Ok(files)
}

pub fn is_runtime_prerequisite_dll(name: &str) -> bool {
    visual_cpp_dll(name).is_some()
}

/// Which Visual C++ runtime a DLL is part of, going by its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisualCppRuntime {
    Vs2002,
    Vs2003,
    Vs2005,
    Vs2008,
    Vs2010,
    Vs2012,
    Vs2013,
    /// 2015 to 2022 are one redistributable, and their DLLs are all named for `140`.
    Vs2015Plus,
}

/// A DLL of a Visual C++ runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualCppDll {
    pub runtime: VisualCppRuntime,
    /// A debug build, such as `msvcp140d.dll`, which no redistributable has.
    pub debug: bool,
}

/// What a DLL's name says about which Visual C++ runtime it belongs to, if any.
pub fn visual_cpp_dll(name: &str) -> Option<VisualCppDll> {
    let lower = name.to_ascii_lowercase();
    let stem = lower.strip_suffix(".dll").unwrap_or(&lower);
    let rest = VC_RUNTIME_STEMS
        .iter()
        .find_map(|known| stem.strip_prefix(known))?;
    let digits = rest
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(rest.len());
    let (version, suffix) = rest.split_at(digits);
    let runtime = match version {
        "70" => VisualCppRuntime::Vs2002,
        "71" => VisualCppRuntime::Vs2003,
        "80" => VisualCppRuntime::Vs2005,
        "90" => VisualCppRuntime::Vs2008,
        "100" => VisualCppRuntime::Vs2010,
        "110" => VisualCppRuntime::Vs2012,
        "120" => VisualCppRuntime::Vs2013,
        "140" => VisualCppRuntime::Vs2015Plus,
        _ => return None,
    };
    let debug = matches!(suffix, "d" | "ud") || (suffix.starts_with('_') && suffix.ends_with('d'));
    Some(VisualCppDll { runtime, debug })
}

fn resolve_one_import(
    import: &ImportedDll,
    search_dirs: &[SearchDirectory],
    include_hashes: bool,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> DependencyEntry {
    let lower_name = import.name.to_ascii_lowercase();
    let is_api_set = API_SET_PREFIXES
        .iter()
        .any(|prefix| lower_name.starts_with(prefix));
    let is_known = KNOWN_DLLS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(&lower_name));

    if is_api_set {
        return dependency_entry(
            import,
            ResolvedImport {
                resolved_path: None,
                architecture: AppArchitecture::Unknown,
                hash_sha256: None,
                source_category: DependencySourceCategory::ApiSet,
                should_copy: false,
                system_provided: true,
                runtime_prerequisite: false,
                missing: false,
            },
        );
    }

    // A name in a program's import table is a file's name and no more. A path there comes from
    // whoever made the program, and it could lead anywhere.
    let is_plain_name = !import.name.contains(['\\', '/', ':']);
    if is_known && is_plain_name {
        let resolved = find_in_system_directories(&import.name, search_dirs, system);
        let architecture = resolved
            .as_deref()
            .and_then(|path| read_architecture_with_warning(path, warnings));
        let hash_sha256 = resolved
            .as_deref()
            .and_then(|path| hash_dependency(path, include_hashes, warnings));
        return dependency_entry(
            import,
            ResolvedImport {
                resolved_path: resolved,
                architecture: architecture.unwrap_or(AppArchitecture::Unknown),
                hash_sha256,
                source_category: DependencySourceCategory::KnownDll,
                should_copy: false,
                system_provided: true,
                runtime_prerequisite: is_runtime_prerequisite_dll(&import.name),
                missing: false,
            },
        );
    }

    for search_dir in search_dirs.iter().filter(|_| is_plain_name) {
        let candidate = search_dir.path.join(&import.name);
        if system.is_file(&candidate) {
            let architecture = read_architecture_with_warning(&candidate, warnings)
                .unwrap_or(AppArchitecture::Unknown);
            let system_provided = matches!(
                search_dir.category,
                DependencySourceCategory::System32
                    | DependencySourceCategory::SysWow64
                    | DependencySourceCategory::Windows
            );
            let should_copy = matches!(search_dir.category, DependencySourceCategory::AppLocal);
            return dependency_entry(
                import,
                ResolvedImport {
                    hash_sha256: hash_dependency(&candidate, include_hashes, warnings),
                    resolved_path: Some(candidate),
                    architecture,
                    source_category: search_dir.category.clone(),
                    should_copy,
                    system_provided,
                    runtime_prerequisite: is_runtime_prerequisite_dll(&import.name),
                    missing: false,
                },
            );
        }
    }

    dependency_entry(
        import,
        ResolvedImport {
            resolved_path: None,
            architecture: AppArchitecture::Unknown,
            hash_sha256: None,
            source_category: DependencySourceCategory::Unresolved,
            should_copy: false,
            system_provided: false,
            runtime_prerequisite: is_runtime_prerequisite_dll(&import.name),
            missing: true,
        },
    )
}

fn dependency_entry(import: &ImportedDll, resolved: ResolvedImport) -> DependencyEntry {
    DependencyEntry {
        name: import.name.clone(),
        resolved_path: resolved.resolved_path,
        architecture: resolved.architecture,
        hash_sha256: resolved.hash_sha256,
        version_info: None,
        publisher: None,
        source_category: resolved.source_category,
        should_copy: resolved.should_copy,
        system_provided: resolved.system_provided,
        runtime_prerequisite: resolved.runtime_prerequisite,
        missing: resolved.missing,
        delay_loaded: import.delay_loaded,
    }
}

fn hash_dependency(
    path: &Path,
    include_hashes: bool,
    warnings: &mut Vec<Warning>,
) -> Option<String> {
    if !include_hashes {
        return None;
    }

    match sha256_file(path) {
        Ok(hash) => Some(hash),
        Err(error) => {
            warnings.push(Warning::new(
                "dependency_hash_failed",
                WarningSeverity::Warning,
                format!("Could not hash dependency: {error}"),
                Some(path.display().to_string()),
            ));
            None
        }
    }
}

fn build_search_directories(
    context: &DllResolutionContext,
    system: &SystemContext,
    warnings: &mut Vec<Warning>,
) -> Vec<SearchDirectory> {
    let mut directories = Vec::new();
    let mut seen = BTreeSet::new();

    if let Some(app_dir) = context.executable_path.parent() {
        push_unique(
            &mut directories,
            &mut seen,
            app_dir.to_path_buf(),
            DependencySourceCategory::AppLocal,
            system,
        );
    }

    if let Some(windows_dir) = windows_dir(system) {
        match context.app_architecture {
            AppArchitecture::X86 => {
                push_unique(
                    &mut directories,
                    &mut seen,
                    windows_dir.join(SYSWOW64_DIR),
                    DependencySourceCategory::SysWow64,
                    system,
                );
                push_unique(
                    &mut directories,
                    &mut seen,
                    windows_dir.join(SYSTEM32_DIR),
                    DependencySourceCategory::System32,
                    system,
                );
            }
            AppArchitecture::X64 | AppArchitecture::Arm64 | AppArchitecture::Unknown => {
                push_unique(
                    &mut directories,
                    &mut seen,
                    windows_dir.join(SYSTEM32_DIR),
                    DependencySourceCategory::System32,
                    system,
                );
            }
        }
        push_unique(
            &mut directories,
            &mut seen,
            windows_dir,
            DependencySourceCategory::Windows,
            system,
        );
    } else {
        warnings.push(Warning::new(
            "windows_dir_not_found",
            WarningSeverity::Warning,
            "SystemRoot/windir environment variables are not available; system DLL search is limited.",
            None,
        ));
    }

    // The folder this process is in is on this machine and not the scanned disk, as with `PATH`.
    if let Some(current_directory) = context
        .current_directory
        .as_ref()
        .filter(|_| !system.is_offline())
    {
        push_unique(
            &mut directories,
            &mut seen,
            current_directory.clone(),
            DependencySourceCategory::CurrentDirectory,
            system,
        );
    }

    // The live PATH belongs to this machine, not the scanned disk, so it is only
    // a meaningful DLL search location for a live scan.
    if !system.is_offline()
        && let Some(path_value) = env::var_os("PATH")
    {
        for path in env::split_paths(&path_value) {
            push_unique(
                &mut directories,
                &mut seen,
                path,
                DependencySourceCategory::Path,
                system,
            );
        }
    }

    directories
}

fn push_unique(
    directories: &mut Vec<SearchDirectory>,
    seen: &mut BTreeSet<String>,
    path: PathBuf,
    category: DependencySourceCategory,
    system: &SystemContext,
) {
    if !system.is_dir(&path) {
        return;
    }
    let key = path.to_string_lossy().to_ascii_lowercase();
    if seen.insert(key) {
        directories.push(SearchDirectory { path, category });
    }
}

fn windows_dir(system: &SystemContext) -> Option<PathBuf> {
    if system.is_offline() {
        return Some(system.windows_dir());
    }
    env::var_os(WINDOWS_ENV_VAR)
        .or_else(|| env::var_os(LEGACY_WINDOWS_ENV_VAR))
        .map(PathBuf::from)
}

fn find_in_system_directories(
    name: &str,
    search_dirs: &[SearchDirectory],
    system: &SystemContext,
) -> Option<PathBuf> {
    search_dirs
        .iter()
        .filter(|directory| {
            matches!(
                directory.category,
                DependencySourceCategory::System32
                    | DependencySourceCategory::SysWow64
                    | DependencySourceCategory::Windows
            )
        })
        .map(|directory| directory.path.join(name))
        .find(|candidate| system.is_file(candidate))
}

fn read_architecture_with_warning(
    path: &Path,
    warnings: &mut Vec<Warning>,
) -> Option<AppArchitecture> {
    match read_pe_architecture(path) {
        Ok(architecture) => Some(architecture),
        Err(error) => {
            warnings.push(Warning::new(
                "dependency_architecture_unreadable",
                WarningSeverity::Warning,
                format!("Could not read dependency architecture: {error}"),
                Some(path.display().to_string()),
            ));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dll_is_the_runtime_its_name_says_and_windows_own_are_none() {
        for (name, runtime) in [
            ("msvcr80.dll", VisualCppRuntime::Vs2005),
            ("MSVCP90.DLL", VisualCppRuntime::Vs2008),
            ("msvcm90.dll", VisualCppRuntime::Vs2008),
            ("msvcr100.dll", VisualCppRuntime::Vs2010),
            ("mfc100u.dll", VisualCppRuntime::Vs2010),
            ("vcomp110.dll", VisualCppRuntime::Vs2012),
            ("atl110.dll", VisualCppRuntime::Vs2012),
            ("msvcr120.dll", VisualCppRuntime::Vs2013),
            ("vccorlib120.dll", VisualCppRuntime::Vs2013),
            ("msvcp140.dll", VisualCppRuntime::Vs2015Plus),
            ("msvcp140_1.dll", VisualCppRuntime::Vs2015Plus),
            ("msvcp140_codecvt_ids.dll", VisualCppRuntime::Vs2015Plus),
            ("vcruntime140.dll", VisualCppRuntime::Vs2015Plus),
            ("vcruntime140_1.dll", VisualCppRuntime::Vs2015Plus),
            ("concrt140.dll", VisualCppRuntime::Vs2015Plus),
            ("mfc140u.dll", VisualCppRuntime::Vs2015Plus),
            ("msvcr71.dll", VisualCppRuntime::Vs2003),
        ] {
            let found = visual_cpp_dll(name);
            assert_eq!(found.map(|dll| dll.runtime), Some(runtime), "{name}");
            assert_eq!(found.map(|dll| dll.debug), Some(false), "{name}");
        }
        for name in [
            "msvcp140d.dll",
            "vcruntime140d.dll",
            "vcruntime140_1d.dll",
            "msvcr120d.dll",
            "mfc140ud.dll",
        ] {
            assert_eq!(
                visual_cpp_dll(name).map(|dll| dll.debug),
                Some(true),
                "{name}"
            );
        }
        // Windows' own, whatever they start with.
        for name in [
            "msvcrt.dll",
            "msvcrt20.dll",
            "msvcrt40.dll",
            "msvcirt.dll",
            "msvcp60.dll",
            "mfc42.dll",
            "mfc42u.dll",
            "atl.dll",
            "ucrtbase.dll",
            "kernel32.dll",
            "vcomp.dll",
        ] {
            assert_eq!(visual_cpp_dll(name), None, "{name}");
            assert!(!is_runtime_prerequisite_dll(name), "{name}");
        }
    }

    #[test]
    fn classifies_api_set_as_system_provided() {
        let context = DllResolutionContext {
            executable_path: PathBuf::from(r"C:\App\App.exe"),
            app_architecture: AppArchitecture::X64,
            current_directory: None,
            recursive: true,
            include_hashes: false,
        };
        let imports = vec![ImportedDll {
            name: "api-ms-win-core-file-l1-2-0.dll".to_owned(),
            delay_loaded: false,
        }];

        let (dependencies, warnings) = resolve_imports(&imports, &context, &SystemContext::live());

        assert!(warnings.is_empty());
        assert_eq!(dependencies.len(), 1);
        assert_eq!(
            dependencies[0].source_category,
            DependencySourceCategory::ApiSet
        );
        assert!(dependencies[0].system_provided);
        assert!(!dependencies[0].missing);
    }

    #[test]
    fn resolves_app_local_before_path() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let unique = format!(
            "winappmigrator-test-{}-{}",
            std::process::id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let app_dir = root.join("app");
        fs::create_dir_all(&app_dir)?;
        let dll_path = app_dir.join("custom.dll");
        fs::write(&dll_path, [])?;

        let context = DllResolutionContext {
            executable_path: app_dir.join("app.exe"),
            app_architecture: AppArchitecture::X64,
            current_directory: None,
            recursive: true,
            include_hashes: false,
        };
        let imports = vec![ImportedDll {
            name: "custom.dll".to_owned(),
            delay_loaded: false,
        }];

        let (dependencies, _warnings) = resolve_imports(&imports, &context, &SystemContext::live());
        fs::remove_dir_all(&root)?;

        assert_eq!(dependencies.len(), 1);
        assert_eq!(
            dependencies[0].resolved_path.as_deref(),
            Some(dll_path.as_path())
        );
        assert_eq!(
            dependencies[0].source_category,
            DependencySourceCategory::AppLocal
        );
        assert!(dependencies[0].should_copy);
        Ok(())
    }
}
