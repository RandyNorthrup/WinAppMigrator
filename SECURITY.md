# Security

## Reporting a vulnerability

Please report privately through
[GitHub's security advisories](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new)
rather than in a public issue. Include the version and the steps to reproduce it.

## What to know before you run an installer

An installer built with WinAppMigrator runs **elevated** and reinstalls what its package
describes: files, registry values, services, drivers, scheduled tasks, shortcuts, and runtime
installers.

**The package is not authenticated.** Its hashes catch damage, and an edit by anyone who does not
also update them, because they travel with it. An installer can only be proven to come from
someone by signing the finished file ([how](docs/signing.md)), and an unsigned one proves nothing:
Windows shows "Unknown publisher" for it.

- **Only run installers you built yourself, or got from someone you trust.** Treat one like any
  other program from that source. Whoever hands you a whole executable needs no bug in this
  project to run anything they like.

What the project does protect against is a package that comes from a hostile or merely odd source
drive through the genuine builder, another program on the same PC changing what the elevated
installer is about to trust, and an honest package reaching too far:

- **The package cannot name just anything.** When restoring, a list of keys no app install should
  write is refused, and so are services, shortcuts, scheduled tasks, and autostart values that run
  anything other than a file the package installs. It cannot install to a system folder or take
  over a service, task, or COM class already on the PC. Which installer runs for a runtime, and how,
  is compiled in, and the file must be signed by Microsoft. What is refused is listed. The lists are
  of what is known to be dangerous, so a package can still do something they do not name.
- **The rules do not make a package safe.** A package may still run its own files, as a service
  with full rights if it asks to. That is what installing an app is. Before anything happens, the
  installer's window lists the services, drivers, scheduled tasks, autostart entries, shortcuts, and
  `Path` changes it will set up, what it will leave out, and whether it is signed and by whom. Read it.
- **While it runs,** its staging folder is writable only by administrators, that folder and the
  install location cannot be renamed and replaced, and it will not write through a link at or below
  the install location. From the moment it opens its own file, that cannot be changed or swapped.

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

[docs/PLAN.md](docs/PLAN.md#open-issues-most-important-first) lists open issues. These are the ones
that matter most for deciding whether to run an installer:

- An install location outside a protected folder, such as `D:\Apps`, stays writable by other
  programs on the PC. Links are checked just before each file is written, but a program running at
  that moment could still race the check, and it can change the installed files afterward. Choose a
  location under `C:\Program Files` for anything that installs a service or a driver.
- A program already running on the PC can rename a running installer and put another file at its
  path in the moment before the installer opens its own file. The installer then reads that file's
  package, and shows that file's signature, not the one you approved at the UAC prompt. So read the
  signature line in the window, not just the UAC prompt. Closing this needs a handle to the running
  image, which this project's rules on `unsafe` code do not allow.
- A package that may run its own files can also run them as a Microsoft-signed runtime installer's
  neighbor, since that installer loads DLLs from its folder. This adds nothing to what such a package
  can already do with its own files.

## Licensing data

WinAppMigrator helps an owner move their own license. It reports whether a license is likely tied
to the machine, and restores what can be restored. It never decrypts license data, bypasses or
emulates activation, or emulates a hardware dongle. You are responsible for following the license
terms of the software you migrate.
