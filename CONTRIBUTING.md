# Contributing

Issues and pull requests are welcome. For anything large, please open an issue first.

## The rules that shape this project

- **The app builds and the installer restores.** `winappmigrator-gui` builds an installer and never
  restores anything. Restore lives only in `winappmigrator-installer`. Shared logic goes in
  `winappmigrator-core`, and shared UI in `winappmigrator-ui`.
- **One screen, one job.** The app is a single screen that produces one self-extracting `.exe`.
- **Restore is a dry run until told otherwise.** Nothing is installed without `--apply` or the Install
  button.
- **Never sign the installer stub.** See [docs/signing.md](docs/signing.md).
- **Do not copy shared files that an app does not own.** Report shared runtimes as prerequisites.
- **Licensing data is opt-in**, and is meant only for its owner, though nothing can enforce that. Never
  decrypt it, bypass or emulate activation, or emulate a dongle.
- **No panics in normal flow** and no `unwrap` or `expect`: clippy denies them. `unsafe` is denied
  outside `winappmigrator-ui/src/hardening.rs`.

## Before you open a pull request

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
cargo audit
cargo deny check
```

From a normal shell, the end-to-end test skips its reinstall step, because the installer demands
elevation. Run it from an elevated shell to cover the whole flow.

Keep the docs true: update the README, `docs/`, and `CHANGELOG.md` in the same change as the code.

## Changing the UI

Do not drive a live window with scripted clicks. The UI tests drive the real screens headlessly, and
can write the main states as images, so you can see what you changed:

```powershell
$env:WAM_UI_PREVIEW_DIR = "$env:TEMP\wam-preview"
cargo test -p winappmigrator-gui -p winappmigrator-installer --lib -- --test-threads=1
```

See [docs/gui.md](docs/gui.md) for the conventions.
