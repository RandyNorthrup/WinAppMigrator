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
- `winappmigrator-gui/src/lib.rs` — `WinAppMigratorApp` state and the two background
  workers (installed-app inventory and the build pipeline) over `mpsc` channels and
  threads.
- `winappmigrator-gui/src/views.rs` — the screen itself, implemented as
  `impl WinAppMigratorApp` so it can read and mutate app state directly.
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
  Panel::bottom           → Create installer, or progress + Cancel while building
  CentralPanel            → ScrollArea: title, result, facts, options, output folder
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
2. **Choose what to include**: optionally back up the app's license/activation data,
   and optionally its user data (settings, Documents, Saved Games, ProgramData).
   Program files are always included. An installer holds one account's data. With User data
   on and more than one account to offer, a **Whose data** row of `segmented` controls appears,
   starting at the signed-in user. On a connected drive, the only account with data needs no
   asking, and otherwise it starts at whoever last signed in there. If who is signed in cannot
   be told, a notice says so and no one is chosen until someone is. Selecting an app looks this
   up on a worker thread, because it reads folders that may be on a failing drive. What it needs
   from each account's registry was read once, when the list of apps was.
3. **Create the installer.** One background pipeline reverse-engineers the app
   (analysis is part of building, not a separate step), copies its files, embeds
   everything in a self-extracting `.exe`, and hash-verifies the package. A
   `BuildMonitor` reports each step and carries the cancel request; cancelling takes
   effect at the next file or between steps. The scan itself cannot stop part-way, so
   a cancel during it takes effect when it ends.

**Selection, source, reload, and options are locked while a build runs.** A build and
its result belong to the app it started with.

A build's outcome reads differently depending on what happened: a clean success, a
warning if any file could not be captured (those files have no hash to check, so
"hashes passed" alone would be misleading), or an instruction not to use the installer
if its hash check failed.

The installer stub is **embedded inside WinAppMigrator** at build time (see
`winappmigrator-gui/build.rs`, which stages `winappmigrator-installer(.exe)` for
`include_bytes!`), so the app is a single portable executable. The stub's bytes are
written straight into the installer, never through a temp file. If the app was compiled
without an embedded stub (a bootstrap build) it falls back to a sibling
`winappmigrator-installer.exe`.

## Scan source: this PC or a connected drive

A **scan-source selector** at the top of the app list chooses where installed
apps are read from:

- **This PC** (default) — the live running OS.
- **Connected drive…** — a disk from another (often failing) PC, mounted here
  (e.g. `E:\`). Picking a folder validates it is a Windows volume (`\Windows` and
  `\Users` exist) before use.

Scanning a connected drive **requires administrator rights** (to mount copies of the
drive's registry hives and read protected files). The app already runs elevated:
both executables embed a `requireAdministrator` manifest in every build (see their
`build.rs`), so Windows prompts once via UAC at launch and the whole session is
elevated — there is no mid-task elevation prompt or gate. The source (`ScanSource`) is
cloned into the inventory and packaging worker threads, which build the `SystemContext`
there so offline hive mounts are short-lived. See `docs/discovery.md` for the offline
model.

## Where files are staged (destination-drive staging)

So a large migration never fails for lack of space on the system/temp drive, the
builder stages the (potentially large) package on the **same drive as the chosen
output folder**, not `%TEMP%`, in a randomly named folder removed when the build ends —
the chosen folder still receives only the single installer. The installer is **named
after the application** (`safe_file_stem`), e.g.
`YAMAHA_DIAGNOSTIC_SYSTEM2_installer.exe`, not after the output folder. Each produced
installer likewise extracts its embedded package onto the **install destination's
drive** (falling back to the installer's own drive, then the system temp drive).

Elevated code reads from both folders, so `privilege::ProtectedFolder` restricts them to
Administrators and SYSTEM before anything is written there, and the work stops if that fails.
It also holds the folder open with no delete sharing, because an access list does not stop
whoever controls its parent from renaming it and putting another in its place. The access list
is skipped without elevation, where there is no boundary to protect.

## The produced installer (`winappmigrator-installer`)

Each installer is a standalone GUI self-extractor (its own crate, always embedding a
`requireAdministrator` manifest). Its window opens at once however large the package is,
showing a check while `EmbeddedPackage::open` hashes the whole package on a worker thread.
That also holds the installer's own file open, so it cannot be changed or swapped while
the window is up. Then it reads only the manifest, straight out of the package, and shows the app, an editable **install location** (defaulted to the
original path), **User data** / **License** options — shown only when `PackageManifest.included_user_data`
or `included_program_data`, and `included_license_data`, are set — and a
**Dependencies** list.

Before anything is installed, a card lists what the package sets up beyond files: services,
drivers, scheduled tasks, autostart entries, shortcuts, and `Path` changes, from
`restore_review::system_changes`, which uses the same review as the install. It also lists what
was refused, and when it matters, whose folders and settings per-user data goes to, and where
Documents or Saved Games go if they are somewhere it will not write. The window says whether the
installer itself is signed and by whom, or "Not signed: publisher unknown". A package's own publisher
name is not evidence of that, so the two are separate.
The package also suggests the install location, so **Install** stays disabled while it is a whole
drive, the Windows folder, or a folder such as `Program Files` that holds other things side by side.

**Install** and **Preview changes** unpack the package on a worker thread only at that
point, then run `restore_package_monitored`. **Install** hash-checks each file before writing
anything and verifies the result afterward. **Preview changes** only works out the changes,
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
removed and what will stay, has one option, to also remove user data and licenses, which is off,
shows progress, and then lists what it undid, what it left and why, and what it could not do,
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
- No restore/verify-against-target code lives in the app. The app only builds
  and confirms the package's own hashes.

## Validation

Do **not** drive the live window with scripted mouse clicks or `SendKeys` (it is flaky
and disrupts the desktop). Instead:

- `cargo fmt`, `clippy -D warnings`, and the tests. `egui_kittest` drives the real
  screens of both the app and the installer headlessly, and needs no elevation or GPU.
- To see the result, set `WAM_UI_PREVIEW_DIR` and run the UI tests. The main states of the
  app, the installer, and the uninstaller are written there as PNGs:

  ```powershell
  $env:WAM_UI_PREVIEW_DIR = "$env:TEMP\wam-preview"
  cargo test -p winappmigrator-gui -p winappmigrator-installer --lib -- --test-threads=1
  ```

- The README's screenshots in `docs/images` are copies of those previews, in both themes,
  so regenerate them when a screen changes.
- Launch the app and take a single screenshot. Setting `__COMPAT_LAYER=RunAsInvoker`
  starts it without the UAC prompt, for a visual check only.

The build/reinstall/verify machinery itself is covered by the `winappmigrator-core` test
suite and the end-to-end test.
