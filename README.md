# WinAppMigrator

WinAppMigrator is a Windows 10/11 Rust desktop application migration analyzer and package builder. It inventories per-user and per-machine desktop applications, reverse-engineers an application's full on-disk footprint, scans an executable for PE DLL dependencies, discovers related files and Windows integrations, builds migration package directories or zip archives, and restores packages with dry-run by default.

Footprint discovery is layered: it detects *how* an app was installed and reads the authoritative installed paths where Windows records them (Windows Installer/MSI component database; Inno Setup `unins000.dat`; App Paths), then completes the picture with a conservative heuristic scan of `Program Files`, `Program Files (x86)`, `ProgramData`, and per-user `AppData` (Roaming/Local/LocalLow), including nested `Vendor\App` layouts. See [docs/discovery.md](docs/discovery.md). The native GUI is an `eframe`/`egui` console with a light/dark design system; see [docs/gui.md](docs/gui.md).

This repository is not yet a signed enterprise release. GitHub Actions CI and portable `.zip` release artifacts are configured. Authenticode signing is documented but intentionally disabled until a future release.

## Stack

- Rust workspace, edition 2024, current stable toolchain
- `winappmigrator-core` shared domain logic (inventory, install-source reverse-engineering, footprint discovery, packaging, restore, verification)
- `winappmigrator-cli` command-line frontend
- `winappmigrator-gui` native Windows GUI frontend using `eframe`/`egui` (modules `theme`, `widgets`, `views`, `main`)
- `winreg` for uninstall registry inventory, the MSI Installer component database, and App Paths
- `pelite` for PE metadata and import parsing
- `walkdir` for bounded recursive file discovery
- `sha2` for streaming SHA-256 hashes
- `filetime` for basic timestamp preservation during package creation
- `zip` for migration package archives and self-extracting package payloads
- `tempfile` for temporary extraction during zip/SFX restore
- `serde`/`serde_json` for structured reports
- `tracing` for CLI logging

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

## CLI Usage

List installed applications:

```powershell
cargo run -p winappmigrator-cli -- list
```

Scan an executable and write reports:

```powershell
cargo run -p winappmigrator-cli -- scan --exe "C:\Path\To\App.exe" --out ".\scan-output"
```

Scan an installed application by its display name (resolves the executable,
detects the install method, and reverse-engineers the footprint):

```powershell
cargo run -p winappmigrator-cli -- scan --name "Inkscape" --out ".\scan-output"
```

Exclude per-user/ProgramData application data (program files only) with
`--skip-user-data` on `scan` or `package`:

```powershell
cargo run -p winappmigrator-cli -- scan --name "Inkscape" --out ".\scan-output" --skip-user-data
```

Write an installed-application inventory snapshot:

```powershell
cargo run -p winappmigrator-cli -- snapshot --out ".\inventory-snapshot"
```

Build a migration package directory:

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage"
```

Build a migration package directory plus portable zip archive:

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage" --archive zip
```

Detect and copy the app's own licensing data (opt-in; many licenses are
machine-bound and may need reactivation):

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage" --include-license-data
```

Build a self-extracting restore package:

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage" --archive zip --sfx
```

Preview restore actions:

```powershell
cargo run -p winappmigrator-cli -- restore --package ".\AppMigrationPackage" --target-root ".\RestoredApp" --dry-run
```

Apply packaged file restore:

```powershell
cargo run -p winappmigrator-cli -- restore --package ".\AppMigrationPackage" --target-root ".\RestoredApp" --apply
```

By default, packaged per-user/ProgramData application data is staged under the
target root (`<target-root>\_external\...`). To instead restore it to the target
machine's real `%AppData%`/`%LocalAppData%`/LocalLow/`%ProgramData%` locations,
add `--restore-user-data` (restoring into `%ProgramData%` requires an elevated
shell). Pass the same flag to `verify`:

```powershell
cargo run -p winappmigrator-cli -- restore --package ".\AppMigrationPackage" --target-root ".\RestoredApp" --apply --restore-user-data
cargo run -p winappmigrator-cli -- verify --package ".\AppMigrationPackage" --target-root ".\RestoredApp" --restore-user-data
```

Verify a package and an applied restore:

```powershell
cargo run -p winappmigrator-cli -- verify --package ".\AppMigrationPackage"
cargo run -p winappmigrator-cli -- verify --package ".\AppMigrationPackage.zip" --target-root ".\RestoredApp"
```

Restore also accepts package `.zip` archives. A generated `.sfx.exe` runs as a standalone dry-run by default:

```powershell
.\AppMigrationPackage.sfx.exe --target-root ".\RestoredApp"
.\AppMigrationPackage.sfx.exe --target-root ".\RestoredApp" --apply --force
```

Global logging flags:

```powershell
--verbose
--quiet
--log-file ".\winappmigrator.log"
```

Scan output files:

- `scan_report.json` and `scan_report.md` (include the detected `install_method`, `install_evidence`, `authoritative_file_count`, and `footprint_locations` data-location summary)
- `dependency_graph.json`
- `warnings.md`
- inventory snapshots write `inventory_snapshot.json` and `inventory_snapshot.md`.
- package builds also write `manifest.json`, `restore_plan.md`, `reports\*`, `files\*`, registry/service/driver/task/shortcut/environment artifacts, optional `license\*` (with `--include-license-data`), optional `.zip`, optional `.sfx.exe`, and restore logs.
- verification writes `verification_report.json` and `verification_report.md`.

Smoke-tested in this workspace against the local Cargo executable:

```powershell
cargo run -p winappmigrator-cli -- scan --exe "$((Get-Command cargo).Source)" --out ".\scan-output-self"
```

## GUI Usage

Launch the native GUI:

```powershell
cargo run -p winappmigrator-gui
```

The GUI is an `eframe`/`egui` console with a full-height navigation rail, a
content header, and routed pages (Dashboard, Applications, Analyze, Package,
Restore), plus a light/dark theme toggle. It loads installed applications through
the shared core and runs scan/snapshot/package/zip/SFX/restore workflows on
background threads.

- **Applications**: searchable, virtualized table; selecting an app fills the
  executable path and a default output directory and offers "Back up this app".
- **Analyze** results surface the detected **install method** and the signals
  used to reverse-engineer it, a **Data locations** card (every place the app
  stores files, with sizes), the dependency graph/table, Windows integrations,
  runtime prerequisites, and warnings.
- Scan options toggle recursive app-local DLL scanning, SHA-256 hashing, and
  whether to include user data (AppData/ProgramData).

GUI architecture and conventions are documented in [docs/gui.md](docs/gui.md).
The release GUI executable was smoke-launched locally.

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
  winappmigrator-cli/
  winappmigrator-gui/
    src/
      theme.rs            # design tokens + egui style (light/dark)
      widgets.rs          # reusable theme-aware components + vector icons
      views.rs            # routed pages and result renderers
      main.rs             # app state, workers, app-shell layout, routing
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

## E2E Verification

Verified locally on 2026-06-12:

- User-scoped SumatraPDF 3.6.1 installed from the official installer, packaged by installed-app name, uninstalled, restored from generated SFX, and validated through restored executable metadata and HKCU uninstall entry rewrite.
- Machine-scoped existing 7-Zip install packaged by installed-app name and dry-run restored from generated zip. The manifest recorded `per_machine` scope and HKLM registry entries requiring admin.

VS Code was not used for E2E testing.

Verified locally on 2026-06-19:

- Full local gates passed: format, clippy, check, tests, release build, audit, and deny.
- Release CLI package/verify/zip restore dry-run/apply/verify E2E passed against an isolated executable copy.
- GUI click-through smoke script passed after the UI/UX polish pass.
- Package/verify/restore dry-run performance smoke passed within documented targets.
- Disposable user-scoped and elevated machine-scoped package/zip restore dry-run/apply/verify E2E passed, including registry, scheduled task, shortcut, environment variable, and service coverage where applicable.

Project-local operator scripts:

```powershell
.\scripts\e2e-elevated-machine-restore.ps1 -Package ".\AppMigrationPackage.zip" -TargetRoot "C:\Temp\WinAppMigrator-E2E-Restore"
.\scripts\e2e-disposable-user-scope.ps1
.\scripts\e2e-disposable-machine-scope.ps1
.\scripts\e2e-license-data.ps1
.\scripts\e2e-admin-guard-smoke.ps1
.\scripts\gui-clickthrough-smoke.ps1
.\scripts\perf-package-restore.ps1 -Exe "C:\Path\To\App.exe"
```

See `docs\discovery.md`, `docs\gui.md`, `docs\verification.md`, `docs\user-e2e.md`, `docs\elevated-e2e.md`, `docs\performance.md`, `docs\snapshots.md`, `docs\installer-strategy.md`, `docs\sfx.md`, and `docs\tui-decision.md`.

## Release Notes

Current release format is portable `.zip`. MSI/MSIX/setup installers are not selected for this release line. Authenticode signing is wired for manual CI workflows when the selected signing secrets are present.
