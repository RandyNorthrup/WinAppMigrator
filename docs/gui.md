# GUI Architecture

The native GUI (`winappmigrator-gui`) is an `eframe`/`egui` 0.34 desktop app.
It is the **app**: it runs on the source machine, lists installed applications,
and backs a chosen one up into a single self-extracting installer. It is a
single screen — there are no routed pages, navigation rail, or restore controls.
Reinstalling is the job of the installer it produces, on the target machine.

## Modules

- `src/theme.rs` — the design system. `Theme` holds a fully resolved token
  palette for `ThemeMode::Dark` / `Light` (accent, surfaces, borders, text,
  status colors, shadows) and `Theme::apply(ctx)` maps those tokens onto the
  global `egui` style. Radius/role constants live here too.
- `src/widgets.rs` — reusable, theme-aware components: `card` / `card_titled`,
  `button` (`ButtonKind` Primary/Secondary/Ghost), `status_pill`, `badge`,
  `metric_tiles`, `kv_row`, `mono_path`, `notice`, `empty_state`, `path_field`,
  and the painter-drawn brand mark/empty-state glyph (`paint_logo`,
  `paint_icon`) so no image assets are required.
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

The build requires the installer stub (`winappmigrator.exe`) beside the GUI; if
it is missing the build button is disabled with a clear notice, because the
product is always a self-extracting installer.

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
