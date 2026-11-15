# Readiness re-review 2: after T-I18N-CRATE (WORK-I18N-A11Y)

2026-11-15. The first review's receipt no longer matches its inputs, because this task changed what readiness reads:

- `Cargo.toml` adds the `winappmigrator-i18n` member and four workspace dependencies (`fluent-bundle` 0.16.0,
  `fluent-syntax` 0.12.0, `intl-memoizer` 0.5.3, `unic-langid` 0.9.6); `Cargo.lock` follows. `cargo deny check` passes
  (advisories, bans, licenses, sources), and `cargo audit` reports only the warnings the project already allows.
- `T-I18N-CRATE` names two more files it made: `crates/winappmigrator-i18n/src/tests.rs` (the crate's tests, kept out of
  `lib.rs` for length) and `scripts/red-drill.ps1`, the repeatable drill every later task uses, which refuses to run in
  the repository itself.

Nothing in the requirements, the acceptance, or the order of tasks changed, and no finding of the first review is
reopened. One observation for later tasks: `negotiate` now takes which languages ship, so a language listed in the table
without a catalog is never chosen; until `T-TRANSLATE`, that means English everywhere, which is the intended interim.

Result: still ready; `T-UI-TEXT`, `T-CORE-TEXT`, `T-CLIENT-TEXT`, and `T-UNICODE` may start.
