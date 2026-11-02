# Application Footprint Discovery

This document describes how WinAppMigrator reverse-engineers an installed
application's footprint. The goal is to be **authoritative where Windows records
the truth**, and heuristic where it does not. The heuristics match whole names only, so
they can miss a folder the app keeps under another name but should not take another
product. See [Known limitations](#known-limitations).

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
- `crates/winappmigrator-core/src/product_match.rs`: the name rules layer 3 and the
  registry scan share.
- `crates/winappmigrator-core/src/scan.rs` — orchestrates the layers, folds the
  install-source roots into discovery, and records the result on `ScanReport`.

## The application list

`app_inventory.rs` reads the Uninstall keys of HKLM (64- and 32-bit views) and HKCU.
Entries with no display name are skipped, and so are entries marked `SystemComponent=1`,
as in Windows' own installed-apps list: those are parts of other products. Microsoft
Store apps have no Uninstall entry, so they are not listed.

An `InstallLocation` that is itself a shared folder, such as a drive, `C:\Program Files`, or
`AppData\Local\Programs`, is ignored: taken at its word it would make everything in there
part of the app. Trailing separators are removed.

## Layer 1: Install-method detection

`install_source::detect_install_source(&InstalledApp, &SystemContext)` returns an
`InstallSource { method, product_code, authoritative_file_count, roots, loose_files,
recorded_files, registry_keys, evidence }`. The scan runs it once and passes the result
to whatever needs it. The `InstallMethod` is one of:

| Method | Signal |
| --- | --- |
| `WindowsInstaller` (MSI) | Uninstall key name is a product-code GUID, `UninstallString` contains `msiexec`, or a `{GUID}` is present |
| `Msix` | Install location under `\WindowsApps\` |
| `Squirrel` | Install location under `\AppData\Local\` with `Update.exe` or an `app-*` subdir |
| `Chocolatey` | Install location under `\chocolatey\lib\`, or an uninstall string under `\chocolatey\` |
| `InnoSetup` | A `unins###.dat` log exists beside the uninstaller or in the install directory |
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
  reads the squished-product value. File key paths are collected. Registry key paths
  (encoded `NN:\...`) become exact keys and values (`msi_registry_key`): the ones digit is
  the root, and the tens digit marks the 64-bit view. The registry scan captures them
  whatever they are called.
- The file paths reduce to install roots as described in
  [Install roots](#install-roots-from-recorded-files).
- A Burn bundle, such as the visible `PowerShell 7.6.9.0-x64` or `Microsoft .NET SDK` entry,
  has a code of its own that owns no files: its hidden MSI packages do. `msi_products_of`
  finds them under `Software\Classes\Installer\Dependencies`, where each package's
  `Dependents` key names the bundles that use it, and they are all read in the same pass.
  A runtime the bundle chained (`.NET`, Visual C++, WebView2) stays out and is reported as
  a prerequisite, unless the app is that runtime.

The key paths match the official `WindowsInstaller.Installer` COM `ComponentPath`
results (validated against Inkscape: 11,657 paths; iTunes: 4,408, including
out-of-install-dir `Common Files\Apple` and `ProgramData\Apple Computer`).

> Performance: the sweep is narrowed to the SIDs that actually register the
> product (`component_sweep_sids`: a component only records a product's key path
> under the SID where the product is registered, so other per-user component
> trees are skipped on multi-user machines), with a full-SID fallback if none is
> found. Within a registering SID it still iterates that SID's components, once per
> scan. On a heavy developer machine with about 185,000 components that takes several
> seconds, on a background thread. The result is kept in `InstallSource`, so choosing the
> main executable does not read the database again.
>
> Fully scoping iteration to **one product's** components would require either
> decoding the `Products\<squished>\Features` feature descriptors or calling the
> Windows Installer API (`MsiGetComponentPathW`). This is **deliberately not
> done**: a mis-decoded feature descriptor would silently *under-report* the
> migration footprint, and the exhaustive sweep is the source that was validated
> against COM. Revisit only with the MSI API plus a cross-check against the sweep.

### Inno Setup

Inno Setup records every installed file, directory, and registry key in an uninstall log
`unins000.dat` beside the uninstaller. That is usually the install directory, though GIMP
keeps it in `uninst`, so `inno_log_for` looks in both places.

`parse_inno_log` reads the log's real structure. After a 448-byte header come CRC-framed
blocks, each starting with its size and the size's complement. Their bodies, joined, are
records: a 16-bit type, 32-bit extra data, a 32-bit size, then a list of strings ending in
`0xFF`. Each string has a length prefix of one byte, or `0xFD` and 16 bits, or `0xFE` and 32
bits. A negative length is a byte count of UTF-16 text, and a positive one is ANSI, so the
same parser reads Unicode logs and old ANSI ones.

- `utDeleteFile` records (`0x82`) give the files. Only those still on disk count: a log
  grows with every update, and on one PC VS Code's held 272,656 file records for 6,443
  distinct files. Long paths lose their `\\?\` prefix.
- Registry records give exact keys (`0x86`, `0x88`) and single values (`0x87`, `0x89`),
  and their extra data gives the root and the view. This is how Git for Windows'
  `Software\GitForWindows` is found, whose name is not the product's.
- Shortcut, Start Menu, Desktop, and Windows paths are filtered
  (`is_app_footprint_path`) before deriving roots.

Reading the lengths matters. On a Unicode log the `0xFF` after a path and the next record's
type read together as one UTF-16 unit, such as U+82FF, a CJK letter that looks like path
text, which defeated scanning for strings. A log whose framing is not recognized still
falls back to that scan. It can miss paths but cannot invent one, since a path has to exist
to count.

### App Paths

`app_paths_roots` reads
`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\<exe>` (64- and 32-bit
views) for the registered launcher path and the `Path` DLL search directories.
Used for every method.

### Install roots from recorded files

`minimal_install_roots` reduces the files an install record names (MSI, Inno, NSIS) to the
deepest folders that hold them, so every file under `C:\Program Files\Inkscape\...` gives
that one folder. It stops short of anything that holds unrelated things side by side:

- A shared folder (`Program Files`, `Common Files`, `ProgramData`, `AppData`,
  `AppData\Local\Programs`, a user profile, `Microsoft`, and so on) is never a root. Files
  under one split into a root per child. One app's file under `ProgramData\Microsoft` no
  longer claims all of it.
- A folder that only joins subfolders splits the same way, unless it is named for the
  product. With no recorded file directly in it, it may be a vendor's folder or a family of
  products: the ODBC driver's files share `Microsoft SQL Server` with LocalDB.
- A root ending in a folder that holds executables, like `bin`, moves up one level, unless
  that would reach a shared folder or one named only for the vendor (`NVIDIA Corporation`,
  per `is_vendor_only_folder`).
- A folder directly on a drive, like `C:\CocosDashboard`, is a root like any other. The
  drive itself is not.
- A recorded file with no such folder, for example directly under `Common Files`, is kept
  in `loose_files` and captured by itself.

### Choosing the main executable

The list guesses from the uninstall entry (`guess_main_executable`): its icon, then App
Paths and Start Menu shortcuts, then the install folder, then the uninstall command. If that
finds nothing, the scan calls `primary_executable`: the same App Paths and shortcuts, then
the files the install record names, then the install folders. Throughout, a candidate has
to exist and is rejected if it is an uninstaller, updater, or crash handler
(`is_auxiliary_executable`), under the Windows directory, or an installer's cached copy of
itself (`Package Cache`, `Caphyon`).

- App Paths and shortcuts count when they are named for the product or point inside its
  install folder. Shortcut targets are read from the `.lnk` file directly
  (`lnk_target_path`), which also works on a connected drive.
- Chrome depends on this. Its MSI records only a registry value and its uninstall entry
  names no location, so nothing else says where it is.
- For a Squirrel app, the stub in the app's folder gives way to the same-named executable
  in the newest `app-<version>`.
- Side-by-side versions share a name, so a shortcut carrying the same version is preferred:
  `Python 3.14 (64-bit)` for `Python 3.14.10`.
- When only an uninstaller or a cached installer exists, as with a font pack or a runtime,
  that stands in as a last resort, and the scan warns that architecture and dependency
  results describe it and not the app. A Windows host like `rundll32.exe` never does, though
  an executable the app's own install record puts in the Windows directory can (the Python
  launcher's `py.exe`). With no candidate at all, the scan fails and says so: on one PC, that
  is drivers and add-ins such as `NVIDIA PhysX System Software`.

`resolve_install_location` then picks the folder the app's files are stored relative to,
which is also what the installer offers: the uninstall entry's location if the executable
is in it, otherwise the nearest folder above the executable named for the product
(`Google\Chrome` for `Chrome\Application\chrome.exe`), otherwise the shallowest recorded
root holding it, otherwise its own folder, moving up from a name like `bin`. It is never a
shared folder, so `C:\Windows\py.exe` has no install location. An uninstall entry's location
that does not hold the executable is still captured.

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
`nsis_manifest_files` read it with a path-string scan (`extract_utf16_paths` /
`extract_ansi_paths`, keeping paths that exist), so those
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
- Directories named for the product under `Program Files`, `Program Files (x86)`,
  `ProgramW6432`, `ProgramData`, the `AppData\{Roaming,Local,LocalLow}` of the account being
  scanned, its `Documents` (including `My Games\<App>`) and `Saved Games`, and Public Documents.

### Matching

`collect_matching_dirs` matches directory names at **depth 1 and depth 2**
(`base\App` and `base\Vendor\App`), which nested layouts like
`ProgramData\Apple Computer\iTunes` need. `ProductNames` holds the names an app goes by,
normalized: its display name with and without the version (`7-Zip 25.01 (x64)` and
`PowerShell 7.6.9.0-x64` reduce to `7-Zip` and `PowerShell`), the display name without its
publisher, the executable's name, and the install folder's name.

Matching is on the whole name, never a substring: `Git` is not `Logitech` or `GitHub`, and
`Microsoft Visual Studio Code` is not `Microsoft Visual Studio`. A folder name may carry a
version of its own (`Adobe Photoshop 2024`, `Python311`).

- **At depth 1**, a folder matches on the display name, the executable's name, or the install
  folder's name. `AppData\Roaming\Code` is VS Code's because of `Code.exe`.
- **At depth 2**, a folder matches when its parent and its own name spell the product
  (`Mozilla\Firefox`, `Google\Chrome`), when it carries a product name under its vendor's
  folder or a plain container (`NVIDIA Corporation\NVIDIA app`, `Apple Computer\iTunes`,
  `Programs\Git`), or when it is a multi-word display name under any parent.
- **The publisher only qualifies a parent.** It never selects a folder itself, so `Mozilla`
  alone does not take Thunderbird. A parent qualifies when its first word is the publisher's
  (`Apple Computer` for `Apple Inc.`) or when it is the install folder's parent.
- **The install folder's parent is never a match by itself.** It used to be, which is how an
  app in `AppData\Local\Programs` took everything else installed there.
- **`Documents` and `Saved Games` only match the display name**, and it must be at least four
  characters, because people name those folders themselves. `Code.exe` does not claim
  `Documents\Code`.
- A name is ignored when it is too generic to identify anything (`GENERIC_NAMES`: `bin`,
  `x64`, `launcher`, `installer`).

A folder that refuses to be listed is reported, unless it is a link. Those are the junctions
Windows keeps for old programs (`Application Data`, `Documents and Settings`, `My Music`), which
refuse everyone on purpose and hold nothing of an app's.

Volatile directories (`Temp`, `Cache`, `*cache`, crash dumps, logs) are skipped during
matching and pruned when walking a data folder. They are not pruned inside an install folder:
the same names there can be files the app shipped with, like VS Code's
`resources\app\node-compile-cache`, and an installer missing them is silently broken.

Roots nested inside an already-kept root are dropped. A root that is a known base, a drive,
or inside the Windows directory is never walked, but a folder merely named `windows` inside
an app's own tree is.

### Classification

`FileClassification` per file:

- `AppOwned`: under an install directory, or a folder of its own that an install record
  points at, like `C:\CocosDashboard` (copied).
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

Per-user `Documents`/`Saved Games` are resolved through that account's `User Shell Folders`
registry key first (so OneDrive/folder-redirection targets are followed) before
falling back to the profile path. A variable in that value means what it means for that account,
and not what it means to this process. A file in one records which of the two it is under and
where (`FileEntry::user_folder`), since a redirected folder's path says nothing about that.
**Online-only cloud placeholders** (OneDrive Files On-Demand, detected via the
`OFFLINE`/`RECALL_ON_DATA_ACCESS` file attributes) are reported but never copied, so a backup
cannot silently hydrate or fail on files whose data is not present locally.

## Whose data

A scan reads one account (`ScanOptions.user_profile`, a `ProfileChoice`), since they all land in
one account's on the target, and one person's files are no part of another's app.
`SystemContext::open_for_scan` loads only that account's hive, so another's cannot come along by
accident. `Default` means, in order, the account an app was installed for (`InstalledApp.owner_profile`,
which picks no one else's), on this PC whoever is signed in, and on a drive whoever last signed in
there (`LastLoggedOnUserSID`) or the only account it has.

**Who is signed in** (`path_utils::signed_in_user`) is the owner of this session's real
`explorer.exe`, not the account running this. An elevated process can be run by another administrator,
named at the UAC prompt, whose `%APPDATA%` and `HKEY_CURRENT_USER` are not those of the person at the PC.
Reading goes through `HKEY_USERS\<SID>` and that account's profile from `ProfileList`, and a hive that
is not loaded is copied and loaded. With more than one owner, or none and a system account running this,
it could not be told: no one's is read, and it says so. With no desktop in the session, as under a
scheduled task, SSH, or a build agent, there is no one else it could be meant for, and it is the
account running this.

A folder in another account's profile is left out, by where it really is, however a value came to name
it. A Burn bundle, a hostile uninstall string, or a shortcut can name one. The exception is the install location in an
app's machine-wide record, as it is written with no link on the way. That is the only one of these an account cannot
write with no rights at all.

An app installed for one account is held to more, because all of what describes it is that account's to write
(`SystemContext::is_the_accounts_own`). Its program, that program's folder, and its install location have to be in that
account's profile, or owned by that account, which reads the folder's owner. Its roots go no further than its install
location and that profile, nothing machine-wide goes by its name, in files (`FileDiscoveryContext.installed_for_one_account`)
or the registry (`RegistryScanContext.installed_for_one_account`), and its install log can only name its own registry
(`recorded_keys_of_one_account`). What is found by an app's name in any account's `AppData`, Documents, or Saved Games
has to pass the same check, since a link there or a value for where Documents are could lead anywhere.

A machine-wide app finds its program by `ExecutableHints::read_machine_wide`, which leaves out an account's own shortcuts
and App Paths. Its Inno Setup or NSIS log is only used if only administrators can change it
(`privilege::only_administrators_can_change`), since that names files and registry values to package and sits in the
app's own folder, and an account's own hive does not add MSI products to it. A log is taken at its word on a drive, where it
cannot lead off the drive and is held to the rules above, and a drive keeps who owns a folder, so that is read the same
way there. It cannot be told on a volume with no permissions, and something outside the account's profile is refused then.

A folder named for an app in `ProgramData` or Public Documents that is a link is left out, since any account can make one.
Whether something is in a folder goes by where both really are (`is_really_within`), and a place with no name to give, such
as a shadow copy, cannot be told at all (`location_can_be_told`), so it counts as refused. A build asks again before and
after copying each file on this PC whether it leads into another account's folder or goes through a link that was not there
when it was scanned, or that leads somewhere else now (`OtherAccountsFolders`), since whoever can write in a scanned folder
can change what is there.

`profile_survey.rs` finds out who has data for an app, so that the app can ask. It loads every hive once,
when the list of apps is read, and keeps only the names of keys under `Software`. Asking about an app
then looks at some folders.

`FootprintLocation { path, kind, scope, file_count, total_bytes,
copyable_bytes, included }` groups the inventory by root for the report.
`FootprintKind` ∈ `InstallDirectory`, `ProgramData`, `RoamingAppData`,
`LocalAppData`, `LocalLowAppData`, `Documents`, `SavedGames`, `PublicDocuments`,
`RegistryReference`, `Other`.

## Registry, services, and other integrations

- `registry_scan.rs` captures the keys and values the install record names, values under
  the app's keys, COM registrations and file associations that point at the app, Run keys,
  and App Paths. Under `Software`, in both the normal and `WOW6432Node` views, a key counts
  when:
  - it is named for the product, by the same rules as folders (`Software\7-Zip`,
    `Software\Mozilla\Firefox`), or
  - it records the install folder in a value named for that, such as `InstallDir`, so VLC's
    `Software\VideoLAN\VLC` is found though its name is not the product's. This is checked
    three levels deep under each vendor's key, except `Microsoft` and `Policies`.

  A publisher's key by itself never selects anything. Under `Software\Microsoft` the two
  segments must spell the product, because being published by Microsoft says nothing there:
  `Software\Microsoft\PowerShell` is Windows' own.

  A value points at the app only if it names the executable or a path inside the install
  folder as a whole path, so `C:\Program Files\Git` is not `C:\Program Files\GitHub CLI`. A
  bare `rundll32.exe` in a command matches nothing. A file type counts as the app's only when
  opening it or its icon points there. Otherwise just the verbs the app added to it are
  captured, so Windows' `LibraryFolder` is not copied because PowerShell added a verb.

  Every value under a matched key is captured, whatever its name, so a key's license or token
  values come along even with the license option off.
- `service_scan.rs` captures services and drivers whose image is the main executable or
  under the install location, with a service's arguments and `Parameters` key.
  `parse_image_path` reads an `ImagePath` however it is written: quoted or not, `\??\` and
  other NT forms, or relative to the Windows folder. A service hosted by `svchost.exe` is not
  captured: its binary is Windows', and only its `ServiceDll` would be the app's.
- `task_scan.rs` captures scheduled tasks with any program that is the app's, from the Task XML
  files under `Windows\System32\Tasks`, whichever PC they are on. The Task Scheduler only knows this
  PC, and asking it through PowerShell took one call per task and lost them all to one that broke the
  JSON. A file that cannot be read now costs a warning for that task and no others, and it reads
  UTF-16, with or without a byte order mark, and UTF-8. A task with two programs is one task. One whose
  principal is anyone but the account being read or one of Windows' own is left out, with
  `scheduled_task_left_out`, and so is one that says it runs as Windows, on this PC, whose file others than
  administrators can change, because any account can make a file in that folder.
- `shortcut_scan.rs` captures the Start Menu and Desktop shortcuts of the account being scanned and
  of everyone. On this PC, WScript.Shell reads them. On a drive, the `.lnk` files are parsed directly,
  since COM would resolve what it found against this PC.
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
- `ScanOptions.include_hashes` — SHA-256 of copyable files during a scan. It dominates scan
  time for large apps. Packaging leaves it off and hashes each file as it is staged, so the
  hash describes what is in the package, including files read in backup mode.

## Runtime prerequisites

`scan::runtime_prerequisites_from_signals` infers the runtimes an app needs from
its imports, app-local DLLs, and footprint, and `prerequisites.rs` resolves each
to a concrete on-target plan (bundled installer, winget, or a Windows optional
feature), verified by a registry probe where one exists. A package records which kind and
build it needs and any payload it bundled. Everything that decides what runs comes from the
specs compiled into `prerequisites.rs`, and a payload must be signed by Microsoft.

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

Discovery normally reads the live OS through two seams: the live registry and known
folders. `system_context.rs` funnels both through one `SystemContext` so the same scanners
work against a **mounted disk from another machine** (a failing PC). `ScanSource` is `Live` or
`OfflineVolume { volume_root }` (e.g. `E:\`), threaded via `ScanOptions`/`PackageOptions`.

For an offline volume, `SystemContext`:

- **Copies the disk's hives and loads the copies** with `reg load HKLM\WAM_<run>_…`:
  `SOFTWARE`, `SYSTEM`, each profile's `NTUSER.DAT`, and for a scan its `UsrClass.dat`, which is what
  appears under `Software\Classes` once an account signs in. Each comes with its `.LOG1` and `.LOG2`,
  so what was last written is read, into the copy. They go in an administrators-only folder under
  `%ProgramData%\WinAppMigrator\hives`. Logical `HKLM\…` paths are mapped onto the mounts, resolving
  `SYSTEM\CurrentControlSet` to the disk's active set from `SYSTEM\Select\Current`.
- **Knows the drive by a `VolumeMap`**, with the letter its Windows was on from its own `SystemRoot`.
  That is the one place a path recorded on the source PC becomes where it can be read
  (`to_volume`), and the other way (`to_logical`).
- **Lists its accounts from `ProfileList`** in its `SOFTWARE` hive. A profile folder with a hive and no
  record is kept too, known by its name, since this may be the only chance at it.
- **Reads its computer name and Windows version** from those hives, for the manifest.

Paths come in two kinds. **Logical** paths are what the source PC called things: what its registry
holds, what a value is compared with, and everything in the report and manifest, including the install
location an installer offers. **On-volume** paths are where something can be read now. Scanners work on
disk with the second and compare with the first, and `use_what_the_source_pc_calls_things` turns a scan's
results into the first in one place. Only `FileEntry::path` keeps where it could be read, and the manifest
drops that too.

### A drive is not trusted

What a value on a drive says is only ever a path on that drive. Only a full path on the source's Windows
drive maps to one, in ordinary names:

| A value of | Is |
| --- | --- |
| `C:\Program Files\App` | `E:\Program Files\App` |
| `\Windows\x`, `x\y`, `C:x` | not followed: it would mean somewhere on this PC |
| `C:\a\..\b`, `C:\a\.. \b` | not followed: it could climb out of a volume mounted on a folder |
| `D:\Games\App` | not followed: it was on another disk of that PC, and something else has that letter here |
| `\\server\x`, `\\?\UNC\…`, `\\.\x` | not followed: a server would get this account's password hash |
| `C:\a\NUL`, `C:\a:stream` | not followed |

**Nothing that is not on a disk of this PC is looked at, on a scan of this PC.** A value in any account's registry can
name a share, and looking at it connects as the administrator running this. `SystemContext::may_read` refuses `\\server`,
`\\?\UNC`, `\\.\`, `\\?\GLOBALROOT`, `\\?\Volume{}`, and rooted forms such as `\??\UNC`, by how the path is written.
A known folder there gets `user_folder_out_of_reach`, and anything else is listed in `network_paths_not_followed`. Names
Windows would read as another are refused on this PC as they are on a drive, since any account can make a real folder `x.`
beside a link `x`.

**No link on a drive is followed,** since a junction or symbolic link on another PC's disk resolves against
this PC. `VolumeMap::may_read` walks from the drive's root to a path, and everything that opens a file or
folder there asks it first: each base folder, each root, each hive, each log and shortcut, each candidate for
the main executable, and again from the top just before each copy. A walk does not go through a link it finds, and goes
through `path_utils::walk_asking`, which asks about each entry by name before doing anything with it. `walkdir` opens a
folder before its filter is asked, and what a listing calls the folder `x.` opens as the link `x` beside it, since Windows
drops a dot or a space from the end of a name. `is_ordinary_name` refuses those, names that are not text, and names this
Windows would open as a device.

What was not followed is listed in a warning, `drive_paths_not_followed`. A variable in a value means what it
would on that PC (`expand_source_variables`), and not what it means here: only the folders every Windows has
are known, and the rest stay as they are, and match nothing.

**The drive is not written to.** Loading a hive where it is can replay its logs into it, which is why the
copies exist. Windows itself can still update last-access times and its own NTFS logs on any disk it mounts.

A crash can leave hives loaded and copies behind, which can hold secrets. A run holds a lease, a file in its
folder open with no sharing, for as long as it lives. `sweep_leaked_hive_mounts` runs when the app starts and
before anything is mounted: it unloads what has no lease held, removes those folders, leaves another run's
alone, and only touches a key named the way this app names one.

Prerequisite *presence* on the source is left unknown offline (the `is_present` probe reads the live
registry).

Scanning a drive needs **elevation**, for `reg load` and for reading files through backup privilege. The app
never changes ownership or permissions (no `takeown`/`icacls`); files whose ACL denies a plain read are copied at
package time via `robocopy /B`. **For a failing drive, image it first and scan the image.**

## Restoring user data to real locations

Run headlessly, program and user-data files (including the `files\_external\…` data staged
from outside the install directory) land under one `target_root` unless
`--restore-user-data` is passed. Captured license files go under the install location when
their paths map there, and otherwise to this PC's own folder for where they came from.
Registry values and environment variables always go to their real locations, and shortcuts
to this PC's own Desktop and Start Menu. The installer's window asks instead: its **User data**
option is on by default whenever data was captured. Turning it off still writes the data,
under the install folder.

Opting in, with `--restore-user-data` or that option, re-anchors each user-data file to
the **target machine's** equivalent base (`USER_DATA_ANCHORS`), from where it came from:

| Source anchor | Target base |
| --- | --- |
| `…\AppData\Roaming\…` | That account's `AppData` |
| `…\AppData\Local\…` | That account's `Local AppData` |
| `…\AppData\LocalLow\…` | That account's LocalLow |
| `…\ProgramData\…` | `ProgramData`, from the admin-only registry |
| `…\Users\Public\…` | `Public`, from the admin-only registry |
| Documents, directly in a profile, or recorded as such | That account's `Personal` |
| Saved Games, directly in a profile, or recorded as such | That account's Saved Games |

App-owned install-directory files always restore under `target_root`. Routing is centralized in
`file_discovery::restore_destination_for_entry` (with `data_location_of` saying where a file came from) so
restore and verification resolve identical destinations.

**That account is whoever is signed in,** by the same rule as for a scan, whichever account the data came
from and whichever account is running the installer. Its per-user folders come from its `User Shell Folders`
under `HKEY_USERS\<SID>`, so a folder moved by OneDrive is where it goes. Its per-user settings go to that hive,
and the journal records whose it was. If who is signed in cannot be told, per-user settings are left out and
listed, and user data goes under `target_root`.

Those values are that account's to change with no rights at all, and an elevated process writes wherever they
point, so **each folder has to be inside the profile**, as `ProfileList`, which only administrators can change,
records it. For AppData, one that is not gets the usual folder in the profile. **Documents or Saved Games that are
not, or are under `AppData`, are not written to,** and neither is one with a variable that cannot be told. Those
files stay under `target_root`, in `_external`, and the review says so, which an installer's window shows before
anything happens.

Safety: ProgramData (and other machine-wide protected roots) require elevation on apply —
`privilege::destination_requires_admin` gates this; a re-anchored destination's path cannot
leave its base, and `ensure_within_root` refuses a link anywhere below that base just before
writing; without `--force`, a headless restore stops before changing anything if any file
it would write already exists, while the installer's window always overwrites and warns first if the install
folder has files; and the built-in verification runs with the same user-data setting so it
checks the real locations.

## What a restore refuses

A package is not authenticated, and it directs an elevated process, so scanning conservatively
is not enough. `restore_policy.rs` holds the rules, as pure functions, and `restore_review.rs`
applies them to a whole package. Restoring, verification, and the installer's window all work
from the same review, so verification expects exactly what was allowed. What is refused goes
in `RestoreResult.refused`, and the rest of the install goes ahead.

Any rule added for a new kind of capture belongs there too. Event log sources, for example, live
under `SYSTEM\CurrentControlSet\Services\EventLog`, which is refused today because nothing writes it.

## Known limitations

Full list, with the planned fixes, in [PLAN.md](PLAN.md#open-issues-most-important-first).
The ones that most affect what gets captured:

- **Exact names can miss data.** A folder or key the app uses under another name is left
  behind unless an install record names it or a value points at the install folder.
  `ProgramData\Mozilla-1de4eec8-...` (Firefox's update data) and `Mozilla Maintenance
  Service` are examples. The preview planned in [PLAN.md](PLAN.md#open-issues-most-important-first)
  would offer such near matches for you to add.
- **Same name, different product.** Side-by-side versions of one product can match each
  other's folders (`Programs\Python` for each Python), and an unrelated folder that happens
  to have exactly the product's name is taken.
- **An app made of several packages only gets its own.** NVIDIA App's `NvContainer` and
  `NvTelemetry` are separate entries in the Uninstall key, marked as system components, so an
  installer built from NVIDIA App alone does not have them.
- **A connected drive only gives what is on it.** An app or data on another of that PC's disks,
  or reached through a link, stays behind, and the build lists what it did not follow. Only the
  folders every Windows has are known for a `%Variable%` in a value on a drive.
- **Shortcuts on a drive with no file path.** An MSI's "advertised" shortcut names none, so whether it
  opens the app cannot be told without Windows' help, and it is counted and left out.
- **Size.** Windows cannot start an executable of 4 GiB or more, so a larger package fails
  the build with that reason.
