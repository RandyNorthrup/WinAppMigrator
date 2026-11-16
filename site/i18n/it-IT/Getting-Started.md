<a id="getting-started"></a>

# Per iniziare

Questa pagina ti accompagna dal download alla prima schermata dell'app.

<a id="what-you-need"></a>

## Requisiti

- **Windows 10 o Windows 11 a 64 bit.** L'app è un programma x64. Windows 11 su Arm lo esegue tramite [emulazione](Glossary#emulation). Windows 10 su Arm non può eseguire programmi x64 e Windows a 32 bit non può eseguirlo affatto. Windows 10 e 11 desktop su x64 non sono stati provati nei test del progetto, che si svolgono su Windows Server 2022 e 2025 e su Windows 11 su Arm. Vedi [Requisiti e test](Requirements-and-Testing).
- **Diritti di amministratore** nel PC. L'app li chiede sempre all'avvio. Vedi [Diritti di amministratore](Glossary#administrator-rights).
- **Direct3D 12**, che Windows usa per disegnare la finestra dell'app. Windows 10 e 11 normalmente lo includono. Un driver grafico obsoleto o una macchina virtuale potrebbero non averlo. Vedi [Direct3D 12](Glossary#direct3d-12).
- **Spazio in un'unità** per il programma di installazione che crei. L'app ti indica quanto ne serve prima di iniziare.
- **La connessione a Internet nel vecchio PC è facoltativa.** Se è disponibile durante la creazione, l'app scarica i [runtime](Glossary#runtime) Microsoft che può includere e li inserisce nel programma di installazione. Vedi [Runtime](Runtimes).

Maggiori dettagli, e cosa è stato testato e cosa no, sono in [Requisiti e test](Requirements-and-Testing).

<a id="download"></a>

## Download

Vai alla [versione più recente](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Contiene tre file:

| File | Che cos'è | Scaricalo se |
| --- | --- | --- |
| `WinAppMigrator-v0.7.0.exe` | L'app vera e propria. Un unico file, senza niente da installare. | Vuoi semplicemente usarla. È il file che serve alla maggior parte delle persone. |
| `WinAppMigrator-v0.7.0-windows-x64.zip` | La stessa app, come `winappmigrator.exe`, con il file README, il registro delle modifiche, la licenza e le licenze di terze parti | Vuoi anche la documentazione |
| `SHA256SUMS.txt` | I checksum dei due file precedenti | Vuoi verificare il download |

La parte di installazione inclusa in ogni programma di installazione, lo [stub del programma di installazione](Glossary#installer-stub), non viene mai pubblicata separatamente. È già inclusa nell'app.

<a id="check-the-download"></a>

## Verifica il download

Questo passaggio è facoltativo, ma vale la pena farlo. Ogni controllo seguente è indipendente dagli altri. Eseguili in PowerShell, nella cartella in cui hai salvato il file.

**1. Il checksum.** Un [hash](Glossary#hash) è l'impronta digitale di un file. Se cambia anche un solo byte, cambia anche l'hash.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.0.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Il numero lungo restituito da `Get-FileHash` deve corrispondere a quello sulla riga di quel file. `Get-FileHash` stampa lettere maiuscole, mentre il file usa lettere minuscole. Non è un problema: confronta lettere e cifre.

**2. La firma.** Una [firma](Glossary#signature) indica chi ha creato il file e che da allora non è stato modificato.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.0.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

`Status` deve indicare `Valid` e `Subject` deve includere `CN=Randy Northrup`. Puoi anche fare clic con il pulsante destro del mouse sul file, scegliere **Proprietà** e aprire la scheda **Firme digitali**. La versione 0.5.0 è la prima in cui l'app è firmata. I download delle versioni precedenti non sono firmati.

**3. Dove è stato creato.** Ogni versione a partire dalla 0.5.0 registra che è stata compilata dal flusso di lavoro di rilascio di GitHub, a partire da questo repository. Questa registrazione si chiama [attestazione](Glossary#attestation). Se hai [GitHub CLI](https://cli.github.com/), esegui:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.0.exe --repo RandyNorthrup/WinAppMigrator
```

Lo stesso vale per il file zip.

<a id="start-it"></a>

## Avvia l'app

1. **Fai doppio clic** su `WinAppMigrator-v0.7.0.exe`. Non c'è niente da installare. L'app viene eseguita dal percorso in cui l'hai salvata.
2. **SmartScreen.** [SmartScreen](Glossary#smartscreen) di Windows potrebbe mostrare la schermata "PC protetto da Windows" per un file che non ha visto spesso. In questo caso, scegli **Ulteriori informazioni** e verifica che l'autore sia Randy Northrup prima di scegliere **Esegui comunque**.
3. **La richiesta di amministratore.** Windows chiede se consentire all'app di apportare modifiche. Si tratta della [richiesta di Controllo dell'account utente](Glossary#uac-prompt). L'app la mostra sempre, perché per leggere l'unità di un altro PC, e i file che solo gli amministratori possono leggere, servono i diritti di amministratore. Per il download firmato, la richiesta indica **Randy Northrup** come autore verificato. Se per questo file indica "Autore sconosciuto", non si tratta del file di rilascio firmato, quindi non eseguirlo.
4. **La finestra si apre.** Se invece vedi un messaggio che indica che non è stato possibile aprire la finestra e che è necessario Direct3D 12, vedi [Risoluzione dei problemi](Troubleshooting#the-window-could-not-open). L'app non può essere eseguita senza la sua finestra.

<a id="first-run"></a>

## Primo avvio

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="L'app con Fabrikam Data Sync selezionata" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- L'elenco delle app installate è a sinistra. Durante il caricamento, la piccola etichetta in alto a destra indica **Caricamento** e poi il numero di app trovate, ad esempio **5 app**.
- La finestra si apre in modalità chiara o scura, come Windows. Il pulsante con il sole o la luna in alto a destra cambia modalità e l'app ricorda la tua scelta. Vedi [File che conserva in un PC](Files-It-Keeps-on-a-PC). Con un tema a contrasto di Windows, usa i colori del tema e non mostra questo pulsante.
- Usa la lingua di visualizzazione di Windows se è una delle sue 24, altrimenti l'inglese. **Lingua**, in alto a destra, elenca le lingue ciascuna con il proprio nome e cambia subito la lingua della finestra; l'app ricorda la tua scelta. Arabo, persiano, urdu ed ebraico vengono visualizzati da destra a sinistra. Vedi [Lingue e accessibilità](Languages-and-Accessibility).
- Fai clic su un'app. Il lato destro mostra cosa verrebbe incluso nel suo programma di installazione e inizia subito ad analizzarlo. Non viene creato nulla finché non fai clic su **Crea programma di installazione**.

Passaggio successivo: [Creare un programma di installazione](Building-an-Installer).
