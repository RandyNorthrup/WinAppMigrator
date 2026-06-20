# Elevated Machine-Wide Restore E2E

Use this guide from an elevated PowerShell session when validating a machine-scoped package that contains HKLM registry entries, services, scheduled tasks, machine environment variables, public shortcuts, or protected target paths.

## Preconditions

- Build release binaries first:

```powershell
cargo build --workspace --release
```

- Use a package created from a known machine-scoped application.
- Choose a disposable target root.
- Review `restore_plan.md` and `manifest.json` before apply.
- Confirm the package does not contain secrets, license bypass material, or unrelated user data.

## Dry-Run

```powershell
.\scripts\e2e-elevated-machine-restore.ps1 `
  -Package ".\AppMigrationPackage.zip" `
  -TargetRoot "C:\Temp\WinAppMigrator-E2E-Restore"
```

## Apply

Run from an elevated PowerShell session:

```powershell
.\scripts\e2e-elevated-machine-restore.ps1 `
  -Package ".\AppMigrationPackage.zip" `
  -TargetRoot "C:\Temp\WinAppMigrator-E2E-Restore" `
  -Apply `
  -Force
```

The script performs package verification, dry-run restore, apply restore, and post-restore verification. Restore remains dry-run unless `-Apply` is supplied.

## Disposable Machine-Scoped E2E

For certification, prefer the disposable machine-scope harness because it creates and cleans up its own HKLM uninstall entry, Windows service, scheduled task, common Start Menu shortcut, machine environment variable, package, restore target, and verification reports.

Run from an elevated PowerShell session:

```powershell
.\scripts\e2e-disposable-machine-scope.ps1
```

The script:

- creates a unique disposable app under `Program Files`
- registers machine-scoped integrations
- packages the app by installed-app name
- verifies the package
- removes the source machine-scoped fixture
- performs restore dry-run
- performs restore apply
- verifies files, HKLM registry, service, scheduled task, common shortcut, and machine environment variable
- cleans machine-scoped artifacts after the run unless `-KeepMachineArtifacts` is supplied

## Non-Admin Guard Smoke

Run from a non-elevated shell:

```powershell
.\scripts\e2e-admin-guard-smoke.ps1
```

This verifies that apply mode refuses admin-only restore plans before making changes.

## Required Evidence

- `restore_log.json`
- `restore_log.md`
- `verification_report.json`
- `verification_report.md`
- Console output showing zero verification errors or a documented reason for each remaining error.
- For disposable E2E, preserve the evidence directory printed by the script.

Driver and kernel components are metadata-only. Validate them with the vendor's signed driver installer; WinAppMigrator does not copy or register drivers automatically.
