# WinAppMigrator Plan

## Product (locked)

WinAppMigrator backs up an installed Windows application into a single
**self-extracting installer**. There are two binaries with a strict split:

- **`winappmigrator-gui` — the app** (binary `winappmigrator.exe`). Runs on the
  source machine. Lists installed apps — from **this PC or a connected drive** (a
  disk pulled from another, often failing, PC) — the user picks one and optionally
  includes its license and/or its user data. The app reverse-engineers the
  application's full footprint and builds one self-extracting `.exe`,
  hash-verifying the package. It never restores anything, and always runs elevated
  (every build embeds a `requireAdministrator` manifest).
- **`winappmigrator-installer` — the installer stub.** Embedded into the produced
  `.exe`. Runs on the target machine: extracts its package, reinstalls the app and
  bundled data, and hash-verifies the result. Restore lives only here.

`winappmigrator-core` holds all shared logic (inventory, install-source
reverse-engineering, footprint discovery, packaging, restore, verification).

## Done

- Single-screen GUI: app list → pick → optional license / user-data toggles →
  build self-extracting installer, with progress and cancel. Self-extracting output
  is mandatory.
- Scan source selector: **this PC or a connected offline drive** (a disk from
  another PC). Offline reads load the disk's registry hives with `reg load`, read
  scheduled tasks from the on-disk Task XML, and re-root recorded paths onto the
  volume. The app writes nothing to the drive, though Windows may replay a hive's
  transaction logs on load. The app runs elevated, so there is no mid-task UAC prompt.
- Installer stub: hash-check the package → reinstall → hash-verify the reinstalled
  files; non-zero exit on any mismatch.
- Layered footprint discovery (MSI component DB, Inno `unins*.dat`, NSIS
  `uninstall.log`, App Paths, plus a Program Files/ProgramData/AppData walk), PE/delay-import/SxS/.NET analysis, runtime-prerequisite
  detection, and authoritative file counts.
- Optional per-user/ProgramData data backup and optional license detection/
  backup (opt-in; never decrypts, bypasses activation, or emulates dongles).
- End-to-end test (`crates/winappmigrator-installer/tests/installer_e2e.rs`) runs the
  real stub binary: reverse-engineer → build SFX → reinstall on a clean target →
  verify. Core unit suite covers discovery/packaging/SFX/restore/verification, and
  headless UI tests drive the real screens of the app and the installer.
- Dev/release build profiles tuned (deps + first-party optimized in dev for fast
  tests; LTO + single codegen unit + strip in release).

## Definition of done (every change)

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
cargo audit
cargo deny check
```

Do not add `--all-targets` to `cargo test`: it also runs the executables as test
harnesses, and they demand elevation, so it fails with error 740 from a normal shell.
Their tests live in libraries.

Dead, orphaned, or legacy code is not left in the tree; clippy runs with
warnings denied, so unused code fails the gate. GUI changes are verified with the
gates, the headless UI tests and their rendered previews (`WAM_UI_PREVIEW_DIR`), and
a single launched-window screenshot — never scripted clicks on a live window.

## Open issues, most important first

From the audit of 2026-11-01. Items are removed from this list as they are fixed.

**Accuracy**

1. **Offline builds.** Registry values hold `C:\` paths but the scan compares them with
   `E:\` paths, so COM, file associations, Run keys, App Paths, and environment variables
   do not match. The installer also defaults to the `E:\` location, and `UsrClass.dat` is
   not mounted.
2. **Missing integrations.** URL protocols, shell extensions, TypeLib/AppID, firewall rules,
   fonts, certificates, ODBC, event log sources, and 32-bit Run and App Paths keys.
3. **User data routing.** Documents and Saved Games restore under the install folder, not
   to the user's folders, and every profile's data collapses into one account's.

**Security and data safety**

4. **The package is not authenticated, and it directs an elevated process.** Its manifest
   can name any registry key, service, scheduled task, or shortcut, and which prerequisite
   to run. Hashes sit in the same archive they protect. Enforce app-scoped allowlists when
   restoring, take prerequisite details from compiled specs, and show persistence items in
   the preview.
5. **No rollback.** The first copy failure stops the install part-way, and existing files
   are overwritten with no backup.
6. **Secrets.** All values under an app's registry keys and license files in its install
   folder are captured even with the license option off. Apply the license classifiers
   when opted out, and redact secret-named values.

**Performance**

7. Every byte is read about five times to build and four times to install. Hash while
    copying, in parallel.
8. Compression is single-threaded, and recompresses formats that are already compressed.
9. Scheduled tasks are read through PowerShell one at a time, 13 to 20 seconds, and all are
    lost if one contains a character that breaks the JSON. The offline XML reader could
    serve live scans too.

**Features**

10. Footprint preview with include and exclude before building, offering folders that nearly
    match the product's name, which are now left out.
11. Size and free-space check before building.
12. A way past the 4 GiB limit: a small loader plus a data file beside it.
13. Installer progress, persistent logs, and a change journal enabling rollback and a
    generated uninstaller.
14. Target OS and architecture check.

## Not in scope for this release line

- MSI/MSIX/setup installer output (the self-extracting `.exe` is the product).
- A TUI.

## Code signing

Releases sign the app. The installer stub stays unsigned on purpose, and installers
are signed only by whoever builds them. See [signing.md](signing.md).
