# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.

## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = This console cannot show { $language }, so the client speaks English.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client could not start from a folder only administrators
    can change: { $error }
client-press-enter-to-close = Press Enter to close this window.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    This lets WinAppMigrator on your new PC read the apps on this PC over your
    network. It changes nothing here: it reads a snapshot of the Windows drive,
    which it removes when you stop it.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = The client stopped: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    If this window is closed rather than stopped, the snapshot and firewall
    rules stay until the client is next run here ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Taking a snapshot of the Windows drive...{" "}
client-snapshot-taken = done.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    This PC will not show up by itself ({ $error }).
    On the new PC, type this PC's name or address instead.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = {"  "}This PC:       { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = {"  "}Windows:       { $windows_name }, { $bits }-bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = {"  "}Pairing Code:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    On your new PC, in WinAppMigrator, choose Network PC... and type this code.
    Close the app you are moving first, so its files are as it left them.
    To stop, press Enter here or close this window.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired = Paired with { $from ->
        [unknown] another PC
       *[address] { $from }
    }. It is reading what it needs...
# { $count } is how many requests it answered.
client-builder-done = { $from ->
        [unknown] another PC
       *[address] { $from }
    } is done ({ $count ->
        [one] { $count } request
       *[other] { $count } requests
    } answered).
# { $error } is what went wrong, often in Windows' words.
client-link-ended = The link to { $from ->
        [unknown] another PC
       *[address] { $from }
    } ended: { $error }
client-wrong-code = { $from ->
        [unknown] another PC
       *[address] { $from }
    } tried a code that is not this one.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed = { $from ->
        [unknown] another PC
       *[address] { $from }
    } could not pair: { $error }
client-too-many-wrong-codes =
    Too many wrong codes were tried, so no PC can pair with this one now.
    Something on your network may be guessing. To pair, stop the client
    and start it again, and type the new code it shows.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Too many wrong codes. The new pairing code is: { $code }

## Stopping

client-stopping = Stopping: removing the snapshot and the firewall rules...
client-stop-done = Done.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = the client is already running on this PC, or is cleaning up after one that was closed. Use that one, or wait a moment and start this again.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows did not make a snapshot of { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows Firewall would not let the client listen: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client takes these options:

      --lang <language>  Speak this language, such as it-IT, if the client has it,
                         rather than Windows' display language.
      --code <code>      Start with this six-digit pairing code, for a script that
                         starts WinAppMigrator on the new PC too.
      --help             Show this.
