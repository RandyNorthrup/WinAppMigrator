# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.

## The installer's window: its title and header

installer-window-title = WinAppMigrator Installer
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Install { $app_name }
installer-tagline = Reinstalls an application captured on another PC
# What an app is called when its package gives it no name.
installer-unnamed-app = Application

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = The installer's window could not open: { $error }
installer-window-failed-without-one = It can still install without one: run this file from an administrator's command prompt with --apply, and --target-root followed by a folder to choose where.

## Checking the installer when it opens

installer-checking = Checking the installer…
installer-stub-no-package = This is a WinAppMigrator installer stub, and it carries no application package. Build an installer with WinAppMigrator, then run the produced .exe here.
installer-check-stopped = The check of the installer stopped.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Could not locate the installer: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Could not read the installer: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = This installer's package cannot be used on this PC: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Captured From
installer-install-location = Install Location
# The title of the folder picker.
installer-choose-location = Choose the Install Location
installer-folder-has-files = This folder already has files. Files with the same names are replaced, and come back if you uninstall.
installer-room-needed = Room Needed
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = There may not be enough room here: { $shortfall }. It can unpack on another drive, but the app's files still have to fit.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = An earlier install of { $app_name } never finished. Install undoes it first.
# Beside "Installer", for whether it proves who made it.
installer-signature = Installer
installer-signature-checking = Checking its signature…
installer-signature-unsigned = Not signed: publisher unknown
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Signed by { $publisher }
installer-signature-unnamed = Signed by an unnamed publisher
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = This installer's signature is not valid ({ $reason }). It may have been changed since it was signed.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = { $app_name } is already installed in this folder, so this copy cannot go here. To install it here, first uninstall the copy on this PC, in Windows Settings under Apps, and then open this installer again. Or enter a new, empty folder above.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = { $other_app } is installed in this folder, so { $app_name } cannot go here. Enter a new, empty folder above. Or, if you no longer need { $other_app }, uninstall it first, in Windows Settings under Apps, and then open this installer again.
installer-location-has-files = This folder already has files in it that were not installed by WinAppMigrator. They may belong to another app, so nothing is installed over them. Enter a new, empty folder above.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = This folder cannot be used: { $reason }. Enter another folder above.

## What the install sets up beyond its files

installer-beyond-files = What This Sets Up Beyond Files
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = All of this is done with administrator rights. If the install fails, what it changed is put back, and you are told of anything that could not be. You can also undo the install later from Settings, under Installed apps. Files it replaces come back, and runtimes it installs stay.
installer-beyond-files-none = Nothing: it only copies files, and settings in the app's own registry keys.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = This package cannot go to this location: { $problem }
installer-working-out = Working out what this would set up at this location…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = working out what it would do stopped
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out = { $count ->
        [one] { $count } more was left out
       *[other] { $count } more were left out
    }

## Options

installer-options = Options
installer-user-data = User Data
installer-user-data-detail = On: settings, Documents, and Saved Games go to the usual folders of whoever is signed in. Off: all of it goes under the install folder, and nothing is set up to run from it.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = This holds the data of { $account }.
installer-license = License
installer-license-detail = Activation and license files, and the registry values that hold licenses. Some may need reactivation here.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dependencies
installer-dependencies-detail = What this app needs, and how each is provided.
installer-dependencies-none = No additional runtime dependencies were detected.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Windows Feature: { $feature }
installer-dependency-included = Included
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Installed with winget: { $id }
installer-dependency-manual = Install Manually
installer-dependency-built-in = Built into Windows
installer-dependency-ships-with-app = Ships with the App

## Buttons, and what is said beside them

installer-install = Install
installer-preview = Preview Changes
installer-cancel = Cancel
installer-close = Close
installer-open-folder = Open Install Folder
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Could not open the folder: { $error }
installer-location-blocks = Install and Preview Changes are off until the Install Location can be used. The reason is above, under it.
installer-package-blocks = Install is off because this package cannot go to this location. The reason is above.
installer-list-changed = What this would do here has changed, so nothing was installed. Look over the list below, and click Install again if it is what you want.
installer-cancelled-early = Cancelled. Nothing was changed.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } of { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Getting ready…
installer-step-cancelling = Cancelling after the current step…
installer-step-putting-back = Putting back what was changed
installer-step-unpacking = Unpacking
installer-step-checking-package = Checking the package
installer-step-recovering = Undoing an earlier install that never finished
# What a preview is doing, whatever step it is on.
installer-step-previewing = Working out what would change
installer-step-runtimes = Installing runtimes
installer-step-files = Copying files
installer-step-services = Setting up services
installer-step-drivers = Setting up drivers
installer-step-registry = Writing registry settings
installer-step-license = Restoring license data
installer-step-tasks = Setting up scheduled tasks
installer-step-shortcuts = Making shortcuts
installer-step-environment = Setting environment variables
installer-step-settings-entry = Adding the app to Settings
installer-step-verifying = Checking what was installed
installer-step-undoing = Undoing

## When an install cannot go ahead or stops

installer-carries-no-package = This installer carries no package.
installer-stopped-unexpectedly = The install stopped unexpectedly.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = The install stopped unexpectedly: { $reason }. If it had begun, running an installer again undoes what it did first.
# { $error } is what went wrong reading it.
installer-package-read-failed = Could not read the package: { $error }
installer-no-programdata = This PC's ProgramData folder could not be found, so nothing was installed: an install keeps the record it is undone by there.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Could not create a staging folder: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Could not unpack the package: { $error }
installer-staging-nowhere = there is nowhere to unpack the package that only administrators can get into
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = there is nowhere to unpack the package that only administrators can get into. The one to look at is { $failure }

## What came of an install, or of a preview

installer-result = Install Result
installer-result-preview = Preview: Nothing Has Been Changed Yet
installer-result-installed = The application was installed and passed its hash check.
installer-result-installed-without-left-out = The application was installed and passed its hash check, without what is listed as left out below.
installer-result-not-all-put-back = The install failed, and not all of what it changed could be put back. What is left is listed below, and the next install tries again.
installer-result-cancelled-put-back = The install was cancelled, and what it changed was put back.
installer-result-failed-put-back = The install failed, and what it changed was put back.
installer-result-not-clean = The install did not finish cleanly. Review the problems below.
installer-result-changes-put-back = Changes Put Back
installer-result-changes-made = Changes Made
installer-result-changes-planned = Changes Planned
installer-result-warnings = Warnings
installer-result-errors = Errors
installer-result-package-valid = Package Hashes Valid
installer-result-package-invalid = Package Hashes INVALID
installer-result-files-valid = Installed Files Valid
installer-result-files-invalid = Installed Files INVALID
installer-result-reboot = A dependency needs a reboot to finish. Restart Windows, then launch the app.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Changes ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = and { $count } more.
# Over the path of the install's log file.
installer-log = Log

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = WinAppMigrator installer stub — it carries no application package.
installer-cli-stub-how = Build an installer with the WinAppMigrator app, then run the produced .exe on the target machine.
installer-cli-usage-title = WinAppMigrator self-extracting installer.
installer-cli-usage = Run it with no arguments to open the graphical installer, or:
installer-cli-usage-preview = preview the reinstall: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = reinstall:             --apply [--target-root <DIR>]
installer-cli-no-package = installer does not contain an embedded package
installer-cli-no-programdata = this PC's ProgramData folder could not be found, so nothing was installed: an install keeps the record it is undone by there
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = { $path } cannot be used: { $problem }. Give --force only if you know it is this app's
installer-cli-no-location = this package does not say where its app was, so give a --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = this package's app was in { $path }, and that cannot be used: { $problem }. Give a --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Room: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Note: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Sets up: { $count }
installer-cli-applied = Reinstall applied.
installer-cli-previewed = Reinstall previewed.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Changes logged: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Warnings: { $count }
# { $count } is how many errors.
installer-cli-errors = Errors: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Refused: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Package hashes valid: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Reinstall hashes valid: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = Install id: { $id }
installer-cli-put-back = The install failed, so what it changed was put back.
# { $path } is the log file.
installer-cli-log = Log: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = log: { $path }
# { $error } is what went wrong.
installer-cli-error = error: { $error }

## The uninstaller's window

uninstaller-window-title = Uninstall
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Uninstall { $app_name }
uninstaller-tagline = Undoes what its installer did
uninstaller-reading = Reading the record of this install…
uninstaller-stopped-unexpectedly = The uninstall stopped unexpectedly.
uninstaller-what-will-be-removed = What Will Be Removed
uninstaller-install-location = Install Location
uninstaller-files = Files
uninstaller-registry-values = Registry Values
uninstaller-services = Services
uninstaller-drivers = Drivers
uninstaller-scheduled-tasks = Scheduled Tasks
uninstaller-environment = Environment
uninstaller-plan-stays = Files you added or changed stay, with the folders they are in.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back = { $count ->
        [one] { $count } file this install replaced comes back, or goes beside a file that has changed since.
       *[other] { $count } files this install replaced come back, or go beside a file that has changed since.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = Runtimes it installed stay, because other programs may use them: { $runtimes }.
uninstaller-options = Options
uninstaller-remove-user-data = Also Remove User Data and License Data
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail = { $count ->
        [one] { $count } item of its settings, data, and license. It stays unless this is on. What it replaced comes back, or goes beside it if it has changed since.
       *[other] { $count } items of its settings, data, and license. They stay unless this is on. What they replaced comes back, or goes beside them if they have changed since.
    }
uninstaller-uninstall = Uninstall
uninstaller-cancel = Cancel
uninstaller-close = Close
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Undoing the install: { $done } of { $total }…
uninstaller-step = Undoing the install…
uninstaller-done = The app was uninstalled.
uninstaller-done-except = The app was uninstalled, except for what is listed below.
uninstaller-waits-for-drive = Part of it is on a drive that is not connected. Connect it, and uninstall again from Settings.
uninstaller-not-all-undone = Not all of it could be undone. Close what is using it, and uninstall again from Settings.
# Over the path of the uninstall's log file.
uninstaller-log = Log
uninstaller-undone = Undone
uninstaller-left-in-place = Left in Place
uninstaller-could-not-be-undone = Could Not Be Undone
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = and { $count } more.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = this PC's ProgramData folder could not be found, so there is no record to read

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = WinAppMigrator Uninstaller
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = The uninstaller's window could not open: { $error }
uninstaller-window-failed-quiet = It can still uninstall without one, from an administrator's command prompt:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = this program

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Uninstalling { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Log: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = error: { $error }
