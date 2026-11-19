<a id="installing-on-the-new-pc"></a>

# Installazione nel nuovo PC

Il programma di installazione è una sola finestra. Controlla se stesso, ti mostra dove installerà l'app e tutto ciò che configurerà, e installa solo quando fai clic su **Installa**. Questa pagina descrive la finestra dall'alto verso il basso, usando le parole che compaiono sullo schermo.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Il programma di installazione di Fabrikam Data Sync, che mostra che non è firmato, il percorso di installazione, lo spazio necessario e cosa configura oltre ai file" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Prima di iniziare

- **Copia l'intero programma di installazione.** Se la creazione ha prodotto anche un file `.wamdata` oltre al file `.exe`, copiali entrambi nella stessa cartella e non cambiarne i nomi. Vedi [App di grandi dimensioni](Big-Apps).
- **Esegui solo programmi di installazione attendibili.** Un programma di installazione viene eseguito con [diritti di amministratore](Glossary#administrator-rights) completi. Trattalo come qualsiasi altro programma: eseguilo solo se l'hai creato tu o se l'hai ricevuto da qualcuno di cui ti fidi. Vedi [Privacy e sicurezza](Privacy-and-Security).
- **Servono i diritti di amministratore** in questo PC e [Direct3D 12](Glossary#direct3d-12) per la finestra. Senza Direct3D 12 puoi comunque installare da un prompt dei comandi. Vedi [Riga di comando](Command-Line).

<a id="open-it"></a>

## Apertura

Fai doppio clic sul programma di installazione. Windows mostra la [richiesta di Controllo dell'account utente](Glossary#uac-prompt). Un programma di installazione non è firmato a meno che chi l'ha creato non l'abbia firmato, quindi di solito la richiesta indica **Autore sconosciuto**. È normale. Vedi [Privacy e sicurezza](Privacy-and-Security#signing).

La finestra, "Programma di installazione di WinAppMigrator", "Reinstalla un'app acquisita in un altro PC", si apre subito, indipendentemente dalle dimensioni del programma di installazione. Indica "Controllo del programma di installazione…" mentre controlla l'[hash](Glossary#hash) dell'intero pacchetto. Questo controllo legge tutto il pacchetto, quindi per un programma di installazione di grandi dimensioni richiede un po' di tempo. Da questo momento il programma di installazione tiene aperto il proprio file, e il file di dati se presente, quindi nessuno dei due può essere modificato o sostituito mentre la finestra è aperta.

Il controllo può fermarsi qui, con un messaggio in rosso. I più comuni sono:

- Il programma di installazione è danneggiato o è stato modificato dopo la creazione: copialo di nuovo dal percorso in cui è stato creato.
- Il file di dati manca o non è quello giusto: copia di nuovo entrambi i file in una stessa cartella.
- "Impossibile usare il pacchetto di questo programma di installazione in questo PC", ad esempio perché i programmi dell'app sono compilati per un altro tipo di processore o richiedono una versione di Windows più recente. Non è stato modificato nulla.

Tutti questi casi sono spiegati in [Risoluzione dei problemi](Troubleshooting#installing).

Quando il controllo viene superato, il titolo della finestra diventa "Installa" seguito dal nome dell'app.

<a id="its-language"></a>

### Lingua

Un programma di installazione usa la lingua del PC in cui viene eseguito, non quella del PC in cui è stato creato: la lingua di visualizzazione di Windows se è una delle 24 di WinAppMigrator, altrimenti l'inglese. **Lingua**, in alto a destra, la cambia per questa esecuzione. Anche ciò che elenca dell'app, incluse le note scritte dall'app durante la creazione, viene mostrato in quella lingua. Un programma di installazione creato con WinAppMigrator 0.6 o versioni precedenti mostra queste note in inglese, così come sono state scritte. Un'[installazione senza finestra dalla riga di comando](Command-Line) risponde sempre in inglese, perché gli script possano leggerla.

<a id="the-app-and-where-it-goes"></a>

## L'app e il percorso di installazione

La prima scheda contiene il nome dell'app, con l'autore e la versione sotto, ad esempio "Fabrikam Inc. · 1.7.3". Poi:

| Riga | Cosa indica |
| --- | --- |
| **Acquisita da** | Il nome del PC da cui proviene l'app. Compare solo se il programma di installazione è stato creato con l'opzione **Dati utente** attivata. |
| **Programma di installazione** | Se questo file del programma di installazione è [firmato](Glossary#signature): "Verifica della firma…", poi "Non firmato: autore sconosciuto" oppure "Firmato da" e il nome del firmatario. Se la firma non è valida, al suo posto compare un messaggio in rosso: "La firma di questo programma di installazione non è valida", con il motivo indicato da Windows, e "Il programma potrebbe essere stato modificato dopo la firma." |
| **Percorso di installazione** | La cartella in cui viene installata l'app, con **Sfoglia** per sceglierne una. All'inizio è la cartella in cui si trovava l'app nel vecchio PC. Un'app proveniente da Windows a 32 bit parte invece da `Program Files (x86)`, dove Windows a 64 bit conserva le app a 32 bit. Se questa cartella non può essere usata qui, un avviso in rosso sotto il campo ne indica il motivo. |
| **Spazio necessario** | Lo spazio necessario per l'installazione in quell'unità e lo spazio libero, ad esempio "fino a 824,0 MB in C: (356,0 GB liberi)". Include sia il pacchetto decompresso sia i file dell'app. |

Il nome dell'autore nella prima riga proviene dal pacchetto, quindi chiunque potrebbe averlo scritto. La riga **Programma di installazione** proviene da Windows ed è quella di cui fidarsi.

<a id="when-the-location-cannot-be-used"></a>

### Quando il percorso non può essere usato

Se la cartella indicata in **Percorso di installazione** non può essere usata, un avviso in rosso sotto il campo ne indica il motivo. **Installa** e **Anteprima modifiche** restano disattivati finché non ne scegli un'altra, e una riga accanto ai pulsanti indica "Installa e Anteprima modifiche restano disattivati finché non è possibile usare il percorso di installazione."

Il motivo più comune è che la cartella contiene già dei file. Una cartella che contiene file viene usata solo se li ha messi lì un'installazione precedente di WinAppMigrator, in modo che i file di un'altra app non vengano mai sostituiti:

| Avviso | Significato | Cosa fare |
| --- | --- | --- |
| "L'app", il nome dell'app e poi "è già installata in questa cartella, quindi questa copia non può essere installata qui." | L'app è già installata in questo PC, spesso tramite il suo programma di installazione: il programma principale si trova in quella cartella, oppure l'elenco delle app installate di Windows la indica lì. Succede con le app già presenti in molti PC, come un browser Web, un lettore multimediale o Office. | Disinstalla prima la copia presente in questo PC, da Impostazioni di Windows, in **App**, quindi apri di nuovo il programma di installazione. In alternativa, digita una cartella nuova e vuota in **Percorso di installazione** per installare questa copia accanto all'altra. |
| "L'app", il nome di un'altra app e poi "è installata in questa cartella", e che questa app "non può essere installata qui." | L'elenco delle app installate di Windows indica un'altra app in quella cartella, ad esempio un'altra edizione di Office. | Digita una cartella nuova e vuota in **Percorso di installazione**. Oppure, se l'altra app non ti serve più, disinstallala prima da Impostazioni di Windows, in **App**, quindi apri di nuovo il programma di installazione. |
| "Questa cartella contiene già file che non sono stati installati da WinAppMigrator." | Nella cartella c'è qualcos'altro e non risulta alcuna app installata lì. Potrebbe appartenere a un'altra app. | Digita una cartella nuova e vuota in **Percorso di installazione**. Se i file sono rimasti da un'app che hai disinstallato, puoi eliminare la cartella e poi aprire di nuovo il programma di installazione. |

Per qualsiasi altro motivo, l'avviso indica "Impossibile usare questa cartella:", il motivo e "Immetti qui sopra un'altra cartella." I motivi sono:

| Motivo visualizzato | Cosa fare |
| --- | --- |
| "deve essere un percorso completo" | Digita un percorso che inizi con una lettera di unità, ad esempio `C:\Program Files\Fabrikam Data Sync`. |
| "non può essere un'intera unità" | Usa una cartella nell'unità, non l'unità stessa. |
| "non può essere la cartella Windows" | Scegli una cartella esterna a `C:\Windows`. |
| "contiene altri elementi affiancati, quindi l'app richiede una cartella al suo interno" | Aggiungi una cartella per l'app, ad esempio `C:\Program Files\Fabrikam Data Sync` invece di `C:\Program Files`. |
| "è una cartella che appartiene a Windows o il cui contenuto viene eseguito o caricato automaticamente" | Scegli una cartella normale. Una cartella Esecuzione automatica, ad esempio, viene rifiutata. |
| "deve essere un percorso semplice in un'unità di questo PC" | Una [cartella condivisa in rete](Glossary#network-share) viene rifiutata, così come un nome che Windows interpreterebbe come qualcos'altro. |
| che "è un collegamento simbolico che punta altrove e altri potrebbero modificarne la destinazione" | Usa la cartella a cui punta il [collegamento simbolico](Glossary#link). |
| "è la cartella in cui le installazioni conservano i registri" | Scegli una cartella esterna a `%ProgramData%\WinAppMigrator`. |

Se la cartella contiene file e lì **risulta** un'installazione precedente di questo stesso progetto, il programma di installazione la usa e avvisa: "Questa cartella contiene già dei file. I file con lo stesso nome vengono sostituiti e gli originali vengono ripristinati se disinstalli l'app."

Una volta accettata una cartella, la finestra sostituisce un file, un valore del Registro di sistema, un servizio, un'attività, un collegamento o una variabile con lo stesso nome, come fa `--force` nella [riga di comando](Command-Line#what---force-does). Ciò che appartiene già a un'altra app, come un suo servizio o un suo tipo di file, resta comunque di quell'app.

Se lo spazio nell'unità sembra insufficiente, una nota color ambra indica "Lo spazio qui potrebbe non essere sufficiente" e lo spazio necessario. Il pacchetto può essere decompresso in un'altra unità, ma i file dell'app devono comunque starci in questa. L'installazione ricontrolla lo spazio prima di iniziare.

<a id="an-earlier-install-that-never-finished"></a>

### Un'installazione precedente mai completata

Se un'installazione precedente eseguita da uno di questi programmi di installazione è stata interrotta, da un arresto anomalo o da un'interruzione di corrente, una nota color ambra indica "Un'installazione precedente di" quell'app "non è mai stata completata. Facendo clic su Installa, verrà prima annullata." Questo avviene automaticamente quando fai clic su **Installa**.

<a id="what-this-sets-up-beyond-files"></a>

## Cosa configura oltre ai file

Questa scheda è l'elenco che accetti quando fai clic su **Installa**. Sotto il titolo indica:

> Tutte queste operazioni vengono eseguite con diritti di amministratore. Se l'installazione non riesce, le modifiche apportate vengono ripristinate e ti viene segnalato ciò che non è stato possibile ripristinare. Puoi anche annullare l'installazione in seguito da Impostazioni, in App installate. I file sostituiti vengono ripristinati, mentre i runtime installati restano.

Ogni riga contiene un nome, un'etichetta a destra che indica di che tipo di elemento si tratta e una riga che descrive cosa fa. Un'etichetta nel consueto colore blu indica un elemento ordinario. Un'etichetta **color ambra** segnala qualcosa che viene eseguito con diritti completi, come un servizio eseguito come LocalSystem, un driver o una regola del firewall che consente connessioni in entrata. Ricontrollali con attenzione.

Se l'elenco è vuoto, indica "Nulla: copia solo i file e le impostazioni nelle chiavi del Registro di sistema dell'app."

L'elenco viene calcolato per il percorso indicato nella casella. Se cambi il percorso, per un attimo compare "Analisi di ciò che verrebbe configurato in questo percorso…" e **Installa** resta in attesa.

<a id="what-each-label-means"></a>

### Significato delle etichette

| Etichetta | Che cos'è | Esempio della riga sottostante |
| --- | --- | --- |
| **Servizio** | Un [servizio](Glossary#service): un programma che Windows esegue in background | "Esegue FabrikamSync.exe come NT AUTHORITY\LocalService, si avvia ora e a ogni avvio di questo PC". Altri indicano "si avvia ora e poco dopo ogni avvio di questo PC", "si avvia solo quando richiesto" oppure "è disabilitato". |
| **Driver** | Un [driver](Glossary#driver) di proprietà dell'app | "Carica" un file "direttamente in Windows". Sempre color ambra. Un driver che si avvia con Windows viene avviato subito dopo l'installazione. Quello di VeraCrypt è stato caricato in questo modo, manualmente, in un PC di test; la CI non ne carica nessuno. |
| **Attività pianificata** | Un'[attività pianificata](Glossary#scheduled-task) | "Viene eseguita come SYSTEM" o come un account. "con i privilegi più elevati" è segnalato in color ambra. Un'attività che nel vecchio PC veniva eseguita come una persona viene eseguita come l'utente che la installa. |
| **Avvio automatico** | Un [elemento ad avvio automatico](Glossary#startup-item) | "Si avvia quando accedi", "Si avvia quando accede qualsiasi utente" o "Un collegamento nella cartella Esecuzione automatica, per te" |
| **Collegamento** | Un [collegamento](Glossary#shortcut) sul desktop o nel menu Start | "Apre Fabrikam.exe, per tutti gli utenti di questo PC" o "per te" |
| **PATH** | Una voce aggiunta alla [variabile Path](Glossary#path-variable) | "Aggiunge a Path" e "La cartella di installazione\bin". Viene aggiunta a ciò che c'è già, senza mai sostituirlo. |
| **Variabile di ambiente** | Una [variabile di ambiente](Glossary#environment-variable) | "Impostata su" e il suo valore |
| **Estensione della shell** | Codice che viene caricato in Esplora file | "Carica" un file "in Esplora file, per tutti gli utenti di questo PC". Color ambra. |
| **Comando di menu** | Un comando nel menu di scelta rapida | "Aggiunge un comando al menu di" un tipo di file. "Cambia cosa succede quando si apre uno di questi elementi" è color ambra. |
| **Tipo di collegamento** | Un [protocollo di collegamento](Glossary#link-protocol), ad esempio `fabrikam:` | "I collegamenti di questo tipo si aprono con l'app" |
| **Tipo di file** | Un [tipo di file](Glossary#file-type), ad esempio `.fds` | "I file di questo tipo si aprono con l'app" |
| **COM** | [Classi COM](Glossary#com-class) che altri programmi possono avviare | "COM in" un file, con il numero di impostazioni. "Autorizzazioni COM" è color ambra. |
| **Regola del firewall** | Una [regola del firewall](Glossary#firewall-rule) per uno dei suoi programmi | "Consente le connessioni in entrata sulla porta 8080 per FabrikamSync.exe, su tutte le reti". Consentire le connessioni in entrata è segnalato in color ambra. |
| **Origine del registro eventi** | Un nome con cui l'app scrive nei registri eventi di Windows | "Può scrivere nel registro Application con messaggi presi dai propri file" |
| **Tipo di carattere** | Un tipo di carattere nella cartella di installazione | "Viene caricato dalla cartella di installazione all'accesso" |
| **Driver ODBC** | Un driver di database per [ODBC](Glossary#odbc) | "Viene caricato nei programmi che usano ODBC per accedere a un database". Color ambra. |
| **Origine dati ODBC** | Una connessione a un database salvata | "Una connessione a un database, per tutti gli utenti di questo PC" |
| **Impostazione di compatibilità** | Il modo in cui Windows avvia alcuni dei suoi programmi | "Modalità di avvio dei programmi" |
| **App predefinite** | Una voce che consente di scegliere l'app in App predefinite | "Può essere scelto in App predefinite. Non viene modificato nulla" |
| **Nome del programma** | Un nome con cui Esegui e altri programmi possono trovarlo | "Esegui e altri programmi possono trovarlo con questo nome" |
| **Altre impostazioni** | Valori del [Registro di sistema](Glossary#registry) esterni alle chiavi dell'app e ai tipi precedenti | "3 impostazioni esterne alle chiavi dell'app", "In" e le chiavi. Color ambra se alcune si trovano nelle chiavi di Microsoft. |

Alcuni elementi possono essere valutati solo quando l'installazione arriva a quel punto, ad esempio un servizio con lo stesso nome che appartiene già a un'altra app. Questi restano nell'elenco così come richiesti. Quando l'installazione arriva a quel punto, li lascia invariati e lo segnala.

<a id="notes-and-what-is-left-out"></a>

### Note ed elementi esclusi

Sotto le righe, alcune note color ambra indicano ciò che è bene sapere, ad esempio:

- che le credenziali non sono state incluse nel programma di installazione, quindi potrebbe essere necessario immetterle di nuovo;
- che l'opzione **Dati di licenza e attivazione** o **Dati utente** era disattivata al momento della creazione, quindi quei dati non sono inclusi;
- su quali [certificati](Glossary#certificate) sembrava basarsi l'app, che non vengono mai installati;
- cosa ha escluso e cosa ha aggiunto chi ha creato il programma di installazione;
- che l'app potrebbe non funzionare qui: "Questa app si trovava nella build" di una versione di Windows più recente, oppure Windows eseguirebbe alcuni dei suoi programmi "tramite emulazione";
- che i file di Documenti o Giochi salvati "vanno invece" nella cartella di installazione, "in una cartella denominata _external", perché la cartella di questo PC si trova fuori dal tuo profilo.

Poi, in color ambra, ogni elemento escluso dalle regole di sicurezza, con il motivo. I primi 20 sono mostrati per intero. Gli altri sono raggruppati sotto "Altri N elementi esclusi", che puoi espandere. Il resto dell'installazione procede comunque senza di essi. Cosa viene rifiutato, e perché, è descritto in [Cosa contiene un programma di installazione](What-Goes-in-an-Installer#what-the-installer-refuses).

Se il pacchetto non può essere installato nel percorso scelto, un messaggio in rosso indica "Questo pacchetto non può essere installato in questo percorso" e il motivo.

<a id="options"></a>

## Opzioni

Questa scheda compare solo se il programma di installazione contiene dati utente o dati di licenza. Ogni casella è inizialmente selezionata.

| Opzione | Cosa indica |
| --- | --- |
| **Dati utente** | "Attivato: le impostazioni, Documenti e Giochi salvati vanno nelle cartelle consuete dell'utente che ha eseguito l'accesso. Disattivato: tutto va nella cartella di installazione e non viene configurato nulla da eseguire da lì." Se il programma di installazione indica di chi sono i dati che contiene, una riga sotto riporta "Contiene i dati di" e quell'account. |
| **Licenza** | "File di attivazione e di licenza, e valori del Registro di sistema che contengono licenze. Alcuni potrebbero richiedere una nuova attivazione qui." |

Se cambi una casella, l'elenco precedente si aggiorna subito di conseguenza.

I dati utente vanno sempre all'**utente che ha eseguito l'accesso** a Windows, anche se nella richiesta di Controllo dell'account utente è stato digitato il nome di un altro amministratore. Se il programma di installazione non riesce a determinare di chi si tratta, lo segnala, esclude le impostazioni per utente e mette i dati utente nella cartella di installazione invece di tirare a indovinare. Se nessuno ha eseguito l'accesso a quella sessione, i dati utente e le impostazioni per utente vanno all'account con cui viene eseguito il programma di installazione, e anche questo viene segnalato.

<a id="dependencies"></a>

## Dipendenze

"Ciò di cui l'app ha bisogno e come viene fornito ciascun elemento." Ogni [runtime](Glossary#runtime) ha un'etichetta:

| Etichetta | Significato |
| --- | --- |
| **Incluso** | All'interno del programma di installazione. Viene installato senza Internet, tranne DirectX End-User Runtime, il cui programma di installazione lo scarica da Microsoft. Tra questi, solo il runtime Visual C++ x64 è stato effettivamente installato nei test del progetto. |
| **Installato con winget:** e un ID | Non è stato possibile scaricarlo durante la creazione del programma di installazione, quindi il programma di installazione usa [winget](Glossary#winget), che richiede Internet. Questa modalità non è stata provata nei test del progetto. |
| **Funzionalità di Windows: NetFx3** | .NET Framework 3.5, attivato come [funzionalità di Windows](Glossary#windows-feature). Può richiedere Internet. Questa modalità non è stata provata nei test del progetto. |
| **Integrato in Windows** | .NET Framework 4, già presente in Windows 10 e 11 |
| **Fornito con l'app** | L'app include una propria copia |
| **Da installare manualmente** | Qui non è disponibile un programma di installazione per questo runtime. Installalo tu. |

Se non ce ne sono, indica "Non sono state rilevate altre dipendenze di runtime." Vedi [Runtime](Runtimes).

<a id="preview-changes"></a>

## Anteprima modifiche

**Anteprima modifiche** calcola esattamente cosa farebbe un'installazione, senza modificare nulla. Verifica che ci sia spazio, decomprime il pacchetto in una cartella privata, calcola le modifiche e rimuove di nuovo la cartella. La scheda del risultato ha il titolo "Anteprima: non è stato ancora modificato nulla", con **Modifiche previste** e un elenco delle modifiche, con servizi, attività e collegamenti in cima. La finestra elenca le prime 300, poi indica quante altre ce ne sono.

Un'anteprima non controlla l'hash di ogni file. L'hash dell'intero pacchetto è stato controllato all'apertura della finestra.

<a id="install"></a>

## Installa

**Installa** resta disattivato finché il percorso non può essere usato e l'elenco non corrisponde a quel percorso. Se il pacchetto non può essere installato in quel percorso, una riga accanto al pulsante indica "Installa è disattivato perché questo pacchetto non può essere installato in questo percorso." e il motivo compare in rosso in **Cosa configura oltre ai file**. Quando fai clic sul pulsante:

1. Calcola l'elenco ancora una volta. Se ciò che farebbe è cambiato da quando l'hai controllato, non installa nulla, mostra il nuovo elenco e indica "Ciò che verrebbe eseguito qui è cambiato, quindi non è stato installato nulla. Controlla l'elenco qui sotto e, se è quello che vuoi, fai di nuovo clic su Installa."
2. Verifica che ci sia spazio in ogni unità in cui scrive.
3. Decomprime il pacchetto in una cartella accessibile solo agli amministratori, controllando l'hash di ogni file man mano che viene estratto.
4. Installa, annotando prima ogni modifica in un [journal](Glossary#journal), in modo da poterla annullare.
5. Rilegge ogni file installato e ne controlla l'hash.
6. Avvia i servizi configurati per l'avvio automatico.

In basso vedi cosa sta facendo, un conteggio quando è disponibile, il tempo trascorso, una barra di avanzamento e **Annulla**. I passaggi che potresti vedere sono "Preparazione in corso…", "Decompressione", "Controllo del pacchetto", "Annullamento di un'installazione precedente mai completata", "Installazione dei runtime", "Copia dei file", "Configurazione dei servizi", "Configurazione dei driver", "Scrittura delle impostazioni del Registro di sistema", "Ripristino dei dati di licenza", "Configurazione delle attività pianificate", "Creazione dei collegamenti", "Impostazione delle variabili di ambiente", "Aggiunta dell'app a Impostazioni" e "Controllo di ciò che è stato installato".

**Annulla** mostra "Annullamento al termine del passaggio corrente…" e poi "Ripristino delle modifiche". Chiudere la finestra durante l'installazione equivale a fare clic su **Annulla**: la finestra resta aperta finché tutto non è stato ripristinato. Se non era ancora stato modificato nulla, indica "Operazione annullata. Non è stato modificato nulla."

<a id="the-hash-check"></a>

## Il controllo hash

Il programma di installazione controlla gli [hash](Glossary#hash) tre volte:

1. **All'apertura della finestra**: l'hash dell'intero pacchetto. Una copia troncata, o modificata dopo la creazione, viene rifiutata prima che venga letto qualsiasi suo contenuto.
2. **Durante la decompressione**: l'hash di ogni file, prima che il file venga scritto nella sua posizione.
3. **Dopo l'installazione**: rilegge ogni file installato e lo controlla di nuovo.

Gli hash rilevano i danni e le modifiche apportate da chiunque non aggiorni anche gli hash. Non dimostrano chi ha creato un programma di installazione. Solo una [firma](Glossary#signature) può farlo.

<a id="the-result"></a>

## Il risultato

La scheda del risultato compare in alto, con il titolo "Risultato dell'installazione".

| Messaggio | Significato |
| --- | --- |
| "L'app è stata installata e ha superato il controllo hash." | È tutto a posto. Se le regole hanno escluso qualcosa, aggiunge "senza gli elementi indicati come esclusi qui sotto." |
| "L'installazione è stata annullata e le modifiche apportate sono state ripristinate." | Hai annullato l'installazione. |
| "L'installazione non è riuscita e le modifiche apportate sono state ripristinate." | Qualcosa non è riuscito. Le modifiche apportate sono state ripristinate, tranne gli eventuali runtime installati. Ciò che è elencato in color ambra sotto il messaggio è rimasto, con il motivo, ad esempio un servizio che viene rimosso solo al riavvio del PC. |
| "L'installazione non è riuscita e non è stato possibile ripristinare tutte le modifiche apportate. Ciò che rimane è elencato qui sotto; la prossima installazione riproverà." | Leggi l'elenco. Eseguendo di nuovo un programma di installazione, verrà ritentato il ripristino. |
| "L'installazione non è stata completata correttamente. Esamina i problemi qui sotto." | Leggi i messaggi sotto. |

Poi tre riquadri: **Modifiche apportate** (o **Modifiche ripristinate** dopo un errore), **Avvisi** ed **Errori**. Le etichette indicano **Hash del pacchetto validi** e **File installati validi**, oppure **NON VALIDI** in rosso. Vengono mostrati come messaggi fino a cinque avvisi e tutti gli errori. "Una dipendenza richiede un riavvio per essere completata. Riavvia Windows, quindi avvia l'app." significa che un runtime ha richiesto un riavvio.

**Modifiche**, con un conteggio, apre l'elenco delle operazioni eseguite, fino a 300. Se l'installazione non è stata completata correttamente, il percorso del [log](Glossary#log) viene mostrato come **Log**.

Dopo un'installazione senza problemi, i pulsanti in basso diventano **Chiudi** e **Apri cartella di installazione**.

<a id="where-things-end-up"></a>

## Dove finiscono i file

- I **file del programma** vanno nel percorso di installazione.
- I **Dati utente**, con quella casella selezionata, vanno nelle cartelle AppData, Documenti e Giochi salvati dell'account che ha eseguito l'accesso, e in ProgramData. Una cartella Documenti o Giochi salvati che questo PC conserva fuori dal tuo profilo non viene usata per la scrittura. Quei file vanno nel percorso di installazione, in `_external`, e l'elenco lo indica per primo. Con la casella deselezionata, tutti i dati utente vanno nel percorso di installazione.
- I **collegamenti** vanno nel desktop e nel menu Start di questo PC: i tuoi, oppure quelli di tutti gli utenti se si trovavano lì.
- A **Path** vengono aggiunte solo le voci dell'app. Nient'altro viene modificato.
- **Ciò che qui appartiene già a un'altra app resta di quell'app.** Un tipo di file, un tipo di collegamento, una classe COM, un servizio, un'origine del registro eventi, un tipo di carattere o qualsiasi elemento simile che appartiene già a qualcos'altro viene lasciato invariato, e il programma di installazione lo segnala. Un'app che si aspettava di aprire i collegamenti `http`, ad esempio, deve essere impostata di nuovo come predefinita in Impostazioni.
- In **Impostazioni** viene aggiunta una voce per l'app, quindi puoi disinstallarla come qualsiasi altra. Vedi [Annullare un'installazione](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Motivi comuni di rifiuto

| Cosa vedi | Perché | Cosa fare |
| --- | --- | --- |
| "questa app non può essere eseguita in questo PC, quindi non è stato modificato nulla" | I suoi programmi sono compilati per un altro tipo di processore o richiedono una versione di Windows più recente | Installala in un PC simile a quello vecchio |
| "lo spazio non è sufficiente, quindi non è stato modificato nulla" | Non c'è abbastanza spazio libero in un'unità in cui scrive | Libera spazio o scegli un percorso in un'altra unità |
| "è in corso un'altra installazione o disinstallazione" | Può esserne eseguita solo una alla volta | Attendi che termini, quindi riprova |
| Un file "ha un nome che questa versione di Windows apre come dispositivo e non come file" | L'app contiene un file con un nome come `aux.h`, che le versioni di Windows precedenti a Windows 11 non possono contenere | Installala in Windows 11 |
| Una cartella di dati come `%AppData%` si trova sotto un collegamento simbolico | Il programma di installazione non scrive attraverso un collegamento simbolico | Deseleziona **Dati utente** e **Licenza** e installa senza questi dati |

Altri casi sono descritti in [Risoluzione dei problemi](Troubleshooting#installing).
