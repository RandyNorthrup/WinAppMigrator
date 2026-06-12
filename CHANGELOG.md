# Changelog

All notable project changes are tracked here.

## Unreleased

- Created Rust workspace foundation with shared core, CLI, and GUI crates.
- Added Stage 1 data models, registry inventory, PE import scanning, DLL resolution, scan orchestration, and report generation.
- Added native GUI shell using shared core scan APIs.
- Added project documentation, local agent instructions, and local quality/security configuration.
- Verified Stage 1 gates: formatting, typecheck, tests, clippy, release build, cargo audit, cargo deny, CLI list, CLI scan, and GUI smoke launch.
- Added recursive app-local dependency scanning, bounded file discovery, streaming SHA-256 hashing, and richer reports.
- Added conservative registry, service, scheduled task, and shortcut discovery.
- Added CLI and GUI package creation.
- Added restore dry-run and explicit file apply workflow with restore logs and protected-target elevation check.
- Verified package/restore E2E against an isolated executable copy; restored executable launched successfully.
- Added installed-app scope tracking for per-user versus per-machine applications.
- Added registry uninstall key capture, structured registry artifacts, `.reg` export generation, and path-rewritten registry restore.
- Added targeted registry discovery for Classes Applications and CLSID registrations to cover file associations and COM references.
- Added persistent user/machine environment variable detection, packaging, dry-run, and apply restore with path rewriting.
- Added source Windows version capture in package manifests.
- Added service, scheduled task XML, and shortcut restore apply paths with admin and overwrite checks.
- Added zip package archive creation, zip restore, and self-extracting package generation/restore.
- Added GUI controls for zip/SFX package creation and zip package restore selection.
- Added GitHub Actions Windows CI with format, typecheck, tests, clippy, audit, deny, release build, and portable zip artifact upload.
- Documented future code signing while keeping signing disabled.
- Verified real user-scoped E2E with SumatraPDF 3.6.1: install, package, uninstall, SFX dry-run, SFX apply, restored executable/version, and rewritten HKCU uninstall entry.
- Verified machine-scoped E2E with existing 7-Zip 25.01: package, zip generation, per-machine scope, HKLM registry capture, and zip dry-run restore.
- Refactored `PLAN.md` into checkbox-driven completed, remaining, milestone, gate, and definition-of-done sections.
- Added `.gitattributes` to keep repository text line endings stable.
