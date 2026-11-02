# Changelog

All notable project changes are tracked here.

## Unreleased

- **The code that changes a PC has now run for real.** A CI job on GitHub's throwaway Windows runner
  sets up a harmless app, builds an installer from it with the real code, removes it, installs it,
  and checks the machine with `sc.exe`, `reg.exe`, `schtasks.exe`, and the shell. It starts the
  restored services and a task, to see what Windows really runs and as whom, and it covers packages
  asking for what the rules refuse, links planted at the install location, and a real download and
  install of the Visual C++ runtime. The test does nothing anywhere else. It found these, now fixed:
  - An installer crashed on a file name with a character across its third byte, such as
    `Größe.txt`, because of a bug in the zip library's name check.
  - That check also refused a name such as `aux.h`, which a scan copies, so one such file stopped the
    whole install.
  - On a drive that keeps no permissions, such as FAT32 or exFAT, a staging folder was called
    protected while it stayed open to everyone, because `icacls` reports success there. Its
    permissions are now read back, and the folder is not used unless they came out as asked. The
    installer stages elsewhere, and the app, which may be building to a USB stick, uses the system's
    temp folder.
  - When the folder above holds its permissions in an older form, the account that made a staging
    folder stayed on it after inheritance was cut off.
  - A list of paths in a `REG_MULTI_SZ` value kept pointing at the old location.
  - A scan lost a scheduled task whose command is quoted, and listed one with two programs twice, so
    that the second copy failed against the first.
- **An installer's command line now prints its warnings and errors.** They were only in logs in the
  staging folder, which is gone by the time it exits.
- **The installer says what it will do.** Before installing, its window lists the services, drivers,
  scheduled tasks, autostart entries, shortcuts, and `Path` changes the package sets up, what the
  rules left out, and whether the installer is signed and by whom, or "Not signed: publisher
  unknown". A preview's list leads with those instead of showing them after thousands of files.
- **An installer now checks what its package asks for.** A package can come from a hostile or
  odd source drive and nothing authenticates it, so the rules are applied when restoring, not
  just when scanning. What fails is left out, listed, and does not stop the rest of the install.
  - Registry: keys no app install should write are refused, including Image File Execution Options,
    Winlogon, `AppInit_DLLs`, LSA, SecurityProviders, Session Manager, KnownDLLs, certificate
    stores, and the file types that start programs. Under `SYSTEM`, only the `Parameters` and
    `Instances` keys of a service the package restores are allowed. Registry links are refused.
  - Run, RunOnce, and similar values, and App Paths, are allowed only when they run a file the
    package installs. An app's own `Run` value keeps working.
  - A COM class already registered to something outside the install location is not taken over.
  - Services and shortcuts must run a file the package installs, with no fallback to the path the
    manifest named. Scheduled tasks are checked by the commands in their XML, which is what Windows
    acts on, and must not replace one of Windows' own or another program's. Tasks that run as
    SYSTEM are flagged.
  - Shortcuts only land in this PC's own Desktop and Start Menu folders, so
    `C:\Users\alice\Desktop` is no longer created on a PC with no alice. One in a Startup folder
    counts as autostart.
  - A service, task, or COM class already on the PC that belongs to something outside the install
    location is left alone, and the package cannot install to a whole drive, the Windows folder, or a
    folder such as `Program Files` itself, even though it suggests the location.
  - License files only go under the install location or a known data folder. License values for
    the whole machine are never written, as was always the stated rule.
  - User data cannot land in a Startup folder or in Windows' own folders under AppData and
    ProgramData, such as `Microsoft\WindowsApps`.
  - Environment variables Windows and other programs depend on, such as `ComSpec` or the .NET
    profiler settings, are refused, and `PSModulePath` is merged like `Path` instead of replaced.
  - Task definitions no longer go through a predictable `%TEMP%` file, and are read as UTF-8, so a
    non-ASCII task name survives.
- **Hash checks no longer pass by default.** Every copied file is hashed as it goes into the
  package, including files a scan could not read itself, and a file with no hash now fails
  verification. Driver images and each file of a license store are checked too, both in the package
  and once installed. Only those hashes are computed at build time, so this adds no read.
- **An installer checks its whole package first.** The footer now holds a SHA-256 of the package,
  checked before anything is parsed, so a truncated copy or an edited manifest is refused at
  once with a clear message. The window opens straight away and shows the check. While the installer
  runs, its own file cannot be changed or replaced, so a file swapped in behind a running window is
  not what gets installed.
- **Drivers are checked before they are registered.** An image whose hash is wrong is not placed,
  and an existing `.sys` is no longer overwritten before the `--force` check. A driver already on
  the PC that loads from outside the install location is left alone.
- **A package no longer chooses what runs elevated for a prerequisite.** The URL, arguments,
  winget id, Windows feature, and registry probe come from specs compiled into the installer, looked
  up by the prerequisite's kind. A manifest can only say which kind and build, and supply the
  payload's path, which must be `prerequisites\<the spec's file name>`, and its hash.
- **Prerequisite installers are checked by signature.** A payload must be validly signed by Microsoft
  when it is downloaded and again just before it runs, and one that fails is not run and is
  reported as an error. Downloads are HTTPS only, including on redirects, with at most five redirects.
- **winget is no longer found through `PATH`.** The folder holding its shortcut is one the user can
  write to, so a planted `winget.exe` would have run elevated. The installer now asks Windows where
  winget is installed, and checks that file's signature before running it.
- **Elevated PowerShell gets a fixed environment.** The user's module folders, profilers, or
  `COMPLUS_InstallRoot` could otherwise have replaced the commands it runs, the signature check among
  them, or loaded a DLL into it. Its output is now read as UTF-8, so a path or task name that is not
  ASCII no longer fails verification.
- **Services come back as they were.** A service's path is now stored quoted, so that
  `C:\My Apps\Foo Bar\svc.exe` cannot be hijacked by `C:\My Apps\Foo.exe`. A code comment
  claimed the quoting was handled, but what reached `sc.exe` was unquoted. Arguments are kept, and
  so are the `Parameters` key and the display name. LocalService and NetworkService stay what they were,
  where they used to become LocalSystem. A custom account cannot be restored without its
  password, so its service is set up as LocalService, with less access rather than more, and
  the installer says so. A service already on the PC that runs from outside the install
  location is left alone, even with `--force`. Verification compares the binary, arguments, and
  account exactly: it used to pass any `ImagePath` that merely contained the path.
- **Drivers are found wherever they live.** `\??\` paths are understood and an unquoted path no
  longer stops at its first space, so drivers under `Program Files` are no longer missed.
- **Paths are rewritten exactly.** `C:\App\a.exe` no longer becomes `C:\App22\a.exe` for a
  target of `C:\App2`, `C:\Application Data` is left alone, and a rule for
  `C:\Program Files\Git` no longer touches `C:\Program Files\GitHub CLI`. Verification used
  the same flawed function, so it had been passing these. Text is walked once and a path counts
  only as a whole. Restore and verification now share one implementation, and a scheduled task
  whose path or target holds `&` stays valid XML. Rewriting costs a few hash lookups per path
  instead of a loop over every file: about 1.5 microseconds per value at 100,000 files, measured
  by a test in a development build. It was about 110 milliseconds.
- **Staging folders are locked down while elevated.** The installer's extraction folder and
  the app's build folder are restricted to Administrators and SYSTEM before anything is
  written to them, and the work stops if that fails. A process that is not elevated could
  otherwise change what elevated code was about to read. Those folders and the install location
  are also held open, because permissions do not stop whoever controls a parent folder from
  renaming one and putting another in its place. An installer no longer deletes an
  extraction folder newer than an hour, which could belong to another running installer, and
  no longer follows a link given that folder's name.
- **Writes cannot be carried somewhere else by a link.** A link anywhere from the install
  location down to a file is refused, including at the install location itself, and each
  file is written beside its destination and renamed into place, so a hard link there is
  replaced rather than written through. Names Windows opens as devices, such as `CON`, are left
  out of packages and refused when restoring. `.. .` can no longer pass for an ordinary name.
  Without `--force`, an existing file now stops an install before anything changes. Going on
  would have registered services against whatever was at those paths.
- **The manifest is read once per install,** so the install acts on what its hash check saw.
- **Elevation is asked about once, and from `whoami`.** PowerShell took about a second to answer,
  was asked repeatedly inside per-item loops, and could be made to fail, which read as not elevated.
  Not knowing is now a failure.
- **The app is now signed.** Releases sign `winappmigrator.exe` with Azure Artifact
  Signing through GitHub's OIDC token, with no stored secret, and record build
  provenance. See `docs/signing.md`. The installer stub deliberately stays unsigned:
  a signed stub could be given someone else's package hidden in its certificate table
  while keeping a valid signature (CVE-2013-3900). Building refuses a signed stub, and
  the stub ignores anything inside a certificate table and refuses anything after it. A finished installer
  can be signed with `scripts/sign.ps1`, which previously made it report "no package".
- **Redesigned interface.** The app is now a master-detail screen: the list fills the
  left, and the selected app, options, build, and result sit on the right, with no
  nested scrolling. Native Segoe UI with a real semibold weight, plus Korean, Japanese,
  and Chinese fallback fonts when an app or publisher name needs them. Painter-drawn icons, a window
  and file icon, and file details for Explorer and the UAC prompt. All colors meet WCAG
  AA contrast, enforced by a test. Sortable columns that sort versions by number,
  arrow-key navigation, Ctrl+F, screen-reader support, and the theme is remembered and
  follows Windows on first run.
- **The list shows real applications.** Entries marked `SystemComponent=1` are left out,
  as in Windows' own installed-apps list. They are parts of other products: on one test
  PC they were 481 of the 641 rows.
- **Footprints match the product, not everything that sounds like it.** Folders and registry
  keys now match the product's whole name, or `Vendor\Product`, never a substring. A publisher
  or the install folder's parent can only qualify a parent folder, and selects nothing by
  itself. On one test PC, VS Code went from 175,182 files and 46.83 GB to about 68,900 files
  and 10.40 GB, most of it its own user data: Visual Studio and the rest of
  `AppData\Local\Programs` are gone. Firefox no longer takes Thunderbird or `ms-playwright`,
  NVIDIA App no longer takes `Razer\Installer`, PowerShell no longer takes Store packages, and
  `Software\Logitech` is no longer part of Git.
  - Roots from MSI, Inno Setup, and NSIS records are now the deepest folder that holds the
    recorded files, and never a shared one. Cocos Dashboard no longer takes all 10.67 GB of
    `C:\ProgramData\Microsoft`, and its files in `C:\CocosDashboard` are no longer lost for
    being directly on the drive. An `InstallLocation` that is itself a shared folder is ignored.
  - Install records now also name registry keys, which is how `Software\GitForWindows` is
    still found. Matching covers `WOW6432Node`, looks one level under a vendor's key, and
    ignores versions, so `7-Zip 25.01 (x64)` finds its own key. A key that records the
    install folder in a value like `InstallDir` counts whatever its name.
  - A Burn bundle's files now come from its hidden MSI packages. The .NET SDK, the Windows SDK,
    Ableton Live, and Moonlight are bundles, and they used to rely on name matching or lose
    everything but their bootstrapper.
  - Nothing inside an install folder is skipped any more, however it is named. VS Code was
    losing 1,173 files under `node-compile-cache`.
  - Across all 160 apps on the test PC, the 142 that scan successfully have 424,319 files in
    their install folders, and none are missing from the inventory.
- **The main executable is the application.** Uninstallers, updaters, Windows hosts, and
  installers' cached copies are rejected, and App Paths and Start Menu shortcuts are consulted.
  Git was `unins000.exe`, an x86 file for an x64 app. It is now `git-bash.exe`. NVIDIA App was
  `rundll32.exe`, whose name alone matched 1,258 registry entries including `Classes\htmlfile`.
  It is now `NVIDIA App.exe`, with 102 entries. A bare file name no longer matches anything.
  Chrome no longer fails with "no main executable": its MSI records no files, so only App Paths
  knows where it is. A scan that finds nothing better warns when an uninstaller stands in, and
  an entry with no program at all, such as a driver, fails and says why.
- **Inno Setup logs parse properly.** The reader now follows the log's blocks and length
  prefixes instead of scanning for path text, which a path's trailing bytes defeated. Git for
  Windows went from no paths resolved to 9,220 files, and old ANSI logs work too.
- **The MSI database is read once per scan,** not up to three times.
- **Build progress and cancel.** Builds report each step with a progress bar and elapsed
  time, and can be cancelled. The window stays light while a build runs, because it no
  longer repaints continuously or touches the disk on every frame.
- **The installer opens instantly.** It reads only the manifest at startup and unpacks
  after you click, reading the package in place. Previously it read the whole package
  into memory twice before showing a window, and left it on disk while the window sat
  idle. It also warns before replacing files in a folder that has some, lists planned
  changes in a preview, and offers Close and Open install folder when done.
- **Security.**
  - PowerShell escaping now covers typographic quotes, which PowerShell also treats as
    string delimiters. A crafted file name or task name on a scanned drive could
    otherwise run as code in the elevated app.
  - System tools run by absolute path, and DLLs load from System32 only. A planted
    `sc.exe` or `dwmapi.dll` beside an elevated executable would otherwise run instead.
  - The C runtime is linked statically, so an installer can start on a clean PC.
  - The embedded stub is written straight into the installer, never through a
    predictable folder in `%TEMP%`, and staging folders have random names.
  - Upgraded `zip` to 9.2, `eframe` to 0.37, and `roxmltree` to 0.21, clearing the six
    `cargo audit` advisories. The zip advisories were not reachable from this code,
    which writes symlink entries as plain files.
- **Fixes.**
  - Restoring a captured `Path` no longer replaces the target machine's `Path`. Only the
    app's own entries are added.
  - A package is hash-checked before anything is written, so a damaged copy is refused
    instead of partly installing.
  - Clicking another app during a build no longer starts a second build that deleted the
    first one's staging folder.
  - A build that lost files no longer reads as a clean success, and one that fails its
    hash check says not to use the installer.
  - An installer of 4 GiB or more fails the build with a clear reason. Windows cannot
    start one, and it previously reported success.
  - A single file over 4 GiB no longer fails the build at the very end.
  - A failed build no longer destroys the installer from an earlier run.
  - An empty or relative install location is refused, and headless `--apply` without
    `--target-root` now fails instead of doing nothing and exiting 0.
  - A registry value whose bytes do not fit its type no longer panics the scan.
  - Text that merely starts like the install path no longer matches it: `C:\Program Files\Git`
    is not `C:\Program Files\GitHub CLI`. Registry values, environment variables, services,
    tasks, and shortcuts all compare whole paths now, ignoring case.
  - Restoring rewrites paths from the install location and not the executable's folder, so an
    app whose executable is in a subfolder keeps its layout in rewritten values.
  - `cargo test --workspace` now works without elevation. The executables' tests could
    not start under their `requireAdministrator` manifests, so the logic moved to
    libraries.
- **Tests.** The app and installer screens now have headless tests that drive the real
  UI (`egui_kittest`). Set `WAM_UI_PREVIEW_DIR` to also write PNGs of the main states. The
  end-to-end test can no longer skip its reinstall step on CI.
- **Documentation.** Rewrote the README with badges and theme-aware screenshots, added
  `SECURITY.md`, `CONTRIBUTING.md`, and `docs/signing.md`, and put the audit's open issues
  in `docs/PLAN.md`. Corrected claims that were not true: that no secrets are collected,
  that hives are mounted read-only, that shortcuts are recreated after a build from a
  connected drive, that the GUI has a hash toggle, that the Inno Setup reader is robust,
  and that folder matching is one-directional. The app's own text about offline
  shortcuts, driver handling, and restoring was corrected too.

## 0.4.2.1 — 2026-07-02

- **Licensing.** Added an MIT `LICENSE` (© 2026 Randy Northrup) and declared
  `license = "MIT"` (with `authors`) across the workspace. All dependencies are
  permissively licensed. Added an open-source attribution bundle generated by
  [`cargo about`](https://github.com/EmbarkStudios/cargo-about) (`about.toml`,
  `about.hbs`, `THIRD-PARTY-LICENSES.md`); the release workflow regenerates it and
  ships it plus `LICENSE` alongside the binaries.
- **Installers match the app's theme.** The app records its active theme in the
  package (`PackageManifest.ui_theme`), and a produced installer presents in that
  theme (older packages carry no tag and keep the default dark). Also fixed the
  installer clearing its window to eframe's near-black default instead of the theme
  background, which showed through in light mode.
- **README screenshots** of the pick → build → install flow (`docs/images/`).

## 0.4.2.0 — 2026-07-02

- **First tagged release + release automation.** Set the workspace version to
  `0.4.2` (released as `v0.4.2.0`) and added a GitHub Actions release workflow
  (`.github/workflows/release.yml`) that, on a `v*` tag, builds the installer stub
  then the app and publishes a GitHub Release with `winappmigrator.exe`, a
  portable `.zip`, and `SHA256SUMS.txt`. CI (`ci.yml`) was slimmed to quality
  gates only.
- **The app and its installers always run elevated.** Both embed a
  `requireAdministrator` manifest in every build, so Windows prompts once via UAC
  at launch and the whole session is elevated — there is no mid-task elevation
  prompt. The old "Restart as administrator" gate (and the app's
  `elevated`/`restart_as_admin` code and the offline-mount elevation guard) was
  removed as dead. The end-to-end test skips its reinstall step when it cannot
  spawn the elevated installer on a non-elevated host; CI runs elevated and
  exercises it fully.
- **Removed code signing.** The app and installers are distributed unsigned; the
  Authenticode signing hook, `docs/signing.md`, and `scripts/sign-release.ps1`
  were removed.
- **Docs/repo cleanup for accuracy.** Rewrote the README to current reality; fixed
  stale docs (references to a removed `winappmigrator-cli` crate, non-existent CLI
  flags, the removed elevation gate, an outdated `FootprintKind` list); moved
  `PLAN.md` into `docs/`; and removed decision docs (installer-strategy, sfx,
  tui-decision), the manual QA scripts, and `AGENTS.md`.
- **Production hardening pass** (cleanup, shared helpers, no silenced errors):
  - **Removed dead/legacy code** left after the offline-scan refactor: the
    live-only `list_installed_apps()` wrapper, `SystemContext::{source,describe}()`
    and the now-unused `SystemContext.source` field, `package::files_dir()`,
    `path_utils::path_eq_case_insensitive()`, and the write-only
    `InstallSource.authoritative_registry_count` field.
  - **Consolidated duplicated helpers.** The registry↔`winreg` mappings shared by
    restore and verify (`registry_root`, `environment_registry_target`,
    `registry_hive_name`, `registry_value_name`, `registry_raw_type` — including
    the load-bearing `…\Session Manager\Environment` path) now live once in a new
    `registry_target` module, so the write and verify sides can never drift.
    Three copies of a bytes→hex encoder collapsed onto `hashing::hex_lower`; the
    filesystem-name sanitizer shared by packaging and restore moved to
    `path_utils::sanitize_filename`; the GUI's two name sanitizers share one
    helper; and the installer's two staging-directory functions merged into
    `cleanup::create_staging_dir`, which (with every call site) now uses the
    `EXTRACTION_PREFIX` constant the stale-dir sweeper matches on — closing a
    latent gap where changing the prefix would silently break cleanup.
  - **Stopped silently dropping data on error.** Footprint discovery now records a
    warning when a base folder (`Program Files`, `ProgramData`, per-user
    `AppData`, …) can't be enumerated — previously an access-denied read on an
    offline/foreign disk dropped that whole category with no notice. Offline
    scheduled-task scanning warns on unreadable/malformed task XML instead of
    skipping silently; license scanning warns when a vendor sub-key can't be
    opened and still reports a license-named key's *location* even when its values
    can't be read; and the app's "Restart as administrator" action now surfaces a
    message if the elevated relaunch can't be started instead of exiting silently.
- Added **scanning an app from a connected drive** — a disk pulled from another
  (often failing) PC and mounted on the working machine (e.g. `E:\`) — instead of
  only the live OS. A new `SystemContext` abstraction (`system_context.rs`,
  `ScanSource::{Live, OfflineVolume}`) is the single seam every scanner reads
  through, replacing the two live-only seams (`RegKey::predef` and
  `std::env::var_os(known folder)`). For an offline volume it mounts the disk's
  registry hives read-only via `reg load` (using the process's backup privilege,
  so ACL-locked `config` hives read non-destructively), resolving the disk's
  active control set from `SYSTEM\Select\Current`, and unmounts them on drop;
  derives every known-folder base (`Program Files`, `ProgramData`, all user
  profiles, `Public`, `Windows`) from the volume root; and re-roots absolute
  source paths (`C:\…` → `<volume>\…`). All scanners were threaded through it —
  app inventory, registry, environment, license, install-source, and services/
  drivers (from the offline `SYSTEM` hive), with **scheduled tasks read from the
  disk's `Windows\System32\Tasks` XML** (new `roxmltree` dependency) since the
  live Task Scheduler cannot see another disk. Offline scanning **requires
  administrator rights**; the app offers a "Restart as administrator" gate. Per
  the requirement, it is **non-destructive**: the source disk is only ever read
  (backup-mode reads; never `takeown`/`icacls`). Source-shortcut enumeration is
  skipped for offline sources (the installer recreates shortcuts on restore). The
  GUI gained a **scan-source selector** ("This PC" / "Connected drive…") with a
  Windows-volume check and elevation notice.
- Added **non-destructive backup-mode file copying**: when packaging can't copy a
  file because its NTFS ACL denies this process (common on a foreign disk owned by
  another PC's SIDs), it falls back to `robocopy /B` (backup semantics via
  `SeBackupPrivilege`), which reads through the ACL without modifying the source.
  The fast in-process `fs::copy` is still tried first.
- The produced installer is now **named after the application**, not the chosen
  output folder (e.g. `YAMAHA DIAGNOSTIC SYSTEM2_installer.exe`).
- The app now **stages files on the destination drive**, so a large migration
  never fails for lack of space on the system/temp drive: the builder stages the
  package on the same drive as the chosen output folder, and each produced
  installer extracts its embedded package onto the install destination's drive
  (falling back to the installer's own drive, then the system temp drive).
- Extended footprint discovery to the **shared Public Documents** folder
  (`%PUBLIC%\Documents`, i.e. `C:\Users\Public\Documents`), which some apps use
  for machine-wide data instead of `ProgramData`. New `FootprintKind::PublicDocuments`
  (classified as `MachineAppData`) and a `UserDataAnchor::PublicProfile` so those
  files re-anchor to the same absolute `%PUBLIC%` location on restore (Public is
  fixed at `C:\Users\Public` across machines). Surfaced by YAMAHA DIAGNOSTIC
  SYSTEM2, whose diagnostic database lives at `C:\Users\Public\Documents\YDIS_Data`
  and was previously missed, so the app crashed at startup with
  `DirectoryNotFoundException` on a target.
- Turned the installer into a **graphical self-extractor** and made WinAppMigrator a **single portable executable**. WinAppMigrator (`winappmigrator.exe`) is now one file you run from anywhere: it carries the compiled installer stub inside itself (`include_bytes!`, staged by `winappmigrator-gui/build.rs`) and writes it out — plus the chosen app's package — to stamp out each installer. There is no sibling stub file to ship. Each produced installer is a standard **GUI** self-extractor: double-click opens a window showing the app (name/version/publisher), an editable **install location** defaulted to the app's original path, **Restore user data** and **Restore license** checkboxes shown *only when those were captured at build time*, a listed **Dependencies** plan (bundled / winget / Windows feature / already present), and **Preview**/**Install** actions with live progress and post-install hash verification. The installer requests administrator elevation via an embedded manifest (release builds; `embed-manifest`) so Program Files/HKLM writes succeed, and still supports a headless `--target-root … --apply` mode for automation/CI. Shared UI moved to a new `winappmigrator-ui` crate (theme + widgets) reused by both the app and the installer; the console-only `winappmigrator-cli` crate was removed (the installer stub is `winappmigrator-installer`, embedded in the app). `PackageManifest` gained `included_user_data`/`included_program_data`/`included_license_data` (drive the installer's checkboxes) and `RestoreOptions`/`VerificationOptions` gained `restore_license` to gate license restore.
- Fixed .NET runtime prerequisite detection. `pe_scan` now reads the managed image's CLR metadata-root version (`v2.0.50727` vs `v4.0.30319`), so an app hosted by the .NET Framework shim (`mscoree`) maps to **.NET Framework 3.5** (CLR v2.0/1.x) or **.NET Framework 4.x** (v4.0) instead of being mislabeled as the modern **.NET Desktop Runtime 8**. .NET Framework 3.5 is now installed on the target by enabling the **`NetFx3` Windows optional feature via DISM** (`prerequisites::run_windows_feature`, new `PrerequisiteBundle::windows_feature`), with an actionable fallback message when the payload can't be downloaded; 4.x is listed as in-box on Windows 10/11 (probe only). This was surfaced by YAMAHA DIAGNOSTIC SYSTEM2 (YdisUI.exe, CLR v2.0.50727), which the old detection would have tried to satisfy with .NET 8.
- Made the installer fully self-contained and automatic. The build output is now a single self-extracting executable whose file name encodes its contents (e.g. `App_userdata_license_installer.exe`), carrying the app, its data, license, runtime prerequisites, and drivers. (1) License data is now **restored automatically** by the installer instead of being left as a manual step: license files are placed back at their original/re-anchored locations, user-scoped (HKCU) registry values are captured and rewritten — text, numeric, and binary serials/tokens, with DPAPI-encrypted blobs restored best-effort and flagged machine-bound (may need reactivation) — and non-machine-bound framework stores are copied and restored. The misleading "restore manually" warning was removed. (2) Detected runtime prerequisites (Visual C++ runtime, etc.) are **downloaded and embedded** at build time (`ureq` over native-tls/SChannel), then installed silently and verified by registry probe on the target, with a winget/URL fallback when a download is unavailable. (3) **Application-owned drivers** (a `.sys` under the app's install footprint) are bundled and re-registered automatically (`sc create`, elevation required) and verified; vendor/inbox drivers are reported with precise, named reinstall guidance instead of the previous vague "requires a vendor driver installer" warning. Package and restore verification was extended to cover bundled-prerequisite presence, restored license files, and re-registered drivers. `PackageOptions` gained `bundle_prerequisites` and `installer_dir`; the GUI builds in a throwaway staging directory so the chosen folder receives only the single installer.
- Refocused the product to one purpose with a strict two-binary split: the **app** (`winappmigrator-gui`) reverse-engineers a selected installed application and, optionally, its license and/or user data, and builds a single self-extracting installer, hash-verifying the package; the **installer** (`winappmigrator`, the embedded stub) reinstalls that application on a separate device and hash-verifies the result. Restore lives only in the installer; the app never restores. The GUI is now a single screen (removed the Dashboard/Analyze/Package/Restore pages and navigation rail); the CLI crate is now purely the installer stub (removed the `list`/`scan`/`snapshot`/`package`/`restore`/`verify` builder commands). Deleted the inventory-snapshot feature end to end, the QEMU/WSL `vmtest/` harness, and the operator E2E scripts/docs that targeted the previous multi-command CLI. Added an end-to-end test that drives the real stub binary (reverse-engineer → build installer → reinstall on a clean target → verify) and tuned dev/release build profiles (optimized dependencies and first-party code in dev for fast tests; LTO, single codegen unit, and stripped symbols in release). No dead, orphaned, or legacy code remains; `clippy -D warnings` and the full test suite pass.
- Resolved a main-executable gap for MSI applications that record neither `InstallLocation` nor a usable `DisplayIcon` in their uninstall key (common for Windows Installer products such as PuTTY, PowerShell 7, the AWS CLI, TightVNC, and WinAppDriver): when the inventory heuristic finds no launcher, `install_source::primary_executable` now consults the authoritative install source — the Windows Installer component database for MSI products, otherwise the resolved install roots — and picks a representative non-auxiliary executable, so scanning/packaging such apps by name no longer fails with "installed app has no main executable guess."
- Made packaging resilient to locked or access-denied files: a single file that cannot be copied (a running application's binary, an AppData installer-cache entry, etc.) is now recorded as a warning and marked not-copied in the manifest instead of aborting the whole package, keeping verification consistent.
- Added optional license-data backup (`license_scan`): detects the application's own licensing/activation artifacts — license files (`.lic`/`.v2c`/`.lservrc`/`.rlm` and name-signalled ambiguous files), framework stores (CodeMeter `CmAct`, Safenet Sentinel, FlexLM), `*_LICENSE_FILE` environment variables, and vendor registry license keys — and reports each with a transferability and machine-binding hint. Opt-in in the app (default off) copies license *files* into the package `license/` directory; framework/registry/dongle artifacts are reported as metadata only. License data is never restored automatically, and WinAppMigrator never decrypts, bypasses/emulates activation, or emulates a dongle. AGENTS.md security policy updated to scope this as legitimate same-owner migration.
- Reworked user-data backup into a precise, first-class capability: machine-wide `ProgramData` is now classified as `MachineAppData` and controlled separately from genuine per-user data via `include_program_data` and the app's user-data toggle; user-data discovery expanded to `Documents` (including nested `My Games\<App>`) and `Saved Games`, resolving OneDrive/folder redirection through the `User Shell Folders` registry key; and online-only cloud placeholders (OneDrive Files On-Demand, via the OFFLINE/RECALL_ON_DATA_ACCESS attributes) are reported but never copied so a backup cannot silently hydrate or fail.
- Hardened executable reverse-engineering: parse delay-load imports (manual RVA walk of the delay directory, since `pelite` has no delay API; modern v2 descriptors, legacy v1 reported and skipped), read the embedded `RT_MANIFEST` to surface the `requestedExecutionLevel` and side-by-side `<dependentAssembly>` identities, detect managed .NET images via the CLR/COR20 data directory, and broaden runtime-prerequisite detection beyond VC++ to .NET (Framework vs self-contained), Java, Python, and Electron/Node with bundled-vs-manual classification. Confidence scoring now accounts for managed .NET and SxS dependencies. Validated against real binaries (mmc.exe: 323 delay imports + highestAvailable; regedit.exe: SxS Common-Controls; InstallUtil.exe: managed .NET).
- Narrowed the MSI component-database sweep to the SIDs that actually register the product (`component_sweep_sids`), with a full-SID fallback so results are never worse than the exhaustive scan; documented the decision to keep the exhaustive (COM-validated) sweep rather than scope via `Features`/Base85 descriptors, which could silently under-report the footprint.
- Deepened MSIX and Squirrel footprint discovery: MSIX now reads the `AppxManifest.xml` `Identity` name and anchors the app's per-user state at `%LOCALAPPDATA%\Packages\<IdentityName>_*` (a location name-token heuristics miss); Squirrel adds the install location as an authoritative root and records the current `app-<version>` directory using numeric version comparison.
- Added authoritative NSIS footprint extraction: when an NSIS install carries an `AdvUninstLog` `uninstall.log`, `install_source` reads the recorded paths (reusing the Inno UTF-16/ANSI path scan) and derives install roots and an authoritative file count instead of relying on the install directory plus heuristics alone.
- Added opt-in real-location user-data restore: `--restore-user-data` (CLI `restore`/SFX/`verify`) and a GUI checkbox re-anchor packaged `%AppData%`/`%LocalAppData%`/LocalLow/`%ProgramData%` data to the target machine's equivalent base instead of staging it under the target root. Centralized restore destination routing in `file_discovery::restore_destination_for_entry`/`split_user_data_relative` so restore and verification resolve identical paths, gated `%ProgramData%` apply behind elevation (`privilege::destination_requires_admin`), kept each destination bounded by `ensure_within_root`, and threaded the flag through `RestoreOptions`/`VerificationOptions`.
- Created Rust workspace foundation with shared core, CLI, and GUI crates.
- Added Stage 1 data models, registry inventory, PE import scanning, DLL resolution, scan orchestration, and report generation.
- Added native GUI shell using shared core scan APIs.
- Added project documentation, local agent instructions, and local quality/security configuration.
- Verified Stage 1 gates: formatting, typecheck, tests, clippy, release build, cargo audit, cargo deny, CLI list, CLI scan, and GUI smoke launch.
- Added recursive app-local dependency scanning, bounded file discovery, streaming SHA-256 hashing, and richer reports.
- Added conservative registry, service, scheduled task, and shortcut discovery.
- Added CLI and GUI package creation.
- Added restore dry-run and explicit file apply workflow with restore logs and protected-target elevation check.
- Verified package/restore E2E against an isolated executable copy; restored executable launched successfully.
- Added installed-app scope tracking for per-user versus per-machine applications.
- Added registry uninstall key capture, structured registry artifacts, `.reg` export generation, and path-rewritten registry restore.
- Added targeted registry discovery for Classes Applications and CLSID registrations to cover file associations and COM references.
- Added persistent user/machine environment variable detection, packaging, dry-run, and apply restore with path rewriting.
- Added source Windows version capture in package manifests.
- Added service, scheduled task XML, and shortcut restore apply paths with admin and overwrite checks.
- Added zip package archive creation, zip restore, and self-extracting package generation/restore.
- Added GUI controls for zip/SFX package creation and zip package restore selection.
- Added GitHub Actions Windows CI with format, typecheck, tests, clippy, audit, deny, release build, and portable zip artifact upload.
- Documented future code signing while keeping signing disabled.
- Verified real user-scoped E2E with SumatraPDF 3.6.1: install, package, uninstall, SFX dry-run, SFX apply, restored executable/version, and rewritten HKCU uninstall entry.
- Verified machine-scoped E2E with existing 7-Zip 25.01: package, zip generation, per-machine scope, HKLM registry capture, and zip dry-run restore.
- Refactored `PLAN.md` into checkbox-driven completed, remaining, milestone, gate, and definition-of-done sections.
- Added `.gitattributes` to keep repository text line endings stable.
- Added `verify` CLI workflow and shared core verification API for package integrity and post-restore checks.
- Added post-apply restore verification with JSON and Markdown verification reports.
- Added typed COM registration, file association, and driver/kernel component report models.
- Separated driver services from normal service restore and marked driver/kernel components as metadata-only manual reinstall items.
- Added GUI advanced scan settings and compact dependency graph visualization.
- Added project-local elevated machine-wide restore E2E and GUI click-through smoke scripts.
- Added project-local package/verify/restore dry-run performance regression script and guide.
- Updated `time` to 0.3.49 after local dependency policy rejected yanked 0.3.48.
- Added read-only inventory snapshot workflow in core, CLI, and GUI.
- Added GUI progress detail text for background workers.
- Added conditional manual-CI Authenticode signing hook and project-local signing helper.
- Documented installer, SFX, snapshot, and TUI release-line decisions.
- Added disposable user-scoped E2E harness and fixed scheduled task elevation handling so user tasks do not require admin unless the archived task is highest/SYSTEM scoped.
- Added disposable machine-scoped E2E harness for admin-shell certification of HKLM, service, scheduled task, common shortcut, and machine environment restore.
- Hardened scheduled task scanning against PowerShell enum/numeric JSON fields and verified both user-scoped and machine-scoped scheduled task restore E2E.
- Hardened the disposable machine E2E fixture service creation and partial-failure cleanup path.
- Verified elevated disposable machine-scoped restore apply E2E from an admin shell for files, HKLM registry, service, scheduled task, common Start Menu shortcut, and machine environment restore.
- Polished the native GUI with a light enterprise theme, status chips, grouped workflow sections, safer restore mode controls, compact result metrics, artifact folder buttons, and a light-theme dependency graph.
- Redesigned the native GUI as an `eframe`/`egui` console: full-height navigation rail, content header, routed Dashboard/Applications/Analyze/Package/Restore pages, a token-based light/dark design system (`theme.rs`), reusable theme-aware components with painter-drawn icons (`widgets.rs`), a searchable virtualized applications table, and a middle-ground guided flow ("Back up this app", default output dir) alongside the advanced controls.
- Added layered install-source reverse-engineering (`install_source.rs`): detects the install method (Windows Installer/MSI, MSIX, Squirrel, Chocolatey, Inno Setup, NSIS, Generic) and records human-readable evidence.
- Added authoritative MSI footprint enumeration by reading the Windows Installer component database (`Installer\UserData\<SID>\Components`) with squished/SQUID product GUIDs; validated against the `WindowsInstaller.Installer` COM `ComponentPath` results.
- Added authoritative Inno Setup footprint extraction by parsing the `unins000.dat` uninstall log (UTF-16/ANSI resolved paths), with detection narrowed to `unins###.dat` so a plain `uninstall.exe` is not mis-detected.
- Added App Paths registration and multi-signal install-directory roots (InstallLocation, DisplayIcon, uninstaller directory) for reliable non-MSI/EXE discovery.
- Rebuilt file discovery to be footprint-aware: scans `Program Files`, `Program Files (x86)`, `ProgramW6432`, `ProgramData`, and per-user `AppData` (Roaming/Local/LocalLow) with nested `Vendor\App` (depth-2) matching, cache/temp/log pruning, token cleaning and a generic-token stoplist to prevent cross-app false positives, and a structured `FootprintLocation` summary.
- Added `FootprintLocation`/`FootprintKind`/`InstallMethod` models; `install_method`, `install_evidence`, `authoritative_file_count`, and `footprint_locations` on `ScanReport`; a `UserData` file classification; an `include_user_data` option on scan/package; and a `--skip-user-data` CLI flag.
- Improved the installed-app main-executable guess to resolve `bin\`/`app\` launcher layouts and skip uninstaller/helper executables.
- Added `docs/discovery.md` (footprint discovery + install-source reverse-engineering) and `docs/gui.md` (GUI architecture).
