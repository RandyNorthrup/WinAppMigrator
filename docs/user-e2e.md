# Disposable User-Scoped Restore E2E

Use this script from a normal non-elevated PowerShell session to validate user-scoped backup and restore behavior.

```powershell
cargo build --workspace --release
.\scripts\e2e-disposable-user-scope.ps1
```

The script creates a unique disposable user-scoped app under `%LOCALAPPDATA%`, then registers:

- HKCU uninstall entry
- user scheduled task
- user Start Menu shortcut
- HKCU environment variable

It packages the app by installed-app name, verifies the package, removes the source fixture, runs restore dry-run, runs restore apply, verifies restored files and user-scoped integrations, and then cleans user artifacts unless `-KeepUserArtifacts` or `-KeepRestoredFiles` is supplied.

Evidence is written under `target\wam-user-e2e` by default.
