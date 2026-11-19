# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.

## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = what this install would do has changed since it was agreed to, so nothing was installed
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = I/O failed for { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = registry read failed: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = PE parse failed for { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = invalid input: { $detail }
# { $detail } is what went wrong.
core-error-serialization = report serialization failed: { $detail }
# { $detail } is what went wrong.
core-error-archive = archive operation failed: { $detail }
# { $detail } is what went wrong.
core-error-network = network operation failed: { $detail }
# { $detail } is what went wrong.
core-error-time-format = time formatting failed: { $detail }
core-error-cancelled = it was cancelled
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = no reason was given

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = unknown registry value type { $kind }
core-journal-not-elevated = not elevated
core-journal-needs-admin = installing needs administrator rights, to keep its record where others cannot change it
# { $path } is a folder.
core-journal-not-an-install-folder = this is not an install's folder: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = the record of this install could not be written: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = path escapes intended root: { $path }
# { $path } is a folder.
core-journal-nothing-exists = none of this exists on this PC: { $path }
core-journal-busy = another install or uninstall is running. Wait for it to finish, then try again
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = it has no folder
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = what was kept could not be read
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = the record of the install in { $folder } cannot be used: it is empty
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = the record of the install in { $folder } cannot be used: it does not start with what it is about
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = the record of the install in { $folder } cannot be used: a different version of this program wrote it
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = the record of the install in { $folder } cannot be used: it says it belongs to a different install
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = this is not an install's id: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = --apply and --dry-run cannot both be supplied
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = the install location must be a full path: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = that install location cannot be used: { $problem }: "{ $path }"
core-restore-needs-admin-plan = restore plan includes admin-only registry, service, scheduled task, shortcut, or protected target changes
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = target root requires elevated privileges: { $path }
core-restore-needs-admin-user-data = restoring user data into ProgramData or other protected locations requires elevated privileges
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. What had been done could not all be put back: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = there is nowhere safe to keep the record that would let this install be undone, so nothing was installed: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } already exists, so nothing was installed. Use --force to replace it
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = this app has things to restore to { $root }, and { $link } is a link to somewhere else, which others may be able to change, so nothing was installed. If what goes there is user data or license data, install it without its user data and license data. If it is a shortcut in a Startup folder, put that folder back where it was
core-restore-hash-check-failed = the package failed its hash check and nothing was installed
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = the package failed its hash check and nothing was installed: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = refusing to write where installs keep their records: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = refusing to write outside the install location and the known data folders: { $path }
# { $path } is a file.
core-restore-no-parent-folder = destination has no parent folder: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = a folder is where this file has to go: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } is not what the package holds any more, so it was not installed
# { $path } is a file in the package.
core-restore-source-is-link = { $path } is a link, and it is not to be followed
# { $path } is a file in the package.
core-restore-source-became-link = { $path } became a link while it was being opened
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = it stopped again by itself
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = this COM class is already on this PC, as something else's

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = post-restore verification package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = started service { $name }
core-restore-would-copy = would copy { $source } -> { $destination }
core-restore-copied = copied { $source } -> { $destination }
core-restore-would-create-empty-folder = would create the empty folder { $path }
core-restore-created-empty-folder = created the empty folder { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = would restore registry { $key } { $value }
core-restore-would-restore-registry-key = would restore registry { $key } (key)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = would add firewall rule { $name } for { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = firewall rule { $name } is already there
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = added firewall rule { $name } for { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = would restore service { $name } -> { $command } (runs as { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = prerequisite already present: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = would install prerequisite { $name } (Windows feature)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = would install prerequisite { $name } (bundled installer)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = would install prerequisite { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = installed prerequisite { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = installed prerequisite { $name } (reboot required)
core-restore-would-restore-license = would restore license { $source } -> { $destination }
core-restore-restored-license = restored license { $source } -> { $destination }
core-restore-would-restore-license-registry = would restore license registry { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = driver not auto-restored: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = would install driver { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = would restore scheduled task { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = would restore startup shortcut { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = would restore shortcut { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable = { $scope ->
        [user] would restore User environment variable { $name }
       *[machine] would restore Machine environment variable { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable = { $scope ->
        [user] left User environment variable { $name } as it is: none of it belongs to this app
       *[machine] left Machine environment variable { $name } as it is: none of it belongs to this app
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable = { $scope ->
        [user] restored User environment variable { $name }
       *[machine] restored Machine environment variable { $name }
    }
core-restore-set-uninstaller = set the uninstaller for { $key }
core-restore-placed-driver-image = placed driver image { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = configured driver { $name }
# { $name } is a driver's name.
core-restore-created-driver = created driver { $name }
# { $name } is a driver's name.
core-restore-started-driver = started driver { $name }
core-restore-ensured-registry-key = ensured registry key { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = restored registry { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = restored registry { $key } (Default)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = configured service { $name } (runs as { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = created service { $name } (runs as { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = restored scheduled task { $name }
core-restore-restored-shortcut = restored shortcut { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = A record of an install that never finished was left alone, because others can change it. Whatever that install changed is still there.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone = { $count ->
        [one] An earlier install of { $app_name } never finished, so it was undone first. { $count } change was put back.
       *[other] An earlier install of { $app_name } never finished, so it was undone first. { $count } changes were put back.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = An earlier install of { $app_name } never finished. Installing undoes it first.
core-warning-restore-user-data-real-locations = User data is restored to this machine's real %AppData%/%ProgramData% locations instead of the target root.
# { $error } is what went wrong.
core-warning-restore-commit-failed = The record of this install could not be finished: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = The service { $name } was asked to start and has not finished starting. It may yet. It also starts by itself when this PC next starts.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = The service { $name } is installed and did not start: { $reason }. The app may not work until it does. Windows tries again when this PC next starts.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = The service { $name } could not be stopped first: { $reason }. It may go on running the old program until this PC next starts.
core-warning-restore-install-cancelled = The install was cancelled.
core-warning-restore-preview-cancelled = The preview was cancelled.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } was left out: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Post-restore verification could not run: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run = { $runner ->
        [service] { $path } is not this package's to replace, and the service { $name } relies on it, so nothing was installed: { $refusal }
        [task] { $path } is not this package's to replace, and the scheduled task { $name } relies on it, so nothing was installed: { $refusal }
        [firewall] { $path } is not this package's to replace, and the firewall rule { $name } relies on it, so nothing was installed: { $refusal }
        [shortcut] { $path } is not this package's to replace, and the shortcut { $name } relies on it, so nothing was installed: { $refusal }
       *[registry] { $path } is not this package's to replace, and the registry value { $name } relies on it, so nothing was installed: { $refusal }
    }
core-warning-restore-destination-exists = Destination exists; use --force to overwrite.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Could not write { $path }: { $error }
core-warning-restore-not-a-folder = Something other than a folder is where this empty folder goes.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Could not look at { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Could not make { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = this PC already has this, set to something else, which an install does not take over
core-warning-restore-registry-review = Registry values were restored from the archived structured data with known app paths rewritten to the target root.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Windows Firewall's rules could not be read: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = a rule of this name for this program is already there, set to something else, which an install does not change
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: there is no known installer for this runtime.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = Installing { $name } requires elevation; re-run the installer as administrator.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } was installed but a reboot is required to complete it.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Could not record a runtime that was skipped: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Could not install { $name }: { $reason }. Acquire it from { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Could not record an installed runtime: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Packaged license path is invalid: { $error }
core-warning-restore-license-needs-admin = Restoring license data to a protected location requires elevation.
core-warning-restore-license-exists = License destination exists; use --force to overwrite.
# { $error } is what went wrong.
core-warning-restore-license-failed = Could not restore license data: { $error }
core-warning-restore-driver-needs-admin = Driver registration requires elevation; re-run the installer as administrator.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = The scheduled task { $name } runs as SYSTEM, with full rights over this PC.
core-warning-restore-shortcut-exists = Shortcut exists; use --force to overwrite.
core-warning-restore-shortcut-all-users = One or more restored shortcuts target a public desktop or common Start Menu location.
core-warning-restore-environment-refresh = Environment variable changes may require a new process, shell, or sign-in session to be observed.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = The service { $name } ran as { $account } on the source. Its password is not in the installer, so the service is set up as LocalService. If it does not work, set its account and password in Services.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } is not yet registered; it should be active after a reboot.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } reported success but did not register; the install may have failed.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Bundled driver path is invalid: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Could not read the driver's current settings: { $error }
core-warning-restore-driver-exists = Driver service exists; use --force to reconfigure it.
core-warning-restore-driver-image-exists = Driver image exists; use --force to overwrite.
core-warning-restore-driver-hash-bundled = The bundled driver image does not match its recorded hash, so it was not installed.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Could not place driver image: { $error }
core-warning-restore-driver-hash-placed = The driver image changed after it was placed, so it was not registered.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Could not create registry key: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Could not read the value this would replace: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Could not write registry value: { $error }
core-warning-restore-value-exists = Registry value exists; use --force to overwrite.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Could not create environment registry key: { $error }
core-warning-restore-variable-exists = Environment variable exists; use --force to overwrite.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Could not write environment variable: { $error }
core-warning-restore-value-kind-missing = Registry value kind is missing.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Could not read the service's current settings: { $error }
core-warning-restore-service-exists = Service exists; use --force to reconfigure it.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Failed to query scheduled task: { $error }
core-warning-restore-task-exists = Scheduled task exists; use --force to overwrite.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Could not look at the task's folder: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = The shortcut could not be made, and the one it replaced could not be put back: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Failed to start command: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Command failed: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Failed to start PowerShell: { $error }
