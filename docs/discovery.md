# Application Footprint Discovery

This document describes how WinAppMigrator reverse-engineers an installed
application's footprint. The goal is to be **authoritative where Windows records
the truth** and **conservatively heuristic only where it does not**.

## Layered approach

For a selected installed application the scan runs three layers and unions the
results:

1. **Install-method detection** — classify *how* the app was installed so the
   right authoritative source can be consulted.
2. **Authoritative source** — read the exact installed paths from the installer
   database when one exists (MSI, Inno Setup), plus App Paths registration.
3. **Heuristic footprint** — discover the install/data directories that the
   above did not cover, especially per-user/per-machine data the installer never
   records (created at runtime).

Code:

- `crates/winappmigrator-core/src/install_source.rs` — layers 1 and 2.
- `crates/winappmigrator-core/src/file_discovery.rs` — layer 3 plus the unified
  walk, classification, and the `FootprintLocation` summary.
- `crates/winappmigrator-core/src/scan.rs` — orchestrates the layers, folds the
  install-source roots into discovery, and records the result on `ScanReport`.

## Layer 1: Install-method detection

`install_source::detect_install_source(&InstalledApp)` returns an
`InstallSource { method, product_code, authoritative_file_count,
authoritative_registry_count, roots, evidence }`. The `InstallMethod` is one of:

| Method | Signal |
| --- | --- |
| `WindowsInstaller` (MSI) | Uninstall key name is a product-code GUID, `UninstallString` contains `msiexec`, or a `{GUID}` is present |
| `Msix` | Install location under `\WindowsApps\` |
| `Squirrel` | Install location under `\AppData\Local\` with `Update.exe` or an `app-*` subdir |
| `Chocolatey` | Install location/uninstall under `\chocolatey\lib\` |
| `InnoSetup` | A `unins###.dat` log exists in the install directory |
| `Nsis` | The uninstaller binary contains the `Nullsoft` signature |
| `Generic` | None of the above |

`evidence` is a human-readable list of the signals that were used; it is shown
in the GUI ("How this was reverse-engineered") and the Markdown report.

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
- `msi_component_paths` iterates every component subkey across all user SIDs and
  reads the squished-product value; file key paths are collected, registry key
  paths (encoded `NN:\...`) are counted.
- The file paths are collapsed to top-level install roots via
  `minimal_install_roots` (`install_root_for_file` resolves the first segment
  under a known base, e.g. `Program Files\Inkscape`).

This matches the official `WindowsInstaller.Installer` COM `ComponentPath`
results exactly (validated against Inkscape: 11,657 paths; iTunes: 4,408,
including out-of-install-dir `Common Files\Apple` and `ProgramData\Apple
Computer`).

> Performance: the sweep is narrowed to the SIDs that actually register the
> product (`component_sweep_sids`: a component only records a product's key path
> under the SID where the product is registered, so other per-user component
> trees are skipped on multi-user machines), with a full-SID fallback if none is
> found. Within a registering SID it still iterates that SID's components
> (O(components-in-SID)); on a heavy developer machine (~96k machine-wide
> components under `S-1-5-18`) this is ~18s, and it runs on the GUI background
> thread.
>
> Fully scoping iteration to **one product's** components would require either
> decoding the `Products\<squished>\Features` feature descriptors or calling the
> Windows Installer API (`MsiGetComponentPathW`). This is **deliberately not
> done**: a mis-decoded feature descriptor would silently *under-report* the
> migration footprint, and the exhaustive sweep is the source that was validated
> against the `WindowsInstaller.Installer` COM `ComponentPath` results. Revisit
> only with the MSI API plus a cross-check against the exhaustive sweep, which
> would add a Windows FFI dependency the project currently avoids.

### Inno Setup

Inno Setup records every installed file/dir/registry key in an uninstall log
`unins000.dat` in the install directory, as **resolved absolute paths** (UTF-16
in modern Unicode logs, ANSI in old ones).

- `find_inno_log` locates a `unins###.dat` (deliberately not a plain
  `uninstall.exe`, which other installers use).
- `inno_manifest_files` verifies the `Inno Setup Uninstall Log` signature, then
  extracts path strings directly (`extract_utf16_paths` / `extract_ansi_paths`)
  rather than decoding Inno's versioned record framing. This is
  version-independent and robust.
- Shortcut/Start Menu/system paths are filtered (`is_app_footprint_path`) before
  deriving roots so a single `.lnk` does not turn a shared folder into a root.

Validated: Kernel OST Viewer → "Inno Setup uninstall log: 9 recorded paths".

### App Paths

`app_paths_roots` reads
`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\<exe>` (64- and 32-bit
views) for the registered launcher path and the `Path` DLL search directories.
Used for every method.

### MSIX

Detected when the install location is under `\WindowsApps\`. The install payload
there is authoritative, but the app's mutable per-user state lives at
`%LOCALAPPDATA%\Packages\<PackageFamilyName>`, whose folder name is the package
*identity* (not the display name) and is therefore missed by name-token
heuristics. `appx_identity_name` reads the `Identity` `Name` from the package's
`AppxManifest.xml` (a targeted attribute scan, no XML dependency), and
`msix_package_data_roots` adds the matching `…\Packages\<IdentityName>_*`
directory as a root — a prefix match that avoids computing the publisher hash.
MSIX apps are still managed by Windows and normally need reinstallation, but
their per-user state is now captured for backup.

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
`nsis_manifest_files` read it with the same robust path-string scan used for
Inno (`extract_utf16_paths` / `extract_ansi_paths`, keeping paths that exist),
so those installs get an authoritative file count and roots. When the log is
absent the **install directory** (from `InstallLocation` / App Paths /
uninstaller directory — an authoritative *location*) plus the heuristic layer is
the basis.

## Layer 3: Heuristic footprint

`file_discovery::discover_files` walks a set of `DiscoveryRoot`s and produces the
file inventory plus a `FootprintLocation` summary. Roots come from:

- The install location and executable directory.
- `extra_roots` passed by `scan.rs`: service binary dirs, scheduled-task action
  dirs, app shortcut targets, the uninstaller directory, App Paths dirs, and the
  authoritative MSI/Inno roots.
- Token-matched directories under `Program Files`, `Program Files (x86)`,
  `ProgramW6432`, `ProgramData`, every user profile's
  `AppData\{Roaming,Local,LocalLow}`, and the current user's `Documents`
  (including `My Games\<App>`) and `Saved Games`.

### Matching (avoiding false positives)

`collect_matching_dirs` matches directory names at **depth 1 and depth 2**
(`base\App` and `base\Vendor\App`) — essential for nested layouts like
`ProgramData\Apple Computer\iTunes`. `Tokens` are derived from the app name
(version/parenthetical stripped via `primary_name`), publisher, executable stem,
and install dir leaf/parent. Guards:

- `GENERIC_TOKENS` stoplist (e.g. `x64`, `files`, `bin`, `programfilesx86`).
- Length floors and a one-directional `contains` rule so a versioned token like
  `7zip2501x64` cannot match an `x64` directory.
- Volatile directories (`Temp`, `Cache`, `*cache`, crash dumps, logs) are skipped
  during matching and pruned during the walk.
- Roots nested inside an already-kept root are dropped; `Windows`/system bases
  are never walked.

### Classification

`FileClassification` per file:

- `AppOwned` — under an install directory (copied).
- `UserData` — genuine **per-user** data: Roaming/Local/LocalLow AppData,
  `Documents` (including nested `My Games\<App>`), and `Saved Games`. Copied
  when `include_user_data`.
- `MachineAppData` — **machine-wide** `ProgramData` application data shared by
  all users. This is deliberately distinct from per-user data and is copied
  under its own `include_program_data` toggle.
- `ManualRuntimeInstall` — `.sys` drivers (never copied).
- `WindowsSystem` / others — reported, not copied.

Per-user `Documents`/`Saved Games` are resolved through the `User Shell Folders`
registry key first (so OneDrive/folder-redirection targets are followed) before
falling back to the profile path. **Online-only cloud placeholders** (OneDrive
Files On-Demand, detected via the `OFFLINE`/`RECALL_ON_DATA_ACCESS` file
attributes) are reported but never copied, so a backup cannot silently hydrate
or fail on files whose data is not present locally.

`FootprintLocation { path, kind, scope, file_count, total_bytes,
copyable_bytes, included }` groups the inventory by root for display.
`FootprintKind` ∈ `InstallDirectory`, `ProgramData`, `RoamingAppData`,
`LocalAppData`, `LocalLowAppData`, `RegistryReference`, `Other`.

## Options

- `ScanOptions.include_user_data` / `PackageOptions.include_user_data`
  (default `true`) — when false, genuine per-user data (Roaming/Local/LocalLow
  AppData, Documents, Saved Games) is discovered and reported but not copied.
  CLI: `--skip-user-data` on `scan` and `package`.
- `ScanOptions.include_program_data` / `PackageOptions.include_program_data`
  (default `true`) — controls machine-wide `ProgramData` separately from
  per-user data. CLI: `--skip-program-data` on `scan` and `package`.
- `ScanOptions.include_hashes` — SHA-256 of copyable files. This dominates scan
  time for large apps; a GUI toggle exists ("Compute SHA-256 hashes").

## Known limitations / next steps

- Scope the MSI component sweep to the product (Features/Base85) to cut latency.
- Deeper MSIX (`AppxManifest`, package data) and Squirrel (`app-*`) enumeration.

## Restoring user data to real locations

By default restore is conservative: every archived file (including the
`files\_external\…` user-data staged from outside the install directory) lands
under one `target_root`, so a restore never touches the live profile unless asked.

Opting in with `--restore-user-data` (CLI `restore`/SFX, or the "Restore user
data to real AppData/ProgramData" checkbox in the GUI) re-anchors each
user-data file to the **target machine's** equivalent base, derived from its
archived source path:

| Source anchor | Target base |
| --- | --- |
| `…\AppData\Roaming\…` | `%APPDATA%` |
| `…\AppData\Local\…` | `%LOCALAPPDATA%` |
| `…\AppData\LocalLow\…` | `%USERPROFILE%\AppData\LocalLow` |
| `…\ProgramData\…` | `%ProgramData%` |

App-owned install-directory files always restore under `target_root` regardless
of the flag. Routing is centralized in
`file_discovery::restore_destination_for_entry` (with `split_user_data_relative`
doing the anchor split) so restore and verification resolve identical
destinations. Files that cannot be re-anchored fall back to the sandbox path.

Safety: ProgramData (and other machine-wide protected roots) require an elevated
shell on apply — `privilege::destination_requires_admin` gates this; each
re-anchored destination is still bounded to its base by `ensure_within_root`;
existing files are only overwritten with `--force`; and `verify` must be passed
the same `--restore-user-data` value so it checks the real locations.
