# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Trasforma un'app installata in un programma di installazione autoestraente
app-theme-to-light = Passa al tema chiaro
app-theme-to-dark = Passa al tema scuro
app-reload-list = Ricarica l'elenco delle app
app-list-loading = Caricamento
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } app
       *[other] { $count } app
    }
app-list-load-failed = Caricamento non riuscito
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Impossibile aprire la finestra: { $error }

## The list of installed apps

app-list-search = Cerca per nome o autore  (Ctrl+F)
app-list-filter-all = Tutte
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Per utente
app-scope-per-machine = Per computer
app-scope-unknown = Sconosciuto
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Sconosciuta
app-list-loading-apps = Caricamento delle app installate…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Impossibile elencare le app: { $error }
app-list-no-matches = Nessuna app corrispondente
app-list-no-matches-hint = Modifica il testo di ricerca o il filtro per ambito.
app-list-worker-disconnected = Il processo di inventario si è disconnesso.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, versione { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Nome
app-column-name-sort = Ordina per nome
app-column-publisher = Autore
app-column-publisher-sort = Ordina per autore
app-column-version = Versione
app-column-version-sort = Ordina per versione
app-column-scope = Ambito
app-column-scope-sort = Ordina per ambito
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch.
app-column-arch-sort = Ordina per architettura

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Origine dell'analisi
app-source-this-pc = Questo PC
app-source-drive = Unità collegata…
app-source-image = Immagine disco…
app-source-network = PC in rete…
app-source-attaching-image = Montaggio dell'immagine in sola lettura…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } in rete ({ $address })
app-source-disconnect = Disconnetti
app-source-change = Cambia…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Seleziona un'immagine disco dell'altro PC (.vhd o .vhdx)
app-source-pick-image-filter = Immagini disco
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } non contiene alcun volume con Windows.
app-source-image-not-attached = Impossibile montare l'immagine disco.
app-source-pick-drive-title = Seleziona la radice dell'unità collegata (ad es. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } non sembra un'unità di sistema Windows (mancano \Windows e \Users). Scegli la radice dell'unità proveniente dall'altro PC.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Contiene { $windows }. I programmi di installazione creati da qui sono destinati a Windows 10 e 11. Con Dati utente vengono incluse anche le impostazioni che Windows ha messo da parte automaticamente per un'app meno recente, e i runtime che Windows 11 non include, come DirectX 9, vengono installati con l'app.{ $is_32_bit ->
        [yes] { " " }Le sue app sono a 32 bit, quindi vanno in Program Files (x86), dove le conserva Windows a 64 bit.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Contiene una versione precedente di Windows. I programmi di installazione creati da qui sono destinati a Windows 10 e 11. Con Dati utente vengono incluse anche le impostazioni che Windows ha messo da parte automaticamente per un'app meno recente, e i runtime che Windows 11 non include, come DirectX 9, vengono installati con l'app.{ $is_32_bit ->
        [yes] { " " }Le sue app sono a 32 bit, quindi vanno in Program Files (x86), dove le conserva Windows a 64 bit.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Associa il vecchio PC
app-details-pair-first-hint = Quando WinAppMigrator Client è in esecuzione nel vecchio PC e hai digitato il suo codice, le app di quel PC vengono elencate qui e puoi creare un programma di installazione da una di esse, come da questo PC.
app-details-pick-an-app = Scegli un'app
app-details-pick-an-app-hint = Seleziona un'app nell'elenco per scegliere cosa includere e creare il relativo programma di installazione.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, versione { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Autore sconosciuto, versione { $version }
app-details-unknown-publisher = Autore sconosciuto

## What goes in besides the program

app-option-license = Dati di licenza e attivazione
app-option-license-hint = Potrebbero essere legati a questo PC e richiedere una nuova attivazione in quello nuovo.
app-option-user-data = Dati utente
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Impostazioni e cronologia, Documenti, Giochi salvati e ProgramData.
app-option-whose-data = Dati di quale account
app-option-whose-data-hint = Un programma di installazione contiene i dati di un solo account, che vanno all'utente che ha eseguito l'accesso nel PC in cui viene eseguito.

## Building the installer

app-output-folder = Cartella di output
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Utente\WinAppMigrator
app-output-folder-not-full = Immetti il percorso completo di una cartella, ad esempio D:\Installer.
app-build-create = Crea programma di installazione
app-build-create-again = Crea di nuovo il programma di installazione
app-build-cancel = Annulla
app-build-cancelling = Annullamento al termine del passaggio corrente…
app-build-cancelled = Creazione annullata. Non è stato scritto alcun programma di installazione.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Lo stub del programma di installazione (winappmigrator-installer.exe) non è stato trovato accanto a questa app, quindi non è possibile creare un programma di installazione autoestraente.
app-build-select-an-app-first = Seleziona prima un'app da migrare.
app-build-look-again-first = Questa app deve essere analizzata di nuovo prima di poter creare il programma di installazione.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = La creazione si è interrotta in modo imprevisto: { $reason }. Se nella cartella di output è rimasto un programma di installazione, non usarlo.
app-build-worker-disconnected = Il processo di migrazione si è disconnesso.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Impossibile creare la cartella di output { $path }: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Impossibile creare una cartella di gestione temporanea protetta, quindi non è stato creato nulla: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Impossibile creare il programma di installazione. { $error }
app-build-no-installer = La creazione è terminata senza produrre alcun programma di installazione.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Il programma di installazione è stato scritto, ma non è stato possibile eseguire il controllo hash: { $error }. Non usarlo.

## How far a build has got

app-progress-preparing = Analisi di ciò che va incluso…
app-progress-checking-room = Verifica dello spazio disponibile…
app-progress-bundling = Inclusione di runtime e driver…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Lettura e compressione dei file: { $done } di { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Controllo di ciò che indica il programma di installazione: { $done } di { $total }…
app-progress-writing = Scrittura del programma di installazione…
app-progress-verifying = Verifica degli hash…

## What a build made

app-result-failed-hash-check = Non usare questo programma di installazione: non ha superato il controllo hash.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Programma di installazione creato, ma { $count } file non era più presente dopo l'analisi dell'app, quindi non è incluso. Vedi Avvisi qui sotto.
       *[other] Programma di installazione creato, ma { $count } file non erano più presenti dopo l'analisi dell'app, quindi non sono inclusi. Vedi Avvisi qui sotto.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Programma di installazione creato, ma non è stato possibile acquisire { $count } file, in genere perché bloccato o illeggibile. L'app potrebbe non funzionare senza di esso. Vedi Avvisi qui sotto.
       *[other] Programma di installazione creato, ma non è stato possibile acquisire { $count } file, in genere perché bloccati o illeggibili. L'app potrebbe non funzionare senza di essi. Vedi Avvisi qui sotto.
    }
app-result-mentions-who = Programma di installazione creato, ma contiene ancora riferimenti alla sua provenienza. Leggi cosa indica qui sotto prima di darlo a qualcuno.
app-result-ready = Programma di installazione pronto. Il pacchetto ha superato il controllo hash.
app-result-files = File
app-result-installer-size = Dimensioni del programma di installazione
app-result-warnings = Avvisi
app-result-log = Log
app-result-copy-error = Copia errore
app-result-installer = Programma di installazione
app-result-data-file = File di dati
app-result-open-folder = Apri cartella
app-result-copy-path = Copia percorso
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Impossibile aprire la cartella: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Questa app è troppo grande per un unico eseguibile, quindi il programma di installazione è composto da due file, { $installer } e { $data_file }. Devono restare insieme: copiali entrambi nell'altro PC, nella stessa cartella e con questi nomi. Se firmi il programma di installazione, la firma copre anche il file di dati.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Eseguilo nell'altro PC per reinstallare { $app_name }. Il programma di installazione verifica i propri file con SHA-256. Windows mostra "Autore sconosciuto" perché i programmi di installazione che crei non sono firmati.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Errori di verifica ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Avvisi ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = e altri { $count }.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Preparazione dell'analisi dell'app…
app-scan-install-record = Lettura dei dati di installazione…
app-scan-near-matches = Ricerca di ciò che potrebbe farne parte…
app-scan-system-items = Ricerca di servizi, attività e collegamenti…
app-scan-files = Analisi dei file…
app-scan-registry = Lettura dell'integrazione con Windows…
app-scan-finishing = Completamento…
app-scan-look-again = Analizza di nuovo
app-scan-stop = Interrompi
app-scan-running-hint = Durante l'analisi puoi scegliere un'altra app o cambiare cosa includere.
# { $error } is why, in words that follow a colon.
app-scan-failed = Impossibile analizzare questa app: { $error }
app-scan-try-again = Riprova
app-scan-stopped = Analisi interrotta. Potrai creare un programma di installazione dopo che questa app sarà stata analizzata.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = L'analisi si è interrotta in modo imprevisto: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = File
# Over how big the installer can be: up to that size.
app-room-up-to = Fino a
app-room-free = Libero
# { $drive } is a drive, such as C:.
app-room-free-on = Libero su { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Sconosciuto
app-room-most-it-could-need = Lo spazio massimo che potrebbe servire, prima della compressione.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } richiede fino a { $needed } di spazio libero per questa creazione e ne ha { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Lo spazio non è sufficiente. { $drives } Scegli una cartella di output in un'altra unità oppure escludi qualcosa.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Contenuto del programma di installazione
app-footprint-working-out = Analisi di ciò che dipende da questa scelta…
# { $error } is why.
app-footprint-would-not-make-a-package = Ciò che resta non basterebbe a creare un pacchetto: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Questo esclude anche { $list }, perché ciò che eseguono o indicano non sarebbe presente. Non verranno configurati nell'altro PC.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Questo esclude anche { $list } e altri { $count } elementi, perché ciò che eseguono o indicano non sarebbe presente. Non verranno configurati nell'altro PC.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = I dati di licenza contenuti in ciò che hai escluso vengono comunque inclusi, perché dipendono dall'opzione Dati di licenza e attivazione: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Questa app è stata analizzata { $elapsed } fa. I file vengono letti durante la creazione, quindi le modifiche vengono incluse e ciò che nel frattempo è stato eliminato viene elencato. Usa Analizza di nuovo per rilevare gli elementi nuovi.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = File del programma
app-footprint-data-folders = Cartelle di dati
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } con l'opzione Dati utente
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } di { $count } cartella, { $size }
       *[other] { $included } di { $count } cartelle, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } di dati di questa app, in { $count } file, verranno inclusi solo con l'opzione Dati utente.
       *[other] { $size } di dati di questa app, in { $count } file, verranno inclusi solo con l'opzione Dati utente.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } file. { $detail }
       *[other] { $count } file. { $detail }
    }
app-footprint-services = Servizi
app-footprint-startup = Avvio automatico
app-footprint-tasks = Attività pianificate
app-footprint-shortcuts = Collegamenti
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } di { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Escluso anche: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Integrazione con Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } di { $count } tipo
       *[other] { $included } di { $count } tipi
    }
app-footprint-integration-nothing-left = Non ne rimane nulla, a causa di ciò che hai escluso
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } elemento configurato
       *[other] { $count } elementi configurati
    }
app-footprint-prerequisites = Requisiti per l'esecuzione
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } runtime
       *[other] { $count } runtime
    }
app-footprint-near-matches = Potrebbe farne parte
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } di { $count } aggiunti
app-footprint-near-matches-hint = Non sono inclusi nel programma di installazione a meno che non li selezioni. Selezionandone uno, l'app viene analizzata di nuovo con gli stessi controlli del resto, quindi ciò che sembra una password resta comunque escluso.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Incluso con l'opzione Dati utente. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = oltre { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Cartella: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Parte nascosta di un'app: { $place }
app-footprint-nothing-left-out = Non è stato escluso nulla perché simile a una password, una licenza o un certificato.
app-footprint-left-out = Elementi esclusi e motivo
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementi
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Nessun programma di installazione sarebbe autorizzato a configurare questi elementi:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Informazioni utili ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Nessun elemento trovato: { $list }.
app-none-found-services = servizi
app-none-found-startup = elementi ad avvio automatico
app-none-found-tasks = attività pianificate
app-none-found-shortcuts = collegamenti
app-none-found-integrations = integrazione con Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Copia il client nel vecchio PC
app-network-step-client-hint = WinAppMigrator Client viene eseguito nel PC in cui si trova ora l'app, con Windows 7 o versioni successive. Consente solo a questo PC di leggerne il contenuto, in forma crittografata, tramite un codice che visualizza.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Salva nell'unità USB { $drive }
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Salva nell'unità USB { $drive } ({ $name })
app-network-save-to-folder = Salva in una cartella…
app-network-look-for-usb-again = Cerca di nuovo le unità USB
app-network-no-usb = Non è collegata alcuna unità USB. Collegane una oppure salva in una cartella raggiungibile dal vecchio PC, ad esempio una cartella condivisa in rete.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Salvato come { $path }. Nel vecchio PC chiudi l'app che stai spostando, quindi fai doppio clic sul file e consentigli di apportare modifiche. Verrà visualizzato un codice.
app-network-step-pick = 2. Scegli il vecchio PC
app-network-looking = Ricerca in rete dei PC che eseguono il client…
app-network-type-name = Oppure digita il nome o l'indirizzo
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = VECCHIO-PC o 192.168.1.20
app-network-step-code = 3. Digita il codice visualizzato
app-network-code = Codice di associazione
app-network-connect = Connetti
app-network-pairing = Associazione in corso…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = l'associazione si è interrotta prima del completamento
app-network-wrong-code = non è il codice visualizzato nel vecchio PC. Controllalo e riprova.
# { $error } is what went wrong.
app-network-not-reached = impossibile raggiungere il vecchio PC: { $error }
app-network-pick-folder-title = Scegli una cartella per WinAppMigrator Client
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = impossibile salvare il client in { $path }: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = questa build di WinAppMigrator non include il client. Compilalo con scripts\build-client.ps1, quindi compila di nuovo l'app.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = impossibile salvare la nota accanto al client: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Esegui questo programma nel PC da cui stai spostando le app: Windows 7, 8,
    10 o 11.
    
    1. Chiudi l'app che stai spostando.
    2. Fai doppio clic su "WinAppMigrator Client.exe" e consenti le modifiche.
    3. Viene visualizzato un codice di associazione di 6 cifre. In WinAppMigrator
       nel nuovo PC scegli PC in rete..., seleziona questo PC e digita il codice.
    4. Quando il nuovo PC ha terminato, premi INVIO nella finestra del client
       oppure chiudila.
    
    Si limita a leggere. Durante l'esecuzione mantiene uno snapshot dell'unità di
    Windows e apre in Windows Firewall le sue due porte alle connessioni della
    rete locale; quando si arresta, rimuove lo snapshot e chiude le porte. I dati
    trasmessi in rete sono crittografati e solo un PC che conosce il codice può
    leggerli.
