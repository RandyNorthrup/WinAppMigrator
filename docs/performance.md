# Performance Checks

Use `scripts\perf-package-restore.ps1` for a project-local regression check of package creation, package verification, and restore dry-run.

Default timing targets:

- package creation: 120 seconds
- package verification: 30 seconds
- restore dry-run: 60 seconds

Example:

```powershell
cargo build --workspace --release
.\scripts\perf-package-restore.ps1 `
  -Exe "C:\Path\To\App.exe" `
  -OutRoot ".\target\wam-perf"
```

Tune targets only with a documented reason in `PLAN.md`. The script is intentionally read/report-only except for writing test output under the selected `OutRoot`.
