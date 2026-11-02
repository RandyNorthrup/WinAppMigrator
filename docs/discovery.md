# Application Footprint Discovery

This document describes how WinAppMigrator reverse-engineers an installed
application's footprint. The goal is to be **authoritative where Windows records
the truth**, and heuristic where it does not. The heuristics currently match more
broadly than they should; see [Known limitations](#known-limitations).

## Layered approach

For a selected installed application the scan runs three layers and unions the
results:

1. **Install-method detection** — classify *how* the app was installed so the
   right authoritative source can be consulted.
2. **Authoritative source** — read the exact installed paths from the installer
   database when one exists (MSI, Inno Setup, NSIS `AdvUninstLog`), plus App Paths
   registration.
3. **Heuristic footprint** — discover the install/data directories that the
   above did not cover, especially per-user/per-machine data the installer never
   records (created at runtime).

Code:

- `crates/winappmigrator-core/src/app_inventory.rs` — the list of installed apps.
- `crates/winappmigrator-core/src/install_source.rs` — layers 1 and 2.
- `crates/winappmigrator-core/src/file_discovery.rs` — layer 3 plus the unified
  walk, classification, and the `FootprintLocation` summary.
- `crates/winappmigrator-core/src/scan.rs` — orchestrates the layers, folds the
  install-source roots into discovery, and records the result on `ScanReport`.

## The application list

`app_inventory.rs` reads the Uninstall keys of HKLM (64- and 32-bit views) and HKCU.
Entries with no display name are skipped, and so are entries marked `SystemComponent=1`,
as in Windows' own installed-apps list: those are parts of other products. Microsoft
Store apps have no Uninstall entry, so they are not listed.

## Layer 1: Install-method detection

`install_source::detect_install_source(&SystemContext, &InstalledApp)` returns an
`InstallSource { method, product_code, authoritative_file_count, roots, evidence }`.
The `InstallMethod` is one of:

| Method | Signal |
| --- | --- |
| `WindowsInstaller` (MSI) | Uninstall key name is a product-code GUID, `UninstallString` contains `msiexec`, or a `{GUID}` is present |
| `Msix` | Install location under `\WindowsApps\` |
| `Squirrel` | Install location under `\AppData\Local\` with `Update.exe` or an `app-*` subdir |
| `Chocolatey` | Install location under `\chocolatey\lib\`, or an uninstall string under `\chocolatey\` |
| `InnoSetup` | A `unins###.dat` log exists in the install directory |
| `Nsis` | The uninstaller binary contains the `Nullsoft` signature |
| `Generic` | None of the above |

`evidence` is a human-readable list of the signals that were used. It appears in the
Markdown report inside the package.

## Layer 2: Authoritative sources

### Windows Installer (MSI)

MSI keeps a component database under
`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\<SID>\Components`.
Each component subkey is named with a **squished** (packed) component GUID and
holds one value per consuming product (also a squished GUID) whose data is the
component's **key path** — the concrete file or registry key the installer
placed on disk.

- `squish_guid` packs a `{GUID}` into the 32-hex registry form
  (`{C653D4A8-392F-476F-A1B5-C64F217938D7}` → `8A4D356CF293F6741A5B6CF41297837D`).
- `msi_component_paths` iterates every component subkey across the relevant SIDs and
  reads the squished-product value; file key paths are collected, registry key
  paths (encoded `NN:\...`) are counted.
- The file paths are collapsed to top-level install roots via
  `minimal_install_roots` (`install_root_for_file` resolves the first segment
  under a known base, e.g. `Program Files\Inkscape`). This collapse is too coarse for
  shared bases: one app's file under `ProgramData\Microsoft` makes all of
  `ProgramData\Microsoft` a root.

The key paths match the official `WindowsInstaller.Installer` COM `ComponentPath`
results (validated against Inkscape: 11,657 paths; iTunes: 4,408, including
out-of-install-dir `Common Files\Apple` and `ProgramData\Apple Computer`).

> Performance: the sweep is narrowed to the SIDs that actually register the
> product (`component_sweep_sids`: a component only records a product's key path
> under the SID where the product is registered, so other per-user component
> trees are skipped on multi-user machines), with a full-SID fallback if none is
> found. Within a registering SID it still iterates that SID's components, once per
> scan, or three times when the main executable has to be found first. On a heavy
> developer machine that can take about a minute, on a background thread.
>
> Fully scoping iteration to **one product's** components would require either
> decoding the `Products\<squished>\Features` feature descriptors or calling the
> Windows Installer API (`MsiGetComponentPathW`). This is **deliberately not
> done**: a mis-decoded feature descriptor would silently *under-report* the
> migration footprint, and the exhaustive sweep is the source that was validated
> against COM. Revisit only with the MSI API plus a cross-check against the sweep.

### Inno Setup

Inno Setup records every installed file/dir/registry key in an uninstall log
`unins000.dat` in the install directory, as **resolved absolute paths** (UTF-16
in modern Unicode logs, ANSI in old ones).

- `find_inno_log` locates a `unins###.dat` (deliberately not a plain
  `uninstall.exe`, which other installers use).
- `inno_manifest_files` verifies the `Inno Setup Uninstall Log` signature, then
  extracts path strings directly (`extract_utf16_paths` / `extract_ansi_paths`)
  rather than decoding Inno's versioned record framing.
- Shortcut/Start Menu/system paths are filtered (`is_app_footprint_path`) before
  deriving roots so a single `.lnk` does not turn a shared folder into a root.

This reads some logs (Kernel OST Viewer: 9 recorded paths) but **fails on others**: the
framing bytes after each path decode as text and corrupt it, so paths do not resolve (Git
for Windows: 0 of 10,083). The scan then falls back to the install directory and the
heuristic layer. Parsing Inno's length prefixes would fix this.

### App Paths

`app_paths_roots` reads
`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\<exe>` (64- and 32-bit
views) for the registered launcher path and the `Path` DLL search directories.
Used for every method.

### MSIX

Detected when the install location is under `\WindowsApps\`, which only applies to an
entry with an Uninstall key: Store apps are not listed at all. The install payload
there is authoritative, but the app's mutable per-user state lives at
`%LOCALAPPDATA%\Packages\<PackageFamilyName>`, whose folder name is the package
*identity* (not the display name) and is therefore missed by name-token
heuristics. `appx_identity_name` reads the `Identity` `Name` from the package's
`AppxManifest.xml` (a targeted attribute scan, no XML dependency), and
`msix_package_data_roots` adds the matching `…\Packages\<IdentityName>_*`
directory as a root — a prefix match that avoids computing the publisher hash.
MSIX apps are still managed by Windows and normally need reinstallation.

### Squirrel

Detected from the per-user `\AppData\Local\…` layout with `Update.exe` or an
`app-*` subdirectory. The install location is added as an authoritative root and
`latest_squirrel_app_dir` records the current `app-<version>` directory (versions
compared by numeric segments so `app-1.0.10` ranks above `app-1.0.9`).

### Chocolatey

Detected (method + evidence); the heuristic layer captures its directories.

### NSIS / generic EXE

NSIS keeps **no** file manifest by default (its uninstaller has hard-coded
delete logic). When the optional `AdvUninstLog` macro is used it writes a text
`uninstall.log` of installed paths in the install directory; `find_nsis_log` /
`nsis_manifest_files` read it with the same path-string scan used for Inno
(`extract_utf16_paths` / `extract_ansi_paths`, keeping paths that exist), so those
installs get an authoritative file count and roots. When the log is absent the
**install directory** (from `InstallLocation` / App Paths / uninstaller directory —
an authoritative *location*) plus the heuristic layer is the basis.

## Layer 3: Heuristic footprint

`file_discovery::discover_files` walks a set of `DiscoveryRoot`s and produces the
file inventory plus a `FootprintLocation` summary. Roots come from:

- The install location and executable directory.
- `extra_roots` passed by `scan.rs`: service binary dirs, scheduled-task action
  dirs, app shortcut targets, the uninstaller directory, App Paths dirs, and the
  roots from layer 2 (MSI, Inno, NSIS, Squirrel, MSIX).
- Token-matched directories under `Program Files`, `Program Files (x86)`,
  `ProgramW6432`, `ProgramData`, every readable user profile's
  `AppData\{Roaming,Local,LocalLow}`, the current user's `Documents` (including
  `My Games\<App>`) and `Saved Games`, and Public Documents.

### Matching

`collect_matching_dirs` matches directory names at **depth 1 and depth 2**
(`base\App` and `base\Vendor\App`) — essential for nested layouts like
`ProgramData\Apple Computer\iTunes`. `Tokens` are derived from the app name
(version/parenthetical stripped via `primary_name`), publisher, executable stem,
and install dir leaf/parent.

A directory matches a token when they are equal, when the directory's name contains a
token of 4 or more characters, or when a token contains a directory name of 5 or more
characters. Guards:

- `GENERIC_TOKENS` stoplist (e.g. `x64`, `files`, `bin`, `programfilesx86`).
- Length floors, so a versioned token like `7zip2501x64` cannot match an `x64` directory.
- Volatile directories (`Temp`, `Cache`, `*cache`, crash dumps, logs) are skipped
  during matching and pruned during the walk, including inside install directories.
- Roots nested inside an already-kept root are dropped; `Windows`/system bases
  are never walked.

**These guards are not enough.** A token like `Mozilla` pulls in Thunderbird for Firefox, and
the name "Microsoft Visual Studio Code" contains the folder name "Microsoft Visual Studio".
Because a match under Program Files counts as app-owned, it is copied with no option to
exclude it. See [Known limitations](#known-limitations).

### Classification

`FileClassification` per file:

- `AppOwned` — under an install directory (copied).
- `UserData` — genuine **per-user** data: Roaming/Local/LocalLow AppData,
  `Documents` (including nested `My Games\<App>`), and `Saved Games`. Copied
  when `include_user_data`.
- `MachineAppData` — **machine-wide** `ProgramData` and Public Documents data shared by
  all users. This is deliberately distinct from per-user data and is copied
  under its own `include_program_data` toggle.
- `ManualRuntimeInstall` — `.sys` driver files. Not copied as plain files; an
  app-owned driver that backs a registered driver service is instead bundled and
  re-registered via the driver-service path, while vendor/inbox drivers get
  precise reinstall guidance.
- `WindowsSystem` / others — reported, not copied.

Per-user `Documents`/`Saved Games` are resolved through the `User Shell Folders`
registry key first (so OneDrive/folder-redirection targets are followed) before
falling back to the profile path. **Online-only cloud placeholders** (OneDrive
Files On-Demand, detected via the `OFFLINE`/`RECALL_ON_DATA_ACCESS` file
attributes) are reported but never copied, so a backup cannot silently hydrate
or fail on files whose data is not present locally.

`FootprintLocation { path, kind, scope, file_count, total_bytes,
copyable_bytes, included }` groups the inventory by root for the report.
`FootprintKind` ∈ `InstallDirectory`, `ProgramData`, `RoamingAppData`,
`LocalAppData`, `LocalLowAppData`, `Documents`, `SavedGames`, `PublicDocuments`,
`RegistryReference`, `Other`.

## Registry, services, and other integrations

- `registry_scan.rs` captures values under the app's keys, COM registrations and file
  associations that point at the app, Run keys, and App Paths. Keys are matched by app and
  publisher name, as substrings. Every value under a matched key is captured, whatever its
  name, so a key's license or token values come along even with the license option off.
- `service_scan.rs` captures services and drivers whose image is the main executable or
  under the install location.
- `task_scan.rs` captures scheduled tasks whose actions point at the app, through
  PowerShell on a live scan and from the Task XML files on a drive.
- `shortcut_scan.rs` captures Start Menu and Desktop shortcuts on a live scan.
- `env_scan.rs` captures environment variables whose value contains the app's path. A
  captured `Path` is merged on restore: only entries under the install location are added
  to the target's value, which is otherwise left alone.

Not captured: URL protocols, shell extensions, TypeLib and AppID registrations, firewall
rules, fonts, certificates, ODBC sources, and event log sources.

## Options

- `ScanOptions.include_user_data` / `PackageOptions.include_user_data` — when false,
  genuine per-user data (Roaming/Local/LocalLow AppData, Documents, Saved Games) is
  discovered and reported but not copied. Set from the app's "User data" option, which is
  off by default. `for_executable` and the serde default set it to `true`, for library
  callers.
- `ScanOptions.include_program_data` / `PackageOptions.include_program_data` — controls
  machine-wide `ProgramData` separately from per-user data. The app drives it from the
  same option.
- `ScanOptions.include_hashes` — SHA-256 of copyable files. Packaging always sets it. It
  dominates scan time for large apps.

## Runtime prerequisites

`scan::runtime_prerequisites_from_signals` infers the runtimes an app needs from
its imports, app-local DLLs, and footprint, and `prerequisites.rs` resolves each
to a concrete on-target plan (bundled installer, winget, or a Windows optional
feature), verified by a registry probe where one exists.

The Visual C++ redistributable is downloaded at build time and embedded. If that download
fails, the build warns and the installer tries winget on the target. The detection is
coarse: every VC++ version maps to the 2015+ redistributable, and the `msvcr`/`msvcp`
prefixes also match Windows' own `msvcrt.dll`.

.NET is detected by reading the managed image's **CLR metadata-root version**
(`pe_scan::dotnet_metadata_version`, the `BSJB` root's version string):

- `mscoree`/`clr` host + CLR `v1.x`/`v2.0` → **.NET Framework 3.5**
  (`DotNetFramework35`), installed by enabling the **`NetFx3` Windows optional
  feature via DISM** (`/online /enable-feature /featurename:NetFx3 /all`). The
  payload comes from Windows Update or install media; on failure the installer
  gives an actionable fallback message.
- `mscoree` host + CLR `v4.0` → **.NET Framework 4.x** (`DotNetFramework4x`),
  in-box on Windows 10/11 — listed and probed, never installed.
- `hostfxr`/`coreclr` → self-contained modern .NET (ships app-local).
- Otherwise a managed image → modern **.NET Desktop Runtime**, installed on the target
  through winget (`Microsoft.DotNet.DesktopRuntime.8`, whatever version the app needs), so
  it needs internet there. It has no presence probe.

## Scanning an offline volume (a disk from another PC)

Discovery normally reads the live OS through two seams: the live registry
(`RegKey::predef(HKLM/HKCU)`) and known folders (`std::env::var_os(…)` +
`path.exists()`). `system_context.rs` funnels both through one `SystemContext`
so the same scanners work against a **mounted disk from another machine** (a
failing PC). `ScanSource` is `Live` or `OfflineVolume { volume_root }` (e.g.
`E:\`), threaded via `ScanOptions`/`PackageOptions`.

For an offline volume, `SystemContext::open`:

- **Loads the disk's hives** with `reg load HKLM\WAM_<pid>_… <hive>` for `SOFTWARE`,
  `SYSTEM`, and each `Users\<name>\NTUSER.DAT`. `reg load` uses the process's
  backup/restore privileges (present when elevated), so the ACL on a foreign `config` hive
  doesn't block it. A drop guard `reg unload`s every mount; logical `HKLM\…` paths are
  mapped onto the mounts, resolving `SYSTEM\CurrentControlSet` to the disk's active set
  from `SYSTEM\Select\Current` (there is no live `CurrentControlSet` symlink offline).
- **Derives known-folder bases** (`Program Files`, `ProgramData`, `Public`,
  `Windows`, and every `Users\*` profile) from `volume_root` instead of the live
  environment, and enumerates **all** user hives (not just the current user).
- **Re-roots** absolute source paths — a registry-recorded `C:\Program Files\App`
  becomes `<volume_root>\Program Files\App` (`resolve_source_path`).

Scanner coverage offline: app inventory, registry, environment, license, and
install-source read the mounted hives; services/drivers read the offline `SYSTEM`
hive; **scheduled tasks are parsed from the disk's `Windows\System32\Tasks` XML**
(the same schema `Export-ScheduledTask` emits) because the live Task Scheduler
can't see another disk. Prerequisite *presence* on the source is left unknown
offline (the `is_present` probe reads the live registry).

**Requirements and safety:** offline scanning requires **elevation** (for `reg load` and
backup reads). The app writes nothing to the drive and never changes ownership or
permissions (no `takeown`/`icacls`); files whose ACL denies a plain read are copied at
package time via `robocopy /B` (backup semantics). Note that `reg load` is not a read-only
mount: when Windows loads a hive it may replay the hive's transaction logs, which can
write to it. **For a failing drive, image it first and scan the image.**

Restore re-anchoring is unaffected: user-data paths still split on their
`\appdata\roaming\`-style markers, so data re-homes to the *target's* `%APPDATA%`
regardless of the source drive letter.

## Restoring user data to real locations

Run headlessly, program and user-data files (including the `files\_external\…` data staged
from outside the install directory) land under one `target_root` unless
`--restore-user-data` is passed. Captured license files keep their original paths unless
they were among the copied files, and registry values, environment variables, and shortcuts
always go to their real locations. The installer's window asks instead: its **User data**
option is on by default whenever data was captured. Turning it off still writes the data,
under the install folder.

Opting in, with `--restore-user-data` or that option, re-anchors each user-data file to
the **target machine's** equivalent base, derived from its archived source path:

| Source anchor | Target base |
| --- | --- |
| `…\AppData\Roaming\…` | `%APPDATA%` |
| `…\AppData\Local\…` | `%LOCALAPPDATA%` |
| `…\AppData\LocalLow\…` | `%USERPROFILE%\AppData\LocalLow` |
| `…\ProgramData\…` | `%ProgramData%` |
| `…\Users\Public\…` | `%PUBLIC%` |

Everything else stays under `target_root`, including **Documents and Saved Games**, which
land in `<target_root>\_external\…`. App-owned install-directory files always restore under
`target_root`. Routing is centralized in `file_discovery::restore_destination_for_entry`
(with `split_user_data_relative` doing the anchor split) so restore and verification
resolve identical destinations.

The profile name is discarded, so data captured from several accounts lands in the one
running the installer, and when the same file exists in two accounts the last one wins.
Elevating with a different administrator account means that account's `%APPDATA%`.

Safety: ProgramData (and other machine-wide protected roots) require elevation on apply —
`privilege::destination_requires_admin` gates this; each re-anchored destination is bounded
to its base by `ensure_within_root`; headless restore overwrites existing files only with
`--force`, while the installer's window always overwrites and warns first if the install
folder has files; and the built-in verification runs with the same user-data setting so it
checks the real locations.

## Known limitations

Full list, with the planned fixes, in [PLAN.md](PLAN.md#open-issues-most-important-first).
The ones that most affect what gets captured:

- **False positives.** App, publisher, and parent-folder tokens with two-way substring
  matching pull in other apps. Measured on one PC: VS Code captured 175,176 files, including
  `Program Files\Microsoft Visual Studio` (11.1 GB); Firefox pulled in Thunderbird; NVIDIA
  App pulled in `Razer\Installer`. Registry matching is also substring-based: `Git` matched
  `Software\Logitech`.
- **Wrong main executable.** The guess from the uninstall entry can land on an uninstaller
  (`unins000.exe`) or a Windows host (`rundll32.exe`), which skews architecture and import
  analysis, and can pull in unrelated registry entries.
- **Offline path mismatches.** Registry values hold the source's `C:\` paths but are
  compared with paths re-rooted to the drive (`E:\`), so COM registrations, file
  associations, Run keys, App Paths, and environment variables do not match. Shortcuts are
  not read from a drive, so such an installer recreates none, and the installer's default
  location starts with the drive's letter. `UsrClass.dat` is not loaded, so per-user classes
  are missed. Offline known-folder redirection (OneDrive Documents, etc.) uses the default
  profile-relative folders rather than each `NTUSER.DAT`'s `User Shell Folders`.
- **Size.** Windows cannot start an executable of 4 GiB or more, so a larger package fails
  the build with that reason.
