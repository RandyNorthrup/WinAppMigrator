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
- **While it runs,** its staging folder is writable only by administrators, and a drive that cannot
  keep that, such as FAT32 or exFAT, is not used for it. That folder and the install location cannot be
  renamed and replaced, and it will not write through a link at or below the install location. From
  the moment it opens its own file, that cannot be changed or swapped.

## Undoing an install

A rollback and an uninstall both run elevated, and do what a record says, on locations other programs
may be able to write to.

- **The record is in one fixed place.** Each install has a folder at
  `%ProgramData%\WinAppMigrator\installs\<id>`, and nothing reads a record from anywhere else, or takes
  a path to one. Anyone can make a folder in `ProgramData`, so each level is checked to be a real folder
  that administrators own and only they can change, and one that is not is never used. If it cannot be
  made safe, the install does not start, since it could not be undone. An id must be what this code
  would have named a folder before it goes near a path. Nothing a package installs can go there.
- **It can hold secrets.** The record keeps registry values and environment variables as they were, so
  it is closed to everyone else. None of that goes in a log, which holds paths and key names only, and
  which only administrators can read. A file it replaced is moved aside with the permissions it had, so
  whoever could read it before still can, by a path they would have to know.
- **Links.** Before each delete and restore, it holds every folder on the way, checks that none of them has
  become a link, and that the folder it is all under still leads where it did during the install.
  Whatever fails that is left alone and reported, even if what is at the other end is byte for byte what it
  would have removed. It checks again after the last thing that can be made to wait. This is by path, so
  what is listed under [known gaps](#known-gaps) for an install location others can write to applies here too.
- **Only what it did.** An uninstall, and undoing an install that never finished, delete a file only if it is
  still what the install wrote, and put back a value only if it still holds what the install wrote. They
  remove only the entries added to `Path`, a key or folder only if it is empty, and a service or task only if
  it still runs what was installed. The package's own say-so about how to remove it, such as
  `UninstallString`, is dropped when installing. Only one install or uninstall runs at a time.
- **The uninstaller is never in the install folder,** where non-admins may be able to replace it. Settings
  runs it elevated, so it is in the admin-only folder, where anyone can start it and find out it needs
  elevation, and only administrators can change it. It is never signed, even when the installer was:
  [why](docs/signing.md#the-uninstaller).
- **It removes itself without running anything from a place others can write to.** A running program cannot
  delete its own file, so after it exits, `cmd.exe` from the Windows folder removes the folder. It runs
  from its own folder, with the whole command on its command line and `ping` by its full path, and with
  `/D`, so that the `AutoRun` command in the signed-in user's registry does not run with it. If it cannot, what
  is left is that file, in that folder, and the next install removes it.

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
  programs on the PC. Links are checked just before each file is written, removed, or put back, but a
  program running at that moment could still race the check, which matters most for what an elevated
  uninstall deletes. It can also change the installed files afterward. Choose a location under
  `C:\Program Files` for anything that installs a service or a driver. Closing this needs acting through a
  handle to the folder rather than by a path, which this project's rules on `unsafe` code do not allow.
- A file or folder in an install that someone else can write to can be kept open to make an undo fail. It
  is reported and can be tried again, and an install that never finished stops the next install until it
  can be undone. If that cannot ever happen, an administrator removes its folder under
  `%ProgramData%\WinAppMigrator\installs`.
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
