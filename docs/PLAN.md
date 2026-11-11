# WinAppMigrator Plan

## Product (locked)

WinAppMigrator backs up an installed Windows application into a single
**self-extracting installer**, or, for a package past the 4 GiB Windows can start, a small
loader and a data file beside it. There are two binaries with a strict split:

- **`winappmigrator-gui` — the app** (binary `winappmigrator.exe`). Runs on the
  source machine. Lists installed apps — from **this PC or a connected drive** (a
  disk pulled from another, often failing, PC) — the user picks one and optionally
  includes its license and/or its user data. The app reverse-engineers the
  application's full footprint and builds one self-extracting `.exe`, or a loader and a data
  file if it will not fit in one, hash-verifying the package. It never restores anything, and always runs elevated
  (every build embeds a `requireAdministrator` manifest).
- **`winappmigrator-installer` — the installer stub.** Embedded into the produced
  `.exe`. Runs on the target machine: checks that the app can run there and that there is room,
  extracts its package, reinstalls the app and bundled data, and hash-verifies the result.
  It is the only caller of the restore code, which lives in `winappmigrator-core`.
- **`winappmigrator-client` — WinAppMigrator Client.** Carried by the app, which saves it for a person to
  run on a PC an app is moving off, Windows 7 or later, so that the app can read that PC over the network.
  It captures nothing: it takes a snapshot of the Windows drive and answers the app's requests to describe,
  list and read paths in it, read-only, over an encrypted link paired by a code it shows
  (`winappmigrator-link`). One 32-bit program, built for Windows 7 by `scripts/build-client.ps1`.

`winappmigrator-core` holds all shared logic (inventory, install-source
reverse-engineering, footprint discovery, packaging, restore, verification).

## Done

- Single-screen GUI: app list → pick → what goes in, from a scan in the background →
  optional license / user-data toggles and what to leave out → build self-extracting
  installer, with progress and cancel. Self-extracting output is mandatory.
- What goes in is shown before anything is built, in the words of the installer's own list,
  with how big it can be, whether it fits, and what is left out and why. Choices only take away,
  and what is left with nothing to run goes too and is said to.
- Near matches: what exact names leave out and looks related, offered unticked with why, and
  brought in through the scan by the same rules as any other root.
- Each byte of an app is read once to build: hashed and compressed from what that read returned, straight
  into the installer, in parallel and in a fixed order, with already-compressed data stored. The package is
  then read back to check it. An install hashes as it unpacks, and reads back what it installed.
- A size and free-space check before a build and before an install, on every drive each writes to.
- Past the 4 GiB Windows can start: a small loader whose footer holds the hash of a data file beside it, so
  a signature on the loader covers the data file.
- A check of the target's Windows and architecture against what the app's programs and drivers say they
  need, which refuses what cannot run there and warns of what may not.
- A real move between PCs on CI: built on one runner and installed on others, on Windows Server 2022 and 2025
  and Windows 11 on Arm, and checked with Windows' own tools.
- The same with real open-source apps from their publishers, with Windows asked to use what was restored: a real
  archive, a COM class activated, a file type opened, a shortcut started, and services answering a real client, each shown
  to fail without the app, and the machine compared with how it was after an uninstall.
- Scan Source selector: **this PC or a connected offline drive** (a disk from
  another PC). Offline reads load copies of the disk's registry hives, compare what
  its registry says with what that PC called things, and map a path to the volume in one
  place, which will not lead off it. The app writes nothing to the drive. The app runs
  elevated, so there is no mid-task UAC prompt.
- A disk image as the source: a `.vhd` or `.vhdx` the app attaches read-only itself, with no drive letter, and detaches.
- Another PC over the network as the source: **Network PC…** saves WinAppMigrator Client to a USB drive or a folder, finds
  it on the network, and pairs by the code it shows (SPAKE2, then ChaCha20-Poly1305). The app captures through it as from a
  connected drive, reading only what the build needs from a snapshot of that PC's drive. Run for real from Windows 7 SP1
  VMs, 32-bit and 64-bit, and Windows 11 25H2.
- A drive or an image from Windows 7, Vista, 8 or 8.1, onto Windows 10 or 11: which Windows it is, said in words; an app from
  32-bit Windows laid out as 64-bit Windows keeps it, in `Program Files (x86)` and the 32-bit registry view; what Windows kept
  aside for an older app (`VirtualStore`) put in the app's own folder and keys with User Data, since Windows 11 ignores a
  `VirtualStore` key an installer makes; the DirectX End-User Runtime as a prerequisite; DLLs from the old Windows folders that
  Windows 11 lacks named; and shortcuts written with a variable read. Run for real from Windows 7 SP1 VMs, 32-bit and 64-bit,
  onto Windows 11 25H2.
- One account's data in an installer: whoever is signed in, or the account chosen when
  more than one has data, going to whoever is signed in where it runs. Documents and Saved
  Games go to their real folders.
- Installer stub: hash-check the package → reinstall → hash-verify the reinstalled
  files, with progress and cancel; from a script, non-zero exit on any mismatch, and in the window, a failure says so.
- A change journal for each install, in an admin-only folder, with rollback on any failure,
  cancel, or an install that never finished, and a generated uninstaller run from the app's
  entry in Settings. Persistent, secret-free logs.
- Layered footprint discovery (MSI component DB, Inno `unins*.dat`, NSIS
  `uninstall.log`, App Paths, plus a Program Files/ProgramData/AppData walk), PE/delay-import/SxS/.NET analysis, runtime-prerequisite
  detection, and authoritative file counts.
- What an account keeps for an app in the registry goes by the User Data option, shown on real apps with marks searched for
  through a whole package, and with it off, a package says nothing of who built it but what the app needs to work, and a build
  looks through what the package says about the app, though not the app's own files, for anything that does. An app's empty folders are made again.
- A PC with no Visual C++ runtime, tried for real, and real windows opened on runners that use a software renderer, with what a person is
  told when graphics cannot start. Each window fits the screen it opens on, and goes by one rule for light and dark.
- Optional per-user/ProgramData data backup and optional license detection/
  backup (opt-in; never decrypts, bypasses activation, or emulates dongles).
- End-to-end test (`crates/winappmigrator-installer/tests/installer_e2e.rs`) runs the
  real stub binary: reverse-engineer → build SFX → reinstall on a clean target →
  verify. Core unit suite covers discovery/packaging/SFX/restore/verification, and
  headless UI tests drive the real screens of the app and the installer.
- Elevated round trip (`crates/winappmigrator-installer/tests/elevated_round_trip.rs`), run only by
  its own CI job on GitHub's throwaway runner: scan, package, remove, and reinstall a fake app, then
  check the machine with Windows' own tools, and undo it by an uninstall, a late failure, and being killed part-way. Its sibling
  (`elevated_offline_and_profiles.rs`) does the same from a real NTFS drive made hostile on purpose,
  with real second accounts, and against every real scheduled task. See
  [CONTRIBUTING](../CONTRIBUTING.md).
- What ties an app into Windows: COM with its type libraries, interfaces, and AppIDs, ProgIDs, URL
  protocols, `OpenWithProgids`, shell extensions, `RegisteredApplications`, firewall rules, fonts, ODBC, event
  log sources, compatibility settings, and 32-bit Run and App Paths, each the app's by what it points at, all
  restored by rules, in the consent list, and journaled. Certificates are only reported.
- Credentials are left out and named, whatever else was asked for, and with the License option off so are
  licenses. What that cannot catch is in [SECURITY](../SECURITY.md#an-installer-can-contain-secrets).
- Runtimes read from all of an app's programs: Visual C++ by each version, and .NET from its
  `runtimeconfig.json` and its own metadata, telling .NET Framework from modern .NET.
- Dev/release build profiles tuned (deps + first-party optimized in dev for fast
  tests; LTO + single codegen unit + strip in release).

## Definition of done (every change)

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --release -p winappmigrator-installer
scripts/build-client.ps1
cargo build --workspace --release
cargo audit
cargo deny check
```

Do not add `--all-targets` to `cargo test`: it also runs the executables as test
harnesses, and they demand elevation, so it fails with error 740 from a normal shell.
Their tests live in libraries.

Dead, orphaned, or legacy code is not left in the tree; clippy runs with
warnings denied, so unused code fails the gate. GUI changes are verified with the
gates, the headless UI tests and their rendered previews (`WAM_UI_PREVIEW_DIR`), and
a single launched-window screenshot — never scripted clicks on a live window.

## Open issues, most important first

From the audit of 2026-11-01. Items are removed from this list as they are fixed.

**Security and data safety**

1. **An unsigned package still cannot be authenticated.** Restore rules limit what a package
   can ask for, but it can run its own files with full rights, and its hashes travel with it.
   Only signing a finished installer proves origin. This is what a self-extracting package is, and no fix is planned.
2. **Close the two races without lifting the `unsafe` ban,** after `0.5.0`. A program already on the PC can race the link checks
   in an install location outside a protected folder, and can swap a running installer's file in the moment before it opens it.
   Each comes from acting by a path, and closing them means acting through a handle: to the folder, relative to it and
   following no link, and to the running image. Look for a safe crate that does this, such as `cap-std`, before reaching for
   `unsafe`. [Known gaps](../SECURITY.md#known-gaps) says what each can do until then.

An install location outside a protected folder also stays writable by other programs on the PC after an install, which a person can
avoid and code cannot: see [Known gaps](../SECURITY.md#known-gaps).

## Not in scope for this release line

- MSI/MSIX/setup installer output (the self-extracting `.exe` is the product).
- A TUI.

## Code signing

Releases sign the app and WinAppMigrator Client. The installer stub stays unsigned on purpose, and installers
are signed only by whoever builds them. See [signing.md](signing.md).
