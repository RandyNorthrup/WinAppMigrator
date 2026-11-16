# Words the core writes for people as it installs an app from a package: why an install will not start, the list of
# what it did (or for a preview, what it would do) that the installer window shows, its warnings, the record it keeps of
# each install so that it can be undone, and the core's errors. Message names start with core-.
#
# The list of what an install did is short and plain, one line for each thing, as the installer window lists them:
# "copied", "restored", "would copy". Words such as --force and --apply are command-line options, and LocalService,
# SYSTEM, ProgramData, and %AppData% are as Windows names them: keep them as they are.


## Errors: what went wrong, when the core cannot go on. Each is shown on its own, or after words that say what was being
## done.

core-error-list-changed = ciò che questa installazione farebbe è cambiato dopo l'approvazione, quindi non è stato installato nulla
# { $path } is a file or folder, such as C:\Program Files\App\app.exe, or words that name one, such as "the installer".
# { $error } is what Windows said went wrong.
core-error-io = operazione di I/O non riuscita per { $path }: { $error }
# { $detail } is what went wrong.
core-error-registry = lettura del Registro di sistema non riuscita: { $detail }
# { $path } is a program file. { $detail } is what was wrong with it.
core-error-pe = analisi PE non riuscita per { $path }: { $detail }
# { $detail } is what was wrong, already in words.
core-error-invalid-input = input non valido: { $detail }
# { $detail } is what went wrong.
core-error-serialization = serializzazione del report non riuscita: { $detail }
# { $detail } is what went wrong.
core-error-archive = operazione sull'archivio non riuscita: { $detail }
# { $detail } is what went wrong.
core-error-network = operazione di rete non riuscita: { $detail }
# { $detail } is what went wrong.
core-error-time-format = formattazione dell'ora non riuscita: { $detail }
core-error-cancelled = l'operazione è stata annullata
# What is said when part of the program stopped unexpectedly and gave no reason.
core-error-panic-no-reason = non è stato indicato alcun motivo

## The record of each install, kept so that it can be undone.

# { $kind } is a number that says what type of registry value it is.
core-journal-unknown-value-type = tipo di valore del Registro di sistema sconosciuto: { $kind }
core-journal-not-elevated = senza privilegi elevati
core-journal-needs-admin = l'installazione richiede diritti di amministratore, per conservare il registro dove altri non possono modificarlo
# { $path } is a folder.
core-journal-not-an-install-folder = questa non è la cartella di un'installazione: { $path }
# { $path } is the file the record is kept in.
core-journal-not-written = impossibile scrivere il registro di questa installazione: { $path }
# { $path } is a folder.
core-journal-path-escapes-root = il percorso esce dalla radice prevista: { $path }
# { $path } is a folder.
core-journal-nothing-exists = nessuno di questi elementi esiste in questo PC: { $path }
core-journal-busy = è in corso un'altra installazione o disinstallazione. Attendi che termini, quindi riprova
# Said of a file that was to get back the permissions it had.
core-journal-no-folder = non ha una cartella
# Said of the permissions a file had, which were kept to give back to it.
core-journal-permissions-unreadable = impossibile leggere ciò che era stato conservato
# { $folder } is the folder of one install's record.
core-journal-unreadable-empty = impossibile usare il registro dell'installazione in { $folder }: è vuoto
# { $folder } is the folder of one install's record.
core-journal-unreadable-no-header = impossibile usare il registro dell'installazione in { $folder }: non inizia con l'indicazione del suo contenuto
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-version = impossibile usare il registro dell'installazione in { $folder }: è stato scritto da un'altra versione di questo programma
# { $folder } is the folder of one install's record.
core-journal-unreadable-other-install = impossibile usare il registro dell'installazione in { $folder }: indica di appartenere a un'altra installazione
# { $id } is an install's id, such as wam-1a2b3c.
core-journal-not-an-install-id = questo non è l'ID di un'installazione: { $id }

## Why an install will not start, or stopped before it changed anything.

core-restore-apply-and-dry-run = non è possibile specificare sia --apply sia --dry-run
# { $path } is the folder that was chosen to install in.
core-restore-location-not-full-path = il percorso di installazione deve essere un percorso completo: "{ $path }"
# { $problem } says what is wrong with the folder, already in words. { $path } is the folder.
core-restore-location-unusable = impossibile usare il percorso di installazione: { $problem }: "{ $path }"
core-restore-needs-admin-plan = il piano di ripristino include modifiche riservate agli amministratori al Registro di sistema, ai servizi, alle attività pianificate, ai collegamenti o a destinazioni protette
# { $path } is the folder that was chosen to install in.
core-restore-needs-admin-target = la radice di destinazione richiede privilegi elevati: { $path }
core-restore-needs-admin-user-data = il ripristino dei dati utente in ProgramData o in altri percorsi protetti richiede privilegi elevati
# { $error } is why the install stopped. { $failed } is what could not be put back, as sentences of their own.
core-restore-not-all-put-back = { $error }. Non è stato possibile ripristinare tutto ciò che era stato fatto: { $failed }
# { $error } is what went wrong.
core-restore-nowhere-to-keep-record = non esiste un percorso sicuro in cui conservare il registro che consentirebbe di annullare questa installazione, quindi non è stato installato nulla: { $error }
# { $path } is a file the app would have put there.
core-restore-already-exists = { $path } esiste già, quindi non è stato installato nulla. Usa --force per sostituirlo
# { $root } is a folder the app has files for. { $link } is that folder, or one it is in.
core-restore-root-under-link = questa app ha elementi da ripristinare in { $root } e { $link } è un collegamento simbolico che punta altrove e che altri potrebbero modificare, quindi non è stato installato nulla. Se ciò che va lì sono dati utente o dati di licenza, installa l'app senza i dati utente e i dati di licenza. Se è un collegamento nella cartella Esecuzione automatica, riporta quella cartella nella posizione originale
core-restore-hash-check-failed = il pacchetto non ha superato il controllo hash e non è stato installato nulla
# { $detail } is the first thing found wrong, already in words.
core-restore-hash-check-failed-because = il pacchetto non ha superato il controllo hash e non è stato installato nulla: { $detail }
# { $path } is a file or folder.
core-restore-refuse-records-folder = scrittura rifiutata nel percorso in cui le installazioni conservano i registri: { $path }
# { $path } is a file or folder.
core-restore-refuse-outside = scrittura rifiutata all'esterno del percorso di installazione e delle cartelle di dati note: { $path }
# { $path } is a file.
core-restore-no-parent-folder = la destinazione non ha una cartella padre: { $path }
# { $path } is where a file was to go.
core-restore-folder-in-the-way = nella posizione di questo file si trova una cartella: { $path }
# { $path } is a file in the package.
core-restore-file-changed = { $path } non corrisponde più al contenuto del pacchetto, quindi non è stato installato
# { $path } is a file in the package.
core-restore-source-is-link = { $path } è un collegamento simbolico e non deve essere seguito
# { $path } is a file in the package.
core-restore-source-became-link = { $path } è diventato un collegamento simbolico durante l'apertura
# Why a service did not start, after "did not start:".
core-restore-service-stopped-by-itself = si è arrestato di nuovo da solo
# Why part of the app was left out, after "was left out:".
core-restore-com-class-someone-elses = questa classe COM esiste già in questo PC e appartiene ad altro

## What an install did, one line for each thing, or for a preview, what it would do. { $source } is a file in the
## package, and { $destination } and { $path } are where it goes on this PC. { $key } is a registry key, such as
## HKLM\Software\App.

# { $package_valid } is true or false, and { $restore_valid } is true, false, or not_run: keep them as they are.
core-restore-verification-result = verifica post-ripristino package_valid={ $package_valid } restore_valid={ $restore_valid }
# { $name } is a service's name.
core-restore-started-service = avviato il servizio { $name }
core-restore-would-copy = verrebbe copiato { $source } -> { $destination }
core-restore-copied = copiato { $source } -> { $destination }
core-restore-would-create-empty-folder = verrebbe creata la cartella vuota { $path }
core-restore-created-empty-folder = creata la cartella vuota { $path }
# { $value } is the name of a value in the key.
core-restore-would-restore-registry-value = verrebbe ripristinato nel Registro di sistema { $key } { $value }
core-restore-would-restore-registry-key = verrebbe ripristinato nel Registro di sistema { $key } (chiave)
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-would-add-firewall-rule = verrebbe aggiunta la regola del firewall { $name } per { $program }
# { $name } is a firewall rule's name.
core-restore-firewall-rule-already-there = la regola del firewall { $name } è già presente
# { $name } is a firewall rule's name. { $program } is the program it is for.
core-restore-added-firewall-rule = aggiunta la regola del firewall { $name } per { $program }
# { $name } is a service's name, { $command } what it runs, and { $account } the account it runs as, such as LocalService.
core-restore-would-restore-service = verrebbe ripristinato il servizio { $name } -> { $command } (eseguito come { $account })
# { $name } is a runtime, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-restore-prerequisite-present = prerequisito già presente: { $name }
# { $name } is a runtime that Windows has as a Windows feature.
core-restore-would-install-prerequisite-feature = verrebbe installato il prerequisito { $name } (funzionalità di Windows)
# { $name } is a runtime whose installer is in the package.
core-restore-would-install-prerequisite-bundled = verrebbe installato il prerequisito { $name } (programma di installazione incluso)
# { $name } is a runtime that would come through winget, which is a program's name.
core-restore-would-install-prerequisite-winget = verrebbe installato il prerequisito { $name } (winget)
# { $name } is a runtime.
core-restore-installed-prerequisite = installato il prerequisito { $name }
# { $name } is a runtime.
core-restore-installed-prerequisite-reboot = installato il prerequisito { $name } (riavvio necessario)
core-restore-would-restore-license = verrebbe ripristinata la licenza { $source } -> { $destination }
core-restore-restored-license = ripristinata la licenza { $source } -> { $destination }
core-restore-would-restore-license-registry = verrebbe ripristinata la licenza nel Registro di sistema { $key }
# { $name } is a driver's name.
core-restore-driver-not-restored = driver non ripristinato automaticamente: { $name }
# { $name } is a driver's name.
core-restore-would-install-driver = verrebbe installato il driver { $name } -> { $destination }
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-would-restore-task = verrebbe ripristinata l'attività pianificata { $name }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-startup-shortcut = verrebbe ripristinato il collegamento di avvio automatico { $shortcut } -> { $target }
# { $shortcut } is where the shortcut goes, and { $target } what it opens.
core-restore-would-restore-shortcut = verrebbe ripristinato il collegamento { $shortcut } -> { $target }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-would-restore-variable =
    { $scope ->
        [user] verrebbe ripristinata la variabile di ambiente dell'utente { $name }
       *[machine] verrebbe ripristinata la variabile di ambiente di sistema { $name }
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-left-variable =
    { $scope ->
        [user] lasciata invariata la variabile di ambiente dell'utente { $name }: nessuna sua parte appartiene a questa app
       *[machine] lasciata invariata la variabile di ambiente di sistema { $name }: nessuna sua parte appartiene a questa app
    }
# { $scope } says whose the variable is: user (the person's) or machine (every account's). { $name } is the variable's
# name, such as PATH.
core-restore-restored-variable =
    { $scope ->
        [user] ripristinata la variabile di ambiente dell'utente { $name }
       *[machine] ripristinata la variabile di ambiente di sistema { $name }
    }
core-restore-set-uninstaller = impostato il programma di disinstallazione per { $key }
core-restore-placed-driver-image = collocata l'immagine del driver { $source } -> { $destination }
# { $name } is a driver's name.
core-restore-configured-driver = configurato il driver { $name }
# { $name } is a driver's name.
core-restore-created-driver = creato il driver { $name }
core-restore-ensured-registry-key = verificata la presenza della chiave del Registro di sistema { $key }
# { $value } is the name of a value in the key.
core-restore-restored-registry-value = ripristinato nel Registro di sistema { $key } { $value }
# "(Default)" is how Windows names the value a key has with no name: write it as Registry Editor does.
core-restore-restored-registry-default = ripristinato nel Registro di sistema { $key } (Predefinito)
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-configured-service = configurato il servizio { $name } (eseguito come { $account })
# { $name } is a service's name, and { $account } the account it runs as, such as LocalService.
core-restore-created-service = creato il servizio { $name } (eseguito come { $account })
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-restore-restored-task = ripristinata l'attività pianificata { $name }
core-restore-restored-shortcut = ripristinato il collegamento { $path }

## Warnings an install gives, and why it failed. They are shown after it, on the PC it ran on.

core-warning-restore-earlier-not-trusted = Il registro di un'installazione mai completata non è stato toccato, perché altri possono modificarlo. Le modifiche apportate da quell'installazione sono ancora presenti.
# { $app_name } is the app's name. { $count } is how many changes were put back.
core-warning-restore-earlier-undone =
    { $count ->
        [one] Un'installazione precedente di { $app_name } non è mai stata completata, quindi è stata prima annullata. È stata ripristinata { $count } modifica.
       *[other] Un'installazione precedente di { $app_name } non è mai stata completata, quindi è stata prima annullata. Sono state ripristinate { $count } modifiche.
    }
# { $app_name } is the app's name.
core-warning-restore-earlier-never-finished = Un'installazione precedente di { $app_name } non è mai stata completata. L'installazione la annullerà prima di procedere.
core-warning-restore-user-data-real-locations = I dati utente vengono ripristinati nei percorsi %AppData%/%ProgramData% effettivi di questo computer anziché nella radice di destinazione.
# { $error } is what went wrong.
core-warning-restore-commit-failed = Impossibile completare il registro di questa installazione: { $error }
# { $name } is a service's name.
core-warning-restore-service-still-starting = È stato richiesto l'avvio del servizio { $name }, che non ha ancora terminato l'avvio. Potrebbe ancora farlo. Si avvia comunque automaticamente al prossimo avvio di questo PC.
# { $name } is a service's name. { $reason } is why it did not start, as Windows said it.
core-warning-restore-service-not-started = Il servizio { $name } è installato ma non si è avviato: { $reason }. L'app potrebbe non funzionare finché il servizio non si avvia. Windows riproverà al prossimo avvio di questo PC.
# { $name } is a service's name. { $reason } is why it could not be stopped.
core-warning-restore-service-not-stopped = Impossibile arrestare prima il servizio { $name }: { $reason }. Potrebbe continuare a eseguire il programma precedente fino al prossimo avvio di questo PC.
core-warning-restore-install-cancelled = L'installazione è stata annullata.
core-warning-restore-preview-cancelled = L'anteprima è stata annullata.
# { $item } is what was left out, such as a file or a service's name. { $reason } is why, already in words.
core-warning-restore-left-out = { $item } è stato escluso: { $reason }.
# { $error } is what went wrong.
core-warning-restore-verification-failed = Impossibile eseguire la verifica post-ripristino: { $error }
# { $runner } says what relies on the file: a service, a scheduled task, a firewall rule, a shortcut, or a registry value.
# { $path } is the file. { $name } is the name of what relies on it. { $refusal } is why the file was not replaced,
# already in words.
core-warning-restore-program-would-run =
    { $runner ->
        [service] Questo pacchetto non può sostituire { $path } e il servizio { $name } dipende da esso, quindi non è stato installato nulla: { $refusal }
        [task] Questo pacchetto non può sostituire { $path } e l'attività pianificata { $name } dipende da esso, quindi non è stato installato nulla: { $refusal }
        [firewall] Questo pacchetto non può sostituire { $path } e la regola del firewall { $name } dipende da esso, quindi non è stato installato nulla: { $refusal }
        [shortcut] Questo pacchetto non può sostituire { $path } e il collegamento { $name } dipende da esso, quindi non è stato installato nulla: { $refusal }
       *[registry] Questo pacchetto non può sostituire { $path } e il valore del Registro di sistema { $name } dipende da esso, quindi non è stato installato nulla: { $refusal }
    }
core-warning-restore-destination-exists = La destinazione esiste; usa --force per sovrascriverla.
# { $path } is a file. { $error } is what went wrong.
core-warning-restore-file-not-written = Impossibile scrivere { $path }: { $error }
core-warning-restore-not-a-folder = Nella posizione di questa cartella vuota si trova qualcosa che non è una cartella.
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-not-looked-at = Impossibile esaminare { $path }: { $error }
# { $path } is a folder. { $error } is what went wrong.
core-warning-restore-folder-not-made = Impossibile creare { $path }: { $error }
# Why part of the app was left out, after "was left out:".
core-warning-restore-registration-exists = questo PC ha già questo elemento, impostato in modo diverso, e un'installazione non lo sostituisce
core-warning-restore-registry-review = I valori del Registro di sistema sono stati ripristinati dai dati strutturati archiviati, con i percorsi noti dell'app riscritti in base alla radice di destinazione.
# { $error } is what went wrong.
core-warning-restore-firewall-unreadable = Impossibile leggere le regole di Windows Firewall: { $error }
# Why a firewall rule was left out, after "was left out:".
core-warning-restore-firewall-rule-exists = esiste già una regola con questo nome per questo programma, impostata in modo diverso, e un'installazione non la modifica
# { $name } is a runtime.
core-warning-restore-prerequisite-unknown = { $name }: non esiste un programma di installazione noto per questo runtime.
# { $name } is a runtime.
core-warning-restore-prerequisite-needs-admin = L'installazione di { $name } richiede privilegi elevati; esegui di nuovo il programma di installazione come amministratore.
# { $name } is a runtime.
core-warning-restore-prerequisite-reboot = { $name } è stato installato, ma è necessario un riavvio per completare l'operazione.
# { $error } is what went wrong.
core-warning-restore-skipped-not-recorded = Impossibile registrare un runtime ignorato: { $error }
# { $name } is a runtime. { $reason } is why it was skipped, already in words.
core-warning-restore-prerequisite-skipped = { $name }: { $reason }
# { $name } is a runtime. { $reason } is why it could not be installed. { $url } is a web address to get it from.
core-warning-restore-prerequisite-failed = Impossibile installare { $name }: { $reason }. Scaricalo da { $url }.
# { $error } is what went wrong.
core-warning-restore-runtime-not-recorded = Impossibile registrare un runtime installato: { $error }
# { $error } is what went wrong.
core-warning-restore-license-source-invalid = Il percorso della licenza nel pacchetto non è valido: { $error }
core-warning-restore-license-needs-admin = Il ripristino dei dati di licenza in un percorso protetto richiede privilegi elevati.
core-warning-restore-license-exists = La destinazione della licenza esiste; usa --force per sovrascriverla.
# { $error } is what went wrong.
core-warning-restore-license-failed = Impossibile ripristinare i dati di licenza: { $error }
core-warning-restore-driver-needs-admin = La registrazione del driver richiede privilegi elevati; esegui di nuovo il programma di installazione come amministratore.
# { $name } is a scheduled task, with its folder, such as \App\Update.
core-warning-restore-task-runs-as-system = L'attività pianificata { $name } viene eseguita come SYSTEM, con diritti completi su questo PC.
core-warning-restore-shortcut-exists = Il collegamento esiste; usa --force per sovrascriverlo.
core-warning-restore-shortcut-all-users = Uno o più collegamenti ripristinati si trovano nel desktop pubblico o nel menu Start comune.
core-warning-restore-environment-refresh = Le modifiche alle variabili di ambiente potrebbero essere visibili solo in un nuovo processo, in una nuova shell o dopo un nuovo accesso.
# { $name } is a service's name. { $account } is the account it ran as on the PC it came from.
core-warning-restore-service-account = Il servizio { $name } veniva eseguito come { $account } nel PC di origine. La password non è inclusa nel programma di installazione, quindi il servizio viene configurato come LocalService. Se non funziona, imposta account e password in Servizi.
# { $name } is a runtime.
core-warning-restore-prerequisite-pending-reboot = { $name } non è ancora registrato; dovrebbe essere attivo dopo un riavvio.
# { $name } is a runtime.
core-warning-restore-prerequisite-not-verified = { $name } ha segnalato l'esito positivo ma non risulta registrato; l'installazione potrebbe non essere riuscita.
# { $error } is what went wrong.
core-warning-restore-driver-source-invalid = Il percorso del driver incluso non è valido: { $error }
# { $error } is what went wrong.
core-warning-restore-driver-not-read = Impossibile leggere le impostazioni correnti del driver: { $error }
core-warning-restore-driver-exists = Il servizio del driver esiste; usa --force per riconfigurarlo.
core-warning-restore-driver-image-exists = L'immagine del driver esiste; usa --force per sovrascriverla.
core-warning-restore-driver-hash-bundled = L'immagine del driver inclusa non corrisponde all'hash registrato, quindi non è stata installata.
# { $error } is what went wrong.
core-warning-restore-driver-not-placed = Impossibile collocare l'immagine del driver: { $error }
core-warning-restore-driver-hash-placed = L'immagine del driver è cambiata dopo essere stata collocata, quindi non è stata registrata.
# { $error } is what went wrong.
core-warning-restore-key-not-created = Impossibile creare la chiave del Registro di sistema: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-read = Impossibile leggere il valore che verrebbe sostituito: { $error }
# { $error } is what went wrong.
core-warning-restore-value-not-written = Impossibile scrivere il valore del Registro di sistema: { $error }
core-warning-restore-value-exists = Il valore del Registro di sistema esiste; usa --force per sovrascriverlo.
# { $error } is what went wrong.
core-warning-restore-environment-key-not-created = Impossibile creare la chiave del Registro di sistema per l'ambiente: { $error }
core-warning-restore-variable-exists = La variabile di ambiente esiste; usa --force per sovrascriverla.
# { $error } is what went wrong.
core-warning-restore-variable-not-written = Impossibile scrivere la variabile di ambiente: { $error }
core-warning-restore-value-kind-missing = Manca il tipo del valore del Registro di sistema.
# { $error } is what went wrong.
core-warning-restore-service-not-read = Impossibile leggere le impostazioni correnti del servizio: { $error }
core-warning-restore-service-exists = Il servizio esiste; usa --force per riconfigurarlo.
# { $error } is what went wrong.
core-warning-restore-task-not-queried = Impossibile interrogare l'attività pianificata: { $error }
core-warning-restore-task-exists = L'attività pianificata esiste; usa --force per sovrascriverla.
# { $error } is what went wrong.
core-warning-restore-task-folder-not-looked-at = Impossibile esaminare la cartella dell'attività: { $error }
# { $error } is what went wrong.
core-warning-restore-shortcut-not-put-back = Impossibile creare il collegamento e ripristinare quello che sostituiva: { $error }
# { $error } is what went wrong.
core-warning-restore-command-not-started = Impossibile avviare il comando: { $error }
# { $output } is what the command wrote, as it wrote it.
core-warning-restore-command-failed = Comando non riuscito: { $output }
# { $error } is what went wrong.
core-warning-restore-powershell-not-started = Impossibile avviare PowerShell: { $error }
