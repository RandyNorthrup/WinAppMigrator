# 0.7.1, released

0.7.1 carries the one fix found in 0.7.0 after its release ([release-0.7.0.md](release-0.7.0.md)): in Italian, the
heading of what goes in an installer ran under its Look Again button. Seen on 2026-11-15, after it happened.

- **Published.** Tag `v0.7.1` on `f794262` ("Cut 0.7.1": the version, the changelog, the conformance report's and the
  issue forms' version). The code is `1e0f2fd`'s, which, like `f794262` itself, passed all 18 CI jobs (runs 43804075617
  and 43810085508). The Release workflow (run 43810089738) built, signed and published
  <https://github.com/RandyNorthrup/WinAppMigrator/releases/tag/v0.7.1>.
- **The download** ([release-0.7.1/download-check.log](release-0.7.1/download-check.log)) matches `SHA256SUMS.txt`, is
  signed by Randy Northrup (valid, timestamped by Microsoft), and both files' build provenance verifies against
  `refs/tags/v0.7.1` at `f794262`.
- **It starts in the lab PC's language** ([release-0.7.1/language-check.log](release-0.7.1/language-check.log)): on
  testpc1, with the display-language list set to Italian, then Arabic, the downloaded exe opened in each, read through
  UI Automation. In Italian the heading now reads "Contenuto del programma di ins…" beside "Analizza di nuovo"
  ([it-IT-app.png](release-0.7.1/it-IT-app.png)); Arabic is as in 0.7.0 ([ar-SA-app.png](release-0.7.1/ar-SA-app.png)).
- **winget:** [microsoft/winget-pkgs#481324](https://github.com/microsoft/winget-pkgs/pull/481324), 0.7.0's 26
  manifests with the version, URLs and SHA-256 changed; `winget validate` passed.
- **The website** names the 0.7.1 downloads on every translated Getting Started page and on the wiki's, and serves them.

The delivery ledger in `docs/plans/i18n-a11y.md` was closed at `1e0f2fd`, whose code this is. Since then only the
version named has changed in files its receipts fingerprint (`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`,
`docs/ACCESSIBILITY.md`, the issue forms, the translated Getting Started pages), so a resume check of the ledger reports
those as changed. Its receipts were not made again for that.
