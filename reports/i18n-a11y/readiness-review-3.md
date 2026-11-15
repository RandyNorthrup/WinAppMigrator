# Readiness re-review 3: catalogs split by part (WORK-I18N-A11Y)

2026-11-15. A language's catalog may now be several `.ftl` files, concatenated in name order when built
(`crates/winappmigrator-i18n/build.rs`), and `t_english!` formats a message in English whatever the language in use.

Why: moving text into the catalog is the largest part of this work, and it can then go on in several parts of the program
at once without two of them editing one file; translators also get pieces of a size they can review.

Plan changes, with no change to any requirement, criterion, or the order of tasks:

- Each text task creates its own English file: `ui.ftl` (`T-UI-TEXT`), `core.ftl` (`T-CORE-TEXT`), `app.ftl`
  (`T-APP-TEXT`), `installer.ftl` (`T-INSTALLER-TEXT`), `client.ftl` (`T-CLIENT-TEXT`). `winappmigrator.ftl` keeps the
  shared messages (sizes).
- `T-TRANSLATE` creates the same six files for each of the 23 languages.
- Message names start with their part (`ui-`, `core-`, `app-`, `installer-`, `uninstaller-`, `client-`), so files cannot
  clash; the catalog test already fails on a name given twice, since a bundle refuses a duplicate.

`T-I18N-CRATE`'s tests and five drills were run again on this code; its earlier receipts are kept as stale history.

Result: ready; extraction may proceed in parallel by part.
