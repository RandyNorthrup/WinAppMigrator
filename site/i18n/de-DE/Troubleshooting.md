<a id="troubleshooting"></a>

# Problembehandlung

Suchen Sie unten, was Sie sehen: nach dem, was passiert, oder nach den Wörtern auf dem Bildschirm. Zitierte Wörter sind das, was das Programm anzeigt, manchmal mit einem Namen oder einer Zahl in der Mitte.

Wenn nichts davon hilft, lesen Sie [Hilfe erhalten](#getting-help) am Ende.

<a id="the-window-could-not-open"></a>

## Das Fenster konnte nicht geöffnet werden

**Was Sie sehen.** Ein Meldungsfeld statt des Fensters:

- von der App: „Das Fenster konnte nicht geöffnet werden“ und „Zum Darstellen des Fensters wird Direct3D 12 benötigt. Ein alter Grafiktreiber oder ein virtueller Computer bietet das möglicherweise nicht.“
- von einem Installationsprogramm: dasselbe und „Die Installation ist auch ohne Fenster möglich: Führen Sie diese Datei in einer Eingabeaufforderung mit Administratorrechten mit --apply aus. Mit --target-root und einem Ordner legen Sie fest, wohin installiert wird.“
- von einem Deinstallationsprogramm: dasselbe und „Die Deinstallation ist auch ohne Fenster möglich, in einer Eingabeaufforderung mit Administratorrechten:“ mit dem genauen Befehl.

**Warum.** Die Fenster werden mit [Direct3D 12](Glossary#direct3d-12) dargestellt, und es gibt nichts anderes, womit sie dargestellt werden könnten.

**Was zu tun ist.**

- Aktualisieren Sie den Grafiktreiber des PCs.
- Verwenden Sie für ein Installations- oder Deinstallationsprogramm die Befehlszeile. Siehe [Befehlszeile](Command-Line).
- Die App hat keine Befehlszeile. Erstellen Sie das Installationsprogramm auf einem anderen PC: Sie können das Laufwerk des alten PCs an diesen anschließen. Siehe [Übertragen von einem angeschlossenen Laufwerk](Moving-from-a-Connected-Drive). Dieser PC kann den alten auch über das Netzwerk lesen. Siehe [Übertragen von einem PC über das Netzwerk](Moving-over-the-Network).

<a id="starting-the-app"></a>

## Die App starten

| Was Sie sehen | Bedeutung | Was zu tun ist |
| --- | --- | --- |
| Die Aufforderung der Benutzerkontensteuerung zeigt für die App „Unbekannter Herausgeber“ | Ab 0.5.0 ist die App signiert. Eine nicht signierte App ist nicht die Release-Datei oder stammt von vor 0.5.0. | Laden Sie sie erneut aus dem [neuesten Release](https://github.com/RandyNorthrup/WinAppMigrator/releases/latest) herunter, und [überprüfen Sie sie](Getting-Started#check-the-download). |
| SmartScreen meldet „Der Computer wurde durch Windows geschützt“ | SmartScreen hat diese Datei noch selten gesehen | Wählen Sie **Weitere Informationen**, prüfen Sie, ob Randy Northrup als Herausgeber angegeben ist, und wählen Sie dann **Trotzdem ausführen**. |
| Die Beschriftung oben zeigt **Laden fehlgeschlagen**, und die Liste zeigt „Anwendungen konnten nicht aufgelistet werden“ | Die Liste der Apps konnte nicht gelesen werden | Klicken Sie zum erneuten Laden auf den kreisförmigen Pfeil. Prüfen Sie bei einem angeschlossenen Laufwerk, ob es noch angeschlossen ist. |
| Die gewünschte App ist nicht in der Liste | Es ist eine Microsoft Store-App, ein ausgeblendeter Teil einer anderen App, oder sie hat überhaupt keinen Eintrag in den Einstellungen, z. B. eine portable App | Store-Apps können nicht übertragen werden. Ein ausgeblendeter Teil wird möglicherweise unter **Gehört möglicherweise auch dazu** angeboten, wenn Sie die Haupt-App auswählen. Eine App ohne Eintrag in den Einstellungen kann nicht ausgewählt werden. |
| „Keine passenden Anwendungen“ | Die Suche oder der Filter blendet alles aus | Leeren Sie das Suchfeld, und wählen Sie **Alle** aus. |

<a id="building"></a>

## Erstellen

<a id="picking-a-drive-or-a-disk-image"></a>

### Ein Laufwerk oder Datenträgerabbild auswählen

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| Das Laufwerk „sieht nicht wie ein Windows-Systemlaufwerk aus (\Windows und \Users fehlen). Wählen Sie das Stammverzeichnis des Laufwerks vom anderen PC aus.“ | Wählen Sie die oberste Ebene des Laufwerks aus, z. B. `E:\`, keinen Ordner darauf. Prüfen Sie, ob es das Laufwerk ist, auf dem Windows installiert war. |
| Ein Abbild „ist kein Datenträgerabbild, das Windows anfügen kann (.vhd oder .vhdx)“ | Wählen Sie eine `.vhd`- oder `.vhdx`-Datei aus. Speichern Sie einen VMware- oder VirtualBox-Datenträger zuerst in einem dieser Formate. Exportieren Sie bei einem Hyper-V-Prüfpunkt, `.avhdx`, den Datenträger des virtuellen Computers. |
| Ein Abbild „konnte nicht angefügt werden: Windows hat das Abbild bereits angefügt, und was es angefügt hat, schreibt möglicherweise darauf.“ | Es ist an anderer Stelle geöffnet, z. B. im Datei-Explorer bereitgestellt. Wählen Sie stattdessen sein Laufwerk unter **Angeschlossenes Laufwerk…** aus, oder trennen Sie es, und wählen Sie es dann erneut aus. |
| Ein Abbild „enthält kein lesbares Volume mit Windows. Ein mit BitLocker verschlüsseltes Volume muss zuerst entsperrt werden.“ | Prüfen Sie, ob es ein Abbild des Laufwerks ist, auf dem Windows installiert war. Wenn es mit [BitLocker](Glossary#bitlocker) gesperrt ist, stellen Sie es selbst in Windows bereit, entsperren Sie es, und wählen Sie sein Laufwerk unter **Angeschlossenes Laufwerk…** aus. |
| Ein Abbild „ist ein differenzierender Datenträger, der nur die Änderungen seit dem Datenträger enthält, aus dem er erstellt wurde“ | Führen Sie es mit „Datenträger bearbeiten“ in Hyper-V mit dem Datenträger zusammen, aus dem es erstellt wurde, und wählen Sie diesen aus. |
| Ein Abbild „konnte nicht angefügt werden:“ und ein anderer Grund | Der Grund stammt von Windows selbst. Lesen Sie ihn, prüfen Sie, ob nichts anderes das Abbild verwendet, und versuchen Sie es erneut. |

<a id="reading-a-pc-over-the-network"></a>

### Einen PC über das Netzwerk lesen

Die vollständige Liste, einschließlich der Meldungen des Clients auf dem alten PC, finden Sie unter [Übertragen von einem PC über das Netzwerk](Moving-over-the-Network#if-something-goes-wrong).

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| Der alte PC wird unter **2. Den alten PC auswählen** nicht aufgeführt | Prüfen Sie, ob der Client dort ausgeführt wird und einen Code anzeigt und ob sich beide PCs im selben Netzwerk befinden. Geben Sie dann seinen Namen oder seine Adresse ein. Über einen Router hinweg geben Sie seine Adresse ein. Eine andere Firewall als die von Windows muss TCP 47732 und UDP 47731 durchlassen. |
| Ein Name „wurde im Netzwerk nicht gefunden“ | Geben Sie stattdessen die Adresse des alten PCs ein. |
| „Der alte PC war nicht erreichbar:“ und ein Grund | Prüfen Sie, ob der Client noch ausgeführt wird und ob Name oder Adresse stimmen. |
| „Das ist nicht der Code, den der alte PC anzeigt. Prüfen Sie ihn, und versuchen Sie es erneut.“ | Geben Sie den Code ein, den der Client jetzt anzeigt. Nach fünf falschen Codes erzeugt er einen neuen. Nach 20 lässt er keine Kopplung mehr zu, bis Sie ihn neu starten. |
| Auf dem alten PC meldet Windows, dass der Herausgeber des Clients unbekannt ist | Das ist nur auf einem Windows 7 ohne das Microsoft-Stammzertifikat von 2020 zu erwarten, das die Signatur des Clients nicht prüfen kann. Wählen Sie dort nur dann **Ja**, wenn Sie den Client selbst aus der App gespeichert haben. Überall sonst sollte die Aufforderung Randy Northrup nennen. Wenn nicht, führen Sie diese Kopie nicht aus. |
| Die App kann den alten PC nach einer langen Pause nicht mehr lesen | Der Client beendet eine Verbindung, an die die App 30 Minuten lang keine Anfrage gestellt hat. Klicken Sie auf **Trennen**, und koppeln Sie erneut. |

<a id="looking-at-the-app"></a>

### Die App analysieren

| Was Sie sehen | Bedeutung | Was zu tun ist |
| --- | --- | --- |
| „Diese App konnte nicht analysiert werden:“ und ein Grund | Die Analyse ist fehlgeschlagen. Ein häufiger Grund ist ein Eintrag ganz ohne Programm, z. B. ein Treiberpaket. | Klicken Sie auf **Erneut versuchen**. Wenn es auf dieselbe Weise fehlschlägt, ist der Eintrag möglicherweise keine App, die WinAppMigrator übertragen kann. |
| „Beendet. Ein Installationsprogramm kann erstellt werden, sobald diese App analysiert wurde.“ | Sie haben auf **Beenden** geklickt | Klicken Sie auf **Erneut analysieren**. |
| Die Analyse dauert lange | Eine große App, ein langsames oder ausfallendes Laufwerk oder viele Konten | Wenn das Laufwerk ausfallen könnte, klicken Sie auf **Beenden**, erstellen Sie ein [Datenträgerabbild](Glossary#disk-image) davon, und wählen Sie stattdessen dieses aus, denn das Lesen eines ausfallenden Laufwerks kann ihm den Rest geben. Ab 0.6.0 wählen Sie es mit **Datenträgerabbild…** aus. Unter 0.5.0 stellen Sie es bereit und wählen sein Laufwerk aus. Andernfalls warten Sie. Sie können in der Zwischenzeit eine andere App auswählen. |
| Eine gewünschte Zeile ist ausgegraut und aktiviert | Sie enthält das Hauptprogramm oder etwas, das es zum Starten benötigt | Sie wird immer aufgenommen. Siehe [die Zeilen, die Sie nicht deaktivieren können](Building-an-Installer#the-groups). |
| Eine Zeile zeigt „Ebenfalls ausgelassen:“ | Etwas, das Sie deaktiviert haben, hat sie mitgenommen | Aktivieren Sie dieses Element wieder, wenn Sie diese Zeile möchten. |
| „Der Rest ergäbe kein Paket“ | Was Sie deaktiviert haben, lässt nichts übrig, woraus etwas erstellt werden kann | Aktivieren Sie wieder etwas. |
| „Lizenzdaten in dem, was Sie ausgelassen haben, werden trotzdem aufgenommen“ | Lizenzen richten sich nur nach der Option **Lizenz- und Aktivierungsdaten** | Deaktivieren Sie diese Option, wenn Sie die Lizenzen nicht aufnehmen möchten. |

<a id="create-installer-stays-greyed-out"></a>

### „Installationsprogramm erstellen“ bleibt ausgegraut

Die Schaltfläche wartet, bis alle folgenden Bedingungen erfüllt sind:

- die Analyse der App ist für die aktuellen Optionen abgeschlossen („Es wird ermittelt, was dazugehört…“ bedeutet, dass noch ermittelt wird, was Ihre letzte Deaktivierung mitnimmt);
- der **Ausgabeordner** ist ein vollständiger Pfad, z. B. `D:\Installers` („Geben Sie einen vollständigen Ordnerpfad ein, z. B. D:\Installationsprogramme.“);
- auf diesem Laufwerk ist genügend Speicherplatz vorhanden.

<a id="room"></a>

### Speicherplatz

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| „Es ist nicht genügend Speicherplatz vorhanden.“ und wie viel ein Laufwerk benötigt und hat | Wählen Sie einen Ausgabeordner auf einem anderen Laufwerk aus, oder lassen Sie etwas weg. Die Angabe ist das Maximum, das eine Erstellung vor der Komprimierung belegen kann, daher kann sie größer sein als das fertige Installationsprogramm. |
| Ein Laufwerk „gibt nicht an, wie viel Speicherplatz frei ist“ | Die Erstellung wird fortgesetzt, ohne dieses Laufwerk zu prüfen. Stellen Sie sicher, dass dort genügend Speicherplatz vorhanden ist. |
| Ein FAT32-Laufwerk und eine große App | FAT32 kann keine Datei mit 4 GiB speichern. Verwenden Sie ein NTFS- oder exFAT-Laufwerk. |

<a id="the-done-screen"></a>

### Der Abschlussbildschirm

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| „Installationsprogramm erstellt, aber“ einige Dateien „waren seit der Analyse der App nicht mehr vorhanden“ | Klicken Sie auf **Erneut analysieren** und dann auf **Installationsprogramm erneut erstellen**, wenn diese Dateien wichtig sind. |
| „Installationsprogramm erstellt, aber“ einige Dateien „konnten nicht erfasst werden, meist weil sie gesperrt oder nicht lesbar waren.“ | Schließen Sie die App und alles andere, was ihre Dateien verwendet, und erstellen Sie dann erneut. **Warnungen** führt die Dateien auf. |
| „Installationsprogramm erstellt, aber es enthält noch Hinweise darauf, von wem es stammt.“ | Lesen Sie die gelben Meldungen darunter. Siehe [die letzte Prüfung](Building-an-Installer#the-last-look). |
| **Warnungen** meldet, dass eine Laufzeitkomponente „nicht heruntergeladen werden“ konnte, und „Das Installationsprogramm versucht es auf dem Zielcomputer mit winget.“ | Diese Laufzeitkomponente ist nicht im Installationsprogramm enthalten, daher benötigt der neue PC dafür Internet. Erstellen Sie das Installationsprogramm erneut, während Sie online sind. Siehe [Laufzeitkomponenten](Runtimes#at-build-time). |
| „Verwenden Sie dieses Installationsprogramm nicht: Es hat die Hashprüfung nicht bestanden.“ | Verwenden Sie es nicht. Erstellen Sie es erneut. Wenn das wieder passiert, fällt das Laufwerk möglicherweise aus. |
| „Das Installationsprogramm wurde geschrieben, aber seine Hashprüfung konnte nicht ausgeführt werden“ | Verwenden Sie es nicht. Erstellen Sie es erneut. |
| „Erstellung abgebrochen. Es wurde kein Installationsprogramm geschrieben.“ | Sie haben auf **Abbrechen** geklickt. |
| „Das Installationsprogramm konnte nicht erstellt werden.“ und ein Grund | Lesen Sie den Grund. **Fehler kopieren** kopiert ihn, und **Protokoll** zeigt, wo sich das Protokoll befindet. |
| „Der Ausgabeordner D:\Installers konnte nicht erstellt werden“ | Wählen Sie einen Ordner aus, in den Sie schreiben können. |
| „Es konnte kein geschützter Bereitstellungsordner erstellt werden, daher wurde nichts erstellt“ | Die Erstellung konnte ihren privaten Arbeitsordner weder neben dem Ausgabeordner noch im temporären Ordner von Windows anlegen. Wählen Sie einen Ausgabeordner auf einem anderen Laufwerk aus. |
| „Die Erstellung wurde unerwartet beendet“ | Etwas ist schiefgegangen, womit nichts gerechnet hat. „Falls dabei ein Installationsprogramm im Ausgabeordner zurückgeblieben ist, verwenden Sie es nicht.“ Bitte [melden Sie es](#getting-help). |

<a id="installing"></a>

## Installation

<a id="when-the-installer-opens"></a>

### Beim Öffnen des Installationsprogramms

| Was Sie sehen | Bedeutung | Was zu tun ist |
| --- | --- | --- |
| „Installationsprogramm wird überprüft…“ für lange Zeit | Es liest das gesamte Paket, um seinen Hash zu prüfen | Warten Sie. Ein großes Installationsprogramm braucht eine Weile. |
| „Das Installationsprogramm konnte nicht gelesen werden:“ und „dieses Installationsprogramm ist beschädigt oder wurde nach der Erstellung geändert, daher wurde nichts aus seinem Paket gelesen. Kopieren Sie es erneut von dem Ort, an dem es erstellt wurde.“ | Die Datei ist abgeschnitten oder geändert | Kopieren Sie sie erneut. |
| „dieses Installationsprogramm speichert sein Paket in einer Datei namens“ und der Name einer `.wamdata`-Datei, „im selben Ordner, und diese Datei ist nicht vorhanden“ | Einem Installationsprogramm aus zwei Dateien fehlt seine Datendatei | Kopieren Sie die `.wamdata`-Datei in denselben Ordner. Siehe [Große Apps](Big-Apps). |
| die Datendatei „ist nicht die Datendatei dieses Installationsprogramms oder ist beschädigt oder wurde nach der Erstellung geändert“ | Die Datendatei passt nicht | Kopieren Sie beide Dateien erneut von dem Ort, an dem sie erstellt wurden. |
| „Das Paket dieses Installationsprogramms kann auf diesem PC nicht verwendet werden:“ und „diese App kann auf diesem PC nicht ausgeführt werden, daher wurde nichts geändert“ | Die Programme der App sind für einen anderen Prozessor erstellt, oder ein Programm gibt an, ein neueres Windows zu benötigen | Installieren Sie auf einem PC wie dem alten. Es wurde nichts geändert. |
| „Dies ist ein Installationsprogramm-Stub von WinAppMigrator, der kein Anwendungspaket enthält.“ | Das ist der bloße Stub, kein erstelltes Installationsprogramm | Erstellen Sie mit der App ein Installationsprogramm, und führen Sie dieses aus. |
| „Die Signatur dieses Installationsprogramms ist ungültig“ mit der Begründung von Windows | Es wurde signiert, aber Windows akzeptiert die Signatur nicht. Möglicherweise wurde es seitdem geändert, oder seinem Zertifikat wird auf diesem PC nicht vertraut. | Installieren Sie es nicht. Fragen Sie bei der Person nach, die es erstellt hat. |
| Die Aufforderung der Benutzerkontensteuerung zeigt „Unbekannter Herausgeber“ | Das Installationsprogramm ist nicht signiert. Bei einem selbst erstellten ist das normal. | Führen Sie es nur aus, wenn Sie der Quelle vertrauen. |

<a id="the-install-location"></a>

### Der Installationsort

Wenn **Installieren** und **Änderungsvorschau** ausgegraut sind, nennt ein roter Hinweis unter **Installationsort** den Grund. Der häufigste Grund ist, dass die App auf diesem PC bereits in diesem Ordner installiert ist, wie es bei einem Webbrowser, einem Mediaplayer oder Office oft der Fall ist: Deinstallieren Sie zuerst diese Kopie in den Windows-Einstellungen unter **Apps**, und öffnen Sie das Installationsprogramm erneut, oder geben Sie einen neuen, leeren Ordner ein. Die Gründe und was zu tun ist, finden Sie unter [Installation auf dem neuen PC](Installing-on-the-New-PC#when-the-location-cannot-be-used).

<a id="before-or-during-the-install"></a>

### Vor oder während der Installation

| Was Sie sehen | Bedeutung | Was zu tun ist |
| --- | --- | --- |
| „Hier ist möglicherweise nicht genügend Speicherplatz vorhanden“ | Auf dem Laufwerk scheint der Speicherplatz knapp zu sein | Geben Sie Speicherplatz frei, oder wählen Sie einen Ort auf einem anderen Laufwerk aus. |
| „es ist nicht genügend Speicherplatz dafür vorhanden, daher wurde nichts geändert“ | Ein Laufwerk, auf das geschrieben wird, ist zu voll | Geben Sie dort Speicherplatz frei, oder wählen Sie einen Installationsort auf einem anderen Laufwerk aus. |
| „Was die Installation an diesem Ort tun würde, hat sich geändert, daher wurde nichts installiert.“ | Auf dem PC hat sich seit der Ermittlung der Liste etwas geändert | Lesen Sie die Liste erneut, und klicken Sie noch einmal auf **Installieren**, wenn sie Ihren Wünschen entspricht. |
| „Eine frühere Installation von“ einer App „wurde nie abgeschlossen. ‚Installieren‘ macht sie zuerst rückgängig.“ | Eine frühere Installation wurde unterbrochen | Klicken Sie auf **Installieren**. Die frühere Installation wird zuerst rückgängig gemacht. |
| „eine andere Installation oder Deinstallation wird ausgeführt. Warten Sie, bis sie abgeschlossen ist, und versuchen Sie es dann erneut“ | Es kann jeweils nur eine ausgeführt werden | Warten Sie, und versuchen Sie es dann erneut. |
| Eine Datei „hat einen Namen, den diese Windows-Version als Gerät und nicht als Datei öffnet“ | Die App hat eine Datei mit einem Namen wie `aux.h`, die Windows vor Windows 11 nicht speichern kann | Installieren Sie unter Windows 11. |
| „Es konnte kein Bereitstellungsordner erstellt werden“ | Es gibt keinen privaten Ort mit genügend Speicherplatz, um das Paket zu entpacken | Geben Sie Speicherplatz auf dem Laufwerk des Installationsorts oder auf dem Systemlaufwerk frei. |
| Ein Datenordner wie `%AppData%` wird als Link gemeldet | Das Installationsprogramm schreibt nicht über einen [Link](Glossary#link) | Deaktivieren Sie **Benutzerdaten** und **Lizenz**, und installieren Sie ohne sie. |
| „Der ProgramData-Ordner dieses PCs wurde nicht gefunden, daher wurde nichts installiert“ | Die eigene Angabe von Windows, wo sich ProgramData befindet, konnte nicht gelesen werden | Ohne einen sicheren Ort für seine Aufzeichnung installiert das Installationsprogramm nichts. |

<a id="the-result"></a>

### Das Ergebnis

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| „Die Installation ist fehlgeschlagen, und ihre Änderungen wurden rückgängig gemacht.“ | Den Grund finden Sie in den roten Meldungen. Der PC ist wie zuvor, mit Ausnahme installierter Laufzeitkomponenten. |
| „Die Installation ist fehlgeschlagen, und nicht alle Änderungen konnten rückgängig gemacht werden.“ | Lesen Sie, was aufgeführt ist. Schließen Sie, was diese Dateien verwenden könnte, und führen Sie erneut ein Installationsprogramm aus: Es versucht zuerst, die Änderungen rückgängig zu machen. |
| Eine Laufzeitkomponente „konnte nicht installiert werden“ | Die Laufzeitkomponente konnte nicht installiert werden, daher wurde die Installation rückgängig gemacht. Für die DirectX-Endbenutzer-Runtime benötigt der neue PC Internet. Siehe [Laufzeitkomponenten](Runtimes#with-no-internet-on-the-new-pc). |
| „Eine Abhängigkeit benötigt einen Neustart, um abgeschlossen zu werden. Starten Sie Windows neu, und starten Sie dann die App.“ | Starten Sie neu, und starten Sie dann die App. |
| Der Dienst „wurde auf dem Quell-PC als“ ein bestimmtes Konto „ausgeführt. Sein Kennwort ist nicht im Installationsprogramm enthalten, daher wird der Dienst als LocalService eingerichtet.“ | Öffnen Sie „Dienste“, und legen Sie Konto und Kennwort des Dienstes fest, falls er so nicht funktioniert. |
| Einige Dateien „werden stattdessen unter“ dem Installationsordner „in einem Ordner namens _external abgelegt“ | Dieser PC führt „Dokumente“ oder „Gespeicherte Spiele“ außerhalb Ihres Profils. Verschieben Sie diese Dateien selbst in diesen Ordner. |
| Etwas „wurde ausgelassen“, mit einem Grund | Die Regeln haben es abgelehnt, oder es gehört hier bereits zu einer anderen App. Der Rest ist installiert. Wenn die App es benötigt, lesen Sie den Grund. |
| **Paket-Hashes UNGÜLTIG** oder **Installierte Dateien UNGÜLTIG** | Etwas stimmte nicht überein. Kopieren Sie das Installationsprogramm erneut, und versuchen Sie es noch einmal. |

<a id="after-installing"></a>

### Nach der Installation

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| Die App startet nicht | Prüfen Sie unter **Abhängigkeiten**, ob eine Laufzeitkomponente mit **Manuell installieren** markiert ist, und installieren Sie sie. Prüfen Sie die Warnungen im Ergebnis. |
| Eine App von Windows 7 startet nicht, und eine Warnung bei der Erstellung nannte eine DLL, die „in Windows 10 und 11 nicht enthalten“ ist | Diese DLL wurde von etwas anderem in die eigenen Ordner des alten Windows gelegt, daher wurde sie nicht übernommen. Installieren Sie, was sie bereitstellt, und starten Sie die App dann erneut. Siehe [Apps von Windows 7, Vista, 8 oder 8.1](Moving-from-a-Connected-Drive#apps-from-windows-7-vista-8-or-81). |
| Die App startet wie neu | Das ist bei einem Installationsprogramm zu erwarten, das mit deaktivierter Option **Benutzerdaten** erstellt wurde. Es passiert auch, wenn **Benutzerdaten** bei der Installation deaktiviert war, weil die Daten dann unter dem Installationsordner abgelegt werden, und wenn das Installationsprogramm nicht ermitteln konnte, wer angemeldet war, was es meldet. |
| Die App fordert eine Aktivierung | Eine an den alten PC gebundene Lizenz muss erneut aktiviert werden. |
| Der eigene Updater der App meint, sie sei nicht installiert | Windows Installer hat hier keine Aufzeichnung von ihr. Siehe [Einschränkungen](Limitations). |
| Ein Dateityp oder Linktyp wird nicht mit der App geöffnet | Hier gehörte er bereits zu einer anderen App und blieb daher unangetastet. Wählen Sie die App in den Einstellungen unter „Standard-Apps“ aus. |
| Ein neuer `Path`-Eintrag wird nicht erkannt | Öffnen Sie eine neue PowerShell oder Eingabeaufforderung. Manche Programme erkennen eine Änderung erst, nachdem Sie sich ab- und wieder angemeldet haben. |

<a id="uninstalling"></a>

## Deinstallation

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| „Ein Teil davon befindet sich auf einem nicht angeschlossenen Laufwerk.“ | Schließen Sie dieses Laufwerk an, und deinstallieren Sie erneut über die Einstellungen. |
| „Nicht alles konnte rückgängig gemacht werden. Schließen Sie, was darauf zugreift, und deinstallieren Sie die App erneut über die Einstellungen.“ | Schließen Sie die App und alles, was ihre Dateien verwendet. Wenn ein Dienst nicht beendet wird, starten Sie den PC neu. Deinstallieren Sie dann erneut. |
| „this is not an install's id“ | Die an `--uninstall` übergebene ID ist falsch. Kopieren Sie sie aus dem Eintrag in den Einstellungen. Siehe [Befehlszeile](Command-Line#finding-the-id). |
| Eine Datei namens `.before-install` ist erschienen | Es ist das Original einer Datei, die die Installation ersetzt hat und die Sie seitdem geändert hatten. Behalten Sie die Version, die Sie möchten. |
| Eine Laufzeitkomponente ist noch installiert | Laufzeitkomponenten bleiben erhalten, da andere Programme sie möglicherweise verwenden. Lassen Sie sie installiert, es sei denn, Sie sind sicher, dass nichts anderes sie verwendet. |

<a id="scripts"></a>

## Skripts

| Was Sie sehen | Was zu tun ist |
| --- | --- |
| „Der angeforderte Vorgang erfordert erhöhte Rechte“ | Führen Sie PowerShell als Administrator aus. |
| Es wird nichts ausgegeben, und `$LASTEXITCODE` ist nicht festgelegt | Leiten Sie die Ausgabe des Installationsprogramms weiter, z. B. an `Out-String`. Siehe [Befehlszeile](Command-Line). |
| Exitcode `2` | Ein Parameter ist falsch, oder `--apply` und `--dry-run` wurden zusammen verwendet. |
| „this package does not say where its app was, so give a --target-root“ | Fügen Sie `--target-root` mit einem vollständigen Pfad hinzu. |
| „this package's app was in“ und ein Ordner, dann „and that cannot be used“ | Fügen Sie `--target-root` mit einem anderen Ordner hinzu. |
| Ein Ordner, dann „cannot be used: it already has files“ und „Give --force only if you know it is this app's“ | Wählen Sie einen leeren Ordner aus. Verwenden Sie `--force` nur für einen Ordner, von dem Sie wissen, dass er zu dieser App gehört. |

<a id="language-and-text"></a>

## Sprache und Text

- **Die falsche Sprache.** Wählen Sie unter **Sprache** oben rechts in der App oder auf dem ersten Bildschirm eines Installationsprogramms eine andere aus. Die App merkt sich die Wahl; ein Installationsprogramm behält sie für diese Ausführung bei. Ohne Auswahl spricht jedes Programm die erste Windows-Anzeigesprache, die es beherrscht, andernfalls Englisch.
- **Eine Übersetzung klingt falsch.** Alles außer Englisch wurde maschinell übersetzt. Wählen Sie unter **Sprache** die Option **Übersetzungsproblem melden** aus. Dadurch wird ein Formular geöffnet, in dem Ihre Sprache bereits eingetragen ist. Beschreiben Sie, was auf dem Bildschirm steht und was dort stehen sollte.
- **Buchstaben werden als leere Kästchen angezeigt.** Das Fenster verwendet für jede Schrift die Schriftarten, die mit Windows geliefert werden, aus `C:\Windows\Fonts`. Ein Windows, aus dem einige davon entfernt wurden, stellt diese Buchstaben als Kästchen dar; auch die Namen anderer Apps in der Liste können sie benötigen.
- **Wörter werden abgeschnitten.** Eine Beschriftung, die nicht passt, endet mit `…`. Zeigen Sie mit der Maus darauf, oder wechseln Sie mit der Tabulatortaste dorthin, um den vollständigen Text zu sehen. Melden Sie es mit Sprache und Bildschirm, damit das Fenster angepasst werden kann.
- **WinAppMigrator Client gibt auf einem Windows mit Rechts-nach-links-Sprache Englisch aus.** Die Konsole, in der er läuft, kann arabische oder hebräische Buchstaben weder verbinden noch richtig anordnen, daher gibt er Englisch aus und nennt den Grund. Mit `--lang` wählen Sie jede Sprache, die er beherrscht.

<a id="getting-help"></a>

## Hilfe erhalten

1. Suchen Sie das [Protokoll](Glossary#log). Sein Pfad wird bei einem Fehler als **Protokoll** angezeigt oder in der Befehlszeile als `Log:` ausgegeben. Protokolle befinden sich in `%ProgramData%\WinAppMigrator\logs`. Sie enthalten Pfade und Schlüsselnamen, aber nie den Inhalt einer Einstellung, einer Lizenzdatei oder einer Umgebungsvariable. Sie können dennoch Ihre Ordnernamen enthalten. Lesen Sie ein Protokoll daher, bevor Sie es weitergeben.
2. [Erstellen Sie ein Issue](https://github.com/RandyNorthrup/WinAppMigrator/issues) mit der Version, Ihren Schritten und dem, was auf dem Bildschirm stand. **Fehler kopieren** auf dem Abschlussbildschirm der App kopiert die Meldung.
3. **Hängen Sie nie ein Installationsprogramm an ein Issue an.** Es kann private Daten enthalten.
4. Melden Sie ein Sicherheitsproblem stattdessen vertraulich über die [Sicherheitshinweise von GitHub](https://github.com/RandyNorthrup/WinAppMigrator/security/advisories/new).
