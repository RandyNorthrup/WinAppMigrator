# Words the core says about what an install may do: why an install location or something in a package is refused,
# the installer's list of what an app sets up beyond its files, the checks of a package and of an install, and whether
# there is room for either. Message names start with core-.
#
# Many messages here are a reason, the second half of a sentence that starts "... was left out:" or "that install
# location cannot be used:", so they start with a small letter and have no full stop.


## Why an install location cannot be used. Shown in red under the Install Location field.

core-review-location-state-root = è la cartella in cui le installazioni conservano i registri
core-review-location-has-files = contiene già file e qui non risulta alcuna installazione, quindi usa una cartella vuota
core-policy-location-not-full-path = deve essere un percorso completo
core-policy-location-parent-step = non deve contenere ".."
core-policy-location-not-plain = deve essere un percorso semplice in un'unità di questo PC
core-policy-location-whole-drive = non può essere un'intera unità
core-policy-location-windows = non può essere la cartella Windows
core-policy-location-runs-by-itself = è una cartella che appartiene a Windows o il cui contenuto viene eseguito o caricato automaticamente
core-policy-location-shared = contiene altri elementi affiancati, quindi l'app richiede una cartella al suo interno

## Whether an app can run on this PC at all, said before anything is installed.

# { $reason } is why, as the second half of a sentence, such as "App.exe is built for Arm64, and this PC is x64, so
# Windows would not start it".
core-review-cannot-run-here = questa app non può essere eseguita in questo PC, quindi non è stato modificato nulla: { $reason }
core-policy-architecture-x86 = x86 a 32 bit
core-policy-architecture-x64 = x64
core-policy-architecture-arm64 = Arm64
core-policy-architecture-arm32 = Arm a 32 bit
core-policy-architecture-unknown = un'architettura sconosciuta
# { $code } is the number Windows gives a kind of processor, such as 0x01c4.
core-policy-machine-type = tipo di macchina { $code }
# { $names } is a list of program names, such as "a.exe, b.exe, c.exe", and { $count } how many more there are.
core-policy-names-and-more = { $names } e altri { $count }
core-policy-fit-programs-not-said = Questo programma di installazione non indica i requisiti dei suoi programmi, quindi non è stato verificato se possono essere eseguiti in questo PC.
core-policy-fit-architecture-not-told = Non è stato possibile determinare l'architettura di questo PC, quindi non è stato verificato se i programmi di questa app possono esservi eseguiti.
core-policy-fit-main-not-said = Questo programma di installazione non indica i requisiti del programma principale, quindi le indicazioni fornite qui si basano sul resto del contenuto.
# { $driver } is a driver's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-driver = il driver { $driver } è compilato per { $built_for } e questo PC è { $here }. Un driver deve corrispondere esattamente a Windows, quindi questa app non può essere eseguita qui
# { $program } is a program's file name. { $built_for } and { $here } are kinds of processor, such as "Arm64" and "x64".
core-policy-fit-program = { $program } è compilato per { $built_for } e questo PC è { $here }, quindi Windows non lo avvierebbe
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-none-start = nessuno dei suoi programmi può essere avviato in questo PC, che è { $here }: { $programs }
# { $here } is a kind of processor, such as "x64". { $programs } is a list of program names.
core-policy-fit-some-other-architecture = Alcuni programmi di questa app sono compilati per un'altra architettura e non si avvieranno in questo PC, che è { $here }: { $programs }. Il resto dell'app può avviarsi, quindi potrebbe non essere un problema.
# { $here } is a kind of processor, such as "Arm64". { $programs } is a list of program names.
core-policy-fit-emulated = Questo PC è { $here }, quindi Windows eseguirebbe questi programmi tramite emulazione: { $programs }. La maggior parte dei programmi funziona correttamente in questo modo, ma alcuni no.
# { $program } is a program's file name. { $needs } and { $runs } are versions of Windows, such as 10.0 and 6.1.
core-policy-fit-needs-newer = { $program } indica di richiedere Windows { $needs } o versioni successive e in questo PC è installata la versione { $runs }, quindi Windows non lo avvierebbe
# { $programs } is a list of program names.
core-policy-fit-some-need-newer = Alcuni programmi di questa app indicano di richiedere una versione di Windows più recente di questa e non si avvieranno qui: { $programs }.
# { $from_build } and { $this_build } are Windows build numbers, such as 22631, written as Windows writes them.
core-policy-fit-older-build = Questa app si trovava nella build { $from_build } di Windows e questo PC esegue la build { $this_build }. I suoi programmi indicano di potersi avviare in questa versione di Windows, quindi potrebbe funzionare, ma ciò su cui si basava potrebbe non essere presente qui.
core-policy-fit-build-not-told = Non è stato possibile determinare la build di Windows in uso, quindi non è stata confrontata con quella in cui si trovava questa app.

## Room on a drive for a build or an install.

# Each of these ends the sentence core-space-shortfall, after "free for".
core-space-for-unpacking = la decompressione del pacchetto
core-space-for-install-record = il registro dell'installazione
core-space-for-app-files = i file dell'app
core-space-for-what-is-replaced = la conservazione di ciò che viene sostituito
core-space-for-build-work = i file di lavoro della creazione
core-space-for-installer = il programma di installazione
core-space-for-package = il pacchetto
core-space-for-unpacking-and-installing = la decompressione e l'installazione del pacchetto
# What goes between two of those, as in "unpacking the package and the app's files". Keep the quotes: they keep the
# spaces.
core-space-purposes-joiner = { " e " }
# { $drive } is a drive, such as C:, or a folder. { $needed } and { $free } are sizes, such as 1.2 GB. { $purposes } is
# what the room is for, such as "unpacking the package and the app's files".
core-space-shortfall = { $drive } richiede fino a { $needed } di spazio libero per { $purposes } e ne ha { $free }
# { $drive } is a drive, such as C:, or a folder. { $needed } is a size, such as 1.2 GB.
core-space-not-told = { $drive } non indica lo spazio disponibile, quindi non è stato verificato se c'è spazio per i { $needed } necessari
# One drive in a list of where room is needed. { $needed } and { $free } are sizes, and { $drive } a drive, such as C:.
core-space-summary = fino a { $needed } in { $drive } ({ $free } liberi)
# { $needed } is a size, and { $drive } a drive, such as C:.
core-space-summary-not-told = fino a { $needed } in { $drive } (impossibile determinare lo spazio libero)
# What goes between two sentences of core-space-shortfall inside core-review-no-room. Keep the quotes: they keep the
# space.
core-review-sentence-joiner = { ". " }
# { $shortfalls } is one or more sentences of core-space-shortfall.
core-review-no-room = lo spazio non è sufficiente, quindi non è stato modificato nulla. { $shortfalls }. Libera spazio in quell'unità o scegli un percorso di installazione in un'altra unità.

## The installer's list of what an app sets up beyond its files: what each kind is called.

core-review-kind-service = Servizio
core-review-kind-driver = Driver
core-review-kind-scheduled-task = Attività pianificata
core-review-kind-autostart = Avvio automatico
core-review-kind-shortcut = Collegamento
core-review-kind-path = PATH
core-review-kind-environment-variable = Variabile di ambiente
core-review-kind-shell-extension = Estensione della shell
core-review-kind-menu-command = Comando di menu
core-review-kind-link-type = Tipo di collegamento
core-review-kind-file-type = Tipo di file
core-review-kind-com = COM
core-review-kind-firewall-rule = Regola del firewall
core-review-kind-event-log-source = Origine del registro eventi
core-review-kind-font = Tipo di carattere
core-review-kind-odbc-driver = Driver ODBC
core-review-kind-odbc-source = Origine dati ODBC
core-review-kind-compatibility-setting = Impostazione di compatibilità
core-review-kind-default-apps = App predefinite
core-review-kind-program-name = Nome del programma
core-review-kind-other-settings = Altre impostazioni

## The installer's list of what an app sets up: what each line says of it.

# { $file } is a program's file name. { $account } is the Windows account it runs as, such as LocalSystem. { $starts }
# says when it starts, and is not shown itself; for "other", { $start } is one of the core-review-start- words below.
core-review-service-detail =
    { $starts ->
        [automatic] Esegue { $file } come { $account }, si avvia ora e a ogni avvio di questo PC
        [delayed] Esegue { $file } come { $account }, si avvia ora e poco dopo ogni avvio di questo PC
        [manual] Esegue { $file } come { $account }, si avvia solo quando richiesto
        [disabled] Esegue { $file } come { $account }, è disabilitato
        [other] Esegue { $file } come { $account }, { $start }
       *[unknown] Esegue { $file } come { $account }
    }
# { $account } is a Windows account, such as LocalService.
core-policy-account-custom = { $account } (account personalizzato non ripristinato)
# { $file } is a driver's file name.
core-review-driver-detail = Carica { $file } direttamente in Windows
# { $file } is a driver's file name. { $start } is when Windows starts it: one of the core-review-start- words below.
core-review-driver-detail-starts = Carica { $file } direttamente in Windows, { $start }
# When a service or driver starts, ending a sentence such as "Loads disk.sys into Windows itself, starts …".
core-review-start-boot = si avvia con Windows, prima di tutto il resto
core-review-start-system = si avvia con Windows, durante l'avvio del sistema
core-review-start-automatic = si avvia a ogni avvio di questo PC
core-review-start-delayed = si avvia poco dopo ogni avvio di questo PC
core-review-start-manual = si avvia solo quando richiesto
core-review-start-disabled = non si avvia mai, perché è disabilitato
core-review-start-unknown = si avvia in un modo che Windows non specifica
# { $account } is the account a task runs as: an account's name, or core-review-task-runs-as-you, or
# core-review-task-runs-as-creator.
core-review-task-detail = Viene eseguita come { $account }
# { $account } is the account a task runs as, as for core-review-task-detail.
core-review-task-detail-highest = Viene eseguita come { $account }, con i privilegi più elevati
core-review-task-runs-as-you = te, ovvero chiunque abbia eseguito l'accesso
core-review-task-runs-as-creator = l'utente che l'ha creata
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-autostart-detail =
    Si avvia { $scope ->
        [everyone] quando accede qualsiasi utente
       *[you] quando accedi
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shortcut-startup-detail =
    Un collegamento nella cartella Esecuzione automatica, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $file } is the file name of what the shortcut opens. { $scope } is whom it is set up for: everyone on this PC,
# or you. It is not shown itself.
core-review-shortcut-detail =
    Apre { $file }, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $name } is an environment variable's name, such as PATH.
core-review-path-title = Aggiunge a { $name }
core-review-path-install-folder = La cartella di installazione
# { $path } is a folder inside the install folder, such as bin\tools.
core-review-path-below-install-folder = La cartella di installazione\{ $path }
# { $value } is the variable's value, as it will be set.
core-review-variable-detail = Impostata su { $value }
# { $count } is how many registry values.
core-review-other-settings-title =
    { $count ->
        [one] { $count } impostazione esterna alle chiavi dell'app
       *[other] { $count } impostazioni esterne alle chiavi dell'app
    }
# { $keys } is a list of registry keys, such as HKCU\Software\Vendor.
core-review-other-settings-detail = In { $keys }
# { $keys } is a list of registry keys, and { $count } how many more there are.
core-review-other-settings-detail-more = In { $keys } e altre { $count }
# { $way } says whether the rule allows or blocks, and incoming or outgoing connections; it is not shown itself.
# { $ports } is a list of port numbers, and { $port_count } how many. { $program } is the program's file name.
# { $networks } is a list of the kinds of network Windows names in English (domain, private, public), and
# { $network_count } how many.
# The kinds of network a firewall rule is for, as Windows Defender Firewall names its profiles in your language. They
# fill { $networks } in core-review-firewall-detail, joined into a list.
core-review-network-domain = Dominio
core-review-network-private = Privato
core-review-network-public = Pubblico
core-review-firewall-detail =
    { $way ->
        [allow-in] Consente le connessioni in entrata
        [allow-out] Consente le connessioni in uscita
        [block-in] Blocca le connessioni in entrata
       *[block-out] Blocca le connessioni in uscita
    }{ $port_count ->
        [0] { "" }
        [one] { " " }sulla porta { $ports }
       *[other] { " " }sulle porte { $ports }
    } per { $program }, { $network_count ->
        [0] su tutte le reti
        [one] nel profilo di rete { $networks }
       *[other] nei profili di rete { $networks }
    }
# { $server } is the file name of what Explorer loads. { $scope } is whom it is set up for: everyone on this PC, or
# you. It is not shown itself.
core-review-shell-extension-detail =
    Carica { $server } in Esplora file, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-shell-extension-detail-own-file =
    Carica in Esplora file un file installato dall'app, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# The name of a shell extension: { $handler } is its name in the registry, and { $class } the kind of file or folder it
# is for, such as * or Directory.
core-policy-shell-extension-name = { $handler } per { $class }
# { $on } is the kind of file or folder whose menu it is, as the registry names it, such as * or
# Directory. { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-menu-command-detail =
    Aggiunge un comando al menu di { $on }, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-default-verb-detail =
    Cambia cosa succede quando si apre uno di questi elementi, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
core-review-client-entry-detail = Può essere scelto come programma per questa funzione. Non viene modificato nulla
core-review-com-permissions-title = Autorizzazioni COM
core-review-com-permissions-detail = Stabilisce chi può avviare le sue classi COM o con quale account vengono eseguite
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-link-type-detail =
    I collegamenti di questo tipo si aprono con l'app, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-file-type-detail =
    I file di questo tipo si aprono con l'app, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $log } is the name of a Windows event log, such as Application.
core-review-event-log-source-detail = Può scrivere nel registro { $log } con messaggi presi dai propri file
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-font-detail =
    Viene caricato dalla cartella di installazione all'accesso, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
core-review-odbc-driver-detail = Viene caricato nei programmi che usano ODBC per accedere a un database
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-odbc-source-detail =
    Una connessione a un database, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
core-review-default-apps-detail = Può essere scelto in App predefinite. Non viene modificato nulla
# { $scope } is whom it is set up for: everyone on this PC, or you. It is not shown itself.
core-review-program-name-detail =
    Esegui e altri programmi possono trovarlo con questo nome, { $scope ->
        [everyone] per tutti gli utenti di questo PC
       *[you] per te
    }
# { $server } is the file name of what serves the classes.
core-review-com-title = COM in { $server }
core-review-com-title-own-file = COM in un file installato dall'app
# { $count } is how many registry values.
core-review-com-detail =
    { $count ->
        [one] { $count } impostazione delle classi che altri programmi possono avviare
       *[other] { $count } impostazioni delle classi che altri programmi possono avviare
    }
core-review-com-support-title = Librerie dei tipi, interfacce e AppID COM
# { $libraries }, { $interfaces }, and { $app_ids } are how many registry values there are for its type libraries, its
# interfaces, and its AppIDs.
core-review-com-support-detail = Impostazioni delle sue librerie dei tipi ({ $libraries }), interfacce ({ $interfaces }) e AppID ({ $app_ids }), che accompagnano le sue classi
core-review-compatibility-title = Modalità di avvio dei programmi
# { $count } is how many registry values.
core-review-compatibility-detail =
    { $count ->
        [one] { $count } impostazione di compatibilità per i programmi che installa
       *[other] { $count } impostazioni di compatibilità per i programmi che installa
    }

## The installer's notes on whose folders and settings these are, and what is not coming.

# { $account } is the name of a Windows account, such as alice.
core-review-account-signed-in = I dati utente e le impostazioni per utente vanno a { $account }, che ha eseguito l'accesso qui, e non all'amministratore con cui è in esecuzione questo programma.
# { $account } is the name of a Windows account, such as alice.
core-review-account-no-desktop = Nessuno ha eseguito l'accesso a questa sessione, quindi i dati utente e le impostazioni per utente vanno a { $account }, l'account con cui è in esecuzione questo programma.
# { $reason } is what Windows said, in its own words.
core-review-account-not-told = Impossibile determinare chi ha eseguito l'accesso ({ $reason }), quindi le impostazioni per utente vengono escluse e i dati utente vanno nel percorso di installazione.
# { $reason } is what Windows said, in its own words.
core-review-signed-in-unknown = impossibile determinare chi ha eseguito l'accesso: { $reason }
# Headings of what is left out, shared by the app's list and the installer's notes.
core-review-left-out-credentials = Non inclusi in questo programma di installazione, perché sembrano credenziali
core-review-left-out-settings = Impostazioni e cronologia che un account conserva per l'app, incluse solo con l'opzione Dati utente
core-review-left-out-certificates = L'app sembra basarsi su certificati considerati attendibili nel suo PC, che non vengono installati
core-review-left-out-fonts = Questi tipi di carattere portano il nome dell'app, ma nulla indica che le appartengano, quindi non vengono installati
# { $names } is a list of names, such as the names of saved passwords.
core-review-note-credentials = Non inclusi in questo programma di installazione, perché sembrano credenziali: { $names }. Quelli già presenti in questo PC restano invariati e potrebbe essere necessario immetterli di nuovo.
# { $names } is a list of names of license data.
core-review-note-licenses = Non inclusi in questo programma di installazione, perché l'opzione Dati di licenza e attivazione era disattivata al momento della creazione: { $names }.
# { $names } is a list of places an account keeps settings, such as folders.
core-review-note-settings = Non incluse in questo programma di installazione, perché l'opzione Dati utente era disattivata al momento della creazione: le impostazioni e la cronologia che un account conserva per l'app, in { $names }. Qui l'app si avvia come per chi la usa per la prima volta.
# { $names } is a list of certificates, each as core-review-certificate.
core-review-note-certificates = L'app sembra basarsi su certificati considerati attendibili nel suo PC, che non vengono installati: { $names }. Spetta a te decidere se considerarli attendibili qui.
# { $names } is a list of font names.
core-review-note-fonts = Questi tipi di carattere portano il nome dell'app, ma nulla indica che le appartengano, quindi non vengono installati: { $names }.
# { $names } is a list of names.
core-review-note-left-out-by-choice = Esclusi da chi ha creato questo programma di installazione: { $names }.
# { $names } is a list of names.
core-review-note-added-by-choice = Aggiunti da chi ha creato questo programma di installazione, perché li ha ritenuti parte dell'app: { $names }.
# { $name } is a certificate's subject, such as CN=Contoso, and { $store } the certificate store it was in, such as
# Root.
core-review-certificate = { $name } in { $store }
# { $names } is a list of names, and { $count } how many more there are.
core-review-names-and-more = { $names } e altri { $count }

## Why something in a package is left out. Each is the second half of a sentence of core-warning-review-left-out.

core-review-no-account = appartiene alle impostazioni di un account e non è stato possibile determinare chi ha eseguito l'accesso
core-review-name-not-accepted = Windows non accetta il suo nome
# { $error } is what Windows said, in its own words.
core-review-service-unreadable = impossibile leggere le sue impostazioni correnti: { $error }
core-review-service-dependency = dipende da un servizio con un nome non valido
# { $path } is the program the service runs.
core-review-service-runs-other = esegue { $path }, che questo pacchetto non installa
core-review-service-settings = le sue impostazioni fanno riferimento a qualcosa che questo pacchetto non installa o a una chiave che non è sua
core-review-driver-no-hash = non ha un hash con cui verificarlo
core-review-license-for-the-machine = una licenza archiviata per l'intero computer viene solo segnalata
core-review-license-starts-something = una licenza non ha motivo di avviare qualcosa
core-review-task-not-captured = la sua definizione non è stata acquisita
# { $error } is what was wrong with the task's XML, in English.
core-review-task-unreadable = impossibile leggere la sua definizione: { $error }
core-review-task-no-account = viene eseguita come l'utente che la installa e non è stato possibile determinare chi ha eseguito l'accesso qui
# { $path } is what the shortcut opens.
core-review-shortcut-opens-other = apre { $path }, che questo pacchetto non installa
# { $service } is the service's name, and { $path } the program it runs now.
core-policy-service-exists = { $service } esiste già in questo PC ed esegue { $path }, che non proviene da un'installazione precedente eseguita qui. Se è una copia meno recente di questa app, disinstallala prima
core-policy-key-control-characters = il nome della chiave contiene caratteri di controllo
core-policy-value-control-characters = il nome del valore contiene caratteri di controllo
core-policy-registry-link = è un collegamento simbolico del Registro di sistema
# { $reason } is why the key it stands for is left out, such as "no app install should change this key".
core-policy-virtual-store = rappresenta una chiave del PC e { $reason }
core-policy-registry-part = nessuna installazione di app dovrebbe scrivere in questa parte del Registro di sistema
core-policy-key-refused = nessuna installazione di app dovrebbe modificare questa chiave
core-policy-net-debugger = imposterebbe un debugger per ogni programma .NET
core-policy-net-install-root = cambierebbe il percorso da cui ogni programma .NET carica .NET
core-policy-class-redirects = cambierebbe il modo in cui viene aperto un tipo di programma o di cartella
core-policy-app-path-name = indica un programma con qualcosa di diverso da un nome di file
core-policy-app-path-not-installed = registrerebbe un nome di programma per un file che questo pacchetto non installa
core-policy-shared-default-verb = cambierebbe cosa succede quando si apre qualsiasi file o cartella di un determinato tipo
core-policy-command-runs-other = aggiunge un comando che esegue qualcosa che questo pacchetto non installa
core-policy-autostart-not-command = si avvia automaticamente e il suo valore non è un comando
core-policy-autostart-runs-other = si avvia automaticamente ed esegue qualcosa che questo pacchetto non installa
core-policy-environment-own-rules = le variabili di ambiente vengono ripristinate secondo regole specifiche
core-policy-service-not-restored = appartiene a un servizio che questo pacchetto non ripristina
core-policy-service-own-rules = le impostazioni di un servizio vengono ripristinate solo tramite le regole dei servizi
core-policy-service-dll = il suo ServiceDll non è un file installato da questo pacchetto
core-policy-event-log-source-only = è possibile ripristinare solo i valori propri di un'origine del registro eventi
core-policy-security-log = nessuna app scrive nel registro Sicurezza
core-policy-event-log-messages = i suoi messaggi proverrebbero da un file che questo pacchetto non installa
core-policy-type-library = la sua libreria dei tipi non è un file installato da questo pacchetto
# { $name } is a program's file name, such as app.exe.
core-policy-application-opened = cambierebbe il modo in cui viene aperto { $name }, che questo pacchetto non installa
core-policy-event-log-no-file = un'origine del registro eventi deve prendere i messaggi da un file
core-policy-com-runs-other = esegue qualcosa che non è un file installato da questo pacchetto
core-policy-com-redirect = trasformerebbe questa classe in qualcosa che questo pacchetto non installa
core-policy-com-server = esegue qualcosa che non è né un file installato da questo pacchetto né parte di Windows
core-policy-interface = deciderebbe come vengono effettuate le chiamate a un'interfaccia, tramite qualcosa che questo pacchetto non installa
# { $name } is a program's file name, such as app.exe.
core-policy-app-id-started = cambierebbe il modo in cui viene avviato { $name }, che questo pacchetto non installa
core-policy-app-id-surrogate = sarebbe ospitato da un programma che questo pacchetto non installa
core-policy-app-id-service = verrebbe eseguito in un servizio che questo pacchetto non ripristina
core-policy-app-id-run-as = verrebbe eseguito con un proprio account
core-policy-app-id-link = non indica un AppID
core-policy-app-id-remote = verrebbe eseguito in un computer diverso da questo PC
core-policy-odbc-own-driver = è un'impostazione propria di ODBC e non un driver
core-policy-odbc-driver-not-installed = il suo driver non è un file installato da questo pacchetto
core-policy-loads-other-file = caricherebbe un file che questo pacchetto non installa
core-policy-odbc-own-source = è un'impostazione propria di ODBC e non un'origine dati
core-policy-odbc-source-missing = elenca un'origine dati che questo pacchetto non contiene
core-policy-odbc-source-driver = il suo driver non è né un file installato da questo pacchetto né parte di Windows
core-policy-shell-extension = caricherebbe in Esplora file un'estensione della shell che non è un file installato da questo pacchetto
core-policy-not-text = il suo contenuto non è testo, mentre verrebbe letto come tale
core-policy-network-place = fa riferimento a un percorso di rete
core-policy-font = registrerebbe un tipo di carattere che questo pacchetto non installa o che si trova nella cartella Windows
core-policy-compatibility = cambierebbe il modo in cui viene eseguito un programma che questo pacchetto non installa
core-policy-capabilities = elenca un'app i cui dettagli non sono in questo pacchetto
core-policy-default-program = cambierebbe il programma che apre gli elementi per impostazione predefinita
core-policy-url-protocol = si dichiara un tipo di collegamento, ma non si apre con nulla di ciò che questo pacchetto installa
core-policy-command-handler = aggiunge un comando gestito da qualcosa che questo pacchetto non installa
core-policy-installer-command = eseguirebbe qualcosa che solo Windows Installer può leggere
core-policy-creates-class = creerebbe una classe che questo pacchetto non installa
core-policy-opens-links = apre i collegamenti con qualcosa che questo pacchetto non installa
core-policy-opens-files = apre i file con qualcosa che questo pacchetto non installa
core-policy-firewall-name = il suo nome non può essere trasferito
core-policy-firewall-description = la sua descrizione non può essere trasferita
core-policy-firewall-every-program = si applica a tutti i programmi, mentre una regola può riguardarne solo uno
# { $path } is the program the rule is for.
core-policy-firewall-program = riguarda { $path }, che questo pacchetto non installa
core-policy-firewall-direction = non indica una direzione che è possibile impostare
core-policy-firewall-action = avrebbe la precedenza sulle altre regole, e la regola di un'app non dovrebbe mai averla
# { $setting } is a part of a firewall rule, as the scan of its PC described it.
core-policy-firewall-not-carried = indica { $setting }, che non è stato possibile impostare così com'era o che indebolirebbe l'effetto di altre regole
core-policy-firewall-service = riguarda un servizio che questo pacchetto non ripristina
core-policy-firewall-parts = una sua parte non può essere espressa in una regola del firewall
core-policy-folder-already-there = quella cartella era già presente, all'esterno del percorso di installazione, e nulla indica che appartenga a questa app, quindi non vi viene inserito alcun programma
core-policy-variable-name = il suo nome non è un nome di variabile valido
core-policy-variable-relied-on = Windows e altri programmi si basano su questa variabile
core-policy-task-name = il suo nome non è accettato da Windows
core-policy-task-folder = la sua cartella non è accettata da Windows
core-policy-task-windows = sostituirebbe una delle attività di Windows
core-policy-task-other-action = esegue un'azione diversa dall'esecuzione di un programma
core-policy-task-no-program = non indica alcun programma da eseguire
# { $path } is the program the task runs.
core-policy-task-runs-other = esegue qualcosa che questo pacchetto non installa: { $path }
core-policy-task-exists-unreadable = esiste già un'attività con questo nome e non è stato possibile leggerla
core-policy-task-exists-other = esiste già un'attività con questo nome e non esegue l'app
# { $path } is the program the task already on this PC runs.
core-policy-task-exists-runs = esiste già un'attività con questo nome ed esegue { $path }, che non proviene da un'installazione precedente eseguita qui
core-policy-shortcut-not-lnk = non è un collegamento .lnk
core-policy-shortcut-place = non si trova in un menu Start o sul desktop
core-policy-short-name = il suo percorso usa un nome che non può essere verificato
core-policy-shortcut-no-folder = questo PC non ha una cartella corrispondente
core-policy-data-startup = andrebbe in una cartella Esecuzione automatica e verrebbe eseguito a ogni accesso
core-policy-data-powershell = andrebbe nel percorso in cui PowerShell esegue ciò che trova all'avvio
core-policy-data-windows = andrebbe in una cartella che appartiene a Windows
core-policy-license-outside = si trova all'esterno del percorso di installazione e di ogni cartella di dati nota
core-rewrite-cdata = contiene una sezione CDATA
core-rewrite-too-much-to-look-through = questo pacchetto contiene troppe impostazioni per verificare ciò a cui fanno riferimento

## Warnings a review gives, which the installer shows with what it sets up.

# { $kind } says what was left out, and is not shown itself. { $item } is its path or name, or a registry value such
# as HKLM\Software\App\Name. { $reason } is why, as the second half of the sentence, such as "it cannot be a whole
# drive".
core-warning-review-left-out =
    { $kind ->
        [file] Il file { $item } è stato escluso: { $reason }.
        [folder] La cartella { $item } è stata esclusa: { $reason }.
        [service] Il servizio { $item } è stato escluso: { $reason }.
        [driver] Il driver { $item } è stato escluso: { $reason }.
        [firewall-rule] La regola del firewall { $item } è stata esclusa: { $reason }.
        [license-data] I dati di licenza in { $item } sono stati esclusi: { $reason }.
        [license-value] Il valore di licenza { $item } è stato escluso: { $reason }.
        [scheduled-task] L'attività pianificata { $item } è stata esclusa: { $reason }.
        [shortcut] Il collegamento { $item } è stato escluso: { $reason }.
        [environment-variable] La variabile di ambiente { $item } è stata esclusa: { $reason }.
       *[value] { $item } è stato escluso: { $reason }.
    }
core-warning-review-package-path-missing = Un file copiabile non ha un percorso relativo nel pacchetto.
# { $count } is the most empty folders a package makes.
core-warning-review-too-many-empty-folders = Questo pacchetto elenca più di { $count } cartelle vuote e ne vengono create solo { $count }.
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Documents folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-documents-elsewhere =
    { $count ->
        [one] { $count } file della cartella Documenti va invece in { $target }, in una cartella denominata _external, perché la cartella Documenti di questo PC si trova in { $folder } e i file vengono scritti solo in cartelle all'interno del tuo profilo. Per usarlo, spostalo tu stesso nella cartella Documenti.
       *[other] { $count } file della cartella Documenti vanno invece in { $target }, in una cartella denominata _external, perché la cartella Documenti di questo PC si trova in { $folder } e i file vengono scritti solo in cartelle all'interno del tuo profilo. Per usarli, spostali tu stesso nella cartella Documenti.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-documents-unknown =
    { $count ->
        [one] { $count } file della cartella Documenti va invece in { $target }, in una cartella denominata _external, perché non è stato possibile determinare dove si trova la cartella Documenti di questo PC. Per usarlo, spostalo tu stesso nella cartella Documenti.
       *[other] { $count } file della cartella Documenti vanno invece in { $target }, in una cartella denominata _external, perché non è stato possibile determinare dove si trova la cartella Documenti di questo PC. Per usarli, spostali tu stesso nella cartella Documenti.
    }
# { $count } is how many files. { $target } is the install location, and { $folder } where this PC's Saved Games folder
# is. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-elsewhere =
    { $count ->
        [one] { $count } file della cartella Giochi salvati va invece in { $target }, in una cartella denominata _external, perché la cartella Giochi salvati di questo PC si trova in { $folder } e i file vengono scritti solo in cartelle all'interno del tuo profilo. Per usarlo, spostalo tu stesso nella cartella Giochi salvati.
       *[other] { $count } file della cartella Giochi salvati vanno invece in { $target }, in una cartella denominata _external, perché la cartella Giochi salvati di questo PC si trova in { $folder } e i file vengono scritti solo in cartelle all'interno del tuo profilo. Per usarli, spostali tu stesso nella cartella Giochi salvati.
    }
# { $count } is how many files, and { $target } the install location. Keep _external as it is: it is a folder's name.
core-warning-review-saved-games-unknown =
    { $count ->
        [one] { $count } file della cartella Giochi salvati va invece in { $target }, in una cartella denominata _external, perché non è stato possibile determinare dove si trova la cartella Giochi salvati di questo PC. Per usarlo, spostalo tu stesso nella cartella Giochi salvati.
       *[other] { $count } file della cartella Giochi salvati vanno invece in { $target }, in una cartella denominata _external, perché non è stato possibile determinare dove si trova la cartella Giochi salvati di questo PC. Per usarli, spostali tu stesso nella cartella Giochi salvati.
    }

## Errors when a folder that only administrators may change cannot be made so, or checked.

# { $path } is a folder.
core-privilege-not-a-folder = { $path } è un collegamento simbolico o un file, non una cartella
# { $path } is a folder. { $error } is what Windows said, in its own words.
core-privilege-not-made-here = La cartella { $path } non è stata creata da WinAppMigrator e altri possono modificarne il contenuto. Un amministratore dovrebbe controllarla e rimuoverla: { $error }
# { $path } is a folder.
core-privilege-others-put-something-in = La cartella { $path } contiene qualcosa che non è stato creato da WinAppMigrator e che altri possono modificare. Un amministratore dovrebbe controllarla e rimuoverla
# { $path } is a folder.
core-privilege-became-a-link-in-use = La cartella { $path } è diventata un collegamento simbolico mentre veniva usata
# { $path } is a folder.
core-privilege-became-a-link-looked-at = La cartella { $path } è diventata un collegamento simbolico mentre veniva esaminata
# { $path } is a file. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-users-cannot-start = impossibile consentire agli utenti di avviare { $path }: { $said }
core-privilege-elevation-unknown = impossibile determinare se questo processo dispone di privilegi elevati
# { $path } is a folder. { $said } is what Windows' icacls tool printed, in its own words.
core-privilege-cannot-restrict = impossibile limitare agli amministratori l'accesso a { $path }: { $said }
# { $path } is a folder. { $said } is the folder's permissions as Windows wrote them, and what else it printed.
core-privilege-not-restricted = impossibile limitare agli amministratori l'accesso a { $path } e l'unità potrebbe non conservare le autorizzazioni: { $said }
# { $path } is a folder.
core-privilege-written-to-first = un altro processo ha scritto in { $path } prima che fosse possibile proteggerla
core-privilege-no-program-data = Windows non indica dove si trova ProgramData

## Checks of a package's files, and of what an install put in place. Each is said of one file, value, service, or the
## like, which the report names beside it.

# { $path } is the installer's file.
core-verify-no-package = { $path } non contiene alcun pacchetto
# { $path } is a folder.
core-verify-not-unpacked = il pacchetto di questo programma di installazione non è stato decompresso in { $path }
core-verify-schema-supported = Lo schema del manifesto del pacchetto è supportato.
core-verify-packaged-file-missing = Manca il file nel pacchetto.
core-verify-no-package-path = Il file copiabile non ha un percorso relativo nel pacchetto.
core-verify-folder-there = La cartella vuota è presente.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-unreadable = Impossibile leggerlo per verificarlo: { $error }
core-verify-restored-file-missing = Manca il file ripristinato.
# { $expected } and { $actual } are sizes in bytes.
core-verify-size-mismatch = Le dimensioni del file non corrispondono. Previste: { $expected } byte, trovate: { $actual } byte.
core-verify-no-hash = Il manifesto non registra alcun hash per questo file, quindi non è possibile verificarlo.
core-verify-hash-mismatch = L'hash SHA-256 non corrisponde.
core-verify-file-matches = Dimensioni e hash del file corrispondono al manifesto.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-payload-failed = Verifica del payload della licenza non riuscita: { $error }
core-verify-license-payload-missing = Manca il payload della licenza nel pacchetto.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-failed = Verifica dell'archivio licenze non riuscita: { $error }
core-verify-license-store-file-missing = Manca il file dell'archivio licenze nel pacchetto.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-license-store-path-invalid = Il percorso dell'archivio licenze nel pacchetto non è valido: { $error }
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-prerequisite-failed = Verifica del payload del prerequisito non riuscita: { $error }
core-verify-prerequisite-missing = Manca il payload del prerequisito incluso.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-failed = Verifica dell'immagine del driver non riuscita: { $error }
core-verify-bundled-driver-missing = Manca l'immagine del driver inclusa.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-path-invalid = Il percorso del driver incluso non è valido: { $error }
core-verify-prerequisite-present = Il prerequisito di runtime è presente nel computer di destinazione.
core-verify-prerequisite-not-detected = Prerequisito di runtime non rilevato; potrebbe richiedere un riavvio o il completamento manuale (vedi il log del ripristino per l'esito definitivo).
core-verify-prerequisite-not-probed = Non è possibile verificare la presenza del prerequisito tramite il Registro di sistema; l'installazione è stata eseguita senza garanzia di riuscita.
core-verify-license-file-missing = Manca il file di licenza ripristinato.
core-verify-license-store-restored-missing = Manca il file dell'archivio licenze ripristinato.
core-verify-registry-key-there = La chiave del Registro di sistema esiste.
core-verify-registry-value-matches = Il valore del Registro di sistema corrisponde ai dati archiviati.
core-verify-registry-value-differs = Il valore del Registro di sistema non corrisponde ai dati archiviati.
core-verify-firewall-rule-there = Windows Firewall contiene questa regola.
core-verify-variable-matches = Il valore della variabile di ambiente corrisponde ai dati archiviati.
core-verify-variable-differs = Il valore della variabile di ambiente non corrisponde ai dati archiviati.
# { $error } is what Windows said, in its own words.
core-verify-variable-unreadable = La variabile di ambiente è mancante o illeggibile: { $error }
core-verify-service-runs-other = Il servizio esiste ma non esegue il file binario ripristinato con i relativi argomenti.
core-verify-service-other-account = Il servizio esiste ma non viene eseguito con l'account previsto.
core-verify-service-matches = Il servizio esiste ed esegue il file binario ripristinato con i relativi argomenti e account.
core-verify-service-missing = Manca il servizio.
# { $error } is what Windows said, in its own words.
core-verify-service-unreadable = Il servizio è illeggibile: { $error }
core-verify-driver-other-image = Il servizio del driver non punta all'immagine ripristinata.
core-verify-driver-not-registered = Il servizio del driver incluso non è registrato dopo il ripristino.
# { $error } is what Windows said, in its own words.
core-verify-driver-unreadable = Il servizio del driver è illeggibile: { $error }
core-verify-driver-image-missing = Manca l'immagine del driver.
# { $error } is what went wrong, in Windows' or the program's own words.
core-verify-driver-image-unchecked = Impossibile verificare l'immagine del driver: { $error }
core-verify-task-matches = L'attività pianificata esiste ed esegue i programmi ripristinati.
core-verify-task-runs-other = L'attività pianificata esiste ma non esegue i programmi ripristinati.
core-verify-task-missing = Manca l'attività pianificata.
# { $error } is what Windows said, in its own words.
core-verify-task-unreadable = Interrogazione dell'attività pianificata non riuscita: { $error }
core-verify-shortcut-matches = Il collegamento esiste e il percorso di destinazione corrisponde a quello ripristinato.
core-verify-shortcut-opens-other = Il collegamento esiste ma il percorso di destinazione non corrisponde a quello ripristinato.
core-verify-shortcut-missing = Manca il collegamento.
# { $error } is what Windows said, in its own words.
core-verify-shortcut-unreadable = Interrogazione del collegamento non riuscita: { $error }

## Warnings and errors a check gives, which the app and the installer show.

core-warning-verify-schema-unsupported = Lo schema del manifesto del pacchetto non è supportato.
# { $error } is what went wrong, in Windows' or the program's own words.
core-warning-verify-package-file-failed = Verifica del file del pacchetto non riuscita: { $error }
core-warning-verify-folder-missing = Manca la cartella vuota.
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-key-missing = La chiave del Registro di sistema è mancante o illeggibile: { $error }
# { $error } is what Windows said, in its own words.
core-warning-verify-registry-value-missing = Il valore del Registro di sistema è mancante o illeggibile: { $error }
core-warning-verify-registry-kind-missing = Manca il tipo del valore del Registro di sistema.
core-warning-verify-firewall-rule-missing = Windows Firewall non contiene questa regola.
core-warning-verify-service-scope = La verifica dei servizi legge la configurazione dei servizi in HKLM e non avvia i servizi.
core-warning-verify-driver-manual = Uno o più driver di produttori o integrati in Windows non vengono ridistribuiti da WinAppMigrator; reinstallali nel computer di destinazione con il pacchetto driver firmato del produttore.
core-warning-verify-task-scope = La verifica delle attività pianificate controlla solo la registrazione e il riferimento al comando.

## The verification report written beside a package, verification_report.md. Its headings follow a "#".

core-verify-report-title = Report di verifica di WinAppMigrator
# { $time } is when, as a date and time in UTC, such as 2026-11-15T10:00:00Z.
core-verify-report-verified = Verificato (UTC): { $time }
# { $valid } is true or false, which are not translated.
core-verify-report-package-valid = Pacchetto valido: { $valid }
# { $valid } is true or false, which are not translated.
core-verify-report-restore-valid = Ripristino valido: { $valid }
core-verify-report-restore-not-run = Ripristino valido: non eseguito
core-verify-report-items = Elementi
# { $message } is what a check found, and { $source } the file or setting it looked at.
core-verify-report-item-from = { $message } Origine: { $source }
core-verify-report-warnings = Avvisi
core-verify-report-errors = Errori
core-verify-report-none = Nessuno.
