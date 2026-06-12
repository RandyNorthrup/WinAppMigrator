# WinAppMigrator Agent Instructions

Use `/caveman` status updates in chat: short, direct, no filler. Keep code, docs, tests, and architecture rigorous.

## Project Rules

- Build a Windows-only Rust desktop migration analyzer named `WinAppMigrator`.
- Keep core logic in `winappmigrator-core`; CLI and GUI must call shared core APIs.
- Do not add global tools, global IDE settings, machine-wide instructions, or global memory.
- Do not export secrets, license bypass artifacts, credentials, or unrelated user data.
- Restore behavior must default to dry-run and require explicit apply.
- Per-user and per-machine installations must be handled equivalently; preserve archived scope and require elevation for machine-wide registry, environment, service, task, and shortcut apply work.
- Do not copy non-app-owned shared files outside the app directory. Report shared runtimes as prerequisites.
- Package archives may be directory, zip, or generated self-extracting exe; all archive extraction must prevent path traversal.
- Keep documentation synchronized with implementation.
- Update `CHANGELOG.md` after meaningful changes.
- Update `PLAN.md` when scope, assumptions, dependencies, or gate status changes.

## Quality Gates

Run the relevant local gates before claiming completion:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo build --workspace --release
cargo audit
cargo deny check
```

Do not bypass failing gates. Record failures and fixes.

## Code Standards

- No normal-flow panics.
- No fake implementation paths.
- No unused dependencies.
- No stale docs.
- Use Windows-safe path handling.
- Use structured errors and warnings.
- Prefer conservative scan and restore decisions.
