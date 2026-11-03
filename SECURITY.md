# Security

## Reporting a vulnerability

Please report privately through
[GitHub's security advisories](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new)
rather than in a public issue. Include the version and the steps to reproduce it.

## What to know before you run an installer

An installer built with WinAppMigrator runs **elevated** and reinstalls what its package
describes: files, registry values, services, drivers, scheduled tasks, shortcuts, firewall rules,
and runtime installers.

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
- **What ties an app into Windows has rules of its own,** in one place. What makes a class, interface, type
  library, AppID, event source, or ODBC driver or source unusable takes all of it, and a value refused for what it
  says by itself, such as an icon on a share, takes only that value:
  - A COM class's server, a link protocol's command, and what a shell extension loads have to be files the
    package installs. Windows' own only counts as a server from a short list of what hosts a class, by exact name
    and with nothing after it. So `scrobj.dll` and `cmd.exe` do not count, and neither does a share.
  - A file type can run through a program of Windows' only from a short list, such as `wscript.exe` and
    `rundll32.exe`, by its full path, and only if it is handed a file of the package straight away. `cmd.exe /c anything <file>` does
    not count, and what comes after that file goes to it. A value registering any of this cannot start with a network
    path, even for an icon, since Windows opens one just by showing it. One that reaches a share through a variable is
    not caught by that, and it is shown in the list of what is set up.
  - An interface goes through a class of the package, or Windows' marshaler and a type library of the package. An
    AppID cannot run as an account or on another PC. `AppID\<program>` has to be for a file of the package, and not
    one in Windows' own program folders. That is by file name, so a package could ship a file named for
    another app's program, if that app has no such key.
  - `Applications\<program>`, which is what "Open with" goes by, has to be for a file of the package too. A verb
    can only run what its command says: `IsolatedCommand` is held to the same rule, and `CommandStore` is refused.
  - A firewall rule is for one program the package installs, never for all, and cannot override other rules or let
    traffic in through NAT. It is set through `netsh`, and an uninstall takes out a rule of that name, way, program,
    protocol, and ports, if it is still as it was added. One that says something else by now stays, and it says so.
  - An event log source is only a new one, whose messages come from files the package installs. Nothing about a log
    itself can be changed, and no source is added to the Security log.
  - An ODBC driver's files have to be the package's, and a source's driver has to be the package's or Windows', with
    what else it loads the package's. ODBC's own settings, including what traces every call, cannot be set. A font is
    only registered by a path the package installs.
  - **A certificate is never added.** `SystemCertificates` stays refused, and the installer names the ones the app
    seemed to rely on for you to decide about.
  - **What the PC already has as something else's stays.** That goes for a protocol, a file type, what opening one does,
    a class in either view or either hive, with or without a server of its own, an interface, a type library, an AppID,
    an event source in any log, a font, a driver, a source, a program name, what "Open with" goes by, a Default apps entry,
    an app's row in Settings, an autostart value, and a firewall rule that says something else. `--force` does not
    change it. The only thing a
    package can then ask for is exactly what is there. So a server copied from this PC does not let in what comes
    beside it, and for these, a value only added to something of someone else's is refused like one that replaces it.
    Any app can add a verb of its own to a type that is another's, as "Git Bash Here" does, if it runs the package's own
    file, and `http` and the like can gain one too, while what opens them now stays.
  - **Being in the install location says nothing by itself.** A package suggests that place, so a package that said its
    home was another app's folder could otherwise call everything of that app's its own. What is on this PC only counts
    as an earlier install there if a record in the admin-only folder says one was, of any app. That goes for services,
    drivers, and tasks too, and a service left out takes what would have gone under its key along.
  - **Replacing a file is limited, and is the exception to all of that.** In the install location, `--force` on a command
    line replaces a file of the same name, and an uninstall puts it back. If that file is a program another app's service
    runs, the next start of that service runs the package's, which no rule above sees. So only use `--force` for a folder
    you know is this app's. The installer's window will not use a folder with files in it unless a record says an install
    was there. Outside the install location, without such a record, no program goes in a folder of a data location that
    was already there, new or replacing, with `--force` or not, and that goes for a license store's files too. That is by
    kind of file, with what a program is told to load by, such as `.exe.config`, among them. A data file there is replaced,
    by the window as well, since restoring a person's data is what that is for, and it comes back on uninstall. A
    setting can decide what an app runs, so a package could use that to change what another app does, and that is not on
    the list.
- **The rules do not make a package safe.** A package may still run its own files, as a service
  with full rights if it asks to. That is what installing an app is. Before anything happens, the
  installer's window lists the services, drivers, scheduled tasks, autostart entries, shortcuts,
  `Path` changes, shell extensions, menu commands, link protocols, file types, COM classes and
  permissions, firewall rules, event log sources, fonts, ODBC drivers and sources, compatibility settings,
  and Default apps entries it will set up, what it will leave out, and whether it is signed and by whom. Read it.
  Each of them is journaled first, so a rollback and an uninstall take it back out.
- **While it runs,** its staging folder is writable only by administrators, and a drive that cannot
  keep that, such as FAT32 or exFAT, is not used for it. That folder and the install location cannot be
  renamed and replaced, and it will not write through a link at or below the install location. From
  the moment it opens its own file, and a data file if it has one, those cannot be changed or swapped.
- **What a package says it needs is not a protection.** An app that cannot run on this PC is refused before
  anything changes, by what the package records of its programs and drivers. That is there to save a person a
  failed install. A package can say what it likes, and the rules above are what stand between it and the PC.

### An installer in two files

A package too big for one executable is a data file, beside a small loader that is the stub and a footer.

- **A signature on the loader covers the data file.** The footer holds the data file's SHA-256 and length, and sits
  in what a signature covers. So changing the footer breaks the signature, and changing the data file leaves the
  signature valid, with a file that no longer matches what it vouches for, which is refused. The elevated tests
  check both ways with a throwaway certificate: one byte of the data file changed, a whole other package swapped in,
  and one byte of the footer changed.
- **A loader with no signature proves as much as any other unsigned installer,** which is nothing. Whoever can change
  the data file can change the footer too. The hash then only catches damage.
- **Nothing is read from a data file before it is checked.** It is looked for in one place, beside the loader, under
  the loader's name with `.wamdata`, and whatever has that name is what is opened, not what a link there leads to. It
  is held open with no write or delete sharing, hashed through that one handle, and only then parsed, through that
  handle. So once it has been checked, nothing can change it for as long as the installer runs. A file that is missing,
  a link, or of the wrong length or hash stops the installer, and it says what to do.
- **A bare stub still cannot come out signed.** What an install leaves to uninstall with is cut from the loader as
  from any installer, and is byte for byte the stub, with no signature.

## What a person chooses before a build

The app runs elevated and shows what it would package, and a person chooses. So that cannot be a way around any of this:

- **A choice only takes away.** There is a box for what a scan found, and none for what it left out. So what looks like a
  credential cannot be ticked, whatever it is in, and a license only comes with the License option. That also means a
  license in a folder left out still goes when that option is on, and the screen says so. A choice goes by what each thing
  is called, and one that names nothing does nothing.
- **Adding goes through the scan.** A near match is one more root for the same scan, so everything above and below holds for it.
  What is ticked is only ever an id, compared with what the scan would offer now, so it cannot name a folder of its own.
  [What is never offered](docs/discovery.md#near-matches): what is shared, another app's, under a link, in another account's
  folders, or holds any of those, however it is written: a short name such as `PROGRA~3` is refused, since these rules go
  by names. A hidden entry's folder is asked the same, one that calls itself another app or only its publisher is not offered,
  what goes by its name only comes from under its own publisher's folder or key, a key of another app's or a file another app's
  install record also lists does not come with it, and one in an account's own hive says nothing about a machine-wide app, since that hive
  takes no rights to write. What is left is a person ticking something
  that is close to the app and not its own, which an install record could not have said either. It says why it is offered.
- **What is left with nothing to run is not set up.** What a choice leaves out goes through the review an install goes by, and
  what that now refuses goes too. A service, task, or shortcut told by a path to use something left out goes with it,
  including where it is told to start, a shortcut whose icon only is there loses the icon, and
  a package says what was left out, for whoever installs it. A plain setting of the app's that names such a folder stays. What a
  program does by itself, such as loading a file it knows the name of, cannot be told.
- **A build goes by the scan on the screen, and only that.** One of another app, or of what the window no longer says, is not
  used. A package holds what is read when it is built, with the same checks before and after each file. A file or license file
  that has come to hold a private key since the scan stays out, and the installer lists it with the rest. A `.key` or `.pem` that
  cannot be read to tell, because it is denied or too big to read, counts as one, since a package is read with more rights than
  that look has.

## A drive from another PC

The app runs elevated and reads a drive it has no reason to trust, which may be from a machine someone else
controlled. So:

- **What a drive says is only ever a path on that drive.** One thing maps a recorded path to where it is now.
  Only a full path on the source's Windows drive has an answer, in ordinary names. A rooted or relative path
  would depend on where this process is, `..` could climb out of a volume mounted on a folder, and a share would
  hand its server this account's password hash. All of those are left unread, and the build lists them.
- **No link on the drive is followed.** A junction or symbolic link on another PC's disk is resolved against this
  PC when opened, whatever it looked like there, and looking through one to a share is enough to connect to it. Each
  path is checked from the drive's root down when it is found and again just before it is read, and that check comes
  before anything else looks at it. The Windows folder, the hives, and the profiles are checked too.
- **What a folder's listing says is not what opening it does.** Windows drops a dot or a space from the end of a name,
  so a real folder `x.` opens the link `x` beside it, and a link `x ` reads as the real `x`. So each thing found by
  listing a folder is asked about by name before it is opened, and a walk goes in only after that. A name that ends
  that way, a name that is not text, or one this Windows would open as a device, is not read. A path a value records is
  first read as Windows would read it, so that it is the real name that is asked about.
- **There are limits on what a drive can make this do.** A hive is copied by reading it, up to 4 GiB, and 32 GiB for
  all of a run's. There are limits on a program, an install log, an AppX manifest, a task, how many tasks an app keeps,
  how many accounts are listed, and how much of the registry is captured. How much of an app's own files can be read
  into a package is checked against the room there is, by what the listing says of each, and a file can only hold as
  much as was read.
- **The drive is not written to.** Loading a registry hive can replay its logs into it, so hives are copied to an
  administrators-only folder under `%ProgramData%\WinAppMigrator\hives`, and the copies are loaded. They can hold
  secrets, and are removed afterward. A run that dies leaves them, and hives loaded, so the app sweeps those when
  it starts. It goes by a lease each run holds, so it leaves another run's alone, and it only touches a key named
  the way this app names one.
- **Registry links depend on Windows.** A key in a hive can be a link to somewhere else in the registry, and this does
  not open them specially. On the runner, Windows refused `reg query` through one out of a hive loaded from `SOFTWARE` and
  from `NTUSER.DAT`, and what a link in those and in `SYSTEM` led to did not reach a package, which is tested. If Windows
  ever allowed that, nothing here would stop it.
- **Loading a hive runs Windows' own parser on it.** A drive made to attack that parser needs no bug here, and this
  project cannot do anything about one. Read a drive from someone you do not trust on a machine you can lose.

## Another account's data

An installer holds one account's data. A scan loads only the hive of that account, and leaves out what is in
another's folder, however a value came to name it. A folder name is what is compared, by where it really is, so a short
name, or a link that points at another account's folder, is left out too.

An account can write some of what says where an app is with no rights at all: an uninstall entry and an App Paths key in
its own registry, and its own shortcuts. So what these say does not count for much:

- Only the install location in an app's machine-wide record may be in another account's folder, and only as it is
  written, with no link on the way. Any account may be able to make a folder on `C:\`.
- An app installed for one account has to be in that account's profile or be owned by that account, its folder as well as
  its program. That covers a redirected folder outside a profile, which no profile check would see, and on a drive, what is
  left of an account in `Windows.old` or the recycle bin. It goes by the folder's owner, which a drive keeps, so an app that
  account's installer put somewhere as an administrator, or on a volume with no permissions, is refused too, and says so.
- A machine-wide app does not go by an account's shortcut or App Paths key to find its program, since either could name
  any folder, which would be taken whole.
- What is found by an app's name in an account's `AppData`, Documents, or Saved Games has the same rule: it has to be in
  that account's profile or owned by it. A link there, or a value for where Documents are, could otherwise reach anything
  an administrator can read.
- An app installed for one account gets nothing machine-wide by its name, in files or the registry, since its name is whatever its entry says
  and `ProgramData\<name>` may be anyone's. Its install log can only name that account's registry and files in its own
  folders.
- A machine-wide app's install log is only believed if only administrators can change it. It names files and registry
  values to package, it sits in the app's own folder, and editing a file keeps its owner, so its permissions are read too.
- A folder in `ProgramData` or Public Documents that is a link is not followed, because anyone can make one there. A
  license store there is treated the same way.
- A scheduled task that runs as anyone but the account being read or one of Windows' own is left out, with a warning.
  Task principals are usually SIDs. A name that cannot be told from a person's counts as one. One that says it runs as
  Windows, on this PC, has to be a file only administrators can change, since tasks are read from files and any account
  can make a file in that folder, which the Task Scheduler would never run but a package would register.

**On this PC, a share is not looked at** either, since a value in any account's registry could name one, and looking
would connect as the administrator running this, including when the app starts. Anything that is written as not on a disk
of this PC, such as `\\localhost\C$`, `\\?\GLOBALROOT`, or `\??\UNC`, is left out, and the build lists it. So a Documents
folder redirected to a share is not included, and an app on a share cannot be built from. A drive letter mapped to a share
counts as a disk, if the elevated session has one. Names Windows would read as another, as on a drive, are refused here too,
since any account can make a real folder `x.` beside a link `x`.

**What is left is a race.** A build asks again before and after reading each file whether it leads into another account's
folder, or goes through a link that was not there when it was scanned, and a file that fails that is taken back out of the
package. Whoever can write in a scanned folder could make
part of it a link and undo that around the moment a file is opened, and they would have to know a name that is there.
Closing that needs the opened file's own path from Windows, which takes `unsafe` code this project does not otherwise
have in the core. A link that was there when scanned is asked about too, in case it leads elsewhere by then. A stale
machine-wide record that names a folder any account can make, on `C:\`, can lead to wherever a link there does, as long
as that is not in another account's profile. A symbolic link to a share, which needs Developer Mode or that right, is also
still followed on this PC, by anything looking at a folder an account's value or listing names. A hard link in a scanned
folder looks like any other file, though making one takes being able to open the file. Folder and task names of other
accounts can appear in a package's warnings. The folder picker is Windows' own and runs before any of this, so what the
shell itself does on browsing a hostile drive is not something this controls. An installer is written where whoever runs
the app says, by default in that account's folders, not those of whoever is signed in, whose data it may hold.

An elevated process can be run by another administrator than the person at the PC, whose `%APPDATA%` and
`HKEY_CURRENT_USER` are not that person's. So who a scan or an install is for comes from the owner of this session's
real `explorer.exe`, and what is written goes to their folders and hive. With more than one owner, or none and a
system account running this, it could not be told, and per-user data and settings are left out and it says so. With
no desktop in the session at all, as under a scheduled task, it is the account running this.

The folders are the ones the registry records for that account, which it can change with no rights at all, and an
elevated process writes wherever they point. So each has to be inside the profile, which only administrators can
change. Documents and Saved Games also cannot be under `AppData`, where a file can run by itself. What cannot go to
one goes under the install location, and the installer says so before it starts.

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
  to empty anything. Use the place it leads to instead. A volume mounted on a folder is allowed if Windows
  gives that folder as where it is, though if others can write beside that folder, they can change that too.
  If Windows gives a drive letter instead, it is refused, so use that. Neither has been run on a real
  mounted volume. For each
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

Treat an installer as being as sensitive as the PC it came from.

**What is left out.** A registry value, a service's setting, an environment variable, or a command-line
option named for a credential (`Password`, `ApiKey`, `Token`, `ClientSecret`, and so on), or holding a connection
string or URL with one in it, or a blob only DPAPI can read, is not captured at all, whatever else was asked
for. Nor are files named for what they hold (`.env`, `id_rsa`, `credentials.json`, `passwords.txt`, a browser's `Login Data`
and what SQLite keeps beside it, `*.pfx`, `*.kdbx`, `*.ovpn`, and so on), a `.pem` or `.key` that holds a private key or cannot be
read to tell, a `.key` that is a key in binary, or a `.crt` or `.cer` that holds a private key. They are named when you build and in the
installer, and never with what they hold. Since there is nothing of them in a package, there is nothing to
overwrite what the other PC has, and you may have to enter them again.

With the License option off, the same goes for license files, values, variables, and options, by the same rules
that find them when it is on. With it on, licenses come along and credentials still do not.

With the User data option off, what an account keeps for an app in its part of the registry stays behind too: recent files
and folders, saved sessions, the hosts it has been to. All of it goes but what Windows itself looks in for an app, such as
file types, COM classes, App Paths, Run values, its entry in Settings, and what that entry in Default apps points at, and a
plainly named value in the app's own key, no more than two below `Software`, whose whole data is a path in the app's own
folder. This goes by where a value is, so nothing that finds one can get round it, and it says how many went from
which app's key and nothing below that, since a host can be what a key is called. What stays can still say something: an
ODBC source can name a server and a user. With the License option on as well, a license value in an account's key goes where it was,
so its key's name goes with it.

**What this cannot catch.** This goes by names and a few shapes of text, and a heuristic cannot find every secret:

- It does not look inside an app's own files. A password in a settings file, a database, or a format of its own is
  copied like the rest of the file, and that goes for everything the User data option adds.
- A value whose name does not say what it holds, such as `Data` or `Blob1`, is captured, unless what it holds has one
  of those shapes.
- Rules that catch more would break more apps, so `TokenizerPath` and `PasswordPolicyVersion` are kept, and a file
  called `tokens.txt` is too. A `.key` in binary is known by its shape alone, so one of a kind of its own is not, and a `.crt`
  is only read up to 1 MiB, so a private key in one bigger than that is not found.
- An installer says which PC it came from, and what it holds can say more. Some of what a setup keeps of who ran it stays
  behind: an Inno Setup app's uninstall log, `unins000.dat`, which names the PC and the account and can hold paths in
  that account's folders, the account, name, and organization in its entry in Settings, and where a Windows Installer setup
  was run from. A serial typed into a setup is a license like any other, so it stays behind with the License option off, and
  with it on, one for a machine-wide app is only reported. An app installed for one account has that account's folders in
  the paths a package records, and a shortcut or a service of any app can name the account if that is where it points.
- What is encrypted some other way than DPAPI is copied as it is, and is not decrypted to look.

So an installer can contain secrets, and **it has to be handled accordingly**: do not post one or send it to anyone you
would not hand the PC to. Reading the list of what was left out tells you what was found, and says nothing of what
was not.

The package also records the name of the computer the app was on, and the name of that account's profile folder.

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
