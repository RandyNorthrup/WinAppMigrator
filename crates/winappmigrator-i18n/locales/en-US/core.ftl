# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.

## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable = { $count ->
        [one] { $count } shortcut on this drive names no file, so whether it opens this app could not be told.
       *[other] { $count } shortcuts on this drive name no file, so whether they open this app could not be told.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Part of the record of { $app_name } could not be read, so anything it described is still in place, and a file it replaced may not come back.
core-rollback-settings-entry-stays = The app's entry in Settings stays, so that this can be run again from there.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } was installed, or may have been, and stays, because other programs may be using it.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = What this install put in { $folder } is on a drive that is not connected. Connect it, and this can be finished.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = What this install put in { $folder } stays, because that part of the record could not be read.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = something was changing { $folder } while this ran, so nothing more was done in it. Close what is doing that, and run this again
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = { $folder } could not be opened: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = there is no record of where { $folder } was
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } now leads to { $actual }, not where it was during the install
# Why a service was not stopped.
core-rollback-reason-did-not-stop = it did not stop in time

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = The folder { $path } was left: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Removed the folder { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = The folder { $path } stays, because it holds files this install did not put there.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = The folder { $path } could not be removed: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = The file { $path } was left: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } is not being put back, because the folder it was under has gone.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = A partial file could not be removed: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = { $path } could not be checked: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Removed { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = { $path } could not be removed: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Put back { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = { $path } could not be put back: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = the backup has gone
# { $path } is a file.
core-rollback-no-parent-folder = { $path } has no parent folder

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays = { $why ->
        [still-used] { $path } stays, because something that stays still uses it.
        [changed] { $path } stays, because it has changed since it was installed.
       *[not-asked] { $path } stays, since it was not asked to be removed.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside = { $why ->
        [still-used] { $path } stays, because something that stays still uses it, and the file it replaced could not be put beside it: every name for it is taken.
        [changed] { $path } stays, because it has changed since it was installed, and the file it replaced could not be put beside it: every name for it is taken.
       *[not-asked] { $path } stays, since it was not asked to be removed, and the file it replaced could not be put beside it: every name for it is taken.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside = { $why ->
        [still-used] { $path } stays, because something that stays still uses it. The file it replaced is beside it, as { $name }.
        [changed] { $path } stays, because it has changed since it was installed. The file it replaced is beside it, as { $name }.
       *[not-asked] { $path } stays, since it was not asked to be removed. The file it replaced is beside it, as { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside = { $why ->
        [still-used] { $path } stays, because something that stays still uses it, and the file it replaced could not be put beside it: { $error }.
        [changed] { $path } stays, because it has changed since it was installed, and the file it replaced could not be put beside it: { $error }.
       *[not-asked] { $path } stays, since it was not asked to be removed, and the file it replaced could not be put beside it: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Default)
core-rollback-setting-other-account = { $kind ->
        [key] The key { $name } stays, because it belongs to the account that installed this, which is not signed in.
        [user-variable] The user's environment variable { $name } stays, because it belongs to the account that installed this, which is not signed in.
        [machine-variable] The machine's environment variable { $name } stays, because it belongs to the account that installed this, which is not signed in.
        [user-list] The user's { $name } stays, because it belongs to the account that installed this, which is not signed in.
        [machine-list] The machine's { $name } stays, because it belongs to the account that installed this, which is not signed in.
       *[value] The registry value { $name } stays, because it belongs to the account that installed this, which is not signed in.
    }
core-rollback-setting-not-reached = { $kind ->
        [key] The key { $name } could not be reached: { $error }.
        [user-variable] The user's environment variable { $name } could not be reached: { $error }.
        [machine-variable] The machine's environment variable { $name } could not be reached: { $error }.
        [user-list] The user's { $name } could not be reached: { $error }.
        [machine-list] The machine's { $name } could not be reached: { $error }.
       *[value] The registry value { $name } could not be reached: { $error }.
    }
core-rollback-setting-not-read = { $kind ->
        [key] The key { $name } could not be read: { $error }.
        [user-variable] The user's environment variable { $name } could not be read: { $error }.
        [machine-variable] The machine's environment variable { $name } could not be read: { $error }.
        [user-list] The user's { $name } could not be read: { $error }.
        [machine-list] The machine's { $name } could not be read: { $error }.
       *[value] The registry value { $name } could not be read: { $error }.
    }
core-rollback-setting-changed = { $kind ->
        [user-variable] The user's environment variable { $name } stays, because it has changed since it was installed.
        [machine-variable] The machine's environment variable { $name } stays, because it has changed since it was installed.
        [user-list] The user's { $name } stays, because it has changed since it was installed.
        [machine-list] The machine's { $name } stays, because it has changed since it was installed.
       *[value] The registry value { $name } stays, because it has changed since it was installed.
    }
# Said when it is back as it was before the install. The capital "The" is as the app has always said it.
core-rollback-setting-put-back = { $kind ->
        [user-variable] Put back The user's environment variable { $name }
        [machine-variable] Put back The machine's environment variable { $name }
        [user-list] Put back The user's { $name }
        [machine-list] Put back The machine's { $name }
       *[value] Put back The registry value { $name }
    }
core-rollback-setting-not-put-back = { $kind ->
        [user-variable] The user's environment variable { $name } could not be put back: { $error }.
        [machine-variable] The machine's environment variable { $name } could not be put back: { $error }.
        [user-list] The user's { $name } could not be put back: { $error }.
        [machine-list] The machine's { $name } could not be put back: { $error }.
       *[value] The registry value { $name } could not be put back: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = The registry value { $name } stays, since it was not asked to be removed.
core-rollback-key-holds-others = The key { $name } stays, because it holds what this install did not put there.
core-rollback-key-removed = Removed the key { $name }
core-rollback-key-not-removed = The key { $name } could not be removed: { $error }.
# { $kind } is user-list or machine-list. The capital "The" is as the app has always said it.
core-rollback-list-entries-removed = { $kind ->
        [user-list] Took this install's entries out of The user's { $name }
       *[machine-list] Took this install's entries out of The machine's { $name }
    }
core-rollback-list-entries-not-removed = { $kind ->
        [user-list] This install's entries could not be taken out of The user's { $name }: { $error }.
       *[machine-list] This install's entries could not be taken out of The machine's { $name }: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read = { $kind ->
        [driver] The driver { $name } could not be read: { $error }.
       *[service] The service { $name } could not be read: { $error }.
    }
core-rollback-service-changed = { $kind ->
        [driver] The driver { $name } stays, because it has changed since it was installed.
       *[service] The service { $name } stays, because it has changed since it was installed.
    }
core-rollback-service-not-stopped = { $kind ->
        [driver] The driver { $name } could not be stopped ({ $reason }), so what it is using may stay until the PC restarts.
       *[service] The service { $name } could not be stopped ({ $reason }), so what it is using may stay until the PC restarts.
    }
core-rollback-service-stopped-for-this = { $kind ->
        [driver] The driver { $name } was stopped to do this. Start it again when you need it.
       *[service] The service { $name } was stopped to do this. Start it again when you need it.
    }
core-rollback-service-not-removed = { $kind ->
        [driver] The driver { $name } could not be removed: { $reason }.
       *[service] The service { $name } could not be removed: { $reason }.
    }
core-rollback-service-marked-for-removal = { $kind ->
        [driver] The driver { $name } is marked for removal, and goes when the PC restarts or whatever has it open lets go.
       *[service] The service { $name } is marked for removal, and goes when the PC restarts or whatever has it open lets go.
    }
core-rollback-service-removed = { $kind ->
        [driver] Removed the driver { $name }
       *[service] Removed the service { $name }
    }
core-rollback-service-not-restored = { $kind ->
        [driver] The driver { $name } could not be put back as it was: { $reason }.
       *[service] The service { $name } could not be put back as it was: { $reason }.
    }
core-rollback-service-restored = { $kind ->
        [driver] Put back the settings of the driver { $name }
       *[service] Put back the settings of the service { $name }
    }
core-rollback-service-not-restarted = The service { $name } was running before this install, and did not start again: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = The service { $name } ran as { $account }. Its password is not kept, so set its account again in Services.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = The firewall rule { $name } could not be read: { $error }.
core-rollback-firewall-says-otherwise = The firewall rule { $name } stays, because what is there by that name says something else.
core-rollback-firewall-changed = The firewall rule { $name } stays, because it has changed since it was installed.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Removed The firewall rule { $name }
core-rollback-firewall-not-removed = The firewall rule { $name } could not be removed: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = The task folder { $path } stays, because it holds tasks this install did not put there.
core-rollback-task-folder-not-removed = The task folder { $path } could not be removed: { $error }.
core-rollback-task-not-read = The task { $name } could not be read: { $error }.
core-rollback-task-changed = The task { $name } stays, because it has changed since it was installed.
core-rollback-task-removed = Removed the task { $name }
core-rollback-task-not-removed = The task { $name } could not be removed: { $reason }.
core-rollback-task-put-back = Put back the task { $name }
core-rollback-task-not-put-back = The task { $name } could not be put back: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = The record of this install could not be read, so nothing was undone: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Nothing was undone: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } If it can never be undone, an administrator can give up on it by removing { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = an earlier install that never finished could not be fully undone, so nothing was installed. Close whatever is using these and try again. { $details }
core-rollback-install-busy = this install is being changed by another installer or uninstaller

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Undone: { $count }
core-rollback-count-failed = Could not be undone: { $count }
core-rollback-count-kept = Left in place: { $count }
core-rollback-count-left-behind = Left behind: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32-bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64-bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP or older

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = reading a connected drive or another account's settings needs administrator rights
core-system-no-program-data = this PC's ProgramData folder could not be found
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = not a hive file: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } is { $size } bytes, too large to be a registry hive, or more than this drive's hives could add up to
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = offline hive has no mapping for HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } failed: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } failed: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } is not a Windows system drive (no \Windows folder).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } is this PC's own system drive. Choose This PC to read this PC.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } is not a Windows system drive: the way to its registry goes through a link.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = The account { $account } is not on record here, so no one's per-user data or settings were included.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = The account signed in here, { $account }, has no profile on record, so no one's per-user data or settings were included.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Could not tell who is signed in ({ $reason }), so no one's per-user data or settings were included. Choose whose to include.
core-warning-scan-last-account-unknown = This drive does not say who last signed in, and it has more than one account, so no one's per-user data or settings were included. Choose whose to include.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = The settings of { $account } could not be read, so they were not included: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = The COM classes and file types registered for { $account } could not be read, so they were not included: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed = { $count ->
        [one] A path recorded on this drive could not be read, so nothing there was included. Each is on another disk, is not a full path, or goes through a link, which would lead off the drive: { $paths }
       *[other] Some paths recorded on this drive could not be read, so nothing there was included. Each is on another disk, is not a full path, or goes through a link, which would lead off the drive: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Some paths recorded on this drive could not be read, so nothing there was included. Each is on another disk, is not a full path, or goes through a link, which would lead off the drive: { $paths } and { $more } more
core-warning-scan-network-paths-not-followed = { $count ->
        [one] A path recorded on this PC could not be read, so nothing there was included. Each is not on a disk of this PC, where looking at it would connect to it, or has a name that would open something else: { $paths }
       *[other] Some paths recorded on this PC could not be read, so nothing there was included. Each is not on a disk of this PC, where looking at it would connect to it, or has a name that would open something else: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Some paths recorded on this PC could not be read, so nothing there was included. Each is not on a disk of this PC, where looking at it would connect to it, or has a name that would open something else: { $paths } and { $more } more

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Shortcuts in this account's own folders
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why = { $count ->
        [one] { $count } shortcut, which only goes with the User Data option
       *[other] { $count } shortcuts, which only go with the User Data option
    }
core-account-left-out-variables = Environment variables of this account's
# { $count } is how many variables.
core-account-left-out-variables-why = { $count ->
        [one] { $count } variable, which only goes with the User Data option
       *[other] { $count } variables, which only go with the User Data option
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = The scheduled task { $task }
core-account-task-works-on-account-folder = It works on a place in an account's folders, or is named for one, and only goes with the User Data option
core-account-task-runs-from-account-folder = What it also runs from an account's folders only goes with the User Data option
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = The service { $service }
core-account-service-works-on-account-folder = It works on a place in an account's folders, and only goes with the User Data option

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = the paths of the app's own files
core-account-part-registry = registry values
core-account-part-settings-entry = the app's entry in Settings
core-account-part-main-program = where its main program is
core-account-part-shortcuts = shortcuts
core-account-part-tasks = scheduled tasks
core-account-part-services = services
core-account-part-variables = environment variables
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions = { $kind ->
        [account] This package mentions an account's name in { $places }. It was built with none of an account's data, and anyone who has the installer can read this.
        [folder] This package mentions where an account's folder is in { $places }. It was built with none of an account's data, and anyone who has the installer can read this.
        [computer] This package mentions the PC's name in { $places }. It was built with none of an account's data, and anyone who has the installer can read this.
       *[id] This package mentions an account's id in { $places }. It was built with none of an account's data, and anyone who has the installer can read this.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable = { $count ->
        [one] { $count } folder outside the app could not be read, so data of the app's there may be missing.
       *[other] { $count } folders outside the app could not be read, so data of the app's there may be missing.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = it looks like a credential
core-secret-why-license-option-off = the License option is off
# { $count } is how many values of a registry key.
core-secret-why-account-values = { $count ->
        [one] { $count } value, which only goes with the User Data option
       *[other] { $count } values, which only go with the User Data option
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (a value whose name holds a credential)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = an argument holding a credential
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from = { $place ->
        [service] { $option } in how { $name } is run
        [task] { $option } in how the task { $name } is run
        [shortcut] { $option } in the shortcut { $name }
        [opener] { $option } in what opens { $name }
        [opener-unnamed] { $option } in what opens a file type
       *[uninstall] { $option } in how { $name } is uninstalled
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = The definition of the task { $task }, which could not be read to check it
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = The environment variable { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = What the class { $class } says it runs
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = The description of the firewall rule { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = License file detected: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = { $framework } license store present
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = License environment variable { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Registry value suggests licensing data: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Registry value suggests licensing data
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Registry key suggests licensing data: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Licensing/activation data was detected. With the License option on, the installer restores what can be moved; machine-wide values and hardware-bound stores are only reported. WinAppMigrator never decrypts, bypasses activation, or emulates dongles.
core-warning-scan-license-machine-bound = Some licenses are machine-bound (hardware dongle, machine-locked activation, or encrypted). The installer restores what it can; these may still require reactivation on the new PC.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Could not read App Paths child key: { $error }
core-warning-scan-registry-limit = Registry capture reached the configured safety limit.
core-warning-scan-registry-subtree-limit = Registry subtree capture reached the configured safety limit.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Could not read registry key: { $error }
core-warning-scan-registry-value-too-large = Registry value exceeded the configured capture size limit.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = The folder of scheduled tasks on this drive goes through a link, so its tasks were not read.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = The scheduled task { $task } was left out: it runs as another account than the one whose settings are being included, others than administrators can change its file, or the app has too many to keep.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Could not read part of the scheduled tasks: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Could not read the scheduled task { $path }: { $reason }. If it runs the app, it was not included.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = it is { $size } bytes, too large to be a task
# Why a task's file could not be read.
core-task-ends-mid-character = it ends part-way through a character
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = it is not valid UTF-16: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = it is not valid UTF-8 or UTF-16: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Restore uses Register-ScheduledTask with the archived XML definition.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = it could not be read: { $reason }
core-pe-headers-misaligned = its headers are not on the boundaries a program's have to be
# { $size } is the file's size in bytes.
core-pe-too-large = it is { $size } bytes, too large to be a program
# PE is the name of the format Windows programs are in.
core-pe-no-headers = it has no PE headers

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Import table could not be read: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = An import descriptor name could not be read: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports = { $count ->
        [one] { $count } legacy (v1) delay-load descriptor was skipped; only modern RVA-based delay imports are parsed.
       *[other] { $count } legacy (v1) delay-load descriptors were skipped; only modern RVA-based delay imports are parsed.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = executable has no parent directory: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL located beside selected executable
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Could not scan recursive dependency: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } was in the Windows folder of the PC this app was on, and Windows 10 and 11 do not come with it. Something installed it there, so it is not part of the app's own files and does not come along. The app may not start until whatever provides it is installed.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Could not hash dependency: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = SystemRoot/windir environment variables are not available; system DLL search is limited.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Could not read dependency architecture: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Could not read services registry key: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Could not read services registry key for driver discovery: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Could not query shortcuts: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Could not parse shortcut query output: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Could not read machine environment registry key: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled = { $signed ->
        [yes] Application-owned { $kind } driver '{ $name }' ({ $path }, signed by { $signer }). The installer bundles its .sys and re-registers the driver service automatically (sc create, requires elevation), then confirms the service is registered.
       *[no] Application-owned { $kind } driver '{ $name }' ({ $path }, signature unknown). The installer bundles its .sys and re-registers the driver service automatically (sc create, requires elevation), then confirms the service is registered.
    }
core-service-driver-from-vendor = { $signed ->
        [yes] Vendor/inbox { $kind } driver '{ $name }' ({ $path }, signed by { $signer }). WinAppMigrator does not redistribute third-party drivers; reinstall it with the vendor's signed driver package on the target before launching the app.
       *[no] Vendor/inbox { $kind } driver '{ $name }' ({ $path }, signature unknown). WinAppMigrator does not redistribute third-party drivers; reinstall it with the vendor's signed driver package on the target before launching the app.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Built into Windows
core-runtime-handling-ships-with-app = Ships with the App
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Windows Feature: { $feature }
core-runtime-handling-included = Included
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Installed with winget: { $id }
core-runtime-handling-manual = Install Manually

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = refusing to download { $url }: it is not HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = TLS init failed: { $error }
core-runtime-download-failed = download { $url } failed: { $error }
core-runtime-download-unreadable = reading { $url } failed: { $error }
core-runtime-download-empty = download { $url } returned no data
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = download { $url } was cut short: { $came } of { $expected } bytes came

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = there is no known installer for this runtime
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = the package names { $path } as this runtime's installer, which is not where it belongs, so it was not run
# { $path } is a file.
core-runtime-payload-missing = bundled installer is missing: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = bundled installer path is invalid: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = no bundled installer or winget id; acquire it from { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = could not make a folder for it to unpack in: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = the bundled installer failed its signature check and was not run: { $reason }
core-runtime-payload-timed-out = bundled installer did not finish within the timeout and was terminated
# { $error } is what went wrong.
core-runtime-payload-not-started = could not start bundled installer: { $error }
core-runtime-installer-killed = installer was terminated without an exit code
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = installer exited with code { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM could not enable the { $feature } Windows feature (exit { $code }); the payload likely could not be downloaded. Enable it via Windows Features, or run `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM could not enable the { $feature } Windows feature (exit unknown); the payload likely could not be downloaded. Enable it via Windows Features, or run `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = enabling the { $feature } Windows feature did not finish within the timeout and was terminated
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM is not available to enable the { $feature } Windows feature: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = could not look for winget: { $error }
core-runtime-winget-not-installed = winget is not installed
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } is not where Windows keeps its own packages, so it was not run
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } failed its signature check and was not run: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = could not use winget to install { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } exited with { $code }
core-runtime-winget-failed-no-code = winget install { $id } exited with an unknown code
core-runtime-winget-timed-out = winget install { $id } did not finish within the timeout and was terminated
# { $error } is what went wrong.
core-runtime-winget-not-started = winget is not available to install { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } was not used: others than administrators can change it
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle = { $count ->
        [one] Burn bundle of { $count } Windows Installer package
       *[other] Burn bundle of { $count } Windows Installer packages
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database = { $count ->
        [one] Windows Installer database: { $count } component key path
       *[other] Windows Installer database: { $count } component key paths
    }
core-source-inno-log = Inno Setup uninstall log (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths = { $count ->
        [one] Inno Setup uninstall log: { $count } recorded path
       *[other] Inno Setup uninstall log: { $count } recorded paths
    }
core-source-nsis = NSIS (Nullsoft) uninstaller signature
# { $count } is how many paths the log records.
core-source-nsis-log-paths = { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } recorded path
       *[other] NSIS AdvUninstLog uninstall.log: { $count } recorded paths
    }
core-source-squirrel = Squirrel per-user layout (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Squirrel current version: { $version }
core-source-chocolatey = Chocolatey package metadata
core-source-msix = MSIX / Appx package
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = AppxManifest identity: { $identity }
# { $count } is how many folders.
core-source-msix-data = { $count ->
        [one] MSIX per-user package data: { $count } location
       *[other] MSIX per-user package data: { $count } locations
    }
# App Paths is a registry key's name.
core-source-app-paths = App Paths registration

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } imports { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = A font the app's install record names
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = it is named for the app or who published it
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = the app's install record names it

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows would not say whose desktop this is: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = Windows could not be asked whose desktop this is: { $error }
core-profile-signed-in-account-no-profile = the account signed in here, { $account }, has no profile on record
core-profile-desktop-not-a-person = the desktop here belongs to { $account }, which is not a person's account
core-profile-nobody-signed-in-running-account-no-profile = no one is signed in here, and the account running this, { $account }, has no profile on record
core-profile-nobody-signed-in-running-not-a-person = no one is signed in here, and { $account }, which is running this, is not a person's account
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = no one is signed in here, and this account has no SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = more than one account has a desktop in this session: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Could not tell who is signed in ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Could not tell who is signed in ({ $reason }). Choose whose data to include.
