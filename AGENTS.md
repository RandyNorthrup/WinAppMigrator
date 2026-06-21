# WinAppMigrator Agent Instructions

Use `/caveman` status updates in chat: short, direct, no filler. Keep code, docs, tests, and architecture rigorous.

## Project Rules

- Build a Windows-only Rust desktop migration analyzer named `WinAppMigrator`.
- Keep core logic in `winappmigrator-core`; CLI and GUI must call shared core APIs.
- Do not add global tools, global IDE settings, machine-wide instructions, or global memory.
- Do not export secrets, credentials, or unrelated user data.
- Licensing data may be captured and restored to help an owner migrate their **own** license: detect license files, framework stores, environment variables, and registry entries on explicit opt-in (`--include-license-data` / GUI toggle, default off), and always report transferability/machine-binding. The installer restores it automatically for the same owner — license files to their original/re-anchored locations, transferable (non-machine-bound, user-scoped) registry values, and non-machine-bound framework stores; machine-bound licenses are skipped silently. Never decrypt license data, never bypass or emulate activation, and never emulate a hardware dongle.
- Restore behavior must default to dry-run and require explicit apply.
- Per-user and per-machine installations must be handled equivalently; preserve archived scope and require elevation for machine-wide registry, environment, service, task, and shortcut apply work.
- Do not copy non-app-owned shared files outside the app directory. Report shared runtimes as prerequisites.
- Package archives may be directory, zip, or generated self-extracting exe; all archive extraction must prevent path traversal.
- Keep documentation synchronized with implementation.
- Update `CHANGELOG.md` after meaningful changes.
- Update `PLAN.md` when scope, assumptions, dependencies, or gate status changes.

## Architecture

- Footprint discovery is layered: `winappmigrator-core/src/install_source.rs`
  (install-method detection + authoritative MSI component database and Inno
  Setup `unins000.dat` parsing + App Paths) and `file_discovery.rs` (heuristic
  scan of Program Files/(x86)/ProgramData/AppData with nested `Vendor\App`
  matching, cache pruning, classification, and the `FootprintLocation` summary),
  orchestrated by `scan.rs`. Read authoritative installer records before
  guessing; keep heuristics conservative and guarded against cross-app false
  positives. See `docs/discovery.md`.
- The GUI is the app (build a self-extracting installer; never restores). It is
  a single `eframe`/`egui` screen split into `theme.rs` (design tokens + style),
  `widgets.rs` (reusable components + painter-drawn icons), and `views.rs` (the
  application screen + build-result renderer). The `winappmigrator-cli` crate is
  the installer stub run on the target (reinstall + hash-verify); restore lives
  only there. See `docs/gui.md`.
- Verify GUI changes with `cargo` gates and a single launched-window screenshot;
  do **not** drive the live window with scripted mouse clicks or `SendKeys`.

## Quality Gates

Run the relevant local gates before claiming completion:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo build --workspace --release
cargo audit
cargo deny check
```

Do not bypass failing gates. Record failures and fixes.

## Code Standards

- No normal-flow panics.
- No fake implementation paths.
- No unused dependencies.
- No stale docs.
- Use Windows-safe path handling.
- Use structured errors and warnings.
- Prefer conservative scan and restore decisions.
