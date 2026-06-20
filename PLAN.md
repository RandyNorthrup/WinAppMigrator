# WinAppMigrator Plan

Current status: functional CLI/GUI MVP with scan, inventory snapshot, package, archive, SFX, restore, verification, typed Windows integration reporting, signing hooks, and project-local E2E scripts implemented. User-scoped and elevated machine-scoped disposable E2E certification now pass with files, registry, services, scheduled tasks, shortcuts, and environment variables verified.

Footprint discovery engine, console GUI, and docs are committed (4da43ef). Most recent change (2026-06-20): opt-in real-location user-data restore (`--restore-user-data`) that re-anchors packaged `%AppData%`/`%ProgramData%` data to the target machine's equivalent base, with destination routing centralized in `file_discovery::restore_destination_for_entry`/`split_user_data_relative`, ProgramData apply gated behind elevation, and the flag threaded through CLI/SFX/GUI/verification. Full local gates pass on the current workstation (rustc 1.96.0): fmt, clippy `-D warnings`, 42 core + 8 CLI tests, release build, `cargo audit`, `cargo deny`.

## At A Glance

### Completed

- [x] Rust workspace created with shared core, CLI, and GUI crates.
- [x] Shared serializable domain models and structured error types added.
- [x] CLI `list`, `scan`, `package`, and `restore` commands implemented.
- [x] CLI `snapshot` command implemented for read-only inventory snapshots.
- [x] Native GUI shell implemented with background scan/package/restore workers.
- [x] Installed app inventory supports HKLM 64-bit, HKLM 32-bit, and HKCU uninstall registry locations.
- [x] Per-user and per-machine install scope is recorded and surfaced.
- [x] Scan by executable path implemented.
- [x] Scan by installed app name implemented.
- [x] PE architecture and direct import parsing implemented.
- [x] Basic recursive app-local DLL dependency scanning implemented.
- [x] Basic DLL resolution and classification implemented.
- [x] Bounded file discovery implemented.
- [x] Streaming SHA-256 hashing implemented.
- [x] JSON, Markdown, dependency graph, and warnings report output implemented.
- [x] Conservative registry, service, scheduled task, shortcut, and environment variable discovery implemented.
- [x] Package directory creation implemented.
- [x] Optional `.zip` package archive creation implemented.
- [x] Optional self-extracting `.sfx.exe` package creation implemented.
- [x] Restore dry-run implemented and defaulted.
- [x] Restore apply implemented with explicit `--apply`.
- [x] Package and restore verification command implemented.
- [x] Post-apply restore verification implemented for file hashes and restored registry values.
- [x] Restore logs written as JSON and Markdown.
- [x] Restore path rewriting implemented for archived app paths.
- [x] Admin checks implemented for protected targets and machine-wide integrations.
- [x] GitHub Actions CI configured.
- [x] Project-local elevated machine-wide E2E script and operator checklist added.
- [x] Project-local disposable machine-scoped E2E harness added for HKLM, service, scheduled task, common shortcut, and machine environment restore apply certification from an admin shell.
- [x] Project-local disposable user-scoped E2E harness added and passed.
- [x] Non-admin admin-only restore guard smoke added.
- [x] Driver and kernel component discovery added with explicit metadata-only no-copy warnings.
- [x] First-class COM registration and file association report models added.
- [x] Deeper scheduled task and service validation added through restore verification.
- [x] Native GUI click-through smoke automation script added.
- [x] GUI dependency graph visualization added.
- [x] GUI advanced scan settings added.
- [x] GUI UI/UX polish added with status chips, grouped workflow controls, safer restore mode controls, result metrics, and artifact folder actions.
- [x] Project-local package/verify/restore dry-run performance regression script added.
- [x] Inventory snapshot workflow and GUI launch point added.
- [x] Future installer technology decision recorded: portable `.zip` remains selected for this release line.
- [x] Authenticode signing policy and conditional CI signing hook added.
- [x] SFX UX decision recorded: minimal standalone restore wrapper.
- [x] Optional TUI decision recorded: no TUI for this release line.
- [x] Portable Windows `.zip` release artifact configured.
- [x] Future Authenticode signing documented and disabled.
- [x] Local quality gates passed on 2026-06-19.
- [x] Isolated executable package/restore E2E passed.
- [x] Real user-scoped app E2E passed with SumatraPDF 3.6.1.
- [x] Real machine-scoped app package and dry-run E2E passed with existing 7-Zip 25.01.
- [x] GUI release smoke launch passed.

### Remaining

- [x] Run elevated machine-wide restore apply E2E from an admin shell.
- [x] Add trace/snapshot workflows if selected for a later milestone.
- [x] Select future installer technology if portable `.zip` is not enough.
- [x] Activate Authenticode signing after certificate, timestamp server, and CI secret policy are selected.
- [x] Decide whether standalone SFX should evolve into a richer installer-style experience.
- [x] Re-run full gates after every remaining production-hardening change.

## Decisions And Assumptions

### Resolved

- [x] Product name: `WinAppMigrator`.
- [x] Target OS: modern Windows 10/11.
- [x] Language: Rust.
- [x] Rust toolchain/MSRV: current stable Rust in this environment, `1.96.0`, with `x86_64-pc-windows-msvc`.
- [x] Interfaces: CLI and GUI are both required.
- [x] Architecture: shared core crate plus separate CLI and GUI crates.
- [x] GUI framework: `eframe`/`egui`, selected as a maintained native Rust GUI stack after package metadata verification.
- [x] CI provider: GitHub Actions.
- [x] Release format: portable `.zip`.
- [x] Installer strategy: MSI/MSIX/setup installers are not selected for this release line; portable `.zip` remains sufficient until fleet deployment or installed update/uninstall lifecycle requirements appear.
- [x] Signing: conditional Authenticode signing is wired for manual CI runs when selected secrets are present.
- [x] Package archive format: directory package plus optional `.zip`; optional self-extracting `.sfx.exe` wrapper is implemented.
- [x] Restore posture: dry-run by default, explicit apply required.
- [x] Security posture: conservative; no secret export; opt-in license capture only (detect + report transferability, copy license files on `--include-license-data`, never restore them automatically), and never decrypt, bypass/emulate activation, or emulate a dongle.
- [x] Shared-file policy: never copy non-app-owned shared files outside the app directory. Report shared runtimes as prerequisites.
- [x] Per-user and per-machine installations must be backed up equivalently.
- [x] Do not use VS Code as the real E2E test app.

### Open

- [x] Future installer technology: MSI, MSIX, setup executable, or multiple formats.
- [x] Future Authenticode certificate source, timestamp server, and CI secret policy.
- [x] Final SFX UX: minimal standalone restore wrapper or fuller installer-like restore wizard.

## Architecture Checklist

### Core

- [x] `winappmigrator-core` owns domain models, inventory, PE parsing, DLL resolution, scan orchestration, package creation, restore planning/apply, reports, and errors.
- [x] Core APIs are reused by CLI and GUI.
- [x] Core avoids hidden system modification during scan/list/package.
- [x] Restore applies changes only through explicit restore APIs.
- [x] Add deeper driver discovery.
- [x] Add first-class COM/file association models.
- [x] Add package/restore verification APIs.
- [x] Add layered install-source reverse-engineering: install-method detection (MSI/MSIX/Squirrel/Chocolatey/Inno/NSIS/Generic), authoritative MSI component-database enumeration (SQUID + `UserData\Components`), Inno Setup `unins000.dat` parsing, and App Paths.
- [x] Add footprint-aware discovery across Program Files/(x86)/ProgramData/AppData with nested `Vendor\App` (depth-2) matching, cache/temp pruning, `UserData` classification, and cross-app false-positive guards, surfaced as `FootprintLocation`s.
- [x] Narrow the MSI component sweep to product-registering SIDs (`component_sweep_sids`, with full-SID fallback) — safe, identical results, faster on multi-user machines.
- [~] Fully scope MSI component enumeration to one product (`Features`/Base85 or MSI API): **deferred by decision**. A mis-decoded feature descriptor would silently under-report the migration footprint, and the exhaustive sweep is the COM-validated source of truth. Revisit only via `MsiGetComponentPathW` cross-checked against the exhaustive sweep, which adds a Windows FFI dependency the project avoids. See `docs/discovery.md`.
- [x] Parse NSIS `AdvUninstLog` `uninstall.log` when present (`find_nsis_log`/`nsis_manifest_files`).
- [x] Deepen MSIX (`AppxManifest` identity → `%LOCALAPPDATA%\Packages\<id>_*`) and Squirrel (authoritative root + latest `app-*`) enumeration.
- [x] Restore user data back to the real `%AppData%`/`%ProgramData%` on the target machine (opt-in `--restore-user-data`; routing centralized in `file_discovery::restore_destination_for_entry`; ProgramData apply requires elevation).
- [x] Harden EXE reverse-engineering: delay-load imports (manual data-dir[13] RVA walk, since `pelite` has no delay API), embedded `RT_MANIFEST` (`requestedExecutionLevel` + SxS `<dependentAssembly>`), managed .NET detection (COR20 data-dir[14]), and runtime-prerequisite breadth (.NET Framework/self-contained, Java, Python, Electron/Node). Validated against mmc.exe (323 delay imports), regedit.exe (SxS Common-Controls), InstallUtil.exe (managed .NET).
- [x] Optional user-data backup as a first-class capability: separate `MachineAppData` (ProgramData) from genuine per-user data via `include_program_data`/`--skip-program-data`; expand user-data discovery to Documents (incl. `My Games\<App>`) and Saved Games with `User Shell Folders` redirection resolution; skip online-only cloud placeholders (OneDrive Files On-Demand).
- [x] Optional license-data backup (`license_scan`): detect license files/framework stores/`*_LICENSE_FILE` env vars/vendor registry keys with transferability + machine-binding hints; opt-in `--include-license-data` copies license files into package `license/`; never decrypt/bypass/emulate; never auto-restored. Validated end-to-end with a `.lic` fixture. AGENTS.md policy updated.

### CLI


- [x] `winappmigrator-cli` owns argument parsing, logging setup, terminal output, and command orchestration.
- [x] `--verbose`, `--quiet`, and `--log-file` are supported globally.
- [x] `list` shows installed application metadata and install scope.
- [x] `snapshot` writes read-only installed-application inventory snapshots.
- [x] `scan` supports executable path and installed app name workflows.
- [x] `package` supports directory, `.zip`, and SFX output.
- [x] `restore` supports directory, `.zip`, and SFX workflows.
- [x] Restore defaults to dry-run and requires explicit `--apply`.
- [x] Add a `verify` command or equivalent package/restore verification workflow.

### GUI

- [x] `winappmigrator-gui` owns UI state and background worker orchestration.
- [x] GUI lists installed apps or surfaces inventory errors.
- [x] GUI supports manual executable and output directory selection.
- [x] GUI runs scan through shared core logic.
- [x] GUI supports inventory snapshot workflows.
- [x] GUI supports package creation and restore workflows.
- [x] GUI distinguishes idle, running, completed, warning, and failed states.
- [x] Long-running work stays off the UI thread.
- [x] Add native GUI interaction automation.
- [x] Add dependency graph visualization.
- [x] Add advanced scan settings.
- [x] Redesign the GUI as an `eframe`/`egui` console: navigation rail, content header, routed pages, and a token-based light/dark design system split across `theme.rs`, `widgets.rs`, and `views.rs`.
- [x] Surface the detected install method, reverse-engineering evidence, and the Data locations footprint summary in Analyze results.
- [x] Add a searchable, virtualized applications table and a middle-ground guided flow ("Back up this app", default output directory) alongside advanced controls.

## Dependency Verification Notes

Verified on 2026-06-12 using `cargo search`, `cargo info`, local Rust version output, and workspace build gates.

| Dependency | Version | Purpose | Metadata compatibility |
| --- | ---: | --- | --- |
| `clap` | 4.6.1 | CLI parsing | Rust version 1.85, compatible with Rust 1.96.0 |
| `serde` | 1.0.228 | Data model serialization | Rust version 1.56, compatible |
| `serde_json` | 1.0.150 | JSON report output | Rust version 1.71, compatible |
| `thiserror` | 2.0.18 | Structured errors | Rust version 1.68, compatible |
| `tracing` | 0.1.44 | Structured logging API | Rust version 1.65, compatible |
| `tracing-subscriber` | 0.3.23 | CLI logging subscriber | Rust version 1.65, compatible |
| `tracing-appender` | 0.2.5 | Optional CLI log file writer | Rust version 1.63, compatible |
| `winreg` | 0.56.0 | Windows registry inventory and restore | Rust version 1.60, Windows-focused |
| `pelite` | 0.10.0 | PE metadata and import parsing | Rust version not declared; verified by local build gate |
| `time` | 0.3.49 | RFC3339 UTC timestamps | Rust version 1.88, compatible |
| `eframe`/`egui` | 0.34.3 | Native GUI | Rust version 1.92, compatible |
| `rfd` | 0.17.2 | Native file/folder dialogs | Rust version not declared; verified by local build gate |
| `walkdir` | 2.5.0 | Bounded recursive file discovery | Rust version not declared; verified by local build gate |
| `sha2` | 0.11.0 | Streaming SHA-256 hashing | Rust version 1.85, compatible |
| `filetime` | 0.2.29 | Preserve basic timestamps when packaging files | Rust version 1.75, compatible |
| `tempfile` | 3.27.0 | Temporary extraction for zip/SFX restore and tests | Rust version 1.63, compatible |
| `zip` | 6.0.0 | Stable non-prerelease zip writer/reader for package archives | Rust version 1.83, compatible |

## Security Checklist

- [x] `cargo audit` passed on 2026-06-12.
- [x] `cargo deny check` passed on 2026-06-12.
- [x] Required OSI/free GUI transitive licenses are allowed in `deny.toml`.
- [x] Unavoidable transitive duplicate versions from the native GUI stack are allowed in `deny.toml`.
- [x] No secrets are committed to project files.
- [x] List and scan workflows are read/report-only.
- [x] Package creation does not modify the source system.
- [x] Restore defaults to dry-run.
- [x] Restore apply requires explicit `--apply`.
- [x] Restore refuses conflicting `--dry-run` plus `--apply`.
- [x] Existing destination files require `--force` before overwrite.
- [x] Machine-wide registry, services, scheduled tasks, public desktop, common Start Menu, machine environment variables, and protected targets require elevation.
- [x] User-scoped scheduled task restore does not require elevation when the archived task does not require highest run level or SYSTEM account.
- [x] Non-elevated admin-only apply refusal is smoke-tested locally.
- [x] Shared non-app-owned files outside the app directory are not copied.
- [x] Elevated machine-wide restore apply must be E2E verified.
- [x] Post-restore verification must be added for enterprise certification.
- [x] Driver/kernel component discovery must be expanded before production-ready claims.

## Performance Checklist

- [x] Scan starts from one selected executable instead of broad machine scanning.
- [x] Recursive DLL scanning avoids duplicate loops.
- [x] File discovery is bounded and avoids broad system tree copying.
- [x] Hashing streams through a fixed-size buffer.
- [x] Large files are copied through filesystem streaming APIs.
- [x] GUI scan/package/restore work runs on background threads.
- [x] Add measurable timing targets for large app packages.
- [x] Add performance regression checks for package creation and restore verification.

## Milestone Checklists

### Milestone 1: Workspace, Shared Core, CLI MVP, GUI Skeleton

Goal: build the smallest safe vertical slice for list and scan workflows.

Status:

- [x] Implemented.
- [x] Certified locally on 2026-06-12.

Scope:

- [x] Rust workspace.
- [x] Shared serializable models and structured errors.
- [x] Installed application inventory from uninstall registry locations.
- [x] Scan by executable path.
- [x] Basic direct PE import parsing.
- [x] Basic DLL resolution across app directory, Windows system directories, current directory, and `PATH`.
- [x] JSON, Markdown, dependency graph, and warnings reports.
- [x] CLI `list` and `scan`.
- [x] GUI shell with inventory loading, manual executable/out directory selection, background scan, summary, warnings, and report location.

Files/areas affected:

- [x] `Cargo.toml`
- [x] `crates/winappmigrator-core`
- [x] `crates/winappmigrator-cli`
- [x] `crates/winappmigrator-gui`
- [x] `README.md`
- [x] `CHANGELOG.md`
- [x] `PLAN.md`
- [x] `AGENTS.md`

Acceptance criteria:

- [x] `cargo run -p winappmigrator-cli -- list` builds and attempts inventory.
- [x] `cargo run -p winappmigrator-cli -- scan --exe "C:\SomeApp\App.exe" --out ".\scan-output"` writes scan report files for a readable PE executable.
- [x] `cargo run -p winappmigrator-gui` launches the native GUI.
- [x] GUI scan remains off the UI thread.
- [x] Scan reports identify executable architecture, direct imports, resolved/missing DLLs, app-local DLLs, system DLLs, warnings, and confidence score.

Tests and gates:

- [x] CLI command parsing.
- [x] Registry uninstall entry parsing.
- [x] PE import parser behavior against a local PE test binary.
- [x] DLL resolution classification.
- [x] JSON report serialization.
- [x] Markdown report generation.
- [x] `cargo fmt --check`
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] `cargo check --workspace --all-targets --all-features`
- [x] `cargo test --workspace --all-targets --all-features`
- [x] `cargo build --workspace --release`
- [x] `cargo audit`
- [x] `cargo deny check`

Documentation and verification:

- [x] README usage and gate commands updated.
- [x] CHANGELOG updated with real changes.
- [x] PLAN dependency verification and certification status updated.
- [x] List/scan confirmed read/report-only.
- [x] GUI scan confirmed on background thread.
- [x] Scans confirmed not to recursively scan broad system filesystem trees.
- [x] Native GUI release executable smoke-launched.
- [x] Lighthouse not applicable because there is no web UI.

### Milestone 2: Recursive Dependencies, File Discovery, Hashing, Reports

Goal: broaden scan depth after Stage 1 is certified.

Status:

- [x] Implemented.
- [x] Gate-verified locally on 2026-06-12.

Scope and acceptance:

- [x] Recursive app-local DLL scanning follows copyable dependencies.
- [x] Dependency graph avoids loops and duplicate entries.
- [x] Install-directory file discovery is bounded.
- [x] App-owned, shared runtime, system, missing, and no-copy classifications are explicit.
- [x] SHA-256 hashing streams through a fixed-size buffer.
- [x] Markdown reports include file inventory and hashes.
- [x] Warning classification exists.
- [x] GUI report summary improved enough for MVP workflows.

Remaining:

- [x] Add timing targets and regression checks for large app scans.
- [x] Add richer GUI dependency graph visualization in Milestone 6.

### Milestone 3: Windows Integration Discovery

Goal: discover registry, shortcuts, services, scheduled tasks, and environment variables conservatively.

Status:

- [x] Implemented as conservative discovery.
- [x] Gate-verified locally on 2026-06-12.

Scope and acceptance:

- [x] Registry scan reports App Paths.
- [x] Registry scan reports uninstall keys.
- [x] Registry scan reports Run and RunOnce references.
- [x] Registry scan reports first-level related software keys.
- [x] Registry scan reports targeted `Classes\Applications` entries.
- [x] Registry scan reports targeted CLSID registrations.
- [x] Services scan matches service `ImagePath`.
- [x] Scheduled task scan uses read-only PowerShell queries and parses JSON output.
- [x] Shortcut scan uses read-only PowerShell queries and parses JSON output.
- [x] Persistent user and machine environment variables referencing the app are detected.
- [x] Registry matching avoids broad unrelated exports.
- [x] Low-confidence registry matches are warnings.

Remaining:

- [x] Add first-class COM registration report models beyond raw registry capture.
- [x] Add first-class file association report models beyond raw registry capture.
- [x] Add driver/kernel component discovery and warnings.

### Milestone 4: Package Creation And Restore Dry-Run

Goal: create migration package directories and dry-run restore plans.

Status:

- [x] Implemented.
- [x] Gate-verified locally on 2026-06-12.

Package output:

- [x] `files\`
- [x] `registry\`
- [x] `shortcuts\`
- [x] `services\`
- [x] `drivers\`
- [x] `scheduled_tasks\`
- [x] `environment\`
- [x] `manifests\`
- [x] `reports\`
- [x] `restore_plan.md`
- [x] `manifest.json`
- [x] Structured JSON artifacts.
- [x] `.reg`-compatible registry exports.
- [x] Optional `.zip` archive.
- [x] Optional `.sfx.exe` self-extracting archive.

Acceptance:

- [x] Package creation copies app-owned files only.
- [x] Package manifest records source scope and source Windows version.
- [x] Dry-run restore can be generated without system changes.
- [x] Restore dry-run produces JSON and Markdown logs.

Remaining:

- [x] Add package verification command/checks.
- [x] Add richer SFX installer-style UX decision.

### Milestone 5: Restore Apply Mode

Goal: implement explicit, logged restore changes with admin checks.

Status:

- [x] Code implemented.
- [x] User-scoped real-app apply E2E passed.
- [x] Machine-scoped package and dry-run E2E passed.
- [x] Elevated machine-wide apply E2E passed from an admin shell.

Scope and acceptance:

- [x] Default restore remains dry-run.
- [x] `--apply` is required for system changes.
- [x] Every planned/applied change is logged.
- [x] File restore preserves timestamps where practical.
- [x] File restore refuses overwrite without `--force`.
- [x] Registry restore rewrites archived app paths to target root.
- [x] Registry restore preserves raw value kinds where practical.
- [x] Environment variable restore rewrites archived app paths to target root.
- [x] Service restore uses Windows service tooling.
- [x] Scheduled task restore uses archived XML.
- [x] Shortcut restore uses Windows shell automation.
- [x] Admin-only operations check elevation and fail clearly when not elevated.
- [x] Elevated machine-wide registry apply verified.
- [x] Elevated service apply verified.
- [x] Elevated scheduled task apply verified.
- [x] Elevated public/common shortcut apply verified.
- [x] Post-apply verification added.

### Milestone 6: Advanced GUI/TUI Polish

Goal: improve operator experience after MVP workflows work.

Status:

- [x] Implemented for selected release-line scope.
- [x] Optional TUI is intentionally not selected for this release line.

Completed:

- [x] GUI supports scan workflows.
- [x] GUI supports package workflows.
- [x] GUI supports restore dry-run controls.
- [x] GUI supports restore apply controls.
- [x] GUI runs work on background workers.
- [x] GUI supports inventory snapshot launch point.
- [x] GUI surfaces progress detail text for scan, package, restore, and snapshot workers.
- [x] GUI polished for operator workflow hierarchy, status visibility, and restore safety.

Remaining:

- [x] Dependency graph visualization.
- [x] Improved progress details.
- [x] Advanced scan settings.
- [x] Trace/snapshot launch points.
- [x] Native GUI interaction automation.
- [x] Optional TUI decision.

## Latest Gate Results

Recorded on 2026-06-20 after adding opt-in real-location user-data restore (`--restore-user-data`), centralizing restore destination routing, NSIS AdvUninstLog `uninstall.log` parsing, and MSIX/Squirrel footprint deepening, on a freshly provisioned workstation (rustc 1.96.0 via rustup):

- [x] `cargo fmt --check`: passed.
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- [x] `cargo check --workspace --all-targets --all-features`: passed.
- [x] `cargo test --workspace --all-targets --all-features`: passed, 46 core + 8 CLI tests (added user-data anchor split, entry-aware restore destination, sandbox-fallback, app-owned routing, `--restore-user-data` CLI parse, NSIS log name/path extraction, AppxManifest identity parse, and Squirrel version-selection coverage).
- [x] `cargo build --workspace --release`: passed.
- [x] `cargo audit`: passed.
- [x] `cargo deny check`: passed.
- [x] Disposable user-scoped package/zip restore dry-run/apply/verify E2E (`scripts\e2e-disposable-user-scope.ps1`): passed after the restore-routing centralization and user-data classification rework — `package_valid=true`, `restore_valid=true`, 488 items checked, 0 errors. Certifies the refactored restore/verify path did not regress.
- [x] License-data E2E (`scripts\e2e-license-data.ps1`): passed — `.lic` and `.v2c` detected (`.v2c` reported machine-bound), a non-license `notes.txt` correctly ignored, and both license files copied into the package `license/` directory with `--include-license-data`.
- [ ] Elevated machine-scoped E2E not re-run this session (requires an admin shell); rerun `scripts\e2e-disposable-machine-scope.ps1` from an elevated shell to recertify HKLM/service restore after the refactors.

Recorded on 2026-06-19 after verification, driver, GUI, UI/UX polish, documentation, dependency-policy, scheduled-task scanner hardening, and elevated E2E work:

- [x] `cargo fmt --check`: passed.
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- [x] `cargo check --workspace --all-targets --all-features`: passed.
- [x] `cargo test --workspace --all-targets --all-features`: passed, 31 tests.
- [x] `cargo build --workspace --release`: passed.
- [x] `cargo audit`: passed.
- [x] `cargo deny check`: passed after updating yanked `time` 0.3.48 to 0.3.49.
- [x] PowerShell script syntax check: passed for all project-local scripts.
- [x] Release CLI package/verify/zip restore dry-run/apply/verify E2E: passed against an isolated executable copy in `target\wam-final-e2e-release`; package verification reported valid, post-apply restore verification reported valid, and restore logs were written outside temporary zip extraction.
- [x] GUI click-through smoke script: passed after the UI/UX polish pass.
- [x] Release CLI inventory snapshot smoke: passed; wrote JSON and Markdown snapshots for 379 installed application entries.
- [x] Disposable user-scoped package/zip restore dry-run/apply/verify E2E: passed after scheduled-task scanner hardening; verified restored files, HKCU registry, user scheduled task, user Start Menu shortcut, and HKCU environment variable.
- [x] Non-admin admin-only restore guard smoke: passed; apply mode was blocked before changes.
- [x] Disposable machine-scope elevated E2E script parse/admin-precondition smoke: passed from non-admin shell with the expected elevation requirement.
- [x] Disposable machine-scoped elevated package/zip restore dry-run/apply/verify E2E: passed from an admin shell; verification reported `package_valid=true`, `restore_valid=true`, 13 checked items, and 0 errors for restored files, HKLM registry, service, scheduled task, common Start Menu shortcut, and machine environment variable. Evidence: `target\wam-elevated-machine-e2e\run`.
- [x] Package/verify/restore dry-run performance smoke against release CLI executable: passed; package 58.59s <= 120s, verify 7.59s <= 30s, restore dry-run 12.37s <= 60s.

Previous record from 2026-06-12 after Stage 2-5 work:

- [x] `cargo fmt --check`: passed.
- [x] `cargo check --workspace --all-targets --all-features`: passed.
- [x] `cargo test --workspace --all-targets --all-features`: passed, 24 tests.
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- [x] `cargo build --workspace --release`: passed.
- [x] `cargo audit`: passed.
- [x] `cargo deny check`: passed.
- [x] CLI package smoke after environment/manifest updates: passed; package manifest included source Windows version and environment-variable artifact support.
- [x] Local portable zip command: passed, produced `dist\WinAppMigrator-portable-windows-x64.zip`.
- [x] CLI package/restore E2E: passed against an isolated executable copy; package copied 1 file, dry-run logged 1 action, apply copied 1 file, restored executable `--help` launched successfully.
- [x] User-scope real app E2E: passed with SumatraPDF 3.6.1 from the official installer. Installed under HKCU/current user, packaged by installed-app name, uninstalled, restored from generated SFX, validated restored executable version and rewritten HKCU uninstall entry, then cleaned test registry/shortcut state.
- [x] Machine-scope real app E2E: passed with existing 7-Zip 25.01 machine install. Packaged by installed-app name, generated zip, recorded `per_machine` scope and HKLM registry entries requiring admin, and dry-run restored from zip without system changes.
- [x] `target\release\winappmigrator-gui.exe` smoke launch: passed.

Not yet run:

- [x] None for the documented production gate set as of 2026-06-19.

## Documentation Checklist

- [x] `README.md` includes project overview, stack, requirements, development commands, CLI usage, GUI usage, build commands, test commands, quality gates, environment variables, project structure, release notes, security notes, and troubleshooting.
- [x] `CHANGELOG.md` tracks actual implementation history.
- [x] `PLAN.md` tracks decisions, architecture, dependencies, milestones, gates, and remaining work.
- [x] `AGENTS.md` reinforces local coding, testing, security, changelog, planning, and shared-core rules.
- [x] `docs/signing.md` documents conditional Authenticode signing policy and CI secrets.
- [x] Add elevated E2E operator guide.
- [x] Add package/restore verification guide after verification command exists.
- [x] `docs/snapshots.md` documents read-only inventory snapshots.
- [x] `docs/discovery.md` documents footprint discovery and install-source reverse-engineering.
- [x] `docs/gui.md` documents the GUI console architecture (theme/widgets/views).
- [x] `docs/user-e2e.md` documents disposable user-scoped E2E.
- [x] `docs/installer-strategy.md` documents release packaging decision.
- [x] `docs/sfx.md` documents SFX UX decision.
- [x] `docs/tui-decision.md` documents TUI scope decision.

## Definition Of Done

- [x] All remaining enterprise blockers are resolved.
- [x] Relevant milestone checklists are fully satisfied.
- [x] Quality gates pass after the final production-hardening changes.
- [x] Elevated machine-wide apply E2E passes.
- [x] User-scoped and machine-scoped app backup/restore remain verified.
- [x] README, CHANGELOG, and PLAN match actual behavior.
- [x] No production-readiness claim is made before all documented production-readiness gates pass.
