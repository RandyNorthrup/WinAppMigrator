# GUI Architecture

The native GUI (`winappmigrator-gui`) is an `eframe`/`egui` 0.34 desktop app.
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
  tokens onto the global `egui` style. Radius/role constants live here too.
- `winappmigrator-ui/src/widgets.rs` — reusable, theme-aware components: `card` /
  `card_titled`, `button` (`ButtonKind` Primary/Secondary/Ghost), `status_pill`,
  `badge`, `metric_tiles`, `kv_row`, `mono_path`, `notice`, `empty_state`,
  `path_field`, and the painter-drawn brand mark/empty-state glyph
  (`paint_logo`, `paint_icon`) so no image assets are required.
- `src/views.rs` — the single application screen (`view_applications`): the
  apps table, the per-app migrate panel (license/user-data toggles + build
  button), and the build-result renderer. Implemented as
  `impl WinAppMigratorApp` so it can read/mutate app state directly.
- `src/main.rs` — `WinAppMigratorApp` state, the two background workers
  (installed-app inventory, and the build pipeline) over `mpsc` channels and
  threads, the app-shell layout, and small status helpers.

## Layout (eframe 0.34)

eframe 0.34's `App` trait uses `logic()` (poll workers) + `ui()` (draw), where
`ui()` receives a margin-less central `Ui`. The shell nests two panels with
`show_inside`:

```text
Panel::top("header")  → brand, subtitle, inventory pill, Reload apps, theme toggle
CentralPanel          → ScrollArea → view_applications()
```

> Note: `egui::SidePanel`/`TopBottomPanel` are deprecated aliases in 0.34 — use
> `egui::Panel::top`. The `App` trait's `update()` is deprecated in favor of
> `ui()`.

## The one flow

1. **Pick an app** from the searchable, virtualized table (name, publisher,
   version, scope, arch), with an All / Per-user / Per-machine filter.
2. **Choose what to include** in the migrate panel: optionally back up the
   app's license/activation data, and optionally its user data (settings,
   Documents, Saved Games, ProgramData). Program files are always included.
3. **Create the self-extracting installer.** One background pipeline
   reverse-engineers the app (analysis is part of building, not a separate
   step), copies its files, embeds everything in a self-extracting `.exe`, and
   hash-verifies the package. The result card shows the installer path and a
   "Hashes confirmed" status.

The installer stub is **embedded inside WinAppMigrator** at build time (see
`winappmigrator-gui/build.rs`, which stages `winappmigrator-installer(.exe)` for
`include_bytes!`), so the app is a single portable executable. At build time
`installer_stub_path()` writes the embedded stub to a temp file and uses it;
if the app was compiled without an embedded stub (a bootstrap build) it falls
back to a sibling `winappmigrator-installer.exe`.

## Scan source: this PC or a connected drive

A **scan-source selector** at the top of the app list chooses where installed
apps are read from:

- **This PC** (default) — the live running OS.
- **Connected drive…** — a disk from another (often failing) PC, mounted here
  (e.g. `E:\`). Picking a folder validates it is a Windows volume
  (`\Windows\System32\config\SOFTWARE` exists) before use.

Scanning a connected drive **requires administrator rights** (to mount the
drive's registry hives and read protected files). The app already runs elevated:
release builds embed a `requireAdministrator` manifest (see
`winappmigrator-gui/build.rs`), so Windows prompts once via UAC at launch and the
whole session is elevated — there is no mid-task elevation prompt or gate. The
source (`ScanSource`) is cloned into the inventory and packaging worker threads,
which build the `SystemContext` there so offline hive mounts are short-lived. See
`docs/discovery.md` for the offline model. Reading a foreign disk is strictly
non-destructive.

## Where files are staged (destination-drive staging)

So a large migration never fails for lack of space on the system/temp drive, the
builder stages the (potentially large) package on the **same drive as the chosen
output folder** (`build_staging_parent`), not `%TEMP%`, and cleans it up after —
the chosen folder still receives only the single installer. The installer is
**named after the application** (`safe_file_stem`), e.g.
`YAMAHA DIAGNOSTIC SYSTEM2_installer.exe`, not after the output folder. Each
produced installer likewise extracts its embedded package onto the **install
destination's drive** (falling back to the installer's own drive, then the system
temp drive).

## The produced installer (`winappmigrator-installer`)

Each installer is a standalone GUI self-extractor (its own crate, embedding a
`requireAdministrator` manifest in release so it auto-elevates; debug stays
`asInvoker` for testing). On launch it extracts its embedded package, reads the
manifest, and shows the app, an editable **install location** (defaulted to the
original path), **Restore user data** / **Restore license** checkboxes — shown
only when `PackageManifest.included_user_data`/`included_license_data` are set —
and a **Dependencies** list. **Install** runs `restore_package` +
`verify_package` on a worker thread with live progress. Given command-line args
it instead runs headlessly (`--target-root … --apply`) for automation and the
end-to-end test. The stub reuses `winappmigrator-ui` for its look.

## Conventions

- Theme is cloned per method (`let theme = self.theme.clone();`) to keep borrow
  handling simple; widgets take `&Theme`.
- Long-running work never blocks the UI thread; `request_repaint()` drives
  polling while a worker runs.
- No external fonts/images; icons and the brand mark are painter-drawn.
- No restore/verify-against-target code lives in the app. The app only builds
  and confirms the package's own hashes.

## Validation

Build/visual checks only — do **not** drive the live window with scripted mouse
clicks or `SendKeys` (it is flaky and disrupts the desktop). Verify via
`cargo fmt`/`clippy -D warnings`/tests, and by launching the app and taking a
single screenshot. The build/reinstall/verify machinery itself is covered by the
`winappmigrator-core` test suite.
