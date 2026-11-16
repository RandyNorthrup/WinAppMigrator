# Words the core writes for people: warnings, refusals, errors, and the installer's list of what it sets up.
# Message names start with core-.


## Warnings a package records when it is built, which an installer shows later, maybe on another PC, in that PC's
## language.

# { $count } is how many shortcuts.
core-warning-shortcuts-unreadable =
    { $count ->
        [one] { $count } collegamento in questa unità non fa riferimento ad alcun file, quindi non è stato possibile determinare se apre questa app.
       *[other] { $count } collegamenti in questa unità non fanno riferimento ad alcun file, quindi non è stato possibile determinare se aprono questa app.
    }

## Undoing an install: what an uninstall, or a failed install, says it put back, left, or could not undo. Each line is
## shown on its own, in a list.

# { $app_name } is the app's name.
core-rollback-record-damaged = Non è stato possibile leggere parte del registro di { $app_name }, quindi tutto ciò che vi era descritto è ancora presente e un file sostituito potrebbe non tornare al suo posto.
core-rollback-settings-entry-stays = La voce dell'app in Impostazioni resta, in modo che questa operazione possa essere eseguita di nuovo da lì.
# { $name } is a runtime's name, such as Microsoft Visual C++ 2015-2022 Redistributable (x64).
core-rollback-prerequisite-stays = { $name } è stato installato, o potrebbe esserlo stato, e resta, perché altri programmi potrebbero usarlo.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-drive-not-connected = Ciò che questa installazione ha inserito in { $folder } si trova in un'unità non collegata. Collegala per completare l'operazione.
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-record-part-unread = Ciò che questa installazione ha inserito in { $folder } resta, perché non è stato possibile leggere quella parte del registro.

## Why a file or folder was left, shown after "was left:" in the messages below them. Each is part of a sentence, so
## starts with a small letter and has no full stop.

# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-being-changed = qualcosa stava modificando { $folder } durante l'operazione, quindi non vi è stato fatto altro. Chiudi il programma responsabile ed esegui di nuovo l'operazione
# { $folder } is a folder; { $error } is what Windows said went wrong.
core-rollback-reason-not-opened = impossibile aprire { $folder }: { $error }
# { $folder } is a folder, such as C:\Program Files\App.
core-rollback-reason-no-record = non risulta dove si trovava { $folder }
# { $folder } is a folder; { $actual } is the folder it now leads to.
core-rollback-reason-moved = { $folder } ora porta a { $actual }, non dove si trovava durante l'installazione
# Why a service was not stopped.
core-rollback-reason-did-not-stop = non si è arrestato in tempo

## Files and folders

# { $path } is a folder; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-folder-left = La cartella { $path } è stata lasciata al suo posto: { $reason }.
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-removed = Rimossa la cartella { $path }
# { $path } is a folder, such as C:\Program Files\App\Plugins.
core-rollback-folder-holds-others = La cartella { $path } resta, perché contiene file che questa installazione non vi ha inserito.
# { $path } is a folder; { $error } is what Windows said went wrong.
core-rollback-folder-not-removed = Impossibile rimuovere la cartella { $path }: { $error }.
# { $path } is a file; { $reason } is one of the reasons above, or what Windows said went wrong.
core-rollback-file-left = Il file { $path } è stato lasciato al suo posto: { $reason }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-folder-gone = { $path } non viene ripristinato, perché la cartella che lo conteneva non esiste più.
# { $path } is a file; { $error } is what went wrong.
core-rollback-partial-not-removed = Impossibile rimuovere un file parziale: { $path } ({ $error }).
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-checked = Impossibile verificare { $path }: { $error }.
# { $path } is a file, such as C:\Program Files\App\app.exe.
core-rollback-file-removed = Rimosso { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-removed = Impossibile rimuovere { $path }: { $error }.
# { $path } is a file this install had replaced, and has put back.
core-rollback-file-put-back = Ripristinato { $path }
# { $path } is a file; { $error } is what went wrong.
core-rollback-file-not-put-back = Impossibile ripristinare { $path }: { $error }.
# Why a file could not be put back.
core-rollback-backup-gone = il backup non esiste più
# { $path } is a file.
core-rollback-no-parent-folder = { $path } non ha una cartella padre

## A file that stays. { $why } says why: still-used (something that stays still uses it), changed (it has changed since
## it was installed), or not-asked (it is someone's data, and they did not ask for it to go). { $path } is the file.

core-rollback-file-stays =
    { $why ->
        [still-used] { $path } resta, perché qualcosa che resta lo usa ancora.
        [changed] { $path } resta, perché è cambiato dopo l'installazione.
       *[not-asked] { $path } resta, perché non ne è stata richiesta la rimozione.
    }
# The file it replaced was to be put beside it, under another name, and every name was taken.
core-rollback-file-stays-no-name-beside =
    { $why ->
        [still-used] { $path } resta, perché qualcosa che resta lo usa ancora, e non è stato possibile mettere accanto a esso il file che ha sostituito: tutti i nomi possibili sono già in uso.
        [changed] { $path } resta, perché è cambiato dopo l'installazione, e non è stato possibile mettere accanto a esso il file che ha sostituito: tutti i nomi possibili sono già in uso.
       *[not-asked] { $path } resta, perché non ne è stata richiesta la rimozione, e non è stato possibile mettere accanto a esso il file che ha sostituito: tutti i nomi possibili sono già in uso.
    }
# { $name } is the name the file it replaced now has, such as settings.ini.before-install.
core-rollback-file-stays-beside =
    { $why ->
        [still-used] { $path } resta, perché qualcosa che resta lo usa ancora. Il file che ha sostituito si trova accanto a esso, con il nome { $name }.
        [changed] { $path } resta, perché è cambiato dopo l'installazione. Il file che ha sostituito si trova accanto a esso, con il nome { $name }.
       *[not-asked] { $path } resta, perché non ne è stata richiesta la rimozione. Il file che ha sostituito si trova accanto a esso, con il nome { $name }.
    }
# { $error } is what went wrong.
core-rollback-file-stays-not-beside =
    { $why ->
        [still-used] { $path } resta, perché qualcosa che resta lo usa ancora, e non è stato possibile mettere accanto a esso il file che ha sostituito: { $error }.
        [changed] { $path } resta, perché è cambiato dopo l'installazione, e non è stato possibile mettere accanto a esso il file che ha sostituito: { $error }.
       *[not-asked] { $path } resta, perché non ne è stata richiesta la rimozione, e non è stato possibile mettere accanto a esso il file che ha sostituito: { $error }.
    }

## The registry and environment variables. { $kind } says what { $name } is: key (a registry key, such as
## HKLM\Software\App), value (a registry value, such as HKLM\Software\App\Version), user-variable or machine-variable
## (an environment variable of the user's or of the whole PC, such as APP_HOME), or user-list or machine-list (a
## variable that is a list of folders, such as Path). { $error } is what Windows said went wrong.

# How a registry key's unnamed value is shown, as Registry Editor shows it.
core-registry-default-value-name = (Predefinito)
core-rollback-setting-other-account =
    { $kind ->
        [key] La chiave { $name } resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
        [user-variable] La variabile di ambiente dell'utente { $name } resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
        [machine-variable] La variabile di ambiente di sistema { $name } resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
        [user-list] La variabile { $name } dell'utente resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
        [machine-list] La variabile { $name } di sistema resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
       *[value] Il valore del Registro di sistema { $name } resta, perché appartiene all'account che ha installato l'app, e quell'account non ha eseguito l'accesso.
    }
core-rollback-setting-not-reached =
    { $kind ->
        [key] Impossibile accedere alla chiave { $name }: { $error }.
        [user-variable] Impossibile accedere alla variabile di ambiente dell'utente { $name }: { $error }.
        [machine-variable] Impossibile accedere alla variabile di ambiente di sistema { $name }: { $error }.
        [user-list] Impossibile accedere alla variabile { $name } dell'utente: { $error }.
        [machine-list] Impossibile accedere alla variabile { $name } di sistema: { $error }.
       *[value] Impossibile accedere al valore del Registro di sistema { $name }: { $error }.
    }
core-rollback-setting-not-read =
    { $kind ->
        [key] Impossibile leggere la chiave { $name }: { $error }.
        [user-variable] Impossibile leggere la variabile di ambiente dell'utente { $name }: { $error }.
        [machine-variable] Impossibile leggere la variabile di ambiente di sistema { $name }: { $error }.
        [user-list] Impossibile leggere la variabile { $name } dell'utente: { $error }.
        [machine-list] Impossibile leggere la variabile { $name } di sistema: { $error }.
       *[value] Impossibile leggere il valore del Registro di sistema { $name }: { $error }.
    }
core-rollback-setting-changed =
    { $kind ->
        [user-variable] La variabile di ambiente dell'utente { $name } resta, perché è cambiata dopo l'installazione.
        [machine-variable] La variabile di ambiente di sistema { $name } resta, perché è cambiata dopo l'installazione.
        [user-list] La variabile { $name } dell'utente resta, perché è cambiata dopo l'installazione.
        [machine-list] La variabile { $name } di sistema resta, perché è cambiata dopo l'installazione.
       *[value] Il valore del Registro di sistema { $name } resta, perché è cambiato dopo l'installazione.
    }
# Said when it is back as it was before the install.
core-rollback-setting-put-back =
    { $kind ->
        [user-variable] Ripristinata la variabile di ambiente dell'utente { $name }
        [machine-variable] Ripristinata la variabile di ambiente di sistema { $name }
        [user-list] Ripristinata la variabile { $name } dell'utente
        [machine-list] Ripristinata la variabile { $name } di sistema
       *[value] Ripristinato il valore del Registro di sistema { $name }
    }
core-rollback-setting-not-put-back =
    { $kind ->
        [user-variable] Impossibile ripristinare la variabile di ambiente dell'utente { $name }: { $error }.
        [machine-variable] Impossibile ripristinare la variabile di ambiente di sistema { $name }: { $error }.
        [user-list] Impossibile ripristinare la variabile { $name } dell'utente: { $error }.
        [machine-list] Impossibile ripristinare la variabile { $name } di sistema: { $error }.
       *[value] Impossibile ripristinare il valore del Registro di sistema { $name }: { $error }.
    }
# A registry value the install wrote, which is the user's data, and stays unless they asked for it to go.
core-rollback-value-kept-by-request = Il valore del Registro di sistema { $name } resta, perché non ne è stata richiesta la rimozione.
core-rollback-key-holds-others = La chiave { $name } resta, perché contiene elementi che questa installazione non vi ha inserito.
core-rollback-key-removed = Rimossa la chiave { $name }
core-rollback-key-not-removed = Impossibile rimuovere la chiave { $name }: { $error }.
# { $kind } is user-list or machine-list.
core-rollback-list-entries-removed =
    { $kind ->
        [user-list] Rimosse le voci di questa installazione dalla variabile { $name } dell'utente
       *[machine-list] Rimosse le voci di questa installazione dalla variabile { $name } di sistema
    }
core-rollback-list-entries-not-removed =
    { $kind ->
        [user-list] Impossibile rimuovere le voci di questa installazione dalla variabile { $name } dell'utente: { $error }.
       *[machine-list] Impossibile rimuovere le voci di questa installazione dalla variabile { $name } di sistema: { $error }.
    }

## Services and drivers. { $kind } is service or driver; { $name } is its name, such as AppUpdater. { $error } and
## { $reason } are what Windows said went wrong.

core-rollback-service-not-read =
    { $kind ->
        [driver] Impossibile leggere il driver { $name }: { $error }.
       *[service] Impossibile leggere il servizio { $name }: { $error }.
    }
core-rollback-service-changed =
    { $kind ->
        [driver] Il driver { $name } resta, perché è cambiato dopo l'installazione.
       *[service] Il servizio { $name } resta, perché è cambiato dopo l'installazione.
    }
core-rollback-service-not-stopped =
    { $kind ->
        [driver] Impossibile arrestare il driver { $name } ({ $reason }), quindi ciò che sta usando potrebbe restare fino al riavvio del PC.
       *[service] Impossibile arrestare il servizio { $name } ({ $reason }), quindi ciò che sta usando potrebbe restare fino al riavvio del PC.
    }
core-rollback-service-stopped-for-this =
    { $kind ->
        [driver] Il driver { $name } è stato arrestato per eseguire questa operazione. Avvialo di nuovo quando ti serve.
       *[service] Il servizio { $name } è stato arrestato per eseguire questa operazione. Avvialo di nuovo quando ti serve.
    }
core-rollback-service-not-removed =
    { $kind ->
        [driver] Impossibile rimuovere il driver { $name }: { $reason }.
       *[service] Impossibile rimuovere il servizio { $name }: { $reason }.
    }
core-rollback-service-marked-for-removal =
    { $kind ->
        [driver] Il driver { $name } è contrassegnato per la rimozione e verrà rimosso al riavvio del PC o quando verrà rilasciato da ciò che lo tiene aperto.
       *[service] Il servizio { $name } è contrassegnato per la rimozione e verrà rimosso al riavvio del PC o quando verrà rilasciato da ciò che lo tiene aperto.
    }
core-rollback-service-removed =
    { $kind ->
        [driver] Rimosso il driver { $name }
       *[service] Rimosso il servizio { $name }
    }
core-rollback-service-not-restored =
    { $kind ->
        [driver] Impossibile riportare il driver { $name } allo stato precedente: { $reason }.
       *[service] Impossibile riportare il servizio { $name } allo stato precedente: { $reason }.
    }
core-rollback-service-restored =
    { $kind ->
        [driver] Ripristinate le impostazioni del driver { $name }
       *[service] Ripristinate le impostazioni del servizio { $name }
    }
core-rollback-service-not-restarted = Il servizio { $name } era in esecuzione prima di questa installazione e non si è riavviato: { $reason }.
# { $account } is the account the service ran as, such as CONTOSO\backup.
core-rollback-service-account-again = Il servizio { $name } veniva eseguito come { $account }. La password non viene conservata, quindi imposta di nuovo l'account in Servizi.

## Firewall rules. { $name } is the rule's name; { $error } is what went wrong.

core-rollback-firewall-not-read = Impossibile leggere la regola del firewall { $name }: { $error }.
core-rollback-firewall-says-otherwise = La regola del firewall { $name } resta, perché la regola presente con quel nome indica qualcos'altro.
core-rollback-firewall-changed = La regola del firewall { $name } resta, perché è cambiata dopo l'installazione.
# The capital "The" is as the app has always said it.
core-rollback-firewall-removed = Rimossa la regola del firewall { $name }
core-rollback-firewall-not-removed = Impossibile rimuovere la regola del firewall { $name }: { $error }.

## Scheduled tasks. { $path } is a folder of tasks, and { $name } a task with its folder, such as \App\Nightly Update.
## { $error } and { $reason } are what went wrong.

core-rollback-task-folder-holds-others = La cartella delle attività { $path } resta, perché contiene attività che questa installazione non vi ha inserito.
core-rollback-task-folder-not-removed = Impossibile rimuovere la cartella delle attività { $path }: { $error }.
core-rollback-task-not-read = Impossibile leggere l'attività { $name }: { $error }.
core-rollback-task-changed = L'attività { $name } resta, perché è cambiata dopo l'installazione.
core-rollback-task-removed = Rimossa l'attività { $name }
core-rollback-task-not-removed = Impossibile rimuovere l'attività { $name }: { $reason }.
core-rollback-task-put-back = Ripristinata l'attività { $name }
core-rollback-task-not-put-back = Impossibile ripristinare l'attività { $name }: { $reason }.

## When an install cannot be undone at all

# { $error } is what went wrong.
core-rollback-record-unreadable = Impossibile leggere il registro di questa installazione, quindi non è stato annullato nulla: { $error }.
# { $error } is what went wrong.
core-rollback-nothing-undone = Non è stato annullato nulla: { $error }.
# One app's part of the message below. { $app_name } is the app's name; { $failures } is what could not be undone, a
# sentence or more; { $folder } is the folder that holds the install's record.
core-rollback-earlier-install-stuck = { $app_name }: { $failures } Se non può mai essere annullata, un amministratore può rinunciarvi rimuovendo { $folder }.
# Why an install did not start. { $details } is one or more of the message above.
core-rollback-earlier-installs-not-undone = non è stato possibile annullare completamente un'installazione precedente mai completata, quindi non è stato installato nulla. Chiudi i programmi che usano questi elementi e riprova. { $details }
core-rollback-install-busy = questa installazione è in fase di modifica da parte di un altro programma di installazione o di disinstallazione

## The summary an uninstall writes on the command line, one line each, with what each says below it. { $count } is how
## many.

core-rollback-count-undone = Annullati: { $count }
core-rollback-count-failed = Impossibile annullare: { $count }
# What was left on purpose: it has changed since the install, or it is the user's.
core-rollback-count-kept = Lasciati al loro posto: { $count }
# What no uninstall can take back, such as a runtime that was installed, which other programs may use.
core-rollback-count-left-behind = Rimasti: { $count }

## Which Windows a PC or drive has, as the app shows it beside the drive.

# { $name } is the Windows' name, such as Windows 7 Professional Service Pack 1.
core-system-windows-32-bit = { $name }, 32 bit
# { $name } is the Windows' name, such as Windows 11 Pro.
core-system-windows-64-bit = { $name }, 64 bit
# A drive's Windows, when it only says it is older than Vista.
core-system-windows-xp-or-older = Windows XP o versioni precedenti

## Reading a connected drive, or another account's settings. Each is why it could not be done, and some start with a
## small letter, as they follow other words.

core-system-needs-administrator = la lettura di un'unità collegata o delle impostazioni di un altro account richiede diritti di amministratore
core-system-no-program-data = impossibile trovare la cartella ProgramData di questo PC
# { $path } is a file, such as D:\Windows\System32\config\SOFTWARE.
core-system-not-a-hive-file = non è un file hive del Registro di sistema: { $path }
# { $path } is a file; { $size } is its size in bytes.
core-system-hive-too-large = { $path } è di { $size } byte, troppo grande per essere un hive del Registro di sistema o più di quanto possano occupare tutti gli hive di questa unità
# { $key } is a registry key, such as Software\Microsoft\Windows\CurrentVersion.
core-system-hive-no-mapping = l'hive offline non ha alcun mapping per HKLM\{ $key }
# { $key } is where a drive's registry was to be loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said.
# "reg load" is a command's name.
core-system-hive-not-loaded = reg load { $key } non riuscito: { $error }
# { $key } is where a drive's registry was loaded, such as HKLM\WAM_SOFTWARE; { $error } is what Windows said. "reg
# unload" is a command's name.
core-system-hive-not-unloaded = reg unload { $key } non riuscito: { $error }
# { $drive } is a drive, such as D:\.
core-system-not-a-system-drive = { $drive } non è un'unità di sistema Windows (manca la cartella \Windows).
# { $drive } is a drive, such as C:\. "This PC" is the name of a choice in the app.
core-system-own-system-drive = { $drive } è l'unità di sistema di questo PC. Scegli Questo PC per leggere questo PC.
# { $drive } is a drive, such as D:\.
core-system-registry-through-link = { $drive } non è un'unità di sistema Windows: il percorso del suo Registro di sistema passa attraverso un collegamento simbolico.

## Warnings a scan records about whose data it read.

# { $account } is an account's name or id, such as alice or S-1-5-21-...-1001.
core-warning-scan-account-not-on-record = L'account { $account } non risulta registrato qui, quindi non sono stati inclusi dati o impostazioni per utente di alcun account.
# { $account } is an account's name, such as CONTOSO\alice.
core-warning-scan-signed-in-account-no-profile = L'account che ha eseguito l'accesso qui, { $account }, non ha un profilo registrato, quindi non sono stati inclusi dati o impostazioni per utente di alcun account.
# { $reason } is why Windows could not say.
core-warning-scan-signed-in-account-unknown = Impossibile determinare chi ha eseguito l'accesso ({ $reason }), quindi non sono stati inclusi dati o impostazioni per utente di alcun account. Scegli di quale account includerli.
core-warning-scan-last-account-unknown = Questa unità non indica chi ha eseguito l'ultimo accesso e contiene più di un account, quindi non sono stati inclusi dati o impostazioni per utente di alcun account. Scegli di quale account includerli.
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-settings-unreadable = Impossibile leggere le impostazioni di { $account }, quindi non sono state incluse: { $error }
# { $account } is an account's name; { $error } is what went wrong.
core-warning-scan-account-classes-unreadable = Impossibile leggere le classi COM e i tipi di file registrati per { $account }, quindi non sono stati inclusi: { $error }

## Paths a scan did not follow. { $count } is how many; { $paths } is some of them, separated by semicolons; { $more } is
## how many more there are, not shown.

core-warning-scan-drive-paths-not-followed =
    { $count ->
        [one] Impossibile leggere un percorso registrato in questa unità, quindi non è stato incluso nulla da lì. Ogni percorso di questo tipo si trova in un altro disco, non è un percorso completo o passa attraverso un collegamento simbolico, che porterebbe fuori dall'unità: { $paths }
       *[other] Impossibile leggere alcuni percorsi registrati in questa unità, quindi non è stato incluso nulla da lì. Ognuno si trova in un altro disco, non è un percorso completo o passa attraverso un collegamento simbolico, che porterebbe fuori dall'unità: { $paths }
    }
core-warning-scan-drive-paths-not-followed-more = Impossibile leggere alcuni percorsi registrati in questa unità, quindi non è stato incluso nulla da lì. Ognuno si trova in un altro disco, non è un percorso completo o passa attraverso un collegamento simbolico, che porterebbe fuori dall'unità: { $paths } e altri { $more }
core-warning-scan-network-paths-not-followed =
    { $count ->
        [one] Impossibile leggere un percorso registrato in questo PC, quindi non è stato incluso nulla da lì. Ogni percorso di questo tipo non si trova in un disco di questo PC, per cui esaminarlo comporterebbe una connessione, oppure ha un nome che aprirebbe qualcos'altro: { $paths }
       *[other] Impossibile leggere alcuni percorsi registrati in questo PC, quindi non è stato incluso nulla da lì. Ognuno non si trova in un disco di questo PC, per cui esaminarlo comporterebbe una connessione, oppure ha un nome che aprirebbe qualcos'altro: { $paths }
    }
core-warning-scan-network-paths-not-followed-more = Impossibile leggere alcuni percorsi registrati in questo PC, quindi non è stato incluso nulla da lì. Ognuno non si trova in un disco di questo PC, per cui esaminarlo comporterebbe una connessione, oppure ha un nome che aprirebbe qualcos'altro: { $paths } e altri { $more }

## What a scan with none of an account's data leaves out of a package, in the app's list of what was left out. Each
## has a name, and below it why.

core-account-left-out-shortcuts = Collegamenti nelle cartelle di questo account
# { $count } is how many shortcuts.
core-account-left-out-shortcuts-why =
    { $count ->
        [one] { $count } collegamento, incluso solo con l'opzione Dati utente
       *[other] { $count } collegamenti, inclusi solo con l'opzione Dati utente
    }
core-account-left-out-variables = Variabili di ambiente di questo account
# { $count } is how many variables.
core-account-left-out-variables-why =
    { $count ->
        [one] { $count } variabile, inclusa solo con l'opzione Dati utente
       *[other] { $count } variabili, incluse solo con l'opzione Dati utente
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-account-left-out-task = L'attività pianificata { $task }
core-account-task-works-on-account-folder = Opera su un percorso nelle cartelle di un account, o ne porta il nome, e viene inclusa solo con l'opzione Dati utente
core-account-task-runs-from-account-folder = Ciò che esegue anche dalle cartelle di un account viene incluso solo con l'opzione Dati utente
# { $service } is a service's name, such as AppUpdater.
core-account-left-out-service = Il servizio { $service }
core-account-service-works-on-account-folder = Opera su un percorso nelle cartelle di un account e viene incluso solo con l'opzione Dati utente

## What a package was found to still say of whose PC it came from, after a build with none of an account's data.

# The parts of a package, as the warning below names them.
core-account-part-files = i percorsi dei file dell'app
core-account-part-registry = valori del Registro di sistema
core-account-part-settings-entry = la voce dell'app in Impostazioni
core-account-part-main-program = il percorso del programma principale
core-account-part-shortcuts = collegamenti
core-account-part-tasks = attività pianificate
core-account-part-services = servizi
core-account-part-variables = variabili di ambiente
# One place in the list below. { $place } is one of the parts above, or a file's name; { $count } is how many times.
core-account-mention-place = { $place } ({ $count })
# { $kind } is what was found: account (an account's name), folder (where an account's folder is), computer (the PC's
# name), or id (an account's id). { $places } is a list of where, from the message above.
core-warning-scan-package-mentions =
    { $kind ->
        [account] Questo pacchetto contiene il nome di un account in { $places }. È stato creato senza i dati di alcun account, e chiunque abbia il programma di installazione può leggere queste informazioni.
        [folder] Questo pacchetto indica dove si trova la cartella di un account in { $places }. È stato creato senza i dati di alcun account, e chiunque abbia il programma di installazione può leggere queste informazioni.
        [computer] Questo pacchetto contiene il nome del PC in { $places }. È stato creato senza i dati di alcun account, e chiunque abbia il programma di installazione può leggere queste informazioni.
       *[id] Questo pacchetto contiene l'ID di un account in { $places }. È stato creato senza i dati di alcun account, e chiunque abbia il programma di installazione può leggere queste informazioni.
    }
# { $count } is how many folders.
core-warning-scan-folders-of-others-unreadable =
    { $count ->
        [one] Impossibile leggere { $count } cartella esterna all'app, quindi potrebbero mancare dati dell'app presenti lì.
       *[other] Impossibile leggere { $count } cartelle esterne all'app, quindi potrebbero mancare dati dell'app presenti lì.
    }

## What a scan leaves out because it looks like a credential, or is a license with the License option off, in the app's
## list of what was left out. Each has a name, and below it why.

core-secret-why-credential = sembra una credenziale
core-secret-why-license-option-off = l'opzione Dati di licenza e attivazione è disattivata
# { $count } is how many values of a registry key.
core-secret-why-account-values =
    { $count ->
        [one] { $count } valore, incluso solo con l'opzione Dati utente
       *[other] { $count } valori, inclusi solo con l'opzione Dati utente
    }
# Shown in place of a registry value's name when its name itself looks like a password or a key.
core-secret-name-holds-credential = (un valore il cui nome contiene una credenziale)
# Shown in place of an option's name, for an argument on a command line that has no name.
core-secret-unnamed-argument = un argomento che contiene una credenziale
# { $option } is an option taken out of a command line, such as /password, or the message above. { $place } says
# where the command line was: service, task, shortcut, opener (what opens a file type), opener-unnamed (the same, of a
# type with no name), or uninstall. { $name } is the service, task, shortcut, file type, or app.
core-secret-taken-from =
    { $place ->
        [service] { $option } nel comando di esecuzione di { $name }
        [task] { $option } nel comando di esecuzione dell'attività { $name }
        [shortcut] { $option } nel collegamento { $name }
        [opener] { $option } nel programma che apre { $name }
        [opener-unnamed] { $option } nel programma che apre un tipo di file
       *[uninstall] { $option } nel comando di disinstallazione di { $name }
    }
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-secret-task-definition-unreadable = La definizione dell'attività { $task }, che non è stato possibile leggere per verificarla
# { $name } is an environment variable's name, such as APP_TOKEN.
core-secret-left-out-variable = La variabile di ambiente { $name }
# { $class } is a COM class's id, such as {00000000-0000-0000-0000-000000000000}.
core-secret-left-out-class-server = Ciò che la classe { $class } dichiara di eseguire
# { $rule } is a firewall rule's name.
core-secret-left-out-firewall-description = La descrizione della regola del firewall { $rule }

## Licenses a scan found, as the package's report describes each.

# { $name } is a file's name, such as license.lic.
core-license-detail-file = File di licenza rilevato: { $name }
# { $framework } is a licensing product's name, such as FlexLM / FlexNet.
core-license-detail-store = Archivio licenze { $framework } presente
# { $name } is an environment variable's name, such as LM_LICENSE_FILE.
core-license-detail-variable = Variabile di ambiente della licenza { $name }
# { $name } is a registry value's name, such as SerialNumber.
core-license-detail-registry-value = Valore del Registro di sistema che sembra contenere dati di licenza: { $name }
# Said of a registry value in an account's key, which is not named.
core-license-detail-registry-value-of-an-account = Valore del Registro di sistema che sembra contenere dati di licenza
# { $key } is a registry key, such as HKLM\Software\Contoso\License.
core-license-detail-registry-key = Chiave del Registro di sistema che sembra contenere dati di licenza: { $key }

## Warnings a scan records about licenses.

core-warning-scan-license-data = Sono stati rilevati dati di licenza o attivazione. Se l'opzione Dati di licenza e attivazione è attiva, il programma di installazione ripristina ciò che può essere trasferito; i valori a livello di computer e gli archivi legati all'hardware vengono solo segnalati. WinAppMigrator non decrittografa mai, non aggira l'attivazione e non emula dongle.
core-warning-scan-license-machine-bound = Alcune licenze sono legate al computer (dongle hardware, attivazione bloccata sul computer o crittografia). Il programma di installazione ripristina ciò che può; queste licenze potrebbero comunque richiedere una nuova attivazione nel nuovo PC.

## Warnings a scan records about the registry. What could not be read is not named in them: it is kept beside them.

# { $error } is what Windows said went wrong. App Paths is a registry key's name.
core-warning-scan-app-paths-key-unreadable = Impossibile leggere una sottochiave di App Paths: { $error }
core-warning-scan-registry-limit = L'acquisizione del Registro di sistema ha raggiunto il limite di sicurezza configurato.
core-warning-scan-registry-subtree-limit = L'acquisizione di un sottoalbero del Registro di sistema ha raggiunto il limite di sicurezza configurato.
# { $error } is what Windows said went wrong.
core-warning-scan-registry-key-unreadable = Impossibile leggere la chiave del Registro di sistema: { $error }
core-warning-scan-registry-value-too-large = Il valore del Registro di sistema ha superato il limite di dimensioni configurato per l'acquisizione.

## Scheduled tasks

core-warning-scan-tasks-folder-through-link = La cartella delle attività pianificate in questa unità passa attraverso un collegamento simbolico, quindi le sue attività non sono state lette.
# { $task } is a scheduled task with its folder, such as \App\Nightly Sync.
core-warning-scan-task-left-out = L'attività pianificata { $task } è stata esclusa: viene eseguita con un account diverso da quello di cui vengono incluse le impostazioni, il suo file può essere modificato anche da utenti diversi dagli amministratori, oppure l'app ha troppe attività da conservare.
# { $error } is what Windows said went wrong.
core-warning-scan-tasks-folder-unreadable = Impossibile leggere parte delle attività pianificate: { $error }
# { $path } is a task's file, such as C:\Windows\System32\Tasks\App\Nightly Sync; { $reason } is one of the reasons
# below, or what Windows said went wrong.
core-warning-scan-task-unreadable = Impossibile leggere l'attività pianificata { $path }: { $reason }. Se esegue l'app, non è stata inclusa.
# Why a task's file could not be read. { $size } is its size in bytes.
core-task-too-large = è di { $size } byte, troppo grande per essere un'attività
# Why a task's file could not be read.
core-task-ends-mid-character = termina a metà di un carattere
# Why a task's file could not be read. { $error } says where.
core-task-not-utf16 = non è in formato UTF-16 valido: { $error }
# Why a task's file could not be read. { $error } says where.
core-task-not-text = non è in formato UTF-8 o UTF-16 valido: { $error }
# What a package records of how a task is put back. Register-ScheduledTask is a command's name.
core-task-restore-instructions = Il ripristino usa Register-ScheduledTask con la definizione XML archiviata.

## Reading a program's file. Each is why it could not be read, after the program's name.

# { $reason } is what went wrong.
core-pe-unreadable = impossibile leggerlo: { $reason }
core-pe-headers-misaligned = le sue intestazioni non sono allineate come devono esserlo quelle di un programma
# { $size } is the file's size in bytes.
core-pe-too-large = è di { $size } byte, troppo grande per essere un programma
# PE is the name of the format Windows programs are in.
core-pe-no-headers = non ha intestazioni PE

## Warnings a scan records about what a program needs. Which program is kept beside each.

# { $error } is what went wrong.
core-warning-scan-imports-unreadable = Impossibile leggere la tabella delle importazioni: { $error }
# { $error } is what went wrong.
core-warning-scan-import-name-unreadable = Impossibile leggere il nome di un descrittore di importazione: { $error }
# { $count } is how many were skipped.
core-warning-scan-legacy-delay-imports =
    { $count ->
        [one] { $count } descrittore di caricamento ritardato legacy (v1) è stato ignorato; vengono analizzate solo le importazioni a caricamento ritardato moderne basate su RVA.
       *[other] { $count } descrittori di caricamento ritardato legacy (v1) sono stati ignorati; vengono analizzate solo le importazioni a caricamento ritardato moderne basate su RVA.
    }

## The DLLs a program needs. Which DLL or program is kept beside each warning.

# { $path } is a program, such as C:\Program Files\App\app.exe.
core-dll-program-has-no-folder = l'eseguibile non ha una cartella padre: { $path }
# Why a DLL is among the app's files, in the package's list of them.
core-dll-beside-program = DLL presente accanto all'eseguibile selezionato
# { $error } is what went wrong.
core-warning-scan-dependency-unscanned = Impossibile analizzare la dipendenza ricorsiva: { $error }
# { $name } is a DLL's name, such as d3dx9_43.dll.
core-warning-scan-dependency-not-in-newer-windows = { $name } si trovava nella cartella Windows del PC in cui era installata questa app, ma Windows 10 e 11 non lo includono. È stato installato lì da un altro programma, quindi non fa parte dei file dell'app e non viene incluso. L'app potrebbe non avviarsi finché non viene installato ciò che lo fornisce.
# { $error } is what went wrong.
core-warning-scan-dependency-not-hashed = Impossibile calcolare l'hash della dipendenza: { $error }
# SystemRoot and windir are the names of environment variables.
core-warning-scan-windows-folder-unknown = Le variabili di ambiente SystemRoot/windir non sono disponibili; la ricerca delle DLL di sistema è limitata.
# { $error } is what went wrong.
core-warning-scan-dependency-architecture-unreadable = Impossibile leggere l'architettura della dipendenza: { $error }

## Services, drivers, shortcuts, and environment variables a scan reads. Where it was reading is kept beside each
## warning.

# { $error } is what Windows said went wrong.
core-warning-scan-services-unreadable = Impossibile leggere la chiave del Registro di sistema dei servizi: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-drivers-unreadable = Impossibile leggere la chiave del Registro di sistema dei servizi per il rilevamento dei driver: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-queried = Impossibile interrogare i collegamenti: { $error }
# { $error } is what went wrong.
core-warning-scan-shortcuts-not-parsed = Impossibile analizzare l'output dell'interrogazione dei collegamenti: { $error }
# { $error } is what Windows said went wrong.
core-warning-scan-environment-unreadable = Impossibile leggere la chiave del Registro di sistema delle variabili di ambiente di sistema: { $error }

## What a package records of how a driver is put back, which an installer shows. { $kind } is the driver's type as
## Windows names it (kernel, file-system, recognizer, or unknown-type); { $name } is its service's name; { $path } is
## its file. { $signed } is yes when { $signer } names who signed it, and no when that is not known.

core-service-driver-bundled =
    { $signed ->
        [yes] Driver { $kind } di proprietà dell'app "{ $name }" ({ $path }, firmato da { $signer }). Il programma di installazione include il relativo file .sys e registra di nuovo automaticamente il servizio del driver (sc create, richiede privilegi elevati), quindi verifica che il servizio sia registrato.
       *[no] Driver { $kind } di proprietà dell'app "{ $name }" ({ $path }, firma sconosciuta). Il programma di installazione include il relativo file .sys e registra di nuovo automaticamente il servizio del driver (sc create, richiede privilegi elevati), quindi verifica che il servizio sia registrato.
    }
core-service-driver-from-vendor =
    { $signed ->
        [yes] Driver { $kind } di un produttore o integrato in Windows "{ $name }" ({ $path }, firmato da { $signer }). WinAppMigrator non ridistribuisce driver di terze parti; reinstallalo nel computer di destinazione con il pacchetto driver firmato del produttore prima di avviare l'app.
       *[no] Driver { $kind } di un produttore o integrato in Windows "{ $name }" ({ $path }, firma sconosciuta). WinAppMigrator non ridistribuisce driver di terze parti; reinstallalo nel computer di destinazione con il pacchetto driver firmato del produttore prima di avviare l'app.
    }

## Runtimes an app needs, such as the Visual C++ runtime or .NET: how each is provided, as the Dependencies list says.

core-runtime-handling-built-in = Integrato in Windows
core-runtime-handling-ships-with-app = Fornito con l'app
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-handling-windows-feature = Funzionalità di Windows: { $feature }
core-runtime-handling-included = Incluso
# { $id } is the runtime's name in winget, such as Microsoft.VCRedist.2015+.x64.
core-runtime-handling-winget = Installato con winget: { $id }
core-runtime-handling-manual = Da installare manualmente

## Downloading a runtime's installer, when an installer is built. Each is why it could not be, and starts with a small
## letter, as it follows other words. { $url } is a web address; { $error } is what went wrong.

core-runtime-download-not-https = download di { $url } rifiutato: non usa HTTPS
# TLS is what keeps a download private.
core-runtime-tls-failed = inizializzazione TLS non riuscita: { $error }
core-runtime-download-failed = download di { $url } non riuscito: { $error }
core-runtime-download-unreadable = lettura di { $url } non riuscita: { $error }
core-runtime-download-empty = il download di { $url } non ha restituito dati
# { $came } and { $expected } are numbers of bytes.
core-runtime-download-cut-short = il download di { $url } si è interrotto prima del termine: ricevuti { $came } byte su { $expected }

## Installing a runtime: why it was not installed. Each starts with a small letter, as it follows other words.

core-runtime-no-known-installer = non esiste un programma di installazione noto per questo runtime
# { $path } is where in the package, such as prerequisites\vc_redist.x64.exe.
core-runtime-payload-misplaced = il pacchetto indica { $path } come programma di installazione di questo runtime, ma non si trova nella posizione prevista, quindi non è stato eseguito
# { $path } is a file.
core-runtime-payload-missing = manca il programma di installazione incluso: { $path }
# { $error } is what went wrong.
core-runtime-payload-path-invalid = il percorso del programma di installazione incluso non è valido: { $error }
# { $url } is the web address it can be downloaded from. winget is a program's name.
core-runtime-no-installer-route = nessun programma di installazione incluso né ID winget; scaricalo da { $url }
# { $error } is what went wrong.
core-runtime-no-temp-folder = impossibile creare una cartella in cui decomprimerlo: { $error }
# { $reason } is what the check found.
core-runtime-payload-signature-failed = il programma di installazione incluso non ha superato il controllo della firma e non è stato eseguito: { $reason }
core-runtime-payload-timed-out = il programma di installazione incluso non è terminato entro il tempo limite ed è stato interrotto
# { $error } is what went wrong.
core-runtime-payload-not-started = impossibile avviare il programma di installazione incluso: { $error }
core-runtime-installer-killed = il programma di installazione è stato interrotto senza un codice di uscita
# { $code } is the number it ended with, such as 1603.
core-runtime-installer-failed = il programma di installazione è terminato con il codice { $code }
# { $feature } is a Windows feature's name, such as NetFx3; { $code } is the number DISM ended with. Windows Features is
# the name of a Windows window. Keep the command as it is.
core-runtime-feature-failed = DISM non è riuscito ad abilitare la funzionalità di Windows { $feature } (codice di uscita { $code }); probabilmente non è stato possibile scaricare il payload. Abilitala tramite Funzionalità Windows oppure esegui `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# The same, when DISM gave no number.
core-runtime-feature-failed-no-code = DISM non è riuscito ad abilitare la funzionalità di Windows { $feature } (codice di uscita sconosciuto); probabilmente non è stato possibile scaricare il payload. Abilitala tramite Funzionalità Windows oppure esegui `dism /online /enable-feature /featurename:{ $feature } /all /Source:<install-media>\sources\sxs /LimitAccess`.
# { $feature } is a Windows feature's name, such as NetFx3.
core-runtime-feature-timed-out = l'abilitazione della funzionalità di Windows { $feature } non è terminata entro il tempo limite ed è stata interrotta
# { $feature } is a Windows feature's name; { $error } is what went wrong.
core-runtime-feature-no-dism = DISM non è disponibile per abilitare la funzionalità di Windows { $feature }: { $error }

## Installing a runtime with winget. { $id } is its name in winget, such as Microsoft.VCRedist.2015+.x64.

# { $error } is what went wrong.
core-runtime-winget-not-looked-for = impossibile cercare winget: { $error }
core-runtime-winget-not-installed = winget non è installato
# { $path } is where winget is.
core-runtime-winget-misplaced = { $path } non si trova nel percorso in cui Windows conserva i propri pacchetti, quindi non è stato eseguito
# { $path } is where winget is; { $reason } is what the check found.
core-runtime-winget-signature-failed = { $path } non ha superato il controllo della firma e non è stato eseguito: { $reason }
# { $reason } is one of the two messages above, or what went wrong.
core-runtime-winget-unusable = impossibile usare winget per installare { $id }: { $reason }
# { $code } is the number winget ended with.
core-runtime-winget-failed = winget install { $id } è terminato con il codice { $code }
core-runtime-winget-failed-no-code = winget install { $id } è terminato con un codice sconosciuto
core-runtime-winget-timed-out = winget install { $id } non è terminato entro il tempo limite ed è stato interrotto
# { $error } is what went wrong.
core-runtime-winget-not-started = winget non è disponibile per installare { $id }: { $error }

## What told a scan how an app was installed, as the package's report lists it. The names of installers and files are
## as their makers write them.

# { $path } is an installer's record of what it installed, such as C:\Program Files\App\unins000.dat.
core-source-record-not-believed = { $path } non è stato usato: può essere modificato anche da utenti diversi dagli amministratori
# { $count } is how many Windows Installer packages the bundle holds.
core-source-burn-bundle =
    { $count ->
        [one] Bundle Burn di { $count } pacchetto Windows Installer
       *[other] Bundle Burn di { $count } pacchetti Windows Installer
    }
# { $count } is how many paths Windows Installer recorded.
core-source-msi-database =
    { $count ->
        [one] Database di Windows Installer: { $count } percorso chiave di componente
       *[other] Database di Windows Installer: { $count } percorsi chiave di componenti
    }
core-source-inno-log = Log di disinstallazione di Inno Setup (unins000.dat)
# { $count } is how many paths the log records.
core-source-inno-log-paths =
    { $count ->
        [one] Log di disinstallazione di Inno Setup: { $count } percorso registrato
       *[other] Log di disinstallazione di Inno Setup: { $count } percorsi registrati
    }
core-source-nsis = Firma del programma di disinstallazione NSIS (Nullsoft)
# { $count } is how many paths the log records.
core-source-nsis-log-paths =
    { $count ->
        [one] NSIS AdvUninstLog uninstall.log: { $count } percorso registrato
       *[other] NSIS AdvUninstLog uninstall.log: { $count } percorsi registrati
    }
core-source-squirrel = Struttura per utente di Squirrel (Update.exe / app-*)
# { $version } is a folder's name that says the version, such as app-1.2.3.
core-source-squirrel-version = Versione corrente di Squirrel: { $version }
core-source-chocolatey = Metadati del pacchetto Chocolatey
core-source-msix = Pacchetto MSIX / Appx
# { $identity } is a package's name, such as Contoso.App.
core-source-appx-identity = Identità AppxManifest: { $identity }
# { $count } is how many folders.
core-source-msix-data =
    { $count ->
        [one] Dati per utente del pacchetto MSIX: { $count } percorso
       *[other] Dati per utente del pacchetto MSIX: { $count } percorsi
    }
# App Paths is a registry key's name.
core-source-app-paths = Registrazione in App Paths

## Why a runtime is needed, as the package's report says it.

# { $program } is a program's file name, such as app.exe; { $dll } is a DLL's, such as msvcp140.dll.
core-runtime-program-imports = { $program } importa { $dll }

## What else of an app's a scan finds in the registry: fonts and certificates, and why each is the app's.

# Why a font is among the app's files, in the package's list of them.
core-registry-font-recorded = Un tipo di carattere indicato nei dati di installazione dell'app
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-named-for-the-app = porta il nome dell'app o del suo autore
# Why a certificate is said to be the app's, in the app's list of what was left out.
core-registry-certificate-recorded = è indicato nei dati di installazione dell'app

## Who is signed in, for the Whose Data choice. Each of the first six is why it could not be told, and starts with a
## small letter, as it is shown inside other words. An account is shown by its id, such as S-1-5-21-...-1001.

# { $error } is what Windows said.
core-profile-desktop-owner-not-said = Windows non ha indicato a chi appartiene questo desktop: { $error }
# { $error } is what went wrong.
core-profile-desktop-owner-not-asked = impossibile chiedere a Windows a chi appartiene questo desktop: { $error }
core-profile-signed-in-account-no-profile = l'account che ha eseguito l'accesso qui, { $account }, non ha un profilo registrato
core-profile-desktop-not-a-person = il desktop qui appartiene a { $account }, che non è l'account di una persona
core-profile-nobody-signed-in-running-account-no-profile = nessuno ha eseguito l'accesso qui e l'account che esegue questo programma, { $account }, non ha un profilo registrato
core-profile-nobody-signed-in-running-not-a-person = nessuno ha eseguito l'accesso qui e { $account }, che esegue questo programma, non è l'account di una persona
# SID is the name Windows gives an account's id.
core-profile-nobody-signed-in-no-sid = nessuno ha eseguito l'accesso qui e questo account non ha un SID
# { $accounts } is a list of accounts.
core-profile-several-desktops = più di un account ha un desktop in questa sessione: { $accounts }
# Under the Whose Data choice. { $reason } is one of the messages above.
core-profile-signed-in-unknown = Impossibile determinare chi ha eseguito l'accesso ({ $reason }).
# Under the Whose Data choice, when there are accounts to choose from. { $reason } is one of the messages above.
core-profile-signed-in-unknown-choose = Impossibile determinare chi ha eseguito l'accesso ({ $reason }). Scegli di quale account includere i dati.
