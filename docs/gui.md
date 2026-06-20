# GUI Architecture

The native GUI (`winappmigrator-gui`) is an `eframe`/`egui` 0.34 desktop app
structured as an enterprise "console": a full-height navigation rail, a content
header, and routed pages, with a cohesive light/dark design system.

## Modules

- `src/theme.rs` — the design system. `Theme` holds a fully resolved token
  palette for `ThemeMode::Dark` / `Light` (accent, surfaces, borders, text,
  status colors, shadows) and `Theme::apply(ctx)` maps those tokens onto the
  global `egui` style (widget visuals, selection, panel/window fills, text
  styles, spacing, corner radii). Radius/role constants live here too.
- `src/widgets.rs` — reusable, theme-aware components: `card` / `card_titled`,
  `button` (`ButtonKind` Primary/Secondary/Danger/Ghost), `status_pill`,
  `badge`, `metric_tiles`, `kv_row`, `mono_path`, `notice`, `empty_state`,
  `path_field`, `nav_item`, and hand-drawn vector icons (`Icon`, `paint_icon`,
  `paint_logo`) so no image assets are required.
- `src/views.rs` — one method per page plus the result renderers (dependency
  graph/table, Windows-integration collapsibles, **Data locations** footprint
  card, verification, etc.). Implemented as `impl WinAppMigratorApp` so pages can
  read/mutate app state directly.
- `src/main.rs` — `WinAppMigratorApp` state, background worker orchestration
  (scan/package/restore/snapshot/inventory over `mpsc` channels and threads),
  the app-shell layout, routing, and small status helpers.

## Layout (eframe 0.34)

eframe 0.34's `App` trait uses `logic()` (poll workers) + `ui()` (draw), where
`ui()` receives a margin-less central `Ui`. The shell nests panels with
`show_inside`:

```text
Panel::left("nav")      → full-height navigation rail (brand, items, footer)
Panel::top("topbar")    → page title, status pills, Reload apps, theme toggle
CentralPanel            → ScrollArea → draw_active_view() (the routed page)
```

> Note: `egui::SidePanel`/`TopBottomPanel` are deprecated aliases in 0.34 — use
> `egui::Panel::left`/`top` with `.exact_size`. The `App` trait's `update()` is
> deprecated in favor of `ui()`.

## Pages (`View`)

- **Dashboard** — KPI tiles, a 4-stage migration workflow with status, quick
  actions, environment info.
- **Applications** — searchable/virtualized table of installed apps (name,
  publisher, version, scope, arch) with scope filter and a "Save snapshot"
  action; selecting a row fills the executable path and a default output dir and
  shows a detail card ("Back up this app" / "Analyze").
- **Analyze** — source + scan options (recursive DLLs, SHA-256, include user
  data), and the results: summary metrics, **install method + evidence**, the
  **Data locations** footprint card, dependency graph + table, Windows
  integrations (registry/COM/services/drivers/tasks/shortcuts/env), runtime
  prerequisites, and warnings.
- **Package** — source, contents (include user data), archive options
  (zip/SFX), and result artifacts.
- **Restore** — package + target, dry-run/apply segmented control, force,
  result metrics + verification + logged changes.

## UX intent (middle ground)

Keep the common path simple — pick an app, click "Back up this app", review the
prefilled Package page — while leaving all advanced controls available (custom
paths, scan toggles, dry-run vs apply, include/exclude user data). The heavy
discovery work runs on background threads with progress text.

## Conventions

- Theme is cloned per view method (`let theme = self.theme.clone();`) to keep
  borrow handling simple; widgets take `&Theme`.
- Long-running work never blocks the UI thread; `request_repaint()` drives
  polling while a worker runs.
- No external fonts/images; icons and the brand mark are painter-drawn.

## Validation

Build/visual checks only — do **not** drive the live window with scripted mouse
clicks or `SendKeys` (it is flaky and disrupts the desktop). Verify via
`cargo fmt`/`clippy -D warnings`/tests, by launching the app and taking a single
screenshot, or by exercising scan/package/restore logic through the CLI.
