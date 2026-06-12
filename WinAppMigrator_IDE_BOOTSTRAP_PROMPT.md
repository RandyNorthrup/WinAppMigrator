# WinAppMigrator IDE Bootstrap Prompt

You are starting a new production-grade Windows-only Rust project named **WinAppMigrator**.

Use `/caveman` communication style for chat/status updates: short, direct, no filler. Keep full technical accuracy, documentation quality, code comments, plans, tests, and architectural rationale. Compress status updates only; do not compress engineering rigor.

Do not create or modify global user memory, global IDE settings, machine-wide agent instructions, or globally installed tools unless explicitly approved. Only create project-local files and project-local configuration.

---

## 1. Primary Objective

Create a clean, secure, maintainable, production-grade project foundation from the first commit.

Build **WinAppMigrator**, a Windows 10/11 Rust desktop application migration analyzer and package builder.

WinAppMigrator analyzes an already-installed Windows desktop application and builds a migration package containing the files, registry entries, shortcuts, service definitions, scheduled tasks, environment settings, and dependency reports needed to recreate or move the application to another Windows PC when the original installer is unavailable.

The product must include **both CLI and GUI** interfaces.

Use the most current stable stack versions that are verified compatible with each other.

Do not guess compatibility. Before adding dependencies or choosing tooling, verify compatibility using official documentation, release notes, package metadata, dependency requirements, or compatibility matrices. Document verification notes in `PLAN.md`.

---

## 2. Resolved Decisions

Use these decisions without asking again unless project files conflict.

- Product name: `WinAppMigrator`
- Application type: Windows desktop application migration analyzer and package builder
- Interfaces: **CLI and GUI are both required**
- Language: Rust
- Target OS: Modern Windows 10/11
- Architecture: shared core library plus separate CLI and GUI frontends
- Initial implementation scope: Rust workspace, shared data models, CLI skeleton, GUI skeleton, Stage 1 scan/list implementation
- Restore behavior: dry-run by default, explicit `--apply` required for changes
- Security posture: conservative, no secret export, no licensing bypass

Ask clarification questions only for unresolved stack-defining or product-risk decisions, including:

- Preferred Rust toolchain/MSRV if not obvious from project files.
- GUI framework choice if not already selected.
- CI provider.
- Release/distribution format.
- Installer/signing requirements.
- Package archive format if archive output is implemented.
- Whether advanced flags should ever copy non-app-owned shared files.

If the user wants no clarification and asks to proceed, use reasonable defaults only when they do not create product risk. Document all assumptions in `PLAN.md` before implementation.

Recommended non-binding default for GUI if the user does not choose: a native Rust GUI framework such as `eframe`/`egui`, after current compatibility and maintenance status are verified.

---

## 3. Required Architecture

Create a Rust workspace with clear separation between core logic and user interfaces.

Preferred workspace shape:

```text
winappmigrator/
  Cargo.toml
  crates/
    winappmigrator-core/
    winappmigrator-cli/
    winappmigrator-gui/
  tests/
  docs/
  samples/
```

The shared core must contain reusable business logic. CLI and GUI must call the same core APIs rather than duplicating scanning, packaging, restore, or reporting logic.

Core areas/modules:

- `app_inventory`
- `pe_scan`
- `dll_resolver`
- `registry_scan`
- `file_discovery`
- `shortcut_scan`
- `service_scan`
- `task_scan`
- `package`
- `restore`
- `reporting`
- `hashing`
- `privilege`
- `path_utils`
- `models`
- `errors`

Frontend crates:

- `winappmigrator-cli`: command-line parser, terminal output, command orchestration
- `winappmigrator-gui`: Windows desktop GUI, app selection, scan/package/restore workflows, progress display, report viewer/summary
- `winappmigrator-core`: all shared domain logic, models, scanners, reports, validation, safety rules

Document any deviation from this layout in `PLAN.md`.

---

## 4. CLI Requirements

Every CLI command must support:

```powershell
--verbose
--quiet
--log-file "path"
```

Use structured logging.

Use strong error handling. Do not panic for normal scan failures. Return clear errors and warnings.

### 4.1 List Installed Applications

Command:

```powershell
winappmigrator list
```

Behavior:
Enumerate installed applications from common Windows uninstall registry locations:

- `HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall`
- `HKLM\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall`
- `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall`

For each app, display:

- Display name
- Publisher
- Version
- Install location
- Uninstall string, if present
- Main executable guess, if possible
- Registry source path
- Whether app appears to be 32-bit or 64-bit

### 4.2 Scan by Executable Path

Command:

```powershell
winappmigrator scan --exe "C:\Path\To\App.exe" --out "C:\Output\Scan"
```

Behavior:
Analyze the selected executable and produce dependency reports.

Output files:

- `scan_report.json`
- `scan_report.md`
- `dependency_graph.json`
- `warnings.md`

The scan must include:

- Main executable metadata
- File version info
- Digital signature status, if practical
- PE architecture: x86, x64, ARM64
- Imported DLLs
- Delay-loaded DLLs
- Recursively resolved DLL dependencies
- DLLs found beside the executable
- DLLs resolved from System32, SysWOW64, Windows, PATH, and app-local folders
- Missing DLLs
- Suspicious or unresolved dependencies
- Side-by-side manifest dependencies, if present
- .NET application indicators
- Java application indicators
- Electron/Node application indicators
- Python application indicators
- Runtime dependencies such as VC++ Redistributable or .NET Desktop Runtime, if detected

### 4.3 Scan by Installed App Name

Command:

```powershell
winappmigrator scan --name "Application Name" --out "C:\Output\Scan"
```

Behavior:
Search installed app registry entries for a matching display name. Resolve likely install location and main executable. If multiple matches are found, ask the user to choose unless `--yes` or `--non-interactive` is supplied.

### 4.4 Build Migration Package

Command:

```powershell
winappmigrator package --exe "C:\Path\To\App.exe" --out "C:\Output\AppMigrationPackage"
```

Behavior:
Perform a full scan and create a migration package directory.

Package contents:

```text
files\
registry\
shortcuts\
services\
scheduled_tasks\
manifests\
reports\
restore_plan.md
manifest.json
```

`manifest.json` must include:

- Package schema version
- Source computer name
- Source Windows version
- Scan timestamp
- Main executable path
- Application metadata
- File inventory
- File hashes
- File sizes
- Original paths
- Suggested destination paths
- Registry keys exported
- Services detected
- Scheduled tasks detected
- Shortcuts detected
- Environment variables detected
- Runtime prerequisites
- Warnings
- Items intentionally excluded

### 4.5 Restore Package

Dry-run command:

```powershell
winappmigrator restore --package "C:\Package" --target-root "C:\RestoredApps\AppName" --dry-run
```

Apply command:

```powershell
winappmigrator restore --package "C:\Package" --target-root "C:\RestoredApps\AppName" --apply
```

Restore mode must:

- Default to dry-run
- Require explicit `--apply` for changes
- Require admin privileges for machine-wide registry, services, Program Files, and scheduled tasks
- Never overwrite existing files without confirmation unless `--force` is supplied
- Log every change
- Produce `restore_log.json` and `restore_log.md`
- Never silently modify the system

---

## 5. GUI Requirements

The GUI is required, not optional.

The GUI must use the same shared core APIs as the CLI.

Minimum GUI MVP:

- Launches as a Windows desktop app.
- Shows installed applications discovered by `app_inventory`.
- Allows selecting an installed app.
- Allows selecting an executable path manually.
- Allows selecting an output directory.
- Runs scan using shared core logic.
- Shows scan progress and non-fatal warnings.
- Shows summary, architecture, missing dependencies, runtime prerequisites, and confidence score.
- Provides links/buttons to open generated report files.
- Does not require admin rights for basic list/scan.
- Clearly labels operations that require admin rights.
- Defaults restore workflows to dry-run.
- Requires explicit confirmation before any system modification.

GUI future milestones:

- Package creation workflow.
- Restore dry-run viewer.
- Restore apply workflow with explicit confirmations.
- Dependency graph visualization.
- Report viewer.
- Advanced settings for scan scope.
- Optional trace/snapshot workflows after MVP.

GUI quality requirements:

- Shared logic with CLI; no duplicated scanner implementations.
- UI state must distinguish idle, running, completed, warning, and failed states.
- Long-running work must not freeze the UI thread.
- Errors must be surfaced in user-readable form without losing structured diagnostic details.
- Dangerous operations must require explicit user action.
- No hidden background system modifications.

---

## 6. File Discovery Requirements

Collect candidate files from:

- Main executable directory
- Subdirectories under the app install directory
- App-local DLLs
- App-local config files
- App-local data files
- App-local manifests
- App-local plugins
- Start Menu shortcuts pointing to the app
- Desktop shortcuts pointing to the app
- ProgramData folders related to app name, publisher, executable name, or product name
- AppData Roaming folders related to the app
- AppData Local folders related to the app
- AppData LocalLow folders related to the app
- Common Files folders related to publisher or product
- Registry App Paths entries
- Service executable paths matching the app directory
- Scheduled task executable paths matching the app directory
- File associations referencing the app executable
- COM registrations referencing DLLs or EXEs inside the app directory

Do not blindly copy the Windows folder, System32, SysWOW64, or unrelated large branches.

Reports must distinguish:

- App-owned files
- Shared runtime files
- Windows system files
- Missing files
- Files that should not be copied
- Files requiring manual runtime installation

---

## 7. Dependency Resolution Requirements

Implement PE dependency scanning.

Parse:

- PE import table
- PE delay import table
- Embedded application manifest, if present
- External `.manifest` file beside the executable, if present

DLL resolution should approximate Windows DLL search behavior:

- Known DLLs should be marked Windows-provided
- App directory should be searched
- System32 should be searched
- SysWOW64 should be considered for 32-bit apps
- Windows directory should be searched
- Current working directory should be considered
- PATH directories should be searched
- App Paths registry entries should be considered
- Side-by-side assemblies should be reported as complex/manual when not safely resolvable

Recursive scanning must avoid infinite loops and duplicate entries.

Each dependency entry must include:

- Name
- Resolved path, if found
- Architecture
- Hash
- Version info
- Publisher/company metadata
- Source category
- Whether it should be copied
- Whether it is system-provided
- Whether it is a runtime prerequisite
- Whether it is missing

---

## 8. Registry Scan Requirements

Registry discovery must be conservative.

Search:

- Uninstall entries
- App Paths
- File associations
- Classes/COM registrations
- Services
- Application-specific keys under `HKCU\Software`
- Application-specific keys under `HKLM\Software`
- `WOW6432Node` equivalents
- `Run` and `RunOnce` entries referencing the app path
- Environment variables referencing the app path

Registry matching should use:

- Executable path
- Install directory
- Display name
- Product name
- Publisher name
- Executable filename
- Company name from version metadata

Export discovered registry data into `.reg`-compatible files where practical and into structured JSON.

Do not export unrelated large registry branches.

Include warnings when a registry key appears related but confidence is low.

---

## 9. Services Requirements

Detect Windows services whose `ImagePath` points to:

- Selected executable
- Install directory
- Any collected app-owned executable

For each service, record:

- Service name
- Display name
- Binary path
- Start type
- Account name, without passwords or secrets
- Dependencies
- Description
- Whether admin privileges are required
- Restore command suggestion using `sc.exe` or PowerShell

---

## 10. Scheduled Tasks Requirements

Detect scheduled tasks whose actions reference:

- Selected executable
- Install directory
- Any collected app-owned executable

For each task, record:

- Task name
- Task path
- Action command
- Arguments
- Working directory
- Trigger summary
- Run level
- Account, without secrets
- Restore instructions

---

## 11. Shortcut Requirements

Find shortcuts in:

- User Desktop
- Public Desktop
- User Start Menu
- Common Start Menu

For each shortcut, record:

- Original `.lnk` path
- Target path
- Arguments
- Working directory
- Icon path
- Whether target is part of the selected app

---

## 12. Package Requirements

The `package` command must copy app-owned files into the package while preserving, where practical:

- Relative directory structure
- Timestamps
- File attributes
- Hashes
- Basic metadata

Large files must be supported.

Locked files:

- Retry a small, named, documented number of times.
- Report failure.
- Suggest closing the app.
- Optionally support future Volume Shadow Copy mode.
- Do not require Volume Shadow Copy for MVP.

---

## 13. Report Requirements

Generate human-readable Markdown reports and structured JSON reports.

Markdown scan report sections:

- Summary
- Source app
- Main executable
- Architecture
- Install location
- Files collected
- DLL dependencies
- Missing dependencies
- Runtime prerequisites
- Registry entries
- Services
- Scheduled tasks
- Shortcuts
- Items excluded
- Warnings
- Manual steps likely required
- Restore instructions
- Confidence score

---

## 14. Confidence Score

Calculate an approximate migration confidence score from 0 to 100.

Reduce confidence for:

- Missing DLLs
- Services
- Drivers
- COM components
- Hardware-bound licensing indicators
- Microsoft Store app indicators
- .NET Framework GAC dependencies
- Installed VC++ runtime dependency
- System-wide shared components
- Database services
- Kernel drivers
- Protected directories
- Unreadable files
- Registry entries requiring admin
- Files outside install directory

Document scoring logic in code comments and `PLAN.md`.

---

## 15. Data Models

Define serializable structs for:

- `InstalledApp`
- `AppSelection`
- `ScanOptions`
- `ScanReport`
- `PackageManifest`
- `FileEntry`
- `DependencyEntry`
- `RegistryEntry`
- `ServiceEntry`
- `ScheduledTaskEntry`
- `ShortcutEntry`
- `RuntimePrerequisite`
- `Warning`
- `RestorePlan`
- `RestoreResult`

Each model should derive:

- `Debug`
- `Clone`
- `Serialize`
- `Deserialize`

Add other derives only when useful and justified.

---

## 16. Suggested Rust Crates

Use appropriate crates after verification.

Suggested options:

- `clap` for CLI parsing
- `serde` and `serde_json` for structured output
- `thiserror` or `anyhow` for error handling
- `tracing` and `tracing-subscriber` for logging
- `walkdir` for filesystem traversal
- `sha2` or `blake3` for hashing
- `windows` crate for Windows APIs
- `winreg` or Windows registry APIs for registry access
- `pelite` or `goblin` for PE parsing
- `chrono` or `time` for timestamps
- `fs_extra` or standard fs APIs for copying
- `rayon` for optional parallel hashing/scanning
- Native Rust GUI framework, selected after compatibility verification
- `zip`, `tar`, or `zstd`-related crates for optional archive creation later

Do not hard-code crate versions unless needed.

For each dependency added:

1. Identify purpose.
2. Verify current stable version.
3. Verify compatibility with Rust toolchain, Windows target, and selected GUI stack.
4. Prefer official, maintained, widely adopted packages.
5. Avoid abandoned, deprecated, or unnecessary packages.
6. Document major dependency decisions in `PLAN.md`.

After installing:

1. Lock dependencies.
2. Run install verification.
3. Run available lint/type/build/test gates.
4. Update documentation.

---

## 17. Code Standards

Follow these rules throughout the project:

- No unexplained magic numbers, strings, booleans, or timeout values.
- Use named constants, typed constants, enums, literal unions, config objects, or schema-validated configuration.
- Common idioms are allowed only when clarity is not reduced, such as `0`, `1`, `-1`, empty arrays, empty strings, and boolean literals in direct conditions.
- No dead code.
- No commented-out legacy code.
- No unused files.
- No unused exports.
- No unused dependencies.
- No stale configuration.
- No silent fallbacks.
- No fake implementations.
- No placeholder production code.
- No mock data outside test/dev/demo boundaries.
- No unchecked casts unless justified inline and tracked in documentation.
- No suppressed lint rules or ignored type errors unless justified inline and tracked in documentation.
- No global installs unless explicitly required and documented.
- No dependency added without purpose, compatibility check, and security consideration.
- Use Windows-safe path handling.
- Normalize paths carefully.
- Handle long paths where practical.
- Do not assume Program Files paths are English-language or located on `C:`.
- Support both 32-bit and 64-bit applications.

---

## 18. Security Requirements

Set up security from the beginning:

- Dependency vulnerability scanning.
- Secret scanning where practical.
- Environment variable validation where applicable.
- Safe default configuration.
- Secure output handling.
- Input validation.
- Path traversal prevention during package/restore operations.
- Least-privilege behavior.
- No secrets committed to the repository.
- `.env.example` must document required environment variables without real secrets, if environment variables are used.
- Restore mode must require explicit confirmation for risky operations.
- Admin-only operations must check privileges and report clear errors.

Document security assumptions and remaining risks in `PLAN.md`.

---

## 19. Performance Requirements

Set up performance controls where applicable:

- Production build validation.
- Avoid unnecessary dependencies.
- Avoid repeated filesystem scans when results can be cached safely during one command run.
- Support large files.
- Stream hashing and copying instead of loading large files entirely into memory.
- Keep GUI responsive during long scans.
- Use worker threads/tasks for long-running scan/package/restore operations.
- Use measurable performance targets where possible.
- Consider optional parallel hashing/scanning only after correctness and safety are established.

For future web-based GUI milestones, if applicable:

- Add visual verification.
- Run Lighthouse Performance, Accessibility, and Best Practices when there is a web UI that can be locally previewed.
- Exclude Lighthouse SEO unless explicitly requested.

---

## 20. Quality Gate Commands

Set up scripts or documented commands equivalent to:

- `format`
- `format:check`
- `lint`
- `typecheck`
- `test`
- `test:unit`
- `test:integration`
- `test:e2e`
- `build`
- `security:audit`
- `quality`
- `quality:ci`

For Rust, prefer project-local documented commands such as:

```powershell
cargo fmt
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo build --workspace --release
cargo audit
cargo deny check
```

Only include commands that are actually installed/configured or clearly documented as optional/deferred.

The aggregate quality command must fail on any blocking issue.

Do not bypass gates to claim completion.

---

## 21. Testing Requirements

Include unit tests for:

- Registry uninstall parsing
- PE import parsing
- DLL search order approximation
- File classification
- Manifest serialization
- Package creation
- Restore dry-run planning

Add integration or CLI tests where practical.

Add GUI tests where practical for non-visual state transitions and shared core integration.

For Stage 1, prioritize:

- CLI command parsing
- GUI app shell launch/build
- Shared core API boundaries
- Registry uninstall entry parsing with test fixtures
- PE import parser behavior with sample or fixture data
- DLL resolution classification
- JSON report serialization
- Markdown report generation

Provide sample output files for a small test executable when practical.

---

## 22. Required Project Documents

Create and maintain these files.

### `README.md`

Must include:

- Project overview
- Stack
- Requirements
- Installation
- Development commands
- CLI usage
- GUI usage
- Build commands
- Test commands
- Quality gate commands
- Environment variables
- Project structure
- Deployment/release notes, if known
- Security notes
- Troubleshooting

README must never contain false claims, unverified claims, stale commands, or features that do not exist.

### `CHANGELOG.md`

Use this as the actual implementation history.

Rules:

- Update after meaningful code/config/documentation changes.
- Track real changes only.
- Do not list planned work as completed.
- Keep historical entries even if later superseded.

### `PLAN.md`

Create a full project execution plan.

The plan must include:

- Project assumptions
- Resolved decisions
- Open questions
- Architecture notes
- CLI architecture notes
- GUI architecture notes
- Research notes
- Dependency/version verification notes
- Milestones
- Tasks per milestone
- Test requirements per milestone
- Certification gates per milestone
- Visual verification gates for GUI milestones
- Lighthouse gates only if a web UI exists and is locally previewable
- Security gates
- Performance gates
- Documentation update requirements
- Definition of done

Each milestone must include:

- Goal
- Scope
- Files/areas likely affected
- Implementation steps
- Acceptance criteria
- Required tests
- Required quality gates
- Required documentation updates
- Required security checks
- Required performance checks
- GUI verification where applicable
- Certification checklist

A milestone is not complete until its certification checklist is satisfied.

---

## 23. Agent / IDE Instruction Files

Create project-local instruction files needed by the active IDE or coding agent, such as:

- `AGENTS.md`
- `.cursor/rules/*`
- `CLAUDE.md`
- `.github/copilot-instructions.md`
- Other project-local instruction files supported by the selected tooling

Do not create all possible files blindly if the tooling is unknown. Create `AGENTS.md` by default. Create IDE-specific files only when the active IDE/tooling is known or requested.

Instruction files must reinforce:

- Code standards
- Quality gates
- Documentation rules
- Testing rules
- Security rules
- Changelog discipline
- Planning discipline
- CLI and GUI shared-core discipline
- `/caveman` communication style for status updates

---

## 24. Implementation Stages

Work milestone by milestone.

Do not skip certification gates.

Do not create fallback, legacy, temporary, fake, or dead code to force progress.

Ask clarification questions when blocked by product decisions.

Make documented assumptions only when they do not create product risk.

Report progress using short `/caveman` style updates. Include changed files, gates run, failures, fixes, and remaining blockers.

### Stage 1: Workspace, Shared Core, CLI MVP, GUI Skeleton

Implement:

- Rust workspace
- `winappmigrator-core` crate
- `winappmigrator-cli` crate
- `winappmigrator-gui` crate
- Shared data models
- Shared error types
- Shared scan orchestration API
- CLI command skeleton
- GUI application shell
- Installed app enumeration
- Scan by EXE path
- Basic PE import parsing
- Basic DLL resolution
- JSON report
- Basic Markdown report
- Warnings file
- Project documents
- Project-local agent instructions
- Initial quality gates

CLI acceptance:

```powershell
winappmigrator list
winappmigrator scan --exe "C:\SomeApp\App.exe" --out ".\scan-output"
```

GUI acceptance:

- GUI launches.
- GUI shows app list or clear unsupported/error state.
- GUI allows executable path selection.
- GUI allows output directory selection.
- GUI runs basic scan through shared core.
- GUI shows scan summary, warnings, and confidence score.
- GUI links or opens generated report location.
- GUI remains responsive during scan.

Stage 1 output files from scan:

- `scan_report.json`
- `scan_report.md`
- `dependency_graph.json`
- `warnings.md`

The scanner must identify:

- App executable
- Architecture
- Direct DLL imports
- Resolved DLL paths
- Missing DLLs
- App-local DLLs
- System DLLs that should not be copied
- Basic migration confidence score

### Stage 2: Recursive Dependencies, File Discovery, Hashing, Reports

Implement:

- Recursive DLL scanning
- File discovery from install directory
- Hashing
- Rich Markdown report
- Warning classification
- GUI report summary improvements

### Stage 3: Windows Integration Discovery

Implement:

- Registry discovery
- Shortcut discovery
- Service discovery
- Scheduled task discovery
- GUI tabs/sections for discovered integrations

### Stage 4: Package Creation and Restore Dry-Run

Implement:

- Package creation
- Manifest generation
- CLI package command
- GUI package workflow
- Restore dry-run
- GUI restore dry-run viewer

Package command success criteria:

```powershell
winappmigrator package --exe "C:\SomeApp\App.exe" --out ".\SomeAppPackage"
```

Produces:

- Copied app-owned files
- `manifest.json`
- Reports
- Registry exports if found
- `restore_plan.md`

### Stage 5: Restore Apply Mode

Implement:

- Restore apply mode
- Admin privilege checks
- Safer overwrite handling
- GUI explicit confirmation flow
- Better report formatting

### Stage 6: Advanced GUI/TUI Polish

Implement only after MVP works:

- Dependency graph visualization
- Improved scan progress
- Advanced scan settings
- Trace/snapshot launch points
- Optional TUI only if useful

---

## 25. Advanced Optional Features

Add only after MVP works.

### Dynamic Runtime Tracing

Command:

```powershell
winappmigrator trace --exe "C:\App\App.exe" --duration 120 --out ".\trace-output"
```

Potential implementation:

- ETW or Windows APIs
- Monitor files, DLLs, registry keys, child processes, and network endpoints accessed during capture window

### Snapshot Comparison

Take before/after snapshots of file and registry changes while the app runs.

### Dependency Graph Visualization

Export DOT/Graphviz or Mermaid diagrams.

### Archive Output

Optionally create compressed package archives.

### Plugin Rules

Allow custom YAML/TOML rules for special applications.

---

## 26. Installation Rules

Install only what is needed.

Before adding dependencies:

1. Identify the purpose.
2. Verify current stable version.
3. Verify compatibility with the selected stack.
4. Prefer official, maintained, widely adopted packages.
5. Avoid abandoned, deprecated, or unnecessary packages.
6. Document major dependency decisions in `PLAN.md`.

After installing:

1. Lock dependencies.
2. Run install verification.
3. Run lint/type/build/test gates where available.
4. Update documentation.

No global installs unless explicitly required and documented.

---

## 27. Documentation Accuracy Rule

Keep documentation synchronized with the code.

Before marking any task or milestone complete:

- Verify README commands still work.
- Verify GUI launch/build instructions still work.
- Verify CLI usage examples still work.
- Verify changelog reflects actual changes.
- Verify PLAN status is accurate.
- Remove stale claims.
- Remove obsolete instructions.
- Mark deferred items clearly as deferred, not complete.

---

## 28. Completion Rule

At the end of setup, provide:

- Summary of created files
- Installed dependencies
- Configured quality gates
- Commands to run CLI
- Commands to run GUI
- Commands to build release artifacts
- Known assumptions
- Open questions
- First milestone status
- Any failed or deferred gates
- Next recommended action

Do not claim the project is production-ready unless all documented production-readiness gates pass.

---

## 29. Initial First Action

Start by creating or updating:

- Rust workspace
- Shared core crate
- CLI crate
- GUI crate
- `README.md`
- `CHANGELOG.md`
- `PLAN.md`
- `AGENTS.md`
- `.gitignore`
- Formatting/lint/test/security configuration

Then implement Stage 1 in the smallest safe vertical slice:

1. Shared models and errors.
2. CLI parsing for `list` and `scan`.
3. GUI shell using shared core.
4. Installed app enumeration.
5. Basic PE import parsing.
6. Basic DLL resolution.
7. Basic scan reports.
8. Quality gates.
9. Documentation updates.
10. Stage 1 certification checklist.
