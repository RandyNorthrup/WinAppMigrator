# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.

## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } goes through a link or is not on the connected drive, so it was not read.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } is not on a disk of this PC, or has a name that would open something else, so it was not read.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } is in another account's folders or is not this account's, and this app's own record does not say it belongs there, so it was not read.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-no-executable = could not find an executable for { $app_name }. A driver, runtime, or add-in may have no program of its own to package.
# { $path } is the file that was picked.
core-scan-not-a-file = scan target is not a file: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = The environment variable { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Its name starts with this app's name.
core-near-has-app-name = Its name has this app's name in it.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Its name starts with the publisher's name, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = A part hidden from the list of apps, installed beside this app in { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = A part hidden from the list of apps, from the same publisher, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = These always go with the main program
core-footprint-locked-main-program = Holds the main program, which always goes in
core-footprint-locked-needed = Holds what the main program cannot start without
core-footprint-locked-font = Goes with its font, under what ties it into Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Settings in your profile (AppData)
core-footprint-place-local = Settings for this PC in your profile (AppData)
core-footprint-place-program-data = Shared by everyone on the PC (ProgramData)
core-footprint-place-documents = In your Documents
core-footprint-place-saved-games = In your Saved Games
core-footprint-place-public-documents = In Public Documents
core-footprint-place-app-folder = In the app's folder
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = In { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = Files directly in { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Not in this installer, because the License option is off
core-footprint-font-stays-behind = It stays behind.
core-footprint-drivers-left-out = Drivers that belong to Windows or a vendor, which cannot be passed on, so reinstall them on the other PC
core-footprint-driver-reinstall = Reinstall it from its vendor's own package.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = it needs { $service }, which is left out
core-footprint-told-to-use-left-out = it is told to use something in a folder you left out
core-footprint-file-in-left-out = its file is in a folder you left out
core-footprint-program-in-left-out = the program it is for is in a folder you left out
core-footprint-names-left-out = it names a folder you left out
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = whoever built this installer left it out
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = this scan is of another app than the one being built, so nothing was built
core-package-scan-brought-in-more = this scan brought in data that this build was not asked to, so nothing was built
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } is not there any more, so nothing was built. Look at the app again.
core-package-needs-stub = a self-extracting archive requires an installer stub
# { $reason } is what went wrong.
core-package-staged-file-failed = a file in the staging folder could not go in the package: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = there is not enough room to build this installer, so nothing was read or written. { $shortfalls }. Free up some space there, or choose an output folder on another drive.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } goes through a link or is not on the connected drive, so it was not read
# { $path } is a file's path.
core-package-not-read-not-local = { $path } is not on a disk of this PC, so it was not read
# { $path } is a file's path.
core-package-not-read-other-account = { $path } leads into another account's folders, so it was not read
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } is not on the connected drive, so it was not read
# { $path } is a file's path.
core-package-backup-copy-no-parent = cannot backup-copy a file without a parent directory: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = backup-copy destination has no parent directory: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = robocopy backup-mode copy failed (exit { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = temporary package folder
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = the installer
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = another file has the same place in the package: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } changed in the protected folder while the package was being made
core-package-worker-died = a thread compressing files stopped unexpectedly, so no installer was written
# { $path } is a file's path.
core-package-link-not-read = { $path } is a link, so it was not read
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = { $path } could not be compressed: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = { $path } could not be read back: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = the folder to write under must be a full path: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = the folder to write under is not a folder: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = path escapes intended root: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = refusing to write through a link: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = refusing to go through a link: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = refusing to go through a link: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } is a link to somewhere else, and others may be able to change where it leads. Use the place it leads to
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } has no parent folder
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = the install location must be a full path: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = the install location is a link to somewhere else: { $path }
# { $path } is a file's path.
core-discovery-device-name = path uses a name reserved for a Windows device: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } has a name that this version of Windows opens as a device and not as a file, so nothing was written to it. It can be installed on Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = unsafe path: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = file has no package relative path: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = installer package
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = embedded installer stub
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = entry { $index } of the package could not be read: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = zip entry has unsafe path: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } holds more than it says it does
# { $name } is a file's name inside the package.
core-archive-name-too-long = this name is too long for a zip: { $name }
core-archive-changed-while-copied = the package changed while it was being put after the stub, so no installer was written
core-archive-stub-not-a-program = the part of this installer before its package is not a program
core-archive-damaged = this installer is damaged or was changed after it was built, so nothing was read from its package. Copy it again from where it was built.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = this installer keeps its package in a file named { $name }, in the same folder, and that file is not there. The two files have to stay together: copy { $name } here from where the installer was built, then run this again.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = this installer's data file, { $name }, could not be opened: { $error }. The two files have to stay together, in a folder this PC can read.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } is a link or a folder, and this installer's data file has to be a file of its own, right beside it, so nothing was read from it. Copy the real file here.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } is not this installer's data file, or it is damaged or was changed after it was built, so nothing was read from it. Copy both files again from where they were built.
core-archive-stub-signed = the installer stub is signed; it must stay unsigned, so sign the finished installer instead
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = the installer would be { $size }, and Windows cannot run an executable of 4 GB or more; nothing was written
core-archive-outside-signature = installer has data outside its signature and may have been tampered with
core-archive-footer-length = self-extracting archive footer length is invalid
core-archive-seek-outside = seek outside the package
# { $path } is a file's path.
core-archive-unsafe-package-path = unsafe package path: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = empty package path: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } is not a disk image Windows can attach (.vhd or .vhdx)
core-image-needs-administrator = attaching a disk image needs administrator rights
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } is not a file
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } is a differencing disk, which only holds what changed since the disk it was made from, and Windows would open that disk too, wherever it says it is. Merge it into that disk first, with Hyper-V's Edit Disk, and pick that
# { $path } is the disk image file.
core-image-unreadable = { $path } is not a .vhd or .vhdx that could be read
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = this PC's ProgramData folder could not be found
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows already has it attached, and whatever attached it may be writing to it. Detach it first, or pick its drive as a connected drive
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = { $path } could not be attached: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } has no volume with Windows on it that could be read. A volume encrypted with BitLocker has to be unlocked first.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows did not say why

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = the other PC took too long to answer
core-remote-closed = the link to the other PC is closed
core-remote-failed = the link to the other PC failed
core-remote-answered-something-else = the client answered something else
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } was not found on the network
core-remote-did-not-say-what-it-is = the client did not say what it is
core-remote-ordinary-names-only = only a path below the other PC's root, in ordinary names, is asked about
core-remote-more-but-none = the other PC said a folder had more in it, and gave none of it
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = the other PC listed more than { $count } things in one folder
core-remote-sent-too-much = the other PC sent more of a file than was asked for
core-remote-name-not-text = a name that is not text cannot be asked about
core-remote-below-root-only = only a path below the other PC's root can be asked about
core-remote-file-ended-early = the other PC's file ended early
core-remote-link-not-followed = a link on another PC is not followed
core-remote-only-files-opened = only a file is opened on another PC
core-remote-listing-said-nothing = the listing said nothing of it
core-remote-change-time-unknown = when it was changed is not known
core-remote-seek-before-start = a seek to before the start
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large = { $size ->
        [one] it is { $size } byte, and no more than { $limit } were expected
       *[other] it is { $size } bytes, and no more than { $limit } were expected
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = could not start PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = could not read the signature: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = it is signed by { $signer }, not Microsoft
core-signature-not-microsoft-unnamed = it is signed by someone else, not Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = its signature status is { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = What this needs from Windows could not be read, so whether it can run on the other PC will not be checked.
core-warning-build-external-manifest = External application manifest detected; side-by-side dependency interpretation is deferred.
core-warning-build-non-exe = Selected file does not use the .exe extension; PE scanning will continue.
core-warning-build-stand-in-program = No ordinary application executable was found, so this file stands in for it. Architecture and dependency results describe this file, and may not describe the application.
core-warning-build-other-apps-files-unknown = Not all of what Windows Installer has of other apps could be read, so a folder this app has on record can hold another app's files with nothing here to say so.
core-warning-build-app-folder-has-another-apps-file = Windows Installer says another app has a file in the folder this app's program is in, and it comes along, as all of that folder is taken.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = This app's App Paths entry says it looks in a folder that is not its own, so that folder does not come along. It may not start on a PC without what is there.
core-warning-build-uninstaller-folder-left-out = The folder this app's uninstaller is in was left out, because Windows Installer says another app has a file there.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out = Something on record for this app names a folder that was left out, because { $why ->
        [unreadable] it may not be read
        [missing] it is not there
        [link] it goes through a link
        [short-name] it is not written the way it is really called
        [another-account] it is in another account's folders
        [one-account] this app was installed for one account, and this is not that account's
        [shared] it holds what is shared, or is part of Windows
        [generic-name] its name says nothing of whose it is
        [publisher] it is named only for the publisher, and may hold its other products
        [holds-the-app] it holds this app's own folder, and may hold more
        [another-apps-name] it goes by another app's name
        [another-apps-record] its install record is another app's
        [in-another-apps-folder] it is in another app's folder
       *[another-apps] it is another app's
    }.
core-warning-build-app-folder-holds-another-app = Another app on this PC is on record as being inside this app's folder, and comes along with it, as all of that folder is taken.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = This app was on 32-bit Windows. Its registry keys are laid out as 64-bit Windows keeps a 32-bit app's, and the installer offers Program Files (x86) for it.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = A program needs { $dll }, a debug build of the Visual C++ runtime. Microsoft has no installer for those, and may not let them be passed on, so it will not run on a PC without Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } is a .NET program packed into one file, which says nothing of which .NET it needs, so none is planned for it. If it needs one, it will not start on a PC without it.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected = { $count ->
        [one] { $count } of the app's programs was not looked at for the runtimes it needs, as it has so many or they are so large.
       *[other] { $count } of the app's programs were not looked at for the runtimes they need, as it has so many or they are so large.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = The app needs { $runtime }, and there is no installer for it here, so none is run. Install it yourself on the other PC.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Left out of the package: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Dependency could not be resolved: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } was only found through this PC's search path, and is not part of the app, so it does not come along. The app may not start on a PC without it.
core-warning-build-driver-detected = Driver or kernel component detected. A driver the app owns is bundled and re-registered; a vendor or inbox driver is not redistributed and must be reinstalled from its signed vendor package.
core-warning-build-near-no-longer-offered = Something ticked in the list of what might also be this app's is not on that list any more, so it was not added. Look at the list again.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out = Part of what { $app_name } says is its own was left out: { $why ->
        [unreadable] it may not be read
        [missing] it is not there
        [link] it goes through a link
        [short-name] it is not written the way it is really called
        [another-account] it is in another account's folders
        [one-account] this app was installed for one account, and this is not that account's
        [shared] it holds what is shared, or is part of Windows
        [generic-name] its name says nothing of whose it is
        [publisher] it is named only for the publisher, and may hold its other products
        [holds-the-app] it holds this app's own folder, and may hold more
        [another-apps-name] it goes by another app's name
        [another-apps-record] its install record is another app's
        [in-another-apps-folder] it is in another app's folder
       *[another-apps] it is another app's
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = The registry keys { $app_name }'s install record lists were left out, since that record also names what is another app's.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = A program was in the folder where Windows keeps what an app wrote for one account. Windows never puts one there, so something else did, and it was left out.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } was left out: it goes through a link on a connected drive, or it belongs to another account than the one whose data is being included.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = { $user }'s { $folder } folder was at { $path }, which is not on this drive, so what is there was not included.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = { $user }'s { $folder } folder was at { $path }, which is not on a disk of this PC, so what is there was not included.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Could not read { $path } ({ $error }); application data stored there was not included.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Could not read { $path } ({ $error }); nested application data there may be missing.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Could not read a filesystem entry: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed = { $count ->
        [one] { $count } link, or name that would open something else, was not followed in this folder, so what it points at is not in the installer: { $names }
       *[other] { $count } links, or names that would open something else, were not followed in this folder, so what they point at is not in the installer: { $names }
    }{ $more ->
        [0] .
       *[other] , and { $more } more.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep = { $count ->
        [one] { $count } folder is more than { $depth } levels down, so what is in it is not in the installer: { $names }
       *[other] { $count } folders are more than { $depth } levels down, so what is in them is not in the installer: { $names }
    }{ $more ->
        [0] .
       *[other] , and { $more } more.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Could not read file metadata: { $error }
core-warning-build-reserved-name = This file's name is reserved for a Windows device, so it was not copied.
# { $error } is what went wrong.
core-warning-build-hash-failed = Could not hash file: { $error }
core-warning-build-cloud-placeholder = Online-only cloud file (e.g. OneDrive Files On-Demand) has no local data; pin or download it before backup. Reported but not copied.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Driver/kernel file '{ $file_name }' is not copied as a plain file. If it backs a registered driver service for this app, the installer bundles it and re-registers the driver automatically; otherwise reinstall it with the vendor's signed driver package.
core-warning-build-file-gone = This was there when the app was looked at and is gone now, so it is not in the installer.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Could not copy file into package: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through = { $count ->
        [one] This package has a file that could not be looked through for who it came from: { $names }. What it says may say that.
       *[other] This package has files that could not be looked through for who it came from: { $names }. What it says may say that.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = This app has more than { $count } empty folders, and only that many will be made again.
core-warning-build-credential-by-now = Left out of the package: it looks like a credential by now.
core-warning-build-no-package-path = Copyable file had no package relative path.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Could not place bundled prerequisite { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Could not download { $runtime } ({ $error }); the installer will try winget on the target.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = The download of { $runtime } failed its signature check, so it was not bundled: { $reason }. The installer will try winget on the target.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = Downloaded { $runtime } but could not hash it ({ $error }); the installer will try winget on the target instead.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Could not bundle driver image for '{ $service }': { $error }
core-warning-build-license-not-copied = License data could not be copied (missing or unreadable); recorded as metadata only.
core-warning-build-license-file-credential = A license file looks like a credential, and was left out.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Could not enumerate part of a license store: { $error }
core-warning-build-license-store-credential = A file in a license store looks like a credential, and was left out.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = Skipped a license store file: { $error }
