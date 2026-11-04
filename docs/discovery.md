# Application Footprint Discovery

This document describes how WinAppMigrator reverse-engineers an installed
application's footprint. The goal is to be **authoritative where Windows records
the truth**, and heuristic where it does not. The heuristics match whole names only, so
they can miss a folder the app keeps under another name but should not take another
product. What is close to a name is [offered, and never taken](#near-matches). See
[Known limitations](#known-limitations).

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
  install-source roots into discovery, and records the result on `ScanReport`. It writes no reports
  (`scan_app`), so a window can look before anything is chosen.
- `crates/winappmigrator-core/src/near_match.rs`: what is close to the app's names, for a person to choose from.
- `crates/winappmigrator-core/src/footprint.rs`: what a scan found as rows to choose from, and what comes of a
  choice.

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
Used for every method. The key has to say it is this program before anything it lists counts, as a program of the same
file name is not the app.

### The uninstaller

The folder an app's uninstaller is in is a root only if it is the app's (`uninstaller_folder_is_the_apps`): it is inside where the
app's entry says the app is, or with no such place said, it holds the app's program, says so by its name, or is in `Program Files`. A
folder named only for the publisher is not, as it can hold all they make, and neither is a person's own, such as `Downloads`. That file
comes by itself if it is somewhere programs are put, `Program Files` or `ProgramData`, and otherwise stays, so a setup run from
`D:\Downloads` brings nothing. With no location on record, Windows Installer is asked whether another app has a file in that folder
(`another_app_has_a_file_in`), which takes some seconds, and if so it is left out, and where the app is goes by what is left.

### What every record's folder is asked

Each folder something on record names, other than where the app itself is, is asked by `root_refusal` whether it may be read, is
there, is written as it is really called, holds what is shared or is Windows', is named only for the publisher, or holds another app's
folder or a file Windows Installer says is another app's. That is asked only of a folder in `Program Files`, or if something is on
offer as a near match, as it goes through every product on the PC, and where its records cannot be read it says nothing against a
folder. Microsoft's own apps are in `Program Files`
under the names that say a folder is everyone's, so those names do not count there for them. A folder known only because an
uninstaller is there is also refused if it is in another app's folder. What a record names of a folder that fails still
goes, file by file, and the folder is said to be left out, by `folder_left_out_of_the_app`.

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
  dirs, app shortcut targets, the uninstaller's folder if that is the app's, App Paths dirs, and the
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

## Near matches

Exact names are what keep a scan from taking another product, and they leave things behind. `near_match.rs`
finds what is close and says why, for a person to tick. None of it is taken by itself.

At most 200 are offered, since a drive could have any number of folders.

**A folder** directly in `Program Files`, `ProgramData`, or the scanned account's `AppData`, or one level down, is close when
(`ProductNames::near_match`, all by whole words):

- it starts with the product's name and goes on (`Stream Deck Plugins`), or
- it has the product's name in it, for a name of more than one word or of six letters (`Elgato Stream Deck`), or
- directly in one of those folders, it starts with all of the publisher's name and goes on (`Mozilla-1de4eec8-...`).

One level down, under anything but the publisher's folder or a plain container, a name of one word does not count, since
`chrome-prof` under another tool's folder is that tool's. Being in the publisher's folder says nothing by itself. `Git` is not near
`GitHub`, and one word of a longer name by itself is not a name: `Studio` from `OBS Studio` does not make `LM Studio` near it.
A short executable or folder name such as `Code` is a word anyone could use, so it does not count. Microsoft is on
everything, so its name says nothing either. `Documents` and `Saved Games` are not looked in, since people name those folders
themselves.

**A hidden entry** is one in the Uninstall key marked `SystemComponent=1`, which no list of apps shows. It counts when it is
beside the app in the same folder, or has the same publisher, by all of its name. A runtime the app uses stays a prerequisite. An
entry in an account's own hive takes no rights to write, so it only counts for an app of that same account. What an entry calls
itself is only what it says, so one that goes by the name another app would take, or by nothing but its publisher's, or whose
install record is another listed product's or whose uninstaller or icon is in another app's folder, is not offered, wherever it says it is, and its folder is not offered by its name
either. Several in one folder are one row, named for
all of them.

**What is never offered, or added.** Each of these is asked by `folder_refusal`, of a folder on the list and of each
root a hidden entry goes on to name:

- A drive, the Windows folder, `Program Files`, `Common Files`, `ProgramData`, a profile, `AppData`, or anything that holds one
  of those, by the same `is_walkable_app_dir` a scan goes by.
- What is Windows' or everyone's by name (`NEVER_OFFERED`, such as `dotnet`, `Windows Defender`, `Package Cache`), anything
  under `Microsoft`, `Microsoft Shared`, `Microsoft.NET`, `WindowsApps`, `dotnet`, `Internet Explorer`, `WindowsPowerShell`,
  `InstallShield Installation Information`, `Windows.old`, or a folder whose name starts with `Windows `, leaving aside what an account
  is called, and `System`, `Services`, `SpeechEngines`, `Designer`, `InstallShield`, `Merge Modules`, and `ODBC` in `Common Files`.
- A folder named only for the publisher, which may hold its other products, or one that holds this app's own folder.
- **Another app's.** Where any other entry says it is, or anything that holds that or is inside it, or what that app would take
  by its own name. An entry on the list of apps is another app, even by the same publisher: it is there to be built by itself. An
  MSI's entry often does not say where it is, so anything that Windows Installer's database says holds one of its files counts
  too. That database is only gone through when there is something to offer, and takes a few seconds on a PC with a lot on it.
- Whatever goes through a link, or may not be read on a drive.
- A path written some other way than the folder is really called, such as `C:\PROGRA~3\MICROS~1`, since all of this goes by
  names. A drive with a letter that is also mounted on a folder is refused by that too, when it is read by the folder.
- What is in another account's folders. An app of one account is offered nothing outside what is its own.
- A name that says nothing, such as `bin` or a `Cache` folder.

**Ticking one.** `ScanOptions.near_matches` holds what a window ticked, by id. A scan makes the same list again and takes only
what is on it now, so an id cannot name a folder of its own, and it warns of one that is not there any more. What is
ticked is one more root for `collect_roots`, which goes by its rules, and what goes by where the app is (services, tasks, shortcuts,
variables, and registry values) goes by that folder too. An entry also brings its install record, what it says of itself is asked
about again. If it has the app's publisher, by all of that name, so do data folders and keys named exactly for it, as long as that
is under a folder or key with all of its publisher's name, or starts with all of that and goes on, so that one calling itself
`Windows NT` or `Update Service` does not bring in what has that name elsewhere. Being beside the app says nothing of what it calls
itself, so nothing comes by name for one with another publisher. A key found by name, by where something added is, or in an install
record is left out when it is what another app would take by its name, and one found by name is left out when many apps share it.
A file its install record lists is left out when another app's record lists it too, and if that record names anything of another
app's at all, none of the keys it lists come along. All of this is before `take_secret_files` and
`license_scan`, so what looks like a credential stays out and a license goes by its option, however it came. What is in a
data folder goes by the User data option, like any other.

`wam_near_matches` runs this for every app on a PC. See [CONTRIBUTING](../CONTRIBUTING.md#seeing-what-near-matches-would-offer).

What these rules cannot tell, all of it from a person ticking something that starts unticked:

- A publisher goes by its words without the ones most companies have, so `Data Labs` passes as `Data Systems`.
- Windows Installer's records are only asked about what another app has. A record naming another app's key and no files passes by
  its names alone, and so does one that names files only by their short names. A candidate inside a Windows Installer app's folder
  that holds no key file of that app's, or a Burn bundle's folder, can be offered.
- In the safe direction, a real part can lose keys: one whose publisher is named by fewer words, or one with a file its sibling also has.
- A hidden entry ticked, with its folder then unticked, is said to be both added and left out, since its keys and data can still go.

## Registry, services, and other integrations

- `registry_scan.rs` captures the keys and values the install record names, values under
  the app's keys, Run keys, and App Paths, and takes what ties the app into Windows from
  `integration_scan.rs`, below. Under `Software`, in both the normal and `WOW6432Node` views, a key
  counts when:
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
  bare `rundll32.exe` in a command matches nothing.

  On Windows 10 and 11, `WOW6432Node\...\App Paths` is the same key as the other, seen through the 32-bit
  view, which the runner shows, and a mounted hive does not have that link to read. So only what the
  plain view does not have is taken from it, which on those versions is nothing. A 32-bit `Run` key is its own.
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
  administrators can change, because any account can make a file in that folder. Whoever a task was written by, ran as, or waited for
  is someone on that PC, so `task_for_whoever_installs_it` takes them out. A package says `WinAppMigrator-whoever-installs-it`, and a
  restore puts in the SID of whoever is signed in, or refuses if that cannot be told.
- `shortcut_scan.rs` captures the Start Menu and Desktop shortcuts of the account being scanned and
  of everyone. On this PC, WScript.Shell reads them. On a drive, the `.lnk` files are parsed directly,
  since COM would resolve what it found against this PC.
- `env_scan.rs` captures environment variables whose value contains the app's path, expanding what it says first. Of a `Path`, or any
  list of folders such as `PYTHONPATH`, only the app's entries are captured, since the rest is what else is on that PC. It is merged
  on restore: only entries under the install location are added to the target's value, which is otherwise left alone.
- An app's entry in Settings goes, with its uninstall string replaced by this project's uninstaller, less what is only true
  where it was installed: `InstallSource`, `ModifyPath`, and `WindowsInstaller`, because Windows Installer has never heard of
  the app on another PC, and Inno Setup's `Inno Setup: User`, `User Info: Name`, and `User Info: Organization`, because they
  say who ran the setup. `Inno Setup: User Info: Serial` is license data, by the same rules as any other license value.
- Inno Setup's uninstall log, `unins000.dat` beside `unins000.exe`, is read to find what the app has and is not copied. It
  names the PC and the account that ran the setup, and lists where things were on that PC, so its uninstaller would look in
  the wrong places anywhere else. Without it, that uninstaller only says it is missing. Settings removes the app another way.

### What ties an app into Windows

`integration_scan.rs` finds the rest, for the machine and for the account being read, on this PC or a
connected drive, through `SystemContext`. **An entry is the app's if what it points at is.** Its name says
nothing, since these sit side by side with everyone else's. Each is read for the file it names, and that is compared
with the files the app has, as its own PC calls them. On a connected drive that is only ever text, so nothing can be
led to the host's files, and per-user classes are read from `UsrClass.dat`.

| | What makes it the app's |
| --- | --- |
| **COM class** | A server is a file of the app and none is someone else's, or Windows hosts it and its `CodeBase`, icon, or instance properties are. A server that is someone else's makes the class theirs, whatever else in it mentions the app. Read in both views, for the machine and the account |
| **ProgID** | What opening it runs is the app, or with no command to go by, its `CLSID` is. A class naming a ProgID does not make it the app's unless that names the class back |
| **Type library** | A `win32`, `win64`, or `arm64` file is the app's. A class naming one does not make it so |
| **Interface** | Its `ProxyStubClsid32` is a class of the app's, or its `TypeLib` is a library of the app's |
| **AppID** | A class of the app's names it, or `AppID\<its server>` names one of those, and what hosts it, if anything, is the app. Windows' own that every preview handler names stays out |
| **URL protocol, file type** | What it opens is the app, which goes for `http` only if it is the browser that has it. Anything else only has the verbs and handlers the app added to it |
| **`OpenWithProgids`, `OpenWithList`** | Only the app's own line in a list that is the type's, not the rest of it |
| **Shell extensions** | `shellex` handlers, icon overlays, the `Approved` list, and Explorer namespaces, that name a class of the app's |
| **`RegisteredApplications`, `Clients`** | Its capabilities have an icon of the app's or open a type of the app's, or what a client opens or shows as its icon is the app |
| **Firewall rule** | Its program is a file of the app. One with a `TTK` condition or two `App`s is not carried, as it would let in more or read as another program's |
| **Font** | Registered by a path to a file of the app. One in `C:\Windows\Fonts` only if a machine-wide install record names it, which is then copied under the install location and never into Windows' folder. One with the app's name that nothing says is its own is reported |
| **ODBC driver** | Its `Driver` or `Setup` is a file of the app |
| **ODBC source** | It uses a driver of the app's, or has the app's exact name. One using Windows' driver under another name says nothing of whose it is, so it is missed, on purpose. Its password is left out like any |
| **Event log source** | One of its message files is a file of the app |
| **Compatibility setting** | It is for a program of the app |
| **Certificate** | Reported only, never restored: one in a trust store of the machine or the account whose subject names the app or its publisher. So one that names neither is not found |

The reverse lookup is once per scan. Each key under `CLSID`, `Interface`, and `Classes` has only what says what runs
read from it, in place of a walk of each subtree. These are the largest keys there are, 30,000 under `Interface` on a
PC with Visual Studio, so they are gone through on a few threads, each with its own handle, which only read. What they
find is then captured one at a time. `Interface` is only read in a view that has one of the app's classes in it,
or with a type library of the app's.

### What is not captured at all

`secret_rules.rs` holds what looks like a credential, and its rules are pure, with false positives pinned by tests. What
it takes for one is not in a package, whatever else was asked for, and is named in `secrets_left_out`, in the report, and
by the installer. There is nothing to redact, so nothing can overwrite a real value on the other PC.

- A value goes by its name, by words: `DbPassword`, `db_password`, `dbpassword`, and `PassWord2` are all one, and
  `PasswordPolicyVersion`, `TokenizerPath`, `MaxTokens`, and `RememberPassword` are settings about one. A default
  value goes by its key's name, unless that is a file type. A number or flag holds nothing, and neither does what
  Windows fills in, such as `%1`.
- Whatever it is called, a value is left out if it holds a connection string or URL with a password in it, a command with
  an option named for one, or a blob only DPAPI can read.
- A command line loses just the option: a service's arguments and its suggested `sc.exe` command, a task's arguments and
  definition, a shortcut's arguments, what opens a file type, and the uninstall string. A task definition that cannot be
  read to check it is dropped.
- A file goes by its name (`.env`, `id_rsa`, `credentials.json`, and so on) or its extension (`*.pfx`, `*.kdbx`). A `.pem` or
  `.key` only if a private key is in it, since Git and Python each ship a `cacert.pem` they cannot work without, or if it cannot be
  read to tell.

With the License option off, `license_scan.rs`'s own rules take license files, values, service settings, command-line
options, and variables out in the same way, named in `licenses_left_out`. Its FlexLM, CodeMeter, and Sentinel stores count only
for an app with that framework's client files, and a `*_LICENSE_FILE` variable only if it names the app or its folder. A file
in a store that looks like a credential is not copied either.

This cannot catch everything. See [SECURITY.md](../SECURITY.md#an-installer-can-contain-secrets).

## Options

- `ScanOptions.include_user_data` / `PackageOptions.include_user_data` — when false,
  genuine per-user data (Roaming/Local/LocalLow AppData, Documents, Saved Games) is
  discovered and reported to the screen but not copied, and a package says nothing of it. Set from the app's "User data" option, which is
  off by default. `for_executable` and the serde default set it to `true`, for library
  callers.
- `ScanOptions.include_program_data` / `PackageOptions.include_program_data` — controls
  machine-wide `ProgramData` separately from per-user data. The app drives it from the
  same option.
- With `include_user_data` false, `account_settings.rs` also takes out of the captured registry what an account keeps for the
  app. It goes by where a value is: of `HKEY_CURRENT_USER`, what stays is what `is_where_windows_looks_for_an_app` knows, what an
  entry in Default apps points at if it is called `Capabilities`, and a plainly named text value in the app's own key, no more
  than two below `Software`, or three if it is called what an installer calls that, whose whole data is a path in one of the app's
  folders. It runs after `license_scan`, so a license value goes by the
  License option either way, and before anything names what it finds, since a key's name can be what was being kept.
- With it false, `account_traces.rs` takes out the rest of what says whose PC this was, by what each thing is, when a scan is asked for
  none of an account's data and again when a person has left all of that out of one that was. A shortcut in an account's own folders
  goes only if it just opens the app, and is said with no account in it, unless the app was installed for that account. A task goes
  without what else it runs in someone's folders, and goes whole if it was told to work on something there. A variable of an account's
  goes only if all it says is where the app is. A service that ran as someone says an account of its PC. A DLL found through an account's
  `Path` is known by its name, and gets a warning in any case, as it may not be on the other PC. What is only read, such as warnings and
  what is left out, says `<an account>` and the like (`say_nothing_of_who_it_was`). `what_a_package_still_says` then looks through all that
  was written for the account, profile, PC, and SID, and names on the done screen where it found any, changing nothing.
- `ScanOptions.include_hashes` — SHA-256 of copyable files during a scan. It dominates scan
  time for large apps. Packaging leaves it off and hashes each file from what it reads into
  the package, so the hash describes what is in the package, including files read in backup
  mode.

## Runtime prerequisites

`runtime_detection.rs` reads what an app needs from all of its programs, and `prerequisites.rs` resolves each
to a concrete on-target plan (bundled installer, winget, or a Windows optional feature), verified by a registry probe
where one exists. A package records which kind and build it needs and any payload it bundled. Everything that decides
what runs comes from the specs compiled into `prerequisites.rs`, and a payload must be signed by Microsoft. What winget fetches is
winget's to check, and only its own signature is checked here.

**Which programs.** Each of the app's programs counts for its own build, shallowest first, with the DLLs they import from their
own folders, up to 64 programs, 512 files, and 2 GiB read, and a warning if that was reached. What exists to install or support an
app is left out, by `is_auxiliary_executable`, by folders such as `_CommonRedist`, and by the likes of `wininst` and
Squirrel. So are the Arm64 launchers packaging tools leave around, on an app that is not Arm64.

**Visual C++.** A DLL is the runtime its name says, by the digits in it (`msvcr120`, `msvcp140`, `vcruntime140`, `mfc100`, and the
rest), so `msvcrt.dll` and `mfc42.dll`, which are Windows' own, are not one at all. Every version from 2005 has a spec of its own,
with its own Microsoft URL, downloaded at build time and embedded, and winget on the target if that failed. 2005 and 2008 have
no probe, so their installers are always run. 2003 and earlier have no installer here, and a debug build's DLLs are
not something Microsoft redistributes, so each of those is named in a warning.

**.NET** is told by what it says of itself, since .NET Framework and modern .NET both say `v4.0.30319`:

- A `*.runtimeconfig.json` beside a program gives the framework and its major version, and a self-contained one needs nothing.
- Otherwise the assembly's own metadata: its `TargetFramework` attribute, and whether it refers to `mscorlib`, and to the
  desktop and ASP.NET Core assemblies.
- CLR `v1.x` or `v2.0` is **.NET Framework 3.5**, installed by enabling the **`NetFx3` Windows optional feature via DISM**
  (`/online /enable-feature /featurename:NetFx3 /all`). The payload comes from Windows Update or install media.
- CLR `v4.0` without those is **.NET Framework 4.x**, in-box on Windows 10 and 11: listed and probed, never installed.
- Modern .NET is the **.NET, Desktop, or ASP.NET Core runtime** of that major version. 6 to 10 have a spec, from `aka.ms/dotnet`,
  embedded like Visual C++, with winget as what is left. No such ASP.NET Core 6 for Arm64 exists. Any other version
  is named in a warning, and nothing is installed in its place. There is no probe, since its versions are values
  named after each build, and an installer run on top of itself just exits.

Each URL is really downloaded, and checked as Microsoft's with `Get-AuthenticodeSignature`, on the throwaway runner.

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
- **Reads its computer name and Windows version** from those hives, for the manifest. The name only
  goes in with the User data option on.

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
beside a link `x`. A recorded path is first read as Windows would read it, without what it drops from the end of a name, so
a value naming `Acme, Inc.\App` is asked about as the real `Acme, Inc\App`.

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

A hive that a plain read is denied is copied in backup mode, where the room it takes is watched and a cancel reaches it, since that brings
streams its size says nothing of. That route has only been run in tests that do not deny a real hive.

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
writing; without `--force`, a headless restore stops before changing anything if the folder has files
and no install on record, or if any file it would write already exists; the installer's window refuses a folder with files
unless an install is on record there, and then warns and replaces; and the built-in verification runs with the same
user-data setting so it checks the real locations.

## What a restore refuses

A package is not authenticated, and it directs an elevated process, so scanning conservatively
is not enough. `restore_policy.rs` holds the rules, as pure functions, and `restore_review.rs`
applies them to a whole package. Restoring, verification, and the installer's window all work
from the same review, with the same options, so verification expects exactly what was allowed. What is refused goes
in `RestoreResult.refused`, and the rest of the install goes ahead. A list of refusals can never be complete, so values outside
the app's own keys and the kinds on the list show as a row of their own, `SystemChangeKind::OtherSettings`.

Any rule added for a new kind of capture belongs there too.

What ties an app into Windows goes by what points at what. `PackageRegistrations` says what a whole package has,
and `registration_refusal` and `registered_value_refusal` go by that. A class counts as the package's only if a server it has
is, and no other is anything but one of Windows' hosts. A unit, meaning a class, interface, type library, AppID, event source, ODBC
driver or source, program name, `Applications\<program>`, row in Settings, or `Clients` entry, is all or nothing when it is
refused as a whole.
A single value refused for what it says by itself, such as an icon on a share, is only that value.

What depends on what this PC has can only be seen when installing, so `restore.rs` looks then. A class, or a unit of any of those
kinds, whose key is there and is not from an earlier install here, is left alone unless the package has exactly what is there. That
goes for both views, and for the machine's as well as the account's, since a per-user registration wins over the machine's. So do
a type or protocol that says what it is, by what opens it or by which verb it starts with, an autostart value, a font, and a Default
apps entry. A verb the package only adds to a type goes by that verb alone. They go in `RestoreResult.refused`, as "left out", and
`--force` does not change it.

**An earlier install here** means a record in the admin-only folder says one was at this location. A package suggests the location
itself, so what is on this PC does not count as the package's for naming it. Services, drivers, and tasks go by the same, and an
app's row in Settings, which is left alone if it is there at all, and the install gets its own. In the install location, files
do not go by this when `--force` is asked for on a command line, but the installer's window will not use a folder with files in
it without such a record. Outside the install location, without one, no program goes in a folder of a data location that was
there before the install began, new or replacing, and each file of a license store goes by that too. A data file is replaced.
What is left out goes in `RestoreResult.refused` by where it would have gone. If a service or anything else the package sets up would
run or rely on such a file, the install stops and nothing is installed (`what_would_run`), since it would run what was already there.

An install record that names a key is not taken at its word for a file type's lists and verbs, which have other apps' lines in
them, so `integration_scan.rs` goes through those. A verb that runs another app's program does not come along with its type, and
an AppID hosted by something else, as the one preview handlers share is, stays Windows'.

A firewall rule is told from another by its name, way, program, protocol, and ports, and one like it that says something else
is left alone. A journal entry goes first, and rollback and uninstall take out only that one by all of those. Rules that differ
by nothing but their profile or addresses are taken for one, so only the first is added, and it says it was already there.

## Known limitations

These are the ones that most affect what gets captured. Open issues are in
[PLAN.md](PLAN.md#open-issues-most-important-first).

- **Exact names can miss data.** A folder or key the app uses under another name is left
  behind unless an install record names it, a value points at the install folder, or it is close enough
  to be [offered](#near-matches) and you tick it. `ProgramData\Mozilla-1de4eec8-...` (Firefox's update data) is
  offered, and `Mozilla Maintenance Service` is an app of its own in the list, so it is not. What is under a name
  nothing like the app's is not looked for.
- **Same name, different product.** Side-by-side versions of one product can match each
  other's folders (`Programs\Python` for each Python), and an unrelated folder that happens
  to have exactly the product's name is taken.
- **An app made of several packages only gets its own, unless you add the rest.** NVIDIA App's `NVIDIA Container` and
  `NVIDIA Telemetry Client` are separate entries in the Uninstall key, marked as system components, so they are
  [offered](#near-matches), and only in the installer if ticked.
- **A link in an app's folder is not followed, and nor is anything more than 64 levels down.** Each is named in a warning, by its
  first few, so a `current` link or a `node_modules` that is one will show there, and what they point at stays behind.
- **A connected drive only gives what is on it.** An app or data on another of that PC's disks,
  or reached through a link, stays behind, and the build lists what it did not follow. Only the
  folders every Windows has are known for a `%Variable%` in a value on a drive.
- **Shortcuts on a drive with no file path.** An MSI's "advertised" shortcut names none, so whether it
  opens the app cannot be told without Windows' help, and it is counted and left out.
- **An ODBC source that uses Windows' own driver goes by the app's exact name.** It points at a driver, and not at
  the app, so one named anything else stays behind.
- **A secret can be missed.** The rules go by names and a few shapes of text. See
  [What is not captured at all](#what-is-not-captured-at-all).
- **At most 10,000 empty folders come along.** More than that is said, and the rest are not made.
- **The registry is captured up to a limit:** 1 MiB for a value, and 512 keys and 8 levels for what is under one key, with 100,000 values
  and a limit on how much it can hold in memory for an app. Reaching one is a warning, and the rest stays behind.
