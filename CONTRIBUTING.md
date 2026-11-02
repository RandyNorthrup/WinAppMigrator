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
elevation. From an elevated shell it stops at a preview, because an install writes to `ProgramData`
and to Settings. It installs for real only on GitHub's runner.

## Tests must not change the machine

Tests and manual runs stay in temp folders. What really changes a PC is covered by
`crates/winappmigrator-installer/tests/elevated_round_trip.rs`, which runs only in the
`Elevated round trip` job in CI, on GitHub's runner, which is elevated and is thrown away afterward.

It sets up a harmless app there, with files, services, a driver entry, scheduled tasks, registry
values, environment variables, and shortcuts. It builds an installer from that with the real code,
removes all of it, runs the installer, and checks the machine with Windows' own tools, not with the
app's verifier. Packages written by hand cover what a scan cannot produce, such as asking for what
the rules refuse.

Another app is tied into Windows every way there is: COM, a link protocol, a file type, a shell extension,
firewall rules, an event log source, fonts, an ODBC driver and source, 32-bit Run and App Paths, and compatibility
settings, with another app's entries beside it from before to the end. Canaries in what looks like a credential
or a license are searched for in the unpacked package and the logs, and hostile packages ask for each thing
the rules refuse, and for what is already there. Each runtime spec's URL is really downloaded and checked as
Microsoft's.

It then takes that install back out every way that can happen, comparing the machine before and after:
an uninstall run from the command line Windows has for it, with and without things another program
changed in between, a failure late in an install, and an installer killed part-way and while undoing
that. A real account that is not an administrator tries to change what an uninstaller trusts.

Its sibling, `elevated_offline_and_profiles.rs`, has a job of its own, so that it stands on a clean
runner. It makes a real NTFS VHD from what is really on the runner, laid out as a Windows install, with
hives `reg save` wrote and edited to be another PC's. Its values point at a marker on the runner in every
way they can be written, and links on it lead back there. The app is taken off the runner before the drive
is scanned at another letter, the drive's tree is hashed before and after, and a scan is killed with hives
loaded to see the next start sweep them. It also makes a second real account with a real profile, whose hive
is not loaded, and compares what the task reader says of every real task with `schtasks`.

They do nothing unless `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted`, and
`WAM_ELEVATED_E2E=1` are all set, and with them set they fail if not elevated. **Never set those on a real
PC.** Without them they skip, so they are safe to run locally, and that shows only that they compile.

When you change how something is restored, record what it replaces in the journal, teach
`rollback.rs` to undo it, extend this test, and push to see it run. What it cannot do: load a driver,
since one that is not signed must never start, sign in, so Run values and Startup shortcuts are checked
but not run, and click in a window, so the uninstaller's is only started.

**Logs hold paths and key names only.** What a registry value, license file, or environment variable holds
must never reach one. `crates/winappmigrator-core/tests/log_has_no_secrets.rs` pins that.

Keep the docs true: update the README, `docs/`, and `CHANGELOG.md` in the same change as the code.

## Changing the UI

Do not drive a live window with scripted clicks. The UI tests drive the real screens headlessly, and
can write the main states as images, so you can see what you changed:

```powershell
$env:WAM_UI_PREVIEW_DIR = "$env:TEMP\wam-preview"
cargo test -p winappmigrator-gui -p winappmigrator-installer --lib -- --test-threads=1
```

See [docs/gui.md](docs/gui.md) for the conventions.
