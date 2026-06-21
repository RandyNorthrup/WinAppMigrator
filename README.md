# WinAppMigrator

WinAppMigrator is a Windows 10/11 desktop app that backs up an installed application into a single **self-extracting installer**. You pick an installed app, optionally include its license and its user data, and the app reverse-engineers the application's full on-disk footprint and Windows integrations and writes one `.exe`. Run that `.exe` on another machine and it reinstalls the app — and the bundled license/user data — then hash-verifies every file.

There are two binaries with a strict split of responsibility:

- **`winappmigrator-gui` — the app.** Runs on the source machine. Lists installed apps and builds the installer. It never restores anything.
- **`winappmigrator` — the installer stub.** Embedded into the produced `.exe`. Runs on the target machine and performs the reinstall + hash verification. Restore lives only here.

Footprint discovery is layered: it detects *how* an app was installed and reads the authoritative installed paths where Windows records them (Windows Installer/MSI component database; Inno Setup `unins000.dat`; App Paths), then completes the picture with a conservative heuristic scan of `Program Files`, `Program Files (x86)`, `ProgramData`, and per-user `AppData` (Roaming/Local/LocalLow), including nested `Vendor\App` layouts. See [docs/discovery.md](docs/discovery.md). The native GUI is an `eframe`/`egui` single-screen app with a light/dark design system; see [docs/gui.md](docs/gui.md).

This repository is not yet a signed enterprise release. GitHub Actions CI and portable `.zip` release artifacts are configured. Authenticode signing is documented but intentionally disabled until a future release.

## Stack

- Rust workspace, edition 2024, current stable toolchain
- `winappmigrator-core` shared domain logic (inventory, install-source reverse-engineering, footprint discovery, packaging, restore, verification)
- `winappmigrator-cli` the self-extracting installer stub (`winappmigrator.exe`): reinstall + hash-verify of an embedded package on the target
- `winappmigrator-gui` the app: native Windows GUI using `eframe`/`egui` (modules `theme`, `widgets`, `views`, `main`)
- `winreg` for uninstall registry inventory, the MSI Installer component database, and App Paths
- `pelite` for PE metadata and import parsing
- `walkdir` for bounded recursive file discovery
- `sha2` for streaming SHA-256 hashes
- `filetime` for basic timestamp preservation during package creation
- `zip` for migration package archives and self-extracting package payloads
- `tempfile` for temporary extraction during installer reinstall
- `serde`/`serde_json` for structured reports
- `tracing` for core diagnostics

## Requirements

- Windows 10 or Windows 11
- Rust stable toolchain with MSVC target
- Optional local security tools already supported by this repo: `cargo audit`, `cargo deny`

No environment variables are required.

## Development Commands

```powershell
cargo fmt
cargo fmt --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace --release
cargo audit
cargo deny check
```

`cargo quality` is a local alias for the workspace test command only. Run the full command list above for the complete quality gate.

Release binaries are written to:

- `target\release\winappmigrator.exe`
- `target\release\winappmigrator-gui.exe`

## Building an installer (the app)

Launch the app on the source machine:

```powershell
cargo run -p winappmigrator-gui
```

The app is a single screen:

1. **Pick an installed app** from the searchable table (filter by All / Per-user
   / Per-machine). Selecting a row detects its install method, reverse-engineers
   its footprint, and prefills a default output folder.
2. **Optionally include** the app's license/activation data and/or its user data
   (settings, Documents, Saved Games, ProgramData) with two checkboxes. Program
   files are always included.
3. **Create the self-extracting installer.** Building reverse-engineers the app,
   copies its files and the selected data, embeds everything in one `.exe`, and
   hash-verifies the package. The result card shows the installer path and a
   "Hashes confirmed" status.

`winappmigrator.exe` (the installer stub) must sit beside the GUI executable —
the build uses it as the self-extracting stub. Release builds place both at
`target\release\`.

The app never restores anything; it only builds and confirms the package's own
hashes. GUI architecture is documented in [docs/gui.md](docs/gui.md).

## Running the installer (the target)

Copy the produced `<App>.sfx.exe` to the target machine and run it. It extracts
its embedded package, reinstalls the application (and any bundled license/user
data), then hash-verifies the result.

```powershell
# Preview the reinstall (default):
.\App.sfx.exe --target-root "C:\Program Files\App"

# Reinstall:
.\App.sfx.exe --target-root "C:\Program Files\App" --apply --force
```

By default, packaged per-user/ProgramData data is staged under the target root
(`<target-root>\_external\...`). To instead reinstall it into the target
machine's real `%AppData%`/`%LocalAppData%`/LocalLow/`%ProgramData%` locations,
add `--restore-user-data` (writing into `%ProgramData%` requires an elevated
shell). The installer exits non-zero if any package or reinstall hash fails to
verify.

Package contents (inside the embedded payload):

- `manifest.json`, `restore_plan.md`, `reports\*`, `files\*`,
  registry/service/driver/task/shortcut/environment artifacts, and optional
  `license\*` (when license backup was selected).
- `verification_report.json` / `verification_report.md` are written by the
  installer during the hash check.

## CI And Release

GitHub Actions is configured in `.github/workflows/ci.yml`.

The CI gate runs on `windows-latest`:

- format check
- typecheck
- tests
- clippy with warnings denied
- `cargo audit`
- `cargo deny check`
- release build
- conditional manual-workflow Authenticode signing when signing secrets are present
- portable `WinAppMigrator-portable-windows-x64.zip` artifact upload

Signing policy and secret names are documented in `docs/signing.md`.

## Project Structure

```text
Cargo.toml
crates/
  winappmigrator-core/
    src/
      app_inventory.rs    # installed-app inventory from uninstall registry
      install_source.rs   # install-method detection + MSI/Inno authoritative paths
      file_discovery.rs   # footprint roots, walk, classification, FootprintLocation
      scan.rs             # scan orchestration (PE, deps, integrations, footprint)
      package.rs / restore.rs / verify.rs / reporting.rs / models.rs ...
  winappmigrator-cli/       # the installer stub: reinstall + hash-verify on target
  winappmigrator-gui/       # the app: build a self-extracting installer
    src/
      theme.rs            # design tokens + egui style (light/dark)
      widgets.rs          # reusable theme-aware components + vector icons
      views.rs            # the single application screen + build-result renderer
      main.rs             # app state, inventory + build workers, app-shell layout
docs/
scripts/
samples/
tests/
```

## Security Notes

- Scan/list operations do not intentionally modify the system. Footprint discovery and install-source reverse-engineering are read-only: they read the registry (uninstall keys, MSI Installer component database, App Paths), parse the Inno Setup uninstall log, and walk install/data directories without writing.
- Genuine per-user data (Roaming/Local/LocalLow AppData, Documents incl. `My Games`, Saved Games) is treated separately from machine-wide `ProgramData`. Each is included by default and can be excluded independently with `--skip-user-data` / `--skip-program-data` (CLI) or the corresponding toggles (GUI). `Documents`/`Saved Games` follow OneDrive/folder redirection via `User Shell Folders`. Cache/temp/log directories are pruned, and online-only cloud placeholders (OneDrive Files On-Demand) are reported but never copied.
- Restore defaults to dry-run. File writes require explicit `--apply` or the GUI apply checkbox.
- Restore stages user/ProgramData data under the target root by default; restoring it to the real `%AppData%`/`%ProgramData%` locations is opt-in (`--restore-user-data`), bounded to each known base, and blocks `%ProgramData%` apply when not elevated.
- Restore blocks apply into protected Windows locations when not elevated.
- Per-machine registry, services, scheduled tasks, public desktop, common Start Menu, and protected target locations require elevation for apply.
- DLL resolution reports system files but package creation copies only app-owned files.
- Non-app-owned shared files outside the app directory are never copied; shared runtimes are reported as prerequisites.
- Registry, environment variable, service, shortcut, and scheduled task restore paths are archived and apply-capable when privileges and overwrite policy allow.
- Package and restore verification checks file hashes, rewritten registry values, environment variables, services, scheduled tasks, and shortcuts.
- Driver and kernel components are reported as metadata-only manual reinstall items; WinAppMigrator does not copy or register drivers automatically.
- Secrets and credentials are not collected. Licensing data is detected only to help an owner migrate their own license: it is opt-in (`--include-license-data`), reports transferability/machine-binding, copies license files only into the package `license/` directory, and is never restored automatically. WinAppMigrator never decrypts license data, bypasses or emulates activation, or emulates a hardware dongle.

## Troubleshooting

- If app inventory is empty, run the CLI without elevation first, then try an elevated shell only if the target app is machine-wide and registry permissions require it.
- If a scan reports missing DLLs, check whether the dependency is an API-set or runtime prerequisite before copying any file manually.
- If GUI rendering fails, verify the installed GPU driver and try `cargo check -p winappmigrator-gui` to separate build issues from runtime graphics issues.

## Verification

The whole product pipeline is exercised end-to-end by an integration test
(`crates/winappmigrator-cli/tests/installer_e2e.rs`): it reverse-engineers a real
executable, builds the self-extracting installer with the actual stub binary,
runs that installer against a clean target directory, and asserts the reinstall
plus hash verification pass (the installer exits non-zero on any hash mismatch).
The `winappmigrator-core` suite covers footprint discovery, packaging, SFX
embed/extract, restore, and verification at unit level. Run the full gate:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo build --workspace --release
.\scripts\gui-clickthrough-smoke.ps1   # launches the release GUI and exits
```

See `docs\discovery.md`, `docs\gui.md`, `docs\installer-strategy.md`, `docs\sfx.md`, and `docs\tui-decision.md`.

## Release Notes

Current release format is portable `.zip`. MSI/MSIX/setup installers are not selected for this release line. Authenticode signing is wired for manual CI workflows when the selected signing secrets are present.
