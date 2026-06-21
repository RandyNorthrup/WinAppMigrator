# WinAppMigrator Plan

## Product (locked)

WinAppMigrator backs up an installed Windows application into a single
**self-extracting installer**. There are two binaries with a strict split:

- **`winappmigrator-gui` — the app.** Runs on the source machine. Lists
  installed apps; the user picks one and optionally includes its license and/or
  its user data. The app reverse-engineers the application's full footprint and
  builds one self-extracting `.exe`, hash-verifying the package. It never
  restores anything.
- **`winappmigrator` (cli crate) — the installer stub.** Embedded into the
  produced `.exe`. Runs on the target machine: extracts its package, reinstalls
  the app and bundled data, and hash-verifies the result. Restore lives only
  here.

`winappmigrator-core` holds all shared logic (inventory, install-source
reverse-engineering, footprint discovery, packaging, restore, verification).

## Done

- Single-screen GUI: app list → pick → optional license / user-data toggles →
  build self-extracting installer → "Hashes confirmed". Self-extracting output
  is mandatory.
- Installer stub: extract embedded package → reinstall → hash-verify package and
  reinstalled files; non-zero exit on any mismatch.
- Layered footprint discovery (MSI component DB, Inno `unins*.dat`, NSIS
  `uninstall.log`, App Paths, plus a conservative Program Files/ProgramData/
  AppData walk), PE/delay-import/SxS/.NET analysis, runtime-prerequisite
  detection, and authoritative file counts.
- Optional per-user/ProgramData data backup and optional license detection/
  backup (opt-in; never decrypts, bypasses activation, or emulates dongles).
- End-to-end test (`crates/winappmigrator-cli/tests/installer_e2e.rs`) runs the
  real stub binary: reverse-engineer → build SFX → reinstall on a clean target →
  verify. Core unit suite covers discovery/packaging/SFX/restore/verification.
- Dev/release build profiles tuned (deps + first-party optimized in dev for fast
  tests; LTO + single codegen unit + strip in release).

## Definition of done (every change)

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo build --workspace --release
```

Dead, orphaned, or legacy code is not left in the tree; clippy runs with
warnings denied, so unused code fails the gate. GUI changes are verified with the
gates plus a single launched-window screenshot — never scripted clicks.

## Not in scope for this release line

- MSI/MSIX/setup installer output (the self-extracting `.exe` is the product).
- A TUI. Authenticode signing is wired for manual CI but disabled by default.
