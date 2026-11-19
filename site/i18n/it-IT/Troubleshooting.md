<a id="troubleshooting"></a>

# Risoluzione dei problemi

Cerca qui sotto ciò che vedi, in base a ciò che succede o alle parole sullo schermo. Le parole tra virgolette sono quelle mostrate dal programma, a volte con un nome o un numero in mezzo.

Se nulla di quanto descritto qui ti aiuta, vedi [Come ottenere assistenza](#getting-help) alla fine.

<a id="the-window-could-not-open"></a>

## Impossibile aprire la finestra

**Cosa vedi.** Una finestra di messaggio al posto della finestra:

- dall'app: "Impossibile aprire la finestra" e "Per disegnare la finestra è necessario Direct3D 12, che potrebbe non essere disponibile con un driver grafico obsoleto o in una macchina virtuale."
- da un programma di installazione: lo stesso messaggio e "È comunque possibile installare senza finestra: da un prompt dei comandi con diritti di amministratore, esegui questo file con --apply e, per scegliere la destinazione, con --target-root seguito da una cartella."
- da un programma di disinstallazione: lo stesso messaggio e "È comunque possibile disinstallare senza finestra, da un prompt dei comandi con diritti di amministratore:" con il comando esatto.

**Perché.** Le finestre vengono disegnate con [Direct3D 12](Glossary#direct3d-12) e non c'è nient'altro con cui disegnarle.

**Cosa fare.**

- Aggiorna il driver grafico del PC.
- Per un programma di installazione o di disinstallazione, usa la riga di comando. Vedi [Riga di comando](Command-Line).
- L'app non ha una riga di comando. Crea il programma di installazione in un altro PC: puoi collegare a quel PC l'unità del vecchio PC. Vedi [Spostare da un'unità collegata](Moving-from-a-Connected-Drive). Quel PC può anche leggere il vecchio PC tramite la rete. Vedi [Spostare da un PC tramite la rete](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Avvio dell'app

| Cosa vedi | Significato | Cosa fare |
| --- | --- | --- |
| La richiesta di Controllo dell'account utente indica "Autore sconosciuto" per l'app | A partire dalla versione 0.5.0, l'app è firmata. Un'app non firmata non è il file di rilascio oppure è precedente alla 0.5.0. | Scaricala di nuovo dalla [versione più recente](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) e [verificala](Getting-Started#check-the-download). |
| SmartScreen indica "PC protetto da Windows" | SmartScreen non ha visto spesso quel file | Scegli **Ulteriori informazioni**, verifica che l'autore sia Randy Northrup, quindi scegli **Esegui comunque**. |
| L'etichetta in alto indica **Caricamento non riuscito** e l'elenco indica "Impossibile elencare le app" | Non è stato possibile leggere l'elenco delle app | Fai clic sulla freccia circolare per ricaricare. Per un'unità collegata, verifica che sia ancora collegata. |
| L'app che cerchi non è nell'elenco | È un'app di Microsoft Store, una parte nascosta di un'altra app, oppure non ha alcuna voce in Impostazioni, come un'app portabile | Le app di Microsoft Store non possono essere spostate. Una parte nascosta potrebbe essere proposta in **Potrebbe farne parte** quando scegli l'app principale. Un'app senza voce in Impostazioni non può essere scelta. |
| "Nessuna app corrispondente" | La ricerca o il filtro nascondono tutto | Cancella la casella di ricerca e scegli **Tutte**. |

<a id="building"></a>

## Creazione

<a id="picking-a-drive-or-a-disk-image"></a>

### Scelta di un'unità o di un'immagine disco

| Cosa vedi | Cosa fare |
| --- | --- |
| L'unità "non sembra un'unità di sistema Windows (mancano \Windows e \Users). Scegli la radice dell'unità proveniente dall'altro PC." | Scegli la radice dell'unità, ad esempio `E:\`, non una cartella al suo interno. Verifica che sia l'unità in cui era installato Windows. |
| Un'immagine "non è un'immagine disco che Windows può montare (.vhd o .vhdx)" | Scegli un file `.vhd` o `.vhdx`. Salva prima un disco VMware o VirtualBox in uno di questi formati. Per un checkpoint di Hyper-V, `.avhdx`, esporta il disco della macchina virtuale. |
| "impossibile montare" un'immagine: "Windows l'ha già montata e il programma che l'ha montata potrebbe scriverci." | L'immagine è aperta altrove, ad esempio montata in Esplora file. Scegli invece la sua unità in **Unità collegata…** oppure smontala, quindi sceglila di nuovo. |
| Un'immagine "non contiene alcun volume leggibile con Windows. Un volume crittografato con BitLocker deve essere prima sbloccato." | Verifica che sia un'immagine dell'unità in cui era installato Windows. Se è bloccata con [BitLocker](Glossary#bitlocker), montala tu stesso in Windows, sbloccala e scegli la sua unità in **Unità collegata…**. |
| Un'immagine "è un disco differenze, che contiene solo le modifiche rispetto al disco da cui è stato creato" | Uniscila al disco da cui è stata creata, con Modifica disco di Hyper-V, e scegli quel disco. |
| "impossibile montare" un'immagine, con un altro motivo | Il motivo è quello indicato da Windows. Leggilo, verifica che nient'altro stia usando l'immagine e riprova. |

<a id="reading-a-pc-over-the-network"></a>

### Lettura di un PC tramite la rete

L'elenco completo, incluso ciò che indica il client nel vecchio PC, è in [Spostare da un PC tramite la rete](Moving-over-the-Network#if-something-goes-wrong).

| Cosa vedi | Cosa fare |
| --- | --- |
| Il vecchio PC non è elencato in **2. Scegli il vecchio PC** | Verifica che il client sia in esecuzione nel vecchio PC e mostri un codice, e che entrambi i PC siano nella stessa rete. Poi digita il nome o l'indirizzo del vecchio PC. Se tra i due c'è un router, digita l'indirizzo. Un firewall diverso da quello di Windows deve consentire il traffico TCP 47732 e UDP 47731. |
| Un nome "non è stato trovato nella rete" | Digita invece l'indirizzo del vecchio PC. |
| "impossibile raggiungere il vecchio PC:" e un motivo | Verifica che il client sia ancora in esecuzione e che il nome o l'indirizzo sia corretto. |
| "non è il codice visualizzato nel vecchio PC. Controllalo e riprova." | Digita il codice che il client mostra ora. Dopo cinque codici errati, ne genera uno nuovo. Dopo 20, non consente più alcuna associazione finché non lo riavvii. |
| Nel vecchio PC, Windows indica che l'autore del client è sconosciuto | È normale solo in Windows 7 senza il certificato radice Microsoft del 2020, che non può verificare la firma del client. In quel caso, scegli **Sì** solo per un client che hai salvato tu stesso dall'app. In tutti gli altri casi, la richiesta deve indicare Randy Northrup. In caso contrario, non eseguire quella copia. |
| L'app non riesce più a leggere il vecchio PC, dopo una lunga pausa | Il client chiude una connessione a cui l'app non chiede nulla da 30 minuti. Fai clic su **Disconnetti** ed esegui di nuovo l'associazione. |

<a id="looking-at-the-app"></a>

### Analisi dell'app

| Cosa vedi | Significato | Cosa fare |
| --- | --- | --- |
| "Impossibile analizzare questa app:" e un motivo | L'analisi non è riuscita. | Fai clic su **Riprova**. Se l'errore si ripete, la voce potrebbe non essere un'app che WinAppMigrator può spostare. |
| "non c'è più, anche se i dati di installazione di questa app lo indicano come il programma dell'app" | La voce dell'app nell'elenco delle app indica un programma che è stato spostato o rimosso | Ripara o reinstalla l'app in questo PC, quindi sceglila di nuovo. |
| "Non è stato trovato alcun programma proprio di" un'app, "quindi non c'è nulla da includere in un programma di installazione" | La voce non ha un programma proprio, come spesso accade con un pacchetto driver, ad esempio un driver grafico, con un runtime o con un componente aggiuntivo | Nel nuovo PC, installa il driver dal produttore o da Windows Update, e il runtime o il componente aggiuntivo insieme all'app che lo usa. |
| "è un'app web che un browser ha installato da un sito web" | Un sito web installato come app, ad esempio Chrome Remote Desktop, senza un programma proprio | Nel nuovo PC, apri il sito web nello stesso browser e installa l'app da lì. |
| "Analisi interrotta. Potrai creare un programma di installazione dopo che questa app sarà stata analizzata." | Hai fatto clic su **Interrompi** | Fai clic su **Analizza di nuovo**. |
| L'analisi richiede molto tempo | Un'app di grandi dimensioni, un'unità lenta o che si sta guastando, oppure molti account | Se l'unità potrebbe guastarsi, fai clic su **Interrompi**, crea un'[immagine disco](Glossary#disk-image) dell'unità e scegli quella, perché leggere un'unità che si sta guastando può danneggiarla definitivamente. A partire dalla versione 0.6.0, sceglila con **Immagine disco…**. Con la versione 0.5.0, montala e scegli la sua unità. Altrimenti, attendi. Nel frattempo puoi scegliere un'altra app. |
| Una riga che vuoi è disattivata e selezionata | Contiene il programma principale o ciò che serve per avviarlo | Viene sempre inclusa. Vedi [le righe che non puoi deselezionare](Building-an-Installer#the-groups). |
| Una riga indica "Escluso anche:" | Un elemento che hai deselezionato l'ha escluso insieme a sé | Seleziona di nuovo quell'elemento, se vuoi includere questa riga. |
| "Ciò che resta non basterebbe a creare un pacchetto" | Ciò che hai deselezionato non lascia nulla da cui creare un pacchetto | Seleziona di nuovo qualcosa. |
| "I dati di licenza contenuti in ciò che hai escluso vengono comunque inclusi" | Le licenze dipendono solo dall'opzione **Dati di licenza e attivazione** | Deseleziona quell'opzione se vuoi escluderle. |

<a id="create-installer-stays-greyed-out"></a>

### Il pulsante Crea programma di installazione resta disattivato

Il pulsante resta in attesa finché tutte queste condizioni non sono vere:

- l'analisi dell'app è terminata, per le opzioni così come sono ora ("Analisi di ciò che dipende da questa scelta…" indica che sta ancora calcolando cosa esclude la tua ultima deselezione);
- la **Cartella di output** è un percorso completo, ad esempio `D:\Installers` ("Immetti il percorso completo di una cartella, ad esempio D:\Installer.");
- c'è spazio in quell'unità.

<a id="room"></a>

### Spazio

| Cosa vedi | Cosa fare |
| --- | --- |
| "Lo spazio non è sufficiente." e quanto spazio serve e quanto ne ha un'unità | Scegli una cartella di output in un'altra unità oppure escludi qualcosa. La cifra è lo spazio massimo che una creazione potrebbe occupare, prima della compressione, quindi può essere superiore alle dimensioni del programma di installazione. |
| Un'unità "non indica lo spazio disponibile" | La creazione procede senza controllare quell'unità. Assicurati che abbia spazio. |
| Un'unità FAT32 e un'app di grandi dimensioni | FAT32 non può contenere un file di 4 GiB. Usa un'unità NTFS o exFAT. |

<a id="the-done-screen"></a>

### La schermata finale

| Cosa vedi | Cosa fare |
| --- | --- |
| "Programma di installazione creato, ma" alcuni file "non erano più presenti dopo l'analisi dell'app" | Fai clic su **Analizza di nuovo**, quindi su **Crea di nuovo il programma di installazione**, se quei file sono importanti. |
| "Programma di installazione creato, ma non è stato possibile acquisire" alcuni file, "in genere perché bloccati o illeggibili." | Chiudi l'app, e qualsiasi altro programma che usa i suoi file, quindi crea di nuovo il programma di installazione. **Avvisi** elenca i file. |
| "Programma di installazione creato, ma contiene ancora riferimenti alla sua provenienza." | Leggi i messaggi color ambra sotto. Vedi [l'ultimo controllo](Building-an-Installer#the-last-look). |
| **Avvisi** indica "Impossibile scaricare" un runtime e "il programma di installazione proverà a usare winget nel computer di destinazione." | Quel runtime non è incluso nel programma di installazione, quindi il nuovo PC avrà bisogno di Internet per installarlo. Crea di nuovo il programma di installazione mentre sei connesso a Internet. Vedi [Runtime](Runtimes#at-build-time). |
| "Non usare questo programma di installazione: non ha superato il controllo hash." | Non usarlo. Crealo di nuovo. Se succede ancora, l'unità potrebbe guastarsi. |
| "Il programma di installazione è stato scritto, ma non è stato possibile eseguire il controllo hash" | Non usarlo. Crealo di nuovo. |
| "Creazione annullata. Non è stato scritto alcun programma di installazione." | Hai fatto clic su **Annulla**. |
| "Impossibile creare il programma di installazione." e un motivo | Leggi il motivo. **Copia errore** lo copia e **Log** indica dove si trova il log. |
| "Impossibile creare la cartella di output" | Scegli una cartella in cui puoi scrivere. |
| "Impossibile creare una cartella di gestione temporanea protetta, quindi non è stato creato nulla" | La creazione non è riuscita a creare la propria cartella di lavoro privata accanto alla cartella di output o nella cartella temporanea di Windows. Scegli una cartella di output in un'altra unità. |
| "La creazione si è interrotta in modo imprevisto" | Si è verificato un problema che nulla aveva previsto. "Se nella cartella di output è rimasto un programma di installazione, non usarlo." [Segnalalo](#getting-help). |

<a id="installing"></a>

## Installazione

<a id="when-the-installer-opens"></a>

### All'apertura del programma di installazione

| Cosa vedi | Significato | Cosa fare |
| --- | --- | --- |
| "Controllo del programma di installazione…" per molto tempo | Legge l'intero pacchetto per controllarne l'hash | Attendi. Un programma di installazione di grandi dimensioni richiede un po' di tempo. |
| "Impossibile leggere il programma di installazione:" e "questo programma di installazione è danneggiato o è stato modificato dopo la creazione, quindi non è stato letto nulla dal pacchetto. Copialo di nuovo dal percorso in cui è stato creato." | Il file è troncato o modificato | Copialo di nuovo. |
| "questo programma di installazione conserva il pacchetto in un file denominato" un file `.wamdata`, "nella stessa cartella, e quel file non è presente" | A un programma di installazione composto da due file manca il file di dati | Copia il file `.wamdata` nella stessa cartella. Vedi [App di grandi dimensioni](Big-Apps). |
| il file di dati "non è il file di dati di questo programma di installazione, oppure è danneggiato o è stato modificato dopo la creazione" | Il file di dati non corrisponde | Copia di nuovo entrambi i file dal percorso in cui sono stati creati. |
| "Impossibile usare il pacchetto di questo programma di installazione in questo PC:" e "questa app non può essere eseguita in questo PC, quindi non è stato modificato nulla" | I programmi dell'app sono compilati per un altro processore, oppure un programma indica di richiedere una versione di Windows più recente | Installala in un PC simile a quello vecchio. Non è stato modificato nulla. |
| "Questo è uno stub del programma di installazione di WinAppMigrator e non contiene alcun pacchetto di app." | Questo è lo stub vuoto, non un programma di installazione creato con l'app | Crea un programma di installazione con l'app ed esegui quello. |
| "La firma di questo programma di installazione non è valida", con il motivo indicato da Windows | Il programma è stato firmato, ma Windows non accetta la firma. Potrebbe essere stato modificato in seguito, oppure il suo certificato non è considerato attendibile in questo PC. | Non installarlo. Rivolgiti a chi l'ha creato. |
| La richiesta di Controllo dell'account utente indica "Autore sconosciuto" | Il programma di installazione non è firmato. È normale per uno che hai creato tu. | Eseguilo solo se ti fidi della sua provenienza. |

<a id="the-install-location"></a>

### Il percorso di installazione

Quando **Installa** e **Anteprima modifiche** sono disattivati, un avviso in rosso sotto **Percorso di installazione** ne indica il motivo. Il motivo più comune è che l'app è già installata in quella cartella in questo PC, come accade spesso per un browser Web, un lettore multimediale o Office: disinstalla prima quella copia, da Impostazioni di Windows, in **App**, e apri di nuovo il programma di installazione, oppure digita una cartella nuova e vuota. I motivi, e cosa fare, sono descritti in [Installazione nel nuovo PC](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Prima o durante l'installazione

| Cosa vedi | Significato | Cosa fare |
| --- | --- | --- |
| "Lo spazio qui potrebbe non essere sufficiente" | Lo spazio nell'unità sembra insufficiente | Libera spazio o scegli un percorso in un'altra unità. |
| "lo spazio non è sufficiente, quindi non è stato modificato nulla" | Un'unità in cui scrive è troppo piena | Libera spazio in quell'unità o scegli un percorso di installazione in un'altra unità. |
| "Ciò che verrebbe eseguito qui è cambiato, quindi non è stato installato nulla." | Qualcosa nel PC è cambiato da quando è stato calcolato l'elenco | Rileggi l'elenco e fai di nuovo clic su **Installa** se è quello che vuoi. |
| "Un'installazione precedente di" un'app "non è mai stata completata. Facendo clic su Installa, verrà prima annullata." | Un'installazione precedente è stata interrotta | Fai clic su **Installa**. Prima annulla quell'installazione. |
| "è in corso un'altra installazione o disinstallazione. Attendi che termini, quindi riprova" | Può esserne eseguita solo una alla volta | Attendi, quindi riprova. |
| Un file "ha un nome che questa versione di Windows apre come dispositivo e non come file" | L'app contiene un file con un nome come `aux.h`, che le versioni di Windows precedenti a Windows 11 non possono contenere | Installala in Windows 11. |
| "Impossibile creare una cartella di gestione temporanea" | Non c'è alcuna posizione privata, con spazio sufficiente, in cui decomprimere il pacchetto | Libera spazio nell'unità del percorso di installazione o nell'unità di sistema. |
| Viene indicato che una cartella di dati come `%AppData%` è un collegamento simbolico | Il programma di installazione non scrive attraverso un [collegamento simbolico](Glossary#link) | Deseleziona **Dati utente** e **Licenza** e installa senza questi dati. |
| "Impossibile trovare la cartella ProgramData di questo PC, quindi non è stato installato nulla" | Non è stato possibile leggere il dato di Windows che indica dove si trova ProgramData | Il programma di installazione non installa nulla senza un posto sicuro in cui conservare il proprio registro. |

<a id="the-result"></a>

### Il risultato

| Cosa vedi | Cosa fare |
| --- | --- |
| "L'installazione non è riuscita e le modifiche apportate sono state ripristinate." | Leggi i messaggi in rosso per capire il motivo. Il PC è com'era prima, tranne gli eventuali runtime installati. |
| "L'installazione non è riuscita e non è stato possibile ripristinare tutte le modifiche apportate." | Leggi l'elenco. Chiudi ciò che potrebbe usare quei file ed esegui di nuovo un programma di installazione: prima riprova a ripristinare. |
| "Impossibile installare" un runtime | Non è stato possibile installare il runtime, quindi l'installazione è stata annullata. Per DirectX End-User Runtime, il nuovo PC ha bisogno di Internet. Vedi [Runtime](Runtimes#with-no-internet-on-the-new-pc). |
| "Una dipendenza richiede un riavvio per essere completata. Riavvia Windows, quindi avvia l'app." | Riavvia, quindi avvia l'app. |
| Il servizio "veniva eseguito come" un account "nel PC di origine. La password non è inclusa nel programma di installazione, quindi il servizio viene configurato come LocalService." | Apri Servizi e imposta l'account e la password del servizio, se non funziona così com'è. |
| Alcuni file "vanno invece" nella cartella di installazione, "in una cartella denominata _external" | Questo PC conserva Documenti o Giochi salvati fuori dal tuo profilo. Sposta tu stesso quei file in quella cartella. |
| Qualcosa è "escluso", con un motivo | Le regole l'hanno rifiutato oppure appartiene già a un'altra app in questo PC. Il resto viene installato. Se l'app ne ha bisogno, leggi il motivo. |
| **Hash del pacchetto NON VALIDI** o **File installati NON VALIDI** | Qualcosa non corrisponde. Copia di nuovo il programma di installazione e riprova. |

<a id="after-installing"></a>

### Dopo l'installazione

| Cosa vedi | Cosa fare |
| --- | --- |
| L'app non si avvia | Controlla in **Dipendenze** se c'è un runtime contrassegnato con **Da installare manualmente** e installalo. Controlla gli avvisi nel risultato. |
| Un'app proveniente da Windows 7 non si avvia e, durante la creazione, un avviso indicava che "Windows 10 e 11 non lo includono" a proposito di una DLL | Quella DLL era stata messa nelle cartelle di sistema del vecchio Windows da un altro programma, quindi non è stata inclusa. Installa ciò che la fornisce, quindi avvia di nuovo l'app. Vedi [App da Windows 7, Vista, 8 o 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| L'app si avvia come se fosse nuova | È normale con un programma di installazione creato con **Dati utente** disattivato. Succede anche quando **Dati utente** è stato deselezionato durante l'installazione, perché i dati vanno nella cartella di installazione, e quando il programma di installazione non è riuscito a determinare chi aveva eseguito l'accesso, cosa che segnala. |
| L'app chiede di essere attivata | Una licenza legata al vecchio PC deve essere attivata di nuovo. |
| Il programma di aggiornamento dell'app ritiene che l'app non sia installata | Windows Installer non ha dati di installazione dell'app in questo PC. Vedi [Limitazioni](Limitations). |
| Un tipo di file o un tipo di collegamento non si apre con l'app | Un'altra app lo possedeva già in questo PC, quindi è stato lasciato invariato. Scegli l'app in Impostazioni, in App predefinite. |
| Una nuova voce di `Path` non viene rilevata | Apri una nuova finestra di PowerShell o del prompt dei comandi. Alcuni programmi rilevano la modifica solo dopo che esci e accedi di nuovo. |

<a id="uninstalling"></a>

## Disinstallazione

| Cosa vedi | Cosa fare |
| --- | --- |
| "Una parte si trova in un'unità non collegata." | Collega quell'unità e disinstalla di nuovo da Impostazioni. |
| "Non è stato possibile annullare tutto. Chiudi i programmi che usano ciò che resta e disinstalla di nuovo da Impostazioni." | Chiudi l'app e tutto ciò che usa i suoi file. Se un servizio non si arresta, riavvia il PC. Poi disinstalla di nuovo. |
| "questo non è l'ID di un'installazione" (con `--quiet`, in inglese: "this is not an install's id") | L'ID passato a `--uninstall` è errato. Copialo dalla voce in Impostazioni. Vedi [Riga di comando](Command-Line#finding-the-id). |
| Compare un file denominato `.before-install` | È l'originale di un file che l'installazione ha sostituito e che nel frattempo avevi modificato. Tieni quello che preferisci. |
| Un runtime è ancora installato | I runtime restano installati, perché altri programmi potrebbero usarli. Lascialo, a meno che tu non sia sicuro che nient'altro lo usi. |

<a id="scripts"></a>

## Script

| Cosa vedi | Cosa fare |
| --- | --- |
| "The requested operation requires elevation", o lo stesso messaggio nella lingua di Windows | Esegui PowerShell come amministratore. |
| Non viene stampato nulla e `$LASTEXITCODE` non è impostato | Reindirizza l'output del programma di installazione, ad esempio a `Out-String`. Vedi [Riga di comando](Command-Line). |
| Codice di uscita `2` | Un flag è errato, oppure `--apply` e `--dry-run` sono stati usati insieme. |
| "this package does not say where its app was, so give a --target-root" | Aggiungi `--target-root` con un percorso completo. |
| L'app "was in" una cartella, "and that cannot be used" | Aggiungi `--target-root` con un'altra cartella. |
| Una cartella "cannot be used: it already has files" "Give --force only if you know it is this app's" | Scegli una cartella vuota. Usa `--force` solo per una cartella che sai appartenere a questa app. |

<a id="language-and-text"></a>

## Lingua e testo

- **La lingua è sbagliata.** Scegline un'altra in **Lingua**, in alto a destra nell'app o nella prima schermata di un programma di installazione. L'app ricorda la scelta; un programma di installazione la mantiene per quell'esecuzione. Se non è stata fatta alcuna scelta, ciascuno usa la prima delle lingue di visualizzazione di Windows che conosce, altrimenti l'inglese.
- **Una traduzione non è corretta.** Tutte le lingue tranne l'inglese sono state tradotte automaticamente. In **Lingua**, scegli **Segnala un problema di traduzione**, che apre un modulo con la tua lingua già compilata. Indica cosa mostra lo schermo e cosa dovrebbe dire.
- **Le lettere vengono visualizzate come caselle vuote.** La finestra usa i tipi di carattere inclusi in Windows per ogni sistema di scrittura, da `C:\Windows\Fonts`. Se in Windows alcuni di questi sono stati rimossi, quelle lettere vengono disegnate come caselle; anche i nomi di altre app nell'elenco possono averne bisogno.
- **Le parole sono troncate.** Un'etichetta che non entra nello spazio termina con `…`; passa il puntatore del mouse sopra l'etichetta o spostati su di essa con il tasto TAB per vederla per intero. Segnalalo indicando la lingua e la schermata, in modo che la finestra possa essere adattata.
- **WinAppMigrator Client stampa in inglese in un Windows con scrittura da destra a sinistra.** La console in cui viene eseguito non è in grado di unire o ordinare le lettere arabe o ebraiche, quindi stampa in inglese e ne spiega il motivo. `--lang` sceglie qualsiasi lingua disponibile.

<a id="getting-help"></a>

## Come ottenere assistenza

1. Trova il [log](Glossary#log). Il suo percorso viene mostrato come **Log** in caso di errore oppure stampato come `Log:` dalla riga di comando. I log si trovano in `%ProgramData%\WinAppMigrator\logs`. Contengono percorsi e nomi di chiavi, ma mai il contenuto di un'impostazione, di un file di licenza o di una variabile di ambiente. Possono comunque contenere i nomi delle tue cartelle, quindi leggine uno prima di condividerlo.
2. [Apri una segnalazione](https://github.com/RandyNorthrup/WinAppMigrator/issues) indicando la versione, cosa hai fatto e cosa indicava lo schermo. **Copia errore** nella schermata finale dell'app copia il messaggio.
3. **Non allegare mai un programma di installazione a una segnalazione.** Può contenere dati privati.
4. Per un problema di sicurezza, segnalalo invece in privato, tramite gli [avvisi di sicurezza di GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
