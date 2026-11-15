# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.

## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = it is where installs keep their records
core-review-location-has-files = it already has files, and no install is on record here, so use an empty folder
core-policy-location-not-full-path = it must be a full path
core-policy-location-parent-step = it must not contain ".."
core-policy-location-not-plain = it has to be a plain path on a drive of this PC
core-policy-location-whole-drive = it cannot be a whole drive
core-policy-location-windows = it cannot be the Windows folder
core-policy-location-runs-by-itself = it is a folder that belongs to Windows, or whose contents run or load by themselves
core-policy-location-shared = it holds other things side by side, so the app needs a folder inside it

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = this app cannot run on this PC, so nothing was changed: { $reason }
core-policy-architecture-x86 = 32-bit x86
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = 32-bit Arm
core-policy-architecture-unknown = an unknown architecture
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = machine type { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names }, and { $count } more
core-policy-fit-programs-not-said = This installer does not say what its programs need, so whether they can run on this PC was not checked.
core-policy-fit-architecture-not-told = Which architecture this PC is could not be told, so whether this app's programs can run on it was not checked.
core-policy-fit-main-not-said = This installer does not say what its main program needs, so what is said here goes by the rest of what it has.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = its driver { $driver } is built for { $built_for }, and this PC is { $here }. A driver has to match Windows exactly, so this app cannot run here
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } is built for { $built_for }, and this PC is { $here }, so Windows would not start it
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = none of its programs can start on this PC, which is { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Some of this app's programs are built for another architecture and will not start on this PC, which is { $here }: { $programs }. The rest of the app can, so this may not matter.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = This PC is { $here }, so Windows would run these through an emulator: { $programs }. Most programs are fine that way, and some are not.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } says it needs Windows { $needs } or newer, and this PC runs { $runs }, so Windows would not start it
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Some of this app's programs say they need a newer Windows than this and will not start here: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = This app was on Windows build { $from_build }, and this PC runs build { $this_build }. Its programs say they can start on this Windows, so it may work, but what it relied on there may not be here.
core-policy-fit-build-not-told = Which build of Windows this is could not be told, so it was not compared with the one this app was on.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = unpacking the package
core-space-for-install-record = the record of the install
core-space-for-app-files = the app's files
core-space-for-what-is-replaced = keeping what it replaces
core-space-for-build-work = what a build works with
core-space-for-installer = the installer
core-space-for-package = the package
core-space-for-unpacking-and-installing = unpacking the package and installing it
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = {" and "}
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } needs up to { $needed } free for { $purposes }, and has { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } would not say how much room it has, so whether the { $needed } it needs fits was not checked
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = up to { $needed } on { $drive } ({ $free } free)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = up to { $needed } on { $drive } (how much is free could not be told)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = {". "}
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = there is not enough room for this, so nothing was changed. { $shortfalls }. Free up some space there, or choose an install location on another drive.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Service
core-review-kind-driver = Driver
core-review-kind-scheduled-task = Scheduled Task
core-review-kind-autostart = Starts Automatically
core-review-kind-shortcut = Shortcut
core-review-kind-path = PATH
core-review-kind-environment-variable = Environment Variable
core-review-kind-shell-extension = Shell Extension
core-review-kind-menu-command = Menu Command
core-review-kind-link-type = Link Type
core-review-kind-file-type = File Type
core-review-kind-com = COM
core-review-kind-firewall-rule = Firewall Rule
core-review-kind-event-log-source = Event Log Source
core-review-kind-font = Font
core-review-kind-odbc-driver = ODBC Driver
core-review-kind-odbc-source = ODBC Data Source
core-review-kind-compatibility-setting = Compatibility Setting
core-review-kind-default-apps = Default Apps
core-review-kind-program-name = Program Name
core-review-kind-other-settings = Other Settings

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail = { $starts ->
        [automatic] Runs { $file } as { $account }, starts now and each time this PC starts
        [delayed] Runs { $file } as { $account }, starts now and soon after each time this PC starts
        [manual] Runs { $file } as { $account }, starts only when something asks it to
        [disabled] Runs { $file } as { $account }, is disabled
        [other] Runs { $file } as { $account }, starts { $start }
       *[unknown] Runs { $file } as { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (custom account not restored)
# { $file } is a driver's file name.
core-review-driver-detail = Loads { $file } into Windows itself
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Loads { $file } into Windows itself, starts { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = with Windows, before anything else
core-review-start-system = with Windows, as it starts
core-review-start-automatic = each time this PC starts
core-review-start-delayed = soon after each time this PC starts
core-review-start-manual = only when something asks it to
core-review-start-disabled = never, as it is disabled
core-review-start-unknown = in a way Windows does not say
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Runs as { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Runs as { $account }, with the highest rights it can have
core-review-task-runs-as-you = you, whoever is signed in
core-review-task-runs-as-creator = the user who created it
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail = Runs { $scope ->
        [everyone] for everyone who signs in
       *[you] when you sign in
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail = A shortcut in the Startup folder, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail = Opens { $file }, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Adds to { $name }
core-review-path-install-folder = The install folder
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = The install folder\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Set to { $value }
# { $count } is how many registry values.
core-review-other-settings-title = { $count ->
        [one] { $count } setting outside the app's own keys
       *[other] { $count } settings outside the app's own keys
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = In { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = In { $keys }, and { $count } more
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = domain
core-review-network-private = private
core-review-network-public = public
core-review-firewall-detail = { $way ->
        [allow-in] Allows incoming
        [allow-out] Allows outgoing
        [block-in] Blocks incoming
       *[block-out] Blocks outgoing
    } { $port_count ->
        [0] connections
       *[other] connections on { $ports }
    } for { $program }, on { $network_count ->
        [0] every network
       *[other] { $networks } networks
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail = Loads { $server } into Explorer, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file = Loads a file it installs into Explorer, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } on { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail = Adds a command to the menu of { $on }, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail = Changes what opening one of these does, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
core-review-client-entry-detail = Can be chosen as the program for this. Nothing is changed to it
core-review-com-permissions-title = COM permissions
core-review-com-permissions-detail = Says who may start its COM classes, or what they run as
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail = Links of this kind open with the app, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail = Files of this type open with the app, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Can write to the { $log } log, with messages from its own files
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail = Loads from the install folder when you sign in, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
core-review-odbc-driver-detail = Loads into programs that use ODBC to reach a database
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail = A database connection, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
core-review-default-apps-detail = Can be chosen in Default apps. Nothing is changed to it
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail = Run and other programs can find it by this name, { $scope ->
        [everyone] for everyone on this PC
       *[you] for you
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM in { $server }
core-review-com-title-own-file = COM in a file it installs
# { $count } is how many registry values.
core-review-com-detail = { $count ->
        [one] { $count } setting of the classes other programs can start
       *[other] { $count } settings of the classes other programs can start
    }
core-review-com-support-title = COM type libraries, interfaces, and AppIDs
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are of each.
core-review-com-support-detail = { $libraries } and { $interfaces } and { $app_ids } settings of each, which go with its classes
core-review-compatibility-title = How its programs start
# { $count } is how many registry values.
core-review-compatibility-detail = { $count ->
        [one] { $count } compatibility setting for programs it installs
       *[other] { $count } compatibility settings for programs it installs
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = User data and per-user settings go to { $account }, who is signed in here, and not to the administrator this is running as.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = No one is signed in to this session, so user data and per-user settings go to { $account }, the account this is running as.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Could not tell who is signed in ({ $reason }), so per-user settings are left out, and user data goes under the install location.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = could not tell who is signed in: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Not in this installer, because they look like credentials
core-review-left-out-settings = Settings and history an account keeps for the app, which only go in with the User Data option
core-review-left-out-certificates = The app seems to rely on certificates that were trusted on its PC, and they are not installed
core-review-left-out-fonts = These fonts go by the app's name, and nothing says they are its own, so they are not installed
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Not in this installer, because they look like credentials: { $names }. What is on this PC of them stays as it is, and you may have to enter them again.
# { $names } is a list of names of license data.
core-review-note-licenses = Not in this installer, because the License option was off when it was built: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Not in this installer, because the User Data option was off when it was built: the settings and history an account keeps for the app, in { $names }. The app starts here as it would for someone new to it.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = The app seems to rely on certificates that were trusted on its PC, and they are not installed: { $names }. Whether to trust them here is for you to decide.
# { $names } is a list of font names.
core-review-note-fonts = These fonts go by the app's name, and nothing says they are its own, so they are not installed: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Left out by whoever built this installer: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Added by whoever built this installer, because they took it to be part of the app: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } in { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names }, and { $count } more

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = it belongs to one account's settings, and who is signed in could not be told
core-review-name-not-accepted = Windows would not accept its name
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = its current settings could not be read: { $error }
core-review-service-dependency = it depends on a service with an invalid name
# { $path } is the program the service runs.
core-review-service-runs-other = it runs { $path }, which this package does not install
core-review-service-settings = its settings name something this package does not install, or a key that is not its own
core-review-driver-no-hash = it has no hash to check it by
core-review-license-for-the-machine = a license stored for the whole machine is only reported
core-review-license-starts-something = a license has no reason to start anything
core-review-task-not-captured = its definition was not captured
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = its definition could not be read: { $error }
core-review-task-no-account = it runs as whoever installs it, and who is signed in here could not be told
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = it opens { $path }, which this package does not install
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } already exists on this PC and runs { $path }, which is not from an earlier install here. If it is an older copy of this app, uninstall that first
core-policy-key-control-characters = its key name holds control characters
core-policy-value-control-characters = its value's name holds control characters
core-policy-registry-link = it is a registry link
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = it stands for a key of the PC's, and { $reason }
core-policy-registry-part = no app install should write to this part of the registry
core-policy-key-refused = no app install should change this key
core-policy-net-debugger = it would name a debugger for every .NET program
core-policy-net-install-root = it would change where every .NET program loads .NET from
core-policy-class-redirects = it would change how a kind of program or folder is opened
core-policy-app-path-name = it names a program by something other than a file's name
core-policy-app-path-not-installed = it would register a program name for a file this package does not install
core-policy-shared-default-verb = it would change what opening every file or folder of a kind does
core-policy-command-runs-other = it adds a command that runs something this package does not install
core-policy-autostart-not-command = it starts automatically, and its value is not a command
core-policy-autostart-runs-other = it starts automatically, and runs something this package does not install
core-policy-environment-own-rules = environment variables are restored by their own rules
core-policy-service-not-restored = it belongs to a service this package does not restore
core-policy-service-own-rules = a service's own settings are only restored through its rules
core-policy-service-dll = its ServiceDll is not a file this package installs
core-policy-event-log-source-only = only an event log source's own values can be restored
core-policy-security-log = no app writes to the Security log
core-policy-event-log-messages = its messages would come from a file this package does not install
core-policy-type-library = its type library is not a file this package installs
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = it would change how { $name } is opened, which this package does not install
core-policy-event-log-no-file = an event log source has to take its messages from a file
core-policy-com-runs-other = it runs something that is not a file this package installs
core-policy-com-redirect = it would make this class something this package does not install
core-policy-com-server = it runs something that is neither a file this package installs nor part of Windows
core-policy-interface = it would decide how calls to an interface are made, with something this package does not install
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = it would change how { $name } is started, which this package does not install
core-policy-app-id-surrogate = it would be hosted by a program this package does not install
core-policy-app-id-service = it would run in a service this package does not restore
core-policy-app-id-run-as = it would run as an account of its own
core-policy-app-id-link = it does not name an AppID
core-policy-app-id-remote = it would run somewhere other than this PC
core-policy-odbc-own-driver = that is ODBC's own setting and not a driver
core-policy-odbc-driver-not-installed = its driver is not a file this package installs
core-policy-loads-other-file = it would load a file this package does not install
core-policy-odbc-own-source = that is ODBC's own setting and not a data source
core-policy-odbc-source-missing = it lists a data source this package does not have
core-policy-odbc-source-driver = its driver is neither a file this package installs nor part of Windows
core-policy-shell-extension = it would load a shell extension that is not a file this package installs into Explorer
core-policy-not-text = what it holds is not the text it would be read as
core-policy-network-place = it names a place on the network
core-policy-font = it would register a font this package does not install, or one in the Windows folder
core-policy-compatibility = it would change how a program runs that this package does not install
core-policy-capabilities = it lists an app whose details are not in this package
core-policy-default-program = it would change which program opens things by default
core-policy-url-protocol = it says it is a link type, and opens with nothing this package installs
core-policy-command-handler = it adds a command handled by something this package does not install
core-policy-installer-command = it would run what only Windows Installer can read
core-policy-creates-class = it would create a class this package does not install
core-policy-opens-links = it opens links with something this package does not install
core-policy-opens-files = it opens files with something this package does not install
core-policy-firewall-name = its name is not one this can pass on
core-policy-firewall-description = its description is not one this can pass on
core-policy-firewall-every-program = it applies to every program, and a rule can only be for one
# { $path } is the program the rule is for.
core-policy-firewall-program = it is for { $path }, which this package does not install
core-policy-firewall-direction = it does not say a direction this can set
core-policy-firewall-action = it would override other rules, which no app's rule should
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = it says { $setting }, which could not be set as it was, or would weaken what other rules do
core-policy-firewall-service = it is for a service this package does not restore
core-policy-firewall-parts = part of it is not what a firewall rule can say
core-policy-folder-already-there = that folder was already there, outside the install location, and nothing says it is this app's, so no program goes in it
core-policy-variable-name = its name is not a valid variable name
core-policy-variable-relied-on = Windows and other programs rely on this variable
core-policy-task-name = its name is not one Windows accepts
core-policy-task-folder = its folder is not one Windows accepts
core-policy-task-windows = it would replace one of Windows' own tasks
core-policy-task-other-action = it does something other than run a program
core-policy-task-no-program = it names no program to run
# { $path } is the program the task runs.
core-policy-task-runs-other = it runs something this package does not install: { $path }
core-policy-task-exists-unreadable = a task with this name already exists and could not be read
core-policy-task-exists-other = a task with this name already exists and does not run the app
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = a task with this name already exists and runs { $path }, which is not from an earlier install here
core-policy-shortcut-not-lnk = it is not a .lnk shortcut
core-policy-shortcut-place = it is not in a Start Menu or on a Desktop
core-policy-short-name = its path uses a name that cannot be checked
core-policy-shortcut-no-folder = this PC has no such folder for it
core-policy-data-startup = it would go in a Startup folder, and run at every sign-in
core-policy-data-powershell = it would go where PowerShell runs what it finds when it starts
core-policy-data-windows = it would go in a folder that belongs to Windows
core-policy-license-outside = it is outside the install location and every known data folder
core-rewrite-cdata = it holds a CDATA section
core-rewrite-too-much-to-look-through = this package holds too much in its settings for what they name to be checked

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out = { $kind ->
        [file] The file { $item } was left out: { $reason }.
        [folder] The folder { $item } was left out: { $reason }.
        [service] The service { $item } was left out: { $reason }.
        [driver] The driver { $item } was left out: { $reason }.
        [firewall-rule] The firewall rule { $item } was left out: { $reason }.
        [license-data] The license data at { $item } was left out: { $reason }.
        [license-value] The license value { $item } was left out: { $reason }.
        [scheduled-task] The scheduled task { $item } was left out: { $reason }.
        [shortcut] The shortcut { $item } was left out: { $reason }.
        [environment-variable] The environment variable { $item } was left out: { $reason }.
       *[value] { $item } was left out: { $reason }.
    }
core-warning-review-package-path-missing = A copyable file has no package relative path.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = This package lists more than { $count } empty folders, and only that many are made.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere = { $count ->
        [one] { $count } file from Documents goes under { $target } instead, in a folder named _external, because this PC's Documents folder is at { $folder }, and only a folder inside your profile is written to. To use it, move it to your Documents folder yourself.
       *[other] { $count } files from Documents go under { $target } instead, in a folder named _external, because this PC's Documents folder is at { $folder }, and only a folder inside your profile is written to. To use them, move them to your Documents folder yourself.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown = { $count ->
        [one] { $count } file from Documents goes under { $target } instead, in a folder named _external, because where this PC's Documents folder is could not be told. To use it, move it to your Documents folder yourself.
       *[other] { $count } files from Documents go under { $target } instead, in a folder named _external, because where this PC's Documents folder is could not be told. To use them, move them to your Documents folder yourself.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere = { $count ->
        [one] { $count } file from Saved Games goes under { $target } instead, in a folder named _external, because this PC's Saved Games folder is at { $folder }, and only a folder inside your profile is written to. To use it, move it to your Saved Games folder yourself.
       *[other] { $count } files from Saved Games go under { $target } instead, in a folder named _external, because this PC's Saved Games folder is at { $folder }, and only a folder inside your profile is written to. To use them, move them to your Saved Games folder yourself.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown = { $count ->
        [one] { $count } file from Saved Games goes under { $target } instead, in a folder named _external, because where this PC's Saved Games folder is could not be told. To use it, move it to your Saved Games folder yourself.
       *[other] { $count } files from Saved Games go under { $target } instead, in a folder named _external, because where this PC's Saved Games folder is could not be told. To use them, move them to your Saved Games folder yourself.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } is a link or a file, not a folder
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = { $path } was not made by WinAppMigrator, and others can change what is in it. An administrator should check it and remove it: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = { $path } has something in it that was not made by WinAppMigrator, and others can change it. An administrator should check it and remove it
# { $path } is a folder.
core-privilege-became-a-link-in-use = { $path } became a link while it was being used
# { $path } is a folder.
core-privilege-became-a-link-looked-at = { $path } became a link while it was being looked at
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = could not let users start { $path }: { $said }
core-privilege-elevation-unknown = could not tell whether this process is elevated
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = could not restrict { $path } to administrators: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = { $path } could not be restricted to administrators, and its drive may not keep permissions: { $said }
# { $path } is a folder.
core-privilege-written-to-first = another process wrote to { $path } before it could be protected
core-privilege-no-program-data = Windows does not say where ProgramData is

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } has no package in it
# { $path } is a folder.
core-verify-not-unpacked = this installer's package was not unpacked to { $path }
core-verify-schema-supported = Package manifest schema is supported.
core-verify-packaged-file-missing = Packaged file is missing.
core-verify-no-package-path = Copyable file has no package relative path.
core-verify-folder-there = Empty folder is there.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = It could not be read to check it: { $error }
core-verify-restored-file-missing = Restored file is missing.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = File size mismatch. Expected { $expected } bytes, found { $actual } bytes.
core-verify-no-hash = The manifest records no hash for this file, so it cannot be checked.
core-verify-hash-mismatch = SHA-256 hash mismatch.
core-verify-file-matches = File size and hash match manifest.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = License payload verification failed: { $error }
core-verify-license-payload-missing = Packaged license payload is missing.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = License store verification failed: { $error }
core-verify-license-store-file-missing = Packaged license store file is missing.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Packaged license store path is invalid: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Prerequisite payload verification failed: { $error }
core-verify-prerequisite-missing = Bundled prerequisite payload is missing.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Driver image verification failed: { $error }
core-verify-bundled-driver-missing = Bundled driver image is missing.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Bundled driver path is invalid: { $error }
core-verify-prerequisite-present = Runtime prerequisite is present on the target.
core-verify-prerequisite-not-detected = Runtime prerequisite not detected; it may require a reboot or manual completion (see the restore log for the authoritative outcome).
core-verify-prerequisite-not-probed = Prerequisite presence is not registry-probable; install was best-effort.
core-verify-license-file-missing = Restored license file is missing.
core-verify-license-store-restored-missing = Restored license store file is missing.
core-verify-registry-key-there = Registry key exists.
core-verify-registry-value-matches = Registry value matches archived data.
core-verify-registry-value-differs = Registry value does not match archived data.
core-verify-firewall-rule-there = Windows Firewall has this rule.
core-verify-variable-matches = Environment variable value matches archived data.
core-verify-variable-differs = Environment variable value does not match archived data.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = Environment variable is missing or unreadable: { $error }
core-verify-service-runs-other = Service exists but does not run the restored binary with its arguments.
core-verify-service-other-account = Service exists but does not run as the expected account.
core-verify-service-matches = Service exists, and runs the restored binary with its arguments and account.
core-verify-service-missing = Service is missing.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Service is unreadable: { $error }
core-verify-driver-other-image = Driver service does not point at the restored image.
core-verify-driver-not-registered = Bundled driver service is not registered after restore.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Driver service is unreadable: { $error }
core-verify-driver-image-missing = Driver image is missing.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Driver image could not be checked: { $error }
core-verify-task-matches = Scheduled task exists and runs the restored programs.
core-verify-task-runs-other = Scheduled task exists but does not run the restored programs.
core-verify-task-missing = Scheduled task is missing.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Scheduled task query failed: { $error }
core-verify-shortcut-matches = Shortcut exists and target path matches restored target.
core-verify-shortcut-opens-other = Shortcut exists but target path does not match restored target.
core-verify-shortcut-missing = Shortcut is missing.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Shortcut query failed: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Package manifest schema is unsupported.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Package file verification failed: { $error }
core-warning-verify-folder-missing = Empty folder is missing.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = Registry key is missing or unreadable: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Registry value is missing or unreadable: { $error }
core-warning-verify-registry-kind-missing = Registry value kind is missing.
core-warning-verify-firewall-rule-missing = Windows Firewall does not have this rule.
core-warning-verify-service-scope = Service verification reads HKLM service configuration and does not start services.
core-warning-verify-driver-manual = One or more vendor/inbox drivers are not redistributed by WinAppMigrator; reinstall them with the vendor's signed driver package on the target.
core-warning-verify-task-scope = Scheduled task verification checks registration and command reference only.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = WinAppMigrator Verification Report
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Verified UTC: { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Package valid: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Restore valid: { $valid }
core-verify-report-restore-not-run = Restore valid: not run
core-verify-report-items = Items
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Source: { $source }
core-verify-report-warnings = Warnings
core-verify-report-errors = Errors
core-verify-report-none = None.
