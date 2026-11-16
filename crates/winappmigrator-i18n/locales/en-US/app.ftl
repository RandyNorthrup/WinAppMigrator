# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.

## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Turn an installed app into a self-extracting installer
app-theme-to-light = Switch to Light Theme
app-theme-to-dark = Switch to Dark Theme
app-reload-list = Reload the Application List
app-list-loading = Loading
# { $count } is how many apps are listed.
app-list-count = { $count ->
        [one] { $count } App
       *[other] { $count } Apps
    }
app-list-load-failed = Load Failed
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = The window could not open: { $error }

## The list of installed apps

app-list-search = Search by Name or Publisher  (Ctrl+F)
app-list-filter-all = All
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Per-User
app-scope-per-machine = Per-Machine
app-scope-unknown = Unknown
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Unknown
app-list-loading-apps = Loading installed applications…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Could not list applications: { $error }
app-list-no-matches = No Matching Applications
app-list-no-matches-hint = Adjust the search text or scope filter.
app-list-worker-disconnected = Inventory worker disconnected.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, version { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Name
app-column-name-sort = Sort by Name
app-column-publisher = Publisher
app-column-publisher-sort = Sort by Publisher
app-column-version = Version
app-column-version-sort = Sort by Version
app-column-scope = Scope
app-column-scope-sort = Sort by Scope
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch
app-column-arch-sort = Sort by Arch

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Scan Source
app-source-this-pc = This PC
app-source-drive = Connected Drive…
app-source-image = Disk Image…
app-source-network = Network PC…
app-source-attaching-image = Attaching the image, read-only…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } over the network ({ $address })
app-source-disconnect = Disconnect
app-source-change = Change…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Select a Disk Image of the Other PC (.vhd or .vhdx)
app-source-pick-image-filter = Disk Images
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } has no volume with Windows on it.
app-source-image-not-attached = The disk image could not be attached.
app-source-pick-drive-title = Select the Connected Drive's Root (e.g. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } does not look like a Windows system drive (no \Windows and \Users). Pick the root of the drive from the other PC.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows = This has { $windows }. Installers built from it are made for Windows 10 and 11. With User Data, settings that Windows quietly kept aside for an older app come along, and runtimes Windows 11 does not have, such as DirectX 9, are installed with the app.{ $is_32_bit ->
        [yes] {" "}Its apps are 32-bit, so they go to Program Files (x86), where 64-bit Windows keeps them.
       *[no] {""}
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed = This has an older Windows. Installers built from it are made for Windows 10 and 11. With User Data, settings that Windows quietly kept aside for an older app come along, and runtimes Windows 11 does not have, such as DirectX 9, are installed with the app.{ $is_32_bit ->
        [yes] {" "}Its apps are 32-bit, so they go to Program Files (x86), where 64-bit Windows keeps them.
       *[no] {""}
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Pair with the Old PC
app-details-pair-first-hint = Once WinAppMigrator Client is running there and you have typed its code, its apps are listed here, and you build an installer from one as from this PC.
app-details-pick-an-app = Pick an Application
app-details-pick-an-app-hint = Select an app from the list to choose what to include and build its installer.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, version { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Unknown publisher, version { $version }
app-details-unknown-publisher = Unknown publisher

## What goes in besides the program

app-option-license = License and Activation Data
app-option-license-hint = May be tied to this PC and need reactivation on the new one.
app-option-user-data = User Data
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Settings and history, Documents, Saved Games, and ProgramData.
app-option-whose-data = Whose Data
app-option-whose-data-hint = An installer holds one account's data. It goes to whoever is signed in where it runs.

## Building the installer

app-output-folder = Output Folder
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\You\WinAppMigrator
app-output-folder-not-full = Enter a full folder path, such as D:\Installers.
app-build-create = Create Installer
app-build-create-again = Create Installer Again
app-build-cancel = Cancel
app-build-cancelling = Cancelling after the current step…
app-build-cancelled = Build cancelled. No installer was written.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Installer stub (winappmigrator-installer.exe) was not found beside this app, so a self-extracting installer cannot be built.
app-build-select-an-app-first = Select an application to migrate first.
app-build-look-again-first = This app has to be looked at again before it can be built.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = The build stopped unexpectedly: { $reason }. If it left an installer in the output folder, do not use it.
app-build-worker-disconnected = Migration worker disconnected.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Could not create the output folder { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Could not create a protected staging folder, so nothing was built: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = The installer could not be built. { $error }
app-build-no-installer = The build finished with no installer.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = The installer was written, but its hash check could not run: { $error }. Do not use it.

## How far a build has got

app-progress-preparing = Working out what goes in…
app-progress-checking-room = Checking there is room…
app-progress-bundling = Bundling runtimes and drivers…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Reading and compressing files: { $done } of { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Looking over what the installer says: { $done } of { $total }…
app-progress-writing = Writing the installer…
app-progress-verifying = Verifying hashes…

## What a build made

app-result-failed-hash-check = Do not use this installer: it failed its hash check.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone = { $count ->
        [one] Installer built, but { $count } file was gone since the app was looked at, so it is not in it. See Warnings below.
       *[other] Installer built, but { $count } files were gone since the app was looked at, so they are not in it. See Warnings below.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured = { $count ->
        [one] Installer built, but { $count } file could not be captured, usually because it was locked or unreadable. The app may not work without it. See Warnings below.
       *[other] Installer built, but { $count } files could not be captured, usually because they were locked or unreadable. The app may not work without them. See Warnings below.
    }
app-result-mentions-who = Installer built, and it still says something of who it came from. Read what it says below before you give it to anyone.
app-result-ready = Installer ready. Its package passed its hash check.
app-result-files = Files
app-result-installer-size = Installer Size
app-result-warnings = Warnings
app-result-log = Log
app-result-copy-error = Copy Error
app-result-installer = Installer
app-result-data-file = Data File
app-result-open-folder = Open Folder
app-result-copy-path = Copy Path
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Could not open the folder: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = This app is too big for one executable, so its installer is two files, { $installer } and { $data_file }. They have to stay together: copy both to the other PC, into the same folder, with these names. If you sign the installer, that signature also covers the data file.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Run it on the other PC to reinstall { $app_name }. It checks its files by SHA-256. Windows shows "Unknown publisher" because installers you build are not signed.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Verification Errors ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Warnings ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = and { $count } more.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Getting ready to look at this app…
app-scan-install-record = Reading how it was installed…
app-scan-near-matches = Looking for what might also be part of it…
app-scan-system-items = Looking for its services, tasks, and shortcuts…
app-scan-files = Going through its files…
app-scan-registry = Reading what ties it into Windows…
app-scan-finishing = Finishing up…
app-scan-look-again = Look Again
app-scan-stop = Stop
app-scan-running-hint = You can pick another app or change what is included while this runs.
# { $error } is why, in words that follow a colon.
app-scan-failed = This app could not be looked at: { $error }
app-scan-try-again = Try Again
app-scan-stopped = Stopped. An installer can be built once this app has been looked at.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = It stopped unexpectedly: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Files
# Over how big the installer can be: up to that size.
app-room-up-to = Up To
app-room-free = Free
# { $drive } is a drive, such as C:.
app-room-free-on = Free on { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Unknown
app-room-most-it-could-need = The most it could need, before compressing.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } needs up to { $needed } free to build this, and has { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = There is not enough room. { $drives } Choose an output folder on another drive, or leave something out.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = What Goes in the Installer
app-footprint-working-out = Working out what goes with that…
# { $error } is why.
app-footprint-would-not-make-a-package = What is left would not make a package: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = That also leaves out { $list }, because what they run or name would not be there. They will not be set up on the other PC.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = That also leaves out { $list }, and { $count } more, because what they run or name would not be there. They will not be set up on the other PC.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = License data in what you left out still goes in, because that goes by the License and Activation Data option: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = This was looked at { $elapsed } ago. Files are read when you build, so a change is picked up, and anything gone by then is listed. Look Again for anything new.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Program Files
app-footprint-data-folders = Data Folders
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } with the User Data option
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary = { $count ->
        [one] { $included } of { $count } folder, { $size }
       *[other] { $included } of { $count } folders, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data = { $count ->
        [one] { $size } in { $count } file of this app's data will only go in with the User Data option.
       *[other] { $size } in { $count } files of this app's data will only go in with the User Data option.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row = { $count ->
        [one] { $count } file. { $detail }
       *[other] { $count } files. { $detail }
    }
app-footprint-services = Services
app-footprint-startup = Starts Automatically
app-footprint-tasks = Scheduled Tasks
app-footprint-shortcuts = Shortcuts
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } of { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Left out too: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = What Ties It into Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary = { $count ->
        [one] { $included } of { $count } kind
       *[other] { $included } of { $count } kinds
    }
app-footprint-integration-nothing-left = Nothing left of it, because of what you left out
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things = { $count ->
        [one] { $count } thing set up
       *[other] { $count } things set up
    }
app-footprint-prerequisites = What It Needs to Run
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary = { $count ->
        [one] { $count } runtime
       *[other] { $count } runtimes
    }
app-footprint-near-matches = Might Also Be Part of It
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } of { $count } added
app-footprint-near-matches-hint = Not in the installer unless you tick them. Ticking one looks at the app again, with the same checks as the rest, so what looks like a password still stays out.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Comes with the User Data option. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = over { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Folder: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Hidden Part of an App: { $place }
app-footprint-nothing-left-out = Nothing was left out for looking like a password, a license, or a certificate.
app-footprint-left-out = Left Out, and Why
# { $count } is how many things were left out.
app-footprint-left-out-summary = { $count ->
        [one] { $count } thing
       *[other] { $count } things
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = No installer would be allowed to set these up:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Worth Knowing ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = None found: { $list }.
app-none-found-services = services
app-none-found-startup = startup items
app-none-found-tasks = scheduled tasks
app-none-found-shortcuts = shortcuts
app-none-found-integrations = anything that ties it into Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Put the Client on the Old PC
app-network-step-client-hint = WinAppMigrator Client runs on the PC the app is on now, Windows 7 or later. It only lets this PC read it, encrypted, by a code it shows.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Save to USB Drive { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Save to USB Drive { $drive } ({ $name })
app-network-save-to-folder = Save to a Folder…
app-network-look-for-usb-again = Look for USB Drives Again
app-network-no-usb = No USB drive is plugged in. Plug one in, or save to a folder: one you can reach from the old PC, such as a shared folder on your network.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Saved as { $path }. On the old PC, close the app you are moving, then double-click it and let it make changes. It shows a code.
app-network-step-pick = 2. Pick the Old PC
app-network-looking = Looking on your network for PCs running the client…
app-network-type-name = Or Type Its Name or Address
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = OLD-PC or 192.168.1.20
app-network-step-code = 3. Type the Code It Shows
app-network-code = Pairing Code
app-network-connect = Connect
app-network-pairing = Pairing…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = pairing stopped before it was done
app-network-wrong-code = that is not the code the old PC shows. Check it and try again.
# { $error } is what went wrong.
app-network-not-reached = the old PC could not be reached: { $error }
app-network-pick-folder-title = Choose a Folder for WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = the client could not be saved in { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = this build of WinAppMigrator does not carry the client. Build it with scripts\build-client.ps1, then build the app again.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = the note beside it could not be saved: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client

    Run this on the PC you are moving apps off: Windows 7, 8, 10 or 11.

    1. Close the app you are moving.
    2. Double-click "WinAppMigrator Client.exe" and let it make changes.
    3. It shows a 6-digit pairing code. In WinAppMigrator on your new PC, choose
       Network PC..., pick this PC, and type the code.
    4. When the new PC is done, press Enter in the client's window, or close it.

    It only reads. While it runs it keeps a snapshot of the Windows drive and lets
    its own two ports through Windows Firewall from your local network, and it
    removes both when it stops. What crosses the network is encrypted, and only a
    PC that has the code can read anything.
