# Words of the app, WinAppMigrator itself: its one window, with the installed apps listed on the left and, on the
# right, what goes in an installer of the one picked and the button that builds it; and the steps to read another PC
# through WinAppMigrator Client. Message names start with app-.


## The window and its header

# The program's name, in the window's title bar and at the top of the window. Keep it as it is unless your language
# writes product names in a script of its own.
app-name = WinAppMigrator
app-tagline = Eine installierte App in ein selbstextrahierendes Installationsprogramm umwandeln
app-theme-to-light = Zu hellem Design wechseln
app-theme-to-dark = Zu dunklem Design wechseln
app-reload-list = Anwendungsliste neu laden
app-list-loading = Wird geladen
# { $count } is how many apps are listed.
app-list-count =
    { $count ->
        [one] { $count } App
       *[other] { $count } Apps
    }
app-list-load-failed = Laden fehlgeschlagen
# Shown when the window cannot open at all. { $error } is what went wrong, in the words of Windows or a graphics driver.
app-window-could-not-open = Das Fenster konnte nicht geöffnet werden: { $error }

## The list of installed apps

app-list-search = Nach Name oder Herausgeber suchen  (Strg+F)
app-list-filter-all = Alle
# Whether an app is installed for one account or for everyone on the PC.
app-scope-per-user = Pro Benutzer
app-scope-per-machine = Pro Computer
app-scope-unknown = Unbekannt
# An app whose processor architecture (x86, x64, ARM64) is not known.
app-arch-unknown = Unbekannt
app-list-loading-apps = Installierte Anwendungen werden geladen…
# { $error } is why the apps could not be listed.
app-list-could-not-list = Anwendungen konnten nicht aufgelistet werden: { $error }
app-list-no-matches = Keine passenden Anwendungen
app-list-no-matches-hint = Passen Sie den Suchtext oder den Bereichsfilter an.
app-list-worker-disconnected = Die Verbindung zum Inventarisierungsprozess wurde getrennt.
# What a screen reader says of an app's row. { $name } is the app's name, { $publisher } who made it, and { $version } its
# version, such as 11.4.2, or — when either is not known.
app-list-row = { $name }, { $publisher }, Version { $version }

## The list's columns, and what a screen reader says each column's heading does when clicked

app-column-name = Name
app-column-name-sort = Nach Name sortieren
app-column-publisher = Herausgeber
app-column-publisher-sort = Nach Herausgeber sortieren
app-column-version = Version
app-column-version-sort = Nach Version sortieren
app-column-scope = Bereich
app-column-scope-sort = Nach Bereich sortieren
# Short for architecture: x86, x64, or ARM64. The column is narrow.
app-column-arch = Arch.
app-column-arch-sort = Nach Architektur sortieren

## Where apps are read from: this PC, a drive from another PC, a disk image of one, or another PC on the network

app-source = Quelle
app-source-this-pc = Dieser PC
app-source-drive = Angeschlossenes Laufwerk…
app-source-image = Datenträgerabbild…
app-source-network = Netzwerk-PC…
app-source-attaching-image = Abbild wird schreibgeschützt angefügt…
# { $computer } is the other PC's name, such as OLD-PC, and { $address } its network address, such as 192.168.1.20.
app-source-network-pc = { $computer } über das Netzwerk ({ $address })
app-source-disconnect = Trennen
app-source-change = Ändern…
# The title of the window that picks a disk image. .vhd and .vhdx are file extensions.
app-source-pick-image-title = Datenträgerabbild des anderen PCs auswählen (.vhd oder .vhdx)
app-source-pick-image-filter = Datenträgerabbilder
# { $path } is the disk image file.
app-source-image-has-no-windows = { $path } enthält kein Volume mit Windows.
app-source-image-not-attached = Das Datenträgerabbild konnte nicht angefügt werden.
app-source-pick-drive-title = Stammverzeichnis des angeschlossenen Laufwerks auswählen (z. B. E:\)
# { $path } is the folder picked, such as E:\. \Windows and \Users are folder names, as Windows writes them.
app-source-not-a-windows-drive = { $path } sieht nicht wie ein Windows-Systemlaufwerk aus (\Windows und \Users fehlen). Wählen Sie das Stammverzeichnis des Laufwerks vom anderen PC aus.
# Said of a drive or disk image with a Windows older than Windows 10. { $windows } is its name, such as "Windows 7
# Professional Service Pack 1, 32-bit". { $is_32_bit } is "yes" when that Windows is 32-bit, which adds the last
# sentence, and "no" when it is not.
app-source-older-windows =
    Darauf ist { $windows } installiert. Daraus erstellte Installationsprogramme sind für Windows 10 und 11 bestimmt. Mit der Option „Benutzerdaten“ werden auch Einstellungen übernommen, die Windows für eine ältere App unbemerkt an anderer Stelle gespeichert hat, und Laufzeitkomponenten, die Windows 11 nicht enthält, z. B. DirectX 9, werden mit der App installiert.{ $is_32_bit ->
        [yes] { " " }Die Apps sind 32-Bit-Apps und werden daher in Program Files (x86) installiert, wo 64-Bit-Windows sie ablegt.
       *[no] { "" }
    }
# The same, of a Windows whose name could not be read. { $is_32_bit } is as above.
app-source-older-windows-unnamed =
    Darauf ist eine ältere Windows-Version installiert. Daraus erstellte Installationsprogramme sind für Windows 10 und 11 bestimmt. Mit der Option „Benutzerdaten“ werden auch Einstellungen übernommen, die Windows für eine ältere App unbemerkt an anderer Stelle gespeichert hat, und Laufzeitkomponenten, die Windows 11 nicht enthält, z. B. DirectX 9, werden mit der App installiert.{ $is_32_bit ->
        [yes] { " " }Die Apps sind 32-Bit-Apps und werden daher in Program Files (x86) installiert, wo 64-Bit-Windows sie ablegt.
       *[no] { "" }
    }

## The details panel, before an app is picked, and the app picked

app-details-pair-first = Mit dem alten PC koppeln
app-details-pair-first-hint = Sobald WinAppMigrator Client dort ausgeführt wird und Sie seinen Code eingegeben haben, werden die Apps des alten PCs hier aufgeführt, und Sie erstellen aus einer davon ein Installationsprogramm wie bei diesem PC.
app-details-pick-an-app = Anwendung auswählen
app-details-pick-an-app-hint = Wählen Sie eine App in der Liste aus, um festzulegen, was enthalten sein soll, und ihr Installationsprogramm zu erstellen.
# { $publisher } is who made the app, and { $version } its version, such as 11.4.2.
app-details-publisher-and-version = { $publisher }, Version { $version }
# { $version } is the app's version, such as 11.4.2.
app-details-unknown-publisher-and-version = Unbekannter Herausgeber, Version { $version }
app-details-unknown-publisher = Unbekannter Herausgeber

## What goes in besides the program

app-option-license = Lizenz- und Aktivierungsdaten
app-option-license-hint = Sind möglicherweise an diesen PC gebunden und müssen auf dem neuen PC erneut aktiviert werden.
app-option-user-data = Benutzerdaten
# Documents, Saved Games, and ProgramData are folders of Windows: use the names Windows gives them in your language,
# except ProgramData, which keeps its name.
app-option-user-data-hint = Einstellungen und Verlauf, Dokumente, Gespeicherte Spiele und ProgramData.
app-option-whose-data = Wessen Daten
app-option-whose-data-hint = Ein Installationsprogramm enthält die Daten eines Kontos. Die Daten gehen an die Person, die dort angemeldet ist, wo es ausgeführt wird.

## Building the installer

app-output-folder = Ausgabeordner
# An example of a folder, shown in the empty field. "You" stands for the name of an account.
app-output-folder-example = C:\Users\Benutzername\WinAppMigrator
app-output-folder-not-full = Geben Sie einen vollständigen Ordnerpfad ein, z. B. D:\Installationsprogramme.
app-build-create = Installationsprogramm erstellen
app-build-create-again = Installationsprogramm erneut erstellen
app-build-cancel = Abbrechen
app-build-cancelling = Wird nach dem aktuellen Schritt abgebrochen…
app-build-cancelled = Erstellung abgebrochen. Es wurde kein Installationsprogramm geschrieben.
# winappmigrator-installer.exe is a file name.
app-build-no-stub = Der Installationsprogramm-Stub (winappmigrator-installer.exe) wurde nicht neben dieser App gefunden, daher kann kein selbstextrahierendes Installationsprogramm erstellt werden.
app-build-select-an-app-first = Wählen Sie zuerst eine Anwendung aus, die übertragen werden soll.
app-build-look-again-first = Diese App muss erneut analysiert werden, bevor ihr Installationsprogramm erstellt werden kann.
# { $reason } is what went wrong, in the program's own words, often in English.
app-build-stopped-unexpectedly = Die Erstellung wurde unerwartet beendet: { $reason }. Falls dabei ein Installationsprogramm im Ausgabeordner zurückgeblieben ist, verwenden Sie es nicht.
app-build-worker-disconnected = Die Verbindung zum Übertragungsprozess wurde getrennt.
# { $path } is the output folder, and { $error } what Windows said went wrong.
app-build-no-output-folder = Der Ausgabeordner { $path } konnte nicht erstellt werden: { $error }
# { $error } is what went wrong.
app-build-no-staging-folder = Es konnte kein geschützter Bereitstellungsordner erstellt werden, daher wurde nichts erstellt: { $error }
# { $error } is what went wrong, as a sentence.
app-build-could-not-build = Das Installationsprogramm konnte nicht erstellt werden. { $error }
app-build-no-installer = Die Erstellung wurde ohne Installationsprogramm abgeschlossen.
# { $error } is what went wrong.
app-build-hash-check-did-not-run = Das Installationsprogramm wurde geschrieben, aber seine Hashprüfung konnte nicht ausgeführt werden: { $error }. Verwenden Sie es nicht.

## How far a build has got

app-progress-preparing = Inhalt wird ermittelt…
app-progress-checking-room = Speicherplatz wird überprüft…
app-progress-bundling = Laufzeitkomponenten und Treiber werden gebündelt…
# { $done } is how many files have been read so far, and { $total } how many there are.
app-progress-packing = Dateien werden gelesen und komprimiert: { $done } von { $total }
# { $done } is how many of the installer's records have been looked over so far, and { $total } how many there are.
app-progress-looking-over = Angaben des Installationsprogramms werden überprüft: { $done } von { $total }…
app-progress-writing = Installationsprogramm wird geschrieben…
app-progress-verifying = Hashes werden überprüft…

## What a build made

app-result-failed-hash-check = Verwenden Sie dieses Installationsprogramm nicht: Es hat die Hashprüfung nicht bestanden.
# { $count } is how many files were gone when the build came to read them.
app-result-files-gone =
    { $count ->
        [one] Installationsprogramm erstellt, aber { $count } Datei war seit der Analyse der App nicht mehr vorhanden und ist daher nicht enthalten. Siehe „Warnungen“ unten.
       *[other] Installationsprogramm erstellt, aber { $count } Dateien waren seit der Analyse der App nicht mehr vorhanden und sind daher nicht enthalten. Siehe „Warnungen“ unten.
    }
# { $count } is how many files could not be read into the installer.
app-result-files-not-captured =
    { $count ->
        [one] Installationsprogramm erstellt, aber { $count } Datei konnte nicht erfasst werden, meist weil sie gesperrt oder nicht lesbar war. Die App funktioniert ohne sie möglicherweise nicht. Siehe „Warnungen“ unten.
       *[other] Installationsprogramm erstellt, aber { $count } Dateien konnten nicht erfasst werden, meist weil sie gesperrt oder nicht lesbar waren. Die App funktioniert ohne sie möglicherweise nicht. Siehe „Warnungen“ unten.
    }
app-result-mentions-who = Installationsprogramm erstellt, aber es enthält noch Hinweise darauf, von wem es stammt. Lesen Sie die Angaben unten, bevor Sie es weitergeben.
app-result-ready = Das Installationsprogramm ist bereit. Sein Paket hat die Hashprüfung bestanden.
app-result-files = Dateien
app-result-installer-size = Größe des Installationsprogramms
app-result-warnings = Warnungen
app-result-log = Protokoll
app-result-copy-error = Fehler kopieren
app-result-installer = Installationsprogramm
app-result-data-file = Datendatei
app-result-open-folder = Ordner öffnen
app-result-copy-path = Pfad kopieren
# { $error } is what Windows said went wrong.
app-result-could-not-open-folder = Der Ordner konnte nicht geöffnet werden: { $error }
# { $installer } and { $data_file } are file names, such as App_installer.exe and App_installer.wamdata.
app-result-two-files = Diese App ist zu groß für eine einzelne ausführbare Datei, daher besteht ihr Installationsprogramm aus zwei Dateien: { $installer } und { $data_file }. Die beiden Dateien müssen zusammenbleiben: Kopieren Sie beide mit diesen Namen in denselben Ordner auf dem anderen PC. Wenn Sie das Installationsprogramm signieren, gilt die Signatur auch für die Datendatei.
# { $app_name } is the app's name. "Unknown publisher" is what Windows shows when it asks to run an unsigned program: use
# Windows' own words for it in your language.
app-result-run-it = Führen Sie es auf dem anderen PC aus, um { $app_name } erneut zu installieren. Es überprüft seine Dateien mit SHA-256. Windows zeigt „Unbekannter Herausgeber“ an, da von Ihnen erstellte Installationsprogramme nicht signiert sind.
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-verification-errors = Überprüfungsfehler ({ $count })
# A heading over a list that opens and closes. { $count } is how many are in it.
app-result-warnings-heading = Warnungen ({ $count })
# After the first of a long list of warnings. { $count } is how many more there are.
app-warnings-and-more = und { $count } weitere.

## Looking at an app, to see what an installer of it would hold

app-scan-getting-ready = Analyse dieser App wird vorbereitet…
app-scan-install-record = Es wird gelesen, wie die App installiert wurde…
app-scan-near-matches = Es wird gesucht, was möglicherweise auch dazugehört…
app-scan-system-items = Es wird nach Diensten, Aufgaben und Verknüpfungen gesucht…
app-scan-files = Dateien werden durchgesehen…
app-scan-registry = Einbindung in Windows wird gelesen…
app-scan-finishing = Wird abgeschlossen…
app-scan-look-again = Erneut analysieren
app-scan-stop = Beenden
app-scan-running-hint = Währenddessen können Sie eine andere App auswählen oder ändern, was enthalten sein soll.
# { $error } is why, in words that follow a colon.
app-scan-failed = Diese App konnte nicht analysiert werden: { $error }
app-scan-try-again = Erneut versuchen
app-scan-stopped = Beendet. Ein Installationsprogramm kann erstellt werden, sobald diese App analysiert wurde.
# { $reason } is what went wrong, in the program's own words, often in English.
app-scan-stopped-unexpectedly = Der Vorgang wurde unerwartet beendet: { $reason }.

## How big an installer can be, and whether there is room for it

app-room-files = Dateien
# Over how big the installer can be: up to that size.
app-room-up-to = Bis zu
app-room-free = Frei
# { $drive } is a drive, such as C:.
app-room-free-on = Frei auf { $drive }
# How much room a drive has, when it would not say.
app-room-free-unknown = Unbekannt
app-room-most-it-could-need = Der höchstens benötigte Speicherplatz vor dem Komprimieren.
# { $drive } is a drive, such as C:, { $needed } how much room it needs, such as 1.2 GB, and { $free } how much it has.
app-room-drive-short = { $drive } benötigt bis zu { $needed } freien Speicherplatz für diese Erstellung und hat { $free }.
# { $drives } is one sentence like the one above for each drive without the room, one after another.
app-room-not-enough = Es ist nicht genügend Speicherplatz vorhanden. { $drives } Wählen Sie einen Ausgabeordner auf einem anderen Laufwerk aus, oder lassen Sie etwas weg.
# { $note } is a sentence, without its full stop, about a drive whose room could not be checked. Use your language's
# full stop.
app-room-could-not-tell = { $note }.

## What goes in the installer, part by part

app-footprint-heading = Inhalt des Installationsprogramms
app-footprint-working-out = Es wird ermittelt, was dazugehört…
# { $error } is why.
app-footprint-would-not-make-a-package = Der Rest ergäbe kein Paket: { $error }
# { $list } is the names of what else is left out, such as a service and a shortcut.
app-footprint-also-left-out = Dadurch entfällt auch Folgendes, da das, was es ausführt oder nennt, fehlen würde: { $list }. Diese Elemente werden auf dem anderen PC nicht eingerichtet.
# { $list } is the names of the first few of what else is left out, and { $count } how many more there are.
app-footprint-also-left-out-and-more = Dadurch entfällt auch Folgendes, da das, was es ausführt oder nennt, fehlen würde: { $list } und { $count } weitere. Diese Elemente werden auf dem anderen PC nicht eingerichtet.
# { $list } is the names of license files. The option is the one named "License and Activation Data".
app-footprint-licenses-still-go = Lizenzdaten in dem, was Sie ausgelassen haben, werden trotzdem aufgenommen, da sie sich nach der Option „Lizenz- und Aktivierungsdaten“ richten: { $list }.
# { $elapsed } is how long ago, in minutes and seconds such as 45:00, or hours, minutes, and seconds such as 1:05:00.
# "Look Again" names the button of that name.
app-footprint-looked-at-long-ago = Diese App wurde vor { $elapsed } analysiert. Dateien werden beim Erstellen gelesen, sodass Änderungen übernommen und inzwischen entfernte Elemente aufgeführt werden. Wählen Sie „Erneut analysieren“, um Neues zu erfassen.
# Headings of the parts of the list, each with a line that says what is in it.
app-footprint-program-files = Programmdateien
app-footprint-data-folders = Datenordner
# What could go in with the User Data option, when nothing else is in this part. { $size } is how much, such as 1.2 GB.
app-footprint-folders-with-user-data = { $size } mit der Option „Benutzerdaten“
# { $included } is how many folders go in, { $count } how many there are, and { $size } how much those that go in hold.
app-footprint-folders-summary =
    { $count ->
        [one] { $included } von { $count } Ordner, { $size }
       *[other] { $included } von { $count } Ordnern, { $size }
    }
# { $size } is how much, such as 1.2 GB, and { $count } in how many files.
app-footprint-data-only-with-user-data =
    { $count ->
        [one] { $size } in { $count } Datei mit Daten dieser App werden nur mit der Option „Benutzerdaten“ aufgenommen.
       *[other] { $size } in { $count } Dateien mit Daten dieser App werden nur mit der Option „Benutzerdaten“ aufgenommen.
    }
# A folder's row. { $count } is how many files it holds, and { $detail } where it is or why it cannot be left out.
app-footprint-folder-row =
    { $count ->
        [one] { $count } Datei. { $detail }
       *[other] { $count } Dateien. { $detail }
    }
app-footprint-services = Dienste
app-footprint-startup = Autostart
app-footprint-tasks = Geplante Aufgaben
app-footprint-shortcuts = Verknüpfungen
# { $included } is how many of the part's things go in, and { $count } how many there are.
app-footprint-items-summary = { $included } von { $count }
# { $why } is what else was left out that this goes with.
app-footprint-left-out-too = Ebenfalls ausgelassen: { $why }
# A row's detail. { $kind } is what it is, such as Service, and { $detail } what it does.
app-footprint-change-detail = { $kind }. { $detail }
app-footprint-integrations = Einbindung in Windows
# { $included } is how many kinds go in, such as file types and firewall rules, and { $count } how many kinds there are.
app-footprint-integrations-summary =
    { $count ->
        [one] { $included } von { $count } Art
       *[other] { $included } von { $count } Arten
    }
app-footprint-integration-nothing-left = Wegen Ihrer Auslassungen bleibt davon nichts übrig
# { $count } is how many things of this kind the installer sets up.
app-footprint-integration-things =
    { $count ->
        [one] { $count } eingerichtetes Element
       *[other] { $count } eingerichtete Elemente
    }
app-footprint-prerequisites = Zur Ausführung benötigt
# { $count } is how many runtimes, such as Visual C++ Redistributable.
app-footprint-prerequisites-summary =
    { $count ->
        [one] { $count } Laufzeitkomponente
       *[other] { $count } Laufzeitkomponenten
    }
app-footprint-near-matches = Gehört möglicherweise auch dazu
# { $ticked } is how many were ticked, and { $count } how many there are.
app-footprint-near-matches-summary = { $ticked } von { $count } hinzugefügt
app-footprint-near-matches-hint = Diese Elemente sind nur im Installationsprogramm enthalten, wenn Sie sie aktivieren. Wenn Sie eines aktivieren, wird die App erneut analysiert, mit denselben Prüfungen wie beim Rest, sodass alles, was wie ein Kennwort aussieht, trotzdem draußen bleibt.
# { $why } is why it might be part of the app, as a sentence.
app-footprint-near-match-with-user-data = Wird mit der Option „Benutzerdaten“ aufgenommen. { $why }
# How much a folder holds, when not all of it could be measured. { $size } is how much was, such as 1.2 GB.
app-footprint-near-match-over = über { $size }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-folder = Ordner: { $place }
# { $place } is where it is, a folder or a registry key.
app-footprint-near-match-hidden = Ausgeblendeter Teil einer App: { $place }
app-footprint-nothing-left-out = Es wurde nichts ausgelassen, weil es wie ein Kennwort, eine Lizenz oder ein Zertifikat aussah.
app-footprint-left-out = Ausgelassen und warum
# { $count } is how many things were left out.
app-footprint-left-out-summary =
    { $count ->
        [one] { $count } Element
       *[other] { $count } Elemente
    }
# Above a list of things left out for one reason. { $title } is the reason.
app-footprint-left-out-group = { $title }:
app-footprint-refused = Kein Installationsprogramm dürfte Folgendes einrichten:
# A heading over a list that opens and closes. { $count } is how many are in it.
app-footprint-worth-knowing = Wissenswertes ({ $count })
# { $list } is what kinds of things were looked for and none found, from the five below.
app-none-found = Nicht gefunden: { $list }.
app-none-found-services = Dienste
app-none-found-startup = Autostart-Elemente
app-none-found-tasks = geplante Aufgaben
app-none-found-shortcuts = Verknüpfungen
app-none-found-integrations = Einbindungen in Windows

## Network PC: reading another PC through WinAppMigrator Client, in three steps. "WinAppMigrator Client" is the name of a
## program, and the old PC is the one the apps are moved off.

app-network-step-client = 1. Den Client auf den alten PC bringen
app-network-step-client-hint = WinAppMigrator Client wird auf dem PC ausgeführt, auf dem sich die App jetzt befindet (Windows 7 oder höher). Er lässt nur diesen PC darauf lesend zugreifen, verschlüsselt und mit einem Code, den er anzeigt.
# { $drive } is a USB drive, such as E:\.
app-network-save-to-usb = Auf USB-Laufwerk { $drive } speichern
# { $drive } is a USB drive, such as E:\, and { $name } the name it was given, such as BACKUP.
app-network-save-to-named-usb = Auf USB-Laufwerk { $drive } ({ $name }) speichern
app-network-save-to-folder = In einem Ordner speichern…
app-network-look-for-usb-again = Erneut nach USB-Laufwerken suchen
app-network-no-usb = Es ist kein USB-Laufwerk angeschlossen. Schließen Sie eines an, oder speichern Sie in einem Ordner, auf den Sie vom alten PC aus zugreifen können, z. B. in einem freigegebenen Ordner in Ihrem Netzwerk.
# { $path } is where the client was saved, such as E:\WinAppMigrator Client.exe.
app-network-client-saved = Gespeichert als { $path }. Schließen Sie auf dem alten PC die App, die Sie übertragen, doppelklicken Sie dann auf diese Datei, und lassen Sie Änderungen zu. Der Client zeigt dann einen Code an.
app-network-step-pick = 2. Den alten PC auswählen
app-network-looking = Im Netzwerk wird nach PCs gesucht, auf denen der Client ausgeführt wird…
app-network-type-name = Oder Namen bzw. Adresse eingeben
# An example, shown in the empty field: a PC's name, or its network address.
app-network-type-name-example = ALTER-PC oder 192.168.1.20
app-network-step-code = 3. Den angezeigten Code eingeben
app-network-code = Kopplungscode
app-network-connect = Verbinden
app-network-pairing = Wird gekoppelt…
# Why pairing did not work, shown on its own.
app-network-pairing-stopped = Die Kopplung wurde vor dem Abschluss beendet.
app-network-wrong-code = Das ist nicht der Code, den der alte PC anzeigt. Prüfen Sie ihn, und versuchen Sie es erneut.
# { $error } is what went wrong.
app-network-not-reached = Der alte PC war nicht erreichbar: { $error }
app-network-pick-folder-title = Ordner für WinAppMigrator Client auswählen
# { $path } is the folder, and { $error } what Windows said went wrong.
app-client-could-not-save = Der Client konnte nicht in { $path } gespeichert werden: { $error }
# scripts\build-client.ps1 is a file name.
app-client-not-carried = Dieser Build von WinAppMigrator enthält den Client nicht. Erstellen Sie ihn mit scripts\build-client.ps1, und erstellen Sie dann die App erneut.
# { $error } is what Windows said went wrong.
app-client-could-not-save-note = Der Hinweis daneben konnte nicht gespeichert werden: { $error }
# A note saved as a text file beside the client, for whoever runs it on the old PC. Keep its lines short, as it is read in
# Notepad. "WinAppMigrator Client.exe" is a file name. "Network PC..." names the Scan Source button: use its words in your
# language.
app-client-note =
    WinAppMigrator Client
    
    Führen Sie dieses Programm auf dem PC aus, von dem Sie Apps übertragen:
    Windows 7, 8, 10 oder 11.
    
    1. Schließen Sie die App, die Sie übertragen.
    2. Doppelklicken Sie auf „WinAppMigrator Client.exe“, und lassen Sie
       Änderungen zu.
    3. Ein 6-stelliger Kopplungscode wird angezeigt. Wählen Sie in WinAppMigrator
       auf Ihrem neuen PC Netzwerk-PC..., wählen Sie diesen PC aus, und geben
       Sie den Code ein.
    4. Wenn der neue PC fertig ist, drücken Sie im Fenster des Clients die
       Eingabetaste, oder schließen Sie das Fenster.
    
    Der Client liest nur. Während er läuft, behält er eine Momentaufnahme des
    Windows-Laufwerks und lässt seine beiden eigenen Ports in der
    Windows-Firewall für Ihr lokales Netzwerk zu. Beides entfernt er, wenn er
    beendet wird. Was über das Netzwerk übertragen wird, ist verschlüsselt, und
    nur ein PC, der den Code hat, kann etwas lesen.
