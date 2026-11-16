# Words of the installer that every built installer carries, which runs on the PC an app is moved to and speaks its
# language, and of the uninstaller it leaves behind for Settings. Message names start with installer- for the
# installer's window, installer-cli- for its command line, and uninstaller- for the uninstaller.
#
# The command lines (installer-cli-, uninstaller-cli-) are always shown in English, as scripts read what they print.
# Translate them all the same. Keep their command-line flags, such as --apply and --target-root, exactly as they are.


## The installer's window: its title and header

installer-window-title = Programma di installazione di WinAppMigrator
# The window's title in the taskbar. { $app_name } is the name of the app it installs.
installer-window-title-app = Installa { $app_name }
installer-tagline = Reinstalla un'app acquisita in un altro PC
# What an app is called when its package gives it no name.
installer-unnamed-app = Applicazione

## When the installer's window cannot open. Each is a paragraph of a message box.

# { $error } is what went wrong, as Windows or the graphics driver says it.
installer-window-failed = Impossibile aprire la finestra del programma di installazione: { $error }
installer-window-failed-without-one = È comunque possibile installare senza finestra: da un prompt dei comandi con diritti di amministratore, esegui questo file con --apply e, per scegliere la destinazione, con --target-root seguito da una cartella.

## Checking the installer when it opens

installer-checking = Controllo del programma di installazione…
installer-stub-no-package = Questo è uno stub del programma di installazione di WinAppMigrator e non contiene alcun pacchetto di app. Crea un programma di installazione con WinAppMigrator, quindi esegui qui il file .exe generato.
installer-check-stopped = Il controllo del programma di installazione si è interrotto.
# { $error } is what went wrong, as Windows says it.
installer-locate-failed = Impossibile individuare il programma di installazione: { $error }
# { $error } is what went wrong reading it.
installer-read-failed = Impossibile leggere il programma di installazione: { $error }
# { $error } is why, such as that the app needs a newer Windows.
installer-cannot-be-used-here = Impossibile usare il pacchetto di questo programma di installazione in questo PC: { $error }

## The app, where it goes, and whether the installer is signed

installer-captured-from = Acquisita da
installer-install-location = Percorso di installazione
# The title of the folder picker.
installer-choose-location = Scegli il percorso di installazione
installer-folder-has-files = Questa cartella contiene già dei file. I file con lo stesso nome vengono sostituiti e gli originali vengono ripristinati se disinstalli l'app.
installer-room-needed = Spazio necessario
# { $shortfall } says which drive is short of room, such as "C: needs up to 824.0 MB for the app's files, and has 300.0 MB free".
installer-room-short = Lo spazio qui potrebbe non essere sufficiente: { $shortfall }. Il pacchetto può essere decompresso in un'altra unità, ma i file dell'app devono comunque starci.
# { $app_name } is the name of an app whose earlier install was cut short.
installer-interrupted = Un'installazione precedente di { $app_name } non è mai stata completata. Facendo clic su Installa, verrà prima annullata.
# Beside "Installer", for whether it proves who made it.
installer-signature = Programma di installazione
installer-signature-checking = Verifica della firma…
installer-signature-unsigned = Non firmato: autore sconosciuto
# { $publisher } is the name on the installer's certificate, such as "Fabrikam, Inc.".
installer-signature-signed-by = Firmato da { $publisher }
installer-signature-unnamed = Firmato da un autore senza nome
# { $reason } is Windows' own word for what is wrong, such as HashMismatch.
installer-signature-invalid = La firma di questo programma di installazione non è valida ({ $reason }). Il programma potrebbe essere stato modificato dopo la firma.

## Why the install location cannot be used. Install and Preview Changes stay off while one of these is shown.

# { $app_name } is the name of the app being installed.
installer-location-app-already-there = L'app { $app_name } è già installata in questa cartella, quindi questa copia non può essere installata qui. Per installarla qui, disinstalla prima la copia presente in questo PC da Impostazioni di Windows, in App, quindi apri di nuovo questo programma di installazione. In alternativa, immetti qui sopra una cartella nuova e vuota.
# { $other_app } is the name of another app Windows lists as installed in the folder. { $app_name } is the name of the app being installed.
installer-location-other-app-there = L'app { $other_app } è installata in questa cartella, quindi { $app_name } non può essere installata qui. Immetti qui sopra una cartella nuova e vuota. Oppure, se { $other_app } non ti serve più, disinstallala prima da Impostazioni di Windows, in App, quindi apri di nuovo questo programma di installazione.
installer-location-has-files = Questa cartella contiene già file che non sono stati installati da WinAppMigrator. Potrebbero appartenere a un'altra app, quindi non verrà installato nulla al loro posto. Immetti qui sopra una cartella nuova e vuota.
# { $reason } is why, such as that it cannot be the Windows folder.
installer-location-refused = Impossibile usare questa cartella: { $reason }. Immetti qui sopra un'altra cartella.

## What the install sets up beyond its files

installer-beyond-files = Cosa configura oltre ai file
# Under the heading above. It is part of what a person agrees to when they click Install.
installer-beyond-files-agreed = Tutte queste operazioni vengono eseguite con diritti di amministratore. Se l'installazione non riesce, le modifiche apportate vengono ripristinate e ti viene segnalato ciò che non è stato possibile ripristinare. Puoi anche annullare l'installazione in seguito da Impostazioni, in App installate. I file sostituiti vengono ripristinati, mentre i runtime installati restano.
installer-beyond-files-none = Nulla: copia solo i file e le impostazioni nelle chiavi del Registro di sistema dell'app.
# { $problem } is why, such as that the app cannot run on this PC.
installer-cannot-go-here = Questo pacchetto non può essere installato in questo percorso: { $problem }
installer-working-out = Analisi di ciò che verrebbe configurato in questo percorso…
# Said as the reason after "This package cannot go to this location:".
installer-list-stopped = l'analisi di ciò che verrebbe eseguito si è interrotta
# A heading over a list of what else the package asked for and will not get. { $count } is how many.
installer-more-left-out =
    { $count ->
        [one] { $count } altro elemento escluso
       *[other] Altri { $count } elementi esclusi
    }

## Options

installer-options = Opzioni
installer-user-data = Dati utente
installer-user-data-detail = Attivato: le impostazioni, Documenti e Giochi salvati vanno nelle cartelle consuete dell'utente che ha eseguito l'accesso. Disattivato: tutto va nella cartella di installazione e non viene configurato nulla da eseguire da lì.
# { $account } is the name of the account on the other PC whose data this is.
installer-user-data-whose = Contiene i dati di { $account }.
installer-license = Licenza
installer-license-detail = File di attivazione e di licenza, e valori del Registro di sistema che contengono licenze. Alcuni potrebbero richiedere una nuova attivazione qui.

## Dependencies: what else the app needs, and how each is provided

installer-dependencies = Dipendenze
installer-dependencies-detail = Ciò di cui l'app ha bisogno e come viene fornito ciascun elemento.
installer-dependencies-none = Non sono state rilevate altre dipendenze di runtime.
# { $feature } is the name of an optional part of Windows, such as NetFx3.
installer-dependency-windows-feature = Funzionalità di Windows: { $feature }
installer-dependency-included = Incluso
# { $id } is the package's name in winget, such as Microsoft.VCRedist.2015+.x64.
installer-dependency-winget = Installato con winget: { $id }
installer-dependency-manual = Da installare manualmente
installer-dependency-built-in = Integrato in Windows
installer-dependency-ships-with-app = Fornito con l'app

## Buttons, and what is said beside them

installer-install = Installa
installer-preview = Anteprima modifiche
installer-cancel = Annulla
installer-close = Chiudi
installer-open-folder = Apri cartella di installazione
# { $error } is what went wrong, as Windows says it.
installer-open-folder-failed = Impossibile aprire la cartella: { $error }
installer-location-blocks = Installa e Anteprima modifiche restano disattivati finché non è possibile usare il percorso di installazione. Il motivo è indicato qui sopra, sotto il campo.
installer-package-blocks = Installa è disattivato perché questo pacchetto non può essere installato in questo percorso. Il motivo è indicato qui sopra.
installer-list-changed = Ciò che verrebbe eseguito qui è cambiato, quindi non è stato installato nulla. Controlla l'elenco qui sotto e, se è quello che vuoi, fai di nuovo clic su Installa.
installer-cancelled-early = Operazione annullata. Non è stato modificato nulla.

## How far an install has got, beside its progress bar

# { $step } is one of the steps below. { $done } and { $total } are how many of its items are done, and how many there are.
installer-step-counted = { $step }: { $done } di { $total }…
# { $step } is one of the steps below, when it does not count its items.
installer-step-uncounted = { $step }…
installer-step-getting-ready = Preparazione in corso…
installer-step-cancelling = Annullamento al termine del passaggio corrente…
installer-step-putting-back = Ripristino delle modifiche
installer-step-unpacking = Decompressione
installer-step-checking-package = Controllo del pacchetto
installer-step-recovering = Annullamento di un'installazione precedente mai completata
# What a preview is doing, whatever step it is on.
installer-step-previewing = Analisi di ciò che cambierebbe
installer-step-runtimes = Installazione dei runtime
installer-step-files = Copia dei file
installer-step-services = Configurazione dei servizi
installer-step-drivers = Configurazione dei driver
installer-step-registry = Scrittura delle impostazioni del Registro di sistema
installer-step-license = Ripristino dei dati di licenza
installer-step-tasks = Configurazione delle attività pianificate
installer-step-shortcuts = Creazione dei collegamenti
installer-step-environment = Impostazione delle variabili di ambiente
installer-step-settings-entry = Aggiunta dell'app a Impostazioni
installer-step-verifying = Controllo di ciò che è stato installato
installer-step-undoing = Annullamento

## When an install cannot go ahead or stops

installer-carries-no-package = Questo programma di installazione non contiene alcun pacchetto.
installer-stopped-unexpectedly = L'installazione si è interrotta in modo imprevisto.
# { $reason } is what went wrong, as the program says it.
installer-stopped-unexpectedly-because = L'installazione si è interrotta in modo imprevisto: { $reason }. Se era già iniziata, eseguendo di nuovo un programma di installazione verrà prima annullato ciò che aveva fatto.
# { $error } is what went wrong reading it.
installer-package-read-failed = Impossibile leggere il pacchetto: { $error }
installer-no-programdata = Impossibile trovare la cartella ProgramData di questo PC, quindi non è stato installato nulla: è lì che un'installazione conserva il registro che serve ad annullarla.
# { $error } is why, such as that there is nowhere only administrators can get into.
installer-staging-failed = Impossibile creare una cartella di gestione temporanea: { $error }
# { $error } is what went wrong unpacking it.
installer-unpack-failed = Impossibile decomprimere il pacchetto: { $error }
installer-staging-nowhere = non esiste un percorso accessibile solo agli amministratori in cui decomprimere il pacchetto
# { $failure } is a folder and what went wrong there, such as "C:\ProgramData\WinAppMigrator: Access is denied.".
installer-staging-nowhere-look-at = non esiste un percorso accessibile solo agli amministratori in cui decomprimere il pacchetto. Quello da controllare è { $failure }

## What came of an install, or of a preview

installer-result = Risultato dell'installazione
installer-result-preview = Anteprima: non è stato ancora modificato nulla
installer-result-installed = L'app è stata installata e ha superato il controllo hash.
installer-result-installed-without-left-out = L'app è stata installata e ha superato il controllo hash, senza gli elementi indicati come esclusi qui sotto.
installer-result-not-all-put-back = L'installazione non è riuscita e non è stato possibile ripristinare tutte le modifiche apportate. Ciò che rimane è elencato qui sotto; la prossima installazione riproverà.
installer-result-cancelled-put-back = L'installazione è stata annullata e le modifiche apportate sono state ripristinate.
installer-result-failed-put-back = L'installazione non è riuscita e le modifiche apportate sono state ripristinate.
installer-result-not-clean = L'installazione non è stata completata correttamente. Esamina i problemi qui sotto.
installer-result-changes-put-back = Modifiche ripristinate
installer-result-changes-made = Modifiche apportate
installer-result-changes-planned = Modifiche previste
installer-result-warnings = Avvisi
installer-result-errors = Errori
installer-result-package-valid = Hash del pacchetto validi
installer-result-package-invalid = Hash del pacchetto NON VALIDI
installer-result-files-valid = File installati validi
installer-result-files-invalid = File installati NON VALIDI
installer-result-reboot = Una dipendenza richiede un riavvio per essere completata. Riavvia Windows, quindi avvia l'app.
# A heading over the list of what was changed. { $count } is how many.
installer-result-changes = Modifiche ({ $count })
# After the first part of a long list. { $count } is how many more there are.
installer-result-changes-more = e altre { $count }.
# Over the path of the install's log file.
installer-log = Log

## The installer's command line. Shown in English: scripts read it. A { $count } is plain digits, with no separators.

installer-cli-stub = Stub del programma di installazione di WinAppMigrator: non contiene alcun pacchetto di app.
installer-cli-stub-how = Crea un programma di installazione con l'app WinAppMigrator, quindi esegui il file .exe generato nel computer di destinazione.
installer-cli-usage-title = Programma di installazione autoestraente di WinAppMigrator.
installer-cli-usage = Eseguilo senza argomenti per aprire il programma di installazione grafico, oppure:
installer-cli-usage-preview = anteprima della reinstallazione: --dry-run [--target-root <DIR>]
installer-cli-usage-apply = reinstallazione:                 --apply [--target-root <DIR>]
installer-cli-no-package = il programma di installazione non contiene un pacchetto incorporato
installer-cli-no-programdata = impossibile trovare la cartella ProgramData di questo PC, quindi non è stato installato nulla: è lì che un'installazione conserva il registro che serve ad annullarla
# { $path } is the folder asked for, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-folder-has-files = impossibile usare { $path }: { $problem }. Specifica --force solo se sai che appartiene a questa app
installer-cli-no-location = questo pacchetto non indica dove si trovava l'app, quindi specifica --target-root
# { $path } is where the app was on the other PC, such as C:\Program Files\App. { $problem } is why it cannot be used.
installer-cli-location-refused = l'app di questo pacchetto si trovava in { $path } e non è possibile usare quel percorso: { $problem }. Specifica --target-root
# { $summary } is how much room it needs where, such as "up to 824.0 MB on C: (356.0 GB free)".
installer-cli-room = Spazio: { $summary }
# { $note } is something to know about the install.
installer-cli-note = Nota: { $note }
# { $count } is how many things it sets up beyond its files.
installer-cli-sets-up = Elementi configurati: { $count }
installer-cli-applied = Reinstallazione applicata.
installer-cli-previewed = Anteprima della reinstallazione completata.
# { $count } is how many changes the install logged.
installer-cli-changes-logged = Modifiche registrate: { $count }
# { $count } is how many warnings.
installer-cli-warnings = Avvisi: { $count }
# { $count } is how many errors.
installer-cli-errors = Errori: { $count }
# { $count } is how many things the package asked for and did not get.
installer-cli-refused = Rifiutati: { $count }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-package-valid = Hash del pacchetto validi: { $valid }
# { $valid } is true or false, which scripts read, and is not translated.
installer-cli-reinstall-valid = Hash della reinstallazione validi: { $valid }
# { $id } is the install's id, such as wam-0f3a….
installer-cli-install-id = ID installazione: { $id }
installer-cli-put-back = L'installazione non è riuscita, quindi le modifiche apportate sono state ripristinate.
# { $path } is the log file.
installer-cli-log = Log: { $path }
# { $path } is the log file. Printed before an error.
installer-cli-error-log = log: { $path }
# { $error } is what went wrong.
installer-cli-error = errore: { $error }

## The uninstaller's window

uninstaller-window-title = Disinstalla
# The window's title, and its heading. { $app_name } is the name of the app it uninstalls.
uninstaller-window-title-app = Disinstalla { $app_name }
uninstaller-tagline = Annulla ciò che ha fatto il programma di installazione
uninstaller-reading = Lettura del registro di questa installazione…
uninstaller-stopped-unexpectedly = La disinstallazione si è interrotta in modo imprevisto.
uninstaller-what-will-be-removed = Elementi che verranno rimossi
uninstaller-install-location = Percorso di installazione
uninstaller-files = File
uninstaller-registry-values = Valori del Registro di sistema
uninstaller-services = Servizi
uninstaller-drivers = Driver
uninstaller-scheduled-tasks = Attività pianificate
uninstaller-environment = Variabili di ambiente
uninstaller-plan-stays = I file che hai aggiunto o modificato restano, insieme alle cartelle che li contengono.
# { $count } is how many files the install replaced.
uninstaller-plan-coming-back =
    { $count ->
        [one] { $count } file sostituito da questa installazione viene ripristinato oppure, se il file attuale è cambiato nel frattempo, viene messo accanto a quest'ultimo.
       *[other] { $count } file sostituiti da questa installazione vengono ripristinati oppure, se i file attuali sono cambiati nel frattempo, vengono messi accanto a questi ultimi.
    }
# { $runtimes } is a list of runtimes, such as "Microsoft Visual C++ Redistributable, .NET Desktop Runtime 8".
uninstaller-plan-runtimes = I runtime installati restano, perché altri programmi potrebbero usarli: { $runtimes }.
uninstaller-options = Opzioni
uninstaller-remove-user-data = Rimuovi anche i dati utente e i dati di licenza
# { $count } is how many items of the app's settings, data, and license there are.
uninstaller-remove-user-data-detail =
    { $count ->
        [one] { $count } elemento di impostazioni, dati e licenza. Resta a meno che questa opzione non sia attivata. Ciò che ha sostituito viene ripristinato oppure, se l'elemento è cambiato nel frattempo, viene messo accanto a esso.
       *[other] { $count } elementi di impostazioni, dati e licenza. Restano a meno che questa opzione non sia attivata. Ciò che hanno sostituito viene ripristinato oppure, se gli elementi sono cambiati nel frattempo, viene messo accanto a essi.
    }
uninstaller-uninstall = Disinstalla
uninstaller-cancel = Annulla
uninstaller-close = Chiudi
# { $done } and { $total } are how many of the install's items are undone, and how many there are.
uninstaller-step-counted = Annullamento dell'installazione: { $done } di { $total }…
uninstaller-step = Annullamento dell'installazione…
uninstaller-done = L'app è stata disinstallata.
uninstaller-done-except = L'app è stata disinstallata, ad eccezione di quanto elencato qui sotto.
uninstaller-waits-for-drive = Una parte si trova in un'unità non collegata. Collegala e disinstalla di nuovo da Impostazioni.
uninstaller-not-all-undone = Non è stato possibile annullare tutto. Chiudi i programmi che usano ciò che resta e disinstalla di nuovo da Impostazioni.
# Over the path of the uninstall's log file.
uninstaller-log = Log
uninstaller-undone = Annullati
uninstaller-left-in-place = Lasciati al loro posto
uninstaller-could-not-be-undone = Impossibile annullare
# After the first part of a long list. { $count } is how many more there are.
uninstaller-and-more = e altri { $count }.
# Said as the reason when there is no record of the install to read.
uninstaller-no-programdata = impossibile trovare la cartella ProgramData di questo PC, quindi non c'è alcun registro da leggere

## When the uninstaller's window cannot open. Each is a paragraph of a message box, followed by a command to type.

uninstaller-window-failed-title = Programma di disinstallazione di WinAppMigrator
# { $error } is what went wrong, as Windows or the graphics driver says it.
uninstaller-window-failed = Impossibile aprire la finestra del programma di disinstallazione: { $error }
uninstaller-window-failed-quiet = È comunque possibile disinstallare senza finestra, da un prompt dei comandi con diritti di amministratore:
# Stands in for this program's own path in that command, if it cannot be found.
uninstaller-this-program = questo programma

## The uninstaller's command line (--quiet). Shown in English: scripts read it.

# { $app_name } is the name of the app.
uninstaller-cli-uninstalling = Disinstallazione di { $app_name }.
# { $path } is the log file.
uninstaller-cli-log = Log: { $path }
# { $error } is what went wrong.
uninstaller-cli-error = errore: { $error }
