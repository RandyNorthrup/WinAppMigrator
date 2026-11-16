# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Diese Konsole kann { $language } nicht anzeigen, daher verwendet der Client Englisch.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client konnte nicht aus einem Ordner gestartet werden, den
    nur Administratoren ändern können: { $error }
client-press-enter-to-close = Drücken Sie die Eingabetaste, um dieses Fenster zu schließen.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Damit kann WinAppMigrator auf Ihrem neuen PC die Apps dieses PCs über Ihr
    Netzwerk lesen. Hier wird nichts geändert: Der Client liest eine
    Momentaufnahme des Windows-Laufwerks, die er entfernt, wenn Sie ihn beenden.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Der Client wurde beendet: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Wenn dieses Fenster geschlossen wird, statt den Client zu beenden, bleiben
    die Momentaufnahme und die Firewallregeln bestehen, bis der Client das
    nächste Mal hier ausgeführt wird ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Momentaufnahme des Windows-Laufwerks wird erstellt...{ " " }
client-snapshot-taken = fertig.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Dieser PC wird nicht automatisch gefunden ({ $error }).
    Geben Sie auf dem neuen PC stattdessen Namen oder Adresse dieses PCs ein.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Dieser PC:      { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:        { $windows_name }, { $bits }-Bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Kopplungscode:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Wählen Sie auf Ihrem neuen PC in WinAppMigrator Netzwerk-PC... aus, und
    geben Sie diesen Code ein. Schließen Sie vorher die App, die Sie übertragen,
    damit ihre Dateien so bleiben, wie die App sie hinterlassen hat.
    Zum Beenden drücken Sie hier die Eingabetaste oder schließen das Fenster.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Mit { $from ->
        [unknown] einem anderen PC
       *[address] { $from }
    } gekoppelt. Die benötigten Daten werden gelesen...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Ein anderer PC
       *[address] { $from }
    } ist fertig ({ $count ->
        [one] { $count } Anfrage
       *[other] { $count } Anfragen
    } beantwortet).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    Die Verbindung zu { $from ->
        [unknown] einem anderen PC
       *[address] { $from }
    } wurde beendet: { $error }
client-wrong-code =
    { $from ->
        [unknown] Ein anderer PC
       *[address] { $from }
    } hat einen falschen Code verwendet.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Ein anderer PC
       *[address] { $from }
    } konnte nicht gekoppelt werden: { $error }
client-too-many-wrong-codes =
    Es wurden zu viele falsche Codes versucht. Daher kann jetzt kein PC mehr
    mit diesem gekoppelt werden. Möglicherweise versucht etwas in Ihrem
    Netzwerk, den Code zu erraten. Beenden Sie zum Koppeln den Client, starten
    Sie ihn erneut, und geben Sie den neuen Code ein, den er anzeigt.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Zu viele falsche Codes. Der neue Kopplungscode lautet: { $code }

## Stopping

client-stopping = Wird beendet: Momentaufnahme und Firewallregeln werden entfernt...
client-stop-done = Fertig.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = der Client wird auf diesem PC bereits ausgeführt oder räumt nach einem geschlossenen Client auf. Verwenden Sie den laufenden Client, oder warten Sie einen Moment, und starten Sie diesen dann erneut.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows hat keine Momentaufnahme von { $drive } erstellt: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = die Windows-Firewall lässt nicht zu, dass der Client auf Verbindungen wartet: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client kennt diese Optionen:
    
      --lang <Sprache>  Diese Sprache verwenden, z. B. it-IT, sofern der Client
                        sie hat, statt der Anzeigesprache von Windows.
      --code <Code>     Mit diesem sechsstelligen Kopplungscode starten, für ein
                        Skript, das auch WinAppMigrator auf dem neuen PC startet.
      --help            Diese Hilfe anzeigen.
