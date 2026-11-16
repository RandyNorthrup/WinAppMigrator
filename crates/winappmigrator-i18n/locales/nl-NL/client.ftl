# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Deze console kan { $language } niet weergeven, dus de client gebruikt Engels.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    WinAppMigrator Client kan niet starten vanuit een map die alleen
    administrators kunnen wijzigen: { $error }
client-press-enter-to-close = Druk op Enter om dit venster te sluiten.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Hiermee kan WinAppMigrator op je nieuwe pc via je netwerk de apps op deze
    pc lezen. Hier verandert niets: de client leest een momentopname van het
    Windows-station, die wordt verwijderd wanneer je de client stopt.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = De client is gestopt: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Als dit venster wordt gesloten in plaats van gestopt, blijven de
    momentopname en de firewallregels staan totdat de client hier opnieuw
    wordt uitgevoerd ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Momentopname maken van het Windows-station...{ " " }
client-snapshot-taken = klaar.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Deze pc wordt niet vanzelf gevonden ({ $error }).
    Typ in plaats daarvan op de nieuwe pc de naam of het adres van deze pc.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Deze pc:       { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:       { $windows_name }, { $bits }-bits
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Koppelcode:    { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Kies in WinAppMigrator op je nieuwe pc Netwerk-pc... en typ deze code.
    Sluit eerst de app die je verplaatst, zodat de bestanden zo zijn als de app
    ze heeft achtergelaten.
    Druk hier op Enter of sluit dit venster om te stoppen.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Gekoppeld met { $from ->
        [unknown] een andere pc
       *[address] { $from }
    }. Wat nodig is, wordt nu gelezen...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Een andere pc
       *[address] { $from }
    } is klaar ({ $count ->
        [one] { $count } verzoek
       *[other] { $count } verzoeken
    } beantwoord).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    De verbinding met { $from ->
        [unknown] een andere pc
       *[address] { $from }
    } is verbroken: { $error }
client-wrong-code =
    { $from ->
        [unknown] Een andere pc
       *[address] { $from }
    } heeft een code geprobeerd die niet klopt.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Een andere pc
       *[address] { $from }
    } kon niet koppelen: { $error }
client-too-many-wrong-codes =
    Er zijn te veel verkeerde codes geprobeerd, dus geen enkele pc kan nu met
    deze pc koppelen. Mogelijk probeert iets in je netwerk de code te raden.
    Stop de client en start deze opnieuw om te koppelen, en typ de nieuwe code
    die dan wordt weergegeven.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Te veel verkeerde codes. De nieuwe koppelcode is: { $code }

## Stopping

client-stopping = Stoppen: de momentopname en de firewallregels worden verwijderd...
client-stop-done = Klaar.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = de client wordt al uitgevoerd op deze pc, of ruimt nog op na een client die is gesloten. Gebruik die, of wacht even en start deze opnieuw.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows heeft geen momentopname van { $drive } gemaakt: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows Firewall staat niet toe dat de client luistert: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client accepteert deze opties:
    
      --lang <taal>      Deze taal gebruiken, zoals it-IT, als de client die heeft,
                         in plaats van de weergavetaal van Windows.
      --code <code>      Starten met deze koppelcode van zes cijfers, voor een
                         script dat ook WinAppMigrator op de nieuwe pc start.
      --help             Dit weergeven.
