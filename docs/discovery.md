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

> Performance: iterating all components is O(installed components on the
> machine). On a heavy developer machine (~96k components) this is ~18s; a normal
> machine has far fewer. It runs on the GUI background thread. A future
> optimization is to scope iteration to the product's components via
> `Products\<squished>\Features` (requires decoding the Base85 packed GUIDs).

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

### MSIX / Squirrel / Chocolatey

Currently **detected** (method + evidence) but not deeply enumerated. The
heuristic layer still captures their directories. Deeper support (AppxManifest,
`%LOCALAPPDATA%\Packages\<PFN>`, Squirrel `app-*`) is a future enhancement.

### NSIS / generic EXE

NSIS keeps **no** file manifest by default (its uninstaller has hard-coded
delete logic). Only the optional `AdvUninstLog` writes a text `uninstall.log`.
For these installers the **install directory** (from `InstallLocation` /
App Paths / uninstaller directory — an authoritative *location*) plus the
heuristic layer is the basis; there is no per-file manifest to read.

## Layer 3: Heuristic footprint

`file_discovery::discover_files` walks a set of `DiscoveryRoot`s and produces the
file inventory plus a `FootprintLocation` summary. Roots come from:

- The install location and executable directory.
- `extra_roots` passed by `scan.rs`: service binary dirs, scheduled-task action
  dirs, app shortcut targets, the uninstaller directory, App Paths dirs, and the
  authoritative MSI/Inno roots.
- Token-matched directories under `Program Files`, `Program Files (x86)`,
  `ProgramW6432`, `ProgramData`, and every user profile's
  `AppData\{Roaming,Local,LocalLow}`.

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
- `UserData` — under ProgramData/AppData (copied when `include_user_data`).
- `ManualRuntimeInstall` — `.sys` drivers (never copied).
- `WindowsSystem` / others — reported, not copied.

`FootprintLocation { path, kind, scope, file_count, total_bytes,
copyable_bytes, included }` groups the inventory by root for display.
`FootprintKind` ∈ `InstallDirectory`, `ProgramData`, `RoamingAppData`,
`LocalAppData`, `LocalLowAppData`, `RegistryReference`, `Other`.

## Options

- `ScanOptions.include_user_data` / `PackageOptions.include_user_data`
  (default `true`) — when false, ProgramData/AppData files are discovered and
  reported but not copied. CLI: `--skip-user-data` on `scan` and `package`.
- `ScanOptions.include_hashes` — SHA-256 of copyable files. This dominates scan
  time for large apps; a GUI toggle exists ("Compute SHA-256 hashes").

## Known limitations / next steps

- Scope the MSI component sweep to the product (Features/Base85) to cut latency.
- Parse NSIS `AdvUninstLog` `uninstall.log` when present.
- Deeper MSIX (`AppxManifest`, package data) and Squirrel (`app-*`) enumeration.
- Restore currently lands all files under one `target_root` (user-data files go
  to `target_root\_external\...`); restoring user data back to the real
  `%APPDATA%`/`%ProgramData%` on the target is a future enhancement.
