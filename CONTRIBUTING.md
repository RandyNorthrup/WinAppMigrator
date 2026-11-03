# Contributing

Issues and pull requests are welcome. For anything large, please open an issue first.

## The rules that shape this project

- **The app builds and the installer restores.** `winappmigrator-gui` builds an installer and never
  restores anything. Restore lives only in `winappmigrator-installer`. Shared logic goes in
  `winappmigrator-core`, and shared UI in `winappmigrator-ui`.
- **One screen, one job.** The app is a single screen that produces one self-extracting `.exe`, or, past
  the 4 GiB Windows can start, a small loader and a data file beside it.
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
cargo build --release -p winappmigrator-installer
cargo build --workspace --release
cargo audit
cargo deny check
```

Rust comes through rustup, which picks the version pinned in `rust-toolchain.toml`. The last two need
`cargo install cargo-audit cargo-deny --locked`. `cargo quality` and `cargo typecheck` are short for the tests and
for `cargo check --workspace --all-targets --all-features`. The stub is built first because the app carries it, and
`cargo build --workspace` alone can leave an app holding the build before's, so CI checks the app for its exact bytes with
`.github/scripts/check-stub-embedded.ps1`.

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

`elevated_packaging.rs` has a job of its own too. A file is written to the whole time it is being built from,
and what is in the package is compared with what its manifest says by .NET's own zip reader. A loader is given a
valid signature from a throwaway certificate, and its data file is then changed by one byte, swapped for another
package, cut short, removed, and replaced by a link, with `Get-AuthenticodeSignature` saying what it sees of each.
A small VHD with too little room is what an install and a build are refused on, before anything changes, and
packages say they need what the runner is not. It also says how much a build and an install really read, by
Windows' own count. `elevated_big_package.rs` builds, installs, and checks a real package past 4 GiB, with a real
file over 4 GiB of data that does not compress, on a runner with the room to itself.

`elevated_footprint.rs` has a job of its own too. A fake app has things to leave out, a hidden part to add, and things
that must never be offered: a shared folder, another app's, a link, what is named like Windows', and hidden entries that
name each of those. What is in the package goes by .NET's own zip reader and a search of what that unpacks, and what is on
the machine by `sc.exe`, `schtasks.exe`, and `Get-FileHash`, then it is uninstalled and made to fail part-way.

`elevated_move.rs` and `.github/scripts/check-moved-app.ps1` are a real move between two PCs. One job builds
installers of a fake app and uploads them, unsigned and kept for a day, and others download them onto runners that
have never seen it, each on a Windows of its own, which build nothing and check out only the scripts. They install
them there and look with Windows' own tools, and say what differed.

`elevated_real_apps.rs`, `.github/scripts/real-apps.json`, and the `real-app` scripts beside it do that with real apps.
`install-real-apps.ps1` runs each publisher's installer on a runner, at the version and SHA-256 in the JSON, and asks the
same of it as `check-moved-real-apps.ps1` later asks of the move, so a failure there is no fault of this project's. The test
builds the installers the way the app does, with the License and User data options off and no runtime bundled, and looks
through each package for the runner's account, folders, and tokens before anything is uploaded. On each fresh runner the script removes any
copy it came with, shows that every check fails, installs, shows that every check works, removes each app by the string its
entry recorded, shows that every check fails again, and compares the machine with how it was, by each registry value the
packages say they set and by the names of the services, tasks, firewall rules, shortcuts, variables, and folders an app could add,
once where each app was and once in a folder of another name. A check in `real-app-checks.ps1` asks Windows to do the thing rather than looking for a file or a key,
so that only what was restored can make it pass. **Add a check only if it can fail, and never widen one to go green.**

`install-real-apps.ps1` also registers a scheduled task running 7-Zip as the runner's account, written by it and waiting for it,
which also runs something from its folders, and leaves shortcuts to 7-Zip on its Desktop, one with arguments. The search for
what is the runner's goes through what is built from those, the build's own last look has to find nothing in these apps, and each
fresh runner has to end up with the task running as its own account with nothing of the other's, and the one shortcut.

`install-real-apps.ps1` also leaves what an account would have in 7-Zip's and PuTTY's keys, with marks in it that are in
nothing else. The build goes through each package for those marks, in every form a value is written in, and none may be there
with the User data option off. A step of that job that uploads nothing builds again with it on, where each has to be there, and
`check-account-settings.ps1` installs that on the same runner, and shows each value arrive and go. `Real apps, a PC with no runtime`
builds Mosquitto with its runtime in it, and `check-runtime-arrives.ps1` takes that runtime off the runner and shows the broker
cannot start, then installs with `TEMP` set to a folder nothing may use, and has it serve a client. `Elevated packaging` runs the
app's own code and a real install with `TEMP`, `ProgramData`, `ProgramFiles`, and `SystemRoot` all set wrong, shows that a plain child
goes by them, and that nothing here does. `Real windows` opens the app, an installer and an uninstaller on each kind of runner,
and looks at what is on the screen, without clicking anything. An uninstaller there has no install to undo, so what it shows is
that it has no record of one. Each has to fit in what Windows says a window can use, at the
runner's 1024x768, and be in the mode Windows is in, set both ways, or the one chosen in the app. It does the same again with
graphics that cannot start, which it forces with `WGPU_BACKEND`, and with the environment naming a folder of Direct3D's code, where it plants a stand-in built from
`planted-d3d12core.rs` that only says it ran, and none of it may. A copy of Windows' own would not do: Direct3D only loads one
newer than its own, and does not keep it loaded, so a list of what is loaded misses it. Its pictures are kept for a day, of the test app and of the runner's own list of what it has installed.

What those jobs upload is a redistribution of those apps, for a day, to anyone signed in to GitHub, so an app goes
in the list only if it is open source and its license allows that, and the workflow says which license. Those jobs are given no
secrets, keep no credentials from the checkout, and do not use the `release` environment. Windows' own packages add
firewall rules of their own whenever one first starts, so those are the one thing the comparison does not count.

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

## Measuring a build

`cargo run --release -p winappmigrator-core --example wam_measure -- "<app name>" <stub.exe> <work folder>` builds an
installer of a real app on this PC, and says how long each step took, how much it read and wrote by Windows' own
count, and the most it had on disk. It reads the app, writes only in the work folder, removes that folder afterward,
and never installs anything. Use a folder of your own for that, and read `README.md` on what a package can hold.

## Trying a scan or a build by hand

`cargo run --release -p winappmigrator-core --example wam_pkg -- <source_exe> <stub_exe> <out_dir> <app_name>` builds an installer
from one executable, as the app would, and `... --example offline_scan -- <volume_root> [output_file]` lists the apps on a mounted
disk. That one needs an elevated shell, as it loads copies of the disk's registry hives. Neither installs anything.

## Seeing what near matches would offer

`cargo run --release -p winappmigrator-core --example wam_near_matches -- <out.txt> [--scan "<app name>"]...` says what would
be offered for every app listed on this PC, and what looked close and was refused and why. With `--scan`, that app also has its
whole scan run, with how long it took. It reads names, counts a little of what is offered, and writes only the file it is
given. That file holds paths of this PC, so keep it out of anything you share.

## Changing the UI

Do not drive a live window with scripted clicks. `Real windows` on CI opens the real programs and only looks. The UI tests
drive the real screens headlessly, and
can write the main states as images, so you can see what you changed:

```powershell
$env:WAM_UI_PREVIEW_DIR = "$env:TEMP\wam-preview"
cargo test -p winappmigrator-gui -p winappmigrator-installer --lib -- --test-threads=1
```

See [docs/gui.md](docs/gui.md) for the conventions.

## Cutting a release

Versions and tags have three parts, as in `v0.5.0`. The two four-part tags from before that stay as they are.

1. In `Cargo.toml`, set the workspace `version` and the two path dependencies under `[workspace.dependencies]`, and let
   `Cargo.lock` follow with a `cargo build`.
2. In `CHANGELOG.md`, make `## Unreleased` into `## X.Y.Z — YYYY-MM-DD`.
3. Run `cargo about generate about.hbs -o THIRD-PARTY-LICENSES.md` from the repository's root. A release generates its own
   for the zip, and this keeps the one in the repository the same.
4. Run the checks above, push `main`, and wait for CI to pass on that commit.
5. Tag that commit and push the tag: `git tag vX.Y.Z`, then `git push origin vX.Y.Z`.

`.github/workflows/release.yml` then runs. It stops unless the tag is `v` and the version in `Cargo.toml`, runs clippy and the
tests, stops if the stub has a signature or the app does not hold the stub byte for byte, signs the app, checks that signature,
records where the downloads were built, and publishes them. Only a `v*` tag can reach the signing, in the `release` environment. To
retry a run that failed before publishing, start the workflow from that tag. If the release already exists, delete it first. See
[docs/signing.md](docs/signing.md) for how signing is set up, which a fork does not have.
