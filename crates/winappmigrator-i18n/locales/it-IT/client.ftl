# Words of WinAppMigrator Client, the console program run on the old PC, the one an app is moving off, so that
# WinAppMigrator on the new PC can read it over the network. Message names start with client-.
#
# Its window is a console, which on Windows 7 is 80 columns wide. Each line break in a message is one on the screen:
# keep every line shorter than 80 characters, and break lines where your language needs to, not only where English
# does.


## Starting

# Shown only in English, in place of a right-to-left language the console cannot show. { $language } is that
# language's name in English, such as Arabic.
client-console-rtl-fallback = Questa console non supporta { $language }, quindi il client usa l'inglese.
# The client copies itself to a folder in Program Files before it starts. { $error } is what Windows said went wrong.
client-safe-folder-failed =
    Impossibile avviare WinAppMigrator Client da una cartella che solo gli
    amministratori possono modificare: { $error }
client-press-enter-to-close = Premi INVIO per chiudere questa finestra.
# The first line of its screen. { $version } is the client's version, such as 0.6.2.
client-heading = WinAppMigrator Client { $version }
client-intro =
    Consente a WinAppMigrator nel tuo nuovo PC di leggere le app di questo PC
    attraverso la rete. Qui non modifica nulla: legge uno snapshot dell'unità di
    Windows, che rimuove quando lo arresti.
# { $error } is why: one of the messages under "Why it stopped" below, or what Windows said.
client-stopped = Il client si è arrestato: { $error }
# The caretaker is a copy of the client with no window that cleans up after it. { $error } is what Windows said.
client-caretaker-failed =
    Se questa finestra viene chiusa anziché arrestare il client, lo snapshot e le
    regole del firewall restano fino alla prossima esecuzione del client in questo
    PC ({ $error }).
# Shown while the snapshot is taken, followed on the same line by client-snapshot-taken. Keep the quotes: they keep
# the space before it.
client-snapshot-taking = Creazione dello snapshot dell'unità di Windows...{ " " }
client-snapshot-taken = completata.
# The old PC answers the new one's search for it on the network. { $error } is what Windows said went wrong.
client-discovery-failed =
    Questo PC non verrà rilevato automaticamente ({ $error }).
    Nel nuovo PC digita invece il nome o l'indirizzo di questo PC.

## This PC and its pairing code. The three lines are lined up in two columns, behind two spaces; keep the quotes, which
## keep those spaces.

# { $computer_name } is this PC's name, such as OLD-PC.
client-this-pc = { "  " }Questo PC:               { $computer_name }
# { $windows_name } is this PC's Windows, such as Windows 7 Professional Service Pack 1. { $bits } is 32 or 64.
client-windows = { "  " }Windows:                 { $windows_name }, { $bits } bit
# { $code } is six digits in two groups, such as 482 913.
client-pairing-code = { "  " }Codice di associazione:  { $code }
# Network PC... is the button of that name in WinAppMigrator: use the name it has there in your language.
client-pairing-steps =
    Nel nuovo PC, in WinAppMigrator, scegli PC in rete... e digita questo codice.
    Chiudi prima l'app che stai spostando, così i suoi file restano come li ha
    lasciati. Per arrestare il client, premi INVIO qui o chiudi questa finestra.

## While it runs. { $from } is the network address of the PC that connected, such as 192.168.1.20, or "unknown" when
## it could not be told, which the [unknown] variant words.

client-paired =
    Associato a { $from ->
        [unknown] un altro PC
       *[address] { $from }
    }. Lettura di ciò che serve in corso...
# { $count } is how many requests it answered.
client-builder-done =
    { $from ->
        [unknown] Un altro PC
       *[address] { $from }
    } ha finito ({ $count ->
        [one] { $count } richiesta gestita
       *[other] { $count } richieste gestite
    }).
# { $error } is what went wrong, often in Windows' words.
client-link-ended =
    La connessione con { $from ->
        [unknown] un altro PC
       *[address] { $from }
    } si è interrotta: { $error }
client-wrong-code =
    { $from ->
        [unknown] Un altro PC
       *[address] { $from }
    } ha provato un codice diverso da questo.
# { $error } is what went wrong, often in Windows' words.
client-pair-failed =
    { $from ->
        [unknown] Un altro PC
       *[address] { $from }
    } non è riuscito ad associarsi: { $error }
client-too-many-wrong-codes =
    Sono stati provati troppi codici errati, quindi ora nessun PC può associarsi
    a questo. Qualcosa nella rete potrebbe tentare di indovinarlo. Per associare,
    arresta il client, avvialo di nuovo e digita il nuovo codice visualizzato.
# { $code } is six digits in two groups, such as 482 913.
client-new-code = Troppi codici errati. Il nuovo codice di associazione è: { $code }

## Stopping

client-stopping = Arresto in corso: rimozione dello snapshot e delle regole del firewall...
client-stop-done = Operazione completata.

## Why it stopped, shown after "The client stopped: " (client-stopped), and written to follow it.

client-already-running = il client è già in esecuzione in questo PC, oppure sta completando la pulizia dopo la chiusura di un'altra istanza. Usa quell'istanza oppure attendi un momento e avvia di nuovo questo client.
# { $drive } is the drive Windows is on, such as C:. { $details } is what Windows said, in its own words.
client-snapshot-failed = Windows non ha creato uno snapshot di { $drive }: { $details }
# { $details } is what Windows Firewall said, in its own words.
client-firewall-failed = Windows Firewall non ha consentito al client di rimanere in ascolto: { $details }

## What it takes, shown for --help. The options, such as --lang, are typed as they are and are not translated; keep
## their descriptions lined up.

client-usage =
    WinAppMigrator Client accetta queste opzioni:
    
      --lang <lingua>  Usa questa lingua, ad esempio it-IT, se il client la
                       include, anziché la lingua di visualizzazione di Windows.
      --code <codice>  Avvia con questo codice di associazione di sei cifre, per
                       uno script che avvia anche WinAppMigrator nel nuovo PC.
      --help           Mostra queste informazioni.
