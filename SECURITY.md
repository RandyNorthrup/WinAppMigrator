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
- **Links.** An install location that is a link or under one is refused, and so is a data folder it would
  write to, because whoever can write beside a link can change where it leads at any moment, with no need
  to empty anything. Use the place it leads to instead. A volume mounted on a folder is allowed if it has no
  drive letter of its own, though if others can write beside that folder, they can change that too. For each
  delete and restore, and each file an install writes, it holds every folder on the way and keeps a hidden
  file open in the last one until it is done. A held folder cannot be moved, and a folder with something held
  in it cannot be emptied, which it would have to be to become a link. The folder it is all under is held
  for the whole install, so what is above it cannot change, and it is checked to be real again for each file.
  So making what it does wait, which whoever can write to a file can do to a rename, does not let anyone lead
  it somewhere else. An uninstall also checks that the folder still leads where it did during the install.
  Whatever fails any of that is left alone and reported, even if what is at the other end is byte for byte
  what it would have removed. The hidden file goes in by a path, so see [known gaps](#known-gaps).
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
  programs on the PC, which can change the installed files afterward. Choose a location under
  `C:\Program Files` for anything that installs a service or a driver. While one is being installed or
  undone, [each folder is pinned](#undoing-an-install), and a program running with ordinary rights has one kind
  of way left past that, which is to win a race. It has to turn an empty folder on the way into a link just as the
  hidden file goes in, so that it lands elsewhere, then read that file's name, turn the folder back, and
  put a file of that name there, all before the next check a few microseconds later. An oplock
  cannot pause the steps it has to beat: on Windows 11 we measured the file going in, and the checks on the
  folder and the file, none of them waiting, though a rename's source did wait for over 5 seconds.
  But an oplock on the folder does tell it the moment the file goes in, a program keeping every CPU
  busy could stretch that gap, and other ways of stalling a file being made, such as a cloud sync provider's,
  were not tested, so this has not been shown to be impossible. An install stops at the first sign of it, and
  so does an uninstall for that folder, so a run gives it a few tries at most, and not one for each file. Closing it needs
  acting through a handle to the folder rather than by a path, which this project's rules on `unsafe` code do
  not allow. A power cut in that moment leaves an empty hidden `.wam-pin-` file, which does nothing, and
  goes if the folder it is in does.
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
