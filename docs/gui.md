# GUI Architecture

The native GUI (`winappmigrator-gui`) is an `eframe`/`egui` 0.37 desktop app.
It is the **app**: it runs on the source machine, lists installed applications,
and backs a chosen one up into a single self-extracting installer. It is a
single screen — there are no routed pages, navigation rail, or restore controls.
Reinstalling is the job of the installer it produces, on the target machine.

## Modules

The design system and widgets live in a **shared `winappmigrator-ui` crate** so
the app and the produced installer render identically:

- `winappmigrator-ui/src/theme.rs` — the design system. `Theme` holds a fully
  resolved token palette for `ThemeMode::Dark` / `Light` (accent, surfaces,
  borders, text, status colors, shadows) and `Theme::apply(ctx)` maps those
  tokens onto the global `egui` style. Radius, spacing, and control-height
  constants live here too. `install_fonts` uses the native Segoe UI and Consolas
  from the Windows font folder, and `add_fallback_fonts` loads Korean, Japanese, or
  Chinese fonts only when an app's name or publisher needs them.
- `winappmigrator-ui/src/widgets.rs` — reusable, theme-aware components: `card` /
  `card_titled`, `button` / `icon_button` / `wide_button` (`ButtonKind`
  Primary/Secondary/Ghost), `segmented`, `option_row`, `search_field`, `path_field`,
  `control_row`, `status_pill`, `badge`, `metric_tiles`, `kv_row`, `mono_path`,
  `notice`, `empty_state`, `progress_bar`, and the painter-drawn brand mark and icons
  (`paint_logo`, `paint_icon`).
- `winappmigrator-ui/src/hardening.rs` — `restrict_dll_search`, called first in both
  executables' `main`. See [signing.md](signing.md).
- `winappmigrator-ui/src/lib.rs` — `window_options`, which every window opens with, so that how one is drawn takes
  nothing from the environment. `WGPU_BACKEND` is the only thing it reads, and that can only turn Direct3D 12 off, which is
  how CI sees what a person is told when graphics cannot start.
- `winappmigrator-gui/src/lib.rs`: `WinAppMigratorApp` state and the background workers (the
  installed-app inventory, the scan of the selected app, what goes with a choice, how much room a
  drive has, and the build pipeline) over `mpsc` channels and threads.
- `winappmigrator-gui/src/worker.rs`: `LatestWorker`, one job at a time on its own thread and only
  ever the newest, for what goes stale as a person clicks.
- `winappmigrator-gui/src/views.rs` — the screen itself, implemented as
  `impl WinAppMigratorApp` so it can read and mutate app state directly.
- `winappmigrator-gui/src/footprint_view.rs`: the part of it that says what goes in the installer.
- `winappmigrator-gui/src/main.rs` — only hardening plus a call into the library. Both
  executables keep their logic in a library because they demand elevation: a test
  harness linked into the executable could not start from a normal shell.

## Layout

eframe's `App` trait uses `logic()` (poll workers) + `ui()` (draw). The screen is
master-detail, with no outer scrolling, so the list and the build button are always in
view:

```text
Panel::top("header")      → brand, inventory pill, reload, theme toggle
Panel::right("details")   → the selected app
  Panel::bottom           → output folder and Create installer, or progress + Cancel while building
  CentralPanel            → ScrollArea: title, result, facts, how big and whether it fits,
                            options, then what goes in, what might also, and what stays out
CentralPanel              → scan source, search + filter, the application table
```

A build's result appears directly under the app's title, and the panel scrolls back to
the top when one arrives. It holds the installer's path and folder button, so it is
never left below the fold.

Rows that mix labels and controls use `control_row`. A plain `ui.horizontal` starts from
a short row height and lets taller controls overflow downward, which leaves labels
visibly off-center.

## The one flow

1. **Pick an app** from the searchable, sortable, virtualized table (name, publisher,
   version, scope, arch), with an All / Per-user / Per-machine filter. Arrow keys move
   the selection and Ctrl+F focuses the search.
2. **See what goes in**, which comes from a scan that starts when the app is selected. See
   [What goes in](#what-goes-in).
3. **Choose what to include**: optionally back up the app's license/activation data,
   and optionally its user data (settings and history, Documents, Saved Games, ProgramData), and untick
   what you do not want. An installer holds one account's data. With User data
   on and more than one account to offer, a **Whose data** row of `segmented` controls appears,
   starting at the signed-in user. On a connected drive, the only account with data needs no
   asking, and otherwise it starts at whoever last signed in there. If who is signed in cannot
   be told, a notice says so and no one is chosen until someone is, even if only one account has
   data. Selecting an app looks this
   up on a worker thread, because it reads folders that may be on a failing drive. What it needs
   from each account's registry was read once, when the list of apps was.
4. **Create the installer.** One background pipeline takes the scan on the screen and what was
   chosen, works out how much room that will take and stops if it is not there, reads each file
   straight into a self-extracting `.exe`, hashing and compressing what it read, and then reads
   back what it wrote and hash-verifies it. A `BuildMonitor` reports each step and carries the
   cancel request; cancelling takes effect within a few megabytes of the file being read, or
   between steps. A package that will not fit in one executable comes out as a small loader and
   a data file, and the result says the two have to stay together.

**Selection, source, reload, and options are locked while a build runs.** A build and
its result belong to the app it started with.

A build's outcome reads differently depending on what happened: a clean success, a
warning if any file could not be captured (those files have no hash to check, so
"hashes passed" alone would be misleading), or an instruction not to use the installer
if its hash check failed.

The installer stub is **embedded inside WinAppMigrator** at build time (see
`winappmigrator-gui/build.rs`, which stages `winappmigrator-installer(.exe)` for
`include_bytes!`), so the app is a single portable executable. The stub's bytes are
written straight into the installer, never through a temp file. A release build with no stub
to embed fails, and never looks beside itself for one. Only a development build falls back to a
sibling `winappmigrator-installer.exe`.

## What goes in

Selecting an app starts a scan (`scan_app`, with a `ScanMonitor`). It writes no reports, and nothing to the drive it reads, and all it
keeps on this PC while it runs is copies of registry hives it has to load, in an admin-only folder. It stops
between its steps, and part-way through its files and its install record, when it is told to.
Scans run on one worker, one at a time and only ever the newest, so clicking down a list starts
one and stops it, and only the last goes on to the end. A finished scan is made into a `Footprint`
on that worker: every file is added up onto a row for its folder, so a list never has a row per file.

**A scan's key.** `ScanKey` holds everything its result depends on: the app, where it is read from,
the License and User data options, whose data, and what is ticked of what might also be part of it.
`keep_scan_current` compares that with what the window says, each frame. When they differ, the running
scan is told to stop and another starts, so each of those choices takes effect the same way. Messages
from a scan go by its number, so one stopped or replaced cannot turn up. A build goes by `ready_scan`,
which is `None` unless the key matches what the window says now, so one app's scan cannot become another's
build, and `create_package_from_scan` refuses a scan of anything but what it was told to build.

**What changes need another look, and which do not.** The options and whose data change what a scan reads,
so they look again, and what was unticked stays unticked, since it goes by what each thing is called. What is
last known stays on the screen, greyed, until the new scan is ready. Unticking something, or the output folder,
needs no scan: the size adds up rows there and then, and how much room there is comes from what a drive said
for that folder, asked once on a worker, since it can be a drive that takes a long time to answer. A share is not
asked about at all, as asking would connect to it. **Look again** starts another scan by hand, and past ten minutes
the screen says how long ago it was.

**A scan can get old.** A build goes by it and does not look again. Each file is hashed from what is read of it
then, so a file that changed is right. One that has gone is listed, with its own wording, and the result says so. If
the main program has gone, nothing is built, and it says to look again. The room check is made again when a build starts.

**What goes with a choice.** `what_goes_with` runs on its own worker. It takes what was chosen out of a package as it
would be, goes through the review an install goes by, and takes out what that now refuses and did not before, then does
it again, since what goes can take more with it. A service, task, or shortcut told by a path to use something in a
folder left out goes too, since the rules go by what runs, as does one told to start in such a folder. A shortcut whose icon
only is there keeps going, without the icon. What goes is unticked on its row with why, and is said in a notice under the size.
What ties the app into Windows is listed as that leaves it, so a firewall rule for a program left out is no longer on the
list. A service that needs another that was left out goes too, and so does one whose own settings name a program left out, though not
one whose settings only name a folder. A folder with a row of its own inside one left out still goes in as its row says, since a
file, or a path in a command, goes by the nearest row around it, which is how the size is added up. A folder that holds what the main program imports has no box, as what
holds the main program does not, since it could not start without it. A DLL it only loads when it needs one is not counted. The button waits for that, so what is on the screen is what is built, and a build applies the
same choices by the same code. A plain setting of the app's own that names such a folder stays. So does license data with
the License option on, wherever it is, since a license goes by that option alone. A hint under the size names what is in a
folder left out.

**The words** are the installer's. `described_rows` in `restore_review.rs` is what both go by, so a service reads the same
on both screens. Left out, in order: what looks like a credential, licenses, certificates, fonts that only go by the app's
name, and drivers that cannot be passed on, then what no installer would be allowed to set up. None has a box. A choice
only takes away from what a scan found, so there is no way to ask for one of those.

**Near matches** start unticked, with why. Ticking one looks again, since it is one more root for a scan, and what is in
a data folder is greyed unless User data is on, as that is what brings data in. See
[discovery](discovery.md#near-matches).

Lists use `ScrollArea::show_rows` past six rows, and `widgets::choice_row` and `plain_row` are all of one height so that they can.
A short list is simply in the panel's own scrolling. A row of a long list that gets the focus is brought into view by that list
and then by the panel, since a scroll area takes what is asked of it before the one around it does.

States: nothing selected, `Running` with what step it is on and a Stop button, `Ready`, `Failed` with why and Try again, and
`Stopped` with Look again. Create installer is disabled in all but `Ready`, and while what goes with a choice is being worked
out, or there is not the room.

## Scan source: this PC, a connected drive, or a disk image

A **scan-source selector** at the top of the app list chooses where installed
apps are read from:

- **This PC** (default) — the live running OS.
- **Connected drive…** — a disk from another (often failing) PC, mounted here
  (e.g. `E:\`). Picking a folder validates it is a Windows volume (`\Windows` and
  `\Users` exist) before use.
- **Disk image…** — a `.vhd` or `.vhdx` of another PC. The app attaches it read-only
  with no drive letter on a worker thread (`disk_image::AttachedImage`), says so while it
  does, and from then on it is a connected drive whose place is a folder of the app's. It is
  detached when another source is picked, on a worker thread, and when the app closes.
- **Network PC…** — another PC on the network, read through WinAppMigrator Client
  (`network_view.rs`). Until a PC is paired, three steps take the place of the app list: save
  the client to a USB drive the app lists (`Win32_LogicalDisk`, drive type 2) or to a folder; pick
  the PC, which the app asks the network for every five seconds on a worker thread, or type its
  name or address; and type the code it shows. Pairing runs on a worker thread. Once paired, the
  PC's root is the source, its apps are listed, and **Disconnect** ends the link and goes back to
  the steps. The detail panel says to pair first until then.

Under the selector, a drive or an image shows its place and which Windows it has
(`SourceIdentity::windows_name`), shortened rather than pushing **Change…** out of sight. A
Windows older than 10 gets a note of what becomes of its apps (`views::older_windows_note`).

Scanning a connected drive **requires administrator rights** (to mount copies of the
drive's registry hives and read protected files). The app already runs elevated:
both executables embed a `requireAdministrator` manifest in every build (see their
`build.rs`), so Windows prompts once via UAC at launch and the whole session is
elevated — there is no mid-task elevation prompt or gate. The source (`ScanSource`) is
cloned into the inventory and packaging worker threads, which build the `SystemContext`
there so offline hive mounts are short-lived. See `docs/discovery.md` for the offline
model.

## Where files are staged (destination-drive staging)

An app's own files are not staged: they go straight from where they are into the installer.
What has to be kept as a file along the way (reports, the manifest, license files, driver
images, and downloaded runtimes, plus a file that only a backup-mode copy can read, for as long
as it takes to read it) goes in a randomly named folder on the **same drive as the chosen output
folder**, removed when the build ends. A drive that cannot keep that private, such as FAT32 or exFAT, is not used for it, and it
goes in the system's temp folder instead. The chosen folder still receives only the
installer, and its data file if it has one, written beside where it goes under a name of its own and
renamed when finished. The installer is **named after the application** (`safe_file_stem`), e.g.
`YAMAHA_DIAGNOSTIC_SYSTEM2_installer.exe`, not after the output folder.

Each produced installer unpacks its package somewhere nobody else can have a hold on it
(`cleanup::create_staging_dir`), in this order: the admin-only folder where installs keep their records, if that is on the
install destination's drive; the destination's drive, if only administrators can go on controlling what is made there and it
has room for the package and for what gets installed; that admin-only folder on another drive; and beside the installer or in the
system's temp folder, which usually fail the same test. If nowhere passes, the install does not go ahead.

Elevated code reads from both folders, so `privilege::ProtectedFolder` restricts them to
Administrators and SYSTEM before anything is written there, and the work stops if that fails.
It also holds the folder open with no delete sharing, because an access list does not stop
whoever controls its parent from renaming it and putting another in its place. The access list
is skipped without elevation, where there is no boundary to protect. An install also takes what to
do from the package it has held open since checking it, and hashes each file again as it copies it into place.

## The produced installer (`winappmigrator-installer`)

Each installer is a standalone GUI self-extractor (its own crate, always embedding a
`requireAdministrator` manifest). Its window opens at once however large the package is,
showing a check while `EmbeddedPackage::open` hashes the whole package on a worker thread.
That also holds the installer's own file open, and a data file if it has one, so neither can be
changed or swapped while the window is up. An app that cannot run on this PC stops there, and says
why. Then it reads only the manifest, straight out of the package, and shows the app, an editable
**install location** (defaulted to the original path), how much room it needs there, **User data** /
**License** options (shown only when `PackageManifest.included_user_data` or `included_program_data`, and
`included_license_data`, are set), and a **Dependencies** list.

Before anything is installed, a card lists what the package sets up beyond files, from
`restore_review::system_changes_restoring`, which goes through the same review as the install, for the location in the box and with
the same options, so a list is kept for each state of the boxes, and they are worked out again behind the window when the location
changes. **Install** waits for that, and when clicked works the list out once more, and if it is not what was on the screen, shows
the new one and installs nothing. That is every `SystemChangeKind`: services, drivers, scheduled tasks, autostart entries, shortcuts,
`Path` changes, environment variables, shell extensions, menu commands, link types, file types, COM, firewall rules, event log
sources, fonts, ODBC drivers and sources, compatibility settings, Default apps, program names, and other settings, which is what is outside the
app's own keys and all of those. It also lists what
was refused, and when it matters, whose folders and settings per-user data goes to, and where
Documents or Saved Games go if they are somewhere it will not write. The window says whether the
installer itself is signed and by whom, or "Not signed: publisher unknown". A package's own publisher
name is not evidence of that, so the two are separate.
The package also suggests the install location, so **Install** stays disabled while it is a whole
drive, the Windows folder, a folder such as `Program Files` that holds other things side by side, or a folder with files
in it and no install on record, which is looked at again when it is clicked. Nothing a package names is touched to work
this out if it could be on the network.

**Install** and **Preview changes** check that there is room, then unpack the package on a worker
thread only at that point, hashing each file as it comes out, then run `restore_package_monitored`.
**Install** goes by those hashes to check each file before writing anything, and reads back what it
installed to verify the result. **Preview changes** only works out the changes,
checking no file hashes beyond the package's own; it lists the first 300, with what reaches
beyond files first. The result appears first in the
window, and a clean install offers **Close** and **Open install folder**. Given command-line args it instead runs headlessly
(`--target-root … --apply`) for automation and the end-to-end test. The stub reuses
`winappmigrator-ui` for its look.

While either runs, an `InstallMonitor`, which is the same `Monitor` the app's build uses, reports
each step with a count where there is one, and carries the request to cancel. That takes effect at
the next file or item, and an install that has begun then puts back what it did. A failed
install says so, lists what could not be put back and what stays, such as a runtime, and says
where the log is. The card above the options says the install can be undone.

## The uninstaller (`uninstall.rs`, `uninstall_gui.rs`)

An install leaves a bare copy of the stub in its admin-only folder, and the app's entry in Settings
runs it with `--uninstall <id>`. Its window opens at once and reads the record on a worker
thread, because checking who can change that record runs other programs. It says what will be
removed and what will stay, and, when the install put back user data or licenses, has one option
to also remove those, off unless `--remove-user-data` was given. It shows progress, and then lists what it undid, what it left and why, and what it could not do,
with where the log is if anything failed. `--quiet` does the same with no window, for
`QuietUninstallString`. An uninstall that could not finish, including one waiting for a drive that
is not connected, which is not called a failure, keeps the entry in Settings, so it can
be run again from there.

## Conventions

- Theme is cloned once per frame (`let theme = self.theme.clone();`) and passed down by
  reference, to keep borrow handling simple; widgets take `&Theme`.
- Long-running work never blocks the UI thread. Workers wake the UI with
  `request_repaint()`, and `request_repaint_after` drives animation while one runs, so
  the window does not spend a core repainting during a long build.
- No image files are drawn in the UI; icons and the brand mark are painter-drawn. The
  window and file icon comes from `winappmigrator-ui/assets`, generated by
  `scripts/make-icon.ps1`.
- `RichText::strong()` only changes color. Use `theme::semibold` for real weight.
- Colors must meet WCAG AA contrast; a test in `theme.rs` checks every text color
  against every surface in both themes.
- Custom-painted controls report themselves to screen readers with `widget_info`.
- All three windows open the same way, by `winappmigrator-ui/src/window.rs`. A window is in the mode a person chose,
  which only the app lets them do, or else the one Windows is in. A window that would not fit the screen comes to a
  size that does and maximizes, since only Windows knows exactly what of a screen can be used, and its smallest
  size comes down with it. Everything has to stay usable at those sizes, so the buttons that matter are in a
  panel pinned to the bottom, and the rest scrolls. CI opens real windows at 1024x768 and checks with Windows that
  they fit.
- No restore/verify-against-target code lives in the app. The app only builds
  and confirms the package's own hashes.

## Validation

Do **not** drive the live window with scripted mouse clicks or `SendKeys` (it is flaky
and disrupts the desktop). Instead:

- `cargo fmt`, `clippy -D warnings`, and the tests. `egui_kittest` drives the real
  screens of both the app and the installer headlessly, and needs no elevation. Writing previews draws them with
  wgpu, which needs Direct3D 12 and takes a software renderer if there is no GPU.
- Nearly all of those tests give the window a scan or a build of its own. One,
  `a_person_who_picks_an_app_and_clicks_create_gets_an_installer`, has it do both for real, by clicking,
  as far as an installer on disk, because a build from the window once failed every time with all the rest
  passing. Keep it that way. `a_person_who_clicks_through_a_real_installer_gets_what_it_says` does the same for
  the installer's window, with a real package, and `elevated_packaging.rs` installs through that route on a runner.
- To see the result, set `WAM_UI_PREVIEW_DIR` and run the UI tests. The main states of the
  app, the installer, and the uninstaller are written there as PNGs:

  ```powershell
  $env:WAM_UI_PREVIEW_DIR = "$env:TEMP\wam-preview"
  cargo test -p winappmigrator-gui -p winappmigrator-installer --lib -- --test-threads=1
  ```

- The README's screenshots in `docs/images` are copies of those previews, in both themes,
  so regenerate them when a screen changes. Each comes in `-dark` and `-light`:

  | Preview | README image |
  | --- | --- |
  | `footprint-` | `app-pick-` |
  | `footprint-left-out-folder-` | `app-choose-` |
  | `done-` | `app-done-` |
  | `installer-ready-` | `installer-` |
  | `network-found-` | `network-pair-` |

  These are in `docs/images` for the docs and the wiki, and not on the README's first screen:
  `windows-7-drive-` as `source-windows7-`, `network-steps-` and `network-paired-` as they are.
  `client-windows7.png` is the client itself, on a Windows 7 VM, taken from the VM's screen.

  They tell one story, of an app built with both options off, so the sample data has to agree with itself and with what a real
  scan of that would say.
- Launch the app and take a single screenshot. Setting `__COMPAT_LAYER=RunAsInvoker`
  starts it without the UAC prompt, for a visual check only.

The build/reinstall/verify machinery itself is covered by the `winappmigrator-core` test
suite and the end-to-end test.
