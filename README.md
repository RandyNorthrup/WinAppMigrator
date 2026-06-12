# WinAppMigrator

WinAppMigrator is a Windows 10/11 Rust desktop application migration analyzer and package builder. It inventories per-user and per-machine desktop applications, scans an executable for PE DLL dependencies, discovers related files and Windows integrations, builds migration package directories or zip archives, and restores packages with dry-run by default.

This repository is not yet a signed enterprise release. GitHub Actions CI and portable `.zip` release artifacts are configured. Authenticode signing is documented but intentionally disabled until a future release.

## Stack

- Rust workspace, edition 2024, current stable toolchain
- `winappmigrator-core` shared domain logic
- `winappmigrator-cli` command-line frontend
- `winappmigrator-gui` native Windows GUI frontend using `eframe`/`egui`
- `winreg` for uninstall registry inventory
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

Build a migration package directory:

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage"
```

Build a migration package directory plus portable zip archive:

```powershell
cargo run -p winappmigrator-cli -- package --exe "C:\Path\To\App.exe" --out ".\AppMigrationPackage" --archive zip
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

- `scan_report.json`
- `scan_report.md`
- `dependency_graph.json`
- `warnings.md`
- package builds also write `manifest.json`, `restore_plan.md`, `reports\*`, `files\*`, registry/service/task/shortcut/environment artifacts, optional `.zip`, optional `.sfx.exe`, and restore logs.

Smoke-tested in this workspace against the local Cargo executable:

```powershell
cargo run -p winappmigrator-cli -- scan --exe "$((Get-Command cargo).Source)" --out ".\scan-output-self"
```

## GUI Usage

Launch the native GUI:

```powershell
cargo run -p winappmigrator-gui
```

The GUI loads installed applications through the shared core, accepts a manual executable path and output directory, runs scan/package/zip/SFX/restore workflows on background threads, and shows status, warnings, confidence score, and report/package/restore summaries.

The release GUI executable was smoke-launched locally and remained running until the verification command stopped it.

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
- portable `WinAppMigrator-portable-windows-x64.zip` artifact upload

Signing is disabled for this milestone. Future signing requirements are documented in `docs/signing.md`.

## Project Structure

```text
Cargo.toml
crates/
  winappmigrator-core/
  winappmigrator-cli/
  winappmigrator-gui/
docs/
samples/
tests/
```

## Security Notes

- Scan/list operations do not intentionally modify the system.
- Restore defaults to dry-run. File writes require explicit `--apply` or the GUI apply checkbox.
- Restore blocks apply into protected Windows locations when not elevated.
- Per-machine registry, services, scheduled tasks, public desktop, common Start Menu, and protected target locations require elevation for apply.
- DLL resolution reports system files but package creation copies only app-owned files.
- Non-app-owned shared files outside the app directory are never copied; shared runtimes are reported as prerequisites.
- Registry, environment variable, service, shortcut, and scheduled task restore paths are archived and apply-capable when privileges and overwrite policy allow.
- Secrets and license material are not collected.

## Troubleshooting

- If app inventory is empty, run the CLI without elevation first, then try an elevated shell only if the target app is machine-wide and registry permissions require it.
- If a scan reports missing DLLs, check whether the dependency is an API-set or runtime prerequisite before copying any file manually.
- If GUI rendering fails, verify the installed GPU driver and try `cargo check -p winappmigrator-gui` to separate build issues from runtime graphics issues.

## E2E Verification

Verified locally on 2026-06-12:

- User-scoped SumatraPDF 3.6.1 installed from the official installer, packaged by installed-app name, uninstalled, restored from generated SFX, and validated through restored executable metadata and HKCU uninstall entry rewrite.
- Machine-scoped existing 7-Zip install packaged by installed-app name and dry-run restored from generated zip. The manifest recorded `per_machine` scope and HKLM registry entries requiring admin.

VS Code was not used for E2E testing.

## Release Notes

Current release format is portable `.zip`. Installer/MSIX/MSI packaging remains future work. Code signing is documented but disabled.
