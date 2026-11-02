# Security

## Reporting a vulnerability

Please report privately through
[GitHub's security advisories](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new)
rather than in a public issue. Include the version and the steps to reproduce it.

## What to know before you run an installer

An installer built with WinAppMigrator runs **elevated** and reinstalls whatever its package
describes: files, registry values, services, drivers, scheduled tasks, shortcuts, and runtime
installers. The package is not authenticated, and its hash checks catch damage, not tampering,
because the hashes travel in the same archive.

- **Only run installers you built yourself, or got from someone you trust.** Treat one like any
  other program from that source.
- **An unsigned installer proves nothing about who made it.** Windows shows "Unknown publisher"
  for it. Signing a finished installer ([how](docs/signing.md)) lets the recipient check that it
  is unchanged and who signed it.

## An installer can contain secrets

Treat an installer as being as sensitive as the PC it came from. WinAppMigrator does not hunt
for secrets, but it copies what an app keeps in its own folders and registry keys, which can
include license keys, tokens, and saved passwords. That happens even with the License option
off, for example a license file in the install folder or a value under the app's registry key.
The User data option adds settings and documents. The package also records the name of the
computer that built it.

## How the executables protect themselves

Both executables run elevated and the app is signed, which makes them targets for loading someone
else's code under their name. [docs/signing.md](docs/signing.md) covers the protections, and why
the installer stub must never be signed.

## Known gaps

[docs/PLAN.md](docs/PLAN.md#open-issues-most-important-first) lists open issues, including
restore-time allowlists.

An install location outside a protected folder, such as `D:\Apps`, stays writable by other
programs on the PC. Links are checked just before each file is written, but a program running
at that moment could still race the check, and it can change the installed files afterward.
Choose a location under `C:\Program Files` for anything that installs a service or a driver.

## Licensing data

WinAppMigrator helps an owner move their own license. It reports whether a license is likely tied
to the machine, and restores what can be restored. It never decrypts license data, bypasses or
emulates activation, or emulates a hardware dongle. You are responsible for following the license
terms of the software you migrate.
