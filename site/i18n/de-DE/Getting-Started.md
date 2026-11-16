<a id="getting-started"></a>

# Erste Schritte

Diese Seite führt Sie vom Download bis zum ersten Bildschirm der App.

<a id="what-you-need"></a>

## Was Sie benötigen

- **Windows 10 oder Windows 11, 64 Bit.** Die App ist ein x64-Programm. Windows 11 on Arm führt sie per [Emulation](Glossary#emulation) aus. Windows 10 on Arm kann keine x64-Programme ausführen, und unter 32-Bit-Windows läuft sie überhaupt nicht. Desktopversionen von Windows 10 und 11 auf x64 wurden in den Tests des Projekts nicht ausgeführt; diese verwenden Windows Server 2022 und 2025 sowie Windows 11 on Arm. Siehe [Anforderungen und Tests](Requirements-and-Testing).
- **Administratorrechte** auf dem PC. Die App fordert sie beim Start immer an. Siehe [Administratorrechte](Glossary#administrator-rights).
- **Direct3D 12**, mit dem Windows das Fenster der App darstellt. Windows 10 und 11 verfügen normalerweise darüber. Ein alter Grafiktreiber oder ein virtueller Computer möglicherweise nicht. Siehe [Direct3D 12](Glossary#direct3d-12).
- **Speicherplatz auf einem Laufwerk** für das Installationsprogramm, das Sie erstellen. Die App teilt Ihnen vorher mit, wie viel.
- **Internet auf dem alten PC ist optional.** Ist es beim Erstellen verfügbar, lädt die App die Microsoft-[Laufzeitkomponenten](Glossary#runtime) herunter, die sie aufnehmen kann, und legt sie in das Installationsprogramm. Siehe [Laufzeitkomponenten](Runtimes).

Weitere Details, auch dazu, was getestet wurde und was nicht, finden Sie unter [Anforderungen und Tests](Requirements-and-Testing).

<a id="download"></a>

## Herunterladen

Öffnen Sie das [neueste Release](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest). Es enthält drei Dateien:

| Datei | Was sie ist | Die richtige Wahl, wenn |
| --- | --- | --- |
| `WinAppMigrator-v0.7.1.exe` | Die App selbst. Eine einzige Datei, ohne Installation. | Sie die App einfach verwenden möchten. Das ist die Datei, die die meisten brauchen. |
| `WinAppMigrator-v0.7.1-windows-x64.zip` | Dieselbe App als `winappmigrator.exe`, mit README, Änderungsprotokoll, Lizenz und den Lizenzen von Drittanbietern | Sie auch die Begleitdokumente möchten |
| `SHA256SUMS.txt` | Die Prüfsummen der beiden obigen Dateien | Sie Ihren Download überprüfen möchten |

Der Teil, der in jedem Installationsprogramm steckt, der [Installationsprogramm-Stub](Glossary#installer-stub), wird nie einzeln veröffentlicht. Er ist bereits in der App enthalten.

<a id="check-the-download"></a>

## Download überprüfen

Das ist optional, lohnt sich aber. Jede der folgenden Prüfungen ist von den anderen unabhängig. Führen Sie sie in PowerShell aus, in dem Ordner, in dem Sie die Datei gespeichert haben.

**1. Die Prüfsumme.** Ein [Hash](Glossary#hash) ist der Fingerabdruck einer Datei. Ändert sich auch nur ein Byte, ändert sich der Hash.

```powershell
Get-FileHash .\WinAppMigrator-v0.7.1.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Die lange Zahl von `Get-FileHash` sollte mit der in der Zeile für diese Datei übereinstimmen. `Get-FileHash` gibt Großbuchstaben aus, die Datei verwendet Kleinbuchstaben. Das ist in Ordnung: Vergleichen Sie die Buchstaben und Ziffern.

**2. Die Signatur.** Eine [Signatur](Glossary#signature) gibt an, wer die Datei erstellt hat und dass sie seitdem nicht geändert wurde.

```powershell
$signature = Get-AuthenticodeSignature .\WinAppMigrator-v0.7.1.exe
$signature.Status                     # should say Valid
$signature.SignerCertificate.Subject  # should include CN=Randy Northrup
```

Sie können auch mit der rechten Maustaste auf die Datei klicken, **Eigenschaften** auswählen und die Registerkarte **Digitale Signaturen** ansehen. Version 0.5.0 ist das erste Release, dessen App signiert ist. Downloads aus früheren Releases sind nicht signiert.

**3. Wo sie erstellt wurde.** Für jedes Release ab 0.5.0 ist festgehalten, dass der Release-Workflow von GitHub selbst es aus diesem Repository erstellt hat. Dieser Nachweis heißt [Attestierung](Glossary#attestation). Wenn Sie die [GitHub CLI](https://cli.github.com/) installiert haben, führen Sie Folgendes aus:

```powershell
gh attestation verify .\WinAppMigrator-v0.7.1.exe --repo RandyNorthrup/WinAppMigrator
```

Dasselbe funktioniert für die ZIP-Datei.

<a id="start-it"></a>

## Starten

1. **Doppelklicken** Sie auf `WinAppMigrator-v0.7.1.exe`. Es muss nichts installiert werden. Die App wird dort ausgeführt, wo Sie sie gespeichert haben.
2. **SmartScreen.** Windows [SmartScreen](Glossary#smartscreen) zeigt bei einer Datei, die es noch selten gesehen hat, möglicherweise den Bildschirm „Der Computer wurde durch Windows geschützt“ an. Wählen Sie in diesem Fall **Weitere Informationen**, und prüfen Sie, ob Randy Northrup als Herausgeber angegeben ist, bevor Sie **Trotzdem ausführen** wählen.
3. **Die Administratoraufforderung.** Windows fragt, ob die App Änderungen vornehmen darf. Das ist die [Aufforderung der Benutzerkontensteuerung](Glossary#uac-prompt). Die App fragt immer, weil das Lesen des Laufwerks eines anderen PCs und von Dateien, die nur Administratoren lesen dürfen, Administratorrechte erfordert. Beim signierten Download nennt die Aufforderung **Randy Northrup** als verifizierten Herausgeber. Steht dort für diese Datei „Unbekannter Herausgeber“, ist es nicht die signierte Release-Datei. Führen Sie sie dann nicht aus.
4. **Das Fenster wird geöffnet.** Wenn stattdessen eine Meldung erscheint, dass das Fenster nicht geöffnet werden konnte und Direct3D 12 benötigt wird, lesen Sie [Problembehandlung](Troubleshooting#the-window-could-not-open). Ohne ihr Fenster kann die App nicht ausgeführt werden.

<a id="first-run"></a>

## Erster Start

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-dark.png">
    <img alt="Die App mit der ausgewählten Anwendung Fabrikam Data Sync" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/app-pick-light.png" width="860">
  </picture>
</p>

- Die Liste der installierten Apps befindet sich links. Während sie geladen wird, zeigt die kleine Beschriftung oben rechts **Wird geladen** und dann, wie viele Apps gefunden wurden, z. B. **5 Apps**.
- Das Fenster startet im hellen oder dunklen Modus, je nachdem, welchen Windows verwendet. Die Schaltfläche mit Sonne oder Mond oben rechts schaltet um, und die App merkt sich Ihre Wahl. Siehe [Dateien, die die App auf einem PC speichert](Files-It-Keeps-on-a-PC). In einem Windows-Kontrastdesign verwendet sie die Farben des Designs und hat keine solche Schaltfläche.
- Die App spricht Ihre Windows-Anzeigesprache, wenn diese zu ihren 24 Sprachen gehört, andernfalls Englisch. **Sprache** oben rechts listet die Sprachen unter ihren eigenen Namen auf und ändert das Fenster sofort; die App merkt sich Ihre Wahl. Arabisch, Persisch, Urdu und Hebräisch werden von rechts nach links dargestellt. Siehe [Sprachen und Barrierefreiheit](Languages-and-Accessibility).
- Klicken Sie auf eine App. Die rechte Seite zeigt, was in ihr Installationsprogramm käme, und beginnt sofort, das zu ermitteln. Es wird nichts erstellt, bis Sie auf **Installationsprogramm erstellen** klicken.

Weiter: [Installationsprogramm erstellen](Building-an-Installer).
