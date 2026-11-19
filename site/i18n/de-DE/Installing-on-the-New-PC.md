<a id="installing-on-the-new-pc"></a>

# Installation auf dem neuen PC

Das Installationsprogramm besteht aus einem einzigen Fenster. Es überprüft sich selbst, zeigt Ihnen, wohin es installiert und was es alles einrichtet, und installiert erst, wenn Sie auf **Installieren** klicken. Diese Seite geht es von oben nach unten durch und verwendet dabei die Wörter auf dem Bildschirm.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-dark.png">
    <img alt="Das Installationsprogramm für Fabrikam Data Sync: nicht signiert, mit Installationsort, benötigtem Speicherplatz und dem, was es außer Dateien einrichtet" src="https://raw.githubusercontent.com/RandyNorthrup/WinAppMigrator/main/docs/images/installer-light.png" width="600">
  </picture>
</p>

<a id="before-you-start"></a>

## Bevor Sie beginnen

- **Kopieren Sie das ganze Installationsprogramm.** Wenn die Erstellung neben der `.exe` auch eine `.wamdata`-Datei ergeben hat, kopieren Sie beide in denselben Ordner, und behalten Sie ihre Namen bei. Siehe [Große Apps](Big-Apps).
- **Führen Sie nur Installationsprogramme aus, denen Sie vertrauen.** Ein Installationsprogramm wird mit vollen [Administratorrechten](Glossary#administrator-rights) ausgeführt. Behandeln Sie es wie jedes andere Programm: Führen Sie es nur aus, wenn Sie es selbst erstellt oder von jemandem erhalten haben, dem Sie vertrauen. Siehe [Datenschutz und Sicherheit](Privacy-and-Security).
- **Sie benötigen Administratorrechte** auf diesem PC und [Direct3D 12](Glossary#direct3d-12) für das Fenster. Ohne Direct3D 12 können Sie trotzdem über eine Eingabeaufforderung installieren. Siehe [Befehlszeile](Command-Line).

<a id="open-it"></a>

## Öffnen

Doppelklicken Sie auf das Installationsprogramm. Windows zeigt die [Aufforderung der Benutzerkontensteuerung](Glossary#uac-prompt) an. Ein Installationsprogramm ist nur signiert, wenn die Person, die es erstellt hat, es signiert hat. Daher zeigt die Aufforderung meist **Unbekannter Herausgeber** an. Das ist zu erwarten. Siehe [Datenschutz und Sicherheit](Privacy-and-Security#signing).

Das Fenster „WinAppMigrator-Installationsprogramm“, „Installiert eine Anwendung erneut, die auf einem anderen PC erfasst wurde“, öffnet sich sofort, ganz gleich, wie groß das Installationsprogramm ist. Es zeigt „Installationsprogramm wird überprüft…“, während es den [Hash](Glossary#hash) seines gesamten Pakets überprüft. Dabei wird das ganze Paket gelesen, daher dauert es bei einem großen Installationsprogramm eine Weile. Ab diesem Moment hält das Installationsprogramm seine eigene Datei geöffnet, und auch seine Datendatei, falls es eine hat, sodass keine der beiden geändert oder ausgetauscht werden kann, solange das Fenster geöffnet ist.

Die Überprüfung kann hier mit einer roten Meldung enden. Die häufigsten sind:

- Das Installationsprogramm ist beschädigt oder wurde nach der Erstellung geändert: Kopieren Sie es erneut von dem Ort, an dem es erstellt wurde.
- Seine Datendatei fehlt oder ist nicht die richtige: Kopieren Sie beide Dateien erneut in einen gemeinsamen Ordner.
- „Das Paket dieses Installationsprogramms kann auf diesem PC nicht verwendet werden“, z. B. weil die Programme der App für einen anderen Prozessortyp erstellt sind oder ein neueres Windows benötigen. Es wurde nichts geändert.

Alle diese Fälle werden unter [Problembehandlung](Troubleshooting#installing) erklärt.

Wenn die Überprüfung erfolgreich ist, wird der Fenstertitel zum Namen der App, gefolgt von „installieren“, z. B. „Fabrikam Data Sync installieren“.

<a id="its-language"></a>

### Seine Sprache

Ein Installationsprogramm spricht die Sprache des PCs, auf dem es ausgeführt wird, nicht die des PCs, auf dem es erstellt wurde: die Anzeigesprache von Windows, wenn sie zu den 24 Sprachen von WinAppMigrator gehört, andernfalls Englisch. **Sprache** oben rechts ändert sie für diese Ausführung. Was es über die App auflistet, einschließlich der Hinweise, die die App bei der Erstellung geschrieben hat, erscheint ebenfalls in dieser Sprache. Ein mit WinAppMigrator 0.6 oder älter erstelltes Installationsprogramm zeigt diese Hinweise auf Englisch an, so wie sie geschrieben wurden. Eine [unbeaufsichtigte Installation über die Befehlszeile](Command-Line) antwortet immer auf Englisch, damit Skripts die Ausgabe lesen können.

<a id="the-app-and-where-it-goes"></a>

## Die App und ihr Zielort

Die erste Karte enthält den Namen der App, darunter Herausgeber und Version, z. B. „Fabrikam Inc. · 1.7.3“. Dann folgen:

| Zeile | Was sie anzeigt |
| --- | --- |
| **Erfasst auf** | Der Name des PCs, von dem die App stammt. Wird nur angezeigt, wenn das Installationsprogramm mit aktivierter Option **Benutzerdaten** erstellt wurde. |
| **Installationsprogramm** | Ob diese Installationsprogrammdatei [signiert](Glossary#signature) ist: „Signatur wird überprüft…“, dann „Nicht signiert: Herausgeber unbekannt“ oder „Signiert von“ und der Name des Signierers. Bei einer Signatur, die der Prüfung nicht standhält, erscheint stattdessen eine rote Meldung: „Die Signatur dieses Installationsprogramms ist ungültig“ mit der Begründung von Windows und „Es wurde möglicherweise nach dem Signieren geändert.“ |
| **Installationsort** | Der Ordner, in den die App kommt, mit **Durchsuchen** zur Auswahl. Anfangs ist das der Ordner, in dem sich die App auf dem alten PC befand. Eine App von einem 32-Bit-Windows beginnt stattdessen in `Program Files (x86)`, wo ein 64-Bit-Windows 32-Bit-Apps ablegt. Kann dieser Ordner hier nicht verwendet werden, nennt ein roter Hinweis darunter den Grund. |
| **Benötigter Speicherplatz** | Wie viel Speicherplatz die Installation auf diesem Laufwerk benötigt und wie viel frei ist, z. B. „bis zu 824,0 MB auf C: (356,0 GB frei)“. Berücksichtigt werden das entpackte Paket und die Dateien der App. |

Der Herausgebername in der ersten Zeile stammt aus dem Paket und könnte daher von jedem geschrieben worden sein. Die Zeile **Installationsprogramm** stammt von Windows, und ihr können Sie vertrauen.

<a id="when-the-location-cannot-be-used"></a>

### Wenn der Installationsort nicht verwendet werden kann

Wenn der Ordner unter **Installationsort** nicht verwendet werden kann, nennt ein roter Hinweis darunter den Grund. **Installieren** und **Änderungsvorschau** bleiben ausgegraut, bis Sie einen anderen auswählen, und eine Zeile daneben lautet: „‚Installieren‘ und ‚Änderungsvorschau‘ sind deaktiviert, bis der Installationsort verwendet werden kann.“

Der häufigste Grund ist, dass der Ordner bereits Dateien enthält. Ein Ordner mit Dateien wird nur verwendet, wenn eine frühere Installation durch WinAppMigrator sie dort abgelegt hat, damit nie die Dateien einer anderen App ersetzt werden:

| Hinweis | Bedeutung | Was zu tun ist |
| --- | --- | --- |
| Der Name der App, dann „ist bereits in diesem Ordner installiert, daher kann diese Kopie nicht hier installiert werden.“ | Die App ist auf diesem PC bereits installiert, oft durch ihr eigenes Installationsprogramm: Ihr Hauptprogramm befindet sich in diesem Ordner, oder die Windows-Liste der installierten Apps nennt sie dort. Das kommt bei Apps vor, die viele PCs bereits haben, z. B. einem Webbrowser, einem Mediaplayer oder Office. | Deinstallieren Sie zuerst die Kopie auf diesem PC in den Windows-Einstellungen unter **Apps**, und öffnen Sie das Installationsprogramm dann erneut. Oder geben Sie unter **Installationsort** einen neuen, leeren Ordner ein, um diese Kopie daneben zu installieren. |
| Der Name einer anderen App, dann „ist in diesem Ordner installiert“ und dass diese App „nicht hier installiert werden“ kann. | Die Windows-Liste der installierten Apps nennt eine andere App in diesem Ordner, z. B. eine andere Office-Edition. | Geben Sie unter **Installationsort** einen neuen, leeren Ordner ein. Oder deinstallieren Sie die andere App zuerst in den Windows-Einstellungen unter **Apps**, falls Sie sie nicht mehr benötigen, und öffnen Sie das Installationsprogramm dann erneut. |
| „Dieser Ordner enthält bereits Dateien, die nicht von WinAppMigrator installiert wurden.“ | Etwas anderes befindet sich in dem Ordner, und dort ist keine installierte App verzeichnet. Es könnte zu einer anderen App gehören. | Geben Sie unter **Installationsort** einen neuen, leeren Ordner ein. Wenn die Dateien von einer App übrig geblieben sind, die Sie deinstalliert haben, können Sie den Ordner löschen und das Installationsprogramm dann erneut öffnen. |

Bei jedem anderen Grund lautet der Hinweis „Dieser Ordner kann nicht verwendet werden:“, dann folgen der Grund und „Geben Sie oben einen anderen Ordner ein.“ Die Gründe sind:

| Grund auf dem Bildschirm | Was zu tun ist |
| --- | --- |
| „er muss ein vollständiger Pfad sein“ | Geben Sie einen Pfad ein, der mit einem Laufwerkbuchstaben beginnt, z. B. `C:\Program Files\Fabrikam Data Sync`. |
| „er kann kein ganzes Laufwerk sein“ | Verwenden Sie einen Ordner auf dem Laufwerk, nicht das Laufwerk selbst. |
| „er kann nicht der Windows-Ordner sein“ | Wählen Sie einen Ordner außerhalb von `C:\Windows` aus. |
| „er enthält andere Dinge nebeneinander, daher benötigt die App einen eigenen Ordner darin“ | Fügen Sie einen Ordner für die App hinzu, z. B. `C:\Program Files\Fabrikam Data Sync` statt `C:\Program Files`. |
| „er gehört zu Windows, oder sein Inhalt wird automatisch ausgeführt oder geladen“ | Wählen Sie einen gewöhnlichen Ordner aus. Ein Autostart-Ordner wird beispielsweise abgelehnt. |
| „er muss ein einfacher Pfad auf einem Laufwerk dieses PCs sein“ | Eine [Netzwerkfreigabe](Glossary#network-share) wird abgelehnt, ebenso ein Name, den Windows als etwas anderes lesen würde. |
| dass er „ein Link an einen anderen Ort“ ist, „und andere können möglicherweise ändern, wohin er führt“ | Verwenden Sie den Ordner, zu dem der [Link](Glossary#link) führt. |
| „an diesem Ort legen Installationen ihre Aufzeichnungen ab“ | Wählen Sie einen Ordner außerhalb von `%ProgramData%\WinAppMigrator` aus. |

Wenn der Ordner Dateien enthält und dort eine frühere Installation dieses Projekts **verzeichnet** ist, verwendet das Installationsprogramm ihn und warnt: „Dieser Ordner enthält bereits Dateien. Dateien mit gleichem Namen werden ersetzt und bei einer Deinstallation wiederhergestellt.“

Sobald ein Ordner akzeptiert ist, ersetzt das Fenster eine Datei, einen Registrierungswert, einen Dienst, eine Aufgabe, eine Verknüpfung oder eine Variable mit gleichem Namen, so wie es `--force` in der [Befehlszeile](Command-Line#what---force-does) tut. Was bereits einer anderen App gehört, z. B. ihr Dienst oder Dateityp, bleibt weiterhin bei dieser App.

Wenn auf dem Laufwerk der Speicherplatz knapp zu sein scheint, zeigt ein gelber Hinweis „Hier ist möglicherweise nicht genügend Speicherplatz vorhanden“ und was benötigt wird. Das Paket kann auf einem anderen Laufwerk entpackt werden, aber die Dateien der App müssen trotzdem auf dieses passen. Die Installation selbst prüft den Speicherplatz noch einmal, bevor sie beginnt.

<a id="an-earlier-install-that-never-finished"></a>

### Eine frühere Installation, die nie abgeschlossen wurde

Wenn eine frühere Installation durch eines dieser Installationsprogramme unterbrochen wurde, etwa durch einen Absturz oder einen Stromausfall, zeigt ein gelber Hinweis: „Eine frühere Installation von“ dieser App „wurde nie abgeschlossen. ‚Installieren‘ macht sie zuerst rückgängig.“ Das geschieht automatisch, wenn Sie auf **Installieren** klicken.

<a id="what-this-sets-up-beyond-files"></a>

## Was hierbei außer Dateien eingerichtet wird

Diese Karte ist die Liste, der Sie zustimmen, wenn Sie auf **Installieren** klicken. Unter ihrem Titel steht:

> All dies geschieht mit Administratorrechten. Wenn die Installation fehlschlägt, werden ihre Änderungen rückgängig gemacht, und Sie werden über alles informiert, was sich nicht rückgängig machen ließ. Sie können die Installation auch später in den Einstellungen unter „Installierte Apps“ rückgängig machen. Ersetzte Dateien werden dabei wiederhergestellt, installierte Laufzeitkomponenten bleiben erhalten.

Jede Zeile hat einen Namen, rechts eine Beschriftung, die angibt, um welche Art von Element es sich handelt, und eine Zeile, die beschreibt, was es tut. Eine Beschriftung im üblichen Blau ist gewöhnlich. Eine Beschriftung in **Gelb** kennzeichnet etwas, das mit vollen Rechten ausgeführt wird, z. B. einen Dienst, der als LocalSystem ausgeführt wird, einen Treiber oder eine Firewallregel, die eingehende Verbindungen zulässt. Sehen Sie sich diese Zeilen genau an.

Wenn die Liste leer ist, steht dort „Nichts: Es werden nur Dateien kopiert sowie Einstellungen in den eigenen Registrierungsschlüsseln der App.“

Die Liste wird für den Ort im Feld ermittelt. Wenn Sie den Ort ändern, steht dort kurz „Es wird ermittelt, was hierbei an diesem Ort eingerichtet würde…“, und **Installieren** wartet.

<a id="what-each-label-means"></a>

### Was die Beschriftungen bedeuten

| Beschriftung | Was es ist | Beispiel für die Zeile darunter |
| --- | --- | --- |
| **Dienst** | Ein [Dienst](Glossary#service): ein Programm, das Windows im Hintergrund ausführt | „Führt FabrikamSync.exe als NT AUTHORITY\LocalService aus, startet jetzt und bei jedem Start dieses PCs“. Andere Zeilen lauten „startet jetzt und kurz nach jedem Start dieses PCs“, „startet nur, wenn er angefordert wird“ oder „ist deaktiviert“. |
| **Treiber** | Ein [Treiber](Glossary#driver), der zur App gehört | „Lädt“ eine Datei „in Windows selbst“. Immer gelb. Ein Treiber, der mit Windows startet, wird direkt nach der Installation gestartet. Der Treiber von VeraCrypt wurde auf diese Weise auf einem Test-PC von Hand geladen; die CI lädt keinen. |
| **Geplante Aufgabe** | Eine [geplante Aufgabe](Glossary#scheduled-task) | „Ausgeführt als: SYSTEM“ oder als ein Konto. „mit höchsten Privilegien“ ist gelb markiert. Eine Aufgabe, die auf dem alten PC als eine Person ausgeführt wurde, wird als die Person ausgeführt, die sie installiert. |
| **Autostart** | Ein [Autostart-Element](Glossary#startup-item) | „Wird bei Ihrer Anmeldung ausgeführt“, „Wird bei der Anmeldung jedes Benutzers ausgeführt“ oder „Eine Verknüpfung im Autostart-Ordner, für Sie“ |
| **Verknüpfung** | Eine [Verknüpfung](Glossary#shortcut) auf dem Desktop oder im Startmenü | „Öffnet Fabrikam.exe, für alle auf diesem PC“ oder „für Sie“ |
| **PATH** | Ein Eintrag, der der [Path-Variablen](Glossary#path-variable) hinzugefügt wird | „Ergänzt Path“ und „Der Installationsordner\bin“. Er wird zum Vorhandenen hinzugefügt und ersetzt es nie. |
| **Umgebungsvariable** | Eine [Umgebungsvariable](Glossary#environment-variable) | „Wird auf“ ihren Wert „festgelegt“ |
| **Shellerweiterung** | Code, der in den Datei-Explorer geladen wird | „Lädt“ eine Datei „in den Explorer, für alle auf diesem PC“. Gelb. |
| **Menübefehl** | Ein Befehl im Kontextmenü | „Fügt dem Menü von“ einer Dateiart „einen Befehl hinzu“. „Ändert, was beim Öffnen dieser Elemente geschieht“ ist gelb. |
| **Linktyp** | Ein [Linkprotokoll](Glossary#link-protocol), z. B. `fabrikam:` | „Links dieses Typs werden mit der App geöffnet“ |
| **Dateityp** | Ein [Dateityp](Glossary#file-type), z. B. `.fds` | „Dateien dieses Typs werden mit der App geöffnet“ |
| **COM** | [COM-Klassen](Glossary#com-class), die andere Programme starten können | „COM in“ einer Datei, mit der Anzahl der Einstellungen. „COM-Berechtigungen“ ist gelb. |
| **Firewallregel** | Eine [Firewallregel](Glossary#firewall-rule) für eines ihrer Programme | „Erlaubt eingehende Verbindungen an Port 8080 für FabrikamSync.exe, in allen Netzwerken“. Das Zulassen eingehender Verbindungen ist gelb. |
| **Ereignisprotokollquelle** | Ein Name, unter dem die App in die Ereignisprotokolle von Windows schreibt | „Kann in das Protokoll ‚Application‘ schreiben, mit Meldungen aus eigenen Dateien“ |
| **Schriftart** | Eine Schriftart im Installationsordner | „Wird bei der Anmeldung aus dem Installationsordner geladen“ |
| **ODBC-Treiber** | Ein Datenbanktreiber für [ODBC](Glossary#odbc) | „Wird in Programme geladen, die über ODBC auf eine Datenbank zugreifen“. Gelb. |
| **ODBC-Datenquelle** | Eine gespeicherte Datenbankverbindung | „Eine Datenbankverbindung, für alle auf diesem PC“ |
| **Kompatibilitätseinstellung** | Wie Windows einige ihrer Programme startet | „Wie die Programme starten“ |
| **Standard-Apps** | Ein Eintrag, mit dem Sie die App unter „Standard-Apps“ auswählen können | „Kann unter ‚Standard-Apps‘ ausgewählt werden. Es wird nichts darauf umgestellt“ |
| **Programmname** | Ein Name, unter dem „Ausführen“ und andere Programme die App finden können | „‚Ausführen‘ und andere Programme können es unter diesem Namen finden“ |
| **Weitere Einstellungen** | [Registrierungswerte](Glossary#registry) außerhalb der eigenen Schlüssel der App und der obigen Arten | „3 Einstellungen außerhalb der eigenen Schlüssel der App“, „In“ den Schlüsseln. Gelb, wenn welche in den Schlüsseln von Microsoft liegen. |

Manches lässt sich erst beurteilen, wenn die Installation dort ankommt, z. B. ein gleichnamiger Dienst, der bereits zu einer anderen App gehört. Solche Elemente bleiben wie angefordert auf der Liste. Wenn die Installation dort ankommt, lässt sie sie unangetastet und meldet das.

<a id="notes-and-what-is-left-out"></a>

### Hinweise und Ausgelassenes

Unter den Zeilen stehen gelbe Hinweise zu dem, was Sie wissen sollten, zum Beispiel:

- dass Anmeldeinformationen nicht in das Installationsprogramm aufgenommen wurden, sodass Sie sie möglicherweise erneut eingeben müssen;
- dass die Option **Lizenz** oder **Benutzerdaten** bei der Erstellung deaktiviert war, sodass diese Daten nicht enthalten sind;
- auf welche [Zertifikate](Glossary#certificate) die App angewiesen zu sein scheint; diese werden nie installiert;
- was die Person, die das Installationsprogramm erstellt hat, ausgelassen und was sie hinzugefügt hat;
- dass die App hier möglicherweise nicht funktioniert: „Diese App befand sich auf Windows-Build“ eines neueren Windows, oder Windows würde einige ihrer Programme „über einen Emulator ausführen“;
- dass Dateien aus „Dokumente“ oder „Gespeicherte Spiele“ „stattdessen unter“ dem Installationsordner „in einem Ordner namens _external abgelegt“ werden, weil sich der entsprechende Ordner dieses PCs außerhalb Ihres Profils befindet.

Danach folgt in Gelb jedes Element, das die Sicherheitsregeln auslassen, mit dem Grund. Die ersten 20 werden vollständig angezeigt. Der Rest steht unter einer Zeile wie „25 weitere Elemente wurden ausgelassen“, die Sie öffnen können. Die übrige Installation wird ohne sie fortgesetzt. Was abgelehnt wird und warum, steht unter [Was in ein Installationsprogramm kommt](What-Goes-in-an-Installer#what-the-installer-refuses).

Wenn das Paket überhaupt nicht an den gewählten Ort installiert werden kann, nennt eine rote Meldung „Dieses Paket kann nicht an diesem Ort installiert werden“ und den Grund.

<a id="options"></a>

## Optionen

Diese Karte wird nur angezeigt, wenn das Installationsprogramm Benutzerdaten oder Lizenzdaten enthält. Jedes Kontrollkästchen ist anfangs aktiviert.

| Option | Was sie anzeigt |
| --- | --- |
| **Benutzerdaten** | „Ein: Einstellungen, Dokumente und Gespeicherte Spiele kommen in die üblichen Ordner des angemeldeten Benutzers. Aus: Alles wird unter dem Installationsordner abgelegt, und nichts davon wird so eingerichtet, dass es von dort ausgeführt wird.“ Wenn das Installationsprogramm angibt, wessen Daten es enthält, steht darunter „Dies sind die Daten von“ und das Konto. |
| **Lizenz** | „Aktivierungs- und Lizenzdateien sowie die Registrierungswerte, in denen Lizenzen gespeichert sind. Einige müssen hier möglicherweise erneut aktiviert werden.“ |

Wenn Sie ein Kontrollkästchen ändern, wird die Liste oben sofort entsprechend angepasst.

Benutzerdaten gehen immer an **den Benutzer, der bei Windows angemeldet ist**, auch wenn bei der Aufforderung der Benutzerkontensteuerung der Name eines anderen Administrators eingegeben wurde. Wenn das Installationsprogramm nicht ermitteln kann, wer das ist, meldet es das, lässt benutzerspezifische Einstellungen aus und legt Benutzerdaten unter dem Installationsordner ab, statt zu raten. Wenn in dieser Sitzung niemand angemeldet ist, gehen Benutzerdaten und benutzerspezifische Einstellungen an das Konto, unter dem das Installationsprogramm ausgeführt wird, und es meldet auch das.

<a id="dependencies"></a>

## Abhängigkeiten

„Was diese App benötigt und wie es jeweils bereitgestellt wird.“ Jede [Laufzeitkomponente](Glossary#runtime) hat eine Beschriftung:

| Beschriftung | Bedeutung |
| --- | --- |
| **Enthalten** | Im Installationsprogramm enthalten. Wird ohne Internet installiert, außer der DirectX-Endbenutzer-Runtime, deren Installationsprogramm sie von Microsoft abruft. Davon wurde in den Tests des Projekts nur die x64-Laufzeitkomponente von Visual C++ tatsächlich installiert. |
| **Wird mit winget installiert:** und eine ID | Sie konnte beim Erstellen des Installationsprogramms nicht heruntergeladen werden, daher verwendet das Installationsprogramm [winget](Glossary#winget), das Internet benötigt. Dieser Weg wurde in den Tests des Projekts nicht ausgeführt. |
| **Windows-Feature: NetFx3** | .NET Framework 3.5, aktiviert als [Windows-Feature](Glossary#windows-feature). Dafür kann Internet nötig sein. Dieser Weg wurde in den Tests des Projekts nicht ausgeführt. |
| **In Windows integriert** | .NET Framework 4, das Windows 10 und 11 bereits haben |
| **Wird mit der App geliefert** | Die App bringt ihre eigene Kopie mit |
| **Manuell installieren** | Hier gibt es kein Installationsprogramm dafür. Installieren Sie sie selbst. |

Wenn es keine gibt, steht dort „Es wurden keine zusätzlichen Laufzeitabhängigkeiten erkannt.“ Siehe [Laufzeitkomponenten](Runtimes).

<a id="preview-changes"></a>

## Änderungsvorschau

**Änderungsvorschau** ermittelt genau, was eine Installation tun würde, und ändert nichts. Dabei wird geprüft, ob genügend Speicherplatz vorhanden ist, das Paket wird in einen privaten Ordner entpackt, die Änderungen werden ermittelt, und dieser Ordner wird wieder entfernt. Die Ergebniskarte trägt die Überschrift „Vorschau: Noch wurde nichts geändert“, mit **Geplante Änderungen** und einer Liste der Änderungen, Dienste, Aufgaben und Verknüpfungen zuerst. Das Fenster listet die ersten 300 auf und gibt dann an, wie viele weitere es gibt.

Eine Vorschau prüft nicht den Hash jeder einzelnen Datei. Der Hash des gesamten Pakets wurde beim Öffnen des Fensters geprüft.

<a id="install"></a>

## Installieren

**Installieren** bleibt ausgegraut, bis der Ort verwendet werden kann und die Liste für diesen Ort gilt. Wenn das Paket überhaupt nicht an diesen Ort installiert werden kann, steht daneben „‚Installieren‘ ist deaktiviert, da dieses Paket nicht an diesem Ort installiert werden kann“, und der Grund steht in Rot unter **Was hierbei außer Dateien eingerichtet wird**. Wenn Sie darauf klicken:

1. Die Liste wird noch einmal ermittelt. Wenn sich seit Ihrer Prüfung geändert hat, was die Installation tun würde, wird nichts installiert, die neue Liste wird angezeigt, und es erscheint die Meldung: „Was die Installation an diesem Ort tun würde, hat sich geändert, daher wurde nichts installiert. Prüfen Sie die Liste unten, und klicken Sie erneut auf ‚Installieren‘, wenn sie Ihren Wünschen entspricht.“
2. Es wird geprüft, ob auf jedem Laufwerk, auf das geschrieben wird, genügend Speicherplatz vorhanden ist.
3. Das Paket wird in einen Ordner entpackt, auf den nur Administratoren zugreifen können, und dabei wird der Hash jeder Datei geprüft.
4. Die App wird installiert, wobei jede Änderung zuerst in einem [Journal](Glossary#journal) festgehalten wird, damit sie rückgängig gemacht werden kann.
5. Jede installierte Datei wird zurückgelesen und ihr Hash geprüft.
6. Die Dienste, die von selbst starten sollen, werden gestartet.

Unten sehen Sie, was gerade geschieht, gegebenenfalls einen Zähler, die bisher benötigte Zeit, eine Statusanzeige und **Abbrechen**. Mögliche Schritte sind „Wird vorbereitet…“, „Paket wird entpackt“, „Paket wird überprüft“, „Frühere, nie abgeschlossene Installation wird rückgängig gemacht“, „Laufzeitkomponenten werden installiert“, „Dateien werden kopiert“, „Dienste werden eingerichtet“, „Treiber werden eingerichtet“, „Registrierungseinstellungen werden geschrieben“, „Lizenzdaten werden wiederhergestellt“, „Geplante Aufgaben werden eingerichtet“, „Verknüpfungen werden erstellt“, „Umgebungsvariablen werden festgelegt“, „App wird zu den Einstellungen hinzugefügt“ und „Installation wird überprüft“.

**Abbrechen** zeigt „Wird nach dem aktuellen Schritt abgebrochen…“ und dann „Änderungen werden rückgängig gemacht“. Das Schließen des Fensters während der Installation hat dieselbe Wirkung wie **Abbrechen**: Das Fenster bleibt geöffnet, bis alles rückgängig gemacht ist. Wurde noch nichts geändert, steht dort „Abgebrochen. Es wurde nichts geändert.“

<a id="the-hash-check"></a>

## Die Hashprüfung

Das Installationsprogramm prüft [Hashes](Glossary#hash) dreimal:

1. **Beim Öffnen des Fensters**: den Hash des gesamten Pakets. Eine abgeschnittene Kopie oder eine, die nach der Erstellung bearbeitet wurde, wird abgelehnt, bevor etwas daraus gelesen wird.
2. **Beim Entpacken**: den Hash jeder Datei, bevor etwas an ihren Platz geschrieben wird.
3. **Nach der Installation**: Es liest jede installierte Datei zurück und prüft sie erneut.

Hashes erkennen Beschädigungen und Änderungen durch alle, die nicht auch die Hashes anpassen. Sie beweisen nicht, wer ein Installationsprogramm erstellt hat. Das kann nur eine [Signatur](Glossary#signature).

<a id="the-result"></a>

## Das Ergebnis

Die Ergebniskarte erscheint oben mit der Überschrift „Installationsergebnis“.

| Meldung | Bedeutung |
| --- | --- |
| „Die Anwendung wurde installiert und hat die Hashprüfung bestanden.“ | Alles ist in Ordnung. Wenn die Regeln etwas ausgelassen haben, lautet sie stattdessen „Die Anwendung wurde ohne die unten als ausgelassen aufgeführten Elemente installiert und hat die Hashprüfung bestanden.“ |
| „Die Installation wurde abgebrochen, und ihre Änderungen wurden rückgängig gemacht.“ | Sie haben abgebrochen. |
| „Die Installation ist fehlgeschlagen, und ihre Änderungen wurden rückgängig gemacht.“ | Etwas ist fehlgeschlagen. Die Änderungen wurden rückgängig gemacht, mit Ausnahme installierter Laufzeitkomponenten. Alles, was darunter in Gelb aufgeführt ist, wurde belassen, mit Begründung, z. B. ein Dienst, der erst beim Neustart des PCs entfernt wird. |
| „Die Installation ist fehlgeschlagen, und nicht alle Änderungen konnten rückgängig gemacht werden. Was übrig ist, wird unten aufgeführt. Die nächste Installation versucht es erneut.“ | Lesen Sie die Liste. Wenn Sie erneut ein Installationsprogramm ausführen, wird erneut versucht, diese Änderungen rückgängig zu machen. |
| „Die Installation wurde nicht sauber abgeschlossen. Prüfen Sie die Probleme unten.“ | Lesen Sie die Meldungen darunter. |

Darauf folgen drei Kacheln: **Vorgenommene Änderungen** (bzw. **Rückgängig gemachte Änderungen** nach einem Fehler), **Warnungen** und **Fehler**. Beschriftungen lauten **Paket-Hashes gültig** und **Installierte Dateien gültig** oder in Rot **UNGÜLTIG**. Bis zu fünf Warnungen und alle Fehler werden als Meldungen angezeigt. „Eine Abhängigkeit benötigt einen Neustart, um abgeschlossen zu werden. Starten Sie Windows neu, und starten Sie dann die App.“ bedeutet, dass eine Laufzeitkomponente einen Neustart angefordert hat.

**Änderungen** mit einer Anzahl öffnet die Liste der durchgeführten Änderungen, bis zu 300. Wenn die Installation nicht sauber abgeschlossen wurde, wird der Pfad ihres [Protokolls](Glossary#log) als **Protokoll** angezeigt.

Nach einer sauberen Installation werden die Schaltflächen unten zu **Schließen** und **Installationsordner öffnen**.

<a id="where-things-end-up"></a>

## Wo alles landet

- **Programmdateien** kommen an den Installationsort.
- **Benutzerdaten** kommen bei aktiviertem Kontrollkästchen in die eigenen Ordner AppData, Dokumente und Gespeicherte Spiele des angemeldeten Kontos sowie nach ProgramData. In einen Ordner „Dokumente“ oder „Gespeicherte Spiele“, den dieser PC außerhalb Ihres Profils führt, wird nicht geschrieben. Diese Dateien kommen unter den Installationsort, in `_external`, und die Liste weist zuerst darauf hin. Bei deaktiviertem Kontrollkästchen kommen alle Benutzerdaten unter den Installationsort.
- **Verknüpfungen** kommen auf den Desktop und in das Startmenü dieses PCs: in Ihre eigenen oder in die für alle Benutzer, wenn sie sich dort befanden.
- **Path** erhält nur die Einträge der App. Sonst ändert sich daran nichts.
- **Was hier bereits zu einer anderen App gehört, bleibt bei dieser App.** Ein Dateityp, ein Linktyp, eine COM-Klasse, ein Dienst, eine Ereignisprotokollquelle, eine Schriftart oder Ähnliches, das bereits zu etwas anderem gehört, bleibt unangetastet, und das Installationsprogramm meldet das. Eine App, die z. B. `http`-Links öffnen sollte, muss in den Einstellungen erneut als Standard festgelegt werden.
- **Einstellungen** erhält einen Eintrag für die App, sodass Sie sie wie jede andere deinstallieren können. Siehe [Installation rückgängig machen](Undoing-an-Install).

<a id="common-reasons-it-refuses"></a>

## Häufige Gründe für eine Ablehnung

| Was Sie sehen | Warum | Was zu tun ist |
| --- | --- | --- |
| „diese App kann auf diesem PC nicht ausgeführt werden, daher wurde nichts geändert“ | Ihre Programme sind für einen anderen Prozessortyp erstellt oder benötigen ein neueres Windows | Installieren Sie auf einem PC, der dem alten entspricht |
| „es ist nicht genügend Speicherplatz dafür vorhanden, daher wurde nichts geändert“ | Auf einem Laufwerk, auf das geschrieben wird, ist nicht genügend Speicherplatz frei | Geben Sie Speicherplatz frei, oder wählen Sie einen Ort auf einem anderen Laufwerk aus |
| „eine andere Installation oder Deinstallation wird ausgeführt“ | Es kann jeweils nur eine ausgeführt werden | Warten Sie, bis sie abgeschlossen ist, und versuchen Sie es dann erneut |
| Eine Datei „hat einen Namen, den diese Windows-Version als Gerät und nicht als Datei öffnet“ | Die App hat eine Datei mit einem Namen wie `aux.h`, die Windows vor Windows 11 nicht speichern kann | Installieren Sie unter Windows 11 |
| Ein Datenordner wie `%AppData%` liegt unter einem Link | Das Installationsprogramm schreibt nicht über einen Link | Deaktivieren Sie **Benutzerdaten** und **Lizenz**, und installieren Sie ohne sie |

Weitere Gründe finden Sie unter [Problembehandlung](Troubleshooting#installing).
