//! The first half of moving real apps between PCs, on GitHub's throwaway Windows runner and nowhere
//! else. `install-real-apps.ps1` has installed real open-source apps there from their publishers, each
//! an exact version whose hash it checked, and this builds an installer of each with the real code, the
//! way the app does, with the License and User data options off. They go to other runners, where
//! `check-moved-real-apps.ps1` installs them, asks Windows to use what was restored, and removes them.
//!
//! Those installers are uploaded where anyone signed in to GitHub can get them for a day, which is a
//! redistribution of those apps, so each one's license has to allow that, and is in `real-apps.json`.
//! It is also why this looks through each package for what is the runner's own before it goes.
//!
//! `install-real-apps.ps1` also leaves what an account would have in those apps' keys, with marks in it
//! that are in nothing else. With the User data option off, none of that is anywhere in a package, in
//! any form. Another run of this, in a step of its own that uploads nothing, builds with it on, and
//! `check-account-settings.ps1` shows it arrive. With `WAM_REAL_APPS_RUNTIME` set, it builds Mosquitto
//! the way the app does, with Microsoft's redistributable in it, which is why that goes nowhere either.
//!
//! **This changes the machine it runs on, and nothing here undoes it.** It does nothing unless
//! `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, `WAM_ELEVATED_E2E=1`, and one of the
//! variables naming where to build are all set. Never set those on a real PC. With all of them set and
//! no elevation it fails, so that the job cannot pass without having run.

use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use winappmigrator_core::path_utils::POWERSHELL_UTF8_OUTPUT;
use winappmigrator_core::privilege::is_elevated;
use winappmigrator_core::restore_policy::WHOEVER_INSTALLS_IT;
use winappmigrator_core::{
    AppSelection, BuildMonitor, FootprintChoices, InstalledApp, InstallerStub, PackageManifest,
    PackageOptions, ProfileChoice, ScanMonitor, ScanSource, SystemContext,
    create_package_from_scan, default_install_location, list_installed_apps_from, scan_app,
    scan_options_for,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const OUTPUT_VARIABLE: &str = "WAM_REAL_APPS_OUTPUT";
const USER_DATA_VARIABLE: &str = "WAM_REAL_APPS_USER_DATA";
const RUNTIME_VARIABLE: &str = "WAM_REAL_APPS_RUNTIME";
/// What goes with the User data option in what an account has, as `real-apps.json` says of each.
const GOES_WITH_USER_DATA: &str = "user_data";
const GOES_WITH_LICENSE: &str = "license";
/// Hash, length, and magic, at the end of an installer, after the package.
const FOOTER_BYTES: usize = 32 + 8 + 21;
/// What `install-real-apps.ps1` registers, running 7-Zip as this runner's account.
const PERSONAL_TASK: &str = "WAM Personal Task";
/// What the runner has of its own that a package must never hold, by what it is set to.
const RUNNER_VARIABLES: [&str; 8] = [
    "USERNAME",
    "COMPUTERNAME",
    "RUNNER_TEMP",
    "RUNNER_WORKSPACE",
    "GITHUB_WORKSPACE",
    "GITHUB_TOKEN",
    "ACTIONS_RUNTIME_TOKEN",
    "ACTIONS_ID_TOKEN_REQUEST_TOKEN",
];
const CLASSIC_TOKEN_LETTERS: &[u8] = b"pousr";
const CLASSIC_TOKEN_BODY: usize = 36;
const FINE_GRAINED_TOKEN_PREFIX: &[u8] = b"github_pat_";
const FINE_GRAINED_TOKEN_BODY: usize = 22;

fn apps_file() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/scripts/real-apps.json")
}

fn the_apps() -> TestResult<Vec<Value>> {
    let parsed: Value = serde_json::from_slice(&fs::read(apps_file())?)?;
    parsed["apps"]
        .as_array()
        .cloned()
        .ok_or_else(|| "real-apps.json lists no apps".into())
}

fn text_of<'a>(value: &'a Value, field: &str) -> TestResult<&'a str> {
    value[field]
        .as_str()
        .filter(|text| !text.is_empty())
        .ok_or_else(|| format!("{value} has no {field}").into())
}

fn is_throwaway_runner(
    github_actions: Option<&str>,
    runner_environment: Option<&str>,
    flag: Option<&str>,
) -> bool {
    github_actions == Some("true")
        && runner_environment == Some("github-hosted")
        && flag == Some("1")
}

/// Whether this run may change the machine, and where to put what it builds, by what names that.
fn output_folder(variable: &str) -> TestResult<Option<PathBuf>> {
    let read = |name: &str| std::env::var(name).ok();
    if !is_throwaway_runner(
        read("GITHUB_ACTIONS").as_deref(),
        read("RUNNER_ENVIRONMENT").as_deref(),
        read("WAM_ELEVATED_E2E").as_deref(),
    ) {
        println!(
            "skipping: this changes the machine, so it only runs on GitHub's hosted runner, in \
             the job that sets WAM_ELEVATED_E2E=1"
        );
        return Ok(None);
    }
    let Some(output) = read(variable) else {
        println!("skipping: it is the step that sets {variable} that builds these");
        return Ok(None);
    };
    if !is_elevated() {
        return Err("the runner's signals are all set, but this process is not elevated".into());
    }
    Ok(Some(PathBuf::from(output)))
}

fn stub_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_winappmigrator-installer"))
}

fn powershell(script: &str) -> TestResult<String> {
    let script = format!("{POWERSHELL_UTF8_OUTPUT}$ErrorActionPreference = 'Stop'; {script}");
    let output = Command::new("powershell.exe")
        .env_remove("PSModulePath")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(format!(
        "PowerShell failed with {}: {}{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .into())
}

/// Each file below a folder with its hash, and each folder with nothing in it, by .NET, and nothing
/// of this project's. A file a running service holds open can still be read like this.
fn what_is_in_the_folder(root: &Path) -> TestResult<(Vec<Value>, Vec<String>)> {
    let shown = powershell(&format!(
        r"$root = (Get-Item -LiteralPath '{}').FullName
        $sha = [Security.Cryptography.SHA256]::Create()
        Get-ChildItem -LiteralPath $root -Recurse -Force | ForEach-Object {{
            $relative = $_.FullName.Substring($root.Length).TrimStart('\')
            if ($_.PSIsContainer) {{
                if (@(Get-ChildItem -LiteralPath $_.FullName -Force).Count -eq 0) {{ 'folder|' + $relative }}
            }} else {{
                $stream = [IO.File]::Open($_.FullName, 'Open', 'Read', 'ReadWrite')
                $hash = [BitConverter]::ToString($sha.ComputeHash($stream)).Replace('-', '')
                $stream.Dispose()
                'file|' + $relative + '|' + $hash.ToLowerInvariant()
            }}
        }}",
        root.display()
    ))?;
    let mut files = Vec::new();
    let mut empty_folders = Vec::new();
    for line in shown.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(relative) = line.strip_prefix("folder|") {
            empty_folders.push(relative.to_owned());
        } else if let Some((relative, hash)) = line
            .strip_prefix("file|")
            .and_then(|rest| rest.rsplit_once('|'))
        {
            files.push(json!({ "relative": relative, "sha256": hash }));
        } else {
            return Err(format!("could not read: {line}").into());
        }
    }
    Ok((files, empty_folders))
}

fn is_token_letter(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

/// Whether these bytes hold something the shape of one of GitHub's tokens.
fn has_token_shape(bytes: &[u8]) -> bool {
    let classic = bytes.windows(4 + CLASSIC_TOKEN_BODY).any(|window| {
        window.starts_with(b"gh")
            && CLASSIC_TOKEN_LETTERS.contains(&window[2])
            && window[3] == b'_'
            && window[4..].iter().all(|byte| is_token_letter(*byte))
    });
    let prefix = FINE_GRAINED_TOKEN_PREFIX.len();
    classic
        || bytes
            .windows(prefix + FINE_GRAINED_TOKEN_BODY)
            .any(|window| {
                window.starts_with(FINE_GRAINED_TOKEN_PREFIX)
                    && window[prefix..]
                        .iter()
                        .all(|byte| is_token_letter(*byte) || *byte == b'_')
            })
}

/// What in these bytes is the runner's own, as UTF-8 or as UTF-16, whatever its case. Each literal
/// comes back as it was asked for, so a caller can say what kind of thing it found without showing it.
fn what_is_the_runners(bytes: &[u8], literals: &[String]) -> Vec<String> {
    let every_other = |start: usize| -> Vec<u8> {
        bytes
            .iter()
            .skip(start)
            .step_by(2)
            .map(u8::to_ascii_lowercase)
            .collect()
    };
    let mut found = Vec::new();
    // Tokens are told apart by case, so that search goes by the bytes as they are.
    let forms = [bytes.to_ascii_lowercase(), every_other(0), every_other(1)];
    for literal in literals.iter().filter(|literal| !literal.is_empty()) {
        let wanted = literal.to_ascii_lowercase();
        // And as JSON and a `.reg` file write a path.
        let escaped = wanted.replace('\\', r"\\");
        if [&wanted, &escaped].iter().any(|text| {
            forms
                .iter()
                .any(|form| form.windows(text.len()).any(|at| at == text.as_bytes()))
        }) {
            found.push(literal.clone());
        }
    }
    let wide: Vec<u8> = bytes.iter().step_by(2).copied().collect();
    let wide_odd: Vec<u8> = bytes.iter().skip(1).step_by(2).copied().collect();
    if has_token_shape(bytes) || has_token_shape(&wide) || has_token_shape(&wide_odd) {
        found.push("something the shape of a GitHub token".to_owned());
    }
    found
}

/// Each thing to look for, with the name of the variable it came from, to say instead of its value.
fn runner_literals() -> Vec<(String, &'static str)> {
    RUNNER_VARIABLES
        .iter()
        .filter_map(|name| std::env::var(name).ok().map(|value| (value, *name)))
        .filter(|(value, _)| !value.is_empty())
        .collect()
}

/// What these found, named by variable, since what was found could be a token.
fn say_what_was_found(found: Vec<String>, named: &[(String, &'static str)]) -> Vec<String> {
    found
        .into_iter()
        .map(|literal| {
            named
                .iter()
                .find(|(value, _)| *value == literal)
                .map_or(literal.clone(), |(_, name)| format!("the value of {name}"))
        })
        .collect()
}

/// A mark left in what an account has, and what it is to go with.
struct AccountMark {
    app: String,
    text: String,
    goes_with: String,
}

/// A value's own mark, without what comes before it, which JSON and a `.reg` file write in ways of their
/// own. What the keys and values are called have marks too, since a host can be what a key is called.
fn account_marks() -> TestResult<Vec<AccountMark>> {
    let parsed: Value = serde_json::from_slice(&fs::read(apps_file())?)?;
    let settings = parsed["account_settings"]
        .as_array()
        .ok_or("real-apps.json lists no account settings")?;
    let mut marks = Vec::new();
    for file in parsed["account_files"]
        .as_array()
        .ok_or("real-apps.json lists no account files")?
    {
        let name = text_of(file, "relative")?;
        for text in [
            name.find("WamMark").map_or(name, |start| &name[start..]),
            text_of(file, "content")?,
        ] {
            marks.push(AccountMark {
                app: text_of(file, "app")?.to_owned(),
                text: text.to_owned(),
                goes_with: text_of(file, "goes_with")?.to_owned(),
            });
        }
    }
    for setting in settings {
        for field in ["marker", "key", "name"] {
            let text = text_of(setting, field)?;
            let mark = text
                .find("WamMark")
                .map_or(text, |start| &text[start..])
                .to_owned();
            let is_a_mark = field == "marker" || mark.starts_with("WamMark");
            if is_a_mark && !marks.iter().any(|known: &AccountMark| known.text == mark) {
                marks.push(AccountMark {
                    app: text_of(setting, "app")?.to_owned(),
                    text: mark,
                    goes_with: text_of(setting, "goes_with")?.to_owned(),
                });
            }
        }
    }
    Ok(marks)
}

/// Each form a package could hold this text in: as text of either width, and as the three ways a
/// binary value is written down, which are the hex of a manifest, the same in a `.reg` file, and the
/// numbers its text is shown as. All lowercase, as what is searched is.
fn forms_of_a_mark(mark: &str) -> Vec<(&'static str, Vec<u8>)> {
    let lower = mark.to_ascii_lowercase();
    let wide: Vec<u8> = lower.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let mixed: Vec<u8> = mark.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let in_hex = |bytes: &[u8], between: &str| -> Vec<u8> {
        bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<_>>()
            .join(between)
            .into_bytes()
    };
    let numbers = |bytes: &[u8]| -> Vec<u8> {
        bytes
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ")
            .into_bytes()
    };
    vec![
        ("UTF-8", lower.clone().into_bytes()),
        ("UTF-16", wide),
        ("UTF-8 as hex", in_hex(mark.as_bytes(), "")),
        ("UTF-16 as hex", in_hex(&mixed, "")),
        ("UTF-8 as a .reg file's hex", in_hex(mark.as_bytes(), ",")),
        ("UTF-16 as a .reg file's hex", in_hex(&mixed, ",")),
        ("UTF-8 as numbers", numbers(mark.as_bytes())),
        ("UTF-16 as numbers", numbers(&mixed)),
    ]
}

/// Where in a package each form of a mark is, by the entry's name or what is in it.
fn where_a_mark_is(mark: &str, names: &[String], unpacked: &Path) -> TestResult<Vec<String>> {
    let forms = forms_of_a_mark(mark);
    let holds = |bytes: &[u8]| -> Vec<&'static str> {
        let lowered = bytes.to_ascii_lowercase();
        forms
            .iter()
            .filter(|(_, form)| lowered.windows(form.len()).any(|at| at == form.as_slice()))
            .map(|(name, _)| *name)
            .collect()
    };
    let mut found = Vec::new();
    for name in names {
        for form in holds(name.as_bytes()) {
            found.push(format!("the name of {name}, as {form}"));
        }
    }
    for file in files_below(unpacked)? {
        for form in holds(&fs::read(&file)?) {
            found.push(format!(
                "{}, as {form}",
                file.strip_prefix(unpacked)?.display()
            ));
        }
    }
    Ok(found)
}

/// What is in an installer's package, by .NET's own zip reader, and where that unpacked it.
fn unpack_package(installer: &Path, work: &Path) -> TestResult<(Vec<String>, PathBuf)> {
    let bytes = fs::read(installer)?;
    let footer = bytes
        .get(bytes.len().saturating_sub(FOOTER_BYTES)..)
        .ok_or("the installer is too short")?;
    let mut length = [0_u8; 8];
    length.copy_from_slice(&footer[32..40]);
    let length = usize::try_from(u64::from_le_bytes(length))?;
    let end = bytes.len() - FOOTER_BYTES;
    let zip = work.join("package.zip");
    fs::write(&zip, &bytes[end - length..end])?;
    let unpacked = work.join("unpacked");
    let quoted = |path: &Path| format!("'{}'", path.display().to_string().replace('\'', "''"));
    let shown = powershell(&format!(
        "Add-Type -AssemblyName System.IO.Compression.FileSystem; \
         $zip = [IO.Compression.ZipFile]::OpenRead({zip}); \
         foreach ($e in $zip.Entries) {{ $e.FullName }}; $zip.Dispose(); \
         [IO.Compression.ZipFile]::ExtractToDirectory({zip}, {to})",
        zip = quoted(&zip),
        to = quoted(&unpacked),
    ))?;
    fs::remove_file(&zip)?;
    Ok((
        shown
            .lines()
            .map(|line| line.trim().replace('/', "\\"))
            .filter(|line| !line.is_empty())
            .collect(),
        unpacked,
    ))
}

fn files_below(root: &Path) -> TestResult<Vec<PathBuf>> {
    let mut found = Vec::new();
    let mut folders = vec![root.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                folders.push(entry.path());
            } else {
                found.push(entry.path());
            }
        }
    }
    Ok(found)
}

/// What of the runner's account or its tokens would go out with this package.
fn what_of_the_runners_it_holds(names: &[String], unpacked: &Path) -> TestResult<Vec<String>> {
    let named = runner_literals();
    let literals: Vec<String> = named.iter().map(|(value, _)| value.clone()).collect();
    let mut problems = Vec::new();
    for name in names {
        let lowered = name.to_ascii_lowercase();
        if lowered.contains(r"users\") || lowered.contains("appdata") {
            problems.push(format!("the entry {name}"));
        }
        for found in say_what_was_found(what_is_the_runners(name.as_bytes(), &literals), &named) {
            problems.push(format!("{found} in the name of {name}"));
        }
    }
    for file in files_below(unpacked)? {
        let bytes = fs::read(&file)?;
        let relative = file.strip_prefix(unpacked)?;
        for found in say_what_was_found(what_is_the_runners(&bytes, &literals), &named) {
            problems.push(format!("{found} in {}", relative.display()));
        }
    }
    println!(
        "  looked through {} entries for the runner's account, folders, and tokens",
        names.len()
    );
    Ok(problems)
}

/// What the scan took, for a person to read in the log, with what is outside the app's folder by name.
fn show_what_it_took(manifest: &PackageManifest, root: &Path) {
    let mut copied = 0_usize;
    let mut bytes = 0_u64;
    let mut outside = Vec::new();
    let mut folders: BTreeMap<String, usize> = BTreeMap::new();
    for file in manifest
        .file_inventory
        .iter()
        .filter(|file| file.should_copy)
    {
        copied += 1;
        bytes += file.size_bytes.unwrap_or(0);
        match file.original_path.strip_prefix(root) {
            Ok(relative) => {
                let top = relative
                    .parent()
                    .and_then(|parent| parent.components().next())
                    .map_or_else(
                        || ".".to_owned(),
                        |part| part.as_os_str().to_string_lossy().into_owned(),
                    );
                *folders.entry(top).or_default() += 1;
            }
            Err(_) => outside.push(file.original_path.display().to_string()),
        }
    }
    println!(
        "  files: {copied}, {bytes} bytes, of {} listed; by folder below {}: {folders:?}",
        manifest.file_inventory.len(),
        root.display()
    );
    println!("  files outside that folder: {outside:#?}");
    let not_copied: Vec<String> = manifest
        .file_inventory
        .iter()
        .filter(|file| !file.should_copy)
        .map(|file| format!("{}: {}", file.original_path.display(), file.reason))
        .collect();
    println!("  listed and not copied: {not_copied:#?}");
    let keys: Vec<String> = manifest
        .registry_keys_exported
        .iter()
        .map(|entry| {
            format!(
                "{:?} {} : {}",
                entry.hive,
                entry.key_path,
                entry.value_name.as_deref().unwrap_or("(key)")
            )
        })
        .collect();
    println!("  registry keys: {keys:#?}");
    println!(
        "  COM: {:#?}\n  file types: {:#?}\n  services: {:#?}\n  tasks: {:#?}\n  shortcuts: {:#?}",
        manifest.com_registrations_detected,
        manifest.file_associations_detected,
        manifest.services_detected,
        manifest.scheduled_tasks_detected,
        manifest.shortcuts_detected,
    );
    let environment: Vec<String> = manifest
        .environment_variables_detected
        .iter()
        .map(|variable| format!("{:?} {}", variable.scope, variable.name))
        .collect();
    println!(
        "  environment: {environment:?}\n  firewall: {:#?}\n  needs: {:#?}\n  left out: {:?} {:?} {:?}\n  \
         excluded: {:#?}\n  warnings: {:#?}",
        manifest.firewall_rules_detected,
        manifest.runtime_prerequisites,
        manifest.secrets_left_out,
        manifest.licenses_left_out,
        manifest.settings_left_out,
        manifest.items_intentionally_excluded,
        manifest.warnings,
    );
}

fn the_app_named(listed: &[InstalledApp], name: &str) -> TestResult<InstalledApp> {
    let found: Vec<&InstalledApp> = listed
        .iter()
        .filter(|app| app.display_name.starts_with(name))
        .collect();
    match found.as_slice() {
        [app] => Ok((*app).clone()),
        _ => Err(format!(
            "{} of the listed apps start with {name}: {:?}",
            found.len(),
            found
                .iter()
                .map(|app| app.display_name.as_str())
                .collect::<Vec<_>>()
        )
        .into()),
    }
}

/// These only read, so they run anywhere, and a mistake in the list shows up here and not on a runner.
#[test]
fn each_app_is_an_exact_version_with_a_hash_and_a_license() -> TestResult {
    let apps = the_apps()?;
    assert!(!apps.is_empty());
    let mut ids = HashSet::new();
    for app in &apps {
        assert!(ids.insert(text_of(app, "id")?.to_owned()), "{app}");
        text_of(app, "name")?;
        text_of(app, "license")?;
        assert!(text_of(app, "url")?.starts_with("https://"), "{app}");
        assert!(
            text_of(app, "installed_at")?.starts_with(r"C:\Program Files\"),
            "{app}"
        );
        let sha256 = text_of(app, "sha256")?;
        assert_eq!(sha256.len(), 64, "{app}");
        assert!(
            sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
            "{app}"
        );
    }
    Ok(())
}

#[test]
fn the_search_finds_what_is_the_runners_however_it_is_written() {
    let literals = vec!["runneradmin".to_owned()];
    assert!(what_is_the_runners(b"nothing of note", &literals).is_empty());
    assert_eq!(
        what_is_the_runners(br"C:\Users\RunnerAdmin\x", &literals),
        ["runneradmin"]
    );
    let wide: Vec<u8> = "C:\\Users\\runneradmin"
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    assert_eq!(what_is_the_runners(&wide, &literals), ["runneradmin"]);
    let folder = vec![r"D:\a\_temp".to_owned()];
    assert_eq!(
        what_is_the_runners(br#"{"InstallSource": "D:\\a\\_temp\\x\\"}"#, &folder),
        folder
    );
    // A blank value of a variable that is not set finds nothing.
    assert!(what_is_the_runners(b"anything", &[String::new()]).is_empty());
}

#[test]
fn the_search_goes_by_the_shape_of_a_token_and_not_by_a_prefix_alone() {
    let classic = format!("x ghs_{} y", "a1B2c3D4e5".repeat(4));
    assert!(has_token_shape(classic.as_bytes()));
    let fine_grained = format!("github_pat_{}", "aB1_".repeat(8));
    assert!(has_token_shape(fine_grained.as_bytes()));
    let wide: Vec<u8> = classic.encode_utf16().flat_map(u16::to_le_bytes).collect();
    assert!(!what_is_the_runners(&wide, &[]).is_empty());
    assert!(!has_token_shape(
        b"ghs_short and ghx_a1B2c3D4e5a1B2c3D4e5a1B2c3D4e5a1B2c3"
    ));
}

/// Options the way the app has them for this, with everything the app's own switches leave off.
fn options_for(app: InstalledApp, output: &Path, id: &str, work: &Path) -> PackageOptions {
    PackageOptions {
        selection: AppSelection::InstalledApp(Box::new(app)),
        output_dir: work.join("staging").join(id),
        current_directory: None,
        create_self_extracting_archive: true,
        self_extracting_stub: Some(InstallerStub::File(stub_binary())),
        installer_dir: Some(output.to_path_buf()),
        include_user_data: false,
        include_program_data: false,
        include_license_data: false,
        bundle_prerequisites: false,
        source: ScanSource::Live,
        user_profile: ProfileChoice::Default,
    }
}

/// Build as the app does: a look at it first, told of nowhere to write, and then by what that found.
fn build(options: &PackageOptions) -> TestResult<(PathBuf, PackageManifest)> {
    let report = scan_app(
        &scan_options_for(&PackageOptions {
            output_dir: PathBuf::new(),
            ..options.clone()
        }),
        &ScanMonitor::default(),
    )?;
    let result = create_package_from_scan(
        options,
        &report,
        &FootprintChoices::default(),
        &BuildMonitor::default(),
    )?;
    for warning in &result.warnings {
        println!("  warning {}: {}", warning.code, warning.message);
    }
    let installer = result
        .self_extracting_archive_path
        .clone()
        .ok_or("no installer was built")?;
    let manifest = serde_json::from_slice(&fs::read(&result.manifest_path)?)?;
    Ok((installer, manifest))
}

/// With the User data option on, what an account has for these apps is in what is built, and what is
/// a license only ever goes by the License option, whichever way the other is. Uploads nothing:
/// `check-account-settings.ps1` installs what this leaves, on this runner.
#[test]
fn what_an_account_has_follows_the_options() -> TestResult {
    let Some(output) = output_folder(USER_DATA_VARIABLE)? else {
        return Ok(());
    };
    fs::create_dir_all(&output)?;
    let marks = account_marks()?;
    let listed = list_installed_apps_from(&SystemContext::live())?;
    let mut problems = Vec::new();
    let mut built = Vec::new();
    for spec in the_apps()? {
        let id = text_of(&spec, "id")?;
        if !marks.iter().any(|mark| mark.app == id) {
            continue;
        }
        let name = text_of(&spec, "name")?;
        println!("\n=== {name}, with the User data option on");
        let app = the_app_named(&listed, name)?;
        let work = tempfile::tempdir()?;
        let (installer, manifest) = build(&PackageOptions {
            include_user_data: true,
            include_program_data: true,
            ..options_for(app.clone(), &output, id, work.path())
        })?;
        let (names, unpacked) = unpack_package(&installer, work.path())?;
        println!(
            "  the package says it holds the data of {:?}, and left out {:?} and {:?}",
            manifest.user_data_profile, manifest.settings_left_out, manifest.licenses_left_out
        );
        if !manifest.settings_left_out.is_empty() {
            problems.push(format!("{name}: it left settings out with the option on"));
        }
        // The search of a package with the option off finds nothing of this, and this says it could.
        if manifest.source_computer_name != std::env::var("COMPUTERNAME").ok() {
            problems.push(format!(
                "{name}: with the option on it should say what PC it came from, and says {:?}",
                manifest.source_computer_name
            ));
        }
        for mark in marks.iter().filter(|mark| mark.app == id) {
            let found = where_a_mark_is(&mark.text, &names, &unpacked)?;
            println!(
                "  the mark {} (with {}) is in: {found:#?}",
                mark.text, mark.goes_with
            );
            let should_be_there = mark.goes_with == GOES_WITH_USER_DATA;
            if found.is_empty() == should_be_there {
                problems.push(format!(
                    "{name}: the mark {} goes with {}, and is in {found:?}",
                    mark.text, mark.goes_with
                ));
            }
        }
        built.push(json!({
            "id": id,
            "name": name,
            "installer": installer
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        }));

        if marks
            .iter()
            .any(|mark| mark.app == id && mark.goes_with == GOES_WITH_LICENSE)
        {
            println!("\n=== {name}, with the License option on and the User data option off");
            let aside = tempfile::tempdir()?;
            let elsewhere = aside.path().join("installers");
            fs::create_dir_all(&elsewhere)?;
            let (installer, manifest) = build(&PackageOptions {
                include_license_data: true,
                ..options_for(app, &elsewhere, id, aside.path())
            })?;
            let (names, unpacked) = unpack_package(&installer, aside.path())?;
            for mark in marks.iter().filter(|mark| mark.app == id) {
                let found = where_a_mark_is(&mark.text, &names, &unpacked)?;
                println!(
                    "  the mark {} (with {}) is in: {found:#?}",
                    mark.text, mark.goes_with
                );
                let should_be_there = mark.goes_with == GOES_WITH_LICENSE;
                if found.is_empty() == should_be_there {
                    problems.push(format!(
                        "{name}, with License on: the mark {} goes with {}, and is in {found:?}",
                        mark.text, mark.goes_with
                    ));
                }
            }
            let in_its_list = manifest
                .license_registry_entries
                .iter()
                .any(|entry| entry.value_name.as_deref() == Some("LicenseKey"));
            let in_the_settings = manifest
                .registry_keys_exported
                .iter()
                .any(|entry| entry.value_name.as_deref() == Some("LicenseKey"));
            println!(
                "  LicenseKey: in the license values {in_its_list}, in the rest {in_the_settings}"
            );
            if !in_its_list || in_the_settings {
                problems.push(format!(
                    "{name}: LicenseKey is in the license values {in_its_list}, in the rest {in_the_settings}"
                ));
            }
        }
    }
    if !problems.is_empty() {
        return Err(format!("{problems:#?}").into());
    }
    fs::write(
        output.join("expected.json"),
        serde_json::to_vec_pretty(&json!({ "apps": built }))?,
    )?;
    Ok(())
}

/// Mosquitto as the app builds it, with Microsoft's redistributable in the package, checked by its
/// signature. Not uploaded: that license does not cover it. `check-runtime-arrives.ps1` takes that
/// runtime off this runner, and installs what this leaves.
#[test]
fn build_one_with_its_runtime_in_it() -> TestResult {
    let Some(output) = output_folder(RUNTIME_VARIABLE)? else {
        return Ok(());
    };
    fs::create_dir_all(&output)?;
    let spec = the_apps()?
        .into_iter()
        .find(|app| app["id"] == "mosquitto")
        .ok_or("real-apps.json has no Mosquitto")?;
    let name = text_of(&spec, "name")?;
    let listed = list_installed_apps_from(&SystemContext::live())?;
    let work = tempfile::tempdir()?;
    let (installer, manifest) = build(&PackageOptions {
        bundle_prerequisites: true,
        ..options_for(
            the_app_named(&listed, name)?,
            &output,
            "mosquitto",
            work.path(),
        )
    })?;
    println!("  needs: {:#?}", manifest.runtime_prerequisites);
    let bundled: Vec<_> = manifest
        .runtime_prerequisites
        .iter()
        .filter_map(|prerequisite| {
            prerequisite
                .bundle
                .as_ref()
                .and_then(|bundle| bundle.package_relative_path.as_ref())
                .map(|relative| (prerequisite.name.clone(), relative.clone()))
        })
        .collect();
    let [(prerequisite, relative)] = bundled.as_slice() else {
        return Err(format!("it should hold one runtime, and holds {bundled:?}").into());
    };
    let (_, unpacked) = unpack_package(&installer, work.path())?;
    let payload = unpacked.join(relative);
    let signature = powershell(&format!(
        "$signature = Get-AuthenticodeSignature -LiteralPath '{}'; \
         $signature.Status.ToString() + '|' + $signature.SignerCertificate.Subject",
        payload.display()
    ))?;
    println!(
        "  {prerequisite} is in the package at {}: {signature}",
        relative.display()
    );
    if !signature.trim().starts_with("Valid|") || !signature.contains("O=Microsoft Corporation") {
        return Err(format!("its signature says {signature}").into());
    }
    fs::write(
        output.join("expected.json"),
        serde_json::to_vec_pretty(&json!({
            "name": name,
            "prerequisite": prerequisite,
            "installer": installer
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        }))?,
    )?;
    Ok(())
}

#[test]
fn every_form_of_a_mark_is_looked_for() -> TestResult {
    const MARK: &str = "WamMark";
    let work = tempfile::tempdir()?;
    let wide: Vec<u8> = MARK.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let cases: Vec<(&str, Vec<u8>)> = vec![
        ("UTF-8", b"x wammark y".to_vec()),
        ("UTF-16", wide.clone()),
        (
            "UTF-16 as hex",
            wide.iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
                .into_bytes(),
        ),
        (
            "UTF-16 as a .reg file's hex",
            wide.iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<Vec<_>>()
                .join(",")
                .into_bytes(),
        ),
        ("UTF-16 as numbers", format!("{wide:?}").into_bytes()),
    ];
    for (form, bytes) in cases {
        let file = work.path().join("a.txt");
        fs::write(&file, bytes)?;
        let found = where_a_mark_is(MARK, &[], work.path())?;
        assert!(
            found
                .iter()
                .any(|place| place.ends_with(&format!("as {form}"))),
            "{form}: {found:?}"
        );
    }
    fs::write(work.path().join("a.txt"), b"nothing of note")?;
    assert!(where_a_mark_is(MARK, &[], work.path())?.is_empty());
    assert_eq!(
        where_a_mark_is(MARK, &[r"files\WamMark\a.txt".to_owned()], work.path())?.len(),
        1
    );
    Ok(())
}

#[test]
fn build_what_goes_to_the_other_pcs() -> TestResult {
    let Some(output) = output_folder(OUTPUT_VARIABLE)? else {
        return Ok(());
    };
    let marks = account_marks()?;
    fs::create_dir_all(output.join("manifests"))?;
    let listed = list_installed_apps_from(&SystemContext::live())?;

    let mut built = Vec::new();
    let mut holding_the_runners = Vec::new();
    for spec in the_apps()? {
        let id = text_of(&spec, "id")?;
        let name = text_of(&spec, "name")?;
        println!("\n=== {name}");
        let app = the_app_named(&listed, name)?;
        println!(
            "  listed as {} {}, by {}, at {}",
            app.display_name,
            app.version.as_deref().unwrap_or("?"),
            app.publisher.as_deref().unwrap_or("?"),
            app.install_location
                .as_ref()
                .map_or_else(|| "?".to_owned(), |path| path.display().to_string())
        );

        let work = tempfile::tempdir()?;
        let options = options_for(app, &output, id, work.path());
        // The way the app does: a look at it first, told of nowhere to write, and the build by what
        // that found.
        let report = scan_app(
            &scan_options_for(&PackageOptions {
                output_dir: PathBuf::new(),
                ..options.clone()
            }),
            &ScanMonitor::default(),
        )?;
        let near: Vec<String> = report
            .near_matches
            .iter()
            .map(|near| format!("{}: {}", near.name, near.why))
            .collect();
        println!("  offered and not ticked: {near:#?}");
        let result = create_package_from_scan(
            &options,
            &report,
            &FootprintChoices::default(),
            &BuildMonitor::default(),
        )?;
        let installer = result
            .self_extracting_archive_path
            .clone()
            .ok_or("no installer was built")?;
        let manifest_bytes = fs::read(&result.manifest_path)?;
        let manifest: PackageManifest = serde_json::from_slice(&manifest_bytes)?;
        let root = default_install_location(&manifest)
            .ok_or("the package does not say where its app was")?;
        show_what_it_took(&manifest, &root);
        fs::write(
            output.join("manifests").join(format!("{id}.json")),
            &manifest_bytes,
        )?;
        let (names, unpacked) = unpack_package(&installer, work.path())?;
        for problem in what_of_the_runners_it_holds(&names, &unpacked)? {
            holding_the_runners.push(format!("{name}: {problem}"));
        }
        // With the User data option off and the License option off, as this was built.
        for mark in &marks {
            let found = where_a_mark_is(&mark.text, &names, &unpacked)?;
            println!(
                "  the mark {} ({}'s, with {}): {}",
                mark.text,
                mark.app,
                mark.goes_with,
                if found.is_empty() {
                    "in none of the package, in any form".to_owned()
                } else {
                    format!("FOUND in {found:?}")
                }
            );
            if !found.is_empty() {
                holding_the_runners.push(format!("{name}: the mark {} in {found:?}", mark.text));
            }
        }
        // What the build itself looked for last, in what it wrote. These apps are machine-wide, and
        // were built with none of an account's data, so nothing should have come of that.
        for warning in result
            .warnings
            .iter()
            .filter(|warning| warning.code == "package_mentions_an_account")
        {
            holding_the_runners.push(format!("{name}: the build says: {}", warning.message));
        }
        if id == "7zip" {
            let tasks = &manifest.scheduled_tasks_detected;
            println!("  its tasks: {tasks:#?}");
            println!("  its shortcuts: {:#?}", manifest.shortcuts_detected);
            println!("  what it left out: {:#?}", manifest.settings_left_out);
            let mine = tasks.iter().find(|task| task.task_name == PERSONAL_TASK);
            let says_who = mine.is_some_and(|task| {
                task.account.as_deref() == Some(WHOEVER_INSTALLS_IT)
                    && task
                        .definition_xml
                        .as_deref()
                        .is_some_and(|xml| !xml.contains("<Author>") && !xml.contains("run.cmd"))
            });
            if !says_who {
                holding_the_runners
                    .push(format!("{name}: its task is not as it should be: {mine:?}"));
            }
            let left_out_the_rest = |what: &str| {
                manifest
                    .settings_left_out
                    .iter()
                    .any(|item| item.name.contains(what))
            };
            if !left_out_the_rest(PERSONAL_TASK) || !left_out_the_rest("Shortcuts") {
                holding_the_runners.push(format!(
                    "{name}: it does not say what it left out of what someone made of it"
                ));
            }
            let locations: Vec<String> = manifest
                .shortcuts_detected
                .iter()
                .map(|shortcut| shortcut.original_lnk_path.display().to_string())
                .collect();
            if !locations.iter().any(|location| {
                location.eq_ignore_ascii_case(r"%USERPROFILE%\Desktop\WAM Plain 7-Zip.lnk")
            }) || locations
                .iter()
                .any(|location| location.contains("WAM Personal"))
            {
                holding_the_runners.push(format!("{name}: its shortcuts are {locations:?}"));
            }
        }
        let its_marks = marks.iter().filter(|mark| mark.app == id).count();
        if its_marks > 0 && manifest.settings_left_out.is_empty() {
            holding_the_runners.push(format!(
                "{name}: it has settings to leave out, and says it left none out"
            ));
        }

        let (files, empty_folders) = what_is_in_the_folder(&root)?;
        println!(
            "  its folder has {} files, by .NET, and these empty folders: {empty_folders:?}",
            files.len()
        );
        let mut listed_empty: Vec<String> = manifest
            .empty_folders
            .iter()
            .filter_map(|folder| folder.original_path.strip_prefix(&root).ok())
            .map(|relative| relative.display().to_string())
            .collect();
        listed_empty.sort();
        let mut really_empty = empty_folders.clone();
        really_empty.sort();
        println!("  the package will make: {listed_empty:?}");
        if listed_empty != really_empty {
            holding_the_runners.push(format!(
                "{name}: it lists {listed_empty:?} as empty folders, and .NET found {really_empty:?}"
            ));
        }
        built.push(json!({
            "id": id,
            "files": files,
            "empty_folders": empty_folders,
            "name": name,
            "installer": installer
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
            "source_root": root.display().to_string(),
        }));
    }

    if !holding_the_runners.is_empty() {
        return Err(format!("these hold the runner's own: {holding_the_runners:#?}").into());
    }

    let build = powershell(
        "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion').CurrentBuildNumber",
    )?;
    let expected = json!({
        "source_build": build.trim().parse::<u32>()?,
        "apps": built,
    });
    fs::write(
        output.join("expected.json"),
        serde_json::to_vec_pretty(&expected)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&expected)?);
    Ok(())
}
