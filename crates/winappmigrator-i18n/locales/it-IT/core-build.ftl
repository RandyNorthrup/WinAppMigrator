# Words the core writes for people while it builds a package from an installed app: what a package records for an
# installer to show later, errors shown in the app, the list of what goes in and what is left out, opening a disk
# image or another PC over the network, and what an installer says when the package it carries cannot be read.
# Message names start with core-.


## Scanning an app, before a package is built. Shown in the app when a scan cannot go on.

# { $path } is the app's program, such as D:\Program Files\Acme\Acme.exe, on a drive taken from another PC.
core-scan-program-not-on-the-drive = { $path } passa attraverso un collegamento simbolico o non si trova nell'unità collegata, quindi non è stato letto.
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-not-on-this-pc = { $path } non si trova in un disco di questo PC o ha un nome che aprirebbe qualcos'altro, quindi non è stato letto.
# { $path } is the app's program.
core-scan-program-someone-elses = { $path } si trova nelle cartelle di un altro account o non appartiene a questo account, e i dati di installazione di questa app non indicano che debba trovarsi lì, quindi non è stato letto.
# { $app_name } is the app's name, as the list of apps calls it.
core-scan-nothing-to-package = Non è stato trovato alcun programma proprio di { $app_name }, quindi non c'è nulla da includere in un programma di installazione. Succede spesso con driver, runtime e componenti aggiuntivi: nel nuovo PC, installa il driver dal produttore o da Windows Update, e il runtime o il componente aggiuntivo insieme all'app che lo usa.
# { $app_name } is the app's name, as the list of apps calls it. A web app is a website installed as an app, with
# a window and a Start menu entry of its own.
core-scan-web-app = { $app_name } è un'app web che un browser ha installato da un sito web. Non ha un programma proprio, quindi non c'è nulla da includere in un programma di installazione. Nel nuovo PC, apri il sito web in quel browser e installa l'app da lì.
# { $path } is the program the app's own record names, such as C:\Program Files\Acme\Acme.exe.
core-scan-program-missing = { $path } non c'è più, anche se i dati di installazione di questa app lo indicano come il programma dell'app. L'app potrebbe essere stata spostata o rimossa in parte: ripararla o reinstallarla in questo PC dovrebbe ripristinarlo.
# { $path } is the file that was picked.
core-scan-not-a-file = la destinazione dell'analisi non è un file: { $path }
# Names something left out of a package, in a list of what was left out and why. { $name } is the variable's name,
# such as ACME_LICENSE_KEY.
core-scan-environment-variable-item = La variabile di ambiente { $name }

## Folders and parts that might also be the app's, offered in a list to tick. Each says why it is offered.

core-near-starts-with-app-name = Il nome inizia con il nome di questa app.
core-near-has-app-name = Il nome contiene il nome di questa app.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-starts-with-publisher = Il nome inizia con il nome dell'autore, { $publisher }.
# { $folder } is a folder's name, such as Program Files.
core-near-hidden-part-beside = Una parte nascosta dall'elenco delle app, installata accanto a questa app in { $folder }.
# { $publisher } is the publisher's name, such as Fabrikam Inc.
core-near-hidden-part-publisher = Una parte nascosta dall'elenco delle app, dello stesso autore, { $publisher }.

## What a scan found, as a person chooses from it: the folders that go in, and what is left out, and why

# Why a folder cannot be left out, beside its number of files.
core-footprint-locked-app-folder = Questi file vanno sempre insieme al programma principale
core-footprint-locked-main-program = Contiene il programma principale, che viene sempre incluso
core-footprint-locked-needed = Contiene ciò senza cui il programma principale non può avviarsi
core-footprint-locked-font = Va con il relativo tipo di carattere, in Integrazione con Windows
# Where a folder is. "AppData" and "ProgramData" are the names of Windows folders; keep them as they are.
core-footprint-place-roaming = Impostazioni nel tuo profilo (AppData)
core-footprint-place-local = Impostazioni per questo PC nel tuo profilo (AppData)
core-footprint-place-program-data = Condivisa da tutti gli utenti del PC (ProgramData)
core-footprint-place-documents = Nella cartella Documenti
core-footprint-place-saved-games = Nella cartella Giochi salvati
core-footprint-place-public-documents = Nella cartella Documenti pubblici
core-footprint-place-app-folder = Nella cartella dell'app
# { $folder } is a folder, such as C:\Users\Sam\AppData\Roaming.
core-footprint-place-in = In { $folder }
# A row for the files directly in the app's folder. { $folder } is that folder's name, such as Acme.
core-footprint-files-directly-in = File direttamente in { $folder }
# Headings over what is left out, and why each item is.
core-footprint-licenses-left-out = Non inclusi in questo programma di installazione, perché l'opzione Dati di licenza e attivazione è disattivata
core-footprint-font-stays-behind = Non viene trasferito.
core-footprint-drivers-left-out = Driver che appartengono a Windows o a un produttore e che non possono essere ridistribuiti, quindi reinstallali nell'altro PC
core-footprint-driver-reinstall = Reinstallalo dal pacchetto del produttore.
# Why something else is left out along with what the person left out. { $service } is a service's name, such as AcmeSync.
core-footprint-needs-left-out = richiede { $service }, che è escluso
core-footprint-told-to-use-left-out = è configurato per usare qualcosa in una cartella che hai escluso
core-footprint-file-in-left-out = il suo file si trova in una cartella che hai escluso
core-footprint-program-in-left-out = il programma a cui si riferisce si trova in una cartella che hai escluso
core-footprint-names-left-out = fa riferimento a una cartella che hai escluso
# Why something is left out, as the installer tells whoever installs it.
core-footprint-builder-left-it-out = chi ha creato questo programma di installazione lo ha escluso
# Something the person left out, by kind and name, such as "Service Acme Updater". { $kind } is the kind, such as
# Service or Scheduled Task; { $title } is its name.
core-footprint-left-out-item = { $kind } { $title }

## Building a package and an installer from a scan. Shown in the app when a build cannot go on.

core-package-scan-of-another-app = questa analisi riguarda un'app diversa da quella in fase di creazione, quindi non è stato creato nulla
core-package-scan-brought-in-more = questa analisi ha incluso dati non richiesti per questa creazione, quindi non è stato creato nulla
# { $path } is the app's program, such as C:\Program Files\Acme\Acme.exe.
core-package-program-gone = { $path } non è più presente, quindi non è stato creato nulla. Analizza di nuovo l'app.
core-package-needs-stub = un archivio autoestraente richiede uno stub del programma di installazione
# { $reason } is what went wrong.
core-package-staged-file-failed = impossibile includere nel pacchetto un file della cartella di gestione temporanea: { $reason }
# { $shortfalls } says which drives are short of room and by how much, in sentences of its own.
core-package-not-enough-room = lo spazio non è sufficiente per creare questo programma di installazione, quindi non è stato letto né scritto nulla. { $shortfalls }. Libera spazio in quell'unità o scegli una cartella di output in un'altra unità.
# { $path } is a file's path, on a drive taken from another PC.
core-package-not-read-off-the-drive = { $path } passa attraverso un collegamento simbolico o non si trova nell'unità collegata, quindi non è stato letto
# { $path } is a file's path.
core-package-not-read-not-local = { $path } non si trova in un disco di questo PC, quindi non è stato letto
# { $path } is a file's path.
core-package-not-read-other-account = { $path } porta nelle cartelle di un altro account, quindi non è stato letto
# { $path } is a file's path, as the PC it came from called it.
core-package-not-on-the-drive = { $path } non si trova nell'unità collegata, quindi non è stato letto
# { $path } is a file's path.
core-package-backup-copy-no-parent = impossibile eseguire la copia di backup di un file senza cartella padre: { $path }
# { $path } is a file's path.
core-package-backup-copy-destination-no-parent = la destinazione della copia di backup non ha una cartella padre: { $path }
# robocopy is the name of a Windows command; keep it as it is. { $code } is how it ended, such as Some(8).
core-package-robocopy-failed = copia robocopy in modalità backup non riuscita (codice di uscita { $code })
# Stands for a folder where a file's name would be, as in "I/O failed for temporary package folder: …".
core-package-temporary-folder = cartella temporanea del pacchetto
# Stands for the installer being written where a file's name would be, as in "I/O failed for the installer: …".
core-package-the-installer = il programma di installazione
# { $path } is a file's path, or its name in the package. { $error } is what went wrong with it.
core-package-path-error = { $path }: { $error }
# { $name } is a file's name in the package, such as app/bin/App.exe.
core-package-same-place = un altro file occupa la stessa posizione nel pacchetto: { $name }
# { $name } is a file's name in the package.
core-package-changed-in-protected-folder = { $name } è cambiato nella cartella protetta durante la creazione del pacchetto
core-package-worker-died = un thread di compressione dei file si è interrotto in modo imprevisto, quindi non è stato scritto alcun programma di installazione
# { $path } is a file's path.
core-package-link-not-read = { $path } è un collegamento simbolico, quindi non è stato letto
# { $path } is a file's path. { $error } is what went wrong.
core-package-not-compressed = impossibile comprimere { $path }: { $error }
# { $path } is a file's path. { $reason } is what went wrong.
core-package-not-read-back = impossibile rileggere { $path }: { $reason }

## Where an app's files are, and where an installer may write them. Shown when a path is refused.

# { $path } is a folder, such as C:\Program Files\Acme.
core-discovery-root-not-full-path = la cartella di destinazione deve essere un percorso completo: { $path }
# { $path } is a folder's path.
core-discovery-root-not-a-folder = la cartella di destinazione non è una cartella: { $path }
# { $path } is a file's path.
core-discovery-escapes-root = il percorso esce dalla radice prevista: { $path }
# { $path } is the link's path.
core-discovery-refusing-to-write-through-link = scrittura attraverso un collegamento simbolico rifiutata: { $path }
# { $path } is the link's path.
core-discovery-refusing-link = passaggio attraverso un collegamento simbolico rifiutato: { $path }
# { $error } is what Windows said went wrong.
core-discovery-refusing-link-because = passaggio attraverso un collegamento simbolico rifiutato: { $error }
# { $path } is the link's path.
core-discovery-link-to-somewhere-else = { $path } è un collegamento simbolico che punta altrove e altri potrebbero modificarne la destinazione. Usa il percorso a cui punta
# { $path } is a folder's path.
core-discovery-no-parent-folder = { $path } non ha una cartella padre
# { $path } is the install location as it was given. Keep the quotes, or use your language's own.
core-discovery-install-location-not-full-path = il percorso di installazione deve essere un percorso completo: "{ $path }"
# { $path } is the install location.
core-discovery-install-location-a-link = il percorso di installazione è un collegamento simbolico che punta altrove: { $path }
# { $path } is a file's path.
core-discovery-device-name = il percorso usa un nome riservato a un dispositivo Windows: { $path }
# { $path } is a file's path.
core-discovery-device-name-on-older-windows = { $path } ha un nome che questa versione di Windows apre come dispositivo e non come file, quindi non vi è stato scritto nulla. Può essere installato in Windows 11
# { $path } is a file's path.
core-discovery-unsafe-path = percorso non sicuro: { $path }
# { $path } is a file's path.
core-discovery-no-package-path = il file non ha un percorso relativo nel pacchetto: { $path }

## Writing an installer, and reading the package inside one. Most of these are shown by an installer, on the PC it
## runs on, when it cannot read what it carries. "The package" is the app's files and settings that an installer
## carries; "the stub" is the program part of an installer, before the package; "the data file" is the .wamdata file
## that holds the package of a very large installer, beside it.

# Stands for the package where a file's name would be, as in "I/O failed for installer package: …".
core-archive-installer-package = pacchetto del programma di installazione
# Stands for the installer program built into WinAppMigrator, where a file's name would be.
core-archive-embedded-stub = stub del programma di installazione incorporato
# { $index } is the number of a file in the package. { $reason } is what went wrong.
core-archive-entry-unreadable = impossibile leggere la voce { $index } del pacchetto: { $reason }
# { $name } is a file's name inside the package, such as app/bin/App.exe.
core-archive-entry-unsafe-path = la voce zip ha un percorso non sicuro: { $name }
# { $name } is a file's name inside the package.
core-archive-holds-more = { $name } contiene più dati di quanto dichiarato
# { $name } is a file's name inside the package.
core-archive-name-too-long = questo nome è troppo lungo per un file zip: { $name }
core-archive-changed-while-copied = il pacchetto è cambiato mentre veniva aggiunto dopo lo stub, quindi non è stato scritto alcun programma di installazione
core-archive-stub-not-a-program = la parte di questo programma di installazione che precede il pacchetto non è un programma
core-archive-damaged = questo programma di installazione è danneggiato o è stato modificato dopo la creazione, quindi non è stato letto nulla dal pacchetto. Copialo di nuovo dal percorso in cui è stato creato.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-missing = questo programma di installazione conserva il pacchetto in un file denominato { $name }, nella stessa cartella, e quel file non è presente. I due file devono restare insieme: copia qui { $name } dal percorso in cui è stato creato il programma di installazione, quindi eseguilo di nuovo.
# { $name } is the data file's name, such as App_installer.wamdata. { $error } is what Windows said went wrong.
core-archive-data-file-unopened = impossibile aprire il file di dati di questo programma di installazione, { $name }: { $error }. I due file devono restare insieme, in una cartella leggibile da questo PC.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-a-link = { $name } è un collegamento simbolico o una cartella, mentre il file di dati di questo programma di installazione deve essere un file a sé stante, situato accanto al programma, quindi non ne è stato letto nulla. Copia qui il file reale.
# { $name } is the data file's name, such as App_installer.wamdata.
core-archive-data-file-wrong = { $name } non è il file di dati di questo programma di installazione, oppure è danneggiato o è stato modificato dopo la creazione, quindi non ne è stato letto nulla. Copia di nuovo entrambi i file dal percorso in cui sono stati creati.
core-archive-stub-signed = lo stub del programma di installazione è firmato; deve restare non firmato, quindi firma invece il programma di installazione finito
# { $size } is a size, such as 4.2 GB.
core-archive-too-big-to-run = il programma di installazione avrebbe dimensioni di { $size } e Windows non può eseguire un eseguibile di 4 GB o più; non è stato scritto nulla
core-archive-outside-signature = il programma di installazione contiene dati esterni alla firma e potrebbe essere stato manomesso
core-archive-footer-length = la lunghezza della sezione finale dell'archivio autoestraente non è valida
core-archive-seek-outside = posizionamento all'esterno del pacchetto
# { $path } is a file's path.
core-archive-unsafe-package-path = percorso del pacchetto non sicuro: { $path }
# { $path } is a file's path.
core-archive-empty-package-path = percorso del pacchetto vuoto: { $path }

## Opening a disk image of another PC (a .vhd or .vhdx file) to read apps from it

# { $path } is the disk image file, such as D:\Backups\OldPC.vhdx.
core-image-not-attachable = { $path } non è un'immagine disco che Windows può montare (.vhd o .vhdx)
core-image-needs-administrator = il montaggio di un'immagine disco richiede diritti di amministratore
# { $path } is the disk image file the person picked.
core-image-not-a-file = { $path } non è un file
# { $path } is the disk image file. "Edit Disk" is the name of a Hyper-V Manager command.
core-image-differencing = { $path } è un disco differenze, che contiene solo le modifiche rispetto al disco da cui è stato creato, e Windows aprirebbe anche quel disco, ovunque indichi che si trovi. Prima uniscilo a quel disco con Modifica disco di Hyper-V, quindi scegli quel disco
# { $path } is the disk image file.
core-image-unreadable = { $path } non è un file .vhd o .vhdx leggibile
# ProgramData is the name of a Windows folder; keep it as it is.
core-image-no-program-data = impossibile trovare la cartella ProgramData di questo PC
# The reason given in core-image-could-not-attach when Windows already has the image open.
core-image-already-attached = Windows l'ha già montata e il programma che l'ha montata potrebbe scriverci. Prima smontala oppure scegli la sua unità come unità collegata
# { $path } is the disk image file. { $reason } is why, such as core-image-already-attached or what Windows said.
core-image-could-not-attach = impossibile montare { $path }: { $reason }
# { $path } is the disk image file.
core-image-no-windows-volume = { $path } non contiene alcun volume leggibile con Windows. Un volume crittografato con BitLocker deve essere prima sbloccato.
# Given as the reason when Windows failed without saying anything.
core-image-windows-did-not-say = Windows non ha indicato il motivo

## Reading another PC over the network, through the client running on it. Shown as why something on that PC
## could not be read. "The client" is WinAppMigrator Client, the small program on the other PC.

core-remote-too-slow = l'altro PC ha impiegato troppo tempo a rispondere
core-remote-closed = la connessione con l'altro PC è chiusa
core-remote-failed = la connessione con l'altro PC non è riuscita
core-remote-answered-something-else = il client ha fornito una risposta diversa da quella prevista
# { $name } is the PC's name or address as the person typed it, such as OLDPC or 192.168.1.20.
core-remote-not-found = { $name } non è stato trovato nella rete
core-remote-did-not-say-what-it-is = il client non ha indicato di cosa si tratta
core-remote-ordinary-names-only = è possibile richiedere solo un percorso sotto la radice dell'altro PC, con nomi ordinari
core-remote-more-but-none = l'altro PC ha indicato che una cartella conteneva altri elementi, ma non ne ha inviato nessuno
# { $count } is the most things one folder may hold, such as 250,000.
core-remote-too-many-listed = l'altro PC ha elencato più di { $count } elementi in una cartella
core-remote-sent-too-much = l'altro PC ha inviato una parte di file più grande di quella richiesta
core-remote-name-not-text = non è possibile richiedere un nome che non sia testo
core-remote-below-root-only = è possibile richiedere solo un percorso sotto la radice dell'altro PC
core-remote-file-ended-early = il file dell'altro PC è terminato prima del previsto
core-remote-link-not-followed = un collegamento simbolico in un altro PC non viene seguito
core-remote-only-files-opened = in un altro PC vengono aperti solo file
core-remote-listing-said-nothing = l'elenco non ne indicava nulla
core-remote-change-time-unknown = la data di modifica non è nota
core-remote-seek-before-start = un posizionamento prima dell'inizio
# { $size } is how many bytes a file is; { $limit } is the most that were expected. Both are numbers.
core-remote-too-large =
    { $size ->
        [one] è di { $size } byte, mentre non ne erano previsti più di { $limit }
       *[other] è di { $size } byte, mentre non ne erano previsti più di { $limit }
    }

## Checking that a download is signed by Microsoft. Each is given as the reason a check failed.

# { $error } is what Windows said went wrong.
core-signature-no-powershell = impossibile avviare PowerShell: { $error }
# { $error } is what went wrong reading PowerShell's answer.
core-signature-unreadable = impossibile leggere la firma: { $error }
# { $signer } is who signed the file, such as Contoso Ltd.
core-signature-not-microsoft = è firmato da { $signer }, non da Microsoft
core-signature-not-microsoft-unnamed = è firmato da qualcun altro, non da Microsoft
# { $status } is Windows' own word for the signature's state, such as HashMismatch or NotSigned, which is not translated.
core-signature-status = lo stato della firma è { $status }

## Warnings a package records when it is built from an app, which an installer shows later, maybe on another PC, in
## that PC's language. "This app" is the app being packaged.

core-warning-build-program-needs-unreadable = Impossibile leggere ciò che questo programma richiede a Windows, quindi non verrà verificato se può essere eseguito nell'altro PC.
core-warning-build-external-manifest = Rilevato un manifesto dell'applicazione esterno; l'interpretazione delle dipendenze side-by-side è rinviata.
core-warning-build-non-exe = Il file selezionato non usa l'estensione .exe; l'analisi PE continuerà.
core-warning-build-stand-in-program = Non è stato trovato alcun eseguibile ordinario dell'app, quindi al suo posto viene usato questo file. I risultati relativi ad architettura e dipendenze descrivono questo file e potrebbero non descrivere l'app.
core-warning-build-other-apps-files-unknown = Non è stato possibile leggere tutte le informazioni di Windows Installer sulle altre app, quindi una cartella registrata per questa app potrebbe contenere file di un'altra app senza che nulla qui lo indichi.
core-warning-build-app-folder-has-another-apps-file = Windows Installer indica che un'altra app ha un file nella cartella del programma di questa app, e il file viene incluso, perché viene presa l'intera cartella.
# "App Paths" is the name of a Windows registry key; keep it as it is.
core-warning-build-app-paths-folder-left-out = La voce App Paths di questa app indica che l'app cerca in una cartella che non è sua, quindi quella cartella non viene inclusa. L'app potrebbe non avviarsi in un PC privo del contenuto di quella cartella.
core-warning-build-uninstaller-folder-left-out = La cartella che contiene il programma di disinstallazione di questa app è stata esclusa, perché Windows Installer indica che un'altra app ha un file in quella cartella.
# { $why } is never shown: it chooses the reason. Translate each reason as the end of the sentence.
core-warning-build-recorded-folder-left-out =
    Un elemento registrato per questa app fa riferimento a una cartella che è stata esclusa, perché { $why ->
        [unreadable] il suo percorso esce dall'unità o passa attraverso un collegamento simbolico, quindi non può essere letta in modo sicuro
        [missing] non è presente
        [link] passa attraverso un collegamento simbolico
        [short-name] non è scritta con il suo nome reale
        [another-account] si trova nelle cartelle di un altro account
        [one-account] questa app è stata installata per un solo account e la cartella non appartiene a quell'account
        [shared] contiene elementi condivisi o fa parte di Windows
        [generic-name] il suo nome non indica a chi appartiene
        [publisher] porta solo il nome dell'autore e potrebbe contenere altri suoi prodotti
        [holds-the-app] contiene la cartella di questa app e potrebbe contenere altro
        [another-apps-name] porta il nome di un'altra app
        [another-apps-record] i suoi dati di installazione appartengono a un'altra app
        [in-another-apps-folder] si trova nella cartella di un'altra app
       *[another-apps] appartiene a un'altra app
    }.
core-warning-build-app-folder-holds-another-app = Un'altra app di questo PC risulta installata all'interno della cartella di questa app e viene inclusa con essa, perché viene presa l'intera cartella.
# "Program Files (x86)" is the name of a Windows folder; keep it as Windows calls it.
core-warning-build-from-32-bit-windows = Questa app si trovava in Windows a 32 bit. Le sue chiavi del Registro di sistema sono organizzate come Windows a 64 bit organizza quelle di un'app a 32 bit, e il programma di installazione propone Program Files (x86) per l'app.
# { $dll } is a file's name, such as MSVCP140D.dll.
core-warning-build-debug-runtime = Un programma richiede { $dll }, una build di debug del runtime di Visual C++. Microsoft non fornisce un programma di installazione per queste build e potrebbe non consentirne la ridistribuzione, quindi il programma non verrà eseguito in un PC senza Visual Studio.
# { $program } is a program's file name, such as Tool.exe.
core-warning-build-single-file-program = { $program } è un programma .NET compresso in un unico file, che non indica di quale .NET ha bisogno, quindi non ne viene previsto alcuno. Se ne richiede uno, non si avvierà in un PC che ne è privo.
# { $count } is how many of the app's programs.
core-warning-build-programs-not-inspected =
    { $count ->
        [one] { $count } dei programmi dell'app non è stato esaminato per verificare i runtime necessari, perché i programmi sono troppi o troppo grandi.
       *[other] { $count } dei programmi dell'app non sono stati esaminati per verificare i runtime necessari, perché i programmi sono troppi o troppo grandi.
    }
# { $runtime } is a runtime's name, such as Java Runtime Environment.
core-warning-build-runtime-no-installer = L'app richiede { $runtime } e qui non è disponibile alcun programma di installazione, quindi non ne viene eseguito nessuno. Installalo tu stesso nell'altro PC.
# { $why } is why something was left out, such as "its name says what it holds".
core-warning-build-left-out-of-package = Escluso dal pacchetto: { $why }.
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-missing-dependency = Impossibile risolvere la dipendenza: { $name }
# { $name } is a file's name, such as zlib1.dll.
core-warning-build-dependency-only-here = { $name } è stato trovato solo tramite il percorso di ricerca di questo PC e non fa parte dell'app, quindi non viene incluso. L'app potrebbe non avviarsi in un PC privo di questo file.
core-warning-build-driver-detected = Rilevato un driver o un componente del kernel. Un driver di proprietà dell'app viene incluso e registrato di nuovo; un driver di un produttore o integrato in Windows non viene ridistribuito e deve essere reinstallato dal pacchetto firmato del produttore.
core-warning-build-near-no-longer-offered = Un elemento selezionato nell'elenco di ciò che potrebbe far parte di questa app non è più presente nell'elenco, quindi non è stato aggiunto. Controlla di nuovo l'elenco.
# { $app_name } is the name of a part of the app, as the list of apps calls it. { $why } is never shown: it chooses the
# reason. Translate each reason as the end of the sentence.
core-warning-build-near-part-left-out =
    Una parte di ciò che { $app_name } dichiara come propria è stata esclusa: { $why ->
        [unreadable] il suo percorso esce dall'unità o passa attraverso un collegamento simbolico, quindi non può essere letta in modo sicuro
        [missing] non è presente
        [link] passa attraverso un collegamento simbolico
        [short-name] non è scritta con il suo nome reale
        [another-account] si trova nelle cartelle di un altro account
        [one-account] questa app è stata installata per un solo account e questa parte non appartiene a quell'account
        [shared] contiene elementi condivisi o fa parte di Windows
        [generic-name] il suo nome non indica a chi appartiene
        [publisher] porta solo il nome dell'autore e potrebbe contenere altri suoi prodotti
        [holds-the-app] contiene la cartella di questa app e potrebbe contenere altro
        [another-apps-name] porta il nome di un'altra app
        [another-apps-record] i suoi dati di installazione appartengono a un'altra app
        [in-another-apps-folder] si trova nella cartella di un'altra app
       *[another-apps] appartiene a un'altra app
    }.
# { $app_name } is the name of a part of the app, as the list of apps calls it.
core-warning-build-near-keys-left-out = Le chiavi del Registro di sistema elencate nei dati di installazione di { $app_name } sono state escluse, perché quei dati fanno riferimento anche a elementi di un'altra app.
# "Windows keeps what an app wrote for one account" is Windows' VirtualStore folder.
core-warning-build-virtual-store-program = Un programma si trovava nella cartella in cui Windows conserva ciò che un'app ha scritto per un singolo account. Windows non vi colloca mai programmi, quindi lo ha fatto qualcos'altro, e il programma è stato escluso.
# { $path } is a file or folder, such as C:\Users\Sam\AppData\Roaming\Acme.
core-warning-build-file-left-out = { $path } è stato escluso: passa attraverso un collegamento simbolico in un'unità collegata oppure appartiene a un account diverso da quello di cui vengono inclusi i dati.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as D:\Sam\Documents.
core-warning-build-user-folder-not-on-drive = La cartella { $folder } di { $user } si trovava in { $path }, che non è in questa unità, quindi il suo contenuto non è stato incluso.
# { $user } is an account's name, such as Sam. { $folder } is the folder's name, such as Documents. { $path } is where it
# was, such as \\server\home\Sam\Documents.
core-warning-build-user-folder-not-on-this-pc = La cartella { $folder } di { $user } si trovava in { $path }, che non è in un disco di questo PC, quindi il suo contenuto non è stato incluso.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-base-unreadable = Impossibile leggere { $path } ({ $error }); i dati dell'app archiviati lì non sono stati inclusi.
# { $path } is a folder. { $error } is what Windows said went wrong.
core-warning-build-folder-unreadable = Impossibile leggere { $path } ({ $error }); potrebbero mancare dati dell'app presenti nelle sottocartelle.
# { $error } is what went wrong.
core-warning-build-entry-unreadable = Impossibile leggere una voce del file system: { $error }
# { $count } is how many links. { $names } lists some of them. { $more } is how many more there are, which are not
# named; when it is 0, the sentence ends after the list.
core-warning-build-links-not-followed =
    { $count ->
        [one] { $count } collegamento simbolico, o nome che aprirebbe qualcos'altro, non è stato seguito in questa cartella, quindi ciò a cui punta non è incluso nel programma di installazione: { $names }
       *[other] { $count } collegamenti simbolici, o nomi che aprirebbero qualcos'altro, non sono stati seguiti in questa cartella, quindi ciò a cui puntano non è incluso nel programma di installazione: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }e altri { $more }.
    }
# { $count } is how many folders. { $depth } is how many levels down a folder can be. { $names } lists some of them.
# { $more } is how many more there are, which are not named; when it is 0, the sentence ends after the list.
core-warning-build-folders-too-deep =
    { $count ->
        [one] { $count } cartella si trova a più di { $depth } livelli di profondità, quindi il suo contenuto non è incluso nel programma di installazione: { $names }
       *[other] { $count } cartelle si trovano a più di { $depth } livelli di profondità, quindi il loro contenuto non è incluso nel programma di installazione: { $names }
    }{ $more ->
        [0] .
       *[other] { " " }e altre { $more }.
    }
# { $error } is what went wrong.
core-warning-build-metadata-unreadable = Impossibile leggere i metadati del file: { $error }
core-warning-build-reserved-name = Il nome di questo file è riservato a un dispositivo Windows, quindi il file non è stato copiato.
# { $error } is what went wrong.
core-warning-build-hash-failed = Impossibile calcolare l'hash del file: { $error }
core-warning-build-cloud-placeholder = File cloud solo online (ad esempio con OneDrive File su richiesta) senza dati locali; scegli Mantieni sempre su questo dispositivo o scaricalo prima del backup. Segnalato ma non copiato.
# { $file_name } is a driver's file name, such as acmeflt.sys.
core-warning-build-driver-file = Il file di driver/kernel "{ $file_name }" non viene copiato come file normale. Se è alla base di un servizio driver registrato per questa app, il programma di installazione lo include e registra di nuovo il driver automaticamente; altrimenti reinstallalo con il pacchetto driver firmato del produttore.
core-warning-build-file-gone = Questo elemento era presente durante l'analisi dell'app ma ora non c'è più, quindi non è incluso nel programma di installazione.
# { $error } is what went wrong.
core-warning-build-file-copy-failed = Impossibile copiare il file nel pacchetto: { $error }
# { $count } is how many files. { $names } lists them.
core-warning-build-not-looked-through =
    { $count ->
        [one] Questo pacchetto contiene un file che non è stato possibile esaminare per verificarne la provenienza: { $names }. Il suo contenuto potrebbe rivelarla.
       *[other] Questo pacchetto contiene file che non è stato possibile esaminare per verificarne la provenienza: { $names }. Il loro contenuto potrebbe rivelarla.
    }
# { $count } is the most empty folders an installer makes, such as 10,000.
core-warning-build-empty-folders-truncated = Questa app ha più di { $count } cartelle vuote e ne verranno ricreate solo { $count }.
core-warning-build-credential-by-now = Escluso dal pacchetto: ora sembra una credenziale.
core-warning-build-no-package-path = Il file copiabile non aveva un percorso relativo nel pacchetto.
# { $file_name } is the runtime installer's file name, such as vc_redist.x64.exe. { $error } is what went wrong.
core-warning-build-prerequisite-path-invalid = Impossibile collocare il prerequisito incluso { $file_name }: { $error }
# { $runtime } is a runtime's name, such as Microsoft Visual C++ 2015+ Redistributable (x64). { $error } is what went
# wrong. winget is the name of a Windows command; keep it as it is.
core-warning-build-prerequisite-download-failed = Impossibile scaricare { $runtime } ({ $error }); il programma di installazione proverà a usare winget nel computer di destinazione.
# { $runtime } is a runtime's name. { $reason } is why the check failed, such as "it is signed by Contoso Ltd, not
# Microsoft".
core-warning-build-prerequisite-signature-failed = Il download di { $runtime } non ha superato il controllo della firma, quindi non è stato incluso: { $reason }. Il programma di installazione proverà a usare winget nel computer di destinazione.
# { $runtime } is a runtime's name. { $error } is what went wrong.
core-warning-build-prerequisite-hash-failed = { $runtime } è stato scaricato, ma non è stato possibile calcolarne l'hash ({ $error }); il programma di installazione proverà invece a usare winget nel computer di destinazione.
# { $service } is a driver's service name, such as AcmeFlt. { $error } is what went wrong.
core-warning-build-driver-copy-failed = Impossibile includere l'immagine del driver per "{ $service }": { $error }
core-warning-build-license-not-copied = Impossibile copiare i dati di licenza (mancanti o illeggibili); registrati solo come metadati.
core-warning-build-license-file-credential = Un file di licenza sembra una credenziale ed è stato escluso.
# { $error } is what went wrong.
core-warning-build-license-store-walk-failed = Impossibile enumerare parte di un archivio licenze: { $error }
core-warning-build-license-store-credential = Un file in un archivio licenze sembra una credenziale ed è stato escluso.
# { $error } is what went wrong.
core-warning-build-license-store-file-skipped = File dell'archivio licenze ignorato: { $error }
