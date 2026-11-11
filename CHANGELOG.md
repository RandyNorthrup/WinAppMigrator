# Changelog

All notable project changes are tracked here.

## 0.6.1 — 2026-11-11

Moving an app off another PC over the network is safer. WinAppMigrator Client, on the old PC, now stops anyone on the
network from guessing its pairing code, and leaves no way for another account on that PC to misuse it. The app no longer
takes what another PC, a drive, or a disk image says about its own files on trust, so a broken or hostile one cannot fill
this PC's memory or disk, and it refuses a disk image that would reach out to another disk elsewhere. Titles, labels, and
buttons are in title case, and the guide now has a website.

### A website

- **The guide is on a website search engines can read:** https://randynorthrup.github.io/WinAppMigrator. GitHub keeps a
  wiki out of search results until its repository has 500 stars, so `.github/workflows/pages.yml` publishes the wiki's
  pages there, rendered by GitHub's own Markdown renderer, with a landing page from `site/`. It is rebuilt when the site's
  files change, when the wiki is edited, after a release, and once a day, and the build fails on any link or `#anchor` between pages
  that leads nowhere. Bing and the search engines that share its index are told which pages changed, through IndexNow.

### Title case

- **Titles, labels, and buttons are in title case** in the app, the installer, the uninstaller, and the network steps, such as
  **Create Installer**, **User Data**, **Look Again**, and **Preview Changes**. Sentences, hints, and notices are not. The
  kinds on the installer's list (**Scheduled Task**, **Firewall Rule**) and how a runtime comes (**Install Manually**) follow,
  and the client says **Pairing Code**. The website's titles, its pages' headings, and the README's headings do too, and the
  pictures are of the screens as they now are.

### Security

- **Network PC: the client lets nothing pair after 20 wrong codes.** Before, it made a new code after each five, for as long as
  it ran, so something on the network that kept guessing would get in in time. Each wrong code now holds up the next by 2
  seconds, and pairing has 15 seconds, so a PC that connects and says nothing cannot keep another from pairing.
- **Network PC: the client keeps its record in Program Files.** It was in a folder in ProgramData, which any account can make
  first, and own, and then have a link in it lead the client's writes, and its removal of the record, to a file of its choosing.
  A 0.6.0 record left there is not looked at.
- **Network PC: what the client starts runs from `System32`,** not the folder the client was run from, which could be a USB
  drive anyone can write to, and is told not to look for a program in the folder it starts in. `ping` is named by its full path.
- **Network PC: the client never gives out `SAM` or `SECURITY`,** the hives that hold the PC's passwords and secrets, as no build
  needs them, and lists a folder in pages that fit in a message however long its names are.
- **The website loads only the one file of Mermaid's from jsDelivr,** rather than anything there. `scripts/sign.ps1` installs
  the one version of the signing module the release uses.

- **A release builds in a job that cannot sign or push, and signs in one that runs nothing from the repository or crates.io.**
  Before, the tests, build scripts and `cargo-about` ran where the app was signed, and could have signed anything. `cargo-about`
  is now pinned, and the signing action no longer restores its tools from a cache a build could write.
- **Network PC: a client that always says a folder has more is stopped.** A folder's listing ends at 250,000 entries, and a
  part of one that is empty but says there is more is an error.
- **Network PC: a file is never read past the size the client first gave it.** What it first says of a path is kept until it
  is disconnected, so it cannot say a file is small and then send without end, past the room a build checked for. Shortcuts,
  tasks, programs and key files are read with a limit as well.
- **Network PC: more of a file than was asked for is an error.** Before, a client that sent more stopped the app.
- **Network PC: a name in a listing that would lead out of its folder is dropped,** such as `..`, `C:\Windows` or
  `\\server\share`, which this PC could have sent its sign-in to. What is asked about is checked the same way.
- **Network PC: a request has two minutes in all, and Disconnect ends one.** Before, each wait for a byte had two minutes, so a
  client sending a byte at a time could keep a request going for ever.
- **Hive copies count what was copied.** The 32 GiB a run may copy of a drive's hives went by the sizes the drive gave. It now
  goes by what was copied, a copy stops at what is left of it, and Cancel stops one part-way.
- **A differencing `.vhd` or `.vhdx` is refused before it is attached.** It names the disk it was made from, which Windows
  would open too, wherever it is, a share included.
- **A disk image's record is kept until it is detached,** so one a run attached just before it died is found again by the next
  start, and one with no partitions is detached too.
- **Saving the client never writes through a link.** A folder that is a link is refused, and the client and its note are
  written under a new random name and then renamed, with a link of their name taken away first, not what it leads to.

## 0.6.0 — 2026-11-09

### Network PC: another PC, over your network

- **Move an app off a PC that is still running, Windows 7 or later, without unplugging anything.** Choose **Network PC…**,
  save **WinAppMigrator Client** to a USB drive or a folder, run it on the old PC, and type the code it shows. That PC's apps
  are then listed, and an installer is built from one here, as from this PC.
- **The client only lets this PC read the old one.** It takes a snapshot of its Windows drive, so nothing is locked or
  changing, and the build reads what it needs from it: an app's files and settings, not the whole drive. It changes nothing
  else there, and removes the snapshot and its firewall rules when you stop it or close its window.
- **Encrypted, and paired by the code.** The code never crosses the network, a wrong one gets a few tries before the client
  makes another, and everything after it is encrypted and checked.
- **One small client for Windows 7, 8, 10 and 11, 32-bit or 64-bit.** The app carries it.
- **Signed, as the app is.** The UAC prompt on the old PC names Randy Northrup as its publisher. A DLL put beside the client
  is not loaded into it: it runs from a copy of itself in a folder only an administrator can change, and gives the Windows
  tools it starts an environment of Windows' own.

### Windows 7, and disk images

- **Apps from Windows 7, Vista, 8 and 8.1 move to Windows 10 and 11.** Pick the old PC's drive, or an image of it, and the
  app says which Windows it has. What was different there is taken care of:
  - **Settings an older app kept in Windows' `VirtualStore`** come along with User data. Windows kept them in your profile
    when the app thought it wrote to its own folder or machine keys, so that is where the settings of most apps made for
    Windows XP are. They now go in the app's own folder and keys, where it finds them on Windows 11.
  - **An app from 32-bit Windows** goes to `Program Files (x86)`, and its registry keys to where 64-bit Windows keeps a
    32-bit app's. Before, it would have been installed where it could not find its own settings.
  - **The DirectX 9 runtime** is installed with an app that needs it (`d3dx9_43.dll` and the rest), as no Windows comes
    with it. Installing it takes the internet.
  - **A DLL the app found in Windows 7's own folders that Windows 11 does not have** is named, rather than assumed to be there.
  - The drive's Windows version was read as 10.0 for anything older, and is now read right.
- **Disk image…** reads a `.vhd` or `.vhdx` of another PC, such as one Sysinternals Disk2vhd makes of a PC that still runs, or
  Windows 7's own system image backup. The app attaches it read-only with no drive letter, and detaches it.
- A shortcut that names its target only with a variable, as Windows 7's own do, is read. An MSI's empty install location is no
  longer listed as a path that could not be followed.

## 0.5.0 — 2026-11-03

This is a large release: almost all of what the app and its installers do has changed.

### Highlights

- **You see what goes in an installer before you build it,** how big it can be, and whether it fits, and you can leave things
  out and add what looks related.
- **Real apps are moved between PCs on every push,** and Windows is asked to use what was restored. That found bugs no fake
  app had, and they are fixed.
- **An install can be undone.** Any failure or cancel puts back what it did, and says if anything could not be, and it has its own
  entry in Settings that removes it, with anything it replaced coming back.
- **An installer says what it will do before it does anything,** and the list and the install go by one review: services, tasks, autostart
  entries, file types, COM classes, firewall rules, and the rest, by rules applied when it installs and not only when it was built.
- **With the User data option off, far less says who built it.** Your PC's name, your account, what apps remember in the
  registry, and the names of files that were not even going in, were all in every package. See
  [what to do about installers you already have](https://github.com/RandyNorthrup/WinAppMigrator/blob/main/SECURITY.md#which-versions-get-fixes).
- **Builds are much faster,** nearly ten times on the largest app measured, and past the 4 GiB Windows can start, an installer is
  now a small `.exe` and a data file.
- **A connected drive is not trusted,** and building from one is tested on a VHD made hostile on purpose.
- **The app is signed,** and every release says where it was built. The installer stub stays unsigned on purpose.

### What changes for scripts

Run from a script, an installer does more than it did, and refuses more:

- **`--apply` no longer needs `--target-root`.** It uses where the app was, if the checks the window makes allow that.
- **Without `--force`, a folder with files in it and no install on record stops the install,** whether or not any file would be
  replaced, and so does an existing file. With it, services, tasks, and what ties an app into Windows still stay another
  app's.
- **More fails an install.** A step the package asked for that fails now means the install failed and is put back, including
  registry values, tasks, and licenses, which used to be listed and left. The only exceptions are a service's description, a
  shortcut anywhere but a Startup folder, and a service that will not start, which is a warning. What the rules refuse is
  left out and fails nothing, unless what the package sets up relies on it, and then nothing is installed.
- **More stops an install before it starts:** an app that cannot run on this PC, not enough room on any drive it writes to, a
  network share or anything under a link as the location, a part of a location Windows would read differently, a whole drive
  or a folder of Windows', and nowhere private to unpack.
- **A service set to start automatically is started** once what was installed has been read back, if it was really set up. One that
  starts only when asked stays stopped. Rollback and uninstall stop what they remove, and an install cut off after stopping a
  service to replace what it runs starts it again the next time an installer runs.
- **An install holds itself to what was agreed to.** The window's list is for the location in the box, and if what an install would do
  has changed by the time it is clicked, or by the time it has unpacked, nothing is installed and the new list is shown.
- **What the rewriting of a value costs is refused up front.** A package holding too much to check is refused whole, where a limit
  could run out part way and leave a value written as it was.
- **What it prints has grown:** how much room it needs on each drive, what it sets up, each warning and error and what it is
  about, what it refused, the install's id, and where the log is. An installer is a windowed program, so pipe it, for example
  to `Out-String`, to read that and to have `$LASTEXITCODE` set.
- **An uninstall has its own command line:** `--uninstall <ID>`, with `--quiet` for no window, `--remove-user-data`, and with
  `--quiet`, `--log <FILE>`. Its entry in Settings has it as `QuietUninstallString`.
- **`--log <FILE>`** adds a file of your own to the log in `%ProgramData%\WinAppMigrator\logs`, and must not exist yet.
- **An installer past 4 GiB is two files,** `.exe` and `.wamdata`, which have to stay together with those names. Copy both.

### What goes in an installer

- **You see it first.** Selecting an app looks at it in the background, and the screen says how big the installer can be and
  whether it fits, what it holds, in plain groups and in the words the installer's own list uses, and what is left out and why,
  with credentials, licenses, and certificates by name. Picking another app stops the last look, and the window stays usable
  throughout. A build goes by that look, and each file is still hashed from what is read of it then, so one that changed is
  right and one that has gone is said to have. Changing the License or User data option, whose data, or where apps are read
  from looks again, and keeps what you chose.
- **You can leave things out.** Each folder of the app, each service, task, startup item, and shortcut, and each kind of
  what ties it into Windows, has a box to untick, and the size and the room it needs follow at once. What holds the main
  program, or what it cannot start without, stays. What is left with nothing to run goes too, and the screen says so: a service
  that runs a file in a folder you left out is not set up on the other PC, and neither is a task told to use one, or a service
  that needs one you left out. An installer says what its builder left out and added. A license goes by the License option
  alone, so one in a folder you left out still goes in, and the screen names it.
- **Near matches are offered, not lost.** What exact names leave out and looks related is listed, unticked, each with why:
  a folder whose name starts with or holds the product's, or starts with its publisher's, and an entry hidden from the list
  of apps by the same publisher or beside the app, such as NVIDIA App's `NVIDIA Container`. Ticking one looks at the app
  again with that as one more root, by the same rules as any other. A shared folder, another app's, a link, anything that
  holds one of those, a path by a short name, or a hidden entry that calls itself another app is never offered. Some offers
  are close and still not the app's, which is why each starts unticked and says why. See
  [the rules](https://github.com/RandyNorthrup/WinAppMigrator/blob/main/docs/discovery.md#near-matches).
- **Footprints match the product, not everything that sounds like it.** Folders and registry keys match the product's whole
  name, or `Vendor\Product`, never a substring. A publisher or the install folder's parent can only qualify a parent folder.
  On one test PC, VS Code went from 175,182 files to about 68,900: Visual Studio and the rest of `AppData\Local\Programs` are
  gone. Firefox no longer takes Thunderbird, NVIDIA App no longer takes `Razer\Installer`, and `Software\Logitech` is no longer
  part of Git.
- **A folder is not the app's for having its uninstaller in it.** A setup run from `D:\Downloads` brought all of `D:\Downloads`,
  with both options off. Now it brings that file, unless the folder says by its name that it is the app's. App Paths has to say it
  is this program before anything it lists counts. Every folder a record names is asked much the same as a near match is, unless it
  is where the app itself is. One that fails is said to be left out, and what the record names in it still goes, file by file.
- **Install records are read properly.** Roots from MSI, Inno Setup, and NSIS records are the deepest folder that holds the
  recorded files, and never a shared one, and they also name registry keys, which is how `Software\GitForWindows` is found.
  Inno Setup logs are read by their blocks and length prefixes: Git for Windows went from no paths resolved to 9,220 files, and
  old ANSI logs work too. A Burn bundle's files come from its hidden MSI packages. The MSI database is read once per scan.
- **Nothing inside an install folder is skipped,** however it is named. VS Code was losing 1,173 files under `node-compile-cache`.
- **An app's empty folders come along,** up to 10,000. A package used to go by files alone, so windows_exporter's `textfile_inputs`
  was not on the other PC. An uninstall takes one away if it is still empty.
- **What a walk leaves out is said.** A link in an app's folder is named in a warning, and so is what is more than 64 levels down.
- **The main executable is the application.** Uninstallers, updaters, Windows hosts, and installers' cached copies are
  rejected, and App Paths and Start Menu shortcuts are consulted. Git was `unins000.exe`, and is now `git-bash.exe`. NVIDIA App
  was `rundll32.exe`, whose name alone matched 1,258 registry entries. An app with a 32-bit setup and 64-bit programs is listed
  by its program, not as x86. An entry with no program at all, such as a driver, fails and says why.
- **The list shows real applications.** Entries marked `SystemComponent=1` are left out, as in Windows' own installed-apps list.
  They are parts of other products: on one test PC they were 481 of the 641 rows.
- **What ties an app into Windows comes along.** COM type libraries, interfaces, AppIDs, and ProgIDs, per-user 32-bit classes,
  URL protocols, `OpenWithProgids`, context menu handlers, icon overlays, the `Shell Extensions\Approved` list, Explorer
  namespaces, `RegisteredApplications` and `Clients` entries, firewall rules, fonts, ODBC drivers and sources, event log sources,
  compatibility settings, and 32-bit Run keys. Each is the app's by what it points at, with an exact name only for a data source,
  so another app's class or file type does not come along. A COM class's DLL path with a space in it is read whole.
- **Certificates are reported, and never installed.** A package cannot add to a trust store. The ones an app seems to rely on are
  named for you to decide about.
- **Runtimes are read from all of an app's programs, as what they are.** A Visual C++ DLL is the version its name says, and
  Windows' own `msvcrt.dll` no longer counts as one. Every version from 2005 has its own installer, and every .NET version is no
  longer .NET 8: it comes from `runtimeconfig.json` and the assembly's own metadata, which also tell .NET Framework from modern .NET.
  .NET 6 to 10 are now embedded when you build, like Visual C++. A runtime with no installer here is named in a warning, and nothing
  is installed in its place. A Visual C++ runtime older than the one a package brings gets that one. A .NET program packed into one file says so,
  and a program that cannot be parsed is that file's warning and not a failed scan.
- **Scheduled tasks are read from their files,** on a live scan too. They went through PowerShell one at a time, and one character
  that broke the JSON lost all of them. A task with two programs is one task, and a task whose second program is the app's is found.
- **Fourteen warnings about old folder names are gone.** Every elevated scan warned about `Application Data`, `My Music`, and the
  junctions like them.
- **What a program says of itself is held to what can be believed before it is parsed.** Its headers, sections, and resources have to
  be where any linker puts them, on 66,677 real programs that refused none, and 84,000 damaged ones took nothing down. A folder an
  uninstaller is in is the app's if it holds the program, so `D:\Games\MyGame\Content` comes along, and is not if it is named only for the
  publisher, or if Windows Installer says another app has a file there. A setup in someone's Downloads, on any drive, stays.

### What a package says about who built it

Installers get copied and shared, and they said far more than anyone would expect. With the User data option off:

- **What an account keeps for an app in the registry stays behind.** An app's key under `HKEY_CURRENT_USER` is where it remembers
  things: 7-Zip the folders and archives it last opened, PuTTY the hosts it has been to and who as. None of that is called what
  it is, so the rules for credentials did not see it. All that goes of an account's is what Windows itself looks in for an app,
  and a value that says where the app's own folder is, and the screen says how much was left out of which app's key. So the app
  should start on the other PC much as it would for someone new, except for what it keeps among its own files and machine-wide settings.
- **A package says nothing of data that is not going in it.** Every file of an app's data was listed in the manifest and the
  report, by name, with the account's folders in its path.
- **The PC's name and the account's are gone from what a package says of itself,** including what a scheduled task, a warning, a
  service's account, or a certificate from an account's own store said. An app can keep the name in its own files and machine-wide
  settings, which go as they are, and an app installed for one account has that account's folders in the paths a package records.
- **A shortcut in an account's own folders goes only if it just opens the app.** One with arguments is theirs, and goes with User
  data. An app of one account's keeps its own, and where one goes is said with no account in it.
- **Environment variables say less.** A list of folders such as `Path` keeps only the app's entries, and an account's variable goes
  only if all it says is where the app is. A captured `Path` used to hold all of what else was on the PC.
- **What a setup keeps of who ran it stays behind:** an Inno Setup app's `unins000.dat`, and the account, name, and organization in
  its entry in Settings, and where a Windows Installer setup was run from. So does what said Windows Installer has the app, which on
  another PC it does not.
- **A build takes a last look.** It goes through the manifest and what it wrote, in each form a value can be held in, for the
  account, profile, PC, and SID. It names on the done screen where it found any, and never what. That changes nothing and stops
  nothing, since it can be what makes the app work.

Either way:

- **A scheduled task that ran as a person runs as whoever installs it.** On another PC that person is nobody, so who wrote it and who
  a trigger waited for go too, a password nobody has comes to running while they are signed in, and one run for a group of theirs
  runs as whoever installs it as well, and not as everyone on the PC. With the option off, what else it runs from someone's folders
  goes, and one told to work on something there goes whole.

- **How an app was removed where it was is not said.** `UninstallString` and `QuietUninstallString` can be a setup in someone's
  Downloads, and nothing on another PC reads them.
- **The last look at a package is first on the done screen,** and "Installer ready" no longer reads as all clear above it. It goes
  through each file once and can be cancelled. See [what it looks for](SECURITY.md#an-installer-can-contain-secrets).

### Credentials and licenses

- **Credentials are left out of a package, and named.** Values, service settings, variables, command-line options, and files that
  look like passwords, tokens, or private keys are not captured at all, even with the License option on, so a package has nothing to
  overwrite what the other PC has. That includes `.ovpn` profiles, `passwords.txt`, what SQLite keeps beside a store of passwords,
  and a private key in a `.pem`, `.key`, `.crt`, or `.cer`, including a `.key` in binary and a `.pem` or `.key` in UTF-16. A `.crt`
  with only a certificate in it still goes.
  This goes by names and shapes of text, so it cannot catch everything, and an installer should still be handled as being as
  sensitive as the PC it came from.
- **The License option means what it says.** With it off, license files, values, service settings, options, and variables are not
  captured at all, and only their names are noted, and that includes a serial typed into a setup. A framework's license store and a
  `*_LICENSE_FILE` variable have to be tied to the app.

### Installing

- **An installer says what it will do.** Before installing, its window lists the services, drivers, scheduled tasks, autostart
  entries, shortcuts, `Path` changes, and what ties the app into Windows, what the rules left out, and whether the installer is
  signed and by whom, or "Not signed: publisher unknown". A preview's list leads with those. The list and the install go through one
  review with the same options, and it changes as the boxes do. What depends on what is on the PC, such as a service of that name
  already being another app's, can only be seen as it installs, and is then left alone and said.
- **An installer checks what its package asks for.** A package can come from a hostile or odd source drive and nothing
  authenticates it, so the rules are applied when restoring, not just when scanning. Registry keys no app install should write are
  refused, including Image File Execution Options, Winlogon, `AppInit_DLLs`, LSA, Session Manager, KnownDLLs, certificate stores,
  the file types that start programs, Group Policy's startup scripts, compatibility telemetry's command, and PowerShell's execution
  policy. Under `SYSTEM`, only the `Parameters` and `Instances` keys of a service the package restores are allowed. Services,
  shortcuts, tasks, and Run values must run a file the package installs. Environment variables Windows and other programs depend on
  are refused.
- **What else a package writes is on the list.** A list of refusals can never be complete, so values outside the app's own keys and
  what the list has words for show as one row, "N settings outside the app's own keys", marked for a second look when they are in
  Microsoft's keys. It can be left out in the app.
- **Being told a folder is the app's does not make it so.** A package suggests its install location, so what names that folder only
  counts as an earlier install if a record on this PC says one was there. The window does not use a folder with files in it without
  such a record, and looks again when Install is clicked. Outside the install location no program goes in a folder that was already
  there without one. A service, task, or COM class already on the PC that belongs to something else is left alone, `--force` or not,
  and so is anything else there as something else's, including `http`, a class in either view or hive, and an event source in any log.
- **What ties an app into Windows goes by the rules, in the list, and comes back out.** What it runs or loads has to be a file the package
  installs. Firewall rules go through `netsh`, for one program, never all. Fonts in `C:\Windows\Fonts` are copied to the install
  folder only if a machine-wide install record names them. A rollback and an uninstall take each of them back out.
- **An installer connects to nothing a package names before anyone has agreed to it.** Opening its window could have connected to a
  share a package named, which hands the far end the name and password hash of whoever is looking. One check of how a path is written
  now decides this everywhere, and it knows `//host/share`, `\\?\UNC\`, and `\??\UNC\`.
- **There has to be room before anything starts.** A build works out the most it could take and checks every drive it writes to. An
  install does the same for where it unpacks, the install location, and what it keeps of what it replaces. If there is not room it says
  how much is needed and how much is free, and nothing is changed. A drive that will not say is said not to have said.
- **An app that cannot run on this PC is refused before anything changes.** A package records the Windows and architecture it came
  from, and what each of its programs and drivers needs, from their own headers. x64 on 32-bit Windows, a driver for another
  architecture, and a program that says it needs a newer Windows are refused. x64 on Arm64, and a Windows older than the source that the
  program's headers allow, are in the list you agree to, since they may well work.
- **What an install set to start by itself is started.** A service set to start automatically was only set up, so an app that relies on
  one looked broken until the next restart. Now it starts each after what it needs, and one that will not start is a warning.
- **A package is unpacked where nobody else can have a hold on it.** It goes in the admin-only folder where installs keep their records
  when it can, anywhere else has to give nobody the right to go on changing what is made there, and if there is nowhere like that the
  install does not go ahead. A drive that keeps no permissions, such as FAT32 or exFAT, is not used for it. What to do comes from the
  package held open since it was checked, and each file is hashed as it is copied into place.
- **An installer checks its whole package first.** A SHA-256 of the package is checked before anything is parsed, so a truncated copy or
  an edited manifest is refused at once. Every copied file is hashed as it goes into the package, a file with no hash fails verification,
  and driver images and license store files are checked too. While the installer runs, its own file cannot be changed or replaced.
- **An install location is judged by where it really is.** `C:\.\Windows\x`, `\\?\C:\Windows\x`, a short name, and a link could all pass
  for somewhere else. A network share is refused, and so is a location under a link, since whoever can write beside the link can
  change where it leads. A volume mounted on a folder is allowed if Windows gives that folder as where it is. A Startup folder, or any
  folder that belongs to Windows or whose contents run by themselves, is refused as a location.
- **Writes cannot be carried somewhere else.** Each file is written with the way there held, beside its destination, and renamed into
  place, so a hard link there is replaced rather than written through, and making a write wait does not lead it anywhere else.
- **Services come back as they were.** A service's path is stored quoted, so that `C:\My Apps\Foo Bar\svc.exe` cannot be hijacked by
  `C:\My Apps\Foo.exe`. Arguments, the `Parameters` key, the display name, a delayed start, LocalService, and NetworkService are kept.
  A custom account cannot be restored without its password, so its service is set up as LocalService, with less access rather than
  more, and the installer says so. Verification compares the binary, arguments, and account exactly.
- **Drivers are checked before they are registered,** and found wherever they live, including under `Program Files` and by `\??\` paths.
- **Paths are rewritten exactly.** `C:\App\a.exe` no longer becomes `C:\App22\a.exe` for a target of `C:\App2`, and a rule for
  `C:\Program Files\Git` no longer touches `C:\Program Files\GitHub CLI`. A list of paths in a `REG_MULTI_SZ` value follows too.
- **Registry links are refused,** and so are names Windows opens as devices, such as `CON`. A file named like one with an extension, such
  as `aux.h`, stops an install before Windows 11 with a clear message. It reached the device, which threw its data away and reported a
  failed hash check. This has been run on Windows Server 2022, build 20348, and not on Windows 10 itself.
- **Documents and Saved Games go to the real folders,** wherever the registry says they are, including moved by OneDrive. A folder of
  either outside your profile is not written to: those files go under the install folder, and the installer says so before it starts.
  Files in PowerShell's profile folders are refused, as a Startup folder is, because PowerShell runs them.
- **Shortcuts only land in this PC's own Desktop and Start Menu folders,** so `C:\Users\alice\Desktop` is no longer created on a PC
  with no alice. Each file of a license store is held to the rule for where it lands, so it cannot put something in a Startup folder.
- **`Path` is merged, never replaced.** Only the app's own entries are added, and `PSModulePath` the same way.
- **The installer's window opens at once** however large the package is, and shows the hash check. It unpacks after you click, reading
  the package in place, shows progress for each step, can be cancelled, and offers Close and Open install folder when done.

### Undoing an install

- **An install keeps a journal,** in an admin-only folder of its own under `%ProgramData%\WinAppMigrator\installs`, recording what each
  change replaces before making it. Any failure, a cancel, and the check after installing put it back in reverse order and
  say what could not be, keeping the record to try again. An install killed or cut off by a power cut is found and undone by the next one. Without the record being
  safe to keep, an install does not start, and only one install or uninstall runs at a time. Runtimes it installed stay.
- **An app installed this way has an uninstaller.** Its entry in Settings runs a bare copy of the stub kept in that same folder, never in
  the install folder. It undoes only what the journal says that install did: a file or value that has changed since is kept and
  reported, only added `Path` entries come out, what you added stays, and so do user data and licenses unless you ask. A file that what it
  kept still uses stays with it. It stops a service before deleting it. It can be run again if it could not finish, and removes itself after it
  exits. If part of it is on a drive that is not connected, it says so, and its entry stays to finish from. The source PC's uninstall
  strings do not survive.
- **An uninstall never loses what an install replaced.** What was replaced comes back if the file over it is as it was installed, and
  otherwise goes beside it as `name.before-install`. If that cannot be done, the record stays, with the original in it.
- **An uninstaller says when its window cannot open,** with the command that does the same without a window.

### Connected drives and other accounts

- **Building from a connected drive works.** It compared the registry's `C:\` paths with `E:\` paths, so COM classes, file associations,
  Run keys, App Paths, and environment variables never matched. They are compared as the source PC called them now, per-user classes come
  from `UsrClass.dat`, and shortcuts are parsed from their `.lnk` files. The installer offers the app's location on its PC instead of
  the drive's letter. CI runs this on a VHD built on the runner and made to look like another PC's, not a real disk from one.
- **A connected drive is not trusted.** What it says can only be a path on it, and nothing on it can lead a scan to this PC's files or to
  the network. A rooted, relative, `..`, other-drive, or UNC path is not followed, and neither is any link on the drive, and what was not
  followed is listed. Everything found on a drive is asked about by name before it is looked at, and there are limits on what a drive can
  make this read and copy, including hives, logs, and how much of the registry is captured.
- **The app no longer writes to a connected drive.** Loading a hive where it is can replay its logs into it, so the hives are copied to an
  administrators-only folder and the copies are loaded. A hive that can only be read in backup mode is held to the same limits. A run
  that dies with hives loaded is swept when the app next starts.
- **A share is not looked at on this PC either.** A value in any account's registry could name one, and looking would connect as the
  administrator running this. A Documents folder on a share is left out, with a warning, and an app on a share cannot be built from.
- **An installer holds one account's data.** Every profile's data used to collapse into one. A scan reads whoever is signed in, the app's
  owner if it was installed for one account, or whoever you choose in **Whose data**, which only appears when there is a choice.
  Another account's folders are left out, however a value came to name them, and so are scheduled tasks that run as someone else.
- **What an account can write with no rights no longer decides what is packaged.** An uninstall entry in its own registry or a shortcut could
  name another account's folder, or any folder an administrator can read. An app installed for one account now has to be in that account's
  profile or owned by it, and gets nothing machine-wide by its name. A machine-wide app does not go by an account's shortcuts to find its
  program, and its install log is only believed if only administrators can change it. A build asks again around each copy whether a file leads
  into another account's folders or through a link that is new since the scan.
- **Another administrator at the UAC prompt no longer gets the data.** Reading and restoring go by whoever is signed in, through
  `HKEY_USERS\<SID>`, and the installer says so when that is not the account running it. If it cannot be told, it says that, and per-user data
  and settings are left out rather than guessed at.

### The programs themselves

- **The app is signed.** Releases sign `winappmigrator.exe` with Azure Artifact Signing through GitHub's OIDC token, with no stored secret, and
  record build provenance. See [signing](https://github.com/RandyNorthrup/WinAppMigrator/blob/main/docs/signing.md). The installer stub deliberately stays unsigned: a signed stub could be given someone
  else's package hidden in its certificate table while keeping a valid signature (CVE-2013-3900). Building refuses a signed stub, and the stub
  ignores anything inside a certificate table and refuses anything after it. A finished installer can be signed with `scripts/sign.ps1`.
- **A release has to be what it says.** Its tag has to match the version in `Cargo.toml`, and it has to carry the stub it was built from, byte for
  byte, a check CI runs on every push too. A release build fails with no stub to embed, and a release never takes one from beside it.
- **Nothing about how a window is drawn comes from the environment.** `WGPU_DX12_AGILITY_SDK_PATH` and its version could have Direct3D run
  `D3D12Core.dll` from any folder, past the restriction of DLLs to System32. A check plants a stand-in and sees that none of it runs, in the app, an
  installer, and an uninstaller, on each kind of runner. Without this fix it ran on all of them.
- **Tools started elevated get a fixed environment.** An elevated program gets its environment from what its account has set, by how Windows
  builds it, and DISM and older setup programs load what they have unpacked from `TEMP`. Every tool now gets its `Path` and `TEMP` from the Windows
  folder, and a setup program or DISM gets a folder of its own only administrators can write to, or does not run. Windows' own folders are read
  from the registry, not from `SystemRoot`, `ProgramData`, or `ProgramFiles`. PowerShell's module path and tools come from the Windows folder.
- **Windows' tools and DLLs are the real ones.** System tools run by absolute path, DLLs load from System32 only, and the C runtime is linked
  statically, so an installer can start on a clean PC and nothing can be planted for it. The stub is written straight into an installer, never
  through `%TEMP%`, and winget is found by asking Windows, with its signature checked, not through `PATH`.
- **A package no longer chooses what runs elevated for a runtime.** The URL, arguments, winget id, Windows feature, and registry probe come from
  specs compiled into the installer. A payload must be validly signed by Microsoft when it is downloaded and again just before it runs. Downloads
  are HTTPS only, including on redirects. With no payload, winget is used if it is there, and what it fetches is winget's to check.
- **Staging folders are locked down.** The installer's and the app's are restricted to Administrators and SYSTEM before anything is written to
  them, read back to be sure, held open, and the work stops if that fails.
- **Choosing light or dark cannot be made to write somewhere else.** The app remembers the mode in a folder the account it was started from can
  change, so a link put there could have had it overwrite any file an administrator can. It now puts a new file in place, and writes nothing if a
  link is in the way of the folder.
- **PowerShell escaping covers typographic quotes,** so a crafted file or task name on a scanned drive cannot run as code. Task definitions no longer
  go through a predictable `%TEMP%` file. Elevation is asked of `whoami`, once, and not knowing is a failure.
- **Logs are kept,** in `%ProgramData%\WinAppMigrator\logs`, which only administrators can read, with a limit on size and on how many are kept. They
  hold paths and key names only, and a test pins that.
- **Dependencies are upgraded:** `zip` to 9.2, `eframe` to 0.37, and `roxmltree` to 0.21, clearing six `cargo audit` advisories.

### Speed and size

- **A build reads each byte of an app once.** An app's files used to be copied to a staging folder, hashed there, zipped from there, and hashed
  again. They now go straight from where they are into the installer, and what goes in the manifest is the hash of what was read, so a file changing
  during the build cannot make the two disagree. The finished installer is then read back and checked against the manifest. An install hashes each
  file as it unpacks it, and reads back what it installed.
- **Compression runs on every core, and leaves compressed data alone.** Files are compressed in parallel, in a fixed order, into a standard zip,
  with memory bounded however big they are. What a sample shows will not compress is stored, whatever it is called.
- **Past 4 GiB, an installer is a small loader and a data file.** The loader's footer holds the data file's SHA-256, so a signature on the loader covers
  it: changing one byte of the data file is refused before anything in it is read, and changing the footer breaks the signature. A package that fits
  still comes out as one executable.
- **A scan reads less.** Only the parts of a large program that are looked at are read. On 2,779 real programs that found the same as reading each
  whole, from 22% of the bytes.
- **PowerShell calls no longer search every module.** On CI's Windows Server, a small install went from 6 minutes 33 seconds to 6.4 seconds.

  Building real apps on a 20-thread PC, before and after, by Windows' own count of what the process read, with `wam_measure`. The file cache was not
  controlled for, so take the times as best cases:

  | App | Files | Size | Time | Bytes read, as a share of the app | Most on disk | Installer |
  | --- | --- | --- | --- | --- | --- | --- |
  | GitHub CLI | 1 | 37 MiB | 4.7 s, 4.7 s | 6.8x, 2.2x | 64 MiB, 28 MiB | 27 MiB, 27 MiB |
  | 7-Zip | 107 | 5 MiB | 2.1 s, 2.1 s | 10.8x, 5.3x | 23 MiB, 16 MiB | 16 MiB, 16 MiB |
  | Git | 9,313 | 411 MiB | 24.3 s, 6.1 s | 4.7x, 2.0x | 610 MiB, 193 MiB | 181 MiB, 181 MiB |
  | GIMP | 9,963 | 620 MiB | 27.4 s, 6.8 s | 4.8x, 1.9x | 848 MiB, 221 MiB | 206 MiB, 207 MiB |
  | Ableton Live 11 Lite | 47,023 | 4.3 GiB | 200.5 s, 67.9 s | 4.8x, 3.1x | 7.4 GiB, about 6 GiB | 2.96 GiB, 2.98 GiB |
  | Visual Studio Community 2022 | 53,504 | 11.0 GiB | 857.1 s, 87.7 s | 4.4x, 2.0x | 14.9 GiB, 7.6 GiB | 3.77 GiB, 3.77 GiB |

  What is left after each app's one read is the stub, for a build that reads it from a file, and two reads of the finished package to check it. The
  last two rows are over 4 GiB before compressing and under it after, so each is also copied once to go after the stub, which is what the most on disk
  comes from. A scan's own reads are in those figures too.

### Windows and screens

- **Redesigned interface.** The app is a master-detail screen: the list fills the left, and the selected app, options, build, and result sit on the
  right. Native Segoe UI with a real semibold weight, plus Korean, Japanese, and Chinese fallback fonts when a name needs them. Painter-drawn icons, a
  window and file icon, and file details for Explorer and the UAC prompt. All colors meet WCAG AA contrast, enforced by a test. Sortable columns
  that sort versions by number, arrow-key navigation, Ctrl+F, and screen-reader support.
- **Build progress and cancel.** Builds report each step with a progress bar and elapsed time, and can be cancelled. The window stays light while a
  build runs. A build that lost files no longer reads as a clean success, and one that fails its hash check says not to use the installer.
- **The windows need Direct3D 12.** There is nothing else to draw with now, which makes every installer smaller and means fewer DLLs are ever searched for. Each says
  so if it cannot start, and an installer's and an uninstaller's say how to run without a window.
- **Windows fit the screen they open on.** At 1024x768 the app hung off the right edge with its close button out of reach, and an installer ran under
  the taskbar. A window that would not fit now opens maximized, and can be made as small as the screen is.
- **All three windows go by one rule for light and dark, where they run.** An installer used to open in whatever mode the app happened to be in on the
  PC it was built on. That is out of the package now. Each window is in the mode you chose in the app, on that account, or else the one Windows is
  in. A window stays in its theme, so the white search box and tooltips in light mode are gone.
- **Sizes and counts read the same everywhere.** `1380` is `1,380`, and screens say KB, MB, and GB, by 1024, as Windows Explorer does. The list of apps and
  the install location only offer all of a name or a path when some of it is cut.

### How it is tested

- **The code that changes a PC runs for real, on throwaway runners.** A CI job sets up a harmless app, builds an installer from it with the real code,
  removes it, installs it, and checks the machine with `sc.exe`, `reg.exe`, `schtasks.exe`, and the shell. It starts the restored services and a task, to see
  what Windows really runs and as whom, and covers packages asking for what the rules refuse, links planted at the install location, and undoing an
  install by an uninstall, a failure late in it, and an installer killed part-way. Others do the same from a VHD made hostile on purpose, with a real second account, with packages past 4 GiB, and with
  what a person leaves out and adds. The tests do nothing anywhere else.
- **Real apps are moved between PCs on every push to `main`.** 7-Zip, PuTTY, Eclipse Mosquitto, windows_exporter, and Git LFS, from their publishers at
  exact versions and hashes, go through custom, MSI, NSIS, and Inno Setup installers into real installers on one runner, and on to fresh runners on
  Windows Server 2022 and 2025 and Windows 11 on Arm, where each was and in a folder of another name. Windows is then asked to use what was restored: 7-Zip
  packs and unpacks, its shell extension is activated by CLSID from the new folder, a `.ppk` opens with Pageant, shortcuts start their programs, a new prompt
  finds what `Path` adds, and two services run and answer a real client over a socket. Every check has to fail with the app not there, before the install and
  after the uninstall, which runs the string each entry recorded. See [CONTRIBUTING](https://github.com/RandyNorthrup/WinAppMigrator/blob/main/CONTRIBUTING.md#tests-must-not-change-the-machine). On a Windows older than
  the app's it says so, and on Arm that its programs run through an emulator.
- **A PC with no Visual C++ runtime is tried for real.** A job builds Mosquitto the way the app builds things, takes the 64-bit runtime it uses off a runner,
  shows the broker cannot start, installs, shows the runtime came from what was in the package, and has the broker serve a real client.
- **Real windows are opened for real.** Every other test of the screens draws them with no window. A job opens the app, an installer, and an uninstaller on
  Windows Server 2022 and 2025 and Windows 11 on Arm, on runners whose logs show a software renderer, and looks at what is on the screen, and has Windows check that each fits.
  It does the same with graphics made unable to start, to see what a person is told.
- **Both windows' routes are run.** Nearly all of the tests give a window a scan or a build of its own. One clicks through the app as far as an installer on
  disk, and others work a real package through the installer's window, and on a runner install through it.
- **`cargo test --workspace` works without elevation.** The executables' logic moved to libraries, and their screens have headless tests that drive the real
  UI (`egui_kittest`). Set `WAM_UI_PREVIEW_DIR` to also write PNGs of the main states.

### Other fixes

- A package's reports say what its manifest says. They were written before anything was copied, so a file that could not be read was still listed as going in.
- A report's figures for each folder count each file once, in the innermost place that holds it.
- A file name with a character across its third byte, such as `Größe.txt`, no longer crashes an installer, a bug in the zip library's name check.
- A shortcut with no icon of its own no longer gets one of `,0`.
- Clicking another app during a build no longer starts a second build that deleted the first one's staging folder.
- A single file over 4 GiB no longer fails the build at the very end, and a failed build no longer destroys the installer from an earlier run.
- A registry value whose bytes do not fit its type no longer panics the scan.
- Restoring rewrites paths from the install location and not the executable's folder, so an app whose executable is in a subfolder keeps its layout in rewritten values.
- A task definition with a non-ASCII name survives, a path or task name that is not ASCII no longer fails verification, and a quoted task command is no longer lost.
- One unreadable installed file is one failed check, and not a verification that could not run.
- A firewall rule with a `TTK` condition or two `App`s is not carried, as it would let in more or read as another program's.

### Documentation

- The README is rewritten with badges, theme-aware screenshots, and what it has been tried on, and `SECURITY.md`, `CONTRIBUTING.md`, and `docs/signing.md`
  are new. Claims that were not true are corrected: that no secrets are collected, that hives are mounted read-only, that the GUI has a hash toggle, and
  others, in the app's own text as well.

## 0.4.2.1 — 2026-07-02

- **Licensing.** Added an MIT `LICENSE` (© 2026 Randy Northrup) and declared
  `license = "MIT"` (with `authors`) across the workspace. All dependencies are
  permissively licensed. Added an open-source attribution bundle generated by
  [`cargo about`](https://github.com/EmbarkStudios/cargo-about) (`about.toml`,
  `about.hbs`, `THIRD-PARTY-LICENSES.md`); the release workflow regenerates it and
  ships it plus `LICENSE` alongside the binaries.
- **Installers match the app's theme.** The app records its active theme in the
  package (`PackageManifest.ui_theme`), and a produced installer presents in that
  theme (older packages carry no tag and keep the default dark). Also fixed the
  installer clearing its window to eframe's near-black default instead of the theme
  background, which showed through in light mode.
- **README screenshots** of the pick → build → install flow (`docs/images/`).

## 0.4.2.0 — 2026-07-02

- **First tagged release + release automation.** Set the workspace version to
  `0.4.2` (released as `v0.4.2.0`) and added a GitHub Actions release workflow
  (`.github/workflows/release.yml`) that, on a `v*` tag, builds the installer stub
  then the app and publishes a GitHub Release with `winappmigrator.exe`, a
  portable `.zip`, and `SHA256SUMS.txt`. CI (`ci.yml`) was slimmed to quality
  gates only.
- **The app and its installers always run elevated.** Both embed a
  `requireAdministrator` manifest in every build, so Windows prompts once via UAC
  at launch and the whole session is elevated — there is no mid-task elevation
  prompt. The old "Restart as administrator" gate (and the app's
  `elevated`/`restart_as_admin` code and the offline-mount elevation guard) was
  removed as dead. The end-to-end test skips its reinstall step when it cannot
  spawn the elevated installer on a non-elevated host; CI runs elevated and
  exercises it fully.
- **Removed code signing.** The app and installers are distributed unsigned; the
  Authenticode signing hook, `docs/signing.md`, and `scripts/sign-release.ps1`
  were removed.
- **Docs/repo cleanup for accuracy.** Rewrote the README to current reality; fixed
  stale docs (references to a removed `winappmigrator-cli` crate, non-existent CLI
  flags, the removed elevation gate, an outdated `FootprintKind` list); moved
  `PLAN.md` into `docs/`; and removed decision docs (installer-strategy, sfx,
  tui-decision), the manual QA scripts, and `AGENTS.md`.
- **Production hardening pass** (cleanup, shared helpers, no silenced errors):
  - **Removed dead/legacy code** left after the offline-scan refactor: the
    live-only `list_installed_apps()` wrapper, `SystemContext::{source,describe}()`
    and the now-unused `SystemContext.source` field, `package::files_dir()`,
    `path_utils::path_eq_case_insensitive()`, and the write-only
    `InstallSource.authoritative_registry_count` field.
  - **Consolidated duplicated helpers.** The registry↔`winreg` mappings shared by
    restore and verify (`registry_root`, `environment_registry_target`,
    `registry_hive_name`, `registry_value_name`, `registry_raw_type` — including
    the load-bearing `…\Session Manager\Environment` path) now live once in a new
    `registry_target` module, so the write and verify sides can never drift.
    Three copies of a bytes→hex encoder collapsed onto `hashing::hex_lower`; the
    filesystem-name sanitizer shared by packaging and restore moved to
    `path_utils::sanitize_filename`; the GUI's two name sanitizers share one
    helper; and the installer's two staging-directory functions merged into
    `cleanup::create_staging_dir`, which (with every call site) now uses the
    `EXTRACTION_PREFIX` constant the stale-dir sweeper matches on — closing a
    latent gap where changing the prefix would silently break cleanup.
  - **Stopped silently dropping data on error.** Footprint discovery now records a
    warning when a base folder (`Program Files`, `ProgramData`, per-user
    `AppData`, …) can't be enumerated — previously an access-denied read on an
    offline/foreign disk dropped that whole category with no notice. Offline
    scheduled-task scanning warns on unreadable/malformed task XML instead of
    skipping silently; license scanning warns when a vendor sub-key can't be
    opened and still reports a license-named key's *location* even when its values
    can't be read; and the app's "Restart as administrator" action now surfaces a
    message if the elevated relaunch can't be started instead of exiting silently.
- Added **scanning an app from a connected drive** — a disk pulled from another
  (often failing) PC and mounted on the working machine (e.g. `E:\`) — instead of
  only the live OS. A new `SystemContext` abstraction (`system_context.rs`,
  `ScanSource::{Live, OfflineVolume}`) is the single seam every scanner reads
  through, replacing the two live-only seams (`RegKey::predef` and
  `std::env::var_os(known folder)`). For an offline volume it mounts the disk's
  registry hives read-only via `reg load` (using the process's backup privilege,
  so ACL-locked `config` hives read non-destructively), resolving the disk's
  active control set from `SYSTEM\Select\Current`, and unmounts them on drop;
  derives every known-folder base (`Program Files`, `ProgramData`, all user
  profiles, `Public`, `Windows`) from the volume root; and re-roots absolute
  source paths (`C:\…` → `<volume>\…`). All scanners were threaded through it —
  app inventory, registry, environment, license, install-source, and services/
  drivers (from the offline `SYSTEM` hive), with **scheduled tasks read from the
  disk's `Windows\System32\Tasks` XML** (new `roxmltree` dependency) since the
  live Task Scheduler cannot see another disk. Offline scanning **requires
  administrator rights**; the app offers a "Restart as administrator" gate. Per
  the requirement, it is **non-destructive**: the source disk is only ever read
  (backup-mode reads; never `takeown`/`icacls`). Source-shortcut enumeration is
  skipped for offline sources (the installer recreates shortcuts on restore). The
  GUI gained a **scan-source selector** ("This PC" / "Connected drive…") with a
  Windows-volume check and elevation notice.
- Added **non-destructive backup-mode file copying**: when packaging can't copy a
  file because its NTFS ACL denies this process (common on a foreign disk owned by
  another PC's SIDs), it falls back to `robocopy /B` (backup semantics via
  `SeBackupPrivilege`), which reads through the ACL without modifying the source.
  The fast in-process `fs::copy` is still tried first.
- The produced installer is now **named after the application**, not the chosen
  output folder (e.g. `YAMAHA DIAGNOSTIC SYSTEM2_installer.exe`).
- The app now **stages files on the destination drive**, so a large migration
  never fails for lack of space on the system/temp drive: the builder stages the
  package on the same drive as the chosen output folder, and each produced
  installer extracts its embedded package onto the install destination's drive
  (falling back to the installer's own drive, then the system temp drive).
- Extended footprint discovery to the **shared Public Documents** folder
  (`%PUBLIC%\Documents`, i.e. `C:\Users\Public\Documents`), which some apps use
  for machine-wide data instead of `ProgramData`. New `FootprintKind::PublicDocuments`
  (classified as `MachineAppData`) and a `UserDataAnchor::PublicProfile` so those
  files re-anchor to the same absolute `%PUBLIC%` location on restore (Public is
  fixed at `C:\Users\Public` across machines). Surfaced by YAMAHA DIAGNOSTIC
  SYSTEM2, whose diagnostic database lives at `C:\Users\Public\Documents\YDIS_Data`
  and was previously missed, so the app crashed at startup with
  `DirectoryNotFoundException` on a target.
- Turned the installer into a **graphical self-extractor** and made WinAppMigrator a **single portable executable**. WinAppMigrator (`winappmigrator.exe`) is now one file you run from anywhere: it carries the compiled installer stub inside itself (`include_bytes!`, staged by `winappmigrator-gui/build.rs`) and writes it out — plus the chosen app's package — to stamp out each installer. There is no sibling stub file to ship. Each produced installer is a standard **GUI** self-extractor: double-click opens a window showing the app (name/version/publisher), an editable **install location** defaulted to the app's original path, **Restore user data** and **Restore license** checkboxes shown *only when those were captured at build time*, a listed **Dependencies** plan (bundled / winget / Windows feature / already present), and **Preview**/**Install** actions with live progress and post-install hash verification. The installer requests administrator elevation via an embedded manifest (release builds; `embed-manifest`) so Program Files/HKLM writes succeed, and still supports a headless `--target-root … --apply` mode for automation/CI. Shared UI moved to a new `winappmigrator-ui` crate (theme + widgets) reused by both the app and the installer; the console-only `winappmigrator-cli` crate was removed (the installer stub is `winappmigrator-installer`, embedded in the app). `PackageManifest` gained `included_user_data`/`included_program_data`/`included_license_data` (drive the installer's checkboxes) and `RestoreOptions`/`VerificationOptions` gained `restore_license` to gate license restore.
- Fixed .NET runtime prerequisite detection. `pe_scan` now reads the managed image's CLR metadata-root version (`v2.0.50727` vs `v4.0.30319`), so an app hosted by the .NET Framework shim (`mscoree`) maps to **.NET Framework 3.5** (CLR v2.0/1.x) or **.NET Framework 4.x** (v4.0) instead of being mislabeled as the modern **.NET Desktop Runtime 8**. .NET Framework 3.5 is now installed on the target by enabling the **`NetFx3` Windows optional feature via DISM** (`prerequisites::run_windows_feature`, new `PrerequisiteBundle::windows_feature`), with an actionable fallback message when the payload can't be downloaded; 4.x is listed as in-box on Windows 10/11 (probe only). This was surfaced by YAMAHA DIAGNOSTIC SYSTEM2 (YdisUI.exe, CLR v2.0.50727), which the old detection would have tried to satisfy with .NET 8.
- Made the installer fully self-contained and automatic. The build output is now a single self-extracting executable whose file name encodes its contents (e.g. `App_userdata_license_installer.exe`), carrying the app, its data, license, runtime prerequisites, and drivers. (1) License data is now **restored automatically** by the installer instead of being left as a manual step: license files are placed back at their original/re-anchored locations, user-scoped (HKCU) registry values are captured and rewritten — text, numeric, and binary serials/tokens, with DPAPI-encrypted blobs restored best-effort and flagged machine-bound (may need reactivation) — and non-machine-bound framework stores are copied and restored. The misleading "restore manually" warning was removed. (2) Detected runtime prerequisites (Visual C++ runtime, etc.) are **downloaded and embedded** at build time (`ureq` over native-tls/SChannel), then installed silently and verified by registry probe on the target, with a winget/URL fallback when a download is unavailable. (3) **Application-owned drivers** (a `.sys` under the app's install footprint) are bundled and re-registered automatically (`sc create`, elevation required) and verified; vendor/inbox drivers are reported with precise, named reinstall guidance instead of the previous vague "requires a vendor driver installer" warning. Package and restore verification was extended to cover bundled-prerequisite presence, restored license files, and re-registered drivers. `PackageOptions` gained `bundle_prerequisites` and `installer_dir`; the GUI builds in a throwaway staging directory so the chosen folder receives only the single installer.
- Refocused the product to one purpose with a strict two-binary split: the **app** (`winappmigrator-gui`) reverse-engineers a selected installed application and, optionally, its license and/or user data, and builds a single self-extracting installer, hash-verifying the package; the **installer** (`winappmigrator`, the embedded stub) reinstalls that application on a separate device and hash-verifies the result. Restore lives only in the installer; the app never restores. The GUI is now a single screen (removed the Dashboard/Analyze/Package/Restore pages and navigation rail); the CLI crate is now purely the installer stub (removed the `list`/`scan`/`snapshot`/`package`/`restore`/`verify` builder commands). Deleted the inventory-snapshot feature end to end, the QEMU/WSL `vmtest/` harness, and the operator E2E scripts/docs that targeted the previous multi-command CLI. Added an end-to-end test that drives the real stub binary (reverse-engineer → build installer → reinstall on a clean target → verify) and tuned dev/release build profiles (optimized dependencies and first-party code in dev for fast tests; LTO, single codegen unit, and stripped symbols in release). No dead, orphaned, or legacy code remains; `clippy -D warnings` and the full test suite pass.
- Resolved a main-executable gap for MSI applications that record neither `InstallLocation` nor a usable `DisplayIcon` in their uninstall key (common for Windows Installer products such as PuTTY, PowerShell 7, the AWS CLI, TightVNC, and WinAppDriver): when the inventory heuristic finds no launcher, `install_source::primary_executable` now consults the authoritative install source — the Windows Installer component database for MSI products, otherwise the resolved install roots — and picks a representative non-auxiliary executable, so scanning/packaging such apps by name no longer fails with "installed app has no main executable guess."
- Made packaging resilient to locked or access-denied files: a single file that cannot be copied (a running application's binary, an AppData installer-cache entry, etc.) is now recorded as a warning and marked not-copied in the manifest instead of aborting the whole package, keeping verification consistent.
- Added optional license-data backup (`license_scan`): detects the application's own licensing/activation artifacts — license files (`.lic`/`.v2c`/`.lservrc`/`.rlm` and name-signalled ambiguous files), framework stores (CodeMeter `CmAct`, Safenet Sentinel, FlexLM), `*_LICENSE_FILE` environment variables, and vendor registry license keys — and reports each with a transferability and machine-binding hint. Opt-in in the app (default off) copies license *files* into the package `license/` directory; framework/registry/dongle artifacts are reported as metadata only. License data is never restored automatically, and WinAppMigrator never decrypts, bypasses/emulates activation, or emulates a dongle. AGENTS.md security policy updated to scope this as legitimate same-owner migration.
- Reworked user-data backup into a precise, first-class capability: machine-wide `ProgramData` is now classified as `MachineAppData` and controlled separately from genuine per-user data via `include_program_data` and the app's user-data toggle; user-data discovery expanded to `Documents` (including nested `My Games\<App>`) and `Saved Games`, resolving OneDrive/folder redirection through the `User Shell Folders` registry key; and online-only cloud placeholders (OneDrive Files On-Demand, via the OFFLINE/RECALL_ON_DATA_ACCESS attributes) are reported but never copied so a backup cannot silently hydrate or fail.
- Hardened executable reverse-engineering: parse delay-load imports (manual RVA walk of the delay directory, since `pelite` has no delay API; modern v2 descriptors, legacy v1 reported and skipped), read the embedded `RT_MANIFEST` to surface the `requestedExecutionLevel` and side-by-side `<dependentAssembly>` identities, detect managed .NET images via the CLR/COR20 data directory, and broaden runtime-prerequisite detection beyond VC++ to .NET (Framework vs self-contained), Java, Python, and Electron/Node with bundled-vs-manual classification. Confidence scoring now accounts for managed .NET and SxS dependencies. Validated against real binaries (mmc.exe: 323 delay imports + highestAvailable; regedit.exe: SxS Common-Controls; InstallUtil.exe: managed .NET).
- Narrowed the MSI component-database sweep to the SIDs that actually register the product (`component_sweep_sids`), with a full-SID fallback so results are never worse than the exhaustive scan; documented the decision to keep the exhaustive (COM-validated) sweep rather than scope via `Features`/Base85 descriptors, which could silently under-report the footprint.
- Deepened MSIX and Squirrel footprint discovery: MSIX now reads the `AppxManifest.xml` `Identity` name and anchors the app's per-user state at `%LOCALAPPDATA%\Packages\<IdentityName>_*` (a location name-token heuristics miss); Squirrel adds the install location as an authoritative root and records the current `app-<version>` directory using numeric version comparison.
- Added authoritative NSIS footprint extraction: when an NSIS install carries an `AdvUninstLog` `uninstall.log`, `install_source` reads the recorded paths (reusing the Inno UTF-16/ANSI path scan) and derives install roots and an authoritative file count instead of relying on the install directory plus heuristics alone.
- Added opt-in real-location user-data restore: `--restore-user-data` (CLI `restore`/SFX/`verify`) and a GUI checkbox re-anchor packaged `%AppData%`/`%LocalAppData%`/LocalLow/`%ProgramData%` data to the target machine's equivalent base instead of staging it under the target root. Centralized restore destination routing in `file_discovery::restore_destination_for_entry`/`split_user_data_relative` so restore and verification resolve identical paths, gated `%ProgramData%` apply behind elevation (`privilege::destination_requires_admin`), kept each destination bounded by `ensure_within_root`, and threaded the flag through `RestoreOptions`/`VerificationOptions`.
- Created Rust workspace foundation with shared core, CLI, and GUI crates.
- Added Stage 1 data models, registry inventory, PE import scanning, DLL resolution, scan orchestration, and report generation.
- Added native GUI shell using shared core scan APIs.
- Added project documentation, local agent instructions, and local quality/security configuration.
- Verified Stage 1 gates: formatting, typecheck, tests, clippy, release build, cargo audit, cargo deny, CLI list, CLI scan, and GUI smoke launch.
- Added recursive app-local dependency scanning, bounded file discovery, streaming SHA-256 hashing, and richer reports.
- Added conservative registry, service, scheduled task, and shortcut discovery.
- Added CLI and GUI package creation.
- Added restore dry-run and explicit file apply workflow with restore logs and protected-target elevation check.
- Verified package/restore E2E against an isolated executable copy; restored executable launched successfully.
- Added installed-app scope tracking for per-user versus per-machine applications.
- Added registry uninstall key capture, structured registry artifacts, `.reg` export generation, and path-rewritten registry restore.
- Added targeted registry discovery for Classes Applications and CLSID registrations to cover file associations and COM references.
- Added persistent user/machine environment variable detection, packaging, dry-run, and apply restore with path rewriting.
- Added source Windows version capture in package manifests.
- Added service, scheduled task XML, and shortcut restore apply paths with admin and overwrite checks.
- Added zip package archive creation, zip restore, and self-extracting package generation/restore.
- Added GUI controls for zip/SFX package creation and zip package restore selection.
- Added GitHub Actions Windows CI with format, typecheck, tests, clippy, audit, deny, release build, and portable zip artifact upload.
- Documented future code signing while keeping signing disabled.
- Verified real user-scoped E2E with SumatraPDF 3.6.1: install, package, uninstall, SFX dry-run, SFX apply, restored executable/version, and rewritten HKCU uninstall entry.
- Verified machine-scoped E2E with existing 7-Zip 25.01: package, zip generation, per-machine scope, HKLM registry capture, and zip dry-run restore.
- Refactored `PLAN.md` into checkbox-driven completed, remaining, milestone, gate, and definition-of-done sections.
- Added `.gitattributes` to keep repository text line endings stable.
- Added `verify` CLI workflow and shared core verification API for package integrity and post-restore checks.
- Added post-apply restore verification with JSON and Markdown verification reports.
- Added typed COM registration, file association, and driver/kernel component report models.
- Separated driver services from normal service restore and marked driver/kernel components as metadata-only manual reinstall items.
- Added GUI advanced scan settings and compact dependency graph visualization.
- Added project-local elevated machine-wide restore E2E and GUI click-through smoke scripts.
- Added project-local package/verify/restore dry-run performance regression script and guide.
- Updated `time` to 0.3.49 after local dependency policy rejected yanked 0.3.48.
- Added read-only inventory snapshot workflow in core, CLI, and GUI.
- Added GUI progress detail text for background workers.
- Added conditional manual-CI Authenticode signing hook and project-local signing helper.
- Documented installer, SFX, snapshot, and TUI release-line decisions.
- Added disposable user-scoped E2E harness and fixed scheduled task elevation handling so user tasks do not require admin unless the archived task is highest/SYSTEM scoped.
- Added disposable machine-scoped E2E harness for admin-shell certification of HKLM, service, scheduled task, common shortcut, and machine environment restore.
- Hardened scheduled task scanning against PowerShell enum/numeric JSON fields and verified both user-scoped and machine-scoped scheduled task restore E2E.
- Hardened the disposable machine E2E fixture service creation and partial-failure cleanup path.
- Verified elevated disposable machine-scoped restore apply E2E from an admin shell for files, HKLM registry, service, scheduled task, common Start Menu shortcut, and machine environment restore.
- Polished the native GUI with a light enterprise theme, status chips, grouped workflow sections, safer restore mode controls, compact result metrics, artifact folder buttons, and a light-theme dependency graph.
- Redesigned the native GUI as an `eframe`/`egui` console: full-height navigation rail, content header, routed Dashboard/Applications/Analyze/Package/Restore pages, a token-based light/dark design system (`theme.rs`), reusable theme-aware components with painter-drawn icons (`widgets.rs`), a searchable virtualized applications table, and a middle-ground guided flow ("Back up this app", default output dir) alongside the advanced controls.
- Added layered install-source reverse-engineering (`install_source.rs`): detects the install method (Windows Installer/MSI, MSIX, Squirrel, Chocolatey, Inno Setup, NSIS, Generic) and records human-readable evidence.
- Added authoritative MSI footprint enumeration by reading the Windows Installer component database (`Installer\UserData\<SID>\Components`) with squished/SQUID product GUIDs; validated against the `WindowsInstaller.Installer` COM `ComponentPath` results.
- Added authoritative Inno Setup footprint extraction by parsing the `unins000.dat` uninstall log (UTF-16/ANSI resolved paths), with detection narrowed to `unins###.dat` so a plain `uninstall.exe` is not mis-detected.
- Added App Paths registration and multi-signal install-directory roots (InstallLocation, DisplayIcon, uninstaller directory) for reliable non-MSI/EXE discovery.
- Rebuilt file discovery to be footprint-aware: scans `Program Files`, `Program Files (x86)`, `ProgramW6432`, `ProgramData`, and per-user `AppData` (Roaming/Local/LocalLow) with nested `Vendor\App` (depth-2) matching, cache/temp/log pruning, token cleaning and a generic-token stoplist to prevent cross-app false positives, and a structured `FootprintLocation` summary.
- Added `FootprintLocation`/`FootprintKind`/`InstallMethod` models; `install_method`, `install_evidence`, `authoritative_file_count`, and `footprint_locations` on `ScanReport`; a `UserData` file classification; an `include_user_data` option on scan/package; and a `--skip-user-data` CLI flag.
- Improved the installed-app main-executable guess to resolve `bin\`/`app\` launcher layouts and skip uninstaller/helper executables.
- Added `docs/discovery.md` (footprint discovery + install-source reverse-engineering) and `docs/gui.md` (GUI architecture).
